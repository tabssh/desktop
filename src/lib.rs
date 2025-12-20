//! TabSSH Desktop Library
//!
//! This library provides the core SSH/SFTP functionality for TabSSH Desktop.

pub mod ssh;
pub mod sftp;
pub mod terminal;
pub mod storage;
pub mod crypto;
pub mod platform;
pub mod config;
pub mod ui;
pub mod utils;

// Re-export commonly used types
pub use ssh::{
    SshConnection, ConnectionConfig, SshConfigParser, HostConfig, 
    SessionManager, ActiveSession, SessionEvent, Credentials,
    AuthType
};
pub use sftp::{FileEntry, FileType, TransferDirection, TransferState};
pub use terminal::{TerminalEmulator, VtParser};
pub use storage::Database;
pub use config::{Settings, Theme};

