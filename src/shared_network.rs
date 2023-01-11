use std::fmt::{Debug, Formatter};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use libc::{in6_addr, in_addr, in_addr_t};
use crate::util::{in6addr_to_string, inaddr_to_string};

#[derive(Default, Debug, Clone)]
pub struct SharedNetwork {
    pub if_index: i32,
    pub match_addr: Ipv4Addr,
    pub shared_addr: Ipv4Addr,
    pub match_addr6: Ipv6Addr,
    pub shared_addr6: Ipv6Addr,
    // next
}
