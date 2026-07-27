//! Full workflow integration test

use std::sync::Arc;
use tabssh::ssh::{AuthType, ConnectionConfig, SessionManager};
use tabssh::storage::database::Database;
use tabssh::storage::settings::Settings;
use tokio::runtime::Runtime;

#[test]
fn test_full_application_workflow() {
    // Plain sync test driving its own runtime via block_on, rather than
    // #[tokio::test]: SessionManager owns a real Arc<Runtime> (needed to
    // block_on async SSH work from the sync GUI thread in production),
    // and dropping that nested Runtime from within an outer tokio async
    // context panics ("Cannot drop a runtime in a context where
    // blocking is not allowed").
    // Initialize database
    let db = Database::open().unwrap();

    // Load settings
    let settings = Settings::load(&db).unwrap();
    assert_eq!(settings.default_port, 22);

    // Create session manager
    let runtime = Arc::new(Runtime::new().unwrap());
    let manager = SessionManager::new(runtime.clone());

    // Test config. A short timeout keeps this test fast and deterministic:
    // example.com resolves but does not accept SSH connections, so without
    // an enforced connect timeout this would hang for the OS-level TCP
    // timeout (or indefinitely in a sandboxed network that drops packets).
    let config = ConnectionConfig {
        host: "example.com".to_string(),
        port: 22,
        username: "user".to_string(),
        auth_type: AuthType::Password,
        timeout: 5,
        keepalive: 60,
        compression: false,
    };

    // Connect (fails without a real SSH server, which is expected here)
    let result = runtime.block_on(manager.connect_password(config, "test"));

    assert!(result.is_err());
}
