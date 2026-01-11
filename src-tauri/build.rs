use anyhow::Result;
use reqwest;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::{env};

enum FileEnum {
    Static,
}

fn get_file_path(file_name: &str, file_enum: FileEnum) -> PathBuf {
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");
    let project_dir = Path::new(&project_dir);
    let folder_name = match file_enum {
        FileEnum::Static => "static",
    };
    let static_dir = project_dir.join(folder_name);
    if !static_dir.exists() {
        let _ = std::fs::create_dir_all(&static_dir);
    }
    let static_path = static_dir.join(file_name);
    static_path
}

fn download_geo_file(source_name: &str, file_name: &str) -> Result<()> {
    let url = format!(
        "https://github.com/Loyalsoldier/v2ray-rules-dat/releases/latest/download/{source_name}"
    );
    let mut geo_file = reqwest::blocking::get(url)?;
    let geo_file_path = get_file_path(file_name, FileEnum::Static);
    println!("geo_file_path: {:?}", geo_file_path);
    let mut out = File::create(&geo_file_path)?;
    io::copy(&mut geo_file, &mut out)?;
    Ok(())
}

fn create_empty_geo_file(file_name: &str) -> Result<()> {
    let geo_file_path = get_file_path(file_name, FileEnum::Static);
    let _ = File::create(&geo_file_path)?;
    Ok(())
}

fn main() {
    // Download geo files for routing rules (skip if network fails)
    // let _ = download_geo_file("geoip.dat", "kitty_geoip.dat");
    // let _ = download_geo_file("geosite.dat", "kitty_geosite.dat");
    //
    // // Create empty files if download failed
    // if !get_file_path("kitty_geoip.dat", FileEnum::Static).exists() {
    //     let _ = create_empty_geo_file("kitty_geoip.dat");
    // }
    // if !get_file_path("kitty_geosite.dat", FileEnum::Static).exists() {
    //     let _ = create_empty_geo_file("kitty_geosite.dat");
    // }

    tauri_build::build()
}
