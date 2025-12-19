//! SFTP integration tests

use tabssh::sftp::{SftpBrowser, SortColumn};
use std::path::PathBuf;

#[test]
fn test_sftp_browser_navigation() {
    let mut browser = SftpBrowser::new();
    
    assert_eq!(browser.current_path(),PathBuf::from("/"));
    
    browser.change_directory(PathBuf::from("/home"));
    assert_eq!(browser.current_path(),PathBuf::from("/home"));
    
    let up = browser.go_up();
    assert!(up.is_some());
    assert_eq!(browser.current_path(),PathBuf::from("/"));
}

#[test]
fn test_sftp_browser_selection() {
    let mut browser = SftpBrowser::new();
    
    browser.toggle_selection(0);
    assert_eq!(browser.selected().len(),1);
    
    browser.toggle_selection(0);
    assert_eq!(browser.selected().len(),0);
    
    browser.select_all();
    browser.clear_selection();
    assert_eq!(browser.selected().len(),0);
}
