//! Theme management and loading

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Terminal color theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub selection: String,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    pub bright_black: String,
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_magenta: String,
    pub bright_cyan: String,
    pub bright_white: String,
}

impl Theme {
    /// Parse hex color to RGB
    pub fn parse_color(hex: &str) -> Option<(u8, u8, u8)> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

        Some((r, g, b))
    }

    /// Load theme from JSON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let theme: Theme = serde_json::from_str(&content)?;
        Ok(theme)
    }

    /// Save theme to JSON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Default dark theme
    pub fn default_dark() -> Self {
        Self {
            name: "Default Dark".to_string(),
            background: "#1e1e1e".to_string(),
            foreground: "#d4d4d4".to_string(),
            cursor: "#aeafad".to_string(),
            selection: "#264f78".to_string(),
            black: "#000000".to_string(),
            red: "#cd3131".to_string(),
            green: "#0dbc79".to_string(),
            yellow: "#e5e510".to_string(),
            blue: "#2472c8".to_string(),
            magenta: "#bc3fbc".to_string(),
            cyan: "#11a8cd".to_string(),
            white: "#e5e5e5".to_string(),
            bright_black: "#666666".to_string(),
            bright_red: "#f14c4c".to_string(),
            bright_green: "#23d18b".to_string(),
            bright_yellow: "#f5f543".to_string(),
            bright_blue: "#3b8eea".to_string(),
            bright_magenta: "#d670d6".to_string(),
            bright_cyan: "#29b8db".to_string(),
            bright_white: "#ffffff".to_string(),
        }
    }
}

/// Theme manager
pub struct ThemeManager {
    themes: Vec<Theme>,
    current_theme: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            themes: Vec::new(),
            current_theme: "Default Dark".to_string(),
        };

        // Add default theme
        manager.themes.push(Theme::default_dark());

        // Load bundled themes
        manager.load_bundled_themes();

        manager
    }

    fn load_bundled_themes(&mut self) {
        let theme_files = [
            (
                "assets/themes/dracula.json",
                include_str!("../../assets/themes/dracula.json"),
            ),
            (
                "assets/themes/solarized-dark.json",
                include_str!("../../assets/themes/solarized-dark.json"),
            ),
            (
                "assets/themes/nord.json",
                include_str!("../../assets/themes/nord.json"),
            ),
            (
                "assets/themes/monokai.json",
                include_str!("../../assets/themes/monokai.json"),
            ),
        ];

        for (name, content) in theme_files {
            if let Ok(theme) = serde_json::from_str::<Theme>(content) {
                self.themes.push(theme);
            } else {
                log::warn!("Failed to parse bundled theme: {}", name);
            }
        }
    }

    pub fn get_theme(&self, name: &str) -> Option<&Theme> {
        self.themes.iter().find(|t| t.name == name)
    }

    pub fn current_theme(&self) -> Option<&Theme> {
        self.get_theme(&self.current_theme)
    }

    pub fn set_current_theme(&mut self, name: String) {
        if self.themes.iter().any(|t| t.name == name) {
            self.current_theme = name;
        }
    }

    pub fn list_themes(&self) -> Vec<String> {
        self.themes.iter().map(|t| t.name.clone()).collect()
    }

    pub fn add_custom_theme(&mut self, theme: Theme) {
        self.themes.push(theme);
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        assert_eq!(Theme::parse_color("#ff0000"), Some((255, 0, 0)));
        assert_eq!(Theme::parse_color("#00ff00"), Some((0, 255, 0)));
        assert_eq!(Theme::parse_color("#0000ff"), Some((0, 0, 255)));
        assert_eq!(Theme::parse_color("invalid"), None);
    }

    #[test]
    fn test_theme_manager() {
        let manager = ThemeManager::new();
        assert!(!manager.list_themes().is_empty());
        assert!(manager.current_theme().is_some());
    }

    #[test]
    fn test_parse_color_boundary_cases() {
        // Too short / too long hex strings are rejected.
        assert_eq!(Theme::parse_color("#fff"), None);
        assert_eq!(Theme::parse_color("#ff00000"), None);
        // Empty string.
        assert_eq!(Theme::parse_color(""), None);
        // No leading '#' still parses (only stripped if present).
        assert_eq!(Theme::parse_color("00ff00"), Some((0, 255, 0)));
        // Mixed-case hex digits.
        assert_eq!(Theme::parse_color("#AaBbCc"), Some((170, 187, 204)));
        // Non-hex characters fail to parse.
        assert_eq!(Theme::parse_color("#zzzzzz"), None);
    }

    #[test]
    fn test_default_dark_fields() {
        let theme = Theme::default_dark();
        assert_eq!(theme.name, "Default Dark");
        assert_eq!(theme.background, "#1e1e1e");
        assert_eq!(theme.foreground, "#d4d4d4");
    }

    #[test]
    fn test_save_and_load_from_file_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("custom-theme.json");

        let theme = Theme::default_dark();
        theme.save_to_file(&path).unwrap();

        let loaded = Theme::load_from_file(&path).unwrap();
        assert_eq!(loaded.name, theme.name);
        assert_eq!(loaded.background, theme.background);
        assert_eq!(loaded.bright_white, theme.bright_white);
    }

    #[test]
    fn test_load_from_file_missing_path_errors() {
        let result = Theme::load_from_file("/nonexistent/path/does-not-exist.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_file_invalid_json_errors() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad-theme.json");
        std::fs::write(&path, "not valid json").unwrap();

        let result = Theme::load_from_file(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_theme_found_and_missing() {
        let manager = ThemeManager::new();
        assert!(manager.get_theme("Default Dark").is_some());
        assert!(manager.get_theme("Does Not Exist").is_none());
    }

    #[test]
    fn test_set_current_theme_valid_and_invalid() {
        let mut manager = ThemeManager::new();

        // Setting an unknown theme name is a no-op — current theme stays.
        manager.set_current_theme("Nonexistent Theme".to_string());
        assert_eq!(manager.current_theme().unwrap().name, "Default Dark");

        manager.add_custom_theme(Theme {
            name: "My Custom".to_string(),
            ..Theme::default_dark()
        });
        manager.set_current_theme("My Custom".to_string());
        assert_eq!(manager.current_theme().unwrap().name, "My Custom");
    }

    #[test]
    fn test_add_custom_theme_appears_in_list() {
        let mut manager = ThemeManager::new();
        let before = manager.list_themes().len();

        manager.add_custom_theme(Theme {
            name: "Sunset".to_string(),
            ..Theme::default_dark()
        });

        let after = manager.list_themes();
        assert_eq!(after.len(), before + 1);
        assert!(after.contains(&"Sunset".to_string()));
    }
}
