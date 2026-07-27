//! Integration tests for SSH connection flow

#[cfg(test)]
mod connection_tests {
    use std::sync::Arc;
    use tabssh::ssh::{AuthType, ConnectionConfig, SessionManager};
    use tokio::runtime::Runtime;

    #[test]
    fn test_connection_lifecycle() {
        // Plain sync test driving its own runtime via block_on, rather than
        // #[tokio::test]: SessionManager owns a real Arc<Runtime> (needed to
        // block_on async SSH work from the sync GUI thread in production),
        // and dropping that nested Runtime from within an outer tokio async
        // context panics ("Cannot drop a runtime in a context where
        // blocking is not allowed").
        let runtime = Arc::new(Runtime::new().unwrap());
        let manager = SessionManager::new(runtime.clone());

        let config = ConnectionConfig {
            host: "test.example.com".to_string(),
            port: 22,
            username: "testuser".to_string(),
            auth_type: AuthType::Password,
            timeout: 30,
            keepalive: 60,
            compression: false,
        };

        // This fails without a real SSH server, but tests the flow
        let result = runtime.block_on(manager.connect_password(config.clone(), "test"));

        assert!(result.is_err());
    }
}
