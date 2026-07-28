//! Connection Editor Screen - form for creating/editing SSH connections

#![allow(dead_code)]

use crate::ui::components::{
    card, colors, danger_button, form_row, labeled_dropdown, labeled_input, labeled_number,
    labeled_toggle, primary_button, secondary_button, section_header, spacing,
};
use eframe::egui::{self, RichText};

/// Authentication type used in a connection profile
#[derive(Clone, PartialEq)]
pub enum ProfileAuthType {
    Password,
    PublicKey,
    KeyboardInteractive,
}

/// Saved connection profile
#[derive(Clone)]
pub struct ConnectionProfile {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth_type: ProfileAuthType,
    pub group: Option<String>,
    pub last_connected: Option<String>,
    pub is_favorite: bool,
}

/// Authentication method for the editor form
#[derive(Clone, PartialEq)]
pub enum FormAuthMethod {
    Password,
    PublicKey,
    KeyboardInteractive,
    Agent,
}

impl std::fmt::Display for FormAuthMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormAuthMethod::Password => write!(f, "Password"),
            FormAuthMethod::PublicKey => write!(f, "Public Key"),
            FormAuthMethod::KeyboardInteractive => write!(f, "Keyboard Interactive"),
            FormAuthMethod::Agent => write!(f, "SSH Agent"),
        }
    }
}

/// Connection editor screen state
pub struct ConnectionEditorScreen {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,

    pub auth_method: FormAuthMethod,
    pub password: String,
    pub private_key_path: String,
    pub passphrase: String,
    pub save_password: bool,

    pub compression: bool,
    pub keepalive_interval: u16,
    pub connection_timeout: u16,
    pub tcp_keepalive: bool,

    pub terminal_type: String,
    pub initial_command: String,
    pub encoding: String,

    pub enable_x11_forwarding: bool,
    pub enable_agent_forwarding: bool,
    pub local_forwards: Vec<PortForward>,
    pub remote_forwards: Vec<PortForward>,

    pub use_jump_host: bool,
    pub jump_host: String,
    pub jump_port: u16,
    pub jump_username: String,

    pub group: String,
    pub is_favorite: bool,
    pub notes: String,

    pub editing_id: Option<String>,
    pub is_dirty: bool,
}

#[derive(Clone)]
pub struct PortForward {
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
    pub enabled: bool,
}

impl Default for ConnectionEditorScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionEditorScreen {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            host: String::new(),
            port: 22,
            username: String::from("root"),

            auth_method: FormAuthMethod::Password,
            password: String::new(),
            private_key_path: String::new(),
            passphrase: String::new(),
            save_password: false,

            compression: false,
            keepalive_interval: 30,
            connection_timeout: 30,
            tcp_keepalive: true,

            terminal_type: String::from("xterm-256color"),
            initial_command: String::new(),
            encoding: String::from("UTF-8"),

            enable_x11_forwarding: false,
            enable_agent_forwarding: false,
            local_forwards: Vec::new(),
            remote_forwards: Vec::new(),

            use_jump_host: false,
            jump_host: String::new(),
            jump_port: 22,
            jump_username: String::new(),

            group: String::new(),
            is_favorite: false,
            notes: String::new(),

