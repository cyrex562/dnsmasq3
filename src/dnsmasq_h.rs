use std::{
    fs,
    net::{self, IpAddr},
    time,
};

use crate::{
    config::{MAX_PROCS, RANDOM_SOCKS},
    dhcp_protocol::DHCP_CHADDR_MAX,
};

/* dnsmasq is Copyright (c) 2000-2021 Simon Kelley

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

type pid_t = i32;
type uid_t = i32;
type gid_t = i32;

//#define COPYRIGHT "Copyright (c) 2000-2021 Simon Kelley"

/* We do defines that influence behavior of stdio.h, so complain
if included too early. */
//  _STDIO_H
// #  error "Header file stdio.h included too early!"
//

// #ifndef NO_LARGEFILE
/* Ensure we can use files >2GB (log files may grow this big) */
pub const _LARGEFILE_SOURCE: u32 = 1;
pub const _FILE_OFFSET_BITS: u32 = 64;
//

/* Get linux C library versions and define _GNU_SOURCE for kFreeBSD. */
// #if defined(__linux__) || defined(__GLIBC__)
// #  ifndef __ANDROID__
// #      define _GNU_SOURCE
// #  endif
// #  include <features.h>
//

/* Need these defined early */
// #if defined(__sun) || defined(__sun__)
// pub const _XPG4_: u32 = 2;
// #  define __EXTENSIONS__
//

// #if (defined(__GNUC__) && __GNUC__ >= 3) || defined(__clang__)
// #define ATTRIBUTE_NORETURN __attribute__ ((noreturn))
//
// #define ATTRIBUTE_NORETURN
//

/* get these before config.h  for IPv6 stuff... */
// #include <sys/types.h>
// #include <sys/socket.h>

//  __APPLE__
// /* Define before netinet/in.h to select API. OSX Lion onwards. */
// pub const __APPLE_USE_RFC_354: u32 = 2;
//
// #include <netinet/in.h>

/* Also needed before config.h. */
// #include <getopt.h>

// #include "config.h"
// #include "ip6addr.h"
// #include "metrics.h"

// typedef unsigned let mut u8: u8;
// typedef u16 u16;
// typedef let mut u32: u32;
// typedef unsigned long let u64: i32;

// #define countof(x)      (long)(sizeof(x) / sizeof(x[0]))
// #define MIN(a,b)        ((a) < (b) ? (a) : (b))

// #include "dns-protocol.h"
// #include "dhcp-protocol.h"
//
// #include "dhcp6-protocol.h"
// #include "radv-protocol.h"
//

// #define gettext_noop(S) (S)
// #ifndef LOCALEDIR
// #  define format!(S) (S)
//
// #  include <libintl.h>
// #  include <locale.h>
// #  define format!(S) gettext(S)
//

// #include <arpa/inet.h>
// #include <sys/stat.h>
// #include <sys/ioctl.h>
// #if defined(HAVE_SOLARIS_NETWORK)
// #  include <sys/sockio.h>
//
// #if defined(HAVE_POLL_H)
// #  include <poll.h>
//
// #  include <sys/poll.h>
//
// #include <sys/wait.h>
// #include <sys/time.h>
// #include <sys/un.h>
// #include <limits.h>
// #include <net/if.h>
// #if defined(HAVE_SOLARIS_NETWORK) && !defined(ifr_mtu)
// /* Some solaris net/if./h omit this. */
// #  define ifr_mtu  ifr_ifru.ifru_metric
//
// #include <unistd.h>
// #include <stdio.h>
// #include <string.h>
// #include <stdlib.h>
// #include <fcntl.h>
// #include <ctype.h>
// #include <signal.h>
// #include <stddef.h>
// #include <time.h>
// #include <errno.h>
// #include <pwd.h>
// #include <grp.h>
// #include <stdarg.h>
// #if defined(__OpenBSD__) || defined(__NetBSD__) || defined(__sun__) || defined (__sun) || defined (__ANDROID__)
// #  include <netinet/if_ether.h>
//
// #  include <net/ethernet.h>
//
// #include <net/if_arp.h>
// #include <netinet/in_systm.h>
// #include <netinet/ip.h>
// #include <netinet/ip6.h>
// #include <netinet/ip_icmp.h>
// #include <netinet/tcp.h>
// #include <sys/uio.h>
// #include <syslog.h>
// #include <dirent.h>
// #ifndef
// #  include <net/if_dl.h>
//

// #if defined(HAVE_LINUX_NETWORK)
// #include <linux/version.h>
// #include <linux/sockios.h>
// #include <linux/capability.h>
/* There doesn't seem to be a universally-available
userspace header for these. */
// extern int capset(cap_user_header_t header, cap_user_data_t data);
// extern int capget(cap_user_header_t header, cap_user_data_t data);
pub const LINUX_CAPABILITY_VERSION_1: u32 = 0x19980330;
pub const LINUX_CAPABILITY_VERSION_2: u32 = 0x20071026;
pub const LINUX_CAPABILITY_VERSION_3: u32 = 0x20080522;

// #include <sys/prctl.h>
// #elif defined(HAVE_SOLARIS_NETWORK)
// #include <priv.h>
//

/* Backwards compat with 2.83 */
// #if defined(HAVE_NETTLEHASH)
// #  define HAVE_CRYPTOHASH
//
// #if defined(HAVE_DNSSEC) || defined(HAVE_CRYPTOHASH)
// #  include <nettle/nettle-meta.h>
//

/* daemon is function in the C library.... */
// #define daemon dnsmasq_daemon

// #define ADDRSTRLEN INET6_ADDRSTRLEN

