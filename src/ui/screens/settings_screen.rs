//! Settings screen UI

use crate::storage::settings::Settings;
use egui::{Context, Ui};

pub struct SettingsScreen {
    settings: Settings,
    modified: bool,
}

impl SettingsScreen {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            modified: false,
        }
    }

    pub fn render(&mut self, _ctx: &Context, ui: &mut Ui) -> Option<SettingsAction> {
        let mut action = None;

        ui.heading("Settings");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            // General
            ui.collapsing("General", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Default shell:");
                    if ui
                        .text_edit_singleline(&mut self.settings.default_shell)
                        .changed()
                    {
                        self.modified = true;
                    }
                });

                if ui
                    .checkbox(
                        &mut self.settings.auto_connect_on_startup,
                        "Auto-connect on startup",
                    )
                    .changed()
                {
                    self.modified = true;
                }

                if ui
                    .checkbox(
                        &mut self.settings.restore_previous_sessions,
                        "Restore previous sessions",
                    )
                    .changed()
                {
                    self.modified = true;
                }
            });

            ui.separator();

            // Terminal
            ui.collapsing("Terminal", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Font family:");
                    if ui
                        .text_edit_singleline(&mut self.settings.font_family)
                        .changed()
                    {
                        self.modified = true;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Font size:");
                    if ui
                        .add(egui::Slider::new(&mut self.settings.font_size, 8.0..=32.0))
                        .changed()
                    {
                        self.modified = true;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Scrollback lines:");
                    let mut lines = self.settings.scrollback_lines as i32;
                    if ui
                        .add(
                            egui::DragValue::new(&mut lines)
                                .speed(100)
                                .range(1000..=100000),
                        )
                        .changed()
                    {
                        self.settings.scrollback_lines = lines as usize;
                        self.modified = true;
                    }
                });

                if ui
                    .checkbox(&mut self.settings.cursor_blink, "Cursor blink")
                    .changed()
                {
                    self.modified = true;
                }
            });

            ui.separator();

            // Theme
            ui.collapsing("Theme", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Selected theme:");
                    egui::ComboBox::from_id_salt("theme_select")
                        .selected_text(&self.settings.selected_theme)
                        .show_ui(ui, |ui| {
                            let themes = vec![
                                "Default Dark",
                                "Dracula",
                                "Solarized Dark",
                                "Solarized Light",
                                "Nord",
                                "Monokai",
                                "Gruvbox Dark",
                                "One Dark",
                                "Tokyo Night",
                            ];
                            for theme in themes {
                                if ui
                                    .selectable_value(
                                        &mut self.settings.selected_theme,
                                        theme.to_string(),
                                        theme,
                                    )
                                    .changed()
                                {
                                    self.modified = true;
                                }
                            }
                        });
                });
            });

            ui.separator();

            // Connection
            ui.collapsing("Connection", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Default port:");
                    let mut port = self.settings.default_port as i32;
                    if ui
                        .add(egui::DragValue::new(&mut port).range(1..=65535))
                        .changed()
                    {
                        self.settings.default_port = port as u16;
                        self.modified = true;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Connection timeout (s):");
                    let mut timeout = self.settings.connection_timeout as i32;
                    if ui
                        .add(egui::DragValue::new(&mut timeout).range(5..=300))
                        .changed()
                    {
                        self.settings.connection_timeout = timeout as u32;
                        self.modified = true;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Keepalive interval (s):");
                    let mut interval = self.settings.keepalive_interval as i32;
                    if ui
                        .add(egui::DragValue::new(&mut interval).range(0..=600))
                        .changed()
                    {
                        self.settings.keepalive_interval = interval as u32;
                        self.modified = true;
                    }
                });

                if ui
                    .checkbox(&mut self.settings.compression, "Enable compression")
                    .changed()
                {
                    self.modified = true;
                }
            });

            ui.separator();

            // Security
            ui.collapsing("Security", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Auto-lock timeout (min, 0=disabled):");
                    let mut timeout = self.settings.auto_lock_timeout as i32;
                    if ui
                        .add(egui::DragValue::new(&mut timeout).range(0..=120))
                        .changed()
                    {
                        self.settings.auto_lock_timeout = timeout as u32;
                        self.modified = true;
                    }
                });

                if ui
                    .checkbox(&mut self.settings.remember_passwords, "Remember passwords")
                    .changed()
                {
                    self.modified = true;
                }

                if ui
                    .checkbox(
                        &mut self.settings.strict_host_key_checking,
                        "Strict host key checking",
                    )
                    .changed()
                {
                    self.modified = true;
                }
            });

            ui.separator();

            // Advanced
            ui.collapsing("Advanced", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Log level:");
                    egui::ComboBox::from_id_salt("log_level")
                        .selected_text(&self.settings.log_level)
                        .show_ui(ui, |ui| {
                            for level in &["error", "warn", "info", "debug", "trace"] {
                                if ui
                                    .selectable_value(
                                        &mut self.settings.log_level,
                                        level.to_string(),
                                        *level,
                                    )
                                    .changed()
                                {
                                    self.modified = true;
                                }
                            }
                        });
                });
            });
        });

        ui.separator();

        // Action buttons
        ui.horizontal(|ui| {
            if ui.button("💾 Save").clicked() && self.modified {
                action = Some(SettingsAction::Save(self.settings.clone()));
                self.modified = false;
            }

            if ui.button("↺ Reset to Defaults").clicked() {
                self.settings = Settings::default();
                self.modified = true;
            }

            if self.modified {
                ui.colored_label(egui::Color32::YELLOW, "● Modified");
            }
        });

        action
    }
}

