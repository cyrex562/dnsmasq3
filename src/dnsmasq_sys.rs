//
// Created by JoshMadden on 9/24/2019.
//
use libc::c_char;
use libc::c_int;
use std::ffi::{c_void};

#[cfg(target_os="windows")]
pub struct iovec {
    pub iov_base: *const c_void,// starting address
    pub iov_len: usize, // number of bytes to transfer
}

// typedef unsigned int uid_t;
#[cfg(target_os="windows")]
pub type uid_t = u32;
// typedef unsigned int gid_t;
#[cfg(target_os="windows")]
pub type gid_t = u32;
// typedef uint32_t in_addr_t;
#[cfg(target_os="windows")]
pub type in_addr_t = u32;

#[cfg(target_os="windows")]
pub type time_t = i64;

#[cfg(target_os="windows")]
pub type off_t = i64;

#[cfg(target_os="windows")]
pub type dev_t = u64;

#[cfg(target_os="windows")]
pub type ino_t = u64;

#[cfg(target_os="windows")]
pub type pid_t = i32;

#[cfg(target_os="windows")]



#[cfg(target_os="windows")]
pub const LOG_DAEMON: c_int = 3 << 3;
#[cfg(target_os="windows")]
pub const LOG_USER: c_int = 1 << 3;
#[cfg(target_os="windows")]
pub const LOG_MAIL: c_int = 2 << 3;

#[cfg(target_os="windows")]
pub type sa_family_t = u16;

#[cfg(target_os="windows")]
pub type in_port_t = u16;

#[cfg(target_os="windows")]
#[derive(Debug,Clone, Copy, Default)]
pub struct InAddr {
    pub s_addr: in_addr_t
}

#[cfg(target_os="windows")]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct In6Addr {
    pub s6_addr: [u8;16]
}

#[cfg(target_os="windows")]
pub struct Sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char;14],
    pub sa_len: usize,
}

#[cfg(target_os="windows")]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: InAddr,
    pub sin_zero: [u8;8]
}

#[cfg(target_os="windows")]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: In6Addr,
    pub sin6_scope_id: u32,
}

