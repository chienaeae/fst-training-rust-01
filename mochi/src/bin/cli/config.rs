use std::net::{IpAddr, SocketAddr};

use clap::Args;

use mochi::env;

#[derive(Clone, Debug, Args)]
pub struct Config {
    #[clap(flatten)]
    pub api: ApiConfig,

    #[clap(flatten)]
    pub postgres: PostgresConfig,
}

#[derive(Clone, Debug, Args)]
pub struct ApiConfig {
    #[clap(
    name = "server address",
    long = "api-address",
    env = env::API_ADDRESS,
    default_value = "127.0.0.1"
  )]
    address: IpAddr,

    #[clap(
    name = "server port",
    long = "api-port",
    env = env::API_PORT,
    default_value = "3000"
  )]
    port: u16,
}

impl ApiConfig {
    #[inline]
    pub fn socket_address(&self) -> SocketAddr { SocketAddr::new(self.address, self.port) }
}

#[derive(Clone, Debug, Args)]
pub struct PostgresConfig {
    #[clap(
    name = "postgres host name",
    long = "pg-host",
    env = env::POSTGRES_HOST,
  )]
    pub host: String,

    #[clap(
    name = "postgres port",
    long = "pg-port",
    env = env::POSTGRES_PORT,
  )]
    pub port: u16,

    #[clap(
    name = "postgres user name",
    long = "pg-user",
    env = env::POSTGRES_USER,
  )]
    pub user: String,

    #[clap(
    name = "postgres password",
    long = "pg-password",
    env = env::POSTGRES_PASSWORD,
  )]
    pub password: String,

    #[clap(
    name = "postgres database name",
    long = "pg-database",
    env = env::POSTGRES_DATABASE,
  )]
    pub database: String,
}
