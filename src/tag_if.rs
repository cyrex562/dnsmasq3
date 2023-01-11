use std::ptr::null_mut;
use crate::dhcp_netid::DhcpNetid;
use crate::dhcp_netid_list::DhcpNetidList;

#[derive(Default, Debug, Clone)]
pub struct TagIf {
    pub list_set: Vec<DhcpNetidList>,
    pub tag: Vec<DhcpNetid>,
    // next
}
