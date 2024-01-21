use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .create_table(
                Table::create()
                    .table(Xray::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Xray::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Xray::Name).string().not_null())
                    .col(ColumnDef::new(Xray::Protocol).string().not_null())
                    .col(ColumnDef::new(Xray::Uuid).string().not_null())
                    .col(ColumnDef::new(Xray::Address).string().not_null())
                    .col(ColumnDef::new(Xray::Port).integer().not_null())
                    .col(ColumnDef::new(Xray::StreamSettings).json().not_null())
                    .col(ColumnDef::new(Xray::SubscribeId).integer().null())
                    .to_owned(),
            )
            .await;

        manager
            .create_table(
                Table::create()
                    .table(Subscribe::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subscribe::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Subscribe::Url).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .drop_table(Table::drop().table(Xray::Table).to_owned())
            .await;
        manager
            .drop_table(Table::drop().table(Subscribe::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Xray {
    Table,
    Id,
    Name,
    Protocol,
    Uuid,
    Address,
    Port,
    StreamSettings,
    SubscribeId,
}

#[derive(DeriveIden)]
enum Subscribe {
    Table,
    Id,
    Url,
}
