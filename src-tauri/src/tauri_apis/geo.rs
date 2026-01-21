//! GeoIP and GeoSite related Tauri APIs.
//!
//! This module provides commands for managing GeoIP and GeoSite data,
//! including loading, querying, and updating the geo data files.

use crate::config_converter::ShoesRuleEntry;
use crate::geo::{GeoDataManager, RuleExpander};
use anyhow::{anyhow, Result};
use entity::rules;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;

use crate::state::DatabaseState;
use crate::types::{CommandResult, KittyCommandError, KittyResponse};

/// Global state for GeoDataManager
pub struct GeoState(pub Mutex<Option<GeoDataManager>>);

/// Information about available GeoIP countries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoCountriesInfo {
    /// Available GeoIP country codes
    pub geoip_countries: Vec<String>,
    /// Available GeoSite country codes
    pub geosite_countries: Vec<String>,
}

/// Statistics about rule expansion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleExpansionStats {
    /// Map of rule ID to expanded rule count
    pub expansion_stats: HashMap<String, usize>,
    /// Total number of expanded rules
    pub total_expanded: usize,
}

/// Load or reload GeoIP and GeoSite data from .dat files.
///
/// This command loads the geo data files into memory for use in rule expansion.
#[tauri::command(rename_all = "snake_case")]
pub async fn load_geo_data<'a>(
    app_handle: AppHandle,
) -> CommandResult<KittyResponse<GeoCountriesInfo>> {
    // Get or create GeoDataManager
    let geo_state = app_handle.try_state::<GeoState>();
    if geo_state.is_none() {
        return Err(KittyCommandError::AnyHowError(anyhow!("GeoState not initialized")));
    }

    let geo_state = geo_state.unwrap();
    let mut manager_guard = geo_state.0.lock().await;

    // Initialize GeoDataManager if not already done
    if manager_guard.is_none() {
        *manager_guard = Some(GeoDataManager::new()?);
    }

    // Get mutable reference and load data
    if let Some(geo_manager) = manager_guard.as_mut() {
        geo_manager.load().await
            .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to load geo data: {}", e)))?;

        // Get available countries while still holding the lock
        let geoip_countries = geo_manager.get_geoip_countries();
        let geosite_countries = geo_manager.get_geosite_countries();

        return Ok(KittyResponse::from_data(GeoCountriesInfo {
            geoip_countries,
            geosite_countries,
        }));
    }

    Err(KittyCommandError::AnyHowError(anyhow!("Failed to access GeoDataManager")))
}

/// Get information about available GeoIP/GeoSite countries.
#[tauri::command(rename_all = "snake_case")]
pub async fn get_geo_countries_info<'a>(
    app_handle: AppHandle,
) -> CommandResult<KittyResponse<GeoCountriesInfo>> {
    let geo_state = app_handle.try_state::<GeoState>();
    if geo_state.is_none() {
        return Err(KittyCommandError::AnyHowError(anyhow!("GeoState not initialized")));
    }

    let geo_state = geo_state.unwrap();
    let manager_guard = geo_state.0.lock().await;

    let geo_manager = manager_guard.as_ref()
        .ok_or_else(|| anyhow!("Geo data not loaded. Call load_geo_data first."))?;

    let geoip_countries = geo_manager.get_geoip_countries();
    let geosite_countries = geo_manager.get_geosite_countries();

    Ok(KittyResponse::from_data(GeoCountriesInfo {
        geoip_countries,
        geosite_countries,
    }))
}

/// Check if a specific country code is available in GeoIP.
#[tauri::command(rename_all = "snake_case")]
pub async fn has_geoip<'a>(
    app_handle: AppHandle,
    country_code: String,
) -> CommandResult<KittyResponse<bool>> {
    let geo_state = app_handle.try_state::<GeoState>();
    if geo_state.is_none() {
        return Ok(KittyResponse::from_data(false));
    }

    let geo_state = geo_state.unwrap();
    let manager_guard = geo_state.0.lock().await;

    let has_geoip = manager_guard
        .as_ref()
        .and_then(|m| Some(m.has_geoip(&country_code)))
        .unwrap_or(false);

    Ok(KittyResponse::from_data(has_geoip))
}

