use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use libc::{c_char, in6_addr};
use crate::all_addr::all_addr;
use crate::util::in6addr_to_string;


#[derive(Clone)]
pub struct snoop_record {
    pub client: in6_addr,
    pub prefix: in6_addr,
    pub prefix_len: i32,
    // next
}

impl Default for snoop_record {
    fn default() -> Self {
        Self {
            client: in6_addr { s6_addr: [0; 16] },
            prefix: in6_addr { s6_addr: [0; 16] },
            prefix_len: 0,
        }
    }
}

impl Debug for snoop_record {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ client: {}, prefix: {}, prefix_len: {} }}", in6addr_to_string(&self.client), in6addr_to_string(&self.prefix_len), self.prefix_len)
    }
}


#[derive(Debug, Clone)]
pub struct dhcp_relay {
    pub local: all_addr,
    pub server: all_addr,
    pub interface: *mut c_char,
    pub iface_index: i32,
    pub port: i32,
    pub snoop_records: *mut snoop_record,
}

impl Default for dhcp_relay {
    fn default() -> Self {
        Self {
            local: Default::default(),
            server: Default::default(),
            interface: null_mut(),
            iface_index: 0,
            port: 0,
            snoop_records: null_mut(),
        }
    }
}
