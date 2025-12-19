//! Tab manager for handling multiple SSH sessions

#![allow(dead_code)]

use super::Tab;
use uuid::Uuid;

/// Manages multiple tabs
pub struct TabManager {
    /// All open tabs
    tabs: Vec<Tab>,

    /// Currently active tab ID
    active_tab_id: Option<Uuid>,
}

impl TabManager {
    /// Create a new tab manager
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab_id: None,
        }
    }

    /// Get all tabs
    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    /// Get number of tabs
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Get the active tab ID
    pub fn active_tab_id(&self) -> Option<Uuid> {
        self.active_tab_id
    }

    /// Get the active tab
    pub fn active_tab(&self) -> Option<&Tab> {
        self.active_tab_id
            .and_then(|id| self.tabs.iter().find(|t| t.id() == id))
    }

    /// Get the active tab mutably
    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        let id = self.active_tab_id?;
        self.tabs.iter_mut().find(|t| t.id() == id)
    }

    /// Add a new tab and make it active
    pub fn add_tab(&mut self, tab: Tab) {
        let id = tab.id();
        self.tabs.push(tab);
        self.active_tab_id = Some(id);
        log::info!("Added new tab, total tabs: {}", self.tabs.len());
    }

    /// Close a tab by ID
    pub fn close_tab(&mut self, id: Uuid) {
        if let Some(pos) = self.tabs.iter().position(|t| t.id() == id) {
            self.tabs.remove(pos);
            log::info!("Closed tab, remaining tabs: {}", self.tabs.len());

            // Update active tab
            if self.active_tab_id == Some(id) {
                self.active_tab_id = if self.tabs.is_empty() {
                    None
                } else if pos >= self.tabs.len() {
                    Some(self.tabs[self.tabs.len() - 1].id())
                } else {
                    Some(self.tabs[pos].id())
                };
            }
        }
    }

    /// Set the active tab by ID
    pub fn set_active_tab(&mut self, id: Uuid) {
        if self.tabs.iter().any(|t| t.id() == id) {
            // Clear unread flag when switching to tab
            if let Some(tab) = self.tabs.iter_mut().find(|t| t.id() == id) {
                tab.clear_unread();
            }
            self.active_tab_id = Some(id);
        }
    }

    /// Switch to the next tab
    pub fn next_tab(&mut self) {
        if self.tabs.is_empty() {
            return;
        }

        let current_pos = self
            .active_tab_id
            .and_then(|id| self.tabs.iter().position(|t| t.id() == id))
            .unwrap_or(0);

        let next_pos = (current_pos + 1) % self.tabs.len();
        self.active_tab_id = Some(self.tabs[next_pos].id());
    }

    /// Switch to the previous tab
    pub fn previous_tab(&mut self) {
        if self.tabs.is_empty() {
            return;
        }

        let current_pos = self
            .active_tab_id
            .and_then(|id| self.tabs.iter().position(|t| t.id() == id))
            .unwrap_or(0);

        let prev_pos = if current_pos == 0 {
            self.tabs.len() - 1
        } else {
            current_pos - 1
        };
        self.active_tab_id = Some(self.tabs[prev_pos].id());
    }

    /// Set active tab by index (0-based)
    pub fn set_active_tab_by_index(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab_id = Some(self.tabs[index].id());
        }
    }

    /// Get a tab by ID
    pub fn get_tab(&self, id: Uuid) -> Option<&Tab> {
        self.tabs.iter().find(|t| t.id() == id)
    }

    /// Get a tab mutably by ID
    pub fn get_tab_mut(&mut self, id: Uuid) -> Option<&mut Tab> {
        self.tabs.iter_mut().find(|t| t.id() == id)
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}
