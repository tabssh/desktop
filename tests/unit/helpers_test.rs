//! Utility helpers tests

use tabssh::utils::helpers::*;

#[test]
fn test_format_file_size() {
    assert_eq!(format_file_size(0),"0B");
    assert_eq!(format_file_size(500),"500B");
    assert_eq!(format_file_size(1024),"1.00KB");
    assert_eq!(format_file_size(1048576),"1.00MB");
    assert_eq!(format_file_size(1073741824),"1.00GB");
}

#[test]
fn test_format_permissions() {
    assert_eq!(format_permissions(0o755),"rwxr-xr-x");
    assert_eq!(format_permissions(0o644),"rw-r--r--");
    assert_eq!(format_permissions(0o777),"rwxrwxrwx");
    assert_eq!(format_permissions(0o000),"---------");
}

#[test]
fn test_get_file_extension() {
    assert_eq!(get_file_extension("file.txt"),Some("txt"));
    assert_eq!(get_file_extension("archive.tar.gz"),Some("gz"));
    assert_eq!(get_file_extension("noext"),None);
}

#[test]
fn test_sanitize_filename() {
    assert_eq!(sanitize_filename("normal.txt"),"normal.txt");
    assert_eq!(sanitize_filename("path/to/file.txt"),"path_to_file.txt");
    assert_eq!(sanitize_filename("bad:name?.txt"),"bad_name_.txt");
}
