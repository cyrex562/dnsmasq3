use std::ptr::null_mut;
use crate::dhcp_netid::DhcpNetid;

#[derive(Default, Debug, Clone)]
pub struct DhcpNetidList {
    pub list: Vec<DhcpNetid>,
    // next
}
