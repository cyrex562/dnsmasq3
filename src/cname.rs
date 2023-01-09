use libc::c_char;

#[derive(Default,Debug,Clone)]
pub struct cname {
  // int ttl, flag;
  pub ttl: i32,
  pub flag: i32,
  // char *alias, *target;
  pub alias: *mut c_char,
  pub target: *mut c_char,
  // struct cname *next, *targetp;
}
