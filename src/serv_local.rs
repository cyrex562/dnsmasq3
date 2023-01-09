use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct serv_local {
    // u16 flags, domain_len;
    pub flags: u16,
    pub domain_len: u16,
    // char *domain;
    domain: *mut c_char,
    // struct server *next;
}
