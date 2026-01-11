//! Tauri API handlers for kitty.
//!
//! This module contains all the Tauri command handlers for interacting with
//! the kitty application.

use anyhow::{anyhow, Result};
use entity::utils::is_port_available;
use entity::{
    base_config,
    hysteria::{self as hysteria_entity},
    rules::{self},
    xray::{self as xray_entity},
};

use kitty_proxy::{HttpProxy, NodeInfo, SocksProxy};

use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
};
use tauri::{AppHandle, Manager, State};
use tokio::sync::watch;

use std::sync::Arc;

use crate::{
    config_converter::ShoesConfigConverter,
    proxy::system_proxy::{clear_system_proxy, set_system_proxy},
    state::{DatabaseState, KittyProxyState, ProcessManagerState},
    tauri_apis::utils::add_rule2match_proxy,
    types::{CommandResult, KittyCommandError, KittyResponse},
};

pub mod common;
// Legacy API modules for xray and hysteria (still using database entities)
#[cfg(feature = "hysteria")]
pub mod hysteria;
#[cfg(feature = "xray")]
pub mod xray;
pub mod utils;
pub mod server;

// ============================================================================
// System Proxy Commands (moved from tauri_apis.rs)
// ============================================================================

async fn init_state<'a>(
    process_state: &State<'a, ProcessManagerState>,
    proxy_state: &State<'a, KittyProxyState>,
) -> Result<()> {
    // Abort all running shoes servers
    let mut running_servers = process_state.running_servers.lock().await;
    for handle in running_servers.iter() {
        handle.abort();
    }
    running_servers.clear();

    // Stop HTTP and SOCKS proxies
    let mut http_proxy_sx = proxy_state.http_proxy_sx.lock().await;
    if let Some(kill_sx) = http_proxy_sx.as_ref() {
        kill_sx.send(true).unwrap_or(());
    }
    *http_proxy_sx = None;

    let mut socks_proxy_sx = proxy_state.socks_proxy_sx.lock().await;
    if let Some(kill_sx) = socks_proxy_sx.as_ref() {
        kill_sx.send(true).unwrap_or(());
    }
    *socks_proxy_sx = None;
    proxy_state.used_ports.lock().await.clear();
    Ok(())
}

/// Start shoes proxy servers from YAML configuration
async fn start_shoes_servers(yaml_config: &str) -> Result<Vec<tokio::task::JoinHandle<()>>> {
    // Parse YAML config using shoes library (ffi feature enables load_config_str)
    let configs = shoes::config::load_config_str(yaml_config)
        .map_err(|e| anyhow!("Failed to parse shoes YAML config: {e}"))?;

    let mut all_handles = Vec::new();
    for config in configs {
        // Start each server using shoes library tcp_server API (now public)
        let handles = start_servers_internal(config).await
            .map_err(|e| anyhow!("Failed to start shoes server: {e}"))?;
        all_handles.extend(handles);
    }

    Ok(all_handles)
}

/// Internal function to start a single shoes server.
///
/// This is a wrapper around shoes::tcp::tcp_server::start_servers.
pub(super) async fn start_servers_internal(
    config: shoes::config::Config,
) -> std::io::Result<Vec<tokio::task::JoinHandle<()>>> {
    shoes::tcp::tcp_server::start_servers(config).await
}

