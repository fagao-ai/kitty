use sea_orm::{entity::prelude::*, FromJsonQueryResult};

use anyhow::Result;
use port_scanner::local_port_available;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "hysteria")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[serde(skip)]
    pub id: i32,
    pub name: String,
    pub server: String,
    pub auth: String,
    #[sea_orm(column_type = "Text")]
    tls: Tls,
    #[sea_orm(column_type = "Text")]
    bandwidth: Bandwidth,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Tls {
    sni: String,
    insecure: bool,
    #[serde(rename = "pinSHA256")]
    pin_sha256: Option<String>,
    ca: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Bandwidth {
    up: String,
    down: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HysteriaModelWithoutName {
    #[serde(skip)]
    pub name: String,
    pub server: String,
    pub auth: String,
    tls: Tls,
    bandwidth: Bandwidth,
}

impl<'a> From<&'a Model> for HysteriaModelWithoutName {
    fn from(source: &'a Model) -> Self {
        HysteriaModelWithoutName {
            name: source.name.clone(),
            server: source.server.clone(),
            auth: source.auth.clone(),
            tls: source.tls.clone(),
            bandwidth: source.bandwidth.clone(),
        }
    }
}

impl Model {
    pub async fn insert_one(&self, db: &DatabaseConnection) -> Result<Model, DbErr> {
        let json_value = serde_json::to_value(self).unwrap();
        let hysteria_record = self::ActiveModel::from_json(json_value)?;
        let hysteria_res = hysteria_record.insert(db).await;
        hysteria_res
    }

    pub async fn fetch_all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        let hysterias = self::Entity::find().all(db).await?;
        Ok(hysterias)
    }
}

#[derive(Serialize, Deserialize)]
struct ListenAddr {
    pub listen: String,
}

impl ListenAddr {
    fn new(port: u16) -> Self {
        Self {
            listen: format!("127.0.0.1:{port}"),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CommandHysteria {
    pub server: String,
    pub auth: String,
    pub bandwidth: Bandwidth,
    pub tls: Tls,
    pub socks5: ListenAddr,
    pub http: ListenAddr,
}

impl CommandHysteria {
    fn get_http_port(&self) -> u16 {
        let http_addr = &self.http.listen;
        http_addr.split(":").nth(1).unwrap().parse::<u16>().unwrap()
    }

    fn get_socks_port(&self) -> u16 {
        let http_addr = &self.socks5.listen;
        http_addr.split(":").nth(1).unwrap().parse::<u16>().unwrap()
    }
}

impl TryFrom<&Model> for CommandHysteria {
    type Error = String;

    fn try_from(record: &Model) -> Result<Self, Self::Error> {
        let http_port = 11186;
        let socks_port = 11186;
        for port in [http_port, socks_port] {
            if !local_port_available(port) {
                return Err(format!("port {port} already used."));
            }
        }
        Ok(Self {
            server: record.server.clone(),
            auth: record.auth.clone(),
            bandwidth: record.bandwidth.clone(),
            tls: record.tls.clone(),
            socks5: ListenAddr::new(socks_port),
            http: ListenAddr::new(http_port),
        })
    }
}
