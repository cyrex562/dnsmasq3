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
// #pragma once

// const COPYRIGHT :u32 = ;"Copyright (c) 2000-2018 Simon Kelley"



/* We do defines that influence behavior of stdio.h, so complain
   if included too early. */
// #ifdef _STDIO_H
// #  error "Header file stdio.h included too early!"
// #endif

// #ifndef NO_LARGEFILE
// /* Ensure we can use files >2GB (log files may grow this big) */
// #  define _LARGEFILE_SOURCE 1
// #  define _FILE_OFFSET_BITS 64
// #endif

/* Get linux C library versions and define _GNU_SOURCE for kFreeBSD. */
// #if defined(__linux__) || defined(__GLIBC__)
// #  ifndef __ANDROID__
// //#      define _GNU_SOURCE
// #  endif

// #  include <features.h>

// #endif

/* Need these defined early */
// #if defined(__sun) || defined(__sun__)
// #  define _XPG4_2
// #  define __EXTENSIONS__
// #endif

// #if (defined(__GNUC__) && __GNUC__>=3) || defined(__clang__)
// const ATTRIBUTE_NORETURN :u32 = ;__attribute__ ((noreturn))
// #else
// const ATTRIBUTE_NORETURN :u32 = ;
// #endif

// /* get these before config.h  for IPv6 stuff... */
// #include <sys/types.h>

// #include <cstdint>

// #ifdef __linux__

// #include <sys/socket.h>

// #endif

// #ifdef _WIN32

// #include <winsock2.h>

// #endif

// #ifdef __APPLE__
// /* Define before netinet/in.h to select API. OSX Lion onwards. */
// #  define __APPLE_USE_RFC_3542
// #endif
// #ifdef __linux__

// #include <netinet/in.h>

// #endif

// /* Also needed before config.h. */
// #include <getopt.h>

// #include "config.h"
mod config;
// #include "ip6addr.h"
mod ip6addr;
// #include "metrics.h"
mod metrics;

// const countof :u32 = ;(x)      (long)(sizeof(x) / sizeof(x[0]))
// TODO: define or replace

//const MIN :u32 = ;(a, b)        ((a) < (b) ? (a) : (b))
// TODO: define or replace

// #include "dns-protocol.h"
mod dns_protocol;
// #include "dhcp-protocol.h"
mod dhcp_protocol;

// #ifdef HAVE_DHCP6

// #include "dhcp6-protocol.h"
mod dhcp6_protocol;
// #include "radv-protocol.h"
mod radv_protocol;

// #endif

// const gettext_noop :u32 = ;(S) (S)
// #ifndef LOCALEDIR
// #  define _(S) (S)
// #else
// #  include <libintl.h>
// #  include <locale.h>   
// #  define _(S) gettext(S)
// #endif

// #ifdef __linux__

// #include <arpa/inet.h>

// #endif

// #include <sys/stat.h>

// #ifdef __linux__

// #include <sys/ioctl.h>

// #endif
// #if defined(HAVE_SOLARIS_NETWORK)
// #  include <sys/sockio.h>
// #endif
// #ifdef __linux__

// #include <sys/poll.h>
// #include <sys/wait.h>

// #endif

// #include <ctime>

// #ifdef __linux__

// #include <sys/un.h>

// #endif

// #include <climits>

// #ifdef __linux__

// #include <net/if.h>

// #endif
// #if defined(HAVE_SOLARIS_NETWORK) && !defined(ifr_mtu)
// /* Some solaris net/if./h omit this. */
// #  define ifr_mtu  ifr_ifru.ifru_metric
// #endif

// #include <unistd.h>
// #include <cstdio>
// #include <cstring>
// #include <cstdlib>
// #include <fcntl.h>
// #include <cctype>
// #include <csignal>
// #include <cstddef>
// #include <ctime>
// #include <cerrno>
// #include <string>

// #ifdef __linux__

// #include <pwd.h>
// #include <grp.h>

// #endif

// #include <cstdarg>

// #if defined(__OpenBSD__) || defined(__NetBSD__) || defined(__sun__) || defined (__sun) || defined (__ANDROID__)
// #include <netinet/if_ether.h>
// #else
// #ifdef __linux__

// #include <net/ethernet.h>

// #endif
// #endif
// #ifdef __linux__

// #include <net/if_arp.h>
// #include <netinet/in_systm.h>
// #include <netinet/ip.h>

// #endif
// #ifdef HAVE_IPV6

// #include <netinet/ip6.h>

// #endif
// #ifdef __linux__

// #include <netinet/ip_icmp.h>
// #include <sys/uio.h>
// #include <syslog.h>

// #endif

// #include <dirent.h>

// #ifndef HAVE_LINUX_NETWORK
// #ifdef __linux__
// #  include <net/if_dl.h>
// #endif
// #endif

// #if defined(HAVE_LINUX_NETWORK)

// #include <linux/capability.h>

/* There doesn't seem to be a universally-available
   userspace header for these. */
// TODO: replace
   // extern int
// capset(cap_user_header_t header, cap_user_data_t data);

// TODO: replace
// extern int
// capget(cap_user_header_t header, cap_user_data_t data);

const LINUX_CAPABILITY_VERSION_1: u32 = 0x19980330;
const LINUX_CAPABILITY_VERSION_2: u32 = 0x20071026;
const LINUX_CAPABILITY_VERSION_3: u32 = 0x20080522;

// #include <sys/prctl.h>

// #elif defined(HAVE_SOLARIS_NETWORK)
// #include <priv.h>
// #endif

// #ifdef HAVE_DNSSEC
// #  include <nettle/nettle-meta.h>
// #endif

// #include "dnsmasq_sys.h"
mod dnsmasq_sys;

/* daemon is function in the C library.... */
// const daemon :u32 = ;dnsmasq_daemon

/* Async event queue */
struct event_desc {
    msg_sz: i32,
    event: i32,
    data: i32,
}

const EVENT_RELOAD: u32 =     1;
const EVENT_DUMP: u32 =      2;
const EVENT_ALARM: u32 =     3;
const EVENT_TERM: u32 =       4;
const EVENT_CHILD: u32 =      5;
const EVENT_REOPEN: u32 =    6;
const EVENT_EXITED: u32 =     7;
const EVENT_KILLED: u32 =    8;
const EVENT_EXEC_ERR: u32 =   9;
const EVENT_PIPE_ERR: u32 =   10;
const EVENT_USER_ERR: u32 =  11;
const EVENT_CAP_ERR: u32 =   12;
const EVENT_PIDFILE: u32 =   13;
const EVENT_HUSER_ERR: u32 = 14;
const EVENT_GROUP_ERR: u32 = 15;
const EVENT_DIE: u32 =       16;
const EVENT_LOG_ERR: u32 =   17;
const EVENT_FORK_ERR: u32 =  18;
const EVENT_LUA_ERR: u32 =   19;
const EVENT_TFTP_ERR: u32 =  20;
const EVENT_INIT: u32 =      21;
const EVENT_NEWADDR: u32 =   22;
const EVENT_NEWROUTE: u32 =  23;
const EVENT_TIME_ERR: u32 = 24;
const EVENT_SCRIPT_LOG: u32 = 25;
const EVENT_TIME: u32 =       26;

/* Exit codes. */
const EC_GOOD: u32 =        0;
const EC_BADCONF: u32 =     1;
const EC_BADNET: u32 =      2;
const EC_FILE: u32 =        3;
const EC_NOMEM: u32 =       4;
const EC_MISC: u32 =        5;
const EC_INIT_OFFSET: u32 = 10;

/* Trust the compiler dead-code eliminator.... */
// const option_bool :u32 = ;(x) (((x) < 32) ? daemon->options & (1u << (x)) : daemon->options2 & (1u << ((x) - 32)))

