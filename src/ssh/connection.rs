//! SSH connection handling using russh

use anyhow::{anyhow, Result};
use russh::client::{self, Handle};
use russh::keys::{PublicKey, PublicKeyBase64};
use russh::{Channel, ChannelId, Disconnect};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{ConnectionConfig, Credentials};
use crate::storage::Database;

/// Host key information for verification
///
/// Implemented per IDEA.md's host-key-verification/TOFU requirement but not
/// yet wired into the connect flow — see TODO.AI.md Phase 1.1.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HostKeyInfo {
    pub host: String,
    pub port: u16,
    pub key_type: String,
    pub fingerprint: String,
    pub key_data: Vec<u8>,
}

impl HostKeyInfo {
    #[allow(dead_code)]
    pub fn from_public_key(host: &str, port: u16, key: &PublicKey) -> Self {
        let fingerprint = key.fingerprint(Default::default()).to_string();
        let key_type = key.algorithm().to_string();

        Self {
            host: host.to_string(),
            port,
            key_type,
            fingerprint,
            key_data: key.public_key_base64().into_bytes(),
        }
    }
}

/// Verify host key against known hosts in database
///
/// Implemented per IDEA.md's host-key-verification/TOFU requirement but not
/// yet wired into the connect flow — see TODO.AI.md Phase 1.1.
#[allow(dead_code)]
pub async fn verify_host_key(
    host: &str,
    port: u16,
    key: &PublicKey,
    database: Option<&Database>,
) -> Result<bool> {
    let key_info = HostKeyInfo::from_public_key(host, port, key);

    // If no database, accept (for testing/initial connection)
    let db = match database {
        Some(d) => d,
        None => {
            log::warn!("No database available for host key verification");
            return Ok(true);
        }
    };

    // Check if host is known
    match db.get_known_host(&key_info.host, key_info.port)? {
        Some(known_key) => {
            // Host is known, verify fingerprint matches
            if known_key.fingerprint == key_info.fingerprint {
                log::info!("Host key verified for {}:{}", host, port);
                // Update last_seen timestamp
                db.update_known_host_last_seen(&key_info.host, key_info.port)?;
                Ok(true)
            } else {
                // MITM ATTACK DETECTED!
                log::error!(
                    "⚠️  HOST KEY MISMATCH for {}:{} - Possible MITM attack!",
                    host,
                    port
                );
                log::error!("Expected: {}", known_key.fingerprint);
                log::error!("Got:      {}", key_info.fingerprint);
                Err(anyhow!(
                    "Host key verification failed! Expected {}, got {}",
                    known_key.fingerprint,
                    key_info.fingerprint
                ))
            }
        }
        None => {
            // First time seeing this host - should prompt user
            log::info!(
                "New host {}:{} with fingerprint: {}",
                host,
                port,
                key_info.fingerprint
            );
            // For now, auto-accept and store (in production, should show dialog)
            db.add_known_host(
                &key_info.host,
                key_info.port,
                &key_info.key_type,
                &key_info.fingerprint,
                &key_info.key_data,
            )?;
            log::info!("Added new host key to known_hosts");
            Ok(true)
        }
    }
}

/// SSH client handler for russh callbacks
pub struct SshClientHandler {
    host: String,
    server_public_key: Option<PublicKey>,
}

impl SshClientHandler {
    pub fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            server_public_key: None,
        }
    }
}

impl client::Handler for SshClientHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &PublicKey,
    ) -> Result<bool, Self::Error> {
        log::info!(
            "Server key for {}: {}",
            self.host,
            server_public_key.fingerprint(Default::default())
        );
        self.server_public_key = Some(server_public_key.clone());
        Ok(true)
    }
}

/// Active SSH connection
pub struct SshConnection {
    handle: Handle<SshClientHandler>,
    config: ConnectionConfig,
    channels: Arc<Mutex<Vec<ChannelId>>>,
}

