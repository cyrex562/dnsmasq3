/* Note that this is used widely as a container for IPv4/IPv6 addresses,
   so for that reason, was well as to avoid wasting memory in almost every
   cache entry, the other variants should not be larger than
   sizeof(struct in6_addr) - 16 bytes.
*/

use std::fmt::{Debug, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ptr::{null, null_mut};
use libc::{c_char, in6_addr, in_addr};
use crate::blockdata::BlockData;
use crate::cname_struct::Cname;
use crate::util::{in6addr_to_string, inaddr_to_string};


// struct {
//     struct blockdata *keydata;
//     unsigned short keylen, flags, keytag;
//     unsigned char algo;
//   } key;
#[derive(Debug, Clone)]
pub struct KeyStruct {
    pub keydata: *mut BlockData,
    pub keylen: u16,
    pub flags: u16,
    pub keytag: u16,
    pub algo: u8,
}

impl Default for KeyStruct {
    fn default() -> Self {
        Self {
            keydata: null_mut(),
            keylen: 0,
            flags: 0,
            keytag: 0,
            algo: 0
        }
    }
}

// struct {
//     struct blockdata *keydata;
//     unsigned short keylen, keytag;
//     unsigned char algo;
//     unsigned char digest;
//   } ds;
#[derive(Debug, Clone)]
pub struct DsStruct {
    pub keydata: *mut BlockData,
    pub keylen: u16,
    pub keytag: u16,
    pub algo: u8,
    pub digest: u8,
}

impl Default for DsStruct {
    fn default() -> Self {
        Self {
            keydata: null_mut(),
            keylen: 0,
            keytag: 0,
            algo: 0,
            digest: 0
        }
    }
}

//  struct {
//     struct blockdata *target;
//     unsigned short targetlen, srvport, priority, weight;
//   } srv;
#[derive(Debug, Clone)]
pub struct SrvStruct {
    pub target: *mut BlockData,
    pub targetlent: u16,
    pub srvport: u16,
    pub priority: u16,
    pub weight: u16,
}

impl Default for SrvStruct {
    fn default() -> Self {
        Self {
            target: null_mut(),
            targetlent: 0,
            srvport: 0,
            priority: 0,
            weight: 0
        }
    }
}

//  struct {
//     unsigned short keytag, algo, digest, rcode;
//     ede: i32;
//   } log;
#[derive(Default, Debug, Clone)]
pub struct LogStruct {
    pub keytag: u16,
    pub algo: u16,
    pub digest: u16,
    pub rcode: u16,
    pub ede: i32,
}

pub union AllAddr {
    pub addr4: Ipv4Addr,
    pub addr6: Ipv6Addr,
    pub cname: Cname,
    pub key: KeyStruct,
    pub ds: DsStruct,
    pub srv: SrvStruct,
    pub log: LogStruct,
}

impl Default for AllAddr {
    fn default() -> Self {
        Self {
            addr6: in6_addr { s6_addr: [0;16] }
        }
    }
}

impl Debug for AllAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{addr4: {}, addr6: {}, cname: {:?}, key: {:?}, ds: {:?}, srv: {:?}, log: {:?}}}", inaddr_to_string(&self.addr4), in6addr_to_string(&self.addr6), &self.cname, &self.key, &self.ds, &self.srv, &self.log)
    }
}

impl Clone for AllAddr {
    fn clone(&self) -> Self {
        Self {
            addr6: self.addr6.clone()
        }
    }
}
