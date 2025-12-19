//! SFTP client implementation
//!
//! This module provides SFTP file operations. The actual SFTP session
//! integration with russh will be completed when connecting to SSH sessions.

use super::{FileEntry, FileType, TransferDirection, TransferState};
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};

/// Transfer task for tracking file transfers
#[derive(Debug, Clone)]
pub struct TransferTask {
    pub id: uuid::Uuid,
    pub direction: TransferDirection,
    pub local_path: PathBuf,
    pub remote_path: PathBuf,
    pub file_name: String,
    pub total_bytes: u64,
    pub transferred_bytes: u64,
    pub state: TransferState,
}

impl TransferTask {
    pub fn new_download(remote_path: PathBuf, local_path: PathBuf, size: u64) -> Self {
        let file_name = remote_path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Self {
            id: uuid::Uuid::new_v4(),
            direction: TransferDirection::Download,
            local_path,
            remote_path,
            file_name,
            total_bytes: size,
            transferred_bytes: 0,
            state: TransferState::Pending,
        }
    }

    pub fn new_upload(local_path: PathBuf, remote_path: PathBuf, size: u64) -> Self {
        let file_name = local_path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Self {
            id: uuid::Uuid::new_v4(),
            direction: TransferDirection::Upload,
            local_path,
            remote_path,
            file_name,
            total_bytes: size,
            transferred_bytes: 0,
            state: TransferState::Pending,
        }
    }

    pub fn progress_percent(&self) -> f32 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        (self.transferred_bytes as f32 / self.total_bytes as f32) * 100.0
    }
}

/// Read local directory entries
pub fn read_local_directory(path: &Path) -> Result<Vec<FileEntry>> {
    let mut entries = Vec::new();

    let read_dir = std::fs::read_dir(path)?;

    for entry in read_dir.flatten() {
        let metadata = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().to_string();

        let file_type = if metadata.is_dir() {
            FileType::Directory
        } else if metadata.file_type().is_symlink() {
            FileType::Symlink
        } else if metadata.is_file() {
            FileType::File
        } else {
            FileType::Other
        };

        let modified = metadata.modified().ok().map(|t| {
            DateTime::<Utc>::from(t)
        });

        #[cfg(unix)]
        let permissions = {
            use std::os::unix::fs::PermissionsExt;
            metadata.permissions().mode()
        };
        #[cfg(not(unix))]
        let permissions = 0o644;

        entries.push(FileEntry {
            name,
            file_type,
            size: metadata.len(),
            modified,
            permissions,
            owner: String::new(),
            group: String::new(),
        });
    }

    entries.sort_by(|a, b| {
        let type_order = |t: &FileType| match t {
            FileType::Directory => 0,
            FileType::Symlink => 1,
            FileType::File => 2,
            FileType::Other => 3,
        };

        let type_cmp = type_order(&a.file_type).cmp(&type_order(&b.file_type));
        if type_cmp != std::cmp::Ordering::Equal {
            return type_cmp;
        }
        a.name.to_lowercase().cmp(&b.name.to_lowercase())
    });

    Ok(entries)
}

/// Create a local directory
pub fn create_local_directory(path: &Path) -> Result<()> {
    std::fs::create_dir(path)?;
    Ok(())
}

/// Delete a local file or directory
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
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Format unix permissions for display
pub fn format_permissions(mode: u32) -> String {
    let mut result = String::with_capacity(10);

    let file_type = if mode & 0o40000 != 0 {
        'd'
    } else if mode & 0o120000 == 0o120000 {
        'l'
    } else {
        '-'
    };
    result.push(file_type);

    for shift in [6, 3, 0] {
        let bits = (mode >> shift) & 0o7;
        result.push(if bits & 4 != 0 { 'r' } else { '-' });
        result.push(if bits & 2 != 0 { 'w' } else { '-' });
        result.push(if bits & 1 != 0 { 'x' } else { '-' });
    }

    result
}
