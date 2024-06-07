use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use entity::subscribe;
use entity::xray;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::ModelTrait;
use sea_orm::Set;
use sea_orm::TransactionTrait;
use std::str::FromStr;

use crate::apis::api_traits::APIServiceTrait;

use super::parse_subscription::download_subcriptions;

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
        let mut xray_models = Vec::new();
        let txn = db.begin().await?;
        if url.starts_with("http") {
            let subscriptions = download_subcriptions(url).await?;
            let subscribe = subscribe::ActiveModel {
                url: Set(url.to_owned()),
                ..Default::default()
            };
            let exec_subscribe_res = subscribe.insert(&txn).await?;
            for line in subscriptions {
                if !line.is_xray() {
                    continue;
                }
                println!("asdsadas: {}", line.line);
                if let Ok(mut xray_model) = xray::Model::from_str(&line.line.trim()) {
                    xray_model.subscribe_id = Some(exec_subscribe_res.id);
                    xray_models.push(xray_model);
                }
            }
        }else  {
            let trimed_line = if let Some(pos) = url.rfind('#') {
                &url[..pos]
            } else {
                url
            };
            if let Ok(xray_model) = xray::Model::from_str(&trimed_line.trim()) {
                xray_models.push(xray_model);
            }
        }
        println!("xray_models: {}", xray_models.len());
        xray::Model::insert_many(&txn, xray_models).await?;
        txn.commit().await?;
        Ok(())
    }

    pub async fn refresh_subscribe(db: &DatabaseConnection, ids: Option<Vec<i32>>) -> Result<()> {
        let res = if let Some(subscribe_ids) = ids {
            subscribe::Model::fetch_by_ids(db, subscribe_ids).await?
        } else {
            subscribe::Model::fetch_all(db).await?
        };
        if res.len() > 0 {
            for subscribe_item in res {
                let subscriptions = download_subcriptions(&subscribe_item.url).await?;
                let xray_records = subscribe_item
                    .find_related(xray::Entity)
                    .all(db)
                    .await
                    .unwrap();
                let txn = db.begin().await?;
                let xray_ids: Vec<i32> = xray_records.iter().map(|x| x.id).collect();
                let _ = xray::Model::delete_by_ids(&txn, xray_ids).await?;
                let mut xray_models = Vec::new();
                for line in subscriptions {
                    if !line.is_xray() {
                        continue;
                    }
                    let mut xray_model = xray::Model::from_str(&line.line.trim())?;
                    xray_model.subscribe_id = Some(subscribe_item.id);
                    xray_models.push(xray_model)
                }
                xray::Model::insert_many(&txn, xray_models).await?;
                txn.commit().await?;
            }
        }
        Ok(())
    }

    pub async fn batch_get_subscriptions(db: &DatabaseConnection) -> Result<Vec<subscribe::Model>> {
        Ok(subscribe::Model::fetch_all(db).await?)
    }
}
