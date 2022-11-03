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

    async fn exists(&self, id: Uuid) -> Result<bool>;

    async fn get_linked_logic_info_by_generic_logic_id(
        &self,
        generic_logic_id: Uuid,
    ) -> Result<Vec<model::LinkedLogicInfo>>;

    async fn get_linked_logic_info_by_id(&self, id: Uuid) -> Result<Vec<model::LinkedLogicInfo>>;

    async fn link_to_generic_logic_id(&self, id: Uuid, generic_logic_id: Uuid) -> Result<Uuid>;

    async fn unlink_from_generic_logic_id(
        &self,
        id: Uuid,
        generic_logic_id: Uuid,
    ) -> Result<Option<Uuid>>;
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

    async fn exists(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query_file_as!(entity::Card, "sql/card/get_by_id.sql", id)
            .fetch_optional(&self.postgres)
            .await
            .context(error::GetCardSnafu { condition: Condition::with_id(id) })?;

        Ok(result.is_some())
    }

    async fn get_linked_logic_info_by_generic_logic_id(
        &self,
        generic_logic_id: Uuid,
    ) -> Result<Vec<model::LinkedLogicInfo>> {
        let linked_generic_logic_entities = sqlx::query_file_as!(
            entity::LinkedGenericLogic,
            "sql/card_linked_generic_logic/get_by_generic_logic_id.sql",
            generic_logic_id
        )
        .fetch_all(&self.postgres)
        .await
        .context(error::GetCardLinkedGenericLogicSnafu)?;
        let linked_logic_info =
            linked_generic_logic_entities.into_iter().map(model::LinkedLogicInfo::from).collect();

        Ok(linked_logic_info)
    }

    async fn get_linked_logic_info_by_id(&self, id: Uuid) -> Result<Vec<model::LinkedLogicInfo>> {
        let linked_generic_logic_entities = sqlx::query_file_as!(
            entity::LinkedGenericLogic,
            "sql/card_linked_generic_logic/get_by_card_id.sql",
            id
        )
        .fetch_all(&self.postgres)
        .await
        .context(error::GetCardLinkedGenericLogicSnafu)?;

        let linked_logic_info =
            linked_generic_logic_entities.into_iter().map(model::LinkedLogicInfo::from).collect();

        Ok(linked_logic_info)
    }

    async fn link_to_generic_logic_id(&self, id: Uuid, generic_logic_id: Uuid) -> Result<Uuid> {
        let card_exists = self.exists(id).await?;
        if !card_exists {
            return error::CardNotExistsSnafu { condition: Condition::with_id(id) }.fail();
        }

        let linked_logic_info = self.get_linked_logic_info_by_id(id).await?;
        if let Some(..) = linked_logic_info.iter().position(|r| r.id == generic_logic_id) {
            return error::LinkedGenericLogicExistsSnafu { condition: Condition::with_id(id) }
                .fail();
        }

        let record =
            sqlx::query_file!("sql/card_linked_generic_logic/insert.sql", id, generic_logic_id)
                .fetch_one(&self.postgres)
                .await
                .context(error::LinkedGenericLogicIdSnafu { condition: Condition::with_id(id) })?;

        Ok(record.id)
    }

    async fn unlink_from_generic_logic_id(
        &self,
        id: Uuid,
        generic_logic_id: Uuid,
    ) -> Result<Option<Uuid>> {
        let id = sqlx::query_file!(
            "sql/card_linked_generic_logic/delete_by_id.sql",
            id,
            generic_logic_id
        )
        .fetch_optional(&self.postgres)
        .await
        .context(error::LinkedGenericLogicIdSnafu { condition: Condition::with_id(id) })?
        .map(|record| record.id);

        Ok(id)
    }
}
