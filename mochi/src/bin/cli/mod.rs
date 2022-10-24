mod config;

use clap::{Parser, Subcommand};

use snafu::ResultExt;
use sqlx::migrate::Migrator;
use tokio::runtime::Runtime;

use mochi::{web, DefaultContext};

use crate::{error, error::Result};

pub use self::config::Config;

const APP_NAME: &str = "Mochi";
const MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Runs mochi")]
    Run {
        #[clap(flatten)]
        config: Box<Config>,
    },
}

impl Default for Cli {
    #[inline]
    fn default() -> Self { Self::parse() }
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.commands {
            Commands::Run { config } => {
                let Config { api, postgres } = *config;

                Runtime::new().context(mochi::error::InitializeTokioRuntimeSnafu)?.block_on(
                    async move {
                        let _postgres = init_postgres(&postgres).await?;

                        let ctx = DefaultContext::new(_postgres.clone());

                        tokio::spawn({
                            async move {
                                MIGRATOR
                                    .run(&_postgres)
                                    .await
                                    .context(mochi::error::MigrateSchemaSnafu)?;

                                error::Result::Ok(())
                            }
                        });

                        web::new_api_server::<DefaultContext, error::Error>(
                            api.socket_address(),
                            ctx,
                        )?
                        .serve()
                        .await
                    },
                )?;
                Ok(())
            }
        }
    }
}

async fn init_postgres(
    config::PostgresConfig { host, port, user, password, database }: &config::PostgresConfig,
) -> Result<sqlx::Pool<sqlx::Postgres>> {
    let connect_opts = sqlx::postgres::PgConnectOptions::new()
        .host(host)
        .port(*port)
        .username(user)
        .password(password)
        .database(database)
        .application_name(APP_NAME)
        .ssl_mode(sqlx::postgres::PgSslMode::Disable);

    let pool_opts = sqlx::postgres::PgPoolOptions::new().max_connections(5);

    let pool = pool_opts.connect_with(connect_opts).await.with_context(|_| {
        mochi::error::ConnectPostgresSnafu {
            host: host.clone(),
            port: *port,
            user: user.clone(),
            database: database.clone(),
        }
    })?;

    Ok(pool)
}
