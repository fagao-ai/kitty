use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::path::PathBuf;

pub async fn init_db(app_dir: PathBuf) -> Result<DatabaseConnection, DbErr> {
    let sqlite_path = app_dir.join("MyApp.sqlite");
    let sqlite_url = format!("sqlite://{}?mode=rwc", sqlite_path.to_string_lossy());
    let db: DatabaseConnection = Database::connect(&sqlite_url).await?;
    Migrator::up(&db, None).await?;
    // let migrations = Migrator::get_applied_migrations(&db).await?;

    Ok(db)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::utils::setup_schema;
//     use hysteria;
//     #[tokio::test]
//     async fn test_add_hysteria_item() {
//         let config_str = r#"{
//           "server": "ip:port",
//           "auth": "password",
//           "bandwidth": {
//             "up": "10 mbps",
//             "down": "100 mbps"
//           },
//           "tls": {
//             "sni": "bing.com",
//             "insecure": true
//           }
//         }"#;
//         let hy_record: hysteria::Model = serde_json::from_str(&config_str).unwrap();
//         let db = Database::connect("sqlite::memory:").await.unwrap();

//         // Setup database schema
//         setup_schema(&db, hysteria::Entity).await;
//         // setup_schema(&db, base_config::Entity).await;

//         add_hysteria_item(&db, hy_record).await.unwrap();
//         let hysterias = get_all_hysteria_item(&db).await.unwrap();
//         assert_eq!(hysterias[0].id, 1);
//     }
// }
