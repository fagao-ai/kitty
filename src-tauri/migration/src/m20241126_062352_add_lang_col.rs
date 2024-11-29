use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(BaseConfig::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Alias::new("language"))
                            .string()
                            .not_null()
                            .default("zh-CN"),
                    )
                    .to_owned(),
            )
            .await;
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(BaseConfig::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Alias::new("update_interval"))
                            .integer()
                            .not_null()
                            .default(3),
                    )
                    .to_owned(),
            )
            .await;
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(BaseConfig::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Alias::new("allow_lan"))
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await;
        manager
            .alter_table(
                Table::alter()
                    .table(BaseConfig::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Alias::new("mode"))
                            .string()
                            .not_null()
                            .default("Rules"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(BaseConfig::Table)
                    .drop_column(Alias::new("language"))
                    .to_owned(),
            )
            .await;

        let _ = manager
            .alter_table(
                Table::alter()
                    .table(BaseConfig::Table)
                    .drop_column(Alias::new("update_interval"))
                    .to_owned(),
            )
            .await;
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(BaseConfig::Table)
                    .drop_column(Alias::new("allow_lan"))
                    .to_owned(),
            )
            .await;
        manager
            .alter_table(
                Table::alter()
                    .table(BaseConfig::Table)
                    .drop_column(Alias::new("mode"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum BaseConfig {
    Table,
}
