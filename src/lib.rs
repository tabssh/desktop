//! TabSSH Desktop Library
//!
//! This library provides the core SSH/SFTP functionality for TabSSH Desktop.

pub mod assets;
pub mod config;
pub mod crypto;
pub mod platform;
pub mod sftp;
pub mod ssh;
pub mod storage;
pub mod terminal;
pub mod ui;
pub mod utils;

// Re-export commonly used types
pub use config::{AppConfig, Theme};
pub use sftp::{FileEntry, FileType, TransferDirection, TransferState};
pub use ssh::{
    ActiveSession, AuthType, ConnectionConfig, Credentials, HostConfig, SessionEvent,
    SessionManager, SshConfigParser, SshConnection,
};
pub use storage::Database;
pub use terminal::{TerminalEmulator, VtParser};
