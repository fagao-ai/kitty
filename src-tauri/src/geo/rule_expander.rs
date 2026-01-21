//! Rule expander for GeoIP and GeoSite rules.
//!
//! Expands GeoIP/GeoSite rules into concrete CIDR and domain rules that can be used
//! by the shoes library.

use super::geo_data_manager::{DomainEntry, DomainType, GeoDataManager};
use anyhow::{anyhow, Result};
use entity::rules;
use std::collections::HashMap;

/// Concrete rule that can be used in shoes configuration
#[derive(Debug, Clone)]
pub enum ConcreteRule {
    /// CIDR rule (e.g., "192.168.0.0/16")
    Cidr(String),
    /// Full domain match (e.g., "example.com")
    FullDomain(String),
    /// Domain suffix match (e.g., "example.com" matches "*.example.com")
    DomainSuffix(String),
    /// Domain regex match
    DomainRegex(String),
}

/// Expanded rule with source ID and action
#[derive(Debug, Clone)]
pub struct ExpandedRule {
    /// Original database rule ID
    pub source_id: i32,
    /// Rule action (Proxy, Direct, Reject)
    pub action: rules::RuleAction,
    /// Expanded concrete rules
    pub concrete_rules: Vec<ConcreteRule>,
}

/// Rule expander for GeoIP/GeoSite rules
pub struct RuleExpander;

impl RuleExpander {
    /// Expand all rules, converting GeoIP/GeoSite references to concrete rules
    ///
    /// # Arguments
    /// * `db_rules` - Rules from the database
    /// * `geo_manager` - GeoDataManager with loaded GeoIP/GeoSite data
    ///
    /// # Returns
    /// Vector of expanded rules
    pub fn expand_rules(
        db_rules: Vec<rules::Model>,
        geo_manager: &GeoDataManager,
    ) -> Result<Vec<ExpandedRule>> {
        let mut expanded = Vec::new();

        for db_rule in db_rules {
            let concrete_rules = match db_rule.rule_type {
                rules::RuleType::GeoIP => Self::expand_geoip(&db_rule, geo_manager)?,
                rules::RuleType::GeoSite => Self::expand_geosite(&db_rule, geo_manager)?,
                rules::RuleType::Cidr => {
                    // Direct CIDR rule
                    vec![ConcreteRule::Cidr(db_rule.rule)]
                }
                rules::RuleType::FullDomain => {
                    // Direct full domain rule
                    vec![ConcreteRule::FullDomain(db_rule.rule)]
                }
                rules::RuleType::DomainSuffix => {
                    // Direct domain suffix rule
                    vec![ConcreteRule::DomainSuffix(db_rule.rule)]
                }
                rules::RuleType::DomainPreffix => {
                    // Domain prefix - convert to regex or keep as is
                    vec![ConcreteRule::FullDomain(db_rule.rule)]
                }
                rules::RuleType::DomainRoot => {
                    // Domain root - treat as suffix
                    vec![ConcreteRule::DomainSuffix(db_rule.rule)]
                }
            };

            expanded.push(ExpandedRule {
                source_id: db_rule.id,
                action: db_rule.rule_action,
                concrete_rules,
            });
        }

        Ok(expanded)
    }

    /// Expand a GeoIP rule into CIDR rules
    ///
    /// # Format
    /// Input: "geoip:cn" or just "cn"
    /// Output: ["1.0.1.0/24", "1.0.2.0/24", ...]
    fn expand_geoip(rule: &rules::Model, geo_manager: &GeoDataManager) -> Result<Vec<ConcreteRule>> {
        // Parse country code from rule value
        // Supports formats: "geoip:cn", "CN", "cn"
        let country_code = Self::parse_country_code(&rule.rule, "geoip")?;

        // Get CIDR list from GeoDataManager
        let cidrs = geo_manager.get_geoip_cidrs(&country_code)
            .ok_or_else(|| anyhow!("Country code '{}' not found in GeoIP data", country_code))?;

        // Convert to ConcreteRule::Cidr
        let rules = cidrs.iter()
            .map(|cidr| ConcreteRule::Cidr(cidr.clone()))
            .collect();

        Ok(rules)
    }

