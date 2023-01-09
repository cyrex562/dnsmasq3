use libc::c_char;
use crate::addrlist::addrlist;

#[derive(Default, Debug, Clone)]
pub struct interface_name {
    // char *name; /* domain name */
    pub name: *mut c_char,
    // char *intr; /* interface name */
    pub intr: *mut c_char,
    flags: i32,
    pub proto4: in_addr,
    pub proto6: in6_addr,
    addr: *mut addrlist,
    // struct interface_name *next;
}
