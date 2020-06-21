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

package dnsmasq

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
// #include "ip6addr.h"
// #include "metrics.h"

// const countof :u32 = ;(x)      (long)(sizeof(x) / sizeof(x[0]))
// TODO: define or replace

//const MIN :u32 = ;(a, b)        ((a) < (b) ? (a) : (b))
// TODO: define or replace

// #include "dns-protocol.h"
// #include "dhcp-protocol.h"

// #ifdef HAVE_DHCP6

// #include "dhcp6-protocol.h"
// #include "radv-protocol.h"

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

const LINUX_CAPABILITY_VERSION_1 = 0x19980330
const LINUX_CAPABILITY_VERSION_2 = 0x20071026
const LINUX_CAPABILITY_VERSION_3 = 0x20080522

// #include <sys/prctl.h>

// #elif defined(HAVE_SOLARIS_NETWORK)
// #include <priv.h>
// #endif

// #ifdef HAVE_DNSSEC
// #  include <nettle/nettle-meta.h>
// #endif

// #include "dnsmasq_sys.h"

/* daemon is function in the C library.... */
// const daemon :u32 = ;dnsmasq_daemon

/* Async event queue */
type event_desc struct {
	msg_sz uint32
	event  uint32
	data   uint32
}

const EVENT_RELOAD = 1
const EVENT_DUMP = 2
const EVENT_ALARM = 3
const EVENT_TERM = 4
const EVENT_CHILD = 5
const EVENT_REOPEN = 6
const EVENT_EXITED = 7
const EVENT_KILLED = 8
const EVENT_EXEC_ERR = 9
const EVENT_PIPE_ERR = 10
const EVENT_USER_ERR = 11
const EVENT_CAP_ERR = 12
const EVENT_PIDFILE = 13
const EVENT_HUSER_ERR = 14
const EVENT_GROUP_ERR = 15
const EVENT_DIE = 16
const EVENT_LOG_ERR = 17
const EVENT_FORK_ERR = 18
const EVENT_LUA_ERR = 19
const EVENT_TFTP_ERR = 20
const EVENT_INIT = 21
const EVENT_NEWADDR = 22
const EVENT_NEWROUTE = 23
const EVENT_TIME_ERR = 24
const EVENT_SCRIPT_LOG = 25
const EVENT_TIME = 26

/* Exit codes. */
const EC_GOOD = 0
const EC_BADCONF = 1
const EC_BADNET = 2
const EC_FILE = 3
const EC_NOMEM = 4
const EC_MISC = 5
const EC_INIT_OFFSET = 10

/* Trust the compiler dead-code eliminator.... */
// const option_bool :u32 = ;(x) (((x) < 32) ? daemon->options & (1u << (x)) : daemon->options2 & (1u << ((x) - 32)))

const OPT_BOGUSPRIV = 0
const OPT_FILTER = 1
const OPT_LOG = 2
const OPT_SELFMX = 3
const OPT_NO_HOSTS = 4
const OPT_NO_POLL = 5
const OPT_DEBUG = 6
const OPT_ORDER = 7
const OPT_NO_RESOLV = 8
const OPT_EXPAND = 9
const OPT_LOCALMX = 10
const OPT_NO_NEG = 11
const OPT_NODOTS_LOCAL = 12
const OPT_NOWILD = 13
const OPT_ETHERS = 14
const OPT_RESOLV_DOMAIN = 15
const OPT_NO_FORK = 16
const OPT_AUTHORITATIVE = 17
const OPT_LOCALISE = 18
const OPT_DBUS = 19
const OPT_DHCP_FQDN = 20
const OPT_NO_PING = 21
const OPT_LEASE_RO = 22
const OPT_ALL_SERVERS = 23
const OPT_RELOAD = 24
const OPT_LOCAL_REBIND = 25
const OPT_TFTP_SECURE = 26
const OPT_TFTP_NOBLOCK = 27
const OPT_LOG_OPTS = 28
const OPT_TFTP_APREF_IP = 29
const OPT_NO_OVERRIDE = 30
const OPT_NO_REBIND = 31
const OPT_ADD_MAC = 32
const OPT_DNSSEC_PROXY = 33
const OPT_CONSEC_ADDR = 34
const OPT_CONNTRACK = 35
const OPT_FQDN_UPDATE = 36
const OPT_RA = 37
const OPT_TFTP_LC = 38
const OPT_CLEVERBIND = 39
const OPT_TFTP = 40
const OPT_CLIENT_SUBNET = 41
const OPT_QUIET_DHCP = 42
const OPT_QUIET_DHCP6 = 43
const OPT_QUIET_RA = 44
const OPT_DNSSEC_VALID = 45
const OPT_DNSSEC_TIME = 46
const OPT_DNSSEC_DEBUG = 47
const OPT_DNSSEC_IGN_NS = 48
const OPT_LOCAL_SERVICE = 49
const OPT_LOOP_DETECT = 50
const OPT_EXTRALOG = 51
const OPT_TFTP_NO_FAIL = 52
const OPT_SCRIPT_ARP = 53
const OPT_MAC_B64 = 54
const OPT_MAC_HEX = 55
const OPT_TFTP_APREF_MAC = 56
const OPT_RAPID_COMMIT = 57
const OPT_UBUS = 58
const OPT_LAST = 59

