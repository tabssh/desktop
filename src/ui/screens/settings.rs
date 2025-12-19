//! Settings Screen - application preferences with categories

use eframe::egui::{self, RichText};
use crate::ui::components::{colors, spacing, primary_button, secondary_button, danger_button,
    labeled_toggle, labeled_dropdown, labeled_number, section_header, card, form_row, nav_item};

/// Settings category
#[derive(Clone, Copy, PartialEq)]
pub enum SettingsCategory {
    General,
    Appearance,
    Terminal,
    SSH,
    Security,
    KeyManagement,
    Backup,
}

impl SettingsCategory {
    fn icon(&self) -> &str {
        match self {
            SettingsCategory::General => "\u{2699}",
            SettingsCategory::Appearance => "\u{1F3A8}",
            SettingsCategory::Terminal => "\u{1F5A5}",
            SettingsCategory::SSH => "\u{1F511}",
            SettingsCategory::Security => "\u{1F512}",
            SettingsCategory::KeyManagement => "\u{1F5DD}",
            SettingsCategory::Backup => "\u{1F4BE}",
        }
    }

    fn label(&self) -> &str {
        match self {
            SettingsCategory::General => "General",
            SettingsCategory::Appearance => "Appearance",
            SettingsCategory::Terminal => "Terminal",
            SettingsCategory::SSH => "SSH",
            SettingsCategory::Security => "Security",
            SettingsCategory::KeyManagement => "Key Management",
            SettingsCategory::Backup => "Backup & Sync",
        }
    }
}

/// Theme mode options
#[derive(Clone, Copy, PartialEq)]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeMode::Dark => write!(f, "Dark"),
            ThemeMode::Light => write!(f, "Light"),
            ThemeMode::System => write!(f, "System"),
        }
    }
}

/// Color theme options for terminal
#[derive(Clone, Copy, PartialEq)]
pub enum ColorTheme {
    Dracula,
    SolarizedDark,
    SolarizedLight,
    Nord,
    Monokai,
    OneDark,
    Gruvbox,
    TomorrowNight,
    HighContrast,
}

impl std::fmt::Display for ColorTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorTheme::Dracula => write!(f, "Dracula"),
            ColorTheme::SolarizedDark => write!(f, "Solarized Dark"),
            ColorTheme::SolarizedLight => write!(f, "Solarized Light"),
            ColorTheme::Nord => write!(f, "Nord"),
            ColorTheme::Monokai => write!(f, "Monokai"),
            ColorTheme::OneDark => write!(f, "One Dark"),
            ColorTheme::Gruvbox => write!(f, "Gruvbox"),
            ColorTheme::TomorrowNight => write!(f, "Tomorrow Night"),
            ColorTheme::HighContrast => write!(f, "High Contrast"),
        }
    }
}

/// Cursor style options
#[derive(Clone, PartialEq)]
pub enum CursorStyle {
    Block,
    Beam,
    Underline,
}

impl std::fmt::Display for CursorStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursorStyle::Block => write!(f, "Block"),
            CursorStyle::Beam => write!(f, "Beam"),
            CursorStyle::Underline => write!(f, "Underline"),
        }
    }
}

/// Settings screen state
pub struct SettingsScreen {
    pub active_category: SettingsCategory,

    // General settings
    pub start_minimized: bool,
    pub minimize_to_tray: bool,
    pub check_for_updates: bool,
    pub language: String,

    // Appearance settings
    pub theme_mode: ThemeMode,
    pub color_theme: ColorTheme,
    pub font_family: String,
    pub font_size: u16,
    pub window_opacity: u16,
    pub show_tab_close_buttons: bool,
    pub confirm_close_multiple_tabs: bool,

    // Terminal settings
    pub scrollback_lines: u32,
    pub cursor_style: CursorStyle,
    pub cursor_blink: bool,
    pub bell_enabled: bool,
    pub bell_visual: bool,
    pub word_wrap: bool,
    pub copy_on_select: bool,
    pub paste_on_right_click: bool,

    // SSH settings
    pub default_port: u16,
    pub default_username: String,
    pub default_terminal_type: String,
    pub connection_timeout: u16,
    pub keepalive_interval: u16,
    pub strict_host_key_checking: bool,
    pub compression_enabled: bool,
    pub prefer_ed25519: bool,

