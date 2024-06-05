use anyhow::{anyhow, Result};
use build_target::{Arch, Os, Target};
use reqwest;
use std::{env, io};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

fn set_execute_permission(binaries_path: &PathBuf) {
    #[cfg(target_family = "unix")]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&binaries_path)
            .expect("metadata failed.")
            .permissions();
        perms.set_mode(0o755);
        let _ = fs::set_permissions(binaries_path, perms);
    }
    #[cfg(not(target_family = "unix"))]
    {}
}

enum FileEnum {
    Binary,
    Static,
}

fn get_file_path(file_name: &str, file_enum: FileEnum) -> PathBuf {
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");
    let project_dir = Path::new(&project_dir);
    let folder_name = match file_enum {
        FileEnum::Binary => "binaries",
        FileEnum::Static => "static",
    };
    let binaries_dir = project_dir.join(folder_name);
    if !binaries_dir.exists() {
        fs::create_dir_all(&binaries_dir).expect("create_dir_all failed!");
    }
    let binaries_path = binaries_dir.join(file_name);
    binaries_path
}

fn download_file(url: &str, file_name: &str) -> PathBuf {
    let response =
        reqwest::blocking::get(url).expect(format!("download {} failed!", file_name).as_str());

    let binaries_path = get_file_path(file_name, FileEnum::Binary);
    let mut out = File::create(&binaries_path).expect("failed to create file");
    let content = response.bytes().expect("read response failed.");
    out.write_all(&content).expect("write binary file failed.");
    binaries_path
}

fn download_file_from_zip(
    target: &Target,
    url: &str,
    save_file_name: &str,
    extract_target_file: &str,
) -> Result<PathBuf> {
    let response = reqwest::blocking::get(url);
    let mut response = match response {
        Ok(response) => response,
        Err(_) => panic!("download xray failed."),
    };
    println!("download xray ing.");
    let mut buffer: Vec<u8> = vec![];
    response.read_to_end(&mut buffer).unwrap();

    let mut archive = ZipArchive::new(std::io::Cursor::new(buffer)).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let file_path = file.mangled_name();
        eprintln!("unzip file_path : {:?}", file_path);
        let target_zip_file_name = file_path.file_name().unwrap().to_str().unwrap();
        eprintln!("download target_zip_file_name: {:?}", target_zip_file_name);
        eprintln!("download extract_target_file: {:?}", extract_target_file);
        let extract_target_file = match target.os {
            Os::Windows => extract_target_file.to_string() + ".exe",
            _ => extract_target_file.to_string(),
        };
        if target_zip_file_name == extract_target_file.as_str() {
            let binaries_path = get_file_path(save_file_name, FileEnum::Binary);
            let mut extracted_file = File::create(&binaries_path).unwrap();
            std::io::copy(&mut file, &mut extracted_file).unwrap();
            eprintln!("download success: {:?}", binaries_path);
            return Ok(binaries_path);
        }
    }
    Err(anyhow!("down zip not match"))
}

fn get_hysteria_source_name(target: &Target) -> String {
    let source_name = match target.os {
        Os::Windows => {
            let os_name = "windows";
            let arch = match target.arch {
                Arch::X86_64 => "amd64",
                Arch::X86 => "386",
                Arch::AARCH64 => "arm64",
                _ => panic!("Not support this arch."),
            };
            format!("hysteria-{}-{}.exe", os_name, arch)
        }
        Os::Linux => {
            let os_name = "linux";
            let arch = match target.arch {
                Arch::X86_64 => "amd64",
                Arch::X86 => "386",
                Arch::ARM => "arm",
                Arch::THUMBV6 => "armv5",
                Arch::AARCH64 => "arm64",
                Arch::S390X => "s390x",
                Arch::MIPS => "mipsle",
                Arch::RISCV => "riscv64",
                _ => panic!("Not support this arch."),
            };
            format!("hysteria-{}-{}", os_name, arch)
        }
        Os::MacOs => {
            let os_name = "darwin";
            let arch = match target.arch {
                Arch::X86_64 => "amd64",
                Arch::AARCH64 => "arm64",
                _ => panic!("Not support this arch."),
            };
            format!("hysteria-{}-{}", os_name, arch)
        }
        _ => panic!("Not support this system."),
    };
    source_name
}

