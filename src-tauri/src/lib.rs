mod database;
mod hysteria;
mod state;
mod utils;

use std::fs;

use hysteria::HyConfig;
use hysteria_rs::start_from_json;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{ClickType, TrayIconBuilder},
};



use state::{AppState, ServiceAccess};
use tauri::{AppHandle, Manager, State};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(app_handle: AppHandle, name: &str) -> String {
    // Should handle errors instead of unwrapping here
    // app_handle.db(|db| database::add_item(name, db)).unwrap();

    // let items = app_handle.db(|db| database::get_all(db)).unwrap();

    // let items_string = items.join(" | ");

    // format!("Your name log: {}", items_string)
    "".to_string()
}
#[tauri::command]
fn start_hy(hy_config: HyConfig) {
    let serialized_hy_config = serde_json::to_string(&hy_config).unwrap();
    start_from_json(&serialized_hy_config);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let config_str = r#"{
            "server": "ip:port",
            "auth": "password",
            "bandwidth": {
              "up": "10 mbps",
              "down": "100 mbps"
            },
            "tls": {
              "sni": "bing.com",
              "insecure": true
            },
            "socks5": {
              "listen": "127.0.0.1:1080"
            },
            "http": {
              "listen": "127.0.0.1:8080"
            }
          }"#;
        let hy_config: HyConfig = serde_json::from_str(&config_str).unwrap();
        start_hy(hy_config);
    }
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}


fn set_system_tray<'a>(app: &'a mut tauri::App) ->  Result<(), Box<dyn std::error::Error>>{
    let toggle = MenuItemBuilder::with_id("toggle", "Toggle").build(app);
    let menu = MenuBuilder::new(app).items(&[&toggle]).build()?;
    let path = concat!(, "/examples/icon.png");
    let icon = load_icon(std::path::Path::new(path));
    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(icon)
        .on_menu_event(move |app, event| match event.id().as_ref() {
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
                panic!("Error: {}" , err);
            }
        }
    });
    *app_state.db.lock().unwrap() = Some(db);
    let _ = set_system_tray(app);
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // TrayIconBuilder::new()
//   let tray_menu = SystemTrayMenu::new(); // insert the menu items here
//   let system_tray = SystemTray::new()
//     .with_menu(tray_menu);
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .setup(setup)
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![start_hy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
