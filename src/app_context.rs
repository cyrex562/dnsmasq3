use std::os::unix::raw::pid_t;
use libc::{c_char, iovec, nfds_t, pollfd, time_t};
use crate::arp_record::ArpRecord;
use crate::daemon::Daemon;
use crate::surf_rand::SurfRandContext;

#[derive(Default, Debug,Clone)]
pub struct AppContext {
    pub arp_records: Vec<ArpRecord>,
    pub last: time_t,
    pub packet_count: usize,
    // pub lua: *mut lua_State,
    #[cfg(target_os = "linux")]
    pub inotify_buffer: Vec<u8>,
    #[cfg(target_os = "linux")]
    pub iov: iovec,
    #[cfg(target_os = "linux")]
    pub netlink_pid: u32,
    #[cfg(target_os = "linux")]
    pub nft_ctx: *mut nft_ctx,
    pub mem_reover: i32,
    pub outpacket_counter: usize,
    pub pollfds: *mut pollfd,
    pub nfds: nfds_t,
    pub arrsize: nfds_t,
    pub hop_limit: i32,
    pub daemon: Daemon,
    pub pid: pid_t,
    pub pipewrite: i32,
    pub rand_ctx: SurfRandContext,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            arp_records: vec![],
            last: 0,
            ..Default::default()
        }
    }
}
