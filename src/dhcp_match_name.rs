use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use libc::c_char;
use crate::dhcp_netid::dhcp_netid;

#[derive(Clone)]
pub struct dhcp_match_name {
    pub name: *mut c_char,
    pub wildcard: i32,
    pub netid: *mut dhcp_netid,
    // next
}

impl Default for dhcp_match_name {
    fn default() -> Self {
        Self {
            name: null_mut(),
            wildcard: 0,
            netid: null_mut(),
        }
    }
}

impl Debug for dhcp_match_name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ name: {:?}, wildcard: {}, netid: {:?} }}", self.name, self.wildcard, self.netid)
    }
}
