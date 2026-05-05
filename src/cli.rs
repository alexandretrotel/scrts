use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "scrts",
    about = "A minimal secrets manager using native OS secret storage."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    List,
    Add {
        #[arg(long)]
        name: Option<String>,
    },
    Delete,
}
