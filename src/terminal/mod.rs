//! Terminal emulation

pub mod emulator;
pub mod vt;

pub use emulator::TerminalEmulator;
pub use vt::{VtParser, VtCommand, AnsiColor, CellStyle};
