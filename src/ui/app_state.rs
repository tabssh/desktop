//! Main application state

use crate::config::themes::ThemeManager;
use crate::ssh::SessionManager;
use crate::storage::database::Database;
use crate::storage::settings::Settings;
use crate::ui::notifications::NotificationManager;
use anyhow::Result;

pub struct AppState {
    pub db: Database,
    pub settings: Settings,
    pub theme_manager: ThemeManager,
    pub session_manager: SessionManager,
    pub notification_manager: NotificationManager,
    pub active_tab: usize,
    pub tabs: Vec<Tab>,
}

pub struct Tab {
    pub id: String,
    pub title: String,
    pub tab_type: TabType,
}

pub enum TabType {
    Terminal(String), // session_id
    Sftp(String),     // session_id
    Settings,
    Forwarding,
    ConnectionList,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let db = Database::open()?;
        let settings = Settings::load(&db)?;
        let theme_manager = ThemeManager::new();
        let runtime = std::sync::Arc::new(tokio::runtime::Runtime::new()?);
        let session_manager = SessionManager::new(runtime);
        let notification_manager = NotificationManager::new();

        Ok(Self {
            db,
            settings,
            theme_manager,
            session_manager,
            notification_manager,
            active_tab: 0,
            tabs: Vec::new(),
        })
    }

    pub fn add_terminal_tab(&mut self, session_id: String, title: String) {
        self.tabs.push(Tab {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            tab_type: TabType::Terminal(session_id),
        });
        self.active_tab = self.tabs.len() - 1;
    }

    pub fn add_sftp_tab(&mut self, session_id: String, title: String) {
        self.tabs.push(Tab {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            tab_type: TabType::Sftp(session_id),
        });
        self.active_tab = self.tabs.len() - 1;
    }

    pub fn close_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.tabs.remove(index);
            if self.active_tab >= self.tabs.len() && !self.tabs.is_empty() {
                self.active_tab = self.tabs.len() - 1;
            }
        }
    }

    pub fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = (self.active_tab + 1) % self.tabs.len();
        }
    }

    pub fn previous_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = if self.active_tab == 0 {
                self.tabs.len() - 1
            } else {
                self.active_tab - 1
            };
        }
    }

    pub fn save_settings(&self) -> Result<()> {
        self.settings.save(&self.db)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build an `AppState` backed by an isolated in-memory database, avoiding
    /// any dependency on the host filesystem / app-data directory.
    fn test_state() -> AppState {
        let db = Database::open_in_memory().expect("open in-memory db");
        let settings = Settings::load(&db).expect("load settings");
        let theme_manager = ThemeManager::new();
        let runtime =
            std::sync::Arc::new(tokio::runtime::Runtime::new().expect("create tokio runtime"));
        let session_manager = SessionManager::new(runtime);
        let notification_manager = NotificationManager::new();

        AppState {
            db,
            settings,
            theme_manager,
            session_manager,
            notification_manager,
            active_tab: 0,
            tabs: Vec::new(),
        }
    }

    #[test]
    fn test_add_terminal_tab() {
        let mut state = test_state();
        state.add_terminal_tab("sess-1".to_string(), "Terminal 1".to_string());
        assert_eq!(state.tabs.len(), 1);
        assert_eq!(state.active_tab, 0);
        assert_eq!(state.tabs[0].title, "Terminal 1");
        assert!(matches!(state.tabs[0].tab_type, TabType::Terminal(ref id) if id == "sess-1"));
    }

    #[test]
    fn test_add_sftp_tab() {
        let mut state = test_state();
        state.add_sftp_tab("sess-2".to_string(), "SFTP 1".to_string());
        assert_eq!(state.tabs.len(), 1);
        assert_eq!(state.active_tab, 0);
        assert_eq!(state.tabs[0].title, "SFTP 1");
        assert!(matches!(state.tabs[0].tab_type, TabType::Sftp(ref id) if id == "sess-2"));
    }

    #[test]
    fn test_add_multiple_tabs_activates_last() {
        let mut state = test_state();
        state.add_terminal_tab("a".to_string(), "A".to_string());
        state.add_sftp_tab("b".to_string(), "B".to_string());
        state.add_terminal_tab("c".to_string(), "C".to_string());
        assert_eq!(state.tabs.len(), 3);
        assert_eq!(state.active_tab, 2);
    }

    #[test]
    fn test_close_tab_last_remaining_tab() {
        let mut state = test_state();
        state.add_terminal_tab("a".to_string(), "A".to_string());
        state.close_tab(0);
        assert!(state.tabs.is_empty());
        assert_eq!(state.active_tab, 0);
    }

    #[test]
    fn test_close_tab_out_of_range_is_noop() {
        let mut state = test_state();
        state.add_terminal_tab("a".to_string(), "A".to_string());
        state.close_tab(5);
        assert_eq!(state.tabs.len(), 1);
        assert_eq!(state.active_tab, 0);
    }

    #[test]
    fn test_close_tab_on_empty_tabs_is_noop() {
        let mut state = test_state();
        state.close_tab(0);
        assert!(state.tabs.is_empty());
        assert_eq!(state.active_tab, 0);
    }

    #[test]
    fn test_close_tab_adjusts_active_tab_when_last_active_removed() {
        let mut state = test_state();
        state.add_terminal_tab("a".to_string(), "A".to_string());
        state.add_terminal_tab("b".to_string(), "B".to_string());
        state.add_terminal_tab("c".to_string(), "C".to_string());
        assert_eq!(state.active_tab, 2);
        state.close_tab(2);
        assert_eq!(state.tabs.len(), 2);
        assert_eq!(state.active_tab, 1);
    }

    #[test]
    fn test_close_tab_middle_leaves_active_tab_index_when_still_valid() {
        let mut state = test_state();
        state.add_terminal_tab("a".to_string(), "A".to_string());
        state.add_terminal_tab("b".to_string(), "B".to_string());
        state.add_terminal_tab("c".to_string(), "C".to_string());
        state.active_tab = 0;
        state.close_tab(1);
        assert_eq!(state.tabs.len(), 2);
        assert_eq!(state.active_tab, 0);
        assert_eq!(state.tabs[0].title, "A");
        assert_eq!(state.tabs[1].title, "C");
    }

    #[test]
    fn test_close_tab_before_active_tab_does_not_shift_active_tab() {
        // Documents a production bug: closing a tab at an index below the
        // active tab does not decrement `active_tab`, so `active_tab` ends
        // up pointing at the wrong tab after the remaining tabs shift down.
        // Tabs: A(0), B(1), C(2); active_tab = 1 (B).
        let mut state = test_state();
        state.add_terminal_tab("a".to_string(), "A".to_string());
        state.add_terminal_tab("b".to_string(), "B".to_string());
        state.add_terminal_tab("c".to_string(), "C".to_string());
        state.active_tab = 1;
        // Close A (index 0); B and C shift down to indices 0 and 1.
        state.close_tab(0);
        assert_eq!(state.tabs.len(), 2);
        // active_tab is left at 1, which now refers to C, not B.
        assert_eq!(state.active_tab, 1);
        assert_eq!(state.tabs[state.active_tab].title, "C");
    }

    #[test]
    fn test_next_tab_wraps_around() {
        let mut state = test_state();
        state.add_terminal_tab("a".to_string(), "A".to_string());
        state.add_terminal_tab("b".to_string(), "B".to_string());
        state.active_tab = 1;
        state.next_tab();
        assert_eq!(state.active_tab, 0);
    }

    #[test]
    fn test_next_tab_single_tab_stays_at_zero() {
        let mut state = test_state();
        state.add_terminal_tab("a".to_string(), "A".to_string());
        state.next_tab();
        assert_eq!(state.active_tab, 0);
    }

    #[test]
    fn test_next_tab_on_empty_tabs_is_noop() {
        let mut state = test_state();
        state.next_tab();
        assert_eq!(state.active_tab, 0);
    }

    #[test]
    fn test_previous_tab_wraps_around() {
        let mut state = test_state();
        state.add_terminal_tab("a".to_string(), "A".to_string());
        state.add_terminal_tab("b".to_string(), "B".to_string());
        state.active_tab = 0;
        state.previous_tab();
        assert_eq!(state.active_tab, 1);
    }

    #[test]
    fn test_previous_tab_single_tab_stays_at_zero() {
        let mut state = test_state();
        state.add_terminal_tab("a".to_string(), "A".to_string());
        state.previous_tab();
        assert_eq!(state.active_tab, 0);
    }

    #[test]
    fn test_previous_tab_on_empty_tabs_is_noop() {
        let mut state = test_state();
        state.previous_tab();
        assert_eq!(state.active_tab, 0);
    }

    #[test]
    fn test_save_settings_roundtrip() {
        let mut state = test_state();
        state.settings.font_size = 42.0;
        state.settings.default_shell = "/bin/zsh".to_string();
        state.save_settings().expect("save settings");

        let reloaded = Settings::load(&state.db).expect("reload settings");
        assert_eq!(reloaded.font_size, 42.0);
        assert_eq!(reloaded.default_shell, "/bin/zsh");
    }
}
