pub mod cli;
pub mod commands;
pub mod index;
pub mod platform;
pub mod store;

use anyhow::Result;
use cli::{Cli, Commands};

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::List => commands::list::run(),
        Commands::Add { name } => commands::add::run(name),
        Commands::Delete => commands::delete::run(),
    }
}
