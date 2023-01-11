use libc::c_char;

#[derive(Default,Debug,Clone)]
pub struct serv_addr4 {
  // u16 flags, domain_len;
  pub flags: u16,
  pub domain_len: u16,
  // char *domain;
  pub domain: String,
  // struct server *next;
  pub addr: in_addr,
}
