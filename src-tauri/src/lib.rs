mod database;
mod protocol;
mod proxy;
mod state;
mod types;
mod utils;
use futures::lock::Mutex;

use crate::protocol::hysteria::HysteriaManager;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, env, fs, io::Write, path::PathBuf, process::exit};

use crate::proxy::system_proxy::clear_system_proxy;
use entity::{
    base_config,
    hysteria::HysteriaModelWithoutName,
    hysteria::{self},
};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{ClickType, TrayIconBuilder},
    Icon, WindowEvent,
};

use state::{DatabaseState, ProcessManagerState};
use tauri::{AppHandle, Manager, State};

use uuid::Uuid;

use crate::protocol::traits::CommandManagerTrait;
use tauri_plugin_autostart::MacosLauncher;
use types::{CommandResult, KittyResponse};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn stop_hysteria<'a>(state: State<'a, ProcessManagerState>) -> CommandResult<()> {
    let mut process_manager = state.process_manager.lock().await;
    let _kill_result = process_manager.terminate_backend()?;
    println!("stop_hy called!!!");
    let _ = clear_system_proxy();
    println!("clear_system_proxy called!!!");
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn get_hysterie_status<'a>(
    state: State<'a, ProcessManagerState>,
) -> CommandResult<KittyResponse<bool>> {
    let process_manager = state.process_manager.lock().await;
    let res = process_manager.is_open();
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
    let hy_proxies = hysteria::Model::fectch_all(&db).await?;
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
    hyteria_config: &hysteria::Model,
    base_config: Option<&base_config::Model>,
) -> Result<String> {
    let uuid = Uuid::new_v4();
    let uuid_string: String = uuid.to_string();

    let temp_json_file = app_tmp_dir.join(format!("{}/hysteria_config.json", uuid_string));
    fs::create_dir_all(temp_json_file.parent().unwrap())?;
    println!("temp_json_file {:?}", temp_json_file);
    let mut file = std::fs::File::create(&temp_json_file).expect("Failed to create temporary file");
    let config_hashmap = merge_hysteria_config(hyteria_config, base_config);
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
    let items = hysteria::Model::fectch_all(&db).await?;
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
    let mut process_manager = state.process_manager.lock().await;
    let response = match config_path {
        Some(file) => {
            let args = vec!["client", "--config", file.as_str()];
            let _ = process_manager.start_backend(app_handle, args)?;
            let _ = process_manager.check_status().await?;
            let _ = fs::remove_file(file)?;
            KittyResponse::from_data(None)
        }
        None => KittyResponse::from_msg(100, "hysteria config is empty, please ad"),
    };

    Ok(response)
}

#[tauri::command]
async fn incre_base_config<'a>(
    state: State<'a, DatabaseState>,
    record: base_config::Model,
) -> CommandResult<KittyResponse<base_config::Model>> {
    let db = state.get_db();
    let added_record = record.insert_one(&db).await?;
    let response = KittyResponse::from_data(added_record);
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

fn set_system_tray<'a>(app: &'a mut tauri::App) -> Result<()> {
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app);
    let hide = MenuItemBuilder::with_id("hide", "Hide").build(app);
    let menu = MenuBuilder::new(app).items(&[&quit, &hide]).build()?;
    let current_path = env::current_dir()?;
    println!("current_path: {:?}", current_path);
    let parent_dir = current_path.to_owned();
    let icon_path = parent_dir.join("icons").join("icons8-48.png");
    println!("icon_path: {:?}", icon_path);
    let icon = Icon::File(icon_path);
    print!("set_system_tray");
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(icon)
        .on_menu_event(
            move |app, event: tauri::menu::MenuEvent| match event.id().as_ref() {
                "hide" => {
                    let window: tauri::Window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "quit" => {
                    app.exit(0);
                }

                _ => (),
            },
        )
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
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }
    println!("app_dir: {:?}", app_dir);
    let app_state: State<DatabaseState> = handle.state();
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

async fn on_window_exit(event: tauri::GlobalWindowEvent) {
    match event.event() {
        WindowEvent::Destroyed => {
            println!("exit!!!");
            let am: State<ProcessManagerState> = event.window().state();
            am.process_manager
                .lock()
                .await
                .terminate_backend()
                .expect("");
        }
        _ => {}
    }
}

fn on_window_exit_func(event: tauri::GlobalWindowEvent) {
    tauri::async_runtime::block_on(on_window_exit(event))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DatabaseState {
            db: Default::default(),
        })
        .manage(ProcessManagerState {
            process_manager: Mutex::new(HysteriaManager::new()),
        })
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .setup(setup)
        .on_window_event(on_window_exit_func)
        .invoke_handler(tauri::generate_handler![
            stop_hysteria,
            start_hysteria,
            add_hy_item,
            get_all_proxies,
            incre_base_config,
            query_base_config,
            update_base_config,
            get_hysterie_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
