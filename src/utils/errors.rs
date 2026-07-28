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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_ssh_connection() {
        let err = TabSshError::SshConnection("timed out".to_string());
        assert_eq!(err.to_string(), "SSH connection error: timed out");
    }

    #[test]
    fn test_display_authentication_failed() {
        let err = TabSshError::AuthenticationFailed("bad password".to_string());
        assert_eq!(err.to_string(), "Authentication failed: bad password");
    }

    #[test]
    fn test_display_host_key_verification() {
        let err = TabSshError::HostKeyVerification("mismatch".to_string());
        assert_eq!(err.to_string(), "Host key verification failed: mismatch");
    }

    #[test]
    fn test_display_sftp() {
        let err = TabSshError::Sftp("permission denied".to_string());
        assert_eq!(err.to_string(), "SFTP error: permission denied");
    }

    #[test]
    fn test_display_port_forwarding() {
        let err = TabSshError::PortForwarding("bind failed".to_string());
        assert_eq!(err.to_string(), "Port forwarding error: bind failed");
    }

    #[test]
    fn test_display_parse() {
        let err = TabSshError::Parse("unexpected token".to_string());
        assert_eq!(err.to_string(), "Parse error: unexpected token");
    }

    #[test]
    fn test_display_config() {
        let err = TabSshError::Config("missing field".to_string());
        assert_eq!(err.to_string(), "Configuration error: missing field");
    }

    #[test]
    fn test_display_unknown() {
        let err = TabSshError::Unknown("oops".to_string());
        assert_eq!(err.to_string(), "Unknown error: oops");
    }

    #[test]
    fn test_display_empty_message() {
        // Boundary: empty message string still formats correctly.
        let err = TabSshError::Unknown(String::new());
        assert_eq!(err.to_string(), "Unknown error: ");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let err: TabSshError = io_err.into();
        assert!(matches!(err, TabSshError::Io(_)));
        assert!(err.to_string().starts_with("IO error: "));
    }

    #[test]
    fn test_from_rusqlite_error() {
        let sqlite_err = rusqlite::Error::QueryReturnedNoRows;
        let err: TabSshError = sqlite_err.into();
        assert!(matches!(err, TabSshError::Database(_)));
        assert!(err.to_string().starts_with("Database error: "));
    }

    #[test]
    fn test_user_message_ssh_connection() {
        let err = TabSshError::SshConnection("refused".to_string());
        assert_eq!(err.user_message(), "Connection failed: refused");
    }

    #[test]
    fn test_user_message_authentication_failed() {
        let err = TabSshError::AuthenticationFailed("no key".to_string());
        assert_eq!(err.user_message(), "Authentication failed: no key");
    }

    #[test]
    fn test_user_message_host_key_verification() {
        let err = TabSshError::HostKeyVerification("changed".to_string());
        assert_eq!(err.user_message(), "Host key error: changed");
    }

    #[test]
    fn test_user_message_sftp() {
        let err = TabSshError::Sftp("not found".to_string());
        assert_eq!(err.user_message(), "File transfer error: not found");
    }

    #[test]
    fn test_user_message_port_forwarding() {
        let err = TabSshError::PortForwarding("in use".to_string());
        assert_eq!(err.user_message(), "Port forwarding error: in use");
    }

    #[test]
    fn test_user_message_database() {
        let sqlite_err = rusqlite::Error::QueryReturnedNoRows;
        let err = TabSshError::Database(sqlite_err);
        assert!(err.user_message().starts_with("Database error: "));
    }

    #[test]
    fn test_user_message_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let err = TabSshError::Io(io_err);
        assert!(err.user_message().starts_with("IO error: "));
    }

    #[test]
    fn test_user_message_parse() {
        let err = TabSshError::Parse("bad json".to_string());
        assert_eq!(err.user_message(), "Parse error: bad json");
    }

    #[test]
    fn test_user_message_config() {
        let err = TabSshError::Config("bad toml".to_string());
        assert_eq!(err.user_message(), "Configuration error: bad toml");
    }

    #[test]
    fn test_user_message_unknown() {
        let err = TabSshError::Unknown("mystery".to_string());
        assert_eq!(err.user_message(), "Error: mystery");
    }

    #[test]
    fn test_result_alias_ok() {
        let ok: Result<u32> = Ok(42);
        match ok {
            Ok(v) => assert_eq!(v, 42),
            Err(_) => panic!("expected Ok(42)"),
        }
    }

    #[test]
    fn test_result_alias_err() {
        let err: Result<u32> = Err(TabSshError::Unknown("fail".to_string()));
        assert!(err.is_err());
    }
}
