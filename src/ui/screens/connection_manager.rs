//! Connection Manager Screen - displays list of saved connections

use eframe::egui::{self, RichText, Vec2};
use crate::ui::components::{colors, spacing, primary_button, secondary_button, icon_button, empty_state};

/// Connection profile for display
#[derive(Clone)]
pub struct ConnectionProfile {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth_type: AuthType,
    pub group: Option<String>,
    pub last_connected: Option<String>,
    pub is_favorite: bool,
}

#[derive(Clone, PartialEq)]
pub enum AuthType {
    Password,
    PublicKey,
    KeyboardInteractive,
}

impl std::fmt::Display for AuthType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthType::Password => write!(f, "Password"),
            AuthType::PublicKey => write!(f, "Public Key"),
            AuthType::KeyboardInteractive => write!(f, "Keyboard Interactive"),
        }
    }
}

impl Default for ConnectionProfile {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::new(),
            host: String::new(),
            port: 22,
            username: String::from("root"),
            auth_type: AuthType::Password,
            group: None,
            last_connected: None,
            is_favorite: false,
        }
    }
}

/// Connection manager screen state
pub struct ConnectionManagerScreen {
    pub connections: Vec<ConnectionProfile>,
    pub search_query: String,
    pub selected_connection_id: Option<String>,
    pub selected_group: Option<String>,
    pub groups: Vec<String>,
}

impl Default for ConnectionManagerScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionManagerScreen {
    pub fn new() -> Self {
        // Sample connections for demo
        let connections = vec![
            ConnectionProfile {
                id: "1".to_string(),
                name: "Production Server".to_string(),
                host: "prod.example.com".to_string(),
                port: 22,
                username: "admin".to_string(),
                auth_type: AuthType::PublicKey,
                group: Some("Production".to_string()),
                last_connected: Some("2024-01-15 14:30".to_string()),
                is_favorite: true,
            },
            ConnectionProfile {
                id: "2".to_string(),
                name: "Dev Server".to_string(),
                host: "dev.example.com".to_string(),
                port: 22,
                username: "developer".to_string(),
                auth_type: AuthType::Password,
                group: Some("Development".to_string()),
                last_connected: Some("2024-01-14 09:15".to_string()),
                is_favorite: false,
            },
            ConnectionProfile {
                id: "3".to_string(),
                name: "Database Server".to_string(),
                host: "db.example.com".to_string(),
                port: 2222,
                username: "dba".to_string(),
                auth_type: AuthType::PublicKey,
                group: Some("Production".to_string()),
                last_connected: None,
                is_favorite: true,
            },
        ];

        let groups = vec![
            "All Connections".to_string(),
            "Favorites".to_string(),
            "Production".to_string(),
            "Development".to_string(),
        ];

        Self {
            connections,
            search_query: String::new(),
            selected_connection_id: None,
            selected_group: Some("All Connections".to_string()),
            groups,
        }
    }

