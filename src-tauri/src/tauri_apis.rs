use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use std::sync::Arc;

use protocols::{KittyCommandGroup};
use tauri::{Manager, State};

use crate::{
    state::{DatabaseState, KittyProxyState, ProcessManagerState},
    types::{CommandResult, KittyResponse},
};
use anyhow::{anyhow, Result};
use kitty_proxy::MatchProxy;
use tauri::utils::platform;
use tokio::sync::watch;

use entity::{
    base_config,
    hysteria::{self as hysteria_entity, CommandHysteria},
};

#[cfg(feature = "hysteria")]
pub mod hysteria;

#[cfg(feature = "xray")]
pub mod xray;

fn relative_command_path(command: &Path) -> Result<PathBuf> {
    match platform::current_exe()?.parent() {
        #[cfg(windows)]
        Some(exe_dir) => Ok(exe_dir.join(command).with_extension("exe")),
        #[cfg(not(windows))]
        Some(exe_dir) => Ok(exe_dir.join(command)),
        None => Err(anyhow!("current exe not has parent.")),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_system_proxy<'a>(
    app: tauri::App,
    state: State<'a, ProcessManagerState>,
    proxy_state: State<'a, KittyProxyState>,
    db_state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<bool>> {
    let db = db_state.get_db();

    let config_dir = app.app_handle().path().config_dir()?;
    #[cfg(feature = "hysteria")]
        let (kill_tx, mut kill_rx) = watch::channel(false);
    #[cfg(feature = "hysteria")]
    {
        let hysteria_record = hysteria_entity::Model::first(&db).await?.unwrap();
        let command_hysteria = CommandHysteria::try_from(&hysteria_record)?;
        let hysteria_bin_path = relative_command_path("hysteria".as_ref())?;
        let mut hysteria_command_group = KittyCommandGroup::new(String::from("hysteria"), hysteria_bin_path, config_dir);
        let mut config_hash_map = HashMap::new();
        let _http_port = command_hysteria.get_http_port();
        let _socks_port = command_hysteria.get_socks_port();
        config_hash_map.insert(command_hysteria.server.clone(), command_hysteria);
        let _ = hysteria_command_group.start_commands(config_hash_map, None);
        *state.hy_process_manager.lock().await = Some(hysteria_command_group);
        let mut http_proxy = proxy_state.http_proxy.lock().await.unwrap();
        let match_proxy = proxy_state.match_proxy.lock().await;
        let match_proxy_clone = match_proxy.clone().unwrap();

        let vpn_node_infos = vec![];
        tokio::spawn(async move {
            http_proxy.serve(match_proxy_clone, &mut kill_rx, vpn_node_infos)
        });
        *proxy_state.http_proxy_sx.lock().await = Some(kill_tx);
    }

    #[cfg(feature = "xray")]
    {
        let xray_bin_path = relative_command_path("xray".as_ref())?;
        let hysteria_record = hysteria_entity::Model::first(&db).await?.unwrap();
        let command_xray = CommandHysteria::try_from(&hysteria_record)?;
        let resource_dir = app.app_handle().path().resource_dir()?;
        let mut env_var = HashMap::new();
        env_var.insert(
            "XRAY_LOCATION_ASSET".to_string(),
            resource_dir.to_string_lossy().to_string(),
        );
        // let mut xray_manager = XrayManager::new(xray_bin_path, env_var);
        // xray_manager.start_backend(command_xray, config_dir)?;
        // *state.xray_process_manager.lock().await = Some(xray_manager);
    }

    Ok(KittyResponse::from_data(false))
}
