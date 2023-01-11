use std::ptr::{null, null_mut};
use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct RaInterface {
    pub name: String,
    pub mut_name: String,
    pub interval: i32,
    pub lifetime: i32,
    pub prio: i32,
    pub mtu: i32,
    // next
}
