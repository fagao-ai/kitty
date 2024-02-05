use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Rules::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Rules::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Rules::RuleAction).string().not_null())
                    .col(ColumnDef::new(Rules::RuleType).string().not_null())
                    .col(ColumnDef::new(Rules::Rule).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Rules::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Rules {
    Table,
    Id,
    RuleAction,
    RuleType,
    Rule,
}
