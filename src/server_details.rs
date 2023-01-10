use std::ptr::null_mut;
use libc::{addrinfo, c_char};
use crate::mysockaddr::mysockaddr;
use crate::server::server;

#[derive(Debug, Clone)]
pub struct server_details {
    pub addr: *mut mysockaddr,
    pub source_addr: *mut mysockaddr,
    pub hostinfo: *mut addrinfo,
    pub orig_hostinfo: *mut addrinfo,
    pub interface: *mut c_char,
    pub source: *mut c_char,
    pub scope_id: *mut c_char,
    pub interface_opt: *mut c_char,
    pub serv_port: i32,
    pub source_port: i32,
    pub addr_type: i32,
    pub scope_index: i32,
    pub valid: i32,
    pub flags: *mut u16,
}

impl Default for server_details {
    fn default() -> Self {
        Self {
            addr: null_mut(),
            source_addr: null_mut(),
            hostinfo: null_mut(),
            orig_hostinfo: null_mut(),
            interface: null_mut(),
            source: null_mut(),
            scope_id: null_mut(),
            interface_opt: null_mut(),
            serv_port: 0,
            source_port: 0,
            addr_type: 0,
            scope_index: 0,
            valid: 0,
            flags: null_mut(),
        }
    }
}
