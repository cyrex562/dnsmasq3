mod util;

use log::{debug, error, info};

type DmqPidT = i32;
type DmqInAddrT = u32;

#[derive(Debug,Copy,Clone)]
pub struct DmqInAddr {
    pub s_addr: DmqInAddrT,
}

#[derive(Debug,Copy,Clone)]
pub struct DmqIn6Addr {
    pub s6_addr: [u8; 16],
}

#[derive(Debug,Copy,Clone)]
pub struct DnsmasqDaemon {

}

#[derive(Debug,Copy,Clone)]
pub struct EventDesc {
    msg_sz: i32,
    event: i32,
    data: i32,
}

#[derive(Debug,Copy,Clone)]
pub struct AddrLog {
    keytag: u16,
    algo: u16,
    digest: u16
}

#[derive(Debug,Copy,Clone)]
pub struct AddrRCode {
    rcode: u32,
}

#[derive(Debug,Copy,Clone)]
pub struct AddrDnssec {
    _class: u16,
    _type: u16
}



#[repr(C)]
#[derive(Copy,Clone)]
union AddrUnion {
    addr4: DmqInAddr,
    addr6: DmqIn6Addr,
    log: AddrLog,
    rcode: AddrRCode,
    dnssec: AddrDnssec,
}

impl std::fmt::Debug for AddrUnion {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "union x {:?}", self)
    }
}

#[derive(Debug,Copy,Clone)]
pub struct AllAddr {
    addr: AddrUnion,
}

// struct bogus_addr {
//     struct in_addr addr;
//     struct bogus_addr* next;
// };
// TODO: replace bogus_addr with vec of InAddr
pub struct BogusAddr {
    addr: DmqInAddr,
    // todo: replace next field with vec
}

/* dns doctor param */
pub struct doctor {
    // struct in_addr in, end, out, mask;
    in_addr: DmqInAddr,
    end_addr: DmqInAddr,
    out_addr: DmqInAddr,
    mask_addr: DmqInAddr,
    // struct doctor* next;
    // TODO: replace next field with vec
}

pub struct mx_srv_record {
    // char* name, * target;
    name: String,
    target: String,
    // int issrv, srvport, priority, weight;
    issrv: i32,
    srv_port: i32,
    priority: i32,
    weight: i32,
    // uint32_t offset;
    offset: u32,
    // struct mx_srv_record* next;
    // TODO: replace next with vec
}

#[derive(Debug, Clone)]
pub struct Naptr {
    // char* name, * replace, * regexp, * services, * flags;
    name: String,
    replace: String, 
    regexp: String, 
    services: String,
    // uint32_t order, pref;
    order: u32,
    pref: u32,
    // struct naptr* next;
    // TODO: replace next field with vec
}

#[derive(Debug, Clone)]
pub struct TxtRecord {
    //char* name;
    name: String,
    // unsigned char* txt;
    txt: String,
    // uint16_t _class, len;
    _class: u16,
    len: u16,
    // int stat;
    stat: u32,
    // struct txt_record* next;
    // TODO: replace next field with vec
}

#[derive(Debug, Clone)]
pub struct PtrRecord {
    // char* name, * ptr;
    name: String,
    ptr: String,
    // struct ptr_record* next;
    // TODO: replace next field with vec
}

#[derive(Debug, Clone)]
pub struct Cname {
    // int ttl, flag;
    ttl: i32,
    flag: i32,
    // char* alias, * target;
    alias: String,
    target: String,
    // struct cname* next, * targetp;
    // TODO: use vec or something else to represent next and targetp
}

#[derive(Debug, Clone)]
pub struct DsConfig {
    // char* name, * digest;
    name: String,
    digest: String,
    // int digestlen, _class , algo, keytag, digest_type;
    digest_len: i32,
    _class: i32,
    algo: i32,
    keytag: i32,
    digest_type: i32
    // struct ds_config* next;
    // TODO: use vec to represent next field
}

#[derive(Debug, Clone)]
pub struct AddrList {
    // struct all_addr addr;
    addr: AllAddr,
    // int flags, prefixlen;
    flags: i32,
    prefixlen: i32,
    // struct addrlist* next;
    // TODO: use vec to represent next field
}

#[derive(Debug, Clone)]
pub struct AuthNameList {
    name: String,
    flags: i32,
    // TODO: next field replace with vec
}

#[derive(Debug, Clone)]
pub struct AuthZone {
    // char* domain;
    domain: String,
    // struct auth_name_list {
    //     char* name;
    //     int flags;
    //     struct auth_name_list* next;
    // } * interface_names;
    interface_names: Vec<AuthNameList>,
    // struct addrlist* subnet;
    subnet: Vec<AddrList>,
    // struct addrlist* exclude;
    exclude: Vec<AddrList>,
    // struct auth_zone* next;
    // TODO: replace next field with vec
}

#[derive(Debug, Clone)]
pub struct NameList {
    // char* name;
    name: String,
    // struct name_list* next;
    // TODO: replace next field with vec
}

