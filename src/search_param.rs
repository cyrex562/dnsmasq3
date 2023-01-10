use libc::{c_char, IF_NAMESIZE, time_t};

#[derive(Default, Debug, Clone)]
pub struct search_param {
    pub now: time_t,
    pub iface: i32,
    pub name: [c_char; IF_NAMESIZE + 1],
}