            editing_id: None,
            is_dirty: false,
        }
    }

    pub fn from_profile(profile: &ConnectionProfile) -> Self {
        let mut editor = Self::new();
        editor.name = profile.name.clone();
        editor.host = profile.host.clone();
        editor.port = profile.port;
        editor.username = profile.username.clone();
        editor.auth_method = match profile.auth_type {
            ProfileAuthType::Password => FormAuthMethod::Password,
            ProfileAuthType::PublicKey => FormAuthMethod::PublicKey,
            ProfileAuthType::KeyboardInteractive => FormAuthMethod::KeyboardInteractive,
        };
        editor.group = profile.group.clone().unwrap_or_default();
        editor.is_favorite = profile.is_favorite;
        editor.editing_id = Some(profile.id.clone());
        editor
    }

    /// Render the connection editor form
    pub fn render(&mut self, ui: &mut egui::Ui) -> Option<ConnectionEditorAction> {
        let mut action = None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            let is_new = self.editing_id.is_none();
            let title = if is_new {
                "New Connection"
            } else {
                "Edit Connection"
            };

            ui.horizontal(|ui| {
                ui.heading(RichText::new(title).color(colors::TEXT_PRIMARY).size(20.0));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if danger_button(ui, "Cancel").clicked() {
                        action = Some(ConnectionEditorAction::Cancel);
                    }

                    ui.add_space(spacing::SM);

                    if primary_button(ui, if is_new { "Create" } else { "Save" }).clicked() {
                        action = Some(ConnectionEditorAction::Save(self.to_profile()));
                    }
                });
            });

            ui.add_space(spacing::LG);

            section_header(ui, "Basic Settings");

            card(ui, |ui| {
                form_row(ui, |ui| {
                    labeled_input(ui, "Connection Name", &mut self.name, "My Server");
                });
                form_row(ui, |ui| {
                    labeled_input(ui, "Host", &mut self.host, "example.com or 192.168.1.1");
                });
                form_row(ui, |ui| {
                    labeled_number(ui, "Port", &mut self.port, 1, 65535);
                });
                form_row(ui, |ui| {
                    labeled_input(ui, "Username", &mut self.username, "root");
                });
            });

            section_header(ui, "Authentication");

            card(ui, |ui| {
                form_row(ui, |ui| {
                    let auth_methods = [
                        FormAuthMethod::Password,
                        FormAuthMethod::PublicKey,
                        FormAuthMethod::KeyboardInteractive,
                        FormAuthMethod::Agent,
                    ];
                    labeled_dropdown(
                        ui,
                        "Method",
                        "auth_method",
                        &mut self.auth_method,
                        &auth_methods,
                    );
                });

                match self.auth_method {
                    FormAuthMethod::Password => {
                        form_row(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Password").color(colors::TEXT_PRIMARY));
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        let input = egui::TextEdit::singleline(&mut self.password)
                                            .hint_text(
                                                RichText::new("Enter password")
                                                    .color(colors::TEXT_MUTED),
                                            )
                                            .text_color(colors::TEXT_PRIMARY)
                                            .password(true)
                                            .desired_width(200.0)
                                            .margin(egui::vec2(8.0, 6.0));
                                        ui.add(input);
                                    },
                                );
                            });
                        });
                        form_row(ui, |ui| {
                            labeled_toggle(
                                ui,
                                "Save password in keychain",
                                &mut self.save_password,
                            );
                        });
                    }
                    FormAuthMethod::PublicKey => {
                        form_row(ui, |ui| {
                            ui.horizontal(|ui| {
                                labeled_input(
                                    ui,
                                    "Private Key",
                                    &mut self.private_key_path,
                                    "~/.ssh/id_ed25519",
                                );
                                if secondary_button(ui, "Browse...").clicked() {
                                    log::info!("File picker not yet implemented");
                                }
                            });
                        });
                        form_row(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Passphrase").color(colors::TEXT_PRIMARY));
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        let input =
                                            egui::TextEdit::singleline(&mut self.passphrase)
                                                .hint_text(
                                                    RichText::new("Optional")
                                                        .color(colors::TEXT_MUTED),
                                                )
                                                .text_color(colors::TEXT_PRIMARY)
                                                .password(true)
                                                .desired_width(200.0)
                                                .margin(egui::vec2(8.0, 6.0));
                                        ui.add(input);
                                    },
                                );
                            });
                        });
                    }
                    FormAuthMethod::KeyboardInteractive => {
                        ui.label(
                            RichText::new(
                                "You will be prompted for authentication during connection.",
                            )
                            .color(colors::TEXT_SECONDARY)
                            .size(12.0),
                        );
                    }
                    FormAuthMethod::Agent => {
                        ui.label(
                            RichText::new("SSH Agent will be used for authentication.")
                                .color(colors::TEXT_SECONDARY)
                                .size(12.0),
                        );
                    }
                }
            });

            section_header(ui, "Terminal");

            card(ui, |ui| {
                form_row(ui, |ui| {
                    let term_types = ["xterm-256color", "xterm", "vt100", "linux", "screen"];
                    let term_idx = term_types
                        .iter()
                        .position(|&t| t == self.terminal_type)
                        .unwrap_or(0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Terminal Type").color(colors::TEXT_PRIMARY));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            egui::ComboBox::from_id_salt("term_type")
                                .selected_text(
                                    RichText::new(&self.terminal_type).color(colors::TEXT_PRIMARY),
                                )
                                .width(200.0)
                                .show_ui(ui, |ui: &mut egui::Ui| {
                                    for (i, term) in term_types.iter().enumerate() {
                                        if ui.selectable_label(i == term_idx, *term).clicked() {
                                            self.terminal_type = term.to_string();
                                        }
                                    }
                                });
                        });
                    });
                });
                form_row(ui, |ui| {
                    labeled_input(
                        ui,
                        "Initial Command",
                        &mut self.initial_command,
                        "Optional command to run on connect",
                    );
                });
                form_row(ui, |ui| {
                    let encodings = ["UTF-8", "ISO-8859-1", "GBK", "Big5"];
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Character Encoding").color(colors::TEXT_PRIMARY));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            egui::ComboBox::from_id_salt("encoding")
                                .selected_text(
                                    RichText::new(&self.encoding).color(colors::TEXT_PRIMARY),
                                )
                                .width(200.0)
                                .show_ui(ui, |ui: &mut egui::Ui| {
                                    for enc in encodings {
                                        if ui.selectable_label(self.encoding == enc, enc).clicked()
                                        {
                                            self.encoding = enc.to_string();
                                        }
                                    }
                                });
                        });
                    });
                });
            });

            section_header(ui, "Advanced SSH Options");

            card(ui, |ui| {
                form_row(ui, |ui| {
                    labeled_toggle(ui, "Enable compression", &mut self.compression);
                });
                form_row(ui, |ui| {
                    labeled_toggle(ui, "TCP keep-alive", &mut self.tcp_keepalive);
                });
                form_row(ui, |ui| {
                    labeled_number(
                        ui,
                        "Keep-alive interval (seconds)",
                        &mut self.keepalive_interval,
                        0,
                        600,
                    );
                });
                form_row(ui, |ui| {
                    labeled_number(
                        ui,
                        "Connection timeout (seconds)",
                        &mut self.connection_timeout,
                        5,
                        300,
                    );
                });
            });

            section_header(ui, "Forwarding");

            card(ui, |ui| {
                form_row(ui, |ui| {
                    labeled_toggle(ui, "Enable X11 forwarding", &mut self.enable_x11_forwarding);
                });
                form_row(ui, |ui| {
                    labeled_toggle(
                        ui,
                        "Enable agent forwarding",
                        &mut self.enable_agent_forwarding,
                    );
                });

                ui.add_space(spacing::SM);
                ui.label(
                    RichText::new("Port Forwarding")
                        .color(colors::TEXT_SECONDARY)
                        .size(13.0),
                );
                ui.add_space(spacing::XS);

                ui.horizontal(|ui| {
                    if secondary_button(ui, "+ Local Forward").clicked() {
                        self.local_forwards.push(PortForward {
                            local_port: 8080,
                            remote_host: "localhost".to_string(),
                            remote_port: 80,
                            enabled: true,
                        });
                    }
                    if secondary_button(ui, "+ Remote Forward").clicked() {
                        self.remote_forwards.push(PortForward {
                            local_port: 8080,
                            remote_host: "localhost".to_string(),
                            remote_port: 80,
                            enabled: true,
                        });
                    }
                });

                for (i, fwd) in self.local_forwards.clone().iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!(
                                "L: {}:{}:{}",
                                fwd.local_port, fwd.remote_host, fwd.remote_port
                            ))
                            .color(colors::TEXT_SECONDARY),
                        );
                        if ui.small_button("x").clicked() {
                            self.local_forwards.remove(i);
                        }
                    });
                }

                for (i, fwd) in self.remote_forwards.clone().iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!(
                                "R: {}:{}:{}",
                                fwd.local_port, fwd.remote_host, fwd.remote_port
                            ))
                            .color(colors::TEXT_SECONDARY),
                        );
                        if ui.small_button("x").clicked() {
                            self.remote_forwards.remove(i);
                        }
                    });
                }
            });

            section_header(ui, "Jump Host / Proxy");

            card(ui, |ui| {
                form_row(ui, |ui| {
                    labeled_toggle(ui, "Use jump host (ProxyJump)", &mut self.use_jump_host);
                });

                if self.use_jump_host {
                    form_row(ui, |ui| {
                        labeled_input(ui, "Jump Host", &mut self.jump_host, "bastion.example.com");
                    });
                    form_row(ui, |ui| {
                        labeled_number(ui, "Jump Port", &mut self.jump_port, 1, 65535);
                    });
                    form_row(ui, |ui| {
                        labeled_input(
                            ui,
                            "Jump Username",
                            &mut self.jump_username,
                            "Same as connection if empty",
                        );
                    });
                }
            });

            section_header(ui, "Organization");

            card(ui, |ui| {
                form_row(ui, |ui| {
                    labeled_input(
                        ui,
                        "Group",
                        &mut self.group,
                        "Production, Development, etc.",
                    );
                });
                form_row(ui, |ui| {
                    labeled_toggle(ui, "Add to favorites", &mut self.is_favorite);
                });

                ui.add_space(spacing::SM);
                ui.label(RichText::new("Notes").color(colors::TEXT_PRIMARY));
                ui.add_space(spacing::XS);

                let notes_input = egui::TextEdit::multiline(&mut self.notes)
                    .hint_text(
                        RichText::new("Optional notes about this connection")
                            .color(colors::TEXT_MUTED),
                    )
                    .text_color(colors::TEXT_PRIMARY)
                    .desired_width(ui.available_width())
                    .desired_rows(3);
                ui.add(notes_input);
            });

            ui.add_space(spacing::XXL);
        });

        action
    }

    /// Convert form state to a ConnectionProfile
    pub fn to_profile(&self) -> ConnectionProfile {
        ConnectionProfile {
            id: self
                .editing_id
                .clone()
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            name: if self.name.is_empty() {
                format!("{}@{}", self.username, self.host)
            } else {
                self.name.clone()
            },
            host: self.host.clone(),
            port: self.port,
            username: self.username.clone(),
            auth_type: match self.auth_method {
                FormAuthMethod::Password => ProfileAuthType::Password,
                FormAuthMethod::PublicKey => ProfileAuthType::PublicKey,
                FormAuthMethod::KeyboardInteractive => ProfileAuthType::KeyboardInteractive,
                // SSH Agent auth is negotiated via public key mechanism
                FormAuthMethod::Agent => ProfileAuthType::PublicKey,
            },
            group: if self.group.is_empty() {
                None
            } else {
                Some(self.group.clone())
            },
            last_connected: None,
            is_favorite: self.is_favorite,
        }
    }
}

