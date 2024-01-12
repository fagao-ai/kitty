mod database;
mod proxy;
mod state;
mod types;
mod utils;
mod api_traits;
mod xray_apis;
mod hysteria_apis;

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
use std::{collections::HashMap, env, fs, io::Write, path::PathBuf, borrow::BorrowMut};
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

use protocols::{HysteriaManager, XrayManager, CommandManagerTrait};

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
async fn proxies_delay(proxies: Vec<ProxyInfo>) -> CommandResult<KittyResponse<Vec<ProxyDelay>>> {
    let results = kitty_proxies_delay(proxies).await;
    Ok(KittyResponse::<Vec<ProxyDelay>>::from_data(results))
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

fn setup_db<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
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

fn setup_kitty_proxy<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let resource_dir = handle.path().resource_dir()?;
    let app_state: State<KittyProxyState> = handle.state();
    let db_state: State<DatabaseState> = handle.state();
    let db = db_state.get_db();
    tauri::async_runtime::block_on(async move {
        let record = base_config::Model::first(&db).await.unwrap().unwrap();
        let http_port = record.http_port;
        let socks_port = record.socks_port;
        let geoip_file = resource_dir.join("geoip.dat");
        let geosite_file = resource_dir.join("geosite.dat");
        let _match_proxy =
            MatchProxy::from_geo_dat(Some(&geoip_file), Some(&geosite_file)).unwrap();
        let http_proxy = HttpProxy::new("127.0.0.1", 10088, None, "127.0.0.1", 10809)
            .await
            .unwrap();
        let socks_proxy = SocksProxy::new("127.0.0.1", 10089, None, "127.0.0.1", 10809)
            .await
            .unwrap();
        *app_state.socks_proxy.lock().await = socks_proxy;
        *app_state.http_proxy.lock().await = http_proxy;
        // (http_proxy, socks_proxy)
    });

    Ok(())
}

async fn on_window_exit(event: tauri::GlobalWindowEvent) {
    match event.event() {
        WindowEvent::Destroyed => {
            println!("exit!!!");
            let am: State<ProcessManagerState> = event.window().state();
            am.hy_process_manager
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
            hy_process_manager: Mutex::new(HysteriaManager::new("".into())),
            xray_process_manager: Mutex::new(XrayManager::new("".into(), HashMap::new())),
        })
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .setup(setup_db)
        .setup(setup_kitty_proxy)
        .on_window_event(on_window_exit_func)
        .invoke_handler(tauri::generate_handler![
            stop_hysteria,
            start_hysteria,
            add_hy_item,
            get_all_proxies,
            query_base_config,
            update_base_config,
            proxies_delay,
            get_hysteria_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