fn get_xray_source_name(target: &Target) -> String {
    let source_name = match target.os {
        Os::Windows => {
            let os_name = "windows";
            let arch = match target.arch {
                Arch::X86_64 => "64",
                Arch::X86 => "32",
                Arch::AARCH64 => "arm64",
                _ => panic!("Not support this arch."),
            };
            format!("Xray-{}-{}.zip", os_name, arch)
        }
        Os::Linux => {
            let os_name = "linux";
            let arch = match target.arch {
                Arch::X86_64 => "64",
                Arch::X86 => "32",
                Arch::ARM => "arm",
                Arch::THUMBV6 => "arm32-v5",
                Arch::AARCH64 => "arm64-v8a",
                Arch::S390X => "s390x",
                Arch::MIPS => "mips32",
                Arch::RISCV => "riscv64",
                _ => panic!("Not support this arch."),
            };
            format!("Xray-{}-{}.zip", os_name, arch)
        }
        Os::MacOs => {
            let os_name = "macos";
            let arch = match target.arch {
                Arch::X86_64 => "64",
                Arch::AARCH64 => "arm64-v8a",
                _ => panic!("Not support this arch."),
            };
            format!("Xray-{}-{}.zip", os_name, arch)
        }
        _ => panic!("Not support this system."),
    };
    source_name
}

fn download_hysteria() {
    let target = build_target::target().unwrap();
    let source_name = get_hysteria_source_name(&target);
    let download_url = format!("https://download.hysteria.network/app/latest/{source_name}");
    println!("download url: {}", download_url);
    let suffix = match target.os {
        Os::Windows => ".exe",
        _ => "",
    };
    let target_name = format!("hysteria-{}{}", target.triple, suffix);
    let binaries_path = download_file(download_url.as_str(), target_name.as_str());
    set_execute_permission(&binaries_path);
}

fn download_xray() -> Result<()> {
    let target = build_target::target().unwrap();
    let source_name = get_xray_source_name(&target);
    let download_url =
        format!("https://github.com/XTLS/Xray-core/releases/download/v1.8.7/{source_name}");
    println!("download url: {}", download_url);
    let suffix = match target.os {
        Os::Windows => ".exe",
        _ => "",
    };
    let target_name = format!("xray-{}{}", target.triple, suffix);
    eprintln!("Debug message: This is a debug print.");

    let extract_target_file = "xray";
    let binaries_path = download_file_from_zip(
        &target,
        download_url.as_str(),
        target_name.as_str(),
        extract_target_file,
    )?;
    eprintln!("Debug message: This is a debug print1.");
    set_execute_permission(&binaries_path);
    Ok(())
}

fn download_binaries() -> Result<()> {
    let hysteria_feature = env::var("CARGO_FEATURE_HYSTERIA").is_ok();
    let xray_feature = env::var("CARGO_FEATURE_XRAY").is_ok();
    if !hysteria_feature && !xray_feature {
        let _ = download_hysteria();
        let _ = download_xray()?;
    }
    let _ = download_hysteria();
    let _ = download_xray()?;
    Ok(())
}

fn download_geo_file(source_name: &str, file_name: &str) -> Result<()> {
    let url = format!(
        "https://github.com/Loyalsoldier/v2ray-rules-dat/releases/latest/download/{source_name}"
    );
    let mut geo_file = reqwest::blocking::get(url).expect("download geoip.dat failed!");
    let geo_file_path = get_file_path(file_name, FileEnum::Static);
    println!("geo_file_path: {:?}", geo_file_path);
    let mut out = File::create(&geo_file_path).expect("failed to create file");
    io::copy(&mut geo_file, &mut out).expect("failed to copy content");
    Ok(())
}

fn create_empty_geo_file(file_name: &str) -> Result<()> {
    let geo_file_path = get_file_path(file_name, FileEnum::Static);
    let _ = File::create(&geo_file_path)?;
    Ok(())
}

fn main() {
    // let _ = download_binaries();
    // let _ = download_geo_file("geoip.dat", "kitty_geoip.dat");
    // let _ = download_geo_file("geosite.dat", "kitty_geosite.dat");
    tauri_build::build()
}
