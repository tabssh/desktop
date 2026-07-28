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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::Tab;

    fn make_tab() -> Tab {
        Tab::new_ssh("host", "user", 22)
    }

    #[test]
    fn test_new_manager_is_empty() {
        let manager = TabManager::new();
        assert_eq!(manager.tab_count(), 0);
        assert!(manager.tabs().is_empty());
        assert_eq!(manager.active_tab_id(), None);
        assert!(manager.active_tab().is_none());
    }

    #[test]
    fn test_default_matches_new() {
        let manager = TabManager::default();
        assert_eq!(manager.tab_count(), 0);
        assert_eq!(manager.active_tab_id(), None);
    }

    #[test]
    fn test_add_tab_becomes_active() {
        let mut manager = TabManager::new();
        let tab = make_tab();
        let id = tab.id();
        manager.add_tab(tab);

        assert_eq!(manager.tab_count(), 1);
        assert_eq!(manager.active_tab_id(), Some(id));
        assert_eq!(manager.active_tab().unwrap().id(), id);
    }

    #[test]
    fn test_add_multiple_tabs_last_added_is_active() {
        let mut manager = TabManager::new();
        manager.add_tab(make_tab());
        let second = make_tab();
        let second_id = second.id();
        manager.add_tab(second);

        assert_eq!(manager.tab_count(), 2);
        assert_eq!(manager.active_tab_id(), Some(second_id));
    }

    #[test]
    fn test_close_only_tab_leaves_manager_empty() {
        let mut manager = TabManager::new();
        let tab = make_tab();
        let id = tab.id();
        manager.add_tab(tab);

        manager.close_tab(id);

        assert_eq!(manager.tab_count(), 0);
        assert_eq!(manager.active_tab_id(), None);
        assert!(manager.active_tab().is_none());
    }

    #[test]
    fn test_close_tab_is_idempotent() {
        let mut manager = TabManager::new();
        let tab = make_tab();
        let id = tab.id();
        manager.add_tab(tab);

        manager.close_tab(id);
        // Closing an already-closed tab must be a no-op, not a panic.
        manager.close_tab(id);

        assert_eq!(manager.tab_count(), 0);
    }

    #[test]
    fn test_close_tab_unknown_id_is_noop() {
        let mut manager = TabManager::new();
        manager.add_tab(make_tab());

        manager.close_tab(Uuid::new_v4());

        assert_eq!(manager.tab_count(), 1);
    }

    #[test]
    fn test_close_active_tab_falls_back_to_new_last_tab() {
        let mut manager = TabManager::new();
        let first = make_tab();
        let first_id = first.id();
        manager.add_tab(first);
        let second = make_tab();
        let second_id = second.id();
        manager.add_tab(second);

        // second is active; closing it should fall back to first
        manager.close_tab(second_id);

        assert_eq!(manager.tab_count(), 1);
        assert_eq!(manager.active_tab_id(), Some(first_id));
    }

    #[test]
    fn test_close_inactive_tab_keeps_active_unchanged() {
        let mut manager = TabManager::new();
        let first = make_tab();
        let first_id = first.id();
        manager.add_tab(first);
        let second = make_tab();
        let second_id = second.id();
        manager.add_tab(second);
        // second is active

        manager.close_tab(first_id);

        assert_eq!(manager.tab_count(), 1);
        assert_eq!(manager.active_tab_id(), Some(second_id));
    }

    #[test]
    fn test_set_active_tab_clears_unread() {
        let mut manager = TabManager::new();
        let mut tab = make_tab();
        tab.mark_unread();
        let id = tab.id();
        manager.add_tab(tab);

        // Switch away then back so we exercise the clear-unread path.
        manager.set_active_tab(id);

        assert!(!manager.get_tab(id).unwrap().has_unread());
        assert_eq!(manager.active_tab_id(), Some(id));
    }

    #[test]
    fn test_set_active_tab_unknown_id_is_noop() {
        let mut manager = TabManager::new();
        let tab = make_tab();
        let id = tab.id();
        manager.add_tab(tab);

        manager.set_active_tab(Uuid::new_v4());

        // active tab should remain the original one
        assert_eq!(manager.active_tab_id(), Some(id));
    }

    #[test]
    fn test_next_tab_on_empty_manager_is_noop() {
        let mut manager = TabManager::new();
        manager.next_tab();
        assert_eq!(manager.active_tab_id(), None);
    }

    #[test]
    fn test_previous_tab_on_empty_manager_is_noop() {
        let mut manager = TabManager::new();
        manager.previous_tab();
        assert_eq!(manager.active_tab_id(), None);
    }

    #[test]
    fn test_next_tab_wraps_around() {
        let mut manager = TabManager::new();
        let first = make_tab();
        let first_id = first.id();
        manager.add_tab(first);
        let second = make_tab();
        let second_id = second.id();
        manager.add_tab(second);
        // second is active

        manager.next_tab();
        assert_eq!(manager.active_tab_id(), Some(first_id));

        manager.next_tab();
        assert_eq!(manager.active_tab_id(), Some(second_id));
    }

    #[test]
    fn test_previous_tab_wraps_around() {
        let mut manager = TabManager::new();
        let first = make_tab();
        let first_id = first.id();
        manager.add_tab(first);
        let second = make_tab();
        let second_id = second.id();
        manager.add_tab(second);
        // second is active

        manager.previous_tab();
        assert_eq!(manager.active_tab_id(), Some(first_id));

        manager.previous_tab();
        assert_eq!(manager.active_tab_id(), Some(second_id));
    }

    #[test]
    fn test_next_tab_single_tab_stays_active() {
        let mut manager = TabManager::new();
        let tab = make_tab();
        let id = tab.id();
        manager.add_tab(tab);

        manager.next_tab();
        assert_eq!(manager.active_tab_id(), Some(id));
    }

    #[test]
    fn test_set_active_tab_by_index() {
        let mut manager = TabManager::new();
        let first = make_tab();
        let first_id = first.id();
        manager.add_tab(first);
        manager.add_tab(make_tab());

        manager.set_active_tab_by_index(0);
        assert_eq!(manager.active_tab_id(), Some(first_id));
    }

    #[test]
    fn test_set_active_tab_by_index_out_of_range_is_noop() {
        let mut manager = TabManager::new();
        let tab = make_tab();
        let id = tab.id();
        manager.add_tab(tab);

        manager.set_active_tab_by_index(99);

        // active tab remains unchanged
        assert_eq!(manager.active_tab_id(), Some(id));
    }

    #[test]
    fn test_get_tab_and_get_tab_mut() {
        let mut manager = TabManager::new();
        let tab = make_tab();
        let id = tab.id();
        manager.add_tab(tab);

        assert!(manager.get_tab(id).is_some());
        assert!(manager.get_tab(Uuid::new_v4()).is_none());

        manager
            .get_tab_mut(id)
            .unwrap()
            .set_title("renamed".to_string());
        assert_eq!(manager.get_tab(id).unwrap().title(), "renamed");

        assert!(manager.get_tab_mut(Uuid::new_v4()).is_none());
    }

    #[test]
    fn test_active_tab_mut_none_when_empty() {
        let mut manager = TabManager::new();
        assert!(manager.active_tab_mut().is_none());
    }

    #[test]
    fn test_active_tab_mut_modifies_active_tab() {
        let mut manager = TabManager::new();
        manager.add_tab(make_tab());

        manager
            .active_tab_mut()
            .unwrap()
            .set_title("edited".to_string());

        assert_eq!(manager.active_tab().unwrap().title(), "edited");
    }
}
