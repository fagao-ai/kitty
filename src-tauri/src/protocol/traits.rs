use std::ffi::OsStr;

use anyhow::Result;

use async_trait::async_trait;
use tauri::AppHandle;

#[async_trait]
pub(crate) trait CommandManagerTrait {
    fn start_backend<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(
        &mut self,
        app_handle: AppHandle,
        args: I,
    ) -> Result<()>;
    fn terminate_backend(&mut self) -> Result<()>;
    fn restart_backend<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(
        &mut self,
        app_handle: AppHandle,
        args: I,
    ) -> Result<()>;

    async fn check_status(&mut self) -> Result<()>;

    fn is_open(&self) -> bool;
}
