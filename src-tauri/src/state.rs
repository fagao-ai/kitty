use kitty_proxy::MatchProxy;
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
pub struct ProcessManagerState {
    /// Running shoes server handles
    pub running_servers: Mutex<Vec<JoinHandle<()>>>,
}

impl<'a> Default for ProcessManagerState {
    fn default() -> Self {
        Self {
            running_servers: Mutex::new(Vec::new()),
        }
    }
}

pub struct KittyProxyState {
    // pub http_proxy: Mutex<Option<HttpProxy>>,
    // pub socks_proxy: Mutex<Option<SocksProxy>>,
    pub match_proxy: Mutex<Option<Arc<RwLock<MatchProxy>>>>,
    pub http_proxy_sx: Mutex<Option<Sender<bool>>>,
    pub socks_proxy_sx: Mutex<Option<Sender<bool>>>,
    pub used_ports: Mutex<HashSet<u16>>,
}

impl Default for KittyProxyState {
    fn default() -> Self {
        Self {
            // http_proxy: Mutex::new(None),
            // socks_proxy: Mutex::new(None),
            match_proxy: Mutex::new(None),
            http_proxy_sx: Mutex::new(None),
            socks_proxy_sx: Mutex::new(None),
            used_ports: Mutex::new(HashSet::new()),
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
