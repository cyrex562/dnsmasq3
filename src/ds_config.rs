use std::ptr::null_mut;
use libc::c_char;

#[derive(Debug, Clone)]
pub struct ds_config {
    // char *name, *digest;
    pub name: *mut c_char,
    pub digest: *mut c_char,
    // int digestlen, class, algo, keytag, digest_type;
    pub digestlen: i32,
    pub class: i32,
    pub alog: i32,
    pub keytag: i32,
    pub digest_type: i32,
    // struct ds_config *next;
}

impl Default for ds_config {
    fn default() -> Self {
        Self {
            name: null_mut(),
            digest: null_mut(),
            digestlen: 0,
            class: 0,
            alog: 0,
            keytag: 0,
            digest_type: 0,
        }
    }
}
