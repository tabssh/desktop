//! Keyboard shortcut handling

use egui::{Context, Key};

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

            // Ctrl+Shift+Tab - Previous tab (must be checked before the
            // plain Ctrl+Tab case below, since Shift+Ctrl+Tab also
            // satisfies that condition)
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(Key::Tab) {
                return Some(KeyboardAction::PreviousTab);
            }

            // Ctrl+Tab - Next tab
            if i.modifiers.ctrl && i.key_pressed(Key::Tab) {
                return Some(KeyboardAction::NextTab);
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

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Modifiers, RawInput};

    /// Build a headless egui context and feed it a single key event with the
    /// given modifiers, then run the shortcut handler against that frame.
    fn press(key: Key, modifiers: Modifiers) -> Option<KeyboardAction> {
        let ctx = Context::default();
        let raw_input = RawInput {
            modifiers,
            events: vec![egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers,
            }],
            ..Default::default()
        };
        ctx.begin_pass(raw_input);
        let action = KeyboardHandler::handle_shortcuts(&ctx);
        let _ = ctx.end_pass();
        action
    }

    #[test]
    fn test_no_input_produces_no_action() {
        let ctx = Context::default();
        ctx.begin_pass(RawInput::default());
        let action = KeyboardHandler::handle_shortcuts(&ctx);
        let _ = ctx.end_pass();
        assert_eq!(action, None);
    }

    #[test]
    fn test_ctrl_t_new_tab() {
        assert_eq!(press(Key::T, Modifiers::CTRL), Some(KeyboardAction::NewTab));
    }

    #[test]
    fn test_ctrl_w_close_tab() {
        assert_eq!(
            press(Key::W, Modifiers::CTRL),
            Some(KeyboardAction::CloseTab)
        );
    }

    #[test]
    fn test_ctrl_tab_next_tab() {
        assert_eq!(
            press(Key::Tab, Modifiers::CTRL),
            Some(KeyboardAction::NextTab)
        );
    }

    #[test]
    fn test_ctrl_shift_tab_previous_tab() {
        let modifiers = Modifiers::CTRL | Modifiers::SHIFT;
        assert_eq!(
            press(Key::Tab, modifiers),
            Some(KeyboardAction::PreviousTab)
        );
    }

    #[test]
    fn test_ctrl_n_new_connection() {
        assert_eq!(
            press(Key::N, Modifiers::CTRL),
            Some(KeyboardAction::NewConnection)
        );
    }

    #[test]
    fn test_ctrl_comma_open_settings() {
        assert_eq!(
            press(Key::Comma, Modifiers::CTRL),
            Some(KeyboardAction::OpenSettings)
        );
    }

    #[test]
    fn test_ctrl_q_quit() {
        assert_eq!(press(Key::Q, Modifiers::CTRL), Some(KeyboardAction::Quit));
    }

    #[test]
    fn test_ctrl_f_find() {
        assert_eq!(press(Key::F, Modifiers::CTRL), Some(KeyboardAction::Find));
    }

    #[test]
    fn test_ctrl_plus_increase_font() {
        assert_eq!(
            press(Key::Plus, Modifiers::CTRL),
            Some(KeyboardAction::IncreaseFontSize)
        );
    }

    #[test]
    fn test_ctrl_minus_decrease_font() {
        assert_eq!(
            press(Key::Minus, Modifiers::CTRL),
            Some(KeyboardAction::DecreaseFontSize)
        );
    }

    #[test]
    fn test_ctrl_0_reset_font() {
        assert_eq!(
            press(Key::Num0, Modifiers::CTRL),
            Some(KeyboardAction::ResetFontSize)
        );
    }

    #[test]
    fn test_alt_1_switches_to_first_tab() {
        assert_eq!(
            press(Key::Num1, Modifiers::ALT),
            Some(KeyboardAction::SwitchToTab(0))
        );
    }

    #[test]
    fn test_alt_9_switches_to_ninth_tab() {
        assert_eq!(
            press(Key::Num9, Modifiers::ALT),
            Some(KeyboardAction::SwitchToTab(8))
        );
    }

    #[test]
    fn test_key_without_ctrl_produces_no_action() {
        // Plain T (no modifiers) must not trigger the Ctrl+T shortcut.
        assert_eq!(press(Key::T, Modifiers::NONE), None);
    }

    #[test]
    fn test_keyboard_action_equality_and_debug() {
        assert_eq!(KeyboardAction::NewTab, KeyboardAction::NewTab);
        assert_ne!(KeyboardAction::NewTab, KeyboardAction::CloseTab);
        assert_eq!(
            KeyboardAction::SwitchToTab(3),
            KeyboardAction::SwitchToTab(3)
        );
        assert_ne!(
            KeyboardAction::SwitchToTab(3),
            KeyboardAction::SwitchToTab(4)
        );
        // Debug is derived; just make sure it doesn't panic and is non-empty.
        assert!(!format!("{:?}", KeyboardAction::Quit).is_empty());
    }
}
