use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use protocols::{CommandManagerTrait, HysteriaManager, XrayManager};
use tauri::{Manager, State};

use crate::{
    state::{DatabaseState, KittyProxyState, ProcessManagerState},
    types::{CommandResult, KittyResponse},
};
use anyhow::{anyhow, Result};
use tauri::utils::platform;

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
    {
        let hysteria_record = hysteria_entity::Model::first(&db).await?.unwrap();
        let command_hysteria = CommandHysteria::try_from(&hysteria_record)?;
        let hysteria_bin_path = relative_command_path("hysteria".as_ref())?;
        let mut hy_manager = HysteriaManager::new(hysteria_bin_path);
        hy_manager.start_backend(command_hysteria, config_dir.clone())?;
        *state.hy_process_manager.lock().await = Some(hy_manager);
        // let http_port = command_hysteria.get_http_port();
        // let socks_port = command_hysteria.get_socks_port();
        // let http_proxy = proxy_state.http_proxy.lock().await.unwrap();
        // tokio::spawn(future)
        // http_proxy.serve(match_proxy, rx, vpn_node_infos)
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
        let mut xray_manager = XrayManager::new(xray_bin_path, env_var);
        xray_manager.start_backend(command_xray, config_dir)?;
        *state.xray_process_manager.lock().await = Some(xray_manager);
    }

    Ok(KittyResponse::from_data(false))
}
