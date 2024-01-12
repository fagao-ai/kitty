use anyhow::Result;
use serde::Serialize;
use std::path::PathBuf;

pub trait CommandManagerTrait {
    fn start_backend<T: Serialize>(&mut self, config: T, config_dir: PathBuf) -> Result<()>;
    fn start_backend_from_path(&mut self, config_path: PathBuf) -> Result<()>;
    fn check_status(&self) -> Result<()>;
    fn terminate_backend(&mut self) -> Result<()>;
    fn restart_backend(&mut self) -> Result<()>;
    fn is_running(&self) -> bool;
    fn name(&self) -> String;
}
