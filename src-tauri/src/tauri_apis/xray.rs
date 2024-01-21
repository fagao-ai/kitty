use entity::{
    base_config,
    xray::{self},
};
use tauri::State;

use crate::apis::api_traits::APIServiceTrait;
use crate::apis::xray_apis;
use crate::apis::xray_apis::XrayAPI;
use crate::state::{DatabaseState, ProcessManagerState};
use crate::types::{CommandResult, KittyResponse};

// #[tauri::command(rename_all = "snake_case")]
// pub async fn get_hysteria_status<'a>(
//     state: State<'a, ProcessManagerState>,
// ) -> CommandResult<KittyResponse<bool>> {
//     let process_manager = state.hy_process_manager.lock().await;
//     let process_manager = process_manager.as_ref();
//     if let Some(process_manager) = process_manager {
//         let is_running = process_manager.is_running();
//         Ok(KittyResponse::from_data(is_running))
//     } else {
//         Ok(KittyResponse::from_data(false))
//     }
// }

#[tauri::command(rename_all = "snake_case")]
pub async fn add_xray_item<'a>(
    state: State<'a, DatabaseState>,
    record: xray::Model,
) -> CommandResult<()> {
    let db = state.get_db();
    XrayAPI.add_xray_item(&db, record).await?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_xrays<'a>(
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<Vec<xray::Model>>> {
    let db = state.get_db();
    let hy_proxies = XrayAPI.get_all(&db).await?;
    Ok(KittyResponse::from_data(hy_proxies))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn import_by_subscribe_url<'a>(
    state: State<'a, DatabaseState>,
    url: &str,
) -> CommandResult<()> {
    let db = state.get_db();
    let _res = XrayAPI.import_xray_from_subscribe(&db, url).await?;
    Ok(())
}
