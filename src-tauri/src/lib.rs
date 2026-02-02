use state::{DatabaseState, ProcessManagerState};
use std::env;
use std::sync::OnceLock;
use tauri::RunEvent;
use tauri::{generate_handler, ipc::Invoke};
use tauri_apis::server as server_api;
use tauri_init::init_setup;
use tracing_subscriber::EnvFilter;

use tauri_apis::common as common_api;
use tauri_apis::proxy as proxy_api;
use tauri_plugin_autostart::MacosLauncher;

use crate::tauri_apis::{start_all_servers, set_system_proxy_only, stop_system_proxy};
use crate::tauri_event_handler::on_exit_clear_commands;

mod apis;
mod auto_starter;
mod config_converter;
mod proxy;
mod rules;
mod state;
mod tauri_apis;
mod tauri_event_handler;
mod tauri_init;
mod tray;
mod types;

// Global channel for log messages - must be initialized before any other code
static LOG_SENDER: OnceLock<tokio::sync::broadcast::Sender<String>> = OnceLock::new();
static LOG_BUFFER: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
static FILTER_RELOAD_HANDLE: OnceLock<std::sync::Mutex<tracing_subscriber::reload::Handle<EnvFilter, tracing_subscriber::Registry>>> = OnceLock::new();

/// CRITICAL: Initialize logging system BEFORE any other code runs
/// This uses the ctor crate to run initialization code before main() is called
#[ctor::ctor]
fn init_log_tracer() {
    use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt};

    // Create a log writer that writes to global buffer
    let log_writer = BufferedLogWriter;

    // Create env filter - default to debug to capture shoes logs
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug,shoes=debug"));

    let (filter, reload_handle) = tracing_subscriber::reload::Layer::new(env_filter);

    // Store reload handle globally for later use
    let _ = FILTER_RELOAD_HANDLE.set(std::sync::Mutex::new(reload_handle));

    // Build subscriber - always output to stderr for visibility
    // PLUS buffered writer for frontend
    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(std::io::stderr)
                .with_ansi(true)
        )
        .with(
            fmt::layer()
                .with_writer(log_writer)
                .with_ansi(false)
                .with_target(false)
        );

    // Set as global default WITHOUT initializing log crate
    let _ = tracing::subscriber::set_global_default(subscriber);

    // Initialize LogTracer to bridge log -> tracing
    if tracing_log::LogTracer::init().is_ok() {
        // CRITICAL: Set log crate's max level to match tracing's level
        // Without this, log crate will filter out debug/trace logs before they reach tracing
        log::set_max_level(log::LevelFilter::Trace);
    }
}

/// Buffered log writer that writes to global LOG_BUFFER
#[derive(Clone, Copy)]
struct BufferedLogWriter;

impl std::io::Write for BufferedLogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let msg = String::from_utf8_lossy(buf).to_string();
        if let Ok(mut buffer) = LOG_BUFFER.lock() {
            buffer.push(msg);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for BufferedLogWriter {
    type Writer = BufferedLogWriter;

    fn make_writer(&'a self) -> Self::Writer {
        BufferedLogWriter
    }
}

/// Early initialization of logger frontend - creates broadcast channel
/// This is called after LogTracer is already initialized in #[ctor]
fn early_init_logger() {
    // Create broadcast channel for log messages
    let (sender, _receiver) = tokio::sync::broadcast::channel(1000);

    // Store sender globally so setup_kitty_logger can access it later
    let _ = LOG_SENDER.set(sender.clone());
}

/// Get the global filter reload handle for dynamic log level changes
pub fn get_filter_reload_handle() -> Option<&'static std::sync::Mutex<tracing_subscriber::reload::Handle<tracing_subscriber::EnvFilter, tracing_subscriber::Registry>>> {
    FILTER_RELOAD_HANDLE.get()
}

/// Get the global log channel sender
pub fn get_log_sender() -> Option<&'static tokio::sync::broadcast::Sender<String>> {
    LOG_SENDER.get()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // CRITICAL: Initialize logger BEFORE anything else (especially before shoes is loaded)
    // This must be the first thing that happens to capture all logs including shoes logs
    early_init_logger();

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .manage(DatabaseState {
            db: Default::default(),
        });
    let builder = builder.manage(ProcessManagerState::default());
    let builder = builder
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(init_setup);
    let handler: fn(Invoke) -> bool = generate_handler![
        // Proxy commands
        proxy_api::get_all_proxies,
        proxy_api::get_all_hysterias,
        proxy_api::get_all_xrays,
        proxy_api::add_hysteria_item,
        proxy_api::add_xray_item,
        proxy_api::update_hysteria_item,
        proxy_api::update_xray_item,
        proxy_api::delete_hysteria_item,
        proxy_api::delete_xray_item,
        proxy_api::get_hysteria_by_id,
        proxy_api::get_xray_by_id,
        proxy_api::batch_get_subscriptions,
        proxy_api::refresh_subscriptions,
        proxy_api::import_subscription,
        proxy_api::refresh_xray_subscription,
        proxy_api::import_xray_subscribe,
        proxy_api::proxies_delay_test,
        // Subscription commands
        crate::tauri_apis::subscription::get_all_subscriptions,
        crate::tauri_apis::subscription::create_subscription,
        crate::tauri_apis::subscription::update_subscription,
        crate::tauri_apis::subscription::delete_subscription,
        crate::tauri_apis::subscription::switch_subscription,
        crate::tauri_apis::subscription::refresh_subscription,
        // Common commands
        common_api::query_base_config,
        common_api::update_base_config,
        common_api::copy_proxy_env_cmd,
        common_api::query_rules,
        common_api::delete_rules,
        common_api::add_rules,
        common_api::update_rules_item,
        common_api::export_rules,
        common_api::import_rules,
        common_api::test_current_proxy,
        common_api::get_log_level,
        common_api::set_log_level,
        // Server commands
        server_api::start_proxy_server,
        server_api::stop_proxy_server,
        server_api::is_proxy_server_running,
        // State commands
        start_all_servers,
        set_system_proxy_only,
        stop_system_proxy,
        tauri_apis::get_active_proxy,
        tauri_apis::switch_to_proxy,
    ];

    let builder = builder.invoke_handler(handler);
    builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            if let RunEvent::Exit = event {
                // clear_command(app);
                on_exit_clear_commands(app);
            }
        });
}
