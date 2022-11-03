use axum::{
    extract::Path,
    headers::{authorization::Bearer, Authorization},
    Extension, TypedHeader,
};

use saffron_client::GenericLogicClient;
use saffron_repository::model::GenericLogic;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{
    condition::Condition, domain::model, response::EncapsulatedJson, service::CardPersistService,
    web::error, Context,
};

const RESOURCE: &str = "Logic";

pub async fn get_all<CoreClient>(
    Extension(core_client): Extension<CoreClient>,
    TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
) -> error::Result<EncapsulatedJson<Vec<GenericLogic>>>
where
    CoreClient: GenericLogicClient + 'static,
{
    let token = bearer_token.token();

    let logics = GenericLogicClient::get_all(&core_client, token)
        .await
        .context(error::SaffronClientSnafu {})?;

    Ok(EncapsulatedJson::ok(logics))
}

pub async fn link_card<C, CoreClient>(
    Extension(ctx): Extension<C>,
    Extension(core_client): Extension<CoreClient>,
    TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
    Path((id, card_id)): Path<(Uuid, Uuid)>,
) -> error::Result<EncapsulatedJson<model::LinkedLogicInfo>>
where
    C: Context + 'static,
    CoreClient: GenericLogicClient + 'static,
{
    let token = bearer_token.token();

    let service = ctx.card_persist_service();

    GenericLogicClient::get_latest_by_permanent_identity(&core_client, token, id)
        .await
        .context(error::SaffronClientSnafu {})?;

    let card_id =
        service.link_to_generic_logic_id(card_id, id).await.context(error::PersistServiceSnafu)?;

    Ok(EncapsulatedJson::ok(model::LinkedLogicInfo { id, card_id }))
}

pub async fn unlink_card<C, CoreClient>(
    Extension(ctx): Extension<C>,
    Extension(core_client): Extension<CoreClient>,
    TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
    Path((id, card_id)): Path<(Uuid, Uuid)>,
) -> error::Result<EncapsulatedJson<model::DeleteInfo>>
where
    C: Context + 'static,
    CoreClient: GenericLogicClient + 'static,
{
    let token = bearer_token.token();

    let service = ctx.card_persist_service();

    GenericLogicClient::get_latest_by_permanent_identity(&core_client, token, id)
        .await
        .context(error::SaffronClientSnafu {})?;

    let card_id = service
        .unlink_from_generic_logic_id(card_id, id)
        .await?
        .context(error::NotFoundSnafu { resource: RESOURCE, condition: Condition::with_id(id) })?;

    Ok(EncapsulatedJson::ok(model::DeleteInfo::new(card_id)))
}
