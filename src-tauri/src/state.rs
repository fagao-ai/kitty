use sea_orm::DatabaseConnection;
use std::collections::HashSet;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use tokio::sync::watch::Sender;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;

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

/// ProcessManagerState stores running shoes server handles
/// Instead of using command groups, we directly store JoinHandles from shoes library
#[derive(Clone)]
pub struct ProcessManagerState {
    /// Running shoes server handles
    pub running_servers: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl Default for ProcessManagerState {
    fn default() -> Self {
        Self {
            running_servers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

pub struct KittyLoggerState {
    pub logger_reciver: Mutex<Option<Receiver<String>>>,
}

impl Default for KittyLoggerState {
    fn default() -> Self {
        Self {
            logger_reciver: Mutex::new(None),
        }
    }
}
