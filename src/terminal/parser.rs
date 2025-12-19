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
            0x0a | 0x0b | 0x0c => {
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
        let param = |i: usize, default: u16| params.get(i).copied().unwrap_or(default);

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
            'J' => {
                match param(0, 0) {
                    0 => self.buffer.clear_to_end(),
                    1 => self.buffer.clear_to_start(),
                    2 | 3 => self.buffer.clear(),
                    _ => {}
                }
            }
            'K' => {
                match param(0, 0) {
                    0 => self.buffer.clear_line_to_end(),
                    1 => self.buffer.clear_line_to_start(),
                    2 => self.buffer.clear_line(),
                    _ => {}
                }
            }
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
                let bottom = param(1, self.buffer.size().rows).saturating_sub(1) as usize;
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
                match *param {
                    4 => self.buffer.set_insert_mode(enable),
                    _ => {}
                }
            }
        }
    }
}

