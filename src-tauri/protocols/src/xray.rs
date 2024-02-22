use anyhow::{anyhow, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;

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
            println!("xray runed");
            let socket_addrs = self.get_socket_addrs(&config)?;
            kitty_command.check_status(
                "Reading config:",
                CheckStatusCommandPipe::StdOut,
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
        let mut res = Vec::new();
        if let Some(inbounds) = config_value["inbounds"].as_array() {
            for inbound in inbounds {
                let mut listen = inbound["listen"].as_str().unwrap();
                if listen == "0.0.0.0" {
                    listen = "127.0.0.1";
                }
                let port = inbound["port"].as_i64().unwrap();
                let ip_addr: IpAddr = IpAddr::from_str(listen)?;
                let socket_addr = SocketAddr::new(ip_addr, port.to_owned() as u16);
                res.push(socket_addr);
            }
            Ok(res)
        } else {
            Err(anyhow!("get_socket_addrs failed."))
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
        "xray".into()
    }
}
