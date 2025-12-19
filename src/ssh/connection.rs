//! SSH connection handling using russh

use anyhow::{anyhow, Result};
use russh::client::{self, Handle};
use russh::keys::key;
use russh::{Channel, ChannelId, Disconnect};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::ConnectionConfig;
use crate::storage::database::Database;

/// Host key information for verification
#[derive(Debug, Clone)]
pub struct HostKeyInfo {
    pub host: String,
    pub port: u16,
    pub key_type: String,
    pub fingerprint: String,
    pub key_data: Vec<u8>,
}

impl HostKeyInfo {
    pub fn from_public_key(host: &str, port: u16, key: &key::PublicKey) -> Self {
        let fingerprint = key.fingerprint();
        let key_type = key.name().to_string();
        
        Self {
            host: host.to_string(),
            port,
            key_type,
            fingerprint,
            key_data: key.public_key_bytes(),
        }
    }
}

/// Verify host key against known hosts in database
pub async fn verify_host_key(
    host: &str,
    port: u16,
    key: &key::PublicKey,
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
                    host, port
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
            log::info!("New host {}:{} with fingerprint: {}", host, port, key_info.fingerprint);
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
    server_public_key: Option<key::PublicKey>,
}

impl SshClientHandler {
    pub fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            server_public_key: None,
        }
    }
}

#[async_trait::async_trait]
impl client::Handler for SshClientHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        log::info!(
            "Server key for {}: {}",
            self.host,
            server_public_key.fingerprint()
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
    pub async fn connect_password(
        config: ConnectionConfig,
        password: &str,
    ) -> Result<Self> {
        let ssh_config = client::Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(config.keepalive as u64)),
            ..Default::default()
        };

        let addr = format!("{}:{}", config.host, config.port);
        log::info!("Connecting to {}", addr);

        let handler = SshClientHandler::new(&config.host);
        let mut handle = client::connect(Arc::new(ssh_config), &addr, handler).await?;

        log::info!("Connected, authenticating as {}", config.username);

        let authenticated = handle
            .authenticate_password(&config.username, password)
            .await?;

        if !authenticated {
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
        let mut handle = client::connect(Arc::new(ssh_config), &addr, handler).await?;

        log::info!("Connected, authenticating with key as {}", config.username);

        let key_data = tokio::fs::read_to_string(key_path).await?;
        let key_pair = if let Some(pass) = passphrase {
            russh_keys::decode_secret_key(&key_data, Some(pass))?
        } else {
            russh_keys::decode_secret_key(&key_data, None)?
        };

        let authenticated = handle
            .authenticate_publickey(&config.username, Arc::new(key_pair))
            .await?;

        if !authenticated {
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
            .request_pty(
                false,
                term,
                cols,
                rows,
                0,
                0,
                &[],
            )
            .await?;
        Ok(())
    }

    /// Request a shell on the channel
    pub async fn request_shell(channel: &Channel<client::Msg>) -> Result<()> {
        channel.request_shell(false).await?;
        Ok(())
    }

    /// Resize the PTY
    pub async fn resize_pty(
        channel: &Channel<client::Msg>,
        cols: u32,
        rows: u32,
    ) -> Result<()> {
        channel.window_change(cols, rows, 0, 0).await?;
        Ok(())
    }

    /// Send data to the channel
    pub async fn send_data(
        channel: &Channel<client::Msg>,
        data: &[u8],
    ) -> Result<()> {
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
