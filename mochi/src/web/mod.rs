// FIXME: allow: clippy bug https://github.com/rust-lang/rust-clippy/issues/8757
#![allow(clippy::trait_duplication_in_bounds)]

use std::net::SocketAddr;

pub(crate) mod controller;
mod error;
pub mod jwt;

use axum::{Extension, Router};
use tower_http::auth::AsyncRequireAuthorizationLayer;

use crate::{axum::AxumWebServer, Context};

/// # Errors
///
/// * if it cannot create runtime
/// * if it cannot bind server
pub fn new_api_server<C, CoreClient, E>(
    socket_address: SocketAddr,
    authorization_secret: &str,
    core_client: CoreClient,
    ctx: C,
) -> Result<AxumWebServer<E>, E>
where
    C: Context + 'static,
    CoreClient: saffron_client::DataProcessClient
        + saffron_client::ApiRouteClient
        + saffron_client::GenericLogicClient
        + saffron_client::AggregatorLogicClient
        + saffron_client::TagClient
        + saffron_client::DatabaseAgentConfigurationClient
        + saffron_client::FileStorageAgentConfigurationClient
        + saffron_client::HttpAgentConfigurationClient
        + saffron_client::MailAgentConfigurationClient
        + saffron_client::LogicCompilationTaskClient
        + Clone
        + Send
        + Sync
        + 'static,
    E: std::error::Error + From<crate::error::Error> + From<jwt::error::Error> + Send,
{
    let private_router = self::controller::api_v1_index::<C>()
        .layer(AsyncRequireAuthorizationLayer::new(jwt::JwtAuth::new(authorization_secret)?))
        .layer(Extension(core_client))
        .layer(Extension(ctx));

    let router = Router::new().merge(private_router);

    Ok(AxumWebServer::new("API Server", socket_address, router))
}