#[derive(Debug, Clone)]
pub enum SettingsAction {
    Save(Settings),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::settings::{BellStyle, CursorStyle};

    /// Run `f` against a live `Context` + `Ui` pair inside a headless
    /// egui frame, matching the pattern in `keyboard.rs`.
    fn with_ctx_and_ui(f: impl FnOnce(&Context, &mut Ui)) {
        let ctx = Context::default();
        let mut f = Some(f);
        let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
            if let Some(f) = f.take() {
                let inner_ctx = ui.ctx().clone();
                f(&inner_ctx, ui);
            }
        });
    }

    #[test]
    fn test_new_starts_unmodified() {
        let screen = SettingsScreen::new(Settings::default());
        assert!(!screen.modified);
    }

    #[test]
    fn test_render_default_settings_without_interaction_returns_none() {
        let mut screen = SettingsScreen::new(Settings::default());
        with_ctx_and_ui(|ctx, ui| {
            let action = screen.render(ctx, ui);
            assert!(action.is_none());
        });
        // No user interaction occurred, so nothing should have changed.
        assert!(!screen.modified);
    }

    #[test]
    fn test_render_with_unicode_settings_does_not_panic() {
        let settings = Settings {
            default_shell: "/bin/日本語シェル".to_string(),
            font_family: "コード 🖥️".to_string(),
            selected_theme: "Custom テーマ".to_string(),
            log_level: "info".to_string(),
            ..Settings::default()
        };
        let mut screen = SettingsScreen::new(settings);
        with_ctx_and_ui(|ctx, ui| {
            screen.render(ctx, ui);
        });
    }

    #[test]
    fn test_render_with_empty_strings_does_not_panic() {
        let settings = Settings {
            default_shell: String::new(),
            font_family: String::new(),
            selected_theme: String::new(),
            log_level: String::new(),
            ..Settings::default()
        };
        let mut screen = SettingsScreen::new(settings);
        with_ctx_and_ui(|ctx, ui| {
            screen.render(ctx, ui);
        });
    }

    #[test]
    fn test_render_with_boundary_numeric_settings_does_not_panic() {
        let settings = Settings {
            font_size: 8.0,
            scrollback_lines: 1000,
            default_port: 1,
            connection_timeout: 5,
            keepalive_interval: 0,
            auto_lock_timeout: 0,
            ..Settings::default()
        };
        let mut screen = SettingsScreen::new(settings);
        with_ctx_and_ui(|ctx, ui| {
            screen.render(ctx, ui);
        });

        let settings_max = Settings {
            font_size: 32.0,
            scrollback_lines: 100_000,
            default_port: 65535,
            connection_timeout: 300,
            keepalive_interval: 600,
            auto_lock_timeout: 120,
            ..Settings::default()
        };
        let mut screen_max = SettingsScreen::new(settings_max);
        with_ctx_and_ui(|ctx, ui| {
            screen_max.render(ctx, ui);
        });
    }

    #[test]
    fn test_render_with_all_cursor_and_bell_style_variants() {
        for cursor_style in [
            CursorStyle::Block,
            CursorStyle::Beam,
            CursorStyle::Underline,
        ] {
            for bell_style in [BellStyle::None, BellStyle::Visual, BellStyle::Audio] {
                let settings = Settings {
                    cursor_style: cursor_style.clone(),
                    bell_style: bell_style.clone(),
                    ..Settings::default()
                };
                let mut screen = SettingsScreen::new(settings);
                with_ctx_and_ui(|ctx, ui| {
                    screen.render(ctx, ui);
                });
            }
        }
    }

    #[test]
    fn test_screen_retains_constructed_settings_fields() {
        let settings = Settings {
            font_size: 21.5,
            default_shell: "/bin/zsh".to_string(),
            ..Settings::default()
        };
        let screen = SettingsScreen::new(settings);
        assert_eq!(screen.settings.font_size, 21.5);
        assert_eq!(screen.settings.default_shell, "/bin/zsh");
    }

    #[test]
    fn test_settings_action_save_debug_and_clone() {
        let action = SettingsAction::Save(Settings::default());
        let cloned = action.clone();
        match cloned {
            SettingsAction::Save(settings) => {
                assert_eq!(settings.default_shell, Settings::default().default_shell);
            }
        }
        assert!(!format!("{:?}", action).is_empty());
    }
}
