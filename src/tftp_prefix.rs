use std::ptr::null_mut;
use libc::c_char;

#[derive(Debug, Clone)]
pub struct tftp_prefix {
    pub interface: *mut c_char,
    pub prefix: *mut c_char,
    pub missing: i32,
    // next
}

impl Default for tftp_prefix {
    fn default() -> Self {
        Self {
            interface: null_mut(),
            prefix: null_mut(),
            missing: 0,
        }
    }
}
