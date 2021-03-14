use crate::dnsmasq_sys::sa_family_t;

pub enum AddrType {
    Ipv4Addr,
    MacAddr,
    Ipv6Addr,
}

pub struct DrAddr {
    pub addr: [u8;16],
    pub addr_type: AddrType,
}

pub const AF_INET: sa_family_t = 2;