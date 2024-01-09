use anyhow::Result;

use async_trait::async_trait;
use tauri::AppHandle;

#[async_trait]
pub(crate) trait CommandManagerTrait {
    fn start_backend(&mut self, app_handle: AppHandle, file_content: &str) -> Result<()>;
    fn terminate_backend(&mut self) -> Result<()>;
    fn restart_backend(&mut self, app_handle: AppHandle, file_content: &str) -> Result<()>;

    async fn check_status(&mut self) -> Result<()>;

    fn is_open(&self) -> bool;
}
