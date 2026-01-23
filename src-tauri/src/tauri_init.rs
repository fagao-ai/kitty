use log::{debug, trace, LevelFilter};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use std::{path::PathBuf, sync::mpsc};
use tauri::Emitter;
use tauri_plugin_autostart::AutoLaunchManager;

use crate::{logger::KittyLogger, state::DatabaseState, tray::Tray};
use anyhow::Result;
use entity::base_config;
use std::fs;
use tauri::{Manager, State};

pub async fn init_db(app_dir: PathBuf) -> Result<DatabaseConnection, DbErr> {
    let sqlite_path = app_dir.join("MyApp.sqlite");
    trace!("{:?}", sqlite_path);
    println!("{:?}", sqlite_path);
    let sqlite_url = format!("sqlite://{}?mode=rwc", sqlite_path.to_string_lossy());
    let connect_options = ConnectOptions::new(sqlite_url)
        .sqlx_logging(false)
        .to_owned();
    let db: DatabaseConnection = Database::connect(connect_options).await?;
    Migrator::up(&db, None).await?;
    base_config::Model::update_sysproxy_flag(&db, false).await?;
    trace!("Migrator");
    Ok(db)
}

fn setup_db<'a>(handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let app_dir = handle
        .path()
        .app_local_data_dir()
        .expect("The app data directory should exist.");
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }
    trace!("app_dir: {:?}", app_dir);
    let app_state: State<DatabaseState> = handle.state();
    let db = tauri::async_runtime::block_on(async move {
        let db = init_db(app_dir).await;
        match db {
            Ok(db) => db,
            Err(err) => {
                panic!("Error: {}", err);
            }
        }
    });
    *app_state.db.lock().unwrap() = Some(db);
    Ok(())
}

fn setup_system_autostart<'a>(handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let db_state: State<DatabaseState> = handle.state();
    let auto_start_state: State<AutoLaunchManager> = handle.state();
    let db = db_state.get_db();
    tauri::async_runtime::block_on(async move {
        let record = base_config::Model::first(&db).await;
        if let Ok(record) = record {
            if let Some(auto_start) = record {
                if auto_start.auto_start {
                    if let Ok(is_enable) = auto_start_state.is_enabled() {
                        if !is_enable {
                            let _ = auto_start_state.enable();
                        }
                    }
                } else {
                    if let Ok(is_enable) = auto_start_state.is_enabled() {
                        if is_enable {
                            let _ = auto_start_state.disable();
                        }
                    }
                }
            }
        }
    });

    Ok(())
}

fn setup_kitty_logger(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel();
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        KittyLogger::new(LevelFilter::Info, Config::default(), sender),
    ])
    .unwrap();
    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        loop {
            match receiver.recv() {
                Ok(message) => app_clone.emit("kitty_logger", message).unwrap(),
                Err(_) => {
                    debug!("Channel closed");
                    break;
                }
            }
        }
    });

    Ok(())
}

/// Auto-measure and start the fastest proxy server on app startup.
/// This is the default behavior - no configuration needed.
fn setup_auto_start_fastest<'a>(handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    use crate::auto_starter::AutoStarter;
    use crate::state::ProcessManagerState;
    use log::info;

    let db_state: State<DatabaseState> = handle.state();
    let process_manager: State<ProcessManagerState> = handle.state();
    let db = db_state.get_db();

    // Clone ProcessManagerState for use in async task
    let process_manager_clone = process_manager.inner().clone();

    // Auto-start fastest is enabled by default
    tauri::async_runtime::spawn(async move {
        info!("Auto-start fastest: beginning delay measurement");

        let auto_starter = AutoStarter::new(db, process_manager_clone);
        match auto_starter.start_fastest_server().await {
            Ok(result) => {
                info!("Auto-start completed: {:?}", result);
            }
            Err(e) => {
                log::error!("Auto-start failed: {}", e);
            }
        }
    });

    Ok(())
}

// fn setup_global_shortcut<'a>(handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
//     use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

//     let command_w_shortcut = Shortcut::new(Some(Modifiers::META), Code::KeyW);
//     // let command_w_shortcut = Shortcut::new(Some(Modifiers::META), Code::KeyW);
//     let app_handle = handle.clone();
//     handle.plugin(
//         tauri_plugin_global_shortcut::Builder::with_handler(move |_app, shortcut| {
//             if shortcut == &command_w_shortcut {
//                 let window = app_handle.get_webview_window("main").unwrap();
//                 window.hide().unwrap();
//             }
//         })
//         .build(),
//     )?;

//     handle.global_shortcut().register(command_w_shortcut)?;
//     Ok(())
// }

pub fn init_setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let _ = setup_kitty_logger(handle)?;
    let _ = setup_db(handle)?;
    let _ = setup_db(handle)?;
    let _ = setup_system_autostart(handle)?;
    let _ = setup_auto_start_fastest(handle)?;
    let _ = Tray::init_tray(handle)?;
    // let _ = setup_global_shortcut(handle)?;
    Ok(())
}
