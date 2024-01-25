use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::path::PathBuf;

use crate::{state::DatabaseState, tray::Tray};
use anyhow::Result;
use kitty_proxy::MatchProxy;
use std::fs;
use std::sync::Arc;
use tauri::{Manager, State};

use crate::state::KittyProxyState;

pub async fn init_db(app_dir: PathBuf) -> Result<DatabaseConnection, DbErr> {
    println!("app_dir");
    let sqlite_path = app_dir.join("MyApp.sqlite");
    let sqlite_url = format!("sqlite://{}?mode=rwc", sqlite_path.to_string_lossy());
    let db: DatabaseConnection = Database::connect(&sqlite_url).await?;
    Migrator::up(&db, None).await?;
    println!("Migrator");
    Ok(db)
}

fn setup_db<'a>(handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("setup_db!!!");
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
        println!(
            "resource_dir: {:?}, exists: {}",
            resource_dir,
            resource_dir.exists()
        );
        let geoip_file = resource_dir.join("kitty_geoip.dat");
        let geosite_file = resource_dir.join("kitty_geosite.dat");
        println!("geoip_file: {:?}", geoip_file);
        println!("geosite_file: {:?}", geosite_file);
        let match_proxy = MatchProxy::from_geo_dat(Some(&geoip_file), Some(&geosite_file)).unwrap();
        *app_state.match_proxy.lock().await = Some(Arc::new(match_proxy));
    });

    Ok(())
}

pub fn init_setup<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();
    let _ = setup_db(handle)?;
    let _ = setup_kitty_proxy(handle)?;
    let _ = Tray::init_tray(handle);
    Ok(())
}
