use std::fmt::{Debug, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ptr::null_mut;
use libc::{c_char, in6_addr, in_addr};
use crate::addrlist::AddrList;
use crate::util::{in6addr_to_string, inaddr_to_string};

#[derive(Debug, Default, Clone)]
pub struct CondDomain {
    pub domain: String,
    pub prefix: String,
    pub al: Vec<AddrList>,
    pub start: Ipv4Addr,
    pub end: Ipv4Addr,
    pub start6: Ipv6Addr,
    pub end6: Ipv6Addr,
    pub is6: i32,
    pub indexed: i32,
    pub prefixlen: i32,
    // next
}
