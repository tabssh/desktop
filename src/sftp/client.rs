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
        let old_str = old_path.to_string_lossy().to_string();
        let new_str = new_path.to_string_lossy().to_string();
        
        sftp.rename(old_str, new_str).await?;
        
        log::info!("SFTP: Rename complete");
        Ok(())
    }

    /// Get file/directory stats
    pub async fn stat(&mut self, path: &Path) -> Result<FileEntry> {
        log::debug!("SFTP: Getting stats for {:?}", path);
        
        let sftp = self.sftp_mut()?;
        let path_str = path.to_string_lossy().to_string();
        
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
        let path_str = path.to_string_lossy().to_string();
        
        // russh-sftp uses setstat instead of set_permissions
        let mut attrs = russh_sftp::protocol::FileAttributes::default();
        attrs.permissions = Some(mode);
        sftp.setstat(&path_str, attrs).await?;
        
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

/// Transfer task for tracking file transfers
#[derive(Debug, Clone)]
pub struct TransferTask {
    pub id: uuid::Uuid,
    pub local_path: PathBuf,
    pub remote_path: PathBuf,
    pub direction: super::TransferDirection,
    pub total_bytes: u64,
    pub transferred_bytes: u64,
    pub state: super::TransferState,
}

/// Read local directory contents
pub fn read_local_directory(path: &Path) -> Result<Vec<FileEntry>> {
    let mut entries = Vec::new();
    
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().to_string();
        
        let modified = metadata.modified()
            .ok()
            .and_then(|t| chrono::DateTime::from(t).into())
            .unwrap_or_else(chrono::Utc::now);
        
        #[cfg(unix)]
        let permissions = {
            use std::os::unix::fs::PermissionsExt;
            metadata.permissions().mode()
        };
        #[cfg(not(unix))]
        let permissions = 0o644;
        
        entries.push(FileEntry {
            name,
            path: entry.path(),
            size: metadata.len(),
            is_directory: metadata.is_dir(),
            permissions,
            modified,
        });
    }
    
    Ok(entries)
}

/// Create local directory
pub fn create_local_directory(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

/// Delete local path (file or directory)
pub fn delete_local_path(path: &Path) -> Result<()> {
    if path.is_dir() {
        std::fs::remove_dir_all(path)?;
    } else {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

/// Format file size for display
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    
    if unit_idx == 0 {
        format!("{} {}", bytes, UNITS[0])
    } else {
        format!("{:.2} {}", size, UNITS[unit_idx])
    }
}

/// Format Unix permissions for display
pub fn format_permissions(mode: u32) -> String {
    let perms = [
        (mode & 0o400 != 0, 'r'),
        (mode & 0o200 != 0, 'w'),
        (mode & 0o100 != 0, 'x'),
        (mode & 0o040 != 0, 'r'),
        (mode & 0o020 != 0, 'w'),
        (mode & 0o010 != 0, 'x'),
        (mode & 0o004 != 0, 'r'),
        (mode & 0o002 != 0, 'w'),
        (mode & 0o001 != 0, 'x'),
    ];
    
    perms
        .iter()
        .map(|(has_perm, ch)| if *has_perm { *ch } else { '-' })
        .collect()
}