/* extra flags for my_syslog, we use a couple of facilities since they are known
   not to occupy the same bits as priorities, no matter how syslog.h is set up. */
// TODO: define LOG_USER
// const MS_TFTP = LOG_USER
// TODO: define LOG_DAEMON
// const MS_DHCP = LOG_DAEMON
// TODO: define LOG_MAIL
// const MS_SCRIPT = LOG_MAIL

type all_addr_log struct {
	keytag uint16
	algo   uint16
	digest uint16
}

type all_addr_rcode struct {
	rcode uint32
}

type all_addr_dnssec struct {
	_class uint16
	_type  uint16
}

type all_addr_addr struct {
	addr4  in_addr
	addr6  in6_addr
	log    all_addr_log
	rcode  all_addr_rcode
	dnssec all_addr_dnssec
}

type all_addr struct {
	addr all_addr_addr
}

type bogus_addr struct {
	addr in_addr
	// next: &bogus_addr,;
}

/* dns doctor param */
type doctor struct {
	_in  in_addr
	end  in_addr
	out  in_addr
	mask in_addr
	// next: doctor*
}

type mx_srv_record struct {
	name     string
	target   string
	issrv    int32
	srvport  int32
	priority int32
	weight   int32
	offset   uint
	// next: mx_srv_record
}

type naptr struct {
	name     string
	replace  string
	regexp   string
	services string
	flages   string
	pref     string
	order    string
	// next: naptr*
}

const TXT_STAT_CACHESIZE = 1
const TXT_STAT_INSERTS = 2
const TXT_STAT_EVICTIONS = 3
const TXT_STAT_MISSES = 4
const TXT_STAT_HITS = 5
const TXT_STAT_AUTH = 6
const TXT_STAT_SERVERS = 7

type txt_record struct {
	name   string
	txt    string
	_class uint16
	len    uint16
	stat   int32
	// next: txt_record*
}

type ptr_record struct {
	name string
	ptr  string
	// next: ptr_record*
}

type cname struct {
	flag   int32
	ttl    int32
	alias  string
	target string
	// next: cname*
	// targetp: cname*
}

type ds_config struct {
	name        string
	digest      string
	digestlen   int32
	_class      int32
	algo        int32
	keytag      int32
	digest_type int32
	// next: ds_config*
}

const ADDRLIST_LITERAL = 1
const ADDRLIST_IPV6 = 2
const ADDRLIST_REVONLY = 4

type addrlist struct {
	addr      all_addr
	flags     int32
	prefixlen int32
	// next: addrlist*
}

const AUTH6 = 1
const AUTH4 = 2

type auth_name_list struct {
	name  string
	flags int32
	// next: auth_name_list*
}

type auth_zone struct {
	domain          string
	interface_names []auth_name_list
	subnet          addrlist
	exclude         addrlist
	//next: auth_zone*
}

type name_list struct {
	name string
	// next: name_list*
}

type host_record struct {
	ttl   int32
	names []name_list
	addr  in_addr
	addr6 in6_addr
	// next: host_record*
}

type interface_name struct {
	name   string
	intr   string
	family int32
	// TODO: should this be a list of addrlist structs or a pointer to one?
	addr []addrlist
	// next: interface_name*
}

type bigname struct {
	name string
	// next: bigname*
}

type blockdata struct {
	// next blockdata*
	key [KEYBLOCK_LEN]byte
}

type crec_target struct {
	// TODO: should cache and int_name be vectors or just pointers?
	cache    []crec
	int_name []interface_name
}

type crec_cname struct {
	target crec_target
	uid    uint32
}

type crec_key struct {
	keydata []blockdata
	keylen  uint16
	flags   uint16
	keytag  uint16
	algo    uint8
}

type crec_ds struct {
	keydata []blockdata
	keylen  uint16
	keytag  uint16
	algo    uint8
	digest  uint8
}

type crec_addr struct {
	addr  all_addr
	cname crec_cname
	key   crec_key
	ds    crec_ds
}

type crec_name struct {
	sname string
	bname bigname
	namep String
}

type crec struct {
	// hash_next: crec*
	// prev: crec*
	// next: crec*
	addr  crec_addr
	ttd   time_t
	uid   uint32
	flags uint16
	name  crec_name
}

// TODO: implement
// const SIZEOF_BARE_CREC = (sizeof(struct crec) - SMALLDNAME)
// TODO: implement
// const SIZEOF_POINTER_CREC = (sizeof(struct crec) + sizeof(char *) - SMALLDNAME)

