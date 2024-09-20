use std::{io::Write, net::SocketAddrV4};

use socket2::{SockAddr, Socket};

use crate::{input::Buffer, settings, utils::udp::build_socket};

use super::Output;
pub struct UdpOutput {
    socket: Socket,
}

impl UdpOutput {
    pub fn new() -> UdpOutput {
        let settings = settings::get();

        // Build a udp socket with mcast connection
        let socket = build_socket(
            &settings.output_udp_ip,
            &settings.udp_local_ip,
            settings.output_udp_port as u16,
            0,
        );

        // Connect to mcast ip
        // Required for sending packets
        socket
            .connect(&SockAddr::from(SocketAddrV4::new(
                settings.output_udp_ip.parse().unwrap(),
                settings.output_udp_port as u16,
            )))
            .unwrap();

        UdpOutput { socket }
    }
}

impl Output for UdpOutput {
    fn write(&mut self, data: Buffer) {
        self.socket.write(&data.0).unwrap();
    }
}
