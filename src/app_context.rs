use std::os::unix::raw::pid_t;
use libc::{c_char, iovec, nfds_t, pollfd, time_t};
use crate::arp_record::arp_record;
use crate::daemon::daemon;

#[derive(Default, Debug,Clone)]
pub struct AppContext {
    pub arps: Vec<arp_record>,
    pub last: time_t,
    pub packet_count: u32,
    // pub lua: *mut lua_State,
    pub inotify_buffer: *mut c_char,
    pub iov: iovec,
    pub netlink_pid: u32,
    pub nft_ctx: *mut nft_ctx,
    pub mem_reover: i32,
    pub outpacket_counter: usize,
    pub pollfds: *mut pollfd,
    pub nfds: nfds_t,
    pub arrsize: nfds_t,
    pub hop_limit: i32,
    pub daemon: daemon,
    pub pid: pid_t,
    pub pipewrite: i32,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            arps: vec![],
            last: 0,
            ..Default::default()
        }
    }
}
