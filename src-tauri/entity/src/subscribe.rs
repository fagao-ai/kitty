use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "subscribe")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    // #[serde(skip)]
    pub id: i32,
    pub url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
