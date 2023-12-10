use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BaseConfig::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BaseConfig::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BaseConfig::HttpPort).string().not_null())
                    .col(ColumnDef::new(BaseConfig::SocksPort).string().not_null())
                    .to_owned(),
            )
            .await;
        let insert = Query::insert()
            .into_table(BaseConfig::Table)
            .columns([BaseConfig::SocksPort])
            .values_panic([10086.into()])
            .columns([BaseConfig::HttpPort])
            .values_panic([10087.into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BaseConfig::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BaseConfig {
    Table,
    Id,
    HttpPort,
    SocksPort,
}
