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
        let fonts = egui::FontDefinitions::default();
        cc.egui_ctx.set_fonts(fonts);

        let state = AppState::new().unwrap_or_else(|e| {
            log::error!("Failed to initialize app state: {}", e);
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
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts
        if let Some(action) = KeyboardHandler::handle_shortcuts(ctx) {
            match action {
                KeyboardAction::NewTab => {
                    log::info!("New tab");
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
                    log::info!("New connection");
                }
                KeyboardAction::OpenSettings => {
                    log::info!("Open settings");
                }
                KeyboardAction::Quit => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                _ => {}
            }
        }

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            let _action = Toolbar::render(ui);
        });

        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            let _action = self.tab_bar.render(ui);
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            self.status_bar.render(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.state.tabs.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.heading("Welcome to TabSSH Desktop");
                    ui.label("Press Ctrl+N to create a new connection");
                });
            } else {
                ui.label("Tab content here");
            }
        });

        self.state.notification_manager.render(ctx);
    }
}