    /// Expand a GeoSite rule into domain rules
    ///
    /// # Format
    /// Input: "geosite:google" or just "google"
    /// Output: ["google.com", "*.google.com", "google.co.uk", ...]
    fn expand_geosite(rule: &rules::Model, geo_manager: &GeoDataManager) -> Result<Vec<ConcreteRule>> {
        // Parse country code or category from rule value
        // Supports formats: "geosite:cn", "geosite:google", "cn", "google"
        let code = Self::parse_country_code(&rule.rule, "geosite")?;

        // Get domain list from GeoDataManager
        let domains = geo_manager.get_geosite_domains(&code)
            .ok_or_else(|| anyhow!("GeoSite code '{}' not found in GeoSite data", code))?;

        // Convert DomainEntry to ConcreteRule based on type
        let mut rules = Vec::new();
        for domain in domains {
            let rule = match domain.entry_type {
                DomainType::Full => ConcreteRule::FullDomain(domain.value.clone()),
                DomainType::Plain => ConcreteRule::FullDomain(domain.value.clone()),
                DomainType::DomainRoot => {
                    // Root domain - convert to suffix match
                    // shoes doesn't directly support suffix, so we store as DomainSuffix
                    // and let the config converter handle it
                    ConcreteRule::DomainSuffix(domain.value.clone())
                }
                DomainType::Regex => ConcreteRule::DomainRegex(domain.value.clone()),
            };
            rules.push(rule);
        }

        Ok(rules)
    }

    /// Parse country code from rule value
    ///
    /// Supports formats:
    /// - "geoip:cn" -> "cn"
    /// - "geosite:cn" -> "cn"
    /// - "cn" -> "cn"
    /// - "CN" -> "cn"
    fn parse_country_code(rule_value: &str, prefix: &str) -> Result<String> {
        let value = rule_value.trim().to_lowercase();

        // Check if it has the prefix (e.g., "geoip:cn")
        if value.starts_with(&format!("{}:", prefix)) {
            let code = value[prefix.len() + 1..].to_string();
            if code.is_empty() {
                return Err(anyhow!("Empty country code after prefix '{}:'", prefix));
            }
            return Ok(code);
        }

        // Otherwise, treat the whole value as country code
        if value.is_empty() {
            return Err(anyhow!("Empty country code"));
        }

        Ok(value)
    }

    /// Convert expanded rules to shoes-compatible format
    ///
    /// This converts ConcreteRule types into the format expected by shoes' NetLocationMask.
    ///
    /// # Returns
    /// Vector of (masks_string, action_string) tuples
    pub fn to_shoes_rules(
        expanded_rules: Vec<ExpandedRule>,
    ) -> Result<Vec<(String, String)>> {
        let mut shoes_rules = Vec::new();

        for expanded in expanded_rules {
            let action = match expanded.action {
                rules::RuleAction::Proxy => "allow",
                rules::RuleAction::Direct => "allow", // Direct is also allow, but without proxy chain
                rules::RuleAction::Reject => "block",
            };

            for concrete in expanded.concrete_rules {
                let masks = match concrete {
                    ConcreteRule::Cidr(cidr) => cidr,
                    ConcreteRule::FullDomain(domain) => domain,
                    ConcreteRule::DomainSuffix(suffix) => {
                        // shoes doesn't support wildcards, so we prepend "*."
                        // Note: This may not work properly in shoes without additional support
                        format!("*.{}", suffix)
                    }
                    ConcreteRule::DomainRegex(regex) => {
                        // shoes may not support regex, include as-is with a warning
                        log::warn!("Regex rules may not be supported by shoes: {}", regex);
                        regex
                    }
                };

                shoes_rules.push((masks, action.to_string()));
            }
        }

        Ok(shoes_rules)
    }

    /// Get statistics about expanded rules
    pub fn get_expansion_stats(
        expanded_rules: &[ExpandedRule],
    ) -> HashMap<i32, usize> {
        let mut stats = HashMap::new();
        for rule in expanded_rules {
            stats.insert(rule.source_id, rule.concrete_rules.len());
        }
        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_country_code_with_prefix() {
        assert_eq!(
            RuleExpander::parse_country_code("geoip:cn", "geoip").unwrap(),
            "cn"
        );
        assert_eq!(
            RuleExpander::parse_country_code("geosite:google", "geosite").unwrap(),
            "google"
        );
    }

    #[test]
    fn test_parse_country_code_without_prefix() {
        assert_eq!(
            RuleExpander::parse_country_code("cn", "geoip").unwrap(),
            "cn"
        );
        assert_eq!(
            RuleExpander::parse_country_code("CN", "geoip").unwrap(),
            "cn"
        );
        assert_eq!(
            RuleExpander::parse_country_code("google", "geosite").unwrap(),
            "google"
        );
    }

    #[test]
    fn test_parse_country_code_empty() {
        assert!(
            RuleExpander::parse_country_code("", "geoip").is_err()
        );
        assert!(
            RuleExpander::parse_country_code("geoip:", "geoip").is_err()
        );
    }
}
