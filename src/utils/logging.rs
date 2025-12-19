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
    
    log::info!("Logginginitializedat{}level",log_level);
}

pub fn log_session_start(host: &str, user: &str) {
    log::info!("===Sessionstarted:{}@{}===",user,host);
}

pub fn log_session_end(host: &str, duration: std::time::Duration) {
    log::info!("===Sessionended:{}(duration:{:?})===",host,duration);
}

pub fn log_transfer(direction: &str, filename: &str, bytes: u64) {
    log::info!("{}{}({}bytes)",direction,filename,bytes);
}
