#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int,
              __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t)
     -> libc::c_int;
    #[no_mangle]
    fn sendto(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t,
              __flags: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
              __addr_len: socklen_t) -> ssize_t;
    #[no_mangle]
    fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> libc::c_int;
    #[no_mangle]
    static in6addr_any: in6_addr;
    #[no_mangle]
    fn inet_pton(__af: libc::c_int, __cp: *const libc::c_char,
                 __buf: *mut libc::c_void) -> libc::c_int;
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
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
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
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
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn nanosleep(__requested_time: *const timespec,
                 __remaining: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
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
    fn __wcstoul_internal(__nptr: *const __gwchar_t,
                          __endptr: *mut *mut __gwchar_t, __base: libc::c_int,
                          __group: libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    static mut dnsmasq_daemon: *mut dnsmasq_daemon;
    #[no_mangle]
    fn rand64() -> u64_0;
    #[no_mangle]
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn whine_malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn is_same_net6(a: *mut in6_addr, b: *mut in6_addr,
                    prefixlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn addr6part(addr: *mut in6_addr) -> u64_0;
    #[no_mangle]
    fn setaddr6part(addr: *mut in6_addr, host: u64_0);
    #[no_mangle]
    fn retry_send(rc: ssize_t) -> libc::c_int;
    #[no_mangle]
    fn wildcard_match(wildcard: *const libc::c_char,
                      match_0: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn wildcard_matchn(wildcard: *const libc::c_char,
                       match_0: *const libc::c_char, num: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn die(message: *mut libc::c_char, arg1: *mut libc::c_char,
           exit_code: libc::c_int) -> !;
    #[no_mangle]
    fn my_syslog(priority: libc::c_int, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn indextoname(fd: libc::c_int, index: libc::c_int,
                   name: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn iface_check(family: libc::c_int, addr: *mut all_addr,
                   name: *mut libc::c_char, auth: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn fix_fd(fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn set_ipv6pktinfo(fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lease_update_file(now: time_t);
    #[no_mangle]
    fn lease_update_dns(force: libc::c_int);
    #[no_mangle]
    fn lease6_find_by_addr(net: *mut in6_addr, prefix: libc::c_int,
                           addr: u64_0) -> *mut dhcp_lease;
    #[no_mangle]
    fn lease_find_max_addr6(context: *mut dhcp_context) -> u64_0;
    #[no_mangle]
    fn lease_update_slaac(now: time_t);
    #[no_mangle]
    fn lease_prune(target: *mut dhcp_lease, now: time_t);
    #[no_mangle]
    fn send_alarm(event: time_t, now: time_t);
    #[no_mangle]
    fn iface_enumerate(family: libc::c_int, parm: *mut libc::c_void,
                       callback:
                           Option<unsafe extern "C" fn() -> libc::c_int>)
     -> libc::c_int;
    #[no_mangle]
    fn save_counter(newval: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn dhcp6_reply(context: *mut dhcp_context, interface: libc::c_int,
                   iface_name: *mut libc::c_char, fallback: *mut in6_addr,
                   ll_addr: *mut in6_addr, ula_addr: *mut in6_addr,
                   sz: size_t, client_addr: *mut in6_addr, now: time_t)
     -> libc::c_ushort;
    #[no_mangle]
    fn relay_upstream6(relay: *mut dhcp_relay, sz: ssize_t,
                       peer_address: *mut in6_addr, scope_id: u32_0,
                       now: time_t);
    #[no_mangle]
    fn relay_reply6(peer: *mut sockaddr_in6, sz: ssize_t,
                    arrival_interface: *mut libc::c_char) -> libc::c_ushort;
    #[no_mangle]
    fn recv_dhcp_packet(fd: libc::c_int, msg: *mut msghdr) -> ssize_t;
    #[no_mangle]
    fn match_netid(check: *mut dhcp_netid, pool: *mut dhcp_netid,
                   tagnotneeded: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn periodic_ra(now: time_t) -> time_t;
    #[no_mangle]
    fn log_context(family: libc::c_int, context: *mut dhcp_context);
    #[no_mangle]
    fn ra_start_unsolicited(now: time_t, context: *mut dhcp_context);
    #[no_mangle]
    fn find_mac(addr: *mut mysockaddr, mac: *mut libc::c_uchar,
                lazy: libc::c_int, now: time_t) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
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
pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;
pub type ino_t = __ino64_t;
pub type dev_t = __dev_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
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
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = __uint32_t;
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
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IPPROTO_MH: C2RustUnnamed_1 = 135;
pub const IPPROTO_DSTOPTS: C2RustUnnamed_1 = 60;
pub const IPPROTO_NONE: C2RustUnnamed_1 = 59;
pub const IPPROTO_ICMPV6: C2RustUnnamed_1 = 58;
pub const IPPROTO_FRAGMENT: C2RustUnnamed_1 = 44;
pub const IPPROTO_ROUTING: C2RustUnnamed_1 = 43;
pub const IPPROTO_HOPOPTS: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_pktinfo {
    pub ipi6_addr: in6_addr,
    pub ipi6_ifindex: libc::c_uint,
}
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct neigh_packet {
    pub type_0: u8_0,
    pub code: u8_0,
    pub checksum: u16_0,
    pub reserved: u16_0,
    pub target: in6_addr,
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
pub struct ifmap {
    pub mem_start: libc::c_ulong,
    pub mem_end: libc::c_ulong,
    pub base_addr: libc::c_ushort,
    pub irq: libc::c_uchar,
    pub dma: libc::c_uchar,
    pub port: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: C2RustUnnamed_3,
    pub ifr_ifru: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
pub union C2RustUnnamed_3 {
    pub ifrn_name: [libc::c_char; 16],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union all_addr {
    pub addr4: in_addr,
    pub addr6: in6_addr,
    pub cname: C2RustUnnamed_8,
    pub key: C2RustUnnamed_7,
    pub ds: C2RustUnnamed_6,
    pub srv: C2RustUnnamed_5,
    pub log: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub keytag: libc::c_ushort,
    pub algo: libc::c_ushort,
    pub digest: libc::c_ushort,
    pub rcode: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
pub struct C2RustUnnamed_6 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub flags: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub target: C2RustUnnamed_9,
    pub uid: libc::c_uint,
    pub is_name_ptr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
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
    pub name: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
#[derive(Copy, Clone)]
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
    pub u: C2RustUnnamed_11,
    pub val: *mut libc::c_uchar,
    pub netid: *mut dhcp_netid,
    pub next: *mut dhcp_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iface_param {
    pub current: *mut dhcp_context,
    pub relay: *mut dhcp_relay,
    pub fallback: in6_addr,
    pub relay_local: in6_addr,
    pub ll_addr: in6_addr,
    pub ula_addr: in6_addr,
    pub ind: libc::c_int,
    pub addr_match: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub c: *mut libc::c_uchar,
    pub p: *mut in6_pktinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub align: cmsghdr,
    pub control6: [libc::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cparam {
    pub now: time_t,
    pub newone: libc::c_int,
    pub newname: libc::c_int,
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
unsafe extern "C" fn vprintf(mut __fmt: *const libc::c_char,
                             mut __arg: ::std::ffi::VaList) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int { return getc(stdin); }
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
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
unsafe extern "C" fn getchar_unlocked() -> libc::c_int {
    return if ((*stdin)._IO_read_ptr >= (*stdin)._IO_read_end) as libc::c_int
                  as libc::c_long != 0 {
               __uflow(stdin)
           } else {
               let fresh1 = (*stdin)._IO_read_ptr;
               (*stdin)._IO_read_ptr = (*stdin)._IO_read_ptr.offset(1);
               *(fresh1 as *mut libc::c_uchar) as libc::c_int
           };
}
#[inline]
unsafe extern "C" fn fgetc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int as
                  libc::c_long != 0 {
               __uflow(__fp)
           } else {
               let fresh2 = (*__fp)._IO_read_ptr;
               (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
               *(fresh2 as *mut libc::c_uchar) as libc::c_int
           };
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
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
#[inline]
unsafe extern "C" fn wcstoumax(mut nptr: *const __gwchar_t,
                               mut endptr: *mut *mut __gwchar_t,
                               mut base: libc::c_int) -> uintmax_t {
    return __wcstoul_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dhcp6_init() {
    let mut fd: libc::c_int = 0;
    let mut saddr: sockaddr_in6 =
        sockaddr_in6{sin6_family: 0,
                     sin6_port: 0,
                     sin6_flowinfo: 0,
                     sin6_addr:
                         in6_addr{__in6_u:
                                      C2RustUnnamed{__u6_addr8: [0; 16],},},
                     sin6_scope_id: 0,};
    let mut class: libc::c_int = 0xc0 as libc::c_int;
    let mut oneopt: libc::c_int = 1 as libc::c_int;
    fd =
        socket(10 as libc::c_int, SOCK_DGRAM as libc::c_int,
               IPPROTO_UDP as libc::c_int);
    if fd == -(1 as libc::c_int) ||
           setsockopt(fd, IPPROTO_IPV6 as libc::c_int, 67 as libc::c_int,
                      &mut class as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) ||
           setsockopt(fd, IPPROTO_IPV6 as libc::c_int, 26 as libc::c_int,
                      &mut oneopt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) || fix_fd(fd) == 0
           || set_ipv6pktinfo(fd) == 0 {
        die(b"cannot create DHCPv6 socket: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 2 as libc::c_int);
    }
    /* When bind-interfaces is set, there might be more than one dnsmasq
     instance binding port 547. That's OK if they serve different networks.
     Need to set REUSEADDR|REUSEPORT to make this possible.
     Handle the case that REUSEPORT is defined, but the kernel doesn't 
     support it. This handles the introduction of REUSEPORT on Linux. */
    if (*dnsmasq_daemon).options[(13 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (13 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 ||
           (*dnsmasq_daemon).options[(39 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (39 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        let mut rc: libc::c_int = 0 as libc::c_int;
        rc =
            setsockopt(fd, 1 as libc::c_int, 15 as libc::c_int,
                       &mut oneopt as *mut libc::c_int as *const libc::c_void,
                       ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                           as socklen_t);
        if rc == -(1 as libc::c_int) &&
               *__errno_location() == 92 as libc::c_int {
            rc = 0 as libc::c_int
        }
        if rc != -(1 as libc::c_int) {
            rc =
                setsockopt(fd, 1 as libc::c_int, 2 as libc::c_int,
                           &mut oneopt as *mut libc::c_int as
                               *const libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as socklen_t)
        }
        if rc == -(1 as libc::c_int) {
            die(b"failed to set SO_REUSE{ADDR|PORT} on DHCPv6 socket: %s\x00"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char, 2 as libc::c_int);
        }
    }
    memset(&mut saddr as *mut sockaddr_in6 as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong);
    saddr.sin6_family = 10 as libc::c_int as sa_family_t;
    saddr.sin6_addr = in6addr_any;
    saddr.sin6_port = __bswap_16(547 as libc::c_int as __uint16_t);
    if bind(fd,
            __CONST_SOCKADDR_ARG{__sockaddr__:
                                     &mut saddr as *mut sockaddr_in6 as
                                         *mut sockaddr,},
            ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as
                socklen_t) != 0 {
        die(b"failed to bind DHCPv6 server socket: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 2 as libc::c_int);
    }
    (*dnsmasq_daemon).dhcp6fd = fd;
}
#[no_mangle]
pub unsafe extern "C" fn dhcp6_packet(mut now: time_t) {
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut relay: *mut dhcp_relay = 0 as *mut dhcp_relay;
    let mut parm: iface_param =
        iface_param{current: 0 as *mut dhcp_context,
                    relay: 0 as *mut dhcp_relay,
                    fallback:
                        in6_addr{__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},},
                    relay_local:
                        in6_addr{__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},},
                    ll_addr:
                        in6_addr{__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},},
                    ula_addr:
                        in6_addr{__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},},
                    ind: 0,
                    addr_match: 0,};
    let mut cmptr: *mut cmsghdr = 0 as *mut cmsghdr;
    let mut msg: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut if_index: libc::c_int = 0 as libc::c_int;
    let mut control_u: C2RustUnnamed_13 =
        C2RustUnnamed_13{align:
                             cmsghdr{cmsg_len: 0,
                                     cmsg_level: 0,
                                     cmsg_type: 0,
                                     __cmsg_data: [],},};
    let mut from: sockaddr_in6 =
        sockaddr_in6{sin6_family: 0,
                     sin6_port: 0,
                     sin6_flowinfo: 0,
                     sin6_addr:
                         in6_addr{__in6_u:
                                      C2RustUnnamed{__u6_addr8: [0; 16],},},
                     sin6_scope_id: 0,};
    let mut sz: ssize_t = 0;
    let mut ifr: ifreq =
        ifreq{ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_2{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut tmp: *mut iname = 0 as *mut iname;
    let mut port: libc::c_ushort = 0;
    let mut dst_addr: in6_addr =
        in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    memset(&mut dst_addr as *mut in6_addr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<in6_addr>() as libc::c_ulong);
    msg.msg_control = control_u.control6.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen =
        ::std::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong;
    msg.msg_flags = 0 as libc::c_int;
    msg.msg_name = &mut from as *mut sockaddr_in6 as *mut libc::c_void;
    msg.msg_namelen =
        ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as socklen_t;
    msg.msg_iov = &mut (*dnsmasq_daemon).dhcp_packet;
    msg.msg_iovlen = 1 as libc::c_int as size_t;
    sz = recv_dhcp_packet((*dnsmasq_daemon).dhcp6fd, &mut msg);
    if sz == -(1 as libc::c_int) as libc::c_long { return }
    cmptr =
        if msg.msg_controllen >=
               ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
            msg.msg_control as *mut cmsghdr
        } else { 0 as *mut cmsghdr };
    while !cmptr.is_null() {
        if (*cmptr).cmsg_level == IPPROTO_IPV6 as libc::c_int &&
               (*cmptr).cmsg_type == (*dnsmasq_daemon).v6pktinfo {
            let mut p: C2RustUnnamed_12 =
                C2RustUnnamed_12{c: 0 as *mut libc::c_uchar,};
            p.c = (*cmptr).__cmsg_data.as_mut_ptr();
            if_index = (*p.p).ipi6_ifindex as libc::c_int;
            dst_addr = (*p.p).ipi6_addr
        }
        cmptr = __cmsg_nxthdr(&mut msg, cmptr)
    }
    if indextoname((*dnsmasq_daemon).dhcp6fd, if_index,
                   ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) == 0 {
        return
    }
    port = relay_reply6(&mut from, sz, ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
    if port as libc::c_int != 0 as libc::c_int {
        from.sin6_port = __bswap_16(port);
        while retry_send(sendto((*dnsmasq_daemon).dhcp6fd,
                                (*dnsmasq_daemon).outpacket.iov_base,
                                save_counter(-(1 as libc::c_int)) as size_t,
                                0 as libc::c_int,
                                __CONST_SOCKADDR_ARG{__sockaddr__:
                                                         &mut from as
                                                             *mut sockaddr_in6
                                                             as
                                                             *mut sockaddr,},
                                ::std::mem::size_of::<sockaddr_in6>() as
                                    libc::c_ulong as socklen_t)) != 0 {
        }
    } else {
        let mut bridge: *mut dhcp_bridge = 0 as *mut dhcp_bridge;
        let mut alias: *mut dhcp_bridge = 0 as *mut dhcp_bridge;
        tmp = (*dnsmasq_daemon).if_except;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                return
            }
            tmp = (*tmp).next
        }
        tmp = (*dnsmasq_daemon).dhcp_except;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                return
            }
            tmp = (*tmp).next
        }
        parm.current = 0 as *mut dhcp_context;
        parm.relay = 0 as *mut dhcp_relay;
        memset(&mut parm.relay_local as *mut in6_addr as *mut libc::c_void,
               0 as libc::c_int, 16 as libc::c_int as libc::c_ulong);
        parm.ind = if_index;
        parm.addr_match = 0 as libc::c_int;
        memset(&mut parm.fallback as *mut in6_addr as *mut libc::c_void,
               0 as libc::c_int, 16 as libc::c_int as libc::c_ulong);
        memset(&mut parm.ll_addr as *mut in6_addr as *mut libc::c_void,
               0 as libc::c_int, 16 as libc::c_int as libc::c_ulong);
        memset(&mut parm.ula_addr as *mut in6_addr as *mut libc::c_void,
               0 as libc::c_int, 16 as libc::c_int as libc::c_ulong);
        /* If the interface on which the DHCPv6 request was received is
         an alias of some other interface (as specified by the
         --bridge-interface option), change parm.ind so that we look
         for DHCPv6 contexts associated with the aliased interface
         instead of with the aliasing one. */
        bridge = (*dnsmasq_daemon).bridges;
        while !bridge.is_null() {
            alias = (*bridge).alias;
            while !alias.is_null() {
                if wildcard_matchn((*alias).iface.as_mut_ptr(),
                                   ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                                   16 as libc::c_int) != 0 {
                    parm.ind =
                        if_nametoindex((*bridge).iface.as_mut_ptr()) as
                            libc::c_int;
                    if parm.ind == 0 {
                        my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                      4 as libc::c_int,
                                  b"unknown interface %s in bridge-interface\x00"
                                      as *const u8 as *const libc::c_char,
                                  (*bridge).iface.as_mut_ptr());
                        return
                    }
                    break ;
                } else { alias = (*alias).next }
            }
            if !alias.is_null() { break ; }
            bridge = (*bridge).next
        }
        context = (*dnsmasq_daemon).dhcp6;
        while !context.is_null() {
            if ({
                    let mut __a: *const in6_addr =
                        &mut (*context).start6 as *mut in6_addr as
                            *const in6_addr;
                    ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                         0 as libc::c_int as libc::c_uint &&
                         (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize]
                             == 0 as libc::c_int as libc::c_uint &&
                         (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize]
                             == 0 as libc::c_int as libc::c_uint &&
                         (*__a).__in6_u.__u6_addr32[3 as libc::c_int as usize]
                             == 0 as libc::c_int as libc::c_uint) as
                        libc::c_int
                }) != 0 && (*context).prefix == 0 as libc::c_int {
                /* wildcard context for DHCP-stateless only */
                parm.current = context;
                (*context).current = 0 as *mut dhcp_context
            } else {
                /* unlinked contexts are marked by context->current == context */
                (*context).current = context;
                memset(&mut (*context).local6 as *mut in6_addr as
                           *mut libc::c_void, 0 as libc::c_int,
                       16 as libc::c_int as libc::c_ulong);
            }
            context = (*context).next
        }
        relay = (*dnsmasq_daemon).relay6;
        while !relay.is_null() {
            (*relay).current = relay;
            relay = (*relay).next
        }
        if iface_enumerate(10 as libc::c_int,
                           &mut parm as *mut iface_param as *mut libc::c_void,
                           ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                   *mut in6_addr,
                                                                               _:
                                                                                   libc::c_int,
                                                                               _:
                                                                                   libc::c_int,
                                                                               _:
                                                                                   libc::c_int,
                                                                               _:
                                                                                   libc::c_int,
                                                                               _:
                                                                                   libc::c_uint,
                                                                               _:
                                                                                   libc::c_uint,
                                                                               _:
                                                                                   *mut libc::c_void)
                                                              -> libc::c_int>,
                                                   Option<unsafe extern "C" fn()
                                                              ->
                                                                  libc::c_int>>(Some(complete_context6
                                                                                         as
                                                                                         unsafe extern "C" fn(_:
                                                                                                                  *mut in6_addr,
                                                                                                              _:
                                                                                                                  libc::c_int,
                                                                                                              _:
                                                                                                                  libc::c_int,
                                                                                                              _:
                                                                                                                  libc::c_int,
                                                                                                              _:
                                                                                                                  libc::c_int,
                                                                                                              _:
                                                                                                                  libc::c_uint,
                                                                                                              _:
                                                                                                                  libc::c_uint,
                                                                                                              _:
                                                                                                                  *mut libc::c_void)
                                                                                             ->
                                                                                                 libc::c_int)))
               == 0 {
            return
        }
        if !(*dnsmasq_daemon).if_names.is_null() ||
               !(*dnsmasq_daemon).if_addrs.is_null() {
            tmp = (*dnsmasq_daemon).if_names;
            while !tmp.is_null() {
                if !(*tmp).name.is_null() &&
                       wildcard_match((*tmp).name,
                                      ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) !=
                           0 {
                    break ;
                }
                tmp = (*tmp).next
            }
            if tmp.is_null() && parm.addr_match == 0 { return }
        }
        if !parm.relay.is_null() {
            /* Ignore requests sent to the ALL_SERVERS multicast address for relay when
	     we're listening there for DHCPv6 server reasons. */
            let mut all_servers: in6_addr =
                in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
            inet_pton(10 as libc::c_int,
                      b"FF05::1:3\x00" as *const u8 as *const libc::c_char,
                      &mut all_servers as *mut in6_addr as *mut libc::c_void);
            if ({
                    let mut __a: *const in6_addr =
                        &mut dst_addr as *mut in6_addr as *const in6_addr;
                    let mut __b: *const in6_addr =
                        &mut all_servers as *mut in6_addr as *const in6_addr;
                    ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                         (*__b).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                         &&
                         (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize]
                             ==
                             (*__b).__in6_u.__u6_addr32[1 as libc::c_int as
                                                            usize] &&
                         (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize]
                             ==
                             (*__b).__in6_u.__u6_addr32[2 as libc::c_int as
                                                            usize] &&
                         (*__a).__in6_u.__u6_addr32[3 as libc::c_int as usize]
                             ==
                             (*__b).__in6_u.__u6_addr32[3 as libc::c_int as
                                                            usize]) as
                        libc::c_int
                }) == 0 {
                relay_upstream6(parm.relay, sz, &mut from.sin6_addr,
                                from.sin6_scope_id, now);
            }
            return
        }
        /* May have configured relay, but not DHCP server */
        if (*dnsmasq_daemon).doing_dhcp6 == 0 {
            return
        } /* lose any expired leases */
        lease_prune(0 as *mut dhcp_lease, now);
        port =
            dhcp6_reply(parm.current, if_index,
                        ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                        &mut parm.fallback, &mut parm.ll_addr,
                        &mut parm.ula_addr, sz as size_t, &mut from.sin6_addr,
                        now);
        /* The port in the source address of the original request should
	 be correct, but at least once client sends from the server port,
	 so we explicitly send to the client port to a client, and the
	 server port to a relay. */
        if port as libc::c_int != 0 as libc::c_int {
            from.sin6_port = __bswap_16(port);
            while retry_send(sendto((*dnsmasq_daemon).dhcp6fd,
                                    (*dnsmasq_daemon).outpacket.iov_base,
                                    save_counter(-(1 as libc::c_int)) as
                                        size_t, 0 as libc::c_int,
                                    __CONST_SOCKADDR_ARG{__sockaddr__:
                                                             &mut from as
                                                                 *mut sockaddr_in6
                                                                 as
                                                                 *mut sockaddr,},
                                    ::std::mem::size_of::<sockaddr_in6>() as
                                        libc::c_ulong as socklen_t)) != 0 {
            }
        }
        /* These need to be called _after_ we send DHCPv6 packet, since lease_update_file()
	 may trigger sending an RA packet, which overwrites our buffer. */
        lease_update_file(now);
        lease_update_dns(0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_client_mac(mut client: *mut in6_addr,
                                        mut iface: libc::c_int,
                                        mut mac: *mut libc::c_uchar,
                                        mut maclenp: *mut libc::c_uint,
                                        mut mactypep: *mut libc::c_uint,
                                        mut now: time_t) {
    /* Receiving a packet from a host does not populate the neighbour
     cache, so we send a neighbour discovery request if we can't 
     find the sender. Repeat a few times in case of packet loss. */
    let mut neigh: neigh_packet =
        neigh_packet{type_0: 0,
                     code: 0,
                     checksum: 0,
                     reserved: 0,
                     target:
                         in6_addr{__in6_u:
                                      C2RustUnnamed{__u6_addr8: [0; 16],},},};
    let mut addr: mysockaddr =
        mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut i: libc::c_int = 0;
    let mut maclen: libc::c_int = 0;
    neigh.type_0 = 135 as libc::c_int as u8_0;
    neigh.code = 0 as libc::c_int as u8_0;
    neigh.reserved = 0 as libc::c_int as u16_0;
    neigh.target = *client;
    /* RFC4443 section-2.3: checksum has to be zero to be calculated */
    neigh.checksum = 0 as libc::c_int as u16_0; /* 100ms */
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in6.sin6_family = 10 as libc::c_int as sa_family_t;
    addr.in6.sin6_port =
        __bswap_16(IPPROTO_ICMPV6 as libc::c_int as __uint16_t);
    addr.in6.sin6_addr = *client;
    addr.in6.sin6_scope_id = iface as uint32_t;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        let mut ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
        maclen = find_mac(&mut addr, mac, 0 as libc::c_int, now);
        if maclen != 0 as libc::c_int { break ; }
        sendto((*dnsmasq_daemon).icmp6fd,
               &mut neigh as *mut neigh_packet as *const libc::c_void,
               ::std::mem::size_of::<neigh_packet>() as libc::c_ulong,
               0 as libc::c_int,
               __CONST_SOCKADDR_ARG{__sockaddr__: &mut addr.sa,},
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong as
                   socklen_t);
        ts.tv_sec = 0 as libc::c_int as __time_t;
        ts.tv_nsec = 100000000 as libc::c_int as __syscall_slong_t;
        nanosleep(&mut ts, 0 as *mut timespec);
        i += 1
    }
    *maclenp = maclen as libc::c_uint;
    *mactypep = 1 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn complete_context6(mut local: *mut in6_addr,
                                       mut prefix: libc::c_int,
                                       mut scope: libc::c_int,
                                       mut if_index: libc::c_int,
                                       mut flags: libc::c_int,
                                       mut preferred: libc::c_uint,
                                       mut valid: libc::c_uint,
                                       mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut share: *mut shared_network = 0 as *mut shared_network;
    let mut relay: *mut dhcp_relay = 0 as *mut dhcp_relay;
    let mut param: *mut iface_param = vparam as *mut iface_param;
    let mut tmp: *mut iname = 0 as *mut iname;
    /* warning */
    if if_index != (*param).ind { return 1 as libc::c_int }
    if ({
            let mut __a: *const in6_addr = local as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                 __bswap_32(0xffc00000 as libc::c_uint) ==
                 __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
        }) != 0 {
        (*param).ll_addr = *local
    } else if *(local as *const uint32_t).offset(0 as libc::c_int as isize) &
                  __bswap_32(0xff000000 as libc::c_uint) ==
                  __bswap_32(0xfd000000 as libc::c_uint) {
        (*param).ula_addr = *local
    }
    if ({
            let mut __a: *const in6_addr = local as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                 0 as libc::c_int as libc::c_uint &&
                 (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize] ==
                     0 as libc::c_int as libc::c_uint &&
                 (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize] ==
                     0 as libc::c_int as libc::c_uint &&
                 (*__a).__in6_u.__u6_addr32[3 as libc::c_int as usize] ==
                     __bswap_32(1 as libc::c_int as __uint32_t)) as
                libc::c_int
        }) != 0 ||
           ({
                let mut __a: *const in6_addr = local as *const in6_addr;
                ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                     __bswap_32(0xffc00000 as libc::c_uint) ==
                     __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
            }) != 0 ||
           *(local as *const uint8_t).offset(0 as libc::c_int as isize) as
               libc::c_int == 0xff as libc::c_int {
        return 1 as libc::c_int
    }
    /* if we have --listen-address config, see if the 
     arrival interface has a matching address. */
    tmp = (*dnsmasq_daemon).if_addrs;
    while !tmp.is_null() {
        if (*tmp).addr.sa.sa_family as libc::c_int == 10 as libc::c_int &&
               ({
                    let mut __a: *const in6_addr =
                        &mut (*tmp).addr.in6.sin6_addr as *mut in6_addr as
                            *const in6_addr;
                    let mut __b: *const in6_addr = local as *const in6_addr;
                    ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                         (*__b).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                         &&
                         (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize]
                             ==
                             (*__b).__in6_u.__u6_addr32[1 as libc::c_int as
                                                            usize] &&
                         (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize]
                             ==
                             (*__b).__in6_u.__u6_addr32[2 as libc::c_int as
                                                            usize] &&
                         (*__a).__in6_u.__u6_addr32[3 as libc::c_int as usize]
                             ==
                             (*__b).__in6_u.__u6_addr32[3 as libc::c_int as
                                                            usize]) as
                        libc::c_int
                }) != 0 {
            (*param).addr_match = 1 as libc::c_int
        }
        tmp = (*tmp).next
    }
    /* Determine a globally address on the arrival interface, even
     if we have no matching dhcp-context, because we're only
     allocating on remote subnets via relays. This
     is used as a default for the DNS server option. */
    (*param).fallback = *local;
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 8 as libc::c_int != 0 &&
               (*context).flags as libc::c_uint &
                   ((1 as libc::c_uint) << 10 as libc::c_int |
                        (1 as libc::c_uint) << 16 as libc::c_int) == 0 &&
               prefix <= (*context).prefix && (*context).current == context {
            if is_same_net6(local, &mut (*context).start6, (*context).prefix)
                   != 0 &&
                   is_same_net6(local, &mut (*context).end6,
                                (*context).prefix) != 0 {
                let mut tmp_0: *mut dhcp_context = 0 as *mut dhcp_context;
                let mut up: *mut *mut dhcp_context =
                    0 as *mut *mut dhcp_context;
                /* use interface values only for constructed contexts */
                if (*context).flags as libc::c_uint &
                       (1 as libc::c_uint) << 11 as libc::c_int == 0 {
                    valid = 0xffffffff as libc::c_uint;
                    preferred = valid
                } else if flags & 2 as libc::c_int != 0 {
                    preferred = 0 as libc::c_int as libc::c_uint
                }
                if (*context).flags as libc::c_uint &
                       (1 as libc::c_uint) << 9 as libc::c_int != 0 {
                    preferred = 0 as libc::c_int as libc::c_uint
                }
                /* order chain, longest preferred time first */
                up = &mut (*param).current;
                tmp_0 = (*param).current;
                while !tmp_0.is_null() {
                    if (*tmp_0).preferred <= preferred { break ; }
                    up = &mut (*tmp_0).current;
                    tmp_0 = (*tmp_0).current
                }
                (*context).current = *up;
                *up = context;
                (*context).local6 = *local;
                (*context).preferred = preferred;
                (*context).valid = valid
            } else {
                let mut current_block_37: u64;
                share = (*dnsmasq_daemon).shared_networks;
                while !share.is_null() {
                    /* IPv4 shared_address - ignore */
                    if !((*share).shared_addr.s_addr !=
                             0 as libc::c_int as libc::c_uint) {
                        if (*share).if_index != 0 as libc::c_int {
                            if (*share).if_index != if_index {
                                current_block_37 = 11385396242402735691;
                            } else {
                                current_block_37 = 18435049525520518667;
                            }
                        } else if ({
                                       let mut __a: *const in6_addr =
                                           &mut (*share).match_addr6 as
                                               *mut in6_addr as
                                               *const in6_addr;
                                       let mut __b: *const in6_addr =
                                           local as *const in6_addr;
                                       ((*__a).__in6_u.__u6_addr32[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                            ==
                                            (*__b).__in6_u.__u6_addr32[0 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                            &&
                                            (*__a).__in6_u.__u6_addr32[1 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                ==
                                                (*__b).__in6_u.__u6_addr32[1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize]
                                            &&
                                            (*__a).__in6_u.__u6_addr32[2 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                ==
                                                (*__b).__in6_u.__u6_addr32[2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize]
                                            &&
                                            (*__a).__in6_u.__u6_addr32[3 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                ==
                                                (*__b).__in6_u.__u6_addr32[3
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize])
                                           as libc::c_int
                                   }) == 0 {
                            current_block_37 = 11385396242402735691;
                        } else { current_block_37 = 18435049525520518667; }
                        match current_block_37 {
                            11385396242402735691 => { }
                            _ => {
                                if is_same_net6(&mut (*share).shared_addr6,
                                                &mut (*context).start6,
                                                (*context).prefix) != 0 &&
                                       is_same_net6(&mut (*share).shared_addr6,
                                                    &mut (*context).end6,
                                                    (*context).prefix) != 0 {
                                    (*context).current = (*param).current;
                                    (*param).current = context;
                                    (*context).local6 = *local;
                                    (*context).preferred =
                                        if (*context).flags as libc::c_uint &
                                               (1 as libc::c_uint) <<
                                                   9 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_uint
                                        } else { 0xffffffff as libc::c_uint };
                                    (*context).valid =
                                        0xffffffff as libc::c_uint
                                }
                            }
                        }
                    }
                    share = (*share).next
                }
            }
        }
        context = (*context).next
    }
    relay = (*dnsmasq_daemon).relay6;
    while !relay.is_null() {
        if ({
                let mut __a: *const in6_addr = local as *const in6_addr;
                let mut __b: *const in6_addr =
                    &mut (*relay).local.addr6 as *mut in6_addr as
                        *const in6_addr;
                ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                     (*__b).__in6_u.__u6_addr32[0 as libc::c_int as usize] &&
                     (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize] ==
                         (*__b).__in6_u.__u6_addr32[1 as libc::c_int as usize]
                     &&
                     (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize] ==
                         (*__b).__in6_u.__u6_addr32[2 as libc::c_int as usize]
                     &&
                     (*__a).__in6_u.__u6_addr32[3 as libc::c_int as usize] ==
                         (*__b).__in6_u.__u6_addr32[3 as libc::c_int as
                                                        usize]) as libc::c_int
            }) != 0 && (*relay).current == relay &&
               (({
                     let mut __a: *const in6_addr =
                         &mut (*param).relay_local as *mut in6_addr as
                             *const in6_addr;
                     ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                          0 as libc::c_int as libc::c_uint &&
                          (*__a).__in6_u.__u6_addr32[1 as libc::c_int as
                                                         usize] ==
                              0 as libc::c_int as libc::c_uint &&
                          (*__a).__in6_u.__u6_addr32[2 as libc::c_int as
                                                         usize] ==
                              0 as libc::c_int as libc::c_uint &&
                          (*__a).__in6_u.__u6_addr32[3 as libc::c_int as
                                                         usize] ==
                              0 as libc::c_int as libc::c_uint) as libc::c_int
                 }) != 0 ||
                    ({
                         let mut __a: *const in6_addr =
                             local as *const in6_addr;
                         let mut __b: *const in6_addr =
                             &mut (*param).relay_local as *mut in6_addr as
                                 *const in6_addr;
                         ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as
                                                         usize] ==
                              (*__b).__in6_u.__u6_addr32[0 as libc::c_int as
                                                             usize] &&
                              (*__a).__in6_u.__u6_addr32[1 as libc::c_int as
                                                             usize] ==
                                  (*__b).__in6_u.__u6_addr32[1 as libc::c_int
                                                                 as usize] &&
                              (*__a).__in6_u.__u6_addr32[2 as libc::c_int as
                                                             usize] ==
                                  (*__b).__in6_u.__u6_addr32[2 as libc::c_int
                                                                 as usize] &&
                              (*__a).__in6_u.__u6_addr32[3 as libc::c_int as
                                                             usize] ==
                                  (*__b).__in6_u.__u6_addr32[3 as libc::c_int
                                                                 as usize]) as
                             libc::c_int
                     }) != 0) {
            (*relay).current = (*param).relay;
            (*param).relay = relay;
            (*param).relay_local = *local
        }
        relay = (*relay).next
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn config_find_by_address6(mut configs:
                                                     *mut dhcp_config,
                                                 mut net: *mut in6_addr,
                                                 mut prefix: libc::c_int,
                                                 mut addr: *mut in6_addr)
 -> *mut dhcp_config {
    let mut config: *mut dhcp_config = 0 as *mut dhcp_config;
    config = configs;
    while !config.is_null() {
        if (*config).flags & 4096 as libc::c_int as libc::c_uint != 0 {
            let mut addr_list: *mut addrlist = 0 as *mut addrlist;
            addr_list = (*config).addr6;
            while !addr_list.is_null() {
                if (net.is_null() ||
                        is_same_net6(&mut (*addr_list).addr.addr6, net,
                                     prefix) != 0 ||
                        (*addr_list).flags & 16 as libc::c_int != 0 &&
                            prefix == 64 as libc::c_int) &&
                       is_same_net6(&mut (*addr_list).addr.addr6, addr,
                                    (if (*addr_list).flags & 8 as libc::c_int
                                            != 0 {
                                         (*addr_list).prefixlen
                                     } else { 128 as libc::c_int })) != 0 {
                    return config
                }
                addr_list = (*addr_list).next
            }
        }
        config = (*config).next
    }
    return 0 as *mut dhcp_config;
}
#[no_mangle]
pub unsafe extern "C" fn address6_allocate(mut context: *mut dhcp_context,
                                           mut clid: *mut libc::c_uchar,
                                           mut clid_len: libc::c_int,
                                           mut temp_addr: libc::c_int,
                                           mut iaid: libc::c_uint,
                                           mut serial: libc::c_int,
                                           mut netids: *mut dhcp_netid,
                                           mut plain_range: libc::c_int,
                                           mut ans: *mut in6_addr)
 -> *mut dhcp_context {
    /* Find a free address: exclude anything in use and anything allocated to
     a particular hwaddr/clientid/hostname in our configuration.
     Try to return from contexts which match netids first. 
     
     Note that we assume the address prefix lengths are 64 or greater, so we can
     get by with 64 bit arithmetic.
*/
    let mut start: u64_0 = 0;
    let mut addr: u64_0 = 0;
    let mut c: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut d: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut i: libc::c_int = 0;
    let mut pass: libc::c_int = 0;
    let mut j: u64_0 = 0;
    /* hash hwaddr: use the SDBM hashing algorithm.  This works
     for MAC addresses, let's see how it manages with client-ids! 
     For temporary addresses, we generate a new random one each time. */
    if temp_addr != 0 {
        j = rand64()
    } else {
        j = iaid as u64_0;
        i = 0 as libc::c_int;
        while i < clid_len {
            j =
                (*clid.offset(i as isize) as
                     libc::c_ulonglong).wrapping_add(j <<
                                                         6 as
                                                             libc::c_int).wrapping_add(j
                                                                                           <<
                                                                                           16
                                                                                               as
                                                                                               libc::c_int).wrapping_sub(j);
            i += 1
        }
    }
    pass = 0 as libc::c_int;
    while if pass <= plain_range {
              1 as libc::c_int
          } else { 0 as libc::c_int } != 0 {
        c = context;
        while !c.is_null() {
            if !((*c).flags as libc::c_uint &
                     ((1 as libc::c_uint) << 9 as libc::c_int |
                          (1 as libc::c_uint) << 0 as libc::c_int |
                          (1 as libc::c_uint) << 7 as libc::c_int |
                          (1 as libc::c_uint) << 15 as libc::c_int) != 0) {
                if !(match_netid((*c).filter, netids, pass) == 0) {
                    if temp_addr == 0 &&
                           (*dnsmasq_daemon).options[(34 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (34 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               != 0 {
                        /* seed is largest extant lease addr in this context,
		 skip addresses equal to the number of addresses rejected
		 by clients. This should avoid the same client being offered the same
		 address after it has rjected it. */
                        start =
                            lease_find_max_addr6(c).wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulonglong).wrapping_add(serial
                                                                                                         as
                                                                                                         libc::c_ulonglong).wrapping_add((*c).addr_epoch
                                                                                                                                             as
                                                                                                                                             libc::c_ulonglong);
                        if (*c).addr_epoch != 0 {
                            (*c).addr_epoch = (*c).addr_epoch.wrapping_sub(1)
                        }
                    } else {
                        let mut range: u64_0 =
                            (1 as libc::c_int as
                                 libc::c_ulonglong).wrapping_add(addr6part(&mut (*c).end6)).wrapping_sub(addr6part(&mut (*c).start6));
                        let mut offset: u64_0 =
                            j.wrapping_add((*c).addr_epoch as
                                               libc::c_ulonglong);
                        /* don't divide by zero if range is whole 2^64 */
                        if range != 0 as libc::c_int as libc::c_ulonglong {
                            offset = offset.wrapping_rem(range)
                        }
                        start =
                            addr6part(&mut (*c).start6).wrapping_add(offset)
                    }
                    /* iterate until we find a free address. */
                    addr = start;
                    loop  {
                        /* eliminate addresses in use by the server. */
                        d = context;
                        while !d.is_null() {
                            if addr == addr6part(&mut (*d).local6) { break ; }
                            d = (*d).current
                        }
                        *ans = (*c).start6;
                        setaddr6part(ans, addr);
                        if d.is_null() &&
                               lease6_find_by_addr(&mut (*c).start6,
                                                   (*c).prefix,
                                                   addr).is_null() &&
                               config_find_by_address6((*dnsmasq_daemon).dhcp_conf,
                                                       &mut (*c).start6,
                                                       (*c).prefix,
                                                       ans).is_null() {
                            return c
                        }
                        addr = addr.wrapping_add(1);
                        if addr ==
                               addr6part(&mut (*c).end6).wrapping_add(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulonglong)
                           {
                            addr = addr6part(&mut (*c).start6)
                        }
                        if !(addr != start) { break ; }
                    }
                }
            }
            c = (*c).current
        }
        pass += 1
    }
    return 0 as *mut dhcp_context;
}
/* can dynamically allocate addr */
#[no_mangle]
pub unsafe extern "C" fn address6_available(mut context: *mut dhcp_context,
                                            mut taddr: *mut in6_addr,
                                            mut netids: *mut dhcp_netid,
                                            mut plain_range: libc::c_int)
 -> *mut dhcp_context {
    let mut start: u64_0 = 0;
    let mut end: u64_0 = 0;
    let mut addr: u64_0 = addr6part(taddr);
    let mut tmp: *mut dhcp_context = 0 as *mut dhcp_context;
    tmp = context;
    while !tmp.is_null() {
        start = addr6part(&mut (*tmp).start6);
        end = addr6part(&mut (*tmp).end6);
        if (*tmp).flags as libc::c_uint &
               ((1 as libc::c_uint) << 0 as libc::c_int |
                    (1 as libc::c_uint) << 7 as libc::c_int) == 0 &&
               is_same_net6(&mut (*tmp).start6, taddr, (*tmp).prefix) != 0 &&
               is_same_net6(&mut (*tmp).end6, taddr, (*tmp).prefix) != 0 &&
               addr >= start && addr <= end &&
               match_netid((*tmp).filter, netids, plain_range) != 0 {
            return tmp
        }
        tmp = (*tmp).current
    }
    return 0 as *mut dhcp_context;
}
/* address OK if configured */
#[no_mangle]
pub unsafe extern "C" fn address6_valid(mut context: *mut dhcp_context,
                                        mut taddr: *mut in6_addr,
                                        mut netids: *mut dhcp_netid,
                                        mut plain_range: libc::c_int)
 -> *mut dhcp_context {
    let mut tmp: *mut dhcp_context = 0 as *mut dhcp_context;
    tmp = context;
    while !tmp.is_null() {
        if is_same_net6(&mut (*tmp).start6, taddr, (*tmp).prefix) != 0 &&
               match_netid((*tmp).filter, netids, plain_range) != 0 {
            return tmp
        }
        tmp = (*tmp).current
    }
    return 0 as *mut dhcp_context;
}
#[no_mangle]
pub unsafe extern "C" fn make_duid(mut now: time_t) {
    if !(*dnsmasq_daemon).duid_config.is_null() {
        let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        p =
            safe_malloc((*dnsmasq_daemon).duid_config_len.wrapping_add(6 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)
                            as size_t) as *mut libc::c_uchar;
        (*dnsmasq_daemon).duid = p;
        (*dnsmasq_daemon).duid_len =
            (*dnsmasq_daemon).duid_config_len.wrapping_add(6 as libc::c_int as
                                                               libc::c_uint)
                as libc::c_int;
        let mut t_s: u16_0 = 2 as libc::c_int as u16_0;
        let mut t_cp: *mut libc::c_uchar = p;
        let fresh6 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh6 = (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp = t_s as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        /* DUID_EN */
        let mut t_l: u32_0 = (*dnsmasq_daemon).duid_enterprise;
        let mut t_cp_0: *mut libc::c_uchar = p;
        let fresh7 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh7 = (t_l >> 24 as libc::c_int) as libc::c_uchar;
        let fresh8 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh8 = (t_l >> 16 as libc::c_int) as libc::c_uchar;
        let fresh9 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh9 = (t_l >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_0 = t_l as libc::c_uchar;
        p = p.offset(4 as libc::c_int as isize);
        memcpy(p as *mut libc::c_void,
               (*dnsmasq_daemon).duid_config as *const libc::c_void,
               (*dnsmasq_daemon).duid_config_len as libc::c_ulong);
    } else {
        let mut newnow: time_t = 0 as libc::c_int as time_t;
        /* If we have no persistent lease database, or a non-stable RTC, use DUID_LL (newnow == 0) */
        /* rebase epoch to 1/1/2000 */
        if (*dnsmasq_daemon).options[(22 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (22 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 || !(*dnsmasq_daemon).lease_change_command.is_null() {
            newnow = now - 946684800 as libc::c_int as libc::c_long
        }
        iface_enumerate(1 as libc::c_int,
                        &mut newnow as *mut time_t as *mut libc::c_void,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                libc::c_int,
                                                                            _:
                                                                                libc::c_uint,
                                                                            _:
                                                                                *mut libc::c_char,
                                                                            _:
                                                                                size_t,
                                                                            _:
                                                                                *mut libc::c_void)
                                                           -> libc::c_int>,
                                                Option<unsafe extern "C" fn()
                                                           ->
                                                               libc::c_int>>(Some(make_duid1
                                                                                      as
                                                                                      unsafe extern "C" fn(_:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               libc::c_uint,
                                                                                                           _:
                                                                                                               *mut libc::c_char,
                                                                                                           _:
                                                                                                               size_t,
                                                                                                           _:
                                                                                                               *mut libc::c_void)
                                                                                          ->
                                                                                              libc::c_int)));
        if (*dnsmasq_daemon).duid.is_null() {
            die(b"Cannot create DHCPv6 server DUID: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char, 5 as libc::c_int);
        }
    };
}
unsafe extern "C" fn make_duid1(mut index: libc::c_int,
                                mut type_0: libc::c_uint,
                                mut mac: *mut libc::c_char,
                                mut maclen: size_t,
                                mut parm: *mut libc::c_void) -> libc::c_int {
    /* create DUID as specified in RFC3315. We use the MAC of the
     first interface we find that isn't loopback or P-to-P and
     has address-type < 256. Address types above 256 are things like 
     tunnels which don't have usable MAC addresses. */
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut newnow: time_t = *(parm as *mut time_t);
    if type_0 >= 256 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    if newnow == 0 as libc::c_int as libc::c_long {
        p =
            safe_malloc(maclen.wrapping_add(4 as libc::c_int as
                                                libc::c_ulong)) as
                *mut libc::c_uchar;
        (*dnsmasq_daemon).duid = p;
        (*dnsmasq_daemon).duid_len =
            maclen.wrapping_add(4 as libc::c_int as libc::c_ulong) as
                libc::c_int;
        let mut t_s: u16_0 = 3 as libc::c_int as u16_0;
        let mut t_cp: *mut libc::c_uchar = p;
        let fresh10 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh10 = (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp = t_s as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        /* address type */
        let mut t_s_0: u16_0 = type_0 as u16_0;
        let mut t_cp_0: *mut libc::c_uchar = p;
        let fresh11 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh11 =
            (t_s_0 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_0 = t_s_0 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize)
    } else {
        p =
            safe_malloc(maclen.wrapping_add(8 as libc::c_int as
                                                libc::c_ulong)) as
                *mut libc::c_uchar;
        (*dnsmasq_daemon).duid = p;
        (*dnsmasq_daemon).duid_len =
            maclen.wrapping_add(8 as libc::c_int as libc::c_ulong) as
                libc::c_int;
        let mut t_s_1: u16_0 = 1 as libc::c_int as u16_0;
        let mut t_cp_1: *mut libc::c_uchar = p;
        let fresh12 = t_cp_1;
        t_cp_1 = t_cp_1.offset(1);
        *fresh12 =
            (t_s_1 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_1 = t_s_1 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        /* DUID_LL */
        /* time */
        let mut t_s_2: u16_0 = type_0 as u16_0;
        let mut t_cp_2: *mut libc::c_uchar = p;
        let fresh13 = t_cp_2;
        t_cp_2 = t_cp_2.offset(1);
        *fresh13 =
            (t_s_2 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_2 = t_s_2 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_l: u32_0 = *(parm as *mut time_t) as u32_0;
        let mut t_cp_3: *mut libc::c_uchar = p;
        let fresh14 = t_cp_3;
        t_cp_3 = t_cp_3.offset(1);
        *fresh14 = (t_l >> 24 as libc::c_int) as libc::c_uchar;
        let fresh15 = t_cp_3;
        t_cp_3 = t_cp_3.offset(1);
        *fresh15 = (t_l >> 16 as libc::c_int) as libc::c_uchar;
        let fresh16 = t_cp_3;
        t_cp_3 = t_cp_3.offset(1);
        *fresh16 = (t_l >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_3 = t_l as libc::c_uchar;
        p = p.offset(4 as libc::c_int as isize)
    }
    memcpy(p as *mut libc::c_void, mac as *const libc::c_void, maclen);
    return 0 as libc::c_int;
}
unsafe extern "C" fn construct_worker(mut local: *mut in6_addr,
                                      mut prefix: libc::c_int,
                                      mut scope: libc::c_int,
                                      mut if_index: libc::c_int,
                                      mut flags: libc::c_int,
                                      mut preferred: libc::c_int,
                                      mut valid: libc::c_int,
                                      mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut ifrn_name: [libc::c_char; 16] = [0; 16];
    let mut start6: in6_addr =
        in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    let mut end6: in6_addr =
        in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    let mut template: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut tmp: *mut iname = 0 as *mut iname;
    let mut param: *mut cparam = vparam as *mut cparam;
    if ({
            let mut __a: *const in6_addr = local as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                 0 as libc::c_int as libc::c_uint &&
                 (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize] ==
                     0 as libc::c_int as libc::c_uint &&
                 (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize] ==
                     0 as libc::c_int as libc::c_uint &&
                 (*__a).__in6_u.__u6_addr32[3 as libc::c_int as usize] ==
                     __bswap_32(1 as libc::c_int as __uint32_t)) as
                libc::c_int
        }) != 0 ||
           ({
                let mut __a: *const in6_addr = local as *const in6_addr;
                ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                     __bswap_32(0xffc00000 as libc::c_uint) ==
                     __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
            }) != 0 ||
           *(local as *const uint8_t).offset(0 as libc::c_int as isize) as
               libc::c_int == 0xff as libc::c_int {
        return 1 as libc::c_int
    }
    if flags & 4 as libc::c_int == 0 { return 1 as libc::c_int }
    if flags & 2 as libc::c_int != 0 { return 1 as libc::c_int }
    /* DUID_LLT */
    /* address type */
    /* Ignore interfaces where we're not doing RA/DHCP6 */
    if indextoname((*dnsmasq_daemon).icmp6fd, if_index,
                   ifrn_name.as_mut_ptr()) == 0 ||
           iface_check(1 as libc::c_int, 0 as *mut all_addr,
                       ifrn_name.as_mut_ptr(), 0 as *mut libc::c_int) == 0 {
        return 1 as libc::c_int
    }
    tmp = (*dnsmasq_daemon).dhcp_except;
    while !tmp.is_null() {
        if !(*tmp).name.is_null() &&
               wildcard_match((*tmp).name, ifrn_name.as_mut_ptr()) != 0 {
            return 1 as libc::c_int
        }
        tmp = (*tmp).next
    }
    template = (*dnsmasq_daemon).dhcp6;
    while !template.is_null() {
        if (*template).flags as libc::c_uint &
               ((1 as libc::c_uint) << 10 as libc::c_int |
                    (1 as libc::c_uint) << 11 as libc::c_int) == 0 {
            /* non-template entries, just fill in interface and local addresses */
            if prefix <= (*template).prefix &&
                   is_same_net6(local, &mut (*template).start6,
                                (*template).prefix) != 0 &&
                   is_same_net6(local, &mut (*template).end6,
                                (*template).prefix) != 0 {
                /* First time found, do fast RA. */
                if (*template).if_index == 0 as libc::c_int {
                    ra_start_unsolicited((*param).now, template);
                    (*param).newone = 1 as libc::c_int
                }
                (*template).if_index = if_index;
                (*template).local6 = *local
            }
        } else if wildcard_match((*template).template_interface,
                                 ifrn_name.as_mut_ptr()) != 0 &&
                      (*template).prefix >= prefix {
            start6 = *local;
            setaddr6part(&mut start6, addr6part(&mut (*template).start6));
            end6 = *local;
            setaddr6part(&mut end6, addr6part(&mut (*template).end6));
            context = (*dnsmasq_daemon).dhcp6;
            while !context.is_null() {
                if (*context).flags as libc::c_uint &
                       (1 as libc::c_uint) << 10 as libc::c_int == 0 &&
                       ({
                            let mut __a: *const in6_addr =
                                &mut start6 as *mut in6_addr as
                                    *const in6_addr;
                            let mut __b: *const in6_addr =
                                &mut (*context).start6 as *mut in6_addr as
                                    *const in6_addr;
                            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as
                                                            usize] ==
                                 (*__b).__in6_u.__u6_addr32[0 as libc::c_int
                                                                as usize] &&
                                 (*__a).__in6_u.__u6_addr32[1 as libc::c_int
                                                                as usize] ==
                                     (*__b).__in6_u.__u6_addr32[1 as
                                                                    libc::c_int
                                                                    as usize]
                                 &&
                                 (*__a).__in6_u.__u6_addr32[2 as libc::c_int
                                                                as usize] ==
                                     (*__b).__in6_u.__u6_addr32[2 as
                                                                    libc::c_int
                                                                    as usize]
                                 &&
                                 (*__a).__in6_u.__u6_addr32[3 as libc::c_int
                                                                as usize] ==
                                     (*__b).__in6_u.__u6_addr32[3 as
                                                                    libc::c_int
                                                                    as usize])
                                as libc::c_int
                        }) != 0 &&
                       ({
                            let mut __a: *const in6_addr =
                                &mut end6 as *mut in6_addr as *const in6_addr;
                            let mut __b: *const in6_addr =
                                &mut (*context).end6 as *mut in6_addr as
                                    *const in6_addr;
                            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as
                                                            usize] ==
                                 (*__b).__in6_u.__u6_addr32[0 as libc::c_int
                                                                as usize] &&
                                 (*__a).__in6_u.__u6_addr32[1 as libc::c_int
                                                                as usize] ==
                                     (*__b).__in6_u.__u6_addr32[1 as
                                                                    libc::c_int
                                                                    as usize]
                                 &&
                                 (*__a).__in6_u.__u6_addr32[2 as libc::c_int
                                                                as usize] ==
                                     (*__b).__in6_u.__u6_addr32[2 as
                                                                    libc::c_int
                                                                    as usize]
                                 &&
                                 (*__a).__in6_u.__u6_addr32[3 as libc::c_int
                                                                as usize] ==
                                     (*__b).__in6_u.__u6_addr32[3 as
                                                                    libc::c_int
                                                                    as usize])
                                as libc::c_int
                        }) != 0 {
                    /* If there's an absolute address context covering this address
		 then don't construct one as well. */
                    if (*context).flags as libc::c_uint &
                           (1 as libc::c_uint) << 11 as libc::c_int == 0 {
                        break ;
                    }
                    if (*context).if_index == if_index {
                        let mut cflags: libc::c_int = (*context).flags;
                        (*context).flags =
                            ((*context).flags as libc::c_uint &
                                 !((1 as libc::c_uint) << 12 as libc::c_int |
                                       (1 as libc::c_uint) <<
                                           16 as libc::c_int)) as libc::c_int;
                        if cflags as libc::c_uint &
                               (1 as libc::c_uint) << 16 as libc::c_int != 0 {
                            /* address went, now it's back, and on the same interface */
                            log_context(10 as libc::c_int, context);
                            /* fast RAs for a while */
                            ra_start_unsolicited((*param).now, context);
                            (*param).newone = 1 as libc::c_int;
                            /* Add address to name again */
                            if (*context).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 6 as libc::c_int !=
                                   0 {
                                (*param).newname = 1 as libc::c_int
                            }
                        }
                        break ;
                    }
                }
                context = (*context).next
            }
            if context.is_null() &&
                   {
                       context =
                           whine_malloc(::std::mem::size_of::<dhcp_context>()
                                            as libc::c_ulong) as
                               *mut dhcp_context;
                       !context.is_null()
                   } {
                *context = *template;
                (*context).start6 = start6;
                (*context).end6 = end6;
                (*context).flags =
                    ((*context).flags as libc::c_uint &
                         !((1 as libc::c_uint) << 10 as libc::c_int)) as
                        libc::c_int;
                (*context).flags =
                    ((*context).flags as libc::c_uint |
                         (1 as libc::c_uint) << 11 as libc::c_int) as
                        libc::c_int;
                (*context).if_index = if_index;
                (*context).local6 = *local;
                (*context).saved_valid = 0 as libc::c_int as libc::c_uint;
                (*context).next = (*dnsmasq_daemon).dhcp6;
                (*dnsmasq_daemon).dhcp6 = context;
                ra_start_unsolicited((*param).now, context);
                /* we created a new one, need to call
	       lease_update_file to get periodic functions called */
                (*param).newone = 1 as libc::c_int;
                /* Will need to add new putative SLAAC addresses to existing leases */
                if (*context).flags as libc::c_uint &
                       (1 as libc::c_uint) << 6 as libc::c_int != 0 {
                    (*param).newname = 1 as libc::c_int
                }
                log_context(10 as libc::c_int, context);
            }
        }
        template = (*template).next
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dhcp_construct_contexts(mut now: time_t) {
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut tmp: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut up: *mut *mut dhcp_context = 0 as *mut *mut dhcp_context;
    let mut param: cparam = cparam{now: 0, newone: 0, newname: 0,};
    param.newone = 0 as libc::c_int;
    param.newname = 0 as libc::c_int;
    param.now = now;
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 11 as libc::c_int != 0 {
            (*context).flags =
                ((*context).flags as libc::c_uint |
                     (1 as libc::c_uint) << 12 as libc::c_int) as libc::c_int
        }
        context = (*context).next
    }
    iface_enumerate(10 as libc::c_int,
                    &mut param as *mut cparam as *mut libc::c_void,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut in6_addr,
                                                                        _:
                                                                            libc::c_int,
                                                                        _:
                                                                            libc::c_int,
                                                                        _:
                                                                            libc::c_int,
                                                                        _:
                                                                            libc::c_int,
                                                                        _:
                                                                            libc::c_int,
                                                                        _:
                                                                            libc::c_int,
                                                                        _:
                                                                            *mut libc::c_void)
                                                       -> libc::c_int>,
                                            Option<unsafe extern "C" fn()
                                                       ->
                                                           libc::c_int>>(Some(construct_worker
                                                                                  as
                                                                                  unsafe extern "C" fn(_:
                                                                                                           *mut in6_addr,
                                                                                                       _:
                                                                                                           libc::c_int,
                                                                                                       _:
                                                                                                           libc::c_int,
                                                                                                       _:
                                                                                                           libc::c_int,
                                                                                                       _:
                                                                                                           libc::c_int,
                                                                                                       _:
                                                                                                           libc::c_int,
                                                                                                       _:
                                                                                                           libc::c_int,
                                                                                                       _:
                                                                                                           *mut libc::c_void)
                                                                                      ->
                                                                                          libc::c_int)));
    up = &mut (*dnsmasq_daemon).dhcp6;
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        tmp = (*context).next;
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 12 as libc::c_int != 0 &&
               (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 16 as libc::c_int == 0 {
            if (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 13 as libc::c_int != 0 ||
                   (*dnsmasq_daemon).options[(37 as libc::c_int as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &
                       (1 as libc::c_uint) <<
                           (37 as libc::c_int as
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                       != 0 {
                /* previously constructed context has gone. advertise it's demise */
                (*context).flags =
                    ((*context).flags as libc::c_uint |
                         (1 as libc::c_uint) << 16 as libc::c_int) as
                        libc::c_int;
                (*context).address_lost_time = now;
                /* Apply same ceiling of configured lease time as in radv.c */
                if (*context).saved_valid > (*context).lease_time {
                    (*context).saved_valid = (*context).lease_time
                }
                /* maximum time is 2 hours, from RFC */
                if (*context).saved_valid >
                       7200 as libc::c_int as libc::c_uint {
                    /* 2 hours */
                    (*context).saved_valid =
                        7200 as libc::c_int as libc::c_uint
                } /* include deletion */
                ra_start_unsolicited(now, context);
                param.newone = 1 as libc::c_int;
                if (*context).flags as libc::c_uint &
                       (1 as libc::c_uint) << 6 as libc::c_int != 0 {
                    param.newname = 1 as libc::c_int
                }
                log_context(10 as libc::c_int, context);
                up = &mut (*context).next
            } else {
                /* we were never doing RA for this, so free now */
                *up = (*context).next;
                free(context as *mut libc::c_void);
            }
        } else { up = &mut (*context).next }
        context = tmp
    }
    if param.newone != 0 {
        if !(*dnsmasq_daemon).dhcp.is_null() ||
               (*dnsmasq_daemon).doing_dhcp6 != 0 {
            if param.newname != 0 { lease_update_slaac(now); }
            lease_update_file(now);
        } else {
            /* Not doing DHCP, so no lease system, manage alarms for ra only */
            send_alarm(periodic_ra(now), now);
        }
    };
}
