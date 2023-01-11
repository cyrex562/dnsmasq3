use std::ptr::{null, null_mut};
use libc::time_t;
use crate::all_addr::AllAddr;
use crate::allow_list::AllowList;
use crate::blockdata::BlockData;
use crate::dnsmasq_defines::HASH_SIZE;
use crate::frec_src::FrecSrc;
use crate::mysockaddr::MySockAddr;
use crate::rand_fd_list::RandFdList;
use crate::server::Server;

#[derive(Default, Debug, Clone)]
pub struct Frec {
    pub frec_src: FrecSrc,
    // struct server *sentto; /* NULL means free */
    pub sentto: Vec<Server>,
    // struct randfd_list *rfds;
    pub rfds: Vec<RandFdList>,
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
    pub stash: Vec<BlockData>,
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
