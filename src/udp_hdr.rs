#[derive(Default, Debug, Clone)]
pub struct udphdr {
    // u16 uh_sport;               /* source port */
    pub uh_sport: u16,
    // u16 uh_dport;               /* destination port */
    pub uh_dport: u16,
    // u16 uh_ulen;                /* udp length */
    pub uh_ulen: u16,
    // u16 uh_sum;                 /* udp checksum */
    pub uh_sum: u16,
}
// udp;
