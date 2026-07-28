//! Active SSH session management
//!
//! Bridges async SSH connections with the synchronous UI terminal view.

use anyhow::Result;
use russh::client::{self, Handle};
use russh::keys::PublicKey;
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
        Self {
            host: host.to_string(),
        }
    }
}

impl client::Handler for SessionHandler {
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
            if let Err(e) =
                run_session_password(&host, port, &username, &password, event_tx, command_rx).await
            {
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
            )
            .await
            {
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
    let auth_result = handle.authenticate_password(username, password).await?;

    if !auth_result.success() {
        let _ = event_tx
            .send(SessionEvent::Error("Authentication failed".to_string()))
            .await;
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
    let key_pair = russh::keys::decode_secret_key(&key_data, passphrase)?;

    let auth_result = handle
        .authenticate_publickey(
            username,
            russh::keys::PrivateKeyWithHashAlg::new(Arc::new(key_pair), None),
        )
        .await?;

    if !auth_result.success() {
        let _ = event_tx
            .send(SessionEvent::Error("Key authentication failed".to_string()))
            .await;
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

    channel
        .request_pty(false, "xterm-256color", 80, 24, 0, 0, &[])
        .await?;
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

    let _ = handle
        .disconnect(Disconnect::ByApplication, "Session ended", "en")
        .await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build an ActiveSession backed by channels we control, bypassing the
    /// real SSH connect path so command/event plumbing can be tested
    /// without a live server.
    fn make_session() -> (
        ActiveSession,
        mpsc::Sender<SessionEvent>,
        mpsc::Receiver<SessionCommand>,
    ) {
        let (event_tx, event_rx) = mpsc::channel(16);
        let (command_tx, command_rx) = mpsc::channel(16);
        let session = ActiveSession {
            id: Uuid::new_v4(),
            host: "example.com".to_string(),
            username: "user".to_string(),
            port: 22,
            event_rx,
            command_tx,
        };
        (session, event_tx, command_rx)
    }

    #[test]
    fn test_session_handler_new_stores_host() {
        let handler = SessionHandler::new("example.com");
        assert_eq!(handler.host, "example.com");
    }

    #[test]
    fn test_try_recv_empty_returns_none() {
        let (mut session, _event_tx, _command_rx) = make_session();
        assert!(session.try_recv().is_none());
    }

    #[tokio::test]
    async fn test_try_recv_returns_sent_event() {
        let (mut session, event_tx, _command_rx) = make_session();
        event_tx.send(SessionEvent::Connected).await.unwrap();

        match session.try_recv() {
            Some(SessionEvent::Connected) => {}
            other => panic!("expected Connected event, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_try_recv_preserves_order() {
        let (mut session, event_tx, _command_rx) = make_session();
        event_tx
            .send(SessionEvent::Data(vec![1, 2, 3]))
            .await
            .unwrap();
        event_tx
            .send(SessionEvent::Error("oops".to_string()))
            .await
            .unwrap();

        match session.try_recv() {
            Some(SessionEvent::Data(data)) => assert_eq!(data, vec![1, 2, 3]),
            other => panic!("expected Data event first, got {:?}", other),
        }
        match session.try_recv() {
            Some(SessionEvent::Error(msg)) => assert_eq!(msg, "oops"),
            other => panic!("expected Error event second, got {:?}", other),
        }
        assert!(session.try_recv().is_none());
    }

    #[tokio::test]
    async fn test_send_data_forwards_command() {
        let (session, _event_tx, mut command_rx) = make_session();
        session.send_data(vec![9, 8, 7]);

        match command_rx.recv().await {
            Some(SessionCommand::SendData(data)) => assert_eq!(data, vec![9, 8, 7]),
            other => panic!("expected SendData command, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_send_data_empty_vec() {
        let (session, _event_tx, mut command_rx) = make_session();
        session.send_data(vec![]);

        match command_rx.recv().await {
            Some(SessionCommand::SendData(data)) => assert!(data.is_empty()),
            other => panic!("expected SendData command, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_resize_forwards_command() {
        let (session, _event_tx, mut command_rx) = make_session();
        session.resize(120, 40);

        match command_rx.recv().await {
            Some(SessionCommand::Resize(cols, rows)) => {
                assert_eq!(cols, 120);
                assert_eq!(rows, 40);
            }
            other => panic!("expected Resize command, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_disconnect_forwards_command() {
        let (session, _event_tx, mut command_rx) = make_session();
        session.disconnect();

        match command_rx.recv().await {
            Some(SessionCommand::Disconnect) => {}
            other => panic!("expected Disconnect command, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_send_data_after_receiver_dropped_does_not_panic() {
        let (session, _event_tx, command_rx) = make_session();
        drop(command_rx);
        // try_send on a closed channel must fail silently, not panic.
        session.send_data(vec![1]);
        session.resize(1, 1);
        session.disconnect();
    }
}
