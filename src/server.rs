use libc::{c_char, time_t};
use crate::mysockaddr::mysockaddr;
use crate::server_fd::serverfd;

#[derive(Default, Debug, Clone)]
pub struct server {
    // u16 flags, domain_len;
    pub flags: u16,
    pub domain_len: u16,
    // char *domain;
    pub domain: *mut c_char,
    // struct server *next;
    // int serial, arrayposn;
    pub serial: i32,
    pub last_server: i32,
    // union mysockaddr addr, source_addr;
    pub addr: mysockaddr,
    pub source_addr: mysockaddr,
    pub interface: [c_char; IF_NAMESIZE + 1],
    pub ifindex: u32,
    /* corresponding to interface, above */
    // struct serverfd *sfd;
    pub sfd: *mut serverfd,
    // int tcpfd, edns_pktsz;
    pub tcpfd: i32,
    pub edns_pktsz: i32,
    // time_t pktsz_reduced;
    pub pktsz_reduced: time_t,
    // unsigned int queries, failed_queries, nxdomain_replies, retrys;
    pub queries: u32,
    pub failed_queries: u32,
    pub nxdomain_replies: u32,
    pub retrys: u32,
    // unsigned int query_latency, mma_latency;
    pub query_latency: u32,
    pub mma_latency: u32,
    // time_t forwardtime;
    pub forwardtime: time_t,
    // forwardcount: i32;
    pub forwardcount: i32,

    // #ifdef HAVE_LOOP
//   u32 uid;
    pub uid: u32,
// #endif
}
