//! SSH authentication handling

use anyhow::Result;
use std::path::PathBuf;

/// Credentials for SSH authentication
#[derive(Debug, Clone)]
pub enum Credentials {
    /// Password authentication
    Password { password: String },
    /// Public key authentication
    PublicKey {
        key_path: PathBuf,
        passphrase: Option<String>,
    },
    /// SSH Agent authentication
    Agent,
    /// Keyboard-interactive (will prompt)
    KeyboardInteractive,
}

impl Credentials {
    /// Create password credentials
    pub fn password(password: impl Into<String>) -> Self {
        Self::Password {
            password: password.into(),
        }
    }

    /// Create public key credentials
    pub fn public_key(key_path: impl Into<PathBuf>, passphrase: Option<String>) -> Self {
        Self::PublicKey {
            key_path: key_path.into(),
            passphrase,
        }
    }

    /// Create agent credentials
    pub fn agent() -> Self {
        Self::Agent
    }

    /// Create keyboard-interactive credentials
    pub fn keyboard_interactive() -> Self {
        Self::KeyboardInteractive
    }
}

/// Find default SSH keys in user's .ssh directory
///
/// Implemented per IDEA.md's universal-SSH-key-support requirement but not
/// yet wired into the connect flow — see TODO.AI.md Phase 1.2.
#[allow(dead_code)]
pub fn find_default_keys() -> Vec<PathBuf> {
    let mut keys = Vec::new();

    if let Some(home) = dirs::home_dir() {
        let ssh_dir = home.join(".ssh");

        let key_names = ["id_ed25519", "id_ecdsa", "id_rsa", "id_dsa"];

        for name in key_names {
            let key_path = ssh_dir.join(name);
            if key_path.exists() {
                keys.push(key_path);
            }
        }
    }

    keys
}

/// Read SSH key from file
///
/// Implemented per IDEA.md's universal-SSH-key-support requirement but not
/// yet wired into the connect flow — see TODO.AI.md Phase 1.2.
#[allow(dead_code)]
pub async fn read_key(
    path: &std::path::Path,
    passphrase: Option<&str>,
) -> Result<russh::keys::PrivateKey> {
    let key_data = tokio::fs::read_to_string(path).await?;
    let key = russh::keys::decode_secret_key(&key_data, passphrase)?;
    Ok(key)
}

/// Check if a key file is encrypted
///
/// Implemented per IDEA.md's universal-SSH-key-support requirement but not
/// yet wired into the connect flow — see TODO.AI.md Phase 1.2.
#[allow(dead_code)]
pub fn is_key_encrypted(path: &std::path::Path) -> Result<bool> {
    let key_data = std::fs::read_to_string(path)?;
    Ok(key_data.contains("ENCRYPTED"))
}
