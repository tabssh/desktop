//! Configuration module - settings and themes

mod themes;

pub use themes::Theme;

use serde::{Deserialize, Serialize};

/// Theme mode selection
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Dark
    }
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

/// Application settings
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
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

impl Default for Settings {
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
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CursorStyle {
    Block,
    Beam,
    Underline,
}

impl Default for CursorStyle {
    fn default() -> Self {
        Self::Block
    }
}
