/* interface and address parms from command line. */
use std::net::IpAddr;
use std::ptr::null_mut;
use libc::c_char;
use crate::mysockaddr::MySockAddr;

#[derive(Default, Debug, Clone)]
pub struct Iname {
    // char *name;
    pub name: String,
    // union mysockaddr addr;
    pub addr: IpAddr,
    // used: i32;
    pub used: i32,
    // struct iname *next;
}
