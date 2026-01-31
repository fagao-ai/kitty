use crate::apis::common_apis::CommonAPI;
use crate::proxy::delay::kitty_current_proxy_delay;
use crate::rules::Rule;
use crate::state::DatabaseState;
use crate::types::{CommandResult, KittyResponse};
use entity::base_config;
use sea_orm::DatabaseConnection;
use std::path::PathBuf;
use tauri::{Manager, State};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tracing_subscriber::EnvFilter;

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
use tauri::AppHandle;
use tauri::Runtime;

pub async fn copy_proxy_env<R: Runtime>(
    app_handle: &AppHandle<R>,
    db: &DatabaseConnection,
) -> CommandResult<KittyResponse<String>> {
    let proxy_string = CommonAPI::copy_proxy_env(db).await?;
    app_handle.clipboard().write_text(proxy_string).unwrap();
    Ok(KittyResponse::default())
}

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
#[tauri::command(rename_all = "snake_case")]
pub async fn copy_proxy_env_cmd<'a, R: Runtime>(
    app_handle: AppHandle<R>,
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<String>> {
    let db = state.get_db();
    Ok(copy_proxy_env(&app_handle, &db).await?)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn query_base_config<'a>(
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<base_config::Model>> {
    let db = state.get_db();
    let res = CommonAPI::query_base_config(&db).await?;
    Ok(res)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_base_config<'a>(
    state: State<'a, DatabaseState>,
    record: base_config::Model,
) -> CommandResult<KittyResponse<base_config::Model>> {
    let db = state.get_db();
    let res = CommonAPI::update_base_config(&db, record).await?;
    Ok(res)
}

/// Get the rules file path from the app data directory
fn get_rules_path<R: Runtime>(app_handle: &AppHandle<R>) -> PathBuf {
    app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("custom_rules.json")
}

#[tauri::command(rename_all = "snake_case")]
pub async fn add_rules<'a, R: Runtime>(
    app_handle: AppHandle<R>,
    records: Vec<Rule>,
) -> CommandResult<KittyResponse<()>> {
    let rules_path = get_rules_path(&app_handle);
    let _ = CommonAPI::add_rules(rules_path, records).await?;
    Ok(KittyResponse::default())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn query_rules<'a, R: Runtime>(
    app_handle: AppHandle<R>,
) -> CommandResult<KittyResponse<Vec<Rule>>> {
    let rules_path = get_rules_path(&app_handle);
    let res = CommonAPI::query_rules(rules_path).await?;
    Ok(res)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_rules<'a, R: Runtime>(
    app_handle: AppHandle<R>,
    ids: Vec<usize>,
) -> CommandResult<KittyResponse<()>> {
    let rules_path = get_rules_path(&app_handle);
    let _ = CommonAPI::delete_rules(rules_path, ids).await?;
    Ok(KittyResponse::default())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_rules_item<'a, R: Runtime>(
    app_handle: AppHandle<R>,
    records: Vec<Rule>,
) -> CommandResult<KittyResponse<()>> {
    let rules_path = get_rules_path(&app_handle);
    // Write all rules to file (replaces entire file)
    crate::rules::write_rules_file(&rules_path, &records)?;
    Ok(KittyResponse::default())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn test_current_proxy<'a>(
    proxy: String,
    target_url: String,
) -> CommandResult<KittyResponse<u128>> {
    let res = kitty_current_proxy_delay(proxy, target_url).await;
    Ok(KittyResponse::from_data(res))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_log_level<'a>(
    state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<String>> {
    let db = state.get_db();
    let record = base_config::Model::first(&db).await?
        .ok_or_else(|| anyhow::anyhow!("base_config not exists"))?;
    Ok(KittyResponse::from_data(record.log_level))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_log_level<'a>(
    state_db: State<'a, DatabaseState>,
    log_level: String,
) -> CommandResult<KittyResponse<()>> {
    // Validate log level
    let valid_levels = vec!["debug", "info", "warn", "error"];
    if !valid_levels.contains(&log_level.as_str()) {
        return Err(anyhow::anyhow!("Invalid log level: {}", log_level).into());
    }

    let db = state_db.get_db();
    let mut record = base_config::Model::first(&db).await?
        .ok_or_else(|| anyhow::anyhow!("base_config not exists"))?;

    // Update database
    record.log_level = log_level.clone();
    record.update(&db).await?;

    // Update runtime log level using global filter reload handle
    if let Some(filter_handle) = crate::get_filter_reload_handle() {
        if let Ok(handle) = filter_handle.lock() {
            let new_filter = format!("{},shoes={}", log_level, log_level);
            let _ = handle.modify(|filter| {
                *filter = EnvFilter::new(new_filter);
            });
            tracing::info!("Runtime log level updated to: {}", log_level);
        }
    }

    Ok(KittyResponse::default())
}

/// Export rules as JSON string
#[tauri::command(rename_all = "snake_case")]
pub async fn export_rules<'a, R: Runtime>(
    app_handle: AppHandle<R>,
) -> CommandResult<KittyResponse<String>> {
    let rules_path = get_rules_path(&app_handle);
    let rules = crate::rules::read_rules_file(&rules_path)?;

    // Convert to JSON string
    let json = serde_json::to_string_pretty(&rules)
        .map_err(|e| anyhow::anyhow!("Failed to serialize rules: {}", e))?;

    Ok(KittyResponse::from_data(json))
}

/// Import rules from JSON string
#[tauri::command(rename_all = "snake_case")]
pub async fn import_rules<'a, R: Runtime>(
    app_handle: AppHandle<R>,
    json_content: String,
) -> CommandResult<KittyResponse<()>> {
    // Parse JSON content
    let rules: Vec<Rule> = serde_json::from_str(&json_content)
        .map_err(|e| anyhow::anyhow!("Failed to parse rules JSON: {}", e))?;

    // Validate rules
    for rule in &rules {
        // Validate action
        match rule.action {
            crate::rules::RuleAction::Proxy | crate::rules::RuleAction::Direct | crate::rules::RuleAction::Reject => {},
        }
        // Validate rule type
        match rule.rule_type {
            crate::rules::RuleType::DomainSuffix | crate::rules::RuleType::DomainPrefix | crate::rules::RuleType::FullDomain | crate::rules::RuleType::Cidr | crate::rules::RuleType::DomainRoot => {},
        }
        // Validate pattern is not empty
        if rule.pattern.trim().is_empty() {
            return Err(anyhow::anyhow!("Rule pattern cannot be empty").into());
        }
    }

    // Write to rules file
    let rules_path = get_rules_path(&app_handle);
    crate::rules::write_rules_file(&rules_path, &rules)?;

    log::info!("Imported {} rules from JSON", rules.len());

    Ok(KittyResponse::default())
}
