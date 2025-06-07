mod cli;
mod cracker;
mod error;
mod utils;
mod hash;

use clap::Parser;
use cli::Cli;
use cracker::Cracker;
use error::HashbrownResult;
use log::info;

fn main() -> HashbrownResult<()> {
    env_logger::init();
    info!("[i] Starting Hashbrown hash cracker");

    let cli = Cli::parse();
    let mut cracker = Cracker::new(&cli)?;
    cracker.crack()?;

    Ok(())
}
