#[derive(Default, Debug, Clone)]
pub struct dhcp6_iaaddr_option {
    // type: u16;
    pub opt_type: u16,
    // len: u16;
    pub len: u16,
    // ip: in6_addr;
    pub ip: in6_addr,
    // preferred_lifetime: u32;
    pub preferred_lifetime: u32,
    // valid_lifetime: u32;
    pub valid_lifetime: u32,
}
