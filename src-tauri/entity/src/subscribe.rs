use sea_orm::{entity::prelude::*, ActiveValue::NotSet};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "subscribe")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub url: String,
    pub name: String,
    pub is_active: bool,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
    pub last_sync_at: Option<ChronoDateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::xray::Entity")]
    Xray,
}

impl Related<super::xray::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Xray.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    generate_model_functions!();
}
