pub enum AddrType {
    Ipv4Addr,
    MacAddr,
    Ipv6Addr,
}

pub struct DrAddr {
    pub addr: [u8;16],
    pub addr_type: AddrType,
}