use snafu::{Backtrace, Snafu};
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display(
        "Can not initialize Tokio runtime{}",
        fmt_backtrace_with_source(backtrace, source)
    ))]
    InitializeTokioRuntime { source: tokio::io::Error, backtrace: Backtrace },

    #[snafu(display("Axum Serve error{}", fmt_backtrace_with_source(backtrace, source)))]
    AxumServer { source: hyper::Error, backtrace: Backtrace },

    #[snafu(display(
        "Could not connect PostgreSQL with endpoint `postgres://{user}@{host}:{port}/{database}`{}",
        fmt_backtrace_with_source(backtrace, source)
    ))]
    ConnectPostgres {
        host: String,
        port: u16,
        user: String,
        database: String,
        source: sqlx::Error,
        backtrace: Backtrace,
    },
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

#[inline]
#[must_use]
pub fn fmt_backtrace_with_source(backtrace: &Backtrace, source: impl fmt::Display) -> String {
    format!("{}{}", fmt_backtrace(backtrace), fmt_source(source))
}
