use std::process::Command;
use anyhow::Result;

use async_trait::async_trait;

#[async_trait]
pub(crate) trait CommandManagerTrait {
    fn start_backend(&mut self, command: &Command, file_content: &str) -> Result<()>;
    fn terminate_backend(&mut self) -> Result<()>;
    fn restart_backend(&mut self, file_content: &str) -> Result<()>;

    async fn check_status(&mut self) -> Result<()>;

    fn is_open(&self) -> bool;
}
