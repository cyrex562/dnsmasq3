use std::fmt::{Debug, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ptr::null_mut;
use libc::{c_char, in6_addr, in_addr};
use crate::addrlist::AddrList;
use crate::util::{in6addr_to_string, inaddr_to_string};

#[derive(Default, Debug, Clone)]
pub struct InterfaceName {
    // char *name; /* domain name */
    pub name: String,
    // char *intr; /* interface name */
    pub intr: String,
    flags: i32,
    pub proto4: Ipv4Addr,
    pub proto6: Ipv6Addr,
    addr: Vec<AddrList>,
    // struct interface_name *next;
}
