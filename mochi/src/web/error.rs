use std::borrow::Cow;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use indexmap::{indexmap, IndexMap};
use snafu::Snafu;

use crate::{
    condition::Condition, response, response::EncapsulatedJson, service::PersistError, utils,
};

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("{source}"))]
    Common { source: Box<self::Error> },

    #[snafu(display("{source}"))]
    PersistService { source: PersistError },

    #[snafu(display("Invalid authentication"))]
    InvalidAuthentication,

    #[snafu(display("not yet implemented"))]
    NotImplemented,

    #[snafu(display("resource {resource} not found"))]
    NotFound { resource: Cow<'static, str>, condition: Condition },

    #[snafu(display("bad request"))]
    BadRequest,
}

impl From<PersistError> for Error {
    #[inline]
    fn from(source: PersistError) -> Self { Self::PersistService { source } }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let err = match self {
            Self::Common { source } => return source.into_response(),
            Self::PersistService { source } => return source.into_response(),
            Self::InvalidAuthentication { .. } => response::Error {
                type_: response::ErrorType::Unauthorized,
                code: None,
                message: self.to_string(),
                additional_fields: IndexMap::default(),
            },
            Self::NotImplemented { .. } => response::Error {
                type_: response::ErrorType::Internal,
                code: None,
                message: "Unexpected internal system error.".to_string(),
                additional_fields: IndexMap::default(),
            },
            Self::NotFound { ref condition, .. } => response::Error {
                type_: response::ErrorType::NotFound,
                code: None,
                message: self.to_string(),
                additional_fields: indexmap! {"details".to_string() => utils::to_json_value(condition)},
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
            Self::InvalidAuthentication { .. } => StatusCode::UNAUTHORIZED,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
            Self::PersistService { .. } | Self::NotImplemented { .. } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
