use anyhow::{anyhow, Context, Ok, Result};
use build_target::{Arch, Os, Target};
use reqwest;
use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, io};
use zip::ZipArchive;

fn get_binary_file_path(file_name: &str) -> PathBuf {
    let project_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");
    let project_dir = Path::new(&project_dir);
    let binaries_path = project_dir.join("binaries").join(file_name);
    binaries_path
}

fn download_file(url: &str, file_name: &str) -> PathBuf {
    let mut file =
        reqwest::blocking::get(url).expect(format!("download {} failed!", file_name).as_str());

    let binaries_path = get_binary_file_path(file_name);
    let mut out = File::create(&binaries_path).expect("failed to create file");
    io::copy(&mut file, &mut out).expect("failed to copy content");
    binaries_path
}

fn download_file_from_zip(
    url: &str,
    save_file_name: &str,
    extract_target_file: &str,
) -> Result<PathBuf> {
    let mut response =
        reqwest::blocking::get(url).expect(format!("download {} failed!", save_file_name).as_str());

    let mut buffer: Vec<u8> = vec![];
    response.read_to_end(&mut buffer).unwrap();

    let mut archive = ZipArchive::new(std::io::Cursor::new(buffer)).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let file_path = file.mangled_name();
        let target_zip_file_name = file_path.file_name().unwrap().to_str().unwrap();
        if target_zip_file_name == extract_target_file {
            let binaries_path = get_binary_file_path(save_file_name);
            let mut extracted_file = std::fs::File::create(&binaries_path).unwrap();
            std::io::copy(&mut file, &mut extracted_file).unwrap();
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
            let os_name = "linux";
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
            let os_name = "linux";
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
    let target_name = format!("hysteria-{}", target.triple);
    let binaries_path = download_file(download_url.as_str(), target_name.as_str());
    let mut perms = fs::metadata(&binaries_path)
        .expect("metadata failed.")
        .permissions();
    perms.set_mode(0o755);
    let _ = fs::set_permissions(binaries_path, perms);
}

fn download_xray() -> Result<()> {
    let target = build_target::target().unwrap();
    let source_name = get_xray_source_name(&target);
    let download_url =
        format!("https://github.com/XTLS/Xray-core/releases/download/v1.8.6/{source_name}");
    let target_name = format!("xray-{}", target.triple);
    let extract_target_file = "xray";
    let binaries_path = download_file_from_zip(
        download_url.as_str(),
        target_name.as_str(),
        extract_target_file,
    )?;
    let mut perms = fs::metadata(&binaries_path)
        .expect("metadata failed.")
        .permissions();
    perms.set_mode(0o755);
    let _ = fs::set_permissions(binaries_path, perms);
    Ok(())
}

fn download_binaries() -> Result<()> {
    let hysteria_feature = env::var("CARGO_FEATURE_HYSTERIA").is_ok();
    let xray_feature = env::var("CARGO_FEATURE_XRAY").is_ok();
    if !hysteria_feature && !xray_feature {
        let _ = download_hysteria();
        let _ = download_xray()?;
    }
    Ok(())
}

fn main() {
    let _ = download_binaries();
    tauri_build::build()
}
