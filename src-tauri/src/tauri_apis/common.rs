use crate::apis::common_apis::CommonAPI;
use crate::state::{DatabaseState, KittyProxyState};
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

use super::utils::{add_rule2match_proxy, delete_rule2match_proxy};

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
    proxy_state: State<'a, KittyProxyState>,
    records: Vec<rules::Model>,
) -> CommandResult<KittyResponse<()>> {
    let db = state.get_db();
    let match_proxy = proxy_state.match_proxy.lock().await.clone().unwrap();
    let mut match_proxy_write_share = match_proxy.write().await;
    for rule_record in records.iter() {
        add_rule2match_proxy(&mut match_proxy_write_share, rule_record).await;
    }
    drop(match_proxy_write_share);
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
    proxy_state: State<'a, KittyProxyState>,
    ids: Vec<i32>,
) -> CommandResult<KittyResponse<()>> {
    let db = state.get_db();
    let delete_rules = rules::Model::fetch_by_ids(&db, ids.clone()).await?;
    let txn = db.begin().await?;
    let match_proxy = proxy_state.match_proxy.lock().await.clone().unwrap();
    let mut match_proxy_write_share = match_proxy.write().await;
    let _ = delete_rule2match_proxy(&txn, &mut match_proxy_write_share, delete_rules);
    drop(match_proxy_write_share);
    let _ = CommonAPI::delete_rules(&txn, ids).await?;
    txn.commit().await?;
    Ok(KittyResponse::default())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_rules_item<'a>(
    state: State<'a, DatabaseState>,
    proxy_state: State<'a, KittyProxyState>,
    records: Vec<rules::Model>,
) -> CommandResult<KittyResponse<()>> {
    let db = state.get_db();
    let delete_record_ids: Vec<i32> = records.iter().map(|x| x.id).collect();
    let origin_records = rules::Model::fetch_by_ids(&db, delete_record_ids.clone()).await?;
    if !origin_records.is_empty() {
        let txn = db.begin().await?;
        let match_proxy = proxy_state.match_proxy.lock().await.clone().unwrap();
        let mut match_proxy_write_share = match_proxy.write().await;
        let _ = delete_rule2match_proxy(&txn, &mut match_proxy_write_share, origin_records);
        for rule_record in records.iter() {
            add_rule2match_proxy(&mut match_proxy_write_share, rule_record).await;
        }
        drop(match_proxy_write_share);
        let _ = CommonAPI::delete_rules(&txn, delete_record_ids).await?;
        let res = CommonAPI::add_rules(&txn, records).await?;
        txn.commit().await?;
        Ok(res)
    } else {
        Err(anyhow!("records not exists!").into())
    }
}
