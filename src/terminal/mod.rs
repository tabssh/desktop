//! Terminal module - VT100/xterm terminal emulation
//!
//! Provides full terminal emulation with ANSI escape sequence support,
//! scrollback buffer, and egui rendering.

#![allow(dead_code)]

mod buffer;
mod cell;
mod parser;
mod renderer;

pub use buffer::TerminalBuffer;
pub use parser::TerminalParser;
pub use renderer::{TerminalRenderer, RendererConfig, CursorStyle};

/// Terminal size in columns and rows
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TerminalSize {
    pub cols: u16,
    pub rows: u16,
}

impl Default for TerminalSize {
    fn default() -> Self {
        Self { cols: 80, rows: 24 }
    }
}

impl TerminalSize {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self { cols, rows }
    }
}

/// Terminal color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_egui(&self) -> eframe::egui::Color32 {
        eframe::egui::Color32::from_rgb(self.r, self.g, self.b)
    }
}

impl Color {
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const RED: Color = Color::rgb(205, 49, 49);
    pub const GREEN: Color = Color::rgb(13, 188, 121);
    pub const YELLOW: Color = Color::rgb(229, 229, 16);
    pub const BLUE: Color = Color::rgb(36, 114, 200);
    pub const MAGENTA: Color = Color::rgb(188, 63, 188);
    pub const CYAN: Color = Color::rgb(17, 168, 205);
    pub const WHITE: Color = Color::rgb(229, 229, 229);

    pub const BRIGHT_BLACK: Color = Color::rgb(102, 102, 102);
    pub const BRIGHT_RED: Color = Color::rgb(241, 76, 76);
    pub const BRIGHT_GREEN: Color = Color::rgb(35, 209, 139);
    pub const BRIGHT_YELLOW: Color = Color::rgb(245, 245, 67);
    pub const BRIGHT_BLUE: Color = Color::rgb(59, 142, 234);
    pub const BRIGHT_MAGENTA: Color = Color::rgb(214, 112, 214);
    pub const BRIGHT_CYAN: Color = Color::rgb(41, 184, 219);
    pub const BRIGHT_WHITE: Color = Color::rgb(255, 255, 255);
}

/// Complete terminal emulator combining parser, buffer, and renderer
pub struct Terminal {
    parser: TerminalParser,
    renderer: TerminalRenderer,
}

impl Terminal {
    pub fn new(cols: u16, rows: u16, scrollback: usize) -> Self {
        Self {
            parser: TerminalParser::new(cols, rows, scrollback),
            renderer: TerminalRenderer::new(RendererConfig::default()),
        }
    }

    pub fn with_config(cols: u16, rows: u16, scrollback: usize, config: RendererConfig) -> Self {
        Self {
            parser: TerminalParser::new(cols, rows, scrollback),
            renderer: TerminalRenderer::new(config),
        }
    }

    /// Process input data from SSH
    pub fn process(&mut self, data: &[u8]) {
        self.parser.process(data);
    }

    /// Render the terminal
    pub fn render(&mut self, ui: &mut eframe::egui::Ui) {
        self.renderer.render(ui, self.parser.buffer());
    }

    /// Resize the terminal
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.parser.resize(cols, rows);
    }

    /// Get current terminal size
    pub fn size(&self) -> TerminalSize {
        self.parser.buffer().size()
    }

    /// Get the buffer for reading
    pub fn buffer(&self) -> &TerminalBuffer {
        self.parser.buffer()
    }

    /// Get mutable buffer access
    pub fn buffer_mut(&mut self) -> &mut TerminalBuffer {
        self.parser.buffer_mut()
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.renderer.scroll_to_bottom(self.parser.buffer());
    }

    /// Write text directly (bypassing parser)
    pub fn write(&mut self, text: &str) {
        self.parser.buffer_mut().write_str(text);
    }

    /// Clear the terminal
    pub fn clear(&mut self) {
        self.parser.buffer_mut().clear();
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new(80, 24, 10000)
    }
}
