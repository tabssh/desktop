//! SSH port forwarding implementation

use anyhow::Result;
use russh::client::Handle;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

#[derive(Debug, Clone, PartialEq)]
pub enum ForwardType {
    Local,   // ssh -L
    Remote,  // ssh -R
    Dynamic, // ssh -D (SOCKS)
}

#[derive(Debug, Clone)]
pub struct PortForward {
    pub id: uuid::Uuid,
    pub forward_type: ForwardType,
    pub listen_addr: String,
    pub listen_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
    pub active: bool,
}

impl PortForward {
    pub fn new_local(listen_port: u16, remote_host: String, remote_port: u16) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            forward_type: ForwardType::Local,
            listen_addr: "127.0.0.1".to_string(),
            listen_port,
            remote_host,
            remote_port,
            active: false,
        }
    }

    pub fn new_remote(remote_port: u16, local_host: String, local_port: u16) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            forward_type: ForwardType::Remote,
            listen_addr: "0.0.0.0".to_string(),
            listen_port: remote_port,
            remote_host: local_host,
            remote_port: local_port,
            active: false,
        }
    }

    pub fn new_dynamic(listen_port: u16) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            forward_type: ForwardType::Dynamic,
            listen_addr: "127.0.0.1".to_string(),
            listen_port,
            remote_host: String::new(),
            remote_port: 0,
            active: false,
        }
    }
}

pub struct ForwardingManager {
    forwards: Arc<Mutex<Vec<PortForward>>>,
}

impl ForwardingManager {
    pub fn new() -> Self {
        Self {
            forwards: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add_forward(&self, forward: PortForward) {
        self.forwards.lock().await.push(forward);
    }

    pub async fn remove_forward(&self, id: uuid::Uuid) {
        self.forwards.lock().await.retain(|f| f.id != id);
    }

    pub async fn list_forwards(&self) -> Vec<PortForward> {
        self.forwards.lock().await.clone()
    }

    pub async fn start_local_forward<H>(
        &self,
        forward: PortForward,
        ssh_handle: Arc<Handle<H>>,
    ) -> Result<()>
    where
        H: russh::client::Handler + Send + Sync + 'static,
    {
        let listen_addr: SocketAddr =
            format!("{}:{}", forward.listen_addr, forward.listen_port).parse()?;
        let listener = TcpListener::bind(listen_addr).await?;

        log::info!(
            "Local forward: {} -> {}:{}",
            listen_addr,
            forward.remote_host,
            forward.remote_port
        );

        let remote_host = forward.remote_host.clone();
        let remote_port = forward.remote_port;

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((local_stream, peer)) => {
                        let ssh = ssh_handle.clone();
                        let host = remote_host.clone();
                        let port = remote_port;
                        tokio::spawn(async move {
                            if let Err(e) =
                                pipe_local_to_channel(ssh, local_stream, host, port).await
                            {
                                log::warn!("forward({}): {}", peer, e);
                            }
                        });
                    }
                    Err(e) => {
                        log::error!("Accept error: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn start_dynamic_forward<H>(
        &self,
        forward: PortForward,
        ssh_handle: Arc<Handle<H>>,
    ) -> Result<()>
    where
        H: russh::client::Handler + Send + Sync + 'static,
    {
        let listen_addr: SocketAddr =
            format!("{}:{}", forward.listen_addr, forward.listen_port).parse()?;
        let listener = TcpListener::bind(listen_addr).await?;

        log::info!("Dynamic forward (SOCKS): {}", listen_addr);

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        let ssh = ssh_handle.clone();
                        tokio::spawn(handle_socks_connection(stream, ssh));
                    }
                    Err(e) => {
                        log::error!("Accept error: {}", e);
                    }
                }
            }
        });

        Ok(())
    }
}

impl Default for ForwardingManager {
    fn default() -> Self {
        Self::new()
    }
}

async fn pipe_local_to_channel<H>(
    ssh: Arc<Handle<H>>,
    mut local: TcpStream,
    remote_host: String,
    remote_port: u16,
) -> Result<()>
where
    H: russh::client::Handler + Send + Sync + 'static,
{
    let channel = ssh
        .channel_open_direct_tcpip(remote_host, remote_port as u32, "127.0.0.1", 0)
        .await?;
    let mut channel_stream = channel.into_stream();
    tokio::io::copy_bidirectional(&mut local, &mut channel_stream).await?;
    Ok(())
}

