use std::sync::Arc;
use kitty_proxy::{HttpProxy, MatchProxy, SocksProxy};
use protocols::{KittyCommandGroup};
use sea_orm::DatabaseConnection;
use tokio::sync::watch::Sender;
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
    #[cfg(feature = "hysteria")]
    pub hy_process_manager: Mutex<Option<KittyCommandGroup>>,
    #[cfg(feature = "xray")]
    pub xray_process_manager: Mutex<Option<KittyCommandGroup>>,
}

impl Default for ProcessManagerState {
    fn default() -> Self {
        Self {
            #[cfg(feature = "hysteria")]
            hy_process_manager: Mutex::new(None),

            #[cfg(feature = "xray")]
            xray_process_manager: Mutex::new(None),
        }
    }
}

pub struct KittyProxyState {
    // pub http_proxy: Mutex<Option<HttpProxy>>,
    // pub socks_proxy: Mutex<Option<SocksProxy>>,
    pub match_proxy: Mutex<Option<Arc<MatchProxy>>>,
    pub http_proxy_sx: Mutex<Option<Sender<bool>>>,
    pub socks_proxy_sx: Mutex<Option<Sender<bool>>>,
}

impl Default for KittyProxyState {
    fn default() -> Self {
        Self {
            // http_proxy: Mutex::new(None),
            // socks_proxy: Mutex::new(None),
            match_proxy: Mutex::new(None),
            http_proxy_sx: Mutex::new(None),
            socks_proxy_sx: Mutex::new(None),
        }
    }
}
