use clap::{Parser, Subcommand};
use snafu::ResultExt;
use tokio::runtime::Runtime;

use crate::error::Result;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Runs mochi")]
    Run,
}

impl Default for Cli {
    #[inline]
    fn default() -> Self { Self::parse() }
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.commands {
            Commands::Run => {
                Runtime::new().context(mochi::error::InitializeTokioRuntimeSnafu)?.block_on(
                    async move {
                        println!("Make FST Network great!");
                        println!("Сделайте FST Network отличным!");
                    },
                );
                Ok(())
            }
        }
    }
}
