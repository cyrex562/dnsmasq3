use libc::c_uchar;
use crate::config::KEYBLOCK_LEN;

#[derive(Default,Debug,Clone)]
pub struct blockdata {
  // struct blockdata *next;
  // unsigned char key[KEYBLOCK_LEN];
  pub key: [c_uchar; KEYBLOCK_LEN as usize]
}
