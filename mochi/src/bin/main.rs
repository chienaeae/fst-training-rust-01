mod cli;
mod error;

use std::process;

use self::cli::Cli;

#[cfg(not(miri))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    if let Err(err) = Cli::default().run() {
        eprintln!("{:?}", err);
        process::exit(-1);
    }
}
