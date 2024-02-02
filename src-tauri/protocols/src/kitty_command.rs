use anyhow::{anyhow, Result};
use serde::Serialize;
use shared_child::SharedChild;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;
use uuid::Uuid;
use std::os::windows::process::CommandExt;


use crate::types::CheckStatusCommandPipe;

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
    pub fn spawn<T>(
        bin_path: &PathBuf,
        command_args: [&str; 2],
        config: T,
        config_dir: &PathBuf,
        env_mapping: HashMap<String, String>,
    ) -> Result<KittyCommand>
    where
        T: Serialize,
    {
        let config_content = serde_json::to_string(&config)?;
        let binary_name = bin_path.file_name().unwrap().to_str().unwrap();
        let config_path = config_dir.join(format!("{binary_name}_{}.json", Uuid::new_v4()));
        let mut file = File::create(&config_path)?;
        file.write_all(config_content.as_bytes())?;
        let mut command = Command::new(bin_path);
        let command = command.args(command_args);
        let command = command
            .arg(config_path.as_os_str())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .creation_flags(0x08000000);
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

    pub fn check_status(&self, started_str: &str, std_pipe: CheckStatusCommandPipe) -> Result<()> {
        let child_clone = self.child.clone();
        if let Ok(None) = child_clone.try_wait() {
            match std_pipe {
                CheckStatusCommandPipe::StdErr => {
                    let pipe_out = &mut child_clone.take_stderr();
                    if let Some(pipe_out) = pipe_out {
                        let reader = io::BufReader::new(pipe_out);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                println!("stderr: {line}");
                                if line.to_lowercase().contains(&started_str.to_lowercase()) {
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
                CheckStatusCommandPipe::StdOut => {
                    let pipe_out = &mut child_clone.take_stdout();
                    if let Some(pipe_out) = pipe_out {
                        let reader = io::BufReader::new(pipe_out);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                println!("stdout: {line}");
                                if line.to_lowercase().contains(&started_str.to_lowercase()) {
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(anyhow!("xray start failed!"))
    }

    pub fn terminate_backend(&mut self) -> Result<()> {
        println!("kill command");
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
