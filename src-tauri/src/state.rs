use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::Mutex;
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

/// ProcessManagerState stores running shoes server handles and active proxy info
/// Instead of using command groups, we directly store JoinHandles from shoes library
#[derive(Clone)]
pub struct ProcessManagerState {
    /// Running shoes server handles
    pub running_servers: Arc<Mutex<Vec<JoinHandle<()>>>>,
    /// Active proxy ID
    pub active_proxy_id: Arc<Mutex<Option<u32>>>,
    /// Active proxy type: "hysteria" or "xray"
    pub active_proxy_type: Arc<Mutex<Option<String>>>,
}

impl Default for ProcessManagerState {
    fn default() -> Self {
        Self {
            running_servers: Arc::new(Mutex::new(Vec::new())),
            active_proxy_id: Arc::new(Mutex::new(None)),
            active_proxy_type: Arc::new(Mutex::new(None)),
        }
    }
}
