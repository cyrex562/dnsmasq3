use std::ptr::{null, null_mut};
use libc::c_char;

#[derive(Debug, Clone)]
pub struct ra_interface {
    pub name: *mut c_char,
    pub mut_name: *mut c_char,
    pub interval: i32,
    pub lifetime: i32,
    pub prio: i32,
    pub mtu: i32,
    // next
}

impl Default for ra_interface {
    fn default() -> Self {
        Self {
            name: null_mut(),
            mut_name: null_mut(),
            interval: 0,
            lifetime: 0,
            prio: 0,
            mtu: 0,
        }
    }
}