const F_IMMORTAL = 1
const F_NAMEP = 2
const F_REVERSE = 4
const F_FORWARD = 8
const F_DHCP = 16
const F_NEG = 32
const F_HOSTS = 64
const F_IPV4 = 128
const F_IPV6 = 256
const F_BIGNAME = 512
const F_NXDOMAIN = 1024
const F_CNAME = 2048
const F_DNSKEY = 4096
const F_CONFIG = 8192
const F_DS = 16384
const F_DNSSECOK = 32768

/* below here are only valid as args to log_query: cache
   entries are limited to 16 bits */
const F_UPSTREAM = 65536
const F_RRNAME = 131072
const F_SERVER = 262144
const F_QUERY = 524288
const F_NOERR = 1048576
const F_AUTH = 2097152
const F_DNSSEC = 4194304
const F_KEYTAG = 8388608
const F_SECSTAT = 16777216
const F_NO_RR = 33554432
const F_IPSET = 67108864
const F_NOEXTRA = 134217728
const F_SERVFAIL = 268435456
const F_RCODE = 536870912

const UID_NONE = 0

/* Values of uid in crecs with F_CONFIG bit set. */
/* cname to uid SRC_INTERFACE are to interface names
   so use UID_NONE for that to eliminate clashes with
   any other uid */
const SRC_INTERFACE = UID_NONE
const SRC_CONFIG = 1
const SRC_HOSTS = 2
const SRC_AH = 3

/* struct sockaddr is not large enough to hold any address
   and specifically not big enough to hold an IPv6 address.
   Blech. Roll our own. */
type mysockaddr struct {
	sa  sockaddr
	in4 sockaddr_in
	in6 sockaddr_in6
}

/* bits in flag param to IPv6 callbacks from iface_enumerate() */
const IFACE_TENTATIVE = 1
const IFACE_DEPRECATED = 2
const IFACE_PERMANENT = 4

const SERV_FROM_RESOLV = 1       /* 1 for servers from resolv, 0 for command line. */
const SERV_NO_ADDR = 2           /* no server, this domain is local only */
const SERV_LITERAL_ADDRESS = 4   /* addr is the answer, not the server */
const SERV_HAS_DOMAIN = 8        /* server for one domain only */
const SERV_HAS_SOURCE = 16       /* source address defined */
const SERV_FOR_NODOTS = 32       /* server for names with no domain part only */
const SERV_WARNED_RECURSIVE = 64 /* avoid warning spam */
const SERV_FROM_DBUS = 128       /* 1 if source is DBus */
const SERV_MARK = 256            /* for mark-and-delete */
const SERV_TYPE = (SERV_HAS_DOMAIN | SERV_FOR_NODOTS)
const SERV_COUNTED = 512     /* workspace for log code */
const SERV_USE_RESOLV = 1024 /* forward this domain in the normal way */
const SERV_NO_REBIND = 2048  /* inhibit dns-rebind protection */
const SERV_FROM_FILE = 4096  /* read from --servers-file */
const SERV_LOOP = 8192       /* server causes forwarding loop */
const SERV_DO_DNSSEC = 16384 /* Validate DNSSEC when using this server */
const SERV_GOT_TCP = 32768   /* Got some data from the TCP connection */

type ServerFd struct {
	fd           int32
	source_addr  mysockaddr
	_interface   string
	preallocated uint32
	used         uint32
	ifindex      uint32
	// next: ServerFd*
}

type randfd struct {
	fd       int32
	refcount uint16
	family   uint16
}

type server struct {
	addr           mysockaddr
	source_addr    mysockaddr
	_interface     string
	sfd            ServerFd
	domain         string
	flags          int32
	tcpfd          int32
	edns_pktsz     int32
	pktsz_reduced  time_t
	queries        uint32
	failed_queries uint32
	uid            uint32
	// next: server*
}

type ipsets struct {
	sets   []string
	domain string
	// next: ipsets*
}

type irec struct {
	addr           mysockaddr
	netmask        in_addr
	tftp_ok        int32
	dhcp_ok        int32
	mtu            int32
	done           int32
	warned         int32
	dad            int32
	dns_auth       int32
	index          int32
	multicast_done int32
	found          int32
	label          int32
	name           string
	// next: irec*
}

type listener struct {
	fd     int32
	tcpfd  int32
	tftpfd int32
	family int32
	iface  irec // TODO: ptr?
	// next: listener*
}

/* interface and address parms from command line. */
type Iname struct {
	name string
	addr mysockaddr
	used int32
	// next: Iname*
}

/* subnet parameters from command line */
type mysubnet struct {
	addr      mysockaddr
	addr_used int32
	mask      int32
}

