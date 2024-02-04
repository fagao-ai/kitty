use anyhow::{anyhow, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use crate::kitty_command::KittyCommand;
use crate::traits::KittyCommandGroupTrait;
use crate::types::CheckStatusCommandPipe;

#[derive(Debug)]
pub struct HysteriaCommandGroup {
    bin_path: PathBuf,
    kitty_commands: HashMap<String, KittyCommand>,
    config_dir: PathBuf,
}

impl HysteriaCommandGroup {
    pub fn new(bin_path: PathBuf, config_dir: PathBuf) -> Self {
        Self {
            kitty_commands: HashMap::new(),
            bin_path,
            config_dir,
        }
    }
}

impl Drop for HysteriaCommandGroup {
    fn drop(&mut self) {
        for (_, child) in self.kitty_commands.iter_mut() {
            if child.is_running() {
                child.terminate_backend().ok();
            }
        }
        self.kitty_commands.clear();
    }
}

impl KittyCommandGroupTrait for HysteriaCommandGroup {
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
                ["client", "-c"],
                config,
                &self.config_dir,
                env_mapping.clone().unwrap_or(HashMap::new()),
            )?;
            thread::sleep(Duration::from_secs(1));
            let socket_addrs = self.get_socket_addrs(&config)?;
            kitty_command.check_status(
                "server listening",
                CheckStatusCommandPipe::StdErr,
                socket_addrs,
            )?;
            self.kitty_commands
                .insert(node_server.clone(), kitty_command);
        }
        Ok(())
    }

    fn get_socket_addrs<T>(&self, config: &T) -> Result<Vec<SocketAddr>>
    where
        T: Serialize,
    {
        let config_value = serde_json::to_value(config)?;

        let socks_listen = config_value["socks5"]["listen"].as_str();
        let http_listen = config_value["http"]["listen"].as_str();
        let mut res = Vec::with_capacity(2);
        for listen in [socks_listen, http_listen] {
            if let Some(address_str) = listen {
                let server: SocketAddr = address_str.parse()?;
                res.push(server);
            }
        }
        if res.len() != 2 {
            Err(anyhow!("get_socket_addrs failed."))
        } else {
            Ok(res)
        }
    }

    fn terminate_backends(&mut self) -> Result<()> {
        for (_, child) in self.kitty_commands.iter_mut() {
            child.terminate_backend()?;
        }
        self.kitty_commands.clear();
        Ok(())
    }

    fn name(&self) -> String {
        "hysteria".into()
    }
}
