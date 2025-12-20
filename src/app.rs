//! Main application structure

use crate::ui::app_state::AppState;
use crate::ui::keyboard::{KeyboardHandler, KeyboardAction};
use crate::ui::components::{TabBar, Toolbar, StatusBar};
use egui::Context;

pub struct TabSshApp {
    state: AppState,
    tab_bar: TabBar,
    toolbar: Toolbar,
    status_bar: StatusBar,
}

impl TabSshApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure fonts
        let mut fonts = egui::FontDefinitions::default();
        // Could load custom fonts here
        cc.egui_ctx.set_fonts(fonts);
        
        let state = AppState::new().unwrap_or_else(|e| {
            eprintln!("Failedtoinitializeappstate:{}",e);
            std::process::exit(1);
        });
        
        Self {
            state,
            tab_bar: TabBar::new(),
            toolbar: Toolbar,
            status_bar: StatusBar::new(),
        }
    }
}

impl eframe::App for TabSshApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts
        if let Some(action) = KeyboardHandler::handle_shortcuts(ctx) {
            match action {
                KeyboardAction::NewTab => {
                    log::info!("Newtab");
                }
                KeyboardAction::CloseTab => {
                    if self.state.active_tab < self.state.tabs.len() {
                        self.state.close_tab(self.state.active_tab);
                    }
                }
                KeyboardAction::NextTab => {
                    self.state.next_tab();
                }
                KeyboardAction::PreviousTab => {
                    self.state.previous_tab();
                }
                KeyboardAction::NewConnection => {
                    log::info!("Newconnection");
                }
                KeyboardAction::OpenSettings => {
                    log::info!("Opensettings");
                }
                KeyboardAction::Quit => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                _ => {}
            }
        }
        
        // Top panel - Toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            let action = Toolbar::render(ui);
            // Handle toolbar actions
        });
        
        // Top panel - Tabs
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            if let Some(action) = self.tab_bar.render(ui) {
                // Handle tab actions
            }
        });
        
        // Bottom panel - Status bar
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            self.status_bar.render(ui);
        });
        
        // Central panel - Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.state.tabs.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.heading("Welcome to TabSSH Desktop");
                    ui.label("Press Ctrl+N to create a new connection");
                });
            } else {
                // Render active tab content
                ui.label("Tab content here");
            }
        });
        
        // Render notifications
        self.state.notification_manager.render(ctx);
    }
}
