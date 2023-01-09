use crate::dhcp_constants::DHCP_CHADDR_MAX;

#[derive(Default, Debug, Clone)]
pub struct dhcp_packet {
    // u8 op, htype, hlen, hops;
    pub op: u8,
    pub htype: u8,
    pub hlen: u8,
    pub hops: u8,
    // u32 xid;
    pub xid: u32,
    // u16 secs, flags;
    pub secs: u16,
    pub flags: u16,
    // struct in_addr ciaddr, yiaddr, siaddr, giaddr;
    pub ciaddr: in_addr,
    pub yiaddr: in_addr,
    pub siaddr: in_addr,
    pub giaddr: in_addr,
    // u8 chaddr[DHCP_CHADDR_MAX], sname[64], file[128];
    pub chaddr: [u8; DHCP_CHADDR_MAX],
    pub sname: [u8; 64],
    pub file: [u8; 128],
    // u32 cookie;
    pub cookie: u32,
    pub options: [u8; 312],
    // unsigned options: [u8;308]
}
