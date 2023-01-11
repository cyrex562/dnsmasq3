/* resolv-file parms from command-line */
use libc::{c_char, ino_t, time_t};

#[derive(Default, Debug, Clone)]
pub struct Resolvc {
    // struct resolvc *next;
    pub is_default: i32,
    pub logged: i32,
    pub mtime: time_t,
    pub ino: ino_t,
    pub name: String,
    pub wd: i32,
    pub file: String,
}