    // Security settings
    pub use_master_password: bool,
    pub auto_lock_enabled: bool,
    pub auto_lock_timeout: u16,
    pub clear_clipboard_after: u16,
    pub log_session_data: bool,

    // Key management settings
    pub default_key_path: String,
    pub agent_enabled: bool,
    pub auto_add_keys: bool,

    // Backup settings
    pub auto_backup_enabled: bool,
    pub backup_location: String,
    pub backup_interval_days: u16,
    pub encrypt_backups: bool,

    // Track changes
    pub has_unsaved_changes: bool,
}

impl Default for SettingsScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsScreen {
    pub fn new() -> Self {
        Self {
            active_category: SettingsCategory::General,

            // General defaults
            start_minimized: false,
            minimize_to_tray: true,
            check_for_updates: true,
            language: "English".to_string(),

            // Appearance defaults
            theme_mode: ThemeMode::Dark,
            color_theme: ColorTheme::Dracula,
            font_family: "JetBrains Mono".to_string(),
            font_size: 14,
            window_opacity: 100,
            show_tab_close_buttons: true,
            confirm_close_multiple_tabs: true,

            // Terminal defaults
            scrollback_lines: 10000,
            cursor_style: CursorStyle::Block,
            cursor_blink: true,
            bell_enabled: true,
            bell_visual: false,
            word_wrap: false,
            copy_on_select: false,
            paste_on_right_click: true,

            // SSH defaults
            default_port: 22,
            default_username: "root".to_string(),
            default_terminal_type: "xterm-256color".to_string(),
            connection_timeout: 30,
            keepalive_interval: 30,
            strict_host_key_checking: true,
            compression_enabled: false,
            prefer_ed25519: true,

            // Security defaults
            use_master_password: false,
            auto_lock_enabled: false,
            auto_lock_timeout: 5,
            clear_clipboard_after: 30,
            log_session_data: false,

            // Key management defaults
            default_key_path: "~/.ssh/id_ed25519".to_string(),
            agent_enabled: true,
            auto_add_keys: true,

            // Backup defaults
            auto_backup_enabled: false,
            backup_location: String::new(),
            backup_interval_days: 7,
            encrypt_backups: true,

            has_unsaved_changes: false,
        }
    }

