use std::collections::HashMap;
use std::fs::{self, File};
use std::hash::Hash;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use serde::Serialize;
use shared_child::SharedChild;
use std::process::{Command, Stdio};
use std::sync::Arc;
use uuid::Uuid;

pub struct KittyCommand {
    bin_path: PathBuf,
    child: Arc<SharedChild>,
    config_path: PathBuf,
    env_mapping: HashMap<String, String>,
}


impl Drop for KittyCommand {
    fn drop(&mut self) {
        println!("Executing extra code before dropping XrayManager");
        if self.config_path.exists() {
            fs::remove_file(self.config_path.clone()).expect("config_path remove failed.");
        }
    }
}

impl KittyCommand {
    pub fn spawn<T>(bin_path: &PathBuf, config: T, config_dir: &PathBuf, env_mapping: HashMap<String, String>) -> Result<KittyCommand>
        where
            T: Serialize,
    {
        let config_content = serde_json::to_string(&config)?;
        let binary_name = bin_path.file_name().unwrap().to_str().unwrap();
        let config_path = config_dir.join(format!("{binary_name}_{}.json", Uuid::new_v4()));
        let mut file = File::create(&config_path)?;
        file.write_all(config_content.as_bytes())?;
        let command_str = bin_path.as_os_str();
        let mut command = Command::new(command_str);
        let command = command.args(["run", "-c"]);
        let command = command
            .arg(config_path.as_os_str())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (env_key, env_value) in env_mapping.iter() {
            std::env::set_var(env_key, env_value);
        }

        let share_child = SharedChild::spawn(command)?;
        let child_arc = Arc::new(share_child);
        Ok(Self {
            bin_path: bin_path.to_owned(),
            child: child_arc,
            config_path,
            env_mapping,
        })
    }


    pub fn check_status(&self) -> Result<()> {
        let child_clone = self.child.clone();
        if let Ok(None) = child_clone.try_wait() {
            let std_out = &mut child_clone.take_stdout();
            if let Some(stdout) = std_out {
                let reader = io::BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        println!("stdout: {line}");
                        if line.contains("Reading config:") {
                            return Ok(());
                        }
                    }
                }
            }
        }
        Err(anyhow!("xray start failed!"))
    }

    pub fn terminate_backend(&mut self) -> Result<()> {
        self.child.kill()?;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        if let Ok(None) = self.child.try_wait() {
            return true;
        }
        false
    }
}

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;
//     use std::thread;
//     use std::time::Duration;
//
//     use serde_json::Value;
//
//     use super::*;
//
//     #[test]
//     fn test_it_work() {
//         let mut env_map = HashMap::new();
//         env_map.insert(
//             "XRAY_LOCATION_ASSET".to_string(),
//             "E:\\opdensource\\kitty\\src-tauri\\static".to_string(),
//         );
//         let mut xray = XrayCommand::new(
//             PathBuf::from_str(
//                 "E:\\opdensource\\kitty\\src-tauri\\binaries\\xray-x86_64-pc-windows-msvc.exe",
//             )
//                 .unwrap(),
//             env_map,
//         );
//         let config = r#""#;
//         let config: Value = serde_json::from_str(config).unwrap();
//         let config_dir = PathBuf::from_str("E:\\opdensource\\kitty\\src-tauri\\binaries").unwrap();
//         xray.start_backend(config, config_dir).unwrap();
//         assert_eq!(xray.check_status().unwrap(), ());
//         thread::sleep(Duration::from_secs(10));
//         assert_eq!(xray.is_running(), true);
//         xray.restart_backend().unwrap();
//         assert_eq!(xray.is_running(), true);
//         xray.terminate_backend().unwrap();
//     }
// }
