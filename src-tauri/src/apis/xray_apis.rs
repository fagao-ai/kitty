use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use entity::subscribe;
use entity::xray;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::Set;
use sea_orm::TransactionTrait;
use std::str::FromStr;

use crate::apis::api_traits::APIServiceTrait;

pub struct XrayAPI;

impl APIServiceTrait for XrayAPI {}
impl XrayAPI {
    pub async fn get_all(&self, db: &DatabaseConnection) -> Result<Vec<xray::Model>> {
        let xray_proxies = xray::Model::fetch_all(db).await?;
        let xray_proxies: Vec<xray::Model> =
            xray_proxies.into_iter().map(|model| model.into()).collect();
        Ok(xray_proxies)
    }

    pub async fn get_xray_by_id(
        &self,
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<xray::Model>> {
        Ok(xray::Model::get_by_id(db, id).await?)
    }

    pub async fn add_xray_item(&self, db: &DatabaseConnection, record: xray::Model) -> Result<()> {
        record.insert_one(db).await?;
        Ok(())
    }

    pub async fn delete_xray_item(&self, db: &DatabaseConnection, id: i32) -> Result<()> {
        let _ = xray::Model::delete_by_id(db, id).await?;
        Ok(())
    }

    pub async fn update_xray_item(
        &self,
        db: &DatabaseConnection,
        record: xray::Model,
    ) -> Result<()> {
        let _ = record.update(db).await?;
        Ok(())
    }

    pub async fn import_xray_from_subscribe(
        &self,
        db: &DatabaseConnection,
        url: &str,
    ) -> Result<()> {
        let resp = reqwest::get(url).await?;
        let resp_text = resp.text().await?;
        let decode_bytes = general_purpose::STANDARD.decode(resp_text)?;
        let share_protocol_string =
            std::string::String::from_utf8(decode_bytes).expect("Invalid UTF-8 sequence");
        let mut xray_models = Vec::new();
        let subscribe = subscribe::ActiveModel {
            url: Set(url.to_owned()),
            ..Default::default()
        };
        let txn = db.begin().await?;
        let exec_subscribe_res = subscribe.insert(&txn).await?;
        for line in share_protocol_string.lines() {
            let mut xray_model = xray::Model::from_str(line.trim())?;
            xray_model.subscribe_id = Some(exec_subscribe_res.id);
            xray_models.push(xray_model)
        }
        xray::Model::insert_many(&txn, xray_models).await?;
        txn.commit().await?;
        Ok(())
    }
}
