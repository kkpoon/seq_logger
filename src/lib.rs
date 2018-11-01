extern crate log;

use log::{Level, Log, Metadata, Record, SetLoggerError};
use std::sync::Mutex;

struct SeqLogger {
    level: Level,
    seq: Mutex<i64>,
}

impl Log for SeqLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        let mut seq = self.seq.lock().unwrap();
        println!("[{}] {} - {}", *seq, record.level(), record.args());
        *seq = *seq + 1;
    }

    fn flush(&self) {}
}

pub fn init_with_level_and_seq(level: Level, seq: i64) -> Result<(), SetLoggerError> {
    let logger = SeqLogger {
        level,
        seq: Mutex::new(seq),
    };
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

pub fn init() -> Result<(), SetLoggerError> {
    init_with_level_and_seq(Level::Trace, 1)
}
