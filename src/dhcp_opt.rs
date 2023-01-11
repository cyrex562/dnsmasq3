use std::fmt::{Debug, Formatter};
use std::ptr::{null, null_mut};
use crate::dhcp_netid::DhcpNetid;

#[derive(Default, Debug, Clone)]
pub struct DhcpOptU {
    pub encap: i32,
    pub wildcard_mask: i32,
    pub vendor_class: Vec<u8>,
}

#[derive(Default, Debug, Clone)]
pub struct DhcpOpt {
    pub opt: i32,
    pub len: i32,
    pub flags: i32,
    pub u: DhcpOptU,
    pub val: Vec<u8>,
    pub netid: Vec<DhcpNetid>,
    // next
}
