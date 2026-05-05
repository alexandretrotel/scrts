use crate::{registry::Registry, store};
use anyhow::Result;
use inquire::{InquireError, Select, Text};

pub fn run() -> Result<()> {
    let mut registry = Registry::load()?;
    let names = registry.names().to_vec();

    if names.is_empty() {
        println!("No secrets stored.");
        return Ok(());
    }

    let old_name = match Select::new("Select secret to rename:", names).prompt() {
        Ok(n) => n,
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => return Ok(()),
        Err(e) => return Err(e.into()),
    };

    let new_name = match Text::new("New name:").prompt() {
        Ok(n) => n,
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => return Ok(()),
        Err(e) => return Err(e.into()),
    };

    if new_name == old_name {
        return Ok(());
    }

    let secret = store::get(&old_name)?;
    store::set(&new_name, &secret)?;
    store::delete(&old_name)?;

    registry.remove(&old_name);
    registry.add(new_name.clone());
    registry.save()?;

    println!("\nRenamed \"{old_name}\" to \"{new_name}\".");
    Ok(())
}
