//! Active SSH session management
//!
//! Bridges async SSH connections with the synchronous UI terminal view.

use anyhow::Result;
use russh::client::{self, Handle};
use russh_keys::key;
use russh::{ChannelMsg, Disconnect};
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Messages from SSH session to UI
#[derive(Debug)]
pub enum SessionEvent {
    Connected,
    Data(Vec<u8>),
    Disconnected,
    Error(String),
}

/// Commands from UI to SSH session
#[derive(Debug)]
pub enum SessionCommand {
    SendData(Vec<u8>),
    Resize(u32, u32),
    Disconnect,
}

/// SSH client handler
struct SessionHandler {
    host: String,
}

impl SessionHandler {
    fn new(host: &str) -> Self {
        Self { host: host.to_string() }
    }
}

#[async_trait::async_trait]
impl client::Handler for SessionHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        log::info!("Server key for {}: {}", self.host, server_public_key.fingerprint());
        Ok(true)
    }
}

/// Active SSH session that runs in background
pub struct ActiveSession {
    pub id: Uuid,
    pub host: String,
    pub username: String,
    pub port: u16,
    event_rx: mpsc::Receiver<SessionEvent>,
    command_tx: mpsc::Sender<SessionCommand>,
}

impl ActiveSession {
    /// Connect with password authentication
    pub async fn connect_password(
        host: String,
        port: u16,
        username: String,
        password: String,
    ) -> Result<Self> {
        let id = Uuid::new_v4();
        let (event_tx, event_rx) = mpsc::channel(256);
        let (command_tx, command_rx) = mpsc::channel(256);

        let session_host = host.clone();
        let session_user = username.clone();

        tokio::spawn(async move {
            if let Err(e) = run_session_password(
                &host,
                port,
                &username,
                &password,
                event_tx,
                command_rx,
            ).await {
                log::error!("Session error: {}", e);
            }
        });

        Ok(Self {
            id,
            host: session_host,
            username: session_user,
            port,
            event_rx,
            command_tx,
        })
    }

    /// Connect with key authentication
    pub async fn connect_key(
        host: String,
        port: u16,
        username: String,
        key_path: String,
        passphrase: Option<String>,
    ) -> Result<Self> {
        let id = Uuid::new_v4();
        let (event_tx, event_rx) = mpsc::channel(256);
        let (command_tx, command_rx) = mpsc::channel(256);

        let session_host = host.clone();
        let session_user = username.clone();

        tokio::spawn(async move {
            if let Err(e) = run_session_key(
                &host,
                port,
                &username,
                &key_path,
                passphrase.as_deref(),
                event_tx,
                command_rx,
            ).await {
                log::error!("Session error: {}", e);
            }
        });

        Ok(Self {
            id,
            host: session_host,
            username: session_user,
            port,
            event_rx,
            command_tx,
        })
    }

    /// Try to receive events (non-blocking)
    pub fn try_recv(&mut self) -> Option<SessionEvent> {
        self.event_rx.try_recv().ok()
    }

    /// Send data to the SSH session
    pub fn send_data(&self, data: Vec<u8>) {
        let _ = self.command_tx.try_send(SessionCommand::SendData(data));
    }

    /// Send resize command
    pub fn resize(&self, cols: u32, rows: u32) {
        let _ = self.command_tx.try_send(SessionCommand::Resize(cols, rows));
    }

    /// Disconnect the session
    pub fn disconnect(&self) {
        let _ = self.command_tx.try_send(SessionCommand::Disconnect);
    }
}

async fn run_session_password(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    event_tx: mpsc::Sender<SessionEvent>,
    command_rx: mpsc::Receiver<SessionCommand>,
) -> Result<()> {
    let config = client::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(300)),
        ..Default::default()
    };

    let addr = format!("{}:{}", host, port);
    log::info!("Connecting to {}", addr);

    let handler = SessionHandler::new(host);
    let mut handle = client::connect(Arc::new(config), &addr, handler).await?;

    log::info!("Authenticating as {}", username);
    let authenticated = handle.authenticate_password(username, password).await?;

    if !authenticated {
        let _ = event_tx.send(SessionEvent::Error("Authentication failed".to_string())).await;
        return Err(anyhow::anyhow!("Authentication failed"));
    }

    run_shell_session(handle, event_tx, command_rx).await
}

async fn run_session_key(
    host: &str,
    port: u16,
    username: &str,
    key_path: &str,
    passphrase: Option<&str>,
    event_tx: mpsc::Sender<SessionEvent>,
    command_rx: mpsc::Receiver<SessionCommand>,
) -> Result<()> {
    let config = client::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(300)),
        ..Default::default()
    };

    let addr = format!("{}:{}", host, port);
    log::info!("Connecting to {}", addr);

    let handler = SessionHandler::new(host);
    let mut handle = client::connect(Arc::new(config), &addr, handler).await?;

    log::info!("Authenticating with key as {}", username);
    let key_data = tokio::fs::read_to_string(key_path).await?;
    let key_pair = russh_keys::decode_secret_key(&key_data, passphrase)?;

    let authenticated = handle.authenticate_publickey(username, Arc::new(key_pair)).await?;

    if !authenticated {
        let _ = event_tx.send(SessionEvent::Error("Key authentication failed".to_string())).await;
        return Err(anyhow::anyhow!("Key authentication failed"));
    }

    run_shell_session(handle, event_tx, command_rx).await
}

async fn run_shell_session(
    handle: Handle<SessionHandler>,
    event_tx: mpsc::Sender<SessionEvent>,
    mut command_rx: mpsc::Receiver<SessionCommand>,
) -> Result<()> {
    log::info!("Opening shell channel");
    let mut channel = handle.channel_open_session().await?;

    channel.request_pty(false, "xterm-256color", 80, 24, 0, 0, &[]).await?;
    channel.request_shell(false).await?;

    let _ = event_tx.send(SessionEvent::Connected).await;
    log::info!("Shell session started");

    loop {
        tokio::select! {
            msg = channel.wait() => {
                match msg {
                    Some(ChannelMsg::Data { data }) => {
                        if event_tx.send(SessionEvent::Data(data.to_vec())).await.is_err() {
                            break;
                        }
                    }
                    Some(ChannelMsg::Eof) | Some(ChannelMsg::Close) | None => {
                        log::info!("Channel closed");
                        let _ = event_tx.send(SessionEvent::Disconnected).await;
                        break;
                    }
                    Some(ChannelMsg::ExitStatus { exit_status }) => {
                        log::info!("Exit status: {}", exit_status);
                    }
                    _ => {}
                }
            }
            cmd = command_rx.recv() => {
                match cmd {
                    Some(SessionCommand::SendData(data)) => {
                        if let Err(e) = channel.data(&data[..]).await {
                            log::error!("Failed to send data: {}", e);
                        }
                    }
                    Some(SessionCommand::Resize(cols, rows)) => {
                        if let Err(e) = channel.window_change(cols, rows, 0, 0).await {
                            log::warn!("Failed to resize: {}", e);
                        }
                    }
                    Some(SessionCommand::Disconnect) | None => {
                        log::info!("Disconnect requested");
                        break;
                    }
                }
            }
        }
    }

    let _ = handle.disconnect(Disconnect::ByApplication, "Session ended", "en").await;
    Ok(())
}
