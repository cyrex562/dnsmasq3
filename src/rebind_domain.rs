use libc::c_char;

#[derive(Debug, Clone, Default)]
pub struct RebindDomain {
    // char *domain;
    pub domain: String,
    // struct rebind_domain *next;
}
