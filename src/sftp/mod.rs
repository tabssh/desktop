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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_display() {
        assert_eq!(FileType::File.to_string(), "File");
        assert_eq!(FileType::Directory.to_string(), "Directory");
        assert_eq!(FileType::Symlink.to_string(), "Symlink");
        assert_eq!(FileType::Other.to_string(), "Other");
    }

    #[test]
    fn test_file_type_equality() {
        assert_eq!(FileType::File, FileType::File);
        assert_ne!(FileType::File, FileType::Directory);
        assert_ne!(FileType::Symlink, FileType::Other);
    }

    #[test]
    fn test_file_type_copy_semantics() {
        // FileType derives Copy, so using a value after assigning it
        // elsewhere must not be a move error.
        let a = FileType::Directory;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_transfer_direction_equality() {
        assert_eq!(TransferDirection::Upload, TransferDirection::Upload);
        assert_eq!(TransferDirection::Download, TransferDirection::Download);
        assert_ne!(TransferDirection::Upload, TransferDirection::Download);
    }

    #[test]
    fn test_transfer_state_equality() {
        assert_eq!(TransferState::Pending, TransferState::Pending);
        assert_eq!(TransferState::InProgress, TransferState::InProgress);
        assert_eq!(TransferState::Completed, TransferState::Completed);
        assert_eq!(TransferState::Cancelled, TransferState::Cancelled);
        assert_ne!(TransferState::Pending, TransferState::InProgress);
    }

    #[test]
    fn test_transfer_state_failed_payload_equality() {
        assert_eq!(
            TransferState::Failed("disk full".to_string()),
            TransferState::Failed("disk full".to_string())
        );
        assert_ne!(
            TransferState::Failed("disk full".to_string()),
            TransferState::Failed("permission denied".to_string())
        );
        // Different variants must never compare equal even with the same
        // string payload elsewhere.
        assert_ne!(
            TransferState::Failed(String::new()),
            TransferState::Cancelled
        );
    }

    #[test]
    fn test_transfer_state_failed_empty_string_boundary() {
        let state = TransferState::Failed(String::new());
        assert_eq!(state, TransferState::Failed(String::new()));
    }

    #[test]
    fn test_file_entry_construction_holds_given_values() {
        let entry = FileEntry {
            name: "file.txt".to_string(),
            path: std::path::PathBuf::from("/home/user/file.txt"),
            file_type: FileType::File,
            size: 1024,
            modified: None,
            permissions: 0o644,
            owner: "user".to_string(),
            group: "group".to_string(),
        };

        assert_eq!(entry.name, "file.txt");
        assert_eq!(entry.path, std::path::PathBuf::from("/home/user/file.txt"));
        assert_eq!(entry.file_type, FileType::File);
        assert_eq!(entry.size, 1024);
        assert!(entry.modified.is_none());
        assert_eq!(entry.permissions, 0o644);
        assert_eq!(entry.owner, "user");
        assert_eq!(entry.group, "group");
    }

    #[test]
    fn test_file_entry_clone_is_independent() {
        let entry = FileEntry {
            name: "a".to_string(),
            path: std::path::PathBuf::from("/a"),
            file_type: FileType::Directory,
            size: 0,
            modified: None,
            permissions: 0o755,
            owner: "root".to_string(),
            group: "root".to_string(),
        };
        let cloned = entry.clone();
        assert_eq!(cloned.name, entry.name);
        assert_eq!(cloned.file_type, entry.file_type);
    }
}
