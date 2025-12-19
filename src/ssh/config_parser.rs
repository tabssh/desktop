//! SSH config file parser (~/.ssh/config)

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// SSH config entry for a host
#[derive(Debug, Clone, Default)]
pub struct HostConfig {
    pub host_pattern: String,
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub identity_file: Vec<String>,
    pub proxy_jump: Option<String>,
    pub proxy_command: Option<String>,
    pub local_forward: Vec<(u16, String, u16)>, // (local_port, remote_host, remote_port)
    pub remote_forward: Vec<(u16, String, u16)>,
    pub dynamic_forward: Vec<u16>,
    pub compression: Option<bool>,
    pub server_alive_interval: Option<u32>,
}

/// SSH config parser
pub struct SshConfigParser {
    configs: HashMap<String, HostConfig>,
}

impl SshConfigParser {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    /// Parse SSH config file
    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        self.parse_content(&content)
    }

    /// Parse SSH config content
    pub fn parse_content(&mut self, content: &str) -> Result<()> {
        let mut current_host: Option<HostConfig> = None;

        for line in content.lines() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let keyword = parts[0].to_lowercase();
            
            match keyword.as_str() {
                "host" => {
                    // Save previous host
                    if let Some(host) = current_host.take() {
                        self.configs.insert(host.host_pattern.clone(), host);
                    }
                    
                    // Start new host
                    if parts.len() > 1 {
                        let mut host = HostConfig::default();
                        host.host_pattern = parts[1].to_string();
                        current_host = Some(host);
                    }
                }
                "hostname" if parts.len() > 1 => {
                    if let Some(ref mut host) = current_host {
                        host.hostname = Some(parts[1].to_string());
                    }
                }
                "port" if parts.len() > 1 => {
                    if let Some(ref mut host) = current_host {
                        if let Ok(port) = parts[1].parse::<u16>() {
                            host.port = Some(port);
                        }
                    }
                }
                "user" if parts.len() > 1 => {
                    if let Some(ref mut host) = current_host {
                        host.user = Some(parts[1].to_string());
                    }
                }
                "identityfile" if parts.len() > 1 => {
                    if let Some(ref mut host) = current_host {
                        let path = expand_tilde(parts[1]);
                        host.identity_file.push(path);
                    }
                }
                "proxyjump" if parts.len() > 1 => {
                    if let Some(ref mut host) = current_host {
                        host.proxy_jump = Some(parts[1].to_string());
                    }
                }
                "proxycommand" if parts.len() > 1 => {
                    if let Some(ref mut host) = current_host {
                        host.proxy_command = Some(parts[1..].join(" "));
                    }
                }
                "localforward" if parts.len() > 2 => {
                    if let Some(ref mut host) = current_host {
                        if let Some((local_port, remote_host, remote_port)) = parse_forward(&parts[1..]) {
                            host.local_forward.push((local_port, remote_host, remote_port));
                        }
                    }
                }
                "remoteforward" if parts.len() > 2 => {
                    if let Some(ref mut host) = current_host {
                        if let Some((remote_port, local_host, local_port)) = parse_forward(&parts[1..]) {
                            host.remote_forward.push((remote_port, local_host, local_port));
                        }
                    }
                }
                "dynamicforward" if parts.len() > 1 => {
                    if let Some(ref mut host) = current_host {
                        if let Ok(port) = parts[1].parse::<u16>() {
                            host.dynamic_forward.push(port);
                        }
                    }
                }
                "compression" if parts.len() > 1 => {
                    if let Some(ref mut host) = current_host {
                        host.compression = Some(parts[1].eq_ignore_ascii_case("yes"));
                    }
                }
                "serveraliveinterval" if parts.len() > 1 => {
                    if let Some(ref mut host) = current_host {
                        if let Ok(interval) = parts[1].parse::<u32>() {
                            host.server_alive_interval = Some(interval);
                        }
                    }
                }
                _ => {
                    // Ignore unknown keywords
                }
            }
        }

        // Save last host
        if let Some(host) = current_host {
            self.configs.insert(host.host_pattern.clone(), host);
        }

        Ok(())
    }

    /// Get config for a specific host
    pub fn get_config(&self, host: &str) -> Option<&HostConfig> {
        // Try exact match first
        if let Some(config) = self.configs.get(host) {
            return Some(config);
        }

        // Try wildcard patterns
        for (pattern, config) in &self.configs {
            if pattern == "*" || wildcard_match(pattern, host) {
                return Some(config);
            }
        }

        None
    }

    /// Get all host patterns
    pub fn get_all_hosts(&self) -> Vec<String> {
        self.configs.keys().cloned().collect()
    }

    /// Parse default SSH config location
    pub fn parse_default() -> Result<Self> {
        let mut parser = Self::new();
        
        if let Some(home) = dirs::home_dir() {
            let config_path = home.join(".ssh").join("config");
            if config_path.exists() {
                parser.parse_file(&config_path)?;
            }
        }

        Ok(parser)
    }
}

