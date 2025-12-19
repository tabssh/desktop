//! Utility helper functions

use std::path::Path;

/// Format file size in human-readable format
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    
    if unit_idx == 0 {
        format!("{}{}",bytes,UNITS[unit_idx])
    } else {
        format!("{:.2}{}",size,UNITS[unit_idx])
    }
}

/// Format Unix permissions as rwx string
pub fn format_permissions(mode: u32) -> String {
    let mut perms = String::new();
    
    // Owner
    perms.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    
    // Group
    perms.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    
    // Others
    perms.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    
    perms
}

/// Get file extension
pub fn get_file_extension(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(|s| s.to_str())
}

/// Sanitize filename for safe filesystem operations
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c => c,
        })
        .collect()
}

/// Calculate transfer speed
pub fn format_transfer_speed(bytes_per_second: f64) -> String {
    format_file_size(bytes_per_second as u64) + "/s"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0),"0B");
        assert_eq!(format_file_size(1023),"1023B");
        assert_eq!(format_file_size(1024),"1.00KB");
        assert_eq!(format_file_size(1048576),"1.00MB");
    }

    #[test]
    fn test_format_permissions() {
        assert_eq!(format_permissions(0o755),"rwxr-xr-x");
        assert_eq!(format_permissions(0o644),"rw-r--r--");
        assert_eq!(format_permissions(0o600),"rw-------");
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("test.txt"),"test.txt");
        assert_eq!(sanitize_filename("test/file.txt"),"test_file.txt");
        assert_eq!(sanitize_filename("test:file?.txt"),"test_file_.txt");
    }
}
