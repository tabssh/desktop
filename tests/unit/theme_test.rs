//! Theme system tests

use tabssh::config::themes::{Theme, ThemeManager};

#[test]
fn test_parse_color() {
    assert_eq!(Theme::parse_color("#ffffff"),Some((255,255,255)));
    assert_eq!(Theme::parse_color("#000000"),Some((0,0,0)));
    assert_eq!(Theme::parse_color("#ff00ff"),Some((255,0,255)));
}

#[test]
fn test_default_theme() {
    let theme = Theme::default_dark();
    assert_eq!(theme.name,"DefaultDark");
    assert!(!theme.background.is_empty());
}

#[test]
fn test_theme_manager() {
    let manager = ThemeManager::new();
    let themes = manager.list_themes();
    
    assert!(!themes.is_empty());
    assert!(themes.contains(&"DefaultDark".to_string()));
    
    let current = manager.current_theme();
    assert!(current.is_some());
}
