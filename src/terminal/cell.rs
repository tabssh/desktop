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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_attributes_default() {
        let attrs = CellAttributes::default();
        assert!(!attrs.bold);
        assert!(!attrs.italic);
        assert!(!attrs.underline);
        assert!(!attrs.strikethrough);
        assert!(!attrs.dim);
        assert!(!attrs.inverse);
        assert!(!attrs.hidden);
        assert!(!attrs.blink);
    }

    #[test]
    fn test_cell_attributes_reset() {
        let mut attrs = CellAttributes {
            bold: true,
            italic: true,
            underline: true,
            strikethrough: true,
            dim: true,
            inverse: true,
            hidden: true,
            blink: true,
        };
        attrs.reset();
        assert_eq!(attrs, CellAttributes::default());
    }

    #[test]
    fn test_cell_default() {
        let cell = Cell::default();
        assert_eq!(cell.character, ' ');
        assert_eq!(cell.fg, Color::WHITE);
        assert_eq!(cell.bg, Color::BLACK);
        assert_eq!(cell.attrs, CellAttributes::default());
    }

    #[test]
    fn test_cell_new() {
        let cell = Cell::new('x');
        assert_eq!(cell.character, 'x');
        assert_eq!(cell.fg, Color::WHITE);
        assert_eq!(cell.bg, Color::BLACK);
    }

    #[test]
    fn test_cell_with_colors() {
        let cell = Cell::with_colors('A', Color::RED, Color::GREEN);
        assert_eq!(cell.character, 'A');
        assert_eq!(cell.fg, Color::RED);
        assert_eq!(cell.bg, Color::GREEN);
        assert_eq!(cell.attrs, CellAttributes::default());
    }

    #[test]
    fn test_cell_clear() {
        let mut cell = Cell::with_colors('Z', Color::RED, Color::BLUE);
        cell.attrs.bold = true;
        cell.clear();
        assert_eq!(cell.character, ' ');
        assert_eq!(cell.attrs, CellAttributes::default());
        // Clearing does not touch colors, only character and attrs.
        assert_eq!(cell.fg, Color::RED);
        assert_eq!(cell.bg, Color::BLUE);
    }

    #[test]
    fn test_cell_is_empty_space() {
        let cell = Cell::new(' ');
        assert!(cell.is_empty());
    }

    #[test]
    fn test_cell_is_empty_null() {
        let cell = Cell::new('\0');
        assert!(cell.is_empty());
    }

    #[test]
    fn test_cell_is_empty_false_for_visible_char() {
        let cell = Cell::new('x');
        assert!(!cell.is_empty());
    }
}
