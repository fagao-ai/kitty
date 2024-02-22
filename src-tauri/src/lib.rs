use state::{DatabaseState, ProcessManagerState};
use std::env;
use tauri::RunEvent;
use tauri::{generate_handler, ipc::Invoke};
#[cfg(feature = "hysteria")]
use tauri_apis::hysteria as hysteria_api;
#[cfg(feature = "xray")]
use tauri_apis::xray as xray_api;
use tauri_init::init_setup;

use tauri_apis::common as common_api;
use tauri_plugin_autostart::MacosLauncher;

use crate::state::KittyProxyState;
use crate::tauri_apis::{start_system_proxy, stop_system_proxy};
use crate::tauri_event_handler::on_exit_clear_commands;

mod apis;
mod proxy;
mod state;
mod tauri_apis;
mod tauri_event_handler;
mod tauri_init;
mod tray;
mod types;

// async fn on_window_exit(event: tauri::GlobalWindowEvent) {
//     println!("on_window_exit call!!!");
//     println!("{:?}", event.event());
//     match event.event() {
//         WindowEvent::Destroyed => {
//             println!("exit!!!");
//             let state: State<ProcessManagerState> = event.window().state();
//             #[cfg(feature = "hysteria")]
//             {
//                 let mut process_manager = state.hy_process_manager.lock().await;
//                 let process_manager = process_manager.as_mut();
//                 if let Some(process_manager) = process_manager {
//                     println!("terminate_backends call");
//                     if process_manager.terminate_backends().is_err() {
//                         let app = event.window();
//                         if let Ok(PermissionState::Granted) = app.notification().permission_state()
//                         {
//                             app.notification()
//                                 .builder()
//                                 .body(format!("{} terminate failed.", process_manager.name()))
//                                 .show()
//                                 .unwrap();
//                         }
//                     }
//                 }
//             }

//             #[cfg(feature = "xray")]
//             {
//                 let mut process_manager = state.xray_process_manager.lock().await;
//                 let process_manager = process_manager.as_mut();
//                 if let Some(process_manager) = process_manager {
//                     if process_manager.terminate_backends().is_err() {
//                         let app = event.window();
//                         if let Ok(PermissionState::Granted) = app.notification().permission_state()
//                         {
//                             app.notification()
//                                 .builder()
//                                 .body(format!("{} terminate failed.", process_manager.name()))
//                                 .show()
//                                 .unwrap();
//                         }
//                     }
//                 }
//             }
//         }
//         _ => {}
//     }
// }

// fn on_window_exit_func(event: tauri::GlobalWindowEvent) {
//     tauri::async_runtime::block_on(on_window_exit(event))
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default().manage(DatabaseState {
        db: Default::default(),
    });
    let builder = builder.manage(ProcessManagerState::default());
    let builder = builder
        .manage(KittyProxyState::default())
        // .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(init_setup);
    // .on_window_event(on_window_exit_func);
    #[cfg(feature = "xray")]
    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
    let handler: fn(Invoke) -> bool = generate_handler![
        xray_api::add_xray_item,
        xray_api::get_all_xrays,
        xray_api::import_xray_subscribe,
        xray_api::update_xray_item,
        xray_api::delete_xray_item,
        xray_api::speed_xray_delay,
        xray_api::get_xray_by_id,
        common_api::copy_proxy_env_cmd,
        common_api::query_base_config,
        common_api::update_base_config,
        common_api::query_rules,
        common_api::delete_rules,
        common_api::add_rules,
        common_api::update_rules_item,
        start_system_proxy,
        stop_system_proxy,
    ];

    #[cfg(feature = "xray")]
    let handler: fn(Invoke) -> bool = generate_handler![
        xray_api::add_xray_item,
        xray_api::get_all_xrays,
        xray_api::import_xray_subscribe,
        xray_api::update_xray_item,
        xray_api::delete_xray_item,
        xray_api::speed_xray_delay,
        xray_api::get_xray_by_id,
        common_api::query_base_config,
        common_api::update_base_config,
        common_api::query_rules,
        common_api::delete_rules,
        common_api::add_rules,
        common_api::update_rules_item,
        start_system_proxy,
        stop_system_proxy,
    ];

    #[cfg(feature = "hysteria")]
    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
    let handler: fn(Invoke) -> bool = generate_handler![
        hysteria_api::add_hysteria_item,
        hysteria_api::get_all_hysterias,
        hysteria_api::update_hysteria_item,
        hysteria_api::delete_hysteria_item,
        hysteria_api::speed_hysteria_delay,
        hysteria_api::get_hysteria_by_id,
        common_api::copy_proxy_env_cmd,
        common_api::query_base_config,
        common_api::update_base_config,
        common_api::query_rules,
        common_api::delete_rules,
        common_api::add_rules,
        common_api::update_rules_item,
        start_system_proxy,
        stop_system_proxy,
    ];

    #[cfg(all(feature = "xray", feature = "hysteria"))]
    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
    let handler: fn(Invoke) -> bool = generate_handler![
        hysteria_api::add_hysteria_item,
        hysteria_api::get_all_hysterias,
        hysteria_api::update_hysteria_item,
        hysteria_api::delete_hysteria_item,
        hysteria_api::speed_hysteria_delay,
        hysteria_api::get_hysteria_by_id,
        xray_api::add_xray_item,
        xray_api::get_all_xrays,
        xray_api::import_xray_subscribe,
        xray_api::update_xray_item,
        xray_api::delete_xray_item,
        xray_api::speed_xray_delay,
        xray_api::get_xray_by_id,
        common_api::query_base_config,
        common_api::update_base_config,
        common_api::copy_proxy_env_cmd,
        common_api::query_base_config,
        common_api::update_base_config,
        common_api::query_rules,
        common_api::delete_rules,
        common_api::add_rules,
        common_api::update_rules_item,
        start_system_proxy,
        stop_system_proxy,
    ];

    let builder = builder.invoke_handler(handler);
    builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            if let RunEvent::Exit = event {
                println!("RunEvent exit!!!");
                // clear_command(app);
                on_exit_clear_commands(app);
            }
        });
}
