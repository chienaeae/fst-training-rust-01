use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use indexmap::IndexMap;
use snafu::Snafu;

use crate::{response, response::EncapsulatedJson};

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("{source}"))]
    Common { source: Box<self::Error> },

    #[snafu(display("not yet implemented"))]
    NotImplemented,

    #[snafu(display("bad request"))]
    BadRequest,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let err = match self {
            Self::Common { source } => return source.into_response(),

            Self::NotImplemented { .. } => response::Error {
                type_: response::ErrorType::Internal,
                code: None,
                message: "Unexpected internal system error.".to_string(),
                additional_fields: IndexMap::default(),
            },
            Self::BadRequest { .. } => response::Error {
                type_: response::ErrorType::Request,
                code: None,
                message: "Unexpected request.".to_string(),
                additional_fields: IndexMap::default(),
            },
        };

        EncapsulatedJson::<()>::err(err).status_code(self.status_code()).into_response()
    }
}

impl Error {
    #[inline]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::Common { source } => source.status_code(),
            Self::NotImplemented { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