#[derive(Debug, Clone)]
pub struct HostRecord {
    // int ttl;
    ttl: i32,
    // name_list* names;
    names: Vec<NameList>,
    // struct in_addr addr;
    addr: DmqInAddr,
    // struct in6_addr addr6;
    addr6: DmqIn6Addr,
    // struct host_record* next;
    // TODO: replace next field with vec
}

#[derive(Debug, Clone)]
pub struct InterfaceName {
    // char* name; /* domain name */
    name: String,
    // char* intr; /* interface name */
    intr: String,
    // int family; /* AF_INET, AF_INET6 or zero for both */
    family: i32,
    // struct addrlist* addr;
    addr: Vec<AddrList>,
    // struct interface_name* next;
    // TODO: replace next field with vec
}

// #define MAXDNAME	1025
const MAXDNAME: usize = 1025;

// union bigname {
//     // char name[MAXDNAME];
//     name: [u8;MAXDNAME]
//     union bigname* next; /* freelist */
// };
// TODO: create custom impl for Debug and Clone operations
pub struct Bigname {
    name: [u8;MAXDNAME],
    // TODO: replace next field with vec
}

// #define KEYBLOCK_LEN 40 /* choose to minimise fragmentation when storing DNSSEC keys */
const KEYBLOCK_LEN: usize = 40;

// TODO: create custom impl for Debug, Copy, and Clone operations
pub struct Blockdata {
    // struct blockdata* next;
    // TODO: replace next field with vec
    // unsigned char key[KEYBLOCK_LEN];
    key: [u8;KEYBLOCK_LEN],
}

#[derive(Debug,Clone,Copy)]
pub union CrecAddrCnameTarget {
    // TODO: figure out how to replace crec ptr var cache
    // struct crec* cache;
    int_name: InterfaceName,
}

#[derive(Debug,Clone,Copy)]
pub struct CrecAddrCname {
    target: CrecAddrCnameTarget,
    // uint32_t uid; /* 0 if union is interface-name */
    uid: u32,
}


#[derive(Debug,Clone,Copy)]
pub struct CrecAddrKey {
    // struct blockdata* keydata;
    keydata: Blockdata,
    // uint16_t keylen, flags, keytag;
    keylen: u16,
    flags: u16,
    keytag: u16,
    // unsigned char algo;
    algo: u8,
}

#[derive(Debug,Clone,Copy)]
pub struct {
    struct blockdata* keydata;
    uint16_t keylen, keytag;
    unsigned char algo;
    unsigned char digest;
} ds;

#[derive(Debug,Clone,Copy)]
pub union CrecAddr {
    // struct all_addr addr;
    addr: AllAddr,
    cname: CrecAddrCname,
    key: CrecAddrKey,
    ds: 
} 

pub struct crec {
    // TODO: replace ptr fields with vec usage
    // struct crec* hash_next;
    // struct crec* prev;
    // struct crec* next;
    /* union is 16 bytes when doing IPv6, 8 bytes on 32 bit machines without IPv6 */
    addr: CrecAddr,
    time_t ttd; /* time to die */
    /* used as class if DNSKEY/DS, index to source for F_HOSTS */
    uint32_t uid;
    uint16_t flags;
    union {
        char sname[SMALLDNAME];
        union bigname* bname;
        char* namep;
    } name;
};




// #define AUTH6     1
const AUTH6: i32 = 1;
// #define AUTH4     2
const AUTH4: i32 = 2;

// #define ADDRLIST_LITERAL 1
const ADDRLIST_LITERAL: i32 = 1;
// #define ADDRLIST_IPV6    2
const ADDRLIST_IPV6: i32 = 2;
// #define ADDRLIST_REVONLY 4
const ADDRLIST_REVONLY: i32 = 3;


// #define TXT_STAT_CACHESIZE     1
const TXT_STAT_CACHESIZE: i32 = 1;
// #define TXT_STAT_INSERTS       2
const TXT_STAT_INSERTS: i32 = 2;
// #define TXT_STAT_EVICTIONS     3
const TXT_STAT_EVICTIONS: i32 = 3;
// #define TXT_STAT_MISSES        4
const TXT_STAT_MISSES: i32 = 4;
// #define TXT_STAT_HITS          5
const TXT_STAT_HITS: i32 = 5;
// #define TXT_STAT_AUTH          6
const TXT_STAT_AUTH: i32 = 6;
// #define TXT_STAT_SERVERS       7
const TXT_STAT_SERVERS: i32 = 7;


