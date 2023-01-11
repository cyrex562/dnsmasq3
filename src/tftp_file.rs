use std::ptr::null_mut;
use libc::{c_char, dev_t, ino_t, off_t};

#[derive(Debug, Clone)]
pub struct TftpFile {
    pub refcount: i32,
    pub fd: i32,
    pub size: off_t,
    pub dev: dev_t,
    pub inode: ino_t,
    pub filename: String,
}

impl Default for TftpFile {
    fn default() -> Self {
        Self {
            refcount: 0,
            fd: 0,
            size: 0,
            dev: 0,
            inode: 0,
            filename: null_mut(),
        }
    }
}
