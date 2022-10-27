use axum::{routing, Router};
use saffron_client::GenericLogicClient;

pub fn v1<CoreClient>() -> Router
where
    CoreClient: GenericLogicClient + Clone + 'static,
{
    Router::new().nest(
        "/v1/generic-logic",
        Router::new().route("/", routing::get(self::v1::get_all::<CoreClient>)),
    )
}

mod v1;