const EVENT_RELOAD: i32 = 1;
const EVENT_DUMP: i32 = 2;
const EVENT_ALARM: i32 = 3;
const EVENT_TERM: i32 = 4;
const EVENT_CHILD: i32 = 5;
const EVENT_REOPEN: i32 = 6;
const EVENT_EXITED: i32 = 7;
const EVENT_KILLED: i32 = 8;
const EVENT_EXEC_ERR: i32 = 9;
const EVENT_PIPE_ERR: i32 = 10;
const EVENT_USER_ERR: i32 = 11;
const EVENT_CAP_ERR: i32 = 12;
const EVENT_PIDFILE: i32 = 13;
const EVENT_HUSER_ERR: i32 = 14;
const EVENT_GROUP_ERR: i32 = 15;
const EVENT_DIE: i32 = 16;
const EVENT_LOG_ERR: i32 = 17;
const EVENT_FORK_ERR: i32 = 18;
const EVENT_LUA_ERR: i32 = 19;
const EVENT_TFTP_ERR: i32 = 20;
const EVENT_INIT: i32 = 21;
const EVENT_NEWADDR: i32 = 22;
const EVENT_NEWROUTE: i32 = 23;
const EVENT_TIME_ERR: i32 = 24;
const EVENT_SCRIPT_LOG: i32 = 25;
const EVENT_TIME: i32 = 26;

// exit codes
const EC_GOOD: i32 = 0;
const EC_BADCONF: i32 = 1;
const EC_BADNET: i32 = 2;
const EC_FILE: i32 = 3;
const EC_NOMEM: i32 = 4;
const EC_MISC: i32 = 5;
const EC_INIT_OFFSET: i32 = 6;

// #define option_bool(x) (((x) < 32) ? daemon->options & (1u << (x)) : daemon->options2 & (1u << ((x) - 32)))

const OPT_BOGUSPRIV: i32 = 0;
const OPT_FILTER: i32 = 1;
const OPT_LOG: i32 = 2;
const OPT_SELFMX: i32 = 3;
const OPT_NO_HOSTS: i32 = 4;
const OPT_NO_POLL: i32 = 5;
const OPT_DEBUG: i32 = 6;
const OPT_ORDER: i32 = 7;
const OPT_NO_RESOLV: i32 = 8;
const OPT_EXPAND: i32 = 9;
const OPT_LOCALMX: i32 = 10;
const OPT_NO_NEG: i32 = 11;
const OPT_NODOTS_LOCAL: i32 = 12;
const OPT_NOWILD: i32 = 13;
const OPT_ETHERS: i32 = 14;
const OPT_RESOLV_DOMAIN: i32 = 15;
const OPT_NO_FORK: i32 = 16;
const OPT_AUTHORITATIVE: i32 = 17;
const OPT_LOCALISE: i32 = 18;
const OPT_DBUS: i32 = 19;
const OPT_DHCP_FQDN: i32 = 20;
const OPT_NO_PING: i32 = 21;
const OPT_LEASE_RO: i32 = 22;
const OPT_ALL_SERVERS: i32 = 23;
const OPT_RELOAD: i32 = 24;
const OPT_LOCAL_REBIND: i32 = 25;
const OPT_TFTP_SECURE: i32 = 26;
const OPT_TFTP_NOBLOCK: i32 = 27;
const OPT_LOG_OPTS: i32 = 28;
const OPT_TFTP_APREF_IP: i32 = 29;
const OPT_NO_OVERRIDE: i32 = 30;
const OPT_NO_REBIND: i32 = 31;
const OPT_ADD_MAC: i32 = 32;
const OPT_DNSSEC_PROXY: i32 = 33;
const OPT_CONSEC_ADDR: i32 = 34;
const OPT_CONNTRACK: i32 = 35;
const OPT_FQDN_UPDATE: i32 = 36;
const OPT_RA: i32 = 37;
const OPT_TFTP_LC: i32 = 38;
const OPT_CLEVERBIND: i32 = 39;
const OPT_TFTP: i32 = 40;
const OPT_CLIENT_SUBNET: i32 = 41;
const OPT_QUIET_DHCP: i32 = 42;
const OPT_QUIET_DHCP6: i32 = 43;
const OPT_QUIET_RA: i32 = 44;
const OPT_DNSSEC_VALID: i32 = 45;
const OPT_DNSSEC_TIME: i32 = 46;
const OPT_DNSSEC_DEBUG: i32 = 47;
const OPT_DNSSEC_IGN_NS: i32 = 48;
const OPT_LOCAL_SERVICE: i32 = 49;
const OPT_LOOP_DETECT: i32 = 50;
const OPT_EXTRALOG: i32 = 51;
const OPT_TFTP_NO_FAIL: i32 = 52;
const OPT_SCRIPT_ARP: i32 = 53;
const OPT_MAC_B64: i32 = 54;
const OPT_MAC_HEX: i32 = 55;
const OPT_TFTP_APREF_MAC: i32 = 56;
const OPT_RAPID_COMMIT: i32 = 57;
const OPT_UBUS: i32 = 58;
const OPT_LAST: i32 = 59;

const MS_TFTP: i32 = 60;
const MS_DHCP: i32 = 61;
const MS_SCRIPT: i32 = 62;

static pid: dmq_pid_t = 0;
static pipewrite: i32 = 0;

fn main() {
    match util::init_logger {
        Ok(_val) => println!("logger intialized");
        Err(e) => panic!(
            "failed to init logger: {}", e),

    }

    
}
