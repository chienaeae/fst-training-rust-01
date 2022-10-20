use axum::{extract::Path, http::StatusCode, response::Json, Extension};
use snafu::OptionExt;
use uuid::Uuid;

use crate::{
    condition::Condition, domain::model, response::EncapsulatedJson, service::CardPersistService,
    web::error, Context,
};

const RESOURCE: &str = "Card";

pub async fn create<C>(
    Extension(ctx): Extension<C>,
    Json(card): Json<model::CreateCardRequest>,
) -> error::Result<EncapsulatedJson<model::Card>>
where
    C: Context + 'static,
{
    let service = ctx.card_persist_service();
    let id = service.create(&card).await?;
    let card = service
        .get_by_id(id)
        .await?
        .context(error::NotFoundSnafu { resource: RESOURCE, condition: Condition::with_id(id) })?;

    Ok(EncapsulatedJson::ok(card).status_code(StatusCode::CREATED))
}

pub async fn get_all<C>(
    Extension(ctx): Extension<C>,
) -> error::Result<EncapsulatedJson<Vec<model::Card>>>
where
    C: Context + 'static,
{
    let cards = ctx.card_persist_service().get_all().await?;

    Ok(EncapsulatedJson::ok(cards))
}

pub async fn get_by_id<C>(
    Extension(ctx): Extension<C>,
    Path(id): Path<Uuid>,
) -> error::Result<EncapsulatedJson<model::Card>>
where
    C: Context + 'static,
{
    let service = ctx.card_persist_service();
    let card = service
        .get_by_id(id)
        .await?
        .context(error::NotFoundSnafu { resource: RESOURCE, condition: Condition::with_id(id) })?;

    Ok(EncapsulatedJson::ok(card))
}

pub async fn update_by_id<C>(
    Extension(ctx): Extension<C>,
    Path(id): Path<Uuid>,
    Json(update_card): Json<model::UpdateCardRequest>,
) -> error::Result<EncapsulatedJson<model::Card>>
where
    C: Context + 'static,
{
    let service = ctx.card_persist_service();
    let card = service
        .update_by_id(id, &update_card)
        .await?
        .context(error::NotFoundSnafu { resource: RESOURCE, condition: Condition::with_id(id) })?;

    Ok(EncapsulatedJson::ok(card).status_code(StatusCode::CREATED))
}

pub async fn delete_by_id<C>(
    Extension(ctx): Extension<C>,
    Path(id): Path<Uuid>,
) -> error::Result<EncapsulatedJson<model::DeleteInfo>>
where
    C: Context + 'static,
{
    let service = ctx.card_persist_service();
    let id = service
        .delete_by_id(id)
        .await?
        .context(error::NotFoundSnafu { resource: RESOURCE, condition: Condition::with_id(id) })?;

    Ok(EncapsulatedJson::ok(model::DeleteInfo::new(id)))
}
