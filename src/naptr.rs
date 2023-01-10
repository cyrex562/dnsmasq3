use std::ptr::null_mut;
use libc::c_char;

#[derive(Debug, Clone)]
pub struct naptr {
    pub name: *mut c_char,
    pub replace: *mut c_char,
    pub regexp: *mut c_char,
    pub services: *mut c_char,
    pub flags: *mut c_char,
    pub order: u32,
    pub pref: u32,
    // next
}

impl Default for naptr {
    fn default() -> Self {
        Self {
            name: null_mut(),
            replace: null_mut(),
            regexp: null_mut(),
            services: null_mut(),
            flags: null_mut(),
            order: 0,
            pref: 0,
        }
    }
}
