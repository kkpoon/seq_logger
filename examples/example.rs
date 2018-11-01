extern crate log;
extern crate seq_logger;

use log::info;

fn main() {
    seq_logger::init().unwrap();
    for n in 1..1001 {
        info!("Line {}", n);
    }
}
