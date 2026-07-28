//! Terminal parser using vte crate for ANSI escape sequence handling

use super::buffer::TerminalBuffer;
use super::Color;
use vte::{Params, Perform};

/// Standard ANSI colors
const ANSI_COLORS: [Color; 8] = [
    Color::rgb(0, 0, 0),
    Color::rgb(205, 49, 49),
    Color::rgb(13, 188, 121),
    Color::rgb(229, 229, 16),
    Color::rgb(36, 114, 200),
    Color::rgb(188, 63, 188),
    Color::rgb(17, 168, 205),
    Color::rgb(229, 229, 229),
];

/// Bright ANSI colors
const ANSI_BRIGHT_COLORS: [Color; 8] = [
    Color::rgb(102, 102, 102),
    Color::rgb(241, 76, 76),
    Color::rgb(35, 209, 139),
    Color::rgb(245, 245, 67),
    Color::rgb(59, 142, 234),
    Color::rgb(214, 112, 214),
    Color::rgb(41, 184, 219),
    Color::rgb(255, 255, 255),
];

/// Terminal parser that processes escape sequences
pub struct TerminalParser {
    buffer: TerminalBuffer,
    parser: vte::Parser,
}

impl TerminalParser {
    pub fn new(cols: u16, rows: u16, scrollback: usize) -> Self {
        Self {
            buffer: TerminalBuffer::new(cols, rows, scrollback),
            parser: vte::Parser::new(),
        }
    }

    /// Process input bytes
    pub fn process(&mut self, data: &[u8]) {
        let mut performer = TerminalPerformer {
            buffer: &mut self.buffer,
        };

        for byte in data {
            self.parser.advance(&mut performer, *byte);
        }
    }

    /// Get the underlying buffer
    pub fn buffer(&self) -> &TerminalBuffer {
        &self.buffer
    }

    /// Get mutable access to the buffer
    pub fn buffer_mut(&mut self) -> &mut TerminalBuffer {
        &mut self.buffer
    }

    /// Resize the terminal
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.buffer.resize(cols, rows);
    }
}

/// VTE performer that applies escape sequences to the buffer
struct TerminalPerformer<'a> {
    buffer: &'a mut TerminalBuffer,
}

