//! Error types and handling

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TabSshError {
    #[error("SSH connection error: {0}")]
    SshConnection(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Host key verification failed: {0}")]
    HostKeyVerification(String),

    #[error("SFTP error: {0}")]
    Sftp(String),

    #[error("Port forwarding error: {0}")]
    PortForwarding(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, TabSshError>;

impl TabSshError {
    pub fn user_message(&self) -> String {
        match self {
            TabSshError::SshConnection(msg) => format!("Connection failed: {}", msg),
            TabSshError::AuthenticationFailed(msg) => format!("Authentication failed: {}", msg),
            TabSshError::HostKeyVerification(msg) => format!("Host key error: {}", msg),
            TabSshError::Sftp(msg) => format!("File transfer error: {}", msg),
            TabSshError::PortForwarding(msg) => format!("Port forwarding error: {}", msg),
            TabSshError::Database(err) => format!("Database error: {}", err),
            TabSshError::Io(err) => format!("IO error: {}", err),
            TabSshError::Parse(msg) => format!("Parse error: {}", msg),
            TabSshError::Config(msg) => format!("Configuration error: {}", msg),
            TabSshError::Unknown(msg) => format!("Error: {}", msg),
        }
    }
}
