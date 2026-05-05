use anyhow::Result;

#[cfg(target_os = "macos")]
pub fn init() -> Result<()> {
    use apple_native_keyring_store::keychain::Store;
    keyring_core::set_default_store(Store::new()?);
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn init() -> Result<()> {
    use windows_native_keyring_store::Store;
    keyring_core::set_default_store(Store::new()?);
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn init() -> Result<()> {
    use zbus_secret_service_keyring_store::Store;
    keyring_core::set_default_store(Store::new()?);
    Ok(())
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub fn init() -> Result<()> {
    anyhow::bail!("unsupported platform")
}
