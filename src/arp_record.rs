#[derive(Default,Debug,Clone)]
pub struct ArpRecord {
  // unsigned short hwlen, status;
  hwlen: u16,
  status: u16,
  family: i32,
  // unsigned char hwaddr[DHCP_CHADDR_MAX];
  hwaddr: [u8;DHCP_CHADDR_MAX],
  // union all_addr addr;
  addr: all_addr,
  // struct arp_record *next;
}
