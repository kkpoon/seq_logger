extern crate log;
extern crate seq_logger;

use log::{info, Level};

fn main() {
    seq_logger::init_with_level_and_start_seq(Level::Info, 1, Some(10)).unwrap();
    for n in 1..21 {
        info!("Line {}", n);
    }
}
