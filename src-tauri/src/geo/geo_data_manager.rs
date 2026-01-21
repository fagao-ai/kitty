//! GeoIP and GeoSite data manager.
//!
//! Handles downloading, parsing, and caching GeoIP/GeoSite data from .dat files.

use super::v2ray_config;
use anyhow::{anyhow, Result};
use prost::Message;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::env;

/// Domain entry from GeoSite data
#[derive(Debug, Clone)]
pub struct DomainEntry {
    /// Domain type: Plain, Regex, DomainRoot, or Full
    pub entry_type: DomainType,
    /// Domain value
    pub value: String,
}

/// Domain type matching V2Ray's Domain.Type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainType {
    /// Plain domain (exact match)
    Plain,
    /// Regex domain pattern
    Regex,
    /// Root domain (matches subdomains)
    DomainRoot,
    /// Full domain (exact match)
    Full,
}

impl From<v2ray_config::domain::Type> for DomainType {
    fn from(t: v2ray_config::domain::Type) -> Self {
        match t {
            v2ray_config::domain::Type::Plain => DomainType::Plain,
            v2ray_config::domain::Type::Regex => DomainType::Regex,
            v2ray_config::domain::Type::Domain => DomainType::DomainRoot,
            v2ray_config::domain::Type::Full => DomainType::Full,
        }
    }
}

/// GeoIP and GeoSite data manager
///
/// Manages downloading, parsing, and caching of GeoIP/GeoSite .dat files.
pub struct GeoDataManager {
    /// Path to GeoIP .dat file
    geoip_path: PathBuf,
    /// Path to GeoSite .dat file
    geosite_path: PathBuf,
    /// Cached GeoIP data: country_code -> Vec<CIDR string>
    geoip_cache: HashMap<String, Vec<String>>,
    /// Cached GeoSite data: country_code -> Vec<DomainEntry>
    geosite_cache: HashMap<String, Vec<DomainEntry>>,
}

impl GeoDataManager {
    /// Create a new GeoDataManager with default paths
    ///
    /// Default paths are in the project's static directory:
    /// - kitty_geoip.dat
    /// - kitty_geosite.dat
    pub fn new() -> Result<Self> {
        let project_dir = env::var("CARGO_MANIFEST_DIR")
            .or_else(|_| env::var("OUT_DIR"))
            .map_err(|_| anyhow!("Failed to find project directory"))?;

        let project_path = Path::new(&project_dir);

        // Try static directory first, then fallback to parent directory
        let static_dir = project_path.join("static");
        let base_dir = if static_dir.exists() {
            static_dir
        } else {
            project_path.to_path_buf()
        };

        Ok(Self {
            geoip_path: base_dir.join("kitty_geoip.dat"),
            geosite_path: base_dir.join("kitty_geosite.dat"),
            geoip_cache: HashMap::new(),
            geosite_cache: HashMap::new(),
        })
    }

    /// Create a new GeoDataManager with custom paths
    pub fn with_paths(geoip_path: PathBuf, geosite_path: PathBuf) -> Self {
        Self {
            geoip_path,
            geosite_path,
            geoip_cache: HashMap::new(),
            geosite_cache: HashMap::new(),
        }
    }

    /// Load and parse GeoIP and GeoSite .dat files
    ///
    /// This will parse both files and cache the data in memory for fast lookup.
    pub async fn load(&mut self) -> Result<()> {
        // Load GeoIP data
        if self.geoip_path.exists() {
            self.load_geoip().await?;
        } else {
            log::warn!("GeoIP file not found: {:?}", self.geoip_path);
        }

        // Load GeoSite data
        if self.geosite_path.exists() {
            self.load_geosite().await?;
        } else {
            log::warn!("GeoSite file not found: {:?}", self.geosite_path);
        }

        Ok(())
    }

