use log::trace;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};
use tokio::sync::RwLock;
use std::path::PathBuf;
use tauri_plugin_autostart::AutoLaunchManager;

use crate::{state::DatabaseState, tray::Tray};
use anyhow::Result;
use entity::base_config;
use kitty_proxy::MatchProxy;
use std::fs;
use std::sync::Arc;
use tauri::{Manager, State};

use crate::state::KittyProxyState;

pub async fn init_db(app_dir: PathBuf) -> Result<DatabaseConnection, DbErr> {
    let sqlite_path = app_dir.join("MyApp.sqlite");
    trace!("{:?}", sqlite_path);
    let sqlite_url = format!("sqlite://{}?mode=rwc", sqlite_path.to_string_lossy());
    let db: DatabaseConnection = Database::connect(&sqlite_url).await?;
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

fn setup_kitty_proxy<'a>(handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let resource_dir = handle.path().resource_dir()?.join("static");
    let app_state: State<KittyProxyState> = handle.state();
    tauri::async_runtime::block_on(async move {
        trace!(
            "resource_dir: {:?}, exists: {}",
            resource_dir,
            resource_dir.exists()
        );
        let geoip_file = resource_dir.join("kitty_geoip.dat");
        let geosite_file = resource_dir.join("kitty_geosite.dat");
        let match_proxy = MatchProxy::from_geo_dat(Some(&geoip_file), Some(&geosite_file)).unwrap();
        *app_state.match_proxy.lock().await = Some(Arc::new(RwLock::new(match_proxy)));
    });

    Ok(())
}

fn setup_auto_start<'a>(handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn init_setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let _ = setup_db(handle)?;
    let _ = setup_db(handle)?;
    let _ = setup_auto_start(handle)?;
    let _ = setup_kitty_proxy(handle)?;
    let _ = Tray::init_tray(handle);
    Ok(())
}
