/* dnsmasq is Copyright (c) 2000-2022 Simon Kelley
 
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

// #define COPYRIGHT "Copyright (c) 2000-2022 Simon Kelley"

/* We do defines that influence behavior of stdio.h, so complain
   if included too early. */
// #ifdef _STDIO_H
// #  error "Header file stdio.h included too early!"
// #endif

// #endif NO_LARGEFILE
/* Ensure we can use files >2GB (log files may grow this big) */
// #  define _LARGEFILE_SOURCE 1
// #  define _FILE_OFFSET_BITS 64
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
// #  define _XPG4_2
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
/* Define before netinet/in.h to select API. OSX Lion onwards. */
// #  define __APPLE_USE_RFC_3542
// #endif
// #include <netinet/in.h>

/* Also needed before config.h. */
// #include <getopt.h>

// #include "config.h"
// #include "ip6addr.h"
// #include "metrics.h"

// typedef unsigned char u8;
// typedef unsigned short u16;
// typedef unsigned u32: i32;
// typedef unsigned long long u64;
//
// #define countof(x)      (long)(sizeof(x) / sizeof(x[0]))
// #define MIN(a,b)        ((a) < (b) ? (a) : (b))

// #include "dns-protocol.h"
// #include "dhcp-protocol.h"
// #ifdef HAVE_DHCP6
// #include "dhcp6-protocol.h"
// #include "radv-protocol.h"
// #endif

// #define gettext_noop(S) (S)
// // #endif LOCALEDIR
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
// #include <poll.h>
// #include <sys/wait.h>
// #include <sys/time.h>
// #include <sys/un.h>
// #include <limits.h>
// #include <net/if.h>
// #if defined(HAVE_SOLARIS_NETWORK) && !defined(ifr_mtu)
/* Some solaris net/if./h omit this. */
// #  define ifr_mtu  ifr_ifru.ifru_metric
// #endif
// #include <unistd.h>
// #include <stdio.h>
// #include <stdint.h>
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
// #include <netdb.h>
// #endif HAVE_LINUX_NETWORK
// #  include <net/if_dl.h>
// #endif

// #if defined(HAVE_LINUX_NETWORK)
// #include <linux/version.h>
// #include <linux/sockios.h>
// #include <linux/capability.h>

// TODO:
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

// /* Backwards compat with 2.83 */
// #if defined(HAVE_NETTLEHASH)
// #  define HAVE_CRYPTOHASH
// // #endif
// #if defined(HAVE_DNSSEC) || defined(HAVE_CRYPTOHASH)
// #  include <nettle/nettle-meta.h>
// // #endif

/* daemon is function in the C library.... */
// #define daemon dnsmasq_daemon

// TODO:
// #define ADDRSTRLEN INET6_ADDRSTRLEN



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
pub const OPT_IGNORE_CLID: u32 = 59;
pub const OPT_SINGLE_PORT: u32 = 60;
pub const OPT_LEASE_RENEW: u32 = 61;
pub const OPT_LOG_DEBUG: u32 = 62;
pub const OPT_UMBRELLA: u32 = 63;
pub const OPT_UMBRELLA_DEVID: u32 = 64;
pub const OPT_CMARK_ALST_EN: u32 = 65;
pub const OPT_QUIET_TFTP: u32 = 66;
pub const OPT_FILTER_A: u32 = 67;
pub const OPT_FILTER_AAAA: u32 = 68;
pub const OPT_STRIP_ECS: u32 = 69;
pub const OPT_STRIP_MAC: u32 = 70;
pub const OPT_NORR: u32 = 71;
pub const OPT_LAST: u32 = 72;

// #define OPTION_BITS (sizeof(unsigned int)*8)
// #define OPTION_SIZE ( (OPT_LAST/OPTION_BITS)+((OPT_LAST%OPTION_BITS)!=0) )
// #define option_var(x) (daemon.options[(x) / OPTION_BITS])
// #define option_val(x) ((1u) << ((x) % OPTION_BITS))
// #define option_bool(x) (option_var(x) & option_val(x))

