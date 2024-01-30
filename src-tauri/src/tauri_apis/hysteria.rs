use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use entity::hysteria::{self, HysteriaConfig};
use protocols::KittyCommandGroup;
use sea_orm::DatabaseConnection;
use tauri::State;

use crate::apis::hysteria_apis::HysteriaAPI;
use crate::state::DatabaseState;
use crate::types::{CommandResult, KittyResponse};
use anyhow::Result;

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
pub async fn speed_hysteria_delay(
    db: &DatabaseConnection,
    used_ports: &mut HashSet<u16>,
    config_dir: PathBuf,
) -> Result<HashMap<i32, u128>> {
    let hysteria_records = hysteria::Model::fetch_all(&db).await?;
    let hysteria_bin_path = relative_command_path("hysteria".as_ref())?;
    let mut hysteria_command_group = KittyCommandGroup::new(
        String::from("hysteria"),
        hysteria_bin_path,
        config_dir.clone(),
    );
    let mut config_hash_map: HashMap<String, HysteriaConfig> = HashMap::new();

    let mut port_model_dict = HashMap::new();
    for record in hysteria_records.into_iter() {
        let (http_port, socks_port) = get_http_socks_ports(used_ports);
        let record_id = record.id;
        let hysteria_config = HysteriaConfig::new(http_port, socks_port, record);
        config_hash_map.insert(hysteria_config.server.clone(), hysteria_config);
        port_model_dict.insert(http_port, record_id);
    }
    let _ = hysteria_command_group.start_commands(config_hash_map, None);
    let ports: Vec<u16> = port_model_dict.keys().map(|x| x.to_owned()).collect();
    let total = ports.len();
    let result: HashMap<u16, std::time::Duration> = speed_delay(ports, None).await?;
    let mut new_result: HashMap<i32, u128> = HashMap::with_capacity(total);
    for (k, v) in result.iter() {
        new_result.insert(port_model_dict.get(k).unwrap().to_owned(), v.as_millis());
    }

    Ok(new_result)
}
