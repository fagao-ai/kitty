use anyhow::Result;
use entity::base_config;
use sea_orm::DatabaseConnection;

pub trait APIServiceTrait {
    async fn get_proxy_ports(db: &DatabaseConnection) -> Result<(u16, u16)> {
        let record = base_config::Model::first(db).await?.ok_or_else(|| anyhow::anyhow!("base_config not exists"))?;
        let http_port = record.http_port;
        let socks_port = record.socks_port;
        Ok((http_port, socks_port))
    }
}
