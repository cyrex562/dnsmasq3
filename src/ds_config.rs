use std::ptr::null_mut;
use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct DsConfig {
    // char *name, *digest;
    pub name: String,
    pub digest: String,
    // int digestlen, class, algo, keytag, digest_type;
    pub digestlen: i32,
    pub class: i32,
    pub alog: i32,
    pub keytag: i32,
    pub digest_type: i32,
    // struct ds_config *next;
}
