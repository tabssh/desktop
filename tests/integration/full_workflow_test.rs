//! Full workflow integration test

use tabssh::ssh::{SessionManager, ConnectionConfig, Credentials};
use tabssh::storage::database::Database;
use tabssh::storage::settings::Settings;

#[tokio::test]
async fn test_full_application_workflow() {
    // Initialize database
    let db = Database::open().unwrap();
    
    // Load settings
    let settings = Settings::load(&db).unwrap();
    assert_eq!(settings.default_port,22);
    
    // Create session manager
    let manager = SessionManager::new();
    
    // Test config
    let config = ConnectionConfig {
        host: "example.com".to_string(),
        port: 22,
        username: "user".to_string(),
        timeout: 30,
        keepalive: 60,
        compression: false,
    };
    
    // Create session (would fail without real SSH server)
    let result = manager.create_session(
        config,
        Credentials::Password("test".to_string())
    );
    
    assert!(result.is_ok());
}
