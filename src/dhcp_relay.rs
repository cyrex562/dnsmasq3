use std::fmt::{Debug, Formatter};
use std::net::Ipv6Addr;
use std::ptr::null_mut;
use libc::{c_char, in6_addr};
use crate::all_addr::AllAddr;
use crate::snoop_record::SnoopRecord;
use crate::util::in6addr_to_string;


#[derive(Default,Debug, Clone)]
pub struct DhcpRelay {
    pub local: AllAddr,
    pub server: AllAddr,
    pub interface: String,
    pub iface_index: i32,
    pub port: i32,
    pub snoop_records:  Vec<SnoopRecord>,
}
