use crossbeam::channel::{Receiver, Sender};

use crate::constants::BUF_SIZE;

pub fn process(tx: Sender<[u8; BUF_SIZE]>, rx: Receiver<[u8; BUF_SIZE]>) {

    while let Ok(buf) = rx.recv() {
        println!("{}", String::from_utf8_lossy(&buf));

        tx.send(buf);
    }
}