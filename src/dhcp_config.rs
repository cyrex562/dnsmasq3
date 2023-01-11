use std::fmt::{Debug, Formatter};
use std::ptr::{null_mut};
use libc::{c_char, in_addr, time_t};
use crate::addrlist::AddrList;
use crate::dhcp_netid::DhcpNetid;
use crate::dhcp_netid_list::DhcpNetidList;
use crate::hwaddr_config::HwAddrConfig;
use crate::util::inaddr_to_string;

#[derive(Debug, Default, Clone)]
pub struct DhcpConfig {
    pub flags: i32,
    pub clid_len: i32,
    pub clid: *mut u8,
    pub hostname: String,
    pub domain: String,
    pub netid: Vec<DhcpNetidList>,
    pub filter: Vec<DhcpNetid>,
    pub addr6: *mut AddrList,
    pub addr: in_addr,
    pub decline_time: time_t,
    pub lease_time: u32,
    pub hwaddr: Vec<HwAddrConfig>,
    // next
}

impl DhcpConfig {
    pub fn have_config(&self, mask: i32) -> bool {
        self.flags & mask > 0
    }
}
