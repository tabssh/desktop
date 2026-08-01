//! Command-line interface: argument parsing, color policy, and runtime-mode
//! selection (PART 3 "Runtime Mode Selection" + PART 7 "Standard CLI Flags").

use clap::{Parser, ValueEnum};
use env_logger::WriteStyle;

/// Application version string surfaced by `--version`. Sourced from
/// `release.txt` at build time via `build.rs` (`APP_VERSION`) when present,
/// following the PART 6 "Metadata Priority Rules" precedence, otherwise the
/// crate version from `Cargo.toml`.
const VERSION: &str = match option_env!("APP_VERSION") {
    Some(v) => v,
    None => env!("CARGO_PKG_VERSION"),
};

/// Runtime UI mode. Automatic selection priority is GUI > TUI > CLI; an
/// explicit `--ui` flag overrides detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum UiMode {
    /// Native windowed desktop UI.
    Gui,
    /// Full-screen terminal UI.
    Tui,
    /// Plain, non-interactive command-line output.
    Cli,
}

/// Color output policy for terminal/log output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ColorChoice {
    /// Detect a TTY and honor `NO_COLOR` / `TERM=dumb`.
    Auto,
    /// Force color output on.
    Yes,
    /// Force color output off.
    No,
}

/// Parsed command-line arguments. `--help`/`-h` and `--version`/`-v` are
/// handled by clap and exit 0 before any privilege or capability check.
#[derive(Debug, Parser)]
#[command(
    name = "tabssh",
    about = "Cross-platform SSH/SFTP client with browser-style tabs",
    version = VERSION,
    disable_version_flag = true
)]
pub struct Cli {
    /// Show version and exit.
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    pub version: Option<bool>,

    /// Enable debug output.
    #[arg(long)]
    pub debug: bool,

    /// Color output: `auto` (TTY detect), `yes` (force on), `no` (force off).
    #[arg(long, value_enum, default_value_t = ColorChoice::Auto, value_name = "auto|yes|no")]
    pub color: ColorChoice,

    /// Force a UI mode instead of automatic GUI/TUI/CLI detection.
    #[arg(long = "ui", value_enum, value_name = "gui|tui|cli")]
    pub ui: Option<UiMode>,
}

/// Resolve the effective terminal color style from the `--color` choice,
/// honoring `NO_COLOR` and `TERM=dumb` under `auto`.
pub fn resolve_write_style(color: ColorChoice) -> WriteStyle {
    match color {
        ColorChoice::Yes => WriteStyle::Always,
        ColorChoice::No => WriteStyle::Never,
        ColorChoice::Auto => {
            let no_color = std::env::var_os("NO_COLOR")
                .map(|v| !v.is_empty())
                .unwrap_or(false);
            let dumb = std::env::var("TERM").map(|t| t == "dumb").unwrap_or(false);
            if no_color || dumb {
                WriteStyle::Never
            } else {
                WriteStyle::Auto
            }
        }
    }
}

/// True when the process is running inside an SSH/MOSH remote-shell context,
/// which blocks automatic GUI selection (PART 3 "Smart Detect Rules").
fn is_remote_shell() -> bool {
    [
        "SSH_CONNECTION",
        "SSH_CLIENT",
        "SSH_TTY",
        "MOSH_IP",
        "MOSH_KEY",
    ]
    .iter()
    .any(|k| std::env::var_os(k).is_some())
}

/// True when a local desktop/session display stack is available.
fn display_available() -> bool {
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        // A local Windows desktop session or macOS Aqua launch always has a
        // display stack available to the process.
        true
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        // On Linux/BSD, Wayland (`WAYLAND_DISPLAY`) or X11/XWayland (`DISPLAY`)
        // signals a usable display stack.
        std::env::var_os("WAYLAND_DISPLAY").is_some() || std::env::var_os("DISPLAY").is_some()
    }
}

/// Select the runtime UI mode. An explicit `--ui` flag always wins; otherwise
/// choose GUI when a local display exists and the process is not a remote
/// shell, falling back to CLI (the required non-interactive default).
///
/// TUI is never auto-selected today because the TUI frontend is not yet
/// implemented; automatic selection therefore collapses to GUI-or-CLI.
pub fn detect_ui_mode(forced: Option<UiMode>) -> UiMode {
    if let Some(mode) = forced {
        return mode;
    }

    if display_available() && !is_remote_shell() {
        UiMode::Gui
    } else {
        UiMode::Cli
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forced_mode_wins_over_detection() {
        assert_eq!(detect_ui_mode(Some(UiMode::Tui)), UiMode::Tui);
        assert_eq!(detect_ui_mode(Some(UiMode::Cli)), UiMode::Cli);
        assert_eq!(detect_ui_mode(Some(UiMode::Gui)), UiMode::Gui);
    }

    #[test]
    fn explicit_color_choices_map_directly() {
        assert!(matches!(
            resolve_write_style(ColorChoice::Yes),
            WriteStyle::Always
        ));
        assert!(matches!(
            resolve_write_style(ColorChoice::No),
            WriteStyle::Never
        ));
    }

    #[test]
    fn cli_parses_universal_flags() {
        let cli = Cli::try_parse_from(["tabssh", "--debug", "--color", "no", "--ui", "cli"])
            .expect("valid flags should parse");
        assert!(cli.debug);
        assert!(matches!(cli.color, ColorChoice::No));
        assert_eq!(cli.ui, Some(UiMode::Cli));
    }

    #[test]
    fn color_accepts_both_space_and_equals_forms() {
        let spaced = Cli::try_parse_from(["tabssh", "--color", "yes"]).unwrap();
        let equals = Cli::try_parse_from(["tabssh", "--color=yes"]).unwrap();
        assert!(matches!(spaced.color, ColorChoice::Yes));
        assert!(matches!(equals.color, ColorChoice::Yes));
    }
}
