#[derive(Default, Debug, Clone)]
pub struct dhcp6_iana_option {
    pub opt_type: u16,
    pub len: u16,
    pub iaid: u32,
    pub t1: u32,
    pub t2: u32,
    pub options: [u8; 1024],
};
