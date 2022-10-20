use crate::{
    condition::Condition,
    domain::{entity, model},
    service::persist::{error, error::Result, DefaultPersistService},
};
use async_trait::async_trait;
use snafu::ResultExt;
use uuid::Uuid;

#[async_trait]
pub trait CardPersistService: Send + Sync {
    async fn create(&self, card: &model::CreateCardRequest) -> Result<Uuid>;

    async fn get_all(&self) -> Result<Vec<model::Card>>;

    async fn get_by_id(&self, id: Uuid) -> Result<Option<model::Card>>;

    async fn update_by_id(
        &self,
        id: Uuid,
        model::UpdateCardRequest { name, description }: &model::UpdateCardRequest,
    ) -> Result<Option<model::Card>>;

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Uuid>>;
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

    async fn get_all(&self) -> Result<Vec<model::Card>> {
        let card_entities = sqlx::query_file_as!(entity::Card, "sql/card/get_all.sql")
            .fetch_all(&self.postgres)
            .await
            .context(error::GetAllCardSnafu)?;

        let cards = card_entities.into_iter().map(model::Card::from).collect();

        Ok(cards)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<model::Card>> {
        let card_entity = sqlx::query_file_as!(entity::Card, "sql/card/get_by_id.sql", id)
            .fetch_optional(&self.postgres)
            .await
            .context(error::GetCardSnafu { condition: Condition::with_id(id) })?;
        let card = card_entity.map(model::Card::from);

        Ok(card)
    }

    async fn update_by_id(
        &self,
        id: Uuid,
        model::UpdateCardRequest { name, description }: &model::UpdateCardRequest,
    ) -> Result<Option<model::Card>> {
        let card_entity =
            sqlx::query_file_as!(entity::Card, "sql/card/update_by_id.sql", id, name, description)
                .fetch_optional(&self.postgres)
                .await
                .context(error::UpdateCardSnafu { condition: Condition::with_id(id) })?;

        let card = card_entity.map(model::Card::from);

        Ok(card)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Uuid>> {
        let id = sqlx::query_file!("sql/card/delete_by_id.sql", id)
            .fetch_optional(&self.postgres)
            .await
            .context(error::DeleteCardSnafu { condition: Condition::with_id(id) })?
            .map(|record| record.id);

        Ok(id)
    }
}
