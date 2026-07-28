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

#[cfg(test)]
mod tests {
    use super::*;

    // -- format_file_size --------------------------------------------------

    #[test]
    fn test_format_file_size_zero_bytes() {
        assert_eq!(format_file_size(0), "0 B");
    }

    #[test]
    fn test_format_file_size_bytes_stay_bytes() {
        assert_eq!(format_file_size(512), "512 B");
    }

    #[test]
    fn test_format_file_size_exact_kb_boundary() {
        assert_eq!(format_file_size(1024), "1.00 KB");
    }

    #[test]
    fn test_format_file_size_just_below_kb_boundary() {
        assert_eq!(format_file_size(1023), "1023 B");
    }

    #[test]
    fn test_format_file_size_mb() {
        assert_eq!(format_file_size(5 * 1024 * 1024), "5.00 MB");
    }

    #[test]
    fn test_format_file_size_caps_at_largest_unit() {
        // u64::MAX is far beyond TB but the unit table must not overrun.
        let result = format_file_size(u64::MAX);
        assert!(result.ends_with(" TB"));
    }

    // -- format_permissions --------------------------------------------------

    #[test]
    fn test_format_permissions_zero_mode() {
        assert_eq!(format_permissions(0), "---------");
    }

    #[test]
    fn test_format_permissions_full_mode() {
        assert_eq!(format_permissions(0o777), "rwxrwxrwx");
    }

    #[test]
    fn test_format_permissions_common_mode_644() {
        assert_eq!(format_permissions(0o644), "rw-r--r--");
    }

    #[test]
    fn test_format_permissions_common_mode_755() {
        assert_eq!(format_permissions(0o755), "rwxr-xr-x");
    }

    #[test]
    fn test_format_permissions_ignores_bits_above_mode() {
        // Extra high bits (e.g. setuid/setgid/sticky or file-type bits)
        // must not leak into the rwx string.
        assert_eq!(format_permissions(0o100644), "rw-r--r--");
    }

    // -- file_entry_from -----------------------------------------------------

    #[test]
    fn test_file_entry_from_directory() {
        let mut attrs = FileAttributes::default();
        attrs.set_dir(true);
        attrs.size = Some(0);

        let entry = file_entry_from(Path::new("/home"), "sub".to_string(), &attrs);

        assert_eq!(entry.name, "sub");
        assert_eq!(entry.path, PathBuf::from("/home/sub"));
        assert_eq!(entry.file_type, FileType::Directory);
        assert_eq!(entry.size, 0);
    }

    #[test]
    fn test_file_entry_from_regular_file() {
        // FileAttributes::default() flags a directory (DIR bit + 0o777)
        // by default, so permissions must be reset to 0 first, or the
        // leftover DIR bit would make is_dir() win over is_regular() in
        // file_entry_from's type-detection order. The permissions field
        // packs the file-type bits and the mode together (as real SFTP
        // servers do), so the expected value includes the REG bit.
        let mut attrs = FileAttributes {
            permissions: Some(0),
            ..FileAttributes::default()
        };
        attrs.set_regular(true);
        attrs.permissions = attrs.permissions.map(|p| p | 0o644);
        attrs.size = Some(1234);
        attrs.user = Some("alice".to_string());
        attrs.group = Some("staff".to_string());

        let entry = file_entry_from(Path::new("/data"), "report.txt".to_string(), &attrs);

        assert_eq!(entry.file_type, FileType::File);
        assert_eq!(entry.size, 1234);
        assert_eq!(entry.permissions, 0o100644);
        assert_eq!(entry.owner, "alice");
        assert_eq!(entry.group, "staff");
    }

    #[test]
    fn test_file_entry_from_symlink() {
        // Reset permissions to 0 first: FileAttributes::default() sets the
        // directory type bit, which would otherwise take priority over the
        // symlink bit in file_entry_from's type-detection order.
        let mut attrs = FileAttributes {
            permissions: Some(0),
            ..FileAttributes::default()
        };
        attrs.set_symlink(true);

        let entry = file_entry_from(Path::new("/data"), "link".to_string(), &attrs);
        assert_eq!(entry.file_type, FileType::Symlink);
    }

    #[test]
    fn test_file_entry_from_other_type() {
        // No type bit set at all (not dir/regular/symlink) must fall back
        // to FileType::Other rather than panicking or misclassifying.
        // FileAttributes::default() sets the directory bit, so permissions
        // must be explicitly cleared to represent an untyped entry.
        let attrs = FileAttributes {
            permissions: Some(0),
            ..FileAttributes::default()
        };
        let entry = file_entry_from(Path::new("/dev"), "null".to_string(), &attrs);
        assert_eq!(entry.file_type, FileType::Other);
    }

    #[test]
    fn test_file_entry_from_missing_optional_fields_default_to_zero_or_empty() {
        // FileAttributes::default() sets size/permissions/mtime to non-zero
        // defaults (a directory with 0o777 perms and an epoch mtime), so
        // they must be explicitly overridden to exercise the true
        // missing/zero-value fallback paths in file_entry_from.
        let attrs = FileAttributes {
            size: Some(0),
            permissions: Some(0),
            mtime: None,
            ..FileAttributes::default()
        };
        let entry = file_entry_from(Path::new("/"), "x".to_string(), &attrs);
        assert_eq!(entry.size, 0);
        assert_eq!(entry.permissions, 0);
        assert_eq!(entry.owner, "");
        assert_eq!(entry.group, "");
        assert!(entry.modified.is_none());
    }

