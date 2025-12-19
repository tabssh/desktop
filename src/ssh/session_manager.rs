//! SSH session manager - handles multiple SSH connections

use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use uuid::Uuid;
use anyhow::Result;

use super::connection::SshConnection;
use super::ConnectionConfig;

/// Session state
#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    Connecting,
    Connected,
    Disconnected,
    Error(String),
}

/// Active SSH session
pub struct Session {
    pub id: Uuid,
    pub config: ConnectionConfig,
    pub state: SessionState,
    connection: Option<SshConnection>,
}

impl Session {
    fn new(config: ConnectionConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            config,
            state: SessionState::Disconnected,
            connection: None,
        }
    }
}

/// Manages multiple SSH sessions
pub struct SessionManager {
    runtime: Arc<Runtime>,
    sessions: Arc<Mutex<HashMap<Uuid, Session>>>,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self {
            runtime,
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get the async runtime
    pub fn runtime(&self) -> Arc<Runtime> {
        self.runtime.clone()
    }

    /// Connect with password authentication
    pub async fn connect_password(
        &self,
        config: ConnectionConfig,
        password: &str,
    ) -> Result<Uuid> {
        let mut session = Session::new(config.clone());
        session.state = SessionState::Connecting;

        let session_id = session.id;
        self.sessions.lock().await.insert(session_id, session);

        match SshConnection::connect_password(config.clone(), password).await {
            Ok(conn) => {
                let mut sessions = self.sessions.lock().await;
                if let Some(session) = sessions.get_mut(&session_id) {
                    session.connection = Some(conn);
                    session.state = SessionState::Connected;
                }
                Ok(session_id)
            }
            Err(e) => {
                let mut sessions = self.sessions.lock().await;
                if let Some(session) = sessions.get_mut(&session_id) {
                    session.state = SessionState::Error(e.to_string());
                }
                Err(e)
            }
        }
    }

    /// Connect with public key authentication
    pub async fn connect_key(
        &self,
        config: ConnectionConfig,
        key_path: &str,
        passphrase: Option<&str>,
    ) -> Result<Uuid> {
        let mut session = Session::new(config.clone());
        session.state = SessionState::Connecting;

        let session_id = session.id;
        self.sessions.lock().await.insert(session_id, session);

        match SshConnection::connect_key(config.clone(), key_path, passphrase).await {
            Ok(conn) => {
                let mut sessions = self.sessions.lock().await;
                if let Some(session) = sessions.get_mut(&session_id) {
                    session.connection = Some(conn);
                    session.state = SessionState::Connected;
                }
                Ok(session_id)
            }
            Err(e) => {
                let mut sessions = self.sessions.lock().await;
                if let Some(session) = sessions.get_mut(&session_id) {
                    session.state = SessionState::Error(e.to_string());
                }
                Err(e)
            }
        }
    }

    /// Disconnect a session
    pub async fn disconnect(&self, session_id: Uuid) -> Result<()> {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            if let Some(conn) = session.connection.take() {
                conn.close().await?;
            }
            session.state = SessionState::Disconnected;
        }
        Ok(())
    }

    /// Remove a session
    pub async fn remove_session(&self, session_id: Uuid) {
        let mut sessions = self.sessions.lock().await;
        sessions.remove(&session_id);
    }

    /// Get session state
    pub async fn get_state(&self, session_id: Uuid) -> Option<SessionState> {
        let sessions = self.sessions.lock().await;
        sessions.get(&session_id).map(|s| s.state.clone())
    }

    /// Get all session IDs
    pub async fn list_sessions(&self) -> Vec<Uuid> {
        let sessions = self.sessions.lock().await;
        sessions.keys().copied().collect()
    }

    /// Get session count
    pub async fn session_count(&self) -> usize {
        let sessions = self.sessions.lock().await;
        sessions.len()
    }
}
