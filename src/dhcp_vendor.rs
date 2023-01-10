/* vendorclass, userclass, remote-id or circuit-id */
use std::ffi::c_char;
use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use crate::dhcp_netid::dhcp_netid;

#[derive(Clone)]
pub struct dhcp_vendor {
    pub len: i32,
    pub match_type: i32,
    pub enterprise: u32,
    pub data: *mut c_char,
    pub netid: dhcp_netid,
    // next
}


impl Default for dhcp_vendor {
    fn default() -> Self {
        Self {
            len: 0,
            match_type: 0,
            enterprise: 0,
            data: null_mut(),
            netid: dhcp_netid::default(),
        }
    }
}


impl Debug for dhcp_vendor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ len: {}, match_type: {}, enterprise: {}, data: {:?}, netid: {:?} }}", self.len, self.match_type, self.enterprise, self.data, self.netid)
    }
}