/* interface and address parms from command line. */
use std::ptr::null_mut;
use libc::c_char;
use crate::mysockaddr::mysockaddr;

#[derive(Debug, Clone)]
pub struct iname {
    // char *name;
    pub name: *mut c_char,
    // union mysockaddr addr;
    pub addr: mysockaddr,
    // used: i32;
    pub used: i32,
    // struct iname *next;
}


impl Default for iname {
    fn default() -> Self {
        Self {
            name: null_mut(),
            addr: Default::default(),
            used: 0,
        }
    }
}
