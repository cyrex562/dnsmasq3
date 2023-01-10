use std::ptr::{null, null_mut};
use libc::c_char;

#[derive(Debug, Clone)]
pub struct ptr_record {
    pub name: *mut c_char,
    pub ptr: *mut c_char,
}

impl Default for ptr_record {
    fn default() -> Self {
        Self {
            name: null_mut(),
            ptr: null_mut(),
        }
    }
}
