//! Database unit tests

use tabssh::storage::database::Database;

#[test]
fn test_database_creation() {
    let db = Database::open().unwrap();
    assert!(db.connection().is_some());
}

#[test]
fn test_known_hosts() {
    let db = Database::open().unwrap();
    
    // Add known host
    db.add_known_host(
        "test.example.com",
        22,
        "ssh-ed25519",
        "SHA256:test-fingerprint",
        b"test-key-data",
    ).unwrap();
    
    // Retrieve it
    let host = db.get_known_host("test.example.com", 22).unwrap();
    assert!(host.is_some());
    let host = host.unwrap();
    assert_eq!(host.host,"test.example.com");
    assert_eq!(host.fingerprint,"SHA256:test-fingerprint");
    
    // Update last seen
    db.update_known_host_last_seen("test.example.com", 22).unwrap();
    
    // Remove it
    db.remove_known_host("test.example.com", 22).unwrap();
    let host = db.get_known_host("test.example.com", 22).unwrap();
    assert!(host.is_none());
}

#[test]
fn test_list_known_hosts() {
    let db = Database::open().unwrap();
    
    db.add_known_host("host1.com", 22, "ssh-rsa", "fp1", b"key1").unwrap();
    db.add_known_host("host2.com", 2222, "ssh-ed25519", "fp2", b"key2").unwrap();
    
    let hosts = db.list_known_hosts().unwrap();
    assert!(hosts.len()>=2);
}
