//! Terminal buffer - manages the character grid and scrollback

use super::cell::{Cell, CellAttributes};
use super::{Color, TerminalSize};

/// Terminal buffer with scrollback support
pub struct TerminalBuffer {
    /// Current screen content (rows of cells)
    screen: Vec<Vec<Cell>>,

    /// Scrollback buffer (previous lines)
    scrollback: Vec<Vec<Cell>>,

    /// Maximum scrollback lines
    max_scrollback: usize,

    /// Terminal dimensions
    size: TerminalSize,

    /// Cursor position (0-indexed)
    cursor_x: usize,
    cursor_y: usize,

    /// Saved cursor position for DECSC/DECRC
    saved_cursor_x: usize,
    saved_cursor_y: usize,

    /// Current text attributes
    current_attrs: CellAttributes,

    /// Current foreground color
    current_fg: Color,

    /// Current background color
    current_bg: Color,

    /// Scroll region (top, bottom) - 0-indexed
    scroll_top: usize,
    scroll_bottom: usize,

    /// Alternate screen buffer
    alternate_screen: Option<Vec<Vec<Cell>>>,
    alternate_cursor: Option<(usize, usize)>,

    /// Origin mode (DECOM)
    origin_mode: bool,

    /// Auto-wrap mode
    auto_wrap: bool,

    /// Insert mode
    insert_mode: bool,
}

impl TerminalBuffer {
    pub fn new(cols: u16, rows: u16, max_scrollback: usize) -> Self {
        let size = TerminalSize { cols, rows };
        let screen = Self::create_empty_screen(cols as usize, rows as usize);

        Self {
            screen,
            scrollback: Vec::new(),
            max_scrollback,
            size,
            cursor_x: 0,
            cursor_y: 0,
            saved_cursor_x: 0,
            saved_cursor_y: 0,
            current_attrs: CellAttributes::default(),
            current_fg: Color::WHITE,
            current_bg: Color::BLACK,
            scroll_top: 0,
            scroll_bottom: rows as usize - 1,
            alternate_screen: None,
            alternate_cursor: None,
            origin_mode: false,
            auto_wrap: true,
            insert_mode: false,
        }
    }

    fn create_empty_screen(cols: usize, rows: usize) -> Vec<Vec<Cell>> {
        (0..rows)
            .map(|_| (0..cols).map(|_| Cell::default()).collect())
            .collect()
    }

    pub fn size(&self) -> TerminalSize {
        self.size
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.screen.get(y).and_then(|row| row.get(x))
    }

    pub fn get_row(&self, y: usize) -> Option<&Vec<Cell>> {
        self.screen.get(y)
    }

    pub fn scrollback_len(&self) -> usize {
        self.scrollback.len()
    }

    pub fn get_scrollback_row(&self, index: usize) -> Option<&Vec<Cell>> {
        self.scrollback.get(index)
    }

    /// Write a character at the current cursor position
    pub fn write_char(&mut self, c: char) {
        if c == '\n' {
            self.newline();
            return;
        }

        if c == '\r' {
            self.cursor_x = 0;
            return;
        }

        if c == '\x08' {
            self.backspace();
            return;
        }

        if c == '\t' {
            self.tab();
            return;
        }

        if c == '\x07' {
            return;
        }

        if self.cursor_x >= self.size.cols as usize {
            if self.auto_wrap {
                self.cursor_x = 0;
                self.newline();
            } else {
                self.cursor_x = self.size.cols as usize - 1;
            }
        }

        if self.insert_mode {
            self.insert_blank(1);
        }

        if let Some(row) = self.screen.get_mut(self.cursor_y) {
            if let Some(cell) = row.get_mut(self.cursor_x) {
                cell.character = c;
                cell.fg = self.current_fg;
                cell.bg = self.current_bg;
                cell.attrs = self.current_attrs;
            }
        }

        self.cursor_x += 1;
    }

