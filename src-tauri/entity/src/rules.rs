use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::NotSet;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::generate_model_functions;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "rules")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub rule_action: RuleAction,
    pub rule_type: RuleType,
    pub rule: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum RuleAction {
    #[serde(rename = "proxy")]
    #[sea_orm(string_value = "proxy")]
    Proxy,
    #[serde(rename = "direct")]
    #[sea_orm(string_value = "direct")]
    Direct,
    #[serde(rename = "reject")]
    #[sea_orm(string_value = "reject")]
    Reject,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum RuleType {
    #[serde(rename = "domain_suffix")]
    #[sea_orm(string_value = "domain_suffix")]
    DomainSuffix,
    #[serde(rename = "domain_preffix")]
    #[sea_orm(string_value = "domain_preffix")]
    DomainPreffix,
    #[serde(rename = "full_domain")]
    #[sea_orm(string_value = "full_domain")]
    FullDomain,
    #[serde(rename = "cidr")]
    #[sea_orm(string_value = "cidr")]
    Cidr,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    generate_model_functions!();
}
