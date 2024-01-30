use anyhow::{anyhow, Result};
use entity::utils::get_random_port;
use reqwest;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{self, Duration};
use tauri::utils::platform;
use tokio::task::JoinSet;

pub fn get_http_socks_ports(used_ports: &mut HashSet<u16>) -> (u16, u16) {
    let http_port = get_random_port(&used_ports).unwrap();
    let socks_port = get_random_port(&used_ports).unwrap();
    (http_port, socks_port)
}

pub fn relative_command_path(command: &Path) -> Result<PathBuf> {
    match platform::current_exe()?.parent() {
        #[cfg(windows)]
        Some(exe_dir) => Ok(exe_dir.join(command).with_extension("exe")),
        #[cfg(not(windows))]
        Some(exe_dir) => Ok(exe_dir.join(command)),
        None => Err(anyhow!("current exe not has parent.")),
    }
}

async fn request_test_url(port: u16, url: String) -> Result<(u16, Duration)> {
    let start_time = time::Instant::now();
    let proxy = reqwest::Proxy::http(format!("http://127.0.0.1:{port}"))?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .proxy(proxy)
        .build()?;
    let response = client.get(url).send().await;
    if response.is_err() {
        Ok((port, Duration::from_secs(3)))
    } else {
        let end_time = time::Instant::now();
        let delay = end_time - start_time;
        Ok((port, delay))
    }
}

pub async fn speed_delay(
    ports: Vec<u16>,
    test_url: Option<&str>,
) -> Result<HashMap<u16, Duration>> {
    let mut set = JoinSet::new();
    let url = test_url.unwrap_or("https://gstatic.com/generate_204");
    for port in ports.clone() {
        let url_clone = url.to_string().clone();
        set.spawn(async move { request_test_url(port, url_clone) });
    }
    let mut delay_dict: HashMap<u16, Duration> = ports
        .into_iter()
        .map(|x| (x, Duration::from_secs(3)))
        .collect();
    while let Some(res) = set.join_next().await {
        let aa = res?.await;
        if let Ok((port, delay)) = aa {
            delay_dict.insert(port, delay);
        }
    }
    Ok(delay_dict)
}
