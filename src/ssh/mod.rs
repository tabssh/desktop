//! SSH module - SSH connection and session management

#![allow(dead_code)]

mod active_session;
mod auth;
mod connection;
mod config_parser;
mod forwarding;
mod session_manager;

pub use active_session::{ActiveSession, SessionEvent};
#[allow(unused_imports)]
pub use auth::{Credentials, find_default_keys};
#[allow(unused_imports)]
pub use connection::SshConnection;
pub use config_parser::{SshConfigParser, HostConfig};
pub use forwarding::{ForwardingManager, PortForward, ForwardType};
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
