use crossbeam::channel::{Receiver, Sender};

use crate::input::Buffer;

pub fn process(tx: Sender<Buffer>, rx: Receiver<Buffer>) {
    while let Ok(buf) = rx.recv() {
        let _ = tx.send(buf);
    }
}
