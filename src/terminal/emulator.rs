
/// Search in terminal buffer
pub fn search(&self, pattern: &str, case_sensitive: bool) -> Vec<(usize, usize)> {
    let mut matches = Vec::new();
    let pattern = if case_sensitive {
        pattern.to_string()
    } else {
        pattern.to_lowercase()
    };
    
    for (row_idx, line) in self.scrollback.iter().enumerate() {
        let text = line.iter().map(|cell| cell.c).collect::<String>();
        let text = if case_sensitive {
            text
        } else {
            text.to_lowercase()
        };
        
        let mut start = 0;
        while let Some(pos) = text[start..].find(&pattern) {
            matches.push((row_idx, start + pos));
            start += pos + 1;
        }
    }
    
    matches
}

/// Clear terminal
pub fn clear(&mut self) {
    self.scrollback.clear();
    self.cursor_row = 0;
    self.cursor_col = 0;
}

/// Get terminal title
pub fn title(&self) -> &str {
    &self.title
}

/// Set terminal title
pub fn set_title(&mut self, title: String) {
    self.title = title;
}
