use mochi::error;

use snafu::Snafu;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("{source}"))]
    Common { source: Box<error::Error> },

    #[snafu(display("{source}"))]
    JwtAuthentication { source: Box<mochi::web::jwt::Error> },
}

impl From<error::Error> for Error {
    #[inline]
    fn from(source: error::Error) -> Self { Self::Common { source: Box::new(source) } }
}

impl From<mochi::web::jwt::Error> for Error {
    #[inline]
    fn from(source: mochi::web::jwt::Error) -> Self {
        Self::JwtAuthentication { source: Box::new(source) }
    }
}
