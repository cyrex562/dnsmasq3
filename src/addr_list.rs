use std::fmt::{Debug, Formatter};
use libc::in_addr;
use crate::util::inaddr_to_string;

#[derive(Clone)]
struct addr_list {
    pub addr: in_addr,
    // struct addr_list *next;
}

impl Default for addr_list {
    fn default() -> Self {
        Self {
            addr: in_addr { s_addr: 0 },
        }
    }
}

impl Debug for addr_list {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ addr: {} }}", inaddr_to_string(&self.addr))
    }
}