impl<'a> Perform for TerminalPerformer<'a> {
    fn print(&mut self, c: char) {
        self.buffer.write_char(c);
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            0x07 => {}
            0x08 => {
                let (x, _) = self.buffer.cursor_position();
                if x > 0 {
                    self.buffer.move_cursor(-1, 0);
                }
            }
            0x09 => {
                self.buffer.write_char('\t');
            }
            0x0a..=0x0c => {
                self.buffer.write_char('\n');
            }
            0x0d => {
                self.buffer.write_char('\r');
            }
            _ => {}
        }
    }

    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {}

    fn put(&mut self, _byte: u8) {}

    fn unhook(&mut self) {}

    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {}

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], _ignore: bool, c: char) {
        let params: Vec<u16> = params.iter().map(|p| p[0]).collect();
        // Per ECMA-48, an omitted parameter and an explicit value of 0 both
        // mean "use the default" for these commands; vte reports a bare
        // "CSI @"/"CSI L" etc. (no digits) as a single param of 0 rather
        // than an empty list, so unwrap_or(default) alone never fires.
        let param = |i: usize, default: u16| match params.get(i).copied().unwrap_or(default) {
            0 => default,
            v => v,
        };

        match c {
            'A' => {
                let n = param(0, 1).max(1) as isize;
                self.buffer.move_cursor(0, -n);
            }
            'B' => {
                let n = param(0, 1).max(1) as isize;
                self.buffer.move_cursor(0, n);
            }
            'C' => {
                let n = param(0, 1).max(1) as isize;
                self.buffer.move_cursor(n, 0);
            }
            'D' => {
                let n = param(0, 1).max(1) as isize;
                self.buffer.move_cursor(-n, 0);
            }
            'E' => {
                let n = param(0, 1).max(1) as isize;
                self.buffer.move_cursor(0, n);
                let (_, y) = self.buffer.cursor_position();
                self.buffer.set_cursor(0, y);
            }
            'F' => {
                let n = param(0, 1).max(1) as isize;
                self.buffer.move_cursor(0, -n);
                let (_, y) = self.buffer.cursor_position();
                self.buffer.set_cursor(0, y);
            }
            'G' => {
                let col = param(0, 1).saturating_sub(1) as usize;
                let (_, y) = self.buffer.cursor_position();
                self.buffer.set_cursor(col, y);
            }
            'H' | 'f' => {
                let row = param(0, 1).saturating_sub(1) as usize;
                let col = param(1, 1).saturating_sub(1) as usize;
                self.buffer.set_cursor(col, row);
            }
            'J' => match param(0, 0) {
                0 => self.buffer.clear_to_end(),
                1 => self.buffer.clear_to_start(),
                2 | 3 => self.buffer.clear(),
                _ => {}
            },
            'K' => match param(0, 0) {
                0 => self.buffer.clear_line_to_end(),
                1 => self.buffer.clear_line_to_start(),
                2 => self.buffer.clear_line(),
                _ => {}
            },
            'L' => {
                let n = param(0, 1) as usize;
                self.buffer.insert_lines(n);
            }
            'M' => {
                let n = param(0, 1) as usize;
                self.buffer.delete_lines(n);
            }
            'P' => {
                let n = param(0, 1) as usize;
                self.buffer.delete_chars(n);
            }
            'S' => {
                let n = param(0, 1) as usize;
                self.buffer.scroll_up(n);
            }
            'T' => {
                let n = param(0, 1) as usize;
                self.buffer.scroll_down(n);
            }
            'X' => {
                let n = param(0, 1) as usize;
                self.buffer.erase_chars(n);
            }
            '@' => {
                let n = param(0, 1) as usize;
                self.buffer.insert_blank(n);
            }
            'd' => {
                let row = param(0, 1).saturating_sub(1) as usize;
                let (x, _) = self.buffer.cursor_position();
                self.buffer.set_cursor(x, row);
            }
            'm' => {
                self.handle_sgr(&params);
            }
            'r' => {
                let top = param(0, 1).saturating_sub(1) as usize;
                let bottom = param(1, self.buffer.size().rows as u16).saturating_sub(1) as usize;
                self.buffer.set_scroll_region(top, bottom);
            }
            's' => {
                self.buffer.save_cursor();
            }
            'u' => {
                self.buffer.restore_cursor();
            }
            'h' => {
                self.handle_mode(intermediates, &params, true);
            }
            'l' => {
                self.handle_mode(intermediates, &params, false);
            }
            _ => {}
        }
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], _ignore: bool, byte: u8) {
        match (intermediates, byte) {
            ([], b'7') => self.buffer.save_cursor(),
            ([], b'8') => self.buffer.restore_cursor(),
            ([], b'D') => self.buffer.scroll_up(1),
            ([], b'M') => self.buffer.scroll_down(1),
            ([], b'c') => {
                self.buffer.clear();
                self.buffer.reset_attrs();
                self.buffer.set_cursor(0, 0);
            }
            _ => {}
        }
    }
}

