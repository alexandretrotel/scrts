pub mod cli;
pub mod commands;
pub mod platform;
pub mod registry;
pub mod store;

use anyhow::Result;
use cli::{Cli, Commands};

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::List => commands::list::run(),
        Commands::Add { name } => commands::add::run(name),
        Commands::Delete => commands::delete::run(),
        Commands::Copy => commands::copy::run(),
        Commands::Rename => commands::rename::run(),
        Commands::Replace => commands::replace::run(),
    }
}