const OPT_BOGUSPRIV: u32    =  0;
const OPT_FILTER: u32 =         1;
const OPT_LOG: u32 =            2;
const OPT_SELFMX: u32 =         3;
const OPT_NO_HOSTS: u32 =       4;
const OPT_NO_POLL: u32 =        5;
const OPT_DEBUG: u32 =          6;
const OPT_ORDER: u32 =          7;
const OPT_NO_RESOLV: u32 =      8;
const OPT_EXPAND: u32 =         9;
const OPT_LOCALMX: u32 =        10;
const OPT_NO_NEG: u32 =         11;
const OPT_NODOTS_LOCAL: u32 =   12;
const OPT_NOWILD: u32 =         13;
const OPT_ETHERS: u32 =         14;
const OPT_RESOLV_DOMAIN: u32 =  15;
const OPT_NO_FORK: u32 =        16;
const OPT_AUTHORITATIVE: u32 =  17;
const OPT_LOCALISE :u32 = 18;
const OPT_DBUS :u32 = 19;
const OPT_DHCP_FQDN :u32 = 20;
const OPT_NO_PING :u32 = 21;
const OPT_LEASE_RO :u32 = 22;
const OPT_ALL_SERVERS :u32 = 23;
const OPT_RELOAD :u32 = 24;
const OPT_LOCAL_REBIND :u32 = 25;
const OPT_TFTP_SECURE :u32 = 26;
const OPT_TFTP_NOBLOCK :u32 = 27;
const OPT_LOG_OPTS :u32 = 28;
const OPT_TFTP_APREF_IP :u32 = 29;
const OPT_NO_OVERRIDE :u32 = 30;
const OPT_NO_REBIND :u32 = 31;
const OPT_ADD_MAC :u32 = 32;
const OPT_DNSSEC_PROXY :u32 = 33;
const OPT_CONSEC_ADDR :u32 = 34;
const OPT_CONNTRACK :u32 = 35;
const OPT_FQDN_UPDATE :u32 = 36;
const OPT_RA :u32 = 37;
const OPT_TFTP_LC :u32 = 38;
const OPT_CLEVERBIND :u32 = 39;
const OPT_TFTP :u32 = 40;
const OPT_CLIENT_SUBNET :u32 = 41;
const OPT_QUIET_DHCP :u32 = 42;
const OPT_QUIET_DHCP6 :u32 = 43;
const OPT_QUIET_RA :u32 = 44;
const OPT_DNSSEC_VALID :u32 = 45;
const OPT_DNSSEC_TIME :u32 = 46;
const OPT_DNSSEC_DEBUG :u32 = 47;
const OPT_DNSSEC_IGN_NS :u32 = 48;
const OPT_LOCAL_SERVICE :u32 = 49;
const OPT_LOOP_DETECT :u32 = 50;
const OPT_EXTRALOG :u32 = 51;
const OPT_TFTP_NO_FAIL :u32 = 52;
const OPT_SCRIPT_ARP :u32 = 53;
const OPT_MAC_B64 :u32 = 54;
const OPT_MAC_HEX :u32 = 55;
const OPT_TFTP_APREF_MAC :u32 = 56;
const OPT_RAPID_COMMIT :u32 = 57;
const OPT_UBUS :u32 = 58;
const OPT_LAST :u32 = 59;

/* extra flags for my_syslog, we use a couple of facilities since they are known 
   not to occupy the same bits as priorities, no matter how syslog.h is set up. */
const MS_TFTP :u32 = LOG_USER;
const MS_DHCP :u32 = LOG_DAEMON;
const MS_SCRIPT :u32 = LOG_MAIL;

struct all_addr_log {
    keytag: u16,
    algo: u16,
    digest: u16,
}

struct all_addr_rcode {
    rcode: u32,
}

struct all_addr_dnssec {
    _class: u16,
    _type: u16,
}

struct all_addr_addr {
    addr4: in_addr,
    addr6: in6_addr,
    log: all_addr_log,
    rcode: all_addr_rcode,
    dnssec: all_addr_dnssec,
}



struct all_addr {
    addr: all_addr_addr
}

struct bogus_addr {
    addr: in_addr,
    // next: &bogus_addr,;
}

/* dns doctor param */
struct doctor {
    _in: in_addr,
    end: in_addr,
    out: in_addr,
    mask: in_addr,
    // next: doctor*
}

struct mx_srv_record {
    name: String,
    target: String,
    issrv: i32,
    srvport: i32,
    priority: i32,
    weight: i32,
    offset: u32,
    // next: mx_srv_record
}

struct naptr {
    name: String,
    replace: String,
    regexp: String,
    services: String,
    flages: String,
    pref: String,
    order: String,
    // next: naptr*
}

const TXT_STAT_CACHESIZE :u32 = 1;
const TXT_STAT_INSERTS :u32 = 2;
const TXT_STAT_EVICTIONS :u32 = 3;
const TXT_STAT_MISSES :u32 = 4;
const TXT_STAT_HITS :u32 = 5;
const TXT_STAT_AUTH :u32 = 6;
const TXT_STAT_SERVERS :u32 = 7;


struct txt_record {
    name: String,
    txt: String,
    _class: u16,
    len: u16,
    stat: i32,
    // next: txt_record*
}

struct ptr_record {
    name: String,
    ptr: String,
    // next: ptr_record*
}

struct cname {
    flag: i32,
    ttl: i32,
    alias: String,
    target: String,
    // next: cname*
    // targetp: cname*
}

struct ds_config {
    name: String,
    digest: String,
    digestlen: i32,
    _class: i32,
    algo: i32,
    keytag: i32,
    digest_type: i32,
    // next: ds_config*
}

const ADDRLIST_LITERAL :u32 = 1;
const ADDRLIST_IPV6 :u32 = 2;
const ADDRLIST_REVONLY :u32 = 4;

struct addrlist {
    addr: all_addr,
    flags: i32,
    prefixlen: i32,
    // next: addrlist*
}

const AUTH6 :u32 = 1;
const AUTH4 :u32 = 2;

struct auth_name_list {
    name: String,
    flags: i32,
    // next: auth_name_list*
}

struct auth_zone {
    domain: String,
    interface_names: Vec<auth_name_list>,
    subnet: addrlist,
    exclude: addrlist,
    //next: auth_zone*
}

struct name_list {
    name: String,
    // next: name_list*
}

struct host_record {
    ttl: i32,
    names: Vec<name_list>,
    addr: in_addr,
    addr6: in6_addr,
    // next: host_record*
}

struct interface_name {
    name: String,
    intr: String,
    family: i32,
    // TODO: should this be a list of addrlist structs or a pointer to one?
    addr: Vec<addrlist>,
    // next: interface_name*
}

union bigname {
    name: String,
    // next: bigname*
}

struct blockdata {
    // next blockdata*
    key: [u8;KEYBLOCK_LEN],
}

struct crec_target {
    // TODO: should cache and int_name be vectors or just pointers?
    cache: Vec<crec>
    int_name: Vec<interface_name>,
}

struct crec_cname {
    target: crec_target,
    uid: u32,
}

struct crec_key {
    keydata: Vec<blockdata>,
    keylen: u16,
    flags: u16,
    keytag: u16,
    algo: u8,
}

struct crec_ds {
    keydata: Vec<blockdata>,
    keylen: u16,
    keytag: u16,
    algo:u8,
    digest: u8,
}

struct crec_addr {
    addr: all_addr,
    cname: crec_cname,
    key: crec_key,
    ds: crec_ds,
}

struct crec_name {
    sname: String,
    bname: bigname,
    namep: String
}

struct crec {
    // hash_next: crec*
    // prev: crec*
    // next: crec*
    addr: crec_addr,
    ttd: time_t,
    uid: u32,
    flags: u16,
    name: crec_name,
}

// TODO: implement
// const SIZEOF_BARE_CREC = (sizeof(struct crec) - SMALLDNAME)
// TODO: implement
// const SIZEOF_POINTER_CREC = (sizeof(struct crec) + sizeof(char *) - SMALLDNAME)

