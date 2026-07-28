//! Terminal emulator - high-level facade over parser, buffer, and renderer

use super::buffer::TerminalBuffer;
use super::parser::TerminalParser;
use super::renderer::{RendererConfig, TerminalRenderer};
use super::TerminalSize;
use eframe::egui;

/// Full terminal emulator combining parser, buffer, and renderer
pub struct TerminalEmulator {
    /// VT/ANSI parser and buffer
    parser: TerminalParser,

    /// Renderer for egui output
    renderer: TerminalRenderer,

    /// Window title set via OSC sequences
    title: String,
}

impl TerminalEmulator {
    /// Create terminal emulator with default renderer configuration
    pub fn new(cols: usize, rows: usize) -> Self {
        Self::with_config(cols, rows, 10000, RendererConfig::default())
    }

    /// Create terminal emulator with explicit configuration and scrollback size
    pub fn with_config(
        cols: usize,
        rows: usize,
        scrollback_size: usize,
        config: RendererConfig,
    ) -> Self {
        Self {
            parser: TerminalParser::new(cols as u16, rows as u16, scrollback_size),
            renderer: TerminalRenderer::new(config),
            title: String::new(),
        }
    }

    /// Process raw terminal data (VT/ANSI escape sequences and text)
    pub fn process(&mut self, data: &[u8]) {
        self.parser.process(data);
    }

    /// Current terminal dimensions
    pub fn size(&self) -> TerminalSize {
        self.parser.buffer().size()
    }

    /// Resize terminal to new column/row dimensions
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.parser.resize(cols, rows);
    }

    /// Render terminal contents into an egui Ui
    pub fn render(&mut self, ui: &mut egui::Ui) {
        let buffer = self.parser.buffer();
        self.renderer.render(ui, buffer);
    }

    /// Borrow the underlying terminal buffer
    pub fn buffer(&self) -> &TerminalBuffer {
        self.parser.buffer()
    }

    /// Scroll display to the most recent output
    pub fn scroll_to_bottom(&mut self) {
        let buffer = self.parser.buffer();
        self.renderer.scroll_to_bottom(buffer);
    }

    /// Clear the visible screen and reset cursor
    pub fn clear(&mut self) {
        self.parser.buffer_mut().clear();
    }

    /// Search for a pattern in the scrollback buffer
    pub fn search(&self, pattern: &str, case_sensitive: bool) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        let buf = self.parser.buffer();

        for row_idx in 0..buf.scrollback_len() {
            if let Some(line) = buf.get_scrollback_row(row_idx) {
                let text: String = line.iter().map(|cell| cell.character).collect();
                let haystack = if case_sensitive {
                    text.clone()
                } else {
                    text.to_lowercase()
                };
                let needle = if case_sensitive {
                    pattern.to_string()
                } else {
                    pattern.to_lowercase()
                };

                let mut start = 0;
                while let Some(pos) = haystack[start..].find(&needle) {
                    matches.push((row_idx, start + pos));
                    start += pos + 1;
                }
            }
        }

        matches
    }

    /// Get the current terminal window title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set the terminal window title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_has_requested_size() {
        let term = TerminalEmulator::new(80, 24);
        let size = term.size();
        assert_eq!(size.cols, 80);
        assert_eq!(size.rows, 24);
    }

    #[test]
    fn test_with_config_custom_scrollback() {
        let term = TerminalEmulator::with_config(40, 10, 500, RendererConfig::default());
        let size = term.size();
        assert_eq!(size.cols, 40);
        assert_eq!(size.rows, 10);
    }

    #[test]
    fn test_process_writes_text_into_buffer() {
        let mut term = TerminalEmulator::new(20, 5);
        term.process(b"hi");
        let row = term.buffer().get_row(0).expect("row 0 exists");
        assert_eq!(row[0].character, 'h');
        assert_eq!(row[1].character, 'i');
    }

    #[test]
    fn test_resize_changes_size() {
        let mut term = TerminalEmulator::new(80, 24);
        term.resize(100, 30);
        let size = term.size();
        assert_eq!(size.cols, 100);
        assert_eq!(size.rows, 30);
    }

    #[test]
    fn test_title_defaults_empty_and_can_be_set() {
        let mut term = TerminalEmulator::new(80, 24);
        assert_eq!(term.title(), "");
        term.set_title("my session".to_string());
        assert_eq!(term.title(), "my session");
    }

    #[test]
    fn test_clear_resets_visible_screen() {
        let mut term = TerminalEmulator::new(20, 5);
        term.process(b"hello");
        term.clear();
        let row = term.buffer().get_row(0).expect("row 0 exists");
        assert!(row.iter().all(|c| c.character == ' '));
    }

    #[test]
    fn test_scroll_to_bottom_does_not_panic_on_fresh_terminal() {
        let mut term = TerminalEmulator::new(20, 5);
        // Should be a no-op safe call even with no scrollback content yet.
        term.scroll_to_bottom();
    }

    #[test]
    fn test_search_finds_matches_in_scrollback() {
        let mut term = TerminalEmulator::new(10, 2);
        // Fill more lines than the visible height so earlier lines roll into scrollback.
        for i in 0..5 {
            term.process(format!("line{}\r\n", i).as_bytes());
        }
        let matches = term.search("line", true);
        assert!(!matches.is_empty());
    }

    #[test]
    fn test_search_case_insensitive() {
        let mut term = TerminalEmulator::new(10, 2);
        for i in 0..5 {
            term.process(format!("LINE{}\r\n", i).as_bytes());
        }
        let matches_sensitive = term.search("line", true);
        let matches_insensitive = term.search("line", false);
        assert!(matches_sensitive.is_empty());
        assert!(!matches_insensitive.is_empty());
    }

    #[test]
    fn test_search_no_match_returns_empty() {
        let mut term = TerminalEmulator::new(10, 2);
        for i in 0..5 {
            term.process(format!("line{}\r\n", i).as_bytes());
        }
        let matches = term.search("nonexistent", true);
        assert!(matches.is_empty());
    }

    #[test]
    fn test_search_finds_multiple_matches_in_same_row() {
        let mut term = TerminalEmulator::new(20, 2);
        for _ in 0..3 {
            term.process(b"ab ab\r\n");
        }
        let matches = term.search("ab", true);
        // Each scrolled-off row containing "ab ab" should yield two matches.
        assert!(matches.len() >= 2);
    }
}
