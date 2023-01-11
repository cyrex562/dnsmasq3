use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use libc::{c_char, in6_addr, in_addr, time_t};
use crate::dhcp_constants::DHCP_CHADDR_MAX;
use crate::util::{hex_string_from_byte_slice, in6addr_to_string, inaddr_to_string};

#[derive(Clone)]
pub struct slaac_address {
    pub addr: in6_addr,
    pub ping_time: time_t,
    pub backoff: i32,
}

impl Default for slaac_address {
    fn default() -> Self {
        Self {
            addr: in6_addr { s6_addr: [0; 16] },
            ping_time: 0,
            backoff: 0,
        }
    }
}

impl Debug for slaac_address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{addr: {}, ping_time: {}, backoff: {}}}", in6addr_to_string(&self.addr), self.ping_time, self.backoff)
    }
}

#[derive(Clone)]
pub struct dhcp_lease {
    pub clid_len: i32,
    /* length of client identifier */
    pub clid: *mut u8,
    pub hostname: String,
    pub fqdn: String,
    pub old_hostname: String,
    pub flags: i32,
    pub expires: time_t,
    pub length: i32,
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub hwaddr: [u8; DHCP_CHADDR_MAX as usize],
    pub addr: in_addr,
    pub extradata: *mut u8,
    pub extradata_len: u32,
    pub extradata_size: u32,
    pub last_interface: i32,
    pub new_interface: i32,
    pub new_prefixlen: i32,
    pub addr6: in6_addr,
    pub iaid: u32,
    pub slaac_address: *mut slaac_address,
    pub vendorclass_count: i32,
}

impl Default for dhcp_lease {
    fn default() -> Self {
        Self {
            clid_len: 0,
            clid: null_mut(),
            hostname: null_mut(),
            fqdn: null_mut(),
            old_hostname: null_mut(),
            flags: 0,
            expires: 0,
            length: 0,
            hwaddr_len: 0,
            hwaddr_type: 0,
            hwaddr: [0;DHCP_CHADDR_MAX],
            addr: in_addr{ s_addr: 0 },
            extradata: null_mut(),
            extradata_len: 0,
            extradata_size: 0,
            last_interface: 0,
            new_interface: 0,
            new_prefixlen: 0,
            addr6: in6_addr { s6_addr: [0;16] },
            iaid: 0,
            slaac_address: null_mut(),
            vendorclass_count: 0,
        }
    }
}

impl Debug for dhcp_lease {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{clid_len: {}, clid: {:?}, hostname: {:?}, fqdn: {:?}, old_hostname: {:?}, flags: {}, expires: {}, length: {}, hwaddr len: {}, hwaddr type: {}, hwaddr: {}, addr: {}, extradata: {:?}, extradata len: {}, extradata size: {}, last interface: {}, new interface: {}, new prefix len: {}, addr6: {}, iaid: {}, slaac_address: {:?}, vendorclass count: {}",
        self.clid_len, self.clid, self.hostname, self.fqdn, self.old_hostname, self.flags, self.expires, self.length, self.hwaddr_len, self.hwaddr_type, hex_string_from_byte_slice(&self.hwaddr), inaddr_to_string(&self.addr), self.extradata, self.extradata_len, self.extradata_size, self.last_interface, self.new_interface, self.new_prefixlen, in6addr_to_string(&self.addr6), self.iaid, self.slaac_address, self.vendorclass_count)
    }
}
