#[derive(Default, Debug, Clone)]
pub struct ping_packet {
    pub ping_type: u8,
    pub code: u8,
    pub checksum: u16,
    pub identifier: u16,
    pub sequence_num: u16,

}