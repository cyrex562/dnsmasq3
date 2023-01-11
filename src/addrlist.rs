use libc::time_t;
use crate::all_addr::AllAddr;

#[derive(Default,Debug,Clone)]
pub struct AddrList {
  // union all_addr addr;
  addr: AllAddr,
  // int flags, prefixlen;
  flags: i32,
  prefixlen: i32,
  // time_t decline_time;
  decline_time: time_t,
    // struct addrlist *next;
}
