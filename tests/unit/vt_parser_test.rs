//! VT parser tests

use tabssh::terminal::vt::{VtParser, VtCommand};

#[test]
fn test_simple_text() {
    let mut parser = VtParser::new();
    
    let cmd = parser.parse(b'H');
    assert!(matches!(cmd,Some(VtCommand::Print('H'))));
    
    let cmd = parser.parse(b'i');
    assert!(matches!(cmd,Some(VtCommand::Print('i'))));
}

#[test]
fn test_cursor_movement() {
    let mut parser = VtParser::new();
    
    // ESC [ A (cursor up)
    parser.parse(0x1B);
    parser.parse(b'[');
    let cmd = parser.parse(b'A');
    assert!(matches!(cmd,Some(VtCommand::CursorUp(1))));
}

#[test]
fn test_cursor_position() {
    let mut parser = VtParser::new();
    
    // ESC [ 5 ; 10 H (move to row 5, col 10)
    parser.parse(0x1B);
    parser.parse(b'[');
    parser.parse(b'5');
    parser.parse(b';');
    parser.parse(b'1');
    parser.parse(b'0');
    let cmd = parser.parse(b'H');
    assert!(matches!(cmd,Some(VtCommand::CursorPosition(4,9))));
}

#[test]
fn test_clear_screen() {
    let mut parser = VtParser::new();
    
    // ESC [ 2 J (clear entire screen)
    parser.parse(0x1B);
    parser.parse(b'[');
    parser.parse(b'2');
    let cmd = parser.parse(b'J');
    assert!(matches!(cmd,Some(VtCommand::ClearScreen(2))));
}
