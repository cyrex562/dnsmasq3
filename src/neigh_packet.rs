use std::fmt::{Debug, Formatter};
use libc::in6_addr;
use crate::util::in6addr_to_string;

#[derive(Debug, Clone)]
pub struct neigh_packet {
    pub pkt_type: u8,
    pub code: u8,
    pub checksum: u16,
    pub reserved: u16,
    pub target: in6_addr,
}

impl Default for neigh_packet {
    fn default() -> Self {
        Self {
            pkt_type: 0,
            code: 0,
            checksum: 0,
            reserved: 0,
            target: in6_addr { s6_addr: [0; 16] },
        }
    }
}

impl Debug for neigh_packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ pkt_type: {}, code: {}, checksum: {}, reserved: {}, target: {} }}", self.pkt_type, self.code, self.checksum, self.reserved, in6addr_to_string(&self.target))
    }
}
