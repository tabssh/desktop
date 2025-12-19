//! TabSSH Desktop - Cross-platform SSH/SFTP client

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod crypto;
mod platform;
mod sftp;
mod ssh;
mod storage;
mod terminal;
mod ui;
mod utils;

use app::TabSshApp;

fn main() -> anyhow::Result<()> {
    // Initialize logging
    utils::logging::init_logging("info");
    
    log::info!("StartingTabSSHDesktopv{}",env!("CARGO_PKG_VERSION"));
    
    // Platform-specific initialization
    #[cfg(target_os = "linux")]
    platform::linux::setup();
    
    #[cfg(target_os = "macos")]
    platform::macos::setup();
    
    #[cfg(target_os = "windows")]
    platform::windows::setup();
    
    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    platform::bsd::setup();
    
    // Run application
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("TabSSH Desktop"),
        ..Default::default()
    };
    
    eframe::run_native(
        "TabSSH",
        native_options,
        Box::new(|cc| Box::new(TabSshApp::new(cc))),
    )
    .map_err(|e| anyhow::anyhow!("Failedtorunapplication:{}",e))
}
