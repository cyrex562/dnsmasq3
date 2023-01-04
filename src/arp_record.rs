#[derive(Default,Debug,Clone)]
pub struct arp_record {
//   unsigned short hwlen, status,
pub hwlen: u16,
pub status: u16,
// family: i32,
pub family: i32,
//   unsigned char hwaddr[DHCP_CHADDR_MAX],
pub hwaddr: [u8; DHCP_CHADDR_MAX],
//   union all_addr addr,
pub addr: all_addr,
// struct arp_record *next,
}
