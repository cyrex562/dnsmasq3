/* dnsmasq is Copyright (c) 2000-2018 Simon Kelley

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; version 2 dated June, 1991, or
   (at your option) version 3 dated 29 June, 2007.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
// const COPYRIGHT :u32 = ;"Copyright (c) 2000-2018 Simon Kelley"

// const countof :u32 = ;(x)      (long)(sizeof(x) / sizeof(x[0]))
// TODO: define or replace

//const MIN :u32 = ;(a, b)        ((a) < (b) ? (a) : (b))
// TODO: define or replace

/* There doesn't seem to be a universally-available
userspace header for these. */
// TODO: replace
// extern int
// capset(cap_user_header_t header, cap_user_data_t data);

// TODO: replace
// extern int
// capget(cap_user_header_t header, cap_user_data_t data);

#[cfg(target_os = "windows")]
use crate::dnsmasq_sys::*;
use libc::{c_int, FILE};
#[cfg(not(target_os = "windows"))]
use libc::{
    dev_t, ino_t, iovec, off_t, pid_t, sockaddr, sockaddr_in, sockaddr_in6, time_t, LOG_DAEMON,
    LOG_MAIL, LOG_USER,
};

use std::ffi::c_void;

use crate::{arp::AddressPointer, config};
use crate::dhcp_protocol;

pub const AF_INET6: c_int = 10;
pub const AF_UNSPEC: c_int = 0;

pub const LINUX_CAPABILITY_VERSION_1: u32 = 0x19980330;
pub const LINUX_CAPABILITY_VERSION_2: u32 = 0x20071026;
pub const LINUX_CAPABILITY_VERSION_3: u32 = 0x20080522;

// //#include <sys/prctl.h>

// #elif defined(HAVE_SOLARIS_NETWORK)
// //#include <priv.h>
// //#endif

// //#ifdef HAVE_DNSSEC
// #  include <nettle/nettle-meta.h>
// //#endif

/* daemon is function in the C library.... */
// const daemon :u32 = ;dnsmasq_daemon

/* Async event queue */
pub struct EventDesc {
    pub msg_sz: i32,
    pub event: i32,
    pub data: i32,
}

pub const EVENT_RELOAD: u32 = 1;
pub const EVENT_DUMP: u32 = 2;
pub const EVENT_ALARM: u32 = 3;
pub const EVENT_TERM: u32 = 4;
pub const EVENT_CHILD: u32 = 5;
pub const EVENT_REOPEN: u32 = 6;
pub const EVENT_EXITED: u32 = 7;
pub const EVENT_KILLED: u32 = 8;
pub const EVENT_EXEC_ERR: u32 = 9;
pub const EVENT_PIPE_ERR: u32 = 10;
pub const EVENT_USER_ERR: u32 = 11;
pub const EVENT_CAP_ERR: u32 = 12;
pub const EVENT_PIDFILE: u32 = 13;
pub const EVENT_HUSER_ERR: u32 = 14;
pub const EVENT_GROUP_ERR: u32 = 15;
pub const EVENT_DIE: u32 = 16;
pub const EVENT_LOG_ERR: u32 = 17;
pub const EVENT_FORK_ERR: u32 = 18;
pub const EVENT_LUA_ERR: u32 = 19;
pub const EVENT_TFTP_ERR: u32 = 20;
pub const EVENT_INIT: u32 = 21;
pub const EVENT_NEWADDR: u32 = 22;
pub const EVENT_NEWROUTE: u32 = 23;
pub const EVENT_TIME_ERR: u32 = 24;
pub const EVENT_SCRIPT_LOG: u32 = 25;
pub const EVENT_TIME: u32 = 26;

/* Exit codes. */
pub const EC_GOOD: u32 = 0;
pub const EC_BADCONF: u32 = 1;
pub const EC_BADNET: u32 = 2;
pub const EC_FILE: u32 = 3;
pub const EC_NOMEM: u32 = 4;
pub const EC_MISC: u32 = 5;
pub const EC_INIT_OFFSET: u32 = 10;

/* Trust the compiler dead-code eliminator.... */
// const option_bool :u32 = ;(x) (((x) < 32) ? daemon->options & (1u << (x)) : daemon->options2 & (1u << ((x) - 32)))
pub fn option_bool(opt: u32, daemon: &Daemon) -> u32 {
    if opt < 32 {
        return daemon.options & 1 << opt;
    } else {
        return daemon.options2 & 1 << (opt - 32);
    }
}

pub const OPT_BOGUSPRIV: u32 = 0;
pub const OPT_FILTER: u32 = 1;
pub const OPT_LOG: u32 = 2;
pub const OPT_SELFMX: u32 = 3;
pub const OPT_NO_HOSTS: u32 = 4;
pub const OPT_NO_POLL: u32 = 5;
pub const OPT_DEBUG: u32 = 6;
pub const OPT_ORDER: u32 = 7;
pub const OPT_NO_RESOLV: u32 = 8;
pub const OPT_EXPAND: u32 = 9;
pub const OPT_LOCALMX: u32 = 10;
pub const OPT_NO_NEG: u32 = 11;
pub const OPT_NODOTS_LOCAL: u32 = 12;
pub const OPT_NOWILD: u32 = 13;
pub const OPT_ETHERS: u32 = 14;
pub const OPT_RESOLV_DOMAIN: u32 = 15;
pub const OPT_NO_FORK: u32 = 16;
pub const OPT_AUTHORITATIVE: u32 = 17;
pub const OPT_LOCALISE: u32 = 18;
pub const OPT_DBUS: u32 = 19;
pub const OPT_DHCP_FQDN: u32 = 20;
pub const OPT_NO_PING: u32 = 21;
pub const OPT_LEASE_RO: u32 = 22;
pub const OPT_ALL_SERVERS: u32 = 23;
pub const OPT_RELOAD: u32 = 24;
pub const OPT_LOCAL_REBIND: u32 = 25;
pub const OPT_TFTP_SECURE: u32 = 26;
pub const OPT_TFTP_NOBLOCK: u32 = 27;
pub const OPT_LOG_OPTS: u32 = 28;
pub const OPT_TFTP_APREF_IP: u32 = 29;
pub const OPT_NO_OVERRIDE: u32 = 30;
pub const OPT_NO_REBIND: u32 = 31;
pub const OPT_ADD_MAC: u32 = 32;
pub const OPT_DNSSEC_PROXY: u32 = 33;
pub const OPT_CONSEC_ADDR: u32 = 34;
pub const OPT_CONNTRACK: u32 = 35;
pub const OPT_FQDN_UPDATE: u32 = 36;
pub const OPT_RA: u32 = 37;
pub const OPT_TFTP_LC: u32 = 38;
pub const OPT_CLEVERBIND: u32 = 39;
pub const OPT_TFTP: u32 = 40;
pub const OPT_CLIENT_SUBNET: u32 = 41;
pub const OPT_QUIET_DHCP: u32 = 42;
pub const OPT_QUIET_DHCP6: u32 = 43;
pub const OPT_QUIET_RA: u32 = 44;
pub const OPT_DNSSEC_VALID: u32 = 45;
pub const OPT_DNSSEC_TIME: u32 = 46;
pub const OPT_DNSSEC_DEBUG: u32 = 47;
pub const OPT_DNSSEC_IGN_NS: u32 = 48;
pub const OPT_LOCAL_SERVICE: u32 = 49;
pub const OPT_LOOP_DETECT: u32 = 50;
pub const OPT_EXTRALOG: u32 = 51;
pub const OPT_TFTP_NO_FAIL: u32 = 52;
pub const OPT_SCRIPT_ARP: u32 = 53;
pub const OPT_MAC_B64: u32 = 54;
pub const OPT_MAC_HEX: u32 = 55;
pub const OPT_TFTP_APREF_MAC: u32 = 56;
pub const OPT_RAPID_COMMIT: u32 = 57;
pub const OPT_UBUS: u32 = 58;
pub const OPT_LAST: u32 = 59;

/* extra flags for my_syslog, we use a couple of facilities since they are known
not to occupy the same bits as priorities, no matter how syslog.h is set up. */
pub const MS_TFTP: i32 = LOG_USER;
pub const MS_DHCP: i32 = LOG_DAEMON;
pub const MS_SCRIPT: i32 = LOG_MAIL;

pub struct all_addr_log {
    keytag: u16,
    algo: u16,
    digest: u16,
}