/// Start the system proxy with all configured servers.
///
/// This command:
/// 1. Initializes state by stopping any running servers
/// 2. Reads base config to get proxy settings
/// 3. Starts hysteria server if configured
/// 4. Starts xray server(s) if configured
/// 5. Starts HTTP and SOCKS5 proxies
/// 6. Sets system proxy settings
#[tauri::command(rename_all = "snake_case")]
pub async fn start_system_proxy<'a>(
    app_handle: AppHandle,
    process_state: State<'a, ProcessManagerState>,
    proxy_state: State<'a, KittyProxyState>,
    db_state: State<'a, DatabaseState>,
    xray_id: Option<i32>,
) -> CommandResult<KittyResponse<()>> {
    println!("start_system_proxy: {:?}", xray_id);

    let _ = init_state(&process_state, &proxy_state).await?;
    let db = db_state.get_db();
    let mut http_vpn_node_infos = Vec::new();
    let mut socks_vpn_node_infos = Vec::new();
    let mut used_ports = proxy_state.used_ports.lock().await;
    let mut start_cmd_flag = false;
    let mut all_server_handles = Vec::new();

    let record: base_config::Model = base_config::Model::first(&db).await.unwrap().unwrap();
    let http_port = record.http_port;
    let socks_port = record.socks_port;
    for port in vec![http_port, socks_port] {
        if !is_port_available(port) {
            return Err(KittyCommandError::AnyHowError(anyhow!(
                "port {} already is used.",
                port
            )));
        }
    }

    // Process hysteria record if exists
    let hysteria_record = hysteria_entity::Model::first(&db).await?;
    if let Some(hysteria_record) = hysteria_record {
        let (hy_http_port, hy_socks_port) = utils::get_http_socks_ports(&mut used_ports);
        let yaml_config = ShoesConfigConverter::hysteria_to_socks_http_yaml(
            &hysteria_record,
            hy_http_port,
            hy_socks_port,
        )
        .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to convert hysteria config: {}", e)))?;

        // Start shoes servers directly
        let handles = start_shoes_servers(&yaml_config).await
            .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to start hysteria: {}", e)))?;
        all_server_handles.extend(handles);

        http_vpn_node_infos.push(NodeInfo::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            hy_http_port,
            1,
        ));
        socks_vpn_node_infos.push(NodeInfo::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            hy_socks_port,
            1,
        ));
        start_cmd_flag = true;
    }

    // Process xray records
    let xray_records = if xray_id.is_none() {
        xray_entity::Model::fetch_all(&db).await?
    } else {
        if let Some(item) = xray_entity::Model::get_by_id(&db, xray_id.unwrap()).await? {
            vec![item]
        } else {
            xray_entity::Model::fetch_all(&db).await?
        }
    };

    if !xray_records.is_empty() {
        let node_number = xray_records.len();
        let (xr_http_port, xr_socks_port) = utils::get_http_socks_ports(&mut used_ports);

        let yaml_config = ShoesConfigConverter::xray_multi_to_yaml(
            &xray_records,
            xr_http_port,
            xr_socks_port,
        )
        .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to convert xray config: {}", e)))?;

        // Start shoes servers directly
        let handles = start_shoes_servers(&yaml_config).await
            .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to start xray: {}", e)))?;
        all_server_handles.extend(handles);

        http_vpn_node_infos.push(NodeInfo::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            xr_http_port,
            node_number.try_into().unwrap(),
        ));
        socks_vpn_node_infos.push(NodeInfo::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            xr_socks_port,
            node_number.try_into().unwrap(),
        ));
        start_cmd_flag = true;
    }

    if !start_cmd_flag {
        let error = anyhow!("Not have any proxy, please add proxy");
        return Err(KittyCommandError::AnyHowError(error));
    }

    // Store all server handles in state
    *process_state.running_servers.lock().await = all_server_handles;

    // Start HTTP and SOCKS proxies
    let mut http_proxy = HttpProxy::new(record.local_ip.as_str(), http_port, None)
        .await
        .unwrap();
    let mut socks_proxy = SocksProxy::new(record.local_ip.as_str(), socks_port, None)
        .await
        .unwrap();
    let match_proxy = proxy_state.match_proxy.lock().await.clone().unwrap();
    let shared_match_proxy = Arc::clone(&match_proxy);
    let rule_records = rules::Model::fetch_all(&db).await?;
    let mut match_proxy_write_share: tokio::sync::RwLockWriteGuard<'_, kitty_proxy::MatchProxy> =
        shared_match_proxy.write().await;
    for rule_record in rule_records {
        add_rule2match_proxy(&mut match_proxy_write_share, &rule_record).await;
    }
    drop(match_proxy_write_share);
    let http_match_proxy = Arc::clone(&match_proxy);
    let socks_match_proxy = Arc::clone(&match_proxy);
    let (http_kill_tx, mut http_kill_rx) = watch::channel(false);
    let (socks_kill_tx, mut socks_kill_rx) = watch::channel(false);
    *proxy_state.http_proxy_sx.lock().await = Some(http_kill_tx);
    *proxy_state.socks_proxy_sx.lock().await = Some(socks_kill_tx);
    tokio::spawn(async move {
        http_proxy
            .serve(http_match_proxy, &mut http_kill_rx, http_vpn_node_infos)
            .await;
    });
    tokio::spawn(async move {
        socks_proxy
            .serve(socks_match_proxy, &mut socks_kill_rx, socks_vpn_node_infos)
            .await;
    });
    set_system_proxy(&record.local_ip, record.socks_port, Some(record.http_port));
    let db = db_state.get_db();
    base_config::Model::update_sysproxy_flag(&db, true).await?;
    Ok(KittyResponse::default())
}

/// Stop the system proxy and clear system proxy settings.
#[tauri::command(rename_all = "snake_case")]
pub async fn stop_system_proxy<'a>(
    db_state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<()>> {
    let db = db_state.get_db();
    clear_system_proxy();
    base_config::Model::update_sysproxy_flag(&db, false).await?;
    Ok(KittyResponse::default())
}

// Re-export commonly used items
pub use server::{
    is_any_server_running, is_proxy_server_running, start_hysteria_server_by_id,
    start_proxy_server, start_servers_from_db, start_xray_server_by_id, stop_all_servers,
    stop_proxy_server,
};
