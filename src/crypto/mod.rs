//! Crypto module - encryption and key management
//!
//! Provides secure storage for credentials and SSH keys.

#![allow(dead_code)]

use anyhow::Result;

/// Key types supported for SSH
#[derive(Debug, Clone, PartialEq)]
pub enum KeyType {
    Rsa,
    Ed25519,
    Ecdsa,
}

impl std::fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyType::Rsa => write!(f, "RSA"),
            KeyType::Ed25519 => write!(f, "ED25519"),
            KeyType::Ecdsa => write!(f, "ECDSA"),
        }
    }
}

/// Encrypts sensitive data using platform-specific secure storage
pub struct SecureStorage;

impl SecureStorage {
    /// Store a secret securely
    pub fn store(service: &str, key: &str, secret: &str) -> Result<()> {
        let entry = keyring::Entry::new(service, key)?;
        entry.set_password(secret)?;
        Ok(())
    }

    /// Retrieve a secret
    pub fn retrieve(service: &str, key: &str) -> Result<String> {
        let entry = keyring::Entry::new(service, key)?;
        let password = entry.get_password()?;
        Ok(password)
    }

    /// Delete a secret
    pub fn delete(service: &str, key: &str) -> Result<()> {
        let entry = keyring::Entry::new(service, key)?;
        entry.delete_credential()?;
        Ok(())
    }
}
