use socket2::Socket;
use crate::slack::{__caddr_t, ifmap};

pub type __dev_t = libc::c_ulong;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = u32;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = i32;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __blkcnt64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = u32;
pub type ino_t = __ino64_t;
pub type dev_t = __dev_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;

pub const NULL: i32 = 0 as i32;
pub const NULL_0: i32 = 0 as i32;

#[derive(Copy,Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[derive(Copy,Clone, Default)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: libc::size_t,
}

pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [i8; 14],
}

#[derive(Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name: Vec<u8>,
    pub msg_namelen: socklen_t,
    pub msg_iov: iovec,
    pub msg_iovlen: usize,
    pub msg_buf: Vec<u8>,
    pub msg_control: Vec<u8>,
    pub msg_controllen: usize,
    pub msg_flags: i32,
}

#[derive(Clone)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
    pub __cmsg_data: [u8; 0],
}

// #[inline]
// pub unsafe extern "C" fn __cmsg_nxthdr(
//     mut __mhdr: *mut msghdr,
//     mut __cmsg: *mut cmsghdr,
// ) -> *mut cmsghdr {
//     if (*__cmsg).cmsg_len < ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
//         return 0 as *mut cmsghdr;
//     }
//     __cmsg = (__cmsg as *mut u8).offset(
//         ((*__cmsg)
//             .cmsg_len
//             .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
//             .wrapping_sub(1 as i32 as libc::c_ulong)
//             & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
//                 .wrapping_sub(1 as i32 as libc::c_ulong)) as isize,
//     ) as *mut cmsghdr;
//     if __cmsg.offset(1 as i32 as isize) as *mut u8
//         > ((*__mhdr).msg_control as *mut u8).offset((*__mhdr).msg_controllen as isize)
//         || (__cmsg as *mut u8).offset(
//             ((*__cmsg)
//                 .cmsg_len
//                 .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
//                 .wrapping_sub(1 as i32 as libc::c_ulong)
//                 & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
//                     .wrapping_sub(1 as i32 as libc::c_ulong)) as isize,
//         ) > ((  `   *__mhdr).msg_control as *mut u8)
//             .offset((*__mhdr).msg_controllen as isize)
//     {
//         return 0 as *mut cmsghdr;
//     }
//     return __cmsg;
// }

pub const AF_UNSPEC: i32 = PF_UNSPEC;
pub const PF_UNSPEC: i32 = 0;
pub const AF_INET6: i32 = PF_INET6;
pub const PF_INET6: i32 = 10;
pub const AF_INET: i32 = PF_INET;
pub const PF_INET: i32 = 2;

pub type sa_family_t = u16;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [u8; 16],
    pub __u6_addr16: [u16; 8],
    pub __u6_addr32: [u32; 4],
}

pub type in_port_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

