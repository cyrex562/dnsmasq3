use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use libc::{c_char, in6_addr, in_addr, time_t};
use crate::dhcp_netid::dhcp_netid;
use crate::util::{in6addr_to_string, inaddr_to_string};

#[derive(Clone)]
pub struct dhcp_context {
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
    pub template_interface: *mut c_char,
    pub flags: i32,
    pub netid: dhcp_netid,
    pub filter: *mut dhcp_netid,
    // next
    // current
}

impl Default for dhcp_context {
    fn default() -> Self {
        Self {
            lease_time: 0,
            addr_epoch: 0,
            netmask: in_addr { s_addr: 0 },
            broadcast: in_addr { s_addr: 0 },
            local: in_addr { s_addr: 0 },
            router: in_addr { s_addr: 0 },
            start: in_addr { s_addr: 0 },
            end: in_addr { s_addr: 0 },
            start6: in6_addr { s6_addr: [0; 16] },
            end6: in6_addr { s6_addr: [0; 16] },
            local6: in6_addr { s6_addr: [0; 16] },
            prefix: 0,
            if_index: 0,
            valid: 0,
            preferred: 0,
            saved_valid: 0,
            ra_time: 0,
            ra_short_period_start: 0,
            address_lost_time: 0,
            template_interface: null_mut(),
            flags: 0,
            netid: Default::default(),
            filter: null_mut(),
        }
    }
}

impl Debug for dhcp_context {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ lease_time: {}, addr_epoch: {}, netmask: {}, broadcast: {}, local: {}, router: {}, start: {}, end: {}, start6: {}, end6: {}, local6: {}, prefix: {}, if_index: {}, valid: {}, preferred: {}, saved_valid: {}, ra_time: {}, ra_short_period_start: {}, address_list_time: {}, template_interface: {:?}, flags: {}, netid: {:?}, filter: {:?} }}", self.lease_time, self.addr_epoch, inaddr_to_string(&self.netmask), inaddr_to_string(&self.broadcast), inaddr_to_string(&self.local), inaddr_to_string(&self.router), inaddr_to_string(&self.start), inaddr_to_string(&self.end), in6addr_to_string(&self.start6), in6addr_to_string(&self.end6), in6addr_to_string(&self.local6), self.prefix, self.if_index, self.valid, self.preferred, self.saved_valid, self.ra_time, self.ra_short_period_start, self.address_lost_time, self.template_interface, self.flags, self.netid, self.filter)
    }
}
