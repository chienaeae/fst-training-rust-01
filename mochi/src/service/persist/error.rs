use std::fmt;

use snafu::{Backtrace, Snafu};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display(
        "Error occurs while creating `Unit` in PostgreSQL{}",
        fmt_backtrace_with_source(backtrace, source)
    ))]
    CreateCard { source: sqlx::Error, backtrace: Backtrace },
}

#[inline]
#[must_use]
pub fn fmt_backtrace_with_source(backtrace: &Backtrace, source: impl fmt::Display) -> String {
    format!("{}{}", fmt_backtrace(backtrace), fmt_source(source))
}

#[inline]
#[must_use]
pub fn fmt_backtrace(backtrace: &Backtrace) -> String {
    if cfg!(feature = "backtrace") {
        format!("\n{}", backtrace)
    } else {
        String::new()
    }
}

#[inline]
#[must_use]
pub fn fmt_source(source: impl fmt::Display) -> String { format!("\nCaused by: {}", source) }
