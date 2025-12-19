//! Terminal cell representation

use super::Color;

/// Text attributes for a cell
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CellAttributes {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub dim: bool,
    pub inverse: bool,
    pub hidden: bool,
    pub blink: bool,
}

impl CellAttributes {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// A single cell in the terminal grid
#[derive(Debug, Clone)]
pub struct Cell {
    pub character: char,
    pub fg: Color,
    pub bg: Color,
    pub attrs: CellAttributes,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            character: ' ',
            fg: Color::WHITE,
            bg: Color::BLACK,
            attrs: CellAttributes::default(),
        }
    }
}

impl Cell {
    pub fn new(character: char) -> Self {
        Self {
            character,
            ..Default::default()
        }
    }

    pub fn with_colors(character: char, fg: Color, bg: Color) -> Self {
        Self {
            character,
            fg,
            bg,
            attrs: CellAttributes::default(),
        }
    }

    pub fn clear(&mut self) {
        self.character = ' ';
        self.attrs = CellAttributes::default();
    }

    pub fn is_empty(&self) -> bool {
        self.character == ' ' || self.character == '\0'
    }
}
