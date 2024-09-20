use std::fmt::Debug;

use rdkafka::message::ToBytes;

use crate::constants::BUF_SIZE;

pub mod udp;

#[derive(Clone, Copy)]
pub struct Buffer(pub [u8; BUF_SIZE]);

impl ToBytes for Buffer {
    fn to_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
