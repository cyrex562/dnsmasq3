use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use libc::{c_char, in6_addr, in_addr};
use crate::addrlist::addrlist;
use crate::util::{in6addr_to_string, inaddr_to_string};

#[derive(Clone)]
pub struct interface_name {
    // char *name; /* domain name */
    pub name: *mut c_char,
    // char *intr; /* interface name */
    pub intr: *mut c_char,
    flags: i32,
    pub proto4: in_addr,
    pub proto6: in6_addr,
    addr: *mut addrlist,
    // struct interface_name *next;
}

impl Default for interface_name {
    fn default() -> Self {
        Self {
            name: null_mut(),
            intr: null_mut(),
            flags: 0,
            proto4: in_addr { s_addr: 0 },
            proto6: in6_addr { s6_addr: [0; 16] },
            addr: null_mut(),
        }
    }
}

impl Debug for interface_name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ name: {:?}, intr: {:?}, flags: {}, proto4: {}, proto6: {}, addr: {:?}", self.name, self.intr, self.flags, inaddr_to_string(&self.proto4), in6addr_to_string(&self.proto6), self.addr)
    }
}
