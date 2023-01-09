use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct ipsets {
    // char **sets;
    pub sets: *mut *mut c_char,
    // char *domain;
    pub domain: *mut c_char,
    // struct ipsets *next;
}
