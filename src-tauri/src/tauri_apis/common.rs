use crate::apis::common_apis::CommonAPI;
use crate::proxy::delay::kitty_current_proxy_delay;
use crate::state::DatabaseState;
use crate::types::{CommandResult, KittyResponse};
use anyhow::anyhow;
use entity::base_config;
use entity::rules;
use sea_orm::{DatabaseConnection, TransactionTrait};
use tauri::State;
use tauri_plugin_autostart::AutoLaunchManager;
use tauri_plugin_clipboard_manager::ClipboardExt;

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
use tauri::AppHandle;
use tauri::Runtime;

pub async fn copy_proxy_env<R: Runtime>(
    app_handle: &AppHandle<R>,
    db: &DatabaseConnection,
) -> CommandResult<KittyResponse<String>> {
    let proxy_string = CommonAPI::copy_proxy_env(db).await?;
    app_handle.clipboard().write_text(proxy_string).unwrap();
    Ok(KittyResponse::default())
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
#[tauri::command(rename_all = "snake_case")]
pub async fn copy_proxy_env_cmd<'a, R: Runtime>(
    app_handle: AppHandle<R>,
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<String>> {
    let db = state.get_db();
    Ok(copy_proxy_env(&app_handle, &db).await?)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn query_base_config<'a>(
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<base_config::Model>> {
    let db = state.get_db();
    let res = CommonAPI::query_base_config(&db).await?;
    Ok(res)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_base_config<'a>(
    state: State<'a, DatabaseState>,
    record: base_config::Model,
) -> CommandResult<KittyResponse<base_config::Model>> {
    let db = state.get_db();
    let res = CommonAPI::update_base_config(&db, record).await?;
    Ok(res)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn add_rules<'a>(
    state: State<'a, DatabaseState>,
    records: Vec<rules::Model>,
) -> CommandResult<KittyResponse<()>> {
    let db = state.get_db();
    let _ = CommonAPI::add_rules(&db, records).await?;
    Ok(KittyResponse::default())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn query_rules<'a>(
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<Vec<rules::Model>>> {
    let db = state.get_db();
    let res = CommonAPI::query_rules(&db).await?;
    Ok(res)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_rules<'a>(
    state: State<'a, DatabaseState>,
    ids: Vec<i32>,
) -> CommandResult<KittyResponse<()>> {
    let db = state.get_db();
    let _ = CommonAPI::delete_rules(&db, ids).await?;
    Ok(KittyResponse::default())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_rules_item<'a>(
    state: State<'a, DatabaseState>,
    records: Vec<rules::Model>,
) -> CommandResult<KittyResponse<()>> {
    let db = state.get_db();
    let delete_record_ids: Vec<i32> = records.iter().map(|x| x.id).collect();
    let _ = CommonAPI::delete_rules(&db, delete_record_ids).await?;
    let res = CommonAPI::add_rules(&db, records).await?;
    Ok(res)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn test_current_proxy<'a>(
    proxy: String,
    target_url: String,
) -> CommandResult<KittyResponse<u128>> {
    println!("proxy: {}", proxy);
    let res = kitty_current_proxy_delay(proxy, target_url).await;
    Ok(KittyResponse::from_data(res))
}