pub struct all_addr_rcode {
    rcode: u32,
}

pub struct AllAddrDnssec {
    pub _class: u16,
    pub _type: u16,
}

pub struct AllAddrAddr {
    pub addr4: InAddr,
    pub addr6: In6Addr,
    pub log: all_addr_log,
    pub rcode: all_addr_rcode,
    pub dnssec: AllAddrDnssec,
}

// pub struct AllAddr {
//     pub addr: AllAddrAddr
// }

pub struct BogusAddr {
    pub addr: InAddr,
    // next: &bogus_addr,;
}

/* dns doctor param */
pub struct doctor {
    _in: InAddr,
    end: InAddr,
    out: InAddr,
    mask: InAddr,
    // next: doctor*
}

pub struct mx_srv_record {
    name: String,
    target: String,
    issrv: i32,
    srvport: i32,
    priority: i32,
    weight: i32,
    offset: u32,
    // next: mx_srv_record
}

pub struct naptr {
    name: String,
    replace: String,
    regexp: String,
    services: String,
    flages: String,
    pref: String,
    order: String,
    // next: naptr*
}

pub const TXT_STAT_CACHESIZE: u32 = 1;
pub const TXT_STAT_INSERTS: u32 = 2;
pub const TXT_STAT_EVICTIONS: u32 = 3;
pub const TXT_STAT_MISSES: u32 = 4;
pub const TXT_STAT_HITS: u32 = 5;
pub const TXT_STAT_AUTH: u32 = 6;
pub const TXT_STAT_SERVERS: u32 = 7;

pub struct txt_record {
    name: String,
    txt: String,
    _class: u16,
    len: u16,
    stat: i32,
    // next: txt_record*
}

pub struct ptr_record {
    name: String,
    ptr: String,
    // next: ptr_record*
}

pub struct cname {
    flag: i32,
    ttl: i32,
    alias: String,
    target: String,
    // next: cname*
    // targetp: cname*
}

pub struct ds_config {
    name: String,
    digest: String,
    digestlen: i32,
    _class: i32,
    algo: i32,
    keytag: i32,
    digest_type: i32,
    // next: ds_config*
}

pub const ADDRLIST_LITERAL: u32 = 1;
pub const ADDRLIST_IPV6: u32 = 2;
pub const ADDRLIST_REVONLY: u32 = 4;

pub struct addrlist {
    addr: AllAddr,
    flags: i32,
    prefixlen: i32,
    // next: addrlist*
}

pub const AUTH6: u32 = 1;
pub const AUTH4: u32 = 2;

pub struct auth_name_list {
    name: String,
    flags: i32,
    // next: auth_name_list*
}

pub struct auth_zone {
    domain: String,
    interface_names: Vec<auth_name_list>,
    subnet: addrlist,
    exclude: addrlist,
    //next: auth_zone*
}

pub struct name_list {
    name: String,
    // next: name_list*
}

pub struct host_record {
    ttl: i32,
    names: Vec<name_list>,
    addr: InAddr,
    addr6: In6Addr,
    // next: host_record*
}

pub struct interface_name {
    name: String,
    intr: String,
    family: i32,
    // TODO: should this be a list of addrlist structs or a pointer to one?
    addr: Vec<addrlist>,
    // next: interface_name*
}

pub struct bigname {
    name: String,
    // next: bigname*
}

pub struct blockdata {
    // next blockdata*
    key: [u8; config::KEYBLOCK_LEN],
}

pub struct crec_target {
    // TODO: should cache and int_name be vectors or just pointers?
    cache: Vec<crec>,
    int_name: Vec<interface_name>,
}

pub struct crec_cname {
    target: crec_target,
    uid: u32,
}

pub struct crec_key {
    keydata: Vec<blockdata>,
    keylen: u16,
    flags: u16,
    keytag: u16,
    algo: u8,
}

pub struct crec_ds {
    keydata: Vec<blockdata>,
    keylen: u16,
    keytag: u16,
    algo: u8,
    digest: u8,
}

pub struct CrecAddr {
    addr: Vec<AddressPointer>,
    cname: crec_cname,
    key: crec_key,
    ds: crec_ds,
}

pub struct crec_name {
    sname: String,
    bname: bigname,
    namep: String,
}

pub struct crec {
    // hash_next: crec*
    // prev: crec*
    // next: crec*
    addr: CrecAddr,
    ttd: time_t,
    uid: u32,
    flags: u16,
    name: crec_name,
}

// TODO: implement
// const SIZEOF_BARE_CREC = (sizeof(struct crec) - SMALLDNAME)
// TODO: implement
// const SIZEOF_POINTER_CREC = (sizeof(struct crec) + sizeof(char *) - SMALLDNAME)

pub const F_IMMORTAL: u32 = 1;
pub const F_NAMEP: u32 = 2;
pub const F_REVERSE: u32 = 4;
pub const F_FORWARD: u32 = 8;
pub const F_DHCP: u32 = 16;
pub const F_NEG: u32 = 32;
pub const F_HOSTS: u32 = 64;
pub const F_IPV4: u32 = 128;
pub const F_IPV6: u32 = 256;
pub const F_BIGNAME: u32 = 512;
pub const F_NXDOMAIN: u32 = 1024;
pub const F_CNAME: u32 = 2048;
pub const F_DNSKEY: u32 = 4096;
pub const F_CONFIG: u32 = 8192;
pub const F_DS: u32 = 16384;
pub const F_DNSSECOK: u32 = 32768;

/* below here are only valid as args to log_query: cache
entries are limited to 16 bits */
pub const F_UPSTREAM: u32 = 65536;
pub const F_RRNAME: u32 = 131072;
pub const F_SERVER: u32 = 262144;
pub const F_QUERY: u32 = 524288;
pub const F_NOERR: u32 = 1048576;
pub const F_AUTH: u32 = 2097152;
pub const F_DNSSEC: u32 = 4194304;
pub const F_KEYTAG: u32 = 8388608;
pub const F_SECSTAT: u32 = 16777216;
pub const F_NO_RR: u32 = 33554432;
pub const F_IPSET: u32 = 67108864;
pub const F_NOEXTRA: u32 = 134217728;
pub const F_SERVFAIL: u32 = 268435456;
pub const F_RCODE: u32 = 536870912;

pub const UID_NONE: u32 = 0;
/* Values of uid in crecs with F_CONFIG bit set. */
/* cname to uid SRC_INTERFACE are to interface names,
so use UID_NONE for that to eliminate clashes with
any other uid */
pub const SRC_INTERFACE: u32 = UID_NONE;
pub const SRC_CONFIG: u32 = 1;
pub const SRC_HOSTS: u32 = 2;
pub const SRC_AH: u32 = 3;

/* struct sockaddr is not large enough to hold any address,
and specifically not big enough to hold an IPv6 address.
Blech. Roll our own. */
pub struct Mysockaddr {
    pub sa: Sockaddr,
    pub in4: sockaddr_in,
    pub in6: sockaddr_in6,
}

/* bits in flag param to IPv6 callbacks from iface_enumerate() */
pub const IFACE_TENTATIVE: u32 = 1;
pub const IFACE_DEPRECATED: u32 = 2;
pub const IFACE_PERMANENT: u32 = 4;

pub const SERV_FROM_RESOLV: u32 = 1; /* 1 for servers from resolv, 0 for command line. */
pub const SERV_NO_ADDR: u32 = 2; /* no server, this domain is local only */
pub const SERV_LITERAL_ADDRESS: u32 = 4; /* addr is the answer, not the server */
pub const SERV_HAS_DOMAIN: u32 = 8; /* server for one domain only */
pub const SERV_HAS_SOURCE: u32 = 16; /* source address defined */
pub const SERV_FOR_NODOTS: u32 = 32; /* server for names with no domain part only */
pub const SERV_WARNED_RECURSIVE: u32 = 64; /* avoid warning spam */
pub const SERV_FROM_DBUS: u32 = 128; /* 1 if source is DBus */
pub const SERV_MARK: u32 = 256; /* for mark-and-delete */
pub const SERV_TYPE: u32 = (SERV_HAS_DOMAIN | SERV_FOR_NODOTS);
pub const SERV_COUNTED: u32 = 512; /* workspace for log code */
pub const SERV_USE_RESOLV: u32 = 1024; /* forward this domain in the normal way */
pub const SERV_NO_REBIND: u32 = 2048; /* inhibit dns-rebind protection */
pub const SERV_FROM_FILE: u32 = 4096; /* read from --servers-file */
pub const SERV_LOOP: u32 = 8192; /* server causes forwarding loop */
pub const SERV_DO_DNSSEC: u32 = 16384; /* Validate DNSSEC when using this server */
pub const SERV_GOT_TCP: u32 = 32768; /* Got some data from the TCP connection */

