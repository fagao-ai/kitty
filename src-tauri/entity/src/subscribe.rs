use sea_orm::{entity::prelude::*, ActiveValue::NotSet};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "subscribe")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::xray::Entity")]
    Xray,
}

// impl RelationTrait for Relation {
//     fn def(&self) -> RelationDef {
//         match self {
//             Self::Xray => Entity::has_many(super::xray::Entity)
//         }
//     }
// }

// `Related` trait has to be implemented by hand
impl Related<super::xray::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Xray.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    generate_model_functions!();
}
