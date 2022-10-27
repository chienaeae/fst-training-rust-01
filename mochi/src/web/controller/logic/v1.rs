use axum::{
    headers::{authorization::Bearer, Authorization},
    Extension, TypedHeader,
};

use saffron_client::GenericLogicClient;
use saffron_repository::model::GenericLogic;

use crate::{response::EncapsulatedJson, web::error};

pub async fn get_all<CoreClient>(
    Extension(core_client): Extension<CoreClient>,
    // Extension(claims): Extension<Claims>,
    TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
) -> error::Result<EncapsulatedJson<Vec<GenericLogic>>>
where
    CoreClient: GenericLogicClient + 'static,
{
    let token = bearer_token.token();

    let logics = GenericLogicClient::get_all(&core_client, token).await.unwrap();

    Ok(EncapsulatedJson::ok(logics))
}
