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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_password() {
        let creds = Credentials::password("hunter2");
        match creds {
            Credentials::Password { password } => assert_eq!(password, "hunter2"),
            _ => panic!("expected Password variant"),
        }
    }

    #[test]
    fn test_credentials_password_empty() {
        let creds = Credentials::password("");
        match creds {
            Credentials::Password { password } => assert_eq!(password, ""),
            _ => panic!("expected Password variant"),
        }
    }

    #[test]
    fn test_credentials_public_key_with_passphrase() {
        let creds = Credentials::public_key("/home/user/.ssh/id_rsa", Some("pass".to_string()));
        match creds {
            Credentials::PublicKey {
                key_path,
                passphrase,
            } => {
                assert_eq!(key_path, PathBuf::from("/home/user/.ssh/id_rsa"));
                assert_eq!(passphrase, Some("pass".to_string()));
            }
            _ => panic!("expected PublicKey variant"),
        }
    }

    #[test]
    fn test_credentials_public_key_without_passphrase() {
        let creds = Credentials::public_key("/home/user/.ssh/id_ed25519", None);
        match creds {
            Credentials::PublicKey {
                key_path,
                passphrase,
            } => {
                assert_eq!(key_path, PathBuf::from("/home/user/.ssh/id_ed25519"));
                assert!(passphrase.is_none());
            }
            _ => panic!("expected PublicKey variant"),
        }
    }

    #[test]
    fn test_credentials_agent() {
        assert!(matches!(Credentials::agent(), Credentials::Agent));
    }

    #[test]
    fn test_credentials_keyboard_interactive() {
        assert!(matches!(
            Credentials::keyboard_interactive(),
            Credentials::KeyboardInteractive
        ));
    }

    #[test]
    fn test_find_default_keys_returns_only_existing_files() {
        // We don't control what's in the real ~/.ssh directory in CI, but
        // every path returned must actually exist on disk.
        let keys = find_default_keys();
        for key in &keys {
            assert!(key.exists());
        }
    }

    #[test]
    fn test_is_key_encrypted_true() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "tabssh_test_key_encrypted_{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::write(&path, "this fake test fixture contains ENCRYPTED somewhere").unwrap();

        let result = is_key_encrypted(&path).unwrap();
        std::fs::remove_file(&path).ok();

        assert!(result);
    }

    #[test]
    fn test_is_key_encrypted_false() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("tabssh_test_key_plain_{}", uuid::Uuid::new_v4()));
        std::fs::write(&path, "this fake test fixture has no such marker").unwrap();

        let result = is_key_encrypted(&path).unwrap();
        std::fs::remove_file(&path).ok();

        assert!(!result);
    }

    #[test]
    fn test_is_key_encrypted_missing_file_errors() {
        let path = std::env::temp_dir().join("tabssh_test_key_does_not_exist_at_all");
        assert!(is_key_encrypted(&path).is_err());
    }

    #[tokio::test]
    async fn test_read_key_missing_file_errors() {
        let path = std::env::temp_dir().join("tabssh_test_read_key_does_not_exist_at_all");
        let result = read_key(&path, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_read_key_invalid_contents_errors() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("tabssh_test_bad_key_{}", uuid::Uuid::new_v4()));
        std::fs::write(&path, "not a valid key").unwrap();

        let result = read_key(&path, None).await;
        std::fs::remove_file(&path).ok();

        assert!(result.is_err());
    }
}
