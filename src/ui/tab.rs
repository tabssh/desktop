//! Tab representation for SSH sessions

#![allow(dead_code)]

use uuid::Uuid;

/// Connection status for a tab
#[derive(Debug, Clone, PartialEq)]
pub enum TabStatus {
    /// Not connected
    Disconnected,
    /// Currently connecting
    Connecting,
    /// Connected and active
    Connected,
    /// Connection error
    Error(String),
}

impl std::fmt::Display for TabStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TabStatus::Disconnected => write!(f, "Disconnected"),
            TabStatus::Connecting => write!(f, "Connecting..."),
            TabStatus::Connected => write!(f, "Connected"),
            TabStatus::Error(e) => write!(f, "Error: {}", e),
        }
    }
}

/// A tab representing an SSH session
#[derive(Debug, Clone)]
pub struct Tab {
    /// Unique identifier for this tab
    id: Uuid,

    /// SSH host
    host: String,

    /// SSH username
    user: String,

    /// SSH port
    port: u16,

    /// Tab title (displayed in tab bar)
    title: String,

    /// Connection status
    status: TabStatus,

    /// Has unread output since last view
    has_unread: bool,
}

impl Tab {
    /// Create a new SSH tab
    pub fn new_ssh(host: &str, user: &str, port: u16) -> Self {
        let title = format!("{}@{}", user, host);
        Self {
            id: Uuid::new_v4(),
            host: host.to_string(),
            user: user.to_string(),
            port,
            title,
            status: TabStatus::Disconnected,
            has_unread: false,
        }
    }

    /// Get the tab's unique identifier
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Get the tab title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set the tab title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Get the SSH host
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Get the SSH username
    pub fn user(&self) -> &str {
        &self.user
    }

    /// Get the SSH port
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Get the connection status
    pub fn status(&self) -> &TabStatus {
        &self.status
    }

    /// Set the connection status
    pub fn set_status(&mut self, status: TabStatus) {
        self.status = status;
    }

    /// Check if tab has unread output
    pub fn has_unread(&self) -> bool {
        self.has_unread
    }

    /// Mark tab as having unread output
    pub fn mark_unread(&mut self) {
        self.has_unread = true;
    }

    /// Clear unread flag
    pub fn clear_unread(&mut self) {
        self.has_unread = false;
    }
}
