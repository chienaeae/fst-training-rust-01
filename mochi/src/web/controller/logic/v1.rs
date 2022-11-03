use axum::{
    extract::Path,
    headers::{authorization::Bearer, Authorization},
    Extension, TypedHeader,
};

use saffron_client::GenericLogicClient;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{
    condition::Condition, domain::model, response::EncapsulatedJson, service::CardPersistService,
    web::error, Context,
};

const RESOURCE: &str = "Logic";

pub async fn get_all<C, CoreClient>(
    Extension(ctx): Extension<C>,
    Extension(core_client): Extension<CoreClient>,
    TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
) -> error::Result<EncapsulatedJson<Vec<model::Logic>>>
where
    C: Context + 'static,
    CoreClient: GenericLogicClient + 'static,
{
    let token = bearer_token.token();

    let service = ctx.card_persist_service();

    let logics = GenericLogicClient::get_all(&core_client, token)
        .await
        .context(error::SaffronClientSnafu {})?;

    let mut result: Vec<model::Logic> = Vec::new();

    for logic in &logics {
        let linked_logic_info = service
            .get_linked_logic_info_by_generic_logic_id(logic.metadata.permanent_identity)
            .await
            .context(error::PersistServiceSnafu)?;

        let item = model::Logic { metadata: logic.clone(), linked_card: linked_logic_info };

        result.push(item);
    }

    Ok(EncapsulatedJson::ok(result))
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
