use sea_orm::entity::prelude::*;
use sea_orm::{NotSet, Set};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::generate_model_functions;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "base_config")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub local_ip: String,
    pub http_port: u16,
    pub socks_port: u16,
    pub delay_test_url: String,
    pub sysproxy_flag: bool,
    pub auto_start: bool,
    pub language: String,
    pub update_interval: i32,
    pub allow_lan: bool,
    pub mode: String,
    pub log_level: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub async fn update_sysproxy_flag(db: &DatabaseConnection, value: bool) -> Result<(), DbErr> {
        let record = self::Model::first(db).await?;
        match record {
            Some(record) => {
                let mut record: self::ActiveModel = record.into();
                record.sysproxy_flag = Set(value);
                let _ = record.update(db).await?;
            }
            None => {
                // Create default base_config if not exists
                let default_config = Model {
                    id: 0,
                    local_ip: "127.0.0.1".to_string(),
                    http_port: 10086,
                    socks_port: 10087,
                    delay_test_url: "https://gstatic.com/generate_204".to_string(),
                    sysproxy_flag: value,
                    auto_start: false,
                    language: "zh-CN".to_string(),
                    allow_lan: false,
                    mode: "Rules".to_string(),
                    update_interval: 3,
                    log_level: "debug".to_string(),
                };
                let _ = default_config.insert_one(db).await?;
            }
        }
        Ok(())
    }
    generate_model_functions!();
}
