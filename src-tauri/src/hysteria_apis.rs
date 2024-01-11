mod database;
mod proxy;
mod state;
mod types;
mod utils;

use crate::proxy::system_proxy::clear_system_proxy;
use anyhow::Result;
use entity::{
    base_config,
    hysteria::HysteriaModelWithoutName,
    hysteria::{self},
};
use kitty_proxy::{HttpProxy, MatchProxy, SocksProxy};
use proxy::delay::{kitty_proxies_delay, ProxyDelay, ProxyInfo};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{borrow::BorrowMut, collections::HashMap, env, fs, io::Write, path::PathBuf};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{ClickType, TrayIconBuilder},
    Icon, WindowEvent,
};
use tokio::sync::Mutex;

use state::{DatabaseState, ProcessManagerState};
use tauri::{AppHandle, Manager, State};

use uuid::Uuid;

use crate::state::KittyProxyState;
use tauri_plugin_autostart::MacosLauncher;
use types::{CommandResult, KittyResponse};

use protocols::{CommandManagerTrait, HysteriaManager, XrayManager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn stop_hysteria<'a>(state: State<'a, ProcessManagerState>) -> CommandResult<()> {
    let mut process_manager = state.hy_process_manager.lock().await;
    let _kill_result = process_manager.terminate_backend()?;
    println!("stop_hy called!!!");
    let _ = clear_system_proxy();
    println!("clear_system_proxy called!!!");
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn get_hysteria_status<'a>(
    state: State<'a, ProcessManagerState>,
) -> CommandResult<KittyResponse<bool>> {
    let mut process_manager = state.hy_process_manager.lock().await;
    let res = process_manager.borrow_mut().is_open();
    Ok(KittyResponse::from_data(res))
}

#[tauri::command(rename_all = "snake_case")]
async fn add_hy_item<'a>(
    state: State<'a, DatabaseState>,
    record: hysteria::Model,
) -> CommandResult<()> {
    println!("{:?}", &record);
    let db = state.get_db();
    record.insert_one(&db).await?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn get_all_proxies<'a>(
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<Vec<hysteria::Model>>> {
    println!("called get_all_proxies");
    let db = state.get_db();
    let hy_proxies = hysteria::Model::fetch_all(&db).await?;
    println!("hy_proxies: {:?}", hy_proxies);
    Ok(KittyResponse::from_data(hy_proxies))
}

fn get_hashmap_from_struct<'a, T>(input_struct: &T) -> HashMap<String, Value>
where
    T: Deserialize<'a> + Serialize,
{
    let main_config_json_string = serde_json::to_string(&input_struct).unwrap();
    let json_value: Value = serde_json::from_str(&main_config_json_string).unwrap();
    let mapping = json_value.as_object().unwrap().to_owned();
    let hashmap: HashMap<String, Value> = mapping
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    hashmap
}

fn merge_hysteria_config(
    hysteria_config: &hysteria::Model,
    base_config: Option<&base_config::Model>,
) -> HashMap<String, Value> {
    let hysteria_config = HysteriaModelWithoutName::from(hysteria_config);
    let mut hashmap = get_hashmap_from_struct(&hysteria_config);
    let base_config_hashmap = match base_config {
        Some(config) => {
            let base_config = get_hashmap_from_struct(config);
            let mut new_base_config = HashMap::new();
            for (k, v) in base_config.into_iter() {
                if k.to_string().eq("http_port") | k.to_string().eq("socks_port") {
                    let mut tmp_hash_map = HashMap::new();
                    tmp_hash_map.insert("listen", format!("127.0.0.1:{}", v));
                    let json_value: Value =
                        serde_json::to_value(tmp_hash_map).expect("Failed to convert to JSON");
                    let new_k = if k.eq("http_port") { "http" } else { "socks5" };
                    new_base_config.insert(new_k.to_string(), json_value);
                } else {
                    new_base_config.insert(k, v);
                }
            }
            new_base_config
        }
        None => HashMap::new(),
    };
    hashmap.extend(base_config_hashmap);
    hashmap
}

fn get_hysteria_tmp_config_path(
    app_tmp_dir: &PathBuf,
    hysteria_config: &hysteria::Model,
    base_config: Option<&base_config::Model>,
) -> Result<String> {
    let uuid = Uuid::new_v4();
    let uuid_string: String = uuid.to_string();

    let temp_json_file = app_tmp_dir.join(format!("{}/hysteria_config.json", uuid_string));
    fs::create_dir_all(temp_json_file.parent().unwrap())?;
    println!("temp_json_file {:?}", temp_json_file);
    let mut file = std::fs::File::create(&temp_json_file).expect("Failed to create temporary file");
    let config_hashmap = merge_hysteria_config(hysteria_config, base_config);
    let config_str = serde_json::to_string(&config_hashmap)?;
    let config_bytes = config_str.as_bytes();
    file.write_all(config_bytes)?;
    let os_string = temp_json_file.into_os_string();
    let temp_json_file = os_string
        .into_string()
        .expect("Failed temp_json_file convert to String");
    Ok(temp_json_file)
}

#[tauri::command]
async fn start_hysteria<'a>(
    app_handle: AppHandle,
    // app: &'a mut tauri::App,
    db_state: State<'a, DatabaseState>,
    state: State<'a, ProcessManagerState>,
) -> CommandResult<KittyResponse<Option<hysteria::Model>>> {
    println!("start_hysteria!!!");
    let db = db_state.get_db();
    let items = hysteria::Model::fetch_all(&db).await?;
    let base_config = base_config::Model::first(&db).await?;
    let base_config = base_config.unwrap();
    let config_path = if items.len() > 0 {
        let app_cache_dir = app_handle.path().app_cache_dir()?;
        if !app_cache_dir.exists() {
            fs::create_dir_all(&app_cache_dir).unwrap();
        }
        println!("app_tmp_dir: {:?}", app_cache_dir);
        let config_path: String =
            get_hysteria_tmp_config_path(&app_cache_dir, &(items[0]), Some(&base_config))?;
        Some(config_path)
    } else {
        None
    };
    println!("config_path: {:?}", &config_path);
    let mut process_manager = state.hy_process_manager.lock().await;
    let response = match config_path {
        Some(file) => {
            let _ = process_manager.start_backend(app_handle, "")?;
            let _ = process_manager.check_status()?;
            let _ = fs::remove_file(file)?;
            KittyResponse::from_data(None)
        }
        None => KittyResponse::from_msg(100, "hysteria config is empty, please ad"),
    };

    Ok(response)
}

#[tauri::command]
async fn query_base_config<'a>(
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<base_config::Model>> {
    let db = state.get_db();
    let record = base_config::Model::first(&db).await?;
    let response = match record {
        Some(record) => KittyResponse::<base_config::Model>::from_data(record),
        None => KittyResponse::from_msg(101, "base_config not exists"),
    };
    Ok(response)
}

#[tauri::command(rename_all = "snake_case")]
async fn update_base_config<'a>(
    state: State<'a, DatabaseState>,
    id: i32,
    record: base_config::Model,
) -> CommandResult<KittyResponse<base_config::Model>> {
    let db = state.get_db();
    let updated_record = record.update(&db, id).await?;
    Ok(KittyResponse::<base_config::Model>::from_data(
        updated_record,
    ))
}