    /// Load and parse GeoIP .dat file
    async fn load_geoip(&mut self) -> Result<()> {
        let mut file = File::open(&self.geoip_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        // Parse GeoIPList from protobuf
        let geoip_list = match v2ray_config::GeoIpList::decode(&content[..]) {
            Ok(list) => list,
            Err(e) => {
                return Err(anyhow!("Failed to decode GeoIP file: {}", e));
            }
        };

        // Build cache: country_code -> Vec<CIDR>
        for geo_ip in geoip_list.entry {
            let country_code = geo_ip.country_code.to_lowercase();
            let mut cidrs = Vec::new();

            for cidr in &geo_ip.cidr {
                let cidr_string = self.cidr_to_string(cidr)?;
                cidrs.push(cidr_string);
            }

            self.geoip_cache.insert(country_code, cidrs);
        }

        log::info!("Loaded GeoIP data for {} countries", self.geoip_cache.len());
        Ok(())
    }

    /// Load and parse GeoSite .dat file
    async fn load_geosite(&mut self) -> Result<()> {
        let mut file = File::open(&self.geosite_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        // The .dat file format is: [varint length] + [GeoSiteList message]
        // Try to decode directly as GeoSiteList first
        match v2ray_config::GeoSiteList::decode(&content[..]) {
            Ok(geosite_list) => {
                // Build cache: country_code -> Vec<DomainEntry>
                for geo_site in geosite_list.entry {
                    let country_code = geo_site.country_code.to_lowercase();
                    let mut domains = Vec::new();

                    for domain in &geo_site.domain {
                        let entry_type = match domain.r#type {
                            0 => DomainType::Plain,
                            1 => DomainType::Regex,
                            2 => DomainType::DomainRoot,
                            3 => DomainType::Full,
                            _ => {
                                log::warn!("Unknown domain type: {}", domain.r#type);
                                continue;
                            }
                        };

                        domains.push(DomainEntry {
                            entry_type,
                            value: domain.value.clone(),
                        });
                    }

                    self.geosite_cache.insert(country_code, domains);
                }

                log::info!("Loaded GeoSite data for {} countries", self.geosite_cache.len());
            }
            Err(e) => {
                log::warn!("Failed to decode GeoSiteList directly: {}", e);
                // Try with length prefix
                let mut cursor = io::Cursor::new(&content);

                // Read varint length prefix
                let mut length = 0;
                let mut shift = 0;
                loop {
                    let mut byte = [0u8; 1];
                    if cursor.read_exact(&mut byte).is_err() {
                        break;
                    }
                    let b = byte[0];
                    length |= ((b & 0x7F) as u64) << shift;
                    if b & 0x80 == 0 {
                        break;
                    }
                    shift += 7;
                }

                // Now decode the GeoSiteList message from the remaining content
                let remaining_content = &content[cursor.position() as usize..];
                match v2ray_config::GeoSiteList::decode(remaining_content) {
                    Ok(geosite_list) => {
                        // Build cache: country_code -> Vec<DomainEntry>
                        for geo_site in geosite_list.entry {
                            let country_code = geo_site.country_code.to_lowercase();
                            let mut domains = Vec::new();

                            for domain in &geo_site.domain {
                                let entry_type = match domain.r#type {
                                    0 => DomainType::Plain,
                                    1 => DomainType::Regex,
                                    2 => DomainType::DomainRoot,
                                    3 => DomainType::Full,
                                    _ => {
                                        log::warn!("Unknown domain type: {}", domain.r#type);
                                        continue;
                                    }
                                };

                                domains.push(DomainEntry {
                                    entry_type,
                                    value: domain.value.clone(),
                                });
                            }

                            self.geosite_cache.insert(country_code, domains);
                        }

                        log::info!("Loaded GeoSite data for {} countries (with length prefix)", self.geosite_cache.len());
                    }
                    Err(_) => {
                        // Try the old format as fallback (multiple GeoSite messages)
                        let mut cursor = io::Cursor::new(&content);
                        let mut entries = Vec::new();

                        while (cursor.position() as usize) < content.len() {
                            match v2ray_config::GeoSite::decode(&mut cursor) {
                                Ok(geo_site) => entries.push(geo_site),
                                Err(_) => break,
                            }
                        }

                        for geo_site in entries {
                            let country_code = geo_site.country_code.to_lowercase();
                            let mut domains = Vec::new();

                            for domain in &geo_site.domain {
                                let entry_type = match domain.r#type {
                                    0 => DomainType::Plain,
                                    1 => DomainType::Regex,
                                    2 => DomainType::DomainRoot,
                                    3 => DomainType::Full,
                                    _ => continue,
                                };

                                domains.push(DomainEntry {
                                    entry_type,
                                    value: domain.value.clone(),
                                });
                            }

                            self.geosite_cache.insert(country_code, domains);
                        }

                        log::info!("Loaded GeoSite data for {} countries (fallback format)", self.geosite_cache.len());
                    }
                }
            }
        }

        Ok(())
    }

    /// Get CIDR list for a country code
    ///
    /// # Arguments
    /// * `country_code` - ISO 3166-1 alpha-2 country code (e.g., "cn", "us")
    ///
    /// # Returns
    /// Slice of CIDR strings if found, None otherwise
    pub fn get_geoip_cidrs(&self, country_code: &str) -> Option<&[String]> {
        self.geoip_cache.get(&country_code.to_lowercase()).map(|v| v.as_slice())
    }

    /// Get domain list for a country code
    ///
    /// # Arguments
    /// * `country_code` - ISO 3166-1 alpha-2 country code (e.g., "cn", "us")
    ///
    /// # Returns
    /// Slice of DomainEntry if found, None otherwise
    pub fn get_geosite_domains(&self, country_code: &str) -> Option<&[DomainEntry]> {
        self.geosite_cache.get(&country_code.to_lowercase()).map(|v| v.as_slice())
    }

    /// Get all available country codes in GeoIP cache
    pub fn get_geoip_countries(&self) -> Vec<String> {
        self.geoip_cache.keys().cloned().collect()
    }

    /// Get all available country codes in GeoSite cache
    pub fn get_geosite_countries(&self) -> Vec<String> {
        self.geosite_cache.keys().cloned().collect()
    }

    /// Check if GeoIP data is loaded for a country
    pub fn has_geoip(&self, country_code: &str) -> bool {
        self.geoip_cache.contains_key(&country_code.to_lowercase())
    }

    /// Check if GeoSite data is loaded for a country
    pub fn has_geosite(&self, country_code: &str) -> bool {
        self.geosite_cache.contains_key(&country_code.to_lowercase())
    }

    /// Convert CIDR protobuf message to string representation
    fn cidr_to_string(&self, cidr: &v2ray_config::Cidr) -> Result<String> {
        let ip_bytes = &cidr.ip;
        let prefix = cidr.prefix;

        let ip_str = if ip_bytes.len() == 4 {
            // IPv4
            format!("{}.{}.{}.{}",
                ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3])
        } else if ip_bytes.len() == 16 {
            // IPv6
            // Simplified IPv6 representation
            let groups: Vec<String> = ip_bytes
                .chunks(2)
                .map(|chunk| format!("{:02x}{:02x}", chunk[0], chunk[1]))
                .collect();
            groups.join(":")
        } else {
            return Err(anyhow!("Invalid IP address length: {}", ip_bytes.len()));
        };

        Ok(format!("{}/{}", ip_str, prefix))
    }
}

