use libc::c_char;

#[derive(Debug, Clone, Default)]
pub struct rebind_domain {
    // char *domain;
    pub domain: *mut c_char,
    // struct rebind_domain *next;
}
