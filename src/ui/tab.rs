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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ssh_sets_fields() {
        let tab = Tab::new_ssh("example.com", "root", 22);
        assert_eq!(tab.host(), "example.com");
        assert_eq!(tab.user(), "root");
        assert_eq!(tab.port(), 22);
        assert_eq!(tab.title(), "root@example.com");
        assert_eq!(tab.status(), &TabStatus::Disconnected);
        assert!(!tab.has_unread());
    }

    #[test]
    fn test_ids_are_unique() {
        let a = Tab::new_ssh("host", "user", 22);
        let b = Tab::new_ssh("host", "user", 22);
        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn test_set_title() {
        let mut tab = Tab::new_ssh("host", "user", 22);
        tab.set_title("custom title".to_string());
        assert_eq!(tab.title(), "custom title");
    }

    #[test]
    fn test_set_title_empty_string() {
        let mut tab = Tab::new_ssh("host", "user", 22);
        tab.set_title(String::new());
        assert_eq!(tab.title(), "");
    }

    #[test]
    fn test_set_status() {
        let mut tab = Tab::new_ssh("host", "user", 22);
        tab.set_status(TabStatus::Connecting);
        assert_eq!(tab.status(), &TabStatus::Connecting);

        tab.set_status(TabStatus::Connected);
        assert_eq!(tab.status(), &TabStatus::Connected);

        tab.set_status(TabStatus::Error("boom".to_string()));
        assert_eq!(tab.status(), &TabStatus::Error("boom".to_string()));
    }

    #[test]
    fn test_mark_and_clear_unread() {
        let mut tab = Tab::new_ssh("host", "user", 22);
        assert!(!tab.has_unread());

        tab.mark_unread();
        assert!(tab.has_unread());

        // Idempotent: marking unread twice stays true
        tab.mark_unread();
        assert!(tab.has_unread());

        tab.clear_unread();
        assert!(!tab.has_unread());

        // Idempotent: clearing an already-clear flag stays false
        tab.clear_unread();
        assert!(!tab.has_unread());
    }

    #[test]
    fn test_tab_status_display() {
        assert_eq!(TabStatus::Disconnected.to_string(), "Disconnected");
        assert_eq!(TabStatus::Connecting.to_string(), "Connecting...");
        assert_eq!(TabStatus::Connected.to_string(), "Connected");
        assert_eq!(
            TabStatus::Error("timeout".to_string()).to_string(),
            "Error: timeout"
        );
    }

    #[test]
    fn test_port_boundary_values() {
        let min = Tab::new_ssh("h", "u", 0);
        assert_eq!(min.port(), 0);

        let max = Tab::new_ssh("h", "u", u16::MAX);
        assert_eq!(max.port(), u16::MAX);
    }

    #[test]
    fn test_tab_status_partial_eq() {
        assert_eq!(TabStatus::Disconnected, TabStatus::Disconnected);
        assert_ne!(TabStatus::Disconnected, TabStatus::Connecting);
        assert_ne!(
            TabStatus::Error("a".to_string()),
            TabStatus::Error("b".to_string())
        );
    }
}
