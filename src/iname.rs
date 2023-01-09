/* interface and address parms from command line. */
use libc::c_char;
use crate::mysockaddr::mysockaddr;

#[derive(Default, Debug, Clone)]
pub struct iname {
    // char *name;
    pub name: *mut c_char,
    // union mysockaddr addr;
    pub addr: mysockaddr,
    // used: i32;
    pub used: i32,
    // struct iname *next;
}
