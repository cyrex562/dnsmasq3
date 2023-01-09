use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct serv_addr6 {
    // u16 flags, domain_len;
    pub flags: u16,
    pub domain_len: u16,
    // char *domain;
    pub domain: *mut c_char,
    // struct server *next;
    pub addr: in6_addr,
}
