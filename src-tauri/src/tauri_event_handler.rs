use crate::{
    proxy::system_proxy::{clear_system_proxy, has_sys_proxy},
    state::ProcessManagerState,
};
use protocols::KittyCommandGroupTrait;
use tauri::{AppHandle, Manager, State};

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
    if has_sys_proxy().is_ok() {
        clear_system_proxy().unwrap()
    };
}

pub fn on_exit_clear_commands(app_handle: &AppHandle) {
    println!("on_exit_clear_commands call");
    tauri::async_runtime::block_on(clear_command(app_handle))
}
