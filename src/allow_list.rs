use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct allowlist {
    // u32 mark, mask;
    pub mask: u32,
    pub mark: u32,
    // char **patterns;
    pub patterns: *mut *mut c_char,
    // struct allowlist *next;
}
