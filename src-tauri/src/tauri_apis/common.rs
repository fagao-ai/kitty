use crate::apis::common_apis::CommonAPI;
use crate::state::DatabaseState;
use crate::types::{CommandResult, KittyResponse};
use entity::base_config;
use tauri::State;

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
#[tauri::command(rename_all = "snake_case")]
pub async fn copy_proxy_env<'a>(
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<String>> {
    let db = state.get_db();
    let proxy_string = CommonAPI::copy_proxy_env(&db).await?;
    Ok(KittyResponse::from_data(proxy_string))
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
    id: i32,
    record: base_config::Model,
) -> CommandResult<KittyResponse<base_config::Model>> {
    let db = state.get_db();
    let res = CommonAPI::update_base_config(&db, id, record).await?;
    Ok(res)
}
