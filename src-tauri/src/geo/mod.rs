//! GeoIP and GeoSite data management module.
//!
//! This module handles downloading, parsing, and caching GeoIP and GeoSite data
//! from .dat files for use in routing rules.

pub mod geo_data_manager;
pub mod rule_expander;

// Re-export the generated protobuf module
pub mod v2ray_config;

pub use geo_data_manager::{DomainEntry, DomainType, GeoDataManager};
pub use rule_expander::{ConcreteRule, ExpandedRule, RuleExpander};
