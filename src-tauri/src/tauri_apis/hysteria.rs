use std::collections::HashMap;

use entity::base_config;
use entity::hysteria::{self, HysteriaConfig};
use tauri::{AppHandle, Manager, State};

use crate::apis::hysteria_apis::HysteriaAPI;
use crate::state::DatabaseState;
use crate::types::{CommandResult, KittyResponse};

use super::utils::{get_http_socks_ports, speed_delay};

#[tauri::command(rename_all = "snake_case")]
pub async fn add_hysteria_item<'a>(
    state: State<'a, DatabaseState>,
    record: hysteria::Model,
) -> CommandResult<()> {
    let db = state.get_db();
    HysteriaAPI.add_hysteria_item(&db, record).await?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_hysteria_by_id<'a>(
    state: State<'a, DatabaseState>,
    id: i32,
) -> CommandResult<KittyResponse<Option<hysteria::Model>>> {
    let db = state.get_db();
    let hysteria = HysteriaAPI.get_hysteria_by_id(&db, id).await?;
    Ok(KittyResponse::from_data(hysteria))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_hysterias<'a>(
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<Vec<hysteria::Model>>> {
    let db = state.get_db();
    let hy_proxies = HysteriaAPI.get_all(&db).await?;
    Ok(KittyResponse::from_data(hy_proxies))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_hysteria_item<'a>(
    state: State<'a, DatabaseState>,
    id: i32,
) -> CommandResult<()> {
    let db = state.get_db();
    HysteriaAPI.delete_hysteria_item(&db, id).await?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_hysteria_item<'a>(
    state: State<'a, DatabaseState>,
    record: hysteria::Model,
) -> CommandResult<()> {
    let db = state.get_db();
    HysteriaAPI.update_hysteria_item(&db, record).await?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn speed_hysteria_delay<'a>(
    app_handle: AppHandle,
    state: State<'a, DatabaseState>,
    record_ids: Option<Vec<i32>>,
) -> CommandResult<HashMap<i32, u128>> {
    // TODO: Implement delay testing using shoes library
    // For now, this is a stub that returns an error
    // The implementation would:
    // 1. Convert each hysteria record to shoes YAML config
    // 2. Start individual shoes servers for testing
    // 3. Run delay tests against each server
    // 4. Return results and clean up servers

    // Old implementation used XrayCommandGroup with binaries
    // New implementation should use shoes::tcp::tcp_server::start_servers

    Err(anyhow::anyhow!("Delay testing not yet implemented for shoes library. Please use manual testing.").into())
}
