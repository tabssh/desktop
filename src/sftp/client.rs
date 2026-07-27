//! SFTP client implementation using russh-sftp 2.1.x
//!
//! Implemented per IDEA.md's SFTP-browser requirement but not yet wired
//! into the active session/UI flow (`SftpBrowser` in `browser.rs` is
//! UI-state-only with no network I/O) — see TODO.AI.md Phase 1.3.
#![allow(dead_code)]

use anyhow::{anyhow, Context, Result};
use russh::Channel;
use russh_sftp::client::SftpSession;
use russh_sftp::protocol::FileAttributes;
use std::path::{Path, PathBuf};
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::{FileEntry, FileType, TransferDirection, TransferState};

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

    pub async fn connect(&mut self, channel: Channel<russh::client::Msg>) -> Result<()> {
        log::info!("SFTP: Connecting session {}", self.session_id);
        channel
            .request_subsystem(true, "sftp")
            .await
            .context("Failed to request sftp subsystem")?;
        let sftp = SftpSession::new(channel.into_stream())
            .await
            .context("Failed to create SFTP session")?;
        self.sftp = Some(sftp);
        log::info!("SFTP: Connected successfully");
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        log::info!("SFTP: Disconnecting session {}", self.session_id);
        if let Some(sftp) = self.sftp.take() {
            sftp.close().await?;
        }
        Ok(())
    }

    fn sftp(&self) -> Result<&SftpSession> {
        self.sftp
            .as_ref()
            .ok_or_else(|| anyhow!("SFTP not connected"))
    }

    pub async fn list_directory(&self, path: &Path) -> Result<Vec<FileEntry>> {
        log::debug!("SFTP: Listing directory {:?}", path);
        let sftp = self.sftp()?;
        let path_str = path.to_string_lossy().into_owned();

        let read_dir = sftp.read_dir(path_str).await?;

        let mut files = Vec::new();
        for entry in read_dir {
            let name = entry.file_name();
            let attrs = entry.metadata();
            files.push(file_entry_from(path, name, &attrs));
        }

        log::debug!("SFTP: Found {} entries", files.len());
        Ok(files)
    }

    pub async fn download_file(
        &self,
        remote_path: &Path,
        local_path: &Path,
        progress_callback: Option<Box<dyn Fn(u64, u64) + Send>>,
    ) -> Result<()> {
        log::info!("SFTP: Downloading {:?} to {:?}", remote_path, local_path);
        let sftp = self.sftp()?;
        let remote_str = remote_path.to_string_lossy().into_owned();

        let mut remote_file = sftp.open(remote_str).await?;
        let attrs = remote_file.metadata().await?;
        let total_size = attrs.size.unwrap_or(0);

        let mut local_file = TokioFile::create(local_path).await?;

        let mut buffer = vec![0u8; 32 * 1024];
        let mut transferred = 0u64;
        loop {
            let n = remote_file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            local_file.write_all(&buffer[..n]).await?;
            transferred += n as u64;
            if let Some(ref cb) = progress_callback {
                cb(transferred, total_size);
            }
        }
        local_file.flush().await?;
        log::info!("SFTP: Download complete ({} bytes)", transferred);
        Ok(())
    }

    pub async fn upload_file(
        &self,
        local_path: &Path,
        remote_path: &Path,
        progress_callback: Option<Box<dyn Fn(u64, u64) + Send>>,
    ) -> Result<()> {
        log::info!("SFTP: Uploading {:?} to {:?}", local_path, remote_path);
        let sftp = self.sftp()?;
        let remote_str = remote_path.to_string_lossy().into_owned();

        let mut local_file = TokioFile::open(local_path).await?;
        let metadata = local_file.metadata().await?;
        let total_size = metadata.len();

        let mut remote_file = sftp.create(remote_str).await?;

        let mut buffer = vec![0u8; 32 * 1024];
        let mut transferred = 0u64;
        loop {
            let n = local_file.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            remote_file.write_all(&buffer[..n]).await?;
            transferred += n as u64;
            if let Some(ref cb) = progress_callback {
                cb(transferred, total_size);
            }
        }
        remote_file.flush().await?;
        remote_file.shutdown().await?;
        log::info!("SFTP: Upload complete ({} bytes)", transferred);
        Ok(())
    }

    pub async fn create_directory(&self, path: &Path) -> Result<()> {
        log::info!("SFTP: Creating directory {:?}", path);
        let sftp = self.sftp()?;
        let path_str = path.to_string_lossy().into_owned();
        sftp.create_dir(path_str).await?;
        Ok(())
    }

    pub async fn delete_file(&self, path: &Path) -> Result<()> {
        log::info!("SFTP: Deleting file {:?}", path);
        let sftp = self.sftp()?;
        let path_str = path.to_string_lossy().into_owned();
        sftp.remove_file(path_str).await?;
        Ok(())
    }

    pub async fn delete_directory(&self, path: &Path) -> Result<()> {
        log::info!("SFTP: Deleting directory {:?}", path);
        let sftp = self.sftp()?;
        let path_str = path.to_string_lossy().into_owned();
        sftp.remove_dir(path_str).await?;
        Ok(())
    }

    pub async fn rename(&self, old_path: &Path, new_path: &Path) -> Result<()> {
        log::info!("SFTP: Renaming {:?} to {:?}", old_path, new_path);
        let sftp = self.sftp()?;
        let old_str = old_path.to_string_lossy().into_owned();
        let new_str = new_path.to_string_lossy().into_owned();
        sftp.rename(old_str, new_str).await?;
        Ok(())
    }

    pub async fn stat(&self, path: &Path) -> Result<FileEntry> {
        log::debug!("SFTP: Getting stats for {:?}", path);
        let sftp = self.sftp()?;
        let path_str = path.to_string_lossy().into_owned();
        let attrs = sftp.metadata(path_str).await?;
        let parent = path.parent().unwrap_or(Path::new("/"));
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        Ok(file_entry_from(parent, name, &attrs))
    }

    pub async fn chmod(&self, path: &Path, mode: u32) -> Result<()> {
        log::info!("SFTP: Changing permissions of {:?} to {:o}", path, mode);
        let sftp = self.sftp()?;
        let path_str = path.to_string_lossy().into_owned();
        let attrs = FileAttributes {
            permissions: Some(mode),
            ..Default::default()
        };
        sftp.set_metadata(path_str, attrs).await?;
        Ok(())
    }

    pub fn current_path(&self) -> &Path {
        &self.current_path
    }

    pub fn change_directory(&mut self, path: PathBuf) {
        self.current_path = path;
    }
}

