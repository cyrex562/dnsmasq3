/* Note that this is used widely as a container for IPv4/IPv6 addresses,
   so for that reason, was well as to avoid wasting memory in almost every
   cache entry, the other variants should not be larger than
   sizeof(struct in6_addr) - 16 bytes.
*/

use libc::c_char;

#[derive(Debug, Clone)]
pub union TargetUnion {
    //     //     union {
    //     //       struct crec *cache;
    //     //       char *name;
    //     //     } target;
    pub cache: *mut crec,
    pub name: *mut c_char,
}

// struct {
//     struct blockdata *keydata;
//     unsigned short keylen, flags, keytag;
//     unsigned char algo;
//   } key;
#[derive(Default, Debug, Clone)]
pub struct KeyStruct {
    pub keydata: *mut blockdata,
    pub keylen: u16,
    pub flags: u16,
    pub keytag: u16,
    pub algo: u8,
}

#[derive(Default, Debug, Clone)]
pub struct Cname {
    // struct {
    //     unsigned uid: i32;
    //     is_name_ptr: i32;  /* disciminates target union */
    //   } cname;
    pub target: TargetUnion,
    pub uid: i32,
    pub is_name_ptr: i32,
}

// struct {
//     struct blockdata *keydata;
//     unsigned short keylen, keytag;
//     unsigned char algo;
//     unsigned char digest;
//   } ds;
#[derive(Default, Debug, Clone)]
pub struct DsStruct {
    pub keydata: *mut blockdata,
    pub keylen: u16,
    pub keytag: u16,
    pub algo: u8,
    pub digest: u8,
}

//  struct {
//     struct blockdata *target;
//     unsigned short targetlen, srvport, priority, weight;
//   } srv;
#[derive(Default, Debug, Clone)]
pub struct SrvStruct {
    pub target: *mut blockdata,
    pub targetlent: u16,
    pub srvport: u16,
    pub priority: u16,
    pub weight: u16,
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

#[derive(Debug, Clone)]
pub union all_addr {
    // addr4: in_addr;
    pub addr4: in_addr,
    // addr6: in6_addr;
    pub addr6: in6_addr,
    pub cname: Cname,
    pub key: KeyStruct,
    pub ds: DsStruct,
    pub srv: SrvStruct,
    pub log: LogStruct,
}
