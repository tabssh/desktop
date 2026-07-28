//! VT100/xterm escape sequence parser

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Color256(u8),
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CellStyle {
    pub foreground: Option<AnsiColor>,
    pub background: Option<AnsiColor>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub reverse: bool,
    pub dim: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub c: char,
    pub style: CellStyle,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            c: ' ',
            style: CellStyle::default(),
        }
    }
}

pub struct VtParser {
    state: ParserState,
    params: Vec<u32>,
    current_param: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParserState {
    Normal,
    Escape,
    Csi,
    OscString,
}

impl VtParser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Normal,
            params: Vec::new(),
            current_param: String::new(),
        }
    }

    pub fn parse(&mut self, byte: u8) -> Option<VtCommand> {
        match self.state {
            ParserState::Normal => {
                if byte == 0x1B {
                    self.state = ParserState::Escape;
                    None
                } else {
                    Some(VtCommand::Print(byte as char))
                }
            }
            ParserState::Escape => match byte {
                b'[' => {
                    self.state = ParserState::Csi;
                    self.params.clear();
                    self.current_param.clear();
                    None
                }
                b']' => {
                    self.state = ParserState::OscString;
                    None
                }
                _ => {
                    self.state = ParserState::Normal;
                    None
                }
            },
            ParserState::Csi => {
                if byte.is_ascii_digit() {
                    self.current_param.push(byte as char);
                    None
                } else if byte == b';' {
                    if let Ok(param) = self.current_param.parse() {
                        self.params.push(param);
                    }
                    self.current_param.clear();
                    None
                } else {
                    if !self.current_param.is_empty() {
                        if let Ok(param) = self.current_param.parse() {
                            self.params.push(param);
                        }
                    }
                    self.state = ParserState::Normal;
                    self.handle_csi_command(byte as char)
                }
            }
            ParserState::OscString => {
                if byte == 0x07 {
                    self.state = ParserState::Normal;
                }
                None
            }
        }
    }

    fn handle_csi_command(&mut self, cmd: char) -> Option<VtCommand> {
        match cmd {
            'A' => Some(VtCommand::CursorUp(
                self.params.first().copied().unwrap_or(1),
            )),
            'B' => Some(VtCommand::CursorDown(
                self.params.first().copied().unwrap_or(1),
            )),
            'C' => Some(VtCommand::CursorForward(
                self.params.first().copied().unwrap_or(1),
            )),
            'D' => Some(VtCommand::CursorBackward(
                self.params.first().copied().unwrap_or(1),
            )),
            'H' => {
                let row = self.params.first().copied().unwrap_or(1).saturating_sub(1);
                let col = self.params.get(1).copied().unwrap_or(1).saturating_sub(1);
                Some(VtCommand::CursorPosition(row, col))
            }
            'J' => Some(VtCommand::ClearScreen(
                self.params.first().copied().unwrap_or(0),
            )),
            'K' => Some(VtCommand::ClearLine(
                self.params.first().copied().unwrap_or(0),
            )),
            'm' => Some(VtCommand::SetGraphicsMode(self.params.clone())),
            _ => None,
        }
    }
}

impl Default for VtParser {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum VtCommand {
    Print(char),
    CursorUp(u32),
    CursorDown(u32),
    CursorForward(u32),
    CursorBackward(u32),
    CursorPosition(u32, u32),
    ClearScreen(u32),
    ClearLine(u32),
    SetGraphicsMode(Vec<u32>),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feed(parser: &mut VtParser, bytes: &[u8]) -> Option<VtCommand> {
        let mut last = None;
        for &b in bytes {
            last = parser.parse(b);
        }
        last
    }

    #[test]
    fn test_default_matches_new() {
        let mut parser = VtParser::default();
        // Default should start in the same state as new(): plain bytes print immediately.
        assert!(matches!(parser.parse(b'x'), Some(VtCommand::Print('x'))));
    }

    #[test]
    fn test_cursor_down_with_explicit_param() {
        let mut parser = VtParser::new();
        let cmd = feed(&mut parser, b"\x1b[7B");
        assert!(matches!(cmd, Some(VtCommand::CursorDown(7))));
    }

    #[test]
    fn test_cursor_forward_default_param() {
        let mut parser = VtParser::new();
        let cmd = feed(&mut parser, b"\x1b[C");
        assert!(matches!(cmd, Some(VtCommand::CursorForward(1))));
    }

