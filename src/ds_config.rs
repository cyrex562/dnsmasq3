use libc::c_char;

#[derive(Default,Debug,Clone)]
pub struct ds_config {
  // char *name, *digest;
  pub name: *mut c_char,
  pub digest: *mut c_char
  // int digestlen, class, algo, keytag, digest_type;
  pub digestlen: i32,
    pub class: i32,
    pub alog: i32,
    pub keytag: i32,
    pub digest_type: i32,
    // struct ds_config *next;
}
