//! Configuration module - settings and themes

pub mod themes;

pub use themes::Theme;

use serde::{Deserialize, Serialize};

/// Theme mode selection
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
    System,
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeMode::Dark => write!(f, "Dark"),
            ThemeMode::Light => write!(f, "Light"),
            ThemeMode::System => write!(f, "System"),
        }
    }
}

/// Application settings loaded from config file (TOML)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Theme mode (dark/light/system)
    pub theme_mode: ThemeMode,

    /// Current color theme name
    pub color_theme: String,

    /// Font size for terminal
    pub font_size: f32,

    /// Font family for terminal
    pub font_family: String,

    /// Scrollback buffer lines
    pub scrollback_lines: u32,

    /// Cursor blink enabled
    pub cursor_blink: bool,

    /// Cursor style (block, beam, underline)
    pub cursor_style: CursorStyle,

    /// Bell enabled
    pub bell_enabled: bool,

    /// Auto-reconnect on disconnect
    pub auto_reconnect: bool,

    /// Default SSH port
    pub default_port: u16,

    /// Default username
    pub default_username: String,

    /// Connection timeout (seconds)
    pub connection_timeout: u32,

    /// Keep-alive interval (seconds)
    pub keepalive_interval: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Dark,
            color_theme: "Dracula".to_string(),
            font_size: 14.0,
            font_family: "JetBrains Mono".to_string(),
            scrollback_lines: 10000,
            cursor_blink: true,
            cursor_style: CursorStyle::Block,
            bell_enabled: true,
            auto_reconnect: true,
            default_port: 22,
            default_username: String::new(),
            connection_timeout: 30,
            keepalive_interval: 60,
        }
    }
}

/// Cursor style for terminal
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum CursorStyle {
    #[default]
    Block,
    Beam,
    Underline,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_mode_default_is_dark() {
        assert_eq!(ThemeMode::default(), ThemeMode::Dark);
    }

    #[test]
    fn test_theme_mode_display() {
        assert_eq!(ThemeMode::Dark.to_string(), "Dark");
        assert_eq!(ThemeMode::Light.to_string(), "Light");
        assert_eq!(ThemeMode::System.to_string(), "System");
    }

    #[test]
    fn test_cursor_style_default_is_block() {
        assert_eq!(CursorStyle::default(), CursorStyle::Block);
    }

    #[test]
    fn test_app_config_default_values() {
        let cfg = AppConfig::default();
        assert_eq!(cfg.theme_mode, ThemeMode::Dark);
        assert_eq!(cfg.color_theme, "Dracula");
        assert_eq!(cfg.font_size, 14.0);
        assert_eq!(cfg.font_family, "JetBrains Mono");
        assert_eq!(cfg.scrollback_lines, 10000);
        assert!(cfg.cursor_blink);
        assert_eq!(cfg.cursor_style, CursorStyle::Block);
        assert!(cfg.bell_enabled);
        assert!(cfg.auto_reconnect);
        assert_eq!(cfg.default_port, 22);
        assert_eq!(cfg.default_username, "");
        assert_eq!(cfg.connection_timeout, 30);
        assert_eq!(cfg.keepalive_interval, 60);
    }

    #[test]
    fn test_app_config_serde_round_trip() {
        let cfg = AppConfig::default();
        let json = serde_json::to_string(&cfg).unwrap();
        let restored: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.color_theme, cfg.color_theme);
        assert_eq!(restored.theme_mode, cfg.theme_mode);
    }
}