    /// Render the settings screen
    pub fn render(&mut self, ui: &mut egui::Ui) -> Option<SettingsAction> {
        let mut action = None;

        ui.horizontal(|ui| {
            // Left sidebar: category navigation
            ui.vertical(|ui| {
                ui.set_min_width(180.0);
                ui.set_max_width(180.0);

                ui.label(RichText::new("Settings").color(colors::TEXT_PRIMARY).strong().size(18.0));
                ui.add_space(spacing::LG);

                let categories = [
                    SettingsCategory::General,
                    SettingsCategory::Appearance,
                    SettingsCategory::Terminal,
                    SettingsCategory::SSH,
                    SettingsCategory::Security,
                    SettingsCategory::KeyManagement,
                    SettingsCategory::Backup,
                ];

                for cat in categories {
                    if nav_item(ui, cat.icon(), cat.label(), self.active_category == cat).clicked() {
                        self.active_category = cat;
                    }
                }

                ui.add_space(spacing::XXL);

                // Save/Reset buttons
                if self.has_unsaved_changes {
                    if primary_button(ui, "Save Changes").clicked() {
                        action = Some(SettingsAction::Save);
                        self.has_unsaved_changes = false;
                    }
                    ui.add_space(spacing::SM);
                    if secondary_button(ui, "Reset").clicked() {
                        action = Some(SettingsAction::Reset);
                        self.has_unsaved_changes = false;
                    }
                }
            });

            ui.separator();

            // Right content area
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_min_width(500.0);
                ui.add_space(spacing::MD);

                match self.active_category {
                    SettingsCategory::General => self.render_general(ui),
                    SettingsCategory::Appearance => self.render_appearance(ui),
                    SettingsCategory::Terminal => self.render_terminal(ui),
                    SettingsCategory::SSH => self.render_ssh(ui),
                    SettingsCategory::Security => self.render_security(ui),
                    SettingsCategory::KeyManagement => self.render_key_management(ui),
                    SettingsCategory::Backup => self.render_backup(ui),
                }
            });
        });

        action
    }

    fn render_general(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("General Settings").color(colors::TEXT_PRIMARY));
        ui.add_space(spacing::LG);

        card(ui, |ui| {
            section_header(ui, "Startup");

            form_row(ui, |ui| {
                if labeled_toggle(ui, "Start minimized to system tray", &mut self.start_minimized).changed() {
                    self.has_unsaved_changes = true;
                }
            });

            form_row(ui, |ui| {
                if labeled_toggle(ui, "Minimize to system tray on close", &mut self.minimize_to_tray).changed() {
                    self.has_unsaved_changes = true;
                }
            });

            form_row(ui, |ui| {
                if labeled_toggle(ui, "Check for updates automatically", &mut self.check_for_updates).changed() {
                    self.has_unsaved_changes = true;
                }
            });

            section_header(ui, "Language");

            form_row(ui, |ui| {
                let languages = ["English", "Spanish", "French", "German", "Chinese", "Japanese"];
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Interface Language").color(colors::TEXT_PRIMARY));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::ComboBox::from_id_source("language")
                            .selected_text(RichText::new(&self.language).color(colors::TEXT_PRIMARY))
                            .width(200.0)
                            .show_ui(ui, |ui: &mut egui::Ui| {
                                for lang in languages {
                                    if ui.selectable_label(self.language == lang, lang).clicked() {
                                        self.language = lang.to_string();
                                        self.has_unsaved_changes = true;
                                    }
                                }
                            });
                    });
                });
            });
        });
    }

    fn render_appearance(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("Appearance").color(colors::TEXT_PRIMARY));
        ui.add_space(spacing::LG);

        card(ui, |ui| {
            section_header(ui, "Theme Mode");

            let theme_modes = [
                ThemeMode::Dark,
                ThemeMode::Light,
                ThemeMode::System,
            ];

            form_row(ui, |ui| {
                labeled_dropdown(ui, "Theme Mode", "theme_mode", &mut self.theme_mode, &theme_modes);
            });

            ui.label(RichText::new("Controls the application UI appearance")
                .color(colors::TEXT_MUTED)
                .size(11.0));

            section_header(ui, "Terminal Color Theme");

            let color_themes = [
                ColorTheme::Dracula,
                ColorTheme::SolarizedDark,
                ColorTheme::SolarizedLight,
                ColorTheme::Nord,
                ColorTheme::Monokai,
                ColorTheme::OneDark,
                ColorTheme::Gruvbox,
                ColorTheme::TomorrowNight,
                ColorTheme::HighContrast,
            ];

            form_row(ui, |ui| {
                labeled_dropdown(ui, "Color Theme", "color_theme", &mut self.color_theme, &color_themes);
            });

            ui.label(RichText::new("Colors used for terminal sessions")
                .color(colors::TEXT_MUTED)
                .size(11.0));

            section_header(ui, "Font");

            let fonts = ["JetBrains Mono", "Fira Code", "Source Code Pro", "Cascadia Code", "Consolas", "Monaco"];
            form_row(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Font Family").color(colors::TEXT_PRIMARY));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::ComboBox::from_id_source("font_family")
                            .selected_text(RichText::new(&self.font_family).color(colors::TEXT_PRIMARY))
                            .width(200.0)
                            .show_ui(ui, |ui: &mut egui::Ui| {
                                for font in fonts {
                                    if ui.selectable_label(self.font_family == font, font).clicked() {
                                        self.font_family = font.to_string();
                                        self.has_unsaved_changes = true;
                                    }
                                }
                            });
                    });
                });
            });

            form_row(ui, |ui| {
                labeled_number(ui, "Font Size", &mut self.font_size, 8, 32);
            });

            section_header(ui, "Window");

            form_row(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Window Opacity").color(colors::TEXT_PRIMARY));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(egui::Slider::new(&mut self.window_opacity, 50..=100).suffix("%"));
                    });
                });
            });

            section_header(ui, "Tabs");

            form_row(ui, |ui| {
                labeled_toggle(ui, "Show close button on tabs", &mut self.show_tab_close_buttons);
            });

            form_row(ui, |ui| {
                labeled_toggle(ui, "Confirm when closing multiple tabs", &mut self.confirm_close_multiple_tabs);
            });
        });
    }

    fn render_terminal(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("Terminal Settings").color(colors::TEXT_PRIMARY));
        ui.add_space(spacing::LG);

        card(ui, |ui| {
            section_header(ui, "Scrollback");

            form_row(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Scrollback Lines").color(colors::TEXT_PRIMARY));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(egui::Slider::new(&mut self.scrollback_lines, 1000..=100000).logarithmic(true));
                    });
                });
            });

            section_header(ui, "Cursor");

            let cursor_styles = [CursorStyle::Block, CursorStyle::Beam, CursorStyle::Underline];
            form_row(ui, |ui| {
                labeled_dropdown(ui, "Cursor Style", "cursor_style", &mut self.cursor_style, &cursor_styles);
            });

            form_row(ui, |ui| {
                labeled_toggle(ui, "Cursor blink", &mut self.cursor_blink);
            });

            section_header(ui, "Bell");

            form_row(ui, |ui| {
                labeled_toggle(ui, "Enable bell sound", &mut self.bell_enabled);
            });

            form_row(ui, |ui| {
                labeled_toggle(ui, "Visual bell (flash screen)", &mut self.bell_visual);
            });

            section_header(ui, "Text Behavior");

            form_row(ui, |ui| {
                labeled_toggle(ui, "Word wrap", &mut self.word_wrap);
            });

            form_row(ui, |ui| {
                labeled_toggle(ui, "Copy on select", &mut self.copy_on_select);
            });

            form_row(ui, |ui| {
                labeled_toggle(ui, "Paste on right-click", &mut self.paste_on_right_click);
            });
        });
    }

    fn render_ssh(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("SSH Settings").color(colors::TEXT_PRIMARY));
        ui.add_space(spacing::LG);

        card(ui, |ui| {
            section_header(ui, "Connection Defaults");

            form_row(ui, |ui| {
                labeled_number(ui, "Default Port", &mut self.default_port, 1, 65535);
            });

            form_row(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Default Username").color(colors::TEXT_PRIMARY));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let input = egui::TextEdit::singleline(&mut self.default_username)
                            .text_color(colors::TEXT_PRIMARY)
                            .desired_width(200.0)
                            .margin(egui::Margin::symmetric(8.0, 6.0));
                        ui.add(input);
                    });
                });
            });

            let term_types = ["xterm-256color", "xterm", "vt100", "linux", "screen"];
            form_row(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Default Terminal Type").color(colors::TEXT_PRIMARY));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        egui::ComboBox::from_id_source("default_term_type")
                            .selected_text(RichText::new(&self.default_terminal_type).color(colors::TEXT_PRIMARY))
                            .width(200.0)
                            .show_ui(ui, |ui: &mut egui::Ui| {
                                for term in term_types {
                                    if ui.selectable_label(self.default_terminal_type == term, term).clicked() {
                                        self.default_terminal_type = term.to_string();
                                        self.has_unsaved_changes = true;
                                    }
                                }
                            });
                    });
                });
            });

            section_header(ui, "Timeouts");

            form_row(ui, |ui| {
                labeled_number(ui, "Connection Timeout (seconds)", &mut self.connection_timeout, 5, 300);
            });

            form_row(ui, |ui| {
                labeled_number(ui, "Keep-alive Interval (seconds)", &mut self.keepalive_interval, 0, 600);
            });

            section_header(ui, "Security");

            form_row(ui, |ui| {
                labeled_toggle(ui, "Strict host key checking", &mut self.strict_host_key_checking);
            });

            section_header(ui, "Performance");

            form_row(ui, |ui| {
                labeled_toggle(ui, "Enable compression", &mut self.compression_enabled);
            });

            section_header(ui, "Key Preferences");

            form_row(ui, |ui| {
                labeled_toggle(ui, "Prefer Ed25519 keys over RSA", &mut self.prefer_ed25519);
            });
        });
    }

    fn render_security(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("Security Settings").color(colors::TEXT_PRIMARY));
        ui.add_space(spacing::LG);

        card(ui, |ui| {
            section_header(ui, "Master Password");

            form_row(ui, |ui| {
                labeled_toggle(ui, "Require master password", &mut self.use_master_password);
            });

            if self.use_master_password {
                form_row(ui, |ui| {
                    if secondary_button(ui, "Change Master Password").clicked() {
                        // TODO: Show password change dialog
                    }
                });
            }

            section_header(ui, "Auto-Lock");

            form_row(ui, |ui| {
                labeled_toggle(ui, "Auto-lock on idle", &mut self.auto_lock_enabled);
            });

            if self.auto_lock_enabled {
                form_row(ui, |ui| {
                    labeled_number(ui, "Lock after (minutes)", &mut self.auto_lock_timeout, 1, 60);
                });
            }

            section_header(ui, "Clipboard");

            form_row(ui, |ui| {
                labeled_number(ui, "Clear clipboard after (seconds)", &mut self.clear_clipboard_after, 0, 300);
            });

            ui.label(RichText::new("Set to 0 to disable automatic clipboard clearing")
                .color(colors::TEXT_MUTED)
                .size(11.0));

            section_header(ui, "Logging");

            form_row(ui, |ui| {
                labeled_toggle(ui, "Log session data (for debugging)", &mut self.log_session_data);
            });

            ui.label(RichText::new("Warning: Session logs may contain sensitive data")
                .color(colors::WARNING)
                .size(11.0));
        });
    }

    fn render_key_management(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("SSH Key Management").color(colors::TEXT_PRIMARY));
        ui.add_space(spacing::LG);

        card(ui, |ui| {
            section_header(ui, "Default Key");

            form_row(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Default Private Key").color(colors::TEXT_PRIMARY));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if secondary_button(ui, "Browse...").clicked() {
                            // TODO: File picker
                        }
                        let input = egui::TextEdit::singleline(&mut self.default_key_path)
                            .text_color(colors::TEXT_PRIMARY)
                            .desired_width(200.0)
                            .margin(egui::Margin::symmetric(8.0, 6.0));
                        ui.add(input);
                    });
                });
            });

            section_header(ui, "SSH Agent");

            form_row(ui, |ui| {
                labeled_toggle(ui, "Use SSH Agent", &mut self.agent_enabled);
            });

            form_row(ui, |ui| {
                labeled_toggle(ui, "Auto-add keys to agent", &mut self.auto_add_keys);
            });

            section_header(ui, "Key Actions");

            ui.horizontal(|ui| {
                if primary_button(ui, "Generate New Key").clicked() {
                    // TODO: Key generation wizard
                }

                ui.add_space(spacing::SM);

                if secondary_button(ui, "Import Key").clicked() {
                    // TODO: Import key dialog
                }

                ui.add_space(spacing::SM);

                if secondary_button(ui, "Manage Keys").clicked() {
                    // TODO: Key management dialog
                }
            });
        });
    }

    fn render_backup(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("Backup & Sync").color(colors::TEXT_PRIMARY));
        ui.add_space(spacing::LG);

        card(ui, |ui| {
            section_header(ui, "Automatic Backup");

            form_row(ui, |ui| {
                labeled_toggle(ui, "Enable automatic backups", &mut self.auto_backup_enabled);
            });

            if self.auto_backup_enabled {
                form_row(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Backup Location").color(colors::TEXT_PRIMARY));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if secondary_button(ui, "Browse...").clicked() {
                                // TODO: Folder picker
                            }
                            let input = egui::TextEdit::singleline(&mut self.backup_location)
                                .text_color(colors::TEXT_PRIMARY)
                                .desired_width(200.0)
                                .margin(egui::Margin::symmetric(8.0, 6.0));
                            ui.add(input);
                        });
                    });
                });

                form_row(ui, |ui| {
                    labeled_number(ui, "Backup every (days)", &mut self.backup_interval_days, 1, 30);
                });

                form_row(ui, |ui| {
                    labeled_toggle(ui, "Encrypt backups", &mut self.encrypt_backups);
                });
            }

            section_header(ui, "Manual Backup");

            ui.horizontal(|ui| {
                if primary_button(ui, "Export Connections").clicked() {
                    // TODO: Export dialog
                }

                ui.add_space(spacing::SM);

                if secondary_button(ui, "Import Connections").clicked() {
                    // TODO: Import dialog
                }
            });

            ui.add_space(spacing::MD);

            ui.label(RichText::new("Backups include: connections, settings, SSH keys (optional)")
                .color(colors::TEXT_MUTED)
                .size(11.0));

            section_header(ui, "Reset");

            if danger_button(ui, "Reset All Settings").clicked() {
                // TODO: Confirmation dialog
            }

            ui.label(RichText::new("This will reset all settings to their default values. Connections will not be affected.")
                .color(colors::TEXT_MUTED)
                .size(11.0));
        });
    }
}

/// Actions from the settings screen
pub enum SettingsAction {
    Save,
    Reset,
}
