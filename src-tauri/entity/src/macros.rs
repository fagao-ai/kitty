#[macro_export]
macro_rules! generate_model_functions {
    () => {
        pub async fn insert_one<C>(&self, db: &C) -> Result<Self, DbErr>
        where
            C: ConnectionTrait,
        {
            let json_value = serde_json::to_value(self).unwrap().into();
            let mut record = ActiveModel::from_json(json_value)?;
            record.id = NotSet;
            let res = record.insert(db).await;
            res
        }

        pub async fn insert_many<C>(db: &C, records: Vec<Model>) -> Result<(), DbErr>
        where
            C: ConnectionTrait,
        {
            let mut active_models = Vec::with_capacity(records.len());
            for record in records {
                let json_value = serde_json::to_value(record).unwrap().into();
                let mut record = ActiveModel::from_json(json_value)?;
                record.id = NotSet;
                active_models.push(record)
            }
            let _ = self::Entity::insert_many(active_models).exec(db).await?;
            Ok(())
        }

        pub async fn first<C>(db: &C) -> Result<Option<Self>, DbErr>
        where
            C: ConnectionTrait,
        {
            let record = self::Entity::find().one(db).await?;
            Ok(record)
        }

        pub async fn update<C>(&self, db: &C) -> Result<self::Model, DbErr>
        where
            C: ConnectionTrait,
        {
            let origin_id = self.id;
            let json_value = serde_json::to_value(self).unwrap();
            let record = self::Entity::find_by_id(origin_id).one(db).await?;
            let mut record: self::ActiveModel = record.unwrap().into();
            let _ = record.set_from_json(json_value);
            let res = record.update(db).await?;
            Ok(res)
        }

        pub async fn fetch_all<C>(db: &C) -> Result<Vec<Model>, DbErr>
        where
            C: ConnectionTrait,
        {
            let results = self::Entity::find().all(db).await?;
            Ok(results)
        }

        pub async fn fetch_by_ids<C>(db: &C, ids: Vec<i32>) -> Result<Vec<Model>, DbErr>
        where
            C: ConnectionTrait,
        {
            let results = self::Entity::find()
                .filter(self::Column::Id.is_in(ids))
                .all(db)
                .await?;
            Ok(results)
        }

        pub async fn get_by_id<C>(db: &C, id: i32) -> Result<Option<Model>, DbErr>
        where
            C: ConnectionTrait,
        {
            let model = self::Entity::find_by_id(id).one(db).await?;
            Ok(model)
        }

        pub async fn delete_by_id<C>(db: &C, id: i32) -> Result<(), DbErr>
        where
            C: ConnectionTrait,
        {
            let _ = Entity::delete_by_id(id).exec(db).await?;
            Ok(())
        }
    };
}
