use libc::c_uchar;
use crate::config::KEYBLOCK_LEN;

#[derive(Debug, Clone)]
pub struct blockdata {
    // struct blockdata *next;
    // unsigned char key[KEYBLOCK_LEN];
    pub key: [c_uchar; KEYBLOCK_LEN as usize],
}

impl Default for blockdata {
    fn default() -> Self {
        Self {
            key: [0; KEYBLOCK_LEN as usize]
        }
    }
}
