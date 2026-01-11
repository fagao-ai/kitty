//! Config converter module for converting xray/hysteria JSON configs to shoes YAML format.
//!
//! This module provides conversion functions to transform existing xray and hysteria
//! database entities into shoes-compatible YAML configurations.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use entity::{hysteria, xray};

/// Shoes YAML configuration structure.
/// This will be serialized to YAML and passed to the shoes library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShoesYamlConfig {
    #[serde(flatten)]
    pub configs: Vec<ShoesConfig>,
}

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
}

/// Server protocol type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ServerProtocol {
    /// HTTP proxy server
    Http {
        #[serde(skip_serializing_if = "Option::is_none")]
        username: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        udp_enabled: Option<bool>,
    },
    /// SOCKS5 proxy server
    Socks {
        #[serde(skip_serializing_if = "Option::is_none")]
        username: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        udp_enabled: Option<bool>,
    },
    /// Mixed HTTP/SOCKS5 server (auto-detects protocol)
    Mixed {
        #[serde(skip_serializing_if = "Option::is_none")]
        username: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        password: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        udp_enabled: Option<bool>,
    },
}

/// Routing rule for proxy server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// IP/hostname masks to match
    pub masks: Masks,
    /// Action to take (allow/block)
    pub action: String,
    /// Client chain for upstream proxy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_chain: Option<Vec<ClientChainHop>>,
}

/// IP/hostname masks for rule matching.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Masks {
    Single(String),
    Multiple(Vec<String>),
}

/// A single hop in the client proxy chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientChainHop {
    /// Upstream server address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Protocol configuration
    pub protocol: ClientProtocol,
    /// Transport configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
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
        alter_id: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        security: Option<String>,
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
        bandwidth: Option<Bandwidth>,
    },
    /// TLS protocol (wrapper)
    #[serde(rename = "tls")]
    Tls {
        #[serde(skip_serializing_if = "Option::is_none")]
        sni_hostname: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        verify: Option<bool>,
        #[serde(rename = "certificate")]
        #[serde(skip_serializing_if = "Option::is_none")]
        cert: Option<String>,
        /// Nested protocol
        #[serde(skip_serializing_if = "Option::is_none")]
        protocol: Option<Box<ClientProtocol>>,
    },
    /// Reality protocol
    #[serde(rename = "reality")]
    Reality {
        #[serde(skip_serializing_if = "Option::is_none")]
        fingerprint: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        public_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        short_id: Option<String>,
        /// Nested protocol
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

/// Transport configuration (WebSocket, TCP, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Transport {
    /// WebSocket transport
    Ws {
        #[serde(skip_serializing_if = "Option::is_none")]
        path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<HashMap<String, String>>,
    },
    /// TCP transport
    Tcp,
}

/// Main converter for transforming xray/hysteria configs to shoes YAML.
pub struct ShoesConfigConverter;

