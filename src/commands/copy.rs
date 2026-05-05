use crate::{registry::Registry, store};
use anyhow::Result;
use arboard::Clipboard;
use inquire::{InquireError, Select};

pub fn run() -> Result<()> {
    let registry = Registry::load()?;
    let names = registry.names().to_vec();

    if names.is_empty() {
        println!("No secrets stored.");
        return Ok(());
    }

    let name = match Select::new("Select secret to copy:", names).prompt() {
        Ok(n) => n,
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => return Ok(()),
        Err(e) => return Err(e.into()),
    };

    let secret = store::get(&name)?;
    Clipboard::new()?.set_text(secret)?;

    println!("Copied \"{name}\" to clipboard.");
    Ok(())
}
