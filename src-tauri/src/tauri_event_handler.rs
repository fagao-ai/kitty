use crate::state::ProcessManagerState;
#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
use crate::proxy::system_proxy::{clear_system_proxy, has_sys_proxy};
use log::trace;
use protocols::KittyCommandGroupTrait;
use tauri::{AppHandle, Manager, State};

async fn clear_command(app_handle: &AppHandle) {
    let state: State<ProcessManagerState> = app_handle.state();
    #[cfg(feature = "hysteria")]
    {
        let mut process_manager = state.hy_process_manager.lock().await;
        let process_manager = process_manager.as_mut();
        if let Some(process_manager) = process_manager {
            trace!("terminate_backends call");
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
    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
    if has_sys_proxy().is_ok() {
        clear_system_proxy().unwrap()
    };
}

pub fn on_exit_clear_commands(app_handle: &AppHandle) {
    trace!("on_exit_clear_commands call");
    tauri::async_runtime::block_on(clear_command(app_handle))
}
