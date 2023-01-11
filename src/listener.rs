use std::net::IpAddr;
use crate::irec::Irec;
use crate::mysockaddr::MySockAddr;

#[derive(Default, Debug, Clone)]
pub struct Listener {
    // int fd, tcpfd, tftpfd, used;
    pub fd: i32,
    pub tcpfd: i32,
    pub tftpfd: i32,
    pub used: i32,
    // union mysockaddr addr;
    pub addr: IpAddr,
    // struct irec *iface; /* only sometimes valid for non-wildcard */
    pub iface: Vec<Irec>,
    // struct listener *next;
}
