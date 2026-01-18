use anyhow::Error;
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use log::Level;
use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

use crate::types::ShareJsonStruct;
use crate::types::ShareWithProtocol;
use crate::utils::get_random_port;
use sea_orm::ActiveValue::NotSet;

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel, FromJsonQueryResult,
)]
#[sea_orm(table_name = "xray")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub name: String,
    pub protocol: Protocol,
    pub uuid: String,
    pub address: String,
    pub port: u16,
    #[sea_orm(column_type = "Text")]
    stream_settings: StreamSettings,
    pub subscribe_id: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Protocol {
    #[serde(rename = "vless")]
    #[sea_orm(string_value = "vless")]
    Vless,
    #[sea_orm(string_value = "vmess")]
    #[serde(rename = "vmess")]
    Vmess,
    #[sea_orm(string_value = "trojan")]
    #[serde(rename = "trojan")]
    Trojan,
}

impl FromStr for Protocol {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vless" => Ok(Protocol::Vless),
            "vmess" => Ok(Protocol::Vmess),
            "trojan" => Ok(Protocol::Trojan),
            _ => Err(anyhow!("convert error")),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::subscribe::Entity",
        from = "Column::SubscribeId",
        to = "super::subscribe::Column::Id"
    )]
    Subscribe,
}

// `Related` trait has to be implemented by hand
impl Related<super::subscribe::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subscribe.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    generate_model_functions!();

    /// Get the stream settings for this xray configuration.
    pub fn stream_settings(&self) -> &StreamSettings {
        &self.stream_settings
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(rename = "streamSettings")]
pub enum StreamSettings {
    #[serde(untagged)]
    WebSocket(WebSocketProtocol),
    #[serde(untagged)]
    Tcp(TcpProtocol),
    #[serde(untagged)]
    Http2(Http2Protocol),
    #[serde(untagged)]
    Grpc(GrpcProtocol),
    #[serde(untagged)]
    Kcp(KcpProtocol),
    #[serde(untagged)]
    Trojan(TrojanProtocol),
}

impl StreamSettings {
    /// Get the network type (ws, tcp, grpc, http2, kcp)
    pub fn network(&self) -> &str {
        match self {
            StreamSettings::WebSocket(p) => &p.network,
            StreamSettings::Tcp(p) => &p.network,
            StreamSettings::Http2(p) => &p.network,
            StreamSettings::Grpc(p) => &p.network,
            StreamSettings::Kcp(p) => &p.network,
            StreamSettings::Trojan(p) => &p.network,
        }
    }

    /// Get the security setting if present
    pub fn security(&self) -> Option<&Security> {
        match self {
            StreamSettings::WebSocket(p) => p.security.as_ref(),
            StreamSettings::Tcp(p) => p.security.as_ref(),
            StreamSettings::Http2(p) => p.security.as_ref(),
            StreamSettings::Grpc(p) => p.security.as_ref(),
            StreamSettings::Kcp(p) => p.security.as_ref(),
            StreamSettings::Trojan(p) => p.security.as_ref(),
        }
    }

    /// Get TLS settings if present
    pub fn tls_settings(&self) -> Option<&TLSSettings> {
        match self {
            StreamSettings::WebSocket(p) => p.tls_settings.as_ref(),
            StreamSettings::Tcp(p) => p.tls_settings.as_ref(),
            StreamSettings::Http2(p) => p.tls_settings.as_ref(),
            StreamSettings::Grpc(p) => p.tls_settings.as_ref(),
            StreamSettings::Kcp(p) => p.tls_settings.as_ref(),
            StreamSettings::Trojan(p) => p.tls_settings.as_ref(),
        }
    }

    /// Get Reality settings if present
    pub fn reality_settings(&self) -> Option<&RealitySettings> {
        match self {
            StreamSettings::WebSocket(p) => p.reality_settings.as_ref(),
            StreamSettings::Tcp(p) => p.reality_settings.as_ref(),
            StreamSettings::Grpc(p) => p.reality_settings.as_ref(),
            StreamSettings::Kcp(_) | StreamSettings::Http2(_) | StreamSettings::Trojan(_) => None,
        }
    }

    /// Get WebSocket path if this is a WebSocket config
    pub fn ws_path(&self) -> Option<&str> {
        match self {
            StreamSettings::WebSocket(p) => Some(p.ws_settings.path.as_str()),
            _ => None,
        }
    }

    /// Get WebSocket host if this is a WebSocket config
    pub fn ws_host(&self) -> Option<&str> {
        match self {
            StreamSettings::WebSocket(p) => Some(p.ws_settings.headers.host.as_str()),
            _ => None,
        }
    }

    /// Get gRPC service name if this is a gRPC config
    pub fn grpc_service_name(&self) -> Option<&str> {
        match self {
            StreamSettings::Grpc(p) => Some(p.grpc_settings.service_name.as_str()),
            _ => None,
        }
    }
}

