use std::ptr::null_mut;
use crate::dhcp_netid::dhcp_netid;

#[derive(Debug, Clone)]
pub struct delay_config {
    pub delay: i32,
    pub netid: *mut dhcp_netid,
    // next
}

impl Default for delay_config {
    fn default() -> Self {
        Self {
            delay: 0,
            netid: null_mut(),
        }
    }
}
