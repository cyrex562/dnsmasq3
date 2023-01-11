use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;
use libc::c_char;
use crate::dhcp_netid::DhcpNetid;

#[derive(Default,Debug,Clone)]
pub struct DhcpMatchName {
    pub name: String,
    pub wildcard: i32,
    pub netid: Vec<DhcpNetid>,
    // next
}
