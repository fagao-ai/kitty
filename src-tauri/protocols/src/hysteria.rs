use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::PathBuf;

use crate::traits::CommandManagerTrait;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use shared_child::SharedChild;
use std::process::{Command, Stdio};
use std::sync::Arc;
use uuid::Uuid;

pub struct HysteriaManager {
    name: String,
    child: Option<Arc<SharedChild>>,
    config_path: Option<PathBuf>,
}

impl HysteriaManager {
    pub fn new() -> Self {
        Self {
            name: "hysteria".into(),
            child: None,
            config_path: None,
        }
    }
}

impl HysteriaManager {
    fn start_backend<T>(
        &mut self,
        init_command: &mut Command,
        config: T,
        config_dir: &PathBuf,
    ) -> Result<()>
        where
            T: Serialize,
    {
        let command = init_command.args(["client", "--config"]);
        let config_content = serde_json::to_string(&config)?;
        let config_file_path = config_dir.join(format!("{}_{}.json", self.name, Uuid::new_v4()));
        let mut file = File::create(&config_file_path)?;
        file.write_all(config_content.as_bytes())?;
        let command = command
            .arg(&config_file_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let share_child = SharedChild::spawn(command)?;
        let child_arc = Arc::new(share_child);
        self.child = Some(child_arc);
        self.config_path = Some(config_file_path);
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

    // fn restart_backend(&mut self) -> Result<()> {
    //     if self.is_runing() {
    //         let _terminate_result = self.terminate_backend()?;
    //         let command = Command::new()
    //             .args(["client", "--config"])
    //             .arg(self.config_path.unwrap());

    //         Ok(())
    //     } else {
    //         return Err(anyhow!("{} not be runing!", self.name));
    //     }
    // }

    fn restart_backend(&self) -> Result<()> {
        Ok(())
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
        let mut hy = HysteriaManager::new();
        let mut command = Command::new(
            "/Users/hezhaozhao/myself/kitty/src-tauri/binaries/hysteria-aarch64-apple-darwin",
        );
        let config = r#"{
  "server": "155.248.218.187:10086",
  "auth": "Hzz19951218?",
  "bandwidth": {
    "up": "10 mbps",
    "down": "100 mbps"
  },
  "tls": {
    "sni": "bing.com",
    "insecure": true
  },
  "socks5": {
    "listen": "127.0.0.1:1080"
  },
  "http": {
    "listen": "127.0.0.1:8080"
  }
}
"#;
        let config: Value = serde_json::from_str(config).unwrap();
        let config_dir =
            PathBuf::from_str("/Users/hezhaozhao/myself/kitty/src-tauri/binaries").unwrap();
        hy.start_backend(&mut command, config, &config_dir).unwrap();
        assert_eq!(hy.check_status().unwrap(), ());
        thread::sleep(Duration::from_secs(10));
        hy.terminate_backend().unwrap();
    }
}
