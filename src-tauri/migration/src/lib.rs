pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_hysteria;
mod m20231210_094555_create_base_config;mod m20231223_035153_create_xray;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_hysteria::Migration),
            Box::new(m20231210_094555_create_base_config::Migration),
        ]
    }
}
