//! Port forwarding integration tests

use tabssh::ssh::{PortForward, ForwardType, ForwardingManager};

#[tokio::test]
async fn test_port_forward_creation() {
    let forward = PortForward::new_local(8080, "localhost".to_string(), 80);
    
    assert_eq!(forward.forward_type,ForwardType::Local);
    assert_eq!(forward.listen_port,8080);
    assert_eq!(forward.remote_host,"localhost");
    assert_eq!(forward.remote_port,80);
}

#[tokio::test]
async fn test_forwarding_manager() {
    let manager = ForwardingManager::new();
    
    let forward = PortForward::new_dynamic(1080);
    let id = forward.id;
    
    manager.add_forward(forward).await;
    
    let forwards = manager.list_forwards().await;
    assert_eq!(forwards.len(),1);
    
    manager.remove_forward(id).await;
    
    let forwards = manager.list_forwards().await;
    assert_eq!(forwards.len(),0);
}