pub struct ServerFd {
    fd: i32,
    source_addr: Mysockaddr,
    _interface: String,
    preallocated: u32,
    used: u32,
    ifindex: u32,
    // next: ServerFd*
}

pub struct randfd {
    fd: i32,
    refcount: u16,
    family: u16,
}

pub struct server {
    addr: Mysockaddr,
    source_addr: Mysockaddr,
    _interface: String,
    sfd: ServerFd,
    domain: String,
    flags: i32,
    tcpfd: i32,
    edns_pktsz: i32,
    pktsz_reduced: time_t,
    queries: u32,
    failed_queries: u32,
    uid: u32,
    // next: server*
}

pub struct ipsets {
    sets: Vec<String>,
    domain: String,
    // next: ipsets*
}

pub struct irec {
    addr: Mysockaddr,
    netmask: InAddr,
    tftp_ok: i32,
    dhcp_ok: i32,
    mtu: i32,
    done: i32,
    warned: i32,
    dad: i32,
    dns_auth: i32,
    index: i32,
    multicast_done: i32,
    found: i32,
    label: i32,
    name: String,
    // next: irec*
}

pub struct listener {
    fd: i32,
    tcpfd: i32,
    tftpfd: i32,
    family: i32,
    iface: irec, // TODO: ptr?
                 // next: listener*
}

/* interface and address parms from command line. */
pub struct Iname {
    name: String,
    addr: Mysockaddr,
    used: i32,
    // next: Iname*
}

/* subnet parameters from command line */
pub struct mysubnet {
    addr: Mysockaddr,
    addr_used: i32,
    mask: i32,
}

/* resolv-file parms from command-line */
pub struct resolvc {
    // next: resolvc*
    is_default: i32,
    logged: i32,
    mtime: time_t,
    name: String,
    wd: i32,
    file: String,
}

/* adn-hosts parms from command-line (also dhcp-hostsfile and dhcp-optsfile and dhcp-hostsdir*/
pub const AH_DIR: u32 = 1;
pub const AH_INACTIVE: u32 = 2;
pub const AH_WD_DONE: u32 = 4;
pub const AH_HOSTS: u32 = 8;
pub const AH_DHCP_HST: u32 = 16;
pub const AH_DHCP_OPT: u32 = 32;

pub struct hostsfile {
    // next: hostsfile*
    flags: i32,
    fname: String,
    wd: i32,
    index: u32,
}

/* packet-dump flags */
pub const DUMP_QUERY: u32 = 0x0001;
pub const DUMP_REPLY: u32 = 0x0002;
pub const DUMP_UP_QUERY: u32 = 0x0004;
pub const DUMP_UP_REPLY: u32 = 0x0008;
pub const DUMP_SEC_QUERY: u32 = 0x0010;
pub const DUMP_SEC_REPLY: u32 = 0x0020;
pub const DUMP_BOGUS: u32 = 0x0040;
pub const DUMP_SEC_BOGUS: u32 = 0x0080;

/* DNSSEC status values. */
pub const STAT_SECURE: u32 = 1;
pub const STAT_INSECURE: u32 = 2;
pub const STAT_BOGUS: u32 = 3;
pub const STAT_NEED_DS: u32 = 4;
pub const STAT_NEED_KEY: u32 = 5;
pub const STAT_TRUNCATED: u32 = 6;
pub const STAT_SECURE_WILDCARD: u32 = 7;
pub const STAT_OK: u32 = 8;
pub const STAT_ABANDONED: u32 = 9;

pub const FREC_NOREBIND: u32 = 1;
pub const FREC_CHECKING_DISABLED: u32 = 2;
pub const FREC_HAS_SUBNET: u32 = 4;
pub const FREC_DNSKEY_QUERY: u32 = 8;
pub const FREC_DS_QUERY: u32 = 16;
pub const FREC_AD_QUESTION: u32 = 32;
pub const FREC_DO_QUESTION: u32 = 64;
pub const FREC_ADDED_PHEADER: u32 = 128;
pub const FREC_TEST_PKTSZ: u32 = 256;
pub const FREC_HAS_EXTRADATA: u32 = 512;

pub const HASH_SIZE_DNSSEC: u32 = 20; /* SHA-1 digest size */
pub const HASH_SIZE: usize = 32;

pub struct Frec {
    source: Mysockaddr,
    dest: Vec<AddressPointer>,
    sentto: server, // TODO ptr?
    rfd64: randfd,  // TODO: ptr?
    rfd6: randfd,   // TODO: ptr?
    iface: u32,
    orig_id: u32,
    new_id: u32,
    log_id: i32,
    fd: i32,
    forwardall: i32,
    flags: i32,
    time: time_t,
    hash: [u8; HASH_SIZE],
    _class: i32,
    work_counter: i32,
    stash: Vec<blockdata>,
    stash_len: usize,
    // dependent: frec, // TODO: ptr?
    // blocking_query: frec // TODO: ptr?
    // next: frec*
}

/* flags in top of length field for DHCP-option tables */
pub const OT_ADDR_LIST: u32 = 0x8000;
pub const OT_RFC1035_NAME: u32 = 0x4000;
pub const OT_INTERNAL: u32 = 0x2000;
pub const OT_NAME: u32 = 0x1000;
pub const OT_CSTRING: u32 = 0x0800;
pub const OT_DEC: u32 = 0x0400;
pub const OT_TIME: u32 = 0x0200;

/* actions in the daemon->helper RPC */
// pub const ACTION_DEL :u32 = 1;
// pub const ACTION_OLD_HOSTNAME :u32 = 2;
// pub const ACTION_OLD :u32 = 3;
// pub const ACTION_ADD :u32 = 4;
// pub const ACTION_TFTP :u32 = 5;
// pub const ACTION_ARP :u32 = 6;
// pub const ACTION_ARP_DEL :u32 = 7;

pub enum Action {
    ACTION_DEL,
    ACTION_OLD_HOSTNAME,
    ACTION_OLD,
    ACTION_ADD,
    ACTION_TFTP,
    ACTION_ARP,
    ACTION_ARP_DEL,
}

pub const LEASE_NEW: u32 = 1; /* newly created */
pub const LEASE_CHANGED: u32 = 2; /* modified */
pub const LEASE_AUX_CHANGED: u32 = 4; /* CLID or expiry changed */
pub const LEASE_AUTH_NAME: u32 = 8; /* hostname came from config, not from client */
pub const LEASE_USED: u32 = 16; /* used this DHCPv6 transaction */
pub const LEASE_NA: u32 = 32; /* IPv6 no-temporary lease */
pub const LEASE_TA: u32 = 64; /* IPv6 temporary lease */
pub const LEASE_HAVE_HWADDR: u32 = 128; /* Have set hwaddress */

pub struct slaac_address {
    addr: In6Addr,
    ping_time: time_t,
    backoff: i32,
    // next: slaac_address*
}

pub struct dhcp_lease {
    clid_len: i32,
    clid: Vec<u8>,
    hostname: String,
    fqdn: String,
    old_hostname: String,
    flags: i32,
    expires: time_t,
    length: u32,
    hwaddr_len: i32,
    hwaddr_type: i32,
    hwaddr: [u8; dhcp_protocol::DHCP_CHADDR_MAX],
    addr: InAddr,
    _override: InAddr,
    giaddr: InAddr,
    extradata: Vec<u8>,
    extradata_len: u32,
    extradata_size: u32,
    last_interface: i32,
    new_interface: i32,
    new_prefixlen: i32,
    addr6: In6Addr,
    iaid: i32,
    slaac_addr: slaac_address,
    vendorclass_count: i32,
    // next: dhcp_lease*
}

pub struct dhcp_netid {
    net: String,
    // next: dhcp_netid*
}

pub struct dhcp_netid_list {
    list: Vec<dhcp_netid>,
    // next: dhcp_netid_list
}

pub struct tag_if {
    set: dhcp_netid_list,
    tag: dhcp_netid, // TODO: ptr,
                     // next: tag_if*
}