/// Actions from the connection editor
pub enum ConnectionEditorAction {
    Save(ConnectionProfile),
    Cancel,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a bare-bones connection profile for use as test fixture input.
    fn sample_profile() -> ConnectionProfile {
        ConnectionProfile {
            id: "profile-1".to_string(),
            name: "My Box".to_string(),
            host: "host.example.com".to_string(),
            port: 2222,
            username: "alice".to_string(),
            auth_type: ProfileAuthType::PublicKey,
            group: Some("Production".to_string()),
            last_connected: Some("2024-01-01".to_string()),
            is_favorite: true,
        }
    }

    #[test]
    fn test_form_auth_method_display_password() {
        assert_eq!(FormAuthMethod::Password.to_string(), "Password");
    }

    #[test]
    fn test_form_auth_method_display_public_key() {
        assert_eq!(FormAuthMethod::PublicKey.to_string(), "Public Key");
    }

    #[test]
    fn test_form_auth_method_display_keyboard_interactive() {
        assert_eq!(
            FormAuthMethod::KeyboardInteractive.to_string(),
            "Keyboard Interactive"
        );
    }

    #[test]
    fn test_form_auth_method_display_agent() {
        assert_eq!(FormAuthMethod::Agent.to_string(), "SSH Agent");
    }

    #[test]
    fn test_new_has_sane_defaults() {
        let editor = ConnectionEditorScreen::new();
        assert_eq!(editor.name, "");
        assert_eq!(editor.host, "");
        assert_eq!(editor.port, 22);
        assert_eq!(editor.username, "root");
        assert!(editor.auth_method == FormAuthMethod::Password);
        assert!(!editor.save_password);
        assert!(!editor.compression);
        assert_eq!(editor.keepalive_interval, 30);
        assert_eq!(editor.connection_timeout, 30);
        assert!(editor.tcp_keepalive);
        assert_eq!(editor.terminal_type, "xterm-256color");
        assert_eq!(editor.encoding, "UTF-8");
        assert!(editor.local_forwards.is_empty());
        assert!(editor.remote_forwards.is_empty());
        assert!(!editor.use_jump_host);
        assert_eq!(editor.jump_port, 22);
        assert!(editor.editing_id.is_none());
        assert!(!editor.is_dirty);
    }

