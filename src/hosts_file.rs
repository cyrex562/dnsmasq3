use std::ptr::null_mut;
use libc::c_char;

#[derive(Debug, Clone)]
pub struct hostsfile {
    // struct hostsfile *next;
    pub flags: i32,
    // char *fname;
    pub fname: *mut c_char,
    // unsigned index: i32; /* matches to cache entries for logging */
    pub index: u32,
}

impl Default for hostsfile {
    fn default() -> Self {
        Self {
            flags: 0,
            fname: null_mut(),
            index: 0,
        }
    }
}
