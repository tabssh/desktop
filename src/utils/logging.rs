//! Logging configuration

use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init_logging(level: &str) {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    Builder::new()
        .filter_level(log_level)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();

    log::info!("Logging initialized at {} level", log_level);
}

pub fn log_session_start(host: &str, user: &str) {
    log::info!("=== Session started: {}@{} ===", user, host);
}

pub fn log_session_end(host: &str, duration: std::time::Duration) {
    log::info!("=== Session ended: {} (duration: {:?}) ===", host, duration);
}

pub fn log_transfer(direction: &str, filename: &str, bytes: u64) {
    log::info!("{} {} ({} bytes)", direction, filename, bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    // log_session_start/end/transfer just emit `log::info!` calls; without
    // an installed logger these are no-ops. Assert they run without
    // panicking across boundary inputs (empty strings, zero bytes).

    #[test]
    fn test_log_session_start_smoke() {
        log_session_start("example.com", "root");
        log_session_start("", "");
    }

    #[test]
    fn test_log_session_end_smoke() {
        log_session_end("example.com", std::time::Duration::from_secs(0));
        log_session_end("example.com", std::time::Duration::from_secs(3600));
    }

    #[test]
    fn test_log_transfer_smoke() {
        log_transfer("upload", "file.txt", 0);
        log_transfer("download", "", u64::MAX);
    }

    #[test]
    fn test_init_logging_levels_and_repeat_calls() {
        // First call installs the real global logger (only one process-wide
        // logger may ever be installed).
        init_logging("trace");
        assert!(log::max_level() >= LevelFilter::Trace);

        // Every subsequent call still runs the level-parsing match arm
        // (exercising each branch, including the unrecognized-level
        // default) before env_logger rejects the second logger install
        // with a panic — assert that panic actually happens.
        for level in ["debug", "info", "warn", "error", "unknown-level", ""] {
            let result = std::panic::catch_unwind(|| init_logging(level));
            assert!(
                result.is_err(),
                "second init_logging call for level {:?} should panic (logger already set)",
                level
            );
        }
    }
}
