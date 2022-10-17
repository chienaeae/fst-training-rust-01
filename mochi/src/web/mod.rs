use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub(crate) mod controller;
mod error;

use axum::Router;

use crate::axum::AxumWebServer;

/// # Errors
///
/// * if it cannot create runtime
/// * if it cannot bind server
pub fn new_api_server<E>() -> Result<AxumWebServer<E>, E>
where
    E: std::error::Error + From<crate::error::Error> + Send,
{
    let private_router = self::controller::api_v1_index();

    let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
    let router = Router::new().merge(private_router);
    let server = AxumWebServer::new("API Server", socket_address, router);

    Ok(server)
}