impl Default for SshConfigParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Expand tilde in path
fn expand_tilde(path: &str) -> String {
    if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            return path.replacen('~', &home.to_string_lossy(), 1);
        }
    }
    path.to_string()
}

/// Parse forward specification (port host:port or port:host:port)
fn parse_forward(parts: &[&str]) -> Option<(u16, String, u16)> {
    if parts.is_empty() {
        return None;
    }

    // Try "port host:port" format
    if parts.len() >= 2 {
        if let Ok(local_port) = parts[0].parse::<u16>() {
            let remote = parts[1];
            if let Some((host, port_str)) = remote.rsplit_once(':') {
                if let Ok(remote_port) = port_str.parse::<u16>() {
                    return Some((local_port, host.to_string(), remote_port));
                }
            }
        }
    }

    // Try "port:host:port" format
    if parts.len() >= 1 {
        let spec = parts[0];
        let components: Vec<&str> = spec.split(':').collect();
        if components.len() == 3 {
            if let Ok(local_port) = components[0].parse::<u16>() {
                if let Ok(remote_port) = components[2].parse::<u16>() {
                    return Some((local_port, components[1].to_string(), remote_port));
                }
            }
        }
    }

    None
}

/// Simple wildcard matching (* and ?)
fn wildcard_match(pattern: &str, text: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();
    
    wildcard_match_recursive(&pattern_chars, &text_chars, 0, 0)
}

fn wildcard_match_recursive(pattern: &[char], text: &[char], p_idx: usize, t_idx: usize) -> bool {
    if p_idx == pattern.len() {
        return t_idx == text.len();
    }

    if pattern[p_idx] == '*' {
        // Try matching 0 or more characters
        for i in t_idx..=text.len() {
            if wildcard_match_recursive(pattern, text, p_idx + 1, i) {
                return true;
            }
        }
        false
    } else if p_idx < pattern.len() && t_idx < text.len() &&
              (pattern[p_idx] == '?' || pattern[p_idx] == text[t_idx]) {
        wildcard_match_recursive(pattern, text, p_idx + 1, t_idx + 1)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_config() {
        let config = r#"
Host example
    HostName example.com
    Port 2222
    User admin
    IdentityFile ~/.ssh/id_rsa
"#;

        let mut parser = SshConfigParser::new();
        parser.parse_content(config).unwrap();

        let host_config = parser.get_config("example").unwrap();
        assert_eq!(host_config.hostname,Some("example.com".to_string()));
        assert_eq!(host_config.port,Some(2222));
        assert_eq!(host_config.user,Some("admin".to_string()));
    }

    #[test]
    fn test_wildcard_match() {
        assert!(wildcard_match("*.example.com","server.example.com"));
        assert!(wildcard_match("server?","server1"));
        assert!(!wildcard_match("*.com","example.org"));
    }

    #[test]
    fn test_parse_forwards() {
        assert_eq!(
            parse_forward(&["8080", "localhost:80"]),
            Some((8080, "localhost".to_string(), 80))
        );
        assert_eq!(
            parse_forward(&["8080:localhost:80"]),
            Some((8080, "localhost".to_string(), 80))
        );
    }
}
