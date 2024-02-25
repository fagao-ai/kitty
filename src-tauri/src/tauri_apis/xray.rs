use std::collections::HashMap;

use entity::xray::{self, XrayConfig};
use entity::{base_config, subscribe};
use protocols::XrayCommandGroup;
use tauri::{AppHandle, Manager, State};

use crate::apis::xray_apis::XrayAPI;
use crate::state::{DatabaseState, KittyProxyState};
use crate::types::{CommandResult, KittyResponse};

use protocols::KittyCommandGroupTrait;

use super::utils::{relative_command_path, speed_delay};

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
    proxy_state: State<'a, KittyProxyState>,
    record_ids: Option<Vec<i32>>,
) -> CommandResult<HashMap<i32, u128>> {
    let db = state.get_db();
    let xray_records: Vec<xray::Model> = if record_ids.is_none() {
        xray::Model::fetch_all(&db).await?
    } else {
        xray::Model::fetch_by_ids(&db, record_ids.unwrap()).await?
    };
    let base_config_record = base_config::Model::first(&db).await.unwrap();
    let delay_test_url = base_config_record.unwrap().delay_test_url;
    drop(db);
    let config_dir = app_handle.path().config_dir()?;
    let mut used_ports = proxy_state.used_ports.lock().await;
    let hysteria_bin_path = relative_command_path("xray".as_ref())?;
    let mut hysteria_command_group = XrayCommandGroup::new(hysteria_bin_path, config_dir.clone());
    let mut config_hash_map: HashMap<String, XrayConfig> = HashMap::new();

    let server_key: String = xray_records
        .iter()
        .map(|x| x.get_server())
        .collect::<Vec<String>>()
        .join("_");
    let (xray_config, port_model_dict) =
        XrayConfig::from_models4http_delay(xray_records, &mut used_ports);
    drop(used_ports);
    config_hash_map.insert(server_key, xray_config);
    let _ = hysteria_command_group.start_commands(config_hash_map, None);
    let ports: Vec<u16> = port_model_dict.keys().map(|x| x.to_owned()).collect();
    let total = ports.len();
    let result: HashMap<u16, std::time::Duration> =
        speed_delay(ports, Some(delay_test_url.as_str())).await?;
    let mut new_result: HashMap<i32, u128> = HashMap::with_capacity(total);
    for (k, v) in result.iter() {
        new_result.insert(port_model_dict.get(k).unwrap().to_owned(), v.as_millis());
    }

    Ok(new_result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn refresh_xray_subscription<'a>(
    state: State<'a, DatabaseState>,
    record_ids: Option<Vec<i32>>,
) -> CommandResult<()> {
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