impl TryFrom<url::form_urlencoded::Parse<'_>> for StreamSettings {
    type Error = anyhow::Error;
    fn try_from(query_pairs: url::form_urlencoded::Parse<'_>) -> Result<Self> {
        let query_params: HashMap<String, String> = query_pairs
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect();
        let allow_insecure = query_params
            .get("allowInsecure")
            .map(|x| x.as_str())
            .unwrap_or("false");
        let allow_insecure = matches!(allow_insecure, "true" | "1");
        let host = query_params
            .get("host")
            .map(|x| x.to_string())
            .unwrap_or("".into());
        let path = query_params.get("path").map(|x| x.as_str()).unwrap_or("");
        let mut security = query_params
            .get("security")
            .map(|x| x.as_str())
            .unwrap_or("none");
        let r#type = query_params
            .get("type")
            .map(|x| x.as_str())
            .unwrap_or("tcp");
        let sni_key = if query_params.contains_key("peer") {
            "peer"
        } else {
            "sni"
        };
        let server_name = query_params.get(sni_key).map(|x| x.as_str()).unwrap_or("");
        if !server_name.is_empty() {
            security = "tls"
        }
        let security: Security = Security::from_str(security)?;
        let mut tls_settings = None;
        let mut reality_settings = None;
        match security {
            Security::Reality => {
                let fingerprint: String = query_params
                    .get("fp")
                    .map(|x| x.as_str())
                    .unwrap_or("chrome")
                    .into();
                let server_name: String = server_name.into();
                let public_key: String = query_params
                    .get("pbk")
                    .map(|x| x.as_str())
                    .unwrap_or("")
                    .into();
                let short_id: String = "".into();

                let spider_x: String = query_params
                    .get("spx")
                    .map(|x| x.as_str())
                    .unwrap_or("")
                    .into();
                reality_settings = Some(RealitySettings::new(
                    fingerprint,
                    server_name,
                    public_key,
                    short_id,
                    spider_x,
                ))
            }
            Security::Tls => {
                tls_settings = Some(TLSSettings::new(allow_insecure, server_name.into()));
            }
            Security::None => {}
        }
        match r#type {
            "ws" => {
                let ws_protocol: WebSocketProtocol = WebSocketProtocol::new(
                    r#type.into(),
                    Some(security),
                    host,
                    Some(path.into()),
                    tls_settings,
                    None,
                );
                Ok(StreamSettings::WebSocket(ws_protocol))
            }
            "tcp" => {
                let tcp_protocol: TcpProtocol = TcpProtocol::new(
                    r#type.into(),
                    Some(security),
                    tls_settings,
                    reality_settings,
                );
                Ok(StreamSettings::Tcp(tcp_protocol))
            }
            "grpc" => {
                let grpc_protocol: GrpcProtocol = GrpcProtocol::new(
                    r#type.into(),
                    Some(security),
                    tls_settings,
                    reality_settings,
                );
                Ok(StreamSettings::Grpc(grpc_protocol))
            }
            _ => Err(anyhow!("convert stream_settings failed.")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealitySettings {
    fingerprint: String,
    #[serde(rename = "serverName")]
    server_name: String,
    #[serde(rename = "publicKey")]
    public_key: String,
    #[serde(rename = "shortId")]
    short_id: String,
    #[serde(rename = "spiderX")]
    spider_x: String,
}

impl RealitySettings {
    fn new(
        fingerprint: String,
        server_name: String,
        public_key: String,
        short_id: String,
        spider_x: String,
    ) -> Self {
        Self {
            fingerprint,
            server_name,
            public_key,
            short_id,
            spider_x,
        }
    }

    /// Get the server name (SNI)
    pub fn server_name(&self) -> &str {
        &self.server_name
    }

    /// Get the public key
    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    /// Get the short ID
    pub fn short_id(&self) -> &str {
        &self.short_id
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebSocketProtocol {
    network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<Security>,
    #[serde(rename = "tlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_settings: Option<TLSSettings>,
    #[serde(rename = "realitySettings")]
    reality_settings: Option<RealitySettings>,
    #[serde(rename = "wsSettings")]
    ws_settings: WsSettings,
}

impl WebSocketProtocol {
    fn new(
        network: String,
        security: Option<Security>,
        host: String,
        path: Option<String>,
        tls_settings: Option<TLSSettings>,
        reality_settings: Option<RealitySettings>,
    ) -> Self {
        let ws_settings = WsSettings::new(host, path);
        Self {
            network,
            security,
            tls_settings,
            ws_settings,
            reality_settings,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TcpProtocol {
    network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<Security>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tlsSettings")]
    tls_settings: Option<TLSSettings>,
    #[serde(rename = "realitySettings")]
    reality_settings: Option<RealitySettings>,
    #[serde(rename = "tcpSettings")]
    tcp_settings: TcpSettings,
}

impl TcpProtocol {
    fn new(
        network: String,
        security: Option<Security>,
        tls_settings: Option<TLSSettings>,
        reality_settings: Option<RealitySettings>,
    ) -> Self {
        let tcp_settings = TcpSettings::default();
        Self {
            network,
            security,
            tls_settings,
            tcp_settings,
            reality_settings,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Security {
    #[serde(rename = "tls")]
    Tls,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "reality")]
    Reality,
}

impl FromStr for Security {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tls" => Ok(Security::Tls),
            "reality" => Ok(Security::Reality),
            _ => Ok(Security::None),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TLSSettings {
    #[serde(rename = "allowInsecure")]
    allow_insecure: bool,
    #[serde(rename = "serverName")]
    server_name: String,
}

impl TLSSettings {
    fn new(allow_insecure: bool, server_name: String) -> Self {
        Self {
            allow_insecure,
            server_name,
        }
    }

    /// Get the allow_insecure setting
    pub fn allow_insecure(&self) -> bool {
        self.allow_insecure
    }

    /// Get the server name (SNI)
    pub fn server_name(&self) -> &str {
        &self.server_name
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct WsSettings {
    headers: Headers,
    path: String,
}

impl WsSettings {
    fn new(host: String, path: Option<String>) -> Self {
        Self {
            path: path.unwrap_or("".into()),
            headers: Headers::new(host),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Headers {
    #[serde(rename = "Host")]
    host: String,
}

impl Headers {
    fn new(host: String) -> Self {
        Self { host }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TcpSettings {
    header: TcpHeader,
}

impl Default for TcpSettings {
    fn default() -> Self {
        Self {
            header: TcpHeader::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TcpHeader {
    r#type: TcpType,
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<TcpRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<TcpResponse>,
}

impl Default for TcpHeader {
    fn default() -> Self {
        Self {
            r#type: TcpType::default(),
            request: Some(TcpRequest::default()),
            response: Some(TcpResponse::default()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum TcpType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "http")]
    Http,
}

impl Default for TcpType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TcpRequest {
    version: String,
    method: String,
    path: Vec<String>,
    headers: TcpRequestHeaders,
}

impl Default for TcpRequest {
    fn default() -> Self {
        Self {
            version: "1.1".into(),
            method: "GET".into(),
            path: vec!["/".into()],
            headers: { TcpRequestHeaders::default() },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TcpRequestHeaders {
    #[serde(rename = "Host")]
    host: Vec<String>,
    #[serde(rename = "User-Agent")]
    user_agent: Vec<String>,
    #[serde(rename = "Accept-Encoding")]
    accept_encoding: Vec<String>,
    #[serde(rename = "Connection")]
    connection: Vec<String>,
    #[serde(rename = "Pragma")]
    pragma: String,
}

impl Default for TcpRequestHeaders {
    fn default() -> Self {
        Self {
            host: vec!["www.baidu.com".into(), "www.bing.com".into()],
            user_agent: vec![
                "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36".into(),
                "Mozilla/5.0 (iPhone; CPU iPhone OS 10_0_2 like Mac OS X) AppleWebKit/601.1 (KHTML, like Gecko) CriOS/53.0.2785.109 Mobile/14A456 Safari/601.1.46".into(),
            ],
            accept_encoding: vec!["gzip, deflate".into()],
            connection: vec!["keep-alive".into()],
            pragma: "no-cache".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TcpResponse {
    version: String,
    status: String,
    reason: String,
    headers: TcpResponseHeaders,
}

impl Default for TcpResponse {
    fn default() -> Self {
        Self {
            version: "1.1".into(),
            status: "200".into(),
            reason: "OK".into(),
            headers: { TcpResponseHeaders::default() },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TcpResponseHeaders {
    #[serde(rename = "Content-Type")]
    content_type: Vec<String>,
    #[serde(rename = "transfer-Encoding")]
    transfer_encoding: Vec<String>,
    #[serde(rename = "Connection")]
    connection: Vec<String>,
    #[serde(rename = "Pragma")]
    pragma: String,
}

impl Default for TcpResponseHeaders {
    fn default() -> Self {
        Self {
            content_type: vec!["application/octet-stream".into(), "video/mpeg".into()],
            transfer_encoding: vec!["chunked".into()],
            connection: vec!["keep-alive".into()],
            pragma: "no-cache".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrpcProtocol {
    network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<Security>,
    #[serde(rename = "tlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_settings: Option<TLSSettings>,
    #[serde(rename = "realitySettings")]
    reality_settings: Option<RealitySettings>,
    #[serde(rename = "tcpSettings")]
    grpc_settings: GrpcSettings,
}

impl GrpcProtocol {
    fn new(
        network: String,
        security: Option<Security>,
        tls_settings: Option<TLSSettings>,
        reality_settings: Option<RealitySettings>,
    ) -> Self {
        Self {
            network,
            security,
            tls_settings,
            grpc_settings: GrpcSettings::default(),
            reality_settings,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct GrpcSettings {
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "multiMode")]
    multi_mode: bool,
    idle_timeout: u16,
    health_check_timeout: u16,
    permit_without_stream: bool,
    initial_windows_size: u16,
}

impl Default for GrpcSettings {
    fn default() -> Self {
        Self {
            service_name: "".into(),
            multi_mode: true,
            idle_timeout: 60,
            health_check_timeout: 20,
            permit_without_stream: false,
            initial_windows_size: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Http2Protocol {
    network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<Security>,
    #[serde(rename = "tlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_settings: Option<TLSSettings>,
    #[serde(rename = "httpSettings")]
    http2_settings: Http2Settings,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Http2Settings {
    host: Vec<String>,
    path: String,
}

impl Default for Http2Settings {
    fn default() -> Self {
        Self {
            host: vec![],
            path: "/".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KcpProtocol {
    network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<Security>,
    #[serde(rename = "tlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_settings: Option<TLSSettings>,
    #[serde(rename = "kcpSettings")]
    kcp_settings: KcpSettings,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct KcpSettings {
    mtu: u16,
    tti: u16,
    #[serde(rename = "uplinkCapacity")]
    uplink_capacity: u16,
    #[serde(rename = "downlinkCapacity")]
    downlink_capacity: u16,
    congestion: bool,
    #[serde(rename = "readBufferSize")]
    read_buffer_size: u16,
    #[serde(rename = "writeBufferSize")]
    write_buffer_size: u16,
    header: KcpHeader,
    seed: Option<String>,
}

impl Default for KcpSettings {
    fn default() -> Self {
        Self {
            mtu: 1350,
            tti: 20,
            uplink_capacity: 5,
            downlink_capacity: 20,
            congestion: false,
            read_buffer_size: 2,
            write_buffer_size: 2,
            header: KcpHeader::default(),
            seed: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct KcpHeader {
    r#type: KcpType,
    domain: String,
}

impl Default for KcpHeader {
    fn default() -> Self {
        Self {
            r#type: KcpType::default(),
            domain: "www.example.com".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum KcpType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "srtp")]
    Srtp,
    #[serde(rename = "utp")]
    Utp,
    #[serde(rename = "wechat-video")]
    WechatVideo,
    #[serde(rename = "dtls")]
    Dtls,
    #[serde(rename = "wireguard")]
    Wireguard,
    #[serde(rename = "dns")]
    Dns,
}

impl Default for KcpType {
    fn default() -> Self {
        KcpType::None
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct XrayConfig {
    log: XrayLog,
    inbounds: Vec<Inbound>,
    outbounds: Vec<Outbound>,
    routing: Routing,
}

impl XrayConfig {
    pub fn new(http_port: u16, socks_port: u16, models: Vec<Model>) -> Self {
        let outbounds: Vec<Outbound> = models.iter().map(|x| x.to_owned().into()).collect();
        let mut selector_outbound_tags = Vec::new();
        for outbound in outbounds.iter() {
            selector_outbound_tags.push(outbound.tag.clone())
        }
        Self {
            log: XrayLog::default(),
            inbounds: vec![
                Inbound::from_http_port(http_port, false),
                Inbound::from_socks_port(socks_port, false),
            ],
            outbounds,
            routing: Routing::new(selector_outbound_tags),
        }
    }

    pub fn set_log_path(&mut self, log_dir: PathBuf, log_level: Level) {
        self.log.access = log_dir.join("access.log").to_string_lossy().to_string();
        self.log.error = log_dir.join("error.log").to_string_lossy().to_string();
        let log_level = match log_level {
            Level::Debug => "debug",
            Level::Info => "info",
            Level::Warn => "warn",
            Level::Error => "error",
            _ => "debug",
        };
        self.log.loglevel = log_level.into();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct XrayLog {
    access: String,
    error: String,
    loglevel: String,
}

impl Default for XrayLog {
    fn default() -> Self {
        Self {
            access: "".into(),
            error: "".into(),
            loglevel: "info".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Inbound {
    tag: String,
    port: u16,
    protocol: String,
    listen: String,
    settings: InboundSettings,
}

impl Inbound {
    fn from_http_port(http_port: u16, tag_port: bool) -> Self {
        let tag = if tag_port {
            format!("http_ipv4_{}", http_port)
        } else {
            "http_ipv4".into()
        };
        Self {
            tag,
            port: http_port,
            protocol: "http".into(),
            listen: "0.0.0.0".into(),
            settings: InboundSettings::HttpInboundSettings(HttpInboundSettings::default()),
        }
    }

    fn from_socks_port(socks_port: u16, tag_port: bool) -> Self {
        let tag = if tag_port {
            format!("socks_ipv4_{}", socks_port)
        } else {
            "socks_ipv4".into()
        };
        Self {
            tag,
            port: socks_port,
            protocol: "socks".into(),
            listen: "0.0.0.0".into(),
            settings: InboundSettings::SocksInboundSettings(SocksInboundSettings::default()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum InboundSettings {
    #[serde(untagged)]
    HttpInboundSettings(HttpInboundSettings),
    #[serde(untagged)]
    SocksInboundSettings(SocksInboundSettings),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct HttpInboundSettings {
    timeout: u16,
    allow_transparent: bool,
}

impl Default for HttpInboundSettings {
    fn default() -> Self {
        Self {
            timeout: 300,
            allow_transparent: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct SocksInboundSettings {
    auth: String,
    udp: bool,
    ip: String,
}

impl Default for SocksInboundSettings {
    fn default() -> Self {
        Self {
            auth: "noauth".into(),
            udp: true,
            ip: "127.0.0.1".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Outbound {
    tag: String,
    protocol: String,
    settings: OutboundSettings,
    #[serde(rename = "streamSettings")]
    stream_settings: StreamSettings,
}

impl Outbound {
    pub fn new(
        tag: String,
        protocol: String,
        settings: OutboundSettings,
        stream_settings: StreamSettings,
    ) -> Self {
        Self {
            tag,
            protocol,
            settings,
            stream_settings,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum OutboundSettings {
    #[serde(rename = "vnext")]
    Vnext(Vec<Vnext>),
    #[serde(rename = "servers")]
    Servers(Vec<TrojanServer>),
}

impl OutboundSettings {
    pub fn from_vnexts(vnexts: Vec<Vnext>) -> Self {
        Self::Vnext(vnexts)
    }

    pub fn from_servers(servers: Vec<TrojanServer>) -> Self {
        Self::Servers(servers)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Vnext {
    address: String,
    port: u16,
    users: Vec<User>,
}

impl Vnext {
    pub fn new(address: String, port: u16, user: User) -> Self {
        Self {
            address,
            port,
            users: vec![user],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct User {
    id: String,
    encryption: String,
    flow: UserFlow,
}

impl User {
    pub fn new(uuid: String) -> Self {
        Self {
            id: uuid,
            encryption: "none".into(),
            flow: UserFlow::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum UserFlow {
    #[serde(rename = "")]
    None,
    #[serde(rename = "xtls-rprx-vision")]
    XtlsRprxVision,
    #[serde(rename = "xtls-rprx-vision-udp443")]
    XtlsRprxVisionUdp443,
}

impl Default for UserFlow {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Routing {
    #[serde(rename = "domainStrategy")]
    domain_strategy: String,
    rules: Vec<Rule>,
    balancers: Vec<Balancer>,
}

impl Routing {
    pub fn new(selector: Vec<String>) -> Self {
        Self {
            domain_strategy: "AsIs".into(),
            rules: vec![Rule::default()],
            balancers: vec![Balancer::new(selector)],
        }
    }

    pub fn empty() -> Self {
        Self {
            domain_strategy: "AsIs".into(),
            rules: vec![Rule::empty()],
            balancers: vec![],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Rule {
    r#type: String,
    #[serde(rename = "inboundTag")]
    inbound_tag: Vec<String>,
    #[serde(rename = "balancerTag")]
    balancer_tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "outboundTag")]
    outbound_tag: Option<Vec<String>>,
}

impl Rule {
    pub fn empty() -> Self {
        Self {
            r#type: "field".into(),
            inbound_tag: Vec::new(),
            balancer_tag: "balancer".into(),
            outbound_tag: Some(Vec::new()),
        }
    }

    fn add_inbound_tag(&mut self, tag: String) {
        self.inbound_tag.push(tag)
    }

    fn add_outbound_tag(&mut self, tag: String) {
        if self.outbound_tag.is_none() {
            self.outbound_tag = Some(vec![tag])
        } else {
            self.outbound_tag.as_mut().unwrap().push(tag)
        }
    }
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            r#type: "field".into(),
            inbound_tag: vec!["http_ipv4".into(), "socks_ipv4".into()],
            balancer_tag: "balancer".into(),
            outbound_tag: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Balancer {
    tag: String,
    selector: Vec<String>,
}

impl Balancer {
    pub fn new(selector: Vec<String>) -> Self {
        Self {
            tag: "balancer".into(),
            selector,
        }
    }
}

impl From<Model> for Outbound {
    fn from(source: Model) -> Self {
        let out_bound_settings = match source.protocol.to_value().as_str() {
            "trojan" => OutboundSettings::from_servers(vec![TrojanServer::new(
                source.address,
                source.port,
                source.uuid,
            )]),
            _ => {
                let user = User::new(source.uuid);
                OutboundSettings::from_vnexts(vec![Vnext::new(source.address, source.port, user)])
            }
        };
        Outbound::new(
            format!("proxy_{}", source.id),
            source.protocol.to_value(),
            out_bound_settings,
            source.stream_settings,
        )
    }
}

impl TryFrom<Url> for Model {
    type Error = anyhow::Error;
    fn try_from(url: Url) -> Result<Model> {
        let protocol = url.scheme();
        let uuid = Uuid::parse_str(url.username())?;
        let host_port = format!("{}:{}", url.host().unwrap(), url.port().unwrap());
        let address = url.domain().unwrap_or(host_port.as_str());
        let port = url.port().unwrap();
        let pairs: url::form_urlencoded::Parse<'_> = url.query_pairs();
        let stream_settings = StreamSettings::try_from(pairs)?;
        Ok(Self {
            id: Default::default(),
            name: "default".into(),
            protocol: Protocol::from_str(protocol)?,
            uuid: uuid.into(),
            address: address.into(),
            port,
            stream_settings,
            subscribe_id: None,
        })
    }
}

impl TryFrom<ShareWithProtocol> for Model {
    type Error = anyhow::Error;
    fn try_from(value: ShareWithProtocol) -> Result<Model> {
        let share = value.share;
        let network = share.net.clone();
        let uuid = share.id;
        let address = share.add;
        let port: u16 = share.port;
        let name = share.ps;
        let security: Security = Security::from_str(&share.tls)?;
        // let mut tls_settings = None;
        // let reality_settings  = None;
        // match security {
        //     Security::Tls => {
        //         tls_settings = Some(TLSSettings::new(true, share.host.clone()));
        //     }
        //     Security::Reality => {
        //         let fingerprint: String =
        //         // let server_name: String
        //         // let private_key: String
        //         // let short_id: String
        //         // let spider_x: String
        //         reality_settings = Some(RealitySettings::new())
        //     }
        //     Security::None => {

        //     }
        // }
        let tls_settings = match security {
            Security::Tls => Some(TLSSettings::new(true, share.host.clone())),
            _ => None,
        };
        // let tls_settings = TLSSettings::new(true, share.host.clone());
        let res = match network.as_str() {
            "ws" => {
                let ws_protocol: WebSocketProtocol = WebSocketProtocol::new(
                    share.net,
                    Some(security),
                    share.host,
                    Some(share.path),
                    tls_settings,
                    None,
                );
                Ok(StreamSettings::WebSocket(ws_protocol))
            }
            "tcp" => {
                let tcp_protocol: TcpProtocol =
                    TcpProtocol::new(share.net, Some(security), tls_settings, None);
                Ok(StreamSettings::Tcp(tcp_protocol))
            }
            "grpc" => {
                let grpc_protocol: GrpcProtocol =
                    GrpcProtocol::new(share.net, Some(security), tls_settings, None);
                Ok(StreamSettings::Grpc(grpc_protocol))
            }
            _ => Err(anyhow!("not support this protocol.")),
        };

        Ok(Self {
            id: Default::default(),
            name,
            protocol: Protocol::from_str(value.protocol.as_str())?,
            uuid,
            address,
            port,
            stream_settings: res?,
            subscribe_id: None,
        })
    }
}

impl FromStr for Model {
    type Err = anyhow::Error;
    fn from_str(url: &str) -> Result<Self> {
        let url = Url::parse(url)?;
        let username = url.username();
        if username == "" {
            let decode_bytes = general_purpose::STANDARD.decode(url.domain().unwrap())?;
            let share_json = String::from_utf8(decode_bytes).expect("Invalid UTF-8 sequence");
            let share_struct: ShareJsonStruct = serde_json::from_str(share_json.as_str())?;
            let share = ShareWithProtocol::new(url.scheme().into(), share_struct);
            Model::try_from(share)
        } else {
            Model::try_from(url)
        }
    }
}

impl Model {
    pub fn get_network_type(&self) -> &'static str {
        match self.stream_settings {
            StreamSettings::WebSocket(_) => "WebSocket",
            StreamSettings::Tcp(_) => "Tcp",
            StreamSettings::Http2(_) => "Http2",
            StreamSettings::Grpc(_) => "Grpc",
            StreamSettings::Kcp(_) => "Kcp",
            StreamSettings::Trojan(_) => "Trojan",
        }
    }

    pub fn get_server(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct TrojanProtocol {
    network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<Security>,
    #[serde(rename = "tlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_settings: Option<TLSSettings>,
}

impl TrojanProtocol {
    pub fn new(tls_settings: Option<TLSSettings>) -> Self {
        Self {
            network: "tcp".into(),
            security: Some(Security::None),
            tls_settings,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct TrojanServer {
    address: String,
    port: u16,
    password: String,
}

impl TrojanServer {
    pub fn new(address: String, port: u16, password: String) -> Self {
        Self {
            address,
            port,
            password,
        }
    }
}

impl XrayConfig {
    pub fn empty() -> Self {
        let outbounds: Vec<Outbound> = Vec::new();
        Self {
            log: XrayLog::default(),
            inbounds: Vec::new(),
            outbounds,
            routing: Routing::empty(),
        }
    }

    fn insert_inbound(&mut self, port: u16) {
        let inbound = Inbound::from_http_port(port, true);
        self.inbounds.push(inbound)
    }

    fn insert_outbound(&mut self, model: Model) {
        let outbound = Outbound::from(model);
        self.outbounds.push(outbound)
    }

    fn add_route(&mut self, inbound_tag: String, outbound_tag: String) {
        self.routing.rules[0].add_inbound_tag(inbound_tag);
        self.routing.rules[0].add_outbound_tag(outbound_tag);
    }

    pub fn from_models4http_delay(
        models: Vec<Model>,
        used_ports: &HashSet<u16>,
    ) -> (Self, HashMap<u16, i32>) {
        let mut port_model_dict = HashMap::new();
        let mut xray_config = XrayConfig::empty();
        for record in models.into_iter() {
            let port = get_random_port(used_ports).unwrap();
            port_model_dict.insert(port, record.id);
            xray_config.insert_inbound(port);
            let record_id = record.id;
            xray_config.insert_outbound(record);
            xray_config.add_route(
                format!("http_ipv4_{}", port),
                format!("proxy_{}", record_id),
            );
        }
        (xray_config, port_model_dict)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_add() {
        // let aa = r#"trojan://uuid@ip:60195?sni=address#aa"#;
        // let aa = r#"vless://80c39fa5-f23e-4ac1-96cb-ed06ea0e8f47@188.114.97.3:8080?type=ws&path=/Join--ELiV2Ray.El.V2ray.community&host=El.V2ray.Community.&security=none"#;
        let aa = r#"vmess://eyJ2IjoiMiIsInBzIjoiXHU1MjY5XHU0ZjU5XHU2ZDQxXHU5MWNmXHVmZjFhMzQ0NC4xMSBHQiIsImFkZCI6ImhrMDEwNi5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDEyNiIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiXHU1OTU3XHU5OTEwXHU1MjMwXHU2NzFmXHVmZjFhXHU5NTdmXHU2NzFmXHU2NzA5XHU2NTQ4IiwiYWRkIjoiaGswMTA2LmFsaWJhYmFva3ouY29tIiwicG9ydCI6IjYwMTI2IiwiaWQiOiIwZDM4NWM1Yi02MGM2LTRjMmMtOGE0Mi0zMTNmYjY3Y2Q2MGYiLCJhaWQiOiIwIiwibmV0IjoidGNwIiwidHlwZSI6Im5vbmUiLCJob3N0IjoiIiwicGF0aCI6IiIsInRscyI6IiJ9
vmess://eyJ2IjoiMiIsInBzIjoiXHU5MDFhXHU3N2U1XHU5ODc1OiBva3o4Lm1lIFx1NWJhMlx1NjcwZFx1NTcyOFx1NTNmM1x1NGUwYlx1ODlkMiIsImFkZCI6ImhrMDEwNi5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDEyNiIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiXHU1Yjk4XHU3ZjUxOiBva3psaXN0LmNvbSBcdTViYTJcdTY3MGRcdTU3MjhcdTUzZjNcdTRlMGJcdTg5ZDIiLCJhZGQiOiJoazAxMDYuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxMjYiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiXHU0ZTEzXHU3ZWJmMSIsImFkZCI6ImhrMDEwNi5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDEyNiIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiXHU0ZTEzXHU3ZWJmMiIsImFkZCI6ImhrMDEyNy5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDEyNiIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiXHU0ZTEzXHU3ZWJmMyIsImFkZCI6ImhrMDEwNy5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDIzNyIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiXHU0ZTEzXHU3ZWJmNCIsImFkZCI6ImhrMDE5NC5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDIzNyIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiXHU0ZTEzXHU3ZWJmNSIsImFkZCI6ImhrMDAwMi5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDAwMSIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiXHU0ZTEzXHU3ZWJmNiIsImFkZCI6ImhrMDAwMy5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDAwMSIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiXHU0ZTEzXHU3ZWJmNyIsImFkZCI6ImhrMDAwNC5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDIzOCIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiXHU0ZTEzXHU3ZWJmOCIsImFkZCI6ImhrMDAwNC5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDIzOCIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1OTk5OVx1NmUyZiIsImFkZCI6ImhrMDA3NC5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDA0OSIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1OTk5OVx1NmUyZiIsImFkZCI6ImhrMDE3Mi5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDA0OSIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
trojan://0d385c5b-60c6-4c2c-8a42-313fb67cd60f@hk00207.alibabaokz.com:61206?allowInsecure=1#Lv1%20%E9%A6%99%E6%B8%AF1
trojan://0d385c5b-60c6-4c2c-8a42-313fb67cd60f@hk00208.alibabaokz.com:61206?allowInsecure=1#Lv2%20%E9%A6%99%E6%B8%AF1
trojan://0d385c5b-60c6-4c2c-8a42-313fb67cd60f@hk00209.alibabaokz.com:61206?allowInsecure=1#Lv3%20%E9%A6%99%E6%B8%AF1
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1OTk5OVx1NmUyZjIiLCJhZGQiOiJoazAwNjEuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMjUiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1OTk5OVx1NmUyZjIiLCJhZGQiOiJoazAwMjcuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMjUiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1OTk5OVx1NmUyZjMiLCJhZGQiOiJoazAyNDAuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAyMzkiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1OTk5OVx1NmUyZjMiLCJhZGQiOiJoazAyNDEuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAyMzkiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1OTk5OVx1NmUyZjQiLCJhZGQiOiJoazAwNjcuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwNjYiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1OTk5OVx1NmUyZjQiLCJhZGQiOiJoazAwNjkuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwNjYiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1OTk5OVx1NmUyZjUiLCJhZGQiOiJoazAwNDYuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwNDUiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1OTk5OVx1NmUyZjUiLCJhZGQiOiJoazAwOTIuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwNDUiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1OTk5OVx1NmUyZjYiLCJhZGQiOiJoazAyMDguYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAyMDciLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1OTk5OVx1NmUyZjYiLCJhZGQiOiJoazAyMDkuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAyMDciLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1OTk5OVx1NmUyZjciLCJhZGQiOiJoazAyMDguYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAyNDMiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1OTk5OVx1NmUyZjciLCJhZGQiOiJoazAyMDkuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAyNDMiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NTNmMFx1NmU3ZTEiLCJhZGQiOiJ0dzAyNDguYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAyNDciLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NTNmMFx1NmU3ZTEiLCJhZGQiOiJ0dzAyNDkuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAyNDciLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NTNmMFx1NmU3ZTIiLCJhZGQiOiJ0dzAxODIuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxODEiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NTNmMFx1NmU3ZTIiLCJhZGQiOiJ0dzAxODMuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxODEiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NjVlNVx1NjcyYzAiLCJhZGQiOiJqcDAwODguYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiMTQzMTUiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NjVlNVx1NjcyYzAiLCJhZGQiOiJqcDAwOTAuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiMTQzMTUiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NjVlNVx1NjcyYzEiLCJhZGQiOiJqcDAxMDMuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMzAiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NjVlNVx1NjcyYzEiLCJhZGQiOiJqcDAxMDUuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMzAiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NjVlNVx1NjcyYzIiLCJhZGQiOiJqcDAxMTYuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxMTUiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NjVlNVx1NjcyYzIiLCJhZGQiOiJqcDAxMTguYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxMTUiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NjVlNVx1NjcyYzMiLCJhZGQiOiJqcDAxNTkuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMzQiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NjVlNVx1NjcyYzMiLCJhZGQiOiJqcDAxNjAuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMzQiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NjVlNVx1NjcyYzUiLCJhZGQiOiJqcDAwOTUuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMjkiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NjVlNVx1NjcyYzUiLCJhZGQiOiJqcDAwNTYuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMjkiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
trojan://0d385c5b-60c6-4c2c-8a42-313fb67cd60f@kr0196.alibabaokz.com:60195?allowInsecure=0&peer=kr-ora-195.okzdns.com&sni=kr-ora-195.okzdns.com#Lv1%20%E9%9F%A9%E5%9B%BD2
trojan://0d385c5b-60c6-4c2c-8a42-313fb67cd60f@kr0200.alibabaokz.com:60195?allowInsecure=0&peer=kr-ora-195.okzdns.com&sni=kr-ora-195.okzdns.com#Lv2%20%E9%9F%A9%E5%9B%BD2
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1OTdlOVx1NTZmZDQiLCJhZGQiOiJrcjAyMDQuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxOTciLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1OTdlOVx1NTZmZDQiLCJhZGQiOiJrcjAyMDMuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxOTciLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1OTdlOVx1NTZmZDUiLCJhZGQiOiJrcjAxOTkuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxOTgiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1OTdlOVx1NTZmZDUiLCJhZGQiOiJrcjAyMDUuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxOTgiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NjViMFx1NTJhMFx1NTc2MTEiLCJhZGQiOiJzZzAxMTIuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwNDEiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NjViMFx1NTJhMFx1NTc2MTEiLCJhZGQiOiJzZzAxMTQuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwNDEiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NjViMFx1NTJhMFx1NTc2MTIiLCJhZGQiOiJzZzAxMDAuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMzEiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NjViMFx1NTJhMFx1NTc2MTIiLCJhZGQiOiJzZzAxMDIuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMzEiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NjViMFx1NTJhMFx1NTc2MTMiLCJhZGQiOiJzZzAxNjUuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMjEiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NjViMFx1NTJhMFx1NTc2MTMiLCJhZGQiOiJzZzAxNjYuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMjEiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NjViMFx1NTJhMFx1NTc2MTQiLCJhZGQiOiJzZzAxNjguYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMjIiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NjViMFx1NTJhMFx1NTc2MTQiLCJhZGQiOiJzZzAxNjkuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwMjIiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NjViMFx1NTJhMFx1NTc2MTYiLCJhZGQiOiJzZzAxMjAuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxMTkiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NjViMFx1NTJhMFx1NTc2MTYiLCJhZGQiOiJzZzAxMjEuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxMTkiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1ODJmMVx1NTZmZDEiLCJhZGQiOiJldTAwOTcuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNDY3MTkiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1ODJmMVx1NTZmZDEiLCJhZGQiOiJldTAwOTguYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNDY3MTkiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1ODJmMVx1NTZmZDIiLCJhZGQiOiJldTAxMzYuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwNDIiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1ODJmMVx1NTZmZDIiLCJhZGQiOiJldTAxMzcuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAwNDIiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1N2Y4ZVx1NTZmZDEiLCJhZGQiOiJ1czAxNzUuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxNzQiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1N2Y4ZVx1NTZmZDEiLCJhZGQiOiJ1czAxNzYuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxNzQiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1N2Y4ZVx1NTZmZDMiLCJhZGQiOiJ1czAxNDEuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxNDAiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1N2Y4ZVx1NTZmZDMiLCJhZGQiOiJ1czAxNDIuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxNDAiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1N2Y4ZVx1NTZmZDQiLCJhZGQiOiJ1czAxNDcuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxNTciLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1N2Y4ZVx1NTZmZDQiLCJhZGQiOiJ1czAxNDIuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxNTciLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1N2Y4ZVx1NTZmZDUiLCJhZGQiOiJ1czAxNTAuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxNDUiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1N2Y4ZVx1NTZmZDUiLCJhZGQiOiJ1czAxNTEuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxNDUiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1N2Y4ZVx1NTZmZDYiLCJhZGQiOiJ1czAxNTMuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxNDYiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1N2Y4ZVx1NTZmZDYiLCJhZGQiOiJ1czAxNTQuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAxNDYiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1OGZlYVx1NjJkYyIsImFkZCI6ImRiMDEwOS5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDEwOCIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1OGZlYVx1NjJkYyIsImFkZCI6ImRiMDExMS5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDEwOCIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1OGQ4YVx1NTM1NyIsImFkZCI6InluMDA3NS5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDA0MCIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1OGQ4YVx1NTM1NyIsImFkZCI6InluMDA3Ni5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDA0MCIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYzIFx1OGQ4YVx1NTM1NyIsImFkZCI6InluMDA3Ny5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDA0MCIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1ODM3N1x1NTE3MCIsImFkZCI6ImhsMDA3OC5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDA0NCIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTFYyIFx1ODM3N1x1NTE3MCIsImFkZCI6ImhsMDA3OS5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDA0NCIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1ODM3N1x1NTE3MDIiLCJhZGQiOiJobDAyNTIuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAyNTEiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1ODM3N1x1NTE3MDIiLCJhZGQiOiJobDAyNTMuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAyNTEiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1OWE2Y1x1Njc2NVx1ODk3Zlx1NGU5YSB4NSIsImFkZCI6Im1sMDA3MC5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDAyNCIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1OWE2Y1x1Njc2NVx1ODk3Zlx1NGU5YSB4NSIsImFkZCI6Im1sMDA3MS5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDAyNCIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NTM3MFx1NWMzYyIsImFkZCI6InluMDAzOC5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDAzNyIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NTM3MFx1NWMzYyIsImFkZCI6InluMDA1MC5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDAzNyIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NTcxZlx1ODAzM1x1NTE3NiBYNSIsImFkZCI6InR1MDA4Mi5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDA4MSIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NTcxZlx1ODAzM1x1NTE3NiBYNSIsImFkZCI6InR1MDA4My5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDA4MSIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NTcxZlx1ODAzM1x1NTE3NjIgXHU1MzlmXHU3NTFmIiwiYWRkIjoidHUwMTg3LmFsaWJhYmFva3ouY29tIiwicG9ydCI6IjYwMTg2IiwiaWQiOiIwZDM4NWM1Yi02MGM2LTRjMmMtOGE0Mi0zMTNmYjY3Y2Q2MGYiLCJhaWQiOiIwIiwibmV0IjoidGNwIiwidHlwZSI6Im5vbmUiLCJob3N0IjoiIiwicGF0aCI6IiIsInRscyI6IiJ9
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NTcxZlx1ODAzM1x1NTE3NjIgXHU1MzlmXHU3NTFmIiwiYWRkIjoidHUwMTg4LmFsaWJhYmFva3ouY29tIiwicG9ydCI6IjYwMTg2IiwiaWQiOiIwZDM4NWM1Yi02MGM2LTRjMmMtOGE0Mi0zMTNmYjY3Y2Q2MGYiLCJhaWQiOiIwIiwibmV0IjoidGNwIiwidHlwZSI6Im5vbmUiLCJob3N0IjoiIiwicGF0aCI6IiIsInRscyI6IiJ9
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1NTM3MFx1NWVhNiIsImFkZCI6ImluMDIxMi5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDIxMSIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1NTM3MFx1NWVhNiIsImFkZCI6ImluMDIxMy5hbGliYWJhb2t6LmNvbSIsInBvcnQiOiI2MDIxMSIsImlkIjoiMGQzODVjNWItNjBjNi00YzJjLThhNDItMzEzZmI2N2NkNjBmIiwiYWlkIjoiMCIsIm5ldCI6InRjcCIsInR5cGUiOiJub25lIiwiaG9zdCI6IiIsInBhdGgiOiIiLCJ0bHMiOiIifQ==
vmess://eyJ2IjoiMiIsInBzIjoiTHYxIFx1OTYzZlx1NjgzOVx1NWVmNyB4MTAiLCJhZGQiOiJhZ3QwMjE3LmFsaWJhYmFva3ouY29tIiwicG9ydCI6IjYwMjE2IiwiaWQiOiIwZDM4NWM1Yi02MGM2LTRjMmMtOGE0Mi0zMTNmYjY3Y2Q2MGYiLCJhaWQiOiIwIiwibmV0IjoidGNwIiwidHlwZSI6Im5vbmUiLCJob3N0IjoiIiwicGF0aCI6IiIsInRscyI6IiJ9
vmess://eyJ2IjoiMiIsInBzIjoiTHYyIFx1OTYzZlx1NjgzOVx1NWVmNyB4MTAiLCJhZGQiOiJhZ3QwMjE4LmFsaWJhYmFva3ouY29tIiwicG9ydCI6IjYwMjE2IiwiaWQiOiIwZDM4NWM1Yi02MGM2LTRjMmMtOGE0Mi0zMTNmYjY3Y2Q2MGYiLCJhaWQiOiIwIiwibmV0IjoidGNwIiwidHlwZSI6Im5vbmUiLCJob3N0IjoiIiwicGF0aCI6IiIsInRscyI6IiJ9
vmess://eyJ2IjoiMiIsInBzIjoiXHU2Y2YwXHU1NzBiLVx1NjZmY1x1OGMzNyBYMTAiLCJhZGQiOiJ0ZzAyMjYuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAyMjUiLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
vmess://eyJ2IjoiMiIsInBzIjoiXHU4M2YyXHU1ZjhiXHU4Y2QzIFgxMCIsImFkZCI6ImZsYjAyMjcuYWxpYmFiYW9rei5jb20iLCJwb3J0IjoiNjAyMjciLCJpZCI6IjBkMzg1YzViLTYwYzYtNGMyYy04YTQyLTMxM2ZiNjdjZDYwZiIsImFpZCI6IjAiLCJuZXQiOiJ0Y3AiLCJ0eXBlIjoibm9uZSIsImhvc3QiOiIiLCJwYXRoIjoiIiwidGxzIjoiIn0=
trojan://0d385c5b-60c6-4c2c-8a42-313fb67cd60f@ru0195.alibabaokz.com:60194?allowInsecure=1&peer=russia.okzdns.com&sni=russia.okzdns.com#%E4%BF%84%E7%BD%97%E6%96%AF
"#;
        // let aa = r#"vless://»»»»DIGIV2RAY««««@www.Speedtest.Net:2095?path=/DIGIV2RAY--DigiV2ray--digiv2ray--DIGIV2RAY--DigiV2ray--digiv2ray--DigiV2ray--DigiV2ray?ed=1024&security=none&encryption=none&host=www.speedtest.net.ftp.debian.org.xn--ihqvla424c49bba047b50okggl0rcfo5o3aus3a.website.&type=ws"#;
        // let aa = r#"{"v":"2","ps":"D-BROWN-1025","add":"157.245.4.170","port":"8881","id":"db5afae4-ac23-41a6-8378-f307a9a47436","aid":"0","scy":"auto","net":"tcp","type":"http","host":"mihanwebhost.com","path":"/","tls":"none","sni":"","alpn":""}"#;
        for line in aa.lines() {
            let model = Model::from_str(line).unwrap();
        }

        // println!("model: {:?}", model);
        // let stream_settings = serde_json::to_string(&model.stream_settings);
        // println!("stream_settings: {:?}", stream_settings);

        // let xrray_config = XrayConfig::new(10086, 10087, vec![model]);
        // fs::write(
        //     "output.json",
        //     serde_json::to_string_pretty(&xrray_config).unwrap(),
        // )
        // .unwrap();
    }
}