const F_IMMORTAL :u32 = 1;
const F_NAMEP :u32 = 2;
const F_REVERSE :u32 = 4;
const F_FORWARD :u32 = 8;
const F_DHCP :u32 = 16;
const F_NEG :u32 = 32;
const F_HOSTS :u32 = 64;
const F_IPV4 :u32 = 128;
const F_IPV6 :u32 = 256;
const F_BIGNAME :u32 = 512;
const F_NXDOMAIN :u32 = 1024;
const F_CNAME :u32 = 2048;
const F_DNSKEY :u32 = 4096;
const F_CONFIG :u32 = 8192;
const F_DS :u32 = 16384;
const F_DNSSECOK :u32 = 32768;

/* below here are only valid as args to log_query: cache
   entries are limited to 16 bits */
const F_UPSTREAM :u32 = 65536;
const F_RRNAME :u32 = 131072;
const F_SERVER :u32 = 262144;
const F_QUERY :u32 = 524288;
const F_NOERR :u32 = 1048576;
const F_AUTH :u32 = 2097152;
const F_DNSSEC :u32 = 4194304;
const F_KEYTAG :u32 = 8388608;
const F_SECSTAT :u32 = 16777216;
const F_NO_RR :u32 = 33554432;
const F_IPSET :u32 = 67108864;
const F_NOEXTRA :u32 = 134217728;
const F_SERVFAIL :u32 = 268435456;
const F_RCODE :u32 = 536870912;

const UID_NONE :u32 = 0;
/* Values of uid in crecs with F_CONFIG bit set. */
/* cname to uid SRC_INTERFACE are to interface names,
   so use UID_NONE for that to eliminate clashes with
   any other uid */
const SRC_INTERFACE :u32 = UID_NONE;
const SRC_CONFIG :u32 = 1;
const SRC_HOSTS :u32 = 2;
const SRC_AH :u32 = 3;

/* struct sockaddr is not large enough to hold any address,
   and specifically not big enough to hold an IPv6 address.
   Blech. Roll our own. */
struct mysockaddr {
    sa: sockaddr,
    in4: sockaddr_in,
    in6: sockaddr_in6,

}

/* bits in flag param to IPv6 callbacks from iface_enumerate() */
const IFACE_TENTATIVE :u32 = 1;
const IFACE_DEPRECATED :u32 = 2;
const IFACE_PERMANENT :u32 = 4;

const SERV_FROM_RESOLV :u32 = 1;  /* 1 for servers from resolv, 0 for command line. */
const SERV_NO_ADDR :u32 = 2;  /* no server, this domain is local only */
const SERV_LITERAL_ADDRESS :u32 = 4;  /* addr is the answer, not the server */
const SERV_HAS_DOMAIN :u32 = 8;  /* server for one domain only */
const SERV_HAS_SOURCE :u32 = 16;  /* source address defined */
const SERV_FOR_NODOTS :u32 = 32;  /* server for names with no domain part only */
const SERV_WARNED_RECURSIVE :u32 = 64;  /* avoid warning spam */
const SERV_FROM_DBUS :u32 = 128;  /* 1 if source is DBus */
const SERV_MARK :u32 = 256;  /* for mark-and-delete */
const SERV_TYPE :u32 = (SERV_HAS_DOMAIN | SERV_FOR_NODOTS);
const SERV_COUNTED :u32 = 512;  /* workspace for log code */
const SERV_USE_RESOLV :u32 = 1024;  /* forward this domain in the normal way */
const SERV_NO_REBIND :u32 = 2048;  /* inhibit dns-rebind protection */
const SERV_FROM_FILE :u32 = 4096;  /* read from --servers-file */
const SERV_LOOP :u32 = 8192;  /* server causes forwarding loop */
const SERV_DO_DNSSEC :u32 = 16384;  /* Validate DNSSEC when using this server */
const SERV_GOT_TCP :u32 = 32768;  /* Got some data from the TCP connection */

struct ServerFd {
    fd: i32,
    source_addr: mysockaddr,
    _interface: String,
    preallocated: u32,
    used: u32,
    ifindex: u32,
    // next: ServerFd*
}

struct randfd {
    fd: i32,
    refcount: u16,
    family: u16,
}

