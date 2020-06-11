mod util;

use log::{debug, error, info};

type dmq_pid_t = i32;
type dmq_in_addr_t = u32;

#[derive(Debug,Copy,Clone)]
pub struct DmqInAddr {
    pub s_addr: dmq_in_addr_t,
}

#[derive(Debug,Copy,Clone)]
pub struct DmqIn6Addr {
    pub s6_addr: [u8; 16],
}

#[derive(Debug,Copy,Clone)]
pub struct dnsmasq_daemon {

}

#[derive(Debug,Copy,Clone)]
pub struct event_desc {
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
