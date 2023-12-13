use entity::{base_config, hysteria};
use migration::{Migrator, MigratorTrait};
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, DbErr};
use serde_json::Value;
use std::path::PathBuf;

pub async fn init_db(app_dir: PathBuf) -> Result<DatabaseConnection, DbErr> {
    let sqlite_path = app_dir.join("MyApp.sqlite");
    let sqlite_url = format!("sqlite://{}?mode=rwc", sqlite_path.to_string_lossy());
    let db: DatabaseConnection = Database::connect(&sqlite_url).await?;
    Migrator::up(&db, Some(1)).await?;
    // let migrations = Migrator::get_applied_migrations(&db).await?;

    Ok(db)
}

pub async fn add_hysteria_item(
    db: &DatabaseConnection,
    record: hysteria::Model,
) -> Result<hysteria::Model, DbErr> {
    let json_value: Value = serde_json::to_value(record).unwrap();

    let hysteria_record = hysteria::ActiveModel::from_json(json_value)?;
    let hysteria_res = hysteria_record.insert(db).await;
    hysteria_res
}

pub async fn get_all_hysteria_item(db: &DatabaseConnection) -> Result<Vec<hysteria::Model>, DbErr> {
    let hysterias = hysteria::Entity::find().all(db).await?;
    Ok(hysterias)
}

pub async fn add_base_config(
    db: &DatabaseConnection,
    record: base_config::Model,
) -> Result<base_config::Model, DbErr> {
    let json_value: Value = serde_json::to_value(record).unwrap();
    let base_config_record = base_config::ActiveModel::from_json(json_value)?;
    let base_config_res = base_config_record.insert(db).await;
    base_config_res
}

pub async fn get_base_config(db: &DatabaseConnection) -> Result<Option<base_config::Model>, DbErr> {
    let base_config_record = base_config::Entity::find().one(db).await?;
    Ok(base_config_record)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::setup_schema;
    use hysteria;
    #[tokio::test]
    async fn test_add_hysteria_item() {
        let config_str = r#"{
          "server": "ip:port",
          "auth": "password",
          "bandwidth": {
            "up": "10 mbps",
            "down": "100 mbps"
          },
          "tls": {
            "sni": "bing.com",
            "insecure": true
          }
        }"#;
        let hy_record: hysteria::Model = serde_json::from_str(&config_str).unwrap();
        let db = Database::connect("sqlite::memory:").await.unwrap();
        

        // Setup database schema
        setup_schema(&db, hysteria::Entity).await;
        setup_schema(&db, base_config::Entity).await;
        

        add_hysteria_item(&db, hy_record).await.unwrap();
        let hysterias = get_all_hysteria_item(&db).await.unwrap();
        assert_eq!(hysterias[0].id, 1);
    }
}
