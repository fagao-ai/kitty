use anyhow::{anyhow, Error, Result};
use port_scanner::scan_port;
use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

const START_PORT: u16 = 20000;
const END_PORT: u16 = 30000;

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
    generate_model_functions!();
}

#[derive(Serialize, Deserialize)]
pub struct ListenAddr {
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
pub struct CommandHysteria {
    pub server: String,
    pub auth: String,
    pub bandwidth: Bandwidth,
    pub tls: Tls,
    pub socks5: ListenAddr,
    pub http: ListenAddr,
}

impl CommandHysteria {
    pub fn get_http_port(&self) -> u16 {
        let http_addr = &self.http.listen;
        http_addr.split(":").nth(1).unwrap().parse::<u16>().unwrap()
    }

    pub fn get_socks_port(&self) -> u16 {
        let http_addr = &self.socks5.listen;
        http_addr.split(":").nth(1).unwrap().parse::<u16>().unwrap()
    }
}

impl TryFrom<&Model> for CommandHysteria {
    type Error = Error;

    fn try_from(record: &Model) -> Result<Self, Self::Error> {
        let mut available_ports = (0, 0);
        let mut available_count = 0;
        for port in START_PORT..END_PORT {
            if scan_port(port) {
                if available_count == 0 {
                    available_ports.0 = port;
                } else {
                    available_ports.1 = port;
                }

                available_count += 1;
            }
        }
        if available_count != 2 {
            return Err(anyhow!("not have engouh port"));
        }

        Ok(Self {
            server: record.server.clone(),
            auth: record.auth.clone(),
            bandwidth: record.bandwidth.clone(),
            tls: record.tls.clone(),
            socks5: ListenAddr::new(available_ports.0),
            http: ListenAddr::new(available_ports.1),
        })
    }
}
