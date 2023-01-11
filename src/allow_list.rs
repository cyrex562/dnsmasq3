use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct AllowList {
    // u32 mark, mask;
    pub mask: u32,
    pub mark: u32,
    // char **patterns;
    pub patterns: Vec<String>,
    // struct allowlist *next;
}