/* resolv-file parms from command-line */
type resolvc struct {
	// next: resolvc*
	is_default int32
	logged     int32
	mtime      time_t
	name       string
	wd         int32
	file       string
}

/* adn-hosts parms from command-line (also dhcp-hostsfile and dhcp-optsfile and dhcp-hostsdir*/
const AH_DIR = 1
const AH_INACTIVE = 2
const AH_WD_DONE = 4
const AH_HOSTS = 8
const AH_DHCP_HST = 16
const AH_DHCP_OPT = 32

type hostsfile struct {
	// next: hostsfile*
	flags int32
	fname string
	wd    int32
	index uint32
}

/* packet-dump flags */
const DUMP_QUERY = 0x0001
const DUMP_REPLY = 0x0002
const DUMP_UP_QUERY = 0x0004
const DUMP_UP_REPLY = 0x0008
const DUMP_SEC_QUERY = 0x0010
const DUMP_SEC_REPLY = 0x0020
const DUMP_BOGUS = 0x0040
const DUMP_SEC_BOGUS = 0x0080

/* DNSSEC status values. */
const STAT_SECURE = 1
const STAT_INSECURE = 2
const STAT_BOGUS = 3
const STAT_NEED_DS = 4
const STAT_NEED_KEY = 5
const STAT_TRUNCATED = 6
const STAT_SECURE_WILDCARD = 7
const STAT_OK = 8
const STAT_ABANDONED = 9

const FREC_NOREBIND = 1
const FREC_CHECKING_DISABLED = 2
const FREC_HAS_SUBNET = 4
const FREC_DNSKEY_QUERY = 8
const FREC_DS_QUERY = 16
const FREC_AD_QUESTION = 32
const FREC_DO_QUESTION = 64
const FREC_ADDED_PHEADER = 128
const FREC_TEST_PKTSZ = 256
const FREC_HAS_EXTRADATA = 512

const HASH_SIZE_DNSSEC = 20 /* SHA-1 digest size */
const HASH_SIZE = 32

type frec struct {
	source       mysockaddr
	dest         all_addr
	sentto       Server // TODO ptr?
	rfd64        randfd // TODO: ptr?
	rfd6         randfd // TODO: ptr?
	iface        uint32
	orig_id      uint32
	new_id       uint32
	log_id       int32
	fd           int32
	forwardall   int32
	flags        int32
	time         time_t
	hash         [HASH_SIZE]u8
	_class       int32
	work_counter int32
	stash        []blockdata
	stash_len    usize
	// dependent: frec, // TODO: ptr?
	// blocking_query: frec // TODO: ptr?
	// next: frec*
}

/* flags in top of length field for DHCP-option tables */
const OT_ADDR_LIST = 0x8000
const OT_RFC1035_NAME = 0x4000
const OT_INTERNAL = 0x2000
const OT_NAME = 0x1000
const OT_CSTRING = 0x0800
const OT_DEC = 0x0400
const OT_TIME = 0x0200

/* actions in the daemon->helper RPC */
const ACTION_DEL = 1
const ACTION_OLD_HOSTNAME = 2
const ACTION_OLD = 3
const ACTION_ADD = 4
const ACTION_TFTP = 5
const ACTION_ARP = 6
const ACTION_ARP_DEL = 7

const LEASE_NEW = 1           /* newly created */
const LEASE_CHANGED = 2       /* modified */
const LEASE_AUX_CHANGED = 4   /* CLID or expiry changed */
const LEASE_AUTH_NAME = 8     /* hostname came from config, not from client */
const LEASE_USED = 16         /* used this DHCPv6 transaction */
const LEASE_NA = 32           /* IPv6 no-temporary lease */
const LEASE_TA = 64           /* IPv6 temporary lease */
const LEASE_HAVE_HWADDR = 128 /* Have set hwaddress */

type slaac_address struct {
	addr      in6_addr
	ping_time time_t
	backoff   int32
	// next: slaac_address*
}

type dhcp_lease struct {
	clid_len          int32
	clid              []u8
	hostname          string
	fqdn              string
	old_hostname      string
	flags             int32
	expires           time_t
	length            uint32
	hwaddr_len        int32
	hwaddr_type       int32
	hwaddr            [DHCP_CHADDR_MAX]u8
	addr              in_addr
	_override         in_addr
	giaddr            in_addr
	extradata         []u8
	extradata_len     uint32
	extradata_size    uint32
	last_interface    int32
	new_interface     int32
	new_prefixlen     int32
	addr6             in6_addr
	iaid              int32
	slaac_addr        slaac_address
	vendorclass_count int32
	// next: dhcp_lease*
}

type dhcp_netid struct {
	net string
	// next: dhcp_netid*
}

type dhcp_netid_list struct {
	list []dhcp_netid
	// next: dhcp_netid_list
}

type tag_if struct {
	set dhcp_netid_list
	tag dhcp_netid // TODO: ptr,
	// next: tag_if*
}

