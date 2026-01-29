//! Unified proxy management commands.
//!
//! This module provides Tauri commands for managing proxy records.
//! The actual proxy serving is done via the shoes library.

use anyhow::anyhow;
use entity::{hysteria, xray};
use serde::{Deserialize, Serialize};
use serde_json;
use std::str::FromStr;
use tauri::State;

use crate::proxy::delay::{kitty_proxies_delay, ProxyInfo};
use crate::state::DatabaseState;
use crate::types::{CommandResult, KittyResponse};

/// Unified proxy type for frontend.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Proxy {
    pub id: i32,
    pub name: String,
    pub proxy_type: String, // "xray" or "hysteria"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

impl From<hysteria::Model> for Proxy {
    fn from(h: hysteria::Model) -> Self {
        Self {
            id: h.id,
            name: h.name,
            proxy_type: "hysteria".to_string(),
            protocol: Some("hysteria2".to_string()),
            server: Some(h.server),
            port: None, // hysteria doesn't have port in config
        }
    }
}

impl From<xray::Model> for Proxy {
    fn from(x: xray::Model) -> Self {
        Self {
            id: x.id,
            name: x.name,
            proxy_type: "xray".to_string(),
            protocol: Some(serde_json::to_string(&x.protocol).unwrap().trim_matches('"').to_string()),
            server: Some(x.address),
            port: Some(x.port),
        }
    }
}

/// Get all proxy records (both xray and hysteria).
#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_proxies<'a>(
    db_state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<Vec<Proxy>>> {
    let db = db_state.get_db();
    let mut proxies = Vec::new();

    // Get hysteria records
    let hysteria_records = hysteria::Model::fetch_all(&db).await?;
    for record in hysteria_records {
        proxies.push(Proxy::from(record));
    }

    // Get xray records
    let xray_records = xray::Model::fetch_all(&db).await?;
    for record in xray_records {
        proxies.push(Proxy::from(record));
    }

    Ok(KittyResponse::from_data(proxies))
}

// ============================================================================
// Hysteria Commands (kept for compatibility)
// ============================================================================

/// Get all hysteria proxy records.
#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_hysterias<'a>(
    db_state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<Vec<hysteria::Model>>> {
    let db = db_state.get_db();
    let records = hysteria::Model::fetch_all(&db).await?;
    Ok(KittyResponse::from_data(records))
}

/// Add a new hysteria proxy record.
#[tauri::command(rename_all = "snake_case")]
pub async fn add_hysteria_item<'a>(
    db_state: State<'a, DatabaseState>,
    record: hysteria::Model,
) -> CommandResult<KittyResponse<hysteria::Model>> {
    let db = db_state.get_db();
    let result = record.insert_one(&db).await?;
    Ok(KittyResponse::from_data(result))
}

/// Update an existing hysteria proxy record.
#[tauri::command(rename_all = "snake_case")]
pub async fn update_hysteria_item<'a>(
    db_state: State<'a, DatabaseState>,
    record: hysteria::Model,
) -> CommandResult<KittyResponse<hysteria::Model>> {
    let db = db_state.get_db();
    let result = record.update(&db).await?;
    Ok(KittyResponse::from_data(result))
}

/// Delete a hysteria proxy record by ID.
#[tauri::command(rename_all = "snake_case")]
pub async fn delete_hysteria_item<'a>(
    db_state: State<'a, DatabaseState>,
    id: i32,
) -> CommandResult<KittyResponse<()>> {
    let db = db_state.get_db();
    hysteria::Model::delete_by_id(&db, id).await?;
    Ok(KittyResponse::default())
}

/// Get a hysteria proxy record by ID.
#[tauri::command(rename_all = "snake_case")]
pub async fn get_hysteria_by_id<'a>(
    db_state: State<'a, DatabaseState>,
    id: i32,
) -> CommandResult<KittyResponse<Option<hysteria::Model>>> {
    let db = db_state.get_db();
    let record = hysteria::Model::get_by_id(&db, id).await?;
    Ok(KittyResponse::from_data(record))
}

// ============================================================================
// Xray Commands (kept for compatibility)
// ============================================================================

