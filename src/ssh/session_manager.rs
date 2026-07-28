//! SSH session manager - handles multiple SSH connections

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use uuid::Uuid;

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
    /// Connection parameters used to establish this session.
    ///
    /// Not yet read anywhere — reserved for a future "connection details"
    /// UI panel; see TODO.AI.md Phase 1.1.
    #[allow(dead_code)]
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
    pub async fn connect_password(&self, config: ConnectionConfig, password: &str) -> Result<Uuid> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ssh::AuthType;

    fn make_manager(runtime: &Arc<Runtime>) -> SessionManager {
        SessionManager::new(runtime.clone())
    }

    fn refused_config() -> ConnectionConfig {
        // Port 1 on localhost has nothing listening, so the TCP connect
        // fails fast with "connection refused" without touching the network.
        ConnectionConfig {
            host: "127.0.0.1".to_string(),
            port: 1,
            username: "nobody".to_string(),
            auth_type: AuthType::Password,
            timeout: 30,
            keepalive: 60,
            compression: false,
        }
    }

    // Plain sync tests driving their own runtime via block_on, rather than
    // #[tokio::test]: SessionManager owns a real Arc<Runtime>, and dropping
    // that nested Runtime from within an outer #[tokio::test] async context
    // panics with "Cannot drop a runtime in a context where blocking is not
    // allowed." See tests/integration/full_workflow_test.rs for the same
    // established pattern.

    #[test]
    fn test_new_manager_has_no_sessions() {
        let runtime = Arc::new(Runtime::new().unwrap());
        let manager = make_manager(&runtime);
        runtime.block_on(async {
            assert_eq!(manager.session_count().await, 0);
            assert!(manager.list_sessions().await.is_empty());
        });
    }

    #[test]
    fn test_runtime_returns_same_arc() {
        let runtime = Arc::new(Runtime::new().unwrap());
        let manager = SessionManager::new(runtime.clone());
        assert!(Arc::ptr_eq(&manager.runtime(), &runtime));
    }

    #[test]
    fn test_connect_password_failure_sets_error_state() {
        let runtime = Arc::new(Runtime::new().unwrap());
        let manager = make_manager(&runtime);
        runtime.block_on(async {
            let result = manager.connect_password(refused_config(), "password").await;

            assert!(result.is_err());
            // Even on failure, a session entry is created so callers can
            // observe the failure state instead of silently losing the id.
            assert_eq!(manager.session_count().await, 1);

            let ids = manager.list_sessions().await;
            assert_eq!(ids.len(), 1);
            let state = manager.get_state(ids[0]).await.unwrap();
            assert!(matches!(state, SessionState::Error(_)));
        });
    }

    #[test]
    fn test_connect_key_failure_sets_error_state() {
        let runtime = Arc::new(Runtime::new().unwrap());
        let manager = make_manager(&runtime);
        runtime.block_on(async {
            let result = manager
                .connect_key(refused_config(), "/nonexistent/key/path", None)
                .await;

            assert!(result.is_err());
            assert_eq!(manager.session_count().await, 1);

            let ids = manager.list_sessions().await;
            let state = manager.get_state(ids[0]).await.unwrap();
            assert!(matches!(state, SessionState::Error(_)));
        });
    }

    #[test]
    fn test_get_state_unknown_id_returns_none() {
        let runtime = Arc::new(Runtime::new().unwrap());
        let manager = make_manager(&runtime);
        runtime.block_on(async {
            assert!(manager.get_state(Uuid::new_v4()).await.is_none());
        });
    }

    #[test]
    fn test_disconnect_unknown_id_is_noop() {
        let runtime = Arc::new(Runtime::new().unwrap());
        let manager = make_manager(&runtime);
        runtime.block_on(async {
            let result = manager.disconnect(Uuid::new_v4()).await;
            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_disconnect_after_failed_connect_sets_disconnected() {
        let runtime = Arc::new(Runtime::new().unwrap());
        let manager = make_manager(&runtime);
        runtime.block_on(async {
            let _ = manager.connect_password(refused_config(), "password").await;
            let ids = manager.list_sessions().await;
            let id = ids[0];

            // No live connection was established, so disconnect just flips
            // the state without needing to close a socket.
            manager.disconnect(id).await.unwrap();
            let state = manager.get_state(id).await.unwrap();
            assert_eq!(state, SessionState::Disconnected);
        });
    }

    #[test]
    fn test_remove_session() {
        let runtime = Arc::new(Runtime::new().unwrap());
        let manager = make_manager(&runtime);
        runtime.block_on(async {
            let _ = manager.connect_password(refused_config(), "password").await;
            assert_eq!(manager.session_count().await, 1);

            let ids = manager.list_sessions().await;
            manager.remove_session(ids[0]).await;
            assert_eq!(manager.session_count().await, 0);
            assert!(manager.get_state(ids[0]).await.is_none());
        });
    }

    #[test]
    fn test_remove_session_unknown_id_is_noop() {
        let runtime = Arc::new(Runtime::new().unwrap());
        let manager = make_manager(&runtime);
        runtime.block_on(async {
            manager.remove_session(Uuid::new_v4()).await;
            assert_eq!(manager.session_count().await, 0);
        });
    }

    #[test]
    fn test_session_state_equality() {
        assert_eq!(SessionState::Connecting, SessionState::Connecting);
        assert_ne!(SessionState::Connecting, SessionState::Connected);
        assert_eq!(
            SessionState::Error("a".to_string()),
            SessionState::Error("a".to_string())
        );
        assert_ne!(
            SessionState::Error("a".to_string()),
            SessionState::Error("b".to_string())
        );
    }
}
