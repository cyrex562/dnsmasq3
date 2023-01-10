use std::ptr::null_mut;
use crate::dhcp_netid::dhcp_netid;
use crate::dhcp_netid_list::dhcp_netid_list;

#[derive(Debug, Clone)]
pub struct tag_if {
    pub list_set: *mut dhcp_netid_list,
    pub tag: *mut dhcp_netid,
    // next
}

impl Default for tag_if {
    fn default() -> Self {
        Self {
            list_set: null_mut(),
            tag: null_mut(),
        }
    }
}
