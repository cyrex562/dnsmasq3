use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use libc::in6_addr;
use crate::dhcp_context::dhcp_context;
use crate::util::in6addr_to_string;

#[derive(Clone)]
pub struct iface_param {
    // struct dhcp_context *current;
    pub current: *mut dhcp_context,
    pub ind: i32,
    pub fallback: in6_addr,
    pub ll_addr: in6_addr,
    pub ula_addr: in6_addr,
    pub addr_match: i32,
}

impl Default for iface_param {
    fn default() -> Self {
        Self {
            current: null_mut(),
            ind: 0,
            fallback: in6_addr { s6_addr: [0; 16] },
            ll_addr: in6_addr { s6_addr: [0; 16] },
            ula_addr: in6_addr { s6_addr: [0; 16] },
            addr_match: 0,
        }
    }
}

impl Debug for iface_param {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ current: {:?}, ind: {}, fallback: {}, ll_addr: {}, ula_addr: {}, addr_match: {} }}", self.current, self.ind, in6addr_to_string(&self.fallback), in6addr_to_string(&self.ll_addr), in6addr_to_string(&self.ula_addr), self.addr_match)
    }
}
