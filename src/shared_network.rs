use std::fmt::{Debug, Formatter};
use libc::{in6_addr, in_addr, in_addr_t};
use crate::util::{in6addr_to_string, inaddr_to_string};

#[derive(Clone)]
pub struct shared_network {
    pub if_index: i32,
    pub match_addr: in_addr,
    pub shared_addr: in_addr,
    pub match_addr6: in6_addr,
    pub shared_addr6: in6_addr,
    // next
}

impl Default for shared_network {
    fn default() -> Self {
        Self {
            if_index: 0,

            match_addr: in_addr { s_addr: 0 },
            shared_addr: in_addr { s_addr: 0 },
            match_addr6: in6_addr { s6_addr: [0; 16] },
            shared_addr6: in6_addr { s6_addr: [0; 16] },
        }
    }
}

impl Debug for shared_network {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ if_index: {}, match_addr: {}, shared_addr: {}, match_addr6: {}, shared_addr6: {} }}", self.if_index, inaddr_to_string(&self.match_addr), inaddr_to_string(&self.shared_addr), in6addr_to_string(&self.match_addr6), in6addr_to_string(&self.shared_addr6))
    }
}
