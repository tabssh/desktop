//! SSH module - SSH connection and session management

mod active_session;
mod auth;
mod config_parser;
mod connection;
mod forwarding;
mod session_manager;

pub use active_session::{ActiveSession, SessionEvent};
pub use auth::Credentials;
pub use config_parser::{HostConfig, SshConfigParser};
pub use connection::SshConnection;
pub use forwarding::{ForwardType, ForwardingManager, PortForward};
pub use session_manager::SessionManager;

/// SSH authentication type
#[derive(Debug, Clone, PartialEq)]
pub enum AuthType {
    Password,
    PublicKey,
    KeyboardInteractive,
}

/// SSH connection configuration
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth_type: AuthType,
    pub timeout: u32,
    pub keepalive: u32,
    pub compression: bool,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 22,
            username: String::new(),
            auth_type: AuthType::Password,
            timeout: 30,
            keepalive: 60,
            compression: false,
        }
    }
}

impl ConnectionConfig {
    pub fn new(host: impl Into<String>, username: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            username: username.into(),
            ..Default::default()
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_auth(mut self, auth_type: AuthType) -> Self {
        self.auth_type = auth_type;
        self
    }

    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_keepalive(mut self, keepalive: u32) -> Self {
        self.keepalive = keepalive;
        self
    }

    pub fn with_compression(mut self, compression: bool) -> Self {
        self.compression = compression;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_type_equality() {
        assert_eq!(AuthType::Password, AuthType::Password);
        assert_eq!(AuthType::PublicKey, AuthType::PublicKey);
        assert_eq!(AuthType::KeyboardInteractive, AuthType::KeyboardInteractive);
        assert_ne!(AuthType::Password, AuthType::PublicKey);
        assert_ne!(AuthType::PublicKey, AuthType::KeyboardInteractive);
    }

    #[test]
    fn test_connection_config_default() {
        let config = ConnectionConfig::default();
        assert_eq!(config.host, "");
        assert_eq!(config.port, 22);
        assert_eq!(config.username, "");
        assert_eq!(config.auth_type, AuthType::Password);
        assert_eq!(config.timeout, 30);
        assert_eq!(config.keepalive, 60);
        assert!(!config.compression);
    }

    #[test]
    fn test_connection_config_new_uses_defaults_for_rest() {
        let config = ConnectionConfig::new("example.com", "alice");
        assert_eq!(config.host, "example.com");
        assert_eq!(config.username, "alice");
        assert_eq!(config.port, 22);
        assert_eq!(config.auth_type, AuthType::Password);
        assert_eq!(config.timeout, 30);
        assert_eq!(config.keepalive, 60);
        assert!(!config.compression);
    }

    #[test]
    fn test_connection_config_new_accepts_owned_and_borrowed_strings() {
        let owned = ConnectionConfig::new("host".to_string(), "user".to_string());
        assert_eq!(owned.host, "host");
        assert_eq!(owned.username, "user");

        let borrowed = ConnectionConfig::new("host", "user");
        assert_eq!(borrowed.host, "host");
        assert_eq!(borrowed.username, "user");
    }

    #[test]
    fn test_with_port() {
        let config = ConnectionConfig::new("h", "u").with_port(2222);
        assert_eq!(config.port, 2222);
    }

    #[test]
    fn test_with_port_zero_boundary() {
        let config = ConnectionConfig::new("h", "u").with_port(0);
        assert_eq!(config.port, 0);
    }

    #[test]
    fn test_with_auth() {
        let config = ConnectionConfig::new("h", "u").with_auth(AuthType::PublicKey);
        assert_eq!(config.auth_type, AuthType::PublicKey);

        let config = config.with_auth(AuthType::KeyboardInteractive);
        assert_eq!(config.auth_type, AuthType::KeyboardInteractive);
    }

    #[test]
    fn test_with_timeout() {
        let config = ConnectionConfig::new("h", "u").with_timeout(120);
        assert_eq!(config.timeout, 120);
    }

    #[test]
    fn test_with_timeout_zero_boundary() {
        let config = ConnectionConfig::new("h", "u").with_timeout(0);
        assert_eq!(config.timeout, 0);
    }

    #[test]
    fn test_with_keepalive() {
        let config = ConnectionConfig::new("h", "u").with_keepalive(15);
        assert_eq!(config.keepalive, 15);
    }

    #[test]
    fn test_with_compression() {
        let config = ConnectionConfig::new("h", "u").with_compression(true);
        assert!(config.compression);

        let config = config.with_compression(false);
        assert!(!config.compression);
    }

    #[test]
    fn test_builder_chaining_sets_all_fields() {
        let config = ConnectionConfig::new("host.example", "bob")
            .with_port(2200)
            .with_auth(AuthType::PublicKey)
            .with_timeout(45)
            .with_keepalive(20)
            .with_compression(true);

        assert_eq!(config.host, "host.example");
        assert_eq!(config.username, "bob");
        assert_eq!(config.port, 2200);
        assert_eq!(config.auth_type, AuthType::PublicKey);
        assert_eq!(config.timeout, 45);
        assert_eq!(config.keepalive, 20);
        assert!(config.compression);
    }

    #[test]
    fn test_connection_config_clone_is_independent() {
        let original = ConnectionConfig::new("h", "u").with_port(2222);
        let cloned = original.clone();
        assert_eq!(cloned.host, original.host);
        assert_eq!(cloned.port, original.port);
    }
}
