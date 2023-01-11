use std::ptr::null_mut;
use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct HostsFile {
    // struct hostsfile *next;
    pub flags: i32,
    // char *fname;
    pub fname: String,
    // unsigned index: i32; /* matches to cache entries for logging */
    pub index: u32,
}