type delay_config struct {
	delay int32
	netid dhcp_netid // todo ptr
	// next: delay_config*
}

type hwaddr_config struct {
	hwaddr_len    int32
	hwaddr_type   int32
	hwaddr        [DHCP_CHADDR_MAX]byte
	wildcard_mask uint32
	// next: hwaddr_config*
}

type dhcp_config struct {
	flags        uint32
	clid_len     int32
	clid         []byte
	hostname     string
	domain       string
	netid        dhcp_netid_list
	addr6        in6_addr
	addr         in_addr
	decline_time time_t
	lease_time   uint32
	hwaddr       hwaddr_config
	// next: dhcp_config*
}

// todo: re-implement
//const have_config :u32 = ;(config, mask) ((config) && ((config)->flags & (mask)))

const CONFIG_DISABLE = 1
const CONFIG_CLID = 2
const CONFIG_TIME = 8
const CONFIG_NAME = 16
const CONFIG_ADDR = 32
const CONFIG_NOCLID = 128
const CONFIG_FROM_ETHERS = 256 /* entry created by /etc/ethers */
const CONFIG_ADDR_HOSTS = 512  /* address added by from /etc/hosts */
const CONFIG_DECLINED = 1024   /* address declined by client */
const CONFIG_BANK = 2048       /* from dhcp hosts file */
const CONFIG_ADDR6 = 4096
const CONFIG_WILDCARD = 8192

type dhcp_opt_u struct {
	encap         int32
	wildcard_mask uint32
	vendor_class  []byte
}

type dhcp_opt struct {
	opt   int32
	len   int32
	flags int32
	u     dhcp_opt_u
	val   []byte
	netid dhcp_netid
	// next: dhcp_opt,
}

const DHOPT_ADDR = 1
const DHOPT_STRING = 2
const DHOPT_ENCAPSULATE = 4
const DHOPT_ENCAP_MATCH = 8
const DHOPT_FORCE = 16
const DHOPT_BANK = 32
const DHOPT_ENCAP_DONE = 64
const DHOPT_MATCH = 128
const DHOPT_VENDOR = 256
const DHOPT_HEX = 512
const DHOPT_VENDOR_MATCH = 1024
const DHOPT_RFC3925 = 2048
const DHOPT_TAGOK = 4096
const DHOPT_ADDR6 = 8192

type dhcp_boot struct {
	file        string
	sname       string
	tftp_sname  string
	next_server in_addr
	netid       dhcp_netid
	// next: dhcp_boot*
}

type dhcp_match_name struct {
	name     string
	wildcard int32
	netid    dhcp_netid
	// next: dhcp_match_name*
}

type pxe_service struct {
	CSA      uint16
	_type    uint16
	menu     string
	basename string
	sname    string
	server   in_addr
	netid    dhcp_netid
	// next: pxe_service*
}

const MATCH_VENDOR = 1
const MATCH_USER = 2
const MATCH_CIRCUIT = 3
const MATCH_REMOTE = 4
const MATCH_SUBSCRIBER = 5

/* vendorclass, userclass, remote-id or circuit-id */
type dhcp_vendor struct {
	len        int32
	match_type int32
	enterprise uint32
	data       string
	netid      dhcp_netid
	// next: dhcp_vendor*
}

type dhcp_mac struct {
	mask        uint32
	hwaddr_len  int32
	hwaddr_type int32
	hwaddr      [DHCP_CHADDR_MAX]byte
	netid       dhcp_netid
	// next: dhcp_mac*
}

type dhcp_bridge struct {
	iface string
	// alias: dhcp_bridge*
	// next: dhcp_bridge*
}

type cond_domain struct {
	domain  string
	prefix  string
	start   in_addr
	end     in_addr
	start6  in6_addr
	end6    in6_addr
	is6     int32
	indexed int32
	// next: cond_domain*
}

// #ifdef OPTION6_PREFIX_CLASS
type prefix_class struct {
	_class int32
	tag    dhcp_netid
	// next: prefix_class*
}

// #endif

type ra_interface struct {
	name     string
	mtu_name string
	interval int32
	lifetime int32
	prio     int32
	mtu      int32
	// next: ra_interface*
}

type DhcpContext struct {
	lease_time            uint32
	addr_epoch            uint32
	netmask               in_addr
	broadcast             in_addr
	local                 in_addr
	router                in_addr
	start                 in_addr
	end                   in_addr
	start6                in6_addr
	end6                  in6_addr
	local6                in6_addr
	prefix                int32
	if_index              int32
	valid                 uint32
	preferred             uint32
	saved_valid           uint32
	ra_time               time_t
	ra_short_period_start time_t
	address_list_time     time_t
	template_interface    string
	flags                 int32
	netid                 dhcp_netid
	filter                dhcp_netid
	// next: DhcpContext
	// current: DhcpContext
}