/// Check if a specific code is available in GeoSite.
#[tauri::command(rename_all = "snake_case")]
pub async fn has_geosite<'a>(
    app_handle: AppHandle,
    code: String,
) -> CommandResult<KittyResponse<bool>> {
    let geo_state = app_handle.try_state::<GeoState>();
    if geo_state.is_none() {
        return Ok(KittyResponse::from_data(false));
    }

    let geo_state = geo_state.unwrap();
    let manager_guard = geo_state.0.lock().await;

    let has_geosite = manager_guard
        .as_ref()
        .and_then(|m| Some(m.has_geosite(&code)))
        .unwrap_or(false);

    Ok(KittyResponse::from_data(has_geosite))
}

/// Expand database rules and return statistics about the expansion.
///
/// This command expands GeoIP/GeoSite rules into concrete CIDR and domain rules,
/// returning statistics about how many rules were generated.
#[tauri::command(rename_all = "snake_case")]
pub async fn expand_rules_stats<'a>(
    app_handle: AppHandle,
    db_state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<RuleExpansionStats>> {
    let geo_state = app_handle.try_state::<GeoState>();
    if geo_state.is_none() {
        return Err(KittyCommandError::AnyHowError(anyhow!("GeoState not initialized")));
    }

    let geo_state = geo_state.unwrap();
    let manager_guard = geo_state.0.lock().await;

    let geo_manager = manager_guard.as_ref()
        .ok_or_else(|| anyhow!("Geo data not loaded. Call load_geo_data first."))?;

    // Fetch all database rules
    let db = db_state.get_db();
    let db_rules = rules::Model::fetch_all(&db).await
        .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to fetch rules: {}", e)))?;

    // Expand rules
    let expanded_rules = RuleExpander::expand_rules(db_rules, geo_manager)
        .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to expand rules: {}", e)))?;

    // Calculate statistics
    let mut expansion_stats = HashMap::new();
    let mut total_expanded = 0;

    for rule in &expanded_rules {
        let count = rule.concrete_rules.len();
        expansion_stats.insert(rule.source_id.to_string(), count);
        total_expanded += count;
    }

    Ok(KittyResponse::from_data(RuleExpansionStats {
        expansion_stats,
        total_expanded,
    }))
}

/// Generate shoes-compatible rules from database rules.
///
/// This command expands all rules and returns them in a format suitable
/// for the shoes configuration.
#[tauri::command(rename_all = "snake_case")]
pub async fn get_shoes_rules<'a>(
    app_handle: AppHandle,
    db_state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<Vec<ShoesRuleEntry>>> {
    let geo_state = app_handle.try_state::<GeoState>();
    if geo_state.is_none() {
        return Err(KittyCommandError::AnyHowError(anyhow!("GeoState not initialized")));
    }

    let geo_state = geo_state.unwrap();
    let manager_guard = geo_state.0.lock().await;

    let geo_manager = manager_guard.as_ref()
        .ok_or_else(|| anyhow!("Geo data not loaded. Call load_geo_data first."))?;

    // Fetch all database rules
    let db = db_state.get_db();
    let db_rules = rules::Model::fetch_all(&db).await
        .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to fetch rules: {}", e)))?;

    // Expand rules and convert to shoes format
    let expanded_rules = RuleExpander::expand_rules(db_rules, geo_manager)
        .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to expand rules: {}", e)))?;

    let shoes_rules = RuleExpander::to_shoes_rules(expanded_rules)
        .map_err(|e| KittyCommandError::AnyHowError(anyhow!("Failed to convert to shoes rules: {}", e)))?;

    let rule_entries = shoes_rules.into_iter()
        .map(|(masks, action)| ShoesRuleEntry { masks, action })
        .collect();

    Ok(KittyResponse::from_data(rule_entries))
}
