use std::ptr::null_mut;
use libc::c_char;

#[derive(Debug, Clone,Default)]
pub struct TftpPrefix {
    pub interface: String,
    pub prefix: String,
    pub missing: i32,
    // next
}
