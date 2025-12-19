//! Integration tests for SSH connection flow

#[cfg(test)]
mod connection_tests {
    use tabssh::ssh::{SessionManager, ConnectionConfig, Credentials};
    
    #[tokio::test]
    async fn test_connection_lifecycle() {
        let manager = SessionManager::new();
        
        let config = ConnectionConfig {
            host: "test.example.com".to_string(),
            port: 22,
            username: "testuser".to_string(),
            timeout: 30,
            keepalive: 60,
            compression: false,
        };
        
        // This would fail without a real SSH server, but tests the flow
        let result = manager.create_session(
            config.clone(),
            Credentials::Password("test".to_string())
        );
        
        assert!(result.is_ok());
    }
}