async fn handle_socks_connection<H>(mut stream: TcpStream, ssh_handle: Arc<Handle<H>>)
where
    H: russh::client::Handler + Send + Sync + 'static,
{
    let mut buf = [0u8; 2];
    if stream.read_exact(&mut buf).await.is_err() {
        return;
    }
    if buf[0] != 5 {
        return;
    }
    if stream.write_all(&[5, 0]).await.is_err() {
        return;
    }

    let mut req = [0u8; 4];
    if stream.read_exact(&mut req).await.is_err() {
        return;
    }
    if req[1] != 1 {
        return;
    }

    let (host, port) = match req[3] {
        1 => {
            let mut addr = [0u8; 4];
            if stream.read_exact(&mut addr).await.is_err() {
                return;
            }
            let mut port_buf = [0u8; 2];
            if stream.read_exact(&mut port_buf).await.is_err() {
                return;
            }
            (
                format!("{}.{}.{}.{}", addr[0], addr[1], addr[2], addr[3]),
                u16::from_be_bytes(port_buf),
            )
        }
        3 => {
            let mut len = [0u8; 1];
            if stream.read_exact(&mut len).await.is_err() {
                return;
            }
            let mut domain = vec![0u8; len[0] as usize];
            if stream.read_exact(&mut domain).await.is_err() {
                return;
            }
            let mut port_buf = [0u8; 2];
            if stream.read_exact(&mut port_buf).await.is_err() {
                return;
            }
            (
                String::from_utf8_lossy(&domain).to_string(),
                u16::from_be_bytes(port_buf),
            )
        }
        _ => return,
    };

    match ssh_handle
        .channel_open_direct_tcpip(host, port as u32, "127.0.0.1", 0)
        .await
    {
        Ok(channel) => {
            if stream
                .write_all(&[5, 0, 0, 1, 0, 0, 0, 0, 0, 0])
                .await
                .is_err()
            {
                return;
            }
            let mut channel_stream = channel.into_stream();
            let _ = tokio::io::copy_bidirectional(&mut stream, &mut channel_stream).await;
        }
        Err(_) => {
            let _ = stream.write_all(&[5, 1, 0, 1, 0, 0, 0, 0, 0, 0]).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_local_defaults() {
        let forward = PortForward::new_local(8080, "example.com".to_string(), 80);
        assert_eq!(forward.forward_type, ForwardType::Local);
        assert_eq!(forward.listen_addr, "127.0.0.1");
        assert_eq!(forward.listen_port, 8080);
        assert_eq!(forward.remote_host, "example.com");
        assert_eq!(forward.remote_port, 80);
        assert!(!forward.active);
    }

    #[test]
    fn test_new_remote_defaults() {
        let forward = PortForward::new_remote(9090, "localhost".to_string(), 3000);
        assert_eq!(forward.forward_type, ForwardType::Remote);
        assert_eq!(forward.listen_addr, "0.0.0.0");
        assert_eq!(forward.listen_port, 9090);
        assert_eq!(forward.remote_host, "localhost");
        assert_eq!(forward.remote_port, 3000);
        assert!(!forward.active);
    }

    #[test]
    fn test_new_dynamic_defaults() {
        let forward = PortForward::new_dynamic(1080);
        assert_eq!(forward.forward_type, ForwardType::Dynamic);
        assert_eq!(forward.listen_addr, "127.0.0.1");
        assert_eq!(forward.listen_port, 1080);
        assert_eq!(forward.remote_host, "");
        assert_eq!(forward.remote_port, 0);
        assert!(!forward.active);
    }

    #[test]
    fn test_new_local_zero_port() {
        // Boundary: port 0 (ephemeral) must be accepted, not rejected.
        let forward = PortForward::new_local(0, String::new(), 0);
        assert_eq!(forward.listen_port, 0);
        assert_eq!(forward.remote_port, 0);
        assert_eq!(forward.remote_host, "");
    }

    #[test]
    fn test_forward_type_equality() {
        assert_eq!(ForwardType::Local, ForwardType::Local);
        assert_ne!(ForwardType::Local, ForwardType::Remote);
        assert_ne!(ForwardType::Remote, ForwardType::Dynamic);
    }

    #[test]
    fn test_port_forward_ids_are_unique() {
        let a = PortForward::new_local(1, "h".to_string(), 1);
        let b = PortForward::new_local(1, "h".to_string(), 1);
        assert_ne!(a.id, b.id);
    }

    #[tokio::test]
    async fn test_forwarding_manager_add_list_remove() {
        let manager = ForwardingManager::new();
        assert!(manager.list_forwards().await.is_empty());

        let forward = PortForward::new_local(8080, "example.com".to_string(), 80);
        let id = forward.id;
        manager.add_forward(forward).await;

        let forwards = manager.list_forwards().await;
        assert_eq!(forwards.len(), 1);
        assert_eq!(forwards[0].id, id);

        manager.remove_forward(id).await;
        assert!(manager.list_forwards().await.is_empty());
    }

    #[tokio::test]
    async fn test_forwarding_manager_remove_nonexistent_is_noop() {
        let manager = ForwardingManager::new();
        manager
            .add_forward(PortForward::new_local(8080, "example.com".to_string(), 80))
            .await;

        // Removing an id that was never added must not touch existing entries.
        manager.remove_forward(uuid::Uuid::new_v4()).await;
        assert_eq!(manager.list_forwards().await.len(), 1);
    }

    #[tokio::test]
    async fn test_forwarding_manager_add_multiple() {
        let manager = ForwardingManager::new();
        manager
            .add_forward(PortForward::new_local(1, "a".to_string(), 1))
            .await;
        manager
            .add_forward(PortForward::new_remote(2, "b".to_string(), 2))
            .await;
        manager.add_forward(PortForward::new_dynamic(3)).await;

        assert_eq!(manager.list_forwards().await.len(), 3);
    }

    #[tokio::test]
    async fn test_forwarding_manager_default() {
        let manager = ForwardingManager::default();
        assert!(manager.list_forwards().await.is_empty());
    }
}
