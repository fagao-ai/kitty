use log::{trace, info};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::path::PathBuf;
use tauri::Emitter;
use tauri_plugin_autostart::AutoLaunchManager;
use tracing_subscriber::EnvFilter;

use crate::state::DatabaseState;
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

    // Enable WAL mode for better concurrency - allows reads during writes
    use sea_orm::ConnectionTrait;
    db.execute_unprepared("PRAGMA journal_mode=WAL;").await?;

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

    // Get rules file path for migration
    let rules_path = handle
        .path()
        .app_data_dir()
        .map_err(|e| anyhow::anyhow!("Failed to get app data dir: {}", e))?;
    if !rules_path.exists() {
        fs::create_dir_all(&rules_path)?;
    }
    let rules_path = rules_path.join("custom_rules.json");

    let db = tauri::async_runtime::block_on(async move {
        let db = init_db(app_dir).await;
        match db {
            Ok(db) => {
                // Migrate rules from database to file if needed
                if let Err(e) = crate::apis::common_apis::CommonAPI::migrate_rules_from_db_to_file(&db, rules_path).await {
                    log::warn!("Failed to migrate rules from database: {}", e);
                }
                db
            },
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
    // Get the log sender that was created during early initialization
    let sender = crate::get_log_sender()
        .ok_or("Log sender not initialized")?;

    // First, drain the buffered logs accumulated during startup
    let buffered_logs = {
        let mut buffer = crate::LOG_BUFFER.lock().unwrap();
        buffer.drain(..).collect::<Vec<String>>()
    };

    // Send buffered logs to frontend immediately
    if !buffered_logs.is_empty() {
        tracing::info!("Sending {} buffered startup logs to frontend", buffered_logs.len());
        let batch: Vec<String> = buffered_logs.iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if !batch.is_empty() {
            let _ = app.emit("kitty_logger", batch);
        }
    }

    // Create a task that periodically checks buffer and sends to frontend
    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));

        loop {
            interval.tick().await;

            // Check if there are new logs in the buffer
            let new_logs = {
                let mut buffer = crate::LOG_BUFFER.lock().unwrap();
                if buffer.is_empty() {
                    Vec::new()
                } else {
                    buffer.drain(..).collect::<Vec<String>>()
                }
            };

            if !new_logs.is_empty() {
                let batch: Vec<String> = new_logs.iter()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                if !batch.is_empty() {
                    let _ = app_clone.emit("kitty_logger", batch);
                }
            }
        }
    });

    tracing::info!("✓ setup_kitty_logger completed - logs will be sent to frontend");
    Ok(())
}

/// Initialize log level from database on startup
fn setup_log_level_from_db(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let db_state: State<DatabaseState> = app.state();
    let db = db_state.get_db();

    // Get the log level from database
    let log_level = tauri::async_runtime::block_on(async move {
        base_config::Model::first(&db).await.ok()
    });

    if let Some(Some(record)) = log_level {
        let log_level = record.log_level;

        // Update runtime log level using global filter reload handle
        if let Some(filter_handle) = crate::get_filter_reload_handle() {
            if let Ok(handle) = filter_handle.lock() {
                let new_filter = format!("{},shoes={}", log_level, log_level);

                if handle.modify(|filter| {
                    *filter = EnvFilter::new(new_filter.clone());
                }).is_ok() {
                    tracing::info!("Log level initialized from database: {}", log_level);
                }
            }
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

    // Get the resource directory for geo files
    let resource_dir = handle.path().resource_dir()
        .map_err(|e| {
            log::error!("Failed to get resource dir: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;

    let custom_rules_path = handle.path().app_data_dir()
        .map_err(|e| {
            log::error!("Failed to get app data dir: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?
        .join("custom_rules.json");

    // Auto-start fastest is enabled by default
    tauri::async_runtime::spawn(async move {
        // CRITICAL: Increase delay to ensure tracing subscriber is fully initialized
        // This is especially important for release builds where the startup is faster
        // and LogTracer needs time to be fully set up before shoes starts logging
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        info!("Auto-start fastest: beginning delay measurement");
        info!("Custom rules path: {}", custom_rules_path.display());

        let auto_starter = AutoStarter::new(db, process_manager_clone, resource_dir, custom_rules_path);
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

pub fn init_setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let _ = setup_kitty_logger(handle)?;
    let _ = setup_db(handle)?;
    let _ = setup_log_level_from_db(handle)?;
    let _ = setup_system_autostart(handle)?;
    let _ = setup_auto_start_fastest(handle)?;
    let _ = Tray::init_tray(handle)?;
    Ok(())
}
