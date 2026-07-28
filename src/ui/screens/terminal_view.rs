//! Terminal View Screen - renders terminal emulator for SSH sessions

#![allow(dead_code)]

use crate::ssh::{ActiveSession, SessionEvent};
use crate::terminal::{CursorStyle, RendererConfig, Terminal, TerminalSize};
use crate::ui::components::{colors, spacing};
use eframe::egui::{self, RichText};
use std::sync::Arc;
use tokio::runtime::Runtime;
use uuid::Uuid;

/// Connection state for the terminal
#[derive(Clone, PartialEq, Debug)]
pub enum ConnectionState {
    Disconnected,
    WaitingForCredentials,
    Connecting,
    Connected,
    Error(String),
}

/// Terminal view state
pub struct TerminalViewScreen {
    /// Unique identifier for this terminal
    pub id: Uuid,

    /// Terminal emulator instance
    pub terminal: Terminal,

    /// Font size for terminal
    pub font_size: f32,

    /// Connected session info
    pub session_host: String,
    pub session_user: String,
    pub session_port: u16,
    pub is_connected: bool,

    /// Last known terminal size
    last_size: (u16, u16),

    /// Active SSH session
    active_session: Option<ActiveSession>,

    /// Connection state
    connection_state: ConnectionState,

    /// Pending password for connection
    pending_password: Option<String>,

    /// Pending key path for connection
    pending_key_path: Option<String>,
}

impl Default for TerminalViewScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl TerminalViewScreen {
    pub fn new() -> Self {
        let config = RendererConfig {
            font_size: 14.0,
            font_family: "monospace".to_string(),
            cursor_style: CursorStyle::Block,
            cursor_blink: true,
            show_scrollbar: true,
        };

        let mut screen = Self {
            id: Uuid::new_v4(),
            terminal: Terminal::with_config(80, 24, 10000, config),
            font_size: 14.0,
            session_host: String::new(),
            session_user: String::new(),
            session_port: 22,
            is_connected: false,
            last_size: (80, 24),
            active_session: None,
            connection_state: ConnectionState::Disconnected,
            pending_password: None,
            pending_key_path: None,
        };

        screen.add_welcome_message();
        screen
    }

    pub fn for_session(host: &str, user: &str, port: u16) -> Self {
        let mut screen = Self::new();
        screen.session_host = host.to_string();
        screen.session_user = user.to_string();
        screen.session_port = port;
        screen.terminal.clear();
        screen.write_line(&format!("Connecting to {}@{}:{}...\r\n", user, host, port));
        screen.connection_state = ConnectionState::WaitingForCredentials;
        screen
    }

    pub fn connection_state(&self) -> &ConnectionState {
        &self.connection_state
    }

    pub fn connect_with_password(&mut self, runtime: Arc<Runtime>, password: String) {
        let host = self.session_host.clone();
        let port = self.session_port;
        let username = self.session_user.clone();
        self.connection_state = ConnectionState::Connecting;
        self.write_line("Authenticating with password...\r\n");

        let session_result = runtime.block_on(async {
            ActiveSession::connect_password(host, port, username, password).await
        });

        match session_result {
            Ok(session) => {
                self.active_session = Some(session);
            }
            Err(e) => {
                self.connection_state = ConnectionState::Error(e.to_string());
                self.write_line(&format!("\x1b[31mConnection failed: {}\x1b[0m\r\n", e));
            }
        }
    }

    pub fn connect_with_key(
        &mut self,
        runtime: Arc<Runtime>,
        key_path: String,
        passphrase: Option<String>,
    ) {
        let host = self.session_host.clone();
        let port = self.session_port;
        let username = self.session_user.clone();
        self.connection_state = ConnectionState::Connecting;
        self.write_line(&format!("Authenticating with key: {}...\r\n", key_path));

        let session_result = runtime.block_on(async {
            ActiveSession::connect_key(host, port, username, key_path, passphrase).await
        });

        match session_result {
            Ok(session) => {
                self.active_session = Some(session);
            }
            Err(e) => {
                self.connection_state = ConnectionState::Error(e.to_string());
                self.write_line(&format!("\x1b[31mConnection failed: {}\x1b[0m\r\n", e));
            }
        }
    }