/* extra flags for my_syslog, we use facilities since they are known 
   not to occupy the same bits as priorities, no matter how syslog.h is set up. 
   MS_DEBUG messages are suppressed unless --log-debug is set. */
// #define MS_TFTP   LOG_USER
// #define MS_DHCP   LOG_DAEMON
// #define MS_SCRIPT LOG_MAIL
// #define MS_DEBUG  LOG_NEWS

// #endif NO_ID
pub const TXT_STAT_CACHESIZE: u32 = 1;
pub const TXT_STAT_INSERTS: u32 = 2;
pub const TXT_STAT_EVICTIONS: u32 = 3;
pub const TXT_STAT_MISSES: u32 = 4;
pub const TXT_STAT_HITS: u32 = 5;
pub const TXT_STAT_AUTH: u32 = 6;
pub const TXT_STAT_SERVERS: u32 = 7;
// #endif

pub const ADDRLIST_LITERAL: u32 = 1;
pub const ADDRLIST_IPV6: u32 = 2;
pub const ADDRLIST_REVONLY: u32 = 4;
pub const ADDRLIST_PREFIX: u32 = 8;
pub const ADDRLIST_WILDCARD: u32 = 16;
pub const ADDRLIST_DECLINED: u32 = 32;

pub const AUTH6: u32 = 1;
pub const AUTH4: u32 = 2;

pub const HR_6: u32 = 1;
pub const HR_4: u32 = 2;

pub const IN4: u32 = 1;
pub const IN6: u32 = 2;
pub const INP4: u32 = 4;
pub const INP6: u32 = 8;

pub const F_IMMORTAL: u32 = 1 << 0;
pub const F_NAMEP: u32 = 1 << 1;
pub const F_REVERSE: u32 = 1 << 2;
pub const F_FORWARD: u32 = 1 << 3;
pub const F_DHCP: u32 = 1 << 4;
pub const F_NEG: u32 = 1 << 5;
pub const F_HOSTS: u32 = 1 << 6;
pub const F_IPV: u32 = 1 << 7;
pub const F_IPV2: u32 = 1 << 8;
pub const F_BIGNAME: u32 = 1 << 9;
pub const F_NXDOMAIN: u32 = 1 << 10;
pub const F_CNAME: u32 = 1 << 11;
pub const F_DNSKEY: u32 = 1 << 12;
pub const F_CONFIG: u32 = 1 << 13;
pub const F_DS: u32 = 1 << 14;
pub const F_DNSSECOK: u32 = 1 << 15;
pub const F_UPSTREAM: u32 = 1 << 16;
pub const F_RRNAME: u32 = 1 << 17;
pub const F_SERVER: u32 = 1 << 18;
pub const F_QUERY: u32 = 1 << 19;
pub const F_NOERR: u32 = 1 << 20;
pub const F_AUTH: u32 = 1 << 21;
pub const F_DNSSEC: u32 = 1 << 22;
pub const F_KEYTAG: u32 = 1 << 23;
pub const F_SECSTAT: u32 = 1 << 24;
pub const F_NO_RR: u32 = 1 << 25;
pub const F_IPSET: u32 = 1 << 26;
pub const F_NOEXTRA: u32 = 1 << 27;
pub const F_DOMAINSRV: u32 = 1 << 28;
pub const F_RCODE: u32 = 1 << 29;
pub const F_SRV: u32 = 1 << 30;
pub const F_STALE: u32 = 1 << 31;

pub const UID_NONE: u32 = 0;
/* Values of uid in crecs with F_CONFIG bit set. */
pub const SRC_CONFIG: u32 = 1;
pub const SRC_HOSTS: u32 = 2;
pub const SRC_AH: u32 = 3;




/* bits in flag param to IPv6 callbacks from iface_enumerate() */
pub const IFACE_TENTATIVE: u32 = 1;
pub const IFACE_DEPRECATED: u32 = 2;
pub const IFACE_PERMANENT: u32 = 4;


/* The actual values here matter, since we sort on them to get records in the order
   IPv6 addr, IPv4 addr, all zero return, resolvconf servers, upstream server, no-data return  */
