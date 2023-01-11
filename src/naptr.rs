use std::ptr::null_mut;
use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct NaPtr {
    pub name: String,
    pub replace: String,
    pub regexp: String,
    pub services: String,
    pub flags: String,
    pub order: u32,
    pub pref: u32,
    // next
}
