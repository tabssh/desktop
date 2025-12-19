//! Main application state and logic for TabSSH

use crate::ssh::SessionManager;
use crate::storage::Database;
use crate::ui::{Tab, TabManager, TabStatus};
use crate::ui::components::{colors, spacing, icon_button, primary_button, secondary_button};
use crate::ui::screens::{
    ConnectionManagerScreen, ConnectionManagerAction,
    ConnectionEditorScreen, ConnectionEditorAction,
    AuthType,
    SettingsScreen,
    TerminalViewScreen,
};
use eframe::egui::{self, RichText, Vec2};
use std::sync::Arc;
use tokio::runtime::Runtime;
use uuid::Uuid;

/// Current view/screen in the application
#[derive(Clone, Copy, PartialEq)]
pub enum AppView {
    Connections,
    Terminal,
    SFTP,
    Settings,
}

/// Main application state
#[allow(dead_code)]
pub struct TabSSHApp {
    /// Async runtime for SSH connections
    runtime: Arc<Runtime>,

    /// SSH session manager
    session_manager: SessionManager,

    /// Tab manager for terminal tabs
    tab_manager: TabManager,

    /// Database connection
    database: Option<Database>,

    /// Current view
    current_view: AppView,

    /// Connection manager screen
    connection_manager: ConnectionManagerScreen,

    /// Connection editor (shown in modal or separate view)
    connection_editor: Option<ConnectionEditorScreen>,

    /// Settings screen
    settings: SettingsScreen,

    /// Active terminal views (keyed by tab id)
    terminal_views: std::collections::HashMap<Uuid, TerminalViewScreen>,

    /// Show quick connect panel
    show_quick_connect: bool,

    /// Quick connect fields
    quick_connect_host: String,
    quick_connect_user: String,
    quick_connect_port: String,

    /// Sidebar collapsed state
    sidebar_collapsed: bool,

    /// Password prompt dialog state
    show_password_dialog: bool,
    password_dialog_tab_id: Option<Uuid>,
    password_dialog_password: String,
    password_dialog_key_path: String,
    password_dialog_auth_type: AuthType,
    password_dialog_use_key: bool,
}

impl TabSSHApp {
    /// Create a new TabSSH application instance
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set up custom fonts and visual style
        Self::configure_style(&cc.egui_ctx);

