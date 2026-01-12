use std::collections::HashMap;

use entity::xray::{self, XrayConfig};
use entity::{base_config, subscribe};
use tauri::{AppHandle, Manager, State};

use crate::apis::xray_apis::XrayAPI;
use crate::proxy::delay::{kitty_proxies_delay, ProxyDelay, ProxyInfo};
use crate::config_converter::ShoesConfigConverter;
use crate::state::DatabaseState;
use crate::types::{CommandResult, KittyResponse};

use super::utils::speed_delay;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_xray_by_id<'a>(
    state: State<'a, DatabaseState>,
    id: i32,
) -> CommandResult<KittyResponse<Option<xray::Model>>> {
    let db = state.get_db();
    Ok(KittyResponse::from_data(
        XrayAPI.get_xray_by_id(&db, id).await?,
    ))
}

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
    let xraies = XrayAPI.get_all(&db).await?;
    Ok(KittyResponse::from_data(xraies))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn import_xray_subscribe<'a>(
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

#[tauri::command(rename_all = "snake_case")]
pub async fn speed_xray_delay<'a>(
    app_handle: AppHandle,
    state: State<'a, DatabaseState>,
    record_ids: Option<Vec<i32>>,
) -> CommandResult<HashMap<i32, u128>> {
    // TODO: Implement delay testing using shoes library
    // For now, this is a stub that returns an error
    // The implementation would:
    // 1. Convert each xray record to shoes YAML config
    // 2. Start individual shoes servers for testing
    // 3. Run delay tests against each server
    // 4. Return results and clean up servers

    // Old implementation used XrayCommandGroup with binaries
    // New implementation should use shoes::tcp::tcp_server::start_servers

    Err(anyhow::anyhow!("Delay testing not yet implemented for shoes library. Please use manual testing.").into())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn refresh_xray_subscription<'a>(
    state: State<'a, DatabaseState>,
    record_ids: Option<Vec<i32>>,
) -> CommandResult<()> {
    println!("refresh_xray_subscription: {:?}", record_ids);
    let db = state.get_db();
    let _ = XrayAPI::refresh_subscribe(&db, record_ids).await?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn batch_get_subscriptions<'a>(
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<Vec<subscribe::Model>>> {
    let db = state.get_db();
    let subscriptions = XrayAPI::batch_get_subscriptions(&db).await?;
    Ok(KittyResponse::from_data(subscriptions))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn proxies_delay_test<'a>(
    proxies: Vec<ProxyInfo>,
) -> CommandResult<KittyResponse<Vec<ProxyDelay>>> {
    let res = kitty_proxies_delay(proxies).await;
    Ok(KittyResponse::from_data(res))
}