pub const SERV_LITERAL_ADDRESS: u32 = 1;  /* addr is the answer, or NoDATA is the answer, depending on the next four flags */
pub const SERV_USE_RESOLV: u32 = 2;  /* forward this domain in the normal way */
pub const SERV_ALL_ZEROS: u32 = 4;  /* return all zeros for A and AAAA */
pub const SERV_4ADDR: u32 = 8;  /* addr is IPv4 */
pub const SERV_6ADDR: u32 = 16;  /* addr is IPv6 */
pub const SERV_HAS_SOURCE: u32 = 32;  /* source address defined */
pub const SERV_FOR_NODOTS: u32 = 64;  /* server for names with no domain part only */
pub const SERV_WARNED_RECURSIVE: u32 = 128;  /* avoid warning spam */
pub const SERV_FROM_DBUS: u32 = 256;  /* 1 if source is DBus */
pub const SERV_MARK: u32 = 512;  /* for mark-and-delete and log code */
pub const SERV_WILDCARD: u32 = 1024;  /* domain has leading '*' */
pub const SERV_FROM_RESOLV: u32 = 2048;  /* 1 for servers from resolv, 0 for command line. */
pub const SERV_FROM_FILE: u32 = 4096;  /* read from --servers-file */
pub const SERV_LOOP: u32 = 8192;  /* server causes forwarding loop */
pub const SERV_DO_DNSSEC: u32 = 16384;  /* Validate DNSSEC when using this server */
pub const SERV_GOT_TCP: u32 = 32768;  /* Got some data from the TCP connection */










/* First four fields must match struct server in next three definitions.. */



















/* adn-hosts parms from command-line (also dhcp-hostsfile and dhcp-optsfile and dhcp-hostsdir*/
pub const AH_DIR: u32 = 1;
pub const AH_INACTIVE: u32 = 2;
pub const AH_WD_DONE: u32 = 4;
pub const AH_HOSTS: u32 = 8;
pub const AH_DHCP_HST: u32 = 16;
pub const AH_DHCP_OPT: u32 = 32;




/* packet-dump flags */
pub const DUMP_QUERY: u32 = 0x0001;
pub const DUMP_REPLY: u32 = 0x0002;
pub const DUMP_UP_QUERY: u32 = 0x0004;
pub const DUMP_UP_REPLY: u32 = 0x0008;
pub const DUMP_SEC_QUERY: u32 = 0x0010;
pub const DUMP_SEC_REPLY: u32 = 0x0020;
pub const DUMP_BOGUS: u32 = 0x0040;
pub const DUMP_SEC_BOGUS: u32 = 0x0080;
pub const DUMP_DHCP: u32 = 0x1000;
pub const DUMP_DHCPV6: u32 = 0x2000;
pub const DUMP_RA: u32 = 0x4000;
pub const DUMP_TFTP: u32 = 0x8000;

/* DNSSEC status values. */
pub const STAT_SECURE: u32 = 0x10000;
pub const STAT_INSECURE: u32 = 0x20000;
pub const STAT_BOGUS: u32 = 0x30000;
pub const STAT_NEED_DS: u32 = 0x40000;
pub const STAT_NEED_KEY: u32 = 0x50000;
pub const STAT_TRUNCATED: u32 = 0x60000;
pub const STAT_SECURE_WILDCARD: u32 = 0x70000;
pub const STAT_OK: u32 = 0x80000;
pub const STAT_ABANDONED: u32 = 0x90000;

pub const DNSSEC_FAIL_NYV: u32 = 0x0001; /* key not yet valid */
pub const DNSSEC_FAIL_EXP: u32 = 0x0002; /* key expired */
pub const DNSSEC_FAIL_INDET: u32 = 0x0004; /* indetermined */
pub const DNSSEC_FAIL_NOKEYSUP: u32 = 0x0008; /* no supported key algo. */
pub const DNSSEC_FAIL_NOSIG: u32 = 0x0010; /* No RRsigs */
pub const DNSSEC_FAIL_NOZONE: u32 = 0x0020; /* No Zone bit set */
pub const DNSSEC_FAIL_NONSEC: u32 = 0x0040; /* No NSEC */
pub const DNSSEC_FAIL_NODSSUP: u32 = 0x0080; /* no supported DS algo. */
pub const DNSSEC_FAIL_NOKEY: u32 = 0x0100; /* no DNSKEY */

