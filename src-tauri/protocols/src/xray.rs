use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::kitty_command::KittyCommand;
use crate::traits::KittyCommandGroupTrait;
use crate::types::CheckStatusCommandPipe;

#[derive(Debug)]
pub struct XrayCommandGroup {
    bin_path: PathBuf,
    kitty_commands: HashMap<String, KittyCommand>,
    config_dir: PathBuf,
}
impl XrayCommandGroup {
    pub fn new(bin_path: PathBuf, config_dir: PathBuf) -> Self {
        Self {
            kitty_commands: HashMap::new(),
            bin_path,
            config_dir,
        }
    }
}

impl Drop for XrayCommandGroup {
    fn drop(&mut self) {
        for (_, child) in self.kitty_commands.iter_mut() {
            if child.is_running() {
                child.terminate_backend().ok();
            }
        }
        self.kitty_commands.clear();
    }
}

impl KittyCommandGroupTrait for XrayCommandGroup {
    fn start_commands<T>(
        &mut self,
        config: HashMap<String, T>,
        env_mapping: Option<HashMap<String, String>>,
    ) -> Result<()>
    where
        T: Serialize,
    {
        for (node_server, config) in config.iter() {
            let kitty_command = KittyCommand::spawn(
                &self.bin_path,
                ["run", "-c"],
                config,
                &self.config_dir,
                env_mapping.clone().unwrap_or(HashMap::new()),
            )?;
            kitty_command.check_status("Reading config:", CheckStatusCommandPipe::StdOut)?;
            self.kitty_commands
                .insert(node_server.clone(), kitty_command);
        }
        Ok(())
    }

    fn terminate_backends(&mut self) -> Result<()> {
        for (_, child) in self.kitty_commands.iter_mut() {
            child.terminate_backend()?;
        }
        self.kitty_commands.clear();
        Ok(())
    }

    fn name(&self) -> String {
        "xray".into()
    }
}
