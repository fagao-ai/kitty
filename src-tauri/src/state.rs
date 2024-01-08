// use crate::process_manager::ProcessManager;
use crate::protocol::hysteria::HysteriaManager;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;
use kitty_proxy::{HttpProxy, SocksProxy};


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
    pub process_manager: Mutex<HysteriaManager>,
}

pub struct KittyProxyState {
    pub http_proxy: Mutex<HttpProxy>,
    pub socks_proxy: Mutex<SocksProxy>,
}