/// Get all xray proxy records.
#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_xrays<'a>(
    db_state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<Vec<xray::Model>>> {
    let db = db_state.get_db();
    let records = xray::Model::fetch_all(&db).await?;
    Ok(KittyResponse::from_data(records))
}

/// Add a new xray proxy record.
#[tauri::command(rename_all = "snake_case")]
pub async fn add_xray_item<'a>(
    db_state: State<'a, DatabaseState>,
    record: xray::Model,
) -> CommandResult<KittyResponse<xray::Model>> {
    let db = db_state.get_db();
    let result = record.insert_one(&db).await?;
    Ok(KittyResponse::from_data(result))
}

/// Update an existing xray proxy record.
#[tauri::command(rename_all = "snake_case")]
pub async fn update_xray_item<'a>(
    db_state: State<'a, DatabaseState>,
    record: xray::Model,
) -> CommandResult<KittyResponse<xray::Model>> {
    let db = db_state.get_db();
    let result = record.update(&db).await?;
    Ok(KittyResponse::from_data(result))
}

/// Delete an xray proxy record by ID.
#[tauri::command(rename_all = "snake_case")]
pub async fn delete_xray_item<'a>(
    db_state: State<'a, DatabaseState>,
    id: i32,
) -> CommandResult<KittyResponse<()>> {
    let db = db_state.get_db();
    xray::Model::delete_by_id(&db, id).await?;
    Ok(KittyResponse::default())
}

/// Get an xray proxy record by ID.
#[tauri::command(rename_all = "snake_case")]
pub async fn get_xray_by_id<'a>(
    db_state: State<'a, DatabaseState>,
    id: i32,
) -> CommandResult<KittyResponse<Option<xray::Model>>> {
    let db = db_state.get_db();
    let record = xray::Model::get_by_id(&db, id).await?;
    Ok(KittyResponse::from_data(record))
}

// ============================================================================
// Subscription Commands
// ============================================================================

/// Batch get all subscriptions.
#[tauri::command(rename_all = "snake_case")]
pub async fn batch_get_subscriptions<'a>(
    db_state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<Vec<entity::subscribe::Model>>> {
    let db = db_state.get_db();
    let records = entity::subscribe::Model::fetch_all(&db).await?;
    Ok(KittyResponse::from_data(records))
}

/// Refresh subscriptions.
/// If record_ids is provided, refresh only those subscriptions.
/// Otherwise, refresh all subscriptions.
#[tauri::command(rename_all = "snake_case")]
pub async fn refresh_subscription<'a>(
    db_state: State<'a, DatabaseState>,
    record_ids: Option<Vec<i32>>,
) -> CommandResult<KittyResponse<()>> {
    let db = db_state.get_db();

    // Fetch subscriptions to refresh
    let subscriptions = if let Some(ids) = record_ids {
        entity::subscribe::Model::fetch_by_ids(&db, ids).await?
    } else {
        entity::subscribe::Model::fetch_all(&db).await?
    };

    if subscriptions.is_empty() {
        return Ok(KittyResponse::default());
    }

    // Refresh each subscription
    use sea_orm::{ModelTrait, TransactionTrait};
    for subscribe_item in subscriptions {
        // Download new subscription content
        let subscriptions_result =
            crate::apis::parse_subscription::download_subcriptions(&subscribe_item.url).await;

        let new_subscriptions = match subscriptions_result {
            Ok(subs) => subs,
            Err(e) => {
                log::warn!(
                    "Failed to download subscription (id: {}): {}",
                    subscribe_item.id,
                    e
                );
                continue; // Skip this subscription and continue with others
            }
        };

        // Start transaction for this subscription
        let txn = match db.begin().await {
            Ok(t) => t,
            Err(e) => {
                log::error!(
                    "Failed to start transaction for subscription (id: {}): {}",
                    subscribe_item.id,
                    e
                );
                continue;
            }
        };

        // Get and delete old xray records
        let old_xray_records = subscribe_item
            .find_related(xray::Entity)
            .all(&db)
            .await
            .unwrap_or_default();

        let xray_ids: Vec<i32> = old_xray_records.iter().map(|x| x.id).collect();
        if !xray_ids.is_empty() {
            if let Err(e) = xray::Model::delete_by_ids(&txn, xray_ids).await {
                log::error!(
                    "Failed to delete old xray records for subscription (id: {}): {}",
                    subscribe_item.id,
                    e
                );
                continue;
            }
        }

        // Parse and insert new xray records
        let mut xray_models = Vec::new();
        for line in new_subscriptions {
            if !line.is_xray() {
                continue;
            }
            if let Ok(mut xray_model) = xray::Model::from_str(&line.line.trim()) {
                xray_model.subscribe_id = Some(subscribe_item.id);
                xray_models.push(xray_model);
            }
        }

        if !xray_models.is_empty() {
            if let Err(e) = xray::Model::insert_many(&txn, xray_models).await {
                log::error!(
                    "Failed to insert new xray records for subscription (id: {}): {}",
                    subscribe_item.id,
                    e
                );
                continue;
            }
        }

        // Commit transaction
        if let Err(e) = txn.commit().await {
            log::error!(
                "Failed to commit transaction for subscription (id: {}): {}",
                subscribe_item.id,
                e
            );
        }
    }

    Ok(KittyResponse::default())
}

