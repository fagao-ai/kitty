use anyhow::Error;
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use url::Url;

use crate::types::ShareJsonStruct;
use crate::types::ShareWithProtocol;

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel, FromJsonQueryResult,
)]
#[sea_orm(table_name = "xray")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    #[serde(skip)]
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
        let security = query_params
            .get("security")
            .map(|x| x.as_str())
            .unwrap_or("");
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
        let security: Security = Security::from_str(security)?;
        let tls_settings = TLSSettings::new(allow_insecure, server_name.into());
        match r#type {
            "ws" => {
                let ws_protocol: WebSocketProtocol = WebSocketProtocol::new(
                    r#type.into(),
                    Some(security),
                    host,
                    Some(path.into()),
                    Some(tls_settings),
                );
                Ok(StreamSettings::WebSocket(ws_protocol))
            }
            "tcp" => {
                let tcp_protocol: TcpProtocol =
                    TcpProtocol::new(r#type.into(), Some(security), Some(tls_settings));
                Ok(StreamSettings::Tcp(tcp_protocol))
            }
            _ => Err(anyhow!("convert stream_settings failed.")),
        }
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
    ) -> Self {
        let ws_settings = WsSettings::new(host, path);
        Self {
            network,
            security,
            tls_settings,
            ws_settings,
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
    #[serde(rename = "tcpSettings")]
    tcp_settings: TcpSettings,
}

impl TcpProtocol {
    fn new(network: String, security: Option<Security>, tls_settings: Option<TLSSettings>) -> Self {
        let tcp_settings = TcpSettings::default();
        Self {
            network,
            security,
            tls_settings,
            tcp_settings,
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
                Inbound::from_http_port(http_port),
                Inbound::from_socks_port(socks_port),
            ],
            outbounds,
            routing: Routing::new(selector_outbound_tags),
        }
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
    fn from_http_port(http_port: u16) -> Self {
        Self {
            tag: "http_ipv4".into(),
            port: http_port,
            protocol: "http".into(),
            listen: "0.0.0.0".into(),
            settings: InboundSettings::HttpInboundSettings(HttpInboundSettings::default()),
        }
    }

    fn from_socks_port(http_port: u16) -> Self {
        Self {
            tag: "socks_ipv4".into(),
            port: http_port,
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
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Rule {
    r#type: String,
    #[serde(rename = "inboundTag")]
    inbound_tag: Vec<String>,
    #[serde(rename = "balancerTag")]
    bclancer_tag: String,
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            r#type: "field".into(),
            inbound_tag: vec!["http_ipv4".into(), "socks_ipv4".into()],
            bclancer_tag: "balancer".into(),
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
            source.name,
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
        let uuid = url.username();
        let address = url.domain().unwrap();
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
        let port: u16 = share.port.parse().unwrap();
        let name = share.ps;
        let security: Security = Security::from_str("tls")?;
        let tls_settings = TLSSettings::new(true, share.host.clone());
        let res = match network.as_str() {
            "ws" => {
                let ws_protocol: WebSocketProtocol = WebSocketProtocol::new(
                    share.net,
                    Some(security),
                    share.host,
                    Some(share.path),
                    Some(tls_settings),
                );
                Ok(StreamSettings::WebSocket(ws_protocol))
            }
            "tcp" => {
                let tcp_protocol: TcpProtocol =
                    TcpProtocol::new(share.net, Some(security), Some(tls_settings));
                Ok(StreamSettings::Tcp(tcp_protocol))
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
        println!("uuid_or_base64: {username}");
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_add() {
        let aa = r#"trojan://uuid@ip:60195?sni=address#aa"#;
        let model = Model::from_str(aa).unwrap();
        println!("{:?}", model);

        let stream_settings = serde_json::to_string(&model.stream_settings);
        println!("stream_settings: {:?}", stream_settings);

        let xrray_config = XrayConfig::new(10086, 10087, vec![model]);
        fs::write(
            "output.json",
            serde_json::to_string_pretty(&xrray_config).unwrap(),
        )
        .unwrap();
    }
}
