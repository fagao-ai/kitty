use crate::types::{CommandResult, KittyResponse};
use anyhow::Result;
use entity::base_config;
use sea_orm::DatabaseConnection;

pub struct CommonAPI;

impl CommonAPI {

    #[cfg(any(target_os = "macos", target_os = "linux", target_os="windows"))]
    pub async fn copy_proxy_env(db: &DatabaseConnection) -> Result<String> {
        let record = base_config::Model::first(db).await?.unwrap();
        let http_port = record.http_port;
        let socks_port = record.socks_port;
        #[cfg(target_os = "windows")]
            let env_expr = format!("export https_proxy=http://127.0.0.1:{http_port} http_proxy=http://127.0.0.1:{http_port} all_proxy=socks5://127.0.0.1:{socks_port}");

        #[cfg(any(target_os = "macos", target_os = "linux"))]
            let env_expr = format!("export https_proxy=http://127.0.0.1:{http_port} http_proxy=http://127.0.0.1:{http_port} all_proxy=socks5://127.0.0.1:{socks_port}");

        Ok(env_expr)
    }

    pub async fn query_base_config(
        db: &DatabaseConnection,
    ) -> CommandResult<KittyResponse<base_config::Model>> {
        let record = base_config::Model::first(db).await?;
        let response = match record {
            Some(record) => KittyResponse::<base_config::Model>::from_data(record),
            None => KittyResponse::from_msg(101, "base_config not exists"),
        };
        Ok(response)
    }

    pub async fn update_base_config(
        db: &DatabaseConnection,
        id: i32,
        record: base_config::Model,
    ) -> CommandResult<KittyResponse<base_config::Model>> {
        let updated_record = record.update(db, id).await?;
        Ok(KittyResponse::<base_config::Model>::from_data(
            updated_record,
        ))
    }
}
