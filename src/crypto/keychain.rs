//! OS keychain integration

use anyhow::Result;

pub struct KeychainManager;

impl KeychainManager {
    pub fn new() -> Self {
        Self
    }

    #[cfg(target_os = "macos")]
    pub fn store_password(&self, service: &str, account: &str, password: &str) -> Result<()> {
        use security_framework::passwords::*;
        set_generic_password(service, account, password.as_bytes())?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    pub fn get_password(&self, service: &str, account: &str) -> Result<String> {
        use security_framework::passwords::*;
        let (password, _) = find_generic_password(service, account)?;
        Ok(String::from_utf8(password.to_vec())?)
    }

    #[cfg(target_os = "macos")]
    pub fn delete_password(&self, service: &str, account: &str) -> Result<()> {
        use security_framework::passwords::*;
        delete_generic_password(service, account)?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn store_password(&self, service: &str, account: &str, password: &str) -> Result<()> {
        use keyring::Entry;
        let entry = Entry::new(service, account)?;
        entry.set_password(password)?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn get_password(&self, service: &str, account: &str) -> Result<String> {
        use keyring::Entry;
        let entry = Entry::new(service, account)?;
        Ok(entry.get_password()?)
    }

    #[cfg(target_os = "linux")]
    pub fn delete_password(&self, service: &str, account: &str) -> Result<()> {
        use keyring::Entry;
        let entry = Entry::new(service, account)?;
        entry.delete_password()?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn store_password(&self, service: &str, account: &str, password: &str) -> Result<()> {
        use keyring::Entry;
        let entry = Entry::new(service, account)?;
        entry.set_password(password)?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn get_password(&self, service: &str, account: &str) -> Result<String> {
        use keyring::Entry;
        let entry = Entry::new(service, account)?;
        Ok(entry.get_password()?)
    }

    #[cfg(target_os = "windows")]
    pub fn delete_password(&self, service: &str, account: &str) -> Result<()> {
        use keyring::Entry;
        let entry = Entry::new(service, account)?;
        entry.delete_credential()?;
        Ok(())
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    pub fn store_password(&self, service: &str, account: &str, password: &str) -> Result<()> {
        use keyring::Entry;
        let entry = Entry::new(service, account)?;
        entry.set_password(password)?;
        Ok(())
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    pub fn get_password(&self, service: &str, account: &str) -> Result<String> {
        use keyring::Entry;
        let entry = Entry::new(service, account)?;
        Ok(entry.get_password()?)
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    pub fn delete_password(&self, service: &str, account: &str) -> Result<()> {
        use keyring::Entry;
        let entry = Entry::new(service, account)?;
        entry.delete_credential()?;
        Ok(())
    }
}

impl Default for KeychainManager {
    fn default() -> Self {
        Self::new()
    }
}

// `KeychainManager` is a zero-sized marker type; every method
// (`store_password`/`get_password`/`delete_password`) is a thin,
// platform-gated wrapper that talks directly to a real OS keychain
// (macOS Keychain via `security-framework`, or `keyring`'s backend on
// Linux/Windows/other). There is no pure logic in those methods to unit
// test without a real OS keychain/D-Bus secret service present, and
// exercising them here would make the suite flaky/non-hermetic in CI
// containers that lack one. Only the trivial, environment-independent
// constructors are covered below.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_constructs() {
        let _manager = KeychainManager::new();
    }

    #[test]
    fn test_default_constructs() {
        let _manager = KeychainManager;
    }
}
