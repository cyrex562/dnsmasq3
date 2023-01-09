#[derive(Default, Debug, Clone)]
pub struct dhcp6_packet {
    pub len: usize,
    pub buf: [u8; 2048],
};
