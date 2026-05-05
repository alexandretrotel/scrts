use crate::registry::Registry;
use anyhow::Result;

pub fn run() -> Result<()> {
    let registry = Registry::load()?;
    let names = registry.names();

    if names.is_empty() {
        println!("No secrets stored.");
        return Ok(());
    }

    for name in names {
        println!("{name}");
    }

    Ok(())
}
