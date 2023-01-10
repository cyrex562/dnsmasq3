use std::fmt::{Debug, Formatter};
use std::ptr::{null_mut};
use libc::{c_char, in_addr, time_t};
use crate::addrlist::addrlist;
use crate::dhcp_netid::dhcp_netid;
use crate::dhcp_netid_list::dhcp_netid_list;
use crate::hwaddr_config::hwaddr_config;
use crate::util::inaddr_to_string;

#[derive(Clone)]
pub struct dhcp_config {
    pub flags: i32,
    pub clid_len: i32,
    pub clid: *mut u8,
    pub hostname: *mut c_char,
    pub domain: *mut c_char,
    pub netid: *mut dhcp_netid_list,
    pub filter: *mut dhcp_netid,
    pub addr6: *mut addrlist,
    pub addr: in_addr,
    pub decline_time: time_t,
    pub lease_time: u32,
    pub hwaddr: *mut hwaddr_config,
    // next
}

impl Default for dhcp_config {
    fn default() -> Self {
        Self {
            flags: 0,
            clid_len: 0,
            clid: null_mut(),
            hostname: null_mut(),
            domain: null_mut(),
            netid: null_mut(),
            filter: null_mut(),
            addr6: null_mut(),
            addr: in_addr { s_addr: 0 },
            decline_time: 0,
            lease_time: 0,
            hwaddr: null_mut(),
        }
    }
}

impl Debug for dhcp_config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{flags: {}, clid_len: {}, clid: {:?}, hostname: {:?}, domain: {:?}, netid: {:?}, filter: {:?}, addr6: {:?}, addr: {}, decline_time: {}, lease_time: {:?}, hwaddr: {:?}}}", self.flags, self.clid_len, self.clid, self.hostname, self.domain, self.netid, self.filter, self.addr6, inaddr_to_string(&self.addr), self.decline_time, self.lease_time, self.hwaddr)
    }
}

impl dhcp_config {
    pub fn have_config(&self, mask: i32) -> bool {
        self.flags & mask > 0
    }
}