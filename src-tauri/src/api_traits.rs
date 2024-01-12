use sea_orm::{ActiveModelBehavior, DatabaseConnection, EntityTrait};
use entity::base_config;
use crate::types::{CommandResult, KittyResponse};
use protocols::CommandManagerTrait;

pub trait APIServiceTrait {
    async fn query_base_config(db: &DatabaseConnection) -> CommandResult<KittyResponse<base_config::Model>> {
        let record = base_config::Model::first(db).await?;
        let response = match record {
            Some(record) => KittyResponse::<base_config::Model>::from_data(record),
            None => KittyResponse::from_msg(101, "base_config not exists"),
        };
        Ok(response)
    }

    async fn update_base_config(
        db: &DatabaseConnection,
        id: i32,
        record: base_config::Model,
    ) -> CommandResult<KittyResponse<base_config::Model>> {
        let updated_record = record.update(db, id).await?;
        Ok(KittyResponse::<base_config::Model>::from_data(
            updated_record,
        ))
    }

    async fn add_protocol_item<T: ActiveModelBehavior + EntityTrait>(db: &DatabaseConnection, record: T) -> CommandResult<()>;

    #[cfg(feature = "hysteria")]
    async fn get_protocol_status<T: CommandManagerTrait>(
        command_manager: &T
    ) -> CommandResult<KittyResponse<bool>> {
        let res = command_manager.is_running();
        Ok(KittyResponse::from_data(res))
    }
}