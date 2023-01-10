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

// pub const COPYRIGHT: String = String::from("Copyright (c) 2000-2022 Simon Kelley");

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

// #define STAT_ISEQUAL(a, b)  (((a) & 0xffff0000) == (b))
pub fn STAT_ISEQUAL(a: u32, b: u32) -> bool {
    (a & 0xffff0000) == b
}

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


pub const DHCP_PXE_DEF_VENDOR: String = String::from("PXEClient");

pub const MATCH_VENDOR: u32 = 1;
pub const MATCH_USER: u32 = 2;
pub const MATCH_CIRCUIT: u32 = 3;
pub const MATCH_REMOTE: u32 = 4;
pub const MATCH_SUBSCRIBER: u32 = 5;


pub const CONTEXT_STATIC: u32 = 1 << 0;
pub const CONTEXT_NETMASK: u32 = 1 << 1;
pub const CONTEXT_BRDCAST: u32 = 1 << 2;
pub const CONTEXT_PROXY: u32 = 1 << 3;
pub const CONTEXT_RA_ROUTER: u32 = 1 << 4;
pub const CONTEXT_RA_DONE: u32 = 1 << 5;
pub const CONTEXT_RA_NAME: u32 = 1 << 6;
pub const CONTEXT_RA_STATELESS: u32 = 1 << 7;
pub const CONTEXT_DHCP: u32 = 1 << 8;
pub const CONTEXT_DEPRECATE: u32 = 1 << 9;
pub const CONTEXT_TEMPLATE: u32 = 1 << 10;    /* create contexts using addresses */
pub const CONTEXT_CONSTRUCTED: u32 = 1 << 11;
pub const CONTEXT_GC: u32 = 1 << 12;
pub const CONTEXT_RA: u32 = 1 << 13;
pub const CONTEXT_CONF_USED: u32 = 1 << 14;
pub const CONTEXT_USED: u32 = 1 << 15;
pub const CONTEXT_OLD: u32 = 1 << 16;
pub const CONTEXT_V6: u32 = 1 << 17;
pub const CONTEXT_RA_OFF_LINK: u32 = 1 << 18;
pub const CONTEXT_SETLEASE: u32 = 1 << 19;


pub const RRFILTER_EDNS0: u32 = 0;
pub const RRFILTER_DNSSEC: u32 = 1;
pub const RRFILTER_A: u32 = 2;
pub const RRFILTER_AAAA: u32 = 3;