pub struct delay_config {
    delay: i32,
    netid: dhcp_netid, // todo: ptr,
                       // next: delay_config*
}

pub struct hwaddr_config {
    hwaddr_len: i32,
    hwaddr_type: i32,
    hwaddr: [u8; dhcp_protocol::DHCP_CHADDR_MAX],
    wildcard_mask: u32,
    // next: hwaddr_config*
}

pub struct dhcp_config {
    flags: u32,
    clid_len: i32,
    clid: Vec<u8>,
    hostname: String,
    domain: String,
    netid: dhcp_netid_list,
    addr6: In6Addr,
    addr: InAddr,
    decline_time: time_t,
    lease_time: u32,
    hwaddr: hwaddr_config,
    // next: dhcp_config*
}

// todo: re-implement
//const have_config :u32 = ;(config, mask) ((config) && ((config)->flags & (mask)))

pub const CONFIG_DISABLE: u32 = 1;
pub const CONFIG_CLID: u32 = 2;
pub const CONFIG_TIME: u32 = 8;
pub const CONFIG_NAME: u32 = 16;
pub const CONFIG_ADDR: u32 = 32;
pub const CONFIG_NOCLID: u32 = 128;
pub const CONFIG_FROM_ETHERS: u32 = 256; /* entry created by /etc/ethers */
pub const CONFIG_ADDR_HOSTS: u32 = 512; /* address added by from /etc/hosts */
pub const CONFIG_DECLINED: u32 = 1024; /* address declined by client */
pub const CONFIG_BANK: u32 = 2048; /* from dhcp hosts file */
pub const CONFIG_ADDR6: u32 = 4096;
pub const CONFIG_WILDCARD: u32 = 8192;

pub struct dhcp_opt_u {
    encap: i32,
    wildcard_mask: u32,
    vendor_class: Vec<u8>,
}

pub struct dhcp_opt {
    opt: i32,
    len: i32,
    flags: i32,
    u: dhcp_opt_u,
    val: Vec<u8>,
    netid: dhcp_netid,
    // next: dhcp_opt,
}

pub const DHOPT_ADDR: u32 = 1;
pub const DHOPT_STRING: u32 = 2;
pub const DHOPT_ENCAPSULATE: u32 = 4;
pub const DHOPT_ENCAP_MATCH: u32 = 8;
pub const DHOPT_FORCE: u32 = 16;
pub const DHOPT_BANK: u32 = 32;
pub const DHOPT_ENCAP_DONE: u32 = 64;
pub const DHOPT_MATCH: u32 = 128;
pub const DHOPT_VENDOR: u32 = 256;
pub const DHOPT_HEX: u32 = 512;
pub const DHOPT_VENDOR_MATCH: u32 = 1024;
pub const DHOPT_RFC3925: u32 = 2048;
pub const DHOPT_TAGOK: u32 = 4096;
pub const DHOPT_ADDR6: u32 = 8192;

pub struct dhcp_boot {
    file: String,
    sname: String,
    tftp_sname: String,
    next_server: InAddr,
    netid: dhcp_netid,
    // next: dhcp_boot*
}

pub struct dhcp_match_name {
    name: String,
    wildcard: i32,
    netid: dhcp_netid,
    // next: dhcp_match_name*
}

pub struct pxe_service {
    CSA: u16,
    _type: u16,
    menu: String,
    basename: String,
    sname: String,
    server: InAddr,
    netid: dhcp_netid,
    // next: pxe_service*
}

pub const MATCH_VENDOR: u32 = 1;
pub const MATCH_USER: u32 = 2;
pub const MATCH_CIRCUIT: u32 = 3;
pub const MATCH_REMOTE: u32 = 4;
pub const MATCH_SUBSCRIBER: u32 = 5;

/* vendorclass, userclass, remote-id or circuit-id */
pub struct dhcp_vendor {
    len: i32,
    match_type: i32,
    enterprise: u32,
    data: String,
    netid: dhcp_netid,
    // next: dhcp_vendor*
}

pub struct dhcp_mac {
    mask: u32,
    hwaddr_len: i32,
    hwaddr_type: i32,
    hwaddr: [u8; dhcp_protocol::DHCP_CHADDR_MAX],
    netid: dhcp_netid,
    // next: dhcp_mac*
}

pub struct dhcp_bridge {
    iface: String,
    // alias: dhcp_bridge*
    // next: dhcp_bridge*
}

pub struct cond_domain {
    domain: String,
    prefix: String,
    start: InAddr,
    end: InAddr,
    start6: In6Addr,
    end6: In6Addr,
    is6: i32,
    indexed: i32,
    // next: cond_domain*
}

// //#ifdef OPTION6_PREFIX_CLASS
pub struct prefix_class {
    _class: i32,
    tag: dhcp_netid,
    // next: prefix_class*
}
// //#endif

pub struct ra_interface {
    name: String,
    mtu_name: String,
    interval: i32,
    lifetime: i32,
    prio: i32,
    mtu: i32,
    // next: ra_interface*
}

pub struct DhcpContext {
    lease_time: u32,
    addr_epoch: u32,
    netmask: InAddr,
    broadcast: InAddr,
    local: InAddr,
    router: InAddr,
    start: InAddr,
    end: InAddr,
    start6: In6Addr,
    end6: In6Addr,
    local6: In6Addr,
    prefix: i32,
    if_index: i32,
    valid: u32,
    preferred: u32,
    saved_valid: u32,
    ra_time: time_t,
    ra_short_period_start: time_t,
    address_list_time: time_t,
    template_interface: String,
    flags: i32,
    netid: dhcp_netid,
    filter: dhcp_netid,
    // next: DhcpContext
    // current: DhcpContext
}

pub const CONTEXT_STATIC: u32 = 1;
pub const CONTEXT_NETMASK: u32 = 2;
pub const CONTEXT_BRDCAST: u32 = 4;
pub const CONTEXT_PROXY: u32 = 8;
pub const CONTEXT_RA_ROUTER: u32 = 16;
pub const CONTEXT_RA_DONE: u32 = 32;
pub const CONTEXT_RA_NAME: u32 = 64;
pub const CONTEXT_RA_STATELESS: u32 = 128;
pub const CONTEXT_DHCP: u32 = 256;
pub const CONTEXT_DEPRECATE: u32 = 512;
pub const CONTEXT_TEMPLATE: u32 = 1024; /* create contexts using addresses */
pub const CONTEXT_CONSTRUCTED: u32 = 2048;
pub const CONTEXT_GC: u32 = 4096;
pub const CONTEXT_RA: u32 = 8192;
pub const CONTEXT_CONF_USED: u32 = 16384;
pub const CONTEXT_USED: u32 = 32768;
pub const CONTEXT_OLD: u32 = 65536;
pub const CONTEXT_V6: u32 = 131072;
pub const CONTEXT_RA_OFF_LINK: u32 = 262144;

pub struct ping_result {
    addr: InAddr,
    time: time_t,
    hash: u32,
    // next: ping_result*
}

pub struct tftp_file {
    refcount: i32,
    fd: i32,
    size: off_t,
    dev: dev_t,
    inode: ino_t,
    filename: String,
}

pub struct tftp_transfer {
    sockfd: i32,
    timeout: time_t,
    backoff: i32,
    expansion: u32,
    blocksize: u32,
    block: u32,
    peer: Mysockaddr,
    carrylf: u8,
    netascii: u8,
    opt_transize: u8,
    opt_blocksize: u8,
    file: tftp_file,
    // enxt: tftp_transfer
}

pub struct AddrList {
    addr: InAddr,
    // next: AddrList*
}

pub struct TftpPrefix {
    _interface: String,
    prefix: String,
    missing: i32,
    // next: TftpPrefix*
}

pub struct DhcpRelay {
    server: Vec<AddressPointer>,
    local: Vec<AddressPointer>,
    _interface: String,
    iface_index: i32,
    // next: DhcpRelay*
    // current: DhcpRelay*
}