fn file_entry_from(parent: &Path, name: String, attrs: &FileAttributes) -> FileEntry {
    let path = parent.join(&name);
    let file_type = if attrs.is_dir() {
        FileType::Directory
    } else if attrs.is_symlink() {
        FileType::Symlink
    } else if attrs.is_regular() {
        FileType::File
    } else {
        FileType::Other
    };
    let modified = attrs
        .mtime
        .and_then(|t| chrono::DateTime::from_timestamp(t as i64, 0));
    FileEntry {
        name,
        path,
        file_type,
        size: attrs.size.unwrap_or(0),
        modified,
        permissions: attrs.permissions.unwrap_or(0),
        owner: attrs.user.clone().unwrap_or_default(),
        group: attrs.group.clone().unwrap_or_default(),
    }
}

#[derive(Debug, Clone)]
pub struct TransferTask {
    pub id: uuid::Uuid,
    pub local_path: PathBuf,
    pub remote_path: PathBuf,
    pub direction: TransferDirection,
    pub total_bytes: u64,
    pub transferred_bytes: u64,
    pub state: TransferState,
}

pub fn read_local_directory(path: &Path) -> Result<Vec<FileEntry>> {
    let mut entries = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().into_owned();

        let modified = metadata
            .modified()
            .ok()
            .map(chrono::DateTime::<chrono::Utc>::from);

        #[cfg(unix)]
        let permissions = {
            use std::os::unix::fs::PermissionsExt;
            metadata.permissions().mode()
        };
        #[cfg(not(unix))]
        let permissions: u32 = 0o644;

        let file_type = if metadata.is_dir() {
            FileType::Directory
        } else if metadata.file_type().is_symlink() {
            FileType::Symlink
        } else if metadata.is_file() {
            FileType::File
        } else {
            FileType::Other
        };

        entries.push(FileEntry {
            name,
            path: entry.path(),
            file_type,
            size: metadata.len(),
            modified,
            permissions,
            owner: String::new(),
            group: String::new(),
        });
    }
    Ok(entries)
}

pub fn create_local_directory(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

pub fn delete_local_path(path: &Path) -> Result<()> {
    if path.is_dir() {
        std::fs::remove_dir_all(path)?;
    } else {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

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
