use kitty_proxy::MatchProxy;
#[cfg(feature = "hysteria")]
use protocols::HysteriaCommandGroup;
#[cfg(feature = "xray")]
use protocols::XrayCommandGroup;
use sea_orm::DatabaseConnection;
use std::collections::HashSet;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use tokio::sync::watch::Sender;
use tokio::sync::{Mutex, RwLock};

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
    pub hy_process_manager: Mutex<Option<HysteriaCommandGroup>>,
    #[cfg(feature = "xray")]
    pub xray_process_manager: Mutex<Option<XrayCommandGroup>>,
}

impl<'a> Default for ProcessManagerState {
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
