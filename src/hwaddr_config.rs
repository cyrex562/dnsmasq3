use crate::dhcp_constants::DHCP_CHADDR_MAX;

#[derive(Debug, Clone)]
pub struct hwaddr_config {
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub hwaddr: [u8; DHCP_CHADDR_MAX],
    pub wildcard_mask: i32,
    // next
}

impl Default for hwaddr_config {
    fn default() -> Self {
        Self {
            hwaddr_len: 0,
            hwaddr_type: 0,
            hwaddr: [0; DHCP_CHADDR_MAX],
            wildcard_mask: 0,
        }
    }
}