pub struct Daemon {
    /* datastuctures representing the command-line and
    config file arguments. All set (including defaults)
    in option.c */
    pub options2: u32,
    /* datastuctures representing the command-line and
    config file arguments. All set (including defaults)
    in option.c */
    pub options: u32,
    pub resolv_files: resolvc,   // todo: ptr?
    pub default_resolv: resolvc, // todo: ptr?
    pub last_resolv: time_t,
    pub server_file: String,
    pub mxnames: Vec<mx_srv_record>,
    pub naptr: naptr,
    pub rr: txt_record,
    pub txt: txt_record,
    pub ptr: ptr_record,
    pub host_records_tail: Vec<host_record>,
    pub host_records: Vec<host_record>,
    pub cnames: Vec<cname>,
    pub auth_zones: Vec<auth_zone>,
    pub int_names: Vec<interface_name>,
    pub mxtarget: String,
    pub add_subnet4: mysubnet,
    pub add_subnet6: mysubnet,
    pub lease_file: String,
    pub scriptuser: String,
    pub groupname: String,
    pub username: String,
    pub luascript: String,
    pub hostmaster: String,
    pub authserver: String,
    pub authinterface: String,
    pub scondary_foward_server: Vec<name_list>,
    pub osport: i32,
    pub group_set: i32,
    pub domain_suffix: String,
    pub synth_domains: Vec<cond_domain>,
    pub cond_domains: Vec<cond_domain>,
    pub runfile: String,
    pub lease_change_command: String,
    pub if_names: Vec<Iname>,
    pub if_addrs: Vec<Iname>,
    pub if_except: Vec<Iname>,
    pub dhcp_except: Vec<Iname>,
    pub auth_peers: Vec<Iname>,
    pub tftp_interfaces: Vec<Iname>,
    pub bogus_addr: Vec<BogusAddr>,
    pub ignore_addr: Vec<BogusAddr>,
    pub servers: Vec<server>,
    pub ipsets: Vec<ipsets>,
    pub log_fac: i32,
    pub log_file: String,
    pub max_logs: i32,
    pub cachesize: u32,
    pub ftabsize: i32,
    pub port: i32,
    pub query_port: i32,
    pub min_port: i32,
    pub max_port: i32,
    pub local_ttl: u32,
    pub neg_ttl: u32,
    pub max_ttl: u32,
    pub min_cache_ttl: u32,
    pub max_cache_ttl: u32,
    pub auth_ttl: u32,
    pub dhcp_ttl: u32,
    pub use_dhcp_ttl: u32,
    pub dns_client_id: String,
    pub addn_hosts: Vec<hostsfile>,
    pub dhcp: Vec<DhcpContext>,
    pub dhcp6: Vec<DhcpContext>,
    pub ra_interfaces: Vec<ra_interface>,
    pub dhcp_conf: dhcp_config,
    pub dhcp_opts: Vec<dhcp_opt>,
    pub dhcp_match: Vec<dhcp_opt>,
    pub dhcp_opts6: Vec<dhcp_opt>,
    pub dhcp_match6: Vec<dhcp_opt>,
    pub dhcp_name_match: Vec<dhcp_match_name>,
    pub dhcp_vendors: Vec<dhcp_vendor>,
    pub dhcp_macs: Vec<dhcp_mac>,
    pub boot_config: dhcp_boot,
    pub pxe_services: Vec<pxe_service>,
    pub tag_if: Vec<tag_if>,
    pub override_relays: Vec<AddrList>,
    pub relay4: DhcpRelay,
    pub relay6: DhcpRelay,
    pub delay_conf: delay_config,
    pub _override: i32,
    pub enable_pxe: i32,
    pub doing_ra: i32,
    pub doing_dhcp6: i32,
    pub dhcp_ignore: Vec<dhcp_netid_list>,
    pub dhcp_ignore_names: Vec<dhcp_netid_list>,
    pub dhcp_gen_names: Vec<dhcp_netid_list>,
    pub force_broadcast: Vec<dhcp_netid_list>,
    pub bootp_dynamic: Vec<dhcp_netid_list>,
    pub dhcp_hosts_file: hostsfile,
    pub dhcp_opts_file: hostsfile,
    pub dynamic_dirs: Vec<hostsfile>,
    pub dhcp_max: i32,
    pub tftp_max: i32,
    pub tftp_mtu: i32,
    pub dhcp_server_port: i32,
    pub dhcp_client_port: i32,
    pub start_tftp_port: i32,
    pub end_tftp_port: i32,
    pub min_leasetime: u32,
    pub doctors: Vec<doctor>,
    pub edns_pktsz: u16,
    pub tftp_prefix: String,
    pub if_prefix: Vec<TftpPrefix>,
    pub duid_enterprise: u32,
    pub duid_config_len: u32,
    pub duid_config: Vec<u8>,
    pub dbus_name: String,
    pub dump_file: String,
    pub dump_mask: i32,
    pub soa_sn: u32,
    pub soa_refresh: u32,
    pub soa_retry: u32,
    pub soa_expiry: u32,
    pub metrics: Vec<u32>,
    pub prefix_classes: Vec<prefix_class>,
    pub ds: Vec<ds_config>,
    pub timestamp_file: String,
    pub packet: Vec<u8>,
    pub packet_buff_sz: i32,
    pub namebuff: String,
    pub keyname: String,
    pub workspacename: String,
    pub rr_status: String,
    pub rr_status_sz: i32,
    pub dnssec_no_time_check: i32,
    pub back_to_the_future: i32,
    pub frec_list: Vec<Frec>,
    pub sfds: Vec<ServerFd>,
    pub interfaces: Vec<irec>,
    pub listeners: Vec<listener>,
    pub last_server: Vec<server>,
    pub forwardtime: time_t,
    pub forwardcount: i32,
    pub srv_save: Vec<server>,
    pub packet_len: usize,
    pub rfd_save: Vec<randfd>,
    pub tcp_pids: Vec<pid_t>,
    pub randomsocks: Vec<randfd>,
    pub v6pktinfo: i32,
    pub interface_addrs: Vec<addrlist>,
    pub lot_id: i32,
    pub log_display_id: i32,
    pub log_source_addr: Mysockaddr,
    pub pxefd: i32,
    pub helperfd: i32,
    pub dhcpfd: i32,
    pub inotifyfd: i32,
    pub netlinkfd: i32,
    pub dhcp_raw_fd: i32,
    pub dhcp_icmp_fd: i32,
    pub routefd: i32,
    pub dhcp_packet: iovec,
    pub dhcp_buff: Vec<u8>,
    pub dhcp_buff2: Vec<u8>,
    pub dhcp_buff3: Vec<u8>,
    pub ping_results: Vec<ping_result>,
    pub lease_stream: FILE,
    pub bridges: Vec<dhcp_bridge>,
    pub duid_len: i32,
    pub dui: Vec<u8>,
    pub outpacket: iovec,
    pub dhcp6fd: i32,
    pub icmp6fd: i32,
    pub dbus: c_void,
    pub tftp_trans: Vec<tftp_transfer>,
    pub tftp_done_trans: Vec<tftp_transfer>,
    pub addrbuff: Vec<u8>,
    pub addrbuff2: Vec<u8>,
    pub dumpfd: i32,
}

/* cache.c */
// void
// cache_init();

// void
// next_uid(struct crec* crecp);

// void
// log_query(uint32_t flags, char* name, struct all_addr* addr, char* arg);

// char*
// record_source(uint32_t index);

// char*
// querystr(char* desc, uint16_t type);

// int
// cache_find_non_terminal(char* name, time_t now);

// struct crec*
// cache_find_by_addr(struct crec* crecp, struct all_addr* addr, time_t now, uint32_t prot);

// struct crec*
// cache_find_by_name(struct crec* crecp, char* name, time_t now, uint32_t prot);

// void
// cache_end_insert();

// void
// cache_start_insert();

// struct crec*
// cache_insert(char* name,
//         struct all_addr* addr,
//         time_t now,
//         unsigned long ttl,
//         uint16_t flags);

// void
// cache_reload();

// void
// cache_add_dhcp_entry(char* host_name,
//         int prot,
//         struct all_addr* host_address,
//         time_t ttd);

// struct in_addr
// a_record_from_hosts(char* name, time_t now);

// void
// cache_unhash_dhcp();

// void
// dump_cache(time_t now);

// #ifndef NO_ID

// int
// cache_make_stat(struct txt_record* t);

// //#endif

// char*
// cache_get_name(struct crec* crecp);

// char*
// cache_get_cname_target(struct crec* crecp);

// struct crec*
// cache_enumerate(int init);

// int
// read_hostsfile(char* filename,
//         uint32_t index,
//         int cache_size,
//         struct crec** rhash,
//         int hashsz);

