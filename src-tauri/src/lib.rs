mod hysteria;
use hysteria::HyConfig;
use hysteria_rs::start_from_json;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![start_hy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
