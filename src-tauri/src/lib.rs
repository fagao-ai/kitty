mod database;
mod process_manager;
mod state;
mod types;
mod utils;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, env, io::Write, path::PathBuf};

use crate::process_manager::ProcessManager;
use entity::{
    base_config,
    hysteria::{self},
};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{ClickType, TrayIconBuilder},
    Icon,
};

use database::{add_base_config, add_hysteria_item, get_all_hysteria_item, get_base_config};

use state::AppState;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_shell::ShellExt;
use tempfile::Builder;
use types::{CommandResult, KittyResponse, ResponseItem};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn stop_hysteria(app_handle: AppHandle) {
    println!("stop_hy called!!!");
}

#[tauri::command(rename_all = "snake_case")]
async fn add_hy_item<'a>(
    state: State<'a, AppState>,
    hysteria_config: hysteria::Model,
) -> CommandResult<()> {
    let db = state.get_db();
    add_hysteria_item(&db, hysteria_config).await?;
    Ok(())
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
    let mut hashmap = get_hashmap_from_struct(hysteria_config);
    let base_config_hashmap = match base_config {
        Some(config) => get_hashmap_from_struct(config),
        None => HashMap::new(),
    };
    hashmap.extend(base_config_hashmap);
    hashmap
}

fn get_hysteria_tmp_config_path(
    app_tmp_dir: &PathBuf,
    hyteria_config: &hysteria::Model,
    base_config: Option<&base_config::Model>,
) -> Result<String> {
    let temp_dir = Builder::new()
        .prefix("hysteria_")
        .tempdir_in(app_tmp_dir)
        .expect("Failed to create temporary directory");
    let temp_json_file = temp_dir.path().join("config.json");
    let mut file = std::fs::File::create(&temp_json_file).expect("Failed to create temporary file");
    let config_hashmap = merge_hysteria_config(hyteria_config, base_config);
    let config_str = serde_json::to_string(&config_hashmap)?;
    let config_bytes = config_str.as_bytes();
    file.write_all(config_bytes)?;
    let os_string = temp_json_file.into_os_string();
    let temp_json_file_string = os_string
        .into_string()
        .expect("Failed to convert to String");
    Ok(temp_json_file_string)
}

#[tauri::command]
async fn start_hysteria<'a>(
    app_handle: AppHandle,
    // app: &'a mut tauri::App,
    state: State<'a, AppState>,
) -> CommandResult<KittyResponse<hysteria::Model>> {
    let commmand = app_handle
        .shell()
        .sidecar("hysteria")
        .expect("failed to create `hysteria` binary command ");
    let db = state.get_db();
    let items = get_all_hysteria_item(&db).await?;
    let base_config = get_base_config(&db).await?.unwrap();
    let config_path = if items.len() > 0 {
        let app_tmp_dir = app_handle.path().temp_dir()?;
        let config_path: String =
            get_hysteria_tmp_config_path(&app_tmp_dir, &(items[0]), Some(&base_config))?;
        Some(config_path)
    } else {
        None
    };
    let response: KittyResponse<_> = match config_path {
        Some(file) => {
            let (_receiver, child) = commmand.arg("client").arg(file).spawn()?;
            let mut process_manager = state.process_manager.lock().unwrap();
            process_manager.add_child("hysteria", child);
            KittyResponse::default()
        }
        None => KittyResponse::<hysteria::Model>::from_msg(0, "hysteria config is empty."),
    };

    Ok(response)
}

#[tauri::command]
async fn incre_base_config<'a>(
    state: State<'a, AppState>,
    record: base_config::Model,
) -> CommandResult<KittyResponse<base_config::Model>> {
    let db = state.get_db();
    let added_record = add_base_config(&db, record).await?;
    let response =
        KittyResponse::<base_config::Model>::new(0, ResponseItem::Single(added_record), "success");
    Ok(response)
}

#[tauri::command]
async fn query_base_config<'a>(
    state: State<'a, AppState>,
) -> CommandResult<KittyResponse<base_config::Model>> {
    let db = state.get_db();
    let record = get_base_config(&db).await?;
    let response = match record {
        Some(record) => {
            KittyResponse::<base_config::Model>::new(0, ResponseItem::Single(record), "success")
        }
        None => KittyResponse::from_msg(101, "base_config not exists"),
    };
    Ok(response)
}

fn set_system_tray<'a>(app: &'a mut tauri::App) -> Result<()> {
    let toggle = MenuItemBuilder::with_id("toggle", "Toggle").build(app);
    let menu = MenuBuilder::new(app).items(&[&toggle]).build()?;
    let parent_dir = env::current_dir()?.parent().unwrap().to_owned();
    let icon_path = parent_dir.join("icons").join("32x32.png");
    let icon = Icon::File(icon_path);
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(icon)
        .on_menu_event(move |_app, event| match event.id().as_ref() {
            "toggle" => {
                println!("toggle clicked");
            }
            _ => (),
        })
        .on_tray_icon_event(|tray, event| {
            if event.click_type == ClickType::Left {
                let app = tray.app_handle();
                if let Some(window) = app.get_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;
    Ok(())
}

fn setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    let app_dir = handle
        .path()
        .app_local_data_dir()
        .expect("The app data directory should exist.");
    let app_state: State<AppState> = handle.state();
    let db = tauri::async_runtime::block_on(async move {
        let db = database::init_db(app_dir).await;
        match db {
            Ok(db) => db,
            Err(err) => {
                panic!("Error: {}", err);
            }
        }
    });
    *app_state.db.lock().unwrap() = Some(db);
    let _ = set_system_tray(app);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
            process_manager: std::sync::Mutex::new(ProcessManager::new()),
        })
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            stop_hysteria,
            start_hysteria,
            add_hy_item,
            incre_base_config,
            query_base_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
