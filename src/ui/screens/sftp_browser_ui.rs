//! SFTP browser UI screen

use crate::sftp::{SftpBrowser, SortColumn};
use egui::{Context, Ui};
use std::path::PathBuf;

pub struct SftpBrowserScreen {
    browser: SftpBrowser,
    current_path_input: String,
    selected_local_path: Option<PathBuf>,
    transfer_progress: Vec<TransferProgress>,
}

#[derive(Debug, Clone)]
struct TransferProgress {
    filename: String,
    progress: f32,
    status: String,
}

impl SftpBrowserScreen {
    pub fn new() -> Self {
        Self {
            browser: SftpBrowser::new(),
            current_path_input: "/".to_string(),
            selected_local_path: None,
            transfer_progress: Vec::new(),
        }
    }

    pub fn render(&mut self, _ctx: &Context, ui: &mut Ui) {
        ui.heading("SFTP Browser");

        // Path navigation bar
        ui.horizontal(|ui| {
            if ui.button("⬆ Up").clicked() {
                if let Some(path) = self.browser.go_up() {
                    self.current_path_input = path.to_string_lossy().into_owned();
                }
            }

            if ui.button("🏠 Home").clicked() {
                let path = self.browser.go_home();
                self.current_path_input = path.to_string_lossy().into_owned();
            }

            if ui.button("🔄 Refresh").clicked() {
                // Trigger directory refresh
            }

            ui.separator();

            ui.label("Path:");
            if ui.text_edit_singleline(&mut self.current_path_input).lost_focus() {
                self.browser.change_directory(PathBuf::from(&self.current_path_input));
            }
        });

        ui.separator();

        // File list header
        ui.horizontal(|ui| {
            if ui.button("Name").clicked() {
                self.browser.set_sort(SortColumn::Name, true);
            }
            ui.separator();
            if ui.button("Size").clicked() {
                self.browser.set_sort(SortColumn::Size, true);
            }
            ui.separator();
            if ui.button("Modified").clicked() {
                self.browser.set_sort(SortColumn::Modified, true);
            }
            ui.separator();
            if ui.button("Type").clicked() {
                self.browser.set_sort(SortColumn::Type, true);
            }
        });

        ui.separator();

        // File list
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Collect entries to avoid borrow checker issues
            let entries: Vec<_> = self.browser.entries().iter().cloned().collect();
            let selected_indices: Vec<_> = self.browser.selected().iter().cloned().collect();

            for (idx, entry) in entries.iter().enumerate() {
                let is_selected = selected_indices.contains(&idx);

                ui.horizontal(|ui| {
                    let icon = match entry.file_type {
                        crate::sftp::FileType::Directory => "📁",
                        crate::sftp::FileType::File => "📄",
                        crate::sftp::FileType::Symlink => "🔗",
                        crate::sftp::FileType::Other => "❓",
                    };

                    let response = ui.selectable_label(is_selected, format!("{} {}", icon, entry.name));

                    if response.clicked() {
                        self.browser.toggle_selection(idx);
                    }

                    if response.double_clicked() {
                        if matches!(entry.file_type, crate::sftp::FileType::Directory) {
                            let new_path = self.browser.get_full_path(entry);
                            self.browser.change_directory(new_path.clone());
                            self.current_path_input = new_path.to_string_lossy().into_owned();
                        }
                    }

                    ui.label(format!("{} bytes", entry.size));

                    if let Some(modified) = &entry.modified {
                        ui.label(format!("{}", modified.format("%Y-%m-%d %H:%M")));
                    }
                });
            }
        });

        ui.separator();

        // Actions bar
        ui.horizontal(|ui| {
            if ui.button("📥 Download").clicked() {
                let selected = self.browser.get_selected_entries();
                for entry in selected {
                    log::info!("Download: {}", entry.name);
                }
            }

            if ui.button("📤 Upload").clicked() {
                log::info!("Upload clicked");
            }

            if ui.button("🗑 Delete").clicked() {
                let selected = self.browser.get_selected_entries();
                for entry in selected {
                    log::info!("Delete: {}", entry.name);
                }
            }

            if ui.button("📝 Rename").clicked() {
                log::info!("Rename clicked");
            }

            if ui.button("📁 New Folder").clicked() {
                log::info!("New folder clicked");
            }
        });

        // Transfer progress
        if !self.transfer_progress.is_empty() {
            ui.separator();
            ui.heading("Transfers");

            for transfer in &self.transfer_progress {
                ui.horizontal(|ui| {
                    ui.label(&transfer.filename);
                    ui.add(egui::ProgressBar::new(transfer.progress).text(&transfer.status));
                });
            }
        }
    }
}

impl Default for SftpBrowserScreen {
    fn default() -> Self {
        Self::new()
    }
}
