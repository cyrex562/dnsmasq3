use std::ptr::null_mut;
use libc::c_char;

#[derive(Default, ebug, Clone)]
pub struct DhcpNetid {
    pub net: String,
    // struct dhcp_netid *next;
}
