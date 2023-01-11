use std::fmt::{Debug, Formatter};
use std::net::IpAddr;
use std::ptr::null_mut;
use libc::{c_char, in_addr};
use crate::dhcp_netid::DhcpNetid;
use crate::util::inaddr_to_string;

#[derive(Clone)]
pub struct DhcpBoot {
    pub file: String,
    pub sname: String,
    pub tftp_sname: String,
    pub next_server: IpAddr,
    pub netid: Vec<hcpNetid>,
    // next
}