    /// Render the connection manager
    pub fn render(&mut self, ui: &mut egui::Ui) -> Option<ConnectionManagerAction> {
        let mut action = None;

        ui.horizontal(|ui| {
            // Left: Group list sidebar
            ui.vertical(|ui| {
                ui.set_min_width(180.0);
                ui.set_max_width(180.0);

                ui.label(RichText::new("Groups").color(colors::TEXT_SECONDARY).size(12.0));
                ui.add_space(spacing::SM);

                for group in &self.groups.clone() {
                    let selected = self.selected_group.as_ref() == Some(group);
                    let icon = match group.as_str() {
                        "All Connections" => "\u{1F4C1}",
                        "Favorites" => "\u{2B50}",
                        _ => "\u{1F4C2}",
                    };

                    let bg = if selected { colors::BG_TERTIARY } else { egui::Color32::TRANSPARENT };
                    let text_color = if selected { colors::TEXT_PRIMARY } else { colors::TEXT_SECONDARY };

                    let button = egui::Button::new(
                        RichText::new(format!("{} {}", icon, group))
                            .color(text_color)
                            .size(13.0)
                    )
                        .fill(bg)
                        .stroke(egui::Stroke::NONE)
                        .rounding(egui::Rounding::same(4.0))
                        .min_size(Vec2::new(ui.available_width(), 32.0));

                    if ui.add(button).clicked() {
                        self.selected_group = Some(group.clone());
                    }
                }

                ui.add_space(spacing::LG);

                if secondary_button(ui, "+ New Group").clicked() {
                    // TODO: Add new group
                }
            });

            ui.separator();

            // Right: Connection list
            ui.vertical(|ui| {
                // Header with search and actions
                ui.horizontal(|ui| {
                    ui.add_space(spacing::SM);

                    // Search box
                    let search_input = egui::TextEdit::singleline(&mut self.search_query)
                        .hint_text(RichText::new("\u{1F50D} Search connections...").color(colors::TEXT_MUTED))
                        .text_color(colors::TEXT_PRIMARY)
                        .desired_width(250.0)
                        .margin(egui::Margin::symmetric(8.0, 6.0));
                    ui.add(search_input);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if primary_button(ui, "+ New Connection").clicked() {
                            action = Some(ConnectionManagerAction::NewConnection);
                        }

                        ui.add_space(spacing::SM);

                        if icon_button(ui, "\u{2699}", "Import connections").clicked() {
                            // TODO: Import
                        }
                    });
                });

                ui.add_space(spacing::MD);
                ui.separator();
                ui.add_space(spacing::SM);

                // Connection list
                let filtered: Vec<_> = self.connections.iter()
                    .filter(|c| {
                        let matches_search = self.search_query.is_empty()
                            || c.name.to_lowercase().contains(&self.search_query.to_lowercase())
                            || c.host.to_lowercase().contains(&self.search_query.to_lowercase());

                        let matches_group = match self.selected_group.as_deref() {
                            Some("All Connections") => true,
                            Some("Favorites") => c.is_favorite,
                            Some(g) => c.group.as_deref() == Some(g),
                            None => true,
                        };

                        matches_search && matches_group
                    })
                    .collect();

                if filtered.is_empty() {
                    empty_state(
                        ui,
                        "\u{1F4E1}",
                        "No Connections Found",
                        "Create a new connection to get started"
                    );
                } else {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for conn in filtered {
                            let is_selected = self.selected_connection_id.as_ref() == Some(&conn.id);

                            egui::Frame::none()
                                .fill(if is_selected { colors::BG_TERTIARY } else { colors::BG_SECONDARY })
                                .rounding(egui::Rounding::same(6.0))
                                .inner_margin(egui::Margin::same(spacing::MD))
                                .stroke(egui::Stroke::new(
                                    1.0,
                                    if is_selected { colors::PRIMARY } else { colors::BORDER }
                                ))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        // Favorite star
                                        let star = if conn.is_favorite { "\u{2B50}" } else { "\u{2606}" };
                                        if ui.add(egui::Button::new(star).frame(false)).clicked() {
                                            // TODO: Toggle favorite
                                        }

                                        ui.vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(RichText::new(&conn.name)
                                                    .color(colors::TEXT_PRIMARY)
                                                    .strong()
                                                    .size(14.0));

                                                ui.add_space(spacing::SM);

                                                // Auth type badge
                                                let auth_badge = match conn.auth_type {
                                                    AuthType::Password => "\u{1F511}",
                                                    AuthType::PublicKey => "\u{1F5DD}",
                                                    AuthType::KeyboardInteractive => "\u{2328}",
                                                };
                                                ui.label(RichText::new(auth_badge).size(12.0));
                                            });

                                            ui.label(RichText::new(format!("{}@{}:{}", conn.username, conn.host, conn.port))
                                                .color(colors::TEXT_SECONDARY)
                                                .size(12.0));

                                            if let Some(last) = &conn.last_connected {
                                                ui.label(RichText::new(format!("Last: {}", last))
                                                    .color(colors::TEXT_MUTED)
                                                    .size(11.0));
                                            }
                                        });

                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if primary_button(ui, "Connect").clicked() {
                                                action = Some(ConnectionManagerAction::Connect(conn.clone()));
                                            }

                                            ui.add_space(spacing::XS);

                                            if icon_button(ui, "\u{270F}", "Edit").clicked() {
                                                action = Some(ConnectionManagerAction::Edit(conn.id.clone()));
                                            }

                                            if icon_button(ui, "\u{1F5D1}", "Delete").clicked() {
                                                action = Some(ConnectionManagerAction::Delete(conn.id.clone()));
                                            }
                                        });
                                    });
                                });

                            ui.add_space(spacing::SM);

                            // Select on click
                            let response = ui.interact(
                                ui.min_rect(),
                                ui.id().with(&conn.id),
                                egui::Sense::click()
                            );
                            if response.clicked() {
                                self.selected_connection_id = Some(conn.id.clone());
                            }
                        }
                    });
                }
            });
        });

        action
    }
}

/// Actions that can be triggered from the connection manager
pub enum ConnectionManagerAction {
    Connect(ConnectionProfile),
    Edit(String),
    Delete(String),
    NewConnection,
}
