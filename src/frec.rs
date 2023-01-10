use std::ptr::{null, null_mut};
use libc::time_t;
use crate::all_addr::all_addr;
use crate::allow_list::allowlist;
use crate::blockdata::blockdata;
use crate::dnsmasq_defines::HASH_SIZE;
use crate::mysockaddr::mysockaddr;
use crate::rand_fd_list::randfd_list;
use crate::server::server;

#[derive(Default, Debug, Clone)]
pub struct FrecSrc {
    // union mysockaddr source; union all_addr dest; unsigned int iface,
    pub source: mysockaddr,
    pub dest: all_addr,
    pub iface: u32,
    // log_id; fd: i32;
    pub lod_id: i32,
    pub fd: i32,
    // unsigned short orig_id; struct frec_src * next;
    pub orig_id: u16,
}

#[derive(Debug, Clone)]
pub struct frec {
    pub frec_src: FrecSrc,
    // struct server *sentto; /* NULL means free */
    pub sentto: *mut server,
    // struct randfd_list *rfds;
    pub rfds: *mut randfd_list,
    // unsigned short new_id;
    pub new_id: u16,
    // int forwardall, flags;
    pub forwardall: i32,
    pub flags: i32,
    // time_t time;
    pub time: time_t,
    // u32 forward_timestamp;
    pub forward_timestamp: u32,
    // forward_delay: i32;
    pub forward_delay: i32,
    // unsigned char *hash[HASH_SIZE];
    pub hash: [*mut u8; HASH_SIZE as usize],
    // struct blockdata *stash; /* Saved reply, whilst we validate */
    pub stash: *mut blockdata,
    // stash_len: usize;
    pub stash_len: usize,
    // #ifdef HAVE_DNSSEC
//   int class, work_counter;
    pub class: i32,
    pub work_counter: i32,
    // TODO
    // struct frec *dependent; /* Query awaiting internally-generated DNSKEY or DS query */
    // TODO
    // struct frec *next_dependent; /* list of above. */
    // TODO
    // struct frec *blocking_query; /* Query which is blocking us. */
// #endif
//   struct frec *next;
}

impl Default for frec {
    fn default() -> Self {
        Self {
            frec_src: FrecSrc::default(),
            sentto: null_mut(),
            rfds: null_mut(),
            new_id: 0,
            forwardall: 0,
            flags: 0,
            time: 0,
            forward_timestamp: 0,
            forward_delay: 0,
            hash: [null_mut(); HASH_SIZE as usize],
            stash: null_mut(),
            stash_len: 0,
            class: 0,
            work_counter: 0,
        }
    }
}
