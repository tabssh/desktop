//! Settings screen UI

use egui::{Context, Ui};
use crate::storage::settings::{Settings, CursorStyle, BellStyle};

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
    
    pub fn render(&mut self, ctx: &Context, ui: &mut Ui) -> Option<SettingsAction> {
        let mut action = None;
        
        ui.heading("Settings");
        ui.separator();
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            // General
            ui.collapsing("General", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Default shell:");
                    if ui.text_edit_singleline(&mut self.settings.default_shell).changed() {
                        self.modified = true;
                    }
                });
                
                if ui.checkbox(&mut self.settings.auto_connect_on_startup, "Auto-connect on startup").changed() {
                    self.modified = true;
                }
                
                if ui.checkbox(&mut self.settings.restore_previous_sessions, "Restore previous sessions").changed() {
                    self.modified = true;
                }
            });
            
            ui.separator();
            
            // Terminal
            ui.collapsing("Terminal", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Font family:");
                    if ui.text_edit_singleline(&mut self.settings.font_family).changed() {
                        self.modified = true;
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Font size:");
                    if ui.add(egui::Slider::new(&mut self.settings.font_size, 8.0..=32.0)).changed() {
                        self.modified = true;
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Scrollback lines:");
                    let mut lines = self.settings.scrollback_lines as i32;
                    if ui.add(egui::DragValue::new(&mut lines).speed(100).clamp_range(1000..=100000)).changed() {
                        self.settings.scrollback_lines = lines as usize;
                        self.modified = true;
                    }
                });
                
                if ui.checkbox(&mut self.settings.cursor_blink, "Cursor blink").changed() {
                    self.modified = true;
                }
            });
            
            ui.separator();
            
            // Theme
            ui.collapsing("Theme", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Selected theme:");
                    egui::ComboBox::from_id_source("theme_select")
                        .selected_text(&self.settings.selected_theme)
                        .show_ui(ui, |ui| {
                            let themes = vec![
                                "Default Dark", "Dracula", "Solarized Dark", "Solarized Light",
                                "Nord", "Monokai", "Gruvbox Dark", "One Dark", "Tokyo Night"
                            ];
                            for theme in themes {
                                if ui.selectable_value(&mut self.settings.selected_theme, theme.to_string(), theme).changed() {
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
                    if ui.add(egui::DragValue::new(&mut port).clamp_range(1..=65535)).changed() {
                        self.settings.default_port = port as u16;
                        self.modified = true;
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Connection timeout (s):");
                    let mut timeout = self.settings.connection_timeout as i32;
                    if ui.add(egui::DragValue::new(&mut timeout).clamp_range(5..=300)).changed() {
                        self.settings.connection_timeout = timeout as u32;
                        self.modified = true;
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Keepalive interval (s):");
                    let mut interval = self.settings.keepalive_interval as i32;
                    if ui.add(egui::DragValue::new(&mut interval).clamp_range(0..=600)).changed() {
                        self.settings.keepalive_interval = interval as u32;
                        self.modified = true;
                    }
                });
                
                if ui.checkbox(&mut self.settings.compression, "Enable compression").changed() {
                    self.modified = true;
                }
            });
            
            ui.separator();
            
            // Security
            ui.collapsing("Security", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Auto-lock timeout (min, 0=disabled):");
                    let mut timeout = self.settings.auto_lock_timeout as i32;
                    if ui.add(egui::DragValue::new(&mut timeout).clamp_range(0..=120)).changed() {
                        self.settings.auto_lock_timeout = timeout as u32;
                        self.modified = true;
                    }
                });
                
                if ui.checkbox(&mut self.settings.remember_passwords, "Remember passwords").changed() {
                    self.modified = true;
                }
                
                if ui.checkbox(&mut self.settings.strict_host_key_checking, "Strict host key checking").changed() {
                    self.modified = true;
                }
            });
            
            ui.separator();
            
            // Advanced
            ui.collapsing("Advanced", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Log level:");
                    egui::ComboBox::from_id_source("log_level")
                        .selected_text(&self.settings.log_level)
                        .show_ui(ui, |ui| {
                            for level in &["error", "warn", "info", "debug", "trace"] {
                                if ui.selectable_value(&mut self.settings.log_level, level.to_string(), *level).changed() {
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
