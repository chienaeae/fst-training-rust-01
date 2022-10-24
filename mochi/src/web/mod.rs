use std::net::SocketAddr;

pub(crate) mod controller;
mod error;

use axum::{Extension, Router};

use crate::{axum::AxumWebServer, Context};

/// # Errors
///
/// * if it cannot create runtime
/// * if it cannot bind server
pub fn new_api_server<C, E>(socket_address: SocketAddr, ctx: C) -> Result<AxumWebServer<E>, E>
where
    C: Context + 'static,
    E: std::error::Error + From<crate::error::Error> + Send,
{
    let private_router = self::controller::api_v1_index::<C>().layer(Extension(ctx));

    let router = Router::new().merge(private_router);

    Ok(AxumWebServer::new("API Server", socket_address, router))
}
