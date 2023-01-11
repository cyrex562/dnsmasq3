use std::ptr::null_mut;
use libc::c_char;
use crate::hosts_file::HostsFile;

#[derive(Default, Debug, Clone)]
pub struct DynDir {
    // struct dyndir *next;
    // struct hostsfile *files;
    pub files: Vec<HostsFile>,
    pub flags: i32,
    // char *dname;
    pub dname: String,
    // #ifdef HAVE_INOTIFY
    pub wd: i32,
    /* inotify watch descriptor */
// #endif
}