impl Default for GeoDataManager {
    fn default() -> Self {
        Self::new().expect("Failed to create GeoDataManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cidr_to_string_ipv4() {
        let manager = GeoDataManager::new().unwrap();

        let cidr = v2ray_config::Cidr {
            ip: vec![192, 168, 0, 0],
            prefix: 16,
        };

        let result = manager.cidr_to_string(&cidr).unwrap();
        assert_eq!(result, "192.168.0.0/16");
    }

    #[test]
    fn test_cidr_to_string_ipv4_single() {
        let manager = GeoDataManager::new().unwrap();

        let cidr = v2ray_config::Cidr {
            ip: vec![1, 0, 1, 0],
            prefix: 24,
        };

        let result = manager.cidr_to_string(&cidr).unwrap();
        assert_eq!(result, "1.0.1.0/24");
    }

    #[test]
    fn test_domain_type_conversion() {
        assert_eq!(DomainType::from(v2ray_config::domain::Type::Plain), DomainType::Plain);
        assert_eq!(DomainType::from(v2ray_config::domain::Type::Regex), DomainType::Regex);
        assert_eq!(DomainType::from(v2ray_config::domain::Type::Domain), DomainType::DomainRoot);
        assert_eq!(DomainType::from(v2ray_config::domain::Type::Full), DomainType::Full);
    }
}
