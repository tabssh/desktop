//! Main application state

use crate::ssh::SessionManager;
use crate::storage::database::Database;
use crate::storage::settings::Settings;
use crate::config::themes::ThemeManager;
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
            if self.active_tab >= self.tabs.len() && !self.tabs.is_empty(){
                self.active_tab = self.tabs.len() - 1;
            }
        }
    }
    
    pub fn next_tab(&mut self) {
        if !self.tabs.is_empty(){
            self.active_tab = (self.active_tab + 1) % self.tabs.len();
        }
    }
    
    pub fn previous_tab(&mut self) {
        if !self.tabs.is_empty(){
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
