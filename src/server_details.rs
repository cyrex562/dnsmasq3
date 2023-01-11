use std::ptr::null_mut;
use libc::{addrinfo, c_char};
use crate::mysockaddr::MySockAddr;
use crate::server::Server;

#[derive(Debug, Clone)]
pub struct server_details {
    pub addr: *mut MySockAddr,
    pub source_addr: *mut MySockAddr,
    pub hostinfo: *mut addrinfo,
    pub orig_hostinfo: *mut addrinfo,
    pub interface: String,
    pub source: String,
    pub scope_id: String,
    pub interface_opt: String,
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
