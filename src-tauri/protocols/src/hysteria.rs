use std::borrow::Borrow;
use std::cell::RefCell;
// use crate::proxy::system_proxy::set_system_proxy;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::rc::Rc;

use crate::traits::CommandManagerTrait;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::process::{Child, Command, Stdio};
use uuid::Uuid;

pub struct HysteriaManager {
    name: String,
    child: Option<Rc<RefCell<Child>>>,
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

// #[async_trait]
impl CommandManagerTrait for HysteriaManager {
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
        let child = command
            .arg(&config_file_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        self.child = Some(Rc::new(RefCell::new(child)));
        self.config_path = Some(config_file_path);
        Ok(())
    }

    fn check_status(&mut self) -> Result<()> {
        match self.child.as_mut() {
            Some(child) => match child.borrow_mut().try_wait() {
                Ok(None) => {
                    let std_err = &mut child.borrow_mut().stderr;
                    if let Some(stderr) = std_err.take() {
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
                _ => {}
            },

            None => {}
        }

        Err(anyhow!("{} start failed!", self.name))
    }

    fn terminate_backend(&mut self) -> Result<()> {
        if let Some(child) = self.child.borrow() {
            child.borrow_mut().kill()?;
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

    fn is_runing(&self) -> bool {
        if let Some(child) = self.child.borrow() {
            match child.borrow_mut().try_wait() {
                Ok(None) => true,
                _ => false,
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde_json::Value;

    use super::*;
    #[test]
    fn test_it_work() {
        let mut hy = HysteriaManager::new();
        let mut command = Command::new("E:\\opdensource\\kitty\\src-tauri\\binaries\\hysteria-x86_64-pc-windows-msvc.exe");
        let config = "";
        let config: Value = serde_json::from_str(config).unwrap();
        let config_dir = PathBuf::from_str("E:\\opdensource\\kitty\\src-tauri\\binaries").unwrap();
        hy.start_backend(&mut command, config, &config_dir).unwrap();
        hy.check_status().unwrap();
    }
}
