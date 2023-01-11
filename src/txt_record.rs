use libc::{c_char, c_uchar};

#[derive(Default,Debug,Clone)]
pub struct TxtRecord {
  pub name: String,
  pub txt: Vec<u8>
  pub class: u16,
  pub len: u16,
  pub stat: i32,
    // struct txt_record *next;
}
