use anyhow::{Ok, Result};
use protocols::KittyCommandGroupTrait;
use std::env;
use tauri::menu::{Menu, MenuEvent};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{ClickType, TrayIconBuilder},
    Icon,
};
use tauri::{AppHandle, Manager, State, Wry};

use crate::state::{DatabaseState, ProcessManagerState};
use crate::tauri_apis::common as common_api;

pub struct Tray {}

async fn clear_command(app_handle: &AppHandle) {
    let state: State<ProcessManagerState> = app_handle.state();
    #[cfg(feature = "hysteria")]
    {
        let mut process_manager = state.hy_process_manager.lock().await;
        let process_manager = process_manager.as_mut();
        if let Some(process_manager) = process_manager {
            println!("terminate_backends call");
            process_manager.terminate_backends().unwrap();
        }
    }

    #[cfg(feature = "xray")]
    {
        let mut process_manager = state.xray_process_manager.lock().await;
        let process_manager = process_manager.as_mut();
        if let Some(process_manager) = process_manager {
            process_manager.terminate_backends().unwrap();
        }
    }
}

fn on_exit_clear_commands(app_handle: &AppHandle) {
    tauri::async_runtime::block_on(clear_command(app_handle))
}

impl Tray {
    fn tray_menu(app_handle: &AppHandle) -> Result<Menu<Wry>> {
        let quit = MenuItemBuilder::with_id("quit", "Quit")
            .accelerator("CmdOrControl+Q")
            .build(app_handle);
        let hide = MenuItemBuilder::with_id("hide", "Hide")
            .accelerator("CmdOrControl+W")
            .build(app_handle);
        let system_proxy = MenuItemBuilder::with_id("system_proxy", "System Proxy")
            .accelerator("CmdOrControl+Shift+Y")
            .build(app_handle);
        let copy_env = MenuItemBuilder::with_id("copy_env", "Copy ENV")
            .accelerator("CmdOrControl+Shift+C")
            .build(app_handle);
        let menu = MenuBuilder::new(app_handle)
            .items(&[&quit, &hide, &system_proxy, &copy_env])
            .build()?;
        Ok(menu)
    }

    pub fn init_tray(app_handle: &AppHandle) -> Result<()> {
        let menu = Tray::tray_menu(app_handle)?;
        let icon = Tray::icon()?;
        let _tray = TrayIconBuilder::new()
            .menu(&menu)
            .icon(icon)
            .on_menu_event(move |app, event: tauri::menu::MenuEvent| {
                Tray::on_menu_event(app, &event)
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
            .build(app_handle)?;
        Ok(())
    }

    fn icon() -> Result<Icon> {
        let current_path = env::current_dir()?;
        println!("current_path: {:?}", current_path);
        let parent_dir = current_path.to_owned();
        let icon_path = parent_dir.join("icons").join("icons8-48.png");
        println!("icon_path: {:?}", icon_path);
        let icon = Icon::File(icon_path);
        print!("set_system_tray");
        Ok(icon)
    }

    fn on_menu_event(app_handle: &AppHandle, event: &MenuEvent) -> () {
        match event.id().as_ref() {
            "hide" => {
                let window = app_handle.get_window("main").unwrap();
                window.hide().unwrap();
            }
            "quit" => {
                on_exit_clear_commands(app_handle);
                app_handle.exit(0);
                // std::process::exit(0);
            }
            "system_proxy" => (),
            "copy_env" => {
                let db_state: State<DatabaseState> = app_handle.state();
                let db = db_state.get_db();
                tauri::async_runtime::block_on(async move {
                    let _ = common_api::copy_proxy_env(app_handle, &db).await;
                });
            }
            //
            _ => (),
        }
    }
}
