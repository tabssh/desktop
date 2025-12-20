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

#[derive(Debug, Clone, Copy)]
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

impl Default for CellStyle {
    fn default() -> Self {
        Self {
            foreground: None,
            background: None,
            bold: false,
            italic: false,
            underline: false,
            reverse: false,
            dim: false,
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
            ParserState::Escape => {
                match byte {
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
                }
            }
            ParserState::Csi => {
                if byte >= b'0' && byte <= b'9' {
                    self.current_param.push(byte as char);
                    None
                } else if byte == b';' {
                    if let Ok(param) = self.current_param.parse() {
                        self.params.push(param);
                    }
                    self.current_param.clear();
                    None
                } else {
                    if !self.current_param.is_empty(){
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
            'A' => Some(VtCommand::CursorUp(self.params.first().copied().unwrap_or(1))),
            'B' => Some(VtCommand::CursorDown(self.params.first().copied().unwrap_or(1))),
            'C' => Some(VtCommand::CursorForward(self.params.first().copied().unwrap_or(1))),
            'D' => Some(VtCommand::CursorBackward(self.params.first().copied().unwrap_or(1))),
            'H' => {
                let row = self.params.get(0).copied().unwrap_or(1).saturating_sub(1);
                let col = self.params.get(1).copied().unwrap_or(1).saturating_sub(1);
                Some(VtCommand::CursorPosition(row, col))
            }
            'J' => Some(VtCommand::ClearScreen(self.params.first().copied().unwrap_or(0))),
            'K' => Some(VtCommand::ClearLine(self.params.first().copied().unwrap_or(0))),
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
