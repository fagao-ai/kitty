use entity::xray::{self};
use tauri::State;

use crate::apis::xray_apis::XrayAPI;
use crate::apis::xray_apis::XrayRecord;
use crate::state::DatabaseState;
use crate::types::{CommandResult, KittyResponse};

#[tauri::command(rename_all = "snake_case")]
pub async fn add_xray_item<'a>(
    state: State<'a, DatabaseState>,
    record: xray::Model,
) -> CommandResult<()> {
    let db = state.get_db();
    println!("{:?}", record);
    XrayAPI.add_xray_item(&db, record).await?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_xrays<'a>(
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<Vec<XrayRecord>>> {
    let db = state.get_db();
    let xraies = XrayAPI.get_all(&db).await?;
    Ok(KittyResponse::from_data(xraies))
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

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_xray_item<'a>(state: State<'a, DatabaseState>, id: i32) -> CommandResult<()> {
    let db = state.get_db();
    XrayAPI.delete_xray_item(&db, id).await?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_xray_item<'a>(
    state: State<'a, DatabaseState>,
    record: xray::Model,
) -> CommandResult<()> {
    let db = state.get_db();
    XrayAPI.update_xray_item(&db, record).await?;
    Ok(())
}
