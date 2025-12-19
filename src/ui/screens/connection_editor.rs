//! Connection Editor Screen - form for creating/editing SSH connections

#![allow(dead_code)]

use eframe::egui::{self, RichText};
use crate::ui::components::{colors, spacing, primary_button, secondary_button, danger_button,
    labeled_input, labeled_number, labeled_toggle, labeled_dropdown, section_header, card, form_row};
use super::connection_manager::{ConnectionProfile, AuthType};

/// Authentication method for the form
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
    // Basic settings
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,

    // Authentication
    pub auth_method: FormAuthMethod,
    pub password: String,
    pub private_key_path: String,
    pub passphrase: String,
    pub save_password: bool,

    // Advanced SSH options
    pub compression: bool,
    pub keepalive_interval: u16,
    pub connection_timeout: u16,
    pub tcp_keepalive: bool,

    // Terminal settings
    pub terminal_type: String,
    pub initial_command: String,
    pub encoding: String,

    // Forwarding
    pub enable_x11_forwarding: bool,
    pub enable_agent_forwarding: bool,
    pub local_forwards: Vec<PortForward>,
    pub remote_forwards: Vec<PortForward>,

    // Jump host / proxy
    pub use_jump_host: bool,
    pub jump_host: String,
    pub jump_port: u16,
    pub jump_username: String,

    // Organization
    pub group: String,
    pub is_favorite: bool,
    pub notes: String,

    // Edit mode
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
            AuthType::Password => FormAuthMethod::Password,
            AuthType::PublicKey => FormAuthMethod::PublicKey,
            AuthType::KeyboardInteractive => FormAuthMethod::KeyboardInteractive,
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
            let title = if is_new { "New Connection" } else { "Edit Connection" };

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

            // Basic Settings Section
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

            // Authentication Section
            section_header(ui, "Authentication");

            card(ui, |ui| {
                form_row(ui, |ui| {
                    let auth_methods = [
                        FormAuthMethod::Password,
                        FormAuthMethod::PublicKey,
                        FormAuthMethod::KeyboardInteractive,
                        FormAuthMethod::Agent,
                    ];
                    labeled_dropdown(ui, "Method", "auth_method", &mut self.auth_method, &auth_methods);
                });

                match self.auth_method {
                    FormAuthMethod::Password => {
                        form_row(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Password").color(colors::TEXT_PRIMARY));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let input = egui::TextEdit::singleline(&mut self.password)
                                        .hint_text(RichText::new("Enter password").color(colors::TEXT_MUTED))
                                        .text_color(colors::TEXT_PRIMARY)
                                        .password(true)
                                        .desired_width(200.0)
                                        .margin(egui::Margin::symmetric(8.0, 6.0));
                                    ui.add(input);
                                });
                            });
                        });

                        form_row(ui, |ui| {
                            labeled_toggle(ui, "Save password in keychain", &mut self.save_password);
                        });
                    }
                    FormAuthMethod::PublicKey => {
                        form_row(ui, |ui| {
                            ui.horizontal(|ui| {
                                labeled_input(ui, "Private Key", &mut self.private_key_path, "~/.ssh/id_ed25519");
                                if secondary_button(ui, "Browse...").clicked() {
                                    // TODO: File picker
                                }
                            });
                        });

                        form_row(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Passphrase").color(colors::TEXT_PRIMARY));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let input = egui::TextEdit::singleline(&mut self.passphrase)
                                        .hint_text(RichText::new("Optional").color(colors::TEXT_MUTED))
                                        .text_color(colors::TEXT_PRIMARY)
                                        .password(true)
                                        .desired_width(200.0)
                                        .margin(egui::Margin::symmetric(8.0, 6.0));
                                    ui.add(input);
                                });
                            });
                        });
                    }
                    FormAuthMethod::KeyboardInteractive => {
                        ui.label(RichText::new("You will be prompted for authentication during connection.")
                            .color(colors::TEXT_SECONDARY)
                            .size(12.0));
                    }
                    FormAuthMethod::Agent => {
                        ui.label(RichText::new("SSH Agent will be used for authentication. Make sure your agent is running and has the appropriate key loaded.")
                            .color(colors::TEXT_SECONDARY)
                            .size(12.0));
                    }
                }
            });

            // Terminal Settings Section
            section_header(ui, "Terminal");

            card(ui, |ui| {
                form_row(ui, |ui| {
                    let term_types = ["xterm-256color", "xterm", "vt100", "linux", "screen"];
                    let term_idx = term_types.iter().position(|&t| t == self.terminal_type).unwrap_or(0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Terminal Type").color(colors::TEXT_PRIMARY));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            egui::ComboBox::from_id_source("term_type")
                                .selected_text(RichText::new(&self.terminal_type).color(colors::TEXT_PRIMARY))
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
                    labeled_input(ui, "Initial Command", &mut self.initial_command, "Optional command to run on connect");
                });

                form_row(ui, |ui| {
                    let encodings = ["UTF-8", "ISO-8859-1", "GBK", "Big5"];
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Character Encoding").color(colors::TEXT_PRIMARY));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            egui::ComboBox::from_id_source("encoding")
                                .selected_text(RichText::new(&self.encoding).color(colors::TEXT_PRIMARY))
                                .width(200.0)
                                .show_ui(ui, |ui: &mut egui::Ui| {
                                    for enc in encodings {
                                        if ui.selectable_label(self.encoding == enc, enc).clicked() {
                                            self.encoding = enc.to_string();
                                        }
                                    }
                                });
                        });
                    });
                });
            });

            // Advanced SSH Options Section
            section_header(ui, "Advanced SSH Options");

            card(ui, |ui| {
                form_row(ui, |ui| {
                    labeled_toggle(ui, "Enable compression", &mut self.compression);
                });

                form_row(ui, |ui| {
                    labeled_toggle(ui, "TCP keep-alive", &mut self.tcp_keepalive);
                });

                form_row(ui, |ui| {
                    labeled_number(ui, "Keep-alive interval (seconds)", &mut self.keepalive_interval, 0, 600);
                });

                form_row(ui, |ui| {
                    labeled_number(ui, "Connection timeout (seconds)", &mut self.connection_timeout, 5, 300);
                });
            });

            // Forwarding Section
            section_header(ui, "Forwarding");

            card(ui, |ui| {
                form_row(ui, |ui| {
                    labeled_toggle(ui, "Enable X11 forwarding", &mut self.enable_x11_forwarding);
                });

                form_row(ui, |ui| {
                    labeled_toggle(ui, "Enable agent forwarding", &mut self.enable_agent_forwarding);
                });

                ui.add_space(spacing::SM);
                ui.label(RichText::new("Port Forwarding").color(colors::TEXT_SECONDARY).size(13.0));
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

                // Display existing forwards
                for (i, fwd) in self.local_forwards.clone().iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("L: {}:{}:{}", fwd.local_port, fwd.remote_host, fwd.remote_port))
                            .color(colors::TEXT_SECONDARY));
                        if ui.small_button("x").clicked() {
                            self.local_forwards.remove(i);
                        }
                    });
                }

                for (i, fwd) in self.remote_forwards.clone().iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("R: {}:{}:{}", fwd.local_port, fwd.remote_host, fwd.remote_port))
                            .color(colors::TEXT_SECONDARY));
                        if ui.small_button("x").clicked() {
                            self.remote_forwards.remove(i);
                        }
                    });
                }
            });

            // Jump Host Section
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
                        labeled_input(ui, "Jump Username", &mut self.jump_username, "Same as connection if empty");
                    });
                }
            });

            // Organization Section
            section_header(ui, "Organization");

            card(ui, |ui| {
                form_row(ui, |ui| {
                    labeled_input(ui, "Group", &mut self.group, "Production, Development, etc.");
                });

                form_row(ui, |ui| {
                    labeled_toggle(ui, "Add to favorites", &mut self.is_favorite);
                });

                ui.add_space(spacing::SM);
                ui.label(RichText::new("Notes").color(colors::TEXT_PRIMARY));
                ui.add_space(spacing::XS);

                let notes_input = egui::TextEdit::multiline(&mut self.notes)
                    .hint_text(RichText::new("Optional notes about this connection").color(colors::TEXT_MUTED))
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
            id: self.editing_id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            name: if self.name.is_empty() {
                format!("{}@{}", self.username, self.host)
            } else {
                self.name.clone()
            },
            host: self.host.clone(),
            port: self.port,
            username: self.username.clone(),
            auth_type: match self.auth_method {
                FormAuthMethod::Password => AuthType::Password,
                FormAuthMethod::PublicKey => AuthType::PublicKey,
                FormAuthMethod::KeyboardInteractive => AuthType::KeyboardInteractive,
                FormAuthMethod::Agent => AuthType::PublicKey, // Agent uses public key auth
            },
            group: if self.group.is_empty() { None } else { Some(self.group.clone()) },
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
