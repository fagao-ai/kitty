use std::collections::HashMap;

use entity::base_config;
use entity::hysteria::{self, HysteriaConfig};
use protocols::KittyCommandGroupTrait;
use protocols::XrayCommandGroup;
use tauri::{AppHandle, Manager, State};

use crate::apis::hysteria_apis::HysteriaAPI;
use crate::state::{DatabaseState, KittyProxyState};
use crate::types::{CommandResult, KittyResponse};

use super::utils::{get_http_socks_ports, relative_command_path, speed_delay};

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
pub async fn get_hysteria_by_id<'a>(
    state: State<'a, DatabaseState>,
    id: i32,
) -> CommandResult<KittyResponse<Option<hysteria::Model>>> {
    let db = state.get_db();
    let hysteria = HysteriaAPI.get_hysteria_by_id(&db, id).await?;
    Ok(KittyResponse::from_data(hysteria))
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

#[tauri::command(rename_all = "snake_case")]
pub async fn speed_hysteria_delay<'a>(
    app_handle: AppHandle,
    state: State<'a, DatabaseState>,
    proxy_state: State<'a, KittyProxyState>,
    record_ids: Option<Vec<i32>>,
) -> CommandResult<HashMap<i32, u128>> {
    let db = state.get_db();
    let hysteria_records = if record_ids.is_none() {
        hysteria::Model::fetch_all(&db).await?
    } else {
        hysteria::Model::fetch_by_ids(&db, record_ids.unwrap()).await?
    };
    let base_config_record = base_config::Model::first(&db).await.unwrap();
    let delay_test_url = base_config_record.unwrap().delay_test_url;
    drop(db);
    let config_dir = app_handle.path().config_dir()?;
    let hysteria_bin_path = relative_command_path("hysteria".as_ref())?;
    let mut hysteria_command_group = XrayCommandGroup::new(hysteria_bin_path, config_dir.clone());
    let mut config_hash_map: HashMap<String, HysteriaConfig> = HashMap::new();

    let mut port_model_dict = HashMap::new();
    let mut used_ports = proxy_state.used_ports.lock().await;
    for record in hysteria_records.into_iter() {
        let (http_port, socks_port) = get_http_socks_ports(&mut used_ports);
        let record_id = record.id;
        let hysteria_config = HysteriaConfig::new(http_port, socks_port, record);
        config_hash_map.insert(hysteria_config.server.clone(), hysteria_config);
        port_model_dict.insert(http_port, record_id);
    }
    drop(used_ports);
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
