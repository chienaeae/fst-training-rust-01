use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use indexmap::IndexMap;
use snafu::{Backtrace, Snafu};

use crate::{
    condition::Condition,
    error,
    response::{self, EncapsulatedJson},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    Common,
    #[snafu(display(
        "Error occurs while creating `card` in PostgreSQL{}",
        error::fmt_backtrace_with_source(backtrace, source)
    ))]
    CreateCard {
        source: sqlx::Error,
        backtrace: Backtrace,
    },

    #[snafu(display(
        "Error occurs while getting all `card` in PostgreSQL{}",
        error::fmt_backtrace_with_source(backtrace, source)
    ))]
    GetAllCard {
        source: sqlx::Error,
        backtrace: Backtrace,
    },

    #[snafu(display(
        "Error occurs while getting `card`{} from PostgreSQL{}",
        fmt_condition(condition),
        error::fmt_backtrace_with_source(backtrace, source)
    ))]
    GetCard {
        condition: Condition,
        source: sqlx::Error,
        backtrace: Backtrace,
    },

    #[snafu(display(
        "Error occurs while updating `card`{} from PostgreSQL{}",
        fmt_condition(condition),
        error::fmt_backtrace_with_source(backtrace, source)
    ))]
    UpdateCard {
        condition: Condition,
        source: sqlx::Error,
        backtrace: Backtrace,
    },

    #[snafu(display(
        "Error occurs while deleting `card`{} from PostgreSQL{}",
        fmt_condition(condition),
        error::fmt_backtrace_with_source(backtrace, source)
    ))]
    DeleteCard {
        condition: Condition,
        source: sqlx::Error,
        backtrace: Backtrace,
    },

    #[snafu(display(
        "Error occurs while getting `card_linked_generic_logic` in PostgreSQL{}",
        error::fmt_backtrace_with_source(backtrace, source)
    ))]
    GetCardLinkedGenericLogic {
        source: sqlx::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("`card`{} doesn't exist`", fmt_condition(condition)))]
    CardNotExists {
        condition: Condition,
        backtrace: Backtrace,
    },

    #[snafu(display(
        "Error occurs while linking `card_linked_generic_logic`{} from PostgreSQL{}",
        fmt_condition(condition),
        error::fmt_backtrace_with_source(backtrace, source)
    ))]
    LinkedGenericLogicId {
        condition: Condition,
        source: sqlx::Error,
        backtrace: Backtrace,
    },

    #[snafu(display(
        "Generic Logic already linked `card_linked_generic_logic`{}",
        fmt_condition(condition),
    ))]
    LinkedGenericLogicExists {
        condition: Condition,
        backtrace: Backtrace,
    },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let err = match self {
            Self::CardNotExists { ref condition, .. } => response::Error {
                type_: response::ErrorType::NotComplete,
                code: None,
                message: format!("The card `{condition}` doesn't exists."),
                additional_fields: IndexMap::default(),
            },
            Self::LinkedGenericLogicExists { ref condition, .. } => response::Error {
                type_: response::ErrorType::NotComplete,
                code: None,
                message: format!("The generic Logic `{condition}` has already been linked."),
                additional_fields: IndexMap::default(),
            },
            _ => response::Error {
                type_: response::ErrorType::Internal,
                code: None,
                message: "Unexpected internal system error.".to_string(),
                additional_fields: IndexMap::default(),
            },
        };

        let status_code = self.status_code();
        EncapsulatedJson::<()>::err(err).status_code(status_code).into_response()
    }
}

impl Error {
    #[inline]
    #[must_use]
    pub const fn status_code(&self) -> StatusCode {
        match self {
            Self::CardNotExists { .. } | Self::LinkedGenericLogicExists { .. } => {
                StatusCode::BAD_REQUEST
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
fn fmt_condition(condition: &Condition) -> String {
    if condition.is_empty() {
        return String::new();
    }
    condition.to_string()
}
