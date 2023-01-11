use std::fmt::{Debug, Formatter};
use std::net::IpAddr;
use std::ptr::null_mut;
use libc::{c_char, in_addr};
use crate::dhcp_netid::DhcpNetid;
use crate::util::inaddr_to_string;

#[derive(Default,Debug,Clone)]
pub struct PxeService {
    pub CSA: u16,
    pub svc_type: u16,
    pub menu: String,
    pub basename: String,
    pub sname: String,
    pub server: IpAddr,
    pub netid: Vec<DhcpNetid>,
    // next
}
