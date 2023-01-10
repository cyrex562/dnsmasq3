use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use libc::{c_char, in6_addr, time_t};
use crate::dhcp_context::dhcp_context;
use crate::dhcp_netid::dhcp_netid;
use crate::util::in6addr_to_string;

#[derive(Clone)]
pub struct ra_param {
    pub now: time_t,
    pub ind: i32,
    pub managed: i32,
    pub other: i32,
    pub first: i32,
    pub adv_router: i32,
    pub if_name: *mut c_char,
    pub tags: *mut dhcp_netid,
    pub link_local: in6_addr,
    pub link_global: in6_addr,
    pub ula: in6_addr,
    pub glob_pref_time: u32,
    pub link_pref_time: u32,
    pub ula_pref_time: u32,
    pub adv_interval: u32,
    pub prio: u32,
    pub found_context: *mut dhcp_context,
}

impl Default for ra_param {
    fn default() -> Self {
        Self {
            now: 0,
            ind: 0,
            managed: 0,
            other: 0,
            first: 0,
            adv_router: 0,
            if_name: null_mut(),
            tags: null_mut(),
            link_local: in6_addr { s6_addr: [0; 16] },
            link_global: in6_addr { s6_addr: [0; 16] },
            ula: in6_addr { s6_addr: [0; 16] },
            glob_pref_time: 0,
            link_pref_time: 0,
            ula_pref_time: 0,
            adv_interval: 0,
            prio: 0,
            found_context: null_mut(),
        }
    }
}

impl Debug for ra_param {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ now: {}, ind: {}, managed: {}, other: {}, first: {}, adv_router: {}, if_name: {:?}, tags: {:?}, link_local: {}, link_global: {}, ula: {}, glob_pref_time: {}, link_pref_time: {}, ula_pref_time: {}, adv_interval: {}, prio: {}, found_context: {:?} }}", self.now, self.ind, self.managed, self.other, self.first, self.adv_router, self.if_name, self.tags, in6addr_to_string(&self.link_local), in6addr_to_string(&self.link_global), in6addr_to_string(&self.ula), in6addr_to_string(&self.glob_pref_time), self.link_pref_time, self.ula_pref_time, self.adv_interval, self.prio, self.found_context)
    }
}
