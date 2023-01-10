use std::ptr::null_mut;
use crate::dhcp_netid::dhcp_netid;

#[derive(Debug, Clone)]
pub struct dhcp_netid_list {
    pub list: *mut dhcp_netid,
    // next
}

impl Default for dhcp_netid_list {
    fn default() -> Self {
        Self {
            list: null_mut()
        }
    }
}
