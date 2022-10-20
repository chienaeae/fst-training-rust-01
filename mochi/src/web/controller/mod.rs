mod card;

use axum::Router;

use crate::Context;

pub fn api_v1_index<C>() -> Router
where
    C: Context + 'static,
{
    Router::new().nest("/api", Router::new().merge(self::card::v1::<C>()))
}
