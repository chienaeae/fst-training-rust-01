use snafu::{Backtrace, Snafu};

use crate::error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Failed to parse RSA PEM public key{}", error::fmt_source(source)))]
    ReadFromRsaPem { source: jsonwebtoken::errors::Error, backtrace: Backtrace },
}
