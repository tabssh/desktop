//! SFTP browser UI screen

use crate::sftp::{SftpBrowser, SortColumn};
use egui::{Context, Ui};
use std::path::PathBuf;

pub struct SftpBrowserScreen {
    browser: SftpBrowser,
    current_path_input: String,
    /// Local file/directory chosen for an upload; set by the local-side
    /// file picker but not yet read by the (unwired) upload action — see
    /// TODO.AI.md Phase 1.3.
    #[allow(dead_code)]
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
            if ui
                .text_edit_singleline(&mut self.current_path_input)
                .lost_focus()
            {
                self.browser
                    .change_directory(PathBuf::from(&self.current_path_input));
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
            let entries: Vec<_> = self.browser.entries().to_vec();
            let selected_indices: Vec<_> = self.browser.selected().to_vec();

            for (idx, entry) in entries.iter().enumerate() {
                let is_selected = selected_indices.contains(&idx);

                ui.horizontal(|ui| {
                    let icon = match entry.file_type {
                        crate::sftp::FileType::Directory => "📁",
                        crate::sftp::FileType::File => "📄",
                        crate::sftp::FileType::Symlink => "🔗",
                        crate::sftp::FileType::Other => "❓",
                    };

                    let response =
                        ui.selectable_label(is_selected, format!("{} {}", icon, entry.name));

                    if response.clicked() {
                        self.browser.toggle_selection(idx);
                    }

                    if response.double_clicked()
                        && matches!(entry.file_type, crate::sftp::FileType::Directory)
                    {
                        let new_path = self.browser.get_full_path(entry);
                        self.browser.change_directory(new_path.clone());
                        self.current_path_input = new_path.to_string_lossy().into_owned();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sftp::FileType;
    use chrono::Utc;

    fn make_entry(
        name: &str,
        file_type: FileType,
        size: u64,
        has_modified: bool,
    ) -> crate::sftp::FileEntry {
        crate::sftp::FileEntry {
            name: name.to_string(),
            path: PathBuf::from(name),
            file_type,
            size,
            permissions: 0o644,
            modified: if has_modified { Some(Utc::now()) } else { None },
            owner: String::new(),
            group: String::new(),
        }
    }

    /// Render the screen inside a headless egui context/frame and return
    /// nothing; panics propagate as normal test failures.
    fn render_smoke(screen: &mut SftpBrowserScreen) {
        let ctx = egui::Context::default();
        let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
            screen.render(&ctx, ui);
        });
    }

    #[test]
    fn test_new_has_expected_defaults() {
        let screen = SftpBrowserScreen::new();
        assert_eq!(screen.current_path_input, "/");
        assert!(screen.selected_local_path.is_none());
        assert!(screen.transfer_progress.is_empty());
    }

    #[test]
    fn test_default_matches_new() {
        let screen = SftpBrowserScreen::default();
        assert_eq!(screen.current_path_input, "/");
        assert!(screen.transfer_progress.is_empty());
    }

    #[test]
    fn test_render_smoke_empty_directory() {
        let mut screen = SftpBrowserScreen::new();
        render_smoke(&mut screen);
    }

    #[test]
    fn test_render_smoke_with_mixed_entry_types() {
        let mut screen = SftpBrowserScreen::new();
        screen.browser.set_entries(vec![
            make_entry("dir1", FileType::Directory, 0, true),
            make_entry("file1.txt", FileType::File, 1234, true),
            make_entry("link1", FileType::Symlink, 0, false),
            make_entry("other1", FileType::Other, 0, true),
        ]);
        screen.browser.toggle_selection(1);
        render_smoke(&mut screen);
    }

    #[test]
    fn test_render_smoke_with_transfer_progress() {
        let mut screen = SftpBrowserScreen::new();
        screen.transfer_progress.push(TransferProgress {
            filename: "big_file.bin".to_string(),
            progress: 0.5,
            status: "Uploading".to_string(),
        });
        render_smoke(&mut screen);
    }

    #[test]
    fn test_render_smoke_with_boundary_progress_values() {
        let mut screen = SftpBrowserScreen::new();
        screen.transfer_progress.push(TransferProgress {
            filename: String::new(),
            progress: 0.0,
            status: String::new(),
        });
        screen.transfer_progress.push(TransferProgress {
            filename: "done.bin".to_string(),
            progress: 1.0,
            status: "Complete".to_string(),
        });
        render_smoke(&mut screen);
    }

    #[test]
    fn test_render_smoke_with_long_and_unicode_names() {
        let mut screen = SftpBrowserScreen::new();
        screen.browser.set_entries(vec![
            make_entry(&"a".repeat(300), FileType::File, u64::MAX, true),
            make_entry("файл-名前-🎉.txt", FileType::File, 0, false),
        ]);
        render_smoke(&mut screen);
    }

    #[test]
    fn test_render_smoke_with_all_selected() {
        let mut screen = SftpBrowserScreen::new();
        screen.browser.set_entries(vec![
            make_entry("a", FileType::File, 1, true),
            make_entry("b", FileType::File, 2, true),
        ]);
        screen.browser.select_all();
        assert_eq!(screen.browser.get_selected_entries().len(), 2);
        render_smoke(&mut screen);
    }
}
