use std::net::Ipv6Addr;

#[derive(Default,Debug,Clone)]
pub struct SnoopRecord {
    pub client: Ipv6Addr,
    pub prefix: Ipv6Addr,
    pub prefix_len: i32,
    // next
}