/* Async event queue */
// struct event_desc {
//   event: i32, data, msg_sz;
// };
pub struct event_desc {
    pub event: i32,
    pub data: i32,
    pub msg_sz: i32,
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

// pub const OPT_BOGUSPRIV: u32 = 0;
// pub const OPT_FILTER: u32 = 1;
// pub const OPT_LOG: u32 = 2;
// pub const OPT_SELFMX: u32 = 3;
// pub const OPT_NO_HOSTS: u32 = 4;
// pub const OPT_NO_POLL: u32 = 5;
// pub const OPT_DEBUG: u32 = 6;
// pub const OPT_ORDER: u32 = 7;
// pub const OPT_NO_RESOLV: u32 = 8;
// pub const OPT_EXPAND: u32 = 9;
// pub const OPT_LOCALMX: u32 = 10;
// pub const OPT_NO_NEG: u32 = 11;
// pub const OPT_NODOTS_LOCAL: u32 = 12;
// pub const OPT_NOWILD: u32 = 13;
// pub const OPT_ETHERS: u32 = 14;
// pub const OPT_RESOLV_DOMAIN: u32 = 15;
// pub const OPT_NO_FORK: u32 = 16;
// pub const OPT_AUTHORITATIVE: u32 = 17;
// pub const OPT_LOCALISE: u32 = 18;
// pub const OPT_DBUS: u32 = 19;
// pub const OPT_DHCP_FQDN: u32 = 20;
// pub const OPT_NO_PING: u32 = 21;
// pub const OPT_LEASE_RO: u32 = 22;
// pub const OPT_ALL_SERVERS: u32 = 23;
// pub const OPT_RELOAD: u32 = 24;
// pub const OPT_LOCAL_REBIND: u32 = 25;
// pub const OPT_TFTP_SECURE: u32 = 26;
// pub const OPT_TFTP_NOBLOCK: u32 = 27;
// pub const OPT_LOG_OPTS: u32 = 28;
// pub const OPT_TFTP_APREF_IP: u32 = 29;
// pub const OPT_NO_OVERRIDE: u32 = 30;
// pub const OPT_NO_REBIND: u32 = 31;
// pub const OPT_ADD_MAC: u32 = 32;
// pub const OPT_DNSSEC_PROXY: u32 = 33;
// pub const OPT_CONSEC_ADDR: u32 = 34;
// pub const OPT_CONNTRACK: u32 = 35;
// pub const OPT_FQDN_UPDATE: u32 = 36;
// pub const OPT_RA: u32 = 37;
// pub const OPT_TFTP_LC: u32 = 38;
// pub const OPT_CLEVERBIND: u32 = 39;
// pub const OPT_TFTP: u32 = 40;
// pub const OPT_CLIENT_SUBNET: u32 = 41;
// pub const OPT_QUIET_DHCP: u32 = 42;
// pub const OPT_QUIET_DHCP6: u32 = 43;
// pub const OPT_QUIET_RA: u32 = 44;
// pub const OPT_DNSSEC_VALID: u32 = 45;
// pub const OPT_DNSSEC_TIME: u32 = 46;
// pub const OPT_DNSSEC_DEBUG: u32 = 47;
// pub const OPT_DNSSEC_IGN_NS: u32 = 48;
// pub const OPT_LOCAL_SERVICE: u32 = 49;
// pub const OPT_LOOP_DETECT: u32 = 50;
// pub const OPT_EXTRALOG: u32 = 51;
// pub const OPT_TFTP_NO_FAIL: u32 = 52;
// pub const OPT_SCRIPT_ARP: u32 = 53;
// pub const OPT_MAC_B64: u32 = 54;
// pub const OPT_MAC_HEX: u32 = 55;
// pub const OPT_TFTP_APREF_MAC: u32 = 56;
// pub const OPT_RAPID_COMMIT: u32 = 57;
// pub const OPT_UBUS: u32 = 58;
// pub const OPT_IGNORE_CLID: u32 = 59;
// pub const OPT_SINGLE_PORT: u32 = 60;
// pub const OPT_LEASE_RENEW: u32 = 61;
// pub const OPT_LAST: u32 = 62;

// #define OPTION_BITS (sizeof(unsigned int)*8)
// #define OPTION_SIZE ( (OPT_LAST/OPTION_BITS)+((OPT_LAST%OPTION_BITS)!=0) )
// #define option_var(x) (daemon.options[(x) / OPTION_BITS])
// #define option_val(x) ((1u) << ((x) % OPTION_BITS))
// #define option_bool(x) (option_var(x) & option_val(x))

/* extra flags for my_syslog, we use a couple of facilities since they are known
not to occupy the same bits as priorities, no matter how syslog.h is set up. */
// #define MS_TFTP   LOG_USER
// #define MS_DHCP   LOG_DAEMON
// #define MS_SCRIPT LOG_MAIL

/* Note that this is used widely as a container for IPv4/IPv6 addresses,
   so for that reason, was well as to avoid wasting memory in almost every
   cache entry, the other variants should not be larger than
   sizeof(struct in6_addr) - 16 bytes.
*/

// union all_addr {
//   addr4: net::IpAddr;
//   struct in6_addr addr6;
//   struct {
//     union {
//       let mut cache: crec;
//       char *name;
//     } target;
//     let mut uid: u32;
//     let mut is_name_ptr: i32;  /* disciminates target union */
//   } cname;
//   struct {
//     let mut keydata: blockdata;
//     u16 keylen, flags, keytag;
//     unsigned let mut algo: u8;
//   } key;
//   struct {
//     let mut keydata: blockdata;
//     u16 keylen, keytag;
//     unsigned let mut algo: u8;
//     unsigned let mut digest: u8;
//   } ds;
//   struct {
//     let mut target: blockdata;
//     u16 targetlen, srvport, priority, weight;
//   } srv;
//   /* for log_query */
//   struct {
//     u16 keytag, algo, digest, rcode;
//   } log;
// };
pub struct all_addr {
    pub addr_4: net::Ipv4Addr,
    pub addr6: net::Ipv6Addr,
    pub cache: Vec<crec>,
    pub name: String,
    pub uid: u32,
    pub is_name_ptr: bool,
    pub keydata: Vec<blockdata>,
    pub keylen: u16,
    pub keytag: u16,
    pub algo: u8,
    pub digest: u8,
    pub target: Vec<blockdata>,
    pub targetlen: u16,
    pub srvport: u16,
    pub priority: u16,
    pub weight: u16,
    pub rcode: u16,
}

// struct bogus_addr {
//   addr: net::IpAddr;
//   let mut next: bogus_addr;
// };
pub struct bogus_addr {
    pub addr: net::IpAddr,
    // next
}

/* dns doctor param */
// struct doctor {
//   in: net::IpAddr, end, out, mask;
//   let mut next: doctor;
// };
pub struct doctor {
    pub _in: net::IpAddr,
    pub end: net::IpAddr,
    pub out: net::IpAddr,
    pub mask: net::IpAddr,
    // next
}

// struct mx_srv_record {
//   name: &mut String, *target;
//   issrv: i32, srvport, priority, weight;
//   let mut offset: u32;
//   let mut next: mx_srv_record;
// };
pub struct mx_srv_record {
    pub name: String,
    pub target: String,
    pub issrv: i32,
    pub srvport: i32,
    pub priority: i32,
    pub weight: i32,
    pub offest: u32,
    // next
}

// struct naptr {
//   name: &mut String, *replace, *regexp, *services, *flags;
//   unsigned order: i32, pref;
//   let mut next: naptr;
// };
pub struct naptr {
    pub name: String,
    pub replace: String,
    pub regexp: String,
    pub services: String,
    pub flags: String,
    pub order: u32,
    pub pref: u32,
    // next
}

// #ifndef NO_ID
pub const TXT_STAT_CACHESIZE: u32 = 1;
pub const TXT_STAT_INSERTS: u32 = 2;
pub const TXT_STAT_EVICTIONS: u32 = 3;
pub const TXT_STAT_MISSES: u32 = 4;
pub const TXT_STAT_HITS: u32 = 5;
pub const TXT_STAT_AUTH: u32 = 6;
pub const TXT_STAT_SERVERS: u32 = 7;
//

// struct txt_record {
//   char *name;
//   unsigned char *txt;
//   u16 class, len;
//   let mut stat: i32;
//   let mut next: txt_record;
// };
pub struct txt_record {
    pub name: String,
    pub txt: String,
    pub class: u16,
    pub len: u16,
    pub stat: i32,
    // next
}

// struct ptr_record {
//   name: &mut String, *ptr;
//   let mut next: ptr_record;
// };
pub struct ptr_record {
    pub name: String,
    pub ptr: String,
    // next
}

// struct cname {
//   ttl: i32, flag;
//   alias: &mut String, *target;
//   struct cname *next, *targetp;
// };
pub struct cname {
    pub flag: i32,
    pub ttl: i32,
    pub alias: String,
    pub target: String,
    // next
}

// struct ds_config {
//   name: &mut String, *digest;
//   digestlen: i32, class, algo, keytag, digest_type;
//   let mut next: ds_config;
// };
#[derive(Clone, Debug, Default)]
pub struct DsConfig {
    pub name: String,
    pub digest: String,
    pub digestlen: i32,
    pub class: i32,
    pub algo: i32,
    pub keytag: i32,
    pub digest_type: i32,
    // next
}

impl DsConfig {
    pub fn new() -> Self {
        DsConfig {
            name: String::new(),
            digest: String::new(),
            digestlen: 0,
            class: 0,
            algo: 0,
            keytag: 0,
            digest_type: 0,
        }
    }
}

pub const ADDRLIST_LITERAL: u32 = 1;
pub const ADDRLIST_IPV6: u32 = 2;
pub const ADDRLIST_REVONLY: u32 = 4;
pub const ADDRLIST_PREFIX: u32 = 8;
pub const ADDRLIST_WILDCARD: u32 = 16;
pub const ADDRLIST_DECLINED: u32 = 32;

// struct addrlist {
//   union all_addr addr;
//   flags: i32, prefixlen;
//   decline_time: time::Instant;
//   let mut next: addrlist;
// };
pub struct AddrList {
    pub addr: all_addr,
    pub flags: i32,
    pub prefixlen: i32,
    pub decline_time: time::Instant,
    // next
}

pub const AUTH6: u32 = 1;
pub const AUTH4: u32 = 2;

// struct auth_zone {
//   char *domain;
//   struct auth_name_list {
//     char *name;
//     let mut flags: i32;
//     let mut next: auth_name_list;
//   } *interface_names;
//   let mut subnet: addrlist;
//   let mut exclude: addrlist;
//   let mut next: auth_zone;
// };
pub struct auth_name_list {
    pub name: String,
    pub flags: i32,
    // next
}

pub struct auth_zone {
    pub domain: String,
    pub interface_names: auth_name_list,
    pub subnet: AddrList,
    pub exclude: AddrList,
    // next
}

pub const HR_6: u32 = 1;
pub const HR_4: u32 = 2;

// struct host_record {
//   ttl: i32, flags;
//   struct name_list {
//     char *name;
//     let mut next: name_list;
//   } *names;
//   addr: net::IpAddr;
//   struct in6_addr addr6;
//   let mut next: host_record;
// };
pub struct name_list {
    pub name: String,
    // next
}

pub struct host_record {
    pub ttl: i32,
    pub flags: i32,
    pub names: name_list,
    pub addr: net::IpAddr,
    pub addr6: net::IpAddr,
    // next
}

// struct interface_name {
//   char *name; /* domain name */
//   char *intr; /* interface name */
//   let mut family: i32; /* AF_INET, AF_INET6 or zero for both */
//   let mut addr: addrlist;
//   let mut next: interface_name;
// };
pub struct interface_name {
    pub name: String,
    pub intr: String,
    pub family: i32,
    pub addr: AddrList,
    // next
}

// union bigname {
//   char name[MAXDNAME];
//   union bigname *next; /* freelist */
// };
pub struct bigname {
    pub name: String,
    // next
}

// struct blockdata {
//   let mut next: blockdata;
//   unsigned char key[KEYBLOCK_LEN];
// };
pub struct blockdata {
    // next
    key: Vec<u8>,
}
// struct crec {
//   struct crec *next, *prev, *hash_next;
//   union all_addr addr;
//   ttd: time::Instant; /* time to die */
//   /* used as class if DNSKEY/DS, index to source for F_HOSTS */
//   let mut uid: u32;
//   let mut flags: u32;
//   union {
//     char sname[SMALLDNAME];
//     union bigname *bname;
//     char *namep;
//   } name;
// };
pub struct crec {
    // next
    // prev
    // hash_next
    pub addr: all_addr,
    pub ttd: time::Instant,
    pub uid: u32,
    pub flags: u32,
    pub sname: String,
    pub bname: bigname,
    pub namep: String,
}

// #define SIZEOF_BARE_CREC (sizeof(struct crec) - SMALLDNAME)
// #define SIZEOF_POINTER_CREC (sizeof(struct crec) + sizeof - SMALLDNAME)

// #define F_IMMORTAL  (1u<<0)
// #define F_NAMEP     (1u<<1)
// #define F_REVERSE   (1u<<2)
// #define F_FORWARD   (1u<<3)
// #define F_DHCP      (1u<<4)
// #define F_NEG       (1u<<5)
// #define F_HOSTS     (1u<<6)
// pub const F_IPV: u32 = 4;      (1u<<7)
// pub const F_IPV: u32 = 6;      (1u<<8)
// #define F_BIGNAME   (1u<<9)
// #define F_NXDOMAIN  (1u<<10)
// #define F_CNAME     (1u<<11)
// #define F_DNSKEY    (1u<<12)
// #define F_CONFIG    (1u<<13)
// #define F_DS        (1u<<14)
// #define F_DNSSECOK  (1u<<15)
// #define F_UPSTREAM  (1u<<16)
// #define F_RRNAME    (1u<<17)
// #define F_SERVER    (1u<<18)
// #define F_QUERY     (1u<<19)
// #define F_NOERR     (1u<<20)
// #define F_AUTH      (1u<<21)
// #define F_DNSSEC    (1u<<22)
// #define F_KEYTAG    (1u<<23)
// #define F_SECSTAT   (1u<<24)
// #define F_NO_RR     (1u<<25)
// #define F_IPSET     (1u<<26)
// #define F_NOEXTRA   (1u<<27)
// #define F_SERVFAIL  (1u<<28) /* currently unused. */
// #define F_RCODE     (1u<<29)
// #define F_SRV       (1u<<30)

pub const UID_NONE: u32 = 0;
/* Values of uid in crecs with F_CONFIG bit set. */
pub const SRC_CONFIG: u32 = 1;
pub const SRC_HOSTS: u32 = 2;
pub const SRC_AH: u32 = 3;

/* struct sockaddr is not large enough to hold any address,
and specifically not big enough to hold an IPv6 address.
Blech. Roll our own. */
// union mysockaddr {
//   struct sockaddr sa;
//   struct sockaddr_in in;
//   struct sockaddr_in6 in6;
// };
pub struct mysockaddr {
    pub sa: net::IpAddr,
    pub _in: net::IpAddr,
    pub in6: net::IpAddr,
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
// #define SERV_TYPE    (SERV_HAS_DOMAIN | SERV_FOR_NODOTS)
pub const SERV_COUNTED: u32 = 512; /* workspace for log code */
pub const SERV_USE_RESOLV: u32 = 1024; /* forward this domain in the normal way */
pub const SERV_NO_REBIND: u32 = 2048; /* inhibit dns-rebind protection */
pub const SERV_FROM_FILE: u32 = 4096; /* read from --servers-file */
pub const SERV_LOOP: u32 = 8192; /* server causes forwarding loop */
pub const SERV_DO_DNSSEC: u32 = 16384; /* Validate DNSSEC when using this server */
pub const SERV_GOT_TCP: u32 = 32768; /* Got some data from the TCP connection */

// struct randfd {
//   let mut fd: i32;
//   u16 refcount, family;
// };
pub struct randfd {
    pub fd: i32,
    pub refcount: u16,
    pub family: u16,
}

// struct ipsets {
//   char **sets;
//   char *domain;
//   let mut next: ipsets;
// };
pub struct ipsets {
    pub sets: Vec<String>,
    pub domain: Vec<String>,
    // next
}

// struct irec {
//   union mysockaddr addr;
//   netmask: net::IpAddr; /* only valid for IPv4 */
//   tftp_ok: i32, dhcp_ok, mtu, done, warned, dad, dns_auth, index, multicast_done, found, label;
//   char *name;
//   let mut next: irec;
// };
pub struct irec {
    pub addr: net::IpAddr,
    pub netmask: net::Ipv4Addr,
    pub tftp_ok: i32,
    pub dhcp_ok: i32,
    pub mtu: i32,
    pub done: i32,
    pub warned: i32,
    pub dad: i32,
    pub dns_auth: i32,
    pub index: i32,
    pub multicast_done: i32,
    pub found: i32,
    pub label: i32,
    pub name: String,
    // next
}

// struct listener {
//   fd: i32, tcpfd, tftpfd, used;
//   union mysockaddr addr;
//   let mut iface: irec; /* only sometimes valid for non-wildcard */
//   let mut next: listener;
// };
pub struct listener {
    pub fd: i32,
    pub tcpfd: i32,
    pub tftpfd: i32,
    pub used: i32,
    pub addr: net::IpAddr,
    pub iface: irec,
    // next
}

/* interface and address parms from command line. */
// struct iname {
//   char *name;
//   union mysockaddr addr;
//   let mut used: i32;
//   let mut next: iname;
// };
pub struct InterfaceParams {
    pub name: String,
    pub addr: net::IpAddr,
    pub used: i32,
}

/* subnet parameters from command line */
// struct mysubnet {
//   union mysockaddr addr;
//   let mut addr_used: i32;
//   let mut mask: i32;
// };
pub struct mysubnet {
    pub addr: net::IpAddr,
    pub addr_used: i32,
    pub mask: i32,
}

/* resolv-file parms from command-line */
// struct resolvc {
//   let mut next: resolvc;
//   is_default: i32, logged;
//   mtime: time::Instant;
//   char *name;
//  HAVE_INOTIFY
//   let mut wd: i32; /* inotify watch descriptor */
//   char *file; /* pointer to file part if path */
//
// };
pub struct resolvc {
    // next
    pub is_default: i32,
    pub logged: i32,
    pub mtime: time::Instant,
    pub name: String,
    pub wd: i32,
    pub file: String,
}

/* adn-hosts parms from command-line (also dhcp-hostsfile and dhcp-optsfile and dhcp-hostsdir*/
pub const AH_DIR: u32 = 1;
pub const AH_INACTIVE: u32 = 2;
pub const AH_WD_DONE: u32 = 4;
pub const AH_HOSTS: u32 = 8;
pub const AH_DHCP_HST: u32 = 16;
pub const AH_DHCP_OPT: u32 = 32;
// struct hostsfile {
//   let mut next: hostsfile;
//   let mut flags: i32;
//   char *fname;
//  HAVE_INOTIFY
//   let mut wd: i32; /* inotify watch descriptor */
//
//   let mut index: u32; /* matches to cache entries for logging */
// };
pub struct hostsfile {
    // next
    pub flags: i32,
    pub fname: String,
    pub wd: i32,
    pub index: i32,
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
pub const FREC_HAS_PHEADER: u32 = 1024;
pub const FREC_NO_CACHE: u32 = 2048;

pub const HASH_SIZE: u32 = 32; /* SHA-256 digest size */

// struct frec {
//   struct frec_src {
//     union mysockaddr source;
//     union all_addr dest;
//     unsigned iface: i32, log_id;
//     let mut fd: i32;
//     u16 orig_id;
//     let mut next: frec_src;
//   } frec_src;
//   let mut sentto: server; /* NULL means free */
//   let mut rfd4: randfd;
//   let mut rfd6: randfd;
//   u16 new_id;
//   forwardall: i32, flags;
//   time: time::Instant;
//   unsigned char *hash[HASH_SIZE];
//
//   class: i32, work_counter;
//   let mut stash: blockdata; /* Saved reply, whilst we validate */
//   stash_len: usize;
//   let mut dependent: frec; /* Query awaiting internally-generated DNSKEY or DS query */
//   let mut blocking_query: frec; /* Query which is blocking us. */
//
//   let mut next: frec;
// };
pub struct frec_src {
    pub source: net::IpAddr,
    pub dest: net::IpAddr,
    pub iface: u32,
    pub log_id: u32,
    pub fd: i32,
    pub orig_id: u16,
    // next
}

pub struct frec {
    pub frec_src: frec_src,
    pub sentto: Server,
    pub rfd4: randfd,
    pub rfd6: randfd,
    pub new_id: u16,
    pub forwardall: i32,
    pub flags: i32,
    pub time: time::Instant,
    pub hash: Vec<u8>,
    pub class: i32,
    pub work_counter: i32,
    pub stash: blockdata,
    pub stash_len: usize,
    //pub dependent: frec,
    //pub blocking_query: frec,
    // next
}

/* flags in top of length field for DHCP-option tables */
pub const OT_ADDR_LIST: u32 = 0x8000;
pub const OT_RFC1035_NAME: u32 = 0x4000;
pub const OT_INTERNAL: u32 = 0x2000;
pub const OT_NAME: u32 = 0x1000;
pub const OT_CSTRING: u32 = 0x0800;
pub const OT_DEC: u32 = 0x0400;
pub const OT_TIME: u32 = 0x0200;

/* actions in the daemon.helper RPC */
pub const ACTION_DEL: u32 = 1;
pub const ACTION_OLD_HOSTNAME: u32 = 2;
pub const ACTION_OLD: u32 = 3;
pub const ACTION_ADD: u32 = 4;
pub const ACTION_TFTP: u32 = 5;
pub const ACTION_ARP: u32 = 6;
pub const ACTION_ARP_DEL: u32 = 7;

pub const LEASE_NEW: u32 = 1; /* newly created */
pub const LEASE_CHANGED: u32 = 2; /* modified */
pub const LEASE_AUX_CHANGED: u32 = 4; /* CLID or expiry changed */
pub const LEASE_AUTH_NAME: u32 = 8; /* hostname came from config, not from client */
pub const LEASE_USED: u32 = 16; /* used this DHCPv6 transaction */
pub const LEASE_NA: u32 = 32; /* IPv6 no-temporary lease */
pub const LEASE_TA: u32 = 64; /* IPv6 temporary lease */
pub const LEASE_HAVE_HWADDR: u32 = 128; /* Have set hwaddress */
pub const LEASE_EXP_CHANGED: u32 = 256; /* Lease expiry time changed */

// struct dhcp_lease {
//   let mut clid_len: i32;          /* length of client identifier */
//   unsigned char *clid;   /* clientid */
//   hostname: &mut String, *fqdn; /* name from client-hostname option or config */
//   char *old_hostname;    /* hostname before it moved to another lease */
//   let mut flags: i32;
//   expires: time::Instant;        /* lease expiry */
//
//   let mut length: u32;
//
//   hwaddr_len: i32, hwaddr_type;
//   unsigned char hwaddr[DHCP_CHADDR_MAX];
//   addr: net::IpAddr, override, giaddr;
//   unsigned char *extradata;
//   unsigned extradata_len: i32, extradata_size;
//   let mut last_interface: i32;
//   let mut new_interface: i32;     /* save possible originated interface */
//   let mut new_prefixlen: i32;     /* and its prefix length */
//
//   struct in6_addr addr6;
//   let mut iaid: u32;
//   struct slaac_address {
//     struct in6_addr addr;
//     ping_time: time::Instant;
//     let mut backoff: i32; /* zero . confirmed */
//     let mut next: slaac_address;
//   } *slaac_address;
//   let mut vendorclass_count: i32;
//
//   let mut next: dhcp_lease;
// };

pub struct slaac_address {
    pub addr: net::Ipv6Addr,
    pub ping_time: time::Instant,
    pub backoff: i32,
    // next
}

pub struct dhcp_lease {
    pub clid_len: i32,
    pub clid: Vec<u8>,
    pub hostname: String,
    pub fqdn: String,
    pub old_hostname: String,
    pub flags: i32,
    pub expires: time::Instant,
    pub length: i32,
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub hwaddr: [u8; 16],
    pub addr: net::IpAddr,
    pub _override: net::IpAddr,
    pub giadrr: net::IpAddr,
    pub extradata: Vec<u8>,
    pub extradata_len: i32,
    pub extradata_size: i32,
    pub last_interface: i32,
    pub new_prefixlen: i32,
    pub addr6: net::Ipv6Addr,
    pub iaid: u32,
    pub slaac_address: slaac_address,
    pub vendorclass_count: i32,
    // next
}

// struct dhcp_netid {
//   char *net;
//   let mut next: dhcp_netid;
// };
pub struct dhcp_netid {
    pub net: String,
    // next
}

// struct dhcp_netid_list {
//   let mut list: dhcp_netid;
//   let mut next: dhcp_netid_list;
// };
pub struct dhcp_netid_list {
    pub list: Vec<dhcp_netid>,
    // next
}

// struct tag_if {
//   let mut set: dhcp_netid_list;
//   let mut tag: dhcp_netid;
//   let mut next: tag_if;
// };
pub struct tag_if {
    pub set: dhcp_netid_list,
    pub tag: dhcp_netid,
    // next
}

// struct delay_config {
//   let mut delay: i32;
//   let mut netid: dhcp_netid;
//   let mut next: delay_config;
// };
pub struct delay_config {
    pub delay: i32,
    pub netid: dhcp_netid,
    // next
}

// struct hwaddr_config {
//   hwaddr_len: i32, hwaddr_type;
//   unsigned char hwaddr[DHCP_CHADDR_MAX];
//   let mut wildcard_mask: u32;
//   let mut next: hwaddr_config;
// };
pub struct hwaddr_config {
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub hwaddr: [u8; 16],
    pub wildcard_mask: i32,
    // next
}

// struct dhcp_config {
//   let mut flags: u32;
//   let mut clid_len: i32;          /* length of client identifier */
//   unsigned char *clid;   /* clientid */
//   hostname: &mut String, *domain;
//   let mut netid: dhcp_netid_list;
//   let mut filter: dhcp_netid;
//
//   let mut addr6: addrlist;
//
//   addr: net::IpAddr;
//   decline_time: time::Instant;
//   let mut lease_time: u32;
//   let mut hwaddr: hwaddr_config;
//   let mut next: dhcp_config;
// };
pub struct dhcp_config {
    pub flags: u32,
    pub clid_len: i32,
    pub clid: Vec<u8>,
    pub hostname: String,
    pub domain: String,
    pub netid: dhcp_netid_list,
    pub filter: dhcp_netid,
    pub addr6: AddrList,
    pub addr: net::IpAddr,
    pub decline_time: time::Instant,
    pub lease_time: u32,
    pub hwaddr: hwaddr_config,
    // next
}

// #define have_config(config, mask) ((config) && ((config).flags & (mask)))

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
pub const CONFIG_ADDR6_HOSTS: u32 = 16384; /* address added by from /etc/hosts */

// struct dhcp_opt {
//   opt: i32, len, flags;
//   union {
//     let mut encap: i32;
//     let mut wildcard_mask: u32;
//     unsigned char *vendor_class;
//   } u;
//   unsigned char *val;
//   let mut netid: dhcp_netid;
//   let mut next: dhcp_opt;
// };
pub struct dhcp_opt {
    pub opt: i32,
    pub len: i32,
    pub flags: i32,
    pub encap: i32,
    pub wildcard_mask: u32,
    pub vendor_class: Vec<u8>,
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
pub const DHOPT_VENDOR_PXE: u32 = 16384;

// struct dhcp_boot {
//   file: &mut String, *sname, *tftp_sname;
//   next_server: net::IpAddr;
//   let mut netid: dhcp_netid;
//   let mut next: dhcp_boot;
// };
pub struct dhcp_boot {
    pub file: String,
    pub sname: String,
    pub tftp_sname: String,
    pub next_server: net::IpAddr,
    pub netid: dhcp_netid,
    // next
}

// struct dhcp_match_name {
//   char *name;
//   let mut wildcard: i32;
//   let mut netid: dhcp_netid;
//   let mut next: dhcp_match_name;
// };
pub struct dhcp_match_name {
    pub name: String,
    pub wildcard: i32,
    pub netid: dhcp_netid,
    // next
}

// struct pxe_service {
//   u16 CSA, type;
//   menu: &mut String, *basename, *sname;
//   server: net::IpAddr;
//   let mut netid: dhcp_netid;
//   let mut next: pxe_service;
// };
pub struct pxe_service {
    pub CSA: u16,
    pub _type: u16,
    pub menu: String,
    pub basename: String,
    pub sname: String,
}

// #define DHCP_PXE_DEF_VENDOR      "PXEClient"
pub const DHCP_PXE_DEF_VENDOR: String = String::from("PXEClient");

pub const MATCH_VENDOR: u32 = 1;
pub const MATCH_USER: u32 = 2;
pub const MATCH_CIRCUIT: u32 = 3;
pub const MATCH_REMOTE: u32 = 4;
pub const MATCH_SUBSCRIBER: u32 = 5;

/* vendorclass, userclass, remote-id or circuit-id */
// struct dhcp_vendor {
//   len: i32, match_type;
//   let mut enterprise: u32;
//   char *data;
//   struct dhcp_netid netid;
//   let mut next: dhcp_vendor;
// };
pub struct dhcp_vendor {
    pub len: i32,
    pub match_type: i32,
    pub enterprise: u32,
    pub data: String,
    pub netid: dhcp_netid,
    // next
}

// struct dhcp_pxe_vendor {
//   char *data;
//   let mut next: dhcp_pxe_vendor;
// };
pub struct dhcp_pxe_vendor {
    pub data: String,
    // next
}

// struct dhcp_mac {
//   let mut mask: u32;
//   hwaddr_len: i32, hwaddr_type;
//   unsigned char hwaddr[DHCP_CHADDR_MAX];
//   struct dhcp_netid netid;
//   let mut next: dhcp_mac;
// };
pub struct dhcp_mac {
    pub mask: u32,
    pub hwaddr_len: u32,
    pub hwaddr_type: u32,
    pub hwaddr: [u8; 16],
    pub netid: dhcp_netid,
    // next
}

// struct dhcp_bridge {
//   char iface[IF_NAMESIZE];
//   struct dhcp_bridge *alias, *next;
// };
pub struct dhcp_bridge {
    pub iface: String,
    // alias
    // next
}

// struct cond_domain {
//   domain: &mut String, *prefix;
//   start: net::IpAddr, end;
//   struct in6_addr start6, end6;
//   is6: i32, indexed;
//   let mut next: cond_domain;
// };
pub struct cond_domain {
    pub domain: String,
    pub prefix: String,
    pub start: net::Ipv4Addr,
    pub end: net::Ipv4Addr,
    pub start6: net::Ipv6Addr,
    pub end6: net::Ipv6Addr,
    pub is6: i32,
    pub indexed: i32,
    // next
}

// struct ra_interface {
//   char *name;
//   char *mtu_name;
//   interval: i32, lifetime, prio, mtu;
//   let mut next: ra_interface;
// };
pub struct ra_interface {
    pub name: String,
    pub mtu_name: String,
    pub interval: i32,
    pub lifetime: i32,
    pub prio: i32,
    pub mtu: i32,
    //next
}

// struct shared_network {
//   let mut if_index: i32;
//   match_addr: net::IpAddr, shared_addr;
//
//   /* shared_addr == 0 for IP6 entries. */
//   struct in6_addr match_addr6, shared_addr6;
//
//   let mut next: shared_network;
// };
pub struct shared_network {
    pub if_index: i32,
    pub match_addr: net::IpAddr,
    pub shared_addr: net::IpAddr,
    pub match_addr6: net::Ipv6Addr,
    pub shared_addr6: net::Ipv6Addr,
    // next
}

// #define CONTEXT_         (1u<<0)
// #define CONTEXT_NETMASK        (1u<<1)
// #define CONTEXT_BRDCAST        (1u<<2)
// #define CONTEXT_PROXY          (1u<<3)
// #define CONTEXT_RA_ROUTER      (1u<<4)
// #define CONTEXT_RA_DONE        (1u<<5)
// #define CONTEXT_RA_NAME        (1u<<6)
// #define CONTEXT_RA_STATELESS   (1u<<7)
// #define CONTEXT_DHCP           (1u<<8)
// #define CONTEXT_DEPRECATE      (1u<<9)
// #define CONTEXT_TEMPLATE       (1u<<10)    /* create contexts using addresses */
// #define CONTEXT_CONSTRUCTED    (1u<<11)
// #define CONTEXT_GC             (1u<<12)
// #define CONTEXT_RA             (1u<<13)
// #define CONTEXT_CONF_USED      (1u<<14)
// #define CONTEXT_USED           (1u<<15)
// #define CONTEXT_OLD            (1u<<16)
// pub const CONTEXT_V: u32 = 6;             (1u<<17)
// #define CONTEXT_RA_OFF_LINK    (1u<<18)
// #define CONTEXT_SETLEASE       (1u<<19)

// struct ping_result {
//   addr: net::IpAddr;
//   time: time::Instant;
//   let mut hash: u32;
//   let mut next: ping_result;
// };
pub struct ping_result {
    pub addr: net::IpAddr,
    pub time: time::Instant,
    pub hash: u32,
    // next
}

type off_t = usize;
type dev_t = u32;
type ino_t = u32;

// struct tftp_file {
//   refcount: i32, fd;
//   off_t size;
//   dev_t dev;
//   ino_t inode;
//   char filename[];
// };
pub struct tftp_file {
    pub refcount: i32,
    pub fd: i32,
    pub size: off_t,
    pub dev: dev_t,
    pub inode: ino_t,
    pub filename: String,
}

// struct tftp_transfer {
//   let mut sockfd: i32;
//   timeout: time::Instant;
//   let mut backoff: i32;
//   unsigned block: i32, blocksize, expansion;
//   off_t offset;
//   union mysockaddr peer;
//   union all_addr source;
//   let mut if_index: i32;
//   char opt_blocksize, opt_transize, netascii, carrylf;
//   let mut file: tftp_file;
//   let mut next: tftp_transfer;
// };
pub struct tftp_transfer {
    pub socfd: i32,
    pub timeout: time::Instant,
    pub backoff: i32,
    pub block: u32,
    pub blocksize: u32,
    pub expansion: u32,
    pub offset: off_t,
    pub peer: net::IpAddr,
    pub source: net::IpAddr,
    pub if_index: i32,
    pub opt_blocksize: u8,
    pub opt_transize: u8,
    pub netascii: u8,
    pub carrylf: u8,
    pub file: tftp_file,
    // next
}

// struct addr_list {
//   addr: net::IpAddr;
//   let mut next: addr_list;
// };
// pub struct AddrList {
//     pub addr: net::IpAddr,
//     // next
// }

// struct tftp_prefix {
//   char *interface;
//   char *prefix;
//   let mut missing: i32;
//   let mut next: tftp_prefix;
// };
pub struct tftp_prefix {
    pub interface: String,
    pub prefix: String,
    pub missing: i32,
    // next
}

// struct dhcp_relay {
//   union all_addr local, server;
//   char *interface; /* Allowable interface for replies from server, and dest for IPv6 multicast */
//   let mut iface_index: i32; /* working - interface in which requests arrived, for return */
//   struct dhcp_relay *current, *next;
// };
pub struct dhcp_relay {
    pub local: net::IpAddr,
    pub server: net::IpAddr,
    pub interface: String,
    pub iface_index: i32,
    // current
    // next
}

pub struct DnsmasqDaemon {
    /* datastuctures representing the command-line and
    config file arguments. All set (including defaults)
    in option.c */
    //   unsigned int options[OPTION_SIZE];
    // pub options: [u32;OPTION_SIZE],
    // struct resolvc default_resolv, *resolv_files;
    pub old: ArpRecord,
    pub arps: Vec<ArpRecord>,
    pub default_resolv: Vec<resolvc>,
    pub resolv_files: Vec<resolvc>,
    // last_resolv: time::Instant;
    pub last_resolv: time::Instant,
    // char *servers_file;
    pub servers_file: String,
    // let mut mxnames: mx_srv_record;
    pub mxnames: mx_srv_record,
    // let mut naptr: naptr;
    pub naptr: naptr,
    // struct txt_record *txt, *rr;
    pub txt: txt_record,
    pub rr: txt_record,
    // let mut ptr: ptr_record;
    pub ptr: ptr_record,
    // struct host_record *host_records, *host_records_tail;
    pub host_records: host_record,
    pub host_records_tail: host_record,
    // let mut cnames: cname;
    pub cnames: cname,
    // let mut auth_zones: auth_zone;
    pub auth_zones: auth_zone,
    // let mut int_names: interface_name;
    pub int_names: interface_name,
    // char *mxtarget;
    pub mxtarget: String,
    // let mut add_subnet4: mysubnet;
    pub add_subnet4: mysubnet,
    //let mut add_subnet6: mysubnet;
    pub add_subnet6: mysubnet,
    // char *lease_file;
    pub lease_file: String,
    // username: &mut String, *groupname, *scriptuser;
    pub username: String,
    pub groupname: String,
    pub scriptuser: String,
    // char *luascript;
    pub luascript: String,
    // authserver: &mut String, *hostmaster;
    pub authserver: String,
    pub hostmaster: String,
    // let mut authinterface: iname;
    pub authinterface: InterfaceParams,
    //let mut secondary_forward_server: name_list;
    // group_set: i32, osport;
    pub group_set: i32,
    pub osport: i32,
    // char *domain_suffix;
    pub domain_suffix: String,
    // struct cond_domain *cond_domain, *synth_domains;
    pub cond_domain: cond_domain,
    pub synth_domains: cond_domain,
    // char *runfile;
    pub runfile: String,
    // char *lease_change_command;
    pub lease_change_command: String,
    // struct iname *if_names, *if_addrs, *if_except, *dhcp_except, *auth_peers, *tftp_interfaces;
    pub if_names: Vec<InterfaceParams>,
    pub if_addrs: Vec<InterfaceParams>,
    pub if_except: Vec<InterfaceParams>,
    pub dhcp_except: InterfaceParams,
    pub auth_peers: InterfaceParams,
    // struct bogus_addr *bogus_addr, *ignore_addr;
    pub bogus_addr: bogus_addr,
    pub ignore_addr: bogus_addr,
    // let mut servers: server;
    pub servers: Vec<Server>,
    // let mut ipsets: ipsets;
    pub ipsets: ipsets,
    // let mut log_fac: i32; /* log facility */
    pub log_fac: i32,
    // char *log_file; /* optional log file */
    pub log_file: String,
    // let mut max_logs: i32;  /* queue limit */
    pub max_logs: i32,
    // cachesize: i32, ftabsize;
    pub cachesize: i32,
    pub ftabsize: i32,
    // port: i32, query_port, min_port, max_port;
    pub port: i32,
    pub query_port: i32,
    pub min_port: i32,
    pub max_port: i32,
    // unsigned local_ttl: i32, neg_ttl, max_ttl, min_cache_ttl, max_cache_ttl, auth_ttl, dhcp_ttl, use_dhcp_ttl;
    pub local_ttl: u32,
    pub neg_ttl: u32,
    pub max_ttl: u32,
    pub min_cache_ttl: u32,
    pub max_cache_ttl: u32,
    pub auth_ttl: u32,
    // char *dns_client_id;
    pub dns_client_id: String,
    // let mut addn_hosts: hostsfile;
    pub addn_hosts: hostsfile,
    // struct dhcp_context *dhcp, *dhcp6;
    pub dhcp: DhcpContext,
    pub dhcp6: Vec<DhcpContext>,
    // let mut ra_interfaces: ra_interface;
    pub ra_interfaces: ra_interface,
    // let mut dhcp_conf: dhcp_config;
    pub dhcp_conf: dhcp_config,
    // struct dhcp_opt *dhcp_opts, *dhcp_match, *dhcp_opts6, *dhcp_match6;
    pub dhcp_opts: dhcp_opt,
    pub dhcp_match: dhcp_opt,
    pub dhcp_opts6: dhcp_opt,
    pub dhcp_match6: dhcp_opt,
    // let mut dhcp_name_match: dhcp_match_name;
    pub dhcp_name_match: dhcp_match_name,
    // let mut dhcp_pxe_vendors: dhcp_pxe_vendor;
    pub dhcp_pxe_vendors: dhcp_pxe_vendor,
    // let mut dhcp_vendors: dhcp_vendor;
    pub dhcp_vendors: dhcp_vendor,
    // let mut dhcp_macs: dhcp_mac;
    pub dhcp_macs: dhcp_mac,
    // let mut boot_config: dhcp_boot;
    pub boot_config: dhcp_boot,
    // let mut pxe_services: pxe_service;
    pub pxe_services: pxe_service,
    // let mut tag_if: tag_if;
    pub tag_if: tag_if,
    // let mut override_relays: addr_list;
    pub override_relays: AddrList,
    // struct dhcp_relay *relay4, *relay6;
    pub relay4: dhcp_relay,
    pub relay6: dhcp_relay,
    // let mut delay_conf: delay_config;
    pub delay_conf: delay_config,
    // let mut override: i32;
    pub _override: i32,
    // let mut enable_pxe: i32;
    pub enable_pxe: i32,
    // doing_ra: i32, doing_dhcp6;
    pub doing_ra: i32,
    pub doing_dhcp6: i32,
    // struct dhcp_netid_list *dhcp_ignore, *dhcp_ignore_names, *dhcp_gen_names;
    pub dhcp_ignore: dhcp_netid_list,
    pub dhcp_ignore_names: dhcp_netid_list,
    pub dhcp_gen_names: dhcp_netid_list,
    // struct dhcp_netid_list *force_broadcast, *bootp_dynamic;
    pub force_broadcast: dhcp_netid_list,
    pub bootp_dynamic: dhcp_netid_list,
    // struct hostsfile *dhcp_hosts_file, *dhcp_opts_file, *dynamic_dirs;
    pub dhcp_hosts_file: hostsfile,
    pub dhcp_opts_file: hostsfile,
    pub dynamic_dirs: hostsfile,
    // dhcp_max: i32, tftp_max, tftp_mtu;
    pub dhcp_max: i32,
    pub tftp_max: i32,
    pub tftp_mtu: i32,
    // dhcp_server_port: i32, dhcp_client_port;
    pub dhcp_server_port: i32,
    pub dhcp_client_port: i32,
    // start_tftp_port: i32, end_tftp_port;
    pub start_tftp_port: i32,
    pub end_tftp_port: i32,
    // let mut min_leasetime: u32;
    pub min_leasetime: u32,
    // let mut doctors: doctor;
    pub doctors: doctor,
    // u16 edns_pktsz;
    pub edns_pktsz: u16,
    // char *tftp_prefix;
    pub tftp_prefix: String,
    // let mut if_prefix: tftp_prefix; /* per-interface TFTP prefixes */
    pub if_prefix: Vec<tftp_prefix>,
    // unsigned duid_enterprise: i32, duid_config_len;
    pub duid_enterprise: u32,
    pub duid_config_len: u32,
    // unsigned char *duid_config;
    pub duid_config: Vec<u8>,
    // char *dbus_name;
    pub dbus_name: String,
    // char *ubus_name;
    pub ubus_name: String,
    // char *dump_file;
    pub dump_file: String,
    // let mut dump_mask: i32;
    pub dump_mask: i32,
    // unsigned soa_sn: i32, soa_refresh, soa_retry, soa_expiry;
    pub soa_sn: libc::c_ulong,
    pub soa_refresh: libc::c_ulong,
    pub soa_retry: libc::c_ulong,
    pub soa_expiry: libc::c_ulong,
    // u32 metrics[__METRIC_MAX];
    pub metrics: [u32; 0xff],
    //
    // let mut ds: ds_config;
    pub ds: Vec<DsConfig>,
    // char *timestamp_file;
    pub timestamp_file: String,
    //
    /* globally used stuff for DNS */
    // char *packet; /* packet buffer */
    pub packet: String,
    // let mut packet_buff_sz: i32; /* size of above */
    pub packet_buff_sz: i32,
    // char *namebuff; /* MAXDNAME size buffer */
    pub namebuff: String,
    //
    // char *keyname; /* MAXDNAME size buffer */
    pub keyname: String,
    // char *workspacename; /* ditto */
    pub workspacename: String,
    // unsigned long *rr_status; /* ceiling in TTL from DNSSEC or zero for insecure */
    pub rr_status: Vec<libc::c_ulong>,
    // let mut rr_status_sz: i32;
    pub rr_status_sz: i32,
    // let mut dnssec_no_time_check: i32;
    pub dnssec_no_time_check: i32,
    // let mut back_to_the_future: i32;
    pub back_to_the_future: i32,
    //
    // let mut frec_list: frec;
    pub frec_list: frec,
    // let mut free_frec_src: frec_src;
    pub free_frec_src: frec_src,
    // let mut frec_src_count: i32;
    pub frec_src_count: i32,
    // let mut sfds: serverfd;
    pub sfds: serverfd,
    // let mut interfaces: irec;
    pub interfaces: irec,
    // let mut listeners: listener;
    pub listeners: listener,
    // let mut last_server: server;
    pub last_server: Server,
    // forwardtime: time::Instant;
    pub forwardtime: time::Instant,
    // let mut forwardcount: i32;
    pub forwardcount: i32,
    // let mut srv_save: server; /* Used for resend on DoD */
    pub srv_save: Server,
    // packet_len: usize;       /*      "        "        */
    pub packet_len: usize,
    // let mut rfd_save: randfd; /*      "        "        */
    pub rfd_save: randfd,
    // pid_t tcp_pids[MAX_PROCS];
    pub tcp_pids: [pid_t; MAX_PROCS],
    pub tcp_pipes: [i32; MAX_PROCS],
    pub pipe_to_parent: u32,
    // struct randfd randomsocks[RANDOM_SOCKS];
    pub randomsocks: [randfd; RANDOM_SOCKS],
    // let mut v6pktinfo: i32;
    pub v6pktinfo: i32,
    // let mut interface_addrs: addrlist; /* list of all addresses/prefix lengths associated
    // with all local interfaces */
    pub interface_addrs: AddrList,
    // log_id: i32, log_display_id; /* ids of transactions for logging */
    pub log_id: i32,
    pub log_display_id: i32,
    // union mysockaddr *log_source_addr;
    pub log_source_addr: net::IpAddr,
    /* DHCP state */
    // dhcpfd: i32, helperfd, pxefd;
    pub dhcpfd: i32,
    pub helperfd: i32,
    pub pxefd: i32,
    //  HAVE_INOTIFY
    // let mut inotifyfd: i32;
    pub inotifyfd: i32,
    //
    // #if defined(HAVE_LINUX_NETWORK)
    // netlinkfd: i32, kernel_version;
    pub netlinkfd: i32,
    pub kernel_version: i32,
    //
    // dhcp_raw_fd: i32, dhcp_icmp_fd, routefd;
    pub dhcp_raw_fd: i32,
    pub dhcp_icmp_fd: i32,
    pub routefd: i32,
    //
    // let mut dhcp_packet: iovec;
    pub dhcp_packet: Vec<u8>,
    // dhcp_buff: &mut String, *dhcp_buff2, *dhcp_buff3;
    pub dhcp_buff: String,
    pub dhcp_buff2: String,
    pub dhcp_buff3: String,
    // let mut ping_results: ping_result;
    pub ping_results: ping_result,
    // FILE *lease_stream;
    pub lease_stream: fs::File,
    // let mut bridges: dhcp_bridge;
    pub bridges: dhcp_bridge,
    // let mut shared_networks: shared_network;
    pub shared_networks: shared_network,
    //
    // let mut duid_len: i32;
    pub duid_len: i32,
    //  unsigned char *duid;
    pub duid: Vec<u8>,
    // let mut outpacket: iovec;
    pub outpacket: Vec<u8>,
    // dhcp6fd: i32, icmp6fd;
    pub dhcp6fd: i32,
    pub icmp6fd: i32,
    //
    /* DBus stuff */
    /* void * here to avoid depending on dbus headers outside dbus.c */
    // dbus: Vec<u8>;
    pub dbus: *mut libc::c_void,
    //  HAVE_DBUS
    // let mut watches: watch;
    pub watches: watch,
    //
    /* UBus stuff */
    //  HAVE_UBUS
    /* void * here to avoid depending on ubus headers outside ubus.c */
    // ubus: Vec<u8>;
    pub ubus: *mut libc::c_void,
    //

