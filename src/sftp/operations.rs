//! SFTP file operations

use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use super::FileEntry;

/// SFTP file operations handler
pub struct SftpOperations {
    // Will be connected to russh SFTP session
}

impl SftpOperations {
    pub fn new() -> Self {
        Self {}
    }
    
    /// List directory contents
    pub async fn list_directory(&self, path: &Path) -> Result<Vec<FileEntry>> {
        // TODO: Implement with russh SFTP
        log::info!("Listingdirectory:{}",path.display());
        Ok(Vec::new())
    }
    
    /// Download file from remote to local
    pub async fn download_file(
        &self,
        remote_path: &Path,
        local_path: &Path,
        progress_callback: impl Fn(u64, u64),
    ) -> Result<()> {
        // TODO: Implement with russh SFTP
        log::info!("Download:{}->{}",remote_path.display(),local_path.display());
        progress_callback(0, 0);
        Ok(())
    }
    
    /// Upload file from local to remote
    pub async fn upload_file(
        &self,
        local_path: &Path,
        remote_path: &Path,
        progress_callback: impl Fn(u64, u64),
    ) -> Result<()> {
        // TODO: Implement with russh SFTP
        log::info!("Upload:{}->{}",local_path.display(),remote_path.display());
        progress_callback(0, 0);
        Ok(())
    }
    
    /// Delete file or directory
    pub async fn delete(&self, path: &Path, recursive: bool) -> Result<()> {
        // TODO: Implement with russh SFTP
        log::info!("Delete:{}(recursive:{})",path.display(),recursive);
        Ok(())
    }
    
    /// Rename/move file or directory
    pub async fn rename(&self, old_path: &Path, new_path: &Path) -> Result<()> {
        // TODO: Implement with russh SFTP
        log::info!("Rename:{}->{}",old_path.display(),new_path.display());
        Ok(())
    }
    
    /// Create directory
    pub async fn create_directory(&self, path: &Path) -> Result<()> {
        // TODO: Implement with russh SFTP
        log::info!("Createdirectory:{}",path.display());
        Ok(())
    }
    
    /// Change permissions
    pub async fn chmod(&self, path: &Path, mode: u32) -> Result<()> {
        // TODO: Implement with russh SFTP
        log::info!("Chmod:{}to{:o}",path.display(),mode);
        Ok(())
    }
    
    /// Get file info
    pub async fn stat(&self, path: &Path) -> Result<FileEntry> {
        // TODO: Implement with russh SFTP
        Err(anyhow!("Notimplemented"))
    }
}

impl Default for SftpOperations {
    fn default() -> Self {
        Self::new()
    }
}
