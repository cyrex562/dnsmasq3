use std::os::raw::c_void;
use libc::{c_char, in6_addr};
use crate::dhcp_constants::DHCP_CHADDR_MAX;
use crate::dhcp_context::dhcp_context;
use crate::dhcp_netid::dhcp_netid;

#[derive(Default, Debug, Clone)]
pub struct state {
    pub clid: *mut u8,
    pub clid_len: i32,
    pub ia_type: i32,
    pub interface: i32,
    pub hostname_auth: i32,
    pub lease_allocate: i32,
    pub client_hostanme: *mut c_char,
    pub hostname: *mut c_char,
    pub domain: *mut c_char,
    pub send_domain: *mut c_char,
    pub context: *mut dhcp_context,
    pub link_address: *mut in6_addr,
    pub fallback: *mut in6_addr,
    pub ll_addr: *mut in6_addr,
    pub ula_addr: *mut in6_addr,
    pub xid: u32,
    pub fqdn_flags: u32,
    pub iaid: u32,
    pub iface_name: *mut c_char,
    pub packet_options: *mut c_void,
    pub end: *mut c_void,
    pub tags: *mut dhcp_netid,
    pub context_tags: *mut dhcp_netid,
    pub mac: [u8; DHCP_CHADDR_MAX],
    pub mac_len: u32,
    pub mac_type: u32,
}
