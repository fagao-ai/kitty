use crate::apis::common_apis::CommonAPI;
use crate::state::DatabaseState;
use crate::types::{CommandResult, KittyResponse};
use entity::base_config;
use entity::rules;
use log::Record;
use sea_orm::DatabaseConnection;
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
    let clipboard_content = tauri_plugin_clipboard_manager::ClipKind::PlainText {
        label: Some("Label".to_string()),
        text: proxy_string,
    };
    app_handle.clipboard().write(clipboard_content).unwrap();
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
    auto_start_state: State<'a, AutoLaunchManager>,
    record: base_config::Model,
) -> CommandResult<KittyResponse<base_config::Model>> {
    let db = state.get_db();
    if let Ok(is_enable) = auto_start_state.is_enabled() {
        if record.auto_start {
            if !is_enable {
                let _ = auto_start_state.enable();
            }
        } else {
            if is_enable {
                let _ = auto_start_state.disable();
            }
        }
    }
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
pub async fn update_rules_item<'a>(
    state: State<'a, DatabaseState>,
    record: rules::Model,
) -> CommandResult<KittyResponse<rules::Model>> {
    let db = state.get_db();
    let res = CommonAPI::update_rules(&db, record).await?;
    Ok(res)
}
