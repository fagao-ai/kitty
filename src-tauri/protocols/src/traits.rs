use anyhow::Result;
use serde::Serialize;
use std::{path::PathBuf, process::Command};

pub(crate) trait CommandManagerTrait {
    fn start_backend<T: Serialize>(
        &mut self,
        init_command: &mut Command,
        config: T,
        config_dir: &PathBuf,
    ) -> Result<()>;
    fn terminate_backend(&mut self) -> Result<()>;
    fn restart_backend(&self) -> Result<()>;

    fn check_status(&mut self) -> Result<()>;

    fn is_runing(&self) -> bool;
}

// trait GenerateConfig {
//     fn generate_config() -> Res
// }
