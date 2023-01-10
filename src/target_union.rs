use std::ptr::null_mut;
use libc::c_char;
use std::fmt::{Debug, Formatter};
use crate::crec::crec;

pub union TargetUnion {
    //     //     union {
    //     //       struct crec *cache;
    //     //       char *name;
    //     //     } target;
    pub cache: *mut crec,
    pub name: *mut c_char,
}

impl Default for TargetUnion {
    fn default() -> Self {
        Self {
            cache: null_mut()
        }
    }
}

impl Debug for TargetUnion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{cache: {:?}, name: {:?}}}", self.cache, self.name)
    }
}

impl Clone for TargetUnion {
    fn clone(&self) -> Self {
        Self {
            cache: self.cache
        }
    }
}
