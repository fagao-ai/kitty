use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(BaseConfig::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Alias::new("log_level"))
                            .string()
                            .not_null()
                            .default("debug"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(BaseConfig::Table)
                    .drop_column(Alias::new("log_level"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum BaseConfig {
    Table,
}
