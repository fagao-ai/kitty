use std::collections::HashMap;
use kitty_proxy::{HttpProxy, MatchProxy, SocksProxy};
use protocols::{CommandManagerTrait, HysteriaManager, XrayManager};
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

pub struct ProcessManagerState<T>
    where T: CommandManagerTrait
{
    pub process_manager: Mutex<HashMap<String, T>>,
}

pub struct KittyProxyState {
    pub http_proxy: Mutex<Option<HttpProxy>>,
    pub socks_proxy: Mutex<Option<SocksProxy>>,
    pub match_proxy: Mutex<Option<MatchProxy>>,
}