impl ShoesConfigConverter {
    /// Convert an xray Model to shoes YAML configs for SOCKS5/HTTP proxy mode.
    pub fn xray_to_socks_http_yaml(
        model: &xray::Model,
        http_port: u16,
        socks_port: u16,
    ) -> Result<String> {
        // Extract the upstream server address
        let server_addr = format!("{}:{}", model.address, model.port);

        // Build the client chain (upstream proxy configuration)
        let client_chain = Self::build_xray_client_chain(model)?;

        // Create HTTP proxy config
        let http_config = TcpServerConfig {
            address: format!("127.0.0.1:{http_port}"),
            protocol: ServerProtocol::Http {
                username: None,
                password: None,
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: Masks::Single("0.0.0.0/0".to_string()),
                action: "allow".to_string(),
                client_chain: Some(vec![ClientChainHop {
                    address: Some(server_addr.clone()),
                    protocol: client_chain.clone(),
                    transport: Self::build_xray_transport(model)?,
                }]),
            }]),
        };

        // Create SOCKS5 proxy config
        let socks_config = TcpServerConfig {
            address: format!("127.0.0.1:{socks_port}"),
            protocol: ServerProtocol::Socks {
                username: None,
                password: None,
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: Masks::Single("0.0.0.0/0".to_string()),
                action: "allow".to_string(),
                client_chain: Some(vec![ClientChainHop {
                    address: Some(server_addr),
                    protocol: client_chain,
                    transport: Self::build_xray_transport(model)?,
                }]),
            }]),
        };

        // Serialize to YAML
        let configs = vec![ShoesConfig::Server(http_config), ShoesConfig::Server(socks_config)];
        serde_yaml::to_string(&configs).map_err(|e| anyhow!("Failed to serialize YAML: {}", e))
    }

    /// Convert an xray Model to shoes YAML config for TUN/VPN mode.
    pub fn xray_to_tun_yaml(
        model: &xray::Model,
        tun_address: String,
        tun_netmask: String,
    ) -> Result<String> {
        // Extract the upstream server address
        let server_addr = format!("{}:{}", model.address, model.port);

        // Build the client chain
        let client_chain = Self::build_xray_client_chain(model)?;
        let transport = Self::build_xray_transport(model)?;

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
                masks: Masks::Single("0.0.0.0/0".to_string()),
                action: "allow".to_string(),
                client_chain: Some(vec![ClientChainHop {
                    address: Some(server_addr),
                    protocol: client_chain,
                    transport,
                }]),
            }],
        };

        // Serialize to YAML
        let configs = vec![ShoesConfig::TunServer(tun_config)];
        serde_yaml::to_string(&configs).map_err(|e| anyhow!("Failed to serialize YAML: {}", e))
    }

    /// Convert a hysteria Model to shoes YAML configs for SOCKS5/HTTP proxy mode.
    pub fn hysteria_to_socks_http_yaml(
        model: &hysteria::Model,
        http_port: u16,
        socks_port: u16,
    ) -> Result<String> {
        // Hysteria uses QUIC transport (built into the protocol)

        // Build the Hysteria2 client protocol
        let client_protocol = ClientProtocol::Hysteria2 {
            password: model.auth.clone(),
            udp_enabled: Some(true),
            bandwidth: Some(Bandwidth {
                up: model.bandwidth.up.clone(),
                down: model.bandwidth.down.clone(),
            }),
        };

        // Create HTTP proxy config
        let http_config = TcpServerConfig {
            address: format!("127.0.0.1:{http_port}"),
            protocol: ServerProtocol::Http {
                username: None,
                password: None,
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: Masks::Single("0.0.0.0/0".to_string()),
                action: "allow".to_string(),
                client_chain: Some(vec![ClientChainHop {
                    address: Some(model.server.clone()),
                    protocol: client_protocol.clone(),
                    transport: None, // Hysteria2 uses QUIC, no separate transport needed
                }]),
            }]),
        };

        // Create SOCKS5 proxy config
        let socks_config = TcpServerConfig {
            address: format!("127.0.0.1:{socks_port}"),
            protocol: ServerProtocol::Socks {
                username: None,
                password: None,
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: Masks::Single("0.0.0.0/0".to_string()),
                action: "allow".to_string(),
                client_chain: Some(vec![ClientChainHop {
                    address: Some(model.server.clone()),
                    protocol: client_protocol,
                    transport: None,
                }]),
            }]),
        };

        // Serialize to YAML
        let configs = vec![ShoesConfig::Server(http_config), ShoesConfig::Server(socks_config)];
        serde_yaml::to_string(&configs).map_err(|e| anyhow!("Failed to serialize YAML: {}", e))
    }

    /// Convert a hysteria Model to shoes YAML config for TUN/VPN mode.
    pub fn hysteria_to_tun_yaml(
        model: &hysteria::Model,
        tun_address: String,
        tun_netmask: String,
    ) -> Result<String> {
        // Build the Hysteria2 client protocol
        let client_protocol = ClientProtocol::Hysteria2 {
            password: model.auth.clone(),
            udp_enabled: Some(true),
            bandwidth: Some(Bandwidth {
                up: model.bandwidth.up.clone(),
                down: model.bandwidth.down.clone(),
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
                masks: Masks::Single("0.0.0.0/0".to_string()),
                action: "allow".to_string(),
                client_chain: Some(vec![ClientChainHop {
                    address: Some(model.server.clone()),
                    protocol: client_protocol,
                    transport: None,
                }]),
            }],
        };

        // Serialize to YAML
        let configs = vec![ShoesConfig::TunServer(tun_config)];
        serde_yaml::to_string(&configs).map_err(|e| anyhow!("Failed to serialize YAML: {}", e))
    }

    /// Build the client protocol from an xray Model.
    fn build_xray_client_chain(model: &xray::Model) -> Result<ClientProtocol> {
        let base_protocol = match model.protocol {
            xray::Protocol::Vless => ClientProtocol::Vless {
                user_id: model.uuid.clone(),
                udp_enabled: Some(true),
            },
            xray::Protocol::Vmess => ClientProtocol::Vmess {
                user_id: model.uuid.clone(),
                alter_id: Some(0),
                security: Some("auto".to_string()),
            },
            xray::Protocol::Trojan => ClientProtocol::Trojan {
                password: model.uuid.clone(),
                udp_enabled: Some(true),
            },
        };

        // Wrap with TLS or Reality if configured
        Ok(base_protocol)
    }

    /// Build the transport configuration from an xray Model.
    fn build_xray_transport(model: &xray::Model) -> Result<Option<Transport>> {
        // This is a simplified version - in reality you'd need to parse the stream_settings
        // more carefully to extract the transport type and any additional config

        // For now, return TCP as default
        // TODO: Parse stream_settings to determine transport type (ws, tcp, etc.)
        Ok(Some(Transport::Tcp))
    }

    /// Convert multiple xray records to a single shoes YAML config.
    /// This can be used for load balancing or failover scenarios.
    pub fn xray_multi_to_yaml(
        models: &[xray::Model],
        http_port: u16,
        socks_port: u16,
    ) -> Result<String> {
        if models.is_empty() {
            return Err(anyhow!("No models provided"));
        }

        // For simplicity, use the first model for now
        // TODO: Implement proper multi-server configuration
        Self::xray_to_socks_http_yaml(&models[0], http_port, socks_port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_simple_config() {
        let config = TcpServerConfig {
            address: "127.0.0.1:8080".to_string(),
            protocol: ServerProtocol::Http {
                username: None,
                password: None,
                udp_enabled: Some(true),
            },
            rules: Some(vec![Rule {
                masks: Masks::Single("0.0.0.0/0".to_string()),
                action: "allow".to_string(),
                client_chain: Some(vec![ClientChainHop {
                    address: Some("example.com:443".to_string()),
                    protocol: ClientProtocol::Direct,
                    transport: None,
                }]),
            }]),
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("YAML output:\n{}", yaml);
        assert!(yaml.contains("address: 127.0.0.1:8080"));
        assert!(yaml.contains("type: http"));
    }
}