struct server {
    addr: mysockaddr,
    source_addr: mysockaddr,
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

struct ipsets {
    sets: Vec<String>,
    domain: String,
    // next: ipsets*
}

struct irec {
    addr: mysockaddr,
    netmask: in_addr,
    tftp_ok: i32,
    dhcp_ok: i32,
    mtu: i32,
    done: i32,
    warned: i32,
    dad: i32,
    dns_auth:i32,
    index: i32,
    multicast_done: i32,
    found: i32,
    label: i32,
    name: String,
    // next: irec*
}

struct listener {
    fd: i32,
    tcpfd: i32,
    tftpfd: i32,
    family: i32,
    iface: irec, // TODO: ptr?
    // next: listener*
}

/* interface and address parms from command line. */
struct Iname {
    name: String,
    addr: mysockaddr,
    used: i32,
    // next: Iname*
}

/* subnet parameters from command line */
struct mysubnet {
    addr: mysockaddr,
    addr_used: i32,
    mask: i32,
}

/* resolv-file parms from command-line */
struct resolvc {
    // next: resolvc*
    is_default: i32,
    logged: i32,
    mtime: time_t,
    name: String,
    wd: i32,
    file: String,
}

/* adn-hosts parms from command-line (also dhcp-hostsfile and dhcp-optsfile and dhcp-hostsdir*/
const AH_DIR :u32 = 1;
const AH_INACTIVE :u32 = 2;
const AH_WD_DONE :u32 = 4;
const AH_HOSTS :u32 = 8;
const AH_DHCP_HST :u32 = 16;
const AH_DHCP_OPT :u32 = 32;

struct hostsfile {
    // next: hostsfile*
    flags: i32,
    fname: String,
    wd: i32,
    index: u32,
}

/* packet-dump flags */
const DUMP_QUERY :u32 = 0x0001;
const DUMP_REPLY :u32 = 0x0002;
const DUMP_UP_QUERY :u32 = 0x0004;
const DUMP_UP_REPLY :u32 = 0x0008;
const DUMP_SEC_QUERY :u32 = 0x0010;
const DUMP_SEC_REPLY :u32 = 0x0020;
const DUMP_BOGUS :u32 = 0x0040;
const DUMP_SEC_BOGUS :u32 = 0x0080;


/* DNSSEC status values. */
const STAT_SECURE :u32 = 1;
const STAT_INSECURE :u32 = 2;
const STAT_BOGUS :u32 = 3;
const STAT_NEED_DS :u32 = 4;
const STAT_NEED_KEY :u32 = 5;
const STAT_TRUNCATED :u32 = 6;
const STAT_SECURE_WILDCARD :u32 = 7;
const STAT_OK :u32 = 8;
const STAT_ABANDONED :u32 = 9;

const FREC_NOREBIND :u32 = 1;
const FREC_CHECKING_DISABLED :u32 = 2;
const FREC_HAS_SUBNET :u32 = 4;
const FREC_DNSKEY_QUERY :u32 = 8;
const FREC_DS_QUERY :u32 = 16;
const FREC_AD_QUESTION :u32 = 32;
const FREC_DO_QUESTION :u32 = 64;
const FREC_ADDED_PHEADER :u32 = 128;
const FREC_TEST_PKTSZ :u32 = 256;
const FREC_HAS_EXTRADATA :u32 = 512;


const HASH_SIZE_DNSSEC :u32 = 20; /* SHA-1 digest size */
const HASH_SIZE :u32 = 32;


struct frec {
    source: mysockaddr,
    dest: all_addr,
    sentto: Server, // TODO ptr?
    rfd64: randfd, // TODO: ptr?
    rfd6: randfd, // TODO: ptr?
    iface: u32,
    orig_id: u32,
    new_id: u32,
    log_id: i32,
    fd: i32,
    forwardall: i32,
    flags: i32,
    time: time_t,
    hash: [u8;HASH_SIZE],
    _class: i32,
    work_counter: i32,
    stash: Vec<blockdata>,
    stash_len: usize,
    // dependent: frec, // TODO: ptr?
    // blocking_query: frec // TODO: ptr?
    // next: frec*
}

/* flags in top of length field for DHCP-option tables */
const OT_ADDR_LIST :u32 = 0x8000;
const OT_RFC1035_NAME :u32 = 0x4000;
const OT_INTERNAL :u32 = 0x2000;
const OT_NAME :u32 = 0x1000;
const OT_CSTRING :u32 = 0x0800;
const OT_DEC :u32 = 0x0400;
const OT_TIME :u32 = 0x0200;

/* actions in the daemon->helper RPC */
const ACTION_DEL :u32 = 1;
const ACTION_OLD_HOSTNAME :u32 = 2;
const ACTION_OLD :u32 = 3;
const ACTION_ADD :u32 = 4;
const ACTION_TFTP :u32 = 5;
const ACTION_ARP :u32 = 6;
const ACTION_ARP_DEL :u32 = 7;

const LEASE_NEW :u32 = 1;  /* newly created */
const LEASE_CHANGED :u32 = 2;  /* modified */
const LEASE_AUX_CHANGED :u32 = 4;  /* CLID or expiry changed */
const LEASE_AUTH_NAME :u32 = 8;  /* hostname came from config, not from client */
const LEASE_USED :u32 = 16;  /* used this DHCPv6 transaction */
const LEASE_NA :u32 = 32;  /* IPv6 no-temporary lease */
const LEASE_TA :u32 = 64;  /* IPv6 temporary lease */
const LEASE_HAVE_HWADDR :u32 = 128;  /* Have set hwaddress */


struct slaac_address {
    addr: in6_addr,
    ping_time: time_t,
    backoff: i32,
    // next: slaac_address*
}

struct dhcp_lease {
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
    hwaddr: [u8;DHCP_CHADDR_MAX],
    addr: in_addr,
    _override: in_addr,
    giaddr: in_addr,
    extradata: Vec<u8>,
    extradata_len: u32,
    extradata_size: u32,
    last_interface: i32,
    new_interface: i32,
    new_prefixlen: i32,
    addr6: in6_addr,
    iaid: i32,
    slaac_addr: slaac_address,
    vendorclass_count: i32,
    // next: dhcp_lease*
}

struct dhcp_netid {
    net: String,
    // next: dhcp_netid*
}

struct dhcp_netid_list {
    list: Vec<dhcp_netid>,
    // next: dhcp_netid_list
}

struct tag_if {
    set: dhcp_netid_list,
    tag: dhcp_netid, // TODO: ptr,
    // next: tag_if*
}

struct delay_config {
    delay: i32,
    netid: dhcp_netid // todo: ptr,
    // next: delay_config*
}

struct hwaddr_config {
    hwaddr_len: i32,
    hwaddr_type: i32,
    hwaddr: [u8;DHCP_CHADDR_MAX],
    wildcard_mask: u32,
    // next: hwaddr_config*
}

struct dhcp_config {
    flags: u32,
    clid_len: i32,
    clid: Vec<u8>,
    hostname: String,
    domain: String,
    netid: dhcp_netid_list,
    addr6: in6_addr,
    addr: in_addr,
    decline_time: time_t,
    lease_time: u32,
    hwaddr: hwaddr_config,
    // next: dhcp_config*
}

// todo: re-implement
//const have_config :u32 = ;(config, mask) ((config) && ((config)->flags & (mask)))

const CONFIG_DISABLE :u32 = 1;
const CONFIG_CLID :u32 = 2;
const CONFIG_TIME :u32 = 8;
const CONFIG_NAME :u32 = 16;
const CONFIG_ADDR :u32 = 32;
const CONFIG_NOCLID :u32 = 128;
const CONFIG_FROM_ETHERS :u32 = 256;    /* entry created by /etc/ethers */
const CONFIG_ADDR_HOSTS :u32 = 512;    /* address added by from /etc/hosts */
const CONFIG_DECLINED :u32 = 1024;    /* address declined by client */
const CONFIG_BANK :u32 = 2048;    /* from dhcp hosts file */
const CONFIG_ADDR6 :u32 = 4096;
const CONFIG_WILDCARD :u32 = 8192;

struct dhcp_opt_u {
    encap: i32,
    wildcard_mask: u32,
    vendor_class: Vec<u8>,
}

struct dhcp_opt {
    opt: i32,
    len: i32,
    flags: i32,
    u: dhcp_opt_u,
    val: Vec<u8>,
    netid: dhcp_netid,
    // next: dhcp_opt,
}

const DHOPT_ADDR :u32 = 1;
const DHOPT_STRING :u32 = 2;
const DHOPT_ENCAPSULATE :u32 = 4;
const DHOPT_ENCAP_MATCH :u32 = 8;
const DHOPT_FORCE :u32 = 16;
const DHOPT_BANK :u32 = 32;
const DHOPT_ENCAP_DONE :u32 = 64;
const DHOPT_MATCH :u32 = 128;
const DHOPT_VENDOR :u32 = 256;
const DHOPT_HEX :u32 = 512;
const DHOPT_VENDOR_MATCH :u32 = 1024;
const DHOPT_RFC3925 :u32 = 2048;
const DHOPT_TAGOK :u32 = 4096;
const DHOPT_ADDR6 :u32 = 8192;

struct dhcp_boot {
    file: String,
    sname: String,
    tftp_sname: String,
    next_server: in_addr,
    netid: dhcp_netid,
    // next: dhcp_boot*
}

struct dhcp_match_name {
    name: String,
    wildcard: i32,
    netid: dhcp_netid,
    // next: dhcp_match_name*
}

struct pxe_service {
    CSA: u16,
    _type: u16,
    menu: String,
    basename: String,
    sname: String,
    server: in_addr,
    netid: dhcp_netid,
    // next: pxe_service*
}

const MATCH_VENDOR :u32 = 1;
const MATCH_USER :u32 = 2;
const MATCH_CIRCUIT :u32 = 3;
const MATCH_REMOTE :u32 = 4;
const MATCH_SUBSCRIBER :u32 = 5;

/* vendorclass, userclass, remote-id or circuit-id */
struct dhcp_vendor {
    len: i32,
    match_type: i32,
    enterprise: u32,
    data: String,
    netid: dhcp_netid,
    // next: dhcp_vendor*
}

struct dhcp_mac {
    mask: u32,
    hwaddr_len: i32,
    hwaddr_type: i32,
    hwaddr: [u8;DHCP_CHADDR_MAX],
    netid: dhcp_netid,
    // next: dhcp_mac*
}

struct dhcp_bridge {
    iface: String,
    // alias: dhcp_bridge*
    // next: dhcp_bridge*
}

struct cond_domain {
    domain: String,
    prefix: String,
    start: in_addr,
    end: in_addr,
    start6: in6_addr,
    end6: in6_addr,
    is6: i32,
    indexed: i32,
    // next: cond_domain*
}

// #ifdef OPTION6_PREFIX_CLASS
struct prefix_class {
    _class: i32,
    tag: dhcp_netid,
    // next: prefix_class*
}
// #endif

struct ra_interface {
    name: String,
    mtu_name: String,
    interval: i32,
    lifetime: i32,
    prio: i32,
    mtu: i32,
    // next: ra_interface*
}

struct DhcpContext {
    lease_time: u32,
    addr_epoch: u32,
    netmask : in_addr,
    broadcast: in_addr,
    local: in_addr,
    router: in_addr,
    start: in_addr,
    end: in_addr,
    start6: in6_addr,
    end6: in6_addr,
    local6: in6_addr,
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

const CONTEXT_STATIC :u32 = 1;
const CONTEXT_NETMASK :u32 = 2;
const CONTEXT_BRDCAST :u32 = 4;
const CONTEXT_PROXY :u32 = 8;
const CONTEXT_RA_ROUTER :u32 = 16;
const CONTEXT_RA_DONE :u32 = 32;
const CONTEXT_RA_NAME :u32 = 64;
const CONTEXT_RA_STATELESS :u32 = 128;
const CONTEXT_DHCP :u32 = 256;
const CONTEXT_DEPRECATE :u32 = 512;
const CONTEXT_TEMPLATE :u32 = 1024;    /* create contexts using addresses */
const CONTEXT_CONSTRUCTED :u32 = 2048;
const CONTEXT_GC :u32 = 4096;
const CONTEXT_RA :u32 = 8192;
const CONTEXT_CONF_USED :u32 = 16384;
const CONTEXT_USED :u32 = 32768;
const CONTEXT_OLD :u32 = 65536;
const CONTEXT_V6 :u32 = 131072;
const CONTEXT_RA_OFF_LINK :u32 = 262144;

struct ping_result {
    addr: in_addr,
    time: time_t,
    hash: u32,
    // next: ping_result*
}

struct tftp_file {
    refcount: i32,
    fd: i32,
    size: off_t,
    dev: dev_t,
    inode: ino_t,
    filename: String,
}

struct tftp_transfer {
    sockfd: i32,
    timeout: time_t,
    backoff: i32,
    expansion: u32,
    blocksize: u32,
    block: u32,
    peer: mysockaddr,
    carrylf: u8,
    netascii: u8,
    opt_transize: u8,
    opt_blocksize: u8,
    file: tftp_file,
    // enxt: tftp_transfer
}

struct AddrList {
    addr: in_addr,
    // next: AddrList*
}

struct TftpPrefix {
    _interface: String,
    prefix: String,
    missing: i32,
    // next: TftpPrefix*
}

struct DhcpRelay {
    server: all_addr,
    local: all_addr,
    _interface: String,
    iface_index: i32,
    // next: DhcpRelay*
    // current: DhcpRelay*
};

extern struct daemon {
    /* datastuctures representing the command-line and 
       config file arguments. All set (including defaults)
       in option.c */
    options2: u32,
    /* datastuctures representing the command-line and
           config file arguments. All set (including defaults)
           in option.c */
    options: u32,
    resolv_files: resolvc, // todo: ptr?
    default_resolv: resolvc, // todo: ptr?

    struct resolvc* resolv_files;
    struct resolvc default_resolv;
    time_t last_resolv;
    char* servers_file;
    struct mx_srv_record* mxnames;
    struct naptr* naptr;
    struct txt_record* rr;
    struct txt_record* txt;
    struct ptr_record* ptr;
    struct host_record* host_records_tail;
    struct host_record* host_records;
    struct cname* cnames;
    struct auth_zone* auth_zones;
    struct interface_name* int_names;
    char* mxtarget;
    struct mysubnet* add_subnet4;
    struct mysubnet* add_subnet6;
    char* lease_file;
    char* scriptuser;
    char* groupname;
    char* username;
    char* luascript;
    char* hostmaster;
    char* authserver;
    struct Iname* authinterface;
    struct name_list* secondary_forward_server;
    int group_set, osport;
    char* domain_suffix;
    struct cond_domain* synth_domains;
    struct cond_domain* cond_domain;
    char* runfile;
    char* lease_change_command;
    struct Iname* if_names, * if_addrs, * if_except, * dhcp_except, * auth_peers,
            * tftp_interfaces;
    struct bogus_addr* bogus_addr, * ignore_addr;
    struct server* servers;
    struct ipsets* ipsets;
    int log_fac; /* log facility */
    char* log_file; /* optional log file */
    int max_logs;  /* queue limit */
    int cachesize, ftabsize;
    int port, query_port, min_port, max_port;
    unsigned long local_ttl, neg_ttl, max_ttl, min_cache_ttl, max_cache_ttl, auth_ttl,
            dhcp_ttl, use_dhcp_ttl;
    char* dns_client_id;
    struct hostsfile* addn_hosts;
    struct DhcpContext* dhcp, * dhcp6;
    struct ra_interface* ra_interfaces;
    struct dhcp_config* dhcp_conf;
    struct dhcp_opt* dhcp_opts, * dhcp_match, * dhcp_opts6, * dhcp_match6;
    struct dhcp_match_name* dhcp_name_match;
    struct dhcp_vendor* dhcp_vendors;
    struct dhcp_mac* dhcp_macs;
    struct dhcp_boot* boot_config;
    struct pxe_service* pxe_services;
    struct tag_if* tag_if;
    struct AddrList* override_relays;
    struct DhcpRelay* relay4, * relay6;
    struct delay_config* delay_conf;
    int override;
    int enable_pxe;
    int doing_ra, doing_dhcp6;
    struct dhcp_netid_list* dhcp_ignore, * dhcp_ignore_names, * dhcp_gen_names;
    struct dhcp_netid_list* force_broadcast, * bootp_dynamic;
    struct hostsfile* dhcp_hosts_file, * dhcp_opts_file, * dynamic_dirs;
    int dhcp_max, tftp_max, tftp_mtu;
    int dhcp_server_port, dhcp_client_port;
    int start_tftp_port, end_tftp_port;
    uint32_t min_leasetime;
    struct doctor* doctors;
    uint16_t edns_pktsz;
    char* tftp_prefix;
    struct TftpPrefix* if_prefix; /* per-interface TFTP prefixes */
    uint32_t duid_enterprise, duid_config_len;
    unsigned char* duid_config;
    char* dbus_name;
    char* dump_file;
    int dump_mask;
    unsigned long soa_sn, soa_refresh, soa_retry, soa_expiry;
    uint32_t metrics[__METRIC_MAX];
#ifdef OPTION6_PREFIX_CLASS
    struct prefix_class *prefix_classes;
#endif
#ifdef HAVE_DNSSEC
    struct ds_config *ds;
    char *timestamp_file;
#endif

    /* globally used stuff for DNS */
    char* packet; /* packet buffer */
    int packet_buff_sz; /* size of above */
    char* namebuff; /* MAXDNAME size buffer */
#ifdef HAVE_DNSSEC
    char *keyname; /* MAXDNAME size buffer */
    char *workspacename; /* ditto */
    char *rr_status; /* flags for individual RRs */
    int rr_status_sz;
    int dnssec_no_time_check;
    int back_to_the_future;
#endif
    struct frec* frec_list;
    struct ServerFd* sfds;
    struct irec* interfaces;
    struct listener* listeners;
    struct server* last_server;
    time_t forwardtime;
    int forwardcount;
    struct server* srv_save; /* Used for resend on DoD */
    size_t packet_len;       /*      "        "        */
    struct randfd* rfd_save; /*      "        "        */
    pid_t tcp_pids[MAX_PROCS];
    struct randfd randomsocks[RANDOM_SOCKS];
    int v6pktinfo;
    struct addrlist*
            interface_addrs; /* list of all addresses/prefix lengths associated with all local interfaces */
    int log_id, log_display_id; /* ids of transactions for logging */
    union mysockaddr* log_source_addr;

    /* DHCP state */
    int pxefd;
    /* DHCP state */
    int helperfd;
    /* DHCP state */
    int dhcpfd;
#ifdef HAVE_INOTIFY
    int inotifyfd;
#endif
#if defined(HAVE_LINUX_NETWORK)
    int netlinkfd;
#elif defined(HAVE_BSD_NETWORK)
    int dhcp_raw_fd, dhcp_icmp_fd, routefd;
#endif
    struct iovec dhcp_packet;
    char* dhcp_buff, * dhcp_buff2, * dhcp_buff3;
    struct ping_result* ping_results;
    FILE* lease_stream;
    struct dhcp_bridge* bridges;
#ifdef HAVE_DHCP6
    int duid_len;
    unsigned char* duid;
    struct iovec outpacket;
    int dhcp6fd, icmp6fd;
#endif
    /* DBus stuff */
    /* void * here to avoid depending on dbus headers outside dbus.c */
    void* dbus;
#ifdef HAVE_DBUS
    struct watch *watches;
#endif

    /* TFTP stuff */
    struct tftp_transfer* tftp_trans, * tftp_done_trans;

    /* utility string buffer, hold max sized IP address as string */
    char* addrbuff;
    char* addrbuff2; /* only allocated when OPT_EXTRALOG */

#ifdef HAVE_DUMPFILE
    /* file for packet dumps. */
    int dumpfd;
#endif
} * daemon;

/* cache.c */
void
cache_init();

void
next_uid(struct crec* crecp);

void
log_query(uint32_t flags, char* name, struct all_addr* addr, char* arg);

char*
record_source(uint32_t index);

char*
querystr(char* desc, uint16_t type);

int
cache_find_non_terminal(char* name, time_t now);

struct crec*
cache_find_by_addr(struct crec* crecp, struct all_addr* addr, time_t now, uint32_t prot);

struct crec*
cache_find_by_name(struct crec* crecp, char* name, time_t now, uint32_t prot);

void
cache_end_insert();

void
cache_start_insert();

struct crec*
cache_insert(char* name,
        struct all_addr* addr,
        time_t now,
        unsigned long ttl,
        uint16_t flags);

void
cache_reload();

void
cache_add_dhcp_entry(char* host_name,
        int prot,
        struct all_addr* host_address,
        time_t ttd);

struct in_addr
a_record_from_hosts(char* name, time_t now);

void
cache_unhash_dhcp();

void
dump_cache(time_t now);

#ifndef NO_ID

int
cache_make_stat(struct txt_record* t);

#endif

char*
cache_get_name(struct crec* crecp);

char*
cache_get_cname_target(struct crec* crecp);

struct crec*
cache_enumerate(int init);

int
read_hostsfile(char* filename,
        uint32_t index,
        int cache_size,
        struct crec** rhash,
        int hashsz);

/* blockdata.c */
#ifdef HAVE_DNSSEC
void blockdata_init(void);
void blockdata_report(void);
struct blockdata *blockdata_alloc(char *data, size_t len);
void *blockdata_retrieve(struct blockdata *block, size_t len, void *data);
void blockdata_free(struct blockdata *blocks);
#endif

/* domain.c */
char*
get_domain(struct in_addr addr);

#ifdef HAVE_IPV6

char*
get_domain6(struct in6_addr* addr);

#endif

int
is_name_synthetic(int flags, char* name, struct all_addr* addr);

int
is_rev_synth(int flag, struct all_addr* addr, char* name);

/* rfc1035.c */
int
extract_name(struct dns_header* header,
        size_t plen,
        unsigned char** pp,
        char* name,
        int isExtract,
        int extrabytes);

unsigned char*
skip_name(unsigned char* ansp, struct dns_header* header, size_t plen, int extrabytes);

unsigned char*
skip_questions(struct dns_header* header, size_t plen);

unsigned char*
skip_section(unsigned char* ansp, int count, struct dns_header* header, size_t plen);

uint32_t
extract_request(struct dns_header* header, size_t qlen, char* name, uint16_t* typep);

size_t
setup_reply(struct dns_header* header,
        size_t qlen,
        struct all_addr* addrp,
        uint32_t flags,
        unsigned long ttl);

int
extract_addresses(struct dns_header* header,
        size_t qlen,
        char* name,
        time_t now,
        char** ipsets,
        int is_sign,
        int check_rebind,
        int no_cache_dnssec,
        int secure,
        int* doctored);

size_t
answer_request(struct dns_header* header,
        char* limit,
        size_t qlen,
        struct in_addr local_addr,
        struct in_addr local_netmask,
        time_t now,
        int ad_reqd,
        int do_bit,
        int have_pseudoheader);

int
check_for_bogus_wildcard(struct dns_header* header,
        size_t qlen,
        char* name,
        struct bogus_addr* baddr,
        time_t now);

int
check_for_ignored_address(struct dns_header* header,
        size_t qlen,
        struct bogus_addr* baddr);

int
check_for_local_domain(char* name, time_t now);

uint32_t
questions_crc(struct dns_header* header, size_t plen, char* name);

size_t
resize_packet(struct dns_header* header,
        size_t plen,
        unsigned char* pheader,
        size_t hlen);

int
add_resource_record(struct dns_header* header,
        char* limit,
        int* truncp,
        int nameoffset,
        unsigned char** pp,
        unsigned long ttl,
        int* offset,
        uint16_t type,
        uint16_t _class,
        const char* format,
        ...);

unsigned char*
skip_questions(struct dns_header* header, size_t plen);

int
extract_name(struct dns_header* header,
        size_t plen,
        unsigned char** pp,
        char* name,
        int isExtract,
        int extrabytes);

int
in_arpa_name_2_addr(char* namein, struct all_addr* addrp);

int
private_net(struct in_addr addr, int ban_localhost);

/* auth.c */
#ifdef HAVE_AUTH

size_t
answer_auth(struct dns_header* header,
        char* limit,
        size_t qlen,
        time_t now,
        union mysockaddr* peer_addr,
        int local_query,
        int do_bit,
        int have_pseudoheader);

int
in_zone(struct auth_zone* zone, char* name, char** cut);

#endif

/* dnssec.c */
size_t
dnssec_generate_query(struct dns_header* header,
        unsigned char* end,
        char* name,
        int _class,
        int type,
        int edns_pktsz);

int
dnssec_validate_by_ds(time_t now,
        struct dns_header* header,
        size_t plen,
        char* name,
        char* keyname,
        int _class);

int
dnssec_validate_ds(time_t now,
        struct dns_header* header,
        size_t plen,
        char* name,
        char* keyname,
        int _class);

int
dnssec_validate_reply(time_t now,
        struct dns_header* header,
        size_t plen,
        char* name,
        char* keyname,
        int* _class,
        int check_unsigned,
        int* neganswer,
        int* nons);

int
dnskey_keytag(int alg, int flags, unsigned char* key, int keylen);

size_t
filter_rrsigs(struct dns_header* header, size_t plen);

unsigned char*
hash_questions(struct dns_header* header, size_t plen, char* name);

int
setup_timestamp();

/* crypto.c */
const struct nettle_hash*
hash_find(char* name);

int
hash_init(const struct nettle_hash* hash, void** ctxp, unsigned char** digestp);

int
verify(struct blockdata* key_data,
        uint32_t key_len,
        unsigned char* sig,
        size_t sig_len,
        unsigned char* digest,
        size_t digest_len,
        int algo);

char*
ds_digest_name(int digest);

char*
algo_digest_name(int algo);

char*
nsec3_digest_name(int digest);

/* util.c */
void
rand_init();

uint16_t
rand16();

uint32_t
rand32();

uint64_t
rand64();

int
legal_hostname(char* name);

char*
canonicalise(char* in, int* nomem);

unsigned char*
do_rfc1035_name(unsigned char* p, char* sval, char* limit);

void*
safe_malloc(size_t size);

void
safe_strncpy(char* dest, const char* src, size_t size);

void
safe_pipe(int* fd, int read_noblock);

void*
whine_malloc(size_t size);

int
sa_len(union mysockaddr* addr);

int
sockaddr_isequal(union mysockaddr* s1, union mysockaddr* s2);

int
hostname_isequal(const char* a, const char* b);

int
hostname_issubdomain(char* a, char* b);

time_t
dnsmasq_time();

int
netmask_length(struct in_addr mask);

int
is_same_net(struct in_addr a, struct in_addr b, struct in_addr mask);

#ifdef HAVE_IPV6

int
is_same_net6(struct in6_addr* a, struct in6_addr* b, int prefixlen);

uint64_t
addr6part(struct in6_addr* addr);

void
setaddr6part(struct in6_addr* addr, uint64_t host);

#endif

int
retry_send(ssize_t rc);

void
prettyprint_time(char* buf, uint32_t t);

int
prettyprint_addr(union mysockaddr* addr, char* buf);

int
parse_hex(char* in,
        unsigned char* out,
        int maxlen,
        uint32_t* wildcard_mask,
        int* mac_type);

int
memcmp_masked(unsigned char* a, unsigned char* b, int len, uint32_t mask);

int
expand_buf(struct iovec* iov, size_t size);

char*
print_mac(char* buff, unsigned char* mac, int len);

int
read_write(int fd, unsigned char* packet, int size, int rw);

int
wildcard_match(const char* wildcard, const char* match);

int
wildcard_matchn(const char* wildcard, const char* match, int num);

/* log.c */
void
die(char* message, char* arg1, int exit_code) ATTRIBUTE_NORETURN;

int
log_start(struct passwd* ent_pw, int errfd);

int
log_reopen(char* log_file);

void
my_syslog(int priority, const char* format, ...);

void
set_log_writer();

void
check_log_writer(int force);

void
flush_log();

/* option.c */
void
read_opts(int argc, char** argv, char* compile_opts);

char*
option_string(int prot,
        uint32_t opt,
        unsigned char* val,
        int opt_len,
        char* buf,
        int buf_len);

void
reread_dhcp();

void
read_servers_file();

void
set_option_bool(uint32_t opt);

void
reset_option_bool(uint32_t opt);

struct hostsfile*
expand_filelist(struct hostsfile* list);

char*
parse_server(char* arg,
        union mysockaddr* addr,
        union mysockaddr* source_addr,
        std::string _interface,
        int* flags);

int
option_read_dynfile(char* file, int flags);

/* forward.c */
void
reply_query(int fd, int family, time_t now);

void
receive_query(struct listener* listen, time_t now);

unsigned char*
tcp_request(int confd,
        time_t now,
        union mysockaddr* local_addr,
        struct in_addr netmask,
        int auth_dns);

void
server_gone(struct server* server);

struct frec*
get_new_frec(time_t now, int* wait, int force);

int
send_from(int fd,
        int nowild,
        char* packet,
        size_t len,
        union mysockaddr* to,
        struct all_addr* source,
        uint32_t iface);

void
resend_query();

struct randfd*
allocate_rfd(int family);

void
free_rfd(struct randfd* rfd);

/* network.c */
int
indextoname(int fd, int index, char* name);

int
local_bind(int fd, union mysockaddr* addr, char* intname, uint32_t ifindex, int is_tcp);

int
random_sock(int family);

void
pre_allocate_sfds();

int
reload_servers(char* fname);

void
mark_servers(int flag);

void
cleanup_servers();

void
add_update_server(int flags,
        union mysockaddr* addr,
        union mysockaddr* source_addr,
        const std::string _interface,
        const char* domain);

void
check_servers();

int
enumerate_interfaces(int reset);

void
create_wildcard_listeners();

void
create_bound_listeners(int dienow);

void
warn_bound_listeners();

void
warn_wild_labels();

void
warn_int_names();

int
is_dad_listeners();

int
iface_check(int family, struct all_addr* addr, char* name, int* auth);

int
loopback_exception(int fd, int family, struct all_addr* addr, char* name);

int
label_exception(int index, int family, struct all_addr* addr);

int
fix_fd(int fd);

int
tcp_interface(int fd, int af);

#ifdef HAVE_IPV6

int
set_ipv6pktinfo(int fd);

#endif
#ifdef HAVE_DHCP6

void
join_multicast(int dienow);

#endif
#if defined(HAVE_LINUX_NETWORK) || defined(HAVE_BSD_NETWORK)

void
newaddress(time_t now);

#endif


/* dhcp.c */
#ifdef HAVE_DHCP

void
dhcp_init();

void
dhcp_packet(time_t now, int pxe_fd);

struct DhcpContext*
address_available(struct DhcpContext* context,
        struct in_addr taddr,
        struct dhcp_netid* netids);

struct DhcpContext*
narrow_context(struct DhcpContext* context,
        struct in_addr taddr,
        struct dhcp_netid* netids);

struct ping_result*
do_icmp_ping(time_t now, struct in_addr addr, uint32_t hash, int loopback);

int
address_allocate(struct DhcpContext* context,
        struct in_addr* addrp,
        unsigned char* hwaddr,
        int hw_len,
        struct dhcp_netid* netids,
        time_t now,
        int loopback);

void
dhcp_read_ethers();

struct dhcp_config*
config_find_by_address(struct dhcp_config* configs, struct in_addr addr);

char*
host_from_dns(struct in_addr addr);

#endif

/* lease.c */
#ifdef HAVE_DHCP

void
lease_update_file(time_t now);

void
lease_update_dns(int force);

void
lease_init(time_t now);

struct dhcp_lease*
lease4_allocate(struct in_addr addr);

#ifdef HAVE_DHCP6

struct dhcp_lease*
lease6_allocate(struct in6_addr* addrp, int lease_type);

struct dhcp_lease*
lease6_find(unsigned char* clid,
        int clid_len,
        int lease_type,
        int iaid,
        struct in6_addr* addr);

void
lease6_reset();

struct dhcp_lease*
lease6_find_by_client(struct dhcp_lease* first,
        int lease_type,
        unsigned char* clid,
        int clid_len,
        int iaid);

struct dhcp_lease*
lease6_find_by_addr(struct in6_addr* net, int prefix, uint64_t addr);

uint64_t
lease_find_max_addr6(struct DhcpContext* context);

void
lease_ping_reply(struct in6_addr* sender, unsigned char* packet, char* interface);

void
lease_update_slaac(time_t now);

void
lease_set_iaid(struct dhcp_lease* lease, int iaid);

void
lease_make_duid(time_t now);

#endif

void
lease_set_hwaddr(struct dhcp_lease* lease,
        const unsigned char* hwaddr,
        const unsigned char* clid,
        int hw_len,
        int hw_type,
        int clid_len,
        time_t now,
        int force);

void
lease_set_hostname(struct dhcp_lease* lease,
        const char* name,
        int auth,
        char* domain,
        char* config_domain);

void
lease_set_expires(struct dhcp_lease* lease, uint32_t len, time_t now);

void
lease_set_interface(struct dhcp_lease* lease, int _interface, time_t now);

struct dhcp_lease*
lease_find_by_client(unsigned char* hwaddr,
        int hw_len,
        int hw_type,
        unsigned char* clid,
        int clid_len);

struct dhcp_lease*
lease_find_by_addr(struct in_addr addr);

struct in_addr
lease_find_max_addr(struct DhcpContext* context);

void
lease_prune(struct dhcp_lease* target, time_t now);

void
lease_update_from_configs();

int
do_script_run(time_t now);

void
rerun_scripts();

void
lease_find_interfaces(time_t now);

#ifdef HAVE_SCRIPT

void
lease_add_extradata(struct dhcp_lease* lease,
        unsigned char* data,
        uint32_t len,
        int delim);

#endif
#endif

/* rfc2131.c */
#ifdef HAVE_DHCP

size_t
dhcp_reply(struct DhcpContext* context,
        char* iface_name,
        int int_index,
        size_t sz,
        time_t now,
        int unicast_dest,
        int loopback,
        int* is_inform,
        int pxe,
        struct in_addr fallback,
        time_t recvtime);

unsigned char*
extended_hwaddr(int hwtype,
        int hwlen,
        unsigned char* hwaddr,
        int clid_len,
        unsigned char* clid,
        int* len_out);

#endif

/* dnsmasq.c */
#ifdef HAVE_DHCP

int
make_icmp_sock();

int
icmp_ping(struct in_addr addr);

int
delay_dhcp(time_t start, int sec, int fd, uint32_t addr, uint16_t id);

#endif

void
queue_event(int event);

void
send_alarm(time_t event, time_t now);

void
send_event(int fd, int event, int data, char* msg);

void
clear_cache_and_reload(time_t now);

/* netlink.c */
#ifdef HAVE_LINUX_NETWORK

void
netlink_init();

void
netlink_multicast();

#endif

/* bpf.c */
#ifdef HAVE_BSD_NETWORK
void init_bpf(void);
void send_via_bpf(struct dhcp_packet *mess, size_t len,
          struct in_addr iface_addr, struct ifreq *ifr);
void route_init(void);
void route_sock(void);
#endif

/* bpf.c or netlink.c */
using callback_t = int(*)(struct in_addr*, int, int, int, int, int, int, void*);

int
iface_enumerate(int family, void* parm, callback_t callback);

/* dbus.c */
#ifdef HAVE_DBUS
char *dbus_init(void);
void check_dbus_listeners(void);
void set_dbus_listeners(void);
#  ifdef HAVE_DHCP
void emit_dbus_signal(int action, struct dhcp_lease *lease, char *hostname);
#  endif
#endif

/* ubus.c */
#ifdef HAVE_UBUS
void set_ubus_listeners(void);
void check_ubus_listeners(void);
void ubus_event_bcast(const char *type, const char *mac, const char *ip, const char *name, const char *interface);
#endif

/* ipset.c */
#ifdef HAVE_IPSET

void
ipset_init();

int
add_to_ipset(const char* setname, const struct all_addr* ipaddr, int flags, int remove);

#endif

/* helper.c */
#if defined(HAVE_SCRIPT)

int
create_helper(int event_fd, int err_fd, uid_t uid, gid_t gid, long max_fd);

void
helper_write();

void
queue_script(int action, struct dhcp_lease* lease, char* hostname, time_t now);

#ifdef HAVE_TFTP

void
queue_tftp(off_t file_len, char* filename, union mysockaddr* peer);

#endif

void
queue_arp(int action, unsigned char* mac, int maclen, int family, struct all_addr* addr);

int
helper_buf_empty();

#endif

/* tftp.c */
#ifdef HAVE_TFTP

void
tftp_request(struct listener* listen, time_t now);

void
check_tftp_listeners(time_t now);

int
do_tftp_script_run();

#endif

/* conntrack.c */
#ifdef HAVE_CONNTRACK
int get_incoming_mark(union mysockaddr *peer_addr, struct all_addr *local_addr,
              int istcp, uint32_t *markp);
#endif

/* dhcp6.c */
#ifdef HAVE_DHCP6

void
dhcp6_init();

void
dhcp6_packet(time_t now);

struct DhcpContext*
address6_allocate(struct DhcpContext* context,
        unsigned char* clid,
        int clid_len,
        int temp_addr,
        int iaid,
        int serial,
        struct dhcp_netid* netids,
        int plain_range,
        struct in6_addr* ans);

int
config_valid(struct dhcp_config* config,
        struct DhcpContext* context,
        struct in6_addr* addr);

struct DhcpContext*
address6_available(struct DhcpContext* context,
        struct in6_addr* taddr,
        struct dhcp_netid* netids,
        int plain_range);

struct DhcpContext*
address6_valid(struct DhcpContext* context,
        struct in6_addr* taddr,
        struct dhcp_netid* netids,
        int plain_range);

struct dhcp_config*
config_find_by_address6(struct dhcp_config* configs,
        struct in6_addr* net,
        int prefix,
        uint64_t addr);

void
make_duid(time_t now);

void
dhcp_construct_contexts(time_t now);

void
get_client_mac(struct in6_addr* client,
        int iface,
        unsigned char* mac,
        uint32_t* maclenp,
        uint32_t* mactypep,
        time_t now);

#endif

/* rfc3315.c */
#ifdef HAVE_DHCP6

uint16_t
dhcp6_reply(struct DhcpContext* context,
        int interface,
        char* iface_name,
        struct in6_addr* fallback,
        struct in6_addr* ll_addr,
        struct in6_addr* ula_addr,
        size_t sz,
        struct in6_addr* client_addr,
        time_t now);

void
relay_upstream6(struct dhcp_relay* relay,
        ssize_t sz,
        struct in6_addr* peer_address,
        uint32_t scope_id,
        time_t now);

uint16_t
relay_reply6(struct sockaddr_in6* peer, ssize_t sz, char* arrival_interface);

#endif

/* dhcp-common.c */
#ifdef HAVE_DHCP

void
dhcp_common_init();

ssize_t
recv_dhcp_packet(int fd, struct msghdr* msg);

struct dhcp_netid*
run_tag_if(struct dhcp_netid* tags);

struct dhcp_netid*
option_filter(struct dhcp_netid* tags,
        struct dhcp_netid* context_tags,
        struct dhcp_opt* opts);

int
match_netid(struct dhcp_netid* check, struct dhcp_netid* pool, int tagnotneeded);

char*
strip_hostname(char* hostname);

void
log_tags(struct dhcp_netid* netid, uint32_t xid);

int
match_bytes(struct dhcp_opt* o, unsigned char* p, int len);

void
dhcp_update_configs(struct dhcp_config* configs);

void
display_opts();

int
lookup_dhcp_opt(int prot, char* name);

int
lookup_dhcp_len(int prot, int val);

char*
option_string(int prot,
        uint32_t opt,
        unsigned char* val,
        int opt_len,
        char* buf,
        int buf_len);

struct dhcp_config*
find_config(struct dhcp_config* configs,
        struct DhcpContext* context,
        unsigned char* clid,
        int clid_len,
        unsigned char* hwaddr,
        int hw_len,
        int hw_type,
        char* hostname);

int
config_has_mac(struct dhcp_config* config, unsigned char* hwaddr, int len, int type);

#ifdef HAVE_LINUX_NETWORK

char*
whichdevice();

void
bindtodevice(char* device, int fd);

#endif
#  ifdef HAVE_DHCP6

void
display_opts6();

#  endif

void
log_context(int family, struct DhcpContext* context);

void
log_relay(int family, struct DhcpRelay* relay);

#endif

/* outpacket.c */
#ifdef HAVE_DHCP6

void
end_opt6(int container);

void
reset_counter();

int
save_counter(int newval);

void*
expand(size_t headroom);

int
new_opt6(int opt);

void*
put_opt6(void* data, size_t len);

void
put_opt6_long(uint32_t val);

void
put_opt6_short(uint32_t val);

void
put_opt6_char(uint32_t val);

void
put_opt6_string(char* s);

#endif

/* radv.c */
#ifdef HAVE_DHCP6

void
ra_init(time_t now);

void
icmp6_packet(time_t now);

time_t
periodic_ra(time_t now);

void
ra_start_unsolicited(time_t now, struct DhcpContext* context);

#endif

/* slaac.c */
#ifdef HAVE_DHCP6

void
slaac_add_addrs(struct dhcp_lease* lease, time_t now, int force);

time_t
periodic_slaac(time_t now, struct dhcp_lease* leases);

void
slaac_ping_reply(struct in6_addr* sender,
        unsigned char* packet,
        char* interface,
        struct dhcp_lease* leases);

#endif

/* loop.c */
#ifdef HAVE_LOOP

void
loop_send_probes();

int
detect_loop(char* query, int type);

#endif

/* inotify.c */
#ifdef HAVE_INOTIFY

void
inotify_dnsmasq_init();

int
inotify_check(time_t now);

void
set_dynamic_inotify(int flag, int total_size, struct crec** rhash, int revhashsz);

#endif

/* poll.c */
void
poll_reset();

int
poll_check(int fd, short event);

void
poll_listen(int fd, short event);

int
do_poll(int timeout);

/* rrfilter.c */
size_t
rrfilter(struct dns_header* header, size_t plen, int mode);

uint16_t*
rrfilter_desc(int type);

int
expand_workspace(unsigned char*** wkspc, int* szp, int _new);

/* edns0.c */
unsigned char*
find_pseudoheader(struct dns_header* header,
        size_t plen,
        size_t* len,
        unsigned char** p,
        int* is_sign,
        int* is_last);

size_t
add_pseudoheader(struct dns_header* header,
        size_t plen,
        unsigned char* limit,
        uint16_t udp_sz,
        int optno,
        unsigned char* opt,
        size_t optlen,
        int set_do,
        int replace);

size_t
add_do_bit(struct dns_header* header, size_t plen, unsigned char* limit);

size_t
add_edns0_config(struct dns_header* header,
        size_t plen,
        unsigned char* limit,
        union mysockaddr* source,
        time_t now,
        int* check_subnet);

int
check_source(struct dns_header* header,
        size_t plen,
        unsigned char* pseudoheader,
        union mysockaddr* peer);

/* arp.c */
int
find_mac(union mysockaddr* addr, unsigned char* mac, int lazy, time_t now);

int
do_arp_script_run();

/* dump.c */
#ifdef HAVE_DUMPFILE

void
dump_init();

void
dump_packet(int mask,
        void* packet,
        size_t len,
        union mysockaddr* src,
        union mysockaddr* dst);

#endif

//
// END OF FILE
//