impl<'a> TerminalPerformer<'a> {
    /// Handle SGR (Select Graphic Rendition) sequences
    fn handle_sgr(&mut self, params: &[u16]) {
        if params.is_empty() {
            self.buffer.reset_attrs();
            return;
        }

        let mut i = 0;
        while i < params.len() {
            match params[i] {
                0 => self.buffer.reset_attrs(),
                1 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.bold = true;
                    self.buffer.set_attr(attrs);
                }
                2 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.dim = true;
                    self.buffer.set_attr(attrs);
                }
                3 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.italic = true;
                    self.buffer.set_attr(attrs);
                }
                4 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.underline = true;
                    self.buffer.set_attr(attrs);
                }
                5 | 6 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.blink = true;
                    self.buffer.set_attr(attrs);
                }
                7 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.inverse = true;
                    self.buffer.set_attr(attrs);
                }
                8 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.hidden = true;
                    self.buffer.set_attr(attrs);
                }
                9 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.strikethrough = true;
                    self.buffer.set_attr(attrs);
                }
                21 | 22 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.bold = false;
                    attrs.dim = false;
                    self.buffer.set_attr(attrs);
                }
                23 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.italic = false;
                    self.buffer.set_attr(attrs);
                }
                24 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.underline = false;
                    self.buffer.set_attr(attrs);
                }
                25 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.blink = false;
                    self.buffer.set_attr(attrs);
                }
                27 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.inverse = false;
                    self.buffer.set_attr(attrs);
                }
                28 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.hidden = false;
                    self.buffer.set_attr(attrs);
                }
                29 => {
                    let mut attrs = self.buffer.current_attrs();
                    attrs.strikethrough = false;
                    self.buffer.set_attr(attrs);
                }
                30..=37 => {
                    let color_idx = (params[i] - 30) as usize;
                    self.buffer.set_fg(ANSI_COLORS[color_idx]);
                }
                38 => {
                    if i + 2 < params.len() && params[i + 1] == 5 {
                        let color = self.color_from_256(params[i + 2]);
                        self.buffer.set_fg(color);
                        i += 2;
                    } else if i + 4 < params.len() && params[i + 1] == 2 {
                        let color = Color::rgb(
                            params[i + 2] as u8,
                            params[i + 3] as u8,
                            params[i + 4] as u8,
                        );
                        self.buffer.set_fg(color);
                        i += 4;
                    }
                }
                39 => self.buffer.set_fg(Color::WHITE),
                40..=47 => {
                    let color_idx = (params[i] - 40) as usize;
                    self.buffer.set_bg(ANSI_COLORS[color_idx]);
                }
                48 => {
                    if i + 2 < params.len() && params[i + 1] == 5 {
                        let color = self.color_from_256(params[i + 2]);
                        self.buffer.set_bg(color);
                        i += 2;
                    } else if i + 4 < params.len() && params[i + 1] == 2 {
                        let color = Color::rgb(
                            params[i + 2] as u8,
                            params[i + 3] as u8,
                            params[i + 4] as u8,
                        );
                        self.buffer.set_bg(color);
                        i += 4;
                    }
                }
                49 => self.buffer.set_bg(Color::BLACK),
                90..=97 => {
                    let color_idx = (params[i] - 90) as usize;
                    self.buffer.set_fg(ANSI_BRIGHT_COLORS[color_idx]);
                }
                100..=107 => {
                    let color_idx = (params[i] - 100) as usize;
                    self.buffer.set_bg(ANSI_BRIGHT_COLORS[color_idx]);
                }
                _ => {}
            }
            i += 1;
        }
    }

    /// Convert 256-color index to RGB
    fn color_from_256(&self, idx: u16) -> Color {
        match idx {
            0..=7 => ANSI_COLORS[idx as usize],
            8..=15 => ANSI_BRIGHT_COLORS[(idx - 8) as usize],
            16..=231 => {
                let idx = idx - 16;
                let r = (idx / 36) * 51;
                let g = ((idx / 6) % 6) * 51;
                let b = (idx % 6) * 51;
                Color::rgb(r as u8, g as u8, b as u8)
            }
            232..=255 => {
                let gray = ((idx - 232) * 10 + 8) as u8;
                Color::rgb(gray, gray, gray)
            }
            _ => Color::WHITE,
        }
    }

    /// Handle mode settings (DECSET/DECRST)
    fn handle_mode(&mut self, intermediates: &[u8], params: &[u16], enable: bool) {
        let is_dec = intermediates.contains(&b'?');

        for param in params {
            if is_dec {
                match *param {
                    1 => {}
                    6 => self.buffer.set_origin_mode(enable),
                    7 => self.buffer.set_auto_wrap(enable),
                    25 => {}
                    47 | 1047 => {
                        if enable {
                            self.buffer.switch_to_alternate();
                        } else {
                            self.buffer.switch_to_main();
                        }
                    }
                    1049 => {
                        if enable {
                            self.buffer.save_cursor();
                            self.buffer.switch_to_alternate();
                            self.buffer.clear();
                        } else {
                            self.buffer.switch_to_main();
                            self.buffer.restore_cursor();
                        }
                    }
                    _ => {}
                }
            } else {
                if *param == 4 {
                    self.buffer.set_insert_mode(enable)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parser() -> TerminalParser {
        TerminalParser::new(10, 5, 100)
    }

    #[test]
    fn test_plain_printable_text() {
        let mut p = parser();
        p.process(b"hello");
        let buf = p.buffer();
        assert_eq!(buf.get_cell(0, 0).unwrap().character, 'h');
        assert_eq!(buf.get_cell(1, 0).unwrap().character, 'e');
        assert_eq!(buf.get_cell(4, 0).unwrap().character, 'o');
        assert_eq!(buf.cursor_position(), (5, 0));
    }

    #[test]
    fn test_empty_input() {
        let mut p = parser();
        p.process(b"");
        let buf = p.buffer();
        assert_eq!(buf.cursor_position(), (0, 0));
        assert_eq!(buf.get_cell(0, 0).unwrap().character, ' ');
    }

    #[test]
    fn test_c0_newline_carriage_return() {
        let mut p = parser();
        p.process(b"ab\r\ncd");
        let buf = p.buffer();
        // \r moves to col 0, \n moves to next row
        assert_eq!(buf.get_cell(0, 0).unwrap().character, 'a');
        assert_eq!(buf.get_cell(1, 0).unwrap().character, 'b');
        assert_eq!(buf.get_cell(0, 1).unwrap().character, 'c');
        assert_eq!(buf.get_cell(1, 1).unwrap().character, 'd');
    }

    #[test]
    fn test_c0_tab() {
        let mut p = parser();
        p.process(b"\t");
        assert_eq!(p.buffer().cursor_position(), (8, 0));
    }

    #[test]
    fn test_c0_backspace() {
        let mut p = parser();
        p.process(b"abc\x08");
        assert_eq!(p.buffer().cursor_position(), (2, 0));
    }

    #[test]
    fn test_csi_cursor_up_down_left_right() {
        let mut p = parser();
        // move to middle of screen first
        p.process(b"\x1b[3;3H");
        assert_eq!(p.buffer().cursor_position(), (2, 2));

        p.process(b"\x1b[A"); // up 1
        assert_eq!(p.buffer().cursor_position(), (2, 1));

        p.process(b"\x1b[2B"); // down 2
        assert_eq!(p.buffer().cursor_position(), (2, 3));

        p.process(b"\x1b[C"); // right 1
        assert_eq!(p.buffer().cursor_position(), (3, 3));

        p.process(b"\x1b[2D"); // left 2
        assert_eq!(p.buffer().cursor_position(), (1, 3));
    }

    #[test]
    fn test_csi_cursor_position_default_params() {
        let mut p = parser();
        p.process(b"\x1b[H");
        assert_eq!(p.buffer().cursor_position(), (0, 0));

        p.process(b"\x1b[4;5H");
        assert_eq!(p.buffer().cursor_position(), (4, 3));
    }

    #[test]
    fn test_csi_sgr_bold_and_reset() {
        let mut p = parser();
        p.process(b"\x1b[1mA\x1b[0mB");
        let buf = p.buffer();
        assert!(buf.get_cell(0, 0).unwrap().attrs.bold);
        assert!(!buf.get_cell(1, 0).unwrap().attrs.bold);
    }

    #[test]
    fn test_csi_sgr_multiple_params() {
        let mut p = parser();
        // bold + underline + red fg in one sequence
        p.process(b"\x1b[1;4;31mA");
        let cell = p.buffer().get_cell(0, 0).unwrap();
        assert!(cell.attrs.bold);
        assert!(cell.attrs.underline);
        assert_eq!(cell.fg, Color::rgb(205, 49, 49));
    }

    #[test]
    fn test_csi_sgr_basic_fg_bg_colors() {
        let mut p = parser();
        p.process(b"\x1b[32;44mA");
        let cell = p.buffer().get_cell(0, 0).unwrap();
        assert_eq!(cell.fg, Color::rgb(13, 188, 121));
        assert_eq!(cell.bg, Color::rgb(36, 114, 200));
    }

    #[test]
    fn test_csi_sgr_bright_fg_bg_colors() {
        let mut p = parser();
        p.process(b"\x1b[92;104mA");
        let cell = p.buffer().get_cell(0, 0).unwrap();
        assert_eq!(cell.fg, ANSI_BRIGHT_COLORS[2]);
        assert_eq!(cell.bg, ANSI_BRIGHT_COLORS[4]);
    }

    #[test]
    fn test_csi_sgr_256_color() {
        let mut p = parser();
        p.process(b"\x1b[38;5;196mA");
        let cell = p.buffer().get_cell(0, 0).unwrap();
        // index 196 is in the 16..=231 cube range
        let idx = 196u16 - 16;
        let r = ((idx / 36) * 51) as u8;
        let g = (((idx / 6) % 6) * 51) as u8;
        let b = ((idx % 6) * 51) as u8;
        assert_eq!(cell.fg, Color::rgb(r, g, b));
    }

    #[test]
    fn test_csi_sgr_rgb_truecolor() {
        let mut p = parser();
        p.process(b"\x1b[38;2;10;20;30mA");
        let cell = p.buffer().get_cell(0, 0).unwrap();
        assert_eq!(cell.fg, Color::rgb(10, 20, 30));
    }

    #[test]
    fn test_csi_sgr_fg_bg_reset_defaults() {
        let mut p = parser();
        p.process(b"\x1b[31;41mA\x1b[39;49mB");
        let buf = p.buffer();
        assert_eq!(buf.get_cell(0, 0).unwrap().fg, Color::rgb(205, 49, 49));
        assert_eq!(buf.get_cell(1, 0).unwrap().fg, Color::WHITE);
        assert_eq!(buf.get_cell(1, 0).unwrap().bg, Color::BLACK);
    }

    #[test]
    fn test_csi_clear_screen() {
        let mut p = parser();
        p.process(b"hello\x1b[2J");
        let buf = p.buffer();
        for x in 0..5 {
            assert_eq!(buf.get_cell(x, 0).unwrap().character, ' ');
        }
    }

    #[test]
    fn test_csi_clear_line() {
        let mut p = parser();
        p.process(b"hello\r\x1b[K");
        let buf = p.buffer();
        for x in 0..5 {
            assert_eq!(buf.get_cell(x, 0).unwrap().character, ' ');
        }
    }

    #[test]
    fn test_osc_window_title_bel_terminated() {
        let mut p = parser();
        // OSC 0 ; title BEL - parser currently ignores osc_dispatch, but must
        // not corrupt buffer state or panic, and subsequent text still prints.
        p.process(b"\x1b]0;My Title\x07X");
        assert_eq!(p.buffer().get_cell(0, 0).unwrap().character, 'X');
    }

    #[test]
    fn test_osc_window_title_st_terminated() {
        let mut p = parser();
        // OSC terminated with ESC \\ (ST) instead of BEL
        p.process(b"\x1b]0;My Title\x1b\\X");
        assert_eq!(p.buffer().get_cell(0, 0).unwrap().character, 'X');
    }

    #[test]
    fn test_lone_esc_no_followup() {
        let mut p = parser();
        p.process(b"A\x1b");
        // Should not panic; printable char before the lone ESC is retained.
        assert_eq!(p.buffer().get_cell(0, 0).unwrap().character, 'A');
    }

    #[test]
    fn test_truncated_csi_no_terminator() {
        let mut p = parser();
        p.process(b"A\x1b[1;2");
        // No final byte supplied; parser must not panic and prior state stays intact.
        assert_eq!(p.buffer().get_cell(0, 0).unwrap().character, 'A');

        // Feeding the terminator afterwards completes the sequence.
        p.process(b"H");
        assert_eq!(p.buffer().cursor_position(), (1, 0));
    }

    #[test]
    fn test_csi_garbage_intermediate_bytes() {
        let mut p = parser();
        // Unsupported intermediate byte followed by a valid final byte should
        // not panic and should be effectively ignored/handled gracefully.
        p.process(b"A\x1b[1 zB");
        assert_eq!(p.buffer().get_cell(0, 0).unwrap().character, 'A');
        assert_eq!(p.buffer().get_cell(1, 0).unwrap().character, 'B');
    }

    #[test]
    fn test_csi_param_empty_between_semicolons_uses_default() {
        let mut p = parser();
        // Empty param between semicolons should behave as default (0/omitted).
        p.process(b"\x1b[;5H");
        assert_eq!(p.buffer().cursor_position(), (4, 0));
    }

    #[test]
    fn test_csi_param_leading_zeros() {
        let mut p = parser();
        // Row 007 (0-based 6) exceeds the 5-row test buffer, so set_cursor
        // clamps it to the last row (4); col 003 (0-based 2) is unaffected.
        p.process(b"\x1b[007;003H");
        assert_eq!(p.buffer().cursor_position(), (2, 4));
    }

    #[test]
    fn test_csi_param_very_large_value_clamped_by_buffer() {
        let mut p = parser();
        // Large row/col values must be clamped by the buffer, not panic.
        p.process(b"\x1b[9999;9999H");
        let (x, y) = p.buffer().cursor_position();
        assert_eq!(x, p.buffer().size().cols - 1);
        assert_eq!(y, p.buffer().size().rows - 1);
    }

    #[test]
    fn test_resize_delegates_to_buffer() {
        let mut p = parser();
        p.resize(20, 8);
        assert_eq!(p.buffer().size().cols, 20);
        assert_eq!(p.buffer().size().rows, 8);
    }

    #[test]
    fn test_buffer_mut_allows_direct_mutation() {
        let mut p = parser();
        p.buffer_mut().write_char('Z');
        assert_eq!(p.buffer().get_cell(0, 0).unwrap().character, 'Z');
    }

    #[test]
    fn test_csi_cursor_next_line_e() {
        let mut p = parser();
        p.process(b"\x1b[3;3H"); // (2, 2)
        p.process(b"\x1b[2E"); // down 2 rows, col resets to 0
        assert_eq!(p.buffer().cursor_position(), (0, 4));
    }

    #[test]
    fn test_csi_cursor_prev_line_f() {
        let mut p = parser();
        p.process(b"\x1b[3;3H"); // (2, 2)
        p.process(b"\x1b[1F"); // up 1 row, col resets to 0
        assert_eq!(p.buffer().cursor_position(), (0, 1));
    }

    #[test]
    fn test_csi_cursor_horizontal_absolute_g() {
        let mut p = parser();
        p.process(b"\x1b[3;3H"); // (2, 2)
        p.process(b"\x1b[6G"); // column 6 (1-based) -> 5, row unchanged
        assert_eq!(p.buffer().cursor_position(), (5, 2));
    }

    #[test]
    fn test_csi_vertical_position_absolute_d() {
        let mut p = parser();
        p.process(b"\x1b[3;3H"); // (2, 2)
        p.process(b"\x1b[5d"); // row 5 (1-based) -> 4, col unchanged
        assert_eq!(p.buffer().cursor_position(), (2, 4));
    }

    #[test]
    fn test_csi_insert_and_delete_lines() {
        let mut p = parser();
        p.process(b"aaa\r\nbbb\r\nccc");
        p.process(b"\x1b[2;1H"); // row 1 (0-based)
        p.process(b"\x1b[L"); // insert one blank line at row 1
        let buf = p.buffer();
        assert_eq!(buf.get_cell(0, 0).unwrap().character, 'a');
        assert_eq!(buf.get_cell(0, 1).unwrap().character, ' ');
        assert_eq!(buf.get_cell(0, 2).unwrap().character, 'b');

        p.process(b"\x1b[M"); // delete the blank line back out
        let buf = p.buffer();
        assert_eq!(buf.get_cell(0, 1).unwrap().character, 'b');
    }

    #[test]
    fn test_csi_delete_and_insert_chars() {
        let mut p = parser();
        p.process(b"abcde\r");
        p.process(b"\x1b[2P"); // delete 2 chars at cursor (col 0)
        let buf = p.buffer();
        assert_eq!(buf.get_cell(0, 0).unwrap().character, 'c');
        assert_eq!(buf.get_cell(1, 0).unwrap().character, 'd');
        assert_eq!(buf.get_cell(2, 0).unwrap().character, 'e');

        p.process(b"\r\x1b[@"); // insert 1 blank at col 0
        let buf = p.buffer();
        assert_eq!(buf.get_cell(0, 0).unwrap().character, ' ');
        assert_eq!(buf.get_cell(1, 0).unwrap().character, 'c');
    }

    #[test]
    fn test_csi_erase_chars_x() {
        let mut p = parser();
        p.process(b"abcde\r");
        p.process(b"\x1b[3X"); // erase 3 chars at cursor without moving it
        let buf = p.buffer();
        assert_eq!(buf.get_cell(0, 0).unwrap().character, ' ');
        assert_eq!(buf.get_cell(1, 0).unwrap().character, ' ');
        assert_eq!(buf.get_cell(2, 0).unwrap().character, ' ');
        assert_eq!(buf.get_cell(3, 0).unwrap().character, 'd');
    }

    #[test]
    fn test_csi_scroll_up_and_down_s_t() {
        let mut p = parser();
        p.process(b"line1\r\nline2\r\nline3");
        p.process(b"\x1b[1S"); // scroll whole buffer up 1: line1 moves to scrollback
        assert_eq!(p.buffer().scrollback_len(), 1);
        assert_eq!(p.buffer().get_row(0).unwrap()[0].character, 'l');

        p.process(b"\x1b[1T"); // scroll down 1: top row becomes blank again
        assert_eq!(p.buffer().get_row(0).unwrap()[0].character, ' ');
    }

    #[test]
    fn test_csi_save_restore_cursor_s_u() {
        let mut p = parser();
        p.process(b"\x1b[3;4H"); // (3, 2)
        p.process(b"\x1b[s"); // CSI s save
        p.process(b"\x1b[1;1H");
        assert_eq!(p.buffer().cursor_position(), (0, 0));
        p.process(b"\x1b[u"); // CSI u restore
        assert_eq!(p.buffer().cursor_position(), (3, 2));
    }

    #[test]
    fn test_csi_scroll_region_and_origin_mode() {
        let mut p = parser();
        // Set scroll region rows 2..=4 (1-based) -> 1..=3 (0-based)
        p.process(b"\x1b[2;4r");
        // Enable origin mode (DECOM)
        p.process(b"\x1b[?6h");
        // With origin mode on, row 0 is relative to the scroll region top.
        p.process(b"\x1b[1;1H");
        assert_eq!(p.buffer().cursor_position(), (0, 1));

        // Disable origin mode; absolute addressing resumes.
        p.process(b"\x1b[?6l");
        p.process(b"\x1b[1;1H");
        assert_eq!(p.buffer().cursor_position(), (0, 0));
    }

    #[test]
    fn test_csi_auto_wrap_mode_toggle() {
        let mut p = parser();
        // Disable auto-wrap (DECAWM); writing past the last column must clamp
        // the cursor rather than wrap to the next line.
        p.process(b"\x1b[?7l");
        p.process(b"0123456789ABC");
        assert_eq!(p.buffer().cursor_position().1, 0);

        // Re-enable auto-wrap; overflowing text now wraps to row 1.
        let mut p2 = parser();
        p2.process(b"\x1b[?7h");
        p2.process(b"0123456789A");
        assert_eq!(p2.buffer().cursor_position().1, 1);
    }

    #[test]
    fn test_csi_insert_mode_toggle() {
        let mut p = parser();
        p.process(b"abcde\r");
        // Enable insert mode (IRM, ANSI mode 4 without '?')
        p.process(b"\x1b[4h");
        p.process(b"X");
        let buf = p.buffer();
        assert_eq!(buf.get_cell(0, 0).unwrap().character, 'X');
        assert_eq!(buf.get_cell(1, 0).unwrap().character, 'a');

        // Disable insert mode; subsequent writes overwrite in place again.
        p.process(b"\r\x1b[4l");
        p.process(b"Y");
        let buf = p.buffer();
        assert_eq!(buf.get_cell(0, 0).unwrap().character, 'Y');
        assert_eq!(buf.get_cell(1, 0).unwrap().character, 'a');
    }

    #[test]
    fn test_csi_alternate_screen_1047() {
        let mut p = parser();
        p.process(b"main screen text");
        p.process(b"\x1b[?1047h"); // switch to alternate
        assert_eq!(p.buffer().get_cell(0, 0).unwrap().character, ' ');
        p.process(b"alt");
        p.process(b"\x1b[?1047l"); // switch back to main
        assert_eq!(p.buffer().get_cell(0, 0).unwrap().character, 'm');
    }

    #[test]
    fn test_csi_alternate_screen_1049_saves_and_restores_cursor() {
        let mut p = parser();
        p.process(b"\x1b[3;3H"); // (2, 2) on main screen
        p.process(b"\x1b[?1049h"); // save cursor, switch to alt, clear
        assert_eq!(p.buffer().cursor_position(), (0, 0));
        p.process(b"\x1b[?1049l"); // switch back to main, restore cursor
        assert_eq!(p.buffer().cursor_position(), (2, 2));
    }

    #[test]
    fn test_csi_unknown_dec_private_mode_is_noop() {
        let mut p = parser();
        // Modes 1 and 25 are recognized-but-ignored; an entirely unknown one
        // (e.g. 9999) must also be a no-op rather than panicking.
        p.process(b"\x1b[?9999h");
        p.process(b"A");
        assert_eq!(p.buffer().get_cell(0, 0).unwrap().character, 'A');
    }

    #[test]
    fn test_csi_sgr_reset_individual_attrs() {
        let mut p = parser();
        // Set every toggle-able attribute, then reset each one individually
        // and verify it (and only it) clears.
        p.process(b"\x1b[1;2;3;4;5;7;8;9m");
        {
            // SGR alone writes no character; the cell at (0,0) still exists
            // (it's in-bounds) but should be untouched by the pending attrs.
            let cell = p.buffer().get_cell(0, 0).unwrap();
            assert_eq!(cell.character, ' ');
            assert!(!cell.attrs.bold);
        }
        p.process(b"X\x1b[21;23;24;25;27;28;29mY");
        let buf = p.buffer();
        let set = buf.get_cell(0, 0).unwrap();
        assert!(set.attrs.bold);
        assert!(set.attrs.dim);
        assert!(set.attrs.italic);
        assert!(set.attrs.underline);
        assert!(set.attrs.blink);
        assert!(set.attrs.inverse);
        assert!(set.attrs.hidden);
        assert!(set.attrs.strikethrough);

        let reset = buf.get_cell(1, 0).unwrap();
        assert!(!reset.attrs.bold);
        assert!(!reset.attrs.dim);
        assert!(!reset.attrs.italic);
        assert!(!reset.attrs.underline);
        assert!(!reset.attrs.blink);
        assert!(!reset.attrs.inverse);
        assert!(!reset.attrs.hidden);
        assert!(!reset.attrs.strikethrough);
    }

    #[test]
    fn test_csi_sgr_256_color_low_and_gray_ranges() {
        let mut p = parser();
        // idx 3 falls in the 0..=7 basic-color passthrough range.
        p.process(b"\x1b[38;5;3mA");
        assert_eq!(p.buffer().get_cell(0, 0).unwrap().fg, ANSI_COLORS[3]);

        // idx 250 falls in the 232..=255 grayscale ramp.
        p.process(b"\x1b[38;5;250mB");
        let gray = ((250u16 - 232) * 10 + 8) as u8;
        assert_eq!(
            p.buffer().get_cell(1, 0).unwrap().fg,
            Color::rgb(gray, gray, gray)
        );

        // idx 10 falls in the 8..=15 bright-color range.
        p.process(b"\x1b[38;5;10mC");
        assert_eq!(p.buffer().get_cell(2, 0).unwrap().fg, ANSI_BRIGHT_COLORS[2]);
    }

    #[test]
    fn test_csi_sgr_256_bg_color() {
        let mut p = parser();
        p.process(b"\x1b[48;5;196mA");
        let idx = 196u16 - 16;
        let r = ((idx / 36) * 51) as u8;
        let g = (((idx / 6) % 6) * 51) as u8;
        let b = ((idx % 6) * 51) as u8;
        assert_eq!(p.buffer().get_cell(0, 0).unwrap().bg, Color::rgb(r, g, b));
    }

    #[test]
    fn test_csi_sgr_rgb_truecolor_bg() {
        let mut p = parser();
        p.process(b"\x1b[48;2;1;2;3mA");
        assert_eq!(p.buffer().get_cell(0, 0).unwrap().bg, Color::rgb(1, 2, 3));
    }

    #[test]
    fn test_esc_save_restore_cursor_7_8() {
        let mut p = parser();
        p.process(b"\x1b[3;3H"); // (2, 2)
        p.process(b"\x1b7"); // ESC 7 save
        p.process(b"\x1b[1;1H");
        p.process(b"\x1b8"); // ESC 8 restore
        assert_eq!(p.buffer().cursor_position(), (2, 2));
    }

    #[test]
    fn test_esc_scroll_up_down_d_m() {
        let mut p = parser();
        p.process(b"line1\r\nline2\r\nline3");
        p.process(b"\x1bD"); // ESC D scrolls the buffer up by 1
        assert_eq!(p.buffer().scrollback_len(), 1);

        p.process(b"\x1bM"); // ESC M scrolls back down by 1
        assert_eq!(p.buffer().get_row(0).unwrap()[0].character, ' ');
    }

    #[test]
    fn test_esc_reset_terminal_c() {
        let mut p = parser();
        p.process(b"\x1b[1mhello");
        p.process(b"\x1bc"); // ESC c full reset (RIS)
        let buf = p.buffer();
        assert_eq!(buf.cursor_position(), (0, 0));
        assert_eq!(buf.get_cell(0, 0).unwrap().character, ' ');
        assert!(!buf.current_attrs().bold);
    }

    #[test]
    fn test_esc_unknown_sequence_is_noop() {
        let mut p = parser();
        p.process(b"A\x1bZB");
        // Unrecognized ESC dispatch (no intermediates, byte 'Z') must not
        // panic and must not disturb surrounding text.
        assert_eq!(p.buffer().get_cell(0, 0).unwrap().character, 'A');
        assert_eq!(p.buffer().get_cell(1, 0).unwrap().character, 'B');
    }
}
