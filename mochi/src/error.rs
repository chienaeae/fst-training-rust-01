use snafu::{Backtrace, Snafu};
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display(
        "Can not initialize Tokio runtime{}",
        fmt_bracktrace_with_source(backtrace, source)
    ))]
    InitializeTokioRuntime { source: tokio::io::Error, backtrace: Backtrace },
}

#[inline]
#[must_use]
pub fn fmt_bracktrace(backtrace: &Backtrace) -> String {
    if cfg!(feature = "backtrace") {
        format!("\n{}", backtrace)
    } else {
        String::new()
    }
}

#[inline]
#[must_use]
pub fn fmt_source(source: impl fmt::Display) -> String { format!("\nCaused by: {}", source) }

#[inline]
#[must_use]
pub fn fmt_bracktrace_with_source(backtrace: &Backtrace, source: impl fmt::Display) -> String {
    format!("{}{}", fmt_bracktrace(backtrace), fmt_source(source))
}
