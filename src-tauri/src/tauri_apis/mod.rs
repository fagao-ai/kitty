//! Tauri API handlers for kitty.
//!
//! This module contains all the Tauri command handlers for interacting with
//! the kitty application.

use anyhow::{anyhow, Result};
use entity::utils::is_port_available;
use entity::{
    base_config,
    hysteria::{self as hysteria_entity},
    xray::{self as xray_entity},
};
use serde::Serialize;

use tauri::{AppHandle, Manager, State};

use crate::{
    config_converter::ShoesConfigConverter,
    proxy::system_proxy::{clear_system_proxy, set_system_proxy},
    state::{DatabaseState, ProcessManagerState},
    types::{CommandResult, KittyCommandError, KittyResponse},
};

pub mod common;
pub mod proxy;
pub mod utils;
pub mod server;
pub mod subscription;

// ============================================================================
// System Proxy Commands (moved from tauri_apis.rs)
// ============================================================================

async fn init_state<'a>(
    process_state: &State<'a, ProcessManagerState>,
) -> Result<()> {
    // Abort all running shoes servers
    let mut running_servers = process_state.running_servers.lock().await;
    for handle in running_servers.iter() {
        handle.abort();
    }
    running_servers.clear();
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
    let resolver = std::sync::Arc::new(shoes::resolver::CachingNativeResolver::new()) as std::sync::Arc<dyn shoes::resolver::Resolver>;
    shoes::tcp::tcp_server::start_servers(config, resolver).await
}

/// Start all configured proxy servers (without setting system proxy).
///
/// This command:
/// 1. Initializes state by stopping any running servers
/// 2. Reads base config to get proxy settings
/// 3. Starts hysteria server if configured
/// 4. Starts xray server(s) if configured
#[tauri::command(rename_all = "snake_case")]
pub async fn start_all_servers<'a>(
    app_handle: AppHandle,
    process_state: State<'a, ProcessManagerState>,
    db_state: State<'a, DatabaseState>,
    xray_id: Option<i32>,
) -> CommandResult<KittyResponse<()>> {

    let _ = init_state(&process_state).await?;
    let db = db_state.get_db();
    let mut all_server_handles = Vec::new();
    let mut start_cmd_flag = false;

    // Get the resource directory for geo files
    let resource_dir = app_handle.path().resource_dir()
        .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to get resource dir: {}", e)))?;

    // Get custom rules path
    let custom_rules_path = app_handle.path().app_data_dir()
        .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to get app data dir: {}", e)))?
        .join("custom_rules.json");

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
        let yaml_config = ShoesConfigConverter::hysteria_to_socks_http_yaml(
            &hysteria_record,
            http_port,
            socks_port,
            &resource_dir,
            Some(&custom_rules_path),
        )
        .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to convert hysteria config: {}", e)))?;

        // Start shoes servers directly
        let handles = start_shoes_servers(&yaml_config).await
            .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to start hysteria: {}", e)))?;
        all_server_handles.extend(handles);
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
        let yaml_config = ShoesConfigConverter::xray_multi_to_yaml(
            &xray_records,
            http_port,
            socks_port,
            &resource_dir,
            Some(&custom_rules_path),
        )
        .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to convert xray config: {}", e)))?;

        // Start shoes servers directly
        let handles = start_shoes_servers(&yaml_config).await
            .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to start xray: {}", e)))?;
        all_server_handles.extend(handles);
        start_cmd_flag = true;
    }

    if !start_cmd_flag {
        let error = anyhow!("Not have any proxy, please add proxy");
        return Err(KittyCommandError::AnyHowError(error));
    }

    // Store all server handles in state
    *process_state.running_servers.lock().await = all_server_handles;

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

/// Set system proxy only (without starting servers).
///
/// This command only sets the system proxy settings.
/// Servers should already be running before calling this.
#[tauri::command(rename_all = "snake_case")]
pub async fn set_system_proxy_only<'a>(
    db_state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<()>> {
    let db = db_state.get_db();
    let record: base_config::Model = base_config::Model::first(&db).await?
        .ok_or_else(|| anyhow::anyhow!("Base config not found"))?;

    set_system_proxy(&record.local_ip, record.socks_port, Some(record.http_port));
    base_config::Model::update_sysproxy_flag(&db, true).await?;
    Ok(KittyResponse::default())
}

// Re-export commonly used items

// ============================================================================
// Active Proxy Management Commands
// ============================================================================

/// Information about the currently active proxy.
#[derive(Serialize)]
pub struct ActiveProxyInfo {
    id: u32,
    proxy_type: String,
}

/// Get the currently active proxy information.
#[tauri::command(rename_all = "snake_case")]
pub async fn get_active_proxy<'a>(
    process_manager: State<'a, ProcessManagerState>,
) -> CommandResult<KittyResponse<Option<ActiveProxyInfo>>> {
    let id = *process_manager.active_proxy_id.lock().await;
    let proxy_type = process_manager.active_proxy_type.lock().await.clone();

    let info = match (id, proxy_type) {
        (Some(id), Some(pt)) => Some(ActiveProxyInfo { id, proxy_type: pt }),
        _ => None,
    };

    Ok(KittyResponse::from_data(info))
}

/// Switch to a specific proxy server.
///
/// This command:
/// 1. Stops all currently running servers
/// 2. Starts the specified proxy server
/// 3. Updates the active proxy state
#[tauri::command(rename_all = "snake_case")]
pub async fn switch_to_proxy<'a>(
    app_handle: AppHandle,
    db_state: State<'a, DatabaseState>,
    process_manager: State<'a, ProcessManagerState>,
    proxy_id: u32,
    proxy_type: String,
) -> CommandResult<KittyResponse<()>> {
    let db = db_state.get_db();

    // Get the resource directory for geo files
    let resource_dir = app_handle.path().resource_dir()
        .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to get resource dir: {}", e)))?;

    // Stop all running servers
    let mut servers = process_manager.running_servers.lock().await;
    for handle in servers.drain(..) {
        handle.abort();
    }
    drop(servers);

    // Get ports from base config
    let record = base_config::Model::first(&db).await?
        .ok_or_else(|| anyhow!("Base config not found"))?;
    let http_port = record.http_port;
    let socks_port = record.socks_port;

    // Get custom rules path
    let custom_rules_path = app_handle.path().app_data_dir()
        .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to get app data dir: {}", e)))?
        .join("custom_rules.json");

    let yaml_config = if proxy_type == "hysteria" {
        let hysteria_record = hysteria_entity::Model::get_by_id(&db, proxy_id as i32).await?
            .ok_or_else(|| anyhow!("Hysteria record {} not found", proxy_id))?;

        ShoesConfigConverter::hysteria_to_socks_http_yaml(
            &hysteria_record,
            http_port,
            socks_port,
            &resource_dir,
            Some(&custom_rules_path),
        )?
    } else {
        let xray_record = xray_entity::Model::get_by_id(&db, proxy_id as i32).await?
            .ok_or_else(|| anyhow!("Xray record {} not found", proxy_id))?;

        ShoesConfigConverter::xray_to_socks_http_yaml(
            &xray_record,
            http_port,
            socks_port,
            &resource_dir,
            Some(&custom_rules_path),
        )?
    };

    // Start the new server
    let handles = start_shoes_servers(&yaml_config).await?;
    let mut servers = process_manager.running_servers.lock().await;
    servers.extend(handles);

    // Update active proxy state
    *process_manager.active_proxy_id.lock().await = Some(proxy_id);
    *process_manager.active_proxy_type.lock().await = Some(proxy_type.clone());

    Ok(KittyResponse::default())
}
