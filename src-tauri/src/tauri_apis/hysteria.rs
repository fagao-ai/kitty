use entity::hysteria::{self};
use tauri::State;

use crate::apis::hysteria_apis::HysteriaAPI;
use crate::state::DatabaseState;
use crate::types::{CommandResult, KittyResponse};

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
