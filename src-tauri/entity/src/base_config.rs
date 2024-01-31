use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "base_config")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    // #[serde(skip)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub local_ip: String,
    pub http_port: u16,
    pub socks_port: u16,
    pub delay_test_url: String,
    pub start_proxy: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub async fn insert_one(&self, db: &DatabaseConnection) -> Result<Self, DbErr> {
        let json_value = serde_json::to_value(self).unwrap().into();
        let base_config_record = ActiveModel::from_json(json_value)?;
        let base_config_res = base_config_record.insert(db).await;
        base_config_res
    }

    pub async fn first(db: &DatabaseConnection) -> Result<Option<Self>, DbErr> {
        let base_config_record = self::Entity::find().one(db).await?;
        Ok(base_config_record)
    }

    pub async fn update(&self, db: &DatabaseConnection, id: i32) -> Result<self::Model, DbErr> {
        let json_value = serde_json::to_value(self).unwrap();
        let base_config_record = self::Entity::find_by_id(id).one(db).await?;
        let mut base_config_record: self::ActiveModel = base_config_record.unwrap().into();
        let _ = base_config_record.set_from_json(json_value);
        let base_config_res = base_config_record.update(db).await?;
        Ok(base_config_res)
    }

}
