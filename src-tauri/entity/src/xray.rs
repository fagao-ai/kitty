use sea_orm::{entity::prelude::*, FromJsonQueryResult};

use anyhow::{anyhow, Error, Result};
use serde::{Deserialize, Serialize};
const START_PORT: u16 = 20000;
const END_PORT: u16 = 30000;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "xray")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[serde(skip)]
    pub id: i32,
    pub name: String,
    pub protocol: String,
    pub uuid: String,
    pub address: String,
    pub port: u16,
    #[sea_orm(column_type = "Text")]
    stream_settings: StreamSettings,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub enum StreamSettings {
    WebSocket(WebSocket),
    Tcp,
    Http2,
    Grpc,
    Mkcp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct WebSocket {
    network: String,
    security: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_settings: Option<TLSSettings>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
struct TLSSettings {
    allow_insecure: bool,
    server_name: String,
    fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
struct WsSettings {
    path: String,
    headers: Headers,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
struct Headers {
    host: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
