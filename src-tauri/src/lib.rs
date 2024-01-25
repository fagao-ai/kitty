use anyhow::Result;
use state::{DatabaseState, ProcessManagerState};
use std::env;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{ClickType, TrayIconBuilder},
    Icon, WindowEvent,
};
use tauri::{Manager, State};
#[cfg(feature = "hysteria")]
use tauri_apis::hysteria as hysteria_api;
#[cfg(feature = "xray")]
use tauri_apis::xray as xray_api;
use tauri_init::init_setup;

use tauri_apis::common as common_api;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_notification::{NotificationExt, PermissionState};

use crate::state::KittyProxyState;

mod apis;
mod proxy;
mod state;
mod tauri_apis;
mod tauri_init;
mod types;
mod utils;
mod tray;

// fn set_system_tray<'a>(app: &'a mut tauri::App) -> Result<()> {
//     let quit = MenuItemBuilder::with_id("quit", "Quit").build(app);
//     let hide = MenuItemBuilder::with_id("hide", "Hide").build(app);
//     let menu = MenuBuilder::new(app).items(&[&quit, &hide]).build()?;
//     let current_path = env::current_dir()?;
//     println!("current_path: {:?}", current_path);
//     let parent_dir = current_path.to_owned();
//     let icon_path = parent_dir.join("icons").join("icons8-48.png");
//     println!("icon_path: {:?}", icon_path);
//     let icon = Icon::File(icon_path);
//     print!("set_system_tray");
//     let _tray = TrayIconBuilder::new()
//         .menu(&menu)
//         .icon(icon)
//         .on_menu_event(
//             move |app, event: tauri::menu::MenuEvent| match event.id().as_ref() {
//                 "hide" => {
//                     let window: tauri::Window = app.get_window("main").unwrap();
//                     window.hide().unwrap();
//                 }
//                 "quit" => {
//                     app.exit(0);
//                 }

//                 _ => (),
//             },
//         )
//         .on_tray_icon_event(|tray, event| {
//             if event.click_type == ClickType::Left {
//                 let app = tray.app_handle();
//                 if let Some(window) = app.get_window("main") {
//                     let _ = window.show();
//                     let _ = window.set_focus();
//                 }
//             }
//         })
//         .build(app)?;
//     Ok(())
// }

async fn on_window_exit(event: tauri::GlobalWindowEvent) {
    match event.event() {
        WindowEvent::Destroyed => {
            println!("exit!!!");
            let state: State<ProcessManagerState> = event.window().state();
            #[cfg(feature = "hysteria")]
            {
                let mut process_manager = state.hy_process_manager.lock().await;
                let process_manager = process_manager.as_mut();
                if let Some(process_manager) = process_manager {
                    if process_manager.terminate_backends().is_err() {
                        let app = event.window();
                        if let Ok(PermissionState::Granted) = app.notification().permission_state()
                        {
                            app.notification()
                                .builder()
                                .body(format!("{} terminate failed.", process_manager.name()))
                                .show()
                                .unwrap();
                        }
                    }
                }
            }

            #[cfg(feature = "xray")]
            {
                let mut process_manager = state.xray_process_manager.lock().await;
                let process_manager = process_manager.as_mut();
                if let Some(process_manager) = process_manager {
                    if process_manager.terminate_backends().is_err() {
                        let app = event.window();
                        if let Ok(PermissionState::Granted) = app.notification().permission_state()
                        {
                            app.notification()
                                .builder()
                                .body(format!("{} terminate failed.", process_manager.name()))
                                .show()
                                .unwrap();
                        }
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
    let builder = tauri::Builder::default().manage(DatabaseState {
        db: Default::default(),
    });
    let builder = builder.manage(ProcessManagerState::default());
    let builder = builder
        .manage(KittyProxyState::default())
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(init_setup)
        .on_window_event(on_window_exit_func);

    let builder = builder.invoke_handler(tauri::generate_handler![
        common_api::query_base_config,
        common_api::update_base_config,
        tauri_apis::set_system_proxy,
    ]);
    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
        let builder = builder.invoke_handler(tauri::generate_handler![common_api::copy_proxy_env_cmd,]);
    #[cfg(feature = "hysteria")]
        let builder = builder.invoke_handler(tauri::generate_handler![
        hysteria_api::add_hysteria_item,
        hysteria_api::get_all_hysterias,
        hysteria_api::update_hysteria_item,
        hysteria_api::delete_hysteria_item,
    ]);

    #[cfg(feature = "xray")]
        let builder = builder.invoke_handler(tauri::generate_handler![
        xray_api::add_xray_item,
        xray_api::get_all_xrays,
        xray_api::import_by_subscribe_url,
        xray_api::update_xray_item,
        xray_api::delete_xray_item,
    ]);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