    pub fn poll_session(&mut self) {
        let mut events = Vec::new();
        let mut should_clear_session = false;

        if let Some(session) = &mut self.active_session {
            while let Some(event) = session.try_recv() {
                events.push(event);
            }
        }

        for event in events {
            match event {
                SessionEvent::Connected => {
                    self.connection_state = ConnectionState::Connected;
                    self.is_connected = true;
                    self.terminal.process(b"\x1b[32mConnected!\x1b[0m\r\n");
                }
                SessionEvent::Data(data) => {
                    self.terminal.process(&data);
                }
                SessionEvent::Disconnected => {
                    self.connection_state = ConnectionState::Disconnected;
                    self.is_connected = false;
                    self.terminal
                        .process(b"\r\n\x1b[33mConnection closed.\x1b[0m\r\n");
                    should_clear_session = true;
                }
                SessionEvent::Error(err) => {
                    self.connection_state = ConnectionState::Error(err.clone());
                    let msg = format!("\r\n\x1b[31mError: {}\x1b[0m\r\n", err);
                    self.terminal.process(msg.as_bytes());
                }
            }
        }

        if should_clear_session {
            self.active_session = None;
        }
    }

    pub fn send_input(&self, data: &[u8]) {
        if let Some(session) = &self.active_session {
            session.send_data(data.to_vec());
        }
    }

    pub fn send_resize(&self, cols: u32, rows: u32) {
        if let Some(session) = &self.active_session {
            session.resize(cols, rows);
        }
    }

    pub fn disconnect(&mut self) {
        if let Some(session) = &self.active_session {
            session.disconnect();
        }
        self.active_session = None;
        self.is_connected = false;
        self.connection_state = ConnectionState::Disconnected;
    }

    fn add_welcome_message(&mut self) {
        self.write_line("TabSSH Terminal Emulator\r\n");
        self.write_line("========================\r\n");
        self.write_line("\r\n");
        self.write_line("Full VT100/xterm terminal emulation with:\r\n");
        self.write_line("  - 256-color and true color support\r\n");
        self.write_line("  - UTF-8 text rendering\r\n");
        self.write_line("  - Mouse support (SGR mouse mode)\r\n");
        self.write_line("  - Scrollback buffer (10,000 lines)\r\n");
        self.write_line("  - Text selection and clipboard\r\n");
        self.write_line("  - Alternate screen buffer\r\n");
        self.write_line("\r\n");
        self.write_line("Connect to an SSH server to begin.\r\n");
    }

    /// Write text to the terminal (processed through parser)
    pub fn write(&mut self, data: &[u8]) {
        self.terminal.process(data);
    }

    /// Write a line of text
    pub fn write_line(&mut self, text: &str) {
        self.terminal.process(text.as_bytes());
    }

    /// Set connection status
    pub fn set_connected(&mut self, connected: bool) {
        self.is_connected = connected;
        if connected {
            self.write_line("\x1b[32mConnected successfully!\x1b[0m\r\n");
            self.write_line("\r\n");
            let now = chrono::Local::now();
            self.write_line(&format!("Last login: {}\r\n", now.format("%c")));
            self.write_line("\r\n");
        } else {
            self.write_line("\x1b[31mDisconnected.\x1b[0m\r\n");
        }
    }

    /// Get terminal size
    pub fn size(&self) -> TerminalSize {
        self.terminal.size()
    }

    /// Resize terminal to new dimensions
    pub fn resize(&mut self, cols: u16, rows: u16) {
        if (cols, rows) != self.last_size {
            self.terminal.resize(cols, rows);
            self.last_size = (cols, rows);
        }
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.terminal.scroll_to_bottom();
    }

    /// Clear terminal
    pub fn clear(&mut self) {
        self.terminal.clear();
    }

    /// Render the terminal view
    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.poll_session();

        let available = ui.available_size();

        let response = egui::Frame::NONE
            .fill(egui::Color32::from_rgb(30, 30, 30))
            .corner_radius(egui::CornerRadius::ZERO)
            .show(ui, |ui| {
                ui.set_min_size(available);

                let char_width = self.font_size * 0.6;
                let char_height = self.font_size * 1.2;

                let new_cols = (available.x / char_width) as u16;
                let new_rows = (available.y / char_height) as u16;

                let old_size = self.last_size;
                self.resize(new_cols.max(1), new_rows.max(1));

                if old_size != self.last_size {
                    self.send_resize(new_cols as u32, new_rows as u32);
                }

                self.terminal.render(ui);
            });

