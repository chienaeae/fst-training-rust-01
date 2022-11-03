mod card;
mod logic;

use axum::Router;
use saffron_client::GenericLogicClient;

use crate::Context;

pub fn api_v1_index<C, CoreClient>() -> Router
where
    C: Context + 'static,
    CoreClient: GenericLogicClient + Clone + 'static,
{
    Router::new().nest(
        "/api",
        Router::new().merge(self::card::v1::<C>()).merge(self::logic::v1::<C, CoreClient>()),
    )
}
