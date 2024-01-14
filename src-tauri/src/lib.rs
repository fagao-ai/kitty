mod apis;
mod database;
mod proxy;
mod state;
mod tauri_apis;
mod types;
mod utils;

use std::{env, fs};

use kitty_proxy::{HttpProxy, MatchProxy, SocksProxy};

use crate::state::KittyProxyState;
use anyhow::Result;
#[cfg(feature = "hysteria")]
use tauri_apis::hysteria as hysteria_api;

#[cfg(feature = "xray")]
use tauri_apis::xray as xray_api;

use entity::base_config;
use protocols::CommandManagerTrait;
use state::{DatabaseState, ProcessManagerState};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{ClickType, TrayIconBuilder},
    Icon, WindowEvent,
};
use tauri::{Manager, State};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_notification::{NotificationExt, PermissionState};
use tokio::sync::Mutex;

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
        let geoip_file = resource_dir.join("kitty_geoip.dat");
        let geosite_file = resource_dir.join("kitty_geosite.dat");
        let match_proxy =
            MatchProxy::from_geo_dat(Some(&geoip_file), Some(&geosite_file)).unwrap();
        let http_proxy = HttpProxy::new(record.local_ip.as_str(), http_port, None)
            .await
            .unwrap();
        let socks_proxy = SocksProxy::new(record.local_ip.as_str(), socks_port, None)
            .await
            .unwrap();
        *app_state.socks_proxy.lock().await = Some(socks_proxy);
        *app_state.http_proxy.lock().await = Some(http_proxy);
        *app_state.match_proxy.lock().await = Some(match_proxy);
    });

    Ok(())
}

async fn on_window_exit(event: tauri::GlobalWindowEvent) {
    match event.event() {
        WindowEvent::Destroyed => {
            println!("exit!!!");
            let state: State<ProcessManagerState> = event.window().state();
            let mut process_manager = state.hy_process_manager.lock().await;
            let process_manager = process_manager.as_mut();
            if let Some(process_manager) = process_manager {
                if process_manager.terminate_backend().is_err() {
                    let app = event.window();
                    if let Ok(PermissionState::Granted) = app.notification().permission_state() {
                        app.notification()
                            .builder()
                            .body(format!("{} terminate failed.", process_manager.name()))
                            .show()
                            .unwrap();
                    }
                }
            }
        }
        _ => {}
    }
}

fn on_window_exit_func(event: tauri::GlobalWindowEvent) {
    tauri::async_runtime::block_on(on_window_exit(event))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .manage(DatabaseState {
            db: Default::default(),
        })
        .manage(ProcessManagerState {
            process_manager: Mutex::new(HashMap::new()),
        })
        .manage(KittyProxyState {
            http_proxy: Mutex::new(None),
            socks_proxy: Mutex::new(None),
            match_proxy: Mutex::new(None),
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
        .on_window_event(on_window_exit_func);
    let builder = builder.invoke_handler(
        #[cfg(feature = "hysteria")]
        tauri::generate_handler![
            hysteria_api::get_hysteria_status,
            hysteria_api::add_hy_item,
            hysteria_api::get_all_hysterias,
            hysteria_api::query_base_config,
            hysteria_api::update_base_config,
        ],
    );

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
