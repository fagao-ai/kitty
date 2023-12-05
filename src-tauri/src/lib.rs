mod database;
mod hysteria;
mod state;
mod utils;

use std::{env, sync::atomic::AtomicBool};

use hysteria::HyConfig;
use hysteria_rs::start_from_json;
use libc::{c_void, pthread_cancel, pthread_create};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{ClickType, TrayIconBuilder},
    Icon,
};

use state::{AppState, ServiceAccess};
use tauri::{AppHandle, Manager, State};
extern "C" fn thread_func(_: *mut c_void) -> *mut c_void {
    println!("Hello from the child thread");
    let serialized_hy_config = r#"{
        "server": "addr:ip",
        "auth": "xxx",
        "bandwidth": {
          "up": "20 mbps",
          "down": "100 mbps"
        },
        "tls": {
          "sni": "bing.com",
          "insecure": true
        },
        "socks5": {
          "listen": "127.0.0.1:1070"
        },
        "http": {
          "listen": "127.0.0.1:1071"
        }
      }"#;
    start_from_json(&serialized_hy_config);
    0 as *mut c_void
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn start_hy(app_handle: AppHandle) {
    let cancel_flag = AtomicBool::new(false);
    let cancel_flag_ptr = &cancel_flag as *const AtomicBool;
    let thread_id: u64 = unsafe {
        let mut thread_id: libc::pthread_t = std::mem::zeroed();
        let _result = pthread_create(
            &mut thread_id,
            std::ptr::null(),
            thread_func,
            cancel_flag_ptr as *mut c_void,
        );
        thread_id
    };
    println!("stop thread_id: {}", thread_id);
    let app_state: State<AppState> = app_handle.state();
    *app_state.thread_id.lock().unwrap() = Some(thread_id);
}

#[tauri::command]
fn stop_hy(app_handle: AppHandle) {
    println!("stop_hy called!!!");

    let app_state: State<AppState> = app_handle.state();
    let mut thread_id = app_state.thread_id.lock().unwrap();
    let thread_id = thread_id.unwrap();
    println!("stop thread_id: {}", thread_id);
    unsafe {
        let result = pthread_cancel(thread_id);
        println!("pthread_cancel!!!{}", result);
        if result != 0 {
            panic!("Failed to cancel thread");
        }
        *app_state.thread_id.lock().unwrap() = None;
    
    }
    println!("alread stop!!!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let config_str = r#"{
            "server": "129.153.56.56:8887",
            "auth": "h19951218",
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
        // start_hy(hy_config);
        println!("start hy from hy_config !!!");
    }
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
    *app_state.thread_id.lock().unwrap() = None;
    let _ = set_system_tray(app);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
            thread_id: Default::default(),
        })
        .setup(setup)
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![start_hy, stop_hy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