/// Import subscription from URL.
#[tauri::command(rename_all = "snake_case")]
pub async fn import_subscription<'a>(
    db_state: State<'a, DatabaseState>,
    url: String,
) -> CommandResult<KittyResponse<()>> {
    let db = db_state.get_db();

    // Validate URL format - must be http/https
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(anyhow!("Only HTTP/HTTPS subscription URLs are supported").into());
    }

    // Check if subscription URL already exists
    use sea_orm::EntityTrait;
    use sea_orm::ColumnTrait;
    use sea_orm::QueryFilter;
    let existing = entity::subscribe::Entity::find()
        .filter(entity::subscribe::Column::Url.eq(&url))
        .one(&db)
        .await?;

    if existing.is_some() {
        return Err(anyhow!("Subscription URL already exists").into());
    }

    // Download and parse subscription
    let subscriptions = crate::apis::parse_subscription::download_subcriptions(&url).await?;

    // Start transaction
    use sea_orm::TransactionTrait;
    let txn = db.begin().await?;

    // Create subscription record
    use sea_orm::ActiveModelTrait;
    use sea_orm::Set;
    let subscribe = entity::subscribe::ActiveModel {
        url: Set(url.clone()),
        ..Default::default()
    };
    let subscribe_record = subscribe.insert(&txn).await?;

    // Parse and insert xray records
    let mut xray_models = Vec::new();
    for line in subscriptions {
        if !line.is_xray() {
            continue;
        }
        if let Ok(mut xray_model) = xray::Model::from_str(&line.line.trim()) {
            xray_model.subscribe_id = Some(subscribe_record.id);
            xray_models.push(xray_model);
        }
    }

    if xray_models.is_empty() {
        return Err(anyhow!("No valid xray proxies found in subscription").into());
    }

    xray::Model::insert_many(&txn, xray_models).await?;

    // Commit transaction
    txn.commit().await?;

    Ok(KittyResponse::default())
}

/// Legacy alias for compatibility.
#[tauri::command(rename_all = "snake_case")]
pub async fn refresh_xray_subscription<'a>(
    db_state: State<'a, DatabaseState>,
    record_ids: Option<Vec<i32>>,
) -> CommandResult<KittyResponse<()>> {
    refresh_subscription(db_state, record_ids).await
}

/// Legacy alias for compatibility.
#[tauri::command(rename_all = "snake_case")]
pub async fn import_xray_subscribe<'a>(
    db_state: State<'a, DatabaseState>,
    url: String,
) -> CommandResult<KittyResponse<()>> {
    import_subscription(db_state, url).await
}

/// Test proxy delay.
#[tauri::command(rename_all = "snake_case")]
pub async fn proxies_delay_test<'a>(
    db_state: State<'a, DatabaseState>,
    proxies: Vec<ProxyInfo>,
) -> CommandResult<KittyResponse<Vec<crate::proxy::delay::ProxyDelay>>> {
    let results = kitty_proxies_delay(proxies).await;
    Ok(KittyResponse::from_data(results))
}
