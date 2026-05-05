use crate::{registry::Registry, store};
use anyhow::Result;
use inquire::{InquireError, Password, Text};

pub fn run(name_arg: Option<String>) -> Result<()> {
    let name = match name_arg {
        Some(n) => n,
        None => match Text::new("Name:").prompt() {
            Ok(n) => n,
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
                return Ok(());
            }
            Err(e) => return Err(e.into()),
        },
    };

    let secret = match Password::new("Secret:").without_confirmation().prompt() {
        Ok(s) => s,
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => return Ok(()),
        Err(e) => return Err(e.into()),
    };

    store::set(&name, &secret)?;

    let mut registry = Registry::load()?;
    registry.add(name.clone());
    registry.save()?;

    println!("Saved \"{name}\".");
    Ok(())
}
