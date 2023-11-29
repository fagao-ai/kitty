enum ProtocolType {
    Vless,
    Vmess,
}

struct ServerConfig {
    protocol_type: ProtocolType,
    address: String,
    port: i32,
}
enum Network {
    Websocket,
}
enum Security {
    Tls,
    Xtls,
}

struct NetworkSecutiy {
    security: Security,
    allow_insecure: bool,
    tls_service_name: String,
}

struct TransportConfig {
    network: Network,
    hohst: String,
    path: String,
}

struct Protocol {
    sever_config: ServerConfig,
    transport_config: TransportConfig,
}

pub struct Config {
    protocol: Protocol,
}
