use clap::Parser;
use scrts::cli::Cli;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    scrts::platform::init()?;
    let cli = Cli::parse();
    let result = scrts::run(cli);
    keyring_core::unset_default_store();
    result
}
