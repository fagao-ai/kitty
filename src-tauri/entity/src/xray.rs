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
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebSocketProtocol {
    network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<Security>,
    #[serde(rename = "tlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_settings: Option<TLSSettings>,
    #[serde(rename = "wsSettings")]
    ws_settings: WsSettings,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TcpProtocol {
    network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    security: Option<Security>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tlsSettings")]
    tls_settings: Option<TLSSettings>,
    #[serde(rename = "tcpSettings")]
    tcp_settings: TcpSettings,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TLSSettings {
    #[serde(rename = "allowInsecure")]
    allow_insecure: bool,
    #[serde(rename = "serverName")]
    server_name: String,
    fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct WsSettings {
    path: String,
    headers: Headers,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Headers {
    #[serde(rename = "Host")]
    host: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TcpSettings {
    header: TcpHeader,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TcpHeader {
    r#type: TcpType,
    #[serde(skip_serializing_if = "Option::is_none")]
    request: Option<TcpRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<TcpResponse>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum TcpType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "http")]
    Http,
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
    #[serde(rename = "tcpSettings")]
    grpc_settings: GrpcSettings,
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
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub struct XrayConfig {
    log: XrayLog,
    inbounds: Vec<Inbound>,
    outbounds: Vec<Outbound>,
}

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

struct Inbound {
    port: u16,
    protocol: String,
    listen: String,
    settings: InboundSettings,
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
    protocol: String,
    settings: OutboundSettings,
    stream_settings: StreamSettings,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct OutboundSettings {
    vnext: Vec<Vnext>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Vnext {
    address: String,
    port: u16,
    users: Vec<User>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct User {
    id: String,
    encryption: String,
    flow: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum UserFlow {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "xtls-rprx-vision")]
    XtlsRprxVision,
    #[serde(rename = "xtls-rprx-vision-udp443")]
    XtlsRprxVisionUdp443,
}

struct Routing {
    domain_strategy: String,
    domain_matcher: String,
    balancers: Vec<String>,
}


struct Balancer{
    
}