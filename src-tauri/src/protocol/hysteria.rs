use crate::proxy::system_proxy::set_system_proxy;
use std::ffi::OsStr;

use crate::protocol::traits::CommandManagerTrait;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use std::borrow::BorrowMut;
use tauri::async_runtime::Receiver;
use tauri::AppHandle;
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
    fn start_backend<I, S>(&mut self, app_handle: AppHandle, args: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let res = match self.child.borrow_mut() {
            Some(_) => Err(anyhow!(format!("{} already started.", self.name))),
            None => {
                let err_msg = format!("failed to create `{}` binary command ", self.name);
                let t_command = app_handle
                    .shell()
                    .sidecar(self.name.as_str())
                    .expect(err_msg.as_str());
                let (child_receiver, child) = t_command.args(args).spawn()?;

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

    fn restart_backend<I, S>(&mut self, app_handle: AppHandle, args: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let _terminate_result = self.terminate_backend()?;
        self.start_backend(app_handle, args)?;
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
