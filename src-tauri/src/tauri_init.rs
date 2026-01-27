use log::{debug, trace, LevelFilter};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use std::path::PathBuf;
use tauri::Emitter;
use tauri_plugin_autostart::AutoLaunchManager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, registry::Registry};

use crate::state::{DatabaseState, LogLevelState};
use crate::tray::Tray;
use anyhow::Result;
use std::fs;
use tauri::{Manager, State};
use entity::base_config;

pub async fn init_db(app_dir: PathBuf) -> Result<DatabaseConnection, DbErr> {
    let sqlite_path = app_dir.join("MyApp.sqlite");
    trace!("{:?}", sqlite_path);
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
    // Use async channel for better performance
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();

    // Get the LogLevelState to store the filter handle for hot-reloading
    let log_level_state: State<LogLevelState> = app.state();

    // Initialize tracing subscriber (for shoes logs) - MUST be before any shoes function call
    // Default: info level, shoes follows the same level
    let frontend_writer = crate::logger::FrontendWriter::new(sender);
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,shoes=info"));

    // Create a reloadable filter for hot-reloading log level
    let (filter, reload_handle) = tracing_subscriber::reload::Layer::new(env_filter);

    // Store the reload handle in LogLevelState
    let mut handle = log_level_state.filter_handle.blocking_lock();
    *handle = Some(reload_handle);
    drop(handle);

    // Create a subscriber with ONLY frontend output (no stdout)
    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(
            // Frontend writer layer for forwarding to UI
            tracing_subscriber::fmt::layer()
                .with_writer(frontend_writer)
                .with_ansi(false)
                .with_target(false)
        );

    // Try to set as global default - this should work before any shoes call
    match subscriber.try_init() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to initialize tracing subscriber: {}", e);
        }
    }

    // Forward log messages to frontend with batching and throttling
    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        let mut log_buffer = Vec::with_capacity(100);
        let batch_size = 50;  // Send every 50 logs
        let max_interval_ms = 100;  // Or every 100ms
        let mut total_count = 0u64;
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(max_interval_ms));

        loop {
            tokio::select! {
                // Receive new logs
                Some(message) = receiver.recv() => {
                    log_buffer.push(message.trim().to_string());

                    // Send if buffer is full
                    if log_buffer.len() >= batch_size {
                        total_count += log_buffer.len() as u64;
                        let batch: Vec<String> = log_buffer.drain(..).collect();
                        let _ = app_clone.emit("kitty_logger", batch);
                    }
                }
                // Periodic flush
                _ = interval.tick() => {
                    if !log_buffer.is_empty() {
                        total_count += log_buffer.len() as u64;
                        let batch: Vec<String> = log_buffer.drain(..).collect();
                        let _ = app_clone.emit("kitty_logger", batch);
                    }
                }
            }
        }
    });

    Ok(())
}

/// Initialize log level from database on startup
fn setup_log_level_from_db(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let log_level_state: State<LogLevelState> = app.state();
    let db_state: State<DatabaseState> = app.state();
    let db = db_state.get_db();

    // Get the log level from database
    let log_level = tauri::async_runtime::block_on(async move {
        base_config::Model::first(&db).await.ok()
    });

    if let Some(Some(record)) = log_level {
        let log_level = record.log_level;

        // Update runtime log level - shoes follows the same log level
        let handle = log_level_state.filter_handle.blocking_lock();
        if let Some(filter_handle) = handle.as_ref() {
            let new_filter = format!("{},shoes={}", log_level, log_level);
            let _ = filter_handle.modify(|filter| {
                *filter = EnvFilter::new(new_filter);
            });
            tracing::info!("Log level initialized from database: {}", log_level);
        }
    }

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
    let _ = setup_log_level_from_db(handle)?;
    let _ = setup_system_autostart(handle)?;
    let _ = setup_auto_start_fastest(handle)?;
    let _ = Tray::init_tray(handle)?;
    // let _ = setup_global_shortcut(handle)?;
    Ok(())
}
