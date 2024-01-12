use sea_orm::DatabaseConnection;
use entity::hysteria;
use crate::apis::api_traits::APIServiceTrait;
use anyhow::Result;

pub struct HysteriaAPI;


impl APIServiceTrait for HysteriaAPI {}

impl HysteriaAPI {
    pub async fn get_all(&self, db: &DatabaseConnection) -> Result<Vec<hysteria::Model>> {
        let hy_proxies = hysteria::Model::fetch_all(&db).await?;
        println!("hy_proxies: {:?}", hy_proxies);
        Ok(hy_proxies)
    }
}
