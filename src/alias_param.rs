use std::ptr::{null, null_mut};
use crate::dhcp_bridge::DhcpBridge;

#[derive(Debug, Clone)]
pub struct alias_param {
    pub iface: i32,
    pub bridge: *mut DhcpBridge,
    pub num_alias_ifs: i32,
    pub max_alias_ifs: i32,
    pub alias_ifs: *mut i32,
}

impl Default for alias_param {
    fn default() -> Self {
        Self {
            iface: 0,
            bridge: null_mut(),
            num_alias_ifs: 0,
            max_alias_ifs: 0,
            alias_ifs: null_mut(),
        }
    }
}
