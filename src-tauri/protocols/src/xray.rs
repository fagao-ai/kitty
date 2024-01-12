use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::io::{self, BufRead};
use std::path::PathBuf;

use crate::traits::CommandManagerTrait;
use anyhow::{anyhow, Result};
use serde::Serialize;
use serde_json::Value;
use shared_child::SharedChild;
use std::process::{Command, Stdio};
use std::sync::Arc;
use uuid::Uuid;

pub struct XrayManager {
    name: String,
    bin_path: PathBuf,
    child: Option<Arc<SharedChild>>,
    config_path: Option<PathBuf>,
    env_mapping: HashMap<String, String>,
}

impl XrayManager {
    pub fn new(bin_path: PathBuf, env_mapping: HashMap<String, String>) -> Self {
        Self {
            name: "xray".into(),
            child: None,
            bin_path,
            config_path: None,
            env_mapping,
        }
    }
}

impl Drop for XrayManager {
    fn drop(&mut self) {
        println!("Executing extra code before dropping XrayManager");
        let config_path_clone = self.config_path.clone();
        if let Some(config_path) = config_path_clone {
            if config_path.exists() {
                fs::remove_file(config_path).expect("config_path remove failed.");
            }
        }
    }
}

impl CommandManagerTrait for XrayManager {
    fn start_backend<T>(&mut self, config: T, config_dir: PathBuf) -> Result<()>
    where
        T: Serialize,
    {
        let config_content = serde_json::to_string(&config)?;
        let config_file_path = config_dir.join(format!("{}_{}.json", self.name, Uuid::new_v4()));
        let mut file = File::create(&config_file_path)?;
        file.write_all(config_content.as_bytes())?;
        self.start_backend_from_path(config_file_path)?;
        Ok(())
    }

    fn start_backend_from_path(&mut self, config_path: PathBuf) -> Result<()> {
        let command_str = self.bin_path.as_os_str();
        let mut command = Command::new(command_str);
        let command = command.args(["run", "-c"]);
        let command = command
            .arg(config_path.as_os_str())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (env_key, env_value) in self.env_mapping.iter() {
            std::env::set_var(env_key, env_value);
        }

        let share_child = SharedChild::spawn(command)?;
        let child_arc = Arc::new(share_child);
        self.child = Some(child_arc);
        println!("config_file_path: {:?}", config_path);
        self.config_path = Some(config_path);
        Ok(())
    }

    fn check_status(&self) -> Result<()> {
        let child_clone = self.child.clone();
        if let Some(child) = child_clone {
            if let Ok(None) = child.try_wait() {
                let std_out = &mut child.take_stdout();
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
        }
        Err(anyhow!("{} start failed!", self.name))
    }

    fn terminate_backend(&mut self) -> Result<()> {
        let child_clone = self.child.clone();
        if let Some(child) = child_clone {
            child.kill()?;
            self.child = None;
        }
        Ok(())
    }

    fn restart_backend(&mut self) -> Result<()> {
        if self.is_running() {
            let _terminate_result = self.terminate_backend()?;
            let config_path_clone = self.config_path.clone();
            if let Some(config_path) = config_path_clone {
                let file = File::open(&config_path)?;
                let reader: io::BufReader<File> = io::BufReader::new(file);
                let config: Value = serde_json::from_reader(reader)?;
                println!("config: {:?}", config);
                let _ = self.start_backend_from_path(config_path)?;
            }
            Ok(())
        } else {
            return Err(anyhow!("{} not be runing!", self.name));
        }
    }

    fn is_running(&self) -> bool {
        let child_clone = self.child.clone();
        if let Some(child) = child_clone {
            if let Ok(None) = child.try_wait() {
                return true;
            }
        }
        false
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::thread;
    use std::time::Duration;

    use serde_json::Value;

    use super::*;

    #[test]
    fn test_it_work() {
        let mut env_map = HashMap::new();
        env_map.insert(
            "XRAY_LOCATION_ASSET".to_string(),
            "E:\\opdensource\\kitty\\src-tauri\\static".to_string(),
        );
        let mut xray = XrayManager::new(
            PathBuf::from_str(
                "E:\\opdensource\\kitty\\src-tauri\\binaries\\xray-x86_64-pc-windows-msvc.exe",
            )
            .unwrap(),
            env_map,
        );
        let config = r#""#;
        let config: Value = serde_json::from_str(config).unwrap();
        let config_dir = PathBuf::from_str("E:\\opdensource\\kitty\\src-tauri\\binaries").unwrap();
        xray.start_backend(config, config_dir).unwrap();
        assert_eq!(xray.check_status().unwrap(), ());
        thread::sleep(Duration::from_secs(10));
        assert_eq!(xray.is_running(), true);
        xray.restart_backend().unwrap();
        assert_eq!(xray.is_running(), true);
        xray.terminate_backend().unwrap();
    }
}
