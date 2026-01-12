//! Config converter module for converting hysteria configs to shoes YAML format.
//!
//! This module provides conversion functions to transform hysteria
//! database entities into shoes-compatible YAML configurations.

use anyhow::{anyhow, Result};
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
}

/// Bandwidth configuration for Hysteria2.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bandwidth {
    pub up: String,
    pub down: String,
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
        };

        // Serialize to YAML
        let configs = vec![ShoesConfig::TunServer(tun_config)];
        serde_yaml::to_string(&configs).map_err(|e| anyhow!("Failed to serialize YAML: {}", e))
    }

    /// Convert an xray Model to shoes YAML configs for SOCKS5/HTTP proxy mode.
    /// Note: This is a simplified version that may need additional work for full xray support.
    pub fn xray_to_socks_http_yaml(
        _model: &xray::Model,
        _http_port: u16,
        _socks_port: u16,
    ) -> Result<String> {
        Err(anyhow!("xray conversion not yet implemented"))
    }

    /// Convert an xray Model to shoes YAML config for TUN/VPN mode.
    pub fn xray_to_tun_yaml(
        _model: &xray::Model,
        _tun_address: String,
        _tun_netmask: String,
    ) -> Result<String> {
        Err(anyhow!("xray conversion not yet implemented"))
    }

    /// Convert multiple xray records to a single shoes YAML config.
    pub fn xray_multi_to_yaml(
        _models: &[xray::Model],
        _http_port: u16,
        _socks_port: u16,
    ) -> Result<String> {
        Err(anyhow!("xray conversion not yet implemented"))
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
        };

        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("YAML output:\n{}", yaml);
        assert!(yaml.contains("address: 127.0.0.1:1080"));
        assert!(yaml.contains("type: socks"));
        assert!(yaml.contains("type: hysteria2"));
        assert!(yaml.contains("transport: quic"));
        assert!(yaml.contains("sni_hostname:"));
    }
}