// /* blockdata.c */
// //#ifdef HAVE_DNSSEC
// void blockdata_init(void);
// void blockdata_report(void);
// struct blockdata *blockdata_alloc(char *data, size_t len);
// void *blockdata_retrieve(struct blockdata *block, size_t len, void *data);
// void blockdata_free(struct blockdata *blocks);
// //#endif

// /* domain.c */
// char*
// get_domain(struct in_addr addr);

// //#ifdef HAVE_IPV6

// char*
// get_domain6(struct in6_addr* addr);

// //#endif

// int
// is_name_synthetic(int flags, char* name, struct all_addr* addr);

// int
// is_rev_synth(int flag, struct all_addr* addr, char* name);

// /* rfc1035.c */
// int
// extract_name(struct dns_header* header,
//         size_t plen,
//         unsigned char** pp,
//         char* name,
//         int isExtract,
//         int extrabytes);

// unsigned char*
// skip_name(unsigned char* ansp, struct dns_header* header, size_t plen, int extrabytes);

// unsigned char*
// skip_questions(struct dns_header* header, size_t plen);

// unsigned char*
// skip_section(unsigned char* ansp, int count, struct dns_header* header, size_t plen);

// uint32_t
// extract_request(struct dns_header* header, size_t qlen, char* name, uint16_t* typep);

// size_t
// setup_reply(struct dns_header* header,
//         size_t qlen,
//         struct all_addr* addrp,
//         uint32_t flags,
//         unsigned long ttl);

// int
// extract_addresses(struct dns_header* header,
//         size_t qlen,
//         char* name,
//         time_t now,
//         char** ipsets,
//         int is_sign,
//         int check_rebind,
//         int no_cache_dnssec,
//         int secure,
//         int* doctored);

// size_t
// answer_request(struct dns_header* header,
//         char* limit,
//         size_t qlen,
//         struct in_addr local_addr,
//         struct in_addr local_netmask,
//         time_t now,
//         int ad_reqd,
//         int do_bit,
//         int have_pseudoheader);

// int
// check_for_bogus_wildcard(struct dns_header* header,
//         size_t qlen,
//         char* name,
//         struct bogus_addr* baddr,
//         time_t now);

// int
// check_for_ignored_address(struct dns_header* header,
//         size_t qlen,
//         struct bogus_addr* baddr);

// int
// check_for_local_domain(char* name, time_t now);

// uint32_t
// questions_crc(struct dns_header* header, size_t plen, char* name);

// size_t
// resize_packet(struct dns_header* header,
//         size_t plen,
//         unsigned char* pheader,
//         size_t hlen);

// int
// add_resource_record(struct dns_header* header,
//         char* limit,
//         int* truncp,
//         int nameoffset,
//         unsigned char** pp,
//         unsigned long ttl,
//         int* offset,
//         uint16_t type,
//         uint16_t _class,
//         const char* format,
//         ...);

// unsigned char*
// skip_questions(struct dns_header* header, size_t plen);

// int
// extract_name(struct dns_header* header,
//         size_t plen,
//         unsigned char** pp,
//         char* name,
//         int isExtract,
//         int extrabytes);

// int
// in_arpa_name_2_addr(char* namein, struct all_addr* addrp);

// int
// private_net(struct in_addr addr, int ban_localhost);

// /* auth.c */
// //#ifdef HAVE_AUTH

// size_t
// answer_auth(struct dns_header* header,
//         char* limit,
//         size_t qlen,
//         time_t now,
//         union mysockaddr* peer_addr,
//         int local_query,
//         int do_bit,
//         int have_pseudoheader);

// int
// in_zone(struct auth_zone* zone, char* name, char** cut);

// //#endif

// /* dnssec.c */
// size_t
// dnssec_generate_query(struct dns_header* header,
//         unsigned char* end,
//         char* name,
//         int _class,
//         int type,
//         int edns_pktsz);

// int
// dnssec_validate_by_ds(time_t now,
//         struct dns_header* header,
//         size_t plen,
//         char* name,
//         char* keyname,
//         int _class);

// int
// dnssec_validate_ds(time_t now,
//         struct dns_header* header,
//         size_t plen,
//         char* name,
//         char* keyname,
//         int _class);

// int
// dnssec_validate_reply(time_t now,
//         struct dns_header* header,
//         size_t plen,
//         char* name,
//         char* keyname,
//         int* _class,
//         int check_unsigned,
//         int* neganswer,
//         int* nons);

// int
// dnskey_keytag(int alg, int flags, unsigned char* key, int keylen);

// size_t
// filter_rrsigs(struct dns_header* header, size_t plen);

// unsigned char*
// hash_questions(struct dns_header* header, size_t plen, char* name);

// int
// setup_timestamp();

// /* crypto.c */
// const struct nettle_hash*
// hash_find(char* name);

// int
// hash_init(const struct nettle_hash* hash, void** ctxp, unsigned char** digestp);

// int
// verify(struct blockdata* key_data,
//         uint32_t key_len,
//         unsigned char* sig,
//         size_t sig_len,
//         unsigned char* digest,
//         size_t digest_len,
//         int algo);

// char*
// ds_digest_name(int digest);

// char*
// algo_digest_name(int algo);

// char*
// nsec3_digest_name(int digest);

// /* util.c */
// void
// rand_init();

// uint16_t
// rand16();

// uint32_t
// rand32();

// uint64_t
// rand64();

// int
// legal_hostname(char* name);

// char*
// canonicalise(char* in, int* nomem);

// unsigned char*
// do_rfc1035_name(unsigned char* p, char* sval, char* limit);

// void*
// safe_malloc(size_t size);

// void
// safe_strncpy(char* dest, const char* src, size_t size);

// void
// safe_pipe(int* fd, int read_noblock);

// void*
// whine_malloc(size_t size);

// int
// sa_len(union mysockaddr* addr);

// int
// sockaddr_isequal(union mysockaddr* s1, union mysockaddr* s2);

// int
// hostname_isequal(const char* a, const char* b);

// int
// hostname_issubdomain(char* a, char* b);

// time_t
// dnsmasq_time();

// int
// netmask_length(struct in_addr mask);

// int
// is_same_net(struct in_addr a, struct in_addr b, struct in_addr mask);

// //#ifdef HAVE_IPV6

// int
// is_same_net6(struct in6_addr* a, struct in6_addr* b, int prefixlen);

// uint64_t
// addr6part(struct in6_addr* addr);

// void
// setaddr6part(struct in6_addr* addr, uint64_t host);

// //#endif

// int
// retry_send(ssize_t rc);

// void
// prettyprint_time(char* buf, uint32_t t);

// int
// prettyprint_addr(union mysockaddr* addr, char* buf);

// int
// parse_hex(char* in,
//         unsigned char* out,
//         int maxlen,
//         uint32_t* wildcard_mask,
//         int* mac_type);

// int
// memcmp_masked(unsigned char* a, unsigned char* b, int len, uint32_t mask);

// int
// expand_buf(struct iovec* iov, size_t size);

// char*
// print_mac(char* buff, unsigned char* mac, int len);

// int
// read_write(int fd, unsigned char* packet, int size, int rw);

// int
// wildcard_match(const char* wildcard, const char* match);

// int
// wildcard_matchn(const char* wildcard, const char* match, int num);

// /* log.c */
// void
// die(char* message, char* arg1, int exit_code) ATTRIBUTE_NORETURN;

// int
// log_start(struct passwd* ent_pw, int errfd);

// int
// log_reopen(char* log_file);

// void
// my_syslog(int priority, const char* format, ...);

// void
// set_log_writer();

// void
// check_log_writer(int force);

// void
// flush_log();

// /* option.c */
// void
// read_opts(int argc, char** argv, char* compile_opts);

// char*
// option_string(int prot,
//         uint32_t opt,
//         unsigned char* val,
//         int opt_len,
//         char* buf,
//         int buf_len);

// void
// reread_dhcp();

// void
// read_servers_file();

// void
// set_option_bool(uint32_t opt);

// void
// reset_option_bool(uint32_t opt);

// struct hostsfile*
// expand_filelist(struct hostsfile* list);

// char*
// parse_server(char* arg,
//         union mysockaddr* addr,
//         union mysockaddr* source_addr,
//         std::string _interface,
//         int* flags);

// int
// option_read_dynfile(char* file, int flags);

// /* forward.c */
// void
// reply_query(int fd, int family, time_t now);