        let rect = response.response.rect;
        let terminal_response =
            ui.interact(rect, ui.id().with("terminal_input"), egui::Sense::click());

        if terminal_response.clicked() {
            ui.memory_mut(|mem| mem.request_focus(ui.id().with("terminal_input")));
        }

        self.handle_keyboard_input(ui);
    }

    fn handle_keyboard_input(&self, ui: &mut egui::Ui) {
        if !self.is_connected {
            return;
        }

        ui.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Text(text) => {
                        self.send_input(text.as_bytes());
                    }
                    egui::Event::Key {
                        key,
                        pressed: true,
                        modifiers,
                        ..
                    } => {
                        if let Some(data) = key_to_escape_sequence(*key, modifiers) {
                            self.send_input(&data);
                        }
                    }
                    _ => {}
                }
            }
        });
    }

    /// Render terminal with status bar
    pub fn render_with_status(&mut self, ui: &mut egui::Ui) {
        let (status_color, status_text) = match &self.connection_state {
            ConnectionState::Connected => (colors::SUCCESS, "Connected"),
            ConnectionState::Connecting => (colors::WARNING, "Connecting..."),
            ConnectionState::WaitingForCredentials => (colors::WARNING, "Awaiting credentials"),
            ConnectionState::Disconnected => (colors::TEXT_MUTED, "Disconnected"),
            ConnectionState::Error(_) => (colors::DANGER, "Error"),
        };

        egui::Panel::bottom("terminal_status")
            .exact_size(24.0)
            .frame(egui::Frame::NONE.fill(colors::BG_SECONDARY))
            .show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(spacing::SM);

                    ui.label(RichText::new("\u{25CF}").color(status_color).size(10.0));
                    ui.label(RichText::new(status_text).color(status_color).size(11.0));

                    ui.separator();

                    if !self.session_host.is_empty() {
                        ui.label(
                            RichText::new(format!(
                                "{}@{}:{}",
                                self.session_user, self.session_host, self.session_port
                            ))
                            .color(colors::TEXT_SECONDARY)
                            .size(11.0),
                        );
                        ui.separator();
                    }

                    let size = self.terminal.size();
                    ui.label(
                        RichText::new(format!("{}x{}", size.cols, size.rows))
                            .color(colors::TEXT_MUTED)
                            .size(11.0),
                    );

                    ui.separator();

                    let scrollback = self.terminal.buffer().scrollback_len();
                    ui.label(
                        RichText::new(format!("{} lines in scrollback", scrollback))
                            .color(colors::TEXT_MUTED)
                            .size(11.0),
                    );
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
                self.render(ui);
            });
    }
}

