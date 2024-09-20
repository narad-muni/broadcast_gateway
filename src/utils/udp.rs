use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
    time::Duration,
};

use socket2::{Domain, Protocol, Socket, Type};

pub fn build_socket(mcast_ip: &String, interface_ip: &String, port: u16, timeout: u64) -> Socket {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();
    socket.set_reuse_address(true).unwrap();

    socket
        .join_multicast_v4(
            &mcast_ip.parse().unwrap(),
            &Ipv4Addr::from_str(&interface_ip).unwrap(),
        )
        .unwrap();

    socket
        .bind(&SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port).into())
        .unwrap();

    socket
        .set_read_timeout(Some(Duration::from_secs(timeout)))
        .unwrap();

    socket
}
