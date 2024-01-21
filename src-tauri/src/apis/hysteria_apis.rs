use anyhow::Result;
use entity::hysteria;
use sea_orm::DatabaseConnection;

use crate::apis::api_traits::APIServiceTrait;

pub struct HysteriaAPI;

impl APIServiceTrait for HysteriaAPI {}

impl HysteriaAPI {
    pub async fn get_all(&self, db: &DatabaseConnection) -> Result<Vec<hysteria::Model>> {
        let hy_proxies = hysteria::Model::fetch_all(&db).await?;
        println!("hy_proxies: {:?}", hy_proxies);
        Ok(hy_proxies)
    }

    pub async fn add_hysteria_item(
        &self,
        db: &DatabaseConnection,
        record: hysteria::Model,
    ) -> Result<()> {
        record.insert_one(db).await?;
        Ok(())
    }
}
