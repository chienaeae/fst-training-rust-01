use clap::{Parser, Subcommand};

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
                println!("Make FST Network great!");
                println!("Сделайте FST Network отличным!");
                Ok(())
            }
        }
    }
}
