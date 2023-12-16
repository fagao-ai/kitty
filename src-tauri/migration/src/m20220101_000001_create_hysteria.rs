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
                    .table(Hysteria::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Hysteria::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Hysteria::Name).string().not_null())
                    .col(ColumnDef::new(Hysteria::Server).string().not_null())
                    .col(ColumnDef::new(Hysteria::Auth).string().not_null())
                    .col(ColumnDef::new(Hysteria::Tls).json().not_null())
                    .col(ColumnDef::new(Hysteria::Bandwidth).json().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Hysteria::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Hysteria {
    Table,
    Id,
    Name,
    Server,
    Auth,
    Tls,
    Bandwidth,
}