    /// Write a string at the current cursor position
    pub fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }

    /// Move cursor to absolute position
    pub fn set_cursor(&mut self, x: usize, y: usize) {
        let max_x = self.size.cols as usize - 1;
        let max_y = self.size.rows as usize - 1;

        self.cursor_x = x.min(max_x);

        if self.origin_mode {
            self.cursor_y = (y + self.scroll_top).min(self.scroll_bottom);
        } else {
            self.cursor_y = y.min(max_y);
        }
    }

    /// Move cursor relative to current position
    pub fn move_cursor(&mut self, dx: isize, dy: isize) {
        let new_x = (self.cursor_x as isize + dx).max(0) as usize;
        let new_y = (self.cursor_y as isize + dy).max(0) as usize;
        self.set_cursor(new_x, new_y);
    }

    /// Handle newline
    fn newline(&mut self) {
        if self.cursor_y >= self.scroll_bottom {
            self.scroll_up(1);
        } else {
            self.cursor_y += 1;
        }
    }

    /// Handle backspace
    fn backspace(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    /// Handle tab
    fn tab(&mut self) {
        let next_tab = ((self.cursor_x / 8) + 1) * 8;
        self.cursor_x = next_tab.min(self.size.cols as usize - 1);
    }

    /// Scroll the screen up by n lines
    pub fn scroll_up(&mut self, n: usize) {
        for _ in 0..n {
            if self.scroll_top == 0 {
                if let Some(row) = self.screen.get(0).cloned() {
                    self.scrollback.push(row);

                    while self.scrollback.len() > self.max_scrollback {
                        self.scrollback.remove(0);
                    }
                }
            }

            for y in self.scroll_top..self.scroll_bottom {
                if y + 1 < self.screen.len() {
                    self.screen[y] = self.screen[y + 1].clone();
                }
            }

            if self.scroll_bottom < self.screen.len() {
                self.screen[self.scroll_bottom] = (0..self.size.cols as usize)
                    .map(|_| Cell::default())
                    .collect();
            }
        }
    }

    /// Scroll the screen down by n lines
    pub fn scroll_down(&mut self, n: usize) {
        for _ in 0..n {
            for y in (self.scroll_top + 1..=self.scroll_bottom).rev() {
                if y > 0 && y < self.screen.len() {
                    self.screen[y] = self.screen[y - 1].clone();
                }
            }

            if self.scroll_top < self.screen.len() {
                self.screen[self.scroll_top] = (0..self.size.cols as usize)
                    .map(|_| Cell::default())
                    .collect();
            }
        }
    }

    /// Clear the screen
    pub fn clear(&mut self) {
        self.screen = Self::create_empty_screen(
            self.size.cols as usize,
            self.size.rows as usize,
        );
    }

    /// Clear from cursor to end of screen
    pub fn clear_to_end(&mut self) {
        self.clear_line_to_end();

        for y in (self.cursor_y + 1)..self.size.rows as usize {
            if let Some(row) = self.screen.get_mut(y) {
                for cell in row.iter_mut() {
                    cell.clear();
                }
            }
        }
    }

    /// Clear from start of screen to cursor
    pub fn clear_to_start(&mut self) {
        for y in 0..self.cursor_y {
            if let Some(row) = self.screen.get_mut(y) {
                for cell in row.iter_mut() {
                    cell.clear();
                }
            }
        }

        self.clear_line_to_start();
    }

    /// Clear current line
    pub fn clear_line(&mut self) {
        if let Some(row) = self.screen.get_mut(self.cursor_y) {
            for cell in row.iter_mut() {
                cell.clear();
            }
        }
    }

    /// Clear from cursor to end of line
    pub fn clear_line_to_end(&mut self) {
        if let Some(row) = self.screen.get_mut(self.cursor_y) {
            for x in self.cursor_x..row.len() {
                row[x].clear();
            }
        }
    }

    /// Clear from start of line to cursor
    pub fn clear_line_to_start(&mut self) {
        if let Some(row) = self.screen.get_mut(self.cursor_y) {
            for x in 0..=self.cursor_x.min(row.len() - 1) {
                row[x].clear();
            }
        }
    }

    /// Insert blank characters at cursor
    pub fn insert_blank(&mut self, count: usize) {
        if let Some(row) = self.screen.get_mut(self.cursor_y) {
            for _ in 0..count {
                if self.cursor_x < row.len() {
                    row.insert(self.cursor_x, Cell::default());
                    row.pop();
                }
            }
        }
    }

    /// Delete characters at cursor
    pub fn delete_chars(&mut self, count: usize) {
        if let Some(row) = self.screen.get_mut(self.cursor_y) {
            for _ in 0..count {
                if self.cursor_x < row.len() {
                    row.remove(self.cursor_x);
                    row.push(Cell::default());
                }
            }
        }
    }

    /// Insert blank lines at cursor row
    pub fn insert_lines(&mut self, count: usize) {
        for _ in 0..count {
            if self.cursor_y <= self.scroll_bottom {
                self.screen.remove(self.scroll_bottom);
                self.screen.insert(
                    self.cursor_y,
                    (0..self.size.cols as usize).map(|_| Cell::default()).collect(),
                );
            }
        }
    }

    /// Delete lines at cursor row
    pub fn delete_lines(&mut self, count: usize) {
        for _ in 0..count {
            if self.cursor_y <= self.scroll_bottom {
                self.screen.remove(self.cursor_y);
                self.screen.insert(
                    self.scroll_bottom,
                    (0..self.size.cols as usize).map(|_| Cell::default()).collect(),
                );
            }
        }
    }

    /// Erase characters at cursor position without moving cursor (ECH - CSI X)
    pub fn erase_chars(&mut self, count: usize) {
        if let Some(row) = self.screen.get_mut(self.cursor_y) {
            for i in 0..count {
                if self.cursor_x + i < row.len() {
                    row[self.cursor_x + i].clear();
                }
            }
        }
    }

    /// Save cursor position
    pub fn save_cursor(&mut self) {
        self.saved_cursor_x = self.cursor_x;
        self.saved_cursor_y = self.cursor_y;
    }

    /// Restore cursor position
    pub fn restore_cursor(&mut self) {
        self.cursor_x = self.saved_cursor_x;
        self.cursor_y = self.saved_cursor_y;
    }

    /// Set scroll region
    pub fn set_scroll_region(&mut self, top: usize, bottom: usize) {
        let max_row = self.size.rows as usize - 1;
        self.scroll_top = top.min(max_row);
        self.scroll_bottom = bottom.min(max_row).max(self.scroll_top);
    }

    /// Reset scroll region to full screen
    pub fn reset_scroll_region(&mut self) {
        self.scroll_top = 0;
        self.scroll_bottom = self.size.rows as usize - 1;
    }

    /// Switch to alternate screen buffer
    pub fn switch_to_alternate(&mut self) {
        if self.alternate_screen.is_none() {
            self.alternate_screen = Some(self.screen.clone());
            self.alternate_cursor = Some((self.cursor_x, self.cursor_y));
            self.clear();
            self.cursor_x = 0;
            self.cursor_y = 0;
        }
    }

    /// Switch back to main screen buffer
    pub fn switch_to_main(&mut self) {
        if let Some(main_screen) = self.alternate_screen.take() {
            self.screen = main_screen;
            if let Some((x, y)) = self.alternate_cursor.take() {
                self.cursor_x = x;
                self.cursor_y = y;
            }
        }
    }

    /// Set current foreground color
    pub fn set_fg(&mut self, color: Color) {
        self.current_fg = color;
    }

    /// Set current background color
    pub fn set_bg(&mut self, color: Color) {
        self.current_bg = color;
    }

    /// Set text attribute
    pub fn set_attr(&mut self, attr: CellAttributes) {
        self.current_attrs = attr;
    }

    /// Reset text attributes to defaults
    pub fn reset_attrs(&mut self) {
        self.current_attrs = CellAttributes::default();
        self.current_fg = Color::WHITE;
        self.current_bg = Color::BLACK;
    }

    /// Resize the terminal
    pub fn resize(&mut self, cols: u16, rows: u16) {
        let new_cols = cols as usize;
        let new_rows = rows as usize;

        let mut new_screen = Self::create_empty_screen(new_cols, new_rows);

        for (y, row) in self.screen.iter().enumerate() {
            if y >= new_rows {
                break;
            }
            for (x, cell) in row.iter().enumerate() {
                if x >= new_cols {
                    break;
                }
                new_screen[y][x] = cell.clone();
            }
        }

        self.screen = new_screen;
        self.size = TerminalSize { cols, rows };

        self.cursor_x = self.cursor_x.min(new_cols.saturating_sub(1));
        self.cursor_y = self.cursor_y.min(new_rows.saturating_sub(1));

        self.scroll_bottom = new_rows - 1;
        if self.scroll_top >= new_rows {
            self.scroll_top = 0;
        }
    }

    /// Get current attributes
    pub fn current_attrs(&self) -> CellAttributes {
        self.current_attrs
    }

    /// Set auto-wrap mode
    pub fn set_auto_wrap(&mut self, enabled: bool) {
        self.auto_wrap = enabled;
    }

    /// Set insert mode
    pub fn set_insert_mode(&mut self, enabled: bool) {
        self.insert_mode = enabled;
    }

    /// Set origin mode
    pub fn set_origin_mode(&mut self, enabled: bool) {
        self.origin_mode = enabled;
        if enabled {
            self.set_cursor(0, 0);
        }
    }
}

impl Default for TerminalBuffer {
    fn default() -> Self {
        Self::new(80, 24, 10000)
    }
}
