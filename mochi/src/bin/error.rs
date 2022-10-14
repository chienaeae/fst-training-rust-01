use mochi::error;

use snafu::Snafu;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("{source}"))]
    Common { source: Box<error::Error> },
}

impl From<error::Error> for Error {
    #[inline]
    fn from(source: error::Error) -> Self { Self::Common { source: Box::new(source) } }
}
