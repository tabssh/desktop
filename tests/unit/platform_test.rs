//! Platform tests

use tabssh::platform::PlatformManager;

#[test]
fn test_platform_manager() {
    let _manager = PlatformManager::new();
    
    let shell = PlatformManager::get_default_shell();
    assert!(!shell.is_empty());
    
    let home = PlatformManager::get_home_directory();
    assert!(home.is_some());
    
    let config = PlatformManager::get_config_directory();
    assert!(config.is_some());
}
