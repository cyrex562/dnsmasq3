use libc::time_t;
use crate::all_addr::all_addr;

#[derive(Default,Debug,Clone)]
pub struct addrlist {
  // union all_addr addr;
  addr: all_addr,
  // int flags, prefixlen;
  flags: i32,
  prefixlen: i32,
  // time_t decline_time;
  decline_time: time_t,
    // struct addrlist *next;
}
