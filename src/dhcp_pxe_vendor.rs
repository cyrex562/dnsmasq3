use std::ptr::null_mut;
use libc::c_char;

#[derive(Debug, Clone)]
pub struct dhcp_pxe_vendor {
    pub data: *mut c_char,
    // next
}

impl Default for dhcp_pxe_vendor {
    fn default() -> Self {
        Self {
            data: null_mut()
        }
    }
}
