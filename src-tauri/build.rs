use anyhow::{Context, Result};
use build_target::{Arch, Os};
use reqwest;
use std::fs::{self, File};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, io};

fn download_file(url: &str, file_name: &str) -> PathBuf {
    let mut file =
        reqwest::blocking::get(url).expect(format!("download {} failed!", file_name).as_str());

    let project_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");
    let project_dir = Path::new(&project_dir);
    let binaries_path = project_dir.join("binaries").join(file_name);
    let mut out = File::create(&binaries_path).expect("failed to create file");
    io::copy(&mut file, &mut out).expect("failed to copy content");
    binaries_path
}

fn download_hysteria() {
    let target = build_target::target().unwrap();
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
    let download_url = format!("https://download.hysteria.network/app/latest/{source_name}");

    let target_name = format!("hysteria-{}", target.triple);
    let binaries_path = download_file(download_url.as_str(), target_name.as_str());

    let mut perms = fs::metadata(&binaries_path)
        .expect("metadata failed.")
        .permissions();
    perms.set_mode(0o755);
    let _ = fs::set_permissions(binaries_path, perms);
}

fn download_xray() {}

fn download_binaries() {
    let hysteria_feature = env::var("CARGO_FEATURE_HYSTERIA").is_ok();
    let xray_feature = env::var("CARGO_FEATURE_XRAY").is_ok();
    if !hysteria_feature && !xray_feature {
        let _ = download_hysteria();
    }
}

fn main() {
    tauri_build::build()
}
