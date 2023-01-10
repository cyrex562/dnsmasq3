/* struct sockaddr is not large enough to hold any address,
   and specifically not big enough to hold an IPv6 address.
   Blech. Roll our own. */
use std::fmt::{Debug, Formatter};
use libc::{in6_addr, sockaddr, sockaddr_in, sockaddr_in6};
use crate::util::{hex_string_from_byte_slice, hex_string_from_c_char_slice, sockaddr_to_string, sockaddrin6_to_string, sockaddrin_to_string};

#[derive()]
pub union mysockaddr {
    // struct sockaddr sa;
    pub sa: sockaddr,
    // struct sockaddr_in in;
    pub sa_in: sockaddr_in,
    // struct sockaddr_in6 in6;
    pub sa_in6: sockaddr_in6,
}


impl Default for mysockaddr {
    fn default() -> Self {
        Self {
            sa_in6: sockaddr_in6 {
                sin6_family: 0,
                sin6_port: 0,
                sin6_flowinfo: 0,
                sin6_addr: in6_addr { s6_addr: [0; 16] },
                sin6_scope_id: 0,
            }
        }
    }
}


impl Debug for mysockaddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{sa: {}, sa_in: {}, sa_in6: {}}}", sockaddr_to_string(&self.sa), sockaddrin_to_string(&self.sa_in), sockaddrin6_to_string(&self.sa_in6))
    }
}

impl Clone for mysockaddr {
    fn clone(&self) -> Self {
        Self {
            sa_in6: self.sa_in6.clone()
        }
    }
}
