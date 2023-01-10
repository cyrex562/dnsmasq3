use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use libc::{c_char, in6_addr, in_addr};
use crate::addrlist::addrlist;
use crate::util::{in6addr_to_string, inaddr_to_string};

#[derive(Clone)]
pub struct cond_domain {
    pub domain: *mut c_char,
  pub prefix: *mut c_char,
  pub al: *mut addrlist,
  pub start: in_addr,
  pub end: in_addr,
  pub start6: in6_addr,
  pub end6: in6_addr,
  pub is6: i32,
  pub indexed: i32,
  pub prefixlen: i32,
  // next
}

impl Default for cond_domain {
    fn default() -> Self {
        Self {
            domain: null_mut(),
            prefix: null_mut(),
            al: null_mut(),
            start: in_addr{ s_addr: 0 },
            end: in_addr { s_addr: 0 },
            start6: in6_addr { s6_addr: [0;16] },
            end6: in6_addr { s6_addr: [0;16] },
            is6: 0,
            indexed: 0,
            prefixlen: 0,
        }
    }
}

impl Debug for cond_domain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ domain: {:?}, prefix: {:?}, al: {:?}, start: {}, end: {}, start6: {:?}, end6: {:?}, is6: {}, indexed: {}, prefixlen: {} }}", self.domain, self.prefix, self.al, inaddr_to_string(&self.start), inaddr_to_string(&self.end), in6addr_to_string(&self.start6), in6addr_to_string(&self.end6), self.is6, self.indexed, self.prefixlen)
    }
}
