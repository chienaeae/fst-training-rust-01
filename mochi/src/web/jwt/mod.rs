mod claim;
pub mod error;

use std::sync::Arc;

use axum::{body::BoxBody, response::IntoResponse};
use chrono::{NaiveDateTime, Utc};
use futures::future::BoxFuture;
use hyper::{header, Request, Response};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use snafu::ResultExt;
use tower_http::auth::AsyncAuthorizeRequest;

use crate::web::error as web_error;

pub use self::{
    claim::Claims,
    error::{Error, Result},
};

pub struct JwtAuth {
    secret: Arc<DecodingKey>,
}

impl JwtAuth {
    /// # Errors
    ///
    /// if secret is not the correct RSA PEM format
    pub fn new(secret: &str) -> error::Result<Self> {
        Ok(Self {
            secret: Arc::new(
                DecodingKey::from_rsa_pem(secret.as_bytes())
                    .with_context(|_| error::ReadFromRsaPemSnafu {})?,
            ),
        })
    }
}

impl Clone for JwtAuth {
    fn clone(&self) -> Self { Self { secret: self.secret.clone() } }
}

impl<B> AsyncAuthorizeRequest<B> for JwtAuth
where
    B: Send + Sync + 'static,
{
    type Future = BoxFuture<'static, std::result::Result<Request<B>, Response<Self::ResponseBody>>>;
    type RequestBody = B;
    type ResponseBody = BoxBody;

    fn authorize(&mut self, mut request: Request<B>) -> Self::Future {
        let secret_key = self.secret.clone();

        Box::pin(async move {
            authorize_request(&mut request, secret_key.as_ref()).map(|claims| {
                request.extensions_mut().insert(claims);
                request
            })
        })
    }
}

fn authorize_request<B>(
    request: &mut Request<B>,
    secret_key: &DecodingKey,
) -> std::result::Result<Claims, Response<BoxBody>>
where
    B: Send + Sync,
{
    let authorization_value = request.headers().get(header::AUTHORIZATION).map(Clone::clone);

    if let Some(auth) = authorization_value {
        let auth = auth.to_str().unwrap_or_default().trim_start_matches("Bearer ");

        // Decode the user data form JWT
        let validation = Validation::new(Algorithm::RS256);

        match decode::<Claims>(auth, secret_key, &validation) {
            Ok(token_data) => {
                let claims = token_data.claims;

                // check token expiration time
                if Utc::now().naive_utc() > NaiveDateTime::from_timestamp(claims.exp, 0) {
                    return web_error::InvalidAuthenticationSnafu {}
                        .fail()
                        .map_err(IntoResponse::into_response);
                }

                Ok(claims)
            }
            Err(_err) => {
                web_error::InvalidAuthenticationSnafu {}.fail().map_err(IntoResponse::into_response)
            }
        }
    } else {
        web_error::InvalidAuthenticationSnafu {}.fail().map_err(IntoResponse::into_response)
    }
}
