use axum::{routing, Router};

use crate::Context;

pub fn v1<C>() -> Router
where
    C: Context + 'static,
{
    Router::new().nest(
        "/v1/card",
        Router::new()
            .route("/", routing::post(self::v1::create::<C>).get(self::v1::get_all::<C>))
            .route(
                "/:id",
                routing::get(self::v1::get_by_id)
                    .put(self::v1::update_by_id)
                    .delete(self::v1::delete_by_id),
            ),
    )
}

mod v1;
