use anyhow::Result;
use reqwest;
use std::path::{Path, PathBuf};
use std::{env};

fn get_file_path(file_name: &str) -> PathBuf {
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");
    let project_dir = Path::new(&project_dir);
    let static_dir = project_dir.join("static");
    if !static_dir.exists() {
        let _ = std::fs::create_dir_all(&static_dir);
    }
    static_dir.join(file_name)
}

fn download_geo_file(source_name: &str, file_name: &str) -> Result<()> {
    let geo_file_path = get_file_path(file_name);

    // 如果文件已存在且大小大于0，跳过下载
    if geo_file_path.exists() {
        let metadata = std::fs::metadata(&geo_file_path)?;
        if metadata.len() > 0 {
            println!("File {} already exists, skipping download", file_name);
            return Ok(());
        }
    }

    let url = format!(
        "https://github.com/Loyalsoldier/v2ray-rules-dat/releases/latest/download/{source_name}"
    );
    println!("Downloading {} from {}", source_name, url);

    // 使用带超时的客户端
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()?;

    let response = client.get(&url).send()?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to download {}: HTTP {}", source_name, response.status()));
    }

    let bytes = response.bytes()?;
    std::fs::write(&geo_file_path, &bytes)?;

    println!("Successfully downloaded {} to {:?} ({} bytes)", source_name, geo_file_path, bytes.len());
    Ok(())
}

fn main() {
    println!("=== Checking geo files ===");

    if let Err(e) = download_geo_file("geoip.dat", "kitty_geoip.dat") {
        println!("Failed to download geoip.dat: {}", e);
    }

    if let Err(e) = download_geo_file("geosite.dat", "kitty_geosite.dat") {
        println!("Failed to download geosite.dat: {}", e);
    }

    println!("=== Done ===");
    tauri_build::build()
}
