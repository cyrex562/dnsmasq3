/* dns doctor param */
use std::fmt::{Debug, Formatter};
use libc::in_addr;
use crate::util::inaddr_to_string;

#[derive(Clone)]
pub struct doctor {
    // struct in_addr in, end, out, mask;
    pub in_address: in_addr,
    pub end_address: in_addr,
    pub out_address: in_addr,
    pub mask_address: in_addr,
    // struct doctor *next;
}

impl Default for doctor {
    fn default() -> Self {
        Self {
            in_address: in_addr { s_addr: 0 },
            end_address: in_addr { s_addr: 0 },
            out_address: in_addr { s_addr: 0 },
            mask_address: in_addr { s_addr: 0 },
        }
    }
}

impl Debug for doctor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ in_address: {}, end_address: {}, out_address: {}, mask_address: {} }}", inaddr_to_string(&self.in_address), inaddr_to_string(&self.end_address), inaddr_to_string(&self.out_address), inaddr_to_string(&self.mask_address))
    }
}
