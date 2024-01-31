#[macro_export]
macro_rules! generate_model_functions {
    () => {
        pub async fn insert_one(&self, db: &DatabaseConnection) -> Result<Self, DbErr> {
            let json_value = serde_json::to_value(self).unwrap().into();
            let record = ActiveModel::from_json(json_value)?;
            let res = record.insert(db).await;
            res
        }

        pub async fn first(db: &DatabaseConnection) -> Result<Option<Self>, DbErr> {
            let record = self::Entity::find().one(db).await?;
            Ok(record)
        }

        pub async fn update(&self, db: &DatabaseConnection) -> Result<self::Model, DbErr> {
            let origin_id = self.id;
            let json_value = serde_json::to_value(self).unwrap();
            let record = self::Entity::find_by_id(origin_id).one(db).await?;
            let mut record: self::ActiveModel = record.unwrap().into();
            let _ = record.set_from_json(json_value);
            let res = record.update(db).await?;
            Ok(res)
        }

        pub async fn fetch_all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
            let results = self::Entity::find().all(db).await?;
            Ok(results)
        }

        pub async fn fetch_by_ids(db: &DatabaseConnection, ids: Vec<i32>) -> Result<Vec<Model>, DbErr> {
            let results = self::Entity::find()
                .filter(self::Column::Id.is_in(ids))
                .all(db)
                .await?;
            Ok(results)
        }

        pub async fn insert_many(
            db: &DatabaseConnection,
            records: Vec<Model>,
        ) -> Result<(), DbErr> {
            let mut active_models = Vec::with_capacity(records.len());
            for record in records {
                let json_value = serde_json::to_value(record).unwrap().into();
                let record = ActiveModel::from_json(json_value)?;
                active_models.push(record)
            }
            let _ = self::Entity::insert_many(active_models).exec(db).await?;
            Ok(())
        }

        pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> Result<()> {
            let _ = Entity::delete_by_id(id).exec(db).await?;
            Ok(())
        }
    };
}