impl SshConnection {
    /// Connect to an SSH server with password authentication
    pub async fn connect_password(config: ConnectionConfig, password: &str) -> Result<Self> {
        let ssh_config = client::Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(config.keepalive as u64)),
            ..Default::default()
        };

        let addr = format!("{}:{}", config.host, config.port);
        log::info!("Connecting to {}", addr);

        let handler = SshClientHandler::new(&config.host);
        let mut handle = tokio::time::timeout(
            std::time::Duration::from_secs(config.timeout as u64),
            client::connect(Arc::new(ssh_config), &addr, handler),
        )
        .await
        .map_err(|_| anyhow!("Connection to {} timed out after {}s", addr, config.timeout))??;

        log::info!("Connected, authenticating as {}", config.username);

        let auth_result = handle
            .authenticate_password(&config.username, password)
            .await?;

        if !auth_result.success() {
            return Err(anyhow!("Authentication failed"));
        }

        log::info!("Authentication successful");

        Ok(Self {
            handle,
            config,
            channels: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Connect to an SSH server with public key authentication
    pub async fn connect_key(
        config: ConnectionConfig,
        key_path: &str,
        passphrase: Option<&str>,
    ) -> Result<Self> {
        let ssh_config = client::Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(config.keepalive as u64)),
            ..Default::default()
        };

        let addr = format!("{}:{}", config.host, config.port);
        log::info!("Connecting to {}", addr);

        let handler = SshClientHandler::new(&config.host);
        let mut handle = tokio::time::timeout(
            std::time::Duration::from_secs(config.timeout as u64),
            client::connect(Arc::new(ssh_config), &addr, handler),
        )
        .await
        .map_err(|_| anyhow!("Connection to {} timed out after {}s", addr, config.timeout))??;

        log::info!("Connected, authenticating with key as {}", config.username);

        let key_data = tokio::fs::read_to_string(key_path).await?;
        let key_pair = if let Some(pass) = passphrase {
            russh::keys::decode_secret_key(&key_data, Some(pass))?
        } else {
            russh::keys::decode_secret_key(&key_data, None)?
        };

        let auth_result = handle
            .authenticate_publickey(
                &config.username,
                russh::keys::PrivateKeyWithHashAlg::new(Arc::new(key_pair), None),
            )
            .await?;

        if !auth_result.success() {
            return Err(anyhow!("Public key authentication failed"));
        }

        log::info!("Authentication successful");

        Ok(Self {
            handle,
            config,
            channels: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Open a shell channel
    pub async fn open_shell(&self) -> Result<Channel<client::Msg>> {
        let channel = self.handle.channel_open_session().await?;
        let channel_id = channel.id();

        self.channels.lock().await.push(channel_id);

        Ok(channel)
    }

    /// Request a PTY on the channel
    pub async fn request_pty(
        channel: &Channel<client::Msg>,
        term: &str,
        cols: u32,
        rows: u32,
    ) -> Result<()> {
        channel
            .request_pty(false, term, cols, rows, 0, 0, &[])
            .await?;
        Ok(())
    }

    /// Request a shell on the channel
    pub async fn request_shell(channel: &Channel<client::Msg>) -> Result<()> {
        channel.request_shell(false).await?;
        Ok(())
    }

    /// Resize the PTY
    pub async fn resize_pty(channel: &Channel<client::Msg>, cols: u32, rows: u32) -> Result<()> {
        channel.window_change(cols, rows, 0, 0).await?;
        Ok(())
    }

    /// Send data to the channel
    pub async fn send_data(channel: &Channel<client::Msg>, data: &[u8]) -> Result<()> {
        channel.data(data).await?;
        Ok(())
    }

    /// Close the connection
    pub async fn close(&self) -> Result<()> {
        self.handle
            .disconnect(Disconnect::ByApplication, "User disconnected", "en")
            .await?;
        Ok(())
    }

    /// Get the connection configuration
    pub fn config(&self) -> &ConnectionConfig {
        &self.config
    }
}

/// Jump host support for ProxyJump
///
/// Implemented per IDEA.md's jump-host requirement but not yet wired into
/// the connect flow — see TODO.AI.md Phase 1.4. The argument count mirrors
/// distinct jump-host vs. target-host connection parameters (host/port/user/
/// creds for each side) and is left unrefactored until the call site lands.
#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub async fn connect_through_jump_host(
    jump_host: &str,
    jump_port: u16,
    jump_user: &str,
    jump_creds: &Credentials,
    target_host: &str,
    target_port: u16,
    _target_user: &str,
    _target_creds: &Credentials,
) -> Result<SshConnection> {
    // Connect to jump host first
    let jump_config = ConnectionConfig {
        host: jump_host.to_string(),
        port: jump_port,
        username: jump_user.to_string(),
        auth_type: super::AuthType::Password, // Will be set based on credentials
        timeout: 30,
        keepalive: 60,
        compression: false,
    };

    let jump_conn = match jump_creds {
        Credentials::Password { password } => {
            SshConnection::connect_password(jump_config, password).await?
        }
        Credentials::PublicKey {
            key_path,
            passphrase,
        } => {
            SshConnection::connect_key(
                jump_config,
                &key_path.to_string_lossy(),
                passphrase.as_deref(),
            )
            .await?
        }
        _ => return Err(anyhow!("Unsupported credential type for jump host")),
    };

    // Open direct-tcpip channel through jump host to target
    let _channel = jump_conn
        .handle
        .channel_open_direct_tcpip(target_host, target_port as u32, "127.0.0.1", 0)
        .await?;

    log::info!(
        "Established tunnel through jump host to {}:{}",
        target_host,
        target_port
    );

    // Now connect to target through the tunnel
    // This would require wrapping the channel as a transport
    // For now, return jump connection as placeholder
    Ok(jump_conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use russh::client::Handler;

    // Public (non-secret) OpenSSH-formatted ed25519 key, used only to build
    // a `PublicKey` value for the pure-logic tests below. No network I/O.
    const TEST_ED25519_PUBLIC_KEY: &str =
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIF2lLaSatTF5r1oq5a0YNdoIVamIzwmYnQqt7GV9mQ92 test";

    fn test_public_key() -> PublicKey {
        PublicKey::from_openssh(TEST_ED25519_PUBLIC_KEY).unwrap()
    }

    #[test]
    fn test_host_key_info_from_public_key() {
        let key = test_public_key();
        let info = HostKeyInfo::from_public_key("example.com", 2222, &key);
        assert_eq!(info.host, "example.com");
        assert_eq!(info.port, 2222);
        assert_eq!(info.key_type, "ssh-ed25519");
        assert!(!info.fingerprint.is_empty());
        assert!(!info.key_data.is_empty());
    }

    #[test]
    fn test_host_key_info_empty_host_and_zero_port() {
        let key = test_public_key();
        let info = HostKeyInfo::from_public_key("", 0, &key);
        assert_eq!(info.host, "");
        assert_eq!(info.port, 0);
        // Fingerprint/type derivation must not depend on host/port.
        assert_eq!(info.key_type, "ssh-ed25519");
        assert!(!info.fingerprint.is_empty());
    }

    #[test]
    fn test_host_key_info_unicode_host() {
        let key = test_public_key();
        let info = HostKeyInfo::from_public_key("例え.jp", 22, &key);
        assert_eq!(info.host, "例え.jp");
        assert_eq!(info.port, 22);
    }

    #[test]
    fn test_host_key_info_max_port() {
        let key = test_public_key();
        let info = HostKeyInfo::from_public_key("host.example.com", u16::MAX, &key);
        assert_eq!(info.port, u16::MAX);
    }

    #[test]
    fn test_host_key_info_fingerprint_deterministic_for_same_key() {
        let key = test_public_key();
        let info_a = HostKeyInfo::from_public_key("a.example.com", 22, &key);
        let info_b = HostKeyInfo::from_public_key("b.example.com", 2222, &key);
        // Same underlying key must yield the same fingerprint/key_data
        // regardless of host/port, since those aren't part of the key.
        assert_eq!(info_a.fingerprint, info_b.fingerprint);
        assert_eq!(info_a.key_data, info_b.key_data);
    }

    #[tokio::test]
    async fn test_verify_host_key_no_database_accepts() {
        let key = test_public_key();
        let result = verify_host_key("example.com", 22, &key, None).await;
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_verify_host_key_no_database_accepts_empty_host_and_zero_port() {
        let key = test_public_key();
        let result = verify_host_key("", 0, &key, None).await;
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_verify_host_key_new_host_is_auto_added() {
        let db = crate::storage::Database::open_in_memory().unwrap();
        let key = test_public_key();

        assert!(db.get_known_host("new.example.com", 22).unwrap().is_none());

        let result = verify_host_key("new.example.com", 22, &key, Some(&db)).await;
        assert!(result.unwrap());

        let stored = db.get_known_host("new.example.com", 22).unwrap().unwrap();
        let expected = HostKeyInfo::from_public_key("new.example.com", 22, &key);
        assert_eq!(stored.fingerprint, expected.fingerprint);
        assert_eq!(stored.key_type, "ssh-ed25519");
    }

    #[tokio::test]
    async fn test_verify_host_key_matching_fingerprint_accepts_and_updates_last_seen() {
        let db = crate::storage::Database::open_in_memory().unwrap();
        let key = test_public_key();

        // Seed the database as a previously-trusted host.
        verify_host_key("known.example.com", 22, &key, Some(&db))
            .await
            .unwrap();
        let before = db.get_known_host("known.example.com", 22).unwrap().unwrap();

        // Re-verifying the same key against the same host must still accept.
        let result = verify_host_key("known.example.com", 22, &key, Some(&db)).await;
        assert!(result.unwrap());

        let after = db.get_known_host("known.example.com", 22).unwrap().unwrap();
        assert_eq!(after.fingerprint, before.fingerprint);
    }

    #[tokio::test]
    async fn test_verify_host_key_mismatched_fingerprint_is_rejected() {
        let db = crate::storage::Database::open_in_memory().unwrap();
        let trusted_key = test_public_key();

        // Seed the database with the trusted key for this host.
        verify_host_key("mitm.example.com", 22, &trusted_key, Some(&db))
            .await
            .unwrap();

        // A different key presented for the same host must be rejected as a
        // possible MITM attack rather than silently accepted or re-stored.
        const OTHER_ED25519_PUBLIC_KEY: &str =
            "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIDKPz3gx6bgwvZ3HEHcrBAAdVj1CbVZUb540ldEUrsw1 test2";
        let other_key = PublicKey::from_openssh(OTHER_ED25519_PUBLIC_KEY).unwrap();

        let result = verify_host_key("mitm.example.com", 22, &other_key, Some(&db)).await;
        let err = match result {
            Err(e) => e,
            Ok(_) => panic!("expected mismatched host key to be rejected"),
        };
        assert!(err.to_string().contains("Host key verification failed"));
    }

    #[test]
    fn test_ssh_client_handler_new() {
        let handler = SshClientHandler::new("example.com");
        assert_eq!(handler.host, "example.com");
        assert!(handler.server_public_key.is_none());
    }

    #[test]
    fn test_ssh_client_handler_new_empty_host() {
        let handler = SshClientHandler::new("");
        assert_eq!(handler.host, "");
    }

    #[tokio::test]
    async fn test_check_server_key_stores_key_and_accepts() {
        let mut handler = SshClientHandler::new("example.com");
        assert!(handler.server_public_key.is_none());

        let key = test_public_key();
        let accepted = handler.check_server_key(&key).await.unwrap();

        assert!(accepted);
        assert!(handler.server_public_key.is_some());
    }

    #[tokio::test]
    async fn test_connect_through_jump_host_rejects_unsupported_agent_credentials() {
        // Credentials::Agent isn't handled by connect_through_jump_host, so
        // it must be rejected before any network connection is attempted.
        let result = connect_through_jump_host(
            "jump.example.com",
            22,
            "jumpuser",
            &Credentials::Agent,
            "target.example.com",
            2222,
            "targetuser",
            &Credentials::Agent,
        )
        .await;

        let err = match result {
            Err(e) => e,
            Ok(_) => panic!("expected connect_through_jump_host to return an error"),
        };
        assert_eq!(err.to_string(), "Unsupported credential type for jump host");
    }

    #[tokio::test]
    async fn test_connect_through_jump_host_rejects_keyboard_interactive_credentials() {
        let result = connect_through_jump_host(
            "jump.example.com",
            22,
            "jumpuser",
            &Credentials::KeyboardInteractive,
            "target.example.com",
            2222,
            "targetuser",
            &Credentials::Agent,
        )
        .await;

        assert!(result.is_err());
    }
}
