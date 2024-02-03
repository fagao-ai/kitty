use anyhow::Result;
use entity::hysteria;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::apis::api_traits::APIServiceTrait;

pub struct HysteriaAPI;

impl APIServiceTrait for HysteriaAPI {}

impl HysteriaAPI {
    pub async fn get_all(&self, db: &DatabaseConnection) -> Result<Vec<hysteria::Model>> {
        let hy_proxies = hysteria::Model::fetch_all(&db).await?;
        println!("hy_proxies: {:?}", hy_proxies);
        Ok(hy_proxies)
    }

    pub async fn get_hysteria_by_id(
        &self,
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<hysteria::Model>> {
        let res = hysteria::Model::get_by_id(db, id).await?;
        Ok(res)
    }

    pub async fn add_hysteria_item(
        &self,
        db: &DatabaseConnection,
        record: hysteria::Model,
    ) -> Result<()> {
        record.insert_one(db).await?;
        Ok(())
    }

    pub async fn delete_hysteria_item(&self, db: &DatabaseConnection, id: i32) -> Result<()> {
        let _ = hysteria::Model::delete_by_id(db, id).await?;
        Ok(())
    }

    pub async fn update_hysteria_item(
        &self,
        db: &DatabaseConnection,
        record: hysteria::Model,
    ) -> Result<()> {
        let _ = record.update(db).await?;
        Ok(())
    }
}
