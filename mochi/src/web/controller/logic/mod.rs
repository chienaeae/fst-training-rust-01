use axum::{routing, Router};
use saffron_client::GenericLogicClient;

use crate::Context;

pub fn v1<C, CoreClient>() -> Router
where
    C: Context + 'static,
    CoreClient: GenericLogicClient + Clone + 'static,
{
    Router::new().nest(
        "/v1/generic-logic",
        Router::new().route("/", routing::get(self::v1::get_all::<CoreClient>)).route(
            "/:id/link-card-by-id/:card_id",
            routing::post(self::v1::link_card::<C, CoreClient>)
                .delete(self::v1::unlink_card::<C, CoreClient>),
        ),
    )
}

mod v1;
