//! SFTP integration tests

use std::path::PathBuf;
use tabssh::sftp::{FileEntry, FileType, SftpBrowser};

fn test_entry(name: &str) -> FileEntry {
    FileEntry {
        name: name.to_string(),
        path: PathBuf::from("/").join(name),
        file_type: FileType::File,
        size: 0,
        modified: None,
        permissions: 0o644,
        owner: "user".to_string(),
        group: "user".to_string(),
    }
}

#[test]
fn test_sftp_browser_navigation() {
    let mut browser = SftpBrowser::new();

    assert_eq!(browser.current_path(), PathBuf::from("/"));

    browser.change_directory(PathBuf::from("/home"));
    assert_eq!(browser.current_path(), PathBuf::from("/home"));

    let up = browser.go_up();
    assert!(up.is_some());
    assert_eq!(browser.current_path(), PathBuf::from("/"));
}

#[test]
fn test_sftp_browser_selection() {
    let mut browser = SftpBrowser::new();

    // toggle_selection only accepts indices within the current entry list,
    // so entries must be populated first (mirrors src/sftp/browser.rs's own
    // unit test).
    browser.set_entries(vec![test_entry("file1.txt"), test_entry("file2.txt")]);

    browser.toggle_selection(0);
    assert_eq!(browser.selected().len(), 1);

    browser.toggle_selection(0);
    assert_eq!(browser.selected().len(), 0);

    browser.select_all();
    browser.clear_selection();
    assert_eq!(browser.selected().len(), 0);
}
