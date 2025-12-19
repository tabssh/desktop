//! SSH config parser tests

use tabssh::ssh::SshConfigParser;

#[test]
fn test_parse_example_config() {
    let config = r#"
Host myserver
    HostName server.example.com
    Port 2222
    User admin
    IdentityFile ~/.ssh/id_rsa
    LocalForward 8080 localhost:80
    DynamicForward 1080
    Compression yes

Host *.internal
    ProxyJump bastion
    User internal-user

Host *
    ServerAliveInterval 60
"#;

    let mut parser = SshConfigParser::new();
    parser.parse_content(config).unwrap();

    // Test specific host
    let myserver = parser.get_config("myserver").unwrap();
    assert_eq!(myserver.hostname,Some("server.example.com".to_string()));
    assert_eq!(myserver.port,Some(2222));
    assert_eq!(myserver.user,Some("admin".to_string()));
    assert_eq!(myserver.compression,Some(true));
    assert!(!myserver.local_forward.is_empty());
    assert!(!myserver.dynamic_forward.is_empty());

    // Test wildcard pattern
    let internal = parser.get_config("web.internal").unwrap();
    assert_eq!(internal.proxy_jump,Some("bastion".to_string()));
}

#[test]
fn test_empty_config() {
    let parser = SshConfigParser::new();
    assert!(parser.get_config("nonexistent").is_none());
}

#[test]
fn test_comments_and_empty_lines() {
    let config = r#"
# This is a comment
Host test

    # Another comment
    HostName test.com
    
    # Inline comment
    Port 22 # Port number
"#;

    let mut parser = SshConfigParser::new();
    assert!(parser.parse_content(config).is_ok());
}
