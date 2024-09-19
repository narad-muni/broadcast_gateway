use std::thread;

use constants::BUF_SIZE;
use crossbeam::channel::unbounded;
use udp_input::UdpInput;

mod constants;
mod utils;
// mod output;
mod udp_input;
mod settings;
mod processing;

fn main() {
    settings::init();

    let (udp_tx, udp_rx) = unbounded::<[u8; BUF_SIZE]>();

    let (processing_tx, processing_rx) = unbounded::<[u8; BUF_SIZE]>();

    thread::spawn(move || {
        UdpInput::new(udp_tx).read();
    });

    processing::process(processing_tx, udp_rx);
}