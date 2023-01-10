use std::ptr::null_mut;
use libc::{c_char, off_t, time_t};
use crate::all_addr::all_addr;
use crate::mysockaddr::mysockaddr;
use crate::tftp_file::tftp_file;

#[derive(Debug, Clone)]
pub struct tftp_transfer {
    pub sockfd: i32,
    pub timeout: time_t,
    pub backoff: i32,
    pub block: u32,
    pub blocksize: u32,
    pub expansion: u32,
    pub offset: off_t,
    pub peer: mysockaddr,
    pub source: all_addr,
    pub if_index: i32,
    pub opt_blocksize: c_char,
    pub opt_transize: c_char,
    pub netascii: c_char,
    pub carrylf: c_char,
    pub file: *mut tftp_file,
    // next
}


impl Default for tftp_transfer {
    fn default() -> Self {
        Self {
            sockfd: 0,
            timeout: 0,
            backoff: 0,
            block: 0,
            blocksize: 0,
            expansion: 0,
            offset: 0,
            peer: Default::default(),
            source: Default::default(),
            if_index: 0,
            opt_blocksize: 0,
            opt_transize: 0,
            netascii: 0,
            carrylf: 0,
            file: null_mut(),
        }
    }
}
