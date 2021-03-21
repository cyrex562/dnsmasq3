#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]

pub type __uint8_t = libc::c_uchar;

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

pub type __pid_t = libc::c_int;

pub type __time_t = libc::c_long;

pub type __blksize_t = libc::c_long;

pub type __blkcnt_t = libc::c_long;

pub type __blkcnt64_t = libc::c_long;

pub type __ssize_t = libc::c_long;

pub type __syscall_slong_t = libc::c_long;

pub type __socklen_t = libc::c_uint;

pub type ino_t = __ino64_t;

pub type dev_t = __dev_t;

pub type off_t = __off64_t;

pub type pid_t = __pid_t;

pub type time_t = __time_t;

pub type size_t = libc::c_ulong;

pub const NULL: libc::c_int = 0 as libc::c_int;

pub const NULL_0: libc::c_int = 0 as libc::c_int;

#[derive(Copy,Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[derive(Copy,Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}

pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}

#[derive(Clone)]
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

#[derive(Clone)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: libc::c_int,
    pub cmsg_type: libc::c_int,
    pub __cmsg_data: [libc::c_uchar; 0],
}

#[inline]
pub unsafe extern "C" fn __cmsg_nxthdr(
    mut __mhdr: *mut msghdr,
    mut __cmsg: *mut cmsghdr,
) -> *mut cmsghdr {
    if (*__cmsg).cmsg_len < ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
        return 0 as *mut cmsghdr;
    }
    __cmsg = (__cmsg as *mut libc::c_uchar).offset(
        ((*__cmsg)
            .cmsg_len
            .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut cmsghdr;
    if __cmsg.offset(1 as libc::c_int as isize) as *mut libc::c_uchar
        > ((*__mhdr).msg_control as *mut libc::c_uchar).offset((*__mhdr).msg_controllen as isize)
        || (__cmsg as *mut libc::c_uchar).offset(
            ((*__cmsg)
                .cmsg_len
                .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
        ) > ((*__mhdr).msg_control as *mut libc::c_uchar)
            .offset((*__mhdr).msg_controllen as isize)
    {
        return 0 as *mut cmsghdr;
    }
    return __cmsg;
}

pub const AF_UNSPEC: libc::c_int = PF_UNSPEC;

pub const PF_UNSPEC: libc::c_int = 0 as libc::c_int;

pub const AF_INET6: libc::c_int = PF_INET6;

pub const PF_INET6: libc::c_int = 10 as libc::c_int;

pub const AF_INET: libc::c_int = PF_INET;

pub const PF_INET: libc::c_int = 2 as libc::c_int;

pub type sa_family_t = libc::c_ushort;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}

pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}

pub const INET6_ADDRSTRLEN: libc::c_int = 46 as libc::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

pub type in_addr_t = uint32_t;

pub type uint32_t = __uint32_t;

pub type uint16_t = __uint16_t;

pub type uint8_t = __uint8_t;

pub type u32_0 = libc::c_uint;

pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;


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
pub struct C2RustUnnamed_0 {
    pub keytag: libc::c_ushort,
    pub algo: libc::c_ushort,
    pub digest: libc::c_ushort,
    pub rcode: libc::c_ushort,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub target: *mut blockdata,
    pub targetlen: libc::c_ushort,
    pub srvport: libc::c_ushort,
    pub priority: libc::c_ushort,
    pub weight: libc::c_ushort,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockdata {
    pub next: *mut blockdata,
    pub key: [libc::c_uchar; 40],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub flags: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub target: C2RustUnnamed_5,
    pub uid: libc::c_uint,
    pub is_name_ptr: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub cache: *mut crec,
    pub name: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct crec {
    pub next: *mut crec,
    pub prev: *mut crec,
    pub hash_next: *mut crec,
    pub addr: all_addr,
    pub ttd: time_t,
    pub uid: libc::c_uint,
    pub flags: libc::c_uint,
    pub name: C2RustUnnamed_6,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub sname: [libc::c_char; 50],
    pub bname: *mut bigname,
    pub namep: *mut libc::c_char,
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
    pub name: *mut libc::c_char,
    pub target: *mut libc::c_char,
    pub issrv: libc::c_int,
    pub srvport: libc::c_int,
    pub priority: libc::c_int,
    pub weight: libc::c_int,
    pub offset: libc::c_uint,
    pub next: *mut mx_srv_record,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct naptr {
    pub name: *mut libc::c_char,
    pub replace: *mut libc::c_char,
    pub regexp: *mut libc::c_char,
    pub services: *mut libc::c_char,
    pub flags: *mut libc::c_char,
    pub order: libc::c_uint,
    pub pref: libc::c_uint,
    pub next: *mut naptr,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct txt_record {
    pub name: *mut libc::c_char,
    pub txt: *mut libc::c_uchar,
    pub class: libc::c_ushort,
    pub len: libc::c_ushort,
    pub stat: libc::c_int,
    pub next: *mut txt_record,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptr_record {
    pub name: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub next: *mut ptr_record,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cname {
    pub ttl: libc::c_int,
    pub flag: libc::c_int,
    pub alias: *mut libc::c_char,
    pub target: *mut libc::c_char,
    pub next: *mut cname,
    pub targetp: *mut cname,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrlist {
    pub addr: all_addr,
    pub flags: libc::c_int,
    pub prefixlen: libc::c_int,
    pub decline_time: time_t,
    pub next: *mut addrlist,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth_zone {
    pub domain: *mut libc::c_char,
    pub interface_names: *mut auth_name_list,
    pub subnet: *mut addrlist,
    pub exclude: *mut addrlist,
    pub next: *mut auth_zone,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth_name_list {
    pub name: *mut libc::c_char,
    pub flags: libc::c_int,
    pub next: *mut auth_name_list,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct host_record {
    pub ttl: libc::c_int,
    pub flags: libc::c_int,
    pub names: *mut name_list,
    pub addr: in_addr,
    pub addr6: in6_addr,
    pub next: *mut host_record,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_list {
    pub name: *mut libc::c_char,
    pub next: *mut name_list,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct interface_name {
    pub name: *mut libc::c_char,
    pub intr: *mut libc::c_char,
    pub family: libc::c_int,
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
    pub fd: libc::c_int,
    pub source_addr: mysockaddr,
    pub interface: [libc::c_char; 17],
    pub ifindex: libc::c_uint,
    pub used: libc::c_uint,
    pub preallocated: libc::c_uint,
    pub next: *mut serverfd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct randfd {
    pub fd: libc::c_int,
    pub refcount: libc::c_ushort,
    pub family: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub addr: mysockaddr,
    pub source_addr: mysockaddr,
    pub interface: [libc::c_char; 17],
    pub sfd: *mut serverfd,
    pub domain: *mut libc::c_char,
    pub flags: libc::c_int,
    pub tcpfd: libc::c_int,
    pub edns_pktsz: libc::c_int,
    pub pktsz_reduced: time_t,
    pub queries: libc::c_uint,
    pub failed_queries: libc::c_uint,
    pub uid: u32_0,
    pub next: *mut server,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipsets {
    pub sets: *mut *mut libc::c_char,
    pub domain: *mut libc::c_char,
    pub next: *mut ipsets,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct irec {
    pub addr: mysockaddr,
    pub netmask: in_addr,
    pub tftp_ok: libc::c_int,
    pub dhcp_ok: libc::c_int,
    pub mtu: libc::c_int,
    pub done: libc::c_int,
    pub warned: libc::c_int,
    pub dad: libc::c_int,
    pub dns_auth: libc::c_int,
    pub index: libc::c_int,
    pub multicast_done: libc::c_int,
    pub found: libc::c_int,
    pub label: libc::c_int,
    pub name: *mut libc::c_char,
    pub next: *mut irec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listener {
    pub fd: libc::c_int,
    pub tcpfd: libc::c_int,
    pub tftpfd: libc::c_int,
    pub used: libc::c_int,
    pub addr: mysockaddr,
    pub iface: *mut irec,
    pub next: *mut listener,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iname {
    pub name: *mut libc::c_char,
    pub addr: mysockaddr,
    pub used: libc::c_int,
    pub next: *mut iname,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mysubnet {
    pub addr: mysockaddr,
    pub addr_used: libc::c_int,
    pub mask: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct resolvc {
    pub next: *mut resolvc,
    pub is_default: libc::c_int,
    pub logged: libc::c_int,
    pub mtime: time_t,
    pub name: *mut libc::c_char,
    pub wd: libc::c_int,
    pub file: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostsfile {
    pub next: *mut hostsfile,
    pub flags: libc::c_int,
    pub fname: *mut libc::c_char,
    pub wd: libc::c_int,
    pub index: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frec {
    pub frec_src: frec_src,
    pub sentto: *mut server,
    pub rfd4: *mut randfd,
    pub rfd6: *mut randfd,
    pub new_id: libc::c_ushort,
    pub forwardall: libc::c_int,
    pub flags: libc::c_int,
    pub time: time_t,
    pub hash: [*mut libc::c_uchar; 32],
    pub next: *mut frec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frec_src {
    pub source: mysockaddr,
    pub dest: all_addr,
    pub iface: libc::c_uint,
    pub log_id: libc::c_uint,
    pub fd: libc::c_int,
    pub orig_id: libc::c_ushort,
    pub next: *mut frec_src,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_netid {
    pub net: *mut libc::c_char,
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
    pub delay: libc::c_int,
    pub netid: *mut dhcp_netid,
    pub next: *mut delay_config,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hwaddr_config {
    pub hwaddr_len: libc::c_int,
    pub hwaddr_type: libc::c_int,
    pub hwaddr: [libc::c_uchar; 16],
    pub wildcard_mask: libc::c_uint,
    pub next: *mut hwaddr_config,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_config {
    pub flags: libc::c_uint,
    pub clid_len: libc::c_int,
    pub clid: *mut libc::c_uchar,
    pub hostname: *mut libc::c_char,
    pub domain: *mut libc::c_char,
    pub netid: *mut dhcp_netid_list,
    pub filter: *mut dhcp_netid,
    pub addr6: *mut addrlist,
    pub addr: in_addr,
    pub decline_time: time_t,
    pub lease_time: libc::c_uint,
    pub hwaddr: *mut hwaddr_config,
    pub next: *mut dhcp_config,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_opt {
    pub opt: libc::c_int,
    pub len: libc::c_int,
    pub flags: libc::c_int,
    pub u: C2RustUnnamed_7,
    pub val: *mut libc::c_uchar,
    pub netid: *mut dhcp_netid,
    pub next: *mut dhcp_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub encap: libc::c_int,
    pub wildcard_mask: libc::c_uint,
    pub vendor_class: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_boot {
    pub file: *mut libc::c_char,
    pub sname: *mut libc::c_char,
    pub tftp_sname: *mut libc::c_char,
    pub next_server: in_addr,
    pub netid: *mut dhcp_netid,
    pub next: *mut dhcp_boot,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_match_name {
    pub name: *mut libc::c_char,
    pub wildcard: libc::c_int,
    pub netid: *mut dhcp_netid,
    pub next: *mut dhcp_match_name,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pxe_service {
    pub CSA: libc::c_ushort,
    pub type_0: libc::c_ushort,
    pub menu: *mut libc::c_char,
    pub basename: *mut libc::c_char,
    pub sname: *mut libc::c_char,
    pub server: in_addr,
    pub netid: *mut dhcp_netid,
    pub next: *mut pxe_service,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_vendor {
    pub len: libc::c_int,
    pub match_type: libc::c_int,
    pub enterprise: libc::c_uint,
    pub data: *mut libc::c_char,
    pub netid: dhcp_netid,
    pub next: *mut dhcp_vendor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_pxe_vendor {
    pub data: *mut libc::c_char,
    pub next: *mut dhcp_pxe_vendor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_mac {
    pub mask: libc::c_uint,
    pub hwaddr_len: libc::c_int,
    pub hwaddr_type: libc::c_int,
    pub hwaddr: [libc::c_uchar; 16],
    pub netid: dhcp_netid,
    pub next: *mut dhcp_mac,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_bridge {
    pub iface: [libc::c_char; 16],
    pub alias: *mut dhcp_bridge,
    pub next: *mut dhcp_bridge,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cond_domain {
    pub domain: *mut libc::c_char,
    pub prefix: *mut libc::c_char,
    pub start: in_addr,
    pub end: in_addr,
    pub start6: in6_addr,
    pub end6: in6_addr,
    pub is6: libc::c_int,
    pub indexed: libc::c_int,
    pub next: *mut cond_domain,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ra_interface {
    pub name: *mut libc::c_char,
    pub mtu_name: *mut libc::c_char,
    pub interval: libc::c_int,
    pub lifetime: libc::c_int,
    pub prio: libc::c_int,
    pub mtu: libc::c_int,
    pub next: *mut ra_interface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_context {
    pub lease_time: libc::c_uint,
    pub addr_epoch: libc::c_uint,
    pub netmask: in_addr,
    pub broadcast: in_addr,
    pub local: in_addr,
    pub router: in_addr,
    pub start: in_addr,
    pub end: in_addr,
    pub start6: in6_addr,
    pub end6: in6_addr,
    pub local6: in6_addr,
    pub prefix: libc::c_int,
    pub if_index: libc::c_int,
    pub valid: libc::c_uint,
    pub preferred: libc::c_uint,
    pub saved_valid: libc::c_uint,
    pub ra_time: time_t,
    pub ra_short_period_start: time_t,
    pub address_lost_time: time_t,
    pub template_interface: *mut libc::c_char,
    pub flags: libc::c_int,
    pub netid: dhcp_netid,
    pub filter: *mut dhcp_netid,
    pub next: *mut dhcp_context,
    pub current: *mut dhcp_context,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shared_network {
    pub if_index: libc::c_int,
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
    pub hash: libc::c_uint,
    pub next: *mut ping_result,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tftp_file {
    pub refcount: libc::c_int,
    pub fd: libc::c_int,
    pub size: off_t,
    pub dev: dev_t,
    pub inode: ino_t,
    pub filename: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tftp_transfer {
    pub sockfd: libc::c_int,
    pub timeout: time_t,
    pub backoff: libc::c_int,
    pub block: libc::c_uint,
    pub blocksize: libc::c_uint,
    pub expansion: libc::c_uint,
    pub offset: off_t,
    pub peer: mysockaddr,
    pub source: all_addr,
    pub if_index: libc::c_int,
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
    pub interface: *mut libc::c_char,
    pub prefix: *mut libc::c_char,
    pub missing: libc::c_int,
    pub next: *mut tftp_prefix,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dhcp_relay {
    pub local: all_addr,
    pub server: all_addr,
    pub interface: *mut libc::c_char,
    pub iface_index: libc::c_int,
    pub current: *mut dhcp_relay,
    pub next: *mut dhcp_relay,
}
#[derive(Clone)]
#[repr(C)]
pub struct dnsmasq_daemon {
    pub options: [libc::c_uint; 2],
    pub default_resolv: resolvc,
    pub resolv_files: *mut resolvc,
    pub last_resolv: time_t,
    pub servers_file: *mut libc::c_char,
    pub mxnames: *mut mx_srv_record,
    pub naptr: *mut naptr,
    pub txt: *mut txt_record,
    pub rr: *mut txt_record,
    pub ptr: *mut ptr_record,
    pub host_records: *mut host_record,
    pub host_records_tail: *mut host_record,
    pub cnames: *mut cname,
    pub auth_zones: *mut auth_zone,
    pub int_names: *mut interface_name,
    pub mxtarget: *mut libc::c_char,
    pub add_subnet4: *mut mysubnet,
    pub add_subnet6: *mut mysubnet,
    pub lease_file: *mut libc::c_char,
    pub username: *mut libc::c_char,
    pub groupname: *mut libc::c_char,
    pub scriptuser: *mut libc::c_char,
    pub luascript: *mut libc::c_char,
    pub authserver: *mut libc::c_char,
    pub hostmaster: *mut libc::c_char,
    pub authinterface: *mut iname,
    pub secondary_forward_server: *mut name_list,
    pub group_set: libc::c_int,
    pub osport: libc::c_int,
    pub domain_suffix: *mut libc::c_char,
    pub cond_domain: *mut cond_domain,
    pub synth_domains: *mut cond_domain,
    pub runfile: *mut libc::c_char,
    pub lease_change_command: *mut libc::c_char,
    pub if_names: *mut iname,
    pub if_addrs: *mut iname,
    pub if_except: *mut iname,
    pub dhcp_except: *mut iname,
    pub auth_peers: *mut iname,
    pub tftp_interfaces: *mut iname,
    pub bogus_addr: *mut bogus_addr,
    pub ignore_addr: *mut bogus_addr,
    pub servers: *mut server,
    pub ipsets: *mut ipsets,
    pub log_fac: libc::c_int,
    pub log_file: *mut libc::c_char,
    pub max_logs: libc::c_int,
    pub cachesize: libc::c_int,
    pub ftabsize: libc::c_int,
    pub port: libc::c_int,
    pub query_port: libc::c_int,
    pub min_port: libc::c_int,
    pub max_port: libc::c_int,
    pub local_ttl: libc::c_ulong,
    pub neg_ttl: libc::c_ulong,
    pub max_ttl: libc::c_ulong,
    pub min_cache_ttl: libc::c_ulong,
    pub max_cache_ttl: libc::c_ulong,
    pub auth_ttl: libc::c_ulong,
    pub dhcp_ttl: libc::c_ulong,
    pub use_dhcp_ttl: libc::c_ulong,
    pub dns_client_id: *mut libc::c_char,
    pub addn_hosts: *mut hostsfile,
    pub dhcp: *mut dhcp_context,
    pub dhcp6: *mut dhcp_context,
    pub ra_interfaces: *mut ra_interface,
    pub dhcp_conf: *mut dhcp_config,
    pub dhcp_opts: *mut dhcp_opt,
    pub dhcp_match: *mut dhcp_opt,
    pub dhcp_opts6: *mut dhcp_opt,
    pub dhcp_match6: *mut dhcp_opt,
    pub dhcp_name_match: *mut dhcp_match_name,
    pub dhcp_pxe_vendors: *mut dhcp_pxe_vendor,
    pub dhcp_vendors: *mut dhcp_vendor,
    pub dhcp_macs: *mut dhcp_mac,
    pub boot_config: *mut dhcp_boot,
    pub pxe_services: *mut pxe_service,
    pub tag_if: *mut tag_if,
    pub override_relays: *mut addr_list,
    pub relay4: *mut dhcp_relay,
    pub relay6: *mut dhcp_relay,
    pub delay_conf: *mut delay_config,
    pub override_0: libc::c_int,
    pub enable_pxe: libc::c_int,
    pub doing_ra: libc::c_int,
    pub doing_dhcp6: libc::c_int,
    pub dhcp_ignore: *mut dhcp_netid_list,
    pub dhcp_ignore_names: *mut dhcp_netid_list,
    pub dhcp_gen_names: *mut dhcp_netid_list,
    pub force_broadcast: *mut dhcp_netid_list,
    pub bootp_dynamic: *mut dhcp_netid_list,
    pub dhcp_hosts_file: *mut hostsfile,
    pub dhcp_opts_file: *mut hostsfile,
    pub dynamic_dirs: *mut hostsfile,
    pub dhcp_max: libc::c_int,
    pub tftp_max: libc::c_int,
    pub tftp_mtu: libc::c_int,
    pub dhcp_server_port: libc::c_int,
    pub dhcp_client_port: libc::c_int,
    pub start_tftp_port: libc::c_int,
    pub end_tftp_port: libc::c_int,
    pub min_leasetime: libc::c_uint,
    pub doctors: *mut doctor,
    pub edns_pktsz: libc::c_ushort,
    pub tftp_prefix: *mut libc::c_char,
    pub if_prefix: *mut tftp_prefix,
    pub duid_enterprise: libc::c_uint,
    pub duid_config_len: libc::c_uint,
    pub duid_config: *mut libc::c_uchar,
    pub dbus_name: *mut libc::c_char,
    pub ubus_name: *mut libc::c_char,
    pub dump_file: *mut libc::c_char,
    pub dump_mask: libc::c_int,
    pub soa_sn: libc::c_ulong,
    pub soa_refresh: libc::c_ulong,
    pub soa_retry: libc::c_ulong,
    pub soa_expiry: libc::c_ulong,
    pub metrics: [u32_0; 20],
    pub packet: *mut libc::c_char,
    pub packet_buff_sz: libc::c_int,
    pub namebuff: *mut libc::c_char,
    pub frec_list: *mut frec,
    pub free_frec_src: *mut frec_src,
    pub frec_src_count: libc::c_int,
    pub sfds: *mut serverfd,
    pub interfaces: *mut irec,
    pub listeners: *mut listener,
    pub last_server: *mut server,
    pub forwardtime: time_t,
    pub forwardcount: libc::c_int,
    pub srv_save: *mut server,
    pub packet_len: size_t,
    pub rfd_save: *mut randfd,
    pub tcp_pids: [pid_t; 20],
    pub tcp_pipes: [libc::c_int; 20],
    pub pipe_to_parent: libc::c_int,
    pub randomsocks: [randfd; 64],
    pub v6pktinfo: libc::c_int,
    pub interface_addrs: *mut addrlist,
    pub log_id: libc::c_int,
    pub log_display_id: libc::c_int,
    pub log_source_addr: *mut mysockaddr,
    pub dhcpfd: libc::c_int,
    pub helperfd: libc::c_int,
    pub pxefd: libc::c_int,
    pub inotifyfd: libc::c_int,
    pub netlinkfd: libc::c_int,
    pub kernel_version: libc::c_int,
    pub dhcp_packet: iovec,
    pub dhcp_buff: *mut libc::c_char,
    pub dhcp_buff2: *mut libc::c_char,
    pub dhcp_buff3: *mut libc::c_char,
    pub ping_results: *mut ping_result,
    pub lease_stream: *mut FILE,
    pub bridges: *mut dhcp_bridge,
    pub shared_networks: *mut shared_network,
    pub duid_len: libc::c_int,
    pub duid: *mut libc::c_uchar,
    pub outpacket: iovec,
    pub dhcp6fd: libc::c_int,
    pub icmp6fd: libc::c_int,
    pub dbus: *mut libc::c_void,
    pub tftp_trans: *mut tftp_transfer,
    pub tftp_done_trans: *mut tftp_transfer,
    pub addrbuff: *mut libc::c_char,
    pub addrbuff2: *mut libc::c_char,
    pub dumpfd: libc::c_int,
}

pub const F_IPV4: libc::c_uint = (1 as libc::c_uint) << 7 as libc::c_int;
pub const F_IPV6: libc::c_uint = (1 as libc::c_uint) << 8 as libc::c_int;
pub const ADDRLIST_IPV6: libc::c_int = 2 as libc::c_int;
pub const F_DHCP: libc::c_uint = (1 as libc::c_uint) << 4 as libc::c_int;
pub const F_HOSTS: libc::c_uint = (1 as libc::c_uint) << 6 as libc::c_int;
pub const F_FORWARD: libc::c_uint = (1 as libc::c_uint) << 3 as libc::c_int;
pub const F_NXDOMAIN: libc::c_uint = (1 as libc::c_uint) << 10 as libc::c_int;
pub const F_NEG: libc::c_uint = (1 as libc::c_uint) << 5 as libc::c_int;
pub const F_AUTH: libc::c_uint = (1 as libc::c_uint) << 21 as libc::c_int;
pub const F_CNAME: libc::c_uint =
        (1 as libc::c_uint) << 11 as libc::c_int;
pub const F_CONFIG: libc::c_uint =
        (1 as libc::c_uint) << 13 as libc::c_int;
pub const F_RRNAME: libc::c_uint =
        (1 as libc::c_uint) << 17 as libc::c_int;
pub const ADDRSTRLEN: libc::c_int = 46 as libc::c_int;
pub const ADDRLIST_REVONLY: libc::c_int = 4 as libc::c_int;
pub const F_REVERSE: libc::c_uint =
        (1 as libc::c_uint) << 2 as libc::c_int;
        
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

pub const _STAT_VER_LINUX: libc::c_int = 1 as libc::c_int;
pub const _STAT_VER: libc::c_int = _STAT_VER_LINUX;

// https://code.woboq.org/userspace/glibc/libio/libioP.h.html#117
pub type _IO_finish_t = fn(*mut FILE, libc::c_int);
pub type _IO_overflow_t = fn(*mut FILE, libc::c_int) -> libc::c_int;
pub type _IO_underflow_t = fn(*mut FILE) -> libc::c_int;
pub type _IO_pbackfail_t = fn(*mut FILE, libc::c_int) -> libc::c_int;
pub type _IO_xsputn_t = fn(FP: *mut FILE, DATA: *mut libc::c_void, N: libc::size_t) -> __off64_t;
pub type _IO_xsgetn_t = fn(FP: *mut FILE, DATA: *mut libc::c_void, N: libc::size_t) -> libc::size_t;
pub type _IO_seekoff_t = fn(FP: *mut FILE, OFF: __off64_t, DIR: libc::c_int, MODE: libc::c_int) -> __off64_t;
pub type _IO_seekpos_t = fn(*mut FILE, __off64_t, libc::c_int) -> __off64_t;
pub type _IO_setbuf_t = fn(*mut FILE, *mut libc::c_char, libc::ssize_t) -> *mut FILE;
pub type _IO_sync_t = fn(*mut FILE) -> libc::c_int;
pub type _IO_doallocate_t = fn(*mut FILE) -> libc::c_int;
pub type _IO_read_t = fn(*mut FILE, *mut libc::c_void, libc::ssize_t) -> libc::ssize_t;
pub type _IO_write_t = fn(*mut FILE, *mut libc::c_void, libc::ssize_t) -> libc::ssize_t;
pub type _IO_seek_t = fn(*mut FILE, *mut __off64_t, libc::c_int) -> __off64_t;
pub type _IO_close_t = fn(*mut FILE) -> libc::c_int;
pub type _IO_stat_t = fn(*mut FILE, *mut libc::c_void) -> libc::c_int;
pub type _IO_showmanyc_t = fn(*mut FILE) -> libc::c_int;
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

// pub type _IO_lock_t = ();

// pub const _IO_EOF_SEEN: libc::c_int = 0x10 as libc::c_int;

// pub const _IO_ERR_SEEN: libc::c_int = 0x20 as libc::c_int;

// pub type _IO_wide_data;

// pub type _IO_codecvt;

// pub type _IO_marker;

// pub type FILE = _IO_FILE;

pub type __compar_fn_t = Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;

// #[inline]
// pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char)
// -> libc::c_int {
//     return strtol(__nptr,
//                     NULL as *mut libc::c_void as *mut *mut libc::c_char,
//                     10 as libc::c_int) as libc::c_int;
// }

// #[inline]
// pub unsafe extern "C" fn atol(mut __nptr: *const libc::c_char)
//     -> libc::c_long {
//     return strtol(__nptr,
//                     NULL as *mut libc::c_void as *mut *mut libc::c_char,
//                     10 as libc::c_int);
// }

// #[inline]
// pub unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char)
//     -> libc::c_longlong {
//     return strtoll(__nptr,
//                     NULL as *mut libc::c_void as *mut *mut libc::c_char,
//                     10 as libc::c_int);
// }

// #[no_mangle]
// pub fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char)
//     -> libc::c_double;
// #[no_mangle]
// pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
//                 _: libc::c_int) -> libc::c_long;
// #[no_mangle]
// pub fn strtoll(_: *const libc::c_char, _: *mut *mut libc::c_char,
//                 _: libc::c_int) -> libc::c_longlong;

pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type __gwchar_t = libc::c_int;

// #[inline]
// pub unsafe extern "C" fn strtoimax(mut nptr: *const libc::c_char,
//                                     mut endptr: *mut *mut libc::c_char,
//                                     mut base: libc::c_int) -> intmax_t {
//     return __strtol_internal(nptr, endptr, base, 0 as libc::c_int);
// }

// #[inline]
// pub unsafe extern "C" fn strtoumax(mut nptr: *const libc::c_char,
//                                     mut endptr: *mut *mut libc::c_char,
//                                     mut base: libc::c_int) -> uintmax_t {
//     return __strtoul_internal(nptr, endptr, base, 0 as libc::c_int);
// }

// #[inline]
// pub unsafe extern "C" fn wcstoimax(mut nptr: *const __gwchar_t,
//                                     mut endptr: *mut *mut __gwchar_t,
//                                     mut base: libc::c_int) -> intmax_t {
//     return __wcstol_internal(nptr, endptr, base, 0 as libc::c_int);
// }

// #[inline]
// pub unsafe extern "C" fn wcstoumax(mut nptr: *const __gwchar_t,
//                                     mut endptr: *mut *mut __gwchar_t,
//                                     mut base: libc::c_int) -> uintmax_t {
//     return __wcstoul_internal(nptr, endptr, base, 0 as libc::c_int);
// }


// #[no_mangle]
// pub fn __strtol_internal(__nptr: *const libc::c_char,
//                             __endptr: *mut *mut libc::c_char,
//                             __base: libc::c_int, __group: libc::c_int)
//     -> libc::c_long;

// #[no_mangle]
// pub fn __strtoul_internal(__nptr: *const libc::c_char,
//                             __endptr: *mut *mut libc::c_char,
//                             __base: libc::c_int, __group: libc::c_int)
//     -> libc::c_ulong;

// #[no_mangle]
// pub fn __wcstol_internal(__nptr: *const __gwchar_t,
//                             __endptr: *mut *mut __gwchar_t,
//                             __base: libc::c_int, __group: libc::c_int)
//     -> libc::c_long;

// #[no_mangle]
// pub fn __wcstoul_internal(__nptr: *const __gwchar_t,
//                             __endptr: *mut *mut __gwchar_t,
//                             __base: libc::c_int, __group: libc::c_int)
//     -> libc::c_ulong;

#[inline]    
pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                |
                (__bsx as libc::c_int & 0xff as libc::c_int) <<
                    8 as libc::c_int) as __uint16_t;
}

#[inline]
pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
                (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
                (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
                (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}

#[inline]
pub unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
    return ((__bsx as libc::c_ulonglong &
                    0xff00000000000000 as libc::c_ulonglong) >>
                56 as libc::c_int |
                (__bsx as libc::c_ulonglong &
                        0xff000000000000 as libc::c_ulonglong) >>
                    40 as libc::c_int |
                (__bsx as libc::c_ulonglong &
                        0xff0000000000 as libc::c_ulonglong) >>
                    24 as libc::c_int |
                (__bsx as libc::c_ulonglong &
                        0xff00000000 as libc::c_ulonglong) >>
                    8 as libc::c_int |
                (__bsx as libc::c_ulonglong &
                        0xff000000 as libc::c_ulonglong) << 8 as libc::c_int
                |
                (__bsx as libc::c_ulonglong &
                        0xff0000 as libc::c_ulonglong) << 24 as libc::c_int |
                (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong)
                    << 40 as libc::c_int |
                (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong)
                    << 56 as libc::c_int) as __uint64_t;
}

#[inline]
pub unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t)
    -> __uint16_t {
    return __x;
}

#[inline]
pub unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t)
    -> __uint32_t {
    return __x;
}

#[inline]
pub unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t)
    -> __uint64_t {
    return __x;
}

// #[inline]
// pub unsafe extern "C" fn fstat(mut __fd: libc::c_int,
//                                 mut __statbuf: *mut stat) -> libc::c_int {
//     return __fxstat(_STAT_VER, __fd, __statbuf);
// }

// #[inline]
// pub unsafe extern "C" fn stat64(mut __path: *const libc::c_char,
//                                 mut __statbuf: *mut stat64)
//     -> libc::c_int {
//     return __xstat64(_STAT_VER, __path, __statbuf);
// }

// #[inline]
// pub unsafe extern "C" fn fstat64(mut __fd: libc::c_int,
//                                     mut __statbuf: *mut stat64)
//     -> libc::c_int {
//     return __fxstat64(_STAT_VER, __fd, __statbuf);
// }

// #[inline]
// pub unsafe extern "C" fn fstatat(mut __fd: libc::c_int,
//                                     mut __filename: *const libc::c_char,
//                                     mut __statbuf: *mut stat,
//                                     mut __flag: libc::c_int) -> libc::c_int {
//     return __fxstatat(_STAT_VER, __fd, __filename, __statbuf, __flag);
// }

// #[inline]
// pub unsafe extern "C" fn fstatat64(mut __fd: libc::c_int,
//                                     mut __filename: *const libc::c_char,
//                                     mut __statbuf: *mut stat64,
//                                     mut __flag: libc::c_int)
//     -> libc::c_int {
//     return __fxstatat64(_STAT_VER, __fd, __filename, __statbuf, __flag);
// }

// #[inline]
// pub unsafe extern "C" fn lstat(mut __path: *const libc::c_char,
//                                 mut __statbuf: *mut stat) -> libc::c_int {
//     return __lxstat(_STAT_VER, __path, __statbuf);
// }

// #[inline]
// pub unsafe extern "C" fn lstat64(mut __path: *const libc::c_char,
//                                     mut __statbuf: *mut stat64)
//     -> libc::c_int {
//     return __lxstat64(_STAT_VER, __path, __statbuf);
// }

// #[inline]
// pub unsafe extern "C" fn mknod(mut __path: *const libc::c_char,
//                                 mut __mode: __mode_t, mut __dev: __dev_t)
//     -> libc::c_int {
//     return __xmknod(_MKNOD_VER, __path, __mode, &mut __dev);
// }

pub const _MKNOD_VER: libc::c_int = 0 as libc::c_int;

// #[inline]
// pub unsafe extern "C" fn mknodat(mut __fd: libc::c_int,
//                                     mut __path: *const libc::c_char,
//                                     mut __mode: __mode_t, mut __dev: __dev_t)
//     -> libc::c_int {
//     return __xmknodat(_MKNOD_VER, __fd, __path, __mode, &mut __dev);
// }

// #[inline]
// pub unsafe extern "C" fn stat(mut __path: *const libc::c_char,
//                                 mut __statbuf: *mut stat) -> libc::c_int {
//     return __xstat(_STAT_VER, __path, __statbuf);
// }

// #[no_mangle]    
// pub fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
//                 __stat_buf: *mut stat) -> libc::c_int;

// #[no_mangle]
// pub fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int,
//                 __stat_buf: *mut stat) -> libc::c_int;

// #[no_mangle]
// pub fn __xstat64(__ver: libc::c_int, __filename: *const libc::c_char,
//                     __stat_buf: *mut stat64) -> libc::c_int;

// #[no_mangle]
// pub fn __fxstat64(__ver: libc::c_int, __fildes: libc::c_int,
//                     __stat_buf: *mut stat64) -> libc::c_int;

// #[no_mangle]
// pub fn __fxstatat(__ver: libc::c_int, __fildes: libc::c_int,
//                     __filename: *const libc::c_char,
//                     __stat_buf: *mut stat, __flag: libc::c_int)
//     -> libc::c_int;

// #[no_mangle]
// pub fn __fxstatat64(__ver: libc::c_int, __fildes: libc::c_int,
//                     __filename: *const libc::c_char,
//                     __stat_buf: *mut stat64, __flag: libc::c_int)
//     -> libc::c_int;

// #[no_mangle]
// pub fn __lxstat(__ver: libc::c_int, __filename: *const libc::c_char,
//                 __stat_buf: *mut stat) -> libc::c_int;

// #[no_mangle]
// pub fn __lxstat64(__ver: libc::c_int, __filename: *const libc::c_char,
//                     __stat_buf: *mut stat64) -> libc::c_int;

// #[no_mangle]
// pub fn __xmknod(__ver: libc::c_int, __path: *const libc::c_char,
//                 __mode: __mode_t, __dev: *mut __dev_t) -> libc::c_int;

// #[no_mangle]
// pub fn __xmknodat(__ver: libc::c_int, __fd: libc::c_int,
//                     __path: *const libc::c_char, __mode: __mode_t,
//                     __dev: *mut __dev_t) -> libc::c_int;

// #[no_mangle]   
// pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
//                 _: libc::c_ulong) -> *mut libc::c_void;

// #[no_mangle]
// pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
//                 _: libc::c_ulong) -> libc::c_int;


// #[inline]
// pub unsafe extern "C" fn vprintf(mut __fmt: *const libc::c_char,
//                                     mut __arg: ::std::ffi::VaList)
//     -> libc::c_int {
//     return vfprintf(stdout, __fmt, __arg.as_va_list());
// }

// #[inline]
// pub unsafe extern "C" fn getchar() -> libc::c_int { return getc(stdin); }

// #[inline]
// pub unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE)
//     -> libc::c_int {
//     return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as
//                     libc::c_int as libc::c_long != 0 {
//                 __uflow(__fp)
//             } else {
//                 let fresh0 = (*__fp)._IO_read_ptr;
//                 (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
//                 *(fresh0 as *mut libc::c_uchar) as libc::c_int
//             };
// }

// #[inline]
// pub unsafe extern "C" fn getchar_unlocked() -> libc::c_int {
//     return if ((*stdin)._IO_read_ptr >= (*stdin)._IO_read_end) as
//                     libc::c_int as libc::c_long != 0 {
//                 __uflow(stdin)
//             } else {
//                 let fresh1 = (*stdin)._IO_read_ptr;
//                 (*stdin)._IO_read_ptr = (*stdin)._IO_read_ptr.offset(1);
//                 *(fresh1 as *mut libc::c_uchar) as libc::c_int
//             };
// }

// #[inline]
// pub unsafe extern "C" fn fgetc_unlocked(mut __fp: *mut FILE)
//     -> libc::c_int {
//     return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as
//                     libc::c_int as libc::c_long != 0 {
//                 __uflow(__fp)
//             } else {
//                 let fresh2 = (*__fp)._IO_read_ptr;
//                 (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
//                 *(fresh2 as *mut libc::c_uchar) as libc::c_int
//             };
// }

// #[inline]
// pub unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
//     return putc(__c, stdout);
// }

// #[inline]
// pub unsafe extern "C" fn fputc_unlocked(mut __c: libc::c_int,
//                                         mut __stream: *mut FILE)
//     -> libc::c_int {
//     return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as
//                     libc::c_int as libc::c_long != 0 {
//                 __overflow(__stream, __c as libc::c_uchar as libc::c_int)
//             } else {
//                 let fresh3 = (*__stream)._IO_write_ptr;
//                 (*__stream)._IO_write_ptr =
//                     (*__stream)._IO_write_ptr.offset(1);
//                 *fresh3 = __c as libc::c_char;
//                 *fresh3 as libc::c_uchar as libc::c_int
//             };
// }

// #[inline]
// pub unsafe extern "C" fn putc_unlocked(mut __c: libc::c_int,
//                                         mut __stream: *mut FILE)
//     -> libc::c_int {
//     return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as
//                     libc::c_int as libc::c_long != 0 {
//                 __overflow(__stream, __c as libc::c_uchar as libc::c_int)
//             } else {
//                 let fresh4 = (*__stream)._IO_write_ptr;
//                 (*__stream)._IO_write_ptr =
//                     (*__stream)._IO_write_ptr.offset(1);
//                 *fresh4 = __c as libc::c_char;
//                 *fresh4 as libc::c_uchar as libc::c_int
//             };
// }

// #[inline]
// pub unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int)
//     -> libc::c_int {
//     return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as
//                     libc::c_int as libc::c_long != 0 {
//                 __overflow(stdout, __c as libc::c_uchar as libc::c_int)
//             } else {
//                 let fresh5 = (*stdout)._IO_write_ptr;
//                 (*stdout)._IO_write_ptr =
//                     (*stdout)._IO_write_ptr.offset(1);
//                 *fresh5 = __c as libc::c_char;
//                 *fresh5 as libc::c_uchar as libc::c_int
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
//     -> libc::c_int {
//     return ((*__stream)._flags & _IO_EOF_SEEN != 0 as libc::c_int) as
//                 libc::c_int;
// }

// #[inline]
// pub unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE)
//     -> libc::c_int {
//     return ((*__stream)._flags & _IO_ERR_SEEN != 0 as libc::c_int) as
//                 libc::c_int;
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
                                                    isize) as
                *mut libc::c_void;
        __comparison =
            Some(__compar.expect("non-null function pointer")).expect("non-null function pointer")(__key,
                                                                                                    __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else { return __p as *mut libc::c_void }
    }
    return NULL as *mut libc::c_void;
}

pub const DHCP_CHADDR_MAX: libc::c_int = 16 as libc::c_int;

pub const IN6ADDRSZ: libc::c_int = 16 as libc::c_int;

// #[inline]
// pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
//     return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
//                 *(*__ctype_tolower_loc()).offset(__c as isize)
//             } else { __c };
// }

// #[inline]
// pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
//     return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
//                 *(*__ctype_toupper_loc()).offset(__c as isize)
//             } else { __c };
// }

// #[no_mangle]
// pub fn __ctype_tolower_loc() -> *mut *const __int32_t;

// #[no_mangle]
// pub fn __ctype_toupper_loc() -> *mut *const __int32_t;

// #[no_mangle]
// pub fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;

pub const LOG_WARNING: libc::c_int = 4 as libc::c_int;
pub const LOG_INFO: libc::c_int = 6 as libc::c_int;

pub const KEYBLOCK_LEN: libc::c_int = 40 as libc::c_int;

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

pub const HB4_RCODE: libc::c_int = 0xf as libc::c_int;
    
pub const HB3_TC: libc::c_int = 0x2 as libc::c_int;

pub const HB3_AA: libc::c_int = 0x4 as libc::c_int;

pub const HB4_AD: libc::c_int = 0x20 as libc::c_int;

pub const HB4_RA: libc::c_int = 0x80 as libc::c_int;

pub const HB3_QR: libc::c_int = 0x80 as libc::c_int;

pub const C_IN: libc::c_int = 1 as libc::c_int;

pub const T_SOA: libc::c_int = 6 as libc::c_int;

pub const T_A: libc::c_int = 1 as libc::c_int;

pub const T_AAAA: libc::c_int = 28 as libc::c_int;

pub const T_CNAME: libc::c_int = 5 as libc::c_int;

pub const T_NAPTR: libc::c_int = 35 as libc::c_int;

pub const T_TXT: libc::c_int = 16 as libc::c_int;

pub const T_MX: libc::c_int = 15 as libc::c_int;

pub const T_SRV: libc::c_int = 33 as libc::c_int;

pub const T_NS: libc::c_int = 2 as libc::c_int;

pub const T_AXFR: libc::c_int = 252 as libc::c_int;

pub const T_PTR: libc::c_int = 12 as libc::c_int;

pub const QUERY: libc::c_int = 0 as libc::c_int;

pub const HB3_OPCODE: libc::c_int = 0x78 as libc::c_int;

// #[no_mangle]
// pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
//          -> *mut libc::c_char;

// #[no_mangle]
// pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
//     -> *mut libc::c_char;

// #[no_mangle]
// pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
//     -> libc::c_int;

// #[no_mangle]
// pub fn strchr(_: *const libc::c_char, _: libc::c_int)
//     -> *mut libc::c_char;

// #[no_mangle]
// pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;


// #[no_mangle]        
// pub static mut stdin: *mut FILE;

// #[no_mangle]
// pub static mut stdout: *mut FILE;

// #[no_mangle]
// pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
//                 _: ::std::ffi::VaList) -> libc::c_int;

// #[no_mangle]
// pub fn getc(__stream: *mut FILE) -> libc::c_int;

// #[no_mangle]
// pub fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;

// #[no_mangle]
// pub fn __uflow(_: *mut FILE) -> libc::c_int;

// #[no_mangle]
// pub fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;

// #[no_mangle]
// pub fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
//                     __delimiter: libc::c_int, __stream: *mut FILE)
//     -> __ssize_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub type_0: libc::c_uint,
    pub name: *const libc::c_char,
}

pub const SMALLDNAME: libc::c_int = 50 as libc::c_int;

pub const HOSTSFILE: [libc::c_char; 11] =
        unsafe {
            *::std::mem::transmute::<&[u8; 11],
                                     &[libc::c_char; 11]>(b"/etc/hosts\x00")
        };


pub const NOTIMP: libc::c_int = 4 as libc::c_int;
    
pub const REFUSED: libc::c_int = 5 as libc::c_int;

pub const SERVFAIL: libc::c_int = 2 as libc::c_int;

pub const INADDRSZ: libc::c_int = 4 as libc::c_int;

pub const MAXDNAME: libc::c_int = 1025 as libc::c_int;
    
pub const LOG_ERR: libc::c_int = 3 as libc::c_int;

pub const LOG_DAEMON: libc::c_int =
    (3 as libc::c_int) << 3 as libc::c_int;

// pub const errno: libc::c_int = *__errno_location();

// #[no_mangle]
//  pub fn __errno_location() -> *mut libc::c_int;

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
//      -> libc::c_int;
//     #[no_mangle]
//     fn getc(__stream: *mut FILE) -> libc::c_int;
//     #[no_mangle]
//     fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
//     #[no_mangle]
//     fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
//                   __delimiter: libc::c_int, __stream: *mut FILE) -> __ssize_t;
//     #[no_mangle]
//     fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
//                __stat_buf: *mut stat) -> libc::c_int;
//     #[no_mangle]
//     fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int,
//                 __stat_buf: *mut stat) -> libc::c_int;
//     #[no_mangle]
//     fn __xstat64(__ver: libc::c_int, __filename: *const libc::c_char,
//                  __stat_buf: *mut stat64) -> libc::c_int;
//     #[no_mangle]
//     fn __fxstat64(__ver: libc::c_int, __fildes: libc::c_int,
//                   __stat_buf: *mut stat64) -> libc::c_int;
//     #[no_mangle]
//     fn __fxstatat(__ver: libc::c_int, __fildes: libc::c_int,
//                   __filename: *const libc::c_char, __stat_buf: *mut stat,
//                   __flag: libc::c_int) -> libc::c_int;
//     #[no_mangle]
//     fn __fxstatat64(__ver: libc::c_int, __fildes: libc::c_int,
//                     __filename: *const libc::c_char, __stat_buf: *mut stat64,
//                     __flag: libc::c_int) -> libc::c_int;
//     #[no_mangle]
//     fn __lxstat(__ver: libc::c_int, __filename: *const libc::c_char,
//                 __stat_buf: *mut stat) -> libc::c_int;
//     #[no_mangle]
//     fn __lxstat64(__ver: libc::c_int, __filename: *const libc::c_char,
//                   __stat_buf: *mut stat64) -> libc::c_int;
//     #[no_mangle]
//     fn __xmknod(__ver: libc::c_int, __path: *const libc::c_char,
//                 __mode: __mode_t, __dev: *mut __dev_t) -> libc::c_int;
//     #[no_mangle]
//     fn __xmknodat(__ver: libc::c_int, __fd: libc::c_int,
//                   __path: *const libc::c_char, __mode: __mode_t,
//                   __dev: *mut __dev_t) -> libc::c_int;
//     #[no_mangle]
//     fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
//      -> *mut libc::c_void;
//     #[no_mangle]
//     fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
//               _: libc::c_ulong) -> libc::c_int;
//     #[no_mangle]
//     fn __uflow(_: *mut FILE) -> libc::c_int;
//     #[no_mangle]
//     fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
//     #[no_mangle]
//     fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char)
//      -> libc::c_double;
//     #[no_mangle]
//     fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
//               _: libc::c_int) -> libc::c_long;
//     #[no_mangle]
//     fn strtoll(_: *const libc::c_char, _: *mut *mut libc::c_char,
//                _: libc::c_int) -> libc::c_longlong;
//     #[no_mangle]
//     fn __ctype_tolower_loc() -> *mut *const __int32_t;
//     #[no_mangle]
//     fn __ctype_toupper_loc() -> *mut *const __int32_t;
//     #[no_mangle]
//     fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
//     #[no_mangle]
//     fn __strtol_internal(__nptr: *const libc::c_char,
//                          __endptr: *mut *mut libc::c_char,
//                          __base: libc::c_int, __group: libc::c_int)
//      -> libc::c_long;
//     #[no_mangle]
//     fn __strtoul_internal(__nptr: *const libc::c_char,
//                           __endptr: *mut *mut libc::c_char,
//                           __base: libc::c_int, __group: libc::c_int)
//      -> libc::c_ulong;
//     #[no_mangle]
//     fn __wcstol_internal(__nptr: *const __gwchar_t,
//                          __endptr: *mut *mut __gwchar_t, __base: libc::c_int,
//                          __group: libc::c_int) -> libc::c_long;
//     #[no_mangle]
//     fn __wcstoul_internal(__nptr: *const __gwchar_t,
//                           __endptr: *mut *mut __gwchar_t, __base: libc::c_int,
//                           __group: libc::c_int) -> libc::c_ulong;
//     #[no_mangle]
//     static mut dnsmasq_daemon: *mut dnsmasq_daemon;
//     #[no_mangle]
//     fn whine_malloc(size: size_t) -> *mut libc::c_void;
//     #[no_mangle]
//     fn iface_enumerate(family: libc::c_int, parm: *mut libc::c_void,
//                        callback:
//                            Option<unsafe extern "C" fn() -> libc::c_int>)
//      -> libc::c_int;
//     #[no_mangle]
//     fn queue_arp(action: libc::c_int, mac: *mut libc::c_uchar,
//                  maclen: libc::c_int, family: libc::c_int,
//                  addr: *mut all_addr);
// }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}




pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;




/* No MAC addr */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arp_record {
    pub hwlen: libc::c_ushort,
    pub status: libc::c_ushort,
    pub family: libc::c_int,
    pub hwaddr: [libc::c_uchar; 16],
    pub addr: all_addr,
    pub next: *mut arp_record,
}

pub type __kernel_sa_family_t = libc::c_ushort;

pub type __u8 = libc::c_uchar;
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
    pub sat_zer: [libc::c_char;8],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ax25_address {
    pub ax25_call: [libc::c_char;7],
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
    pub sax25_ndigis: libc::c_int,
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
pub type __socket_type = libc::c_uint;
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
    pub sun_path: [libc::c_char; 108],
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

pub const IPPROTO_MAX: libc::c_int = 256;
pub const IPPROTO_RAW: libc::c_int = 255;
pub const IPPROTO_MPLS: libc::c_int = 137;
pub const IPPROTO_UDPLITE: libc::c_int = 136;
pub const IPPROTO_SCTP: libc::c_int = 132;
pub const IPPROTO_COMP: libc::c_int = 108;
pub const IPPROTO_PIM: libc::c_int = 103;
pub const IPPROTO_ENCAP: libc::c_int = 98;
pub const IPPROTO_BEETPH: libc::c_int = 94;
pub const IPPROTO_MTP: libc::c_int = 92;
pub const IPPROTO_AH: libc::c_int = 51;
pub const IPPROTO_ESP: libc::c_int = 50;
pub const IPPROTO_GRE: libc::c_int = 47;
pub const IPPROTO_RSVP: libc::c_int = 46;
pub const IPPROTO_IPV6: libc::c_int = 41;
pub const IPPROTO_DCCP: libc::c_int = 33;
pub const IPPROTO_TP: libc::c_int = 29;
pub const IPPROTO_IDP: libc::c_int = 22;
pub const IPPROTO_UDP: libc::c_int = 17;
pub const IPPROTO_PUP: libc::c_int = 12;
pub const IPPROTO_EGP: libc::c_int = 8;
pub const IPPROTO_TCP: libc::c_int = 6;
pub const IPPROTO_IPIP: libc::c_int = 4;
pub const IPPROTO_IGMP: libc::c_int = 2;
pub const IPPROTO_ICMP: libc::c_int = 1;
pub const IPPROTO_IP: libc::c_int = 0;
pub type __u32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;

// __WINT_TYPE__ unsigned int
pub type __WINT_TYPE__ = libc::c_uint;


#[derive(Copy, Clone)]
#[repr(C)]
pub union Unnamed29 {
    pub __wch: __WINT_TYPE__,
    pub __wchb: [libc::c_char;4],
}


#[derive(Clone, Copy)]
#[repr(C)]
pub struct __mbstate_t
{
//   int __count;
    pub __count: libc::c_int,
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
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_2,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
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
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_12,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut siginfo_t,
                                                  _: *mut libc::c_void)
                                 -> ()>,
}
pub type C2RustUnnamed_13 = libc::c_uint;
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
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip {
    // #[bitfield(name = "ip_hl", ty = "libc::c_uint", bits = "0..=3")]
    // #[bitfield(name = "ip_v", ty = "libc::c_uint", bits = "4..=7")]
    pub ip_hl_ip_v: [u8; 1],
    pub ip_tos: uint8_t,
    pub ip_len: libc::c_ushort,
    pub ip_id: libc::c_ushort,
    pub ip_off: libc::c_ushort,
    pub ip_ttl: uint8_t,
    pub ip_p: uint8_t,
    pub ip_sum: libc::c_ushort,
    pub ip_src: in_addr,
    pub ip_dst: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp_ra_addr {
    pub ira_addr: uint32_t,
    pub ira_preference: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp {
    pub icmp_type: uint8_t,
    pub icmp_code: uint8_t,
    pub icmp_cksum: uint16_t,
    pub icmp_hun: C2RustUnnamed_17,
    pub icmp_dun: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub id_ts: C2RustUnnamed_16,
    pub id_ip: C2RustUnnamed_15,
    pub id_radv: icmp_ra_addr,
    pub id_mask: uint32_t,
    pub id_data: [uint8_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub idi_ip: ip,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub its_otime: uint32_t,
    pub its_rtime: uint32_t,
    pub its_ttime: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub ih_pptr: libc::c_uchar,
    pub ih_gwaddr: in_addr,
    pub ih_idseq: ih_idseq,
    pub ih_void: uint32_t,
    pub ih_pmtu: ih_pmtu,
    pub ih_rtradv: ih_rtradv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_rtradv {
    pub irt_num_addrs: uint8_t,
    pub irt_wpa: uint8_t,
    pub irt_lifetime: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_pmtu {
    pub ipm_void: uint16_t,
    pub ipm_nextmtu: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_idseq {
    pub icd_id: uint16_t,
    pub icd_seq: uint16_t,
}


pub type libc_lock_t = libc::c_int;

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
    pub fd: libc::c_int,
    pub lock: libc_lock_t,
    pub allocation: libc::size_t,
    pub size: libc::size_t,
    pub offset: libc::size_t,
    pub filepos: libc::off_t,
    pub errcode: libc::c_int,
}


pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __user_cap_header_struct {
    pub version: __u32,
    pub pid: libc::c_int,
}
pub type cap_user_header_t = *mut __user_cap_header_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __user_cap_data_struct {
    pub effective: __u32,
    pub permitted: __u32,
    pub inheritable: __u32,
}
pub type cap_user_data_t = *mut __user_cap_data_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_desc {
    pub event: libc::c_int,
    pub data: libc::c_int,
    pub msg_sz: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub keytag: libc::c_ushort,
    pub algo: libc::c_ushort,
    pub digest: libc::c_ushort,
    pub rcode: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub target: *mut blockdata,
    pub targetlen: libc::c_ushort,
    pub srvport: libc::c_ushort,
    pub priority: libc::c_ushort,
    pub weight: libc::c_ushort,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub flags: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub target: C2RustUnnamed_23,
    pub uid: libc::c_uint,
    pub is_name_ptr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
    pub cache: *mut crec,
    pub name: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_24 {
    pub sname: [libc::c_char; 50],
    pub bname: *mut bigname,
    pub namep: *mut libc::c_char,
}

pub type wint_t = libc::c_uint;


pub type __gconv_fct = fn(*mut __gconv_step, *mut __gconv_step_data, *mut*mut libc::c_uchar, *mut libc::c_uchar, *mut*mut libc::c_uchar, libc::size_t, libc::c_int, libc::c_int) -> libc::c_int;
pub type __gconv_btowc_fct = fn(*mut __gconv_step, libc::c_uchar) -> wint_t;
pub type __gconv_init_fct = fn(*mut __gconv_step) -> libc::c_int;
pub type __gconv_end_fct = fn(*mut __gconv_step);
pub type __gconv_trans_fct = fn(*mut __gconv_step, *mut __gconv_step_data, *mut libc::c_void, *mut libc::c_uchar, *mut*mut libc::c_uchar, *mut libc::c_uchar, *mut*mut libc::c_uchar, *mut libc::size_t);
pub type __gconv_trans_context_fct = fn(*mut libc::c_void, *mut libc::c_uchar, *mut libc::c_uchar, *mut libc::c_uchar, *mut libc::c_uchar) -> libc::c_int;
pub type __gconv_trans_query_fct = fn(*mut libc::c_char, *mut*mut*mut libc::c_char, *mut libc::size_t);
pub type __gconv_trans_init_fct = fn(*mut*mut libc::c_void, *mut libc::c_char) -> libc::c_int;

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
    pub name: *mut libc::c_char,
    pub counter: libc::c_int,
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
    pub __modname: *mut libc::c_char,
    pub __counter: libc::c_int,
    pub __from_name: *mut libc::c_char,
    pub __to_name: *mut libc::c_char,
    pub __fct: __gconv_fct,
    pub __btowc_fct: __gconv_btowc_fct,
    pub __init_fct: __gconv_init_fct,
    pub __end_fct: __gconv_end_fct,
    pub __min_needed_from: libc::c_int,
    pub __max_needed_from: libc::c_int,
    pub __min_needed_to: libc::c_int,
    pub __max_needed_to: libc::c_int,
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
    pub __outbuf: *mut libc::c_uchar,
    pub __outbufend: *mut libc::c_uchar,
    pub __flags: libc::c_int,
    pub __invocation_counter: libc::c_int,
    pub __internal_use: libc::c_int,
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
    pub clid_len: libc::c_int,
    pub clid: *mut libc::c_uchar,
    pub hostname: *mut libc::c_char,
    pub fqdn: *mut libc::c_char,
    pub old_hostname: *mut libc::c_char,
    pub flags: libc::c_int,
    pub expires: time_t,
    pub hwaddr_len: libc::c_int,
    pub hwaddr_type: libc::c_int,
    pub hwaddr: [libc::c_uchar; 16],
    pub addr: in_addr,
    pub override_0: in_addr,
    pub giaddr: in_addr,
    pub extradata: *mut libc::c_uchar,
    pub extradata_len: libc::c_uint,
    pub extradata_size: libc::c_uint,
    pub last_interface: libc::c_int,
    pub new_interface: libc::c_int,
    pub new_prefixlen: libc::c_int,
    pub addr6: in6_addr,
    pub iaid: libc::c_uint,
    pub slaac_address: *mut slaac_address,
    pub vendorclass_count: libc::c_int,
    pub next: *mut dhcp_lease,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slaac_address {
    pub addr: in6_addr,
    pub ping_time: time_t,
    pub backoff: libc::c_int,
    pub next: *mut slaac_address,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub encap: libc::c_int,
    pub wildcard_mask: libc::c_uint,
    pub vendor_class: *mut libc::c_uchar,
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
  pub _pos: libc::c_int
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
    pub __codecvt_do_out: fn(*mut _IO_codecvt, *mut __mbstate_t, *mut libc::wchar_t, *mut libc::wchar_t, *mut*mut libc::wchar_t, *mut libc::c_char, *mut libc::c_char, *mut*mut libc::c_char) -> __codecvt_result,
//   enum __codecvt_result (*__codecvt_do_unshift) (struct _IO_codecvt *,
//                                                  __mbstate_t *, char *,
//                                                  char *, char **);
    pub __codecvt_do_unshift: fn(*mut _IO_codecvt, *mut __mbstate_t, *mut libc::c_char, *mut libc::c_char, *mut*mut libc::c_char) -> __codecvt_result,
//   enum __codecvt_result (*__codecvt_do_in) (struct _IO_codecvt *,
//                                             __mbstate_t *,
//                                             const char *, const char *,
//                                             const char **, wchar_t *,
//                                             wchar_t *, wchar_t **);
    pub __codecvt_do_in: fn(*mut _IO_codecvt, *mut __mbstate_t, *mut libc::c_char, *mut libc::c_char, *mut*mut libc::c_char, *mut libc::wchar_t, *mut libc::wchar_t, *mut*mut libc::wchar_t) -> __codecvt_result,
//   int (*__codecvt_do_encoding) (struct _IO_codecvt *);
    pub __codecvt_do_encoding: fn(*mut _IO_codecvt) -> libc::c_int,
//   int (*__codecvt_do_always_noconv) (struct _IO_codecvt *);
    pub __codecvt_do_always_noconv: fn(*mut _IO_codecvt) -> libc::c_int,
//   int (*__codecvt_do_length) (struct _IO_codecvt *, __mbstate_t *,
//                               const char *, const char *, size_t);
    pub __codecvt_do_length: fn(*mut _IO_codecvt, *mut __mbstate_t, *mut libc::c_char) -> libc::c_int,
//   int (*__codecvt_do_max_length) (struct _IO_codecvt *);
    pub __codecvt_do_max_length: fn(*mut _IO_codecvt) -> libc::c_int,
//   _IO_iconv_t __cd_in;
    pub __cd_in: _IO_iconv_t,
//   _IO_iconv_t __cd_out;
    pub __cd_out: _IO_iconv_t,
}