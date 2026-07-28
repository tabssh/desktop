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
        let size = TerminalSize {
            cols: cols as usize,
            rows: rows as usize,
        };
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

        if self.cursor_x >= self.size.cols {
            if self.auto_wrap {
                self.cursor_x = 0;
                self.newline();
            } else {
                self.cursor_x = self.size.cols - 1;
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
        let max_x = self.size.cols - 1;
        let max_y = self.size.rows - 1;

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
        self.cursor_x = next_tab.min(self.size.cols - 1);
    }

    /// Scroll the screen up by n lines
    pub fn scroll_up(&mut self, n: usize) {
        for _ in 0..n {
            if self.scroll_top == 0 {
                if let Some(row) = self.screen.first().cloned() {
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
                self.screen[self.scroll_bottom] =
                    (0..self.size.cols).map(|_| Cell::default()).collect();
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
                self.screen[self.scroll_top] =
                    (0..self.size.cols).map(|_| Cell::default()).collect();
            }
        }
    }

    /// Clear the screen
    pub fn clear(&mut self) {
        self.screen = Self::create_empty_screen(self.size.cols, self.size.rows);
    }

    /// Clear from cursor to end of screen
    pub fn clear_to_end(&mut self) {
        self.clear_line_to_end();

        for y in (self.cursor_y + 1)..self.size.rows {
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
            for cell in row.iter_mut().skip(self.cursor_x) {
                cell.clear();
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
                    (0..self.size.cols).map(|_| Cell::default()).collect(),
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
                    (0..self.size.cols).map(|_| Cell::default()).collect(),
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
        let max_row = self.size.rows - 1;
        self.scroll_top = top.min(max_row);
        self.scroll_bottom = bottom.min(max_row).max(self.scroll_top);
    }

    /// Reset scroll region to full screen
    pub fn reset_scroll_region(&mut self) {
        self.scroll_top = 0;
        self.scroll_bottom = self.size.rows - 1;
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
        self.size = TerminalSize {
            cols: new_cols,
            rows: new_rows,
        };

        self.cursor_x = self.cursor_x.min(new_cols.saturating_sub(1));
        self.cursor_y = self.cursor_y.min(new_rows.saturating_sub(1));

        self.scroll_bottom = new_rows.saturating_sub(1);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn buf(cols: u16, rows: u16, scrollback: usize) -> TerminalBuffer {
        TerminalBuffer::new(cols, rows, scrollback)
    }

    #[test]
    fn test_write_char_sets_cell_contents_and_advances_cursor() {
        let mut b = buf(10, 5, 10);
        b.write_char('a');
        assert_eq!(b.get_cell(0, 0).unwrap().character, 'a');
        assert_eq!(b.cursor_position(), (1, 0));
    }

    #[test]
    fn test_write_str_writes_all_chars() {
        let mut b = buf(10, 5, 10);
        b.write_str("abc");
        assert_eq!(b.get_cell(0, 0).unwrap().character, 'a');
        assert_eq!(b.get_cell(1, 0).unwrap().character, 'b');
        assert_eq!(b.get_cell(2, 0).unwrap().character, 'c');
        assert_eq!(b.cursor_position(), (3, 0));
    }

    #[test]
    fn test_write_char_records_current_colors_and_attrs() {
        let mut b = buf(10, 5, 10);
        b.set_fg(Color::RED);
        b.set_bg(Color::BLUE);
        let attrs = CellAttributes {
            bold: true,
            ..Default::default()
        };
        b.set_attr(attrs);
        b.write_char('x');
        let cell = b.get_cell(0, 0).unwrap();
        assert_eq!(cell.fg, Color::RED);
        assert_eq!(cell.bg, Color::BLUE);
        assert!(cell.attrs.bold);
    }

    #[test]
    fn test_line_wrap_at_right_edge() {
        let mut b = buf(3, 3, 10);
        b.write_str("abcd");
        // "abc" fills row 0, 'd' wraps onto row 1
        assert_eq!(b.get_cell(0, 0).unwrap().character, 'a');
        assert_eq!(b.get_cell(1, 0).unwrap().character, 'b');
        assert_eq!(b.get_cell(2, 0).unwrap().character, 'c');
        assert_eq!(b.get_cell(0, 1).unwrap().character, 'd');
        assert_eq!(b.cursor_position(), (1, 1));
    }

    #[test]
    fn test_no_wrap_when_auto_wrap_disabled() {
        let mut b = buf(3, 3, 10);
        b.set_auto_wrap(false);
        b.write_str("abcd");
        // Without auto-wrap, cursor clamps at last column and overwrites it.
        assert_eq!(b.get_cell(0, 0).unwrap().character, 'a');
        assert_eq!(b.get_cell(1, 0).unwrap().character, 'b');
        assert_eq!(b.get_cell(2, 0).unwrap().character, 'd');
        assert_eq!(b.cursor_position(), (3, 0));
    }

    #[test]
    fn test_newline_scrolls_when_at_bottom_of_screen() {
        let mut b = buf(5, 2, 10);
        b.write_str("row1");
        b.write_char('\r');
        b.write_char('\n');
        b.write_str("row2");
        b.write_char('\r');
        b.write_char('\n');
        b.write_str("row3");

        // "row1" should have scrolled off into scrollback, "row2"/"row3" remain.
        assert_eq!(b.scrollback_len(), 1);
        let row_text = |row: &[Cell]| row.iter().take(4).map(|c| c.character).collect::<String>();
        assert_eq!(row_text(b.get_scrollback_row(0).unwrap()), "row1");
        assert_eq!(row_text(b.get_row(0).unwrap()), "row2");
        assert_eq!(row_text(b.get_row(1).unwrap()), "row3");
    }

    #[test]
    fn test_scrollback_max_limit_enforced() {
        let mut b = buf(5, 1, 2);
        for i in 0..5 {
            b.write_char(char::from(b'a' + i as u8));
            b.write_char('\n');
        }
        assert!(b.scrollback_len() <= 2);
    }

    #[test]
    fn test_scroll_up_moves_top_row_to_scrollback() {
        let mut b = buf(3, 3, 10);
        b.get_row(0); // sanity access
                      // seed rows with distinct content via direct writes at each row
        b.set_cursor(0, 0);
        b.write_str("AAA");
        b.set_cursor(0, 1);
        b.write_str("BBB");
        b.set_cursor(0, 2);
        b.write_str("CCC");

        b.scroll_up(1);

        assert_eq!(b.scrollback_len(), 1);
        assert_eq!(b.get_scrollback_row(0).unwrap()[0].character, 'A');
        assert_eq!(b.get_row(0).unwrap()[0].character, 'B');
        assert_eq!(b.get_row(1).unwrap()[0].character, 'C');
        // bottom row is now blank
        assert_eq!(b.get_row(2).unwrap()[0].character, ' ');
    }

    #[test]
    fn test_scroll_down_shifts_rows_and_blanks_top() {
        let mut b = buf(3, 3, 10);
        b.set_cursor(0, 0);
        b.write_str("AAA");
        b.set_cursor(0, 1);
        b.write_str("BBB");
        b.set_cursor(0, 2);
        b.write_str("CCC");

        b.scroll_down(1);

        assert_eq!(b.get_row(0).unwrap()[0].character, ' ');
        assert_eq!(b.get_row(1).unwrap()[0].character, 'A');
        assert_eq!(b.get_row(2).unwrap()[0].character, 'B');
    }

    #[test]
    fn test_resize_grow_preserves_content() {
        let mut b = buf(3, 2, 10);
        b.write_str("ab");
        b.resize(6, 4);
        assert_eq!(b.size().cols, 6);
        assert_eq!(b.size().rows, 4);
        assert_eq!(b.get_cell(0, 0).unwrap().character, 'a');
        assert_eq!(b.get_cell(1, 0).unwrap().character, 'b');
        // newly added cells are blank
        assert_eq!(b.get_cell(5, 3).unwrap().character, ' ');
    }

    #[test]
    fn test_resize_shrink_truncates_content_and_clamps_cursor() {
        let mut b = buf(5, 5, 10);
        b.set_cursor(4, 4);
        b.write_str("hello");
        b.resize(2, 2);
        assert_eq!(b.size().cols, 2);
        assert_eq!(b.size().rows, 2);
        let (x, y) = b.cursor_position();
        assert!(x < 2);
        assert!(y < 2);
        // no panic accessing within bounds
        assert!(b.get_cell(0, 0).is_some());
        assert!(b.get_cell(2, 0).is_none());
    }

    #[test]
    fn test_resize_to_1x1_does_not_panic() {
        let mut b = buf(10, 10, 10);
        b.set_cursor(9, 9);
        b.resize(1, 1);
        assert_eq!(b.size().cols, 1);
        assert_eq!(b.size().rows, 1);
        assert_eq!(b.cursor_position(), (0, 0));
        assert!(b.get_cell(0, 0).is_some());
    }

    #[test]
    fn test_resize_to_zero_does_not_panic() {
        let mut b = buf(10, 10, 10);
        b.resize(0, 0);
        assert_eq!(b.size().cols, 0);
        assert_eq!(b.size().rows, 0);
        assert_eq!(b.cursor_position(), (0, 0));
        assert!(b.get_cell(0, 0).is_none());
        assert!(b.get_row(0).is_none());
    }

    #[test]
    fn test_cursor_clamped_within_bounds_via_set_cursor() {
        let mut b = buf(5, 5, 10);
        b.set_cursor(100, 100);
        assert_eq!(b.cursor_position(), (4, 4));
    }

    #[test]
    fn test_move_cursor_never_goes_negative() {
        let mut b = buf(5, 5, 10);
        b.set_cursor(1, 1);
        b.move_cursor(-10, -10);
        assert_eq!(b.cursor_position(), (0, 0));
    }

    #[test]
    fn test_cursor_clamped_after_shrink() {
        let mut b = buf(10, 10, 10);
        b.set_cursor(9, 9);
        b.resize(3, 3);
        let (x, y) = b.cursor_position();
        assert!(x <= 2);
        assert!(y <= 2);
    }

    #[test]
    fn test_clear_screen_blanks_all_cells() {
        let mut b = buf(3, 3, 10);
        b.write_str("abc");
        b.clear();
        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(b.get_cell(x, y).unwrap().character, ' ');
            }
        }
    }

    #[test]
    fn test_clear_line_blanks_current_row_only() {
        let mut b = buf(3, 2, 10);
        b.set_cursor(0, 0);
        b.write_str("abc");
        b.set_cursor(0, 1);
        b.write_str("xyz");
        b.clear_line();
        // cursor is on row 1
        for x in 0..3 {
            assert_eq!(b.get_cell(x, 1).unwrap().character, ' ');
        }
        // row 0 untouched
        assert_eq!(b.get_cell(0, 0).unwrap().character, 'a');
    }

    #[test]
    fn test_clear_line_to_end() {
        let mut b = buf(5, 1, 10);
        b.write_str("abcde");
        b.set_cursor(2, 0);
        b.clear_line_to_end();
        assert_eq!(b.get_cell(0, 0).unwrap().character, 'a');
        assert_eq!(b.get_cell(1, 0).unwrap().character, 'b');
        assert_eq!(b.get_cell(2, 0).unwrap().character, ' ');
        assert_eq!(b.get_cell(3, 0).unwrap().character, ' ');
        assert_eq!(b.get_cell(4, 0).unwrap().character, ' ');
    }

    #[test]
    fn test_clear_line_to_start() {
        let mut b = buf(5, 1, 10);
        b.write_str("abcde");
        b.set_cursor(2, 0);
        b.clear_line_to_start();
        assert_eq!(b.get_cell(0, 0).unwrap().character, ' ');
        assert_eq!(b.get_cell(1, 0).unwrap().character, ' ');
        assert_eq!(b.get_cell(2, 0).unwrap().character, ' ');
        assert_eq!(b.get_cell(3, 0).unwrap().character, 'd');
        assert_eq!(b.get_cell(4, 0).unwrap().character, 'e');
    }

    #[test]
    fn test_clear_to_end_clears_current_and_following_rows() {
        let mut b = buf(3, 3, 10);
        b.set_cursor(0, 0);
        b.write_str("aaa");
        b.set_cursor(0, 1);
        b.write_str("bbb");
        b.set_cursor(1, 1);
        b.clear_to_end();

        assert_eq!(b.get_cell(0, 0).unwrap().character, 'a');
        assert_eq!(b.get_cell(0, 1).unwrap().character, 'b');
        assert_eq!(b.get_cell(1, 1).unwrap().character, ' ');
        assert_eq!(b.get_cell(2, 1).unwrap().character, ' ');
        assert_eq!(b.get_cell(0, 2).unwrap().character, ' ');
    }

    #[test]
    fn test_clear_to_start_clears_preceding_rows_and_line_start() {
        let mut b = buf(3, 3, 10);
        b.set_cursor(0, 0);
        b.write_str("aaa");
        b.set_cursor(0, 1);
        b.write_str("bbb");
        b.set_cursor(1, 1);
        b.clear_to_start();

        assert_eq!(b.get_cell(0, 0).unwrap().character, ' ');
        assert_eq!(b.get_cell(0, 1).unwrap().character, ' ');
        assert_eq!(b.get_cell(1, 1).unwrap().character, ' ');
        assert_eq!(b.get_cell(2, 1).unwrap().character, 'b');
    }

    #[test]
    fn test_insert_blank_shifts_row_right() {
        let mut b = buf(5, 1, 10);
        b.write_str("abcde");
        b.set_cursor(1, 0);
        b.insert_blank(2);
        assert_eq!(b.get_cell(0, 0).unwrap().character, 'a');
        assert_eq!(b.get_cell(1, 0).unwrap().character, ' ');
        assert_eq!(b.get_cell(2, 0).unwrap().character, ' ');
        assert_eq!(b.get_cell(3, 0).unwrap().character, 'b');
        assert_eq!(b.get_cell(4, 0).unwrap().character, 'c');
    }

    #[test]
    fn test_delete_chars_shifts_row_left() {
        let mut b = buf(5, 1, 10);
        b.write_str("abcde");
        b.set_cursor(1, 0);
        b.delete_chars(2);
        assert_eq!(b.get_cell(0, 0).unwrap().character, 'a');
        assert_eq!(b.get_cell(1, 0).unwrap().character, 'd');
        assert_eq!(b.get_cell(2, 0).unwrap().character, 'e');
        assert_eq!(b.get_cell(3, 0).unwrap().character, ' ');
        assert_eq!(b.get_cell(4, 0).unwrap().character, ' ');
    }

    #[test]
    fn test_erase_chars_blanks_without_shifting() {
        let mut b = buf(5, 1, 10);
        b.write_str("abcde");
        b.set_cursor(1, 0);
        b.erase_chars(2);
        assert_eq!(b.get_cell(0, 0).unwrap().character, 'a');
        assert_eq!(b.get_cell(1, 0).unwrap().character, ' ');
        assert_eq!(b.get_cell(2, 0).unwrap().character, ' ');
        assert_eq!(b.get_cell(3, 0).unwrap().character, 'd');
    }

    #[test]
    fn test_insert_and_delete_lines() {
        let mut b = buf(3, 3, 10);
        b.set_cursor(0, 0);
        b.write_str("AAA");
        b.set_cursor(0, 1);
        b.write_str("BBB");
        b.set_cursor(0, 2);
        b.write_str("CCC");

        b.set_cursor(0, 0);
        b.insert_lines(1);
        // top row now blank, A/B pushed down, C dropped off bottom
        assert_eq!(b.get_row(0).unwrap()[0].character, ' ');
        assert_eq!(b.get_row(1).unwrap()[0].character, 'A');
        assert_eq!(b.get_row(2).unwrap()[0].character, 'B');

        b.set_cursor(0, 0);
        b.delete_lines(1);
        // deleting the (blank) top row shifts A back up, blank row appended at bottom
        assert_eq!(b.get_row(0).unwrap()[0].character, 'A');
        assert_eq!(b.get_row(1).unwrap()[0].character, 'B');
        assert_eq!(b.get_row(2).unwrap()[0].character, ' ');
    }

    #[test]
    fn test_save_and_restore_cursor() {
        let mut b = buf(10, 10, 10);
        b.set_cursor(3, 4);
        b.save_cursor();
        b.set_cursor(7, 8);
        assert_eq!(b.cursor_position(), (7, 8));
        b.restore_cursor();
        assert_eq!(b.cursor_position(), (3, 4));
    }

    #[test]
    fn test_scroll_region_confines_newline_scroll() {
        let mut b = buf(5, 5, 10);
        b.set_cursor(0, 0);
        b.write_str("row0");
        b.set_cursor(0, 4);
        b.write_str("row4");

        // Region is rows 1..=3; scrolling within it must not touch row 0 or row 4.
        b.set_scroll_region(1, 3);
        b.set_cursor(0, 1);
        b.write_str("AAA");
        b.set_cursor(0, 2);
        b.write_str("BBB");
        b.set_cursor(0, 3);
        b.write_str("CCC");
        b.set_cursor(0, 3);
        b.write_char('\n');

        assert_eq!(b.get_row(0).unwrap()[0].character, 'r');
        assert_eq!(b.get_row(1).unwrap()[0].character, 'B');
        assert_eq!(b.get_row(2).unwrap()[0].character, 'C');
        assert_eq!(b.get_row(4).unwrap()[0].character, 'r');
    }

    #[test]
    fn test_set_scroll_region_bottom_clamped_to_last_row() {
        let mut b = buf(5, 5, 10);
        // bottom far beyond screen must clamp so scrolling still confines to the screen.
        b.set_scroll_region(2, 9999);
        b.set_cursor(0, 4);
        b.write_char('\n');
        // no panic, cursor stays within bounds
        let (_, y) = b.cursor_position();
        assert!(y < 5);
    }

    #[test]
    fn test_switch_to_alternate_and_back_preserves_main_screen() {
        let mut b = buf(5, 2, 10);
        b.write_str("main1");
        b.switch_to_alternate();
        // alternate screen starts blank
        assert_eq!(b.get_cell(0, 0).unwrap().character, ' ');
        b.write_str("alt");
        b.switch_to_main();
        // main screen content restored
        assert_eq!(b.get_cell(0, 0).unwrap().character, 'm');
        assert_eq!(b.get_cell(4, 0).unwrap().character, '1');
    }

    #[test]
    fn test_reset_attrs_restores_defaults() {
        let mut b = buf(5, 5, 10);
        b.set_fg(Color::RED);
        b.set_bg(Color::GREEN);
        let attrs = CellAttributes {
            bold: true,
            ..Default::default()
        };
        b.set_attr(attrs);
        b.reset_attrs();
        assert_eq!(b.current_attrs(), CellAttributes::default());
        b.write_char('x');
        let cell = b.get_cell(0, 0).unwrap();
        assert_eq!(cell.fg, Color::WHITE);
        assert_eq!(cell.bg, Color::BLACK);
    }

    #[test]
    fn test_origin_mode_resets_cursor_to_scroll_top() {
        let mut b = buf(5, 5, 10);
        b.set_scroll_region(1, 3);
        b.set_origin_mode(true);
        assert_eq!(b.cursor_position(), (0, 1));
    }

    #[test]
    fn test_default_impl_creates_standard_terminal() {
        let b = TerminalBuffer::default();
        assert_eq!(b.size().cols, 80);
        assert_eq!(b.size().rows, 24);
    }
}
