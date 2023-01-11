use std::ptr::{null, null_mut};
use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct PtrRecord {
    pub name: String,
    pub ptr: String,
}
