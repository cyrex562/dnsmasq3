use std::fmt::{Debug, Formatter};
use libc::{c_char, time_t};
use crate::all_addr::AllAddr;
use crate::bigname::bigname;
use crate::config::SMALLDNAME;

// union {
//     char sname[SMALLDNAME];
//     union bigname *bname;
//     char *namep;
//   } name;
pub union NameUnion {
    pub sname: [c_char; SMALLDNAME as usize],
    pub bname: *mut bigname,
    pub namep: String,
}

impl Default for NameUnion {
    fn default() -> Self {
        Self {
            sname: [0; SMALLDNAME as usize]
        }
    }
}

impl Debug for NameUnion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.bname.is_null() == false {
            write!(f, "{:?}", self.bname)
        } else if self.namep.is_null() == false {
            write!(f, "{:?}", self.namep)
        } else {
            write!(f, "{:?}", self.sname)
        }
    }
}

impl Clone for NameUnion {
    fn clone(&self) -> Self {
        Self {
            sname: self.sname.clone()
        }
    }
}


#[derive(Default, Debug, Clone)]
pub struct crec {
    // struct crec *next, *prev, *hash_next;
    // union all_addr addr;
    pub addr: AllAddr,
    // time_t ttd; /* time to die */
    pub ttd: time_t,
    /* used as class if DNSKEY/DS, index to source for F_HOSTS */
    // unsigned uid: i32;
    pub uid: u32,
    // unsigned flags: i32;
    pub flags: u32,
    pub name: NameUnion,
}


// #define SIZEOF_BARE_CREC (sizeof(struct crec) - SMALLDNAME)
// #define SIZEOF_POINTER_CREC (sizeof(struct crec) + sizeof(char *) - SMALLDNAME)
