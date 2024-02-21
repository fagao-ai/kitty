use crate::types::{CommandResult, KittyResponse};
use anyhow::Result;
use entity::{base_config, rules};
use sea_orm::{ConnectionTrait, DatabaseConnection};

pub struct CommonAPI;

impl CommonAPI {
    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
    pub async fn copy_proxy_env<C>(db: &C) -> Result<String>
    where
        C: ConnectionTrait,
    {
        use sea_orm::ConnectionTrait;

        let record = base_config::Model::first(db).await?.unwrap();
        let http_port = record.http_port;
        let socks_port = record.socks_port;
        #[cfg(target_os = "windows")]
            let env_expr = format!("set https_proxy=http://127.0.0.1:{http_port} http_proxy=http://127.0.0.1:{http_port} all_proxy=socks5://127.0.0.1:{socks_port}");

        #[cfg(any(target_os = "macos", target_os = "linux"))]
            let env_expr = format!("export https_proxy=http://127.0.0.1:{http_port} http_proxy=http://127.0.0.1:{http_port} all_proxy=socks5://127.0.0.1:{socks_port}");

        Ok(env_expr)
    }

    pub async fn query_base_config<C>(db: &C) -> CommandResult<KittyResponse<base_config::Model>>
    where
        C: ConnectionTrait,
    {
        let record = base_config::Model::first(db).await?;
        let response = match record {
            Some(record) => KittyResponse::<base_config::Model>::from_data(record),
            None => KittyResponse::from_msg(101, "base_config not exists"),
        };
        Ok(response)
    }

    pub async fn update_base_config<C>(
        db: &C,
        record: base_config::Model,
    ) -> CommandResult<KittyResponse<base_config::Model>>
    where
        C: ConnectionTrait,
    {
        let updated_record = record.update(db).await?;
        Ok(KittyResponse::<base_config::Model>::from_data(
            updated_record,
        ))
    }

    pub async fn add_rules<C>(
        db: &C,
        records: Vec<rules::Model>,
    ) -> CommandResult<KittyResponse<()>>
    where
        C: ConnectionTrait,
    {
        let _ = rules::Model::insert_many(db, records).await?;
        Ok(KittyResponse::default())
    }

    pub async fn query_rules<C>(db: &C) -> CommandResult<KittyResponse<Vec<rules::Model>>>
    where
        C: ConnectionTrait,
    {
        let res = rules::Model::fetch_all(db).await?;
        Ok(KittyResponse::from_data(res))
    }

    pub async fn delete_rules<C>(db: &C, ids: Vec<i32>) -> CommandResult<KittyResponse<()>>
    where
        C: ConnectionTrait,
    {
        let _ = rules::Model::delete_by_ids(db, ids).await?;
        Ok(KittyResponse::default())
    }

    pub async fn update_rules<C>(
        db: &C,
        record: rules::Model,
    ) -> CommandResult<KittyResponse<rules::Model>>
    where
        C: ConnectionTrait,
    {
        let updated_record = record.update(db).await?;
        Ok(KittyResponse::<rules::Model>::from_data(updated_record))
    }
}