    /* TFTP stuff */
    // struct tftp_transfer *tftp_trans, *tftp_done_trans;
    pub tftp_trans: Vec<tftp_transfer>,
    pub tftp_don_trans: tftp_transfer,
    /* utility string buffer, hold max sized IP address as string */
    // char *addrbuff;
    pub addrbuff: String,
    // char *addrbuff2; /* only allocated when OPT_EXTRALOG */
    pub addrbuff2: String,
    //  HAVE_DUMPFILE
    /* file for packet dumps. */
    // let mut dumpfd: i32;
    pub dumpfd: i32,
    pub opt_tftp: bool,
    pub opt_cleverbind: bool,
    pub opt_boguspriv: bool,
    pub opt_filter: bool,
    pub opt_log: bool,
    pub opt_selfmx: bool,
    pub opt_no_hosts: bool,
    pub opt_no_poll: bool,
    pub opt_debug: bool,
    pub opt_order: bool,
    pub opt_no_resolv: bool,
    pub opt_no_expand: bool,
    pub opt_localmx: bool,
    pub opt_no_neg: bool,
    pub opt_nodots_local: bool,
    pub opt_nowild: bool,
    pub opt_ethers: bool,
    pub opt_resolv_domain: bool,
    pub opt_no_fork: bool,
    pub opt_authoritative: bool,
    pub opt_localise: bool,
    pub opt_dbus: bool,
    pub opt_dhcp_fqdn: bool,
    pub opt_no_ping: bool,
    pub opt_lease_ro: bool,
    pub opt_all_servers: bool,
    pub opt_reload: bool,
    pub opt_local_rebind: bool,
    pub opt_tftp_secure: bool,
    pub opt_tftp_noblock: bool,
    pub opt_log_opts: bool,
    pub opt_tftp_apref_ip: bool,
    pub opt_no_override: bool,
    pub opt_no_rebind: bool,
    pub opt_add_mac: bool,
    pub opt_dnssec_proxy: bool,
    pub opt_consec_addr: bool,
    pub opt_conntrack: bool,
    pub opt_fqdn_update: bool,
    pub opt_ra: bool,
    pub opt_tftp_lc: bool,
    pub opt_client_subnet: bool,
    pub opt_quiet_dhcp: bool,
    pub opt_quiet_dhcp6: bool,
    pub opt_quiet_ra: bool,
    pub opt_dnssec_valid: bool,
    pub opt_dnssec_time: bool,
    pub opt_dnssec_debug: bool,
    pub opt_dnssec_ign_ns: bool,
    pub opt_local_service: bool,
    pub opt_loop_detect: bool,
    pub opt_extralog: bool,
    pub opt_tftp_no_fail: bool,
    pub opt_script_arp: bool,
    pub opt_mac_b64: bool,
    pub opt_mac_hex: bool,
    pub opt_tftp_apref_mac: bool,
    pub opt_rapid_commit: bool,
    pub opt_ubus: bool,
    pub opt_ignore_clid: bool,
    pub opt_single_port: bool,
    pub opt_lease_renew: bool,
    pub opt_last: bool, //
}

/* cache.c */
// void cache_init();
// void next_uid(struct crec *crecp);
// void log_query(unsigned flags: i32, name: &mut String, union all_addr *addr, arg: &mut String);
// char *record_source(unsigned int index);
// char *querystr(desc: &mut String, u16 type);
// int cache_find_non_terminal(name: &mut String, now: &time::Instant);
// struct crec *cache_find_by_addr(struct crec *crecp,
//         union all_addr *addr, now: &time::Instant,
//         unsigned int prot);
// struct crec *cache_find_by_name(struct crec *crecp,
//         name: &mut String, now: &time::Instant, unsigned int prot);
// void cache_end_insert();
// void cache_start_insert();
// int cache_recv_insert(now: time::Instant, fd: i32);
// struct crec *cache_insert(name: &mut String, union all_addr *addr, u16 class,
//         now: time::Instant, unsigned ttl: i32, unsigned int flags);
// void cache_reload();
// void cache_add_dhcp_entry(host_name: &mut String, prot: i32, union all_addr *host_address, ttd: &time::Instant);
// a_record_from_hosts: net::IpAddr(name: &mut String, now: &time::Instant);
// void cache_unhash_dhcp();
// void dump_cache(now: time::Instant);
// #ifndef NO_ID
// int cache_make_stat(struct txt_record *t);
//
// char *cache_get_name(struct crec *crecp);
// char *cache_get_cname_target(struct crec *crecp);
// struct crec *cache_enumerate(int init);
// int read_hostsfile(filename: &mut String, unsigned index: i32, cache_size: i32,
//        struct crec **rhash, hashsz: i32);

/* blockdata.c */
// void blockdata_init();
// void blockdata_report();
// struct blockdata *blockdata_alloc(data: &mut String, len: usize);
// blockdata_retrieve: Vec<u8>(struct blockdata *block, len: usize, data: Vec<u8>);
// struct blockdata *blockdata_read(fd: i32, len: usize);
// void blockdata_write(struct blockdata *block, len: usize, fd: i32);
// void blockdata_free(struct blockdata *blocks);

/* domain.c */
// char *get_domain(addr: net::IpAddr);
// char *get_domain6(addr: &mut net::IpAddr);
// int is_name_synthetic(flags: i32, name: &mut String, union all_addr *addr);
// int is_rev_synth(flag: i32, union all_addr *addr, name: &mut String);

/* rfc1035.c */
// int extract_name(struct dns_header *header, plen: usize, unsigned char **pp,
//                  name: &mut String, isExtract: i32, extrabytes: i32);
// pub fn skip_name(ansp: &mut Vec<u8>, struct dns_header *header, plen: usize, extrabytes: i32) -> &mut Vec<u8>;
// pub fn skip_questions(struct dns_header *header, plen: usize) -> &mut Vec<u8>;
// pub fn skip_section(ansp: &mut Vec<u8>, count: i32, struct dns_header *header, plen: usize) -> &mut Vec<u8>;
// unsigned int extract_request(struct dns_header *header, qlen: usize,
//              name: &mut String, u16 *typep);
// setup_reply: usize(struct dns_header *header, size_t  qlen,
//        union all_addr *addrp, unsigned flags: i32,
//        unsigned long ttl);
// int extract_addresses(struct dns_header *header, qlen: usize, name: &mut String,
//           now: time::Instant, char **ipsets, is_sign: i32, check_rebind: i32,
//           no_cache_dnssec: i32, secure: i32, int *doctored);
// answer_request: usize(struct dns_header *header, limit: &mut String, qlen: usize,
//           local_addr: net::IpAddr, local_netmask: net::IpAddr,
//           now: time::Instant, ad_reqd: i32, do_bit: i32, have_pseudoheader: i32);
// int check_for_bogus_wildcard(struct dns_header *header, qlen: usize, name: &mut String,
//            struct bogus_addr *baddr, now: &time::Instant);
// int check_for_ignored_address(struct dns_header *header, qlen: usize, struct bogus_addr *baddr);
// int check_for_local_domain(name: &mut String, now: &time::Instant);
// resize_packet: usize(struct dns_header *header, plen: usize,
//       pheader: &mut Vec<u8>, hlen: usize);
// int add_resource_record(struct dns_header *header, limit: &mut String, int *truncp,
//       nameoffset: i32, unsigned char **pp, unsigned ttl: i32,
//       int *offset, u16 type, u16 class, format: &mut String, ...);
// int in_arpa_name_2_addr(namein: &mut String, union all_addr *addrp);
// int private_net(addr: net::IpAddr, ban_localhost: i32);

/* auth.c */
//
// answer_auth: usize(struct dns_header *header, limit: &mut String, qlen: usize,
//        now: time::Instant, union mysockaddr *peer_addr, local_query: i32,
//        do_bit: i32, have_pseudoheader: i32);
// int in_zone(struct auth_zone *zone, name: &mut String, char **cut);
//

/* dnssec.c */
// dnssec_generate_query: usize(struct dns_header *header, end: &mut Vec<u8>, name: &mut String, class: i32, type: i32, edns_pktsz: i32);
// int dnssec_validate_by_ds(now: time::Instant, struct dns_header *header, plen: usize, name: &mut String, keyname: &mut String, class: i32);
// int dnssec_validate_ds(now: time::Instant, struct dns_header *header, plen: usize, name: &mut String, keyname: &mut String, class: i32);
// int dnssec_validate_reply(now: time::Instant, struct dns_header *header, plen: usize, name: &mut String, keyname: &mut String, int *class,
//         check_unsigned: i32, int *neganswer, int *nons, int *nsec_ttl);
// int dnskey_keytag(alg: i32, flags: i32, key: &mut Vec<u8>, keylen: i32);
// filter_rrsigs: usize(struct dns_header *header, plen: usize);
// int setup_timestamp();

/* hash_questions.c */
// void hash_questions_init();
// pub fn hash_questions(struct dns_header *header, plen: usize, name: &mut String) -> &mut Vec<u8>;

/* crypto.c */
// const struct nettle_hash *hash_find(name: &mut String);
// int hash_init(const struct nettle_hash *hash, void **ctxp, unsigned char **digestp);
// int verify(struct blockdata *key_data, unsigned key_len: i32, sig: &mut Vec<u8>, sig_len: usize,
//      digest: &mut Vec<u8>, digest_len: usize, algo: i32);
// char *ds_digest_name(int digest);
// char *algo_digest_name(int algo);
// char *nsec3_digest_name(int digest);

/* util.c */
// void rand_init();
// u16 rand16();
// u32 rand32();
// u64 rand64();
// int legal_hostname(name: &mut String);
// char *canonicalise(in: &mut String, int *nomem);
// pub fn do_rfc1035_name(p: &mut Vec<u8>, sval: &mut String, limit: &mut String) -> &mut Vec<u8>;
// safe_malloc: Vec<u8>(size: usize);
// void safe_strncpy(dest: &mut String, const src: &mut String, size: usize);
// void safe_pipe(int *fd, read_noblock: i32);
// whine_malloc: Vec<u8>(size: usize);
// int sa_len(union mysockaddr *addr);
// int sockaddr_isequal(union mysockaddr *s1, union mysockaddr *s2);
// int hostname_isequal(const a: &mut String, const char *b);
// int hostname_issubdomain(a: &mut String, b: &mut String);
// dnsmasq_time: time::Instant();
// int netmask_length(mask: net::IpAddr);
// int is_same_net(a: net::IpAddr, b: net::IpAddr, mask: net::IpAddr);
// int is_same_net6(a: &mut net::IpAddr, b: &mut net::IpAddr, prefixlen: i32);
// u64 addr6part(addr: &mut net::IpAddr);
// void setaddr6part(addr: &mut net::IpAddr, u64 host);
// int retry_send(src: usize);
// void prettyprint_time(buf: &mut String, unsigned int t);
// int prettyprint_addr(union mysockaddr *addr, buf: &mut String);
// int parse_hex(in: &mut String, out: &mut Vec<u8>, maxlen: i32,
//         wildcard_mask: &mut u32, int *mac_type);
// int memcmp_masked(a: &mut Vec<u8>, b: &mut Vec<u8>, len: i32,
//       unsigned int mask);
// int expand_buf(struct iovec *iov, size: usize);
// char *print_mac(buff: &mut String, mac: &mut Vec<u8>, len: i32);
// int read_write(fd: i32, packet: &mut Vec<u8>, size: i32, rw: i32);
// void close_fds(max_fd: i32, spare1: i32, spare2: i32, spare3: i32);
// int wildcard_match(const char* wildcard, const char* match);
// int wildcard_matchn(const char* wildcard, const char* match, num: i32);
//
// int kernel_version();
//

/* log.c */
// void die(message: &mut String, arg1: &mut String, exit_code: i32) ATTRIBUTE_NORETURN;
// int log_start(struct passwd *ent_pw, errfd: i32);
// int log_reopen(log_file: &mut String);

// void my_syslog(priority: i32, const format: &mut String, ...);

// void set_log_writer();
// void check_log_writer(int force);
// void flush_log();

/* option.c */
// void read_opts (argc: i32, char **argv, compile_opts: &mut String);
// char *option_string(prot: i32, unsigned opt: i32, val: &mut Vec<u8>,
//         opt_len: i32, buf: &mut String, buf_len: i32);
// void reread_dhcp();
// void read_servers_file();
// void set_option_bool(unsigned int opt);
// void reset_option_bool(unsigned int opt);
// struct hostsfile *expand_filelist(struct hostsfile *list);
// char *parse_server(arg: &mut String, union mysockaddr *addr,
//        union mysockaddr *source_addr, interface: &mut String, int *flags);
// int option_read_dynfile(file: &mut String, flags: i32);

/* forward.c */
// void reply_query(fd: i32, family: i32, now: &time::Instant);
// void receive_query(struct listener *listen, now: &time::Instant);
// unsigned char *tcp_request(confd: i32, now: &time::Instant,
//          union mysockaddr *local_addr, netmask: net::IpAddr, auth_dns: i32);
// void server_gone(struct server *server);
// struct frec *get_new_frec(now: time::Instant, int *wait, struct frec *force);
// int send_from(fd: i32, nowild: i32, packet: &mut String, len: usize,
//          union mysockaddr *to, union all_addr *source,
//          unsigned int iface);
// void resend_query();
// struct randfd *allocate_rfd(int family);
// void free_rfd(struct randfd *rfd);

/* network.c */
// int indextoname(fd: i32, index: i32, name: &mut String);
// int local_bind(fd: i32, union mysockaddr *addr, intname: &mut String, unsigned ifindex: i32, is_tcp: i32);
// int random_sock(int family);
// void pre_allocate_sfds();
// int reload_servers(fname: &mut String);
// void mark_servers(int flag);
// void cleanup_servers();
// void add_update_server(flags: i32,
//            union mysockaddr *addr,
//            union mysockaddr *source_addr,
//            const interface: &mut String,
//            const char *domain);
// void check_servers();
// int enumerate_interfaces(int reset);
// void create_wildcard_listeners();
// void create_bound_listeners(int dienow);
// void warn_bound_listeners();
// void warn_wild_labels();
// void warn_int_names();
// int is_dad_listeners();
// int iface_check(family: i32, union all_addr *addr, name: &mut String, int *auth);
// int loopback_exception(fd: i32, family: i32, union all_addr *addr, name: &mut String);
// int label_exception(index: i32, family: i32, union all_addr *addr);
// int fix_fd(int fd);
// int tcp_interface(fd: i32, af: i32);
// int set_ipv6pktinfo(int fd);
//
// void join_multicast(int dienow);
//
// #if defined(HAVE_LINUX_NETWORK) || defined()
// void newaddress(now: time::Instant);
//

/* dhcp.c */
//
// void dhcp_init();
// void dhcp_packet(now: time::Instant, pxe_fd: i32);
// struct dhcp_context *address_available(struct dhcp_context *context,
//                taddr: net::IpAddr,
//                struct dhcp_netid *netids);
// struct dhcp_context *narrow_context(struct dhcp_context *context,
//             taddr: net::IpAddr,
//             struct dhcp_netid *netids);
// struct ping_result *do_icmp_ping(now: time::Instant, addr: net::IpAddr,
//          unsigned hash: i32, loopback: i32);
// int address_allocate(struct dhcp_context *context,
//          struct in_addr *addrp, hwaddr: &mut Vec<u8>, hw_len: i32,
//          struct dhcp_netid *netids, now: &time::Instant, loopback: i32);
// void dhcp_read_ethers();
// struct dhcp_config *config_find_by_address(struct dhcp_config *configs, addr: net::IpAddr);
// char *host_from_dns(addr: net::IpAddr);
//

/* lease.c */
//
// void lease_update_file(now: time::Instant);
// void lease_update_dns(int force);
// void lease_init(now: time::Instant);
// struct dhcp_lease *lease4_allocate(addr: net::IpAddr);
//
// struct dhcp_lease *lease6_allocate(addrp: &mut net::IpAddr, lease_type: i32);
// struct dhcp_lease *lease6_find(clid: &mut Vec<u8>, clid_len: i32,
//              lease_type: i32, unsigned iaid: i32, addr: &mut net::IpAddr);
// void lease6_reset();
// struct dhcp_lease *lease6_find_by_client(struct dhcp_lease *first, lease_type: i32,
//            clid: &mut Vec<u8>, clid_len: i32, unsigned int iaid);
// struct dhcp_lease *lease6_find_by_addr(net: &mut net::IpAddr, prefix: i32, u64 addr);
// u64 lease_find_max_addr6(struct dhcp_context *context);
// void lease_ping_reply(sender: &mut net::IpAddr, packet: &mut Vec<u8>, interface: &mut String);
// void lease_update_slaac(now: time::Instant);
// void lease_set_iaid(struct dhcp_lease *lease, unsigned int iaid);
// void lease_make_duid(now: time::Instant);
//
// void lease_set_hwaddr(struct dhcp_lease *lease, const hwaddr: &mut Vec<u8>,
//           const clid: &mut Vec<u8>, hw_len: i32, hw_type: i32,
//           clid_len: i32, now: &time::Instant, force: i32);
// void lease_set_hostname(struct dhcp_lease *lease, const name: &mut String, auth: i32, domain: &mut String, config_domain: &mut String);
// void lease_set_expires(struct dhcp_lease *lease, unsigned len: i32, now: &time::Instant);
// void lease_set_interface(struct dhcp_lease *lease, interface: i32, now: &time::Instant);
// struct dhcp_lease *lease_find_by_client(hwaddr: &mut Vec<u8>, hw_len: i32, hw_type: i32,
//           clid: &mut Vec<u8>, clid_len: i32);
// struct dhcp_lease *lease_find_by_addr(addr: net::IpAddr);
// lease_find_max_addr: net::IpAddr(struct dhcp_context *context);
// void lease_prune(struct dhcp_lease *target, now: &time::Instant);
// void lease_update_from_configs();
// int do_script_run(now: time::Instant);
// void rerun_scripts();
// void lease_find_interfaces(now: time::Instant);
//
// void lease_add_extradata(struct dhcp_lease *lease, data: &mut Vec<u8>,
//        unsigned len: i32, delim: i32);
//
//

/* rfc2131.c */
//
// dhcp_reply: usize(struct dhcp_context *context, iface_name: &mut String, int_index: i32,
//       sz: usize, now: &time::Instant, unicast_dest: i32, loopback: i32,
//       int *is_inform, pxe: i32, fallback: net::IpAddr, recvtime: &time::Instant);
// unsigned char *extended_hwaddr(hwtype: i32, hwlen: i32, hwaddr: &mut Vec<u8>,
//              clid_len: i32, clid: &mut Vec<u8>, int *len_out);
//

/* dnsmasq.c */
//
// int make_icmp_sock();
// int icmp_ping(addr: net::IpAddr);
// int delay_dhcp(start: time::Instant, sec: i32, fd: i32, uint32_t addr, u16 id);
//
// void queue_event(int event);
// void send_alarm(event: time::Instant, now: &time::Instant);
// void send_event(fd: i32, event: i32, data: i32, msg: &mut String);
// void clear_cache_and_reload(now: time::Instant);

/* netlink.c */
//
// char *netlink_init();
// void netlink_multicast();
//

/* bpf.c */
//
// void init_bpf();
// void send_via_bpf(struct dhcp_packet *mess, len: usize,
//       iface_addr: net::IpAddr, struct ifreq *ifr);
// void route_init();
// void route_sock();
//

/* bpf.c or netlink.c */
// int iface_enumerate(family: i32, parm: Vec<u8>, int (callback)());

/* dbus.c */
//  HAVE_DBUS
// char *dbus_init();
// void check_dbus_listeners();
// void set_dbus_listeners();
// #  ifdef
// void emit_dbus_signal(action: i32, struct dhcp_lease *lease, hostname: &mut String);
// #  endif
//

/* ubus.c */
//  HAVE_UBUS
// void ubus_init();
// void set_ubus_listeners();
// void check_ubus_listeners();
// void ubus_event_bcast(const type: &mut String, const mac: &mut String, const ip: &mut String, const name: &mut String, const char *interface);
//

/* ipset.c */
//  HAVE_IPSET
// void ipset_init();
// int add_to_ipset(const setname: &mut String, const union all_addr *ipaddr, flags: i32, remove: i32);
//

/* helper.c */
// #if defined()
// int create_helper(event_fd: i32, err_fd: i32, uid_t uid, gid_t gid, long max_fd);
// void helper_write();
// void queue_script(action: i32, struct dhcp_lease *lease,
//       hostname: &mut String, now: &time::Instant);
//
// void queue_tftp(off_t file_len, filename: &mut String, union mysockaddr *peer);
//
// void queue_arp(action: i32, mac: &mut Vec<u8>, maclen: i32,
//          family: i32, union all_addr *addr);
// int helper_buf_empty();
//

/* tftp.c */
//
// void tftp_request(struct listener *listen, now: &time::Instant);
// void check_tftp_listeners(now: time::Instant);
// int do_tftp_script_run();
//

/* conntrack.c */
//  HAVE_CONNTRACK
// int get_incoming_mark(union mysockaddr *peer_addr, union all_addr *local_addr,
//           istcp: i32, markp: &mut u32);
//

/* dhcp6.c */
//
// void dhcp6_init();
// void dhcp6_packet(now: time::Instant);
// struct dhcp_context *address6_allocate(struct dhcp_context *context,  clid: &mut Vec<u8>, clid_len: i32, temp_addr: i32,
//                unsigned iaid: i32, serial: i32, struct dhcp_netid *netids, plain_range: i32, ans: &mut net::IpAddr);
// struct dhcp_context *address6_available(struct dhcp_context *context,
//           taddr: &mut net::IpAddr,
//           struct dhcp_netid *netids,
//           int plain_range);
// struct dhcp_context *address6_valid(struct dhcp_context *context,
//             taddr: &mut net::IpAddr,
//             struct dhcp_netid *netids,
//             int plain_range);
// struct dhcp_config *config_find_by_address6(struct dhcp_config *configs, net: &mut net::IpAddr,
//               prefix: i32, addr: &mut net::IpAddr);
// void make_duid(now: time::Instant);
// void dhcp_construct_contexts(now: time::Instant);
// void get_client_mac(client: &mut net::IpAddr, iface: i32, mac: &mut Vec<u8>,
//         maclenp: &mut u32, mactypep: &mut u32, now: &time::Instant);
//

/* rfc3315.c */
//
// u16 dhcp6_reply(struct dhcp_context *context, interface: i32, iface_name: &mut String,
//          fallback: &mut net::IpAddr, ll_addr: &mut net::IpAddr, ula_addr: &mut net::IpAddr,
//          sz: usize, client_addr: &mut net::IpAddr, now: &time::Instant);
// void relay_upstream6(struct dhcp_relay *relay, ssz: usize, peer_address: &mut net::IpAddr,
//          u32 scope_id, now: &time::Instant);

// u16 relay_reply6( struct sockaddr_in6 *peer, ssz: usize, arrival_interface: &mut String);
//

/* dhcp-common.c */
//
// void dhcp_common_init();
// srecv_dhcp_packet: usize(fd: i32, struct msghdr *msg);
// struct dhcp_netid *run_tag_if(struct dhcp_netid *tags);
// struct dhcp_netid *option_filter(struct dhcp_netid *tags, struct dhcp_netid *context_tags,
//          struct dhcp_opt *opts);
// int match_netid(struct dhcp_netid *check, struct dhcp_netid *pool, tagnotneeded: i32);
// char *strip_hostname(hostname: &mut String);
// void log_tags(struct dhcp_netid *netid, u32 xid);
// int match_bytes(struct dhcp_opt *o, p: &mut Vec<u8>, len: i32);
// void dhcp_update_configs(struct dhcp_config *configs);
// void display_opts();
// int lookup_dhcp_opt(prot: i32, name: &mut String);
// int lookup_dhcp_len(prot: i32, val: i32);
// struct dhcp_config *find_config(struct dhcp_config *configs,
//         struct dhcp_context *context,
//         clid: &mut Vec<u8>, clid_len: i32,
//         hwaddr: &mut Vec<u8>, hw_len: i32,
//         hw_type: i32, hostname: &mut String,
//         struct dhcp_netid *filter);
// int config_has_mac(struct dhcp_config *config, hwaddr: &mut Vec<u8>, len: i32, type: i32);
//
// char *whichdevice();
// void bindtodevice(device: &mut String, fd: i32);
//
// #  ifdef
// void display_opts6();
// #  endif
// void log_context(family: i32, struct dhcp_context *context);
// void log_relay(family: i32, struct dhcp_relay *relay);
//

/* outpacket.c */
//
// void end_opt6(int container);
// void reset_counter();
// int save_counter(int newval);
// expand: Vec<u8>(headroom: usize);
// int new_opt6(int opt);
// put_opt6: Vec<u8>(data: Vec<u8>, len: usize);
// void put_opt6_long(unsigned int val);
// void put_opt6_short(unsigned int val);
// void put_opt6_char(unsigned int val);
// void put_opt6_string(s: &mut String);
//

/* radv.c */
//
// void ra_init(now: time::Instant);
// void icmp6_packet(now: time::Instant);
// periodic_ra: time::Instant(now: time::Instant);
// void ra_start_unsolicited(now: time::Instant, struct dhcp_context *context);
//

/* slaac.c */
//
// void slaac_add_addrs(struct dhcp_lease *lease, now: &time::Instant, force: i32);
// periodic_slaac: time::Instant(now: time::Instant, struct dhcp_lease *leases);
// void slaac_ping_reply(sender: &mut net::IpAddr, packet: &mut Vec<u8>, interface: &mut String, struct dhcp_lease *leases);
//

/* loop.c */
//  HAVE_LOOP
// void loop_send_probes();
// int detect_loop(query: &mut String, type: i32);
//

/* inotify.c */
//  HAVE_INOTIFY
// void inotify_dnsmasq_init();
// int inotify_check(now: time::Instant);
// void set_dynamic_inotify(flag: i32, total_size: i32, struct crec **rhash, revhashsz: i32);
//

/* poll.c */
// void poll_reset();
// int poll_check(fd: i32, short event);
// void poll_listen(fd: i32, short event);
// int do_poll(int timeout);

/* rrfilter.c */
// rrfilter: usize(struct dns_header *header, plen: usize, mode: i32);
// u16 *rrfilter_desc(int type);
// int expand_workspace(unsigned char ***wkspc, int *szp, new: i32);

/* edns0.c */
// unsigned char *find_pseudoheader(struct dns_header *header, plen: usize,
//            size_t *len, unsigned char **p, int *is_sign, int *is_last);
// add_pseudoheader: usize(struct dns_header *header, plen: usize, limit: &mut Vec<u8>,
//       u16 udp_sz, optno: i32, opt: &mut Vec<u8>, optlen: usize, set_do: i32, replace: i32);
// add_do_bit: usize(struct dns_header *header, plen: usize, unsigned char *limit);
// add_edns0_config: usize(struct dns_header *header, plen: usize, limit: &mut Vec<u8>,
//       union mysockaddr *source, now: &time::Instant, int *check_subnet, int *cacheable);
// int check_source(struct dns_header *header, plen: usize, pseudoheader: &mut Vec<u8>, union mysockaddr *peer);

/* arp.c */
// int find_mac(union mysockaddr *addr, mac: &mut Vec<u8>, lazy: i32, now: &time::Instant);
// int do_arp_script_run();

/* dump.c */
//  HAVE_DUMPFILE
// void dump_init();
// void dump_packet(mask: i32, packet: Vec<u8>, len: usize, union mysockaddr *src, union mysockaddr *dst);
//

// struct passwd {
// 	char *pw_name;
// 	char *pw_passwd;
// 	uid_t pw_uid;
// 	gid_t pw_gid;
// 	pw_change: time::Instant;
// 	char *pw_class;
// 	char *pw_gecos;
// 	char *pw_dir;
// 	char *pw_shell;
// 	pw_expire: time::Instant;
// };
pub struct passwd {
    pub pw_name: String,
    pub pw_passwd: String,
    pub pw_uid: uid_t,
    pub pw_guid: gid_t,
    pub pw_change: time::Instant,
    pub pw_class: String,
    pub pw_gecos: String,
    pub pw_dir: String,
    pub pw_shell: String,
    pub pw_expire: time::Instant,
}

pub struct group {
    pub gr_name: String,
    pub gr_passwd: String,
    pub gr_gid: gid_t,
    pub gr_mem: Vec<String>,
}

pub struct cap_user_data_t {
    pub effective: u32,
    pub permitted: u32,
    pub inheritable: u32,
}

impl cap_user_data_t {
    pub fn new() -> cap_user_data_t {
        cap_user_data_t {
            effective: 0,
            permitted: 0,
            inheritable: 0,
        }
    }
}

pub struct cap_user_header_t {
    pub version: u32,
    pub pid: i32,
}

impl cap_user_header_t {
    pub fn new() -> cap_user_header_t {
        cap_user_header_t { version: 0, pid: 0 }
    }
}

pub struct serverfd {
    pub fd: i32,
    pub source_addr: net::IpAddr,
    pub interface: String,
    pub ifindex: u32,
    pub used: u32,
    pub preallocated: u32,
    // next
}

pub struct Server {
    //   union mysockaddr addr, source_addr;
    pub addr: net::IpAddr,
    pub source_addr: net::IpAddr,
    pub interface: String,
    pub sfd: serverfd,
    pub domain: String,
    pub flags: i32,
    pub tcpfd: i32,
    pub edns_pktsz: i32,
    pub pktsz_reduced: time::Instant,
    pub queries: u32,
    pub failed_queries: u32,
    pub uid: u32,
    // next
}

pub struct DhcpContext {
    pub lease_time: i32,
    pub addr_epoch: i32,
    pub netmask: net::IpAddr,
    pub broadcast: net::IpAddr,
    pub local: net::IpAddr,
    pub router: net::IpAddr,
    pub start: net::IpAddr,
    pub end: net::IpAddr,
    pub start6: net::IpAddr,
    pub end6: net::IpAddr,
    pub local6: net::IpAddr,
    pub prefix: u16,
    pub if_index: i32,
    pub valid: u32,
    pub preferred: u32,
    pub saved_valid: u32,
    pub ra_time: time::Instant,
    pub ra_short_period_start: time::Instant,
    pub address_lost_time: time::Instant,
    pub template_interface: String,
    // pub flags: i32,
    pub context_dhcp: bool,
    pub context_ra: bool,
    pub context_template: bool,
    pub netid: dhcp_netid,
    pub filter: dhcp_netid,
    // next
    // current
}

pub struct dns_header {
    pub id: u16,
    //   u8  hb3,hb4;
    pub hb3: u8,
    pub hb4: u8,
    //   u16 qdcount,ancount,nscount,arcount;
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

pub struct ArpRecord {
    // u16 hwlen, status;
    pub hwlen: u16,
    pub status: u16,
    // let mut family: i32;
    pub family: u16,
    // unsigned char hwaddr[DHCP_CHADDR_MAX];
    pub hwaddr: [u8; 16],
    // union all_addr addr;
    pub addr: net::IpAddr,
    // let mut next: arp_record;
}

pub struct udphdr {
    pub uh_sport: u16, /* source port */
    pub uh_dport: u16, /* destination port */
    pub uh_ulen: u16,  /* udp length */
    pub uh_sum: u16,   /* udp checksum */
}

pub struct cmsghdr {
    pub cmsg_len: libc::size_t,
    pub cmsg_level: libc::c_int,
    pub cmsg_type: libc::c_int,
}

//   union {
//     struct cmsghdr align; /* this ensures alignment */
//     char control6[CMSG_SPACE(sizeof(struct in6_pktinfo))];
//   } control_u;
pub struct control_u {
    pub align: cmsghdr,
    pub control6: [u8; 256],
}

pub struct Ip4Header {
    pub version_ihl: u8,
    pub dscp_ecn: u8,
    pub tot_len: u16,
    pub ident: u16,
    pub flags_frag_off: u16,
    pub ttl: u8,
    pub proto: u8,
    pub hdr_checksum: u16,
    pub src_ip_addr: u32,
    pub dst_ip_addr: u32,
}

pub struct Ip4HeaderOption {}

pub struct Ip4HeaderEx {
    pub ip_header: Ip4Header,
    pub has_options: bool,
    pub options: Vec<Ip4HeaderOption>,
}

/*
struct watch {
  DBusWatch *watch;
  let mut next: watch;
};
*/

pub struct DBusWatch {}

pub struct watch {
    pub watch: DBusWatch,
    // next
}

/*
struct rt_msghdr {
     u_short rtm_msglen;	    /* to skip over non-understood messages */
     u_char	 rtm_version;	    /* future binary compatibility */
     u_char	 rtm_type;	    /* message type */
     u_short rtm_index;	    /* index for associated ifp	*/
     int	 rtm_flags;	    /* flags, incl kern	& message, e.g.	DONE */
     int	 rtm_addrs;	    /* bitmask identifying sockaddrs in	msg */
     pid_t	 rtm_pid;	    /* identify	sender */
     int	 rtm_seq;	    /* for sender to identify action */
     int	 rtm_errno;	    /* why failed */
     int	 rtm_use;	    /* from rtentry */
     u_long	 rtm_inits;	    /* which metrics we	are initializing */
     struct	 rt_metrics rtm_rmx; /*	metrics	themselves */
     };

*/
pub struct rt_msghdr {
    pub rtm_msglen: libc::c_ushort,
    pub rtm_version: libc::c_uchar,
    pub rtm_type: libc::c_uchar,
    pub rtm_index: libc::c_ushort,
    pub rtm_flags: libc::c_int,
    pub rtm_addrs: libc::c_int,
    pub rtm_pid: pid_t,
    pub rtm_seq: libc::c_int,
    pub rtm_errno: libc::c_int,
    pub rtm_use: libc::c_int,
    pub rtm_inits: libc::c_ulong,
    pub rtm_rmx: rt_metrics,
}

/*
struct rt_metrics {
     u_long	rmx_locks;	    /* Kernel must leave these values alone */
     u_long	rmx_mtu;	    /* MTU for this path */
     u_long	rmx_hopcount;	    /* max hops	expected */
     u_long	rmx_expire;	    /* lifetime	for route, e.g.	redirect */
     u_long	rmx_recvpipe;	    /* inbound delay-bandwidth product */
     u_long	rmx_sendpipe;	    /* outbound	delay-bandwidth	product	*/
     u_long	rmx_ssthresh;	    /* outbound	gateway	buffer limit */
     u_long	rmx_rtt;	    /* estimated round trip time */
     u_long	rmx_rttvar;	    /* estimated rtt variance */
     u_long	rmx_pksent;	    /* packets sent using this route */
     };
*/
pub struct rt_metrics {
    pub rmx_locks: libc::c_ulong,
    pub rmx_mtu: libc::c_ulong,
    pub rmx_hopcount: libc::c_ulong,
    pub rmx_expire: libc::c_ulong,
    pub rmx_recvpipe: libc::c_ulong,
    pub rmx_sendpipe: libc::c_ulong,
    pub rmx_ssthresh: libc::c_ulong,
    pub rmx_rtt: libc::c_ulong,
    pub rmx_rttVar: libc::c_ulong,
    pub rmx_pksent: libc::c_ulong,
}
