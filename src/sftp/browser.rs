//! SFTP file browser implementation

use anyhow::Result;
use std::path::PathBuf;
use super::{FileEntry, FileType};

/// SFTP browser state
pub struct SftpBrowser {
    /// Current directory path
    current_path: PathBuf,
    
    /// List of files in current directory
    entries: Vec<FileEntry>,
    
    /// Selected file indices
    selected: Vec<usize>,
    
    /// Sort column and direction
    sort_by: SortColumn,
    sort_ascending: bool,
}

/// Column to sort by
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortColumn {
    Name,
    Size,
    Modified,
    Type,
}

impl Default for SftpBrowser {
    fn default() -> Self {
        Self::new()
    }
}

impl SftpBrowser {
    /// Create new SFTP browser starting at home directory
    pub fn new() -> Self {
        Self {
            current_path: PathBuf::from("/"),
            entries: Vec::new(),
            selected: Vec::new(),
            sort_by: SortColumn::Name,
            sort_ascending: true,
        }
    }
    
    /// Get current directory path
    pub fn current_path(&self) -> &Path {
        &self.current_path
    }
    
    /// Get list of entries in current directory
    pub fn entries(&self) -> &[FileEntry] {
        &self.entries
    }
    
    /// Get selected indices
    pub fn selected(&self) -> &[usize] {
        &self.selected
    }
    
    /// Set current directory entries
    pub fn set_entries(&mut self, entries: Vec<FileEntry>) {
        self.entries = entries;
        self.sort_entries();
        self.selected.clear();
    }
    
    /// Change to specified directory
    pub fn change_directory(&mut self, path: PathBuf) {
        self.current_path = path;
        self.selected.clear();
    }
    
    /// Go to parent directory
    pub fn go_up(&mut self) -> Option<PathBuf> {
        if let Some(parent) = self.current_path.parent() {
            let parent_path = parent.to_path_buf();
            self.current_path = parent_path.clone();
            self.selected.clear();
            Some(parent_path)
        } else {
            None
        }
    }
    
    /// Go to home directory
    pub fn go_home(&mut self) -> PathBuf {
        self.current_path = PathBuf::from("/");
        self.selected.clear();
        self.current_path.clone()
    }
    
    /// Select/deselect entry at index
    pub fn toggle_selection(&mut self, index: usize) {
        if index < self.entries.len() {
            if let Some(pos) = self.selected.iter().position(|&i| i == index) {
                self.selected.remove(pos);
            } else {
                self.selected.push(index);
            }
        }
    }
    
    /// Select all entries
    pub fn select_all(&mut self) {
        self.selected = (0..self.entries.len()).collect();
    }
    
    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected.clear();
    }
    
    /// Get selected entries
    pub fn get_selected_entries(&self) -> Vec<FileEntry> {
        self.selected
            .iter()
            .filter_map(|&i| self.entries.get(i).cloned())
            .collect()
    }
    
    /// Set sort column and direction
    pub fn set_sort(&mut self, column: SortColumn, ascending: bool) {
        self.sort_by = column;
        self.sort_ascending = ascending;
        self.sort_entries();
    }
    
    /// Sort entries by current sort settings
    fn sort_entries(&mut self) {
        self.entries.sort_by(|a, b| {
            // Directories first
            match (a.file_type, b.file_type) {
                (FileType::Directory, FileType::File) => return std::cmp::Ordering::Less,
                (FileType::File, FileType::Directory) => return std::cmp::Ordering::Greater,
                _ => {}
            }
            
            let cmp = match self.sort_by {
                SortColumn::Name => a.name.cmp(&b.name),
                SortColumn::Size => a.size.cmp(&b.size),
                SortColumn::Modified => a.modified.cmp(&b.modified),
                SortColumn::Type => a.file_type.to_string().cmp(&b.file_type.to_string()),
            };
            
            if self.sort_ascending {
                cmp
            } else {
                cmp.reverse()
            }
        });
    }
    
    /// Get entry at index
    pub fn get_entry(&self, index: usize) -> Option<&FileEntry> {
        self.entries.get(index)
    }
    
    /// Get full path for entry
    pub fn get_full_path(&self, entry: &FileEntry) -> PathBuf {
        self.current_path.join(&entry.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    
    fn create_test_entry(name: &str, file_type: FileType, size: u64) -> FileEntry {
        FileEntry {
            name: name.to_string(),
            path: PathBuf::from(name),
            file_type,
            size,
            permissions: 0o644,
            modified: Utc::now(),
        }
    }
    
    #[test]
    fn test_browser_creation() {
        let browser = SftpBrowser::new();
        assert_eq!(browser.current_path(),Path::new("/"));
        assert!(browser.entries().is_empty());
    }
    
    #[test]
    fn test_selection() {
        let mut browser = SftpBrowser::new();
        let entries = vec![
            create_test_entry("file1.txt", FileType::File, 100),
            create_test_entry("file2.txt", FileType::File, 200),
        ];
        browser.set_entries(entries);
        
        browser.toggle_selection(0);
        assert_eq!(browser.selected(),&[0]);
        
        browser.toggle_selection(1);
        assert_eq!(browser.selected(),&[0,1]);
        
        browser.toggle_selection(0);
        assert_eq!(browser.selected(),&[1]);
    }
    
    #[test]
    fn test_sorting() {
        let mut browser = SftpBrowser::new();
        let entries = vec![
            create_test_entry("zebra.txt", FileType::File, 300),
            create_test_entry("apple.txt", FileType::File, 100),
            create_test_entry("banana", FileType::Directory, 0),
        ];
        browser.set_entries(entries);
        
        // Directories should be first
        assert_eq!(browser.entries()[0].name,"banana");
        
        // Files should be sorted alphabetically
        assert_eq!(browser.entries()[1].name,"apple.txt");
        assert_eq!(browser.entries()[2].name,"zebra.txt");
    }
}
