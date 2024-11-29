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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub async fn update_sysproxy_flag(db: &DatabaseConnection, value: bool) -> Result<(), DbErr> {
        let record = self::Model::first(db).await?.unwrap();
        let mut record: self::ActiveModel = record.into();
        record.sysproxy_flag = Set(value);
        let _ = record.update(db).await?;
        Ok(())
    }
    generate_model_functions!();
}
