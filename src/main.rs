//! TabSSH Desktop - Cross-platform SSH/SFTP client with browser-style tabs
//!
//! A modern SSH client built with Rust and egui, supporting:
//! - Multiple concurrent SSH connections in tabs
//! - Full VT100/xterm terminal emulation
//! - SFTP file browser and transfers
//! - Port forwarding (local, remote, dynamic)
//! - Secure credential storage
//! - Cross-platform: Linux, macOS, Windows, BSD

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

use anyhow::Result;
use app::TabSSHApp;
use eframe::egui;

/// Application name used for window title and config paths
const APP_NAME: &str = "TabSSH";

/// Application version from Cargo.toml
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Git commit ID (set at build time)
const BUILD_COMMIT: &str = env!("TABSSH_BUILD_COMMIT");

/// Build date (set at build time)
const BUILD_DATE: &str = env!("TABSSH_BUILD_DATE");

/// Full version string for display
pub fn version_string() -> String {
    format!("{} ({}) built {}", APP_VERSION, BUILD_COMMIT, BUILD_DATE)
}

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    let version = version_string();
    log::info!("{} {} starting...", APP_NAME, version);

    // Configure native options for the window
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(format!("{} - {}", APP_NAME, version))
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_drag_and_drop(true),
        centered: true,
        ..Default::default()
    };

    // Run the egui application
    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(TabSSHApp::new(cc)))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run application: {}", e))?;

    log::info!("{} shutting down", APP_NAME);
    Ok(())
}
