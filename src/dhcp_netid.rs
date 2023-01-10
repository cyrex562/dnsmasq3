use std::ptr::null_mut;
use libc::c_char;

#[derive(Debug, Clone)]
pub struct dhcp_netid {
    pub net: *mut c_char,
    // struct dhcp_netid *next;
}

impl Default for dhcp_netid {
    fn default() -> Self {
        Self {
            net: null_mut()
        }
    }
}
