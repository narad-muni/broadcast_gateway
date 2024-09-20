use std::io::Read;

use crossbeam::channel::Sender;
use socket2::Socket;

use crate::{constants::BUF_SIZE, settings, utils::udp::build_socket};

use super::Buffer;

enum SocketType {
    Primary,
    Secondary,
}

pub struct UdpInput<'a> {
    primary: Socket,
    secondary: Socket,
    current: Option<&'a Socket>,
    current_id: SocketType,
    tx: Sender<Buffer>,
    auto_switch: bool,
}

impl<'a> UdpInput<'a> {
    pub fn new(tx: Sender<Buffer>) -> UdpInput<'a> {
        let settings = settings::get();

        // Set timeout 0 if autoswitch is false
        let timeout = if settings.udp_auto_switch == true {
            settings.udp_switch_timeout
        } else {
            0
        } as u64;

        UdpInput {
            primary: build_socket(
                &settings.primary_mcast_ip,
                &settings.udp_local_ip,
                settings.primary_mcast_port as u16,
                timeout,
            ),
            secondary: build_socket(
                &settings.secondary_mcast_ip,
                &settings.udp_local_ip,
                settings.secondary_mcast_port as u16,
                timeout,
            ),
            current: None,
            current_id: SocketType::Primary,
            tx,
            auto_switch: settings.udp_auto_switch,
        }
    }

    pub fn read(&'a mut self) {
        // Set current to primary
        self.current = Some(&self.primary);

        loop {
            let mut buf = Buffer([0; BUF_SIZE]);

            // Value can never be none
            assert!(self.current.is_some());

            // If error, then proceed to switch
            if let Err(_) = self.current.unwrap().read(&mut buf.0) {
                // If autoswitch is false, then don't rotate
                if !self.auto_switch {
                    continue;
                }

                // Switch current id
                match self.current_id {
                    SocketType::Primary => self.current_id = SocketType::Secondary,
                    SocketType::Secondary => self.current_id = SocketType::Primary,
                };

                // Rotate between primary and secondary
                self.current = match self.current_id {
                    SocketType::Primary => Some(&self.secondary),
                    SocketType::Secondary => Some(&self.primary),
                };

                continue;
            }

            self.tx.send(buf).unwrap();
        }
    }
}
