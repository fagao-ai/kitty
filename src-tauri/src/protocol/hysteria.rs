use crate::proxy::system_proxy::set_system_proxy;
use std::fs;
use std::fs::File;
use std::io::Write;

use crate::protocol::traits::CommandManagerTrait;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use std::borrow::BorrowMut;
use tauri::AppHandle;
use tauri::{async_runtime::Receiver, Manager};
use tauri_plugin_shell::{process::CommandChild, process::CommandEvent, ShellExt};

pub struct HysteriaManager {
    name: String,
    child: Option<CommandChild>,
    child_receiver: Option<Receiver<CommandEvent>>,
}

impl HysteriaManager {
    pub fn new() -> Self {
        Self {
            name: "hysteria".into(),
            child: None,
            child_receiver: None,
        }
    }
}

#[async_trait]
impl CommandManagerTrait for HysteriaManager {
    fn start_backend(&mut self, app_handle: AppHandle, config_content: &str) -> Result<()> {
        let res = match self.child.borrow_mut() {
            Some(_) => Err(anyhow!(format!("{} already started.", self.name))),
            None => {
                let app_cache_dir = app_handle.path().app_cache_dir()?;
                if !app_cache_dir.exists() {
                    fs::create_dir_all(&app_cache_dir).unwrap();
                }
                println!("app_tmp_dir: {:?}", app_cache_dir);
                let config_path = app_cache_dir.join("hysteria_config.json");
                let mut file = File::create(&config_path).expect("failed to create file");
                file.write_all(config_content.as_bytes());
                let err_msg = format!("failed to create `{}` binary command ", self.name);
                let t_command = app_handle
                    .shell()
                    .sidecar(self.name.as_str())
                    .expect(err_msg.as_str());
                let (child_receiver, child) = t_command
                    .args(["client", "--config"])
                    .arg(config_path.as_os_str())
                    .spawn()?;
                self.child = Some(child);
                self.child_receiver = Some(child_receiver);
                Ok(())
            }
        };
        res
    }

    fn terminate_backend(&mut self) -> Result<()> {
        if let Some(child) = self.child.take() {
            child.kill()?;
            self.child = None;
        }
        Ok(())
    }

    fn restart_backend(&mut self, app_handle: AppHandle, config_content: &str) -> Result<()> {
        let _terminate_result = self.terminate_backend()?;
        self.start_backend(app_handle, config_content)?;
        Ok(())
    }

    async fn check_status(&mut self) -> Result<()> {
        let receive_unwrap = std::mem::replace(&mut self.child_receiver, None);
        let mut receiver = receive_unwrap.unwrap();
        while let Some(event) = receiver.recv().await {
            match event {
                CommandEvent::Terminated(_payload) => {}
                CommandEvent::Stderr(line) => {
                    let line = String::from_utf8(line).unwrap();
                    if line.contains("server listening") {
                        let _ = set_system_proxy("127.0.0.1", 10086, Some(10087))?;
                        print!("stderr: {}", line);
                        break;
                    }
                    print!("stderr: {}", line);
                }
                CommandEvent::Stdout(line) => {
                    print!("stdout: {}", String::from_utf8(line).unwrap());
                }
                _ => {}
            }
        }
        println!("started hysteria!!!");
        tauri::async_runtime::spawn(async move {
            while let Some(event) = receiver.recv().await {
                match event {
                    CommandEvent::Terminated(payload) => {
                        println!("stop hysteria!!");
                        println!("{:?}", payload);
                    }
                    _ => {}
                }
            }
        });
        Ok(())
    }

    fn is_open(&self) -> bool {
        let res = match self.child {
            Some(_) => true,
            None => false,
        };
        res
    }
}