// void
// receive_query(struct listener* listen, time_t now);

// unsigned char*
// tcp_request(int confd,
//         time_t now,
//         union mysockaddr* local_addr,
//         struct in_addr netmask,
//         int auth_dns);

// void
// server_gone(struct server* server);

// struct frec*
// get_new_frec(time_t now, int* wait, int force);

// int
// send_from(int fd,
//         int nowild,
//         char* packet,
//         size_t len,
//         union mysockaddr* to,
//         struct all_addr* source,
//         uint32_t iface);

// void
// resend_query();

// struct randfd*
// allocate_rfd(int family);

// void
// free_rfd(struct randfd* rfd);

// /* network.c */
// int
// indextoname(int fd, int index, char* name);

// int
// local_bind(int fd, union mysockaddr* addr, char* intname, uint32_t ifindex, int is_tcp);

// int
// random_sock(int family);

// void
// pre_allocate_sfds();

// int
// reload_servers(char* fname);

// void
// mark_servers(int flag);

// void
// cleanup_servers();

// void
// add_update_server(int flags,
//         union mysockaddr* addr,
//         union mysockaddr* source_addr,
//         const std::string _interface,
//         const char* domain);

// void
// check_servers();

// int
// enumerate_interfaces(int reset);

// void
// create_wildcard_listeners();

// void
// create_bound_listeners(int dienow);

// void
// warn_bound_listeners();

// void
// warn_wild_labels();

// void
// warn_int_names();

// int
// is_dad_listeners();

// int
// iface_check(int family, struct all_addr* addr, char* name, int* auth);

// int
// loopback_exception(int fd, int family, struct all_addr* addr, char* name);

// int
// label_exception(int index, int family, struct all_addr* addr);

// int
// fix_fd(int fd);

// int
// tcp_interface(int fd, int af);

// //#ifdef HAVE_IPV6

// int
// set_ipv6pktinfo(int fd);

// //#endif
// //#ifdef HAVE_DHCP6

// void
// join_multicast(int dienow);

// //#endif
// #if defined(HAVE_LINUX_NETWORK) || defined(HAVE_BSD_NETWORK)

// void
// newaddress(time_t now);

// //#endif

// /* dhcp.c */
// //#ifdef HAVE_DHCP

// void
// dhcp_init();

// void
// dhcp_packet(time_t now, int pxe_fd);

// struct DhcpContext*
// address_available(struct DhcpContext* context,
//         struct in_addr taddr,
//         struct dhcp_netid* netids);

// struct DhcpContext*
// narrow_context(struct DhcpContext* context,
//         struct in_addr taddr,
//         struct dhcp_netid* netids);

// struct ping_result*
// do_icmp_ping(time_t now, struct in_addr addr, uint32_t hash, int loopback);

// int
// address_allocate(struct DhcpContext* context,
//         struct in_addr* addrp,
//         unsigned char* hwaddr,
//         int hw_len,
//         struct dhcp_netid* netids,
//         time_t now,
//         int loopback);

// void
// dhcp_read_ethers();

// struct dhcp_config*
// config_find_by_address(struct dhcp_config* configs, struct in_addr addr);

// char*
// host_from_dns(struct in_addr addr);

// //#endif

// /* lease.c */
// //#ifdef HAVE_DHCP

// void
// lease_update_file(time_t now);

// void
// lease_update_dns(int force);

// void
// lease_init(time_t now);

// struct dhcp_lease*
// lease4_allocate(struct in_addr addr);

// //#ifdef HAVE_DHCP6

// struct dhcp_lease*
// lease6_allocate(struct in6_addr* addrp, int lease_type);

// struct dhcp_lease*
// lease6_find(unsigned char* clid,
//         int clid_len,
//         int lease_type,
//         int iaid,
//         struct in6_addr* addr);

// void
// lease6_reset();

// struct dhcp_lease*
// lease6_find_by_client(struct dhcp_lease* first,
//         int lease_type,
//         unsigned char* clid,
//         int clid_len,
//         int iaid);

// struct dhcp_lease*
// lease6_find_by_addr(struct in6_addr* net, int prefix, uint64_t addr);

// uint64_t
// lease_find_max_addr6(struct DhcpContext* context);

// void
// lease_ping_reply(struct in6_addr* sender, unsigned char* packet, char* interface);

// void
// lease_update_slaac(time_t now);

// void
// lease_set_iaid(struct dhcp_lease* lease, int iaid);

// void
// lease_make_duid(time_t now);

// //#endif

// void
// lease_set_hwaddr(struct dhcp_lease* lease,
//         const unsigned char* hwaddr,
//         const unsigned char* clid,
//         int hw_len,
//         int hw_type,
//         int clid_len,
//         time_t now,
//         int force);

// void
// lease_set_hostname(struct dhcp_lease* lease,
//         const char* name,
//         int auth,
//         char* domain,
//         char* config_domain);

// void
// lease_set_expires(struct dhcp_lease* lease, uint32_t len, time_t now);

// void
// lease_set_interface(struct dhcp_lease* lease, int _interface, time_t now);

// struct dhcp_lease*
// lease_find_by_client(unsigned char* hwaddr,
//         int hw_len,
//         int hw_type,
//         unsigned char* clid,
//         int clid_len);

// struct dhcp_lease*
// lease_find_by_addr(struct in_addr addr);

// struct in_addr
// lease_find_max_addr(struct DhcpContext* context);

// void
// lease_prune(struct dhcp_lease* target, time_t now);

// void
// lease_update_from_configs();

// int
// do_script_run(time_t now);

// void
// rerun_scripts();

// void
// lease_find_interfaces(time_t now);

// //#ifdef HAVE_SCRIPT

// void
// lease_add_extradata(struct dhcp_lease* lease,
//         unsigned char* data,
//         uint32_t len,
//         int delim);

// //#endif
// //#endif

// /* rfc2131.c */
// //#ifdef HAVE_DHCP

// size_t
// dhcp_reply(struct DhcpContext* context,
//         char* iface_name,
//         int int_index,
//         size_t sz,
//         time_t now,
//         int unicast_dest,
//         int loopback,
//         int* is_inform,
//         int pxe,
//         struct in_addr fallback,
//         time_t recvtime);

// unsigned char*
// extended_hwaddr(int hwtype,
//         int hwlen,
//         unsigned char* hwaddr,
//         int clid_len,
//         unsigned char* clid,
//         int* len_out);

// //#endif

// /* dnsmasq.c */
// //#ifdef HAVE_DHCP

// int
// make_icmp_sock();

// int
// icmp_ping(struct in_addr addr);

// int
// delay_dhcp(time_t start, int sec, int fd, uint32_t addr, uint16_t id);

// //#endif

// void
// queue_event(int event);

// void
// send_alarm(time_t event, time_t now);

// void
// send_event(int fd, int event, int data, char* msg);

// void
// clear_cache_and_reload(time_t now);

// /* netlink.c */
// //#ifdef HAVE_LINUX_NETWORK

// void
// netlink_init();

// void
// netlink_multicast();

// //#endif

// /* bpf.c */
// //#ifdef HAVE_BSD_NETWORK
// void init_bpf(void);
// void send_via_bpf(struct dhcp_packet *mess, size_t len,
//           struct in_addr iface_addr, struct ifreq *ifr);
// void route_init(void);
// void route_sock(void);
// //#endif

// /* bpf.c or netlink.c */
// using callback_t = int(*)(struct in_addr*, int, int, int, int, int, int, void*);

// int
// iface_enumerate(int family, void* parm, callback_t callback);

// /* dbus.c */
// //#ifdef HAVE_DBUS
// char *dbus_init(void);
// void check_dbus_listeners(void);
// void set_dbus_listeners(void);
// #  ifdef HAVE_DHCP
// void emit_dbus_signal(int action, struct dhcp_lease *lease, char *hostname);
// #  endif
// //#endif

// /* ubus.c */
// //#ifdef HAVE_UBUS
// void set_ubus_listeners(void);
// void check_ubus_listeners(void);
// void ubus_event_bcast(const char *type, const char *mac, const char *ip, const char *name, const char *interface);
// //#endif

// /* ipset.c */
// //#ifdef HAVE_IPSET

// void
// ipset_init();

// int
// add_to_ipset(const char* setname, const struct all_addr* ipaddr, int flags, int remove);

// //#endif

