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

use super::utils::{get_http_socks_ports, relative_command_path};

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


pub async fn speed_node_delay(db: &DatabaseConnection, used_ports: &mut HashSet<u16>, config_dir: PathBuf) -> Result<()>{
    let hysteria_records = hysteria::Model::fetch_all(&db).await?;
        let hysteria_bin_path = relative_command_path("hysteria".as_ref())?;
        let mut hysteria_command_group = KittyCommandGroup::new(
            String::from("hysteria"),
            hysteria_bin_path,
            config_dir.clone(),
        );
        let mut config_hash_map: HashMap<String, HysteriaConfig> = HashMap::new();
        for record in hysteria_records.into_iter() {
            let (http_port, socks_port) = get_http_socks_ports(used_ports);
            let hysteria_config = HysteriaConfig::new(http_port, socks_port, record);
            config_hash_map.insert(hysteria_config.server.clone(), hysteria_config);
        }
        let _ = hysteria_command_group.start_commands(config_hash_map, None);
        
        Ok(())

}