use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareJsonStruct {
    pub ps: String,
    pub add: String,
    pub port: String,
    pub id: String,
    pub net: String,
    pub r#type: String,
    pub host: String,
    pub path: String,
    pub tls: String,
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

