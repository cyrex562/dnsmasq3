use std::net::{self, IpAddr};

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


type uid_t = i32;
type gid_t = i32;

//#define COPYRIGHT "Copyright (c) 2000-2021 Simon Kelley"

/* We do defines that influence behavior of stdio.h, so complain
   if included too early. */
// #ifdef _STDIO_H
// #  error "Header file stdio.h included too early!"
// #endif 

// #ifndef NO_LARGEFILE
/* Ensure we can use files >2GB (log files may grow this big) */
pub const _LARGEFILE_SOURCE: u32 = 1;
pub const _FILE_OFFSET_BITS: u32 = 64;
// #endif

/* Get linux C library versions and define _GNU_SOURCE for kFreeBSD. */
// #if defined(__linux__) || defined(__GLIBC__)
// #  ifndef __ANDROID__
// #      define _GNU_SOURCE
// #  endif
// #  include <features.h> 
// #endif

/* Need these defined early */
// #if defined(__sun) || defined(__sun__)
// pub const _XPG4_: u32 = 2;
// #  define __EXTENSIONS__
// #endif

// #if (defined(__GNUC__) && __GNUC__ >= 3) || defined(__clang__)
// #define ATTRIBUTE_NORETURN __attribute__ ((noreturn))
// #else
// #define ATTRIBUTE_NORETURN
// #endif

/* get these before config.h  for IPv6 stuff... */
// #include <sys/types.h> 
// #include <sys/socket.h>

// #ifdef __APPLE__
// /* Define before netinet/in.h to select API. OSX Lion onwards. */
// pub const __APPLE_USE_RFC_354: u32 = 2;
// #endif
// #include <netinet/in.h>

/* Also needed before config.h. */
// #include <getopt.h>

// #include "config.h"
// #include "ip6addr.h"
// #include "metrics.h"

// typedef unsigned char u8;
// typedef unsigned short u16;
// typedef unsigned int u32;
// typedef unsigned long long u64;

// #define countof(x)      (long)(sizeof(x) / sizeof(x[0]))
// #define MIN(a,b)        ((a) < (b) ? (a) : (b))

// #include "dns-protocol.h"
// #include "dhcp-protocol.h"
// #ifdef HAVE_DHCP6
// #include "dhcp6-protocol.h"
// #include "radv-protocol.h"
// #endif

// #define gettext_noop(S) (S)
// #ifndef LOCALEDIR
// #  define _(S) (S)
// #else
// #  include <libintl.h>
// #  include <locale.h>   
// #  define _(S) gettext(S)
// #endif

// #include <arpa/inet.h>
// #include <sys/stat.h>
// #include <sys/ioctl.h>
// #if defined(HAVE_SOLARIS_NETWORK)
// #  include <sys/sockio.h>
// #endif
// #if defined(HAVE_POLL_H)
// #  include <poll.h>
// #else
// #  include <sys/poll.h>
// #endif
// #include <sys/wait.h>
// #include <sys/time.h>
// #include <sys/un.h>
// #include <limits.h>
// #include <net/if.h>
// #if defined(HAVE_SOLARIS_NETWORK) && !defined(ifr_mtu)
// /* Some solaris net/if./h omit this. */
// #  define ifr_mtu  ifr_ifru.ifru_metric
// #endif
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
// #else
// #  include <net/ethernet.h>
// #endif
// #include <net/if_arp.h>
// #include <netinet/in_systm.h>
// #include <netinet/ip.h>
// #include <netinet/ip6.h>
// #include <netinet/ip_icmp.h>
// #include <netinet/tcp.h>
// #include <sys/uio.h>
// #include <syslog.h>
// #include <dirent.h>
// #ifndef HAVE_LINUX_NETWORK
// #  include <net/if_dl.h>
// #endif

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
// #endif

/* Backwards compat with 2.83 */
// #if defined(HAVE_NETTLEHASH)
// #  define HAVE_CRYPTOHASH
// #endif
// #if defined(HAVE_DNSSEC) || defined(HAVE_CRYPTOHASH)
// #  include <nettle/nettle-meta.h>
// #endif

/* daemon is function in the C library.... */
// #define daemon dnsmasq_daemon

// #define ADDRSTRLEN INET6_ADDRSTRLEN

/* Async event queue */
// struct event_desc {
//   int event, data, msg_sz;
// };
pub struct event_desc {
  pub event: i32,
  pub data: i32,
  pub msg_sz: i32
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
// #define option_var(x) (daemon->options[(x) / OPTION_BITS])
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
//   struct in_addr addr4;
//   struct in6_addr addr6;
//   struct {
//     union {
//       struct crec *cache;
//       char *name;
//     } target;
//     unsigned int uid;
//     int is_name_ptr;  /* disciminates target union */
//   } cname;
//   struct {
//     struct blockdata *keydata;
//     unsigned short keylen, flags, keytag;
//     unsigned char algo;
//   } key; 
//   struct {
//     struct blockdata *keydata;
//     unsigned short keylen, keytag;
//     unsigned char algo;
//     unsigned char digest; 
//   } ds;
//   struct {
//     struct blockdata *target;
//     unsigned short targetlen, srvport, priority, weight;
//   } srv;
//   /* for log_query */
//   struct {
//     unsigned short keytag, algo, digest, rcode;
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
  pub rcode: u16
}





// struct bogus_addr {
//   struct in_addr addr;
//   struct bogus_addr *next;
// };
pub struct bogus_addr {
    pub addr: net::IpAddr,
    // next
}

/* dns doctor param */
// struct doctor {
//   struct in_addr in, end, out, mask;
//   struct doctor *next;
// };
pub struct doctor {
    pub _in: net::IpAddr,
    pub end: net::IpAddr,
    pub out: net::IpAddr,
    pub mask: net::IpAddr,
    // next
}

// struct mx_srv_record {
//   char *name, *target;
//   int issrv, srvport, priority, weight;
//   unsigned int offset;
//   struct mx_srv_record *next;
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
//   char *name, *replace, *regexp, *services, *flags;
//   unsigned int order, pref;
//   struct naptr *next;
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
// #endif

// struct txt_record {
//   char *name;
//   unsigned char *txt;
//   unsigned short class, len;
//   int stat;
//   struct txt_record *next;
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
//   char *name, *ptr;
//   struct ptr_record *next;
// };
pub struct ptr_record {
    pub name: String,
    pub ptr: String,
    // next
}


// struct cname {
//   int ttl, flag;
//   char *alias, *target;
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
//   char *name, *digest;
//   int digestlen, class, algo, keytag, digest_type;
//   struct ds_config *next;
// };
#[derive(Copy,Clone,Debug,Default)]
pub struct ds_config {
    pub name: String,
    pub digest: String,
    pub digestlen: i32,
    pub class: i32,
    pub algo: i32,
    pub keytag: i32,
    pub digest_type: i32,
    // next
}