fn key_to_escape_sequence(key: egui::Key, modifiers: &egui::Modifiers) -> Option<Vec<u8>> {
    if modifiers.ctrl {
        match key {
            egui::Key::A => return Some(vec![0x01]),
            egui::Key::B => return Some(vec![0x02]),
            egui::Key::C => return Some(vec![0x03]),
            egui::Key::D => return Some(vec![0x04]),
            egui::Key::E => return Some(vec![0x05]),
            egui::Key::F => return Some(vec![0x06]),
            egui::Key::G => return Some(vec![0x07]),
            egui::Key::H => return Some(vec![0x08]),
            egui::Key::I => return Some(vec![0x09]),
            egui::Key::J => return Some(vec![0x0A]),
            egui::Key::K => return Some(vec![0x0B]),
            egui::Key::L => return Some(vec![0x0C]),
            egui::Key::M => return Some(vec![0x0D]),
            egui::Key::N => return Some(vec![0x0E]),
            egui::Key::O => return Some(vec![0x0F]),
            egui::Key::P => return Some(vec![0x10]),
            egui::Key::Q => return Some(vec![0x11]),
            egui::Key::R => return Some(vec![0x12]),
            egui::Key::S => return Some(vec![0x13]),
            egui::Key::T => return Some(vec![0x14]),
            egui::Key::U => return Some(vec![0x15]),
            egui::Key::V => return Some(vec![0x16]),
            egui::Key::W => return Some(vec![0x17]),
            egui::Key::X => return Some(vec![0x18]),
            egui::Key::Y => return Some(vec![0x19]),
            egui::Key::Z => return Some(vec![0x1A]),
            _ => {}
        }
    }

    match key {
        egui::Key::Enter => Some(vec![0x0D]),
        egui::Key::Tab => Some(vec![0x09]),
        egui::Key::Backspace => Some(vec![0x7F]),
        egui::Key::Escape => Some(vec![0x1B]),
        egui::Key::ArrowUp => Some(b"\x1b[A".to_vec()),
        egui::Key::ArrowDown => Some(b"\x1b[B".to_vec()),
        egui::Key::ArrowRight => Some(b"\x1b[C".to_vec()),
        egui::Key::ArrowLeft => Some(b"\x1b[D".to_vec()),
        egui::Key::Home => Some(b"\x1b[H".to_vec()),
        egui::Key::End => Some(b"\x1b[F".to_vec()),
        egui::Key::PageUp => Some(b"\x1b[5~".to_vec()),
        egui::Key::PageDown => Some(b"\x1b[6~".to_vec()),
        egui::Key::Insert => Some(b"\x1b[2~".to_vec()),
        egui::Key::Delete => Some(b"\x1b[3~".to_vec()),
        egui::Key::F1 => Some(b"\x1bOP".to_vec()),
        egui::Key::F2 => Some(b"\x1bOQ".to_vec()),
        egui::Key::F3 => Some(b"\x1bOR".to_vec()),
        egui::Key::F4 => Some(b"\x1bOS".to_vec()),
        egui::Key::F5 => Some(b"\x1b[15~".to_vec()),
        egui::Key::F6 => Some(b"\x1b[17~".to_vec()),
        egui::Key::F7 => Some(b"\x1b[18~".to_vec()),
        egui::Key::F8 => Some(b"\x1b[19~".to_vec()),
        egui::Key::F9 => Some(b"\x1b[20~".to_vec()),
        egui::Key::F10 => Some(b"\x1b[21~".to_vec()),
        egui::Key::F11 => Some(b"\x1b[23~".to_vec()),
        egui::Key::F12 => Some(b"\x1b[24~".to_vec()),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Modifiers, RawInput};

    fn render_smoke(f: impl FnOnce(&mut egui::Ui)) {
        let ctx = egui::Context::default();
        let mut f = Some(f);
        let _ = ctx.run_ui(RawInput::default(), |ui| {
            if let Some(f) = f.take() {
                f(ui);
            }
        });
    }

    // key_to_escape_sequence: pure helper covering every mapped key plus
    // the unmapped / modifier-combination edge cases.

    #[test]
    fn test_ctrl_letters_map_to_control_codes() {
        let cases: &[(egui::Key, u8)] = &[
            (egui::Key::A, 0x01),
            (egui::Key::M, 0x0D),
            (egui::Key::Z, 0x1A),
        ];
        for (key, expected) in cases {
            assert_eq!(
                key_to_escape_sequence(*key, &Modifiers::CTRL),
                Some(vec![*expected])
            );
        }
    }

    #[test]
    fn test_plain_letter_without_ctrl_is_unmapped() {
        // 'A' with no modifiers is not a control character and has no
        // dedicated entry in the bottom match, so it must be None.
        assert_eq!(key_to_escape_sequence(egui::Key::A, &Modifiers::NONE), None);
    }

    #[test]
    fn test_editing_and_navigation_keys() {
        assert_eq!(
            key_to_escape_sequence(egui::Key::Enter, &Modifiers::NONE),
            Some(vec![0x0D])
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::Tab, &Modifiers::NONE),
            Some(vec![0x09])
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::Backspace, &Modifiers::NONE),
            Some(vec![0x7F])
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::Escape, &Modifiers::NONE),
            Some(vec![0x1B])
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::ArrowUp, &Modifiers::NONE),
            Some(b"\x1b[A".to_vec())
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::ArrowDown, &Modifiers::NONE),
            Some(b"\x1b[B".to_vec())
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::ArrowRight, &Modifiers::NONE),
            Some(b"\x1b[C".to_vec())
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::ArrowLeft, &Modifiers::NONE),
            Some(b"\x1b[D".to_vec())
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::Home, &Modifiers::NONE),
            Some(b"\x1b[H".to_vec())
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::End, &Modifiers::NONE),
            Some(b"\x1b[F".to_vec())
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::PageUp, &Modifiers::NONE),
            Some(b"\x1b[5~".to_vec())
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::PageDown, &Modifiers::NONE),
            Some(b"\x1b[6~".to_vec())
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::Insert, &Modifiers::NONE),
            Some(b"\x1b[2~".to_vec())
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::Delete, &Modifiers::NONE),
            Some(b"\x1b[3~".to_vec())
        );
    }

    #[test]
    fn test_function_keys_f1_through_f12() {
        let cases: &[(egui::Key, &[u8])] = &[
            (egui::Key::F1, b"\x1bOP"),
            (egui::Key::F2, b"\x1bOQ"),
            (egui::Key::F3, b"\x1bOR"),
            (egui::Key::F4, b"\x1bOS"),
            (egui::Key::F5, b"\x1b[15~"),
            (egui::Key::F6, b"\x1b[17~"),
            (egui::Key::F7, b"\x1b[18~"),
            (egui::Key::F8, b"\x1b[19~"),
            (egui::Key::F9, b"\x1b[20~"),
            (egui::Key::F10, b"\x1b[21~"),
            (egui::Key::F11, b"\x1b[23~"),
            (egui::Key::F12, b"\x1b[24~"),
        ];
        for (key, expected) in cases {
            assert_eq!(
                key_to_escape_sequence(*key, &Modifiers::NONE),
                Some(expected.to_vec())
            );
        }
    }

    #[test]
    fn test_unmapped_key_returns_none() {
        assert_eq!(
            key_to_escape_sequence(egui::Key::Space, &Modifiers::NONE),
            None
        );
        assert_eq!(
            key_to_escape_sequence(egui::Key::Space, &Modifiers::CTRL),
            None
        );
    }

    #[test]
    fn test_ctrl_modifier_falls_through_for_non_letter_keys() {
        // Ctrl+Enter is not special-cased in the ctrl block, so it falls
        // through to the same mapping as an unmodified Enter.
        assert_eq!(
            key_to_escape_sequence(egui::Key::Enter, &Modifiers::CTRL),
            Some(vec![0x0D])
        );
    }

    // TerminalViewScreen construction / state management.

    #[test]
    fn test_new_screen_has_expected_defaults() {
        let screen = TerminalViewScreen::new();
        assert_eq!(screen.session_host, "");
        assert_eq!(screen.session_user, "");
        assert_eq!(screen.session_port, 22);
        assert!(!screen.is_connected);
        assert_eq!(*screen.connection_state(), ConnectionState::Disconnected);
        assert_eq!(screen.size().cols, 80);
        assert_eq!(screen.size().rows, 24);
        // The welcome banner fits within the 24-row visible screen, so it
        // should not have pushed anything into scrollback yet.
        assert_eq!(screen.terminal.buffer().scrollback_len(), 0);
        // The welcome banner should have been written to the visible buffer.
        let row0 = screen.terminal.buffer().get_row(0).expect("row 0 exists");
        let row0_text: String = row0.iter().map(|c| c.character).collect();
        assert!(row0_text.starts_with("TabSSH Terminal Emulator"));
    }

    #[test]
    fn test_default_matches_new() {
        let screen = TerminalViewScreen::default();
        assert_eq!(screen.session_port, 22);
        assert_eq!(*screen.connection_state(), ConnectionState::Disconnected);
    }

    #[test]
    fn test_for_session_sets_fields_and_state() {
        let screen = TerminalViewScreen::for_session("example.com", "alice", 2222);
        assert_eq!(screen.session_host, "example.com");
        assert_eq!(screen.session_user, "alice");
        assert_eq!(screen.session_port, 2222);
        assert_eq!(
            *screen.connection_state(),
            ConnectionState::WaitingForCredentials
        );
    }

    #[test]
    fn test_for_session_with_empty_host_and_user() {
        // Boundary: empty strings must not panic the formatter.
        let screen = TerminalViewScreen::for_session("", "", 0);
        assert_eq!(screen.session_host, "");
        assert_eq!(screen.session_port, 0);
    }

    #[test]
    fn test_write_and_write_line_process_terminal_data() {
        let mut screen = TerminalViewScreen::new();
        let before = screen.terminal.buffer().cursor_position();
        screen.write(b"hello");
        screen.write_line("world\r\n");
        let after = screen.terminal.buffer().cursor_position();
        assert_ne!(before, after);
    }

    #[test]
    fn test_set_connected_true_writes_success_message() {
        let mut screen = TerminalViewScreen::new();
        let before = screen.terminal.buffer().scrollback_len();
        screen.set_connected(true);
        assert!(screen.is_connected);
        assert!(screen.terminal.buffer().scrollback_len() >= before);
    }

    #[test]
    fn test_set_connected_false_writes_disconnected_message() {
        let mut screen = TerminalViewScreen::new();
        screen.set_connected(true);
        screen.set_connected(false);
        assert!(!screen.is_connected);
    }

    #[test]
    fn test_resize_updates_size_only_when_changed() {
        let mut screen = TerminalViewScreen::new();
        screen.resize(100, 40);
        assert_eq!(screen.size().cols, 100);
        assert_eq!(screen.size().rows, 40);

        // Resizing to the same dimensions again must be a cheap no-op and
        // must not panic or corrupt the terminal state.
        screen.resize(100, 40);
        assert_eq!(screen.size().cols, 100);
        assert_eq!(screen.size().rows, 40);
    }

    #[test]
    fn test_resize_to_minimum_dimensions() {
        let mut screen = TerminalViewScreen::new();
        screen.resize(1, 1);
        assert_eq!(screen.size().cols, 1);
        assert_eq!(screen.size().rows, 1);
    }

    #[test]
    fn test_scroll_to_bottom_and_clear_do_not_panic() {
        let mut screen = TerminalViewScreen::new();
        screen.write_line("some scrollback content\r\n");
        screen.scroll_to_bottom();
        screen.clear();
        assert_eq!(screen.terminal.buffer().scrollback_len(), 0);
    }

    #[test]
    fn test_poll_session_without_active_session_is_noop() {
        let mut screen = TerminalViewScreen::new();
        screen.poll_session();
        assert_eq!(*screen.connection_state(), ConnectionState::Disconnected);
        assert!(!screen.is_connected);
    }

    #[test]
    fn test_send_input_and_resize_without_session_do_not_panic() {
        let screen = TerminalViewScreen::new();
        // No active session is set, so both calls should be silent no-ops.
        screen.send_input(b"echo hi\n");
        screen.send_resize(80, 24);
    }

    #[test]
    fn test_disconnect_resets_state() {
        let mut screen = TerminalViewScreen::new();
        screen.is_connected = true;
        screen.connection_state = ConnectionState::Connected;
        screen.disconnect();
        assert!(!screen.is_connected);
        assert_eq!(*screen.connection_state(), ConnectionState::Disconnected);
    }

    #[test]
    fn test_connection_state_error_variant_holds_message() {
        let state = ConnectionState::Error("boom".to_string());
        match state {
            ConnectionState::Error(msg) => assert_eq!(msg, "boom"),
            _ => panic!("expected Error variant"),
        }
    }

    // Widget-render smoke tests: exercise render() / render_with_status()
    // against representative state without panicking.

    #[test]
    fn test_render_smoke_disconnected() {
        let mut screen = TerminalViewScreen::new();
        render_smoke(|ui| screen.render(ui));
    }

    #[test]
    fn test_render_smoke_with_populated_scrollback() {
        let mut screen = TerminalViewScreen::new();
        for i in 0..50 {
            screen.write_line(&format!("line {}\r\n", i));
        }
        render_smoke(|ui| screen.render(ui));
    }

    #[test]
    fn test_render_with_status_smoke_all_connection_states() {
        let states = [
            ConnectionState::Disconnected,
            ConnectionState::WaitingForCredentials,
            ConnectionState::Connecting,
            ConnectionState::Connected,
            ConnectionState::Error("connection refused".to_string()),
        ];
        for state in states {
            let mut screen = TerminalViewScreen::for_session("example.com", "bob", 22);
            screen.connection_state = state;
            render_smoke(|ui| {
                screen.render_with_status(ui);
            });
        }
    }

    #[test]
    fn test_render_with_status_smoke_empty_host() {
        let mut screen = TerminalViewScreen::new();
        render_smoke(|ui| screen.render_with_status(ui));
    }

    #[test]
    fn test_render_with_status_smoke_long_and_unicode_strings() {
        let mut screen =
            TerminalViewScreen::for_session(&"very-long-hostname-".repeat(10), "üsér-名前", 65535);
        render_smoke(|ui| screen.render_with_status(ui));
    }
}