    #[test]
    fn test_default_trait_matches_new() {
        let editor = ConnectionEditorScreen::default();
        assert_eq!(editor.port, 22);
        assert_eq!(editor.username, "root");
    }

    #[test]
    fn test_from_profile_copies_fields() {
        let profile = sample_profile();
        let editor = ConnectionEditorScreen::from_profile(&profile);
        assert_eq!(editor.name, "My Box");
        assert_eq!(editor.host, "host.example.com");
        assert_eq!(editor.port, 2222);
        assert_eq!(editor.username, "alice");
        assert!(editor.auth_method == FormAuthMethod::PublicKey);
        assert_eq!(editor.group, "Production");
        assert!(editor.is_favorite);
        assert_eq!(editor.editing_id, Some("profile-1".to_string()));
    }

    #[test]
    fn test_from_profile_maps_password_auth_type() {
        let mut profile = sample_profile();
        profile.auth_type = ProfileAuthType::Password;
        let editor = ConnectionEditorScreen::from_profile(&profile);
        assert!(editor.auth_method == FormAuthMethod::Password);
    }

    #[test]
    fn test_from_profile_maps_keyboard_interactive_auth_type() {
        let mut profile = sample_profile();
        profile.auth_type = ProfileAuthType::KeyboardInteractive;
        let editor = ConnectionEditorScreen::from_profile(&profile);
        assert!(editor.auth_method == FormAuthMethod::KeyboardInteractive);
    }

