//! Platform-specific code
//!
//! Handles OS-specific functionality like keychain access,
//! system directories, and platform integration.

#![allow(dead_code)]

/// Get the platform name
pub fn platform_name() -> &'static str {
    #[cfg(target_os = "linux")]
    {
        "Linux"
    }
    #[cfg(target_os = "macos")]
    {
        "macOS"
    }
    #[cfg(target_os = "windows")]
    {
        "Windows"
    }
    #[cfg(target_os = "freebsd")]
    {
        "FreeBSD"
    }
    #[cfg(target_os = "openbsd")]
    {
        "OpenBSD"
    }
    #[cfg(target_os = "netbsd")]
    {
        "NetBSD"
    }
    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "windows",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    )))]
    {
        "Unknown"
    }
}

/// Get the default SSH directory
pub fn ssh_dir() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|h| h.join(".ssh"))
}

/// Get the default SSH config file path
pub fn ssh_config_path() -> Option<std::path::PathBuf> {
    ssh_dir().map(|d| d.join("config"))
}

/// Get the known_hosts file path
pub fn known_hosts_path() -> Option<std::path::PathBuf> {
    ssh_dir().map(|d| d.join("known_hosts"))
}
