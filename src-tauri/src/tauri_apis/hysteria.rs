use entity::{
    base_config,
    hysteria::{self},
};
use tauri::State;

use crate::apis::api_traits::APIServiceTrait;
use crate::apis::hysteria_apis::HysteriaAPI;
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
pub async fn add_hy_item<'a>(
    state: State<'a, DatabaseState>,
    record: hysteria::Model,
) -> CommandResult<()> {
    let db = state.get_db();
    HysteriaAPI.add_hysteria_item(&db, record).await?;
    Ok(())
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
pub async fn query_base_config<'a>(
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<base_config::Model>> {
    let db = state.get_db();
    HysteriaAPI::query_base_config(&db).await
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_base_config<'a>(
    state: State<'a, DatabaseState>,
    id: i32,
    record: base_config::Model,
) -> CommandResult<KittyResponse<base_config::Model>> {
    let db = state.get_db();
    HysteriaAPI::update_base_config(&db, id, record).await
}
