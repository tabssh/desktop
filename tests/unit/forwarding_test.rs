//! Port forwarding unit tests

use tabssh::ssh::{PortForward, ForwardType};

#[test]
fn test_local_forward_creation() {
    let forward = PortForward::new_local(8080, "example.com".to_string(), 80);
    
    assert_eq!(forward.forward_type,ForwardType::Local);
    assert_eq!(forward.listen_port,8080);
    assert_eq!(forward.remote_host,"example.com");
    assert_eq!(forward.remote_port,80);
    assert!(!forward.active);
}

#[test]
fn test_remote_forward_creation() {
    let forward = PortForward::new_remote(8080, "localhost".to_string(), 3000);
    
    assert_eq!(forward.forward_type,ForwardType::Remote);
    assert_eq!(forward.listen_port,8080);
    assert_eq!(forward.remote_host,"localhost");
    assert_eq!(forward.remote_port,3000);
}

#[test]
fn test_dynamic_forward_creation() {
    let forward = PortForward::new_dynamic(1080);
    
    assert_eq!(forward.forward_type,ForwardType::Dynamic);
    assert_eq!(forward.listen_port,1080);
}
