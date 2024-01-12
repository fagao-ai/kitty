use sea_orm::{ActiveModelBehavior, DatabaseConnection, EntityTrait};
use crate::api_traits::APIServiceTrait;
use crate::types::{CommandResult};

struct HysteriaAPI;


impl APIServiceTrait for HysteriaAPI {
    #[cfg(feature = "hysteria")]
    async fn add_protocol_item<T: ActiveModelBehavior + EntityTrait>(db: &DatabaseConnection, record: T) -> CommandResult<()> {
        record.insert_one(db).await?;
        Ok(())
    }
}