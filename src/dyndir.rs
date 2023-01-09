use libc::c_char;
use crate::hosts_file::hostsfile;

#[derive(Default, Debug, Clone)]
pub struct dyndir {
    // struct dyndir *next;
    // struct hostsfile *files;
    pub files: *mut hostsfile,
    pub flags: i32,
    // char *dname;
    pub dname: *mut c_char,
    // #ifdef HAVE_INOTIFY
    pub wd: i32,
    /* inotify watch descriptor */
// #endif
}
