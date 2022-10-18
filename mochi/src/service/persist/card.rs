use async_trait::async_trait;
use snafu::ResultExt;
use uuid::Uuid;

use crate::{
    domain::model,
    service::persist::{error, error::Result, DefaultPersistService},
};

#[async_trait]
pub trait CardPersistService: Send + Sync {
    async fn create(&self, card: &model::CreateCardRequest) -> Result<Uuid>;

    // async fn get_all(&self) -> Result<Vec<model::Card>>;

    // async fn get_by_id(&self) -> Result<Option<model::Card>>;

    // async fn update_by_id(&self) -> Result<Option<model::Card>>;

    // async fn delete_by_id(&self) -> Result<Option<Uuid>>;
}

#[async_trait]
impl CardPersistService for DefaultPersistService {
    async fn create(
        &self,
        model::CreateCardRequest { name, description }: &model::CreateCardRequest,
    ) -> Result<Uuid> {
        let record = sqlx::query_file!("sql/card/insert.sql", name, description)
            .fetch_one(&self.postgres)
            .await
            .context(error::CreateCardSnafu)?;

        Ok(record.id)
    }
}
