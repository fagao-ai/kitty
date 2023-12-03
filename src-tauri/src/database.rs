use rusqlite::{named_params, Connection};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

/// Initializes the database connection, creating the .sqlite file if needed, and upgrading the database
/// if it's out of date.

pub async fn init_db(app_dir: PathBuf) -> Result<DatabaseConnection, DbErr> {
    let sqlite_path = app_dir.join("MyApp.sqlite");
    let sqlite_url = format!("sqlite://{}", sqlite_path.to_string_lossy());
    let db: DatabaseConnection = Database::connect(sqlite_url).await?;
    // Migrator::up(&connection, None).await?;
    Ok(db)
}

// pub fn add_item(title: &str, db: &Connection) -> Result<(), rusqlite::Error> {
//     let mut statement = db.prepare("INSERT INTO items (title) VALUES (@title)")?;
//     statement.execute(named_params! { "@title": title })?;

//     Ok(())
// }

// pub fn get_all(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
//     let mut statement = db.prepare("SELECT * FROM items")?;
//     let mut rows = statement.query([])?;
//     let mut items = Vec::new();
//     while let Some(row) = rows.next()? {
//       let title: String = row.get("title")?;

//       items.push(title);
//     }

//     Ok(items)
// }
