use anyhow::anyhow;
use entity::subscribe;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tauri::State;

use crate::state::DatabaseState;
use crate::types::{CommandResult, KittyResponse};
use entity::xray;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, TransactionTrait, PaginatorTrait};

// Re-export chrono::Utc from sea-orm
use chrono::Utc;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubscriptionInfo {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub is_active: bool,
    pub node_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub last_sync_at: Option<String>,
}

/// Get all subscriptions with node count statistics
#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_subscriptions<'a>(
    db_state: State<'a, DatabaseState>,
) -> CommandResult<KittyResponse<Vec<SubscriptionInfo>>> {
    let db = db_state.get_db();
    let subscriptions = subscribe::Model::fetch_all(&db).await?;

    let mut result = Vec::new();
    for sub in subscriptions {
        let node_count = xray::Entity::find()
            .filter(xray::Column::SubscribeId.eq(sub.id))
            .count(&db)
            .await?;

        result.push(SubscriptionInfo {
            id: sub.id,
            name: sub.name,
            url: sub.url,
            is_active: sub.is_active,
            node_count: node_count as i64,
            created_at: sub.created_at.to_rfc3339(),
            updated_at: sub.updated_at.to_rfc3339(),
            last_sync_at: sub.last_sync_at.map(|dt| dt.to_rfc3339()),
        });
    }

    Ok(KittyResponse::from_data(result))
}

/// Create a new subscription (without importing nodes)
#[tauri::command(rename_all = "snake_case")]
pub async fn create_subscription<'a>(
    db_state: State<'a, DatabaseState>,
    name: String,
    url: String,
) -> CommandResult<KittyResponse<SubscriptionInfo>> {
    let db = db_state.get_db();

    // Validate URL
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(anyhow!("Only HTTP/HTTPS subscription URLs are supported").into());
    }

    // Check if URL already exists
    let existing = subscribe::Entity::find()
        .filter(subscribe::Column::Url.eq(&url))
        .one(&db)
        .await?;

    if existing.is_some() {
        return Err(anyhow!("Subscription URL already exists").into());
    }

    // Create subscription record
    use sea_orm::ActiveModelTrait;
    use sea_orm::Set;
    let now = Utc::now();
    let active_model = subscribe::ActiveModel {
        name: Set(name),
        url: Set(url),
        is_active: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
        last_sync_at: Set(None),
        ..Default::default()
    };

    let record = active_model.insert(&db).await?;

    Ok(KittyResponse::from_data(SubscriptionInfo {
        id: record.id,
        name: record.name,
        url: record.url,
        is_active: record.is_active,
        node_count: 0,
        created_at: record.created_at.to_rfc3339(),
        updated_at: record.updated_at.to_rfc3339(),
        last_sync_at: record.last_sync_at.map(|dt| dt.to_rfc3339()),
    }))
}

/// Update subscription (name and URL)
#[tauri::command(rename_all = "snake_case")]
pub async fn update_subscription<'a>(
    db_state: State<'a, DatabaseState>,
    id: i32,
    name: String,
    url: String,
) -> CommandResult<KittyResponse<SubscriptionInfo>> {
    let db = db_state.get_db();

    // Validate URL
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(anyhow!("Only HTTP/HTTPS subscription URLs are supported").into());
    }

    // Check if URL already exists (excluding current subscription)
    let existing = subscribe::Entity::find()
        .filter(subscribe::Column::Url.eq(&url))
        .filter(subscribe::Column::Id.ne(id))
        .one(&db)
        .await?;

    if existing.is_some() {
        return Err(anyhow!("Subscription URL already exists").into());
    }

    // Get existing record
    let record = subscribe::Model::get_by_id(&db, id)
        .await?
        .ok_or_else(|| anyhow!("Subscription not found"))?;

    // Update
    use sea_orm::ActiveModelTrait;
    use sea_orm::Set;
    let mut active_record: subscribe::ActiveModel = record.into();
    active_record.name = Set(name);
    active_record.url = Set(url);
    active_record.updated_at = Set(Utc::now());

    let updated = active_record.update(&db).await?;

    Ok(KittyResponse::from_data(SubscriptionInfo {
        id: updated.id,
        name: updated.name,
        url: updated.url,
        is_active: updated.is_active,
        node_count: xray::Entity::find()
            .filter(xray::Column::SubscribeId.eq(id))
            .count(&db)
            .await? as i64,
        created_at: updated.created_at.to_rfc3339(),
        updated_at: updated.updated_at.to_rfc3339(),
        last_sync_at: updated.last_sync_at.map(|dt| dt.to_rfc3339()),
    }))
}