#define STAT_ISEQUAL(a, b)  (((a) & 0xffff0000) == (b))

pub const FREC_NOREBIND: u32 = 1;
pub const FREC_CHECKING_DISABLED: u32 = 2;
pub const FREC_NO_CACHE: u32 = 4;
pub const FREC_DNSKEY_QUERY: u32 = 8;
pub const FREC_DS_QUERY: u32 = 16;
pub const FREC_AD_QUESTION: u32 = 32;
pub const FREC_DO_QUESTION: u32 = 64;
pub const FREC_ADDED_PHEADER: u32 = 128;
pub const FREC_TEST_PKTSZ: u32 = 256;
pub const FREC_HAS_EXTRADATA: u32 = 512;
pub const FREC_HAS_PHEADER: u32 = 1024;

pub const HASH_SIZE: u32 = 32; /* SHA-256 digest size */

struct frec {
  struct frec_src {
    union mysockaddr source;
    union all_addr dest;
    unsigned int iface, log_id;
    fd: i32;
    unsigned short orig_id;
    struct frec_src *next;
  } frec_src;
  struct server *sentto; /* NULL means free */
  struct randfd_list *rfds;
  unsigned short new_id;
  int forwardall, flags;
  time_t time;
  u32 forward_timestamp;
  forward_delay: i32;
  unsigned char *hash[HASH_SIZE];
  struct blockdata *stash; /* Saved reply, whilst we validate */
  stash_len: usize;
// #ifdef HAVE_DNSSEC
  int class, work_counter;
  struct frec *dependent; /* Query awaiting internally-generated DNSKEY or DS query */
  struct frec *next_dependent; /* list of above. */
  struct frec *blocking_query; /* Query which is blocking us. */
// #endif
  struct frec *next;
};

/* flags in top of length field for DHCP-option tables */
pub const OT_ADDR_LIST: u32 = 0x8000;
pub const OT_RFC1035_NAME: u32 = 0x4000;
pub const OT_INTERNAL: u32 = 0x2000;
pub const OT_NAME: u32 = 0x1000;
pub const OT_CSTRING: u32 = 0x0800;
pub const OT_DEC: u32 = 0x0400;
pub const OT_TIME: u32 = 0x0200;

/* actions in the daemon->helper RPC */
pub const ACTION_DEL: u32 = 1;
pub const ACTION_OLD_HOSTNAME: u32 = 2;
pub const ACTION_OLD: u32 = 3;
pub const ACTION_ADD: u32 = 4;
pub const ACTION_TFTP: u32 = 5;
pub const ACTION_ARP: u32 = 6;
pub const ACTION_ARP_DEL: u32 = 7;
pub const ACTION_RELAY_SNOOP: u32 = 8;

pub const LEASE_NEW: u32 = 1;  /* newly created */
pub const LEASE_CHANGED: u32 = 2;  /* modified */
pub const LEASE_AUX_CHANGED: u32 = 4;  /* CLID or expiry changed */
pub const LEASE_AUTH_NAME: u32 = 8;  /* hostname came from config, not from client */
pub const LEASE_USED: u32 = 16;  /* used this DHCPv6 transaction */
pub const LEASE_NA: u32 = 32;  /* IPv6 no-temporary lease */
pub const LEASE_TA: u32 = 64;  /* IPv6 temporary lease */
pub const LEASE_HAVE_HWADDR: u32 = 128;  /* Have set hwaddress */
pub const LEASE_EXP_CHANGED: u32 = 256;  /* Lease expiry time changed */

