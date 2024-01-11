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

pub struct HysteriaManager {
    name: String,
    bin_path: PathBuf,
    child: Option<Arc<SharedChild>>,
    config_path: Option<PathBuf>,
}

impl HysteriaManager {
    pub fn new(bin_path: PathBuf) -> Self {
        Self {
            name: "hysteria".into(),
            child: None,
            bin_path,
            config_path: None,
        }
    }
}

impl Drop for HysteriaManager {
    fn drop(&mut self) {
        println!("Executing extra code before dropping HysteriaManager");
        let config_path_clone = self.config_path.clone();
        if let Some(config_path) = config_path_clone {
            if config_path.exists() {
                fs::remove_file(config_path).expect("config_path remove failed.");
            }
        }
    }
}

impl CommandManagerTrait for HysteriaManager {
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
        let command = command.args(["client", "--config"]);
        let command = command
            .arg(config_path.as_os_str())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
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
                let std_err = &mut child.take_stderr();
                if let Some(stderr) = std_err {
                    let reader = io::BufReader::new(stderr);
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            println!("{line}");
                            if line.contains("server listening") {
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
        let mut hy = HysteriaManager::new(
            PathBuf::from_str(
                "E:\\opdensource\\kitty\\src-tauri\\binaries\\hysteria-x86_64-pc-windows-msvc.exe",
            )
            .unwrap(),
        );
        let config = r#""#;
        let config: Value = serde_json::from_str(config).unwrap();
        let config_dir = PathBuf::from_str("E:\\opdensource\\kitty\\src-tauri\\binaries").unwrap();
        hy.start_backend(config, config_dir).unwrap();
        assert_eq!(hy.check_status().unwrap(), ());
        thread::sleep(Duration::from_secs(10));
        assert_eq!(hy.is_running(), true);
        hy.restart_backend().unwrap();
        assert_eq!(hy.is_running(), true);
        hy.terminate_backend().unwrap();
    }
}