// /* helper.c */
// #if defined(HAVE_SCRIPT)

// int
// create_helper(int event_fd, int err_fd, uid_t uid, gid_t gid, long max_fd);

// void
// helper_write();

// void
// queue_script(int action, struct dhcp_lease* lease, char* hostname, time_t now);

// //#ifdef HAVE_TFTP

// void
// queue_tftp(off_t file_len, char* filename, union mysockaddr* peer);

// //#endif

// void
// queue_arp(int action, unsigned char* mac, int maclen, int family, struct all_addr* addr);

// int
// helper_buf_empty();

// //#endif

// /* tftp.c */
// //#ifdef HAVE_TFTP

// void
// tftp_request(struct listener* listen, time_t now);

// void
// check_tftp_listeners(time_t now);

// int
// do_tftp_script_run();

// //#endif

// /* conntrack.c */
// //#ifdef HAVE_CONNTRACK
// int get_incoming_mark(union mysockaddr *peer_addr, struct all_addr *local_addr,
//               int istcp, uint32_t *markp);
// //#endif

// /* dhcp6.c */
// //#ifdef HAVE_DHCP6

// void
// dhcp6_init();

// void
// dhcp6_packet(time_t now);

// struct DhcpContext*
// address6_allocate(struct DhcpContext* context,
//         unsigned char* clid,
//         int clid_len,
//         int temp_addr,
//         int iaid,
//         int serial,
//         struct dhcp_netid* netids,
//         int plain_range,
//         struct in6_addr* ans);

// int
// config_valid(struct dhcp_config* config,
//         struct DhcpContext* context,
//         struct in6_addr* addr);

// struct DhcpContext*
// address6_available(struct DhcpContext* context,
//         struct in6_addr* taddr,
//         struct dhcp_netid* netids,
//         int plain_range);

// struct DhcpContext*
// address6_valid(struct DhcpContext* context,
//         struct in6_addr* taddr,
//         struct dhcp_netid* netids,
//         int plain_range);

// struct dhcp_config*
// config_find_by_address6(struct dhcp_config* configs,
//         struct in6_addr* net,
//         int prefix,
//         uint64_t addr);

// void
// make_duid(time_t now);

// void
// dhcp_construct_contexts(time_t now);

// void
// get_client_mac(struct in6_addr* client,
//         int iface,
//         unsigned char* mac,
//         uint32_t* maclenp,
//         uint32_t* mactypep,
//         time_t now);

// //#endif

// /* rfc3315.c */
// //#ifdef HAVE_DHCP6

// uint16_t
// dhcp6_reply(struct DhcpContext* context,
//         int interface,
//         char* iface_name,
//         struct in6_addr* fallback,
//         struct in6_addr* ll_addr,
//         struct in6_addr* ula_addr,
//         size_t sz,
//         struct in6_addr* client_addr,
//         time_t now);

// void
// relay_upstream6(struct dhcp_relay* relay,
//         ssize_t sz,
//         struct in6_addr* peer_address,
//         uint32_t scope_id,
//         time_t now);

// uint16_t
// relay_reply6(struct sockaddr_in6* peer, ssize_t sz, char* arrival_interface);

// //#endif

// /* dhcp-common.c */
// //#ifdef HAVE_DHCP

// void
// dhcp_common_init();

// ssize_t
// recv_dhcp_packet(int fd, struct msghdr* msg);

// struct dhcp_netid*
// run_tag_if(struct dhcp_netid* tags);

// struct dhcp_netid*
// option_filter(struct dhcp_netid* tags,
//         struct dhcp_netid* context_tags,
//         struct dhcp_opt* opts);

// int
// match_netid(struct dhcp_netid* check, struct dhcp_netid* pool, int tagnotneeded);

// char*
// strip_hostname(char* hostname);

// void
// log_tags(struct dhcp_netid* netid, uint32_t xid);

// int
// match_bytes(struct dhcp_opt* o, unsigned char* p, int len);

// void
// dhcp_update_configs(struct dhcp_config* configs);

// void
// display_opts();

// int
// lookup_dhcp_opt(int prot, char* name);

// int
// lookup_dhcp_len(int prot, int val);

// char*
// option_string(int prot,
//         uint32_t opt,
//         unsigned char* val,
//         int opt_len,
//         char* buf,
//         int buf_len);

// struct dhcp_config*
// find_config(struct dhcp_config* configs,
//         struct DhcpContext* context,
//         unsigned char* clid,
//         int clid_len,
//         unsigned char* hwaddr,
//         int hw_len,
//         int hw_type,
//         char* hostname);

// int
// config_has_mac(struct dhcp_config* config, unsigned char* hwaddr, int len, int type);

// //#ifdef HAVE_LINUX_NETWORK

// char*
// whichdevice();

// void
// bindtodevice(char* device, int fd);

// //#endif
// #  ifdef HAVE_DHCP6

// void
// display_opts6();

// #  endif

// void
// log_context(int family, struct DhcpContext* context);

// void
// log_relay(int family, struct DhcpRelay* relay);

// //#endif

// /* outpacket.c */
// //#ifdef HAVE_DHCP6

// void
// end_opt6(int container);

// void
// reset_counter();

// int
// save_counter(int newval);

// void*
// expand(size_t headroom);

// int
// new_opt6(int opt);

// void*
// put_opt6(void* data, size_t len);

// void
// put_opt6_long(uint32_t val);

// void
// put_opt6_short(uint32_t val);

// void
// put_opt6_char(uint32_t val);

// void
// put_opt6_string(char* s);

// //#endif

// /* radv.c */
// //#ifdef HAVE_DHCP6

// void
// ra_init(time_t now);

// void
// icmp6_packet(time_t now);

// time_t
// periodic_ra(time_t now);

// void
// ra_start_unsolicited(time_t now, struct DhcpContext* context);

// //#endif

// /* slaac.c */
// //#ifdef HAVE_DHCP6

// void
// slaac_add_addrs(struct dhcp_lease* lease, time_t now, int force);

// time_t
// periodic_slaac(time_t now, struct dhcp_lease* leases);

// void
// slaac_ping_reply(struct in6_addr* sender,
//         unsigned char* packet,
//         char* interface,
//         struct dhcp_lease* leases);

// //#endif

// /* loop.c */
// //#ifdef HAVE_LOOP

// void
// loop_send_probes();

// int
// detect_loop(char* query, int type);

// //#endif

// /* inotify.c */
// //#ifdef HAVE_INOTIFY

// void
// inotify_dnsmasq_init();

// int
// inotify_check(time_t now);

// void
// set_dynamic_inotify(int flag, int total_size, struct crec** rhash, int revhashsz);

// //#endif

// /* poll.c */
// void
// poll_reset();

// int
// poll_check(int fd, short event);

// void
// poll_listen(int fd, short event);

// int
// do_poll(int timeout);

// /* rrfilter.c */
// size_t
// rrfilter(struct dns_header* header, size_t plen, int mode);

// uint16_t*
// rrfilter_desc(int type);

// int
// expand_workspace(unsigned char*** wkspc, int* szp, int _new);

// /* edns0.c */
// unsigned char*
// find_pseudoheader(struct dns_header* header,
//         size_t plen,
//         size_t* len,
//         unsigned char** p,
//         int* is_sign,
//         int* is_last);

// size_t
// add_pseudoheader(struct dns_header* header,
//         size_t plen,
//         unsigned char* limit,
//         uint16_t udp_sz,
//         int optno,
//         unsigned char* opt,
//         size_t optlen,
//         int set_do,
//         int replace);

// size_t
// add_do_bit(struct dns_header* header, size_t plen, unsigned char* limit);

// size_t
// add_edns0_config(struct dns_header* header,
//         size_t plen,
//         unsigned char* limit,
//         union mysockaddr* source,
//         time_t now,
//         int* check_subnet);

// int
// check_source(struct dns_header* header,
//         size_t plen,
//         unsigned char* pseudoheader,
//         union mysockaddr* peer);

// /* arp.c */
// int
// find_mac(union mysockaddr* addr, unsigned char* mac, int lazy, time_t now);

// int
// do_arp_script_run();

// /* dump.c */
// //#ifdef HAVE_DUMPFILE

// void
// dump_init();

// void
// dump_packet(int mask,
//         void* packet,
//         size_t len,
//         union mysockaddr* src,
//         union mysockaddr* dst);

// //#endif

//
// END OF FILE
//
