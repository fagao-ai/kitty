pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_hysteria;
mod m20231210_094555_create_base_config;
mod m20231223_035153_create_xray;
mod m20240205_054639_add_rules;
mod m20241126_062352_add_lang_col;
mod m20260125_202910_add_log_level;
mod m20260130_120000_add_subscription_fields;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_hysteria::Migration),
            Box::new(m20231210_094555_create_base_config::Migration),
            Box::new(m20231223_035153_create_xray::Migration),
            Box::new(m20240205_054639_add_rules::Migration),
            Box::new(m20241126_062352_add_lang_col::Migration),
            Box::new(m20260125_202910_add_log_level::Migration),
            Box::new(m20260130_120000_add_subscription_fields::Migration),
        ]
    }
}