        // Create tokio runtime for async operations
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime"),
        );

        // Initialize database
        let database = match Database::open() {
            Ok(db) => {
                log::info!("Database opened successfully");
                Some(db)
            }
            Err(e) => {
                log::error!("Failed to open database: {}", e);
                None
            }
        };

        // Initialize session manager
        let session_manager = SessionManager::new(runtime.clone());

        // Initialize tab manager
        let tab_manager = TabManager::new();

        Self {
            runtime,
            session_manager,
            tab_manager,
            database,
            current_view: AppView::Connections,
            connection_manager: ConnectionManagerScreen::new(),
            connection_editor: None,
            settings: SettingsScreen::new(),
            terminal_views: std::collections::HashMap::new(),
            show_quick_connect: false,
            quick_connect_host: String::new(),
            quick_connect_user: String::from("root"),
            quick_connect_port: String::from("22"),
            sidebar_collapsed: false,
            show_password_dialog: false,
            password_dialog_tab_id: None,
            password_dialog_password: String::new(),
            password_dialog_key_path: String::from("~/.ssh/id_ed25519"),
            password_dialog_auth_type: AuthType::Password,
            password_dialog_use_key: false,
        }
    }

    /// Configure the visual style for the application
    fn configure_style(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        // Visuals
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = colors::BG_PRIMARY;
        visuals.panel_fill = colors::BG_PRIMARY;
        visuals.faint_bg_color = colors::BG_SECONDARY;
        visuals.extreme_bg_color = colors::BG_TERTIARY;
        visuals.window_stroke = egui::Stroke::new(1.0, colors::BORDER);
        visuals.widgets.noninteractive.bg_fill = colors::BG_SECONDARY;
        visuals.widgets.inactive.bg_fill = colors::BG_TERTIARY;
        visuals.widgets.hovered.bg_fill = colors::BG_SURFACE;
        visuals.widgets.active.bg_fill = colors::PRIMARY;
        visuals.selection.bg_fill = colors::PRIMARY;

        style.visuals = visuals;
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(12.0, 6.0);

        ctx.set_style(style);
    }

    /// Render the main sidebar navigation
    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        let sidebar_width = if self.sidebar_collapsed { 50.0 } else { 200.0 };
        ui.set_min_width(sidebar_width);
        ui.set_max_width(sidebar_width);

        // App logo/title
        ui.vertical_centered(|ui| {
            ui.add_space(spacing::LG);
            if self.sidebar_collapsed {
                ui.label(RichText::new("\u{1F4E1}").size(24.0));
            } else {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("\u{1F4E1}").size(20.0));
                    ui.label(RichText::new("TabSSH").color(colors::TEXT_PRIMARY).strong().size(18.0));
                });
            }
            ui.add_space(spacing::LG);
        });

        ui.separator();
        ui.add_space(spacing::MD);

        // Navigation items
        let nav_items = [
            ("\u{1F4C1}", "Connections", AppView::Connections),
            ("\u{1F5A5}", "Terminal", AppView::Terminal),
            ("\u{1F4C2}", "SFTP", AppView::SFTP),
            ("\u{2699}", "Settings", AppView::Settings),
        ];

        for (icon, label, view) in nav_items {
            let display_label = if self.sidebar_collapsed { "" } else { label };
            let is_selected = self.current_view == view;

            let button_text = if self.sidebar_collapsed {
                RichText::new(icon)
                    .color(if is_selected { colors::TEXT_PRIMARY } else { colors::TEXT_SECONDARY })
                    .size(18.0)
            } else {
                RichText::new(format!("{}  {}", icon, display_label))
                    .color(if is_selected { colors::TEXT_PRIMARY } else { colors::TEXT_SECONDARY })
                    .size(14.0)
            };

            let bg = if is_selected { colors::BG_TERTIARY } else { egui::Color32::TRANSPARENT };

            let button = egui::Button::new(button_text)
                .fill(bg)
                .stroke(egui::Stroke::NONE)
                .rounding(egui::Rounding::same(6.0))
                .min_size(Vec2::new(ui.available_width() - spacing::SM * 2.0, 40.0));

            ui.add_space(spacing::XS);
            if ui.add(button).clicked() {
                self.current_view = view;
            }
        }

        // Spacer
        ui.add_space(ui.available_height() - 100.0);

        // Quick connect button
        ui.separator();
        ui.add_space(spacing::SM);

        if self.sidebar_collapsed {
            if icon_button(ui, "\u{26A1}", "Quick Connect").clicked() {
                self.show_quick_connect = !self.show_quick_connect;
            }
        } else {
            if primary_button(ui, "\u{26A1} Quick Connect").clicked() {
                self.show_quick_connect = !self.show_quick_connect;
            }
        }

        ui.add_space(spacing::SM);

        // Collapse toggle
        let collapse_icon = if self.sidebar_collapsed { "\u{25B6}" } else { "\u{25C0}" };
        if icon_button(ui, collapse_icon, "Toggle sidebar").clicked() {
            self.sidebar_collapsed = !self.sidebar_collapsed;
        }
    }

    /// Render the tab bar for terminal sessions
    fn render_tab_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(spacing::SM);

            let tabs = self.tab_manager.tabs().to_vec();
            let active_id = self.tab_manager.active_tab_id();

            for tab in &tabs {
                let is_active = Some(tab.id()) == active_id;

                let bg = if is_active { colors::BG_TERTIARY } else { colors::BG_SECONDARY };
                let text_color = if is_active { colors::TEXT_PRIMARY } else { colors::TEXT_SECONDARY };

                egui::Frame::none()
                    .fill(bg)
                    .rounding(egui::Rounding { nw: 6.0, ne: 6.0, sw: 0.0, se: 0.0 })
                    .inner_margin(egui::Margin::symmetric(12.0, 6.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Status indicator
                            let status_color = match tab.status() {
                                TabStatus::Connected => colors::SUCCESS,
                                TabStatus::Connecting => colors::WARNING,
                                _ => colors::TEXT_MUTED,
                            };
                            ui.label(RichText::new("\u{25CF}").color(status_color).size(8.0));

                            // Tab title (clickable)
                            let title_response = ui.add(
                                egui::Label::new(RichText::new(tab.title()).color(text_color))
                                    .sense(egui::Sense::click())
                            );
                            if title_response.clicked() {
                                self.tab_manager.set_active_tab(tab.id());
                                self.current_view = AppView::Terminal;
                            }

                            // Close button
                            if ui.add(
                                egui::Button::new(RichText::new("\u{2715}").color(colors::TEXT_MUTED).size(10.0))
                                    .frame(false)
                            ).clicked() {
                                self.tab_manager.close_tab(tab.id());
                                self.terminal_views.remove(&tab.id());
                            }
                        });
                    });

                ui.add_space(2.0);
            }

            // New tab button
            if ui.add(
                egui::Button::new(RichText::new("+").color(colors::TEXT_SECONDARY))
                    .fill(colors::BG_SECONDARY)
                    .rounding(egui::Rounding::same(4.0))
                    .min_size(Vec2::new(28.0, 28.0))
            ).clicked() {
                self.current_view = AppView::Connections;
            }
        });
    }

    /// Render the quick connect panel
    fn render_quick_connect_panel(&mut self, ctx: &egui::Context) {
        egui::Window::new("Quick Connect")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(spacing::SM);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("User").color(colors::TEXT_SECONDARY));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.quick_connect_user)
                            .hint_text("root")
                            .text_color(colors::TEXT_PRIMARY)
                            .desired_width(100.0)
                    );

                    ui.label(RichText::new("@").color(colors::TEXT_MUTED));

                    ui.label(RichText::new("Host").color(colors::TEXT_SECONDARY));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.quick_connect_host)
                            .hint_text("example.com")
                            .text_color(colors::TEXT_PRIMARY)
                            .desired_width(150.0)
                    );

                    ui.label(RichText::new(":").color(colors::TEXT_MUTED));

                    ui.label(RichText::new("Port").color(colors::TEXT_SECONDARY));
                    ui.add(
                        egui::TextEdit::singleline(&mut self.quick_connect_port)
                            .hint_text("22")
                            .text_color(colors::TEXT_PRIMARY)
                            .desired_width(50.0)
                    );
                });

                ui.add_space(spacing::MD);

                ui.horizontal(|ui| {
                    if ui.add(
                        egui::Button::new(RichText::new("Cancel").color(colors::TEXT_SECONDARY))
                            .fill(colors::BG_TERTIARY)
                    ).clicked() {
                        self.show_quick_connect = false;
                    }

                    ui.add_space(spacing::SM);

                    if primary_button(ui, "Connect").clicked() {
                        self.connect_quick();
                        self.show_quick_connect = false;
                    }
                });
            });
    }

    /// Handle quick connect
    fn connect_quick(&mut self) {
        if self.quick_connect_host.is_empty() {
            return;
        }

        let host = self.quick_connect_host.clone();
        let user = if self.quick_connect_user.is_empty() {
            "root".to_string()
        } else {
            self.quick_connect_user.clone()
        };
        let port: u16 = self.quick_connect_port.parse().unwrap_or(22);

        log::info!("Quick connect: {}@{}:{}", user, host, port);

        let tab = Tab::new_ssh(&host, &user, port);
        let tab_id = tab.id();
        self.tab_manager.add_tab(tab);

        let terminal = TerminalViewScreen::for_session(&host, &user, port);
        self.terminal_views.insert(tab_id, terminal);

        self.password_dialog_tab_id = Some(tab_id);
        self.password_dialog_password.clear();
        self.password_dialog_auth_type = AuthType::Password;
        self.password_dialog_use_key = false;
        self.show_password_dialog = true;

        self.current_view = AppView::Terminal;
        self.quick_connect_host.clear();
    }

    fn render_password_dialog(&mut self, ctx: &egui::Context) {
        let tab_id = match self.password_dialog_tab_id {
            Some(id) => id,
            None => return,
        };

        let (host, user) = if let Some(terminal) = self.terminal_views.get(&tab_id) {
            (terminal.session_host.clone(), terminal.session_user.clone())
        } else {
            return;
        };

        egui::Window::new("Authentication Required")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .min_width(400.0)
            .show(ctx, |ui| {
                ui.add_space(spacing::SM);

                ui.label(RichText::new(format!("Connecting to {}@{}", user, host))
                    .color(colors::TEXT_PRIMARY)
                    .size(14.0));

                ui.add_space(spacing::MD);

                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.password_dialog_use_key, false, "Password");
                    ui.radio_value(&mut self.password_dialog_use_key, true, "SSH Key");
                });

                ui.add_space(spacing::SM);

                if self.password_dialog_use_key {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Key Path:").color(colors::TEXT_SECONDARY));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.password_dialog_key_path)
                                .hint_text("~/.ssh/id_ed25519")
                                .text_color(colors::TEXT_PRIMARY)
                                .desired_width(250.0)
                        );
                    });
                } else {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Password:").color(colors::TEXT_SECONDARY));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.password_dialog_password)
                                .password(true)
                                .hint_text("Enter password")
                                .text_color(colors::TEXT_PRIMARY)
                                .desired_width(250.0)
                        );
                    });
                }

                ui.add_space(spacing::LG);

                ui.horizontal(|ui| {
                    if secondary_button(ui, "Cancel").clicked() {
                        self.show_password_dialog = false;
                        self.password_dialog_tab_id = None;
                        if let Some(id) = self.password_dialog_tab_id {
                            self.tab_manager.close_tab(id);
                            self.terminal_views.remove(&id);
                        }
                    }

                    ui.add_space(spacing::SM);

                    let can_connect = if self.password_dialog_use_key {
                        !self.password_dialog_key_path.is_empty()
                    } else {
                        !self.password_dialog_password.is_empty()
                    };

                    if primary_button(ui, "Connect").clicked() && can_connect {
                        self.initiate_connection(tab_id);
                        self.show_password_dialog = false;
                    }
                });
            });
    }

    fn initiate_connection(&mut self, tab_id: Uuid) {
        let runtime = self.runtime.clone();

        if let Some(terminal) = self.terminal_views.get_mut(&tab_id) {
            if self.password_dialog_use_key {
                let key_path = shellexpand::tilde(&self.password_dialog_key_path).to_string();
                terminal.connect_with_key(runtime, key_path, None);
            } else {
                let password = self.password_dialog_password.clone();
                terminal.connect_with_password(runtime, password);
            }
        }

        self.password_dialog_password.clear();
        self.password_dialog_tab_id = None;
    }

    /// Render the connection editor modal
    fn render_connection_editor(&mut self, ctx: &egui::Context) {
        if self.connection_editor.is_none() {
            return;
        }

        egui::Window::new("")
            .title_bar(false)
            .collapsible(false)
            .resizable(true)
            .min_size([600.0, 400.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                if let Some(editor) = &mut self.connection_editor {
                    if let Some(action) = editor.render(ui) {
                        match action {
                            ConnectionEditorAction::Save(profile) => {
                                // Add to connection manager
                                self.connection_manager.connections.push(profile);
                                self.connection_editor = None;
                            }
                            ConnectionEditorAction::Cancel => {
                                self.connection_editor = None;
                            }
                        }
                    }
                }
            });
    }

    /// Handle keyboard shortcuts
    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        // Ctrl+T - Quick connect
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::T)) {
            self.show_quick_connect = true;
        }

        // Ctrl+W - Close current tab
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::W)) {
            if let Some(id) = self.tab_manager.active_tab_id() {
                self.tab_manager.close_tab(id);
                self.terminal_views.remove(&id);
            }
        }

        // Ctrl+Tab - Next tab
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Tab)) {
            if ctx.input(|i| i.modifiers.shift) {
                self.tab_manager.previous_tab();
            } else {
                self.tab_manager.next_tab();
            }
        }

        // Ctrl+1-9 - Switch to tab by number
        for i in 1..=9 {
            let key = match i {
                1 => egui::Key::Num1,
                2 => egui::Key::Num2,
                3 => egui::Key::Num3,
                4 => egui::Key::Num4,
                5 => egui::Key::Num5,
                6 => egui::Key::Num6,
                7 => egui::Key::Num7,
                8 => egui::Key::Num8,
                9 => egui::Key::Num9,
                _ => continue,
            };
            if ctx.input(|inp| inp.modifiers.ctrl && inp.key_pressed(key)) {
                self.tab_manager.set_active_tab_by_index(i - 1);
                self.current_view = AppView::Terminal;
            }
        }

        // Escape - Close modals
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_quick_connect = false;
            self.show_password_dialog = false;
            self.connection_editor = None;
        }
    }
}