    #[test]
    fn test_from_profile_with_no_group_defaults_to_empty_string() {
        let mut profile = sample_profile();
        profile.group = None;
        let editor = ConnectionEditorScreen::from_profile(&profile);
        assert_eq!(editor.group, "");
    }

    #[test]
    fn test_to_profile_generates_uuid_when_no_editing_id() {
        let editor = ConnectionEditorScreen::new();
        let profile = editor.to_profile();
        // Must be a valid UUID string (36 chars, hyphenated).
        assert_eq!(profile.id.len(), 36);
        assert!(uuid::Uuid::parse_str(&profile.id).is_ok());
    }

    #[test]
    fn test_to_profile_preserves_editing_id() {
        let mut editor = ConnectionEditorScreen::new();
        editor.editing_id = Some("existing-id".to_string());
        let profile = editor.to_profile();
        assert_eq!(profile.id, "existing-id");
    }

    #[test]
    fn test_to_profile_defaults_name_from_username_and_host_when_empty() {
        let mut editor = ConnectionEditorScreen::new();
        editor.name = String::new();
        editor.username = "bob".to_string();
        editor.host = "example.org".to_string();
        let profile = editor.to_profile();
        assert_eq!(profile.name, "bob@example.org");
    }

    #[test]
    fn test_to_profile_keeps_explicit_name() {
        let mut editor = ConnectionEditorScreen::new();
        editor.name = "Custom Name".to_string();
        editor.username = "bob".to_string();
        editor.host = "example.org".to_string();
        let profile = editor.to_profile();
        assert_eq!(profile.name, "Custom Name");
    }

    #[test]
    fn test_to_profile_maps_password_auth() {
        let editor = ConnectionEditorScreen::new();
        let profile = editor.to_profile();
        assert!(matches!(profile.auth_type, ProfileAuthType::Password));
    }

    #[test]
    fn test_to_profile_maps_public_key_auth() {
        let mut editor = ConnectionEditorScreen::new();
        editor.auth_method = FormAuthMethod::PublicKey;
        let profile = editor.to_profile();
        assert!(matches!(profile.auth_type, ProfileAuthType::PublicKey));
    }

    #[test]
    fn test_to_profile_maps_keyboard_interactive_auth() {
        let mut editor = ConnectionEditorScreen::new();
        editor.auth_method = FormAuthMethod::KeyboardInteractive;
        let profile = editor.to_profile();
        assert!(matches!(
            profile.auth_type,
            ProfileAuthType::KeyboardInteractive
        ));
    }

