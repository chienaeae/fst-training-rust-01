use std::{fmt, marker::PhantomData, net::SocketAddr};

use axum::{body::Body, Router};
use snafu::ResultExt;

use crate::error;

pub struct AxumWebServer<E> {
    name: String,
    socket_address: SocketAddr,
    router: Router<Body>,
    error_type: PhantomData<E>,
}

impl<E> AxumWebServer<E>
where
    E: From<crate::error::Error> + Send,
{
    #[inline]
    #[must_use]
    pub fn new<T>(name: T, socket_address: SocketAddr, router: Router<Body>) -> Self
    where
        T: fmt::Display,
    {
        Self { name: name.to_string(), socket_address, router, error_type: PhantomData::default() }
    }

    #[inline]
    #[must_use]
    pub fn name(&self) -> &str { &self.name }

    /// # Errors
    ///
    /// * if it cannot create runtime
    /// * if it cannot bind server
    pub async fn serve(self) -> Result<(), E> {
        let router = { self.router.into_make_service_with_connect_info::<SocketAddr>() };

        axum::Server::bind(&self.socket_address)
            .serve(router)
            .await
            .context(error::AxumServerSnafu)?;

        Ok(())
    }
}
