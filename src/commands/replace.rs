use crate::{registry::Registry, store};
use anyhow::Result;
use inquire::{InquireError, Password, PasswordDisplayMode, Select};

pub fn run() -> Result<()> {
    let registry = Registry::load()?;
    let names = registry.names().to_vec();

    if names.is_empty() {
        println!("No secrets stored.");
        return Ok(());
    }

    let name = match Select::new("Select secret to update:", names).prompt() {
        Ok(n) => n,
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => return Ok(()),
        Err(e) => return Err(e.into()),
    };

    let secret = match Password::new("New secret:")
        .without_confirmation()
        .with_display_mode(PasswordDisplayMode::Masked)
        .prompt()
    {
        Ok(s) => s,
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => return Ok(()),
        Err(e) => return Err(e.into()),
    };

    store::set(&name, &secret)?;

    println!("\nUpdated \"{name}\".");
    Ok(())
}
