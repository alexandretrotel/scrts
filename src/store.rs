use anyhow::Result;
use keyring_core::Entry;

const SERVICE: &str = "scrts";

pub fn get(name: &str) -> Result<String> {
    Ok(Entry::new(SERVICE, name)?.get_password()?)
}

pub fn set(name: &str, secret: &str) -> Result<()> {
    Entry::new(SERVICE, name)?.set_password(secret)?;
    Ok(())
}

pub fn delete(name: &str) -> Result<()> {
    Entry::new(SERVICE, name)?.delete_credential()?;
    Ok(())
}
