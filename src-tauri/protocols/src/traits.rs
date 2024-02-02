use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;

pub trait KittyCommandGroupTrait {
    fn start_commands<T: Serialize>(
        &mut self,
        config: HashMap<String, T>,
        env_mapping: Option<HashMap<String, String>>,
    ) -> Result<()>;

    fn terminate_backends(&mut self) -> Result<()>;
    fn name(&self) -> String;
}
