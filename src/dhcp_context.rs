use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use libc::{c_char, in6_addr, in_addr, time_t};
use crate::dhcp_netid::DhcpNetid;
use crate::util::{in6addr_to_string, inaddr_to_string};

#[derive(Default, Debug, Clone)]
pub struct DhcpContext {
    pub lease_time: u32,
    pub addr_epoch: u32,
    pub netmask: in_addr,
    pub broadcast: in_addr,
    pub local: in_addr,
    pub router: in_addr,
    pub start: in_addr,
    pub end: in_addr,
    pub start6: in6_addr,
    pub end6: in6_addr,
    pub local6: in6_addr,
    pub prefix: i32,
    pub if_index: i32,
    pub valid: u32,
    pub preferred: u32,
    pub saved_valid: u32,
    pub ra_time: time_t,
    pub ra_short_period_start: time_t,
    pub address_lost_time: time_t,
    pub template_interface: String,
    pub flags: i32,
    pub netid: DhcpNetid,
    pub filter: Vec<DhcpNetid>,
    // next
    // current
}
