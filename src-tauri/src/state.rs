use kitty_proxy::{HttpProxy, SocksProxy};
use protocols::{HysteriaManager, XrayManager};
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

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
    pub hy_process_manager: Mutex<HysteriaManager>,
    pub xray_process_manager: Mutex<XrayManager>,
}

pub struct KittyProxyState {
    pub http_proxy: Mutex<HttpProxy>,
    pub socks_proxy: Mutex<SocksProxy>,
}