struct dhcp_lease {
  clid_len: i32;          /* length of client identifier */
  unsigned char *clid;   /* clientid */
  char *hostname, *fqdn; /* name from client-hostname option or config */
  char *old_hostname;    /* hostname before it moved to another lease */
  flags: i32;
  time_t expires;        /* lease expiry */
// #ifdef HAVE_BROKEN_RTC
  unsigned length: i32;
// #endif
  int hwaddr_len, hwaddr_type;
  unsigned char hwaddr[DHCP_CHADDR_MAX]; 
  struct in_addr addr, override, giaddr;
  unsigned char *extradata;
  unsigned int extradata_len, extradata_size;
  last_interface: i32;
  new_interface: i32;     /* save possible originated interface */
  new_prefixlen: i32;     /* and its prefix length */
// #ifdef HAVE_DHCP6
  addr6: in6_addr;
  unsigned iaid: i32;
  struct slaac_address {
    addr: in6_addr;
    time_t ping_time;
    backoff: i32; /* zero -> confirmed */
    struct slaac_address *next;
  } *slaac_address;
  vendorclass_count: i32;
// #endif
  struct dhcp_lease *next;
};

struct dhcp_netid {
  char *net;
  struct dhcp_netid *next;
};

struct dhcp_netid_list {
  struct dhcp_netid *list;
  struct dhcp_netid_list *next;
};

struct tag_if {
  struct dhcp_netid_list *set;
  struct dhcp_netid *tag;
  struct tag_if *next;
};

struct delay_config {
  delay: i32;
  struct dhcp_netid *netid;
  struct delay_config *next;
};

struct hwaddr_config {
  int hwaddr_len, hwaddr_type;
  unsigned char hwaddr[DHCP_CHADDR_MAX];
  unsigned wildcard_mask: i32;
  struct hwaddr_config *next;
};

struct dhcp_config {
  unsigned flags: i32;
  clid_len: i32;          /* length of client identifier */
  unsigned char *clid;   /* clientid */
  char *hostname, *domain;
  struct dhcp_netid_list *netid;
  struct dhcp_netid *filter;
// #ifdef HAVE_DHCP6
  addr6: *mut addrlist;
// #endif
  addr: in_addr;
  time_t decline_time;
  unsigned lease_time: i32;
  struct hwaddr_config *hwaddr;
  struct dhcp_config *next;
};

#define have_config(config, mask) ((config) && ((config).flags & (mask)))

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

struct dhcp_opt {
  int opt, len, flags;
  union {
    encap: i32;
    unsigned wildcard_mask: i32;
    unsigned char *vendor_class;
  } u;
  unsigned char *val;
  struct dhcp_netid *netid;
  struct dhcp_opt *next;
};

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

struct dhcp_boot {
  char *file, *sname, *tftp_sname;
  next_server: in_addr;
  struct dhcp_netid *netid;
  struct dhcp_boot *next;
};

struct dhcp_match_name {
  char *name;
  wildcard: i32;
  struct dhcp_netid *netid;
  struct dhcp_match_name *next;
};

struct pxe_service {
  unsigned short CSA, type; 
  char *menu, *basename, *sname;
  server: in_addr;
  struct dhcp_netid *netid;
  struct pxe_service *next;
};

#define DHCP_PXE_DEF_VENDOR      "PXEClient"

pub const MATCH_VENDOR: u32 = 1;
pub const MATCH_USER: u32 = 2;
pub const MATCH_CIRCUIT: u32 = 3;
pub const MATCH_REMOTE: u32 = 4;
pub const MATCH_SUBSCRIBER: u32 = 5;

/* vendorclass, userclass, remote-id or circuit-id */
struct dhcp_vendor {
  int len, match_type;
  unsigned enterprise: i32;
  char *data;
  struct dhcp_netid netid;
  struct dhcp_vendor *next;
};

struct dhcp_pxe_vendor {
  char *data;
  struct dhcp_pxe_vendor *next;
};

struct dhcp_mac {
  unsigned mask: i32;
  int hwaddr_len, hwaddr_type;
  unsigned char hwaddr[DHCP_CHADDR_MAX];
  struct dhcp_netid netid;
  struct dhcp_mac *next;
};

