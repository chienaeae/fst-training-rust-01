use std::net::{IpAddr, SocketAddr};

use clap::Args;

use mochi::env;

#[derive(Clone, Debug, Args)]
pub struct Config {
    #[clap(flatten)]
    pub api: ApiConfig,

    #[clap(flatten)]
    pub postgres: PostgresConfig,

    #[clap(flatten)]
    pub core: CoreClientConfig,
}

#[derive(Clone, Debug, Args)]
pub struct ApiConfig {
    #[clap(
    name = "api server address",
    long = "api-address",
    env = env::API_ADDRESS,
    default_value = "127.0.0.1"
  )]
    address: IpAddr,

    #[clap(
    name = "api server port",
    long = "api-port",
    env = env::API_PORT,
    default_value = "3000"
  )]
    port: u16,

    #[clap(
      name = "api authorization secret",
      long = "api-authorization-secret",
      env = env::API_AUTHORIZATION_SECRET,
      help = "PKCS#1 or PKCS#8 RSA public key in PEM format",
    )]
    authorization_secret: String,
}

impl ApiConfig {
    #[inline]
    pub fn socket_address(&self) -> SocketAddr { SocketAddr::new(self.address, self.port) }

    #[inline]
    pub fn authorization_secret(&self) -> &str { &self.authorization_secret }
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

#[derive(Clone, Debug, Args)]
pub struct CoreClientConfig {
    #[clap(
      name = "loc core host",
      long = "loc-core-host",
      env = env::LOC_CORE_HOST,
      default_value = "127.0.0.1"
    )]
    pub host: String,
    #[clap(
      name = "loc core port",
      long = "loc-core-port",
      env = env::LOC_CORE_PORT,
      default_value = "8787"
    )]
    pub port: u16,
    #[clap(
      name = "is use http",
      long = "loc-core-client-use-http",
      env = env::LOC_CORE_CLIENT_USE_HTTP,
    )]
    pub use_http: bool,
    #[clap(
      name = "skip certificate",
      long = "loc-core-client-skip-cert",
      env = env::LOC_CORE_CLIENT_SKIP_CERT,
    )]
    pub skip_cert: bool,
}
