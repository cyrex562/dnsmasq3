use std::net::IpAddr;
use std::ptr::null_mut;
use libc::{c_char, off_t, time_t};
use crate::all_addr::AllAddr;
use crate::mysockaddr::MySockAddr;
use crate::tftp_file::TftpFile;

#[derive(Default, Debug, Clone)]
pub struct TftpTransfer {
    pub sockfd: i32,
    pub timeout: time_t,
    pub backoff: i32,
    pub block: u32,
    pub blocksize: u32,
    pub expansion: u32,
    pub offset: off_t,
    pub peer: IpAddr,
    pub source: AllAddr,
    pub if_index: i32,
    pub opt_blocksize: c_char,
    pub opt_transize: c_char,
    pub netascii: c_char,
    pub carrylf: c_char,
    pub file: Vec<TftpFile>,
    // next
}
