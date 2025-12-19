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
            TabSshError::SshConnection(msg) => format!("Connectionfailed:{}",msg),
            TabSshError::AuthenticationFailed(msg) => format!("Authenticationfailed:{}",msg),
            TabSshError::HostKeyVerification(msg) => format!("Hostkeyerror:{}",msg),
            TabSshError::Sftp(msg) => format!("Filetransfererror:{}",msg),
            TabSshError::PortForwarding(msg) => format!("Portforwardingerror:{}",msg),
            TabSshError::Database(err) => format!("Databaseerror:{}",err),
            TabSshError::Io(err) => format!("IOerror:{}",err),
            TabSshError::Parse(msg) => format!("Parseerror:{}",msg),
            TabSshError::Config(msg) => format!("Configurationerror:{}",msg),
            TabSshError::Unknown(msg) => format!("Error:{}",msg),
        }
    }
}
