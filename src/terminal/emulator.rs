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
