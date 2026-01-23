//! Config converter module for converting hysteria configs to shoes YAML format.
//!
//! This module provides conversion functions to transform hysteria
//! database entities into shoes-compatible YAML configurations.

use anyhow::{anyhow, Result};
use entity::xray::{Protocol as XrayProtocol, Security as XraySecurity, StreamSettings};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use entity::{hysteria, xray};

/// Represents a single shoes server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShoesConfig {
    /// TCP server (HTTP/SOCKS5/Mixed proxy)
    Server(TcpServerConfig),
    /// TUN server (VPN mode)
    TunServer(TunServerConfig),
}

/// TCP server configuration for local HTTP/SOCKS5/Mixed proxy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpServerConfig {
    /// Bind address for the local proxy server
    pub address: String,
    /// Protocol configuration
    pub protocol: ServerProtocol,
    /// Routing rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    /// Geo routing configuration for Clash-style traffic diversion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_routing: Option<GeoRoutingConfig>,
}

/// TUN server configuration for VPN mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunServerConfig {
    /// TUN device name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// TUN device address
    pub address: String,
    /// Netmask for the TUN device
    pub netmask: String,
    /// Maximum transmission unit
    pub mtu: usize,
    /// Enable TCP
    pub tcp_enabled: bool,
    /// Enable UDP
    pub udp_enabled: bool,
    /// Enable ICMP
    pub icmp_enabled: bool,
    /// Routing rules
    pub rules: Vec<Rule>,
    /// Geo routing configuration for Clash-style traffic diversion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_routing: Option<GeoRoutingConfig>,
}

/// Server protocol type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ServerProtocol {
    /// HTTP proxy server
    Http {
        #[serde(skip_serializing_if = "Option::is_none")]
        udp_enabled: Option<bool>,
    },
    /// SOCKS5 proxy server
    Socks {
        #[serde(skip_serializing_if = "Option::is_none")]
        udp_enabled: Option<bool>,
    },
    /// Mixed HTTP/SOCKS5 server (auto-detects protocol)
    Mixed {
        #[serde(skip_serializing_if = "Option::is_none")]
        udp_enabled: Option<bool>,
    },
}

/// Routing rule for proxy server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// IP/hostname masks to match
    pub masks: String,
    /// Action to take (allow/block)
    pub action: String,
    /// Client chain for upstream proxy - single hop serialized inline
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_chain: Option<ClientChainHop>,
}

/// A single hop in the client proxy chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientChainHop {
    /// Upstream server address
    pub address: String,
    /// Protocol configuration
    pub protocol: ClientProtocol,
    /// Transport configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
    /// QUIC settings (when transport is quic)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quic_settings: Option<QuicSettings>,
}

/// QUIC settings for client connections.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuicSettings {
    /// SNI hostname for TLS
    pub sni_hostname: String,
    /// Whether to verify the server certificate
    pub verify: bool,
    /// ALPN protocols
    pub alpn_protocols: String,
}

/// Client protocol for connecting to upstream proxy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ClientProtocol {
    /// Direct connection (no proxy)
    Direct,
    /// VLESS protocol
    #[serde(rename = "vless")]
    Vless {
        user_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        udp_enabled: Option<bool>,
    },
    /// VMess protocol
    #[serde(rename = "vmess")]
    Vmess {
        user_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        udp_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cipher: Option<String>,
    },
    /// Trojan protocol
    #[serde(rename = "trojan")]
    Trojan {
        password: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        udp_enabled: Option<bool>,
    },
    /// Hysteria2 protocol
    #[serde(rename = "hysteria2")]
    Hysteria2 {
        password: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        udp_enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fast_open: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        bandwidth: Option<Bandwidth>,
    },
    /// TLS protocol wrapper (for TLS over TCP/WebSocket)
    #[serde(rename = "tls")]
    Tls {
        #[serde(skip_serializing_if = "Option::is_none")]
        verify: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        sni_hostname: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        protocol: Option<Box<ClientProtocol>>,
    },
    /// WebSocket protocol wrapper
    #[serde(rename = "ws")]
    Ws {
        #[serde(skip_serializing_if = "Option::is_none")]
        matching_path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        matching_headers: Option<HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        protocol: Option<Box<ClientProtocol>>,
    },
    /// Reality protocol wrapper (for Reality with VLESS)
    #[serde(rename = "reality")]
    Reality {
        public_key: String,
        short_id: String,
        sni_hostname: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        vision: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        protocol: Option<Box<ClientProtocol>>,
    },
}

