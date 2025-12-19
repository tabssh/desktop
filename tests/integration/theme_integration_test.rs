//! Theme integration tests

use tabssh::config::themes::ThemeManager;

#[test]
fn test_theme_manager_loads_all_themes() {
    let manager = ThemeManager::new();
    let themes = manager.list_themes();
    
    assert!(!themes.is_empty());
    assert!(themes.contains(&"DefaultDark".to_string()));
}

#[test]
fn test_switch_theme() {
    let mut manager = ThemeManager::new();
    
    manager.set_current_theme("Nord".to_string());
    let current = manager.current_theme();
    
    assert!(current.is_some());
    assert_eq!(current.unwrap().name,"Nord");
}

#[test]
fn test_all_themes_parse() {
    let manager = ThemeManager::new();
    let themes = manager.list_themes();
    
    for theme_name in themes {
        let theme = manager.get_theme(&theme_name);
        assert!(theme.is_some(),"Theme{}notfound",theme_name);
    }
}
