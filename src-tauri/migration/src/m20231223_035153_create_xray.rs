use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
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
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Xray::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Xray {
    Table,
    Id,
    Name,
}


// struct Mux {
//     pub enabled: bool,
//     pub concurrency: i64,
// }
//
// struct Headers {
//     pub host: String,
// }
//
// struct WsSettings {
//     pub path: String,
//     pub headers: Headers,
// }
//
// struct TlsSettings {
//     pub allow_insecure: bool,
//     pub server_name: String,
//     pub fingerprint: String,
// }
//
// struct StreamSettings {
//     pub network: String,
//     pub security: String,
//     pub tls_settings: TlsSettings,
//     pub ws_settings: WsSettings,
// }
//
// struct Struct1 {
//     pub id: String,
//     pub alter_id: i64,
//     pub email: String,
//     pub security: String,
//     pub encryption: String,
//     pub flow: String,
// }
//
// struct Struct {
//     pub address: String,
//     pub port: i64,
//     pub users: Vec<Struct1>,
// }
//
// struct Settings {
//     pub vnext: Vec<Struct>,
// }
//
// struct Root {
//     pub tag: String,
//     pub protocol: String,
//     pub settings: Settings,
//     pub stream_settings: StreamSettings,
//     pub mux: Mux,
// }