/// Bandwidth configuration for Hysteria2.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bandwidth {
    pub up: String,
    pub down: String,
}

/// Geo routing configuration for Clash-style traffic diversion.
///
/// This matches the shoes library's GeoRoutingConfig structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoRoutingConfig {
    /// Path to GeoIP.dat file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geoip_file: Option<String>,
    /// Path to GeoSite.dat file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geosite_file: Option<String>,
}

impl GeoRoutingConfig {
    /// Create a new GeoRoutingConfig with paths to the .dat files.
    ///
    /// Uses the same path resolution as GeoDataManager:
    /// - First tries CARGO_MANIFEST_DIR/static/
    /// - Falls back to CARGO_MANIFEST_DIR
    /// - Falls back to OUT_DIR
    pub fn new() -> Self {
        let project_dir = std::env::var("CARGO_MANIFEST_DIR")
            .or_else(|_| std::env::var("OUT_DIR"))
            .unwrap_or_else(|_| ".".to_string());

        let project_path = std::path::Path::new(&project_dir);
        let static_dir = project_path.join("static");
        let base_dir = if static_dir.exists() {
            static_dir
        } else {
            project_path.to_path_buf()
        };

        Self {
            geoip_file: Some(base_dir.join("kitty_geoip.dat").to_string_lossy().to_string()),
            geosite_file: Some(base_dir.join("kitty_geosite.dat").to_string_lossy().to_string()),
        }
    }
}

/// Main converter for transforming hysteria configs to shoes YAML.
pub struct ShoesConfigConverter;

