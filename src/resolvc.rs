/* resolv-file parms from command-line */
use libc::{c_char, ino_t, time_t};

#[derive(Default, Debug, Clone)]
pub struct resolvc {
    // struct resolvc *next;
    // int is_default, logged;
    pub is_default: i32,
    pub logged: i32,
    // time_t mtime;
    pub mtime: time_t,
    // ino_t ino;
    pub ino: ino_t,
    // char *name;
    pub name: *mut c_char,
    // #ifdef HAVE_INOTIFY
    pub wd: i32,
    /* inotify watch descriptor */
    // char *file; /* pointer to file part if path */
    pub file: *mut c_char,
// #endif
}
