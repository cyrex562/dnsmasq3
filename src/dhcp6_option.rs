#[derive(Default, Debug, Clone)]
pub struct dhcp6_option {
    // type: u16;
    pub opt_type: u16,
    // len: u16;
    pub len: u16,
    // char  value[1024];
    pub val: [u8; 1024],
}
