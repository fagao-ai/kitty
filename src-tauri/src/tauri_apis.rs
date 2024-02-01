use anyhow::{anyhow, Result};
use entity::base_config;
#[cfg(feature = "hysteria")]
use entity::hysteria::{self as hysteria_entity, HysteriaConfig};

#[cfg(feature = "xray")]
use entity::xray::{self as xray_entity, XrayConfig};

use kitty_proxy::{HttpProxy, NodeInfo, SocksProxy};
use protocols::KittyCommandGroup;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
};
use tauri::{AppHandle, Manager, State};
use tokio::sync::watch;

use crate::{
    state::{DatabaseState, KittyProxyState, ProcessManagerState},
    tauri_apis::utils::relative_command_path,
    types::{CommandResult, KittyCommandError, KittyResponse},
};

use utils::get_http_socks_ports;

#[cfg(feature = "hysteria")]
pub mod hysteria;

pub mod common;
#[cfg(feature = "xray")]
pub mod xray;

pub mod utils;

async fn init_state<'a>(
    process_state: &State<'a, ProcessManagerState>,
    proxy_state: &State<'a, KittyProxyState>,
) -> Result<()> {
    #[cfg(feature = "hysteria")]
    {
        let mut hy_command_group = process_state.hy_process_manager.lock().await;
        let hy_command_group_mut = hy_command_group.as_mut();
        if let Some(hy_command_group) = hy_command_group_mut {
            hy_command_group.terminate_backends()?;
        };
        *hy_command_group = None;
    }

    #[cfg(feature = "xray")]
    {
        let mut xray_command_group = process_state.xray_process_manager.lock().await;
        let xray_command_group_mut = xray_command_group.as_mut();
        if let Some(xray_command_group) = xray_command_group_mut {
            xray_command_group.terminate_backends()?;
        };
        *xray_command_group = None;
    }
    let mut http_proxy_sx = proxy_state.http_proxy_sx.lock().await;
    let http_proxy_kill_sx = http_proxy_sx.as_mut();
    if let Some(kill_sx) = http_proxy_kill_sx {
        kill_sx.send(true).unwrap_or(());
    };
    *http_proxy_sx = None;

    let mut socks_proxy_sx = proxy_state.socks_proxy_sx.lock().await;
    let socks_proxy_kill_sx = socks_proxy_sx.as_mut();
    if let Some(kill_sx) = socks_proxy_kill_sx {
        kill_sx.send(true).unwrap_or(());
    }
    *socks_proxy_sx = None;
    proxy_state.used_ports.lock().await.clear();
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn start_system_proxy<'a>(
    app_handle: AppHandle,
    process_state: State<'a, ProcessManagerState>,
    proxy_state: State<'a, KittyProxyState>,
    db_state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<()>> {
    let _ = init_state(&process_state, &proxy_state).await?;
    let db = db_state.get_db();
    let config_dir = app_handle.path().config_dir()?;
    let mut http_vpn_node_infos = Vec::new();
    let mut socks_vpn_node_infos = Vec::new();
    let mut used_ports = proxy_state.used_ports.lock().await;
    let mut start_cmd_flag = false;
    #[cfg(feature = "hysteria")]
    {
        let hysteria_record = hysteria_entity::Model::first(&db).await?;
        if let Some(hysteria_record) = hysteria_record {
            let (http_port, socks_port) = get_http_socks_ports(&mut used_ports);
            let hysteria_config = HysteriaConfig::new(http_port, socks_port, hysteria_record);
            let hysteria_bin_path = relative_command_path("hysteria".as_ref())?;
            let mut hysteria_command_group = KittyCommandGroup::new(
                String::from("hysteria"),
                hysteria_bin_path,
                config_dir.clone(),
            );
            let mut config_hash_map: HashMap<String, HysteriaConfig> = HashMap::new();
            config_hash_map.insert(hysteria_config.server.clone(), hysteria_config);
            let _ = hysteria_command_group.start_commands(config_hash_map, None);
            *process_state.hy_process_manager.lock().await = Some(hysteria_command_group);
            http_vpn_node_infos.push(NodeInfo::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                http_port,
                1,
            ));
            socks_vpn_node_infos.push(NodeInfo::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                socks_port,
                1,
            ));
            start_cmd_flag = true;
        }
    }

    #[cfg(feature = "xray")]
    {
        let xray_records = xray_entity::Model::fetch_all(&db).await?;
        if xray_records.len() > 0 {
            let node_number = xray_records.len();
            let (http_port, socks_port) = get_http_socks_ports(&mut used_ports);
            let server_key: String = xray_records
                .iter()
                .map(|x| x.get_server())
                .collect::<Vec<String>>()
                .join("_");
            let xray_config = XrayConfig::new(http_port, socks_port, xray_records);
            let xray_bin_path = relative_command_path("xray".as_ref())?;
            let resource_dir = app_handle.path().resource_dir()?;
            let mut env_var = HashMap::new();
            env_var.insert(
                "XRAY_LOCATION_ASSET".to_string(),
                resource_dir.to_string_lossy().to_string(),
            );
            let mut config_hash_map: HashMap<String, XrayConfig> = HashMap::new();
            let mut xray_command_group =
                KittyCommandGroup::new(String::from("xray"), xray_bin_path, config_dir);

            config_hash_map.insert(server_key, xray_config);
            let _ = xray_command_group.start_commands(config_hash_map, None);
            *process_state.hy_process_manager.lock().await = Some(xray_command_group);
            http_vpn_node_infos.push(NodeInfo::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                http_port,
                node_number.try_into().unwrap(),
            ));
            socks_vpn_node_infos.push(NodeInfo::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                socks_port,
                node_number.try_into().unwrap(),
            ));
            start_cmd_flag = true;
        }
    }
    if !start_cmd_flag {
        let error = anyhow!("Not have any proxy, please add proxy");
        return Err(KittyCommandError::AnyHowError(error));
    }
    let record: base_config::Model = base_config::Model::first(&db).await.unwrap().unwrap();
    let http_port = record.http_port;
    let socks_port = record.socks_port;
    let mut http_proxy = HttpProxy::new(record.local_ip.as_str(), http_port, None)
        .await
        .unwrap();
    let mut socks_proxy = SocksProxy::new(record.local_ip.as_str(), socks_port, None)
        .await
        .unwrap();
    let match_proxy = proxy_state.match_proxy.lock().await;
    let http_match_proxy = match_proxy.clone().unwrap();
    let socks_match_proxy = match_proxy.clone().unwrap();
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

    Ok(KittyResponse::default())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn stop_system_proxy<'a>(
    db_state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<()>> {
    let db = db_state.get_db();
    base_config::Model::update_sysproxy_flag(&db, false).await?;
    Ok(KittyResponse::default())
}
