use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Subscribe::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Subscribe::Name)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Subscribe::IsActive)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Subscribe::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Subscribe::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Subscribe::LastSyncAt)
                            .date_time()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add index on is_active for faster queries
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_subscribe_is_active")
                    .table(Subscribe::Table)
                    .col(Subscribe::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Subscribe::Table)
                    .drop_column(Subscribe::Name)
                    .drop_column(Subscribe::IsActive)
                    .drop_column(Subscribe::CreatedAt)
                    .drop_column(Subscribe::UpdatedAt)
                    .drop_column(Subscribe::LastSyncAt)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_subscribe_is_active")
                    .table(Subscribe::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Subscribe {
    Table,
    Name,
    IsActive,
    CreatedAt,
    UpdatedAt,
    LastSyncAt,
}