const CONTEXT_STATIC = 1
const CONTEXT_NETMASK = 2
const CONTEXT_BRDCAST = 4
const CONTEXT_PROXY = 8
const CONTEXT_RA_ROUTER = 16
const CONTEXT_RA_DONE = 32
const CONTEXT_RA_NAME = 64
const CONTEXT_RA_STATELESS = 128
const CONTEXT_DHCP = 256
const CONTEXT_DEPRECATE = 512
const CONTEXT_TEMPLATE = 1024 /* create contexts using addresses */
const CONTEXT_CONSTRUCTED = 2048
const CONTEXT_GC = 4096
const CONTEXT_RA = 8192
const CONTEXT_CONF_USED = 16384
const CONTEXT_USED = 32768
const CONTEXT_OLD = 65536
const CONTEXT_V6 = 131072
const CONTEXT_RA_OFF_LINK = 262144

type ping_result struct {
	addr in_addr
	time time_t
	hash uint32
	// next: ping_result*
}

type tftp_file struct {
	refcount int32
	fd       int32
	size     off_t
	dev      dev_t
	inode    ino_t
	filename string
}

type tftp_transfer struct {
	sockfd        int32
	timeout       time_t
	backoff       int32
	expansion     uint32
	blocksize     uint32
	block         uint32
	peer          mysockaddr
	carrylf       uint8
	netascii      uint8
	opt_transize  uint8
	opt_blocksize uint8
	file          tftp_file
	// enxt: tftp_transfer
}

type AddrList struct {
	addr in_addr
	// next: AddrList*
}

type TftpPrefix struct {
	_interface string
	prefix     string
	missing    int32
	// next: TftpPrefix*
}

type DhcpRelay struct {
	server      all_addr
	local       all_addr
	_interface  string
	iface_index int32
	// next: DhcpRelay*
	// current: DhcpRelay*
}

