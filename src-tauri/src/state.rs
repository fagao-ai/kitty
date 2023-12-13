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

    // pub fn get_process_manager(&self) -> ProcessManager {
    //     let process_manager = self.process_manager.lock().unwrap();
    //     let process_manager_clone = process_manager.clone().unwrap();
    //     process_manager
    // }
}
