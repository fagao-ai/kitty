mod database;
mod process_manager;
mod state;
mod types;
mod utils;

use std::{env, ffi::OsStr, fs, io::Write, os::fd::AsFd, path::PathBuf, sync::atomic::AtomicBool};

use crate::process_manager::ProcessManager;
use entity::hysteria::{self, Model};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{ClickType, TrayIconBuilder},
    Icon,
};

use database::{add_hysteria_item, get_all_hysteria_item};

use state::AppState;
use tauri::{AppHandle, Manager, State, StateManager};
use tauri_plugin_shell::ShellExt;
use tempfile::Builder;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn start_hy(app_handle: AppHandle) {
    println!("start_hy called!!!");
}

#[tauri::command]
fn stop_hy(app_handle: AppHandle) {
    println!("stop_hy called!!!");
    println!("alread stop!!!")
}

fn get_hysteria_tmp_config_path(
    app_tmp_dir: &PathBuf,
    hyteria_config: &hysteria::Model,
) -> Result<String, Box<dyn std::error::Error>> {
    let temp_dir = Builder::new()
        .prefix("hysteria_")
        .tempdir_in(app_tmp_dir)
        .expect("Failed to create temporary directory");
    let temp_json_file = temp_dir.path().join("config.json");
    let mut file = std::fs::File::create(&temp_json_file).expect("Failed to create temporary file");
    let config_str = serde_json::to_string(&hyteria_config)?;
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
    app: &'a mut tauri::App,
    state: State<'_, AppState>,
) -> Result<(), Box<dyn std::error::Error>> {
    let commmand = app
        .shell()
        .sidecar("hysteria")
        .expect("failed to create `hysteria` binary command ");
    let conn = state.db.lock().unwrap();

    let db = conn.as_ref().unwrap();
    let items = get_all_hysteria_item(db).await?;
    let config_path = if items.len() > 0 {
        let app_tmp_dir = app.path().temp_dir()?;
        let aa = get_hysteria_tmp_config_path(&app_tmp_dir, &(items[0]))?;
        Some(aa)
    } else {
        None
    };
    match config_path {
        Some(file) => {
            let (receiver, child) = commmand.arg("client").arg(file).spawn()?;
            let process_manager = state.process_manager.lock().unwrap();
            process_manager.add_child("hysteria", child)
        }
        None => (),
    }

    // match config_path {
    //     Some()
    // }

    // let config_path = match items.get(0){
    //     Some(hysteria_config) => {

    //     }
    //     None => String::from("value")
    // };
    // commmand.arg("client").arg("/config.json");

    Ok(())
}

fn set_system_tray<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
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
    println!("{:?}", app_dir);
    let app_state: State<AppState> = handle.state();
    let db = tauri::async_runtime::block_on(async move {
        let db = database::init_db(app_dir).await;
        match db {
            Ok(db) => {
                println!("Local Server is running");
                db
            }
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
        .invoke_handler(tauri::generate_handler![start_hy, stop_hy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
