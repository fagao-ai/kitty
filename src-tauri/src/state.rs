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
    /// Serializes all server start/stop operations to avoid port bind races
    pub server_start_lock: Arc<Mutex<()>>,
    /// Active proxy ID
    pub active_proxy_id: Arc<Mutex<Option<u32>>>,
    /// Active proxy type: "hysteria" or "xray"
    pub active_proxy_type: Arc<Mutex<Option<String>>>,
}

impl ProcessManagerState {
    /// Abort and clear all running shoes server tasks.
    pub async fn abort_running_servers(&self) {
        let mut running_servers = self.running_servers.lock().await;
        for handle in running_servers.drain(..) {
            handle.abort();
        }
    }
}

impl Default for ProcessManagerState {
    fn default() -> Self {
        Self {
            running_servers: Arc::new(Mutex::new(Vec::new())),
            server_start_lock: Arc::new(Mutex::new(())),
            active_proxy_id: Arc::new(Mutex::new(None)),
            active_proxy_type: Arc::new(Mutex::new(None)),
        }
    }
}
