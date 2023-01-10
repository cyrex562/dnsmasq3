#[derive(Default, Debug, Clone)]
pub struct ra_packet {
    pub pkt_type: u8,
    pub code: u8,
    pub checksum: u16,
    pub hop_limit: u8,
    pub flags: u8,
    pub lifetime: u16,
    pub reachable_time: u32,
    pub retrans_time: u32,
}