pub struct dhcp_packet {
  u8 op, htype, hlen, hops;
  u32 xid;
  u16 secs, flags;
  struct in_addr ciaddr, yiaddr, siaddr, giaddr;
  u8 chaddr[DHCP_CHADDR_MAX], sname[64], file[128];
  u32 cookie;
  unsigned char options[308];
}
