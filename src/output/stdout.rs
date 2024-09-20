use crate::input::Buffer;

use super::Output;

pub struct StdOut {}

impl StdOut {
    pub fn new() -> StdOut {
        StdOut {}
    }
}

impl Output for StdOut {
    fn write(&mut self, data: Buffer) {
        println!("{}", String::from_utf8_lossy(&data.0));
    }
}
