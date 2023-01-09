use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct hostsfile {
    // struct hostsfile *next;
    pub flags: i32,
    // char *fname;
    pub fname: *mut c_char,
    // unsigned index: i32; /* matches to cache entries for logging */
    pub index: u32,
}
