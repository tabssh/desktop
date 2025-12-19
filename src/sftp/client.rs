//! SFTP client implementation using russh-sftp

use anyhow::{anyhow, Context, Result};
use russh::Channel;
use russh_sftp::client::SftpSession;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct SftpClient {
    session_id: String,
    sftp: Option<SftpSession>,
    current_path: PathBuf,
}

impl SftpClient {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            sftp: None,
            current_path: PathBuf::from("/"),
        }
    }

    /// Connect SFTP session over existing SSH channel
    pub async fn connect(&mut self, channel: Channel<russh::client::Msg>) -> Result<()> {
        log::info!("SFTP: Connecting session {}", self.session_id);
        
        let sftp = SftpSession::new(channel)
            .await
            .context("Failed to create SFTP session")?;
        
        self.sftp = Some(sftp);
        log::info!("SFTP: Connected successfully");
        Ok(())
    }

    /// Disconnect SFTP session
    pub async fn disconnect(&mut self) -> Result<()> {
        log::info!("SFTP: Disconnecting session {}", self.session_id);
        
        if let Some(mut sftp) = self.sftp.take() {
            sftp.close().await?;
        }
        
        Ok(())
    }

    /// Get current SFTP session
    fn sftp(&self) -> Result<&SftpSession> {
        self.sftp.as_ref().ok_or_else(|| anyhow!("SFTP not connected"))
    }

    /// Get mutable SFTP session
    fn sftp_mut(&mut self) -> Result<&mut SftpSession> {
        self.sftp.as_mut().ok_or_else(|| anyhow!("SFTP not connected"))
    }

    /// List directory contents
    pub async fn list_directory(&mut self, path: &Path) -> Result<Vec<FileEntry>> {
        log::debug!("SFTP: Listing directory {:?}", path);
        
        let sftp = self.sftp_mut()?;
        let path_str = path.to_string_lossy();
        
        let dir = sftp.open_dir(&path_str).await?;
        let entries = sftp.read_dir(&dir).await?;
        
        let mut files = Vec::new();
        for entry in entries {
            let attrs = entry.attrs();
            
            files.push(FileEntry {
                name: entry.filename().to_string(),
                path: path.join(entry.filename()),
                size: attrs.size.unwrap_or(0),
                is_directory: attrs.is_dir(),
                permissions: attrs.permissions.unwrap_or(0),
                modified: attrs.mtime
                    .map(|t| chrono::DateTime::from_timestamp(t as i64, 0))
                    .flatten()
                    .unwrap_or_else(|| chrono::Utc::now()),
            });
        }
        
        sftp.close_dir(dir).await?;
        
        log::debug!("SFTP: Found {} entries", files.len());
        Ok(files)
    }

    /// Download file from remote to local
    pub async fn download_file(
        &mut self,
        remote_path: &Path,
        local_path: &Path,
        progress_callback: Option<Box<dyn Fn(u64, u64) + Send>>,
    ) -> Result<()> {
        log::info!("SFTP: Downloading {:?} to {:?}", remote_path, local_path);
        
        let sftp = self.sftp_mut()?;
        let remote_str = remote_path.to_string_lossy();
        
        // Open remote file
        let remote_file = sftp.open(&remote_str).await?;
        let attrs = sftp.fstat(&remote_file).await?;
        let total_size = attrs.size.unwrap_or(0);
        
        // Create local file
        let mut local_file = File::create(local_path).await?;
        
        // Read and write in chunks
        let mut buffer = vec![0u8; 32768]; // 32KB buffer
        let mut transferred = 0u64;
        
        loop {
            let n = sftp.read(&remote_file, transferred, &mut buffer).await?;
            if n == 0 {
                break;
            }
            
            local_file.write_all(&buffer[..n]).await?;
            transferred += n as u64;
            
            if let Some(ref callback) = progress_callback {
                callback(transferred, total_size);
            }
        }
        
        sftp.close(remote_file).await?;
        local_file.flush().await?;
        
        log::info!("SFTP: Download complete ({} bytes)", transferred);
        Ok(())
    }

    /// Upload file from local to remote
    pub async fn upload_file(
        &mut self,
        local_path: &Path,
        remote_path: &Path,
        progress_callback: Option<Box<dyn Fn(u64, u64) + Send>>,
    ) -> Result<()> {
        log::info!("SFTP: Uploading {:?} to {:?}", local_path, remote_path);
        
        let sftp = self.sftp_mut()?;
        let remote_str = remote_path.to_string_lossy();
        
        // Open local file
        let mut local_file = File::open(local_path).await?;
        let metadata = local_file.metadata().await?;
        let total_size = metadata.len();
        
        // Create remote file
        let remote_file = sftp.create(&remote_str).await?;
        
        // Read and write in chunks
        let mut buffer = vec![0u8; 32768]; // 32KB buffer
        let mut transferred = 0u64;
        
        loop {
            let n = local_file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            
            sftp.write(&remote_file, transferred, &buffer[..n]).await?;
            transferred += n as u64;
            
            if let Some(ref callback) = progress_callback {
                callback(transferred, total_size);
            }
        }
        
        sftp.close(remote_file).await?;
        
        log::info!("SFTP: Upload complete ({} bytes)", transferred);
        Ok(())
    }

    /// Create directory
    pub async fn create_directory(&mut self, path: &Path) -> Result<()> {
        log::info!("SFTP: Creating directory {:?}", path);
        
        let sftp = self.sftp_mut()?;
        let path_str = path.to_string_lossy();
        
        sftp.create_dir(&path_str).await?;
        
        log::info!("SFTP: Directory created");
        Ok(())
    }

    /// Delete file
    pub async fn delete_file(&mut self, path: &Path) -> Result<()> {
        log::info!("SFTP: Deleting file {:?}", path);
        
        let sftp = self.sftp_mut()?;
        let path_str = path.to_string_lossy();
        
        sftp.remove_file(&path_str).await?;
        
        log::info!("SFTP: File deleted");
        Ok(())
    }

    /// Delete directory
    pub async fn delete_directory(&mut self, path: &Path) -> Result<()> {
        log::info!("SFTP: Deleting directory {:?}", path);
        
        let sftp = self.sftp_mut()?;
        let path_str = path.to_string_lossy();
        
        sftp.remove_dir(&path_str).await?;
        
        log::info!("SFTP: Directory deleted");
        Ok(())
    }

    /// Rename file or directory
    pub async fn rename(&mut self, old_path: &Path, new_path: &Path) -> Result<()> {
        log::info!("SFTP: Renaming {:?} to {:?}", old_path, new_path);
        
        let sftp = self.sftp_mut()?;
        let old_str = old_path.to_string_lossy();
        let new_str = new_path.to_string_lossy();
        
        sftp.rename(&old_str, &new_str).await?;
        
        log::info!("SFTP: Rename complete");
        Ok(())
    }

    /// Get file/directory stats
    pub async fn stat(&mut self, path: &Path) -> Result<FileEntry> {
        log::debug!("SFTP: Getting stats for {:?}", path);
        
        let sftp = self.sftp_mut()?;
        let path_str = path.to_string_lossy();
        
        let attrs = sftp.metadata(&path_str).await?;
        
        Ok(FileEntry {
            name: path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string(),
            path: path.to_path_buf(),
            size: attrs.size.unwrap_or(0),
            is_directory: attrs.is_dir(),
            permissions: attrs.permissions.unwrap_or(0),
            modified: attrs.mtime
                .map(|t| chrono::DateTime::from_timestamp(t as i64, 0))
                .flatten()
                .unwrap_or_else(|| chrono::Utc::now()),
        })
    }

    /// Change permissions
    pub async fn chmod(&mut self, path: &Path, mode: u32) -> Result<()> {
        log::info!("SFTP: Changing permissions of {:?} to {:o}", path, mode);
        
        let sftp = self.sftp_mut()?;
        let path_str = path.to_string_lossy();
        
        sftp.set_permissions(&path_str, mode).await?;
        
        log::info!("SFTP: Permissions changed");
        Ok(())
    }
    
    pub fn current_path(&self) -> &Path {
        &self.current_path
    }
    
    pub fn change_directory(&mut self, path: PathBuf) {
        self.current_path = path;
    }
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub is_directory: bool,
    pub permissions: u32,
    pub modified: chrono::DateTime<chrono::Utc>,
}
