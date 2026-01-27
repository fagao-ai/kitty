use crate::apis::common_apis::CommonAPI;
use crate::proxy::delay::kitty_current_proxy_delay;
use crate::state::{DatabaseState, LogLevelState};
use crate::types::{CommandResult, KittyResponse};
use entity::base_config;
use entity::rules;
use sea_orm::DatabaseConnection;
use tauri::State;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tracing_subscriber::EnvFilter;

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
    let res = kitty_current_proxy_delay(proxy, target_url).await;
    Ok(KittyResponse::from_data(res))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_log_level<'a>(
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<String>> {
    let db = state.get_db();
    let record = base_config::Model::first(&db).await?
        .ok_or_else(|| anyhow::anyhow!("base_config not exists"))?;
    Ok(KittyResponse::from_data(record.log_level))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_log_level<'a>(
    state_db: State<'a, DatabaseState>,
    state_log: State<'a, LogLevelState>,
    log_level: String,
) -> CommandResult<KittyResponse<()>> {
    // Validate log level
    let valid_levels = vec!["trace", "debug", "info", "warn", "error"];
    if !valid_levels.contains(&log_level.as_str()) {
        return Err(anyhow::anyhow!("Invalid log level: {}", log_level).into());
    }

    let db = state_db.get_db();
    let mut record = base_config::Model::first(&db).await?
        .ok_or_else(|| anyhow::anyhow!("base_config not exists"))?;

    // Update database
    record.log_level = log_level.clone();
    record.update(&db).await?;

    // Update runtime log level - shoes follows the same log level
    let handle = state_log.filter_handle.lock().await;
    if let Some(filter_handle) = handle.as_ref() {
        let new_filter = format!("{},shoes={}", log_level, log_level);
        let _ = filter_handle.modify(|filter| {
            *filter = EnvFilter::new(new_filter);
        });
    }

    Ok(KittyResponse::default())
}
