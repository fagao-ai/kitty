use sea_orm_migration::prelude::*;
use sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite doesn't support IF NOT EXISTS in ALTER TABLE ADD COLUMN
        // Use raw SQL and ignore errors if columns already exist
        let conn = manager.get_connection();

        // Add name column if not exists
        let _ = conn
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE subscribe ADD COLUMN name TEXT NOT NULL DEFAULT ''".to_string(),
            ))
            .await;

        // Add is_active column if not exists
        let _ = conn
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE subscribe ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT FALSE"
                    .to_string(),
            ))
            .await;

        // Add created_at column if not exists
        let _ = conn
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE subscribe ADD COLUMN created_at TEXT NOT NULL DEFAULT '2026-01-30 00:00:00'".to_string(),
            ))
            .await;

        // Add updated_at column if not exists
        let _ = conn
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE subscribe ADD COLUMN updated_at TEXT NOT NULL DEFAULT '2026-01-30 00:00:00'".to_string(),
            ))
            .await;

        // Add last_sync_at column if not exists
        let _ = conn
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE subscribe ADD COLUMN last_sync_at TEXT".to_string(),
            ))
            .await;

        // Add index on is_active for faster queries (if not exists)
        let _ = conn
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "CREATE INDEX IF NOT EXISTS idx_subscribe_is_active ON subscribe(is_active)"
                    .to_string(),
            ))
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite doesn't support multiple ALTER TABLE operations in one statement
        // Drop each column separately
        let conn = manager.get_connection();

        // Drop columns (ignore errors if not exist)
        let _ = conn
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE subscribe DROP COLUMN name".to_string(),
            ))
            .await;

        let _ = conn
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE subscribe DROP COLUMN is_active".to_string(),
            ))
            .await;

        let _ = conn
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE subscribe DROP COLUMN created_at".to_string(),
            ))
            .await;

        let _ = conn
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE subscribe DROP COLUMN updated_at".to_string(),
            ))
            .await;

        let _ = conn
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE subscribe DROP COLUMN last_sync_at".to_string(),
            ))
            .await;

        // Drop index
        let _ = conn
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "DROP INDEX IF EXISTS idx_subscribe_is_active".to_string(),
            ))
            .await;

        Ok(())
    }
}