impl ds_config {
  pub fn new() -> Self {
    ds_config {
      name: String::new(),
      digest: String::new(),
      digestlen: 0,
      class: 0,
      algo: 0,
      keytag: 0,
      digest_type: 0
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
//   int flags, prefixlen;
//   time_t decline_time;
//   struct addrlist *next;
// };
pub struct addrlist {
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
//     int flags;
//     struct auth_name_list *next;
//   } *interface_names;
//   struct addrlist *subnet;
//   struct addrlist *exclude;
//   struct auth_zone *next;
// };
pub struct auth_name_list {
    pub name: String,
    pub flags: int
    // next
}

pub struct auth_zone {
    pub domain: String,
    pub interface_names: auth_name_list,
    pub subnet: addr_list,
    pub exclude: addr_list,
    // next
}

pub const HR_6: u32 = 1;
pub const HR_4: u32 = 2;

// struct host_record {
//   int ttl, flags;
//   struct name_list {
//     char *name;
//     struct name_list *next;
//   } *names;
//   struct in_addr addr;
//   struct in6_addr addr6;
//   struct host_record *next;
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
//   int family; /* AF_INET, AF_INET6 or zero for both */
//   struct addrlist *addr;
//   struct interface_name *next;
// };
pub struct interface_name {
    pub name: String,
    pub intr: String,
    pub family: i32,
    pub addr: addr_list,
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
//   struct blockdata *next;
//   unsigned char key[KEYBLOCK_LEN];
// };
pub struct blockdata {
    // next
    key: Vec<u8>
}
// struct crec { 
//   struct crec *next, *prev, *hash_next;
//   union all_addr addr;
//   time_t ttd; /* time to die */
//   /* used as class if DNSKEY/DS, index to source for F_HOSTS */
//   unsigned int uid; 
//   unsigned int flags;
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
// #define SIZEOF_POINTER_CREC (sizeof(struct crec) + sizeof(char *) - SMALLDNAME)

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
    pub in6: net:: IpAddr,
}

/* bits in flag param to IPv6 callbacks from iface_enumerate() */
pub const IFACE_TENTATIVE: u32 = 1;
pub const IFACE_DEPRECATED: u32 = 2;
pub const IFACE_PERMANENT: u32 = 4;


pub const SERV_FROM_RESOLV: u32 = 1;  /* 1 for servers from resolv, 0 for command line. */
pub const SERV_NO_ADDR: u32 = 2;  /* no server, this domain is local only */
pub const SERV_LITERAL_ADDRESS: u32 = 4;  /* addr is the answer, not the server */ 
pub const SERV_HAS_DOMAIN: u32 = 8;  /* server for one domain only */
pub const SERV_HAS_SOURCE: u32 = 16;  /* source address defined */
pub const SERV_FOR_NODOTS: u32 = 32;  /* server for names with no domain part only */
pub const SERV_WARNED_RECURSIVE: u32 = 64;  /* avoid warning spam */
pub const SERV_FROM_DBUS: u32 = 128;  /* 1 if source is DBus */
pub const SERV_MARK: u32 = 256;  /* for mark-and-delete */
// #define SERV_TYPE    (SERV_HAS_DOMAIN | SERV_FOR_NODOTS)
pub const SERV_COUNTED: u32 = 512;  /* workspace for log code */
pub const SERV_USE_RESOLV: u32 = 1024;  /* forward this domain in the normal way */
pub const SERV_NO_REBIND: u32 = 2048;  /* inhibit dns-rebind protection */
pub const SERV_FROM_FILE: u32 = 4096;  /* read from --servers-file */
pub const SERV_LOOP: u32 = 8192;  /* server causes forwarding loop */
pub const SERV_DO_DNSSEC: u32 = 16384;  /* Validate DNSSEC when using this server */
pub const SERV_GOT_TCP: u32 = 32768;  /* Got some data from the TCP connection */



// struct randfd {
//   int fd;
//   unsigned short refcount, family;
// };
pub struct randfd {
    pub fd: i32,
    pub refcount: u16,
    pub family: u16,
}  

// struct ipsets {
//   char **sets;
//   char *domain;
//   struct ipsets *next;
// };
pub struct ipsets {
    pub sets: Vec<String>,
    pub domain: Vec<String>,
    // next
}

// struct irec {
//   union mysockaddr addr;
//   struct in_addr netmask; /* only valid for IPv4 */
//   int tftp_ok, dhcp_ok, mtu, done, warned, dad, dns_auth, index, multicast_done, found, label;
//   char *name; 
//   struct irec *next;
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
//   int fd, tcpfd, tftpfd, used;
//   union mysockaddr addr;
//   struct irec *iface; /* only sometimes valid for non-wildcard */
//   struct listener *next;
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
//   int used;
//   struct iname *next;
// };
pub struct InterfaceParams {
  pub name: String,
  pub addr: net::IpAddr,
  pub used: i32,
}

/* subnet parameters from command line */
// struct mysubnet {
//   union mysockaddr addr;
//   int addr_used;
//   int mask;
// };
pub struct mysubnet {
    pub addr: net::IpAddr,
    pub addr_used: i32,
    pub mask: i32
}


/* resolv-file parms from command-line */
// struct resolvc {
//   struct resolvc *next;
//   int is_default, logged;
//   time_t mtime;
//   char *name;
// #ifdef HAVE_INOTIFY
//   int wd; /* inotify watch descriptor */
//   char *file; /* pointer to file part if path */
// #endif
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
//   struct hostsfile *next;
//   int flags;
//   char *fname;
// #ifdef HAVE_INOTIFY
//   int wd; /* inotify watch descriptor */
// #endif
//   unsigned int index; /* matches to cache entries for logging */
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
//     unsigned int iface, log_id;
//     int fd;
//     unsigned short orig_id;
//     struct frec_src *next;
//   } frec_src;
//   struct server *sentto; /* NULL means free */
//   struct randfd *rfd4;
//   struct randfd *rfd6;
//   unsigned short new_id;
//   int forwardall, flags;
//   time_t time;
//   unsigned char *hash[HASH_SIZE];
// #ifdef HAVE_DNSSEC 
//   int class, work_counter;
//   struct blockdata *stash; /* Saved reply, whilst we validate */
//   size_t stash_len;
//   struct frec *dependent; /* Query awaiting internally-generated DNSKEY or DS query */
//   struct frec *blocking_query; /* Query which is blocking us. */
// #endif
//   struct frec *next;
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
    pub sentto: server,
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
pub const OT_DEC: u32 = 0x0400 ;
pub const OT_TIME: u32 = 0x0200;

/* actions in the daemon->helper RPC */
pub const ACTION_DEL: u32 = 1;
pub const ACTION_OLD_HOSTNAME: u32 = 2;
pub const ACTION_OLD: u32 = 3;
pub const ACTION_ADD: u32 = 4;
pub const ACTION_TFTP: u32 = 5;
pub const ACTION_ARP: u32 = 6;
pub const ACTION_ARP_DEL: u32 = 7;

pub const LEASE_NEW: u32 = 1;  /* newly created */
pub const LEASE_CHANGED: u32 = 2;  /* modified */
pub const LEASE_AUX_CHANGED: u32 = 4;  /* CLID or expiry changed */
pub const LEASE_AUTH_NAME: u32 = 8;  /* hostname came from config, not from client */
pub const LEASE_USED: u32 = 16;  /* used this DHCPv6 transaction */
pub const LEASE_NA: u32 = 32;  /* IPv6 no-temporary lease */
pub const LEASE_TA: u32 = 64;  /* IPv6 temporary lease */
pub const LEASE_HAVE_HWADDR: u32 = 128;  /* Have set hwaddress */
pub const LEASE_EXP_CHANGED: u32 = 256;  /* Lease expiry time changed */

// struct dhcp_lease {
//   int clid_len;          /* length of client identifier */
//   unsigned char *clid;   /* clientid */
//   char *hostname, *fqdn; /* name from client-hostname option or config */
//   char *old_hostname;    /* hostname before it moved to another lease */
//   int flags;
//   time_t expires;        /* lease expiry */
// #ifdef HAVE_BROKEN_RTC
//   unsigned int length;
// #endif
//   int hwaddr_len, hwaddr_type;
//   unsigned char hwaddr[DHCP_CHADDR_MAX]; 
//   struct in_addr addr, override, giaddr;
//   unsigned char *extradata;
//   unsigned int extradata_len, extradata_size;
//   int last_interface;
//   int new_interface;     /* save possible originated interface */
//   int new_prefixlen;     /* and its prefix length */
// #ifdef HAVE_DHCP6
//   struct in6_addr addr6;
//   unsigned int iaid;
//   struct slaac_address {
//     struct in6_addr addr;
//     time_t ping_time;
//     int backoff; /* zero -> confirmed */
//     struct slaac_address *next;
//   } *slaac_address;
//   int vendorclass_count;
// #endif
//   struct dhcp_lease *next;
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
    pub hwaddr: [u8;16],
    pub addr: net::IpAddr,
    pub override: net::IpAddr,
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
//   struct dhcp_netid *next;
// };
pub struct dhcp_netid {
    pub net: String,
    // next
}

// struct dhcp_netid_list {
//   struct dhcp_netid *list;
//   struct dhcp_netid_list *next;
// };
pub struct dhcp_netid_list {
    pub list: Vec<dhcp_netid>,
    // next
}


// struct tag_if {
//   struct dhcp_netid_list *set;
//   struct dhcp_netid *tag;
//   struct tag_if *next;
// };
pub struct tag_if {
    pub set: dhcp_netid_list,
    pub tag: dhcp_netid,
    // next
}

// struct delay_config {
//   int delay;
//   struct dhcp_netid *netid;
//   struct delay_config *next;
// };
pub struct delay_config {
    pub delay: i32,
    pub netid: dhcp_netid,
    // next
}

// struct hwaddr_config {
//   int hwaddr_len, hwaddr_type;
//   unsigned char hwaddr[DHCP_CHADDR_MAX];
//   unsigned int wildcard_mask;
//   struct hwaddr_config *next;
// };
pub struct hwaddr_config {
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub hwaddr: [u8;16],
    pub wildcard_mask: i32,
    // next
}


// struct dhcp_config {
//   unsigned int flags;
//   int clid_len;          /* length of client identifier */
//   unsigned char *clid;   /* clientid */
//   char *hostname, *domain;
//   struct dhcp_netid_list *netid;
//   struct dhcp_netid *filter;
// #ifdef HAVE_DHCP6
//   struct addrlist *addr6;
// #endif
//   struct in_addr addr;
//   time_t decline_time;
//   unsigned int lease_time;
//   struct hwaddr_config *hwaddr;
//   struct dhcp_config *next;
// };
pub struct dhcp_config {
    pub flags: u32,
    pub clid_len: i32,
    pub clid: Vec<u8>,
    pub hostname: String,
    pub domain: String,
    pub netid: dhcp_netid_list,
    pub filter: dhcp_netid,
    pub addr6: addr_list,
    pub addr: net::IpAddr,
    pub decline_time: time::Instant,
    pub lease_time: u32,
    pub hwaddr: hwaddr_config,
    // next
}

// #define have_config(config, mask) ((config) && ((config)->flags & (mask))) 

pub const CONFIG_DISABLE: u32 = 1;
pub const CONFIG_CLID: u32 = 2;
pub const CONFIG_TIME: u32 = 8;
pub const CONFIG_NAME: u32 = 16;
pub const CONFIG_ADDR: u32 = 32;
pub const CONFIG_NOCLID: u32 = 128;
pub const CONFIG_FROM_ETHERS: u32 = 256;    /* entry created by /etc/ethers */
pub const CONFIG_ADDR_HOSTS: u32 = 512;    /* address added by from /etc/hosts */
pub const CONFIG_DECLINED: u32 = 1024;    /* address declined by client */
pub const CONFIG_BANK: u32 = 2048;    /* from dhcp hosts file */
pub const CONFIG_ADDR6: u32 = 4096;
pub const CONFIG_ADDR6_HOSTS: u32 = 16384;    /* address added by from /etc/hosts */

// struct dhcp_opt {
//   int opt, len, flags;
//   union {
//     int encap;
//     unsigned int wildcard_mask;
//     unsigned char *vendor_class;
//   } u;
//   unsigned char *val;
//   struct dhcp_netid *netid;
//   struct dhcp_opt *next;
// };
pub struct dhcp_opt {
    pub opt: i32,
    pub len: i32,
    pub flags: i32,
    pub encap: i32,
    pub wildcard_mask: u32,
    pub vendor_class: Vec<u8>
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
//   char *file, *sname, *tftp_sname;
//   struct in_addr next_server;
//   struct dhcp_netid *netid;
//   struct dhcp_boot *next;
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
//   int wildcard;
//   struct dhcp_netid *netid;
//   struct dhcp_match_name *next;
// };
pub struct dhcp_match_name {
    pub name: String,
    pub wildcard: i32,
    pub netid: dhcp_netid,
    // next
}

// struct pxe_service {
//   unsigned short CSA, type; 
//   char *menu, *basename, *sname;
//   struct in_addr server;
//   struct dhcp_netid *netid;
//   struct pxe_service *next;
// };
pub struct pxe_service {
    pub CSA: u16,
    pub _type: u16,
    pub menu: String,
    pub basename: String,
    pub sname: String,
}

// #define DHCP_PXE_DEF_VENDOR      "PXEClient"
pub const DHCP_PXE_DEF_VENDOR: str = "PXEClient";

pub const MATCH_VENDOR: u32 = 1;
pub const MATCH_USER: u32 = 2;
pub const MATCH_CIRCUIT: u32 = 3;
pub const MATCH_REMOTE: u32 = 4;
pub const MATCH_SUBSCRIBER: u32 = 5;

/* vendorclass, userclass, remote-id or circuit-id */
// struct dhcp_vendor {
//   int len, match_type;
//   unsigned int enterprise;
//   char *data;
//   struct dhcp_netid netid;
//   struct dhcp_vendor *next;
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
//   struct dhcp_pxe_vendor *next;
// };
pub struct dhcp_pxe_vendor {
    pub data: String,
    // next
}

// struct dhcp_mac {
//   unsigned int mask;
//   int hwaddr_len, hwaddr_type;
//   unsigned char hwaddr[DHCP_CHADDR_MAX];
//   struct dhcp_netid netid;
//   struct dhcp_mac *next;
// };
pub struct dhcp_mac {
  pub mask: u32,
  pub hwaddr_len: u32,
  pub hwaddr_type: u32,
  pub hwaddr: [u8;16],
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
//   char *domain, *prefix;
//   struct in_addr start, end;
//   struct in6_addr start6, end6;
//   int is6, indexed;
//   struct cond_domain *next;
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
//   int interval, lifetime, prio, mtu;
//   struct ra_interface *next;
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
//   int if_index;
//   struct in_addr match_addr, shared_addr;
// #ifdef HAVE_DHCP6
//   /* shared_addr == 0 for IP6 entries. */
//   struct in6_addr match_addr6, shared_addr6;
// #endif
//   struct shared_network *next;
// };
pub struct shared_network {
  pub if_index: i32,
  pub match_addr: net::IpAddr,
  pub shared_addr: net::IpAddr,
  pub match_addr6: net::Ipv6Addr,
  pub shared_addr6: net::Ipv6Addr,
  // next
}


// #define CONTEXT_STATIC         (1u<<0)
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
//   struct in_addr addr;
//   time_t time;
//   unsigned int hash;
//   struct ping_result *next;
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
//   int refcount, fd;
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
//   int sockfd;
//   time_t timeout;
//   int backoff;
//   unsigned int block, blocksize, expansion;
//   off_t offset;
//   union mysockaddr peer;
//   union all_addr source;
//   int if_index;
//   char opt_blocksize, opt_transize, netascii, carrylf;
//   struct tftp_file *file;
//   struct tftp_transfer *next;
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
//   struct in_addr addr;
//   struct addr_list *next;
// };
pub struct addr_list {
    pub addr: net::IpAddr,
    // next
}


// struct tftp_prefix {
//   char *interface;
//   char *prefix;
//   int missing;
//   struct tftp_prefix *next;
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
//   int iface_index; /* working - interface in which requests arrived, for return */
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

#[derive(Clone, Copy, Debug, Default)]
pub struct DnsmasqDaemon {
  /* datastuctures representing the command-line and 
     config file arguments. All set (including defaults)
     in option.c */

//   unsigned int options[OPTION_SIZE];
  pub options: [u32;OPTION_SIZE],
    // struct resolvc default_resolv, *resolv_files;
    pub default_resolv: resolvc,
    pub resolv_files: resolvc,
  // time_t last_resolv;
  pub last_resolv: time::Instant,
  // char *servers_file;
  pub servers_file: String,
  // struct mx_srv_record *mxnames;
  pub mxnames: mx_srv_record,
  // struct naptr *naptr;
  pub naptr: naptr,
  // struct txt_record *txt, *rr;
  pub txt: txt_record,
  pub rr: txt_record,
  // struct ptr_record *ptr;
  pub ptr: ptr_record,
  // struct host_record *host_records, *host_records_tail;
  pub host_records: host_record,
  pub host_records_tail: host_record,
  // struct cname *cnames;
  pub cnames: cname,
  // struct auth_zone *auth_zones;
  pub auth_zones: auth_zone,
  // struct interface_name *int_names;
  pub int_names: interface_name,
  // char *mxtarget;
  pub mxtarget: String,
  // struct mysubnet *add_subnet4;
  pub add_subnet4: mysubnet,
  //struct mysubnet *add_subnet6;
  pub add_subnet6: mysubnet,
  // char *lease_file;
  pub lease_file: String,
  // char *username, *groupname, *scriptuser;
  pub username: String,
  pub groupname: String,
  pub scriptuser: String,
  // char *luascript;
  pub luascript: String,
  // char *authserver, *hostmaster;
  pub authserver: String,
  pub hostmaster: String,
  // struct iname *authinterface;
  pub authinterface: InterfaceParams,
  //struct name_list *secondary_forward_server;
  // int group_set, osport;
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
  pub if_names: Vector<InterfaceParams>,
  pub if_addrs: InterfaceParams,
  pub if_except: InterfaceParams,
  pub dhcp_except: InterfaceParams,
  pub auth_peers: InterfaceParams,
  // struct bogus_addr *bogus_addr, *ignore_addr;
  pub bogus_addr: bogus_addr,
  pub ignore_addr: bogus_addr,
  // struct server *servers;
  pub servers: server,
  // struct ipsets *ipsets;
  pub ipsets: ipsets,
  // int log_fac; /* log facility */
  pub log_fac: i32,
  // char *log_file; /* optional log file */
  pub log_file: String,
  // int max_logs;  /* queue limit */
  pub max_logs: i32,
  // int cachesize, ftabsize;
  pub cachesize: i32,
  pub ftabsize: i32,
  // int port, query_port, min_port, max_port;
  pub port: i32,
  pub query_port: i32,
  pub min_port: i32,
  pub max_port: i32,
  // unsigned long local_ttl, neg_ttl, max_ttl, min_cache_ttl, max_cache_ttl, auth_ttl, dhcp_ttl, use_dhcp_ttl;
  pub local_ttl: u32,
  pub neg_ttl: u32,
  pub max_ttl: u32,
  pub min_cache_ttl: u32,
  pub max_cache_ttl: u32,
  pub auth_ttl: u32,
  // char *dns_client_id;
  pub dns_client_id: String,
  // struct hostsfile *addn_hosts;
  pub addn_hosts: hostsfile,
  // struct dhcp_context *dhcp, *dhcp6;
  pub dhcp: DhcpContext,
  pub dhcp6: Vec<DhcpContext>,
  // struct ra_interface *ra_interfaces;
  pub ra_interfaces: ra_interface,
  // struct dhcp_config *dhcp_conf;
  pub dhcp_conf: dhcp_config,
  // struct dhcp_opt *dhcp_opts, *dhcp_match, *dhcp_opts6, *dhcp_match6;
  pub dhcp_opts: dhcp_opt,
  pub dhcp_match: dhcp_opt,
  pub dhcp_opts6: dhcp_opt,
  pub dhcp_match6: dhcp_opt,
  // struct dhcp_match_name *dhcp_name_match;
  pub dhcp_name_match: dhcp_match_name,
  // struct dhcp_pxe_vendor *dhcp_pxe_vendors;
  pub dhcp_pxe_vendors: dhcp_pxe_vendor,
  // struct dhcp_vendor *dhcp_vendors;
  pub dhcp_vendors: dhcp_vendor,
  // struct dhcp_mac *dhcp_macs;
  pub dhcp_macs: dhcp_mac,
  // struct dhcp_boot *boot_config;
  pub boot_config: dhcp_boot,
  // struct pxe_service *pxe_services;
  pub pxe_services: pxe_service,
  // struct tag_if *tag_if; 
  pub tag_if: tag_if,
  // struct addr_list *override_relays;
  pub override_relays: addr_list,
  // struct dhcp_relay *relay4, *relay6;
  pub relay4: dhcp_relay,
  pub relay6: dhcp_relay,
  // struct delay_config *delay_conf;
  pub delay_conf: delay_config,
  // int override;
  pub _override: i32,
  // int enable_pxe;
  pub enable_pxe: i32,
  // int doing_ra, doing_dhcp6;
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
  // int dhcp_max, tftp_max, tftp_mtu;
  pub dhcp_max: i32,
  pub tftp_max: i32,
  pub tftp_mtu: i32,
  // int dhcp_server_port, dhcp_client_port;
  pub dhcp_server_port: i32,
  pub dhcp_client_port: i32,
  // int start_tftp_port, end_tftp_port; 
  pub start_tftp_port: i32,
  pub end_tftp_port: i32,
  // unsigned int min_leasetime;
  pub min_leasetime: u32,
  // struct doctor *doctors;
  pub doctors: doctor,
  // unsigned short edns_pktsz;
  pub edns_pktsz: u16,
  // char *tftp_prefix; 
  pub tftp_prefix: String,
  // struct tftp_prefix *if_prefix; /* per-interface TFTP prefixes */
  pub if_prefix: tftp_prefix,
  // unsigned int duid_enterprise, duid_config_len;
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
  // int dump_mask;
  pub dump_mask: i32,
  // unsigned long soa_sn, soa_refresh, soa_retry, soa_expiry;
  pub soa_sn: libc::c_ulong,
  pub soa_refresh: libc::c_ulong,
  pub soa_retry: libc::c_ulong,
  pub soa_expiry: libc::c_ulong,
  // u32 metrics[__METRIC_MAX];
  pub metrics: [u32;__METRIC_MAX],
// #ifdef HAVE_DNSSEC
  // struct ds_config *ds;
  pub ds: ds_config,
  // char *timestamp_file;
  pub timestamp_file: String,
// #endif
  /* globally used stuff for DNS */
  // char *packet; /* packet buffer */
  pub packet: String,
  // int packet_buff_sz; /* size of above */
  pub packet_buff_sz: i32,
  // char *namebuff; /* MAXDNAME size buffer */
  pub namebuff: String,
// #ifdef HAVE_DNSSEC
  // char *keyname; /* MAXDNAME size buffer */
  pub keyname: String,
  // char *workspacename; /* ditto */
  pub workspacename: String,
  // unsigned long *rr_status; /* ceiling in TTL from DNSSEC or zero for insecure */
  pub rr_status: Vec<libc::c_ulong>,
  // int rr_status_sz;
  pub rr_status_sz: i32,
  // int dnssec_no_time_check;
  pub dnssec_no_time_check: i32,
  // int back_to_the_future;
  pub back_to_the_future: i32,
  // #endif
  // struct frec *frec_list;
  pub frec_list: frec,
  // struct frec_src *free_frec_src;
  pub free_frec_src: frec_src,
  // int frec_src_count;
  pub frec_src_count: i32,
  // struct serverfd *sfds;
  pub sfds: serverfd,
  // struct irec *interfaces;
  pub interfaces: irec,
  // struct listener *listeners;
  pub listeners: listener,
  // struct server *last_server;
  pub last_server: server,
  // time_t forwardtime;
  pub forwardtime: time::Instant,
  // int forwardcount;
  pub forwardcount: i32,
  // struct server *srv_save; /* Used for resend on DoD */
  pub srv_save: server,
  // size_t packet_len;       /*      "        "        */
  pub packet_len: usize,
  // struct randfd *rfd_save; /*      "        "        */
  pub rfd_save: randfd,
  // pid_t tcp_pids[MAX_PROCS];
  pub tcp_pids: [pid_t; MAX_PROCS],
  pub tcp_pipes: [i32;MAX_PROCS],
  pub pipe_to_parent: u32,
  // struct randfd randomsocks[RANDOM_SOCKS];
  pub randomsocks: [randfd;RANDOM_SOCKS],
  // int v6pktinfo; 
  pub v6pktinfo: i32,
  // struct addrlist *interface_addrs; /* list of all addresses/prefix lengths associated 
  // with all local interfaces */
  pub interface_addrs: addrlist,
  // int log_id, log_display_id; /* ids of transactions for logging */
  pub log_id: i32,
  pub log_display_id: i32,
  // union mysockaddr *log_source_addr;
  pub log_source_addr: net::IpAddr,
  /* DHCP state */
  // int dhcpfd, helperfd, pxefd;
  pub dhcpfd: i32,
  pub helperfd: i32,
  pub pxefd: i32, 
// #ifdef HAVE_INOTIFY
  // int inotifyfd;
  pub inotifyfd: i32,
// #endif
// #if defined(HAVE_LINUX_NETWORK)
  // int netlinkfd, kernel_version;
  pub netlinkfd: i32,
  pub kernel_version: i32,
// #elif defined(HAVE_BSD_NETWORK)
  // int dhcp_raw_fd, dhcp_icmp_fd, routefd;
  pub dhcp_raw_fd: i32,
  pub dhcp_icmp_fd: i32,
  pub routefd: i32,
// #endif
  // struct iovec dhcp_packet;
  pub dhcp_packet: Vec<u8>,
  // char *dhcp_buff, *dhcp_buff2, *dhcp_buff3;
  pub dhcp_buff: String,
  pub dhcp_buff2: String,
  pub dhcp_buff3: String,
  // struct ping_result *ping_results;
  pub ping_results: ping_result,
  // FILE *lease_stream;
  pub lease_stream: fs::File,
  // struct dhcp_bridge *bridges;
  pub bridges: dhcp_bridge,
  // struct shared_network *shared_networks;
  pub shared_networks: shared_network,
// #ifdef HAVE_DHCP6
  // int duid_len;
  pub duid_len: i32,
  //  unsigned char *duid;
  pub duid: Vec<u8>,
  // struct iovec outpacket;
  pub outpacket: Vec<u8>,
  // int dhcp6fd, icmp6fd;
  pub dhcp6fd: i32,
  pub icmp6fd: i32,
// #endif
  /* DBus stuff */
  /* void * here to avoid depending on dbus headers outside dbus.c */
  // void *dbus;
  pub dbus: *mut libc::c_void,
// #ifdef HAVE_DBUS
  // struct watch *watches;
  pub watches: watch,
// #endif
  /* UBus stuff */
// #ifdef HAVE_UBUS
  /* void * here to avoid depending on ubus headers outside ubus.c */
  // void *ubus;
  pub ubus: *mut libc::c_void,
// #endif

  /* TFTP stuff */
  // struct tftp_transfer *tftp_trans, *tftp_done_trans;
  pub tftp_trans: tftp_transfer,
  pub tftp_don_trans: tftp_transfer,
  /* utility string buffer, hold max sized IP address as string */
  // char *addrbuff;
  pub addrbuff: String,
  // char *addrbuff2; /* only allocated when OPT_EXTRALOG */
  pub addrbuff2: String,
// #ifdef HAVE_DUMPFILE
  /* file for packet dumps. */
  // int dumpfd;
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
  pub opt_last: bool
// #endif
}


/* cache.c */
// void cache_init(void);
// void next_uid(struct crec *crecp);
// void log_query(unsigned int flags, char *name, union all_addr *addr, char *arg); 
// char *record_source(unsigned int index);
// char *querystr(char *desc, unsigned short type);
// int cache_find_non_terminal(char *name, time_t now);
// struct crec *cache_find_by_addr(struct crec *crecp,
//         union all_addr *addr, time_t now, 
//         unsigned int prot);
// struct crec *cache_find_by_name(struct crec *crecp, 
//         char *name, time_t now, unsigned int prot);
// void cache_end_insert(void);
// void cache_start_insert(void);
// int cache_recv_insert(time_t now, int fd);
// struct crec *cache_insert(char *name, union all_addr *addr, unsigned short class, 
//         time_t now, unsigned long ttl, unsigned int flags);
// void cache_reload(void);
// void cache_add_dhcp_entry(char *host_name, int prot, union all_addr *host_address, time_t ttd);
// struct in_addr a_record_from_hosts(char *name, time_t now);
// void cache_unhash_dhcp(void);
// void dump_cache(time_t now);
// #ifndef NO_ID
// int cache_make_stat(struct txt_record *t);
// #endif
// char *cache_get_name(struct crec *crecp);
// char *cache_get_cname_target(struct crec *crecp);
// struct crec *cache_enumerate(int init);
// int read_hostsfile(char *filename, unsigned int index, int cache_size, 
//        struct crec **rhash, int hashsz);

/* blockdata.c */
// void blockdata_init(void);
// void blockdata_report(void);
// struct blockdata *blockdata_alloc(char *data, size_t len);
// void *blockdata_retrieve(struct blockdata *block, size_t len, void *data);
// struct blockdata *blockdata_read(int fd, size_t len);
// void blockdata_write(struct blockdata *block, size_t len, int fd);
// void blockdata_free(struct blockdata *blocks);

/* domain.c */
// char *get_domain(struct in_addr addr);
// char *get_domain6(struct in6_addr *addr);
// int is_name_synthetic(int flags, char *name, union all_addr *addr);
// int is_rev_synth(int flag, union all_addr *addr, char *name);

/* rfc1035.c */
// int extract_name(struct dns_header *header, size_t plen, unsigned char **pp, 
//                  char *name, int isExtract, int extrabytes);
// unsigned char *skip_name(unsigned char *ansp, struct dns_header *header, size_t plen, int extrabytes);
// unsigned char *skip_questions(struct dns_header *header, size_t plen);
// unsigned char *skip_section(unsigned char *ansp, int count, struct dns_header *header, size_t plen);
// unsigned int extract_request(struct dns_header *header, size_t qlen, 
//              char *name, unsigned short *typep);
// size_t setup_reply(struct dns_header *header, size_t  qlen,
//        union all_addr *addrp, unsigned int flags,
//        unsigned long ttl);
// int extract_addresses(struct dns_header *header, size_t qlen, char *name,
//           time_t now, char **ipsets, int is_sign, int check_rebind,
//           int no_cache_dnssec, int secure, int *doctored);
// size_t answer_request(struct dns_header *header, char *limit, size_t qlen,  
//           struct in_addr local_addr, struct in_addr local_netmask, 
//           time_t now, int ad_reqd, int do_bit, int have_pseudoheader);
// int check_for_bogus_wildcard(struct dns_header *header, size_t qlen, char *name, 
//            struct bogus_addr *baddr, time_t now);
// int check_for_ignored_address(struct dns_header *header, size_t qlen, struct bogus_addr *baddr);
// int check_for_local_domain(char *name, time_t now);
// size_t resize_packet(struct dns_header *header, size_t plen, 
//       unsigned char *pheader, size_t hlen);
// int add_resource_record(struct dns_header *header, char *limit, int *truncp,
//       int nameoffset, unsigned char **pp, unsigned long ttl, 
//       int *offset, unsigned short type, unsigned short class, char *format, ...);
// int in_arpa_name_2_addr(char *namein, union all_addr *addrp);
// int private_net(struct in_addr addr, int ban_localhost);

/* auth.c */
// #ifdef HAVE_AUTH
// size_t answer_auth(struct dns_header *header, char *limit, size_t qlen, 
//        time_t now, union mysockaddr *peer_addr, int local_query,
//        int do_bit, int have_pseudoheader);
// int in_zone(struct auth_zone *zone, char *name, char **cut);
// #endif

/* dnssec.c */
// size_t dnssec_generate_query(struct dns_header *header, unsigned char *end, char *name, int class, int type, int edns_pktsz);
// int dnssec_validate_by_ds(time_t now, struct dns_header *header, size_t plen, char *name, char *keyname, int class);
// int dnssec_validate_ds(time_t now, struct dns_header *header, size_t plen, char *name, char *keyname, int class);
// int dnssec_validate_reply(time_t now, struct dns_header *header, size_t plen, char *name, char *keyname, int *class,
//         int check_unsigned, int *neganswer, int *nons, int *nsec_ttl);
// int dnskey_keytag(int alg, int flags, unsigned char *key, int keylen);
// size_t filter_rrsigs(struct dns_header *header, size_t plen);
// int setup_timestamp(void);

/* hash_questions.c */
// void hash_questions_init(void);
// unsigned char *hash_questions(struct dns_header *header, size_t plen, char *name);

/* crypto.c */
// const struct nettle_hash *hash_find(char *name);
// int hash_init(const struct nettle_hash *hash, void **ctxp, unsigned char **digestp);
// int verify(struct blockdata *key_data, unsigned int key_len, unsigned char *sig, size_t sig_len,
//      unsigned char *digest, size_t digest_len, int algo);
// char *ds_digest_name(int digest);
// char *algo_digest_name(int algo);
// char *nsec3_digest_name(int digest);

/* util.c */
// void rand_init(void);
// unsigned short rand16(void);
// u32 rand32(void);
// u64 rand64(void);
// int legal_hostname(char *name);
// char *canonicalise(char *in, int *nomem);
// unsigned char *do_rfc1035_name(unsigned char *p, char *sval, char *limit);
// void *safe_malloc(size_t size);
// void safe_strncpy(char *dest, const char *src, size_t size);
// void safe_pipe(int *fd, int read_noblock);
// void *whine_malloc(size_t size);
// int sa_len(union mysockaddr *addr);
// int sockaddr_isequal(union mysockaddr *s1, union mysockaddr *s2);
// int hostname_isequal(const char *a, const char *b);
// int hostname_issubdomain(char *a, char *b);
// time_t dnsmasq_time(void);
// int netmask_length(struct in_addr mask);
// int is_same_net(struct in_addr a, struct in_addr b, struct in_addr mask);
// int is_same_net6(struct in6_addr *a, struct in6_addr *b, int prefixlen);
// u64 addr6part(struct in6_addr *addr);
// void setaddr6part(struct in6_addr *addr, u64 host);
// int retry_send(ssize_t rc);
// void prettyprint_time(char *buf, unsigned int t);
// int prettyprint_addr(union mysockaddr *addr, char *buf);
// int parse_hex(char *in, unsigned char *out, int maxlen, 
//         unsigned int *wildcard_mask, int *mac_type);
// int memcmp_masked(unsigned char *a, unsigned char *b, int len, 
//       unsigned int mask);
// int expand_buf(struct iovec *iov, size_t size);
// char *print_mac(char *buff, unsigned char *mac, int len);
// int read_write(int fd, unsigned char *packet, int size, int rw);
// void close_fds(long max_fd, int spare1, int spare2, int spare3);
// int wildcard_match(const char* wildcard, const char* match);
// int wildcard_matchn(const char* wildcard, const char* match, int num);
// #ifdef HAVE_LINUX_NETWORK
// int kernel_version(void);
// #endif

/* log.c */
// void die(char *message, char *arg1, int exit_code) ATTRIBUTE_NORETURN;
// int log_start(struct passwd *ent_pw, int errfd);
// int log_reopen(char *log_file);

// void my_syslog(int priority, const char *format, ...);

// void set_log_writer(void);
// void check_log_writer(int force);
// void flush_log(void);

/* option.c */
// void read_opts (int argc, char **argv, char *compile_opts);
// char *option_string(int prot, unsigned int opt, unsigned char *val, 
//         int opt_len, char *buf, int buf_len);
// void reread_dhcp(void);
// void read_servers_file(void);
// void set_option_bool(unsigned int opt);
// void reset_option_bool(unsigned int opt);
// struct hostsfile *expand_filelist(struct hostsfile *list);
// char *parse_server(char *arg, union mysockaddr *addr, 
//        union mysockaddr *source_addr, char *interface, int *flags);
// int option_read_dynfile(char *file, int flags);

/* forward.c */
// void reply_query(int fd, int family, time_t now);
// void receive_query(struct listener *listen, time_t now);
// unsigned char *tcp_request(int confd, time_t now,
//          union mysockaddr *local_addr, struct in_addr netmask, int auth_dns);
// void server_gone(struct server *server);
// struct frec *get_new_frec(time_t now, int *wait, struct frec *force);
// int send_from(int fd, int nowild, char *packet, size_t len, 
//          union mysockaddr *to, union all_addr *source,
//          unsigned int iface);
// void resend_query(void);
// struct randfd *allocate_rfd(int family);
// void free_rfd(struct randfd *rfd);

/* network.c */
// int indextoname(int fd, int index, char *name);
// int local_bind(int fd, union mysockaddr *addr, char *intname, unsigned int ifindex, int is_tcp);
// int random_sock(int family);
// void pre_allocate_sfds(void);
// int reload_servers(char *fname);
// void mark_servers(int flag);
// void cleanup_servers(void);
// void add_update_server(int flags,
//            union mysockaddr *addr,
//            union mysockaddr *source_addr,
//            const char *interface,
//            const char *domain);
// void check_servers(void);
// int enumerate_interfaces(int reset);
// void create_wildcard_listeners(void);
// void create_bound_listeners(int dienow);
// void warn_bound_listeners(void);
// void warn_wild_labels(void);
// void warn_int_names(void);
// int is_dad_listeners(void);
// int iface_check(int family, union all_addr *addr, char *name, int *auth);
// int loopback_exception(int fd, int family, union all_addr *addr, char *name);
// int label_exception(int index, int family, union all_addr *addr);
// int fix_fd(int fd);
// int tcp_interface(int fd, int af);
// int set_ipv6pktinfo(int fd);
// #ifdef HAVE_DHCP6
// void join_multicast(int dienow);
// #endif
// #if defined(HAVE_LINUX_NETWORK) || defined(HAVE_BSD_NETWORK)
// void newaddress(time_t now);
// #endif


/* dhcp.c */
// #ifdef HAVE_DHCP
// void dhcp_init(void);
// void dhcp_packet(time_t now, int pxe_fd);
// struct dhcp_context *address_available(struct dhcp_context *context, 
//                struct in_addr taddr,
//                struct dhcp_netid *netids);
// struct dhcp_context *narrow_context(struct dhcp_context *context, 
//             struct in_addr taddr,
//             struct dhcp_netid *netids);
// struct ping_result *do_icmp_ping(time_t now, struct in_addr addr,
//          unsigned int hash, int loopback);
// int address_allocate(struct dhcp_context *context,
//          struct in_addr *addrp, unsigned char *hwaddr, int hw_len,
//          struct dhcp_netid *netids, time_t now, int loopback);
// void dhcp_read_ethers(void);
// struct dhcp_config *config_find_by_address(struct dhcp_config *configs, struct in_addr addr);
// char *host_from_dns(struct in_addr addr);
// #endif

/* lease.c */
// #ifdef HAVE_DHCP
// void lease_update_file(time_t now);
// void lease_update_dns(int force);
// void lease_init(time_t now);
// struct dhcp_lease *lease4_allocate(struct in_addr addr);
// #ifdef HAVE_DHCP6
// struct dhcp_lease *lease6_allocate(struct in6_addr *addrp, int lease_type);
// struct dhcp_lease *lease6_find(unsigned char *clid, int clid_len, 
//              int lease_type, unsigned int iaid, struct in6_addr *addr);
// void lease6_reset(void);
// struct dhcp_lease *lease6_find_by_client(struct dhcp_lease *first, int lease_type,
//            unsigned char *clid, int clid_len, unsigned int iaid);
// struct dhcp_lease *lease6_find_by_addr(struct in6_addr *net, int prefix, u64 addr);
// u64 lease_find_max_addr6(struct dhcp_context *context);
// void lease_ping_reply(struct in6_addr *sender, unsigned char *packet, char *interface);
// void lease_update_slaac(time_t now);
// void lease_set_iaid(struct dhcp_lease *lease, unsigned int iaid);
// void lease_make_duid(time_t now);
// #endif
// void lease_set_hwaddr(struct dhcp_lease *lease, const unsigned char *hwaddr,
//           const unsigned char *clid, int hw_len, int hw_type,
//           int clid_len, time_t now, int force);
// void lease_set_hostname(struct dhcp_lease *lease, const char *name, int auth, char *domain, char *config_domain);
// void lease_set_expires(struct dhcp_lease *lease, unsigned int len, time_t now);
// void lease_set_interface(struct dhcp_lease *lease, int interface, time_t now);
// struct dhcp_lease *lease_find_by_client(unsigned char *hwaddr, int hw_len, int hw_type,  
//           unsigned char *clid, int clid_len);
// struct dhcp_lease *lease_find_by_addr(struct in_addr addr);
// struct in_addr lease_find_max_addr(struct dhcp_context *context);
// void lease_prune(struct dhcp_lease *target, time_t now);
// void lease_update_from_configs(void);
// int do_script_run(time_t now);
// void rerun_scripts(void);
// void lease_find_interfaces(time_t now);
// #ifdef HAVE_SCRIPT
// void lease_add_extradata(struct dhcp_lease *lease, unsigned char *data, 
//        unsigned int len, int delim);
// #endif
// #endif

/* rfc2131.c */
// #ifdef HAVE_DHCP
// size_t dhcp_reply(struct dhcp_context *context, char *iface_name, int int_index,
//       size_t sz, time_t now, int unicast_dest, int loopback,
//       int *is_inform, int pxe, struct in_addr fallback, time_t recvtime);
// unsigned char *extended_hwaddr(int hwtype, int hwlen, unsigned char *hwaddr, 
//              int clid_len, unsigned char *clid, int *len_out);
// #endif

/* dnsmasq.c */
// #ifdef HAVE_DHCP
// int make_icmp_sock(void);
// int icmp_ping(struct in_addr addr);
// int delay_dhcp(time_t start, int sec, int fd, uint32_t addr, unsigned short id);
// #endif
// void queue_event(int event);
// void send_alarm(time_t event, time_t now);
// void send_event(int fd, int event, int data, char *msg);
// void clear_cache_and_reload(time_t now);

/* netlink.c */
// #ifdef HAVE_LINUX_NETWORK
// char *netlink_init(void);
// void netlink_multicast(void);
// #endif

/* bpf.c */
// #ifdef HAVE_BSD_NETWORK
// void init_bpf(void);
// void send_via_bpf(struct dhcp_packet *mess, size_t len,
//       struct in_addr iface_addr, struct ifreq *ifr);
// void route_init(void);
// void route_sock(void);
// #endif

/* bpf.c or netlink.c */
// int iface_enumerate(int family, void *parm, int (callback)());

/* dbus.c */
// #ifdef HAVE_DBUS
// char *dbus_init(void);
// void check_dbus_listeners(void);
// void set_dbus_listeners(void);
// #  ifdef HAVE_DHCP
// void emit_dbus_signal(int action, struct dhcp_lease *lease, char *hostname);
// #  endif
// #endif

/* ubus.c */
// #ifdef HAVE_UBUS
// void ubus_init(void);
// void set_ubus_listeners(void);
// void check_ubus_listeners(void);
// void ubus_event_bcast(const char *type, const char *mac, const char *ip, const char *name, const char *interface);
// #endif

/* ipset.c */
// #ifdef HAVE_IPSET
// void ipset_init(void);
// int add_to_ipset(const char *setname, const union all_addr *ipaddr, int flags, int remove);
// #endif

/* helper.c */
// #if defined(HAVE_SCRIPT)
// int create_helper(int event_fd, int err_fd, uid_t uid, gid_t gid, long max_fd);
// void helper_write(void);
// void queue_script(int action, struct dhcp_lease *lease, 
//       char *hostname, time_t now);
// #ifdef HAVE_TFTP
// void queue_tftp(off_t file_len, char *filename, union mysockaddr *peer);
// #endif
// void queue_arp(int action, unsigned char *mac, int maclen,
//          int family, union all_addr *addr);
// int helper_buf_empty(void);
// #endif

/* tftp.c */
// #ifdef HAVE_TFTP
// void tftp_request(struct listener *listen, time_t now);
// void check_tftp_listeners(time_t now);
// int do_tftp_script_run(void);
// #endif

/* conntrack.c */
// #ifdef HAVE_CONNTRACK
// int get_incoming_mark(union mysockaddr *peer_addr, union all_addr *local_addr,
//           int istcp, unsigned int *markp);
// #endif

/* dhcp6.c */
// #ifdef HAVE_DHCP6
// void dhcp6_init(void);
// void dhcp6_packet(time_t now);
// struct dhcp_context *address6_allocate(struct dhcp_context *context,  unsigned char *clid, int clid_len, int temp_addr,
//                unsigned int iaid, int serial, struct dhcp_netid *netids, int plain_range, struct in6_addr *ans);
// struct dhcp_context *address6_available(struct dhcp_context *context, 
//           struct in6_addr *taddr,
//           struct dhcp_netid *netids,
//           int plain_range);
// struct dhcp_context *address6_valid(struct dhcp_context *context, 
//             struct in6_addr *taddr,
//             struct dhcp_netid *netids,
//             int plain_range);
// struct dhcp_config *config_find_by_address6(struct dhcp_config *configs, struct in6_addr *net, 
//               int prefix, struct in6_addr *addr);
// void make_duid(time_t now);
// void dhcp_construct_contexts(time_t now);
// void get_client_mac(struct in6_addr *client, int iface, unsigned char *mac, 
//         unsigned int *maclenp, unsigned int *mactypep, time_t now);
// #endif
  
/* rfc3315.c */
// #ifdef HAVE_DHCP6
// unsigned short dhcp6_reply(struct dhcp_context *context, int interface, char *iface_name,  
//          struct in6_addr *fallback, struct in6_addr *ll_addr, struct in6_addr *ula_addr,
//          size_t sz, struct in6_addr *client_addr, time_t now);
// void relay_upstream6(struct dhcp_relay *relay, ssize_t sz, struct in6_addr *peer_address, 
//          u32 scope_id, time_t now);

// unsigned short relay_reply6( struct sockaddr_in6 *peer, ssize_t sz, char *arrival_interface);
// #endif

/* dhcp-common.c */
// #ifdef HAVE_DHCP
// void dhcp_common_init(void);
// ssize_t recv_dhcp_packet(int fd, struct msghdr *msg);
// struct dhcp_netid *run_tag_if(struct dhcp_netid *tags);
// struct dhcp_netid *option_filter(struct dhcp_netid *tags, struct dhcp_netid *context_tags,
//          struct dhcp_opt *opts);
// int match_netid(struct dhcp_netid *check, struct dhcp_netid *pool, int tagnotneeded);
// char *strip_hostname(char *hostname);
// void log_tags(struct dhcp_netid *netid, u32 xid);
// int match_bytes(struct dhcp_opt *o, unsigned char *p, int len);
// void dhcp_update_configs(struct dhcp_config *configs);
// void display_opts(void);
// int lookup_dhcp_opt(int prot, char *name);
// int lookup_dhcp_len(int prot, int val);
// struct dhcp_config *find_config(struct dhcp_config *configs,
//         struct dhcp_context *context,
//         unsigned char *clid, int clid_len,
//         unsigned char *hwaddr, int hw_len, 
//         int hw_type, char *hostname,
//         struct dhcp_netid *filter);
// int config_has_mac(struct dhcp_config *config, unsigned char *hwaddr, int len, int type);
// #ifdef HAVE_LINUX_NETWORK
// char *whichdevice(void);
// void bindtodevice(char *device, int fd);
// #endif
// #  ifdef HAVE_DHCP6
// void display_opts6(void);
// #  endif
// void log_context(int family, struct dhcp_context *context);
// void log_relay(int family, struct dhcp_relay *relay);
// #endif

/* outpacket.c */
// #ifdef HAVE_DHCP6
// void end_opt6(int container);
// void reset_counter(void);
// int save_counter(int newval);
// void *expand(size_t headroom);
// int new_opt6(int opt);
// void *put_opt6(void *data, size_t len);
// void put_opt6_long(unsigned int val);
// void put_opt6_short(unsigned int val);
// void put_opt6_char(unsigned int val);
// void put_opt6_string(char *s);
// #endif

/* radv.c */
// #ifdef HAVE_DHCP6
// void ra_init(time_t now);
// void icmp6_packet(time_t now);
// time_t periodic_ra(time_t now);
// void ra_start_unsolicited(time_t now, struct dhcp_context *context);
// #endif

/* slaac.c */ 
// #ifdef HAVE_DHCP6
// void slaac_add_addrs(struct dhcp_lease *lease, time_t now, int force);
// time_t periodic_slaac(time_t now, struct dhcp_lease *leases);
// void slaac_ping_reply(struct in6_addr *sender, unsigned char *packet, char *interface, struct dhcp_lease *leases);
// #endif

/* loop.c */
// #ifdef HAVE_LOOP
// void loop_send_probes(void);
// int detect_loop(char *query, int type);
// #endif

/* inotify.c */
// #ifdef HAVE_INOTIFY
// void inotify_dnsmasq_init(void);
// int inotify_check(time_t now);
// void set_dynamic_inotify(int flag, int total_size, struct crec **rhash, int revhashsz);
// #endif

/* poll.c */
// void poll_reset(void);
// int poll_check(int fd, short event);
// void poll_listen(int fd, short event);
// int do_poll(int timeout);

/* rrfilter.c */
// size_t rrfilter(struct dns_header *header, size_t plen, int mode);
// u16 *rrfilter_desc(int type);
// int expand_workspace(unsigned char ***wkspc, int *szp, int new);

/* edns0.c */
// unsigned char *find_pseudoheader(struct dns_header *header, size_t plen,
//            size_t *len, unsigned char **p, int *is_sign, int *is_last);
// size_t add_pseudoheader(struct dns_header *header, size_t plen, unsigned char *limit, 
//       unsigned short udp_sz, int optno, unsigned char *opt, size_t optlen, int set_do, int replace);
// size_t add_do_bit(struct dns_header *header, size_t plen, unsigned char *limit);
// size_t add_edns0_config(struct dns_header *header, size_t plen, unsigned char *limit, 
//       union mysockaddr *source, time_t now, int *check_subnet, int *cacheable);
// int check_source(struct dns_header *header, size_t plen, unsigned char *pseudoheader, union mysockaddr *peer);

/* arp.c */
// int find_mac(union mysockaddr *addr, unsigned char *mac, int lazy, time_t now);
// int do_arp_script_run(void);

/* dump.c */
// #ifdef HAVE_DUMPFILE
// void dump_init(void);
// void dump_packet(int mask, void *packet, size_t len, union mysockaddr *src, union mysockaddr *dst);
// #endif


// struct passwd {
// 	char *pw_name;
// 	char *pw_passwd;
// 	uid_t pw_uid;
// 	gid_t pw_gid;
// 	time_t pw_change;
// 	char *pw_class;
// 	char *pw_gecos;
// 	char *pw_dir;
// 	char *pw_shell;
// 	time_t pw_expire;
// }; 
pub struct passwd {
  pub pw_name: String,
  pub pw_passwd: String,
  pub pw_uid: uid_t,
  pub pw_guid: git_t,
  pub pw_change: time::Instant,
  pub pw_class: String,
  pub pw_gecos: String,
  pub pw_dir: String,
  pub pw_shell: String,
  pub pw_expire: time::Instant,
}

type gid_t = u8;
type uid_t = u16;

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

pub struct cap_user_header_t {
    pub version: u32,
    pub pid: i32,
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

pub struct server {
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

pub struct dhcp_relay {
    //   union all_addr local, server;
    pub local: net::IpAddr,
    pub server: net::IpAddr,
    pub interface: String,
    pub iface_index: i32,
    // current
    // next
}
