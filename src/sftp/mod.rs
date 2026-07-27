//! SFTP module - Secure file transfer

mod browser;
mod client;
pub mod transfer;

pub use browser::{SftpBrowser, SortColumn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
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

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: std::path::PathBuf,
    pub file_type: FileType,
    pub size: u64,
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    pub permissions: u32,
    pub owner: String,
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
