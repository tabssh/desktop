//! Settings tests

use tabssh::storage::settings::{Settings, CursorStyle, BellStyle};
use tabssh::storage::database::Database;

#[test]
fn test_default_settings() {
    let settings = Settings::default();
    
    assert_eq!(settings.default_port,22);
    assert_eq!(settings.connection_timeout,30);
    assert_eq!(settings.font_size,14.0);
    assert_eq!(settings.scrollback_lines,10000);
    assert!(settings.strict_host_key_checking);
}

#[test]
fn test_settings_serialization() {
    let settings = Settings::default();
    let json = serde_json::to_string(&settings).unwrap();
    let deserialized: Settings = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.default_port,settings.default_port);
}

#[test]
fn test_settings_persistence() {
    let db = Database::open().unwrap();
    let settings = Settings::default();
    
    settings.save(&db).unwrap();
    let loaded = Settings::load(&db).unwrap();
    
    assert_eq!(loaded.default_port,settings.default_port);
}
