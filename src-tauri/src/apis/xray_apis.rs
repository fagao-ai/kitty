use entity::xray;
use sea_orm::DatabaseConnection;
use anyhow::Result;

use crate::apis::api_traits::APIServiceTrait;

pub struct XrayAPI;

impl APIServiceTrait for XrayAPI {}
impl XrayAPI {
    pub async fn get_all(&self, db: &DatabaseConnection) -> Result<Vec<xray::Model>> {
        let xray_proxies = xray::Model::fetch_all(&db).await?;
        println!("xray_proxies: {:?}", xray_proxies);
        Ok(xray_proxies)
    }

    pub async fn add_xray_item(
        &self,
        db: &DatabaseConnection,
        record: xray::Model,
    ) -> Result<()> {
        record.insert_one(db).await?;
        Ok(())
    }
}
