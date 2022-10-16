mod card;

use axum::Router;

pub fn api_v1_index() -> Router {
    Router::new().nest("/api", Router::new().merge(self::card::v1()))
}
