use std::fmt::{Debug, Formatter};
use std::ptr::{null, null_mut};
use crate::dhcp_netid::dhcp_netid;

pub union DhcpOptU {
    pub encap: i32,
    pub wildcard_mask: i32,
    pub vendor_class: *mut u8,
}

impl Default for DhcpOptU {
    fn default() -> Self {
        Self {
            encap: 0
        }
    }
}

impl Debug for DhcpOptU {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{encap: {}, wildcard_mask: {}, vendor_clas: {:?}}}", self.encap, self.wildcard_mask, self.vendor_class)
    }
}

impl Clone for DhcpOptU {
    fn clone(&self) -> Self {
        Self {
            encap: self.encap
        }
    }
}

#[derive(Clone)]
pub struct dhcp_opt {
  pub opt: i32,
    pub len: i32,
    pub flags: i32,
    pub u: DhcpOptU,
    pub val: *mut u8,
    pub netid: *mut dhcp_netid,
    // next
}


impl Default for dhcp_opt {
    fn default() -> Self {
        Self {
            opt: 0,
            len: 0,
            flags: 0,
            u: Default::default(),
            val: null_mut(),
            netid: null_mut(),
        }
    }
}

impl Debug for dhcp_opt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ opt: {}, len: {}, flags: {}, u: {:?}, val: {:?}, netid: {:?} }}", self.opt, self.len, self.flags, self.u, self.val, self.netid)
    }
}