impl ShoesConfigConverter {
    /// Convert a hysteria Model to shoes YAML configs for SOCKS5/HTTP proxy mode.
    pub fn hysteria_to_socks_http_yaml(
        model: &hysteria::Model,
        http_port: u16,
        socks_port: u16,
    ) -> Result<String> {
        // Build the Hysteria2 client protocol
        let client_protocol = ClientProtocol::Hysteria2 {
            password: model.auth.clone(),
            udp_enabled: Some(true),
            fast_open: Some(true),
            bandwidth: Some(Bandwidth {
                up: model.bandwidth.up.clone(),
                down: model.bandwidth.down.clone(),
            }),
        };

        // Create the client chain hop with QUIC transport
        let client_chain = ClientChainHop {
            address: model.server.clone(),
            protocol: client_protocol,
            transport: Some("quic".to_string()),
            quic_settings: Some(QuicSettings {
                sni_hostname: "bing.com".to_string(),
                verify: false,
                alpn_protocols: "h3".to_string(),
            }),
        };

        // Create geo routing config
        let geo_routing = Some(GeoRoutingConfig::new());

        // Create HTTP proxy config
        let http_config = TcpServerConfig {
            address: format!("127.0.0.1:{http_port}"),
            protocol: ServerProtocol::Http {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(client_chain.clone()),
            }]),
            geo_routing: geo_routing.clone(),
        };

        // Create SOCKS5 proxy config
        let socks_config = TcpServerConfig {
            address: format!("127.0.0.1:{socks_port}"),
            protocol: ServerProtocol::Socks {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(client_chain),
            }]),
            geo_routing,
        };

        // Serialize to YAML
        let configs = vec![ShoesConfig::Server(http_config), ShoesConfig::Server(socks_config)];
        let yaml = serde_yaml::to_string(&configs).map_err(|e| anyhow!("Failed to serialize YAML: {}", e))?;

        // Debug: Print the generated YAML configuration
        println!("=== Generated Hysteria2 YAML Configuration ===");
        println!("{}", yaml);
        println!("=== End of YAML Configuration ===");

        Ok(yaml)
    }

    /// Convert a hysteria Model to shoes YAML config for TUN/VPN mode.
    #[allow(dead_code)]
    pub fn hysteria_to_tun_yaml(
        model: &hysteria::Model,
        tun_address: String,
        tun_netmask: String,
    ) -> Result<String> {
        // Build the Hysteria2 client protocol
        let client_protocol = ClientProtocol::Hysteria2 {
            password: model.auth.clone(),
            udp_enabled: Some(true),
            fast_open: Some(true),
            bandwidth: Some(Bandwidth {
                up: model.bandwidth.up.clone(),
                down: model.bandwidth.down.clone(),
            }),
        };

        // Create the client chain hop with QUIC transport
        let client_chain = ClientChainHop {
            address: model.server.clone(),
            protocol: client_protocol,
            transport: Some("quic".to_string()),
            quic_settings: Some(QuicSettings {
                sni_hostname: "bing.com".to_string(),
                verify: false,
                alpn_protocols: "h3".to_string(),
            }),
        };

        // Create TUN config
        let tun_config = TunServerConfig {
            device_name: Some("tun0".to_string()),
            address: tun_address,
            netmask: tun_netmask,
            mtu: 1500,
            tcp_enabled: true,
            udp_enabled: true,
            icmp_enabled: true,
            rules: vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(client_chain),
            }],
            geo_routing: Some(GeoRoutingConfig::new()),
        };

        // Serialize to YAML
        let configs = vec![ShoesConfig::TunServer(tun_config)];
        serde_yaml::to_string(&configs).map_err(|e| anyhow!("Failed to serialize YAML: {}", e))
    }

    /// Convert an xray Model to shoes YAML configs for SOCKS5/HTTP proxy mode.
    pub fn xray_to_socks_http_yaml(
        model: &xray::Model,
        http_port: u16,
        socks_port: u16,
    ) -> Result<String> {
        // Build the base client protocol from the xray config
        let stream_settings = model.stream_settings();
        let client_protocol = Self::build_client_protocol(model, stream_settings)?;

        // Build the client chain hop
        let client_chain = Self::build_client_chain(model, stream_settings, client_protocol)?;

        // Create geo routing config
        let geo_routing = Some(GeoRoutingConfig::new());

        // Create HTTP proxy config
        let http_config = TcpServerConfig {
            address: format!("127.0.0.1:{http_port}"),
            protocol: ServerProtocol::Http {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(client_chain.clone()),
            }]),
            geo_routing: geo_routing.clone(),
        };

        // Create SOCKS5 proxy config
        let socks_config = TcpServerConfig {
            address: format!("127.0.0.1:{socks_port}"),
            protocol: ServerProtocol::Socks {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(client_chain),
            }]),
            geo_routing,
        };

        // Serialize to YAML
        let configs = vec![ShoesConfig::Server(http_config), ShoesConfig::Server(socks_config)];
        let yaml = serde_yaml::to_string(&configs).map_err(|e| anyhow!("Failed to serialize YAML: {}", e))?;

        // Debug: Print the generated YAML configuration
        println!("=== Generated XRay YAML Configuration ===");
        println!("{}", yaml);
        println!("=== End of YAML Configuration ===");

        Ok(yaml)
    }

    /// Convert an xray Model to shoes YAML config for TUN/VPN mode.
    #[allow(dead_code)]
    pub fn xray_to_tun_yaml(
        model: &xray::Model,
        tun_address: String,
        tun_netmask: String,
    ) -> Result<String> {
        // Build the base client protocol from the xray config
        let stream_settings = model.stream_settings();
        let client_protocol = Self::build_client_protocol(model, stream_settings)?;

        // Build the client chain hop
        let client_chain = Self::build_client_chain(model, stream_settings, client_protocol)?;

        // Create TUN config
        let tun_config = TunServerConfig {
            device_name: Some("tun0".to_string()),
            address: tun_address,
            netmask: tun_netmask,
            mtu: 1500,
            tcp_enabled: true,
            udp_enabled: true,
            icmp_enabled: true,
            rules: vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(client_chain),
            }],
            geo_routing: Some(GeoRoutingConfig::new()),
        };

        // Serialize to YAML
        let configs = vec![ShoesConfig::TunServer(tun_config)];
        serde_yaml::to_string(&configs).map_err(|e| anyhow!("Failed to serialize YAML: {}", e))
    }

    /// Convert multiple xray records to a single shoes YAML config.
    pub fn xray_multi_to_yaml(
        models: &[xray::Model],
        http_port: u16,
        socks_port: u16,
    ) -> Result<String> {
        if models.is_empty() {
            return Err(anyhow!("No xray models provided"));
        }

        // Use the first model for the primary config
        let first_model = &models[0];
        let stream_settings = first_model.stream_settings();
        let client_protocol = Self::build_client_protocol(first_model, stream_settings)?;
        let client_chain = Self::build_client_chain(first_model, stream_settings, client_protocol)?;

        // Create geo routing config
        let geo_routing = Some(GeoRoutingConfig::new());

        // Create HTTP proxy config
        let http_config = TcpServerConfig {
            address: format!("127.0.0.1:{http_port}"),
            protocol: ServerProtocol::Http {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(client_chain.clone()),
            }]),
            geo_routing: geo_routing.clone(),
        };

        // Create SOCKS5 proxy config
        let socks_config = TcpServerConfig {
            address: format!("127.0.0.1:{socks_port}"),
            protocol: ServerProtocol::Socks {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(client_chain),
            }]),
            geo_routing,
        };

        // Serialize to YAML
        let configs = vec![ShoesConfig::Server(http_config), ShoesConfig::Server(socks_config)];
        serde_yaml::to_string(&configs).map_err(|e| anyhow!("Failed to serialize YAML: {}", e))
    }

    /// Build the client protocol from an xray model and stream settings.
    fn build_client_protocol(
        model: &xray::Model,
        stream_settings: &StreamSettings,
    ) -> Result<ClientProtocol> {
        // First, build the base protocol based on the xray protocol type
        let base_protocol = match model.protocol {
            XrayProtocol::Vless => ClientProtocol::Vless {
                user_id: model.uuid.clone(),
                udp_enabled: Some(true),
            },
            XrayProtocol::Vmess => ClientProtocol::Vmess {
                user_id: model.uuid.clone(),
                udp_enabled: Some(true),
                cipher: Some("any".to_string()),
            },
            XrayProtocol::Trojan => ClientProtocol::Trojan {
                password: model.uuid.clone(),
                udp_enabled: Some(true),
            },
        };

        // Now wrap with security/transport layers based on stream settings
        let protocol = Self::wrap_protocol_with_transport(base_protocol, stream_settings)?;

        Ok(protocol)
    }

    /// Wrap the base protocol with transport and security layers.
    fn wrap_protocol_with_transport(
        base_protocol: ClientProtocol,
        stream_settings: &StreamSettings,
    ) -> Result<ClientProtocol> {
        let network = stream_settings.network();
        let security = stream_settings.security();

        // For WebSocket, we need to wrap with WS first
        let protocol = if network == "ws" {
            let mut headers = HashMap::new();
            if let Some(host) = stream_settings.ws_host() {
                headers.insert("Host".to_string(), host.to_string());
            }

            ClientProtocol::Ws {
                matching_path: stream_settings.ws_path().map(|s| s.to_string()),
                matching_headers: if headers.is_empty() { None } else { Some(headers) },
                protocol: Some(Box::new(base_protocol)),
            }
        } else {
            base_protocol
        };

        // Apply security layer (TLS or Reality)
        let protocol = match security {
            Some(XraySecurity::Tls) => {
                let tls_settings = stream_settings.tls_settings()
                    .ok_or_else(|| anyhow!("TLS security specified but no TLS settings found"))?;
                ClientProtocol::Tls {
                    verify: Some(!tls_settings.allow_insecure()),
                    sni_hostname: Some(tls_settings.server_name().to_string()),
                    protocol: Some(Box::new(protocol)),
                }
            }
            Some(XraySecurity::Reality) => {
                let reality_settings = stream_settings.reality_settings()
                    .ok_or_else(|| anyhow!("Reality security specified but no Reality settings found"))?;
                ClientProtocol::Reality {
                    public_key: reality_settings.public_key().to_string(),
                    short_id: reality_settings.short_id().to_string(),
                    sni_hostname: reality_settings.server_name().to_string(),
                    vision: None,
                    protocol: Some(Box::new(protocol)),
                }
            }
            Some(XraySecurity::None) | None => protocol,
        };

        Ok(protocol)
    }

    /// Build a client chain hop from an xray model and stream settings.
    fn build_client_chain(
        model: &xray::Model,
        stream_settings: &StreamSettings,
        protocol: ClientProtocol,
    ) -> Result<ClientChainHop> {
        let network = stream_settings.network();
        let security = stream_settings.security();

        // Determine transport type
        let transport = match (network, security) {
            ("ws", Some(XraySecurity::Tls)) => Some("tcp".to_string()),  // WSS uses TCP transport
            ("ws", _) => Some("tcp".to_string()),  // WS uses TCP transport
            ("tcp", _) | ("grpc", _) | ("http", _) | ("kcp", _) => Some("tcp".to_string()),
            _ => None,
        };

        Ok(ClientChainHop {
            address: format!("{}:{}", model.address, model.port),
            protocol,
            transport,
            quic_settings: None,  // QUIC is only used for Hysteria2
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_simple_config() {
        let config = TcpServerConfig {
            address: "127.0.0.1:1080".to_string(),
            protocol: ServerProtocol::Socks {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(ClientChainHop {
                    address: "155.248.218.187:10086".to_string(),
                    protocol: ClientProtocol::Hysteria2 {
                        password: "test123".to_string(),
                        udp_enabled: Some(true),
                        fast_open: Some(true),
                        bandwidth: Some(Bandwidth {
                            up: "100 mbps".to_string(),
                            down: "200 mbps".to_string(),
                        }),
                    },
                    transport: Some("quic".to_string()),
                    quic_settings: Some(QuicSettings {
                        sni_hostname: "bing.com".to_string(),
                        verify: false,
                        alpn_protocols: "h3".to_string(),
                    }),
                }),
            }]),
            geo_routing: None,
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("YAML output:\n{}", yaml);
        assert!(yaml.contains("address: 127.0.0.1:1080"));
        assert!(yaml.contains("type: socks"));
        assert!(yaml.contains("type: hysteria2"));
        assert!(yaml.contains("transport: quic"));
        assert!(yaml.contains("sni_hostname:"));
    }

    #[test]
    fn test_vless_protocol_serialization() {
        let config = TcpServerConfig {
            address: "127.0.0.1:1080".to_string(),
            protocol: ServerProtocol::Socks {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(ClientChainHop {
                    address: "example.com:443".to_string(),
                    protocol: ClientProtocol::Vless {
                        user_id: "b85798ef-e9dc-46a4-9a87-8da4499d36d0".to_string(),
                        udp_enabled: Some(true),
                    },
                    transport: Some("tcp".to_string()),
                    quic_settings: None,
                }),
            }]),
            geo_routing: None,
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("VLESS YAML output:\n{}", yaml);
        assert!(yaml.contains("type: vless"));
        assert!(yaml.contains("user_id: b85798ef-e9dc-46a4-9a87-8da4499d36d0"));
    }

    #[test]
    fn test_vmess_protocol_serialization() {
        let config = TcpServerConfig {
            address: "127.0.0.1:1080".to_string(),
            protocol: ServerProtocol::Socks {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(ClientChainHop {
                    address: "example.com:443".to_string(),
                    protocol: ClientProtocol::Vmess {
                        user_id: "b0e80a62-8a51-47f0-91f1-f0f7faf8d9d4".to_string(),
                        udp_enabled: Some(true),
                        cipher: Some("any".to_string()),
                    },
                    transport: Some("tcp".to_string()),
                    quic_settings: None,
                }),
            }]),
            geo_routing: None,
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("VMess YAML output:\n{}", yaml);
        assert!(yaml.contains("type: vmess"));
        assert!(yaml.contains("user_id: b0e80a62-8a51-47f0-91f1-f0f7faf8d9d4"));
        assert!(yaml.contains("cipher: any"));
    }

    #[test]
    fn test_trojan_protocol_serialization() {
        let config = TcpServerConfig {
            address: "127.0.0.1:1080".to_string(),
            protocol: ServerProtocol::Socks {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(ClientChainHop {
                    address: "example.com:443".to_string(),
                    protocol: ClientProtocol::Trojan {
                        password: "trojan-password".to_string(),
                        udp_enabled: Some(true),
                    },
                    transport: Some("tcp".to_string()),
                    quic_settings: None,
                }),
            }]),
            geo_routing: None,
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("Trojan YAML output:\n{}", yaml);
        assert!(yaml.contains("type: trojan"));
        assert!(yaml.contains("password: trojan-password"));
    }

    #[test]
    fn test_vless_with_tls_serialization() {
        let config = TcpServerConfig {
            address: "127.0.0.1:1080".to_string(),
            protocol: ServerProtocol::Socks {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(ClientChainHop {
                    address: "example.com:443".to_string(),
                    protocol: ClientProtocol::Tls {
                        verify: Some(false),
                        sni_hostname: Some("example.com".to_string()),
                        protocol: Some(Box::new(ClientProtocol::Vless {
                            user_id: "b85798ef-e9dc-46a4-9a87-8da4499d36d0".to_string(),
                            udp_enabled: Some(true),
                        })),
                    },
                    transport: Some("tcp".to_string()),
                    quic_settings: None,
                }),
            }]),
            geo_routing: None,
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("VLESS+TLS YAML output:\n{}", yaml);
        assert!(yaml.contains("type: tls"));
        assert!(yaml.contains("sni_hostname: example.com"));
        assert!(yaml.contains("type: vless"));
    }

    #[test]
    fn test_vless_with_ws_serialization() {
        let config = TcpServerConfig {
            address: "127.0.0.1:1080".to_string(),
            protocol: ServerProtocol::Socks {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(ClientChainHop {
                    address: "example.com:443".to_string(),
                    protocol: ClientProtocol::Ws {
                        matching_path: Some("/vless".to_string()),
                        matching_headers: None,
                        protocol: Some(Box::new(ClientProtocol::Vless {
                            user_id: "b85798ef-e9dc-46a4-9a87-8da4499d36d0".to_string(),
                            udp_enabled: Some(true),
                        })),
                    },
                    transport: Some("tcp".to_string()),
                    quic_settings: None,
                }),
            }]),
            geo_routing: None,
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("VLESS+WS YAML output:\n{}", yaml);
        assert!(yaml.contains("type: ws"));
        assert!(yaml.contains("matching_path: /vless"));
        assert!(yaml.contains("type: vless"));
    }

    #[test]
    fn test_vless_with_reality_serialization() {
        let config = TcpServerConfig {
            address: "127.0.0.1:1080".to_string(),
            protocol: ServerProtocol::Socks {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(ClientChainHop {
                    address: "example.com:443".to_string(),
                    protocol: ClientProtocol::Reality {
                        public_key: "test_public_key".to_string(),
                        short_id: "0123456789abcdef".to_string(),
                        sni_hostname: "www.google.com".to_string(),
                        vision: None,
                        protocol: Some(Box::new(ClientProtocol::Vless {
                            user_id: "b85798ef-e9dc-46a4-9a87-8da4499d36d0".to_string(),
                            udp_enabled: Some(true),
                        })),
                    },
                    transport: Some("tcp".to_string()),
                    quic_settings: None,
                }),
            }]),
            geo_routing: None,
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("VLESS+Reality YAML output:\n{}", yaml);
        assert!(yaml.contains("type: reality"));
        assert!(yaml.contains("public_key: test_public_key"));
        assert!(yaml.contains("short_id: 0123456789abcdef"));
        assert!(yaml.contains("sni_hostname: www.google.com"));
        assert!(yaml.contains("type: vless"));
    }

    #[test]
    fn test_geo_routing_serialization() {
        let config = TcpServerConfig {
            address: "127.0.0.1:1080".to_string(),
            protocol: ServerProtocol::Socks {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(ClientChainHop {
                    address: "example.com:443".to_string(),
                    protocol: ClientProtocol::Vless {
                        user_id: "b85798ef-e9dc-46a4-9a87-8da4499d36d0".to_string(),
                        udp_enabled: Some(true),
                    },
                    transport: Some("tcp".to_string()),
                    quic_settings: None,
                }),
            }]),
            geo_routing: Some(GeoRoutingConfig::with_paths(
                "/path/to/geoip.dat".to_string(),
                "/path/to/geosite.dat".to_string(),
            )),
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("Geo routing YAML output:\n{}", yaml);
        assert!(yaml.contains("geo_routing:"));
        assert!(yaml.contains("geoip_file: /path/to/geoip.dat"));
        assert!(yaml.contains("geosite_file: /path/to/geosite.dat"));
    }

    #[test]
    fn test_geo_routing_disabled_serialization() {
        let config = TcpServerConfig {
            address: "127.0.0.1:1080".to_string(),
            protocol: ServerProtocol::Socks {
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: "0.0.0.0/0".to_string(),
                action: "allow".to_string(),
                client_chain: Some(ClientChainHop {
                    address: "example.com:443".to_string(),
                    protocol: ClientProtocol::Vless {
                        user_id: "b85798ef-e9dc-46a4-9a87-8da4499d36d0".to_string(),
                        udp_enabled: Some(true),
                    },
                    transport: Some("tcp".to_string()),
                    quic_settings: None,
                }),
            }]),
            geo_routing: None,  // None = geo routing disabled
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("Geo routing disabled YAML output:\n{}", yaml);
        // When None, geo_routing should not appear in output (skip_serializing_if)
        assert!(!yaml.contains("geo_routing:"));
    }
}
