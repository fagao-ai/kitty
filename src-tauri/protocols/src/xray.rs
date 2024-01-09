use crate::proxy::system_proxy::set_system_proxy;
use std::ffi::OsStr;

use crate::protocol::traits::CommandManagerTrait;
use anyhow::{anyhow, Ok, Result};
use std::borrow::BorrowMut;
use tauri::async_runtime::Receiver;
use tauri::AppHandle;
use tauri_plugin_shell::{process::CommandChild, process::CommandEvent, ShellExt};

pub struct XrayManager {
    name: String,
    child: Option<CommandChild>,
    child_receiver: Option<Receiver<CommandEvent>>,
}

impl XrayManager {
    pub fn new() -> Self {
        Self {
            name: "xray".into(),
            child: None,
            child_receiver: None,
        }
    }
}

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
}
