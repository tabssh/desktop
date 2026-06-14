//! Terminal emulation

pub mod emulator;
pub mod vt;

pub mod buffer;
pub mod cell;
pub mod parser;
pub mod renderer;

pub use emulator::TerminalEmulator;
pub use vt::{VtParser, VtCommand, AnsiColor, CellStyle};
pub use renderer::{RendererConfig, CursorStyle};

/// Alias for the primary terminal emulator type
pub type Terminal = TerminalEmulator;

/// True-color RGB value used across terminal modules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const RED: Self = Self::rgb(205, 49, 49);
    pub const GREEN: Self = Self::rgb(13, 188, 121);
    pub const BLUE: Self = Self::rgb(36, 114, 200);
    pub const YELLOW: Self = Self::rgb(229, 229, 16);
    pub const MAGENTA: Self = Self::rgb(188, 63, 188);
    pub const CYAN: Self = Self::rgb(17, 168, 205);
}

/// Terminal size in character cells
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalSize {
    pub cols: usize,
    pub rows: usize,
}

impl Default for TerminalSize {
    fn default() -> Self {
        Self { cols: 80, rows: 24 }
    }
}
