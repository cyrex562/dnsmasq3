use std::net::IpAddr;
use libc::c_char;
use crate::mysockaddr::MySockAddr;

#[derive(Default, Debug, Clone)]
pub struct ServerFd {
    pub fd: i32,
    // union mysockaddr source_addr;
    pub source_addr: IpAddr,
    pub interface: String,
    // unsigned int ifindex, used, preallocated;
    pub ifindex: u32,
    pub used: u32,
    pub preallocated: u32,
    // struct serverfd *next;
}
