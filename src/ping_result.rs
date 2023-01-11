use std::fmt::{Debug, Formatter};
use libc::{in_addr, time_t};
use crate::util::inaddr_to_string;

#[derive(Clone)]
pub struct PingResult {
    pub addr: in_addr,
    pub time: time_t,
    pub hash: u32,
    // next
}

impl Default for PingResult {
    fn default() -> Self {
        Self {
            addr: in_addr { s_addr: 0 },
            time: 0,
            hash: 0,
        }
    }
}

impl Debug for PingResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ addr: {}, time: {}, hash: {} }}", inaddr_to_string(&self.addr), self.time, self.hash)
    }
}