pub const INET6_ADDRSTRLEN: i32 = 46 as i32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union all_addr {
    pub addr4: in_addr,
    pub addr6: in6_addr,
    pub cname: C2RustUnnamed_4,
    pub key: C2RustUnnamed_3,
    pub ds: C2RustUnnamed_2,
    pub srv: C2RustUnnamed_1,
    pub log: C2RustUnnamed_0,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub target: blockdata,
    pub targetlen: u16,
    pub srvport: u16,
    pub priority: u16,
    pub weight: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockdata {
    // pub next: blockdata,
    pub key: [u8; 40],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub target: C2RustUnnamed_7,
    pub uid: u32,
    pub is_name_ptr: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1A {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: libc::c_short,
    pub ifru_ivalue: libc::c_int,
    pub ifru_mtu: libc::c_int,
    pub ifru_map: ifmap,
    pub ifru_slave: [libc::c_char; 16],
    pub ifru_newname: [libc::c_char; 16],
    pub ifru_data: __caddr_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: C2RustUnnamed_2,
    pub ifr_ifru: C2RustUnnamed_1A,
}


#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub cache: *mut crec,
    pub name: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub keydata: blockdata,
    pub keylen: u16,
    pub keytag: u16,
    pub algo: u8,
    pub digest: u8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub target: C2RustUnnamed_5,
    pub uid: u32,
    pub is_name_ptr: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub cache: crec,
    pub name: String,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct crec {
    pub next: *mut crec,
    pub prev: *mut crec,
    pub hash_next: *mut crec,
    pub addr: all_addr,
    pub ttd: time_t,
    pub uid: u32,
    pub flags: u32,
    pub name: C2RustUnnamed_6,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_32 {
    pub sname: [libc::c_char; 50],
    pub bname: *mut bigname,
    pub namep: String,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union bigname {
    pub name: [libc::c_char; 1025],
    pub next: *mut bigname,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct bogus_addr {
    pub addr: in_addr,
    pub next: *mut bogus_addr,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct doctor {
    pub in_0: in_addr,
    pub end: in_addr,
    pub out: in_addr,
    pub mask: in_addr,
    pub next: *mut doctor,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mx_srv_record {
    pub name: String,
    pub target: String,
    pub issrv: i32,
    pub srvport: i32,
    pub priority: i32,
    pub weight: i32,
    pub offset: u32,
    pub next: *mut mx_srv_record,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct naptr {
    pub name: String,
    pub replace: String,
    pub regexp: String,
    pub services: String,
    pub flags: String,
    pub order: u32,
    pub pref: u32,
    pub next: *mut naptr,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct txt_record {
    pub name: String,
    pub txt: *mut u8,
    pub class: u16,
    pub len: u16,
    pub stat: i32,
    pub next: *mut txt_record,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptr_record {
    pub name: String,
    pub ptr: String,
    pub next: *mut ptr_record,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cname {
    pub ttl: i32,
    pub flag: i32,
    pub alias: String,
    pub target: String,
    pub next: *mut cname,
    pub targetp: *mut cname,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrlist {
    pub addr: all_addr,
    pub flags: i32,
    pub prefixlen: i32,
    pub decline_time: time_t,
    pub next: *mut addrlist,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth_zone {
    pub domain: String,
    pub interface_names: *mut auth_name_list,
    pub subnet: *mut addrlist,
    pub exclude: *mut addrlist,
    pub next: *mut auth_zone,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth_name_list {
    pub name: String,
    pub flags: i32,
    pub next: *mut auth_name_list,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct host_record {
    pub ttl: i32,
    pub flags: i32,
    pub names: *mut name_list,
    pub addr: in_addr,
    pub addr6: in6_addr,
    pub next: *mut host_record,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_list {
    pub name: String,
    pub next: *mut name_list,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct interface_name {
    pub name: String,
    pub intr: String,
    pub family: i32,
    pub addr: *mut addrlist,
    pub next: *mut interface_name,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union mysockaddr {
    pub sa: sockaddr,
    pub in_0: sockaddr_in,
    pub in6: sockaddr_in6,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct serverfd {
    pub fd: i32,
    pub source_addr: mysockaddr,
    pub interface: [i8; 17],
    pub ifindex: u32,
    pub used: u32,
    pub preallocated: u32,
    pub next: *mut serverfd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct randfd {
    pub fd: i32,
    pub refcount: u16,
    pub family: u16,
}
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct server {
    pub addr: mysockaddr,
    pub source_addr: mysockaddr,
    pub interface: [i8; 17],
    pub sfd: *mut serverfd,
    pub domain: String,
    pub flags: i32,
    pub tcpfd: i32,
    pub edns_pktsz: i32,
    pub pktsz_reduced: time_t,
    pub queries: u32,
    pub failed_queries: u32,
    pub uid: u32,
    pub next: *mut server,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipsets {
    pub sets: *mut String,
    pub domain: String,
    pub next: *mut ipsets,
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct irec {
    pub addr: mysockaddr,
    pub netmask: in_addr,
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
    pub next: *mut irec,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct listener {
    pub fd: i32,
    pub tcpfd: i32,
    pub tftpfd: i32,
    pub used: i32,
    pub addr: mysockaddr,
    pub iface: irec,
    pub next: *mut listener,
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct iname {
    pub name: String,
    pub addr: mysockaddr,
    pub used: i32,
    pub next: *mut iname,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mysubnet {
    pub addr: mysockaddr,
    pub addr_used: i32,
    pub mask: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct resolvc {
    pub next: *mut resolvc,
    pub is_default: i32,
    pub logged: i32,
    pub mtime: time_t,
    pub name: String,
    pub wd: i32,
    pub file: String,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostsfile {
    pub next: *mut hostsfile,
    pub flags: i32,
    pub fname: String,
    pub wd: i32,
    pub index: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct frec {
    pub frec_src: frec_src,
    pub sentto: server,
    pub rfd4: randfd,
    pub rfd6: randfd,
    pub new_id: u16,
    pub forwardall: i32,
    pub flags: i32,
    pub time: time_t,
    pub hash: Vec<u8>,
    pub next: *mut frec,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct frec_src {
    pub source: mysockaddr,
    pub dest: all_addr,
    pub iface: u32,
    pub log_id: u32,
    pub fd: i32,
    pub orig_id: u16,
    pub next: *mut frec_src,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_netid {
    pub net: String,
    pub next: *mut dhcp_netid,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_netid_list {
    pub list: *mut dhcp_netid,
    pub next: *mut dhcp_netid_list,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tag_if {
    pub set: *mut dhcp_netid_list,
    pub tag: *mut dhcp_netid,
    pub next: *mut tag_if,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct delay_config {
    pub delay: i32,
    pub netid: *mut dhcp_netid,
    pub next: *mut delay_config,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hwaddr_config {
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub hwaddr: Vec<u8>,
    pub wildcard_mask: u32,
    pub next: *mut hwaddr_config,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_config {
    pub flags: u32,
    pub clid_len: i32,
    pub clid: Vec<u8>,
    pub hostname: String,
    pub domain: String,
    pub netid: dhcp_netid_list,
    pub filter: dhcp_netid,
    pub addr6: addrlist,
    pub addr: in_addr,
    pub decline_time: time_t,
    pub lease_time: u32,
    pub hwaddr: hwaddr_config,
    pub next: *mut dhcp_config,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_opt {
    pub opt: i32,
    pub len: i32,
    pub flags: i32,
    pub u: C2RustUnnamed_7,
    pub val: Vec<u8>,
    pub netid: dhcp_netid,
    pub next: *mut dhcp_opt,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_31 {
    pub encap: i32,
    pub wildcard_mask: u32,
    pub vendor_class: Vec<u8>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_boot {
    pub file: String,
    pub sname: String,
    pub tftp_sname: String,
    pub next_server: in_addr,
    pub netid: dhcp_netid,
    pub next: *mut dhcp_boot,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_match_name {
    pub name: String,
    pub wildcard: i32,
    pub netid: dhcp_netid,
    pub next: *mut dhcp_match_name,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pxe_service {
    pub CSA: u16,
    pub type_0: u16,
    pub menu: String,
    pub basename: String,
    pub sname: String,
    pub server: in_addr,
    pub netid: dhcp_netid,
    pub next: *mut pxe_service,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_vendor {
    pub len: i32,
    pub match_type: i32,
    pub enterprise: u32,
    pub data: String,
    pub netid: dhcp_netid,
    pub next: *mut dhcp_vendor,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_pxe_vendor {
    pub data: String,
    pub next: *mut dhcp_pxe_vendor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_mac {
    pub mask: u32,
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub hwaddr: [u8; 16],
    pub netid: dhcp_netid,
    pub next: *mut dhcp_mac,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_bridge {
    pub iface: [i8; 16],
    pub alias: *mut dhcp_bridge,
    pub next: *mut dhcp_bridge,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cond_domain {
    pub domain: String,
    pub prefix: String,
    pub start: in_addr,
    pub end: in_addr,
    pub start6: in6_addr,
    pub end6: in6_addr,
    pub is6: i32,
    pub indexed: i32,
    pub next: *mut cond_domain,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ra_interface {
    pub name: String,
    pub mtu_name: String,
    pub interval: i32,
    pub lifetime: i32,
    pub prio: i32,
    pub mtu: i32,
    pub next: *mut ra_interface,
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct dhcp_context {
    pub lease_time: u32,
    pub addr_epoch: u32,
    pub netmask: in_addr,
    pub broadcast: in_addr,
    pub local: in_addr,
    pub router: in_addr,
    pub start: in_addr,
    pub end: in_addr,
    pub start6: in6_addr,
    pub end6: in6_addr,
    pub local6: in6_addr,
    pub prefix: i32,
    pub if_index: i32,
    pub valid: u32,
    pub preferred: u32,
    pub saved_valid: u32,
    pub ra_time: time_t,
    pub ra_short_period_start: time_t,
    pub address_lost_time: time_t,
    pub template_interface: String,
    pub flags: i32,
    pub netid: dhcp_netid,
    pub filter: hcp_netid,
    pub next: dhcp_context,
    pub current: dhcp_context,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct shared_network {
    pub if_index: i32,
    pub match_addr: in_addr,
    pub shared_addr: in_addr,
    pub match_addr6: in6_addr,
    pub shared_addr6: in6_addr,
    pub next: *mut shared_network,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ping_result {
    pub addr: in_addr,
    pub time: time_t,
    pub hash: u32,
    pub next: *mut ping_result,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tftp_file {
    pub refcount: i32,
    pub fd: i32,
    pub size: off_t,
    pub dev: dev_t,
    pub inode: ino_t,
    pub filename: [i8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tftp_transfer {
    pub sockfd: i32,
    pub timeout: time_t,
    pub backoff: i32,
    pub block: u32,
    pub blocksize: u32,
    pub expansion: u32,
    pub offset: off_t,
    pub peer: mysockaddr,
    pub source: all_addr,
    pub if_index: i32,
    pub opt_blocksize: libc::c_char,
    pub opt_transize: libc::c_char,
    pub netascii: libc::c_char,
    pub carrylf: libc::c_char,
    pub file: *mut tftp_file,
    pub next: *mut tftp_transfer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addr_list {
    pub addr: in_addr,
    pub next: *mut addr_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tftp_prefix {
    pub interface: String,
    pub prefix: String,
    pub missing: i32,
    pub next: *mut tftp_prefix,
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct dhcp_relay {
    pub local: all_addr,
    pub server: all_addr,
    pub interface: String,
    pub iface_index: i32,
    pub current: *mut dhcp_relay,
    pub next: *mut dhcp_relay,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct opttab_t {
    pub name: String,
    pub val: u16_0,
    pub size: u16_0,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct dnsmasq_daemon {
    pub options: [u32; 2],
    pub default_resolv: resolvc,
    pub resolv_files: resolvc,
    pub last_resolv: time_t,
    pub servers_file: String,
    pub mxnames: mx_srv_record,
    pub naptr: naptr,
    pub txt: txt_record,
    pub rr: txt_record,
    pub ptr: ptr_record,
    pub host_records: host_record,
    pub host_records_tail: host_record,
    pub cnames: cname,
    pub auth_zones: auth_zone,
    pub int_names: interface_name,
    pub mxtarget: String,
    pub add_subnet4: mysubnet,
    pub add_subnet6: mysubnet,
    pub lease_file: String,
    pub username: String,
    pub groupname: String,
    pub scriptuser: String,
    pub luascript: String,
    pub authserver: String,
    pub hostmaster: String,
    pub authinterface: iname,
    pub secondary_forward_server: name_list,
    pub group_set: i32,
    pub osport: i32,
    pub domain_suffix: String,
    pub cond_domain: cond_domain,
    pub synth_domains: cond_domain,
    pub runfile: String,
    pub lease_change_command: String,
    pub if_names: iname,
    pub if_addrs: iname,
    pub if_except: iname,
    pub dhcp_except: iname,
    pub auth_peers: iname,
    pub tftp_interfaces: iname,
    pub bogus_addr: bogus_addr,
    pub ignore_addr: bogus_addr,
    pub servers: server,
    pub ipsets: ipsets,
    pub log_fac: i32,
    pub log_file: String,
    pub max_logs: i32,
    pub cachesize: i32,
    pub ftabsize: i32,
    pub port: i32,
    pub query_port: i32,
    pub min_port: i32,
    pub max_port: i32,
    pub local_ttl: libc::c_ulong,
    pub neg_ttl: libc::c_ulong,
    pub max_ttl: libc::c_ulong,
    pub min_cache_ttl: libc::c_ulong,
    pub max_cache_ttl: libc::c_ulong,
    pub auth_ttl: libc::c_ulong,
    pub dhcp_ttl: libc::c_ulong,
    pub use_dhcp_ttl: libc::c_ulong,
    pub dns_client_id: String,
    pub addn_hosts: hostsfile,
    pub dhcp: dhcp_context,
    pub dhcp6: dhcp_context,
    pub ra_interfaces: ra_interface,
    pub dhcp_conf: dhcp_config,
    pub dhcp_opts: dhcp_opt,
    pub dhcp_match: dhcp_opt,
    pub dhcp_opts6: dhcp_opt,
    pub dhcp_match6: dhcp_opt,
    pub dhcp_name_match: dhcp_match_name,
    pub dhcp_pxe_vendors: dhcp_pxe_vendor,
    pub dhcp_vendors: dhcp_vendor,
    pub dhcp_macs: dhcp_mac,
    pub boot_config: dhcp_boot,
    pub pxe_services: pxe_service,
    pub tag_if: tag_if,
    pub override_relays: addr_list,
    pub relay4: dhcp_relay,
    pub doing_relay_6: bool,
    pub relay6: dhcp_relay,
    pub delay_conf: delay_config,
    pub override_0: i32,
    pub enable_pxe: bool,
    pub doing_ra: bool,
    pub doing_dhcp: bool,
    pub doing_relay4: bool,
    pub doing_dhcp6: bool,
    pub dhcp_ignore: dhcp_netid_list,
    pub dhcp_ignore_names: dhcp_netid_list,
    pub dhcp_gen_names: dhcp_netid_list,
    pub force_broadcast: dhcp_netid_list,
    pub bootp_dynamic: dhcp_netid_list,
    pub dhcp_hosts_file: hostsfile,
    pub dhcp_opts_file: hostsfile,
    pub dynamic_dirs: hostsfile,
    pub dhcp_max: i32,
    pub tftp_max: i32,
    pub tftp_mtu: i32,
    pub dhcp_server_port: i32,
    pub dhcp_client_port: i32,
    pub start_tftp_port: i32,
    pub end_tftp_port: i32,
    pub min_leasetime: u32,
    pub doctors: doctor,
    pub edns_pktsz: u16,
    pub tftp_prefix: String,
    pub if_prefix: tftp_prefix,
    pub duid_enterprise: u32,
    pub duid_config_len: u32,
    pub duid_config: String,
    pub dbus_name: String,
    pub ubus_name: String,
    pub dump_file: String,
    pub dump_mask: i32,
    pub soa_sn: libc::c_ulong,
    pub soa_refresh: libc::c_ulong,
    pub soa_retry: libc::c_ulong,
    pub soa_expiry: libc::c_ulong,
    pub metrics: [u32_0; 20],
    pub packet: Vec<u8>,
    // pub packet_buff_sz: i32,
    pub namebuff: String,
    pub frec_list: frec,
    pub free_frec_src: frec_src,
    pub frec_src_count: i32,
    pub sfds: serverfd,
    pub interfaces: irec,
    pub listeners: listener,
    pub last_server: server,
    pub forwardtime: time_t,
    pub forwardcount: i32,
    pub srv_save: server,
    pub packet_len: size_t,
    pub rfd_save: randfd,
    pub tcp_pids: [pid_t; 20],
    pub tcp_pipes: [i32; 20],
    pub pipe_to_parent: i32,
    pub randomsocks: [randfd; 64],
    pub v6pktinfo: i32,
    pub interface_addrs: addrlist,
    pub log_id: i32,
    pub log_display_id: i32,
    pub log_source_addr: mysockaddr,
    pub dhcpfd: Socket,
    pub helperfd: Socket,
    pub pxefd: Socket,
    pub inotifyfd: i32,
    pub netlinkfd: i32,
    pub kernel_version: i32,
    pub dhcp_packet: Vec<u8>,
    pub dhcp_buff: Vec<u8>,
    pub dhcp_buff2: Vec<u8>,
    pub dhcp_buff3: Vec<u8>,
    pub ping_results: ping_result,
    pub lease_stream: FILE,
    pub bridges: dhcp_bridge,
    pub shared_networks: shared_network,
    pub duid_len: i32,
    pub duid: Vec<u8>,
    pub outpacket: Vec<u8>,
    pub dhcp6fd: Socket,
    pub icmp6fd: i32,
    pub dbus: *mut libc::c_void,
    pub tftp_trans: tftp_transfer,
    pub tftp_done_trans: tftp_transfer,
    pub addrbuff: Vec<u8>,
    pub addrbuff2: Vec<u8>,
    pub dumpfd: i32,
}

pub const F_IPV4: u32 = (1 as u32) << 7 as i32;
pub const F_IPV6: u32 = (1 as u32) << 8 as i32;
pub const ADDRLIST_IPV6: i32 = 2 as i32;
pub const F_DHCP: u32 = (1 as u32) << 4 as i32;
pub const F_HOSTS: u32 = (1 as u32) << 6 as i32;
pub const F_FORWARD: u32 = (1 as u32) << 3 as i32;
pub const F_NXDOMAIN: u32 = (1 as u32) << 10 as i32;
pub const F_NEG: u32 = (1 as u32) << 5 as i32;
pub const F_AUTH: u32 = (1 as u32) << 21 as i32;
pub const F_CNAME: u32 =
        (1 as u32) << 11 as i32;
pub const F_CONFIG: u32 =
        (1 as u32) << 13 as i32;
pub const F_RRNAME: u32 =
        (1 as u32) << 17 as i32;
pub const ADDRSTRLEN: i32 = 46 as i32;
pub const ADDRLIST_REVONLY: i32 = 4 as i32;
pub const F_REVERSE: u32 =
        (1 as u32) << 2 as i32;
        
#[derive(Copy, Clone)]
#[repr(C)]

pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat64 {
    pub st_dev: __dev_t,
    pub st_ino: __ino64_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt64_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}

pub const _STAT_VER_LINUX: i32 = 1 as i32;
pub const _STAT_VER: i32 = _STAT_VER_LINUX;

// https://code.woboq.org/userspace/glibc/libio/libioP.h.html#117
pub type _IO_finish_t = fn(*mut FILE, i32);
pub type _IO_overflow_t = fn(*mut FILE, i32) -> i32;
pub type _IO_underflow_t = fn(*mut FILE) -> i32;
pub type _IO_pbackfail_t = fn(*mut FILE, i32) -> i32;
pub type _IO_xsputn_t = fn(FP: *mut FILE, DATA: *mut libc::c_void, N: libc::size_t) -> __off64_t;
pub type _IO_xsgetn_t = fn(FP: *mut FILE, DATA: *mut libc::c_void, N: libc::size_t) -> libc::size_t;
pub type _IO_seekoff_t = fn(FP: *mut FILE, OFF: __off64_t, DIR: i32, MODE: i32) -> __off64_t;
pub type _IO_seekpos_t = fn(*mut FILE, __off64_t, i32) -> __off64_t;
pub type _IO_setbuf_t = fn(*mut FILE, String, libc::ssize_t) -> *mut FILE;
pub type _IO_sync_t = fn(*mut FILE) -> i32;
pub type _IO_doallocate_t = fn(*mut FILE) -> i32;
pub type _IO_read_t = fn(*mut FILE, *mut libc::c_void, libc::ssize_t) -> libc::ssize_t;
pub type _IO_write_t = fn(*mut FILE, *mut libc::c_void, libc::ssize_t) -> libc::ssize_t;
pub type _IO_seek_t = fn(*mut FILE, *mut __off64_t, i32) -> __off64_t;
pub type _IO_close_t = fn(*mut FILE) -> i32;
pub type _IO_stat_t = fn(*mut FILE, *mut libc::c_void) -> i32;
pub type _IO_showmanyc_t = fn(*mut FILE) -> i32;
pub type _IO_imbue_t = fn(*mut FILE, libc::c_void);


// #define JUMP_FIELD(TYPE, NAME) TYPE NAME
pub struct _IO_jump_t
{
    // JUMP_FIELD(size_t, __dummy);
    pub __dummy: libc::size_t,
    // JUMP_FIELD(size_t, __dummy2);
    pub __dummy2: libc::size_t,
    // JUMP_FIELD(_IO_finish_t, __finish);
    pub __finish: _IO_finish_t,
    // JUMP_FIELD(_IO_overflow_t, __overflow);
    pub __overflow: _IO_overflow_t,
    // JUMP_FIELD(_IO_underflow_t, __underflow);
    pub __underflow: _IO_underflow_t,
    // JUMP_FIELD(_IO_underflow_t, __uflow);
    pub __uflow: _IO_underflow_t,
    // JUMP_FIELD(_IO_pbackfail_t, __pbackfail);
    pub __pbackfail: _IO_pbackfail_t,
    /* showmany */
    // JUMP_FIELD(_IO_xsputn_t, __xsputn);
    pub __xsputn: _IO_xsputn_t,
    // JUMP_FIELD(_IO_xsgetn_t, __xsgetn);
    pub __xsgetn: _IO_xsgetn_t,
    // JUMP_FIELD(_IO_seekoff_t, __seekoff);
    pub __seekpos: _IO_seekpos_t,
    // JUMP_FIELD(_IO_seekpos_t, __seekpos);
    pub __seekoff: _IO_seekoff_t,
    // JUMP_FIELD(_IO_setbuf_t, __setbuf);
    pub __setbuf: _IO_setbuf_t,
    // JUMP_FIELD(_IO_sync_t, __sync);
    pub __sync: _IO_sync_t,
    // JUMP_FIELD(_IO_doallocate_t, __doallocate);
    pub __doallocate: _IO_doallocate_t,
    // JUMP_FIELD(_IO_read_t, __read);
    pub __read: _IO_read_t,
    // JUMP_FIELD(_IO_write_t, __write);
    pub __write: _IO_write_t,
    // JUMP_FIELD(_IO_seek_t, __seek);
    pub __seek: _IO_seek_t,
    // JUMP_FIELD(_IO_close_t, __close);
    pub __close: _IO_close_t,
    // JUMP_FIELD(_IO_stat_t, __stat);
    pub __stat: _IO_stat_t,
    // JUMP_FIELD(_IO_showmanyc_t, __showmanyc);
    pub __showmanyc: _IO_showmanyc_t,
    // JUMP_FIELD(_IO_imbue_t, __imbue);
    pub __imbue: _IO_imbue_t,
}


#[derive(Clone, Copy)]
#[repr(C)]
pub struct _IO_wide_data
{
//   wchar_t *_IO_read_ptr;        /* Current read pointer */
    pub _IO_read_ptr: *mut libc::wchar_t,
//   wchar_t *_IO_read_end;        /* End of get area. */
    pub _IO_read_end: *mut libc::wchar_t,
//   wchar_t *_IO_read_base;        /* Start of putback+get area. */
    pub _IO_read_base: *mut libc::wchar_t,
//   wchar_t *_IO_write_base;        /* Start of put area. */
    pub _IO_wriate_base: *mut libc::wchar_t,
//   wchar_t *_IO_write_ptr;        /* Current put pointer. */
    pub _IO_write_ptr: *mut libc::wchar_t,
//   wchar_t *_IO_write_end;        /* End of put area. */
    pub _IO_write_ned: *mut libc::wchar_t,
//   wchar_t *_IO_buf_base;        /* Start of reserve area. */
    pub _IO_buf_base: *mut libc::wchar_t,
//   wchar_t *_IO_buf_end;                /* End of reserve area. */
    pub _IO_buf_end: *mut libc::wchar_t,
  /* The following fields are used to support backing up and undo. */
//   wchar_t *_IO_save_base;        /* Pointer to start of non-current get area. */
    pub _IO_save_base: *mut libc::wchar_t,
//   wchar_t *_IO_backup_base;        /* Pointer to first valid character of                                    backup area */
    pub _IO_backup_base: *mut libc::wchar_t,
//   wchar_t *_IO_save_end;        /* Pointer to end of non-current get area. */
    pub _IO_save_end: *mut libc::wchar_t,
//   __mbstate_t _IO_state;
    pub _IO_state: *mut __mbstate_t,
//   __mbstate_t _IO_last_state;
    pub __IO_laste_state: *mut __mbstate_t,
//   struct _IO_codecvt _codecvt;
    pub __codecvt: *mut _IO_codecvt,
//   wchar_t _shortbuf[1];
    pub _shortbuf: [libc::wchar_t;1],
//   const struct _IO_jump_t *_wide_vtable;
    pub _wide_vtable: *mut _IO_jump_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: String,
    pub _IO_read_end: String,
    pub _IO_read_base: String,
    pub _IO_write_base: String,
    pub _IO_write_ptr: String,
    pub _IO_write_end: String,
    pub _IO_buf_base: String,
    pub _IO_buf_end: String,
    pub _IO_save_base: String,
    pub _IO_backup_base: String,
    pub _IO_save_end: String,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}

// pub type _IO_lock_t = ();

// pub const _IO_EOF_SEEN: i32 = 0x10 as i32;

// pub const _IO_ERR_SEEN: i32 = 0x20 as i32;

// pub type _IO_wide_data;

// pub type _IO_codecvt;

// pub type _IO_marker;

// pub type FILE = _IO_FILE;

pub type __compar_fn_t = Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32>;

// #[inline]
// pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char)
// -> i32 {
//     return strtol(__nptr,
//                     NULL as *mut libc::c_void as *mut *mut libc::c_char,
//                     10 as i32) as i32;
// }

// #[inline]
// pub unsafe extern "C" fn atol(mut __nptr: *const libc::c_char)
//     -> libc::c_long {
//     return strtol(__nptr,
//                     NULL as *mut libc::c_void as *mut *mut libc::c_char,
//                     10 as i32);
// }

// #[inline]
// pub unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char)
//     -> libc::c_longlong {
//     return strtoll(__nptr,
//                     NULL as *mut libc::c_void as *mut *mut libc::c_char,
//                     10 as i32);
// }

// #[no_mangle]
// pub fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char)
//     -> libc::c_double;
// #[no_mangle]
// pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
//                 _: i32) -> libc::c_long;
// #[no_mangle]
// pub fn strtoll(_: *const libc::c_char, _: *mut *mut libc::c_char,
//                 _: i32) -> libc::c_longlong;

// #[inline]
// pub unsafe extern "C" fn strtoimax(mut nptr: *const libc::c_char,
//                                     mut endptr: *mut *mut libc::c_char,
//                                     mut base: i32) -> intmax_t {
//     return __strtol_internal(nptr, endptr, base, 0 as i32);
// }

// #[inline]
// pub unsafe extern "C" fn strtoumax(mut nptr: *const libc::c_char,
//                                     mut endptr: *mut *mut libc::c_char,
//                                     mut base: i32) -> uintmax_t {
//     return __strtoul_internal(nptr, endptr, base, 0 as i32);
// }

// #[inline]
// pub unsafe extern "C" fn wcstoimax(mut nptr: *const __gwchar_t,
//                                     mut endptr: *mut *mut __gwchar_t,
//                                     mut base: i32) -> intmax_t {
//     return __wcstol_internal(nptr, endptr, base, 0 as i32);
// }

// #[inline]
// pub unsafe extern "C" fn wcstoumax(mut nptr: *const __gwchar_t,
//                                     mut endptr: *mut *mut __gwchar_t,
//                                     mut base: i32) -> uintmax_t {
//     return __wcstoul_internal(nptr, endptr, base, 0 as i32);
// }


// #[no_mangle]
// pub fn __strtol_internal(__nptr: *const libc::c_char,
//                             __endptr: *mut *mut libc::c_char,
//                             __base: i32, __group: i32)
//     -> libc::c_long;

// #[no_mangle]
// pub fn __strtoul_internal(__nptr: *const libc::c_char,
//                             __endptr: *mut *mut libc::c_char,
//                             __base: i32, __group: i32)
//     -> libc::c_ulong;

// #[no_mangle]
// pub fn __wcstol_internal(__nptr: *const __gwchar_t,
//                             __endptr: *mut *mut __gwchar_t,
//                             __base: i32, __group: i32)
//     -> libc::c_long;

// #[no_mangle]
// pub fn __wcstoul_internal(__nptr: *const __gwchar_t,
//                             __endptr: *mut *mut __gwchar_t,
//                             __base: i32, __group: i32)
//     -> libc::c_ulong;

#[inline]    
pub unsafe extern "C" fn __bswap_16(mut __bsx: u16) -> u16 {
    return (__bsx as i32 >> 8 as i32 & 0xff as i32
                |
                (__bsx as i32 & 0xff as i32) <<
                    8 as i32) as u16;
}

#[inline]
pub unsafe extern "C" fn __bswap_32(mut __bsx: u32) -> u32 {
    return (__bsx & 0xff000000 as u32) >> 24 as i32 |
                (__bsx & 0xff0000 as u32) >> 8 as i32 |
                (__bsx & 0xff00 as u32) << 8 as i32 |
                (__bsx & 0xff as u32) << 24 as i32;
}

#[inline]
pub unsafe extern "C" fn __bswap_64(mut __bsx: u64) -> u64 {
    return ((__bsx as libc::c_ulonglong &
                    0xff00000000000000 as libc::c_ulonglong) >>
                56 as i32 |
                (__bsx as libc::c_ulonglong &
                        0xff000000000000 as libc::c_ulonglong) >>
                    40 as i32 |
                (__bsx as libc::c_ulonglong &
                        0xff0000000000 as libc::c_ulonglong) >>
                    24 as i32 |
                (__bsx as libc::c_ulonglong &
                        0xff00000000 as libc::c_ulonglong) >>
                    8 as i32 |
                (__bsx as libc::c_ulonglong &
                        0xff000000 as libc::c_ulonglong) << 8 as i32
                |
                (__bsx as libc::c_ulonglong &
                        0xff0000 as libc::c_ulonglong) << 24 as i32 |
                (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong)
                    << 40 as i32 |
                (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong)
                    << 56 as i32) as __uint64_t;
}

// #[inline]
// pub unsafe extern "C" fn fstat(mut __fd: i32,
//                                 mut __statbuf: *mut stat) -> i32 {
//     return __fxstat(_STAT_VER, __fd, __statbuf);
// }

// #[inline]
// pub unsafe extern "C" fn stat64(mut __path: *const libc::c_char,
//                                 mut __statbuf: *mut stat64)
//     -> i32 {
//     return __xstat64(_STAT_VER, __path, __statbuf);
// }

// #[inline]
// pub unsafe extern "C" fn fstat64(mut __fd: i32,
//                                     mut __statbuf: *mut stat64)
//     -> i32 {
//     return __fxstat64(_STAT_VER, __fd, __statbuf);
// }

// #[inline]
// pub unsafe extern "C" fn fstatat(mut __fd: i32,
//                                     mut __filename: *const libc::c_char,
//                                     mut __statbuf: *mut stat,
//                                     mut __flag: i32) -> i32 {
//     return __fxstatat(_STAT_VER, __fd, __filename, __statbuf, __flag);
// }

// #[inline]
// pub unsafe extern "C" fn fstatat64(mut __fd: i32,
//                                     mut __filename: *const libc::c_char,
//                                     mut __statbuf: *mut stat64,
//                                     mut __flag: i32)
//     -> i32 {
//     return __fxstatat64(_STAT_VER, __fd, __filename, __statbuf, __flag);
// }

// #[inline]
// pub unsafe extern "C" fn lstat(mut __path: *const libc::c_char,
//                                 mut __statbuf: *mut stat) -> i32 {
//     return __lxstat(_STAT_VER, __path, __statbuf);
// }

// #[inline]
// pub unsafe extern "C" fn lstat64(mut __path: *const libc::c_char,
//                                     mut __statbuf: *mut stat64)
//     -> i32 {
//     return __lxstat64(_STAT_VER, __path, __statbuf);
// }

// #[inline]
// pub unsafe extern "C" fn mknod(mut __path: *const libc::c_char,
//                                 mut __mode: __mode_t, mut __dev: __dev_t)
//     -> i32 {
//     return __xmknod(_MKNOD_VER, __path, __mode, &mut __dev);
// }

pub const _MKNOD_VER: i32 = 0 as i32;

// #[inline]
// pub unsafe extern "C" fn mknodat(mut __fd: i32,
//                                     mut __path: *const libc::c_char,
//                                     mut __mode: __mode_t, mut __dev: __dev_t)
//     -> i32 {
//     return __xmknodat(_MKNOD_VER, __fd, __path, __mode, &mut __dev);
// }

// #[inline]
// pub unsafe extern "C" fn stat(mut __path: *const libc::c_char,
//                                 mut __statbuf: *mut stat) -> i32 {
//     return __xstat(_STAT_VER, __path, __statbuf);
// }

// #[no_mangle]    
// pub fn __xstat(__ver: i32, __filename: *const libc::c_char,
//                 __stat_buf: *mut stat) -> i32;

// #[no_mangle]
// pub fn __fxstat(__ver: i32, __fildes: i32,
//                 __stat_buf: *mut stat) -> i32;

// #[no_mangle]
// pub fn __xstat64(__ver: i32, __filename: *const libc::c_char,
//                     __stat_buf: *mut stat64) -> i32;

// #[no_mangle]
// pub fn __fxstat64(__ver: i32, __fildes: i32,
//                     __stat_buf: *mut stat64) -> i32;

// #[no_mangle]
// pub fn __fxstatat(__ver: i32, __fildes: i32,
//                     __filename: *const libc::c_char,
//                     __stat_buf: *mut stat, __flag: i32)
//     -> i32;

// #[no_mangle]
// pub fn __fxstatat64(__ver: i32, __fildes: i32,
//                     __filename: *const libc::c_char,
//                     __stat_buf: *mut stat64, __flag: i32)
//     -> i32;

// #[no_mangle]
// pub fn __lxstat(__ver: i32, __filename: *const libc::c_char,
//                 __stat_buf: *mut stat) -> i32;

// #[no_mangle]
// pub fn __lxstat64(__ver: i32, __filename: *const libc::c_char,
//                     __stat_buf: *mut stat64) -> i32;

// #[no_mangle]
// pub fn __xmknod(__ver: i32, __path: *const libc::c_char,
//                 __mode: __mode_t, __dev: *mut __dev_t) -> i32;

// #[no_mangle]
// pub fn __xmknodat(__ver: i32, __fd: i32,
//                     __path: *const libc::c_char, __mode: __mode_t,
//                     __dev: *mut __dev_t) -> i32;

// #[no_mangle]   
// pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
//                 _: libc::c_ulong) -> *mut libc::c_void;

// #[no_mangle]
// pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
//                 _: libc::c_ulong) -> i32;


// #[inline]
// pub unsafe extern "C" fn vprintf(mut __fmt: *const libc::c_char,
//                                     mut __arg: ::std::ffi::VaList)
//     -> i32 {
//     return vfprintf(stdout, __fmt, __arg.as_va_list());
// }

// #[inline]
// pub unsafe extern "C" fn getchar() -> i32 { return getc(stdin); }

// #[inline]
// pub unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE)
//     -> i32 {
//     return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as
//                     i32 as libc::c_long != 0 {
//                 __uflow(__fp)
//             } else {
//                 let fresh0 = (*__fp)._IO_read_ptr;
//                 (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
//                 *(fresh0 as *mut u8) as i32
//             };
// }

// #[inline]
// pub unsafe extern "C" fn getchar_unlocked() -> i32 {
//     return if ((*stdin)._IO_read_ptr >= (*stdin)._IO_read_end) as
//                     i32 as libc::c_long != 0 {
//                 __uflow(stdin)
//             } else {
//                 let fresh1 = (*stdin)._IO_read_ptr;
//                 (*stdin)._IO_read_ptr = (*stdin)._IO_read_ptr.offset(1);
//                 *(fresh1 as *mut u8) as i32
//             };
// }

// #[inline]
// pub unsafe extern "C" fn fgetc_unlocked(mut __fp: *mut FILE)
//     -> i32 {
//     return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as
//                     i32 as libc::c_long != 0 {
//                 __uflow(__fp)
//             } else {
//                 let fresh2 = (*__fp)._IO_read_ptr;
//                 (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
//                 *(fresh2 as *mut u8) as i32
//             };
// }

// #[inline]
// pub unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
//     return putc(__c, stdout);
// }

// #[inline]
// pub unsafe extern "C" fn fputc_unlocked(mut __c: i32,
//                                         mut __stream: *mut FILE)
//     -> i32 {
//     return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as
//                     i32 as libc::c_long != 0 {
//                 __overflow(__stream, __c as u8 as i32)
//             } else {
//                 let fresh3 = (*__stream)._IO_write_ptr;
//                 (*__stream)._IO_write_ptr =
//                     (*__stream)._IO_write_ptr.offset(1);
//                 *fresh3 = __c as libc::c_char;
//                 *fresh3 as u8 as i32
//             };
// }

// #[inline]
// pub unsafe extern "C" fn putc_unlocked(mut __c: i32,
//                                         mut __stream: *mut FILE)
//     -> i32 {
//     return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as
//                     i32 as libc::c_long != 0 {
//                 __overflow(__stream, __c as u8 as i32)
//             } else {
//                 let fresh4 = (*__stream)._IO_write_ptr;
//                 (*__stream)._IO_write_ptr =
//                     (*__stream)._IO_write_ptr.offset(1);
//                 *fresh4 = __c as libc::c_char;
//                 *fresh4 as u8 as i32
//             };
// }

// #[inline]
// pub unsafe extern "C" fn putchar_unlocked(mut __c: i32)
//     -> i32 {
//     return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as
//                     i32 as libc::c_long != 0 {
//                 __overflow(stdout, __c as u8 as i32)
//             } else {
//                 let fresh5 = (*stdout)._IO_write_ptr;
//                 (*stdout)._IO_write_ptr =
//                     (*stdout)._IO_write_ptr.offset(1);
//                 *fresh5 = __c as libc::c_char;
//                 *fresh5 as u8 as i32
//             };
// }

// #[inline]
// pub unsafe extern "C" fn getline(mut __lineptr: *mut *mut libc::c_char,
//                                     mut __n: *mut size_t,
//                                     mut __stream: *mut FILE) -> __ssize_t {
//     return __getdelim(__lineptr, __n, '\n' as i32, __stream);
// }

// #[inline]
// pub unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE)
//     -> i32 {
//     return ((*__stream)._flags & _IO_EOF_SEEN != 0 as i32) as
//                 i32;
// }

// #[inline]
// pub unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE)
//     -> i32 {
//     return ((*__stream)._flags & _IO_ERR_SEEN != 0 as i32) as
//                 i32;
// }

// #[inline]
// pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char)
//      -> libc::c_double {
//         return strtod(__nptr,
//                       NULL as *mut libc::c_void as *mut *mut libc::c_char);
// }

#[inline]
pub unsafe extern "C" fn bsearch(mut __key: *const libc::c_void,
                                    mut __base: *const libc::c_void,
                                    mut __nmemb: size_t, mut __size: size_t,
                                    mut __compar: __compar_fn_t)
    -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p = 0 as *const libc::c_void;
    let mut __comparison: i32 = 0;
    __l = 0 as i32 as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx =
            __l.wrapping_add(__u).wrapping_div(2 as i32 as
                                                    libc::c_ulong);
        __p =
            (__base as
                    *const libc::c_char).offset(__idx.wrapping_mul(__size) as
                                                    isize) as
                *mut libc::c_void;
        __comparison =
            Some(__compar.expect("non-null function pointer")).expect("non-null function pointer")(__key,
                                                                                                    __p);
        if __comparison < 0 as i32 {
            __u = __idx
        } else if __comparison > 0 as i32 {
            __l = __idx.wrapping_add(1 as i32 as libc::c_ulong)
        } else { return __p as *mut libc::c_void }
    }
    return NULL as *mut libc::c_void;
}

pub const DHCP_CHADDR_MAX: i32 = 16 as i32;

pub const IN6ADDRSZ: i32 = 16 as i32;

// #[inline]
// pub unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
//     return if __c >= -(128 as i32) && __c < 256 as i32 {
//                 *(*__ctype_tolower_loc()).offset(__c as isize)
//             } else { __c };
// }

// #[inline]
// pub unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
//     return if __c >= -(128 as i32) && __c < 256 as i32 {
//                 *(*__ctype_toupper_loc()).offset(__c as isize)
//             } else { __c };
// }

// #[no_mangle]
// pub fn __ctype_tolower_loc() -> *mut *const __int32_t;

// #[no_mangle]
// pub fn __ctype_toupper_loc() -> *mut *const __int32_t;

// #[no_mangle]
// pub fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;

pub const LOG_WARNING: i32 = 4 as i32;
pub const LOG_INFO: i32 = 6 as i32;

pub const KEYBLOCK_LEN: i32 = 40 as i32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_header {
    pub id: u16_0,
    pub hb3: u8_0,
    pub hb4: u8_0,
    pub qdcount: u16_0,
    pub ancount: u16_0,
    pub nscount: u16_0,
    pub arcount: u16_0,
}

pub const HB4_RCODE: i32 = 0xf as i32;
    
pub const HB3_TC: i32 = 0x2 as i32;

pub const HB3_AA: i32 = 0x4 as i32;

pub const HB4_AD: i32 = 0x20 as i32;

pub const HB4_RA: i32 = 0x80 as i32;

pub const HB3_QR: i32 = 0x80 as i32;

pub const C_IN: i32 = 1 as i32;

pub const T_SOA: i32 = 6 as i32;

pub const T_A: i32 = 1 as i32;

pub const T_AAAA: i32 = 28 as i32;

pub const T_CNAME: i32 = 5 as i32;

pub const T_NAPTR: i32 = 35 as i32;

pub const T_TXT: i32 = 16 as i32;

pub const T_MX: i32 = 15 as i32;

pub const T_SRV: i32 = 33 as i32;

pub const T_NS: i32 = 2 as i32;

pub const T_AXFR: i32 = 252 as i32;

pub const T_PTR: i32 = 12 as i32;

pub const QUERY: i32 = 0 as i32;

pub const HB3_OPCODE: i32 = 0x78 as i32;

// #[no_mangle]
// pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
//          -> *mut libc::c_char;

// #[no_mangle]
// pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
//     -> *mut libc::c_char;

// #[no_mangle]
// pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
//     -> i32;

// #[no_mangle]
// pub fn strchr(_: *const libc::c_char, _: i32)
//     -> *mut libc::c_char;

// #[no_mangle]
// pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;


// #[no_mangle]        
// pub static mut stdin: *mut FILE;

// #[no_mangle]
// pub static mut stdout: *mut FILE;

// #[no_mangle]
// pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
//                 _: ::std::ffi::VaList) -> i32;

// #[no_mangle]
// pub fn getc(__stream: *mut FILE) -> i32;

// #[no_mangle]
// pub fn putc(__c: i32, __stream: *mut FILE) -> i32;

// #[no_mangle]
// pub fn __uflow(_: *mut FILE) -> i32;

// #[no_mangle]
// pub fn __overflow(_: *mut FILE, _: i32) -> i32;

// #[no_mangle]
// pub fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
//                     __delimiter: i32, __stream: *mut FILE)
//     -> __ssize_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub type_0: u32,
    pub name: *const libc::c_char,
}

pub const SMALLDNAME: i32 = 50 as i32;

pub const HOSTSFILE: [i8; 11] =
        unsafe {
            *::std::mem::transmute::<&[u8; 11],
                                     &[i8; 11]>(b"/etc/hosts\x00")
        };


pub const NOTIMP: i32 = 4 as i32;
    
pub const REFUSED: i32 = 5 as i32;

pub const SERVFAIL: i32 = 2 as i32;

pub const INADDRSZ: i32 = 4 as i32;

pub const MAXDNAME: i32 = 1025 as i32;
    
pub const LOG_ERR: i32 = 3 as i32;

pub const LOG_DAEMON: i32 =
    (3 as i32) << 3 as i32;

// pub const errno: i32 = *__errno_location();

// #[no_mangle]
//  pub fn __errno_location() -> *mut i32;

//  #[no_mangle] 
// pub fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;

// #[no_mangle]
// pub fn ctime(__timer: *const time_t) -> *mut libc::c_char;

// extern "C" {
//     pub type _IO_wide_data;
//     pub type _IO_codecvt;
//     pub type _IO_marker;
//     #[no_mangle]
//     static mut stdin: *mut FILE;
//     #[no_mangle]
//     static mut stdout: *mut FILE;
//     #[no_mangle]
//     fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
//      -> i32;
//     #[no_mangle]
//     fn getc(__stream: *mut FILE) -> i32;
//     #[no_mangle]
//     fn putc(__c: i32, __stream: *mut FILE) -> i32;
//     #[no_mangle]
//     fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
//                   __delimiter: i32, __stream: *mut FILE) -> __ssize_t;
//     #[no_mangle]
//     fn __xstat(__ver: i32, __filename: *const libc::c_char,
//                __stat_buf: *mut stat) -> i32;
//     #[no_mangle]
//     fn __fxstat(__ver: i32, __fildes: i32,
//                 __stat_buf: *mut stat) -> i32;
//     #[no_mangle]
//     fn __xstat64(__ver: i32, __filename: *const libc::c_char,
//                  __stat_buf: *mut stat64) -> i32;
//     #[no_mangle]
//     fn __fxstat64(__ver: i32, __fildes: i32,
//                   __stat_buf: *mut stat64) -> i32;
//     #[no_mangle]
//     fn __fxstatat(__ver: i32, __fildes: i32,
//                   __filename: *const libc::c_char, __stat_buf: *mut stat,
//                   __flag: i32) -> i32;
//     #[no_mangle]
//     fn __fxstatat64(__ver: i32, __fildes: i32,
//                     __filename: *const libc::c_char, __stat_buf: *mut stat64,
//                     __flag: i32) -> i32;
//     #[no_mangle]
//     fn __lxstat(__ver: i32, __filename: *const libc::c_char,
//                 __stat_buf: *mut stat) -> i32;
//     #[no_mangle]
//     fn __lxstat64(__ver: i32, __filename: *const libc::c_char,
//                   __stat_buf: *mut stat64) -> i32;
//     #[no_mangle]
//     fn __xmknod(__ver: i32, __path: *const libc::c_char,
//                 __mode: __mode_t, __dev: *mut __dev_t) -> i32;
//     #[no_mangle]
//     fn __xmknodat(__ver: i32, __fd: i32,
//                   __path: *const libc::c_char, __mode: __mode_t,
//                   __dev: *mut __dev_t) -> i32;
//     #[no_mangle]
//     fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
//      -> *mut libc::c_void;
//     #[no_mangle]
//     fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
//               _: libc::c_ulong) -> i32;
//     #[no_mangle]
//     fn __uflow(_: *mut FILE) -> i32;
//     #[no_mangle]
//     fn __overflow(_: *mut FILE, _: i32) -> i32;
//     #[no_mangle]
//     fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char)
//      -> libc::c_double;
//     #[no_mangle]
//     fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
//               _: i32) -> libc::c_long;
//     #[no_mangle]
//     fn strtoll(_: *const libc::c_char, _: *mut *mut libc::c_char,
//                _: i32) -> libc::c_longlong;
//     #[no_mangle]
//     fn __ctype_tolower_loc() -> *mut *const __int32_t;
//     #[no_mangle]
//     fn __ctype_toupper_loc() -> *mut *const __int32_t;
//     #[no_mangle]
//     fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
//     #[no_mangle]
//     fn __strtol_internal(__nptr: *const libc::c_char,
//                          __endptr: *mut *mut libc::c_char,
//                          __base: i32, __group: i32)
//      -> libc::c_long;
//     #[no_mangle]
//     fn __strtoul_internal(__nptr: *const libc::c_char,
//                           __endptr: *mut *mut libc::c_char,
//                           __base: i32, __group: i32)
//      -> libc::c_ulong;
//     #[no_mangle]
//     fn __wcstol_internal(__nptr: *const __gwchar_t,
//                          __endptr: *mut *mut __gwchar_t, __base: i32,
//                          __group: i32) -> libc::c_long;
//     #[no_mangle]
//     fn __wcstoul_internal(__nptr: *const __gwchar_t,
//                           __endptr: *mut *mut __gwchar_t, __base: i32,
//                           __group: i32) -> libc::c_ulong;
//     #[no_mangle]
//     static mut dnsmasq_daemon: *mut dnsmasq_daemon;
//     #[no_mangle]
//     fn whine_malloc(size: size_t) -> *mut libc::c_void;
//     #[no_mangle]
//     fn iface_enumerate(family: i32, parm: *mut libc::c_void,
//                        callback:
//                            Option<unsafe extern "C" fn() -> i32>)
//      -> i32;
//     #[no_mangle]
//     fn queue_arp(action: i32, mac: *mut u8,
//                  maclen: i32, family: i32,
//                  addr: *mut all_addr);
// }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}




pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;




/* No MAC addr */
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct arp_record {
    pub hwlen: u16,
    pub status: u16,
    pub family: i32,
    pub hwaddr: [u8; 16],
    pub addr: all_addr,
    // pub next: *mut arp_record,
}

pub type __kernel_sa_family_t = u16;

pub type __u8 = u8;
pub type __be16 = u16;

//    26 struct atalk_addr {
//    27     __be16  s_net;
//    28     __u8    s_node;
//    29 };
#[derive(Copy, Clone)]
#[repr(C)]
pub struct atalk_addr {
    pub s_net: __be16,
    pub s_node: __u8,
}

//    31 struct sockaddr_at {
//    32     __kernel_sa_family_t sat_family;
//    33     __u8          sat_port;
//    34     struct atalk_addr sat_addr;
//    35     char          sat_zero[8];
//    36 };
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_at {
    pub sat_family: __kernel_sa_family_t,
    pub sat_port: __u8,
    pub sat_addr: atalk_addr,
    pub sat_zer: [i8;8],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ax25_address {
    pub ax25_call: [i8;7],
}

//    49 struct sockaddr_ax25 {
//    50     __kernel_sa_family_t sax25_family;
//    51     ax25_address    sax25_call;
//    52     int     sax25_ndigis;
//    53     /* Digipeater ax25_address sets follow */
//    54 };
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_ax25 {
    pub sax25_family: __kernel_sa_family_t,
    pub sax25_call: ax25_address,
    pub sax25_ndigis: i32,
}

// struct sockaddr_dl {
// 	u_char	sdl_len;	/* Total length of sockaddr */
// 	u_char	sdl_family;	/* AF_DLI */
// 	u_short	sdl_index;	/* if != 0, system given index for interface */
// 	u_char	sdl_type;	/* interface type */
// 	u_char	sdl_nlen;	/* interface name length, no trailing 0 reqd. */
// 	u_char	sdl_alen;	/* link level address length */
// 	u_char	sdl_slen;	/* link layer selector length */
// 	char	sdl_data[12];	/* minimum work area, can be larger;
// 				   contains both if name and ll address */
// };
#[derive(Copy,Clone)]
#[repr(C)]
pub struct sockaddr_dl {
    pub sdl_len: u8,
    pub sdl_family: u8,
    pub sdl_index: u16,
    pub sdl_nlen: u8,
    pub sdl_alen: u8,
    pub sdl_slen: u8,
    pub sdl_data: [char;12],
}

// struct sockaddr_eon {
// 	u_char 			seon_len;	/* Length */
// 	u_char 			seon_family;	/* AF_ISO */
// 	u_char			seon_status;	/* overlays session suffixlen */
// #define EON_ESLINK_UP		0x1
// #define EON_ESLINK_DOWN		0x2
// #define EON_ISLINK_UP		0x10
// #define EON_ISLINK_DOWN		0x20
// /* no change is neither up or down */
// 	u_char			seon_pad1;	/* 0, overlays tsfxlen */
// 	u_char			seon_adrlen;
// 	u_char			seon_afi;		/* 47 */
// 	u_char			seon_idi[2];	/* 0006 */
// 	u_char			seon_vers;		/* 03 */
// 	u_char			seon_glbnum[2];	/* see RFC 1069 */
// 	u_char			seon_RDN[2];	/* see RFC 1070 */
// 	u_char			seon_pad2[3];	/* see RFC 1070 */
// 	u_char			seon_LAREA[2];	/* see RFC 1070 */
// 	u_char			seon_pad3[2];	/* see RFC 1070 */
// 		/* right now ip addr is  aligned  -- be careful --
// 		 * future revisions may have it u_char[4]
// 		 */
// 	u_int			seon_ipaddr;	/* a.b.c.d */
// 	u_char			seon_protoid;	/* NSEL */
// };
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_eon {
    pub seon_len: u8,
    pub seon_family: u8,
    pub seon_status: u8,
    pub seon_pad1: u8,
    pub seon_adrlen: u8,
    pub seon_afi: u8,
    pub seon_idi: [u8;2],
    pub seon_vers: u8,
    pub seon_glbnum: [u8;2],
    pub seon_RDN: [u8;2],
    pub seon_pad2: [u8;3],
    pub seon_LAREA: [u8;2],
    pub seon_pad3: [u8;2],
    pub seon_ipaddr: u32,
    pub seon_protoid: u8
}

//  #ifndef BURN_BRIDGES    /* Can be used by third party software. */
//  struct sockaddr_inarp {
//          u_char  sin_len;
//          u_char  sin_family;
//          u_short sin_port;
//          struct  in_addr sin_addr;
//          struct  in_addr sin_srcaddr;
//          u_short sin_tos;
//          u_short sin_other;
//  #define SIN_PROXY 1
//  };
//  #endif /* !BURN_BRIDGES  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_inarp {
    pub sin_len: u8,
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_srcaddr: in_addr,
    pub sin_tos: u16,
    pub sin_other: u16,
}

type __be32 = u32;

pub const IPX_NODE_LEN: usize = 6;

//     9 struct sockaddr_ipx {
//    10     __kernel_sa_family_t sipx_family;
//    11     __be16      sipx_port;
//    12     __be32      sipx_network;
//    13     unsigned char   sipx_node[IPX_NODE_LEN];
//    14     __u8        sipx_type;
//    15     unsigned char   sipx_zero;  /* 16 byte fill */
//    16 };
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_ipx {
    sipx_family: __kernel_sa_family_t,
    sipx_port: __be16,
    sipx_network: __be32,
    sipx_node: [u8;IPX_NODE_LEN],
    sipx_type: __u8,
    sipx_zero: u8
}


// struct iso_addr {
// 	u_char	isoa_len;						/* length (in bytes) */
// 	char	isoa_genaddr[20];				/* general opaque address */
// };
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iso_addr{
    pub isoa_len: u8,
    pub isoa_genaddr: [u8;20]
}

// struct sockaddr_iso {
// 	u_char	 			siso_len;			/* length */
// 	u_char	 			siso_family;		/* family */
// 	u_char				siso_plen;			/* presentation selector length */
// 	u_char				siso_slen;			/* session selector length */
// 	u_char				siso_tlen;			/* transport selector length */
// 	struct 	iso_addr	siso_addr;			/* network address */
// 	u_char				siso_pad[6];		/* space for gosip v2 sels */
// 											/* makes struct 32 bytes long */
// };
#[derive(Copy,Clone)]
#[repr(C)]
pub struct sockaddr_iso {
    siso_len: u8,
    siso_family: u8,
    siso_plen: u8,
    siso_slen: u8,
    siso_tlen: u8,
    siso_addr: iso_addr,
    siso_pad: [u8;6]
}


//  union sockaddr_ns {
//    struct sockaddr sa;
//    struct sockaddr_in sin;
//  #ifdef HAVE_IPv6
//    struct sockaddr_in6 sin6;
//  #endif
//  };
#[derive(Copy,Clone)]
#[repr(C)]
pub struct sockaddr_ns {
    sa: sockaddr,
    sin: sockaddr_in,
    sin6: sockaddr_in6
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct x25_address {
    pub x25_addr: [char;16]
}

// struct sockaddr_x25 {
//    61     __kernel_sa_family_t sx25_family;   /* Must be AF_X25 */
//    62     struct x25_address sx25_addr;       /* X.121 Address */
//    63 };
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_x25 {
    sx25_family: __kernel_sa_family_t,
    sx25_addr: x25_address
}



#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}

pub type __clock_t = libc::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type ssize_t = __ssize_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type __socket_type = u32;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub const SHUT_RDWR: u8 = 2;
pub const SHUT_WR: u8 = 1;
pub const SHUT_RD: u8 = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [i8; 108],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}

pub const IPPROTO_MAX: i32 = 256;
pub const IPPROTO_RAW: i32 = 255;
pub const IPPROTO_MPLS: i32 = 137;
pub const IPPROTO_UDPLITE: i32 = 136;
pub const IPPROTO_SCTP: i32 = 132;
pub const IPPROTO_COMP: i32 = 108;
pub const IPPROTO_PIM: i32 = 103;
pub const IPPROTO_ENCAP: i32 = 98;
pub const IPPROTO_BEETPH: i32 = 94;
pub const IPPROTO_MTP: i32 = 92;
pub const IPPROTO_AH: i32 = 51;
pub const IPPROTO_ESP: i32 = 50;
pub const IPPROTO_GRE: i32 = 47;
pub const IPPROTO_RSVP: i32 = 46;
pub const IPPROTO_IPV6: i32 = 41;
pub const IPPROTO_DCCP: i32 = 33;
pub const IPPROTO_TP: i32 = 29;
pub const IPPROTO_IDP: i32 = 22;
pub const IPPROTO_UDP: i32 = 17;
pub const IPPROTO_PUP: i32 = 12;
pub const IPPROTO_EGP: i32 = 8;
pub const IPPROTO_TCP: i32 = 6;
pub const IPPROTO_IPIP: i32 = 4;
pub const IPPROTO_IGMP: i32 = 2;
pub const IPPROTO_ICMP: i32 = 1;
pub const IPPROTO_IP: i32 = 0;
pub type __u32 = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;

// __WINT_TYPE__ unsigned int
pub type __WINT_TYPE__ = u32;


#[derive(Copy, Clone)]
#[repr(C)]
pub union Unnamed29 {
    pub __wch: __WINT_TYPE__,
    pub __wchb: [i8;4],
}


#[derive(Clone, Copy)]
#[repr(C)]
pub struct __mbstate_t
{
//   int __count;
    pub __count: i32,
//   union
//   {
//     __WINT_TYPE__ __wch;
//     char __wchb[4];
//   } __value;   
             /* Value so far.  */
    pub __value: Unnamed29,
}

// __mbstate_t _IO_state;
pub type _IO_state = __mbstate_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: C2RustUnnamed_2,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: i32,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_12,
    pub sa_mask: __sigset_t,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<unsafe extern "C" fn(_: i32,
                                                  _: *mut siginfo_t,
                                                  _: *mut libc::c_void)
                                 -> ()>,
}
pub type C2RustUnnamed_13 = u32;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_13 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_13 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_13 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_13 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_13 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_13 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_13 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_13 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_13 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_13 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_13 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_13 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_13 = 236;
pub const _SC_IPV6: C2RustUnnamed_13 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_13 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_13 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_13 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_13 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_13 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_13 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_13 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_13 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_13 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_13 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_13 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_13 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_13 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_13 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_13 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_13 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_13 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_13 = 182;
pub const _SC_TRACE: C2RustUnnamed_13 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_13 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_13 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_13 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_13 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_13 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_13 = 175;
pub const _SC_STREAMS: C2RustUnnamed_13 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_13 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_13 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_13 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_13 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_13 = 169;
pub const _SC_2_PBS: C2RustUnnamed_13 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_13 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_13 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_13 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_13 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_13 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_13 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_13 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_13 = 160;
pub const _SC_SPAWN: C2RustUnnamed_13 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_13 = 158;
pub const _SC_SHELL: C2RustUnnamed_13 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_13 = 156;
pub const _SC_REGEXP: C2RustUnnamed_13 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_13 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_13 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_13 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_13 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_13 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_13 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_13 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_13 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_13 = 146;
pub const _SC_PIPE: C2RustUnnamed_13 = 145;
pub const _SC_FIFO: C2RustUnnamed_13 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_13 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_13 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_13 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_13 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_13 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_13 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_13 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_13 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_13 = 135;
pub const _SC_BASE: C2RustUnnamed_13 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_13 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_13 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_13 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_13 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_13 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_13 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_13 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_13 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_13 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_13 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_13 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_13 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_13 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_13 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_13 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_13 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_13 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_13 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_13 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_13 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_13 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_13 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_13 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_13 = 110;
pub const _SC_NZERO: C2RustUnnamed_13 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_13 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_13 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_13 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_13 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_13 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_13 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_13 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_13 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_13 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_13 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_13 = 98;
pub const _SC_2_UPE: C2RustUnnamed_13 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_13 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_13 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_13 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_13 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_13 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_13 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_13 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_13 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_13 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_13 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_13 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_13 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_13 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_13 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_13 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_13 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_13 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_13 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_13 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_13 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_13 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_13 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_13 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_13 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_13 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_13 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_13 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_13 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_13 = 68;
pub const _SC_THREADS: C2RustUnnamed_13 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_13 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_13 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_13 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_13 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_13 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_13 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_13 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_13 = 60;
pub const _SC_SELECT: C2RustUnnamed_13 = 59;
pub const _SC_POLL: C2RustUnnamed_13 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_13 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_13 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_13 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_13 = 54;
pub const _SC_PII: C2RustUnnamed_13 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_13 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_13 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_13 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_13 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_13 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_13 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_13 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_13 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_13 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_13 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_13 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_13 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_13 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_13 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_13 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_13 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_13 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_13 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_13 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_13 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_13 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_13 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_13 = 30;
pub const _SC_VERSION: C2RustUnnamed_13 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_13 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_13 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_13 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_13 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_13 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_13 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_13 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_13 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_13 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_13 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_13 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_13 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_13 = 16;
pub const _SC_FSYNC: C2RustUnnamed_13 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_13 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_13 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_13 = 12;
pub const _SC_TIMERS: C2RustUnnamed_13 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_13 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_13 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_13 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_13 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_13 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_13 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_13 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_13 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_13 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_13 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_13 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: String,
    pub pw_passwd: String,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: String,
    pub pw_dir: String,
    pub pw_shell: String,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: String,
    pub gr_passwd: String,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut String,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip {
    // #[bitfield(name = "ip_hl", ty = "u32", bits = "0..=3")]
    // #[bitfield(name = "ip_v", ty = "u32", bits = "4..=7")]
    pub ip_hl_ip_v: [u8; 1],
    pub ip_tos: u8,
    pub ip_len: u16,
    pub ip_id: u16,
    pub ip_off: u16,
    pub ip_ttl: u8,
    pub ip_p: u8,
    pub ip_sum: u16,
    pub ip_src: in_addr,
    pub ip_dst: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp_ra_addr {
    pub ira_addr: u32,
    pub ira_preference: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp {
    pub icmp_type: u8,
    pub icmp_code: u8,
    pub icmp_cksum: u16,
    pub icmp_hun: C2RustUnnamed_17,
    pub icmp_dun: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub id_ts: C2RustUnnamed_16,
    pub id_ip: C2RustUnnamed_15,
    pub id_radv: icmp_ra_addr,
    pub id_mask: u32,
    pub id_data: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub idi_ip: ip,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub its_otime: u32,
    pub its_rtime: u32,
    pub its_ttime: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub ih_pptr: u8,
    pub ih_gwaddr: in_addr,
    pub ih_idseq: ih_idseq,
    pub ih_void: u32,
    pub ih_pmtu: ih_pmtu,
    pub ih_rtradv: ih_rtradv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_rtradv {
    pub irt_num_addrs: u8,
    pub irt_wpa: u8,
    pub irt_lifetime: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_pmtu {
    pub ipm_void: u16,
    pub ipm_nextmtu: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_idseq {
    pub icd_id: u16,
    pub icd_seq: u16,
}


pub type libc_lock_t = i32;

// struct __dirstream
//   {
//     int fd;                        /* File descriptor.  */
//     __libc_lock_define (, lock) /* Mutex lock for this structure.  */
//     size_t allocation;                /* Space allocated for the block.  */
//     size_t size;                /* Total valid data in the block.  */
//     size_t offset;                /* Current offset into the block.  */
//     off_t filepos;                /* Position of next entry to read.  */
//     int errcode;                /* Delayed error code.  */
//     /* Directory block.  We must make sure that this block starts
//        at an address that is aligned adequately enough to store
//        dirent entries.  Using the alignment of "void *" is not
//        sufficient because dirents on 32-bit platforms can require
//        64-bit alignment.  We use "long double" here to be consistent
//        with what malloc uses.  */
//     char data[0] __attribute__ ((aligned (__alignof__ (long double))));
//   };
pub struct __dirstream {
    pub fd: i32,
    pub lock: libc_lock_t,
    pub allocation: libc::size_t,
    pub size: libc::size_t,
    pub offset: libc::size_t,
    pub filepos: libc::off_t,
    pub errcode: i32,
}


pub type DIR = __dirstream;
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct __user_cap_header_struct {
    pub version: __u32,
    pub pid: i32,
}
pub type cap_user_header_t = *mut __user_cap_header_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __user_cap_data_struct {
    pub effective: __u32,
    pub permitted: __u32,
    pub inheritable: __u32,
}

// pub type cap_user_data_t = *mut __user_cap_data_struct;
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct event_desc {
    pub event: i32,
    pub data: i32,
    pub msg_sz: i32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub keytag: u16,
    pub algo: u16,
    pub digest: u16,
    pub rcode: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub target: *mut blockdata,
    pub targetlen: u16,
    pub srvport: u16,
    pub priority: u16,
    pub weight: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub keydata: *mut blockdata,
    pub keylen: u16,
    pub keytag: u16,
    pub algo: u8,
    pub digest: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub keydata: *mut blockdata,
    pub keylen: u16,
    pub flags: u16,
    pub keytag: u16,
    pub algo: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub target: C2RustUnnamed_23,
    pub uid: u32,
    pub is_name_ptr: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
    pub cache: *mut crec,
    pub name: String,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_24 {
    pub sname: [i8; 50],
    pub bname: *mut bigname,
    pub namep: String,
}

pub type wint_t = u32;


pub type __gconv_fct = fn(*mut __gconv_step, *mut __gconv_step_data, *mut*mut u8, *mut u8, *mut*mut u8, libc::size_t, i32, i32) -> i32;
pub type __gconv_btowc_fct = fn(*mut __gconv_step, u8) -> wint_t;
pub type __gconv_init_fct = fn(*mut __gconv_step) -> i32;
pub type __gconv_end_fct = fn(*mut __gconv_step);
pub type __gconv_trans_fct = fn(*mut __gconv_step, *mut __gconv_step_data, *mut libc::c_void, *mut u8, *mut*mut u8, *mut u8, *mut*mut u8, *mut libc::size_t);
pub type __gconv_trans_context_fct = fn(*mut libc::c_void, *mut u8, *mut u8, *mut u8, *mut u8) -> i32;
pub type __gconv_trans_query_fct = fn(String, *mut*mutString, *mut libc::size_t);
pub type __gconv_trans_init_fct = fn(*mut*mut libc::c_void, String) -> i32;

pub type __gconv_trans_end_fct = fn(*mut libc::c_void);



// struct __gconv_loaded_object
// {
//   /* Name of the object.  It must be the first structure element.  */
//   const char *name;
//   /* Reference counter for the db functionality.  If no conversion is
//      needed we unload the db library.  */
//   int counter;
//   /* The handle for the shared object.  */
//   void *handle;
//   /* Pointer to the functions the module defines.  */
//   __gconv_fct fct;
//   __gconv_init_fct init_fct;
//   __gconv_end_fct end_fct;
// };
#[derive(Copy,Clone)]
#[repr(C)]
pub struct __gconv_loaded_object {
    pub name: String,
    pub counter: i32,
    pub handle: *mut libc::c_void,
    pub fct: __gconv_fct,
    pub init_fct: __gconv_init_fct,
    pub end_fct: __gconv_end_fct,
}


// struct __gconv_step
// {
//   struct __gconv_loaded_object *__shlib_handle;
//   __const char *__modname;
//   int __counter;
//   char *__from_name;
//   char *__to_name;
//   __gcon_fct __fct;
//   __gconv_btowc_fct __btowc_fct;
//   __gconv_init_fct __init_fct;
//   __gconv_end_fct __end_fct;
//   int __min_needed_from;
//   int __max_needed_from;
//   int __min_needed_to;
//   int __max_needed_to;
//   int __stateful;
//   void *__data;
// };
#[derive(Copy,Clone)]
#[repr(C)]
pub struct __gconv_step {
    pub __shlib_handle: *mut __gconv_loaded_object,
    pub __modname: String,
    pub __counter: i32,
    pub __from_name: String,
    pub __to_name: String,
    pub __fct: __gconv_fct,
    pub __btowc_fct: __gconv_btowc_fct,
    pub __init_fct: __gconv_init_fct,
    pub __end_fct: __gconv_end_fct,
    pub __min_needed_from: i32,
    pub __max_needed_from: i32,
    pub __min_needed_to: i32,
    pub __max_needed_to: i32,
    pub __data: *mut libc::c_void,
}




// struct __gconv_trans_data
// {
//   __gconv_trans_fct __trans_fct;
//   __gconv_trans_context_fct __trans_context_fct;
//   __gconv_trans_end_fct __trans_end_fct;
//   void *__data;
//   struct __gconv_trans_data *__next;
// };
#[derive(Copy,Clone)]
#[repr(C)]
pub struct __gconv_trans_data {
    pub __trans_fct: __gconv_trans_fct,
    pub __trans_context_fct: __gconv_trans_context_fct,
    pub __trans_end_fct: __gconv_trans_end_fct,
    pub __data: *mut libc::c_void,
    pub __next: *mut __gconv_trans_data,
}


// struct __gconv_step_data
// {
//   unsigned char *__outbuf;
//   unsigned char *__outbufend;
//   int __flags;
//   int __invocation_counter;
//   int __internal_use;
//   __mbstate_t *__statep;
//   __mbstate_t __state;
//   struct __gconv_trans_data *__trans;
// };
#[derive(Copy,Clone)]
#[repr(C)]
pub struct __gconv_step_data {
    pub __outbuf: *mut u8,
    pub __outbufend: *mut u8,
    pub __flags: i32,
    pub __invocation_counter: i32,
    pub __internal_use: i32,
    pub __statep: *mut __mbstate_t,
    pub __state: __mbstate_t,
    pub __trans: *mut __gconv_trans_data,
}



// typedef struct __gconv_info
// {
//   size_t __nsteps;
//   struct __gconv_step *__steps;
//   __extension__ struct __gconv_step_data __data [];
// } *__gconv_t;
#[derive(Copy,Clone)]
#[repr(C)]
pub struct __gconv_info {
    pub __nsteps: libc::size_t,
    pub __steps: *mut __gconv_step,
    pub __data: *mut __gconv_step_data
}

// typedef union
// {
//   struct __gconv_info __cd;
//   struct
//   {
//     struct __gconv_info __cd;
//     struct __gconv_step_data __data;
//   } __combined;
// } _IO_iconv_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Unnamed_28 {
    pub __cd: __gconv_info,
    pub __data: __gconv_step_data
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union _IO_iconv_t {
    pub __cd: __gconv_info,
    pub __combined: Unnamed_28,
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_lease {
    pub clid_len: i32,
    pub clid: *mut u8,
    pub hostname: String,
    pub fqdn: String,
    pub old_hostname: String,
    pub flags: i32,
    pub expires: time_t,
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub hwaddr: [u8; 16],
    pub addr: in_addr,
    pub override_0: in_addr,
    pub giaddr: in_addr,
    pub extradata: *mut u8,
    pub extradata_len: u32,
    pub extradata_size: u32,
    pub last_interface: i32,
    pub new_interface: i32,
    pub new_prefixlen: i32,
    pub addr6: in6_addr,
    pub iaid: u32,
    pub slaac_address: slaac_address,
    pub vendorclass_count: i32,
    // pub next: *mut dhcp_lease,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slaac_address {
    pub addr: in6_addr,
    pub ping_time: time_t,
    pub backoff: i32,
    // pub next: *mut slaac_address,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub encap: i32,
    pub wildcard_mask: u32,
    pub vendor_class: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub ip: ip,
    pub icmp: icmp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub ip: ip,
    pub icmp: icmp,
}

#[derive(Copy,Clone)]
#[repr(C)]
pub struct _IO_marker {
  pub next: *mut _IO_marker,
  pub sbuf: *mut FILE,
  /* If _pos >= 0
 it points to _buf->Gbase()+_pos. FIXME comment */
  /* if _pos < 0, it points to _buf->eBptr()+_pos. FIXME comment */
  pub _pos: i32
}


pub enum __codecvt_result
{
  __codecvt_ok,
  __codecvt_partial,
  __codecvt_error,
  __codecvt_noconv
}


#[derive(Copy, Clone)]
#[repr(C)]
struct _IO_codecvt
{
//   void (*__codecvt_destr) (struct _IO_codecvt *);
  pub __codecvt_destr: fn(&mut _IO_codecvt),
//   enum __codecvt_result (*__codecvt_do_out) (struct _IO_codecvt *,
//                                              __mbstate_t *,
//                                              const wchar_t *,
//                                              const wchar_t *,
//                                              const wchar_t **, char *,
//                                              char *, char **);
    pub __codecvt_do_out: fn(*mut _IO_codecvt, *mut __mbstate_t, *mut libc::wchar_t, *mut libc::wchar_t, *mut*mut libc::wchar_t, String, String, *mutString) -> __codecvt_result,
//   enum __codecvt_result (*__codecvt_do_unshift) (struct _IO_codecvt *,
//                                                  __mbstate_t *, char *,
//                                                  char *, char **);
    pub __codecvt_do_unshift: fn(*mut _IO_codecvt, *mut __mbstate_t, String, String, *mutString) -> __codecvt_result,
//   enum __codecvt_result (*__codecvt_do_in) (struct _IO_codecvt *,
//                                             __mbstate_t *,
//                                             const char *, const char *,
//                                             const char **, wchar_t *,
//                                             wchar_t *, wchar_t **);
    pub __codecvt_do_in: fn(*mut _IO_codecvt, *mut __mbstate_t, String, String, *mutString, *mut libc::wchar_t, *mut libc::wchar_t, *mut*mut libc::wchar_t) -> __codecvt_result,
//   int (*__codecvt_do_encoding) (struct _IO_codecvt *);
    pub __codecvt_do_encoding: fn(*mut _IO_codecvt) -> i32,
//   int (*__codecvt_do_always_noconv) (struct _IO_codecvt *);
    pub __codecvt_do_always_noconv: fn(*mut _IO_codecvt) -> i32,
//   int (*__codecvt_do_length) (struct _IO_codecvt *, __mbstate_t *,
//                               const char *, const char *, size_t);
    pub __codecvt_do_length: fn(*mut _IO_codecvt, *mut __mbstate_t, String) -> i32,
//   int (*__codecvt_do_max_length) (struct _IO_codecvt *);
    pub __codecvt_do_max_length: fn(*mut _IO_codecvt) -> i32,
//   _IO_iconv_t __cd_in;
    pub __cd_in: _IO_iconv_t,
//   _IO_iconv_t __cd_out;
    pub __cd_out: _IO_iconv_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _code {
    pub c_name: String,
    pub c_val: i32,
}

pub type CODE = _code;

#[inline]
unsafe extern "C" fn __uint16_identity(mut __x: u16) -> u16 {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint32_identity(mut __x: u32) -> u32 {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
    return __x;
}

pub type C2RustUnnamed_1 = u32;
pub const _ISalnum: C2RustUnnamed_1 = 8;
pub const _ISpunct: C2RustUnnamed_1 = 4;
pub const _IScntrl: C2RustUnnamed_1 = 2;
pub const _ISblank: C2RustUnnamed_1 = 1;
pub const _ISgraph: C2RustUnnamed_1 = 32768;
pub const _ISprint: C2RustUnnamed_1 = 16384;
pub const _ISspace: C2RustUnnamed_1 = 8192;
pub const _ISxdigit: C2RustUnnamed_1 = 4096;
pub const _ISdigit: C2RustUnnamed_1 = 2048;
pub const _ISalpha: C2RustUnnamed_1 = 1024;
pub const _ISlower: C2RustUnnamed_1 = 512;
pub const _ISupper: C2RustUnnamed_1 = 256;
pub type intmax_t = libc::c_long;
pub type uintmax_t = libc::c_ulong;
pub type __gwchar_t = i32;

pub const MSG_CMSG_CLOEXEC: u32 = 1073741824;
pub const MSG_FASTOPEN: u32 = 536870912;
pub const MSG_ZEROCOPY: u32 = 67108864;
pub const MSG_BATCH: u32 = 262144;
pub const MSG_WAITFORONE: u32 = 65536;
pub const MSG_MORE: u32 = 32768;
pub const MSG_NOSIGNAL: u32 = 16384;
pub const MSG_ERRQUEUE: u32 = 8192;
pub const MSG_RST: u32 = 4096;
pub const MSG_CONFIRM: u32 = 2048;
pub const MSG_SYN: u32 = 1024;
pub const MSG_FIN: u32 = 512;
pub const MSG_WAITALL: u32 = 256;
pub const MSG_EOR: u32 = 128;
pub const MSG_DONTWAIT: u32 = 64;
pub const MSG_TRUNC: u32 = 32;
pub const MSG_PROXY: u32 = 16;
pub const MSG_CTRUNC: u32 = 8;
pub const MSG_TRYHARD: u32 = 4;
pub const MSG_DONTROUTE: u32 = 4;
pub const MSG_PEEK: u32 = 2;
pub const MSG_OOB: u32 = 1;

// pub type _IO_wide_data;
// pub type _IO_codecvt;
// pub type _IO_marker;
// #[no_mangle]
// fn recvmsg(__fd: i32, __message: *mut msghdr,
//            __flags: i32) -> ssize_t;
// #[no_mangle]
// fn setsockopt(__fd: i32, __level: i32,
//               __optname: i32, __optval: *const libc::c_void,
//               __optlen: socklen_t) -> i32;
// #[no_mangle]
// fn inet_ntop(__af: i32, __cp: *const libc::c_void,
//              __buf: *mut libc::c_char, __len: socklen_t)
//  -> *const libc::c_char;
// #[no_mangle]
// fn __xstat(__ver: i32, __filename: *const libc::c_char,
//            __stat_buf: *mut stat) -> i32;
// #[no_mangle]
// fn __fxstat(__ver: i32, __fildes: i32,
//             __stat_buf: *mut stat) -> i32;
// #[no_mangle]
// fn __xstat64(__ver: i32, __filename: *const libc::c_char,
//              __stat_buf: *mut stat64) -> i32;
// #[no_mangle]
// fn __fxstat64(__ver: i32, __fildes: i32,
//               __stat_buf: *mut stat64) -> i32;
// #[no_mangle]
// fn __fxstatat(__ver: i32, __fildes: i32,
//               __filename: *const libc::c_char, __stat_buf: *mut stat,
//               __flag: i32) -> i32;
// #[no_mangle]
// fn __fxstatat64(__ver: i32, __fildes: i32,
//                 __filename: *const libc::c_char, __stat_buf: *mut stat64,
//                 __flag: i32) -> i32;
// #[no_mangle]
// fn __lxstat(__ver: i32, __filename: *const libc::c_char,
//             __stat_buf: *mut stat) -> i32;
// #[no_mangle]
// fn __lxstat64(__ver: i32, __filename: *const libc::c_char,
//               __stat_buf: *mut stat64) -> i32;
// #[no_mangle]
// fn __xmknod(__ver: i32, __path: *const libc::c_char,
//             __mode: __mode_t, __dev: *mut __dev_t) -> i32;
// #[no_mangle]
// fn __xmknodat(__ver: i32, __fd: i32,
//               __path: *const libc::c_char, __mode: __mode_t,
//               __dev: *mut __dev_t) -> i32;
// #[no_mangle]
// fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
//  -> *mut libc::c_void;
// #[no_mangle]
// fn memset(_: *mut libc::c_void, _: i32, _: libc::c_ulong)
//  -> *mut libc::c_void;
// #[no_mangle]
// fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
//           _: libc::c_ulong) -> i32;
// #[no_mangle]
// fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
//  -> *mut libc::c_char;
// #[no_mangle]
// fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
//  -> *mut libc::c_char;
// #[no_mangle]
// fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
//  -> *mut libc::c_char;
// #[no_mangle]
// fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
// #[no_mangle]
// fn strchr(_: *const libc::c_char, _: i32) -> *mut libc::c_char;
// #[no_mangle]
// fn strlen(_: *const libc::c_char) -> libc::c_ulong;
// #[no_mangle]
// fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
//  -> i32;
// #[no_mangle]
// static mut stdin: *mut FILE;
// #[no_mangle]
// static mut stdout: *mut FILE;
// #[no_mangle]
// fn printf(_: *const libc::c_char, _: ...) -> i32;
// #[no_mangle]
// fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
//  -> i32;
//
// #[no_mangle]
// fn getc(__stream: *mut FILE) -> i32;
// #[no_mangle]
// fn __uflow(_: *mut FILE) -> i32;
// #[no_mangle]
// fn putc(__c: i32, __stream: *mut FILE) -> i32;
// #[no_mangle]
// fn __overflow(_: *mut FILE, _: i32) -> i32;
// #[no_mangle]
// fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
//               __delimiter: i32, __stream: *mut FILE) -> __ssize_t;
// #[no_mangle]
// fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char)
//  -> libc::c_double;
// #[no_mangle]
// fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
//           _: i32) -> libc::c_long;
// #[no_mangle]
// fn strtoll(_: *const libc::c_char, _: *mut *mut libc::c_char,
//            _: i32) -> libc::c_longlong;
// #[no_mangle]
// fn __ctype_b_loc() -> *mut *const u16;
// #[no_mangle]
// fn __ctype_tolower_loc() -> *mut *const __int32_t;
// #[no_mangle]
// fn __ctype_toupper_loc() -> *mut *const __int32_t;
// #[no_mangle]
// fn __errno_location() -> *mut i32;
// #[no_mangle]
// fn __strtol_internal(__nptr: *const libc::c_char,
//                      __endptr: *mut *mut libc::c_char,
//                      __base: i32, __group: i32)
//  -> libc::c_long;
// #[no_mangle]
// fn __strtoul_internal(__nptr: *const libc::c_char,
//                       __endptr: *mut *mut libc::c_char,
//                       __base: i32, __group: i32)
//  -> libc::c_ulong;
// #[no_mangle]
// fn __wcstol_internal(__nptr: *const __gwchar_t,
//                      __endptr: *mut *mut __gwchar_t, __base: i32,
//                      __group: i32) -> libc::c_long;
// #[no_mangle]
// fn __wcstoul_internal(__nptr: *const __gwchar_t,
//                       __endptr: *mut *mut __gwchar_t, __base: i32,
//                       __group: i32) -> libc::c_ulong;
// #[no_mangle]
// static mut dnsmasq_daemon: *mut dnsmasq_daemon;
// #[no_mangle]
// fn cache_find_by_name(crecp: *mut crec, name: *mut libc::c_char,
//                       now: time_t, prot: u32) -> *mut crec;
// #[no_mangle]
// fn safe_malloc(size: size_t) -> *mut libc::c_void;
// #[no_mangle]
// fn whine_malloc(size: size_t) -> *mut libc::c_void;
// #[no_mangle]
// fn hostname_isequal(a: *const libc::c_char, b: *const libc::c_char)
//  -> i32;
// #[no_mangle]
// fn is_same_net(a: in_addr, b: in_addr, mask: in_addr) -> i32;
// #[no_mangle]
// fn is_same_net6(a: *mut in6_addr, b: *mut in6_addr,
//                 prefixlen: i32) -> i32;
// #[no_mangle]
// fn setaddr6part(addr: *mut in6_addr, host: u64_0);
// #[no_mangle]
// fn prettyprint_time(buf: *mut libc::c_char, t: u32);
// #[no_mangle]
// fn memcmp_masked(a: *mut u8, b: *mut u8,
//                  len: i32, mask: u32) -> i32;
// #[no_mangle]
// fn expand_buf(iov: *mut iovec, size: size_t) -> i32;
// #[no_mangle]
// fn print_mac(buff: *mut libc::c_char, mac: *mut u8,
//              len: i32) -> *mut libc::c_char;
// #[no_mangle]
// fn die(message: *mut libc::c_char, arg1: *mut libc::c_char,
//        exit_code: i32) -> !;
// #[no_mangle]
// fn my_syslog(priority: i32, format: *const libc::c_char, _: ...);
// #[no_mangle]
// fn config_find_by_address6(configs: *mut dhcp_config, net: *mut in6_addr,
//                            prefix: i32, addr: *mut in6_addr)
//  -> *mut dhcp_config;
// #[no_mangle]
// fn indextoname(fd: i32, index: i32,
//                name: *mut libc::c_char) -> i32;
// #[no_mangle]
// fn config_find_by_address(configs: *mut dhcp_config, addr: in_addr)
//  -> *mut dhcp_config;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_packet {
    pub op: u8_0,
    pub htype: u8_0,
    pub hlen: u8_0,
    pub hops: u8_0,
    pub xid: u32_0,
    pub secs: u16_0,
    pub flags: u16_0,
    pub ciaddr: in_addr,
    pub yiaddr: in_addr,
    pub siaddr: in_addr,
    pub giaddr: in_addr,
    pub chaddr: [u8_0; 16],
    pub sname: [u8_0; 64],
    pub file: [u8_0; 128],
    pub options: [u8_0; 312],
}

pub static PRIORITY_NAMES: [CODE; 13] =
    [ _code{c_name: String::from("alert"), c_val: 1},
        _code{c_name: String::from("crit"), c_val: 2},
        _code{c_name: String::from("debug"), c_val: 7},
        _code{c_name: String::from("emerg"), c_val: 0},
        _code{c_name: String::from("err"), c_val: 3},
        _code{c_name: String::from("error"), c_val: 3},
        _code{c_name: String::from("info"), c_val: 6},
        _code{c_name: String::from("none"), c_val: 0x10},
        _code{c_name: String::from("notice"), c_val: 5},
        _code{c_name: String::from("panic"), c_val: 0},
        _code{c_name: String::from("warn"), c_val: 4},
        _code{c_name: String::from("warning"), c_val: 4},
        _code{c_name: String::from(""), c_val: -1}];

pub static FACILITYNAMES: [CODE; 23] = [
    _code{c_name: String::from("auth"), c_val: 4 << 3},
    _code{c_name: String::from("authpriv"), c_val: 10 << 3},
    _code{c_name: String::from("cron"), c_val: 9 << 3},
    _code{c_name: String::from("daemon"), c_val: 3 << 3},
    _code{c_name: String::from("ftp"), c_val: 11 << 3},
    _code{c_name: String::from("kern"), c_val: 0 << 3},
    _code{c_name: String::from("lpr"), c_val: 6 << 3},
    _code{c_name: String::from("mail"), c_val: 2 << 3},
    _code{c_name: String::from("mark"), c_val: (24 << 3) | 0},
    _code{c_name: String::from("news"), c_val: 7 << 3},
    _code{c_name: String::from("security"), c_val: 4 << 3},
    _code{c_name: String::from("syslog"), c_val: 5 << 3},
    _code{c_name: String::from("user"), c_val: 1 << 3},
    _code{c_name: String::from("uucp"), c_val: 8 << 3},
    _code{c_name: String::from("local0"), c_val: 16 << 3},
    _code{c_name: String::from("local1"), c_val: 17 << 3},
    _code{c_name: String::from("local2"), c_val: 18 << 3},
    _code{c_name: String::from("local3"), c_val: 19 << 3},
    _code{c_name: String::from("local4"), c_val: 20 << 3},
    _code{c_name: String::from("local5"), c_val: 21 << 3},
    _code{c_name: String::from("local6"), c_val: 22 << 3},
    _code{c_name: String::from("local7"), c_val: 23 << 3},
    _code{c_name: String::from(""), c_val:-1},
];

#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
               __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int,
                __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn __xstat64(__ver: libc::c_int, __filename: *const libc::c_char,
                 __stat_buf: *mut stat64) -> libc::c_int;
    #[no_mangle]
    fn __fxstat64(__ver: libc::c_int, __fildes: libc::c_int,
                  __stat_buf: *mut stat64) -> libc::c_int;
    #[no_mangle]
    fn __fxstatat(__ver: libc::c_int, __fildes: libc::c_int,
                  __filename: *const libc::c_char, __stat_buf: *mut stat,
                  __flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __fxstatat64(__ver: libc::c_int, __fildes: libc::c_int,
                    __filename: *const libc::c_char, __stat_buf: *mut stat64,
                    __flag: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __lxstat(__ver: libc::c_int, __filename: *const libc::c_char,
                __stat_buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn __lxstat64(__ver: libc::c_int, __filename: *const libc::c_char,
                  __stat_buf: *mut stat64) -> libc::c_int;
    #[no_mangle]
    fn __xmknod(__ver: libc::c_int, __path: *const libc::c_char,
                __mode: __mode_t, __dev: *mut __dev_t) -> libc::c_int;
    #[no_mangle]
    fn __xmknodat(__ver: libc::c_int, __fd: libc::c_int,
                  __path: *const libc::c_char, __mode: __mode_t,
                  __dev: *mut __dev_t) -> libc::c_int;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
                -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __uflow(_: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
                  __delimiter: libc::c_int, __stream: *mut FILE) -> __ssize_t;
    #[no_mangle]
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char)
              -> libc::c_double;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn strtoll(_: *const libc::c_char, _: *mut *mut libc::c_char,
               _: libc::c_int) -> libc::c_longlong;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __strtol_internal(__nptr: *const libc::c_char,
                         __endptr: *mut *mut libc::c_char,
                         __base: libc::c_int, __group: libc::c_int)
                         -> libc::c_long;
    #[no_mangle]
    fn __strtoul_internal(__nptr: *const libc::c_char,
                          __endptr: *mut *mut libc::c_char,
                          __base: libc::c_int, __group: libc::c_int)
                          -> libc::c_ulong;
    #[no_mangle]
    fn __wcstol_internal(__nptr: *const __gwchar_t,
                         __endptr: *mut *mut __gwchar_t, __base: libc::c_int,
                         __group: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __wcstoul_internal(__nptr: *const __gwchar_t,
                          __endptr: *mut *mut __gwchar_t, __base: libc::c_int,
                          __group: libc::c_int) -> libc::c_ulong;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __blkcnt64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: libc::c_int,
    pub cmsg_type: libc::c_int,
    pub __cmsg_data: [libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat64 {
    pub st_dev: __dev_t,
    pub st_ino: __ino64_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt64_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type __compar_fn_t
=
Option<unsafe extern "C" fn(_: *const libc::c_void,
                            _: *const libc::c_void) -> libc::c_int>;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type __gwchar_t = libc::c_int;
#[inline]
unsafe extern "C" fn vprintf(mut __fmt: *const libc::c_char,
                             mut __arg: ::std::ffi::VaList) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int { return getc(stdin); }
#[inline]
unsafe extern "C" fn fgetc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int as
        libc::c_long != 0 {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int as
        libc::c_long != 0 {
        __uflow(__fp)
    } else {
        let fresh1 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
        *(fresh1 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn getchar_unlocked() -> libc::c_int {
    return if ((*stdin)._IO_read_ptr >= (*stdin)._IO_read_end) as libc::c_int
        as libc::c_long != 0 {
        __uflow(stdin)
    } else {
        let fresh2 = (*stdin)._IO_read_ptr;
        (*stdin)._IO_read_ptr = (*stdin)._IO_read_ptr.offset(1);
        *(fresh2 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn wcstoumax(mut nptr: *const __gwchar_t,
                               mut endptr: *mut *mut __gwchar_t,
                               mut base: libc::c_int) -> uintmax_t {
    return __wcstoul_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int |
        (__bsx as libc::c_int & 0xff as libc::c_int) <<
            8 as libc::c_int) as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
        (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
        (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
        (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
    return ((__bsx as libc::c_ulonglong &
        0xff00000000000000 as libc::c_ulonglong) >> 56 as libc::c_int
        |
        (__bsx as libc::c_ulonglong &
            0xff000000000000 as libc::c_ulonglong) >>
            40 as libc::c_int |
        (__bsx as libc::c_ulonglong &
            0xff0000000000 as libc::c_ulonglong) >> 24 as libc::c_int
        |
        (__bsx as libc::c_ulonglong &
            0xff00000000 as libc::c_ulonglong) >> 8 as libc::c_int |
        (__bsx as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong)
            << 8 as libc::c_int |
        (__bsx as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong)
            << 24 as libc::c_int |
        (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong) <<
            40 as libc::c_int |
        (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong) <<
            56 as libc::c_int) as __uint64_t;
}
#[inline]
unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
    return __x;
}
#[inline]
unsafe extern "C" fn __cmsg_nxthdr(mut __mhdr: *mut msghdr,
                                   mut __cmsg: *mut cmsghdr) -> *mut cmsghdr {
    if (*__cmsg).cmsg_len < ::std::mem::size_of::<cmsghdr>() as libc::c_ulong
    {
        return 0 as *mut cmsghdr
    }
    __cmsg =
        (__cmsg as
            *mut libc::c_uchar).offset(((*__cmsg).cmsg_len.wrapping_add(::std::mem::size_of::<size_t>()
            as
            libc::c_ulong).wrapping_sub(1
            as
            libc::c_int
            as
            libc::c_ulong)
            &
            !(::std::mem::size_of::<size_t>()
                as
                libc::c_ulong).wrapping_sub(1
                as
                libc::c_int
                as
                libc::c_ulong))
            as isize) as *mut cmsghdr;
    if __cmsg.offset(1 as libc::c_int as isize) as *mut libc::c_uchar >
        ((*__mhdr).msg_control as
            *mut libc::c_uchar).offset((*__mhdr).msg_controllen as isize)
        ||
        (__cmsg as
            *mut libc::c_uchar).offset(((*__cmsg).cmsg_len.wrapping_add(::std::mem::size_of::<size_t>()
            as
            libc::c_ulong).wrapping_sub(1
            as
            libc::c_int
            as
            libc::c_ulong)
            &
            !(::std::mem::size_of::<size_t>()
                as
                libc::c_ulong).wrapping_sub(1
                as
                libc::c_int
                as
                libc::c_ulong))
            as isize) >
            ((*__mhdr).msg_control as
                *mut libc::c_uchar).offset((*__mhdr).msg_controllen as
                isize) {
        return 0 as *mut cmsghdr
    }
    return __cmsg;
}
#[inline]
unsafe extern "C" fn fputc_unlocked(mut __c: libc::c_int,
                                    mut __stream: *mut FILE) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as
        libc::c_int as libc::c_long != 0 {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh3 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr =
            (*__stream)._IO_write_ptr.offset(1);
        *fresh3 = __c as libc::c_char;
        *fresh3 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                          mut __statbuf: *mut stat) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat)
                           -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn stat64(mut __path: *const libc::c_char,
                            mut __statbuf: *mut stat64) -> libc::c_int {
    return __xstat64(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat64(mut __fd: libc::c_int,
                             mut __statbuf: *mut stat64) -> libc::c_int {
    return __fxstat64(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn fstatat(mut __fd: libc::c_int,
                             mut __filename: *const libc::c_char,
                             mut __statbuf: *mut stat,
                             mut __flag: libc::c_int) -> libc::c_int {
    return __fxstatat(1 as libc::c_int, __fd, __filename, __statbuf, __flag);
}
#[inline]
unsafe extern "C" fn fstatat64(mut __fd: libc::c_int,
                               mut __filename: *const libc::c_char,
                               mut __statbuf: *mut stat64,
                               mut __flag: libc::c_int) -> libc::c_int {
    return __fxstatat64(1 as libc::c_int, __fd, __filename, __statbuf,
                        __flag);
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const libc::c_char,
                           mut __statbuf: *mut stat) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat64(mut __path: *const libc::c_char,
                             mut __statbuf: *mut stat64) -> libc::c_int {
    return __lxstat64(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn mknod(mut __path: *const libc::c_char,
                           mut __mode: __mode_t, mut __dev: __dev_t)
                           -> libc::c_int {
    return __xmknod(0 as libc::c_int, __path, __mode, &mut __dev);
}
#[inline]
unsafe extern "C" fn mknodat(mut __fd: libc::c_int,
                             mut __path: *const libc::c_char,
                             mut __mode: __mode_t, mut __dev: __dev_t)
                             -> libc::c_int {
    return __xmknodat(0 as libc::c_int, __fd, __path, __mode, &mut __dev);
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn putc_unlocked(mut __c: libc::c_int,
                                   mut __stream: *mut FILE) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as
        libc::c_int as libc::c_long != 0 {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh4 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr =
            (*__stream)._IO_write_ptr.offset(1);
        *fresh4 = __c as libc::c_char;
        *fresh4 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as
        libc::c_int as libc::c_long != 0 {
        __overflow(stdout, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh5 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = (*stdout)._IO_write_ptr.offset(1);
        *fresh5 = __c as libc::c_char;
        *fresh5 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn getline(mut __lineptr: *mut *mut libc::c_char,
                             mut __n: *mut size_t, mut __stream: *mut FILE)
                             -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x10 as libc::c_int != 0 as libc::c_int) as
        libc::c_int;
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as
        libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int);
}
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char)
                           -> libc::c_longlong {
    return strtoll(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                   10 as libc::c_int);
}
#[inline]
unsafe extern "C" fn bsearch(mut __key: *const libc::c_void,
                             mut __base: *const libc::c_void,
                             mut __nmemb: size_t, mut __size: size_t,
                             mut __compar: __compar_fn_t)
                             -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0 as libc::c_int as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx =
            __l.wrapping_add(__u).wrapping_div(2 as libc::c_int as
                libc::c_ulong);
        __p =
            (__base as
                *const libc::c_char).offset(__idx.wrapping_mul(__size) as
                isize) as *mut libc::c_void;
        __comparison =
            Some(__compar.expect("non-null function pointer")).expect("non-null function pointer")(__key,
                                                                                                   __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else { return __p as *mut libc::c_void }
    }
    return 0 as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else { __c };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else { __c };
}
#[inline]
unsafe extern "C" fn strtoimax(mut nptr: *const libc::c_char,
                               mut endptr: *mut *mut libc::c_char,
                               mut base: libc::c_int) -> intmax_t {
    return __strtol_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn strtoumax(mut nptr: *const libc::c_char,
                               mut endptr: *mut *mut libc::c_char,
                               mut base: libc::c_int) -> uintmax_t {
    return __strtoul_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn wcstoimax(mut nptr: *const __gwchar_t,
                               mut endptr: *mut *mut __gwchar_t,
                               mut base: libc::c_int) -> intmax_t {
    return __wcstol_internal(nptr, endptr, base, 0 as libc::c_int);
}
