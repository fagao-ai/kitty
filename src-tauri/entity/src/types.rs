use std::fmt;

use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer, Visitor};

use uuid::Uuid;

const XRAY_SCHEMAS: [&str; 3] = ["vmess", "vless", "trojan"];
const HYSTERIA_SCHEMAS: [&str; 1] = ["hy2"];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareJsonStruct {
    pub ps: String,
    pub add: String,
    #[serde(deserialize_with = "port_as_u16")]
    pub port: u16,
    #[serde(deserialize_with = "uuid_as_string")]
    pub id: String,
    pub net: String,
    pub r#type: String,
    pub host: String,
    pub path: String,
    pub tls: String,
}

fn uuid_as_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct PortVisitor;

    impl<'de> Visitor<'de> for PortVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or integer representing a port")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Uuid::parse_str(value).map_err(|_| E::custom(format!("invalid port: {}", value))).map(|x| x.to_string())
        }
    }

    deserializer.deserialize_any(PortVisitor)
}

fn port_as_u16<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    struct PortVisitor;

    impl<'de> Visitor<'de> for PortVisitor {
        type Value = u16;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or integer representing a port")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value <= u16::MAX as u64 {
                Ok(value as u16)
            } else {
                Err(E::custom(format!("port out of range: {}", value)))
            }
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value.parse::<u16>().map_err(|_| E::custom(format!("invalid port: {}", value)))
        }
    }

    deserializer.deserialize_any(PortVisitor)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareWithProtocol {
    pub share: ShareJsonStruct,
    pub protocol: String,
}

impl ShareWithProtocol {
    pub fn new(protocol: String, share: ShareJsonStruct) -> Self {
        Self { share, protocol }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolLine {
    pub line: String,
    pub protocol: String,
}

impl ProtocolLine {
    pub fn new(line: String, protocol: String) -> Self {
        Self { line, protocol }
    }

    pub fn is_xray(&self) -> bool {
        XRAY_SCHEMAS.contains(&&self.protocol.as_str())
    }

    pub fn is_hy2(&self) -> bool {
        HYSTERIA_SCHEMAS.contains(&&self.protocol.as_str())
    }
}

impl TryFrom<ProtocolLine> for ShareWithProtocol {
    type Error = anyhow::Error;
    fn try_from(value: ProtocolLine) -> Result<Self, Self::Error> {
        let share: ShareJsonStruct = serde_json::from_str(&value.line)?;
        Ok(ShareWithProtocol::new(value.protocol, share))
    }
}
