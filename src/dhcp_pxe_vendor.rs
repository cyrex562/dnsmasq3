use std::ptr::null_mut;
use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct DhcpPxeVendor {
    pub data: String,
    // next
}
