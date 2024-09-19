use std::{io::Read, net::{Ipv4Addr, SocketAddrV4}, str::FromStr, time::Duration};

use crossbeam::channel::Sender;
use socket2::{Domain, Protocol, Socket, Type};

use crate::{constants::BUF_SIZE, settings};

enum SocketType {
    Primary,
    Secondary
}

pub struct UdpInput<'a> {
    primary: Socket,
    secondary: Socket,
    current: Option<&'a Socket>,
    current_id: SocketType,
    tx: Sender<[u8; BUF_SIZE]>,
    auto_switch: bool,
}

impl<'a> UdpInput<'a> {
    pub fn new(tx: Sender<[u8; BUF_SIZE]>) -> UdpInput<'a> {
        let settings = settings::get();

        UdpInput {
            primary: build_socket(&settings.primary_mcast_ip, &settings.udp_local_ip, settings.primary_mcast_port as u16, settings.udp_switch_timeout as u64),
            secondary: build_socket(&settings.secondary_mcast_ip, &settings.udp_local_ip, settings.secondary_mcast_port as u16, settings.udp_switch_timeout as u64),
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
            let mut buf = [0; BUF_SIZE];

            // Value can never be none
            assert!(self.current.is_some());

            // If error, then proceed to switch
            if let Err(_) = self.current.unwrap().read(&mut buf) {

                // If autoswitch is false, then don't rotate
                if !self.auto_switch { continue; }

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

fn build_socket(
    mcast_ip: &String,
    interface_ip: &String,
    port: u16,
    timeout: u64,
) -> Socket {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();
    socket.set_reuse_address(true).unwrap();
    
    socket
        .join_multicast_v4(&mcast_ip.parse().unwrap(), &Ipv4Addr::from_str(&interface_ip).unwrap())
        .unwrap();

    socket
        .bind(&SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port).into())
        .unwrap();

    socket.set_read_timeout(Some(Duration::from_secs(timeout))).unwrap();

    socket
}