type daemon struct {
	/* datastuctures representing the command-line and
	   config file arguments. All set (including defaults)
	   in option.c */
	options2 uint32
	/* datastuctures representing the command-line and
	   config file arguments. All set (including defaults)
	   in option.c */
	options        uint32
	resolv_files   resolvc // todo: ptr?
	default_resolv resolvc // todo: ptr?

	//struct resolvc* resolv_files;
	resolv_files []resolvc
	//struct resolvc default_resolv;
	default_resolv           []resolvc
	last_resolv              time_t
	servers_file             string
	mxnames                  []mx_srv_record
	naptr                    []naptr
	rr                       []txt_record
	txt                      []txt_record
	ptr                      []ptr_record
	host_records_tail        []host_record
	host_records             []host_record
	cnames                   []string
	auth_zones               []auth_zones
	int_names                []interface_names
	mxtarget                 string
	add_subnet4              []mysubnet
	add_subnet6              []mysubnet
	lease_file               string
	scriptuser               string
	groupname                string
	username                 string
	luascript                string
	hostmaster               string
	authserver               string
	authinterface            []Iname
	secondary_forward_server []name_list
	group_set                int32
	osport                   int32
	domain_suffix            string
	synth_domains            []cond_domain
	cond_domain              []cond_domain
	runfile                  string
	lease_change_command     string
	if_names                 []Iname
	if_addrs                 []Iname
	if_except                []Iname
	dhcp_except              []Iname
	auth_peers               []Iname
	tftp_interfaces          []Iname
	bogus_addr               []bogus_addr
	ignore_addr              []bogus_addr
	servers                  []server
	ipsets                   []ipsets
	log_fac                  int32
	log_file                 string
	max_logs                 int32
	cachesize                int32
	ftabsize                 int32
	port                     int32
	query_port               int32
	min_port                 int32
	max_port                 int32
	local_ttl                uint32
	neg_ttl                  uint32
	max_ttl                  uint32
	min_cache_ttl            uint32
	max_cache_ttl            uint32
	auth_ttl                 uint32
	dhcp_ttl                 uint32
	use_dhcp_ttl             uint32
	dns_client_id            string
	addn_hosts               []hostsfile
	dhcp                     []DhcpContext
	dhcp6                    []DhcpContext
	ra_interfaces            []ra_interface
	dhcp_conf                []dhcp_config
	dhcp_opts                []dhcp_opt
	dhcp_match               []dhcp_opt
	dhcp_opts6               []dhcp_opt
	dhcp_match6              []dhcp_opt
	dhcp_name_match          []dhcp_match_name
	dhcp_vendors             []dhcp_vendor
	dhcp_macs                []dhcp_mac
	boot_config              []dhcp_boot
	pxe_services             []pxe_service
	tag_if                   []tag_if
	override_relays          []AddrList
	relay4                   DhcpRelay
	realy6                   Dhcp6Relay
	delay_conf               delay_config
	override                 int32
	enable_pxe               int32
	doing_ra                 int32
	doing_dhcp6              int32
	dhcp_ignore              []dhcp_netid_list
	dhcp_ignore_names        []dhcp_netid_list
	dhcp_gen_names           []dhcp_netid_list
	force_broadcast          []dhcp_netid_list
	bootp_dynamic            []dhcp_netid_list
	dhcp_hosts_file          []hostsfile
	dhcp_opts_file           []hostsfile
	dynamic_dirs             []hostsfile
	dhcp_max                 int32
	tftp_max                 int32
	tftp_mtu                 int32
	dhcp_server_port         int32
	dhcp_client_port         int32
	start_tftp_port          int32
	end_tftp_port            int32
	min_leasetime            int32
	doctors                  []doctor
	edns_pktsz               uint16
	tftp_prefix              string
	if_prefix                TftPrefix
	duid_enterprise          uint32
	duid_config_len          uint32
	duid_config              []byte
	dbus_name                string
	dump_file                string
	dump_mask                int32
	soa_sn                   uint32
	soa_refresh              uint32
	soa_retry                uint32
	soa_expiry               uint32
	metrics                  []uint32
	prefix_classes           []prefix_class
	ds                       ds_config
	timestamp_file           string
	packet                   String
	packet_buff_sz           int32
	namebuff                 string
	keyname                  string
	workspacename            string
	rr_status                string
	rr_status_sz             int32
	dnsssec_no_time_check    int32
	back_to_the_future       int32
	frec_list                []frec
	sfds                     []ServerFd
	interfaces               []irec
	listeners                []listener
	last_server              []server
	forwardtime              time_t
	forwardcount             int32
	srv_save                 server
	packet_len               uint32
	rfd_save                 randfd
	tcp_pids                 []pid_t
	randomsocks              []randfd
	v6pktinfo                int32
	interface_addrs          []addrlist
	log_id                   int32
	log_display_id           int32
	log_soruce_addr          mysockaddr
	pxefd                    int32
	helperfd                 int32
	dhcpfd                   int32
	inotifyfd                int32
	netlinkfd                int32
	dhcp_raw_fd              int32
	dhcp_icmp_fd             int32
	routefd                  int32
	dhcp_packet              iovec
	dchp_buff                string
	dhcp_buff2               string
	dhcp_buff3               string
	ping_results             []pint_result
	lease_stream             file
	bridges                  []dhcp_bridge
	duid_len                 int32
	duid                     []byte
	outpacket                iovec
	dhcp6fd                  int32
	icmp6fd                  int32
	dbus                     void_ptr
	watches                  []watch
	tftp_trans               []tftp_transfer
	tftp_done_trans          []tftp_transfer
	addrbuff                 string
	addrbuff2                string
	dumpfd                   int32
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

// #endif

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
// #ifdef HAVE_DNSSEC
// void blockdata_init(void);
// void blockdata_report(void);
// struct blockdata *blockdata_alloc(char *data, size_t len);
// void *blockdata_retrieve(struct blockdata *block, size_t len, void *data);
// void blockdata_free(struct blockdata *blocks);
// #endif

// /* domain.c */
// char*
// get_domain(struct in_addr addr);

// #ifdef HAVE_IPV6

// char*
// get_domain6(struct in6_addr* addr);

// #endif

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
// #ifdef HAVE_AUTH

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

// #endif

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

// #ifdef HAVE_IPV6

// int
// is_same_net6(struct in6_addr* a, struct in6_addr* b, int prefixlen);

// uint64_t
// addr6part(struct in6_addr* addr);

// void
// setaddr6part(struct in6_addr* addr, uint64_t host);

// #endif

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

// #ifdef HAVE_IPV6

// int
// set_ipv6pktinfo(int fd);

// #endif
// #ifdef HAVE_DHCP6

// void
// join_multicast(int dienow);

// #endif
// #if defined(HAVE_LINUX_NETWORK) || defined(HAVE_BSD_NETWORK)

// void
// newaddress(time_t now);

// #endif

// /* dhcp.c */
// #ifdef HAVE_DHCP

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

// #endif

// /* lease.c */
// #ifdef HAVE_DHCP

// void
// lease_update_file(time_t now);

// void
// lease_update_dns(int force);

// void
// lease_init(time_t now);

// struct dhcp_lease*
// lease4_allocate(struct in_addr addr);

// #ifdef HAVE_DHCP6

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

// #endif

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

// #ifdef HAVE_SCRIPT

// void
// lease_add_extradata(struct dhcp_lease* lease,
//         unsigned char* data,
//         uint32_t len,
//         int delim);

// #endif
// #endif

// /* rfc2131.c */
// #ifdef HAVE_DHCP

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

// #endif

// /* dnsmasq.c */
// #ifdef HAVE_DHCP

// int
// make_icmp_sock();

// int
// icmp_ping(struct in_addr addr);

// int
// delay_dhcp(time_t start, int sec, int fd, uint32_t addr, uint16_t id);

// #endif

// void
// queue_event(int event);

// void
// send_alarm(time_t event, time_t now);

// void
// send_event(int fd, int event, int data, char* msg);

// void
// clear_cache_and_reload(time_t now);

// /* netlink.c */
// #ifdef HAVE_LINUX_NETWORK

// void
// netlink_init();

// void
// netlink_multicast();

// #endif

// /* bpf.c */
// #ifdef HAVE_BSD_NETWORK
// void init_bpf(void);
// void send_via_bpf(struct dhcp_packet *mess, size_t len,
//           struct in_addr iface_addr, struct ifreq *ifr);
// void route_init(void);
// void route_sock(void);
// #endif

// /* bpf.c or netlink.c */
// using callback_t = int(*)(struct in_addr*, int, int, int, int, int, int, void*);

// int
// iface_enumerate(int family, void* parm, callback_t callback);

// /* dbus.c */
// #ifdef HAVE_DBUS
// char *dbus_init(void);
// void check_dbus_listeners(void);
// void set_dbus_listeners(void);
// #  ifdef HAVE_DHCP
// void emit_dbus_signal(int action, struct dhcp_lease *lease, char *hostname);
// #  endif
// #endif

// /* ubus.c */
// #ifdef HAVE_UBUS
// void set_ubus_listeners(void);
// void check_ubus_listeners(void);
// void ubus_event_bcast(const char *type, const char *mac, const char *ip, const char *name, const char *interface);
// #endif

// /* ipset.c */
// #ifdef HAVE_IPSET

// void
// ipset_init();

// int
// add_to_ipset(const char* setname, const struct all_addr* ipaddr, int flags, int remove);

// #endif

// /* helper.c */
// #if defined(HAVE_SCRIPT)

// int
// create_helper(int event_fd, int err_fd, uid_t uid, gid_t gid, long max_fd);

// void
// helper_write();

// void
// queue_script(int action, struct dhcp_lease* lease, char* hostname, time_t now);

// #ifdef HAVE_TFTP

// void
// queue_tftp(off_t file_len, char* filename, union mysockaddr* peer);

// #endif

// void
// queue_arp(int action, unsigned char* mac, int maclen, int family, struct all_addr* addr);

// int
// helper_buf_empty();

// #endif

// /* tftp.c */
// #ifdef HAVE_TFTP

// void
// tftp_request(struct listener* listen, time_t now);

// void
// check_tftp_listeners(time_t now);

// int
// do_tftp_script_run();

// #endif

// /* conntrack.c */
// #ifdef HAVE_CONNTRACK
// int get_incoming_mark(union mysockaddr *peer_addr, struct all_addr *local_addr,
//               int istcp, uint32_t *markp);
// #endif

// /* dhcp6.c */
// #ifdef HAVE_DHCP6

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

// #endif

// /* rfc3315.c */
// #ifdef HAVE_DHCP6

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

// #endif

// /* dhcp-common.c */
// #ifdef HAVE_DHCP

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

// #ifdef HAVE_LINUX_NETWORK

// char*
// whichdevice();

// void
// bindtodevice(char* device, int fd);

// #endif
// #  ifdef HAVE_DHCP6

// void
// display_opts6();

// #  endif

// void
// log_context(int family, struct DhcpContext* context);

// void
// log_relay(int family, struct DhcpRelay* relay);

// #endif

// /* outpacket.c */
// #ifdef HAVE_DHCP6

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

// #endif

// /* radv.c */
// #ifdef HAVE_DHCP6

// void
// ra_init(time_t now);

// void
// icmp6_packet(time_t now);

// time_t
// periodic_ra(time_t now);

// void
// ra_start_unsolicited(time_t now, struct DhcpContext* context);

// #endif

// /* slaac.c */
// #ifdef HAVE_DHCP6

// void
// slaac_add_addrs(struct dhcp_lease* lease, time_t now, int force);

// time_t
// periodic_slaac(time_t now, struct dhcp_lease* leases);

// void
// slaac_ping_reply(struct in6_addr* sender,
//         unsigned char* packet,
//         char* interface,
//         struct dhcp_lease* leases);

// #endif

// /* loop.c */
// #ifdef HAVE_LOOP

// void
// loop_send_probes();

// int
// detect_loop(char* query, int type);

// #endif

// /* inotify.c */
// #ifdef HAVE_INOTIFY

// void
// inotify_dnsmasq_init();

// int
// inotify_check(time_t now);

// void
// set_dynamic_inotify(int flag, int total_size, struct crec** rhash, int revhashsz);

// #endif

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
// #ifdef HAVE_DUMPFILE

// void
// dump_init();

// void
// dump_packet(int mask,
//         void* packet,
//         size_t len,
//         union mysockaddr* src,
//         union mysockaddr* dst);

// #endif

//
// END OF FILE
//
