use libc::in6_addr;

#[derive(Default, Debug, Clone)]
pub struct prefix_opt {
    pub opt_type: u8,
    pub len: u8,
    pub prefix_len: u8,
    pub flags: u8,
    pub valid_lifetime: u32,
    pub preferred_lifetime: u32,
    pub reserved: u32,
    pub prefix: in6_addr,
}
