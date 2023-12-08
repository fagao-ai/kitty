use crate::process_manager::ProcessManager;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub db: std::sync::Mutex<Option<DatabaseConnection>>,
    pub process_manager: std::sync::Mutex<ProcessManager>,
}
