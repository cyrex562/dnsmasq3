use std::fmt::{Debug, Formatter};
use libc::{c_char, IF_NAMESIZE, in6_addr, in_addr, off_t, time_t};
use crate::dhcp_constants::DHCP_CHADDR_MAX;
use crate::util::{hex_string_from_byte_slice, hex_string_from_c_char_slice, in6addr_to_string, inaddr_to_string};

#[derive(Clone)]
pub struct script_data {
    pub flags: i32,
    pub action: i32,
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub clid_len: i32,
    pub hostname_len: i32,
    pub ed_len: i32,
    pub addr: in_addr,
    pub giaddr: in_addr,
    pub remaining_time: u32,
    pub length: u32,
    pub expires: time_t,
    pub file_len: off_t,
    pub addr6: in6_addr,
    pub vendorclass_count: i32,
    pub iaid: u32,
    pub hwaddr: [u8; DHCP_CHADDR_MAX],
    pub interface: [c_char; IF_NAMESIZE],
}

impl Default for script_data {
    fn default() -> Self {
        Self {
            flags: 0,
            action: 0,
            hwaddr_len: 0,
            hwaddr_type: 0,
            clid_len: 0,
            hostname_len: 0,
            ed_len: 0,
            addr: in_addr { s_addr: 0 },
            giaddr: in_addr { s_addr: 0 },
            remaining_time: 0,
            length: 0,
            expires: 0,
            file_len: 0,
            addr6: in6_addr { s6_addr: [0; 16] },
            vendorclass_count: 0,
            iaid: 0,
            hwaddr: [0; DHCP_CHADDR_MAX],
            interface: [0; IF_NAMESIZE],
        }
    }
}

impl Debug for script_data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ flags: {}, action: {}, hwaddr_len: {}, hwaddr_type: {}, clid_len: {}, hostname_len: {}, ed_len: {}, addr: {}, giaddr: {}, remaining_time: {}, length: {}, expires: {}, file_len: {}, addr6: {}, vendorclass_count: {}, iaid: {}, hwaddr: {}, interface: {} }}", self.flags, self.action, self.hwaddr_len, self.hwaddr_type, self.clid_len, self.hostname_len, self.ed_len, inaddr_to_string(&self.addr), inaddr_to_string(&self.giaddr), self.remaining_time, self.length, self.expires, self.file_len, in6addr_to_string(&self.addr6), self.vendorclass_count, self.iaid, hex_string_from_byte_slice(&self.hwaddr), hex_string_from_c_char_slice(&self.interface))
    }
}
