//! SFTP module - Secure file transfer

#![allow(dead_code)]

mod client;
mod browser;
mod operations;
mod transfer;

#[allow(unused_imports)]
pub use client::{
    SftpClient,
    TransferTask,
    read_local_directory,
    create_local_directory,
    delete_local_path,
    format_file_size,
    format_permissions,
};

pub use browser::{SftpBrowser, SortColumn};
pub use operations::SftpOperations;
pub use transfer::TransferManager;

/// File entry type
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    /// Regular file
    File,
    /// Directory
    Directory,
    /// Symbolic link
    Symlink,
    /// Other (device, socket, etc.)
    Other,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::File => write!(f, "File"),
            FileType::Directory => write!(f, "Directory"),
            FileType::Symlink => write!(f, "Symlink"),
            FileType::Other => write!(f, "Other"),
        }
    }
}

/// Remote file entry
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// File name
    pub name: String,
    /// File type
    pub file_type: FileType,
    /// File size in bytes
    pub size: u64,
    /// Last modified timestamp
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    /// Unix permissions
    pub permissions: u32,
    /// Owner name
    pub owner: String,
    /// Group name
    pub group: String,
}

/// Transfer progress callback
pub type ProgressCallback = Box<dyn Fn(u64, u64) + Send + Sync>;

/// Transfer direction
#[derive(Debug, Clone, PartialEq)]
pub enum TransferDirection {
    Upload,
    Download,
}

/// Transfer state
#[derive(Debug, Clone, PartialEq)]
pub enum TransferState {
    Pending,
    InProgress,
    Completed,
    Failed(String),
    Cancelled,
}
