use crate::dhcp_constants::DHCP_CHADDR_MAX;
use crate::dhcp_netid::dhcp_netid;

#[derive(Default, Debug, Clone)]
pub struct dhcp_mac {
    pub mask: u32,
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub hwaddr: [u8; DHCP_CHADDR_MAX],
    pub netid: dhcp_netid,
    // next
}