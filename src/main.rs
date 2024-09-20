use std::thread;

use crossbeam::channel::unbounded;
use input::{udp::UdpInput, Buffer};

mod constants;
mod input;
mod output;
mod processing;
mod settings;
mod utils;

fn main() {
    settings::init();

    let (udp_tx, udp_rx) = unbounded::<Buffer>();

    let (processing_tx, processing_rx) = unbounded::<Buffer>();

    // Spawn input thread
    thread::spawn(move || {
        UdpInput::new(udp_tx).read();
    });

    // Spawn processing thread
    thread::spawn(move || {
        processing::process(processing_tx, udp_rx);
    });

    // blocking output function
    // Will run infinitely
    output::write(processing_rx);
}
