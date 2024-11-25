use anyhow::{anyhow, Result};
use log::debug;
use serde::Serialize;
use shared_child::SharedChild;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::net::SocketAddr;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::{thread, time};
use uuid::Uuid;

use crate::types::CheckStatusCommandPipe;
use crate::utils::socket_addr_busy;

#[derive(Debug)]
pub struct KittyCommand {
    bin_path: PathBuf,
    child: Arc<SharedChild>,
    config_path: PathBuf,
    env_mapping: HashMap<String, String>,
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
            .stderr(Stdio::piped());
        #[cfg(target_os = "windows")]
        let command = command.creation_flags(0x08000000);
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

    pub fn check_socket_addrs(&self, socket_addrs: Vec<SocketAddr>) -> Result<()> {
        for socket_addr in socket_addrs {
            let res = socket_addr_busy(socket_addr);
            if !res {
                return Err(anyhow!(anyhow!(
                    "check_socket_addrs failed, process start failed!"
                )));
            }
        }
        Ok(())
    }

    pub fn check_status(
        &self,
        started_str: &str,
        std_pipe: CheckStatusCommandPipe,
        socket_addrs: Vec<SocketAddr>,
    ) -> Result<()> {
        let child_clone = self.child.clone();
        if let Ok(None) = child_clone.try_wait() {
            match std_pipe {
                CheckStatusCommandPipe::StdErr => {
                    let pipe_out = &mut child_clone.take_stderr();
                    if let Some(pipe_out) = pipe_out {
                        let reader = io::BufReader::new(pipe_out);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                debug!("stderr: {}", line);
                                if line.to_lowercase().contains(&started_str.to_lowercase()) {
                                    thread::sleep(time::Duration::from_millis(500));
                                    self.check_socket_addrs(socket_addrs)?;
                                    return Ok(());
                                }
                            }
                        }
                        return Ok(());
                    }
                }
                CheckStatusCommandPipe::StdOut => {
                    let pipe_out = &mut child_clone.take_stdout();
                    if let Some(pipe_out) = pipe_out {
                        let reader = io::BufReader::new(pipe_out);
                        for line in reader.lines() {
                            if let Ok(line) = line {
                                debug!("stdout: {}", line);
                                if line.to_lowercase().contains(&started_str.to_lowercase()) {
                                    thread::sleep(time::Duration::from_millis(500));
                                    self.check_socket_addrs(socket_addrs)?;
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(anyhow!("process start failed!"))
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