struct dhcp_bridge {
  char iface[IF_NAMESIZE];
  struct dhcp_bridge *alias, *next;
};

struct cond_domain {
  char *domain, *prefix; /* prefix is text-prefix on domain name */
  char *interface;       /* These two set when domain comes from interface. */
  al: *mut addrlist;
  struct in_addr start, end;
  struct in6_addr start6, end6;
  int is6, indexed, prefixlen;
  struct cond_domain *next;
}; 

struct ra_interface {
  char *name;
  char *mtu_name;
  int interval, lifetime, prio, mtu;
  struct ra_interface *next;
};

struct dhcp_context {
  unsigned int lease_time, addr_epoch;
  struct in_addr netmask, broadcast;
  struct in_addr local, router;
  struct in_addr start, end; /* range of available addresses */
// #ifdef HAVE_DHCP6
  struct in6_addr start6, end6; /* range of available addresses */
  local6: in6_addr;
  int prefix, if_index;
  unsigned int valid, preferred, saved_valid;
  time_t ra_time, ra_short_period_start, address_lost_time;
  char *template_interface;
// #endif
  flags: i32;
  struct dhcp_netid netid, *filter;
  struct dhcp_context *next, *current;
};

struct shared_network {
  if_index: i32;
  struct in_addr match_addr, shared_addr;
// #ifdef HAVE_DHCP6
  /* shared_addr == 0 for IP6 entries. */
  struct in6_addr match_addr6, shared_addr6;
// #endif
  struct shared_network *next;
};

#define CONTEXT_STATIC         (1u<<0)
#define CONTEXT_NETMASK        (1u<<1)
#define CONTEXT_BRDCAST        (1u<<2)
#define CONTEXT_PROXY          (1u<<3)
#define CONTEXT_RA_ROUTER      (1u<<4)
#define CONTEXT_RA_DONE        (1u<<5)
#define CONTEXT_RA_NAME        (1u<<6)
#define CONTEXT_RA_STATELESS   (1u<<7)
#define CONTEXT_DHCP           (1u<<8)
#define CONTEXT_DEPRECATE      (1u<<9)
#define CONTEXT_TEMPLATE       (1u<<10)    /* create contexts using addresses */
#define CONTEXT_CONSTRUCTED    (1u<<11)
#define CONTEXT_GC             (1u<<12)
#define CONTEXT_RA             (1u<<13)
#define CONTEXT_CONF_USED      (1u<<14)
#define CONTEXT_USED           (1u<<15)
#define CONTEXT_OLD            (1u<<16)
pub const CONTEXT_V: u32 = 6;             (1u<<17)
#define CONTEXT_RA_OFF_LINK    (1u<<18)
#define CONTEXT_SETLEASE       (1u<<19)

struct ping_result {
  addr: in_addr;
  time_t time;
  unsigned hash: i32;
  struct ping_result *next;
};

struct tftp_file {
  int refcount, fd;
  off_t size;
  dev_t dev;
  ino_t inode;
  char filename[];
};

struct tftp_transfer {
  sockfd: i32;
  time_t timeout;
  backoff: i32;
  unsigned int block, blocksize, expansion;
  off_t offset;
  union mysockaddr peer;
  union all_addr source;
  if_index: i32;
  char opt_blocksize, opt_transize, netascii, carrylf;
  struct tftp_file *file;
  struct tftp_transfer *next;
};

struct addr_list {
  addr: in_addr;
  struct addr_list *next;
};

struct tftp_prefix {
  char *interface;
  char *prefix;
  missing: i32;
  struct tftp_prefix *next;
};

struct dhcp_relay {
  union all_addr local, server;
  char *interface; /* Allowable interface for replies from server, and dest for IPv6 multicast */
  iface_index: i32; /* working - interface in which requests arrived, for return */
  port: i32;        /* Port of relay we forward to. */
// #ifdef HAVE_SCRIPT
  struct snoop_record {
    struct in6_addr client, prefix;
    prefix_len: i32;
    struct snoop_record *next;
  } *snoop_records;
// #endif
  struct dhcp_relay *next;
};

