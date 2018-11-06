extern crate log;

use log::{Level, Log, Metadata, Record, SetLoggerError};
use std::sync::Mutex;

struct SeqLogger {
    level: Level,
    start: u64,
    end: u64,
    seq: Mutex<u64>,
}

impl Log for SeqLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        let mut seq = self.seq.lock().unwrap();
        println!("[{}] {} - {}", *seq, record.level(), record.args());
        if *seq >= self.end {
            *seq = self.start;
        } else {
            *seq = *seq + 1;
        }
    }

    fn flush(&self) {}
}

pub fn init_with_level_and_start_seq(
    level: Level,
    start: u64,
    max_seq: Option<u64>,
) -> Result<(), SetLoggerError> {
    let logger = SeqLogger {
        level,
        start,
        end: max_seq.unwrap_or(u64::max_value()),
        seq: Mutex::new(start),
    };
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

pub fn init() -> Result<(), SetLoggerError> {
    init_with_level_and_start_seq(Level::Trace, 1, None)
}
