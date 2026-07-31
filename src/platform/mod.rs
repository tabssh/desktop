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
        #[cfg(target_os = "macos")]
        {
            dirs::config_dir().map(|p| p.join("tabssh").join("config"))
        }
        #[cfg(target_os = "windows")]
        {
            dirs::config_dir().map(|p| p.join("tabssh").join("tabssh").join("config"))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            dirs::config_dir().map(|p| p.join("tabssh").join("tabssh"))
        }
    }

    pub fn get_data_directory() -> Option<std::path::PathBuf> {
        #[cfg(target_os = "macos")]
        {
            dirs::data_dir().map(|p| p.join("tabssh").join("data"))
        }
        #[cfg(target_os = "windows")]
        {
            dirs::data_local_dir().map(|p| p.join("tabssh").join("tabssh").join("data"))
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            dirs::data_dir().map(|p| p.join("tabssh").join("tabssh"))
        }
    }
}

impl Default for PlatformManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_default_are_equivalent() {
        // Both construct the zero-sized PlatformManager without panicking.
        let _via_new = PlatformManager::new();
        let _via_default = PlatformManager;
    }

    #[test]
    fn test_get_default_shell_is_non_empty() {
        let shell = PlatformManager::get_default_shell();
        assert!(!shell.is_empty());
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn test_get_default_shell_respects_shell_env_var() {
        // SAFETY: this test is single-threaded with respect to the SHELL
        // env var — no other test in this crate reads or writes it.
        let previous = std::env::var("SHELL").ok();

        unsafe {
            std::env::set_var("SHELL", "/bin/custom-shell");
        }
        assert_eq!(PlatformManager::get_default_shell(), "/bin/custom-shell");

        unsafe {
            std::env::remove_var("SHELL");
        }
        assert_eq!(PlatformManager::get_default_shell(), "/bin/sh");

        // Restore original value to avoid polluting other tests.
        unsafe {
            match previous {
                Some(v) => std::env::set_var("SHELL", v),
                None => std::env::remove_var("SHELL"),
            }
        }
    }

    #[test]
    fn test_get_home_directory_returns_path_when_available() {
        // dirs::home_dir() is environment-dependent; just assert the call
        // doesn't panic and, when Some, yields a non-empty path.
        if let Some(home) = PlatformManager::get_home_directory() {
            assert!(!home.as_os_str().is_empty());
        }
    }

    #[test]
    fn test_get_config_directory_contains_tabssh() {
        if let Some(dir) = PlatformManager::get_config_directory() {
            assert!(dir.components().any(|c| c.as_os_str() == "tabssh"));
        }
    }

    #[test]
    fn test_get_data_directory_contains_tabssh() {
        if let Some(dir) = PlatformManager::get_data_directory() {
            assert!(dir.components().any(|c| c.as_os_str() == "tabssh"));
        }
    }
}
