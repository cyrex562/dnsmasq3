use crate::irec::irec;
use crate::mysockaddr::mysockaddr;

#[derive(Default, Debug, Clone)]
pub struct listener {
    // int fd, tcpfd, tftpfd, used;
    pub fd: i32,
    pub tcpfd: i32,
    pub tftpfd: i32,
    pub used: i32,
    // union mysockaddr addr;
    pub addr: mysockaddr,
    // struct irec *iface; /* only sometimes valid for non-wildcard */
    pub iface: *mut irec,
    // struct listener *next;
}
