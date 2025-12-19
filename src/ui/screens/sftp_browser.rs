//! SFTP file browser screen with dual-pane view

#![allow(dead_code)]

use crate::sftp::{FileEntry, FileType, TransferDirection, TransferState, TransferTask, format_file_size};
use crate::ui::components::{colors, spacing};
use eframe::egui::{self, RichText};
use std::path::PathBuf;
use uuid::Uuid;

/// Actions emitted by the SFTP browser
#[derive(Debug, Clone)]
pub enum SftpBrowserAction {
    NavigateLocal(PathBuf),
    NavigateRemote(String),
    Download(String),
    Upload(PathBuf),
    CreateRemoteDir(String),
    CreateLocalDir(PathBuf),
    DeleteRemote(String),
    DeleteLocal(PathBuf),
    Rename(String, String),
    Refresh,
    Close,
}

/// Sorting options for file lists
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortColumn {
    Name,
    Size,
    Modified,
    Type,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// SFTP file browser screen
pub struct SftpBrowserScreen {
    connection_id: Uuid,
    connection_name: String,

    local_path: PathBuf,
    local_entries: Vec<FileEntry>,
    local_selected: Option<usize>,
    local_sort: (SortColumn, SortOrder),
    local_loading: bool,

    remote_path: String,
    remote_entries: Vec<FileEntry>,
    remote_selected: Option<usize>,
    remote_sort: (SortColumn, SortOrder),
    remote_loading: bool,

    transfers: Vec<TransferTask>,
    show_hidden: bool,
    show_transfers: bool,

    new_dir_name: String,
    show_new_dir_dialog: bool,
    new_dir_target: NewDirTarget,

    rename_old_name: String,
    rename_new_name: String,
    show_rename_dialog: bool,

    error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NewDirTarget {
    Local,
    Remote,
}

impl SftpBrowserScreen {
    pub fn new(connection_id: Uuid, connection_name: String) -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));

        Self {
            connection_id,
            connection_name,
            local_path: home,
            local_entries: Vec::new(),
            local_selected: None,
            local_sort: (SortColumn::Name, SortOrder::Ascending),
            local_loading: false,
            remote_path: "~".to_string(),
            remote_entries: Vec::new(),
            remote_selected: None,
            remote_sort: (SortColumn::Name, SortOrder::Ascending),
            remote_loading: false,
            transfers: Vec::new(),
            show_hidden: false,
            show_transfers: false,
            new_dir_name: String::new(),
            show_new_dir_dialog: false,
            new_dir_target: NewDirTarget::Remote,
            rename_old_name: String::new(),
            rename_new_name: String::new(),
            show_rename_dialog: false,
            error_message: None,
        }
    }

    pub fn set_local_entries(&mut self, entries: Vec<FileEntry>) {
        self.local_entries = self.sort_entries(entries, self.local_sort, self.show_hidden);
        self.local_loading = false;
        self.local_selected = None;
    }

    pub fn set_remote_entries(&mut self, entries: Vec<FileEntry>) {
        self.remote_entries = self.sort_entries(entries, self.remote_sort, self.show_hidden);
        self.remote_loading = false;
        self.remote_selected = None;
    }

    pub fn set_local_path(&mut self, path: PathBuf) {
        self.local_path = path;
    }

    pub fn set_remote_path(&mut self, path: String) {
        self.remote_path = path;
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error_message = error;
    }

    pub fn add_transfer(&mut self, task: TransferTask) {
        self.transfers.push(task);
        self.show_transfers = true;
    }

    pub fn update_transfer_progress(&mut self, id: Uuid, transferred: u64) {
        if let Some(task) = self.transfers.iter_mut().find(|t| t.id == id) {
            task.transferred_bytes = transferred;
            task.state = TransferState::InProgress;
        }
    }

    pub fn complete_transfer(&mut self, id: Uuid, success: bool, error: Option<String>) {
        if let Some(task) = self.transfers.iter_mut().find(|t| t.id == id) {
            task.state = if success {
                TransferState::Completed
            } else {
                TransferState::Failed(error.unwrap_or_else(|| "Unknown error".to_string()))
            };
        }
    }

    fn sort_entries(
        &self,
        mut entries: Vec<FileEntry>,
        sort: (SortColumn, SortOrder),
        filter_hidden: bool,
    ) -> Vec<FileEntry> {
        entries.sort_by(|a, b| {
            let type_order = |t: &FileType| match t {
                FileType::Directory => 0,
                FileType::Symlink => 1,
                FileType::File => 2,
                FileType::Other => 3,
            };

            let type_cmp = type_order(&a.file_type).cmp(&type_order(&b.file_type));
            if type_cmp != std::cmp::Ordering::Equal {
                return type_cmp;
            }

            let cmp = match sort.0 {
                SortColumn::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortColumn::Size => a.size.cmp(&b.size),
                SortColumn::Modified => a.modified.cmp(&b.modified),
                SortColumn::Type => type_cmp,
            };

            match sort.1 {
                SortOrder::Ascending => cmp,
                SortOrder::Descending => cmp.reverse(),
            }
        });

        if filter_hidden {
            entries.retain(|e| !e.name.starts_with('.'));
        }

        entries
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<SftpBrowserAction> {
        let mut action: Option<SftpBrowserAction> = None;

        let toolbar_action = self.show_toolbar(ui);
        if action.is_none() { action = toolbar_action; }

        if let Some(ref error) = self.error_message.clone() {
            ui.add_space(spacing::SM);
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Error: {}", error)).color(colors::ERROR));
                if ui.small_button("Dismiss").clicked() {
                    self.error_message = None;
                }
            });
        }

        ui.add_space(spacing::SM);

        let available_height = if self.show_transfers {
            ui.available_height() - 150.0
        } else {
            ui.available_height()
        };

        let panel_width = (ui.available_width() - spacing::MD) / 2.0;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(panel_width);
                ui.set_height(available_height);
            });

            ui.add_space(spacing::MD);

            ui.vertical(|ui| {
                ui.set_width(panel_width);
                ui.set_height(available_height);
            });
        });

        let local_action = self.show_local_panel_content(ui, panel_width, available_height);
        if action.is_none() { action = local_action; }

        let remote_action = self.show_remote_panel_content(ui, panel_width, available_height);
        if action.is_none() { action = remote_action; }

        if self.show_transfers {
            ui.add_space(spacing::MD);
            self.show_transfers_panel(ui);
        }

        if self.show_new_dir_dialog {
            let dialog_action = self.show_new_dir_dialog_window(ui);
            if action.is_none() { action = dialog_action; }
        }

        if self.show_rename_dialog {
            let rename_action = self.show_rename_dialog_window(ui);
            if action.is_none() { action = rename_action; }
        }

        action
    }

    fn show_toolbar(&mut self, ui: &mut egui::Ui) -> Option<SftpBrowserAction> {
        let mut action = None;

        ui.horizontal(|ui| {
            ui.label(RichText::new(&self.connection_name).strong());

            ui.separator();

            if ui.button("Refresh").clicked() {
                action = Some(SftpBrowserAction::Refresh);
            }

            ui.separator();

            ui.checkbox(&mut self.show_hidden, "Show hidden");

            ui.separator();

            let transfers_label = if self.transfers.is_empty() {
                "Transfers".to_string()
            } else {
                let active = self.transfers.iter()
                    .filter(|t| matches!(t.state, TransferState::InProgress | TransferState::Pending))
                    .count();
                format!("Transfers ({})", active)
            };
            ui.checkbox(&mut self.show_transfers, &transfers_label);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Close").clicked() {
                    action = Some(SftpBrowserAction::Close);
                }
            });
        });

        action
    }

    fn show_local_panel_content(&mut self, ui: &mut egui::Ui, _width: f32, _height: f32) -> Option<SftpBrowserAction> {
        let mut action = None;

        egui::Frame::group(ui.style())
            .fill(colors::BG_SECONDARY)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Local").strong());

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("+").on_hover_text("New folder").clicked() {
                                self.new_dir_target = NewDirTarget::Local;
                                self.new_dir_name.clear();
                                self.show_new_dir_dialog = true;
                            }

                            if ui.small_button("^").on_hover_text("Parent directory").clicked() {
                                if let Some(parent) = self.local_path.parent() {
                                    action = Some(SftpBrowserAction::NavigateLocal(parent.to_path_buf()));
                                }
                            }

                            if ui.small_button("~").on_hover_text("Home directory").clicked() {
                                if let Some(home) = dirs::home_dir() {
                                    action = Some(SftpBrowserAction::NavigateLocal(home));
                                }
                            }
                        });
                    });

                    ui.label(
                        RichText::new(self.local_path.to_string_lossy().to_string())
                            .small()
                            .color(colors::TEXT_MUTED)
                    );

                    ui.add_space(spacing::XS);
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            if self.local_loading {
                                ui.spinner();
                            } else if self.local_entries.is_empty() {
                                ui.label(RichText::new("Empty directory").color(colors::TEXT_MUTED));
                            } else {
                                for (idx, entry) in self.local_entries.iter().enumerate() {
                                    let selected = self.local_selected == Some(idx);
                                    let response = self.show_file_entry(
                                        ui,
                                        &entry.name,
                                        &entry.file_type,
                                        entry.size,
                                        selected,
                                    );

                                    if response.clicked() {
                                        self.local_selected = Some(idx);
                                    }

                                    if response.double_clicked() {
                                        match entry.file_type {
                                            FileType::Directory => {
                                                let new_path = self.local_path.join(&entry.name);
                                                action = Some(SftpBrowserAction::NavigateLocal(new_path));
                                            }
                                            FileType::File => {
                                                let file_path = self.local_path.join(&entry.name);
                                                action = Some(SftpBrowserAction::Upload(file_path));
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        });
                });
            });

        action
    }

    fn show_remote_panel_content(&mut self, ui: &mut egui::Ui, _width: f32, _height: f32) -> Option<SftpBrowserAction> {
        let mut action = None;

        egui::Frame::group(ui.style())
            .fill(colors::BG_SECONDARY)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Remote").strong());

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("+").on_hover_text("New folder").clicked() {
                                self.new_dir_target = NewDirTarget::Remote;
                                self.new_dir_name.clear();
                                self.show_new_dir_dialog = true;
                            }

                            if ui.small_button("^").on_hover_text("Parent directory").clicked() {
                                action = Some(SftpBrowserAction::NavigateRemote("..".to_string()));
                            }

                            if ui.small_button("~").on_hover_text("Home directory").clicked() {
                                action = Some(SftpBrowserAction::NavigateRemote("~".to_string()));
                            }
                        });
                    });

                    ui.label(
                        RichText::new(&self.remote_path)
                            .small()
                            .color(colors::TEXT_MUTED)
                    );

                    ui.add_space(spacing::XS);
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            if self.remote_loading {
                                ui.spinner();
                            } else if self.remote_entries.is_empty() {
                                ui.label(RichText::new("Empty directory").color(colors::TEXT_MUTED));
                            } else {
                                for (idx, entry) in self.remote_entries.iter().enumerate() {
                                    let selected = self.remote_selected == Some(idx);
                                    let response = self.show_file_entry(
                                        ui,
                                        &entry.name,
                                        &entry.file_type,
                                        entry.size,
                                        selected,
                                    );

                                    if response.clicked() {
                                        self.remote_selected = Some(idx);
                                    }

                                    if response.double_clicked() {
                                        match entry.file_type {
                                            FileType::Directory => {
                                                action = Some(SftpBrowserAction::NavigateRemote(entry.name.clone()));
                                            }
                                            FileType::File => {
                                                action = Some(SftpBrowserAction::Download(entry.name.clone()));
                                            }
                                            _ => {}
                                        }
                                    }

                                    response.context_menu(|ui| {
                                        if ui.button("Download").clicked() {
                                            action = Some(SftpBrowserAction::Download(entry.name.clone()));
                                            ui.close_menu();
                                        }
                                        if ui.button("Rename").clicked() {
                                            self.rename_old_name = entry.name.clone();
                                            self.rename_new_name = entry.name.clone();
                                            self.show_rename_dialog = true;
                                            ui.close_menu();
                                        }
                                        ui.separator();
                                        if ui.button("Delete").clicked() {
                                            action = Some(SftpBrowserAction::DeleteRemote(entry.name.clone()));
                                            ui.close_menu();
                                        }
                                    });
                                }
                            }
                        });
                });
            });

        action
    }

    fn show_file_entry(
        &self,
        ui: &mut egui::Ui,
        name: &str,
        file_type: &FileType,
        size: u64,
        selected: bool,
    ) -> egui::Response {
        let icon = match file_type {
            FileType::Directory => "📁",
            FileType::File => "📄",
            FileType::Symlink => "🔗",
            FileType::Other => "❓",
        };

        let bg_color = if selected {
            colors::BG_HIGHLIGHT
        } else {
            egui::Color32::TRANSPARENT
        };

        let response = ui.horizontal(|ui| {
            egui::Frame::none()
                .fill(bg_color)
                .inner_margin(egui::Margin::symmetric(spacing::XS as f32, 2.0))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());

                    ui.label(icon);
                    ui.label(RichText::new(name).color(colors::TEXT_PRIMARY));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if matches!(file_type, FileType::File) {
                            ui.label(
                                RichText::new(format_file_size(size))
                                    .small()
                                    .color(colors::TEXT_MUTED)
                            );
                        }
                    });
                });
        });

        response.response.interact(egui::Sense::click())
    }

    fn show_transfers_panel(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .fill(colors::BG_SECONDARY)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Transfers").strong());

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("Clear completed").clicked() {
                            self.transfers.retain(|t| {
                                !matches!(t.state, TransferState::Completed | TransferState::Failed(_))
                            });
                        }
                    });
                });

                ui.separator();

                egui::ScrollArea::vertical()
                    .max_height(100.0)
                    .show(ui, |ui| {
                        if self.transfers.is_empty() {
                            ui.label(RichText::new("No active transfers").color(colors::TEXT_MUTED));
                        } else {
                            for transfer in &self.transfers {
                                ui.horizontal(|ui| {
                                    let icon = match transfer.direction {
                                        TransferDirection::Upload => "⬆",
                                        TransferDirection::Download => "⬇",
                                    };
                                    ui.label(icon);
                                    ui.label(&transfer.file_name);

                                    match &transfer.state {
                                        TransferState::Pending => {
                                            ui.label(RichText::new("Pending").color(colors::TEXT_MUTED));
                                        }
                                        TransferState::InProgress => {
                                            let progress = transfer.progress_percent();
                                            ui.add(egui::ProgressBar::new(progress / 100.0).show_percentage());
                                        }
                                        TransferState::Completed => {
                                            ui.label(RichText::new("Completed").color(colors::SUCCESS));
                                        }
                                        TransferState::Failed(error) => {
                                            ui.label(RichText::new(format!("Failed: {}", error)).color(colors::ERROR));
                                        }
                                        TransferState::Cancelled => {
                                            ui.label(RichText::new("Cancelled").color(colors::WARNING));
                                        }
                                    }
                                });
                            }
                        }
                    });
            });
    }

    fn show_new_dir_dialog_window(&mut self, ui: &mut egui::Ui) -> Option<SftpBrowserAction> {
        let mut action = None;
        let mut close_dialog = false;

        egui::Window::new("New Folder")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.new_dir_name);
                });

                ui.add_space(spacing::SM);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        close_dialog = true;
                    }

                    if ui.button("Create").clicked() && !self.new_dir_name.is_empty() {
                        match self.new_dir_target {
                            NewDirTarget::Local => {
                                let path = self.local_path.join(&self.new_dir_name);
                                action = Some(SftpBrowserAction::CreateLocalDir(path));
                            }
                            NewDirTarget::Remote => {
                                action = Some(SftpBrowserAction::CreateRemoteDir(self.new_dir_name.clone()));
                            }
                        }
                        close_dialog = true;
                    }
                });
            });

        if close_dialog {
            self.show_new_dir_dialog = false;
        }

        action
    }

    fn show_rename_dialog_window(&mut self, ui: &mut egui::Ui) -> Option<SftpBrowserAction> {
        let mut action = None;
        let mut close_dialog = false;

        egui::Window::new("Rename")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Current name:");
                    ui.label(RichText::new(&self.rename_old_name).color(colors::TEXT_MUTED));
                });

                ui.horizontal(|ui| {
                    ui.label("New name:");
                    ui.text_edit_singleline(&mut self.rename_new_name);
                });

                ui.add_space(spacing::SM);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        close_dialog = true;
                    }

                    if ui.button("Rename").clicked()
                        && !self.rename_new_name.is_empty()
                        && self.rename_new_name != self.rename_old_name
                    {
                        action = Some(SftpBrowserAction::Rename(
                            self.rename_old_name.clone(),
                            self.rename_new_name.clone(),
                        ));
                        close_dialog = true;
                    }
                });
            });

        if close_dialog {
            self.show_rename_dialog = false;
        }

        action
    }
}
