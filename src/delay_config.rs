use std::ptr::null_mut;
use crate::dhcp_netid::DhcpNetid;

#[derive(Default, Debug, Clone)]
pub struct DelayConfig {
    pub delay: i32,
    pub netid:Vec<DhcpNetid>,
    // next
}