impl eframe::App for TabSSHApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts
        self.handle_shortcuts(ctx);

        // Sidebar
        egui::SidePanel::left("sidebar")
            .exact_width(if self.sidebar_collapsed { 50.0 } else { 200.0 })
            .frame(egui::Frame::none().fill(colors::BG_SECONDARY).inner_margin(egui::Margin::same(spacing::SM)))
            .show(ctx, |ui| {
                self.render_sidebar(ui);
            });

        // Tab bar (when we have tabs)
        if !self.tab_manager.tabs().is_empty() {
            egui::TopBottomPanel::top("tab_bar")
                .exact_height(36.0)
                .frame(egui::Frame::none().fill(colors::BG_PRIMARY))
                .show(ctx, |ui| {
                    self.render_tab_bar(ui);
                });
        }

        // Main content area
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(colors::BG_PRIMARY).inner_margin(egui::Margin::same(spacing::LG)))
            .show(ctx, |ui| {
                match self.current_view {
                    AppView::Connections => {
                        if let Some(action) = self.connection_manager.render(ui) {
                            match action {
                                ConnectionManagerAction::Connect(profile) => {
                                    let tab = Tab::new_ssh(&profile.host, &profile.username, profile.port);
                                    let tab_id = tab.id();
                                    self.tab_manager.add_tab(tab);

                                    let terminal = TerminalViewScreen::for_session(
                                        &profile.host,
                                        &profile.username,
                                        profile.port
                                    );
                                    self.terminal_views.insert(tab_id, terminal);

                                    self.password_dialog_tab_id = Some(tab_id);
                                    self.password_dialog_password.clear();
                                    self.password_dialog_auth_type = profile.auth_type.clone();
                                    self.password_dialog_use_key = profile.auth_type == AuthType::PublicKey;
                                    self.show_password_dialog = true;

                                    self.current_view = AppView::Terminal;
                                }
                                ConnectionManagerAction::Edit(id) => {
                                    if let Some(profile) = self.connection_manager.connections.iter().find(|c| c.id == id) {
                                        self.connection_editor = Some(ConnectionEditorScreen::from_profile(profile));
                                    }
                                }
                                ConnectionManagerAction::Delete(id) => {
                                    self.connection_manager.connections.retain(|c| c.id != id);
                                }
                                ConnectionManagerAction::NewConnection => {
                                    self.connection_editor = Some(ConnectionEditorScreen::new());
                                }
                            }
                        }
                    }
                    AppView::Terminal => {
                        if let Some(active_tab) = self.tab_manager.active_tab() {
                            if let Some(terminal) = self.terminal_views.get_mut(&active_tab.id()) {
                                terminal.render_with_status(ui);
                            } else {
                                // Create terminal view if it doesn't exist
                                let mut terminal = TerminalViewScreen::for_session(
                                    active_tab.host(),
                                    active_tab.user(),
                                    active_tab.port()
                                );
                                terminal.render_with_status(ui);
                                self.terminal_views.insert(active_tab.id(), terminal);
                            }
                        } else {
                            // No active terminal - show placeholder
                            ui.centered_and_justified(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(100.0);
                                    ui.label(RichText::new("\u{1F5A5}").size(64.0).color(colors::TEXT_MUTED));
                                    ui.add_space(spacing::LG);
                                    ui.label(RichText::new("No Active Terminal").size(20.0).color(colors::TEXT_PRIMARY));
                                    ui.add_space(spacing::SM);
                                    ui.label(RichText::new("Open a connection to start a terminal session").color(colors::TEXT_SECONDARY));
                                    ui.add_space(spacing::LG);
                                    if primary_button(ui, "Open Connection Manager").clicked() {
                                        self.current_view = AppView::Connections;
                                    }
                                });
                            });
                        }
                    }
                    AppView::SFTP => {
                        // SFTP browser placeholder
                        ui.centered_and_justified(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(100.0);
                                ui.label(RichText::new("\u{1F4C2}").size(64.0).color(colors::TEXT_MUTED));
                                ui.add_space(spacing::LG);
                                ui.label(RichText::new("SFTP File Browser").size(20.0).color(colors::TEXT_PRIMARY));
                                ui.add_space(spacing::SM);
                                ui.label(RichText::new("Coming soon - integrated file transfer").color(colors::TEXT_SECONDARY));
                            });
                        });
                    }
                    AppView::Settings => {
                        if let Some(_action) = self.settings.render(ui) {
                            // Handle settings actions
                        }
                    }
                }
            });

        // Modals
        if self.show_quick_connect {
            self.render_quick_connect_panel(ctx);
        }

        if self.show_password_dialog {
            self.render_password_dialog(ctx);
        }

        self.render_connection_editor(ctx);

        // Request repaint for animations
        ctx.request_repaint();
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // TODO: Save application state
    }
}
