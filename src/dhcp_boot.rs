use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use libc::{c_char, in_addr};
use crate::dhcp_netid::dhcp_netid;
use crate::util::inaddr_to_string;

#[derive(Clone)]
pub struct dhcp_boot {
    pub file: *mut c_char,
    pub sname: *mut c_char,
    pub tftp_sname: *mut c_char,
    pub next_server: in_addr,
    pub netid: *mut dhcp_netid,
    // next
}

impl Default for dhcp_boot {
    fn default() -> Self {
        Self {
            file: null_mut(),
            sname: null_mut(),
            tftp_sname: null_mut(),
            next_server: in_addr { s_addr: 0 },
            netid: null_mut(),
        }
    }
}

impl Debug for dhcp_boot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ file: {:?}, sname: {:?}, tftp_sname: {:?}, next_server: {:?}, netid: {:?} }}", self.file, self.sname, self.tftp_sname, inaddr_to_string(&self.next_server), self.netid)
    }
}
