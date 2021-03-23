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
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE)
     -> *mut libc::c_char;
    #[no_mangle]
    fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
                  __delimiter: libc::c_int, __stream: *mut FILE) -> __ssize_t;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int,
              __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inet_pton(__af: libc::c_int, __cp: *const libc::c_char,
                 __buf: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn inet_ntop(__af: libc::c_int, __cp: *const libc::c_void,
                 __buf: *mut libc::c_char, __len: socklen_t)
     -> *const libc::c_char;
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
    fn if_indextoname(__ifindex: libc::c_uint, __ifname: *mut libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn __uflow(_: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
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
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
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
    fn rand16() -> libc::c_ushort;
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
    fn expand_buf(iov: *mut iovec, size: size_t) -> libc::c_int;
    #[no_mangle]
    fn print_mac(buff: *mut libc::c_char, mac: *mut libc::c_uchar,
                 len: libc::c_int) -> *mut libc::c_char;
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
    fn lease_ping_reply(sender: *mut in6_addr, packet: *mut libc::c_uchar,
                        interface: *mut libc::c_char);
    #[no_mangle]
    fn iface_enumerate(family: libc::c_int, parm: *mut libc::c_void,
                       callback:
                           Option<unsafe extern "C" fn() -> libc::c_int>)
     -> libc::c_int;
    #[no_mangle]
    fn recv_dhcp_packet(fd: libc::c_int, msg: *mut msghdr) -> ssize_t;
    #[no_mangle]
    fn option_filter(tags: *mut dhcp_netid, context_tags: *mut dhcp_netid,
                     opts: *mut dhcp_opt) -> *mut dhcp_netid;
    #[no_mangle]
    fn reset_counter();
    #[no_mangle]
    fn save_counter(newval: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn expand(headroom: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn put_opt6(data: *mut libc::c_void, len: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn put_opt6_long(val: libc::c_uint);
    #[no_mangle]
    fn put_opt6_short(val: libc::c_uint);
    #[no_mangle]
    fn put_opt6_char(val: libc::c_uint);
    #[no_mangle]
    fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> libc::c_int;
    #[no_mangle]
    fn getsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *mut libc::c_void,
                  __optlen: *mut socklen_t) -> libc::c_int;
    #[no_mangle]
    fn sendto(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t,
              __flags: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
              __addr_len: socklen_t) -> ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ra_packet {
    pub type_0: u8_0,
    pub code: u8_0,
    pub checksum: u16_0,
    pub hop_limit: u8_0,
    pub flags: u8_0,
    pub lifetime: u16_0,
    pub reachable_time: u32_0,
    pub retrans_time: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prefix_opt {
    pub type_0: u8_0,
    pub len: u8_0,
    pub prefix_len: u8_0,
    pub flags: u8_0,
    pub valid_lifetime: u32_0,
    pub preferred_lifetime: u32_0,
    pub reserved: u32_0,
    pub prefix: in6_addr,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union all_addr {
    pub addr4: in_addr,
    pub addr6: in6_addr,
    pub cname: C2RustUnnamed_6,
    pub key: C2RustUnnamed_5,
    pub ds: C2RustUnnamed_4,
    pub srv: C2RustUnnamed_3,
    pub log: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub keytag: libc::c_ushort,
    pub algo: libc::c_ushort,
    pub digest: libc::c_ushort,
    pub rcode: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub struct C2RustUnnamed_4 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub flags: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub target: C2RustUnnamed_7,
    pub uid: libc::c_uint,
    pub is_name_ptr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
    pub name: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
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
    pub u: C2RustUnnamed_9,
    pub val: *mut libc::c_uchar,
    pub netid: *mut dhcp_netid,
    pub next: *mut dhcp_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp6_filter {
    pub icmp6_filt: [uint32_t; 8],
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
/* NB. This code may be called during a DHCPv4 or transaction which is in ping-wait
   It therefore cannot use any DHCP buffer resources except outpacket, which is
   not used by DHCPv4 code. This code may also be called when DHCP 4 or 6 isn't
   active, so we ensure that outpacket is allocated here too */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ra_param {
    pub now: time_t,
    pub ind: libc::c_int,
    pub managed: libc::c_int,
    pub other: libc::c_int,
    pub first: libc::c_int,
    pub adv_router: libc::c_int,
    pub if_name: *mut libc::c_char,
    pub tags: *mut dhcp_netid,
    pub link_local: in6_addr,
    pub link_global: in6_addr,
    pub ula: in6_addr,
    pub glob_pref_time: libc::c_uint,
    pub link_pref_time: libc::c_uint,
    pub ula_pref_time: libc::c_uint,
    pub adv_interval: libc::c_uint,
    pub prio: libc::c_uint,
    pub found_context: *mut dhcp_context,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub c: *mut libc::c_uchar,
    pub p: *mut in6_pktinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub align: cmsghdr,
    pub control6: [libc::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alias_param {
    pub iface: libc::c_int,
    pub bridge: *mut dhcp_bridge,
    pub num_alias_ifs: libc::c_int,
    pub max_alias_ifs: libc::c_int,
    pub alias_ifs: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_param {
    pub now: time_t,
    pub iface: libc::c_int,
    pub name: [libc::c_char; 17],
}
#[inline]
unsafe extern "C" fn __cmsg_nxthdr(mut __mhdr: *mut msghdr,
                                   mut __cmsg: *mut cmsghdr) -> *mut cmsghdr {
    if (*__cmsg).cmsg_len < ::std::mem::size_of::<cmsghdr>() as libc::c_ulong
       {
        return 0 as *mut cmsghdr
    } /* radvd uses this value */
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
unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
    return __x;
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
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
               (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
               (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
               (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int |
                (__bsx as libc::c_int & 0xff as libc::c_int) <<
                    8 as libc::c_int) as __uint16_t;
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
static mut hop_limit: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn ra_init(mut now: time_t) {
    let mut filter: icmp6_filter = icmp6_filter{icmp6_filt: [0; 8],};
    let mut fd: libc::c_int = 0;
    let mut class: libc::c_int = 0xc0 as libc::c_int;
    let mut val: libc::c_int = 255 as libc::c_int;
    let mut len: socklen_t =
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    /* ensure this is around even if we're not doing DHCPv6 */
    expand_buf(&mut (*dnsmasq_daemon).outpacket,
               ::std::mem::size_of::<dhcp_packet>() as libc::c_ulong);
    /* See if we're guessing SLAAC addresses, if so we need to receive ping replies */
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 6 as libc::c_int != 0 {
            break ;
        }
        context = (*context).next
    }
    /* Need ICMP6 socket for transmission for DHCPv6 even when not doing RA. */
    memset(&mut filter as *mut icmp6_filter as *mut libc::c_void,
           0xff as libc::c_int,
           ::std::mem::size_of::<icmp6_filter>() as libc::c_ulong);
    if (*dnsmasq_daemon).doing_ra != 0 {
        filter.icmp6_filt[(133 as libc::c_int >> 5 as libc::c_int) as usize]
            &=
            !((1 as libc::c_int) << (133 as libc::c_int & 31 as libc::c_int))
                as libc::c_uint;
        if !context.is_null() {
            filter.icmp6_filt[(129 as libc::c_int >> 5 as libc::c_int) as
                                  usize] &=
                !((1 as libc::c_int) <<
                      (129 as libc::c_int & 31 as libc::c_int)) as
                    libc::c_uint
        }
    }
    fd =
        socket(10 as libc::c_int, SOCK_RAW as libc::c_int,
               IPPROTO_ICMPV6 as libc::c_int);
    if fd == -(1 as libc::c_int) ||
           getsockopt(fd, IPPROTO_IPV6 as libc::c_int, 16 as libc::c_int,
                      &mut hop_limit as *mut libc::c_int as *mut libc::c_void,
                      &mut len) != 0 ||
           setsockopt(fd, IPPROTO_IPV6 as libc::c_int, 67 as libc::c_int,
                      &mut class as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) || fix_fd(fd) == 0
           || set_ipv6pktinfo(fd) == 0 ||
           setsockopt(fd, IPPROTO_IPV6 as libc::c_int, 16 as libc::c_int,
                      &mut val as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) != 0 ||
           setsockopt(fd, IPPROTO_IPV6 as libc::c_int, 18 as libc::c_int,
                      &mut val as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) != 0 ||
           setsockopt(fd, IPPROTO_ICMPV6 as libc::c_int, 1 as libc::c_int,
                      &mut filter as *mut icmp6_filter as *const libc::c_void,
                      ::std::mem::size_of::<icmp6_filter>() as libc::c_ulong
                          as socklen_t) == -(1 as libc::c_int) {
        die(b"cannot create ICMPv6 socket: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 2 as libc::c_int);
    }
    (*dnsmasq_daemon).icmp6fd = fd;
    if (*dnsmasq_daemon).doing_ra != 0 {
        ra_start_unsolicited(now, 0 as *mut dhcp_context);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ra_start_unsolicited(mut now: time_t,
                                              mut context:
                                                  *mut dhcp_context) {
    /* init timers so that we do ra's for some/all soon. some ra_times will end up zeroed
     if it's not appropriate to advertise those contexts.
     This gets re-called on a netlink route-change to re-do the advertisement
     and pick up new interfaces */
    if !context.is_null() {
        (*context).ra_time = now; /* range 0 - 5 */
        (*context).ra_short_period_start = (*context).ra_time
    } else {
        context = (*dnsmasq_daemon).dhcp6;
        while !context.is_null() {
            if (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 10 as libc::c_int == 0 {
                (*context).ra_time =
                    now +
                        (rand16() as libc::c_int / 13000 as libc::c_int) as
                            libc::c_long;
                /* re-do frequently for a minute or so, in case the first gets lost. */
                (*context).ra_short_period_start = now
            }
            context = (*context).next
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn icmp6_packet(mut now: time_t) {
    let mut interface: [libc::c_char; 17] = [0; 17];
    let mut sz: ssize_t = 0;
    let mut if_index: libc::c_int = 0 as libc::c_int;
    let mut cmptr: *mut cmsghdr = 0 as *mut cmsghdr;
    let mut msg: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut control_u: C2RustUnnamed_11 =
        C2RustUnnamed_11{align:
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
    let mut packet: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut iname = 0 as *mut iname;
    /* Note: use outpacket for input buffer */
    msg.msg_control = control_u.control6.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen =
        ::std::mem::size_of::<C2RustUnnamed_11>() as libc::c_ulong;
    msg.msg_flags = 0 as libc::c_int;
    msg.msg_name = &mut from as *mut sockaddr_in6 as *mut libc::c_void;
    msg.msg_namelen =
        ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as socklen_t;
    msg.msg_iov = &mut (*dnsmasq_daemon).outpacket;
    msg.msg_iovlen = 1 as libc::c_int as size_t;
    sz = recv_dhcp_packet((*dnsmasq_daemon).icmp6fd, &mut msg);
    if sz == -(1 as libc::c_int) as libc::c_long ||
           sz < 8 as libc::c_int as libc::c_long {
        return
    }
    packet = (*dnsmasq_daemon).outpacket.iov_base as *mut libc::c_uchar;
    cmptr =
        if msg.msg_controllen >=
               ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
            msg.msg_control as *mut cmsghdr
        } else { 0 as *mut cmsghdr };
    while !cmptr.is_null() {
        if (*cmptr).cmsg_level == IPPROTO_IPV6 as libc::c_int &&
               (*cmptr).cmsg_type == (*dnsmasq_daemon).v6pktinfo {
            let mut p: C2RustUnnamed_10 =
                C2RustUnnamed_10{c: 0 as *mut libc::c_uchar,};
            p.c = (*cmptr).__cmsg_data.as_mut_ptr();
            if_index = (*p.p).ipi6_ifindex as libc::c_int
        }
        cmptr = __cmsg_nxthdr(&mut msg, cmptr)
    }
    if indextoname((*dnsmasq_daemon).icmp6fd, if_index,
                   interface.as_mut_ptr()) == 0 {
        return
    }
    if iface_check(1 as libc::c_int, 0 as *mut all_addr,
                   interface.as_mut_ptr(), 0 as *mut libc::c_int) == 0 {
        return
    }
    tmp = (*dnsmasq_daemon).dhcp_except;
    while !tmp.is_null() {
        if !(*tmp).name.is_null() &&
               wildcard_match((*tmp).name, interface.as_mut_ptr()) != 0 {
            return
        }
        tmp = (*tmp).next
    }
    if *packet.offset(1 as libc::c_int as isize) as libc::c_int !=
           0 as libc::c_int {
        return
    }
    if *packet.offset(0 as libc::c_int as isize) as libc::c_int ==
           129 as libc::c_int {
        lease_ping_reply(&mut from.sin6_addr, packet, interface.as_mut_ptr());
    } else if *packet.offset(0 as libc::c_int as isize) as libc::c_int ==
                  133 as libc::c_int {
        let mut mac: *mut libc::c_char =
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        let mut bridge: *mut dhcp_bridge = 0 as *mut dhcp_bridge;
        let mut alias: *mut dhcp_bridge = 0 as *mut dhcp_bridge;
        /* look for link-layer address option for logging */
        if sz >= 16 as libc::c_int as libc::c_long &&
               *packet.offset(8 as libc::c_int as isize) as libc::c_int ==
                   1 as libc::c_int &&
               (*packet.offset(9 as libc::c_int as isize) as libc::c_int *
                    8 as libc::c_int + 8 as libc::c_int) as libc::c_long <= sz
           {
            if (*packet.offset(9 as libc::c_int as isize) as libc::c_int *
                    8 as libc::c_int - 2 as libc::c_int) * 3 as libc::c_int -
                   1 as libc::c_int >= 1025 as libc::c_int {
                return
            }
            print_mac((*dnsmasq_daemon).namebuff,
                      &mut *packet.offset(10 as libc::c_int as isize),
                      *packet.offset(9 as libc::c_int as isize) as libc::c_int
                          * 8 as libc::c_int - 2 as libc::c_int);
            mac = (*dnsmasq_daemon).namebuff
        }
        if (*dnsmasq_daemon).options[(44 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (44 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          6 as libc::c_int,
                      b"RTR-SOLICIT(%s) %s\x00" as *const u8 as
                          *const libc::c_char, interface.as_mut_ptr(), mac);
        }
        /* If the incoming interface is an alias of some other one (as
         specified by the --bridge-interface option), send an RA using
         the context of the aliased interface. */
        bridge = (*dnsmasq_daemon).bridges;
        while !bridge.is_null() {
            let mut bridge_index: libc::c_int =
                if_nametoindex((*bridge).iface.as_mut_ptr()) as libc::c_int;
            if bridge_index != 0 {
                alias = (*bridge).alias;
                while !alias.is_null() {
                    if wildcard_matchn((*alias).iface.as_mut_ptr(),
                                       interface.as_mut_ptr(),
                                       16 as libc::c_int) != 0 {
                        /* Send an RA on if_index with information from
		       bridge_index. */
                        send_ra_alias(now, bridge_index,
                                      (*bridge).iface.as_mut_ptr(),
                                      0 as *mut in6_addr, if_index);
                        break ;
                    } else { alias = (*alias).next }
                }
                if !alias.is_null() { break ; }
            }
            bridge = (*bridge).next
        }
        /* If the incoming interface wasn't an alias, send an RA using
	 the context of the incoming interface. */
        if bridge.is_null() {
            /* source address may not be valid in solicit request. */
            send_ra(now, if_index, interface.as_mut_ptr(),
                    if ({
                            let mut __a: *const in6_addr =
                                &mut from.sin6_addr as *mut in6_addr as
                                    *const in6_addr;
                            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as
                                                            usize] ==
                                 0 as libc::c_int as libc::c_uint &&
                                 (*__a).__in6_u.__u6_addr32[1 as libc::c_int
                                                                as usize] ==
                                     0 as libc::c_int as libc::c_uint &&
                                 (*__a).__in6_u.__u6_addr32[2 as libc::c_int
                                                                as usize] ==
                                     0 as libc::c_int as libc::c_uint &&
                                 (*__a).__in6_u.__u6_addr32[3 as libc::c_int
                                                                as usize] ==
                                     0 as libc::c_int as libc::c_uint) as
                                libc::c_int
                        }) == 0 {
                        &mut from.sin6_addr
                    } else { 0 as *mut in6_addr });
        }
    };
}
unsafe extern "C" fn send_ra_alias(mut now: time_t, mut iface: libc::c_int,
                                   mut iface_name: *mut libc::c_char,
                                   mut dest: *mut in6_addr,
                                   mut send_iface: libc::c_int) {
    let mut ra: *mut ra_packet = 0 as *mut ra_packet;
    let mut parm: ra_param =
        ra_param{now: 0,
                 ind: 0,
                 managed: 0,
                 other: 0,
                 first: 0,
                 adv_router: 0,
                 if_name: 0 as *mut libc::c_char,
                 tags: 0 as *mut dhcp_netid,
                 link_local:
                     in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},},
                 link_global:
                     in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},},
                 ula: in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},},
                 glob_pref_time: 0,
                 link_pref_time: 0,
                 ula_pref_time: 0,
                 adv_interval: 0,
                 prio: 0,
                 found_context: 0 as *mut dhcp_context,};
    let mut addr: sockaddr_in6 =
        sockaddr_in6{sin6_family: 0,
                     sin6_port: 0,
                     sin6_flowinfo: 0,
                     sin6_addr:
                         in6_addr{__in6_u:
                                      C2RustUnnamed{__u6_addr8: [0; 16],},},
                     sin6_scope_id: 0,};
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut tmp: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut up: *mut *mut dhcp_context = 0 as *mut *mut dhcp_context;
    let mut iface_id: dhcp_netid =
        dhcp_netid{net: 0 as *mut libc::c_char, next: 0 as *mut dhcp_netid,};
    let mut opt_cfg: *mut dhcp_opt = 0 as *mut dhcp_opt;
    let mut ra_param: *mut ra_interface = find_iface_param(iface_name);
    let mut done_dns: libc::c_int = 0 as libc::c_int;
    let mut old_prefix: libc::c_int = 0 as libc::c_int;
    let mut mtu: libc::c_int = 0 as libc::c_int;
    let mut min_pref_time: libc::c_uint = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    parm.ind = iface;
    parm.managed = 0 as libc::c_int;
    parm.other = 0 as libc::c_int;
    parm.found_context = 0 as *mut dhcp_context;
    parm.adv_router = 0 as libc::c_int;
    parm.if_name = iface_name;
    parm.first = 1 as libc::c_int;
    parm.now = now;
    parm.ula_pref_time = 0 as libc::c_int as libc::c_uint;
    parm.link_pref_time = parm.ula_pref_time;
    parm.glob_pref_time = parm.link_pref_time;
    parm.adv_interval = calc_interval(ra_param);
    parm.prio = calc_prio(ra_param);
    reset_counter();
    ra =
        expand(::std::mem::size_of::<ra_packet>() as libc::c_ulong) as
            *mut ra_packet;
    if ra.is_null() { return }
    (*ra).type_0 = 134 as libc::c_int as u8_0;
    (*ra).code = 0 as libc::c_int as u8_0;
    (*ra).hop_limit = hop_limit as u8_0;
    (*ra).flags = parm.prio as u8_0;
    (*ra).lifetime = __bswap_16(calc_lifetime(ra_param) as __uint16_t);
    (*ra).reachable_time = 0 as libc::c_int as u32_0;
    (*ra).retrans_time = 0 as libc::c_int as u32_0;
    /* set tag with name == interface */
    iface_id.net = iface_name;
    iface_id.next = 0 as *mut dhcp_netid;
    parm.tags = &mut iface_id;
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        (*context).flags =
            ((*context).flags as libc::c_uint &
                 !((1 as libc::c_uint) << 5 as libc::c_int)) as libc::c_int;
        (*context).netid.next = &mut (*context).netid;
        context = (*context).next
    }
    /* If no link-local address then we can't advertise since source address of
     advertisement must be link local address: RFC 4861 para 6.1.2. */
    if iface_enumerate(10 as libc::c_int,
                       &mut parm as *mut ra_param as *mut libc::c_void,
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
                                                              libc::c_int>>(Some(add_prefixes
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
           == 0 || parm.link_pref_time == 0 as libc::c_int as libc::c_uint {
        return
    }
    /* Find smallest preferred time within address classes,
     to use as lifetime for options. This is a rather arbitrary choice. */
    min_pref_time = 0xffffffff as libc::c_uint;
    if parm.glob_pref_time != 0 as libc::c_int as libc::c_uint &&
           parm.glob_pref_time < min_pref_time {
        min_pref_time = parm.glob_pref_time
    }
    if parm.ula_pref_time != 0 as libc::c_int as libc::c_uint &&
           parm.ula_pref_time < min_pref_time {
        min_pref_time = parm.ula_pref_time
    }
    if parm.link_pref_time != 0 as libc::c_int as libc::c_uint &&
           parm.link_pref_time < min_pref_time {
        min_pref_time = parm.link_pref_time
    }
    /* Look for constructed contexts associated with addresses which have gone, 
     and advertise them with preferred_time == 0  RFC 6204 4.3 L-13 */
    up = &mut (*dnsmasq_daemon).dhcp6;
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        tmp = (*context).next;
        if (*context).if_index == iface &&
               (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 16 as libc::c_int != 0 {
            let mut old: libc::c_uint =
                difftime(now, (*context).address_lost_time) as libc::c_uint;
            if old > (*context).saved_valid {
                /* We've advertised this enough, time to go */
                /* If this context held the timeout, and there's another context in use
		 transfer the timeout there. */
                if (*context).ra_time != 0 as libc::c_int as libc::c_long &&
                       !parm.found_context.is_null() &&
                       (*parm.found_context).ra_time ==
                           0 as libc::c_int as libc::c_long {
                    new_timeout(parm.found_context, iface_name, now);
                }
                *up = (*context).next;
                free(context as *mut libc::c_void);
            } else {
                let mut opt: *mut prefix_opt = 0 as *mut prefix_opt;
                let mut local: in6_addr = (*context).start6;
                let mut do_slaac: libc::c_int = 0 as libc::c_int;
                old_prefix = 1 as libc::c_int;
                /* zero net part of address */
                setaddr6part(&mut local,
                             addr6part(&mut local) &
                                 !(if (*context).prefix == 64 as libc::c_int {
                                       -(1 as libc::c_longlong) as u64_0
                                   } else {
                                       ((1 as libc::c_ulonglong) <<
                                            128 as libc::c_int -
                                                (*context).prefix).wrapping_sub(1
                                                                                    as
                                                                                    libc::c_ulonglong)
                                   }));
                if (*context).flags as libc::c_uint &
                       (1 as libc::c_uint) << 13 as libc::c_int != 0 {
                    do_slaac = 1 as libc::c_int;
                    if (*context).flags as libc::c_uint &
                           (1 as libc::c_uint) << 8 as libc::c_int != 0 {
                        parm.other = 1 as libc::c_int;
                        if (*context).flags as libc::c_uint &
                               (1 as libc::c_uint) << 7 as libc::c_int == 0 {
                            parm.managed = 1 as libc::c_int
                        }
                    }
                } else if (*dnsmasq_daemon).options[(37 as libc::c_int as
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
                    parm.managed = 1 as libc::c_int;
                    parm.other = 1 as libc::c_int
                }
                opt =
                    expand(::std::mem::size_of::<prefix_opt>() as
                               libc::c_ulong) as *mut prefix_opt;
                if !opt.is_null() {
                    (*opt).type_0 = 3 as libc::c_int as u8_0;
                    (*opt).len = 4 as libc::c_int as u8_0;
                    (*opt).prefix_len = (*context).prefix as u8_0;
                    /* don't do RA for non-ra-only unless --enable-ra is set */
                    /* autonomous only if we're not doing dhcp, set
                     "on-link" unless "off-link" was specified */
                    (*opt).flags =
                        ((if do_slaac != 0 {
                              0x40 as libc::c_int
                          } else { 0 as libc::c_int }) |
                             (if (*context).flags as libc::c_uint &
                                     (1 as libc::c_uint) << 18 as libc::c_int
                                     != 0 {
                                  0 as libc::c_int
                              } else { 0x80 as libc::c_int })) as u8_0;
                    (*opt).valid_lifetime =
                        __bswap_32((*context).saved_valid.wrapping_sub(old));
                    (*opt).preferred_lifetime =
                        __bswap_32(0 as libc::c_int as __uint32_t);
                    (*opt).reserved = 0 as libc::c_int as u32_0;
                    (*opt).prefix = local;
                    inet_ntop(10 as libc::c_int,
                              &mut local as *mut in6_addr as
                                  *const libc::c_void,
                              (*dnsmasq_daemon).addrbuff,
                              46 as libc::c_int as socklen_t);
                    if (*dnsmasq_daemon).options[(44 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (44 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           == 0 {
                        my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                      6 as libc::c_int,
                                  b"RTR-ADVERT(%s) %s old prefix\x00" as
                                      *const u8 as *const libc::c_char,
                                  iface_name, (*dnsmasq_daemon).addrbuff);
                    }
                }
                up = &mut (*context).next
            }
        } else { up = &mut (*context).next }
        context = tmp
    }
    /* If we're advertising only old prefixes, set router lifetime to zero. */
    if old_prefix != 0 && parm.found_context.is_null() {
        (*ra).lifetime = __bswap_16(0 as libc::c_int as __uint16_t)
    }
    /* No prefixes to advertise. */
    if old_prefix == 0 && parm.found_context.is_null() { return }
    /* If we're sending router address instead of prefix in at least on prefix,
     include the advertisement interval option. */
    if parm.adv_router != 0 {
        put_opt6_char(7 as libc::c_int as libc::c_uint);
        put_opt6_char(1 as libc::c_int as libc::c_uint);
        put_opt6_short(0 as libc::c_int as libc::c_uint);
        /* interval value is in milliseconds */
        put_opt6_long((1000 as libc::c_int as
                           libc::c_uint).wrapping_mul(calc_interval(find_iface_param(iface_name))));
    }
    /* Set the MTU from ra_param if any, an MTU of 0 mean automatic for linux, */
  /* an MTU of -1 prevents the option from being sent. */
    if !ra_param.is_null() { mtu = (*ra_param).mtu }
    /* Note that IPv6 MTU is not necessarily the same as the IPv4 MTU
     available from SIOCGIFMTU */
    if mtu == 0 as libc::c_int {
        let mut mtu_name: *mut libc::c_char =
            if !ra_param.is_null() {
                (*ra_param).mtu_name
            } else { 0 as *mut libc::c_char };
        sprintf((*dnsmasq_daemon).namebuff,
                b"/proc/sys/net/ipv6/conf/%s/mtu\x00" as *const u8 as
                    *const libc::c_char,
                if !mtu_name.is_null() { mtu_name } else { iface_name });
        f =
            fopen((*dnsmasq_daemon).namebuff,
                  b"r\x00" as *const u8 as *const libc::c_char);
        if !f.is_null() {
            if !fgets((*dnsmasq_daemon).namebuff, 1025 as libc::c_int,
                      f).is_null() {
                mtu = atoi((*dnsmasq_daemon).namebuff)
            }
            fclose(f);
        }
    }
    if mtu > 0 as libc::c_int {
        put_opt6_char(5 as libc::c_int as libc::c_uint);
        put_opt6_char(1 as libc::c_int as libc::c_uint);
        put_opt6_short(0 as libc::c_int as libc::c_uint);
        put_opt6_long(mtu as libc::c_uint);
    }
    iface_enumerate(1 as libc::c_int,
                    &mut send_iface as *mut libc::c_int as *mut libc::c_void,
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
                                                           libc::c_int>>(Some(add_lla
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
    /* RDNSS, RFC 6106, use relevant DHCP6 options */
    option_filter(parm.tags, 0 as *mut dhcp_netid,
                  (*dnsmasq_daemon).dhcp_opts6);
    let mut current_block_145: u64;
    opt_cfg = (*dnsmasq_daemon).dhcp_opts6;
    while !opt_cfg.is_null() {
        let mut i: libc::c_int = 0;
        /* netids match and not encapsulated? */
        if !((*opt_cfg).flags & 4096 as libc::c_int == 0) {
            if (*opt_cfg).opt == 23 as libc::c_int {
                let mut a: *mut in6_addr = 0 as *mut in6_addr;
                let mut len: libc::c_int = 0;
                done_dns = 1 as libc::c_int;
                if (*opt_cfg).len == 0 as libc::c_int {
                    current_block_145 = 5265702136860997526;
                } else {
                    /* reduce len for any addresses we can't substitute */
                    a = (*opt_cfg).val as *mut in6_addr;
                    len = (*opt_cfg).len;
                    i = 0 as libc::c_int;
                    while i < (*opt_cfg).len {
                        if ({
                                let mut __a: *const in6_addr =
                                    a as *const in6_addr;
                                ((*__a).__in6_u.__u6_addr32[0 as libc::c_int
                                                                as usize] ==
                                     0 as libc::c_int as libc::c_uint &&
                                     (*__a).__in6_u.__u6_addr32[1 as
                                                                    libc::c_int
                                                                    as usize]
                                         == 0 as libc::c_int as libc::c_uint
                                     &&
                                     (*__a).__in6_u.__u6_addr32[2 as
                                                                    libc::c_int
                                                                    as usize]
                                         == 0 as libc::c_int as libc::c_uint
                                     &&
                                     (*__a).__in6_u.__u6_addr32[3 as
                                                                    libc::c_int
                                                                    as usize]
                                         == 0 as libc::c_int as libc::c_uint)
                                    as libc::c_int
                            }) != 0 &&
                               parm.glob_pref_time ==
                                   0 as libc::c_int as libc::c_uint ||
                               *(a as
                                     *const uint32_t).offset(0 as libc::c_int
                                                                 as isize) ==
                                   __bswap_32(0xfd000000 as libc::c_uint) &&
                                   *(a as
                                         *const uint32_t).offset(1 as
                                                                     libc::c_int
                                                                     as isize)
                                       == 0 as libc::c_int as libc::c_uint &&
                                   *(a as
                                         *const uint32_t).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                       == 0 as libc::c_int as libc::c_uint &&
                                   *(a as
                                         *const uint32_t).offset(3 as
                                                                     libc::c_int
                                                                     as isize)
                                       == 0 as libc::c_int as libc::c_uint &&
                                   parm.ula_pref_time ==
                                       0 as libc::c_int as libc::c_uint ||
                               *(a as
                                     *const uint32_t).offset(0 as libc::c_int
                                                                 as isize) ==
                                   __bswap_32(0xfe800000 as libc::c_uint) &&
                                   *(a as
                                         *const uint32_t).offset(1 as
                                                                     libc::c_int
                                                                     as isize)
                                       == 0 as libc::c_int as libc::c_uint &&
                                   *(a as
                                         *const uint32_t).offset(2 as
                                                                     libc::c_int
                                                                     as isize)
                                       == 0 as libc::c_int as libc::c_uint &&
                                   *(a as
                                         *const uint32_t).offset(3 as
                                                                     libc::c_int
                                                                     as isize)
                                       == 0 as libc::c_int as libc::c_uint &&
                                   parm.link_pref_time ==
                                       0 as libc::c_int as libc::c_uint {
                            len -= 16 as libc::c_int
                        }
                        i += 16 as libc::c_int;
                        a = a.offset(1)
                    }
                    if len != 0 as libc::c_int {
                        put_opt6_char(25 as libc::c_int as libc::c_uint);
                        put_opt6_char((len / 8 as libc::c_int +
                                           1 as libc::c_int) as libc::c_uint);
                        put_opt6_short(0 as libc::c_int as libc::c_uint);
                        put_opt6_long(min_pref_time);
                        a = (*opt_cfg).val as *mut in6_addr;
                        i = 0 as libc::c_int;
                        while i < (*opt_cfg).len {
                            if ({
                                    let mut __a: *const in6_addr =
                                        a as *const in6_addr;
                                    ((*__a).__in6_u.__u6_addr32[0 as
                                                                    libc::c_int
                                                                    as usize]
                                         == 0 as libc::c_int as libc::c_uint
                                         &&
                                         (*__a).__in6_u.__u6_addr32[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                             ==
                                             0 as libc::c_int as libc::c_uint
                                         &&
                                         (*__a).__in6_u.__u6_addr32[2 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                             ==
                                             0 as libc::c_int as libc::c_uint
                                         &&
                                         (*__a).__in6_u.__u6_addr32[3 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                             ==
                                             0 as libc::c_int as libc::c_uint)
                                        as libc::c_int
                                }) != 0 {
                                if parm.glob_pref_time !=
                                       0 as libc::c_int as libc::c_uint {
                                    put_opt6(&mut parm.link_global as
                                                 *mut in6_addr as
                                                 *mut libc::c_void,
                                             16 as libc::c_int as size_t);
                                }
                            } else if *(a as
                                            *const uint32_t).offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                          ==
                                          __bswap_32(0xfd000000 as
                                                         libc::c_uint) &&
                                          *(a as
                                                *const uint32_t).offset(1 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                              ==
                                              0 as libc::c_int as libc::c_uint
                                          &&
                                          *(a as
                                                *const uint32_t).offset(2 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                              ==
                                              0 as libc::c_int as libc::c_uint
                                          &&
                                          *(a as
                                                *const uint32_t).offset(3 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                              ==
                                              0 as libc::c_int as libc::c_uint
                             {
                                if parm.ula_pref_time !=
                                       0 as libc::c_int as libc::c_uint {
                                    put_opt6(&mut parm.ula as *mut in6_addr as
                                                 *mut libc::c_void,
                                             16 as libc::c_int as size_t);
                                }
                            } else if *(a as
                                            *const uint32_t).offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
                                          ==
                                          __bswap_32(0xfe800000 as
                                                         libc::c_uint) &&
                                          *(a as
                                                *const uint32_t).offset(1 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                              ==
                                              0 as libc::c_int as libc::c_uint
                                          &&
                                          *(a as
                                                *const uint32_t).offset(2 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                              ==
                                              0 as libc::c_int as libc::c_uint
                                          &&
                                          *(a as
                                                *const uint32_t).offset(3 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                              ==
                                              0 as libc::c_int as libc::c_uint
                             {
                                if parm.link_pref_time !=
                                       0 as libc::c_int as libc::c_uint {
                                    put_opt6(&mut parm.link_local as
                                                 *mut in6_addr as
                                                 *mut libc::c_void,
                                             16 as libc::c_int as size_t);
                                }
                            } else {
                                put_opt6(a as *mut libc::c_void,
                                         16 as libc::c_int as size_t);
                            }
                            i += 16 as libc::c_int;
                            a = a.offset(1)
                        }
                    }
                    current_block_145 = 11235674318412060590;
                }
            } else { current_block_145 = 11235674318412060590; }
            match current_block_145 {
                5265702136860997526 => { }
                _ => {
                    if (*opt_cfg).opt == 24 as libc::c_int &&
                           (*opt_cfg).len != 0 as libc::c_int {
                        let mut len_0: libc::c_int =
                            ((*opt_cfg).len + 7 as libc::c_int) /
                                8 as libc::c_int;
                        put_opt6_char(31 as libc::c_int as libc::c_uint);
                        put_opt6_char((len_0 + 1 as libc::c_int) as
                                          libc::c_uint);
                        put_opt6_short(0 as libc::c_int as libc::c_uint);
                        put_opt6_long(min_pref_time);
                        put_opt6((*opt_cfg).val as *mut libc::c_void,
                                 (*opt_cfg).len as size_t);
                        /* pad */
                        i = (*opt_cfg).len;
                        while i < len_0 * 8 as libc::c_int {
                            put_opt6_char(0 as libc::c_int as libc::c_uint);
                            i += 1
                        }
                    }
                }
            }
        }
        opt_cfg = (*opt_cfg).next
    }
    if (*dnsmasq_daemon).port == 53 as libc::c_int && done_dns == 0 &&
           parm.link_pref_time != 0 as libc::c_int as libc::c_uint {
        /* default == us, as long as we are supplying DNS service. */
        put_opt6_char(25 as libc::c_int as libc::c_uint);
        put_opt6_char(3 as libc::c_int as libc::c_uint);
        put_opt6_short(0 as libc::c_int as libc::c_uint);
        put_opt6_long(min_pref_time);
        put_opt6(&mut parm.link_local as *mut in6_addr as *mut libc::c_void,
                 16 as libc::c_int as size_t);
    }
    /* set managed bits unless we're providing only RA on this link */
    if parm.managed != 0 {
        (*ra).flags =
            ((*ra).flags as libc::c_int | 0x80 as libc::c_int) as u8_0
    } /* M flag, managed, */
    if parm.other != 0 {
        (*ra).flags =
            ((*ra).flags as libc::c_int | 0x40 as libc::c_int) as u8_0
    } /* O flag, other */
    /* decide where we're sending */
    memset(&mut addr as *mut sockaddr_in6 as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong);
    addr.sin6_family = 10 as libc::c_int as sa_family_t;
    addr.sin6_port = __bswap_16(IPPROTO_ICMPV6 as libc::c_int as __uint16_t);
    if !dest.is_null() {
        addr.sin6_addr = *dest;
        if ({
                let mut __a: *const in6_addr = dest as *const in6_addr;
                ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                     __bswap_32(0xffc00000 as libc::c_uint) ==
                     __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
            }) != 0 ||
               *(dest as *const uint8_t).offset(0 as libc::c_int as isize) as
                   libc::c_int == 0xff as libc::c_int &&
                   *(dest as *const uint8_t).offset(1 as libc::c_int as isize)
                       as libc::c_int & 0xf as libc::c_int ==
                       0x2 as libc::c_int {
            addr.sin6_scope_id = iface as uint32_t
        }
    } else {
        inet_pton(10 as libc::c_int,
                  b"FF02::1\x00" as *const u8 as *const libc::c_char,
                  &mut addr.sin6_addr as *mut in6_addr as *mut libc::c_void);
        setsockopt((*dnsmasq_daemon).icmp6fd, IPPROTO_IPV6 as libc::c_int,
                   17 as libc::c_int,
                   &mut send_iface as *mut libc::c_int as *const libc::c_void,
                   ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                       socklen_t);
    }
    while retry_send(sendto((*dnsmasq_daemon).icmp6fd,
                            (*dnsmasq_daemon).outpacket.iov_base,
                            save_counter(-(1 as libc::c_int)) as size_t,
                            0 as libc::c_int,
                            __CONST_SOCKADDR_ARG{__sockaddr__:
                                                     &mut addr as
                                                         *mut sockaddr_in6 as
                                                         *mut sockaddr,},
                            ::std::mem::size_of::<sockaddr_in6>() as
                                libc::c_ulong as socklen_t)) != 0 {
    };
}
unsafe extern "C" fn send_ra(mut now: time_t, mut iface: libc::c_int,
                             mut iface_name: *mut libc::c_char,
                             mut dest: *mut in6_addr) {
    /* Send an RA on the same interface that the RA content is based
     on. */
    send_ra_alias(now, iface, iface_name, dest, iface);
}
unsafe extern "C" fn add_prefixes(mut local: *mut in6_addr,
                                  mut prefix: libc::c_int,
                                  mut scope: libc::c_int,
                                  mut if_index: libc::c_int,
                                  mut flags: libc::c_int,
                                  mut preferred: libc::c_uint,
                                  mut valid: libc::c_uint,
                                  mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut param: *mut ra_param = vparam as *mut ra_param;
    /* warning */
    if if_index == (*param).ind {
        if ({
                let mut __a: *const in6_addr = local as *const in6_addr;
                ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                     __bswap_32(0xffc00000 as libc::c_uint) ==
                     __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
            }) != 0 {
            /* Can there be more than one LL address?
	     Select the one with the longest preferred time 
	     if there is. */
            if preferred > (*param).link_pref_time {
                (*param).link_pref_time = preferred;
                (*param).link_local = *local
            }
        } else if ({
                       let mut __a: *const in6_addr =
                           local as *const in6_addr;
                       ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                            == 0 as libc::c_int as libc::c_uint &&
                            (*__a).__in6_u.__u6_addr32[1 as libc::c_int as
                                                           usize] ==
                                0 as libc::c_int as libc::c_uint &&
                            (*__a).__in6_u.__u6_addr32[2 as libc::c_int as
                                                           usize] ==
                                0 as libc::c_int as libc::c_uint &&
                            (*__a).__in6_u.__u6_addr32[3 as libc::c_int as
                                                           usize] ==
                                __bswap_32(1 as libc::c_int as __uint32_t)) as
                           libc::c_int
                   }) == 0 &&
                      !(*(local as
                              *const uint8_t).offset(0 as libc::c_int as
                                                         isize) as libc::c_int
                            == 0xff as libc::c_int) {
            let mut real_prefix: libc::c_int = 0 as libc::c_int;
            let mut do_slaac: libc::c_int = 0 as libc::c_int;
            let mut deprecate: libc::c_int = 0 as libc::c_int;
            let mut constructed: libc::c_int = 0 as libc::c_int;
            let mut adv_router: libc::c_int = 0 as libc::c_int;
            let mut off_link: libc::c_int = 0 as libc::c_int;
            let mut time: libc::c_uint = 0xffffffff as libc::c_uint;
            let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
            let mut current_block_43: u64;
            context = (*dnsmasq_daemon).dhcp6;
            while !context.is_null() {
                if (*context).flags as libc::c_uint &
                       ((1 as libc::c_uint) << 10 as libc::c_int |
                            (1 as libc::c_uint) << 16 as libc::c_int) == 0 &&
                       prefix <= (*context).prefix &&
                       is_same_net6(local, &mut (*context).start6,
                                    (*context).prefix) != 0 &&
                       is_same_net6(local, &mut (*context).end6,
                                    (*context).prefix) != 0 {
                    (*context).saved_valid = valid;
                    if (*context).flags as libc::c_uint &
                           (1 as libc::c_uint) << 13 as libc::c_int != 0 {
                        do_slaac = 1 as libc::c_int;
                        if (*context).flags as libc::c_uint &
                               (1 as libc::c_uint) << 8 as libc::c_int != 0 {
                            (*param).other = 1 as libc::c_int;
                            if (*context).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 7 as libc::c_int ==
                                   0 {
                                (*param).managed = 1 as libc::c_int
                            }
                        }
                        current_block_43 = 7056779235015430508;
                    } else if (*dnsmasq_daemon).options[(37 as libc::c_int as
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
                                  == 0 {
                        current_block_43 = 10599921512955367680;
                    } else {
                        (*param).managed = 1 as libc::c_int;
                        (*param).other = 1 as libc::c_int;
                        current_block_43 = 7056779235015430508;
                    }
                    match current_block_43 {
                        10599921512955367680 => { }
                        _ => {
                            /* don't do RA for non-ra-only unless --enable-ra is set */
                            /* Configured to advertise router address, not prefix. See RFC 3775 7.2 
		 In this case we do all addresses associated with a context, 
		 hence the real_prefix setting here. */
                            if (*context).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 4 as libc::c_int !=
                                   0 {
                                adv_router = 1 as libc::c_int;
                                (*param).adv_router = 1 as libc::c_int;
                                real_prefix = (*context).prefix
                            }
                            /* find floor time, don't reduce below 3 * RA interval.
		   If the lease time has been left as default, don't
		   use that as a floor. */
                            if (*context).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 19 as libc::c_int !=
                                   0 && time > (*context).lease_time {
                                time = (*context).lease_time;
                                if time <
                                       (3 as libc::c_int as
                                            libc::c_uint).wrapping_mul((*param).adv_interval)
                                   {
                                    time =
                                        (3 as libc::c_int as
                                             libc::c_uint).wrapping_mul((*param).adv_interval)
                                }
                            }
                            if (*context).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 9 as libc::c_int !=
                                   0 {
                                deprecate = 1 as libc::c_int
                            }
                            if (*context).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 11 as libc::c_int !=
                                   0 {
                                constructed = 1 as libc::c_int
                            }
                            /* collect dhcp-range tags */
                            if (*context).netid.next ==
                                   &mut (*context).netid as *mut dhcp_netid &&
                                   !(*context).netid.net.is_null() {
                                (*context).netid.next = (*param).tags;
                                (*param).tags = &mut (*context).netid
                            }
                            /* subsequent prefixes on the same interface 
		   and subsequent instances of this prefix don't need timers.
		   Be careful not to find the same prefix twice with different
		   addresses unless we're advertising the actual addresses. */
                            if (*context).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 5 as libc::c_int ==
                                   0 {
                                if (*param).first == 0 {
                                    (*context).ra_time =
                                        0 as libc::c_int as time_t
                                }
                                (*context).flags =
                                    ((*context).flags as libc::c_uint |
                                         (1 as libc::c_uint) <<
                                             5 as libc::c_int) as libc::c_int;
                                real_prefix = (*context).prefix;
                                off_link =
                                    ((*context).flags as libc::c_uint &
                                         (1 as libc::c_uint) <<
                                             18 as libc::c_int) as libc::c_int
                            }
                            (*param).first = 0 as libc::c_int;
                            /* found_context is the _last_ one we found, so if there's 
		   more than one, it's not the first. */
                            (*param).found_context = context
                        }
                    }
                }
                context = (*context).next
            }
            /* configured time is ceiling */
            if constructed == 0 || valid > time { valid = time }
            if flags & 2 as libc::c_int != 0 {
                preferred = 0 as libc::c_int as libc::c_uint
            }
            if deprecate != 0 { time = 0 as libc::c_int as libc::c_uint }
            /* configured time is ceiling */
            if constructed == 0 || preferred > time { preferred = time }
            if *(local as *const uint32_t).offset(0 as libc::c_int as isize) &
                   __bswap_32(0xff000000 as libc::c_uint) ==
                   __bswap_32(0xfd000000 as libc::c_uint) {
                if preferred > (*param).ula_pref_time {
                    (*param).ula_pref_time = preferred;
                    (*param).ula = *local
                }
            } else if preferred > (*param).glob_pref_time {
                (*param).glob_pref_time = preferred;
                (*param).link_global = *local
            }
            if real_prefix != 0 as libc::c_int {
                let mut opt: *mut prefix_opt = 0 as *mut prefix_opt;
                opt =
                    expand(::std::mem::size_of::<prefix_opt>() as
                               libc::c_ulong) as *mut prefix_opt;
                if !opt.is_null() {
                    /* zero net part of address */
                    if adv_router == 0 {
                        setaddr6part(local,
                                     addr6part(local) &
                                         !(if real_prefix == 64 as libc::c_int
                                              {
                                               -(1 as libc::c_longlong) as
                                                   u64_0
                                           } else {
                                               ((1 as libc::c_ulonglong) <<
                                                    128 as libc::c_int -
                                                        real_prefix).wrapping_sub(1
                                                                                      as
                                                                                      libc::c_ulonglong)
                                           }));
                    }
                    (*opt).type_0 = 3 as libc::c_int as u8_0;
                    (*opt).len = 4 as libc::c_int as u8_0;
                    (*opt).prefix_len = real_prefix as u8_0;
                    /* autonomous only if we're not doing dhcp, set
                     "on-link" unless "off-link" was specified */
                    (*opt).flags =
                        if off_link != 0 {
                            0 as libc::c_int
                        } else { 0x80 as libc::c_int } as u8_0;
                    if do_slaac != 0 {
                        (*opt).flags =
                            ((*opt).flags as libc::c_int |
                                 0x40 as libc::c_int) as u8_0
                    }
                    if adv_router != 0 {
                        (*opt).flags =
                            ((*opt).flags as libc::c_int |
                                 0x20 as libc::c_int) as u8_0
                    }
                    (*opt).valid_lifetime = __bswap_32(valid);
                    (*opt).preferred_lifetime = __bswap_32(preferred);
                    (*opt).reserved = 0 as libc::c_int as u32_0;
                    (*opt).prefix = *local;
                    inet_ntop(10 as libc::c_int, local as *const libc::c_void,
                              (*dnsmasq_daemon).addrbuff,
                              46 as libc::c_int as socklen_t);
                    if (*dnsmasq_daemon).options[(44 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (44 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           == 0 {
                        my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                      6 as libc::c_int,
                                  b"RTR-ADVERT(%s) %s\x00" as *const u8 as
                                      *const libc::c_char, (*param).if_name,
                                  (*dnsmasq_daemon).addrbuff);
                    }
                }
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn add_lla(mut index: libc::c_int, mut type_0: libc::c_uint,
                             mut mac: *mut libc::c_char, mut maclen: size_t,
                             mut parm: *mut libc::c_void) -> libc::c_int {
    if index == *(parm as *mut libc::c_int) {
        /* size is in units of 8 octets and includes type and length (2 bytes)
	 add 7 to round up */
        let mut len: libc::c_int =
            (maclen.wrapping_add(9 as libc::c_int as libc::c_ulong) >>
                 3 as libc::c_int) as libc::c_int;
        let mut p: *mut libc::c_uchar =
            expand((len << 3 as libc::c_int) as size_t) as *mut libc::c_uchar;
        memset(p as *mut libc::c_void, 0 as libc::c_int,
               (len << 3 as libc::c_int) as libc::c_ulong);
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = 1 as libc::c_int as libc::c_uchar;
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = len as libc::c_uchar;
        memcpy(p as *mut libc::c_void, mac as *const libc::c_void, maclen);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn periodic_ra(mut now: time_t) -> time_t {
    let mut param: search_param =
        search_param{now: 0, iface: 0, name: [0; 17],};
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut next_event: time_t = 0;
    let mut aparam: alias_param =
        alias_param{iface: 0,
                    bridge: 0 as *mut dhcp_bridge,
                    num_alias_ifs: 0,
                    max_alias_ifs: 0,
                    alias_ifs: 0 as *mut libc::c_int,};
    param.now = now;
    param.iface = 0 as libc::c_int;
    loop  {
        /* find overdue events, and time of first future event */
        next_event = 0 as libc::c_int as time_t; /* overdue */
        context = (*dnsmasq_daemon).dhcp6;
        while !context.is_null() {
            if (*context).ra_time != 0 as libc::c_int as libc::c_long {
                if difftime((*context).ra_time, now) <= 0.0f64 { break ; }
                if next_event == 0 as libc::c_int as libc::c_long ||
                       difftime(next_event, (*context).ra_time) > 0.0f64 {
                    next_event = (*context).ra_time
                }
            }
            context = (*context).next
        }
        /* none overdue */
        if context.is_null() { break ; }
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 16 as libc::c_int != 0 &&
               (*context).if_index != 0 as libc::c_int &&
               indextoname((*dnsmasq_daemon).icmp6fd, (*context).if_index,
                           param.name.as_mut_ptr()) != 0 {
            /* A context for an old address. We'll not find the interface by 
	     looking for addresses, but we know it anyway, since the context is
	     constructed */
            param.iface = (*context).if_index;
            new_timeout(context, param.name.as_mut_ptr(), now);
        } else if iface_enumerate(10 as libc::c_int,
                                  &mut param as *mut search_param as
                                      *mut libc::c_void,
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
                                                                     ->
                                                                         libc::c_int>,
                                                          Option<unsafe extern "C" fn()
                                                                     ->
                                                                         libc::c_int>>(Some(iface_search
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
                                                                                                        libc::c_int)))
                      != 0 {
            /* There's a context overdue, but we can't find an interface
	   associated with it, because it's for a subnet we dont 
	   have an interface on. Probably we're doing DHCP on
	   a remote subnet via a relay. Zero the timer, since we won't
	   ever be able to send ra's and satisfy it. */
            (*context).ra_time = 0 as libc::c_int as time_t
        }
        if param.iface != 0 as libc::c_int &&
               iface_check(1 as libc::c_int, 0 as *mut all_addr,
                           param.name.as_mut_ptr(), 0 as *mut libc::c_int) !=
                   0 {
            let mut tmp: *mut iname = 0 as *mut iname;
            tmp = (*dnsmasq_daemon).dhcp_except;
            while !tmp.is_null() {
                if !(*tmp).name.is_null() &&
                       wildcard_match((*tmp).name, param.name.as_mut_ptr()) !=
                           0 {
                    break ;
                }
                tmp = (*tmp).next
            }
            if tmp.is_null() {
                send_ra(now, param.iface, param.name.as_mut_ptr(),
                        0 as *mut in6_addr);
                /* Also send on all interfaces that are aliases of this
                 one. */
                aparam.bridge = (*dnsmasq_daemon).bridges;
                while !aparam.bridge.is_null() {
                    if if_nametoindex((*aparam.bridge).iface.as_mut_ptr()) as
                           libc::c_int == param.iface {
                        /* Count the number of alias interfaces for this
                       'bridge', by calling iface_enumerate with
                       send_ra_to_aliases and NULL alias_ifs. */
                        aparam.iface = param.iface;
                        aparam.alias_ifs = 0 as *mut libc::c_int;
                        aparam.num_alias_ifs = 0 as libc::c_int;
                        iface_enumerate(1 as libc::c_int,
                                        &mut aparam as *mut alias_param as
                                            *mut libc::c_void,
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
                                                                           ->
                                                                               libc::c_int>,
                                                                Option<unsafe extern "C" fn()
                                                                           ->
                                                                               libc::c_int>>(Some(send_ra_to_aliases
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
                        my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                      6 as libc::c_int,
                                  b"RTR-ADVERT(%s) %s => %d alias(es)\x00" as
                                      *const u8 as *const libc::c_char,
                                  param.name.as_mut_ptr(),
                                  (*dnsmasq_daemon).addrbuff,
                                  aparam.num_alias_ifs);
                        /* Allocate memory to store the alias interface
                       indices. */
                        aparam.alias_ifs =
                            whine_malloc((aparam.num_alias_ifs as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                              as
                                                                              libc::c_ulong))
                                as *mut libc::c_int;
                        if !aparam.alias_ifs.is_null() {
                            /* Use iface_enumerate again to get the alias
                           interface indices, then send on each of
                           those. */
                            aparam.max_alias_ifs = aparam.num_alias_ifs;
                            aparam.num_alias_ifs = 0 as libc::c_int;
                            iface_enumerate(1 as libc::c_int,
                                            &mut aparam as *mut alias_param as
                                                *mut libc::c_void,
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
                                                                               ->
                                                                                   libc::c_int>,
                                                                    Option<unsafe extern "C" fn()
                                                                               ->
                                                                                   libc::c_int>>(Some(send_ra_to_aliases
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
                            while aparam.num_alias_ifs != 0 {
                                my_syslog((3 as libc::c_int) <<
                                              3 as libc::c_int |
                                              6 as libc::c_int,
                                          b"RTR-ADVERT(%s) %s => i/f %d\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          param.name.as_mut_ptr(),
                                          (*dnsmasq_daemon).addrbuff,
                                          *aparam.alias_ifs.offset((aparam.num_alias_ifs
                                                                        -
                                                                        1 as
                                                                            libc::c_int)
                                                                       as
                                                                       isize));
                                send_ra_alias(now, param.iface,
                                              param.name.as_mut_ptr(),
                                              0 as *mut in6_addr,
                                              *aparam.alias_ifs.offset((aparam.num_alias_ifs
                                                                            -
                                                                            1
                                                                                as
                                                                                libc::c_int)
                                                                           as
                                                                           isize));
                                aparam.num_alias_ifs -= 1
                            }
                            free(aparam.alias_ifs as *mut libc::c_void);
                        }
                        break ;
                    } else { aparam.bridge = (*aparam.bridge).next }
                }
            }
        }
    }
    return next_event;
}
unsafe extern "C" fn send_ra_to_aliases(mut index: libc::c_int,
                                        mut type_0: libc::c_uint,
                                        mut mac: *mut libc::c_char,
                                        mut maclen: size_t,
                                        mut parm: *mut libc::c_void)
 -> libc::c_int {
    let mut aparam: *mut alias_param = parm as *mut alias_param;
    let mut ifrn_name: [libc::c_char; 16] = [0; 16];
    let mut alias: *mut dhcp_bridge = 0 as *mut dhcp_bridge;
    if !if_indextoname(index as libc::c_uint,
                       ifrn_name.as_mut_ptr()).is_null() {
        alias = (*(*aparam).bridge).alias;
        while !alias.is_null() {
            if wildcard_matchn((*alias).iface.as_mut_ptr(),
                               ifrn_name.as_mut_ptr(), 16 as libc::c_int) != 0
               {
                if !(*aparam).alias_ifs.is_null() &&
                       (*aparam).num_alias_ifs < (*aparam).max_alias_ifs {
                    *(*aparam).alias_ifs.offset((*aparam).num_alias_ifs as
                                                    isize) = index
                }
                (*aparam).num_alias_ifs += 1
            }
            alias = (*alias).next
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn iface_search(mut local: *mut in6_addr,
                                  mut prefix: libc::c_int,
                                  mut scope: libc::c_int,
                                  mut if_index: libc::c_int,
                                  mut flags: libc::c_int,
                                  mut preferred: libc::c_int,
                                  mut valid: libc::c_int,
                                  mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut param: *mut search_param = vparam as *mut search_param;
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut tmp: *mut iname = 0 as *mut iname;
    /* ignore interfaces we're not doing DHCP on. */
    if indextoname((*dnsmasq_daemon).icmp6fd, if_index,
                   (*param).name.as_mut_ptr()) == 0 ||
           iface_check(1 as libc::c_int, 0 as *mut all_addr,
                       (*param).name.as_mut_ptr(), 0 as *mut libc::c_int) == 0
       {
        return 1 as libc::c_int
    }
    tmp = (*dnsmasq_daemon).dhcp_except;
    while !tmp.is_null() {
        if !(*tmp).name.is_null() &&
               wildcard_match((*tmp).name, (*param).name.as_mut_ptr()) != 0 {
            return 1 as libc::c_int
        }
        tmp = (*tmp).next
    }
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        if (*context).flags as libc::c_uint &
               ((1 as libc::c_uint) << 10 as libc::c_int |
                    (1 as libc::c_uint) << 16 as libc::c_int) == 0 &&
               prefix <= (*context).prefix &&
               is_same_net6(local, &mut (*context).start6, (*context).prefix)
                   != 0 &&
               is_same_net6(local, &mut (*context).end6, (*context).prefix) !=
                   0 && (*context).ra_time != 0 as libc::c_int as libc::c_long
               && difftime((*context).ra_time, (*param).now) <= 0.0f64 {
            /* found an interface that's overdue for RA determine new 
	   timeout value and arrange for RA to be sent unless interface is
	   still doing DAD.*/
            if flags & 1 as libc::c_int == 0 { (*param).iface = if_index }
            new_timeout(context, (*param).name.as_mut_ptr(), (*param).now);
            /* found, abort */
            context = (*context).next;
            while !context.is_null() {
                if prefix <= (*context).prefix &&
                       is_same_net6(local, &mut (*context).start6,
                                    (*context).prefix) != 0 &&
                       is_same_net6(local, &mut (*context).end6,
                                    (*context).prefix) != 0 {
                    (*context).ra_time = 0 as libc::c_int as time_t
                }
                context = (*context).next
            }
            return 0 as libc::c_int
        }
        context = (*context).next
    }
    return 1 as libc::c_int;
    /* zero timers for other contexts on the same subnet, so they don't timeout 
	   independently */
    /* keep searching */
}
unsafe extern "C" fn new_timeout(mut context: *mut dhcp_context,
                                 mut iface_name: *mut libc::c_char,
                                 mut now: time_t) {
    if difftime(now, (*context).ra_short_period_start) < 60.0f64 {
        /* range 5 - 20 */
        (*context).ra_time =
            now + 5 as libc::c_int as libc::c_long +
                (rand16() as libc::c_int / 4400 as libc::c_int) as
                    libc::c_long
    } else {
        /* range 3/4 - 1 times MaxRtrAdvInterval */
        let mut adv_interval: libc::c_uint =
            calc_interval(find_iface_param(iface_name));
        (*context).ra_time =
            now +
                (3 as libc::c_int as
                     libc::c_uint).wrapping_mul(adv_interval).wrapping_div(4
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
                    as libc::c_long +
                (adv_interval.wrapping_mul(rand16() as libc::c_uint) >>
                     18 as libc::c_int) as libc::c_long
    };
}
unsafe extern "C" fn find_iface_param(mut iface: *mut libc::c_char)
 -> *mut ra_interface {
    let mut ra: *mut ra_interface = 0 as *mut ra_interface;
    ra = (*dnsmasq_daemon).ra_interfaces;
    while !ra.is_null() {
        if wildcard_match((*ra).name, iface) != 0 { return ra }
        ra = (*ra).next
    }
    return 0 as *mut ra_interface;
}
unsafe extern "C" fn calc_interval(mut ra: *mut ra_interface)
 -> libc::c_uint {
    let mut interval: libc::c_int = 600 as libc::c_int;
    if !ra.is_null() && (*ra).interval != 0 as libc::c_int {
        interval = (*ra).interval;
        if interval > 1800 as libc::c_int {
            interval = 1800 as libc::c_int
        } else if interval < 4 as libc::c_int { interval = 4 as libc::c_int }
    }
    return interval as libc::c_uint;
}
unsafe extern "C" fn calc_lifetime(mut ra: *mut ra_interface)
 -> libc::c_uint {
    let mut lifetime: libc::c_int = 0;
    let mut interval: libc::c_int = calc_interval(ra) as libc::c_int;
    if ra.is_null() || (*ra).lifetime == -(1 as libc::c_int) {
        /* not specified */
        lifetime = 3 as libc::c_int * interval
    } else {
        lifetime = (*ra).lifetime;
        if lifetime < interval && lifetime != 0 as libc::c_int {
            lifetime = interval
        } else if lifetime > 9000 as libc::c_int {
            lifetime = 9000 as libc::c_int
        }
    }
    return lifetime as libc::c_uint;
}
unsafe extern "C" fn calc_prio(mut ra: *mut ra_interface) -> libc::c_uint {
    if !ra.is_null() { return (*ra).prio as libc::c_uint }
    return 0 as libc::c_int as libc::c_uint;
}
