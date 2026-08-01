//! TabSSH Desktop - Cross-platform SSH/SFTP client

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod cli;

use app::TabSshApp;
use clap::Parser;
use cli::{Cli, UiMode};
use tabssh::{platform, utils};

fn main() -> anyhow::Result<()> {
    // Parse arguments first: `--help`/`--version` exit here, before any
    // logging, platform setup, or capability check (PART 7 "No escalation").
    let args = Cli::parse();

    let level = if args.debug { "debug" } else { "info" };
    utils::logging::init_logging(level, cli::resolve_write_style(args.color));

    log::info!("Starting TabSSH Desktop v{}", env!("CARGO_PKG_VERSION"));

    match cli::detect_ui_mode(args.ui) {
        UiMode::Gui => run_gui(),
        UiMode::Tui => {
            eprintln!(
                "tabssh: TUI mode is not yet available in this build (planned). \
                 Run in a desktop session for the GUI."
            );
            std::process::exit(1);
        }
        UiMode::Cli => {
            eprintln!(
                "tabssh: no interactive display detected and CLI/TUI mode is not \
                 yet available in this build (planned). Run in a local desktop \
                 session for the GUI, or force it with `--ui gui`."
            );
            std::process::exit(1);
        }
    }
}

/// Launch the native windowed GUI.
fn run_gui() -> anyhow::Result<()> {
    // Platform-specific initialization
    #[cfg(target_os = "linux")]
    platform::linux::setup();

    #[cfg(target_os = "macos")]
    platform::macos::setup();

    #[cfg(target_os = "windows")]
    platform::windows::setup();

    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    platform::bsd::setup();

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
        Box::new(|cc| Ok(Box::new(TabSshApp::new(cc)))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run application: {}", e))
}