    #[test]
    fn test_file_entry_from_unicode_name() {
        let attrs = FileAttributes::default();
        let entry = file_entry_from(Path::new("/home"), "写真.png".to_string(), &attrs);
        assert_eq!(entry.name, "写真.png");
        assert_eq!(entry.path, PathBuf::from("/home/写真.png"));
    }

    #[test]
    fn test_file_entry_from_empty_name() {
        let attrs = FileAttributes::default();
        let entry = file_entry_from(Path::new("/home"), String::new(), &attrs);
        assert_eq!(entry.name, "");
        assert_eq!(entry.path, PathBuf::from("/home"));
    }

    #[test]
    fn test_file_entry_from_mtime_conversion() {
        let attrs = FileAttributes {
            mtime: Some(0),
            ..Default::default()
        };
        let entry = file_entry_from(Path::new("/"), "x".to_string(), &attrs);
        assert_eq!(entry.modified, chrono::DateTime::from_timestamp(0, 0));
    }

    // -- SftpClient ------------------------------------------------------------

    #[test]
    fn test_new_client_starts_at_root_with_no_session() {
        let client = SftpClient::new("session-1".to_string());
        assert_eq!(client.current_path(), Path::new("/"));
        assert!(client.sftp().is_err());
    }

    #[test]
    fn test_change_directory_updates_current_path() {
        let mut client = SftpClient::new("session-1".to_string());
        client.change_directory(PathBuf::from("/home/user"));
        assert_eq!(client.current_path(), Path::new("/home/user"));
    }

    #[test]
    fn test_change_directory_to_empty_path() {
        let mut client = SftpClient::new("session-1".to_string());
        client.change_directory(PathBuf::new());
        assert_eq!(client.current_path(), Path::new(""));
    }

    #[test]
    fn test_sftp_not_connected_error_message() {
        let client = SftpClient::new("session-1".to_string());
        let err = match client.sftp() {
            Err(e) => e,
            Ok(_) => panic!("expected sftp() to return an error"),
        };
        assert_eq!(err.to_string(), "SFTP not connected");
    }

    // -- TransferTask ------------------------------------------------------------

    #[test]
    fn test_transfer_task_construction() {
        let task = TransferTask {
            id: uuid::Uuid::nil(),
            local_path: PathBuf::from("/local/file"),
            remote_path: PathBuf::from("/remote/file"),
            direction: TransferDirection::Upload,
            total_bytes: 100,
            transferred_bytes: 0,
            state: TransferState::Pending,
        };
        assert_eq!(task.direction, TransferDirection::Upload);
        assert_eq!(task.state, TransferState::Pending);
        assert_eq!(task.total_bytes, 100);
        assert_eq!(task.transferred_bytes, 0);
    }

    // -- local filesystem helpers (local disk only, no network) -----------------

    #[test]
    fn test_create_and_read_local_directory() {
        let dir = std::env::temp_dir().join(format!("tabssh_test_sftp_{}", uuid::Uuid::new_v4()));
        create_local_directory(&dir).unwrap();
        std::fs::write(dir.join("a.txt"), b"hello").unwrap();
        create_local_directory(&dir.join("subdir")).unwrap();

        let entries = read_local_directory(&dir).unwrap();
        std::fs::remove_dir_all(&dir).ok();

        assert_eq!(entries.len(), 2);
        let file_entry = entries.iter().find(|e| e.name == "a.txt").unwrap();
        assert_eq!(file_entry.file_type, FileType::File);
        assert_eq!(file_entry.size, 5);
        let dir_entry = entries.iter().find(|e| e.name == "subdir").unwrap();
        assert_eq!(dir_entry.file_type, FileType::Directory);
    }

    #[test]
    fn test_read_local_directory_empty() {
        let dir =
            std::env::temp_dir().join(format!("tabssh_test_sftp_empty_{}", uuid::Uuid::new_v4()));
        create_local_directory(&dir).unwrap();

        let entries = read_local_directory(&dir).unwrap();
        std::fs::remove_dir_all(&dir).ok();

        assert!(entries.is_empty());
    }

    #[test]
    fn test_read_local_directory_missing_path_errors() {
        let dir =
            std::env::temp_dir().join(format!("tabssh_test_sftp_missing_{}", uuid::Uuid::new_v4()));
        assert!(read_local_directory(&dir).is_err());
    }

    #[test]
    fn test_create_local_directory_is_idempotent() {
        let dir =
            std::env::temp_dir().join(format!("tabssh_test_sftp_mkdir_{}", uuid::Uuid::new_v4()));
        create_local_directory(&dir).unwrap();
        // Creating it again must not error (create_dir_all semantics).
        create_local_directory(&dir).unwrap();
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_delete_local_path_removes_file() {
        let dir =
            std::env::temp_dir().join(format!("tabssh_test_sftp_del_{}", uuid::Uuid::new_v4()));
        create_local_directory(&dir).unwrap();
        let file = dir.join("f.txt");
        std::fs::write(&file, b"data").unwrap();

        delete_local_path(&file).unwrap();
        assert!(!file.exists());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_delete_local_path_removes_directory_recursively() {
        let dir =
            std::env::temp_dir().join(format!("tabssh_test_sftp_deldir_{}", uuid::Uuid::new_v4()));
        create_local_directory(&dir.join("nested")).unwrap();
        std::fs::write(dir.join("nested").join("f.txt"), b"data").unwrap();

        delete_local_path(&dir).unwrap();
        assert!(!dir.exists());
    }

    #[test]
    fn test_delete_local_path_missing_path_errors() {
        let path =
            std::env::temp_dir().join(format!("tabssh_test_sftp_nope_{}", uuid::Uuid::new_v4()));
        assert!(delete_local_path(&path).is_err());
    }
}
