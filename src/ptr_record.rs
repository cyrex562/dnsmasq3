use libc::c_char;

#[derive(Default,Debug,Clone)]
pub struct ptr_record {
  // char *name, *ptr;
  pub name: *mut c_char,
  pub ptr: *mut c_char,
  // struct ptr_record *next;
}