    #[test]
    fn test_cursor_backward_with_explicit_param() {
        let mut parser = VtParser::new();
        let cmd = feed(&mut parser, b"\x1b[3D");
        assert!(matches!(cmd, Some(VtCommand::CursorBackward(3))));
    }

    #[test]
    fn test_cursor_position_default_params() {
        let mut parser = VtParser::new();
        let cmd = feed(&mut parser, b"\x1b[H");
        assert!(matches!(cmd, Some(VtCommand::CursorPosition(0, 0))));
    }

    #[test]
    fn test_clear_line_default_param() {
        let mut parser = VtParser::new();
        let cmd = feed(&mut parser, b"\x1b[K");
        assert!(matches!(cmd, Some(VtCommand::ClearLine(0))));
    }

    #[test]
    fn test_clear_line_explicit_param() {
        let mut parser = VtParser::new();
        let cmd = feed(&mut parser, b"\x1b[1K");
        assert!(matches!(cmd, Some(VtCommand::ClearLine(1))));
    }

    #[test]
    fn test_set_graphics_mode_multiple_params() {
        let mut parser = VtParser::new();
        let cmd = feed(&mut parser, b"\x1b[1;31;40m");
        match cmd {
            Some(VtCommand::SetGraphicsMode(params)) => {
                assert_eq!(params, vec![1, 31, 40]);
            }
            other => panic!("expected SetGraphicsMode, got {:?}", other),
        }
    }

    #[test]
    fn test_set_graphics_mode_no_params_resets() {
        let mut parser = VtParser::new();
        let cmd = feed(&mut parser, b"\x1b[m");
        match cmd {
            Some(VtCommand::SetGraphicsMode(params)) => {
                assert!(params.is_empty());
            }
            other => panic!("expected SetGraphicsMode, got {:?}", other),
        }
    }

    #[test]
    fn test_unknown_csi_command_returns_none() {
        let mut parser = VtParser::new();
        let cmd = feed(&mut parser, b"\x1b[Z");
        assert!(cmd.is_none());
    }

    #[test]
    fn test_unknown_escape_sequence_returns_to_normal() {
        let mut parser = VtParser::new();
        // ESC followed by a byte that isn't '[' or ']' should fall back to Normal state.
        parser.parse(0x1B);
        let cmd = parser.parse(b'Q');
        assert!(cmd.is_none());
        // Parser should now treat the next byte as plain text again.
        let cmd = parser.parse(b'x');
        assert!(matches!(cmd, Some(VtCommand::Print('x'))));
    }

    #[test]
    fn test_osc_string_swallowed_until_bell() {
        let mut parser = VtParser::new();
        // ESC ] 0 ; title BEL should produce no commands and return to Normal.
        let mut last = None;
        for &b in b"\x1b]0;my title\x07" {
            last = parser.parse(b);
        }
        assert!(last.is_none());
        // Confirm the parser is back in Normal state afterward.
        let cmd = parser.parse(b'y');
        assert!(matches!(cmd, Some(VtCommand::Print('y'))));
    }

    #[test]
    fn test_csi_with_invalid_param_is_skipped() {
        let mut parser = VtParser::new();
        // A stray ';' with no digits before it should not push a bogus param.
        let cmd = feed(&mut parser, b"\x1b[;5H");
        assert!(matches!(cmd, Some(VtCommand::CursorPosition(4, 0))));
    }

    #[test]
    fn test_cell_style_default() {
        let style = CellStyle::default();
        assert!(style.foreground.is_none());
        assert!(style.background.is_none());
        assert!(!style.bold);
        assert!(!style.italic);
        assert!(!style.underline);
        assert!(!style.reverse);
        assert!(!style.dim);
    }

    #[test]
    fn test_cell_default() {
        let cell = Cell::default();
        assert_eq!(cell.c, ' ');
        assert!(cell.style.foreground.is_none());
        assert!(cell.style.background.is_none());
        assert!(!cell.style.bold);
    }

    #[test]
    fn test_ansi_color_variants_eq() {
        assert_eq!(AnsiColor::Color256(42), AnsiColor::Color256(42));
        assert_ne!(AnsiColor::Color256(42), AnsiColor::Color256(43));
        assert_eq!(AnsiColor::Rgb(1, 2, 3), AnsiColor::Rgb(1, 2, 3));
        assert_ne!(AnsiColor::Red, AnsiColor::Blue);
    }
}
