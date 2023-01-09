use libc::c_char;
use crate::mysockaddr::mysockaddr;

#[derive(Default, Debug, Clone)]
pub struct serverfd {
    pub fd: i32,
    // union mysockaddr source_addr;
    pub source_addr: mysockaddr,
    pub interface: [c_char; IF_NAMESIZE + 1],
    // unsigned int ifindex, used, preallocated;
    pub ifindex: u32,
    pub used: u32,
    pub preallocated: u32,
    // struct serverfd *next;
}
