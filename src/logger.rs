extern crate chrono;
extern crate log;

use log::{Level, LevelFilter, Metadata, Record};
use std::str::FromStr;

pub static LOGGER: Logger = Logger;

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if !record.target().starts_with("elser") {
            return;
        }
        if self.enabled(record.metadata()) {
            println!(
                "{} - {}:{} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.level(),
                record.args()
            );
        }
    }
    fn flush(&self) {}
}

pub fn setup_logger(log_level: &str) {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::from_str(log_level).unwrap_or(LevelFilter::Warn));
}
