use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::path::PathBuf;

pub async fn init_db(app_dir: PathBuf) -> Result<DatabaseConnection, DbErr> {
    println!("app_dir");
    let sqlite_path = app_dir.join("MyApp.sqlite");
    let sqlite_url = format!("sqlite://{}?mode=rwc", sqlite_path.to_string_lossy());
    let db: DatabaseConnection = Database::connect(&sqlite_url).await?;
    Migrator::up(&db, None).await?;
    println!("Migrator");
    Ok(db)
}
