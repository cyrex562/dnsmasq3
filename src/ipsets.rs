use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct IpSets {
    // char **sets;
    pub sets: Vec<String>,
    // char *domain;
    pub domain: String,
    // struct ipsets *next;
}
