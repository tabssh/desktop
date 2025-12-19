//! SSH port forwarding implementation

use anyhow::{anyhow, Result};
use russh::client::Handle;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

/// Port forward type
#[derive(Debug, Clone, PartialEq)]
pub enum ForwardType {
    Local,  // ssh -L
    Remote, // ssh -R
    Dynamic, // ssh -D (SOCKS)
}

/// Port forward configuration
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

/// Port forwarding manager
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
        ssh_handle: Handle<H>,
    ) -> Result<()>
    where
        H: russh::client::Handler + Send + 'static,
    {
        let listen_addr: SocketAddr = format!("{}:{}",forward.listen_addr,forward.listen_port).parse()?;
        let listener = TcpListener::bind(listen_addr).await?;
        
        log::info!("Localforward:{}->{}:{}",
            listen_addr, forward.remote_host, forward.remote_port);

        let remote_host = forward.remote_host.clone();
        let remote_port = forward.remote_port;

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((mut local_stream, _)) => {
                        let ssh = ssh_handle.clone();
                        let host = remote_host.clone();
                        let port = remote_port;

                        tokio::spawn(async move {
                            match ssh.channel_open_direct_tcpip(
                                &host,
                                port as u32,
                                "127.0.0.1",
                                0,
                            ).await {
                                Ok(mut channel) => {
                                    let (mut read_half, mut write_half) = local_stream.split();
                                    
                                    tokio::spawn(async move {
                                        let mut buf = [0u8; 8192];
                                        loop {
                                            match read_half.read(&mut buf).await {
                                                Ok(0) => break,
                                                Ok(n) => {
                                                    if channel.data(&buf[..n]).await.is_err() {
                                                        break;
                                                    }
                                                }
                                                Err(_) => break,
                                            }
                                        }
                                    });

                                    let mut buf = Vec::new();
                                    loop {
                                        match channel.wait().await {
                                            Some(russh::ChannelMsg::Data { data }) => {
                                                if write_half.write_all(&data).await.is_err() {
                                                    break;
                                                }
                                            }
                                            Some(russh::ChannelMsg::Eof) | 
                                            Some(russh::ChannelMsg::Close) | None => break,
                                            _ => {}
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!("FailedtoopenSSHchannel:{}",e);
                                }
                            }
                        });
                    }
                    Err(e) => {
                        log::error!("Accepterror:{}",e);
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn start_dynamic_forward<H>(
        &self,
        forward: PortForward,
        ssh_handle: Handle<H>,
    ) -> Result<()>
    where
        H: russh::client::Handler + Send + 'static,
    {
        let listen_addr: SocketAddr = format!("{}:{}",forward.listen_addr,forward.listen_port).parse()?;
        let listener = TcpListener::bind(listen_addr).await?;
        
        log::info!("Dynamicforward(SOCKS):{}",listen_addr);

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        let ssh = ssh_handle.clone();
                        tokio::spawn(handle_socks_connection(stream, ssh));
                    }
                    Err(e) => {
                        log::error!("Accepterror:{}",e);
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

async fn handle_socks_connection<H>(mut stream: TcpStream, ssh_handle: Handle<H>)
where
    H: russh::client::Handler + Send + 'static,
{
    // Basic SOCKS5 implementation
    let mut buf = [0u8; 2];
    if stream.read_exact(&mut buf).await.is_err() {
        return;
    }

    if buf[0] != 5 {
        return; // Not SOCKS5
    }

    // Send no authentication required
    if stream.write_all(&[5, 0]).await.is_err() {
        return;
    }

    // Read request
    let mut buf = [0u8; 4];
    if stream.read_exact(&mut buf).await.is_err() {
        return;
    }

    if buf[1] != 1 {
        return; // Only CONNECT supported
    }

    let (host, port) = match buf[3] {
        1 => {
            // IPv4
            let mut addr = [0u8; 4];
            if stream.read_exact(&mut addr).await.is_err() {
                return;
            }
            let mut port_buf = [0u8; 2];
            if stream.read_exact(&mut port_buf).await.is_err() {
                return;
            }
            let port = u16::from_be_bytes(port_buf);
            (format!("{}.{}.{}.{}",addr[0],addr[1],addr[2],addr[3]),port)
        }
        3 => {
            // Domain name
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
            let port = u16::from_be_bytes(port_buf);
            (String::from_utf8_lossy(&domain).to_string(), port)
        }
        _ => return,
    };

    // Open SSH channel
    match ssh_handle.channel_open_direct_tcpip(&host, port as u32, "127.0.0.1", 0).await {
        Ok(mut channel) => {
            // Send success
            if stream.write_all(&[5, 0, 0, 1, 0, 0, 0, 0, 0, 0]).await.is_err() {
                return;
            }

            // Relay data
            let (mut read_half, mut write_half) = stream.split();
            
            tokio::spawn(async move {
                let mut buf = [0u8; 8192];
                loop {
                    match read_half.read(&mut buf).await {
                        Ok(0) => break,
                        Ok(n) => {
                            if channel.data(&buf[..n]).await.is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            });

            loop {
                match channel.wait().await {
                    Some(russh::ChannelMsg::Data { data }) => {
                        if write_half.write_all(&data).await.is_err() {
                            break;
                        }
                    }
                    Some(russh::ChannelMsg::Eof) | 
                    Some(russh::ChannelMsg::Close) | None => break,
                    _ => {}
                }
            }
        }
        Err(_) => {
            // Send failure
            let _ = stream.write_all(&[5, 1, 0, 1, 0, 0, 0, 0, 0, 0]).await;
        }
    }
}
