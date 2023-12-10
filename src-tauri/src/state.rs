use crate::process_manager::ProcessManager;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub db: std::sync::Mutex<Option<DatabaseConnection>>,
    pub process_manager: std::sync::Mutex<ProcessManager>,
}

impl AppState {
    pub fn get_db(&self) -> DatabaseConnection {
        let db = self.db.lock().unwrap();
        let db_clone = db.clone().unwrap();
        db_clone
    }
}