    #[test]
    fn test_to_profile_maps_agent_auth_to_public_key() {
        // SSH Agent auth is negotiated via the public key mechanism, so the
        // saved profile should record it as PublicKey.
        let mut editor = ConnectionEditorScreen::new();
        editor.auth_method = FormAuthMethod::Agent;
        let profile = editor.to_profile();
        assert!(matches!(profile.auth_type, ProfileAuthType::PublicKey));
    }

    #[test]
    fn test_to_profile_empty_group_becomes_none() {
        let mut editor = ConnectionEditorScreen::new();
        editor.group = String::new();
        let profile = editor.to_profile();
        assert_eq!(profile.group, None);
    }

    #[test]
    fn test_to_profile_nonempty_group_becomes_some() {
        let mut editor = ConnectionEditorScreen::new();
        editor.group = "Staging".to_string();
        let profile = editor.to_profile();
        assert_eq!(profile.group, Some("Staging".to_string()));
    }

    #[test]
    fn test_to_profile_last_connected_always_none() {
        let editor = ConnectionEditorScreen::new();
        let profile = editor.to_profile();
        assert!(profile.last_connected.is_none());
    }

    #[test]
    fn test_to_profile_unicode_fields_roundtrip() {
        let mut editor = ConnectionEditorScreen::new();
        editor.name = String::new();
        editor.username = "üsér".to_string();
        editor.host = "hôst.example".to_string();
        let profile = editor.to_profile();
        assert_eq!(profile.name, "üsér@hôst.example");
    }

    /// Render the editor form inside a headless egui context and make sure
    /// it does not panic for a given screen state.
    fn render_smoke(editor: &mut ConnectionEditorScreen) {
        let ctx = egui::Context::default();
        let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
            editor.render(ui);
        });
    }

    #[test]
    fn test_render_new_connection_smoke() {
        let mut editor = ConnectionEditorScreen::new();
        render_smoke(&mut editor);
        assert!(editor.editing_id.is_none());
    }

    #[test]
    fn test_render_edit_existing_connection_smoke() {
        let profile = sample_profile();
        let mut editor = ConnectionEditorScreen::from_profile(&profile);
        render_smoke(&mut editor);
        assert!(editor.editing_id.is_some());
    }

    #[test]
    fn test_render_public_key_auth_method_smoke() {
        let mut editor = ConnectionEditorScreen::new();
        editor.auth_method = FormAuthMethod::PublicKey;
        render_smoke(&mut editor);
    }

    #[test]
    fn test_render_keyboard_interactive_auth_method_smoke() {
        let mut editor = ConnectionEditorScreen::new();
        editor.auth_method = FormAuthMethod::KeyboardInteractive;
        render_smoke(&mut editor);
    }

    #[test]
    fn test_render_agent_auth_method_smoke() {
        let mut editor = ConnectionEditorScreen::new();
        editor.auth_method = FormAuthMethod::Agent;
        render_smoke(&mut editor);
    }

    #[test]
    fn test_render_with_jump_host_enabled_smoke() {
        let mut editor = ConnectionEditorScreen::new();
        editor.use_jump_host = true;
        editor.jump_host = "bastion.example.com".to_string();
        editor.jump_username = "jumpuser".to_string();
        render_smoke(&mut editor);
    }

    #[test]
    fn test_render_with_port_forwards_populated_smoke() {
        let mut editor = ConnectionEditorScreen::new();
        editor.local_forwards.push(PortForward {
            local_port: 8080,
            remote_host: "localhost".to_string(),
            remote_port: 80,
            enabled: true,
        });
        editor.remote_forwards.push(PortForward {
            local_port: 9090,
            remote_host: "internal.example.com".to_string(),
            remote_port: 443,
            enabled: false,
        });
        render_smoke(&mut editor);
        assert_eq!(editor.local_forwards.len(), 1);
        assert_eq!(editor.remote_forwards.len(), 1);
    }

    #[test]
    fn test_render_with_unicode_and_long_strings_smoke() {
        let mut editor = ConnectionEditorScreen::new();
        editor.name = "サーバー".to_string();
        editor.host = "例え.テスト".repeat(20);
        editor.notes = "notes ".repeat(200);
        render_smoke(&mut editor);
    }
}