extern struct daemon {
  /* datastuctures representing the command-line and 
     config file arguments. All set (including defaults)
     in option.c */

  unsigned int options[OPTION_SIZE];
  struct resolvc default_resolv, *resolv_files;
  time_t last_resolv;
  char *servers_file;
  struct mx_srv_record *mxnames;
  struct naptr *naptr;
  struct txt_record *txt, *rr;
  struct ptr_record *ptr;
  struct host_record *host_records, *host_records_tail;
  struct cname *cnames;
  struct auth_zone *auth_zones;
  struct interface_name *int_names;
  char *mxtarget;
  struct mysubnet *add_subnet4;
  struct mysubnet *add_subnet6;
  char *lease_file;
  char *username, *groupname, *scriptuser;
  char *luascript;
  char *authserver, *hostmaster;
  struct iname *authinterface;
  struct name_list *secondary_forward_server;
  int group_set, osport;
  char *domain_suffix;
  struct cond_domain *cond_domain, *synth_domains;
  char *runfile; 
  char *lease_change_command;
  struct iname *if_names, *if_addrs, *if_except, *dhcp_except, *auth_peers, *tftp_interfaces;
  struct bogus_addr *bogus_addr, *ignore_addr;
  struct server *servers, *servers_tail, *local_domains, **serverarray;
  struct rebind_domain *no_rebind;
  server_has_wildcard: i32;
  int serverarraysz, serverarrayhwm;
  struct ipsets *ipsets, *nftsets;
  u32 allowlist_mask;
  struct allowlist *allowlists;
  log_fac: i32; /* log facility */
  char *log_file; /* optional log file */
  max_logs: i32;  /* queue limit */
  randport_limit: i32; /* Maximum number of source ports for query. */
  int cachesize, ftabsize;
  int port, query_port, min_port, max_port;
  unsigned long local_ttl, neg_ttl, max_ttl, min_cache_ttl, max_cache_ttl, auth_ttl, dhcp_ttl, use_dhcp_ttl;
  char *dns_client_id;
  u32 umbrella_org;
  u32 umbrella_asset;
  u8 umbrella_device[8];
  host_index: i32;
  struct hostsfile *addn_hosts;
  struct dhcp_context *dhcp, *dhcp6;
  struct ra_interface *ra_interfaces;
  struct dhcp_config *dhcp_conf;
  struct dhcp_opt *dhcp_opts, *dhcp_match, *dhcp_opts6, *dhcp_match6;
  struct dhcp_match_name *dhcp_name_match;
  struct dhcp_pxe_vendor *dhcp_pxe_vendors;
  struct dhcp_vendor *dhcp_vendors;
  struct dhcp_mac *dhcp_macs;
  struct dhcp_boot *boot_config;
  struct pxe_service *pxe_services;
  struct tag_if *tag_if; 
  struct addr_list *override_relays;
  struct dhcp_relay *relay4, *relay6;
  struct delay_config *delay_conf;
  override: i32;
  enable_pxe: i32;
  int doing_ra, doing_dhcp6;
  struct dhcp_netid_list *dhcp_ignore, *dhcp_ignore_names, *dhcp_gen_names; 
  struct dhcp_netid_list *force_broadcast, *bootp_dynamic;
  struct hostsfile *dhcp_hosts_file, *dhcp_opts_file;
  struct dyndir *dynamic_dirs;
  int dhcp_max, tftp_max, tftp_mtu;
  int dhcp_server_port, dhcp_client_port;
  int start_tftp_port, end_tftp_port; 
  unsigned min_leasetime: i32;
  struct doctor *doctors;
  unsigned short edns_pktsz;
  char *tftp_prefix; 
  struct tftp_prefix *if_prefix; /* per-interface TFTP prefixes */
  unsigned int duid_enterprise, duid_config_len;
  unsigned char *duid_config;
  char *dbus_name;
  char *ubus_name;
  char *dump_file;
  dump_mask: i32;
  unsigned long soa_sn, soa_refresh, soa_retry, soa_expiry;
  u32 metrics[__METRIC_MAX];
  int fast_retry_time, fast_retry_timeout;
  cache_max_expiry: i32;
// #ifdef HAVE_DNSSEC
  struct ds_config *ds;
  char *timestamp_file;
// #endif

  /* globally used stuff for DNS */
  char *packet; /* packet buffer */
  packet_buff_sz: i32; /* size of above */
  char *namebuff; /* MAXDNAME size buffer */
#if (defined(HAVE_CONNTRACK) && defined(HAVE_UBUS)) || defined(HAVE_DNSSEC)
  /* CONNTRACK UBUS code uses this buffer, as well as DNSSEC code. */
  char *workspacename;
// #endif
// #ifdef HAVE_DNSSEC
  char *keyname; /* MAXDNAME size buffer */
  unsigned long *rr_status; /* ceiling in TTL from DNSSEC or zero for insecure */
  rr_status_sz: i32;
  dnssec_no_time_check: i32;
  back_to_the_future: i32;
// #endif
  struct frec *frec_list;
  struct frec_src *free_frec_src;
  frec_src_count: i32;
  struct serverfd *sfds;
  struct irec *interfaces;
  struct listener *listeners;
  struct server *srv_save; /* Used for resend on DoD */
  packet_len: usize;       /*      "        "        */
  int    fd_save;          /*      "        "        */
  pid_t tcp_pids[MAX_PROCS];
  int tcp_pipes[MAX_PROCS];
  pipe_to_parent: i32;
  numrrand: i32;
  struct randfd *randomsocks;
  struct randfd_list *rfl_spare, *rfl_poll;
  v6pktinfo: i32;
  interface_addrs: *mut addrlist; /* list of all addresses/prefix lengths associated with all local interfaces */
  int log_id, log_display_id; /* ids of transactions for logging */
  union mysockaddr *log_source_addr;

  /* DHCP state */
  int dhcpfd, helperfd, pxefd; 
// #ifdef HAVE_INOTIFY
  inotifyfd: i32;
// #endif
#if defined(HAVE_LINUX_NETWORK)
  int netlinkfd, kernel_version;
#elif defined(HAVE_BSD_NETWORK)
  int dhcp_raw_fd, dhcp_icmp_fd, routefd;
// #endif
  struct iovec dhcp_packet;
  char *dhcp_buff, *dhcp_buff2, *dhcp_buff3;
  struct ping_result *ping_results;
  FILE *lease_stream;
  struct dhcp_bridge *bridges;
  struct shared_network *shared_networks;
// #ifdef HAVE_DHCP6
  duid_len: i32;
  unsigned char *duid;
  struct iovec outpacket;
  int dhcp6fd, icmp6fd;
#  ifdef HAVE_SCRIPT
  struct snoop_record *free_snoops;
#  endif
// #endif
  
  /* DBus stuff */
  /* void * here to avoid depending on dbus headers outside dbus.c */
  void *dbus;
// #ifdef HAVE_DBUS
  struct watch *watches;
// #endif

  /* UBus stuff */
// #ifdef HAVE_UBUS
  /* void * here to avoid depending on ubus headers outside ubus.c */
  void *ubus;
// #endif

  /* TFTP stuff */
  struct tftp_transfer *tftp_trans, *tftp_done_trans;

  /* utility string buffer, hold max sized IP address as string */
  char *addrbuff;
  char *addrbuff2; /* only allocated when OPT_EXTRALOG */

// #ifdef HAVE_DUMPFILE
  /* file for packet dumps. */
  dumpfd: i32;
// #endif
} *daemon;

struct server_details {
  union mysockaddr *addr, *source_addr;
  struct addrinfo *hostinfo, *orig_hostinfo;
  char *interface, *source, *scope_id, *interface_opt;
  int serv_port, source_port, addr_type, scope_index, valid;
  u16 *flags;
};

pub const RRFILTER_EDNS0: u32 = 0;
pub const RRFILTER_DNSSEC: u32 = 1;
pub const RRFILTER_A: u32 = 2;
pub const RRFILTER_AAAA: u32 = 3;
