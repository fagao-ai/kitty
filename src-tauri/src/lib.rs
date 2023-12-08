mod database;
mod state;
mod types;
mod utils;

use std::{env, sync::atomic::AtomicBool};

use entity::hysteria;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{ClickType, TrayIconBuilder},
    Icon,
};

use database::{add_hysteria_item, get_all_hysteria_item};

use state::{AppState, ServiceAccess};
use tauri::{AppHandle, Manager, State, StateManager};
use tauri_plugin_shell::ShellExt;

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

#[tauri::command]
async fn start_hysteria<'a>(app_handle: AppHandle, hyteria_config: hysteria::Model) -> Result<(), Box<dyn std::error::Error>> {
    let commmand = app_handle
        .shell()
        .sidecar("hysteria")
        .expect("failed to create `hysteria` binary command ");
    let items: Vec<Model> = app_handle.db(|db| {
        get_all_hysteria_item(db).await?;
    });
    let app_tmp_dir = app_handle.app_handle().path().temp_dir()?;
    if items.len() > 0 {
        let temp_dir = Builder::new()
            .prefix("hysteria_")
            .tempdir_in(app_tmp_dir)
            .expect("Failed to create temporary directory");
        let mut temp_json_file = temp_dir_path.join("config.json");
        let mut file =
            std::fs::File::create(&temp_json_file).expect("Failed to create temporary file");
        file.write_all(b"Hello, world!")
            .expect("Failed to write to temporary file");
    }
    let responses = if items.len() == 0 {
        types::KittyResponse {
            data: None,
            code: 100,
            msg: "hyteria config is empty".to_string(),
        }
    } else {
        types::KittyResponse {
            data: items,
            code: 0,
            msg: "".to_string(),
        }
    };

    commmand.arg("client").arg("/config.json");

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
        })
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![start_hy, stop_hy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
