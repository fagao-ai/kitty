// use crate::process_manager::ProcessManager;
use crate::protocol::hysteria::HysteriaManager;
use sea_orm::DatabaseConnection;

pub struct DatabaseState {
    pub db: std::sync::Mutex<Option<DatabaseConnection>>,
}

impl DatabaseState {
    pub fn get_db(&self) -> DatabaseConnection {
        let db = self.db.lock().unwrap();
        let db_clone = db.clone().unwrap();
        db_clone
    }
}

pub struct ProcessManagerState {
    pub process_manager: std::sync::Mutex<HysteriaManager>,
}
