use std::net::IpAddr;
use crate::mysockaddr::MySockAddr;
/* subnet parameters from command line */
#[derive(Default, Debug, Clone)]
pub struct MySubnet {
    // union mysockaddr addr;
    pub addr: IpAddr,
    // addr_used: i32;
    pub addr_used: i32,
    // mask: i32;
    pub mask: i32,
}
