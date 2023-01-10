use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use libc::{c_char, in_addr};
use crate::dhcp_netid::dhcp_netid;
use crate::util::inaddr_to_string;

#[derive(Clone)]
pub struct pxe_service {
    pub CSA: u16,
    pub svc_type: u16,
    pub menu: *mut c_char,
    pub basename: *mut c_char,
    pub sname: *mut c_char,
    pub server: in_addr,
    pub netid: *mut dhcp_netid,
    // next
}

impl Default for pxe_service {
    fn default() -> Self {
        Self {
            CSA: 0,
            svc_type: 0,
            menu: null_mut(),
            basename: null_mut(),
            sname: null_mut(),
            server: in_addr { s_addr: 0 },
            netid: null_mut(),
        }
    }
}

impl Debug for pxe_service {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{CSA: {}, svc_type: {}, menu: {:?}, basename: {:?}, sname: {:?}, server: {}, netid: {:?} }}", self.CSA, self.svc_type, self.menu, self.basename, self.sname, inaddr_to_string(&self.server), self.netid)
    }
}
