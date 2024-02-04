use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::net::SocketAddr;

pub trait KittyCommandGroupTrait {
    fn start_commands<T: Serialize>(
        &mut self,
        config: HashMap<String, T>,
        env_mapping: Option<HashMap<String, String>>,
    ) -> Result<()>;

    fn terminate_backends(&mut self) -> Result<()>;
    fn name(&self) -> String;

    fn get_socket_addrs<T: Serialize>(&self, config: &T) -> Result<Vec<SocketAddr>>;
}
