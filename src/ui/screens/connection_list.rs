//! Connection list screen

use egui::{Context, Ui};

pub struct ConnectionListScreen {
    search_query: String,
    selected_connection: Option<String>,
}

impl ConnectionListScreen {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            selected_connection: None,
        }
    }

    pub fn render(&mut self, _ctx: &Context, ui: &mut Ui) -> Option<ConnectionAction> {
        let mut action = None;

        ui.heading("Connections");

        // Search bar
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_query);

            if ui.button("➕ New").clicked() {
                action = Some(ConnectionAction::New);
            }

            if ui.button("📥 Import SSH Config").clicked() {
                action = Some(ConnectionAction::ImportConfig);
            }
        });

        ui.separator();

        // Connection groups/categories
        ui.collapsing("Recent", |ui| {
            self.render_connection_list(ui, &mut action, true);
        });

        ui.collapsing("All Connections", |ui| {
            self.render_connection_list(ui, &mut action, false);
        });

        action
    }

    fn render_connection_list(
        &mut self,
        ui: &mut Ui,
        action: &mut Option<ConnectionAction>,
        _recent_only: bool,
    ) {
        let connections = vec![
            ("Production Server", "prod.example.com", "22", "admin"),
            ("Dev Server", "dev.example.com", "22", "user"),
            ("Database Server", "db.example.com", "22", "dbadmin"),
        ];

        for (name, host, port, user) in connections {
            ui.horizontal(|ui| {
                let is_selected = self.selected_connection.as_deref() == Some(name);

                if ui
                    .selectable_label(is_selected, format!("🖥 {}", name))
                    .clicked()
                {
                    self.selected_connection = Some(name.to_string());
                }

                ui.label(format!("{}@{}:{}", user, host, port));

                if ui.small_button("🔌").clicked() {
                    *action = Some(ConnectionAction::Connect(name.to_string()));
                }

                if ui.small_button("✏").clicked() {
                    *action = Some(ConnectionAction::Edit(name.to_string()));
                }

                if ui.small_button("🗑").clicked() {
                    *action = Some(ConnectionAction::Delete(name.to_string()));
                }
            });
        }
    }
}

impl Default for ConnectionListScreen {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionAction {
    New,
    Connect(String),
    Edit(String),
    Delete(String),
    ImportConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_has_empty_search_and_no_selection() {
        let screen = ConnectionListScreen::new();
        assert_eq!(screen.search_query, "");
        assert!(screen.selected_connection.is_none());
    }

    #[test]
    fn test_default_trait_matches_new() {
        let screen = ConnectionListScreen::default();
        assert_eq!(screen.search_query, "");
        assert!(screen.selected_connection.is_none());
    }

    #[test]
    fn test_connection_action_debug_and_clone() {
        let action = ConnectionAction::Connect("Production Server".to_string());
        let cloned = action.clone();
        assert!(!format!("{:?}", action).is_empty());
        match cloned {
            ConnectionAction::Connect(name) => assert_eq!(name, "Production Server"),
            _ => panic!("expected Connect variant"),
        }
    }

    /// Render the connection list screen inside a headless egui context and
    /// make sure it does not panic for a given screen state.
    fn render_smoke(screen: &mut ConnectionListScreen) -> Option<ConnectionAction> {
        let ctx = Context::default();
        let mut result = None;
        let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
            result = screen.render(&ctx, ui);
        });
        result
    }

    #[test]
    fn test_render_default_state_smoke() {
        let mut screen = ConnectionListScreen::new();
        let action = render_smoke(&mut screen);
        // No widgets are clicked in a headless pass, so no action fires.
        assert!(action.is_none());
    }

    #[test]
    fn test_render_with_search_query_smoke() {
        let mut screen = ConnectionListScreen::new();
        screen.search_query = "prod".to_string();
        render_smoke(&mut screen);
        assert_eq!(screen.search_query, "prod");
    }

    #[test]
    fn test_render_with_selected_connection_smoke() {
        let mut screen = ConnectionListScreen::new();
        screen.selected_connection = Some("Production Server".to_string());
        render_smoke(&mut screen);
        assert_eq!(
            screen.selected_connection,
            Some("Production Server".to_string())
        );
    }

    #[test]
    fn test_render_with_unicode_search_query_smoke() {
        let mut screen = ConnectionListScreen::new();
        screen.search_query = "サーバー 🔍".to_string();
        render_smoke(&mut screen);
    }

    #[test]
    fn test_render_connection_list_recent_only_smoke() {
        let mut screen = ConnectionListScreen::new();
        let ctx = Context::default();
        let mut action = None;
        let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
            screen.render_connection_list(ui, &mut action, true);
        });
        assert!(action.is_none());
    }

    #[test]
    fn test_render_connection_list_all_smoke() {
        let mut screen = ConnectionListScreen::new();
        let ctx = Context::default();
        let mut action = None;
        let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
            screen.render_connection_list(ui, &mut action, false);
        });
        assert!(action.is_none());
    }
}
