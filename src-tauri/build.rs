use git2::Repository;

use anyhow::{Context, Result};
use std::env;
use std::process::Command;

fn _build_hysteria() -> Result<()> {
    let repo_url = "https://github.com/apernet/hysteria.git";
    let repo_path = env::var("OUT_DIR").unwrap();
    let _repo = match Repository::clone(repo_url, &repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone repository: {}", e),
    };
    env::set_current_dir(&repo_path).unwrap();
    let _platform = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let mut command = Command::new("python");
    command
        .arg("hyperbole.py")
        .arg("build")
        .arg("-r")
        .output()
        .expect("Failed to execute complie hysteria faild.");
    let command_str = format!("{:?}", command);
    println!("command_str: {}", command_str);
    let command_output = command
        .spawn()
        .context("fail spawning hysteria build")?
        .wait_with_output()?;
    if !command_output.status.success() {
        let stdout = String::from_utf8_lossy(&command_output.stdout);
        println!("Command output:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&command_output.stderr);
        eprintln!("Command failed with error:\n{}", stderr);
    }

    Ok(())
}

fn main() {
    tauri_build::build()
}
