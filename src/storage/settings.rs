//! Settings persistence

use anyhow::Result;
use serde::{Deserialize, Serialize};
use super::database::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    // General
    pub default_shell: String,
    pub auto_connect_on_startup: bool,
    pub restore_previous_sessions: bool,
    
    // Terminal
    pub font_family: String,
    pub font_size: f32,
    pub scrollback_lines: usize,
    pub cursor_style: CursorStyle,
    pub cursor_blink: bool,
    pub bell_style: BellStyle,
    
    // Theme
    pub selected_theme: String,
    
    // Connection
    pub default_port: u16,
    pub connection_timeout: u32,
    pub keepalive_interval: u32,
    pub compression: bool,
    
    // Security
    pub auto_lock_timeout: u32,
    pub remember_passwords: bool,
    pub strict_host_key_checking: bool,
    
    // Advanced
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CursorStyle {
    Block,
    Beam,
    Underline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BellStyle {
    None,
    Visual,
    Audio,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_shell: "/bin/bash".to_string(),
            auto_connect_on_startup: false,
            restore_previous_sessions: true,
            font_family: "monospace".to_string(),
            font_size: 14.0,
            scrollback_lines: 10000,
            cursor_style: CursorStyle::Block,
            cursor_blink: true,
            bell_style: BellStyle::Visual,
            selected_theme: "Default Dark".to_string(),
            default_port: 22,
            connection_timeout: 30,
            keepalive_interval: 60,
            compression: false,
            auto_lock_timeout: 0,
            remember_passwords: false,
            strict_host_key_checking: true,
            log_level: "info".to_string(),
        }
    }
}

impl Settings {
    pub fn load(db: &Database) -> Result<Self> {
        let conn = db.connection();
        
        match conn.query_row(
            "SELECT value FROM settings WHERE key = 'app_settings'",
            [],
            |row| row.get::<_, String>(0),
        ) {
            Ok(json) => Ok(serde_json::from_str(&json)?),
            Err(_) => Ok(Self::default()),
        }
    }
    
    pub fn save(&self, db: &Database) -> Result<()> {
        let json = serde_json::to_string(self)?;
        
        db.connection().execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('app_settings', ?1)",
            [&json],
        )?;
        
        Ok(())
    }
}