/// Delete subscription and its associated nodes
#[tauri::command(rename_all = "snake_case")]
pub async fn delete_subscription<'a>(
    db_state: State<'a, DatabaseState>,
    id: i32,
) -> CommandResult<KittyResponse<()>> {
    let db = db_state.get_db();

    // Get subscription
    let _sub = subscribe::Model::get_by_id(&db, id)
        .await?
        .ok_or_else(|| anyhow!("Subscription not found"))?;

    // Use transaction for atomic delete
    let txn = db.begin().await?;

    // Delete associated xray nodes
    xray::Entity::delete_many()
        .filter(xray::Column::SubscribeId.eq(id))
        .exec(&txn)
        .await?;

    // Delete subscription
    subscribe::Model::delete_by_id(&txn, id).await?;

    txn.commit().await?;

    Ok(KittyResponse::default())
}

/// Switch to a subscription (clear old nodes, import new ones)
#[tauri::command(rename_all = "snake_case")]
pub async fn switch_subscription<'a>(
    db_state: State<'a, DatabaseState>,
    id: i32,
) -> CommandResult<KittyResponse<()>> {
    let db = db_state.get_db();

    // Get target subscription
    let target_sub = subscribe::Model::get_by_id(&db, id)
        .await?
        .ok_or_else(|| anyhow!("Subscription not found"))?;

    // Download and parse new subscription BEFORE transaction
    let subscriptions = crate::apis::parse_subscription::download_subcriptions(&target_sub.url)
        .await?;

    // Parse xray records BEFORE transaction
    let mut xray_models = Vec::new();
    for line in subscriptions {
        if !line.is_xray() {
            continue;
        }
        if let Ok(mut xray_model) = xray::Model::from_str(&line.line.trim()) {
            xray_model.subscribe_id = Some(id);
            xray_models.push(xray_model);
        }
    }

    if xray_models.is_empty() {
        return Err(anyhow!("No valid xray proxies found in subscription").into());
    }

    // NOW start transaction (after validation passes)
    let txn = db.begin().await?;

    // Find old active subscription
    if let Ok(Some(old_active)) = subscribe::Entity::find()
        .filter(subscribe::Column::IsActive.eq(true))
        .one(&txn)
        .await
    {
        // Delete old active subscription's nodes
        xray::Entity::delete_many()
            .filter(xray::Column::SubscribeId.eq(old_active.id))
            .exec(&txn)
            .await?;

        // Deactivate old subscription
        use sea_orm::ActiveModelTrait;
        use sea_orm::Set;
        let mut old_active_model: subscribe::ActiveModel = old_active.into();
        old_active_model.is_active = Set(false);
        old_active_model.update(&txn).await?;
    }

    // Insert new nodes
    xray::Model::insert_many(&txn, xray_models).await?;

    // Set new subscription as active and update timestamp
    use sea_orm::ActiveModelTrait;
    use sea_orm::Set;
    let mut target_model: subscribe::ActiveModel = target_sub.into();
    target_model.is_active = Set(true);
    target_model.updated_at = Set(Utc::now());
    target_model.last_sync_at = Set(Some(Utc::now()));
    target_model.update(&txn).await?;

    txn.commit().await?;

    Ok(KittyResponse::default())
}

/// Refresh (re-import nodes) for a subscription by ID
#[tauri::command(rename_all = "snake_case")]
pub async fn refresh_subscription<'a>(
    db_state: State<'a, DatabaseState>,
    id: i32,
) -> CommandResult<KittyResponse<()>> {
    let db = db_state.get_db();

    // Get subscription
    let sub = subscribe::Model::get_by_id(&db, id)
        .await?
        .ok_or_else(|| anyhow!("Subscription not found"))?;

    // Download and parse BEFORE transaction
    let subscriptions = crate::apis::parse_subscription::download_subcriptions(&sub.url)
        .await?;

    // Parse xray records BEFORE transaction
    let mut xray_models = Vec::new();
    for line in subscriptions {
        if !line.is_xray() {
            continue;
        }
        if let Ok(mut xray_model) = xray::Model::from_str(&line.line.trim()) {
            xray_model.subscribe_id = Some(id);
            xray_models.push(xray_model);
        }
    }

    // NOW start transaction
    let txn = db.begin().await?;

    // Delete old nodes
    xray::Entity::delete_many()
        .filter(xray::Column::SubscribeId.eq(id))
        .exec(&txn)
        .await?;

    // Insert new nodes (if any)
    if !xray_models.is_empty() {
        xray::Model::insert_many(&txn, xray_models).await?;
    }

    // Update subscription timestamp
    use sea_orm::ActiveModelTrait;
    use sea_orm::Set;
    let mut sub_model: subscribe::ActiveModel = sub.into();
    sub_model.updated_at = Set(Utc::now());
    sub_model.last_sync_at = Set(Some(Utc::now()));
    sub_model.update(&txn).await?;

    txn.commit().await?;

    Ok(KittyResponse::default())
}
