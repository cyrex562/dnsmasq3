#[derive(Default,Debug,Clone)]
pub struct iface_param {
  // struct dhcp_context *current;
  pub current: *mut dhcp_context,
  pub ind: i32,
    pub fallback: in6_addr,
    pub ll_addr: in6_addr,
    pub ula_addr: in6_addr,
    pub addr_match: i32,
}
