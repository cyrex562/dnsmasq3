use std::fmt::{Debug, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ptr::null_mut;
use libc::{c_char, in6_addr, in_addr};
use crate::name_list::NameList;
use crate::util::{in6addr_to_string, inaddr_to_string};

#[derive(Debug, Default, Clone)]
pub struct HostRecord {
    pub ttl: i32,
    pub flags: i32,
    pub names: Vec<NameList>,
    pub addr: Ipv4Addr,
    pub addr6: Ipv6Addr,
    // struct host_record *next;
}
