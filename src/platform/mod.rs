//! Platform-specific functionality

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
pub mod bsd;

pub struct PlatformManager;

impl PlatformManager {
    pub fn new() -> Self {
        Self
    }
    
    pub fn get_default_shell() -> String {
        #[cfg(target_family = "unix")]
        {
            std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
        }
        
        #[cfg(target_os = "windows")]
        {
            "cmd.exe".to_string()
        }
    }
    
    pub fn get_home_directory() -> Option<std::path::PathBuf> {
        dirs::home_dir()
    }
    
    pub fn get_config_directory() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|p| p.join("tabssh"))
    }
    
    pub fn get_data_directory() -> Option<std::path::PathBuf> {
        dirs::data_dir().map(|p| p.join("tabssh"))
    }
}

impl Default for PlatformManager {
    fn default() -> Self {
        Self::new()
    }
}
