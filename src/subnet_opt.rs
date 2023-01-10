use crate::dns_protocol::IN6ADDRSZ;

#[derive(Default,Debug,Clone)]
pub struct subnet_opt {
  pub family: u16,
    pub source_netmask: u8,
    pub scope_netmask: u8,
    pub addr: [u8;IN6ADDRSZ],
}
