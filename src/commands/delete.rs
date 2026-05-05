use crate::{registry::Registry, store};
use anyhow::Result;
use inquire::{InquireError, MultiSelect};

pub fn run() -> Result<()> {
    let mut registry = Registry::load()?;
    let names = registry.names().to_vec();

    if names.is_empty() {
        println!("No secrets stored.");
        return Ok(());
    }

    let selected = match MultiSelect::new("Select secrets to delete:", names).prompt() {
        Ok(s) => s,
        Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => return Ok(()),
        Err(e) => return Err(e.into()),
    };

    if selected.is_empty() {
        return Ok(());
    }

    for name in &selected {
        store::delete(name)?;
        registry.remove(name);
    }
    registry.save()?;

    println!("Deleted {} secret(s).", selected.len());
    Ok(())
}
