use libc::{c_char, c_uchar};

#[derive(Default,Debug,Clone)]
pub struct txt_record {
  // char *name;
  pub name: *mut c_char,
  // unsigned char *txt;
  pub txt: *mut c_uchar,
  // unsigned short class, len;
  pub class: u16,
  pub len: u16,
  pub stat: i32,
  // struct txt_record *next;
}
