use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use libc::{c_char, in6_addr, in_addr};
use crate::util::{in6addr_to_string, inaddr_to_string};

#[derive(Debug, Clone)]
pub struct name_list {
    // char *name;
    pub name: *mut c_char,
    // struct name_list *next;
}

impl Default for name_list {
    fn default() -> Self {
        Self {
            name: null_mut(),
        }
    }
}


#[derive(Clone)]
pub struct host_record {
    // int ttl, flags;
    pub ttl: i32,
    pub flags: i32,
    pub names: *mut name_list,
    // addr: in_addr;
    pub addr: in_addr,
    // addr6: in6_addr;
    pub in6_addr: in6_addr,
    // struct host_record *next;
}

impl Default for host_record {
    fn default() -> Self {
        Self {
            ttl: 0,
            flags: 0,
            names: null_mut(),
            addr: in_addr { s_addr: 0 },
            in6_addr: in6_addr { s6_addr: [0; 16] },
        }
    }
}

impl Debug for host_record {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ttl: {}, flags: {}, names: {:?}, addr: {}, in6_addr: {} }}", self.ttl, self.flags, self.names, inaddr_to_string(&self.addr), in6addr_to_string(&self.in6_addr))
    }
}
