//! Keyboard shortcut handling

use egui::{Context, Key, Modifiers};

pub struct KeyboardHandler;

impl KeyboardHandler {
    pub fn handle_shortcuts(ctx: &Context) -> Option<KeyboardAction> {
        ctx.input(|i| {
            // Ctrl+T - New tab
            if i.modifiers.ctrl && i.key_pressed(Key::T) {
                return Some(KeyboardAction::NewTab);
            }
            
            // Ctrl+W - Close tab
            if i.modifiers.ctrl && i.key_pressed(Key::W) {
                return Some(KeyboardAction::CloseTab);
            }
            
            // Ctrl+Tab - Next tab
            if i.modifiers.ctrl && i.key_pressed(Key::Tab) {
                return Some(KeyboardAction::NextTab);
            }
            
            // Ctrl+Shift+Tab - Previous tab
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(Key::Tab) {
                return Some(KeyboardAction::PreviousTab);
            }
            
            // Ctrl+N - New connection
            if i.modifiers.ctrl && i.key_pressed(Key::N) {
                return Some(KeyboardAction::NewConnection);
            }
            
            // Ctrl+, - Settings
            if i.modifiers.ctrl && i.key_pressed(Key::Comma) {
                return Some(KeyboardAction::OpenSettings);
            }
            
            // Ctrl+Q - Quit
            if i.modifiers.ctrl && i.key_pressed(Key::Q) {
                return Some(KeyboardAction::Quit);
            }
            
            // Ctrl+F - Find
            if i.modifiers.ctrl && i.key_pressed(Key::F) {
                return Some(KeyboardAction::Find);
            }
            
            // Ctrl++ - Increase font
            if i.modifiers.ctrl && i.key_pressed(Key::Plus) {
                return Some(KeyboardAction::IncreaseFontSize);
            }
            
            // Ctrl+- - Decrease font
            if i.modifiers.ctrl && i.key_pressed(Key::Minus) {
                return Some(KeyboardAction::DecreaseFontSize);
            }
            
            // Ctrl+0 - Reset font
            if i.modifiers.ctrl && i.key_pressed(Key::Num0) {
                return Some(KeyboardAction::ResetFontSize);
            }
            
            // Alt+1-9 - Switch to tab N
            if i.modifiers.alt {
                for n in 1..=9 {
                    if i.key_pressed(Key::from_name(&n.to_string()).unwrap_or(Key::Num0)) {
                        return Some(KeyboardAction::SwitchToTab(n - 1));
                    }
                }
            }
            
            None
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyboardAction {
    NewTab,
    CloseTab,
    NextTab,
    PreviousTab,
    SwitchToTab(usize),
    NewConnection,
    OpenSettings,
    Quit,
    Find,
    IncreaseFontSize,
    DecreaseFontSize,
    ResetFontSize,
}
