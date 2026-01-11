use crate::{proxy::system_proxy::clear_system_proxy, state::ProcessManagerState};
use log::trace;
use tauri::{AppHandle, Manager, State};

async fn clear_command(app_handle: &AppHandle) {
    let state: State<ProcessManagerState> = app_handle.state();

    // Abort all running shoes servers
    let mut running_servers = state.running_servers.lock().await;
    for handle in running_servers.iter() {
        handle.abort();
    }
    running_servers.clear();

    println!("clear_system_proxy called");
    clear_system_proxy();
}

pub fn on_exit_clear_commands(app_handle: &AppHandle) {
    trace!("on_exit_clear_commands call");
    tauri::async_runtime::block_on(clear_command(app_handle))
}
