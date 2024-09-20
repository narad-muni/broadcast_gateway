use crossbeam::channel::Receiver;
use kafka::KafkaOutput;
use stdout::StdOut;
use udp::UdpOutput;

use crate::{
    input::Buffer,
    settings::{self, types::OutputTargets},
};

pub mod kafka;
pub mod stdout;
pub mod udp;

trait Output {
    fn write(&mut self, data: Buffer);
}

pub fn write(rx: Receiver<Buffer>) {
    let mut kafka = KafkaOutput::new();
    let mut udp = UdpOutput::new();
    let mut stdout = StdOut::new();

    let settings = settings::get();

    while let Ok(buf) = rx.recv() {
        if settings.output_targets.contains(OutputTargets::UDP) {
            udp.write(buf);
        }

        if settings.output_targets.contains(OutputTargets::KAFKA) {
            kafka.write(buf);
        }

        if settings.output_targets.contains(OutputTargets::STDOUT) {
            stdout.write(buf);
        }
    }
}
