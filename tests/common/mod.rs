//! Common test utilities

use std::path::PathBuf;

/// Get test data directory
pub fn test_data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("data")
}

/// Create temporary directory for tests
pub fn temp_test_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tabssh-test-{}",uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Cleanup test directory
pub fn cleanup_test_dir(dir: &PathBuf) {
    let _ = std::fs::remove_dir_all(dir);
}
