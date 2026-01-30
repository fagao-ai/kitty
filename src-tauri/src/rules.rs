//! Custom rules file management.
//!
//! This module handles reading and writing custom rules from/to a JSON file.
//! The rules are stored in a format compatible with v2ray-router.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Rule action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
    /// Route through proxy
    Proxy,
    /// Direct connection
    Direct,
    /// Reject connection
    Reject,
}

/// Rule type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleType {
    /// Domain suffix match (e.g., google.com matches www.google.com)
    DomainSuffix,
    /// Domain prefix match
    #[serde(rename = "domain_prefix")]
    DomainPrefix,
    /// Full domain match
    FullDomain,
    /// CIDR IP range match
    Cidr,
    /// Domain root match (same as suffix)
    DomainRoot,
}

impl From<v2ray_router::RouteAction> for RuleAction {
    fn from(action: v2ray_router::RouteAction) -> Self {
        match action {
            v2ray_router::RouteAction::Proxy => RuleAction::Proxy,
            v2ray_router::RouteAction::Direct => RuleAction::Direct,
            v2ray_router::RouteAction::Reject => RuleAction::Reject,
        }
    }
}

impl From<RuleAction> for v2ray_router::RouteAction {
    fn from(action: RuleAction) -> Self {
        match action {
            RuleAction::Proxy => v2ray_router::RouteAction::Proxy,
            RuleAction::Direct => v2ray_router::RouteAction::Direct,
            RuleAction::Reject => v2ray_router::RouteAction::Reject,
        }
    }
}

impl From<v2ray_router::CustomRuleType> for RuleType {
    fn from(rule_type: v2ray_router::CustomRuleType) -> Self {
        match rule_type {
            v2ray_router::CustomRuleType::DomainSuffix => RuleType::DomainSuffix,
            v2ray_router::CustomRuleType::DomainPrefix => RuleType::DomainPrefix,
            v2ray_router::CustomRuleType::FullDomain => RuleType::FullDomain,
            v2ray_router::CustomRuleType::Cidr => RuleType::Cidr,
        }
    }
}

impl From<RuleType> for v2ray_router::CustomRuleType {
    fn from(rule_type: RuleType) -> Self {
        match rule_type {
            RuleType::DomainSuffix | RuleType::DomainRoot => {
                v2ray_router::CustomRuleType::DomainSuffix
            }
            RuleType::DomainPrefix => v2ray_router::CustomRuleType::DomainPrefix,
            RuleType::FullDomain => v2ray_router::CustomRuleType::FullDomain,
            RuleType::Cidr => v2ray_router::CustomRuleType::Cidr,
        }
    }
}

/// Custom rule entry
///
/// This struct's JSON format matches v2ray_router::CustomRule:
/// - "pattern" for the domain/CIDR pattern
/// - "action" for the routing action (proxy/direct/reject)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Rule pattern (domain or CIDR)
    pub pattern: String,

    /// Rule type
    #[serde(rename = "rule_type")]
    pub rule_type: RuleType,

    /// Rule action
    pub action: RuleAction,
}

impl Rule {
    /// Create a new rule
    pub fn new(pattern: String, rule_type: RuleType, action: RuleAction) -> Self {
        Self {
            pattern,
            rule_type,
            action,
        }
    }

    /// Convert from v2ray_router::CustomRule
    pub fn from_v2ray_router(v2ray_rule: v2ray_router::CustomRule) -> Self {
        Self {
            pattern: v2ray_rule.pattern,
            rule_type: RuleType::from(v2ray_rule.rule_type),
            action: RuleAction::from(v2ray_rule.action),
        }
    }

    /// Convert to v2ray_router::CustomRule
    pub fn to_v2ray_router(&self) -> v2ray_router::CustomRule {
        v2ray_router::CustomRule::new(
            self.pattern.clone(),
            self.rule_type.clone().into(),
            self.action.into(),
        )
    }
}

/// Read rules from a JSON file
pub fn read_rules_file(path: &Path) -> Result<Vec<Rule>> {
    if !path.exists() {
        // Return empty rules if file doesn't exist
        return Ok(Vec::new());
    }

    let json = std::fs::read_to_string(path)
        .map_err(|e| anyhow!("Failed to read rules file {}: {}", path.display(), e))?;

    let rules: Vec<Rule> = serde_json::from_str(&json)
        .map_err(|e| anyhow!("Failed to parse rules file {}: {}", path.display(), e))?;

    Ok(rules)
}

/// Write rules to a JSON file
pub fn write_rules_file(path: &Path, rules: &[Rule]) -> Result<()> {
    let json = serde_json::to_string_pretty(rules)
        .map_err(|e| anyhow!("Failed to serialize rules: {}", e))?;

    std::fs::write(path, json)
        .map_err(|e| anyhow!("Failed to write rules file {}: {}", path.display(), e))?;

    log::info!(
        "Wrote {} rules to {}",
        rules.len(),
        path.display()
    );

    Ok(())
}

/// Copy rules file to a destination path
pub fn copy_rules_file(src: &Path, dst: &Path) -> Result<()> {
    if !src.exists() {
        // Create empty rules file if source doesn't exist
        write_rules_file(dst, &[])?;
        return Ok(());
    }

    std::fs::copy(src, dst)
        .map_err(|e| anyhow!("Failed to copy rules file: {}", e))?;

    Ok(())
}
