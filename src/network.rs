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
    fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t)
     -> libc::c_int;
    #[no_mangle]
    fn getsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *mut libc::c_void,
                  __optlen: *mut socklen_t) -> libc::c_int;
    #[no_mangle]
    fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> libc::c_int;
    #[no_mangle]
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static in6addr_any: in6_addr;
    #[no_mangle]
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
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
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
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
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
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
    fn private_net(addr: in_addr, ban_localhost: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn rand16() -> libc::c_ushort;
    #[no_mangle]
    fn rand32() -> u32_0;
    #[no_mangle]
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn safe_strncpy(dest: *mut libc::c_char, src: *const libc::c_char,
                    size: size_t);
    #[no_mangle]
    fn whine_malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn sa_len(addr: *mut mysockaddr) -> libc::c_int;
    #[no_mangle]
    fn sockaddr_isequal(s1: *mut mysockaddr, s2: *mut mysockaddr)
     -> libc::c_int;
    #[no_mangle]
    fn hostname_isequal(a: *const libc::c_char, b: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn prettyprint_addr(addr: *mut mysockaddr, buf: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn wildcard_match(wildcard: *const libc::c_char,
                      match_0: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn die(message: *mut libc::c_char, arg1: *mut libc::c_char,
           exit_code: libc::c_int) -> !;
    #[no_mangle]
    fn my_syslog(priority: libc::c_int, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn server_gone(server: *mut server);
    #[no_mangle]
    fn loop_send_probes();
    #[no_mangle]
    fn iface_enumerate(family: libc::c_int, parm: *mut libc::c_void,
                       callback:
                           Option<unsafe extern "C" fn() -> libc::c_int>)
     -> libc::c_int;
    #[no_mangle]
    fn lease_find_interfaces(now: time_t);
    #[no_mangle]
    fn dhcp_construct_contexts(now: time_t);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_pktinfo {
    pub ipi_ifindex: libc::c_int,
    pub ipi_spec_dst: in_addr,
    pub ipi_addr: in_addr,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_pktinfo {
    pub ipi6_addr: in6_addr,
    pub ipi6_ifindex: libc::c_uint,
}
pub type u32_0 = libc::c_uint;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IFF_DYNAMIC: C2RustUnnamed_1 = 32768;
pub const IFF_AUTOMEDIA: C2RustUnnamed_1 = 16384;
pub const IFF_PORTSEL: C2RustUnnamed_1 = 8192;
pub const IFF_MULTICAST: C2RustUnnamed_1 = 4096;
pub const IFF_SLAVE: C2RustUnnamed_1 = 2048;
pub const IFF_MASTER: C2RustUnnamed_1 = 1024;
pub const IFF_ALLMULTI: C2RustUnnamed_1 = 512;
pub const IFF_PROMISC: C2RustUnnamed_1 = 256;
pub const IFF_NOARP: C2RustUnnamed_1 = 128;
pub const IFF_RUNNING: C2RustUnnamed_1 = 64;
pub const IFF_NOTRAILERS: C2RustUnnamed_1 = 32;
pub const IFF_POINTOPOINT: C2RustUnnamed_1 = 16;
pub const IFF_LOOPBACK: C2RustUnnamed_1 = 8;
pub const IFF_DEBUG: C2RustUnnamed_1 = 4;
pub const IFF_BROADCAST: C2RustUnnamed_1 = 2;
pub const IFF_UP: C2RustUnnamed_1 = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iface_param {
    pub spare: *mut addrlist,
    pub fd: libc::c_int,
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
    pub c: *mut libc::c_uchar,
    pub p: *mut in_pktinfo,
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
#[no_mangle]
pub unsafe extern "C" fn indextoname(mut fd: libc::c_int,
                                     mut index: libc::c_int,
                                     mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut ifr: ifreq =
        ifreq{ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_2{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    if index == 0 as libc::c_int { return 0 as libc::c_int }
    ifr.ifr_ifru.ifru_ivalue = index;
    if ioctl(fd, 0x8910 as libc::c_int as libc::c_ulong,
             &mut ifr as *mut ifreq) == -(1 as libc::c_int) {
        return 0 as libc::c_int
    }
    safe_strncpy(name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                 16 as libc::c_int as size_t);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn iface_check(mut family: libc::c_int,
                                     mut addr: *mut all_addr,
                                     mut name: *mut libc::c_char,
                                     mut auth: *mut libc::c_int)
 -> libc::c_int {
    let mut tmp: *mut iname = 0 as *mut iname;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut match_addr: libc::c_int = 0 as libc::c_int;
    /* Note: have to check all and not bail out early, so that we set the
     "used" flags.

     May be called with family == AF_LOCALto check interface by name only. */
    if !auth.is_null() { *auth = 0 as libc::c_int }
    if !(*dnsmasq_daemon).if_names.is_null() ||
           !(*dnsmasq_daemon).if_addrs.is_null() {
        ret = 0 as libc::c_int;
        tmp = (*dnsmasq_daemon).if_names;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name, name) != 0 {
                (*tmp).used = 1 as libc::c_int;
                ret = (*tmp).used
            }
            tmp = (*tmp).next
        }
        if !addr.is_null() {
            tmp = (*dnsmasq_daemon).if_addrs;
            while !tmp.is_null() {
                if (*tmp).addr.sa.sa_family as libc::c_int == family {
                    if family == 2 as libc::c_int &&
                           (*tmp).addr.in_0.sin_addr.s_addr ==
                               (*addr).addr4.s_addr {
                        (*tmp).used = 1 as libc::c_int;
                        match_addr = (*tmp).used;
                        ret = match_addr
                    } else if family == 10 as libc::c_int &&
                                  ({
                                       let mut __a: *const in6_addr =
                                           &mut (*tmp).addr.in6.sin6_addr as
                                               *mut in6_addr as
                                               *const in6_addr;
                                       let mut __b: *const in6_addr =
                                           &mut (*addr).addr6 as *mut in6_addr
                                               as *const in6_addr;
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
                                   }) != 0 {
                        (*tmp).used = 1 as libc::c_int;
                        match_addr = (*tmp).used;
                        ret = match_addr
                    }
                }
                tmp = (*tmp).next
            }
        }
    }
    if match_addr == 0 {
        tmp = (*dnsmasq_daemon).if_except;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name, name) != 0 {
                ret = 0 as libc::c_int
            }
            tmp = (*tmp).next
        }
    }
    tmp = (*dnsmasq_daemon).authinterface;
    while !tmp.is_null() {
        if !(*tmp).name.is_null() {
            if strcmp((*tmp).name, name) == 0 as libc::c_int &&
                   ((*tmp).addr.sa.sa_family as libc::c_int ==
                        0 as libc::c_int ||
                        (*tmp).addr.sa.sa_family as libc::c_int == family) {
                break ;
            }
        } else {
            if !addr.is_null() &&
                   (*tmp).addr.sa.sa_family as libc::c_int == 2 as libc::c_int
                   && family == 2 as libc::c_int &&
                   (*tmp).addr.in_0.sin_addr.s_addr == (*addr).addr4.s_addr {
                break ;
            }
            if !addr.is_null() &&
                   (*tmp).addr.sa.sa_family as libc::c_int ==
                       10 as libc::c_int && family == 10 as libc::c_int &&
                   ({
                        let mut __a: *const in6_addr =
                            &mut (*tmp).addr.in6.sin6_addr as *mut in6_addr as
                                *const in6_addr;
                        let mut __b: *const in6_addr =
                            &mut (*addr).addr6 as *mut in6_addr as
                                *const in6_addr;
                        ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                             ==
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
                    }) != 0 {
                break ;
            }
        }
        tmp = (*tmp).next
    }
    if !tmp.is_null() && !auth.is_null() {
        *auth = 1 as libc::c_int;
        ret = 1 as libc::c_int
    }
    return ret;
}
/* Fix for problem that the kernel sometimes reports the loopback interface as the
   arrival interface when a packet originates locally, even when sent to address of 
   an interface other than the loopback. Accept packet if it arrived via a loopback 
   interface, even when we're not accepting packets that way, as long as the destination
   address is one we're believing. Interface list must be up-to-date before calling. */
#[no_mangle]
pub unsafe extern "C" fn loopback_exception(mut fd: libc::c_int,
                                            mut family: libc::c_int,
                                            mut addr: *mut all_addr,
                                            mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut ifr: ifreq =
        ifreq{ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_2{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut iface: *mut irec = 0 as *mut irec;
    safe_strncpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), name,
                 16 as libc::c_int as size_t);
    if ioctl(fd, 0x8913 as libc::c_int as libc::c_ulong,
             &mut ifr as *mut ifreq) != -(1 as libc::c_int) &&
           ifr.ifr_ifru.ifru_flags as libc::c_int &
               IFF_LOOPBACK as libc::c_int != 0 {
        iface = (*dnsmasq_daemon).interfaces;
        while !iface.is_null() {
            if (*iface).addr.sa.sa_family as libc::c_int == family {
                if family == 2 as libc::c_int {
                    if (*iface).addr.in_0.sin_addr.s_addr ==
                           (*addr).addr4.s_addr {
                        return 1 as libc::c_int
                    }
                } else if ({
                               let mut __a: *const in6_addr =
                                   &mut (*iface).addr.in6.sin6_addr as
                                       *mut in6_addr as *const in6_addr;
                               let mut __b: *const in6_addr =
                                   &mut (*addr).addr6 as *mut in6_addr as
                                       *const in6_addr;
                               ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as
                                                               usize] ==
                                    (*__b).__in6_u.__u6_addr32[0 as
                                                                   libc::c_int
                                                                   as usize]
                                    &&
                                    (*__a).__in6_u.__u6_addr32[1 as
                                                                   libc::c_int
                                                                   as usize]
                                        ==
                                        (*__b).__in6_u.__u6_addr32[1 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                    &&
                                    (*__a).__in6_u.__u6_addr32[2 as
                                                                   libc::c_int
                                                                   as usize]
                                        ==
                                        (*__b).__in6_u.__u6_addr32[2 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                    &&
                                    (*__a).__in6_u.__u6_addr32[3 as
                                                                   libc::c_int
                                                                   as usize]
                                        ==
                                        (*__b).__in6_u.__u6_addr32[3 as
                                                                       libc::c_int
                                                                       as
                                                                       usize])
                                   as libc::c_int
                           }) != 0 {
                    return 1 as libc::c_int
                }
            }
            iface = (*iface).next
        }
    }
    return 0 as libc::c_int;
}
/* If we're configured with something like --interface=eth0:0 then we'll listen correctly
   on the relevant address, but the name of the arrival interface, derived from the
   index won't match the config. Check that we found an interface address for the arrival 
   interface: daemon->interfaces must be up-to-date. */
#[no_mangle]
pub unsafe extern "C" fn label_exception(mut index: libc::c_int,
                                         mut family: libc::c_int,
                                         mut addr: *mut all_addr)
 -> libc::c_int {
    let mut iface: *mut irec = 0 as *mut irec;
    /* labels only supported on IPv4 addresses. */
    if family != 2 as libc::c_int { return 0 as libc::c_int }
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).index == index &&
               (*iface).addr.sa.sa_family as libc::c_int == 2 as libc::c_int
               && (*iface).addr.in_0.sin_addr.s_addr == (*addr).addr4.s_addr {
            return 1 as libc::c_int
        }
        iface = (*iface).next
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn iface_allowed(mut param: *mut iface_param,
                                   mut if_index: libc::c_int,
                                   mut label: *mut libc::c_char,
                                   mut addr: *mut mysockaddr,
                                   mut netmask: in_addr,
                                   mut prefixlen: libc::c_int,
                                   mut iface_flags: libc::c_int)
 -> libc::c_int {
    let mut iface: *mut irec = 0 as *mut irec;
    let mut mtu: libc::c_int = 0 as libc::c_int;
    let mut loopback: libc::c_int = 0;
    let mut ifr: ifreq =
        ifreq{ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_2{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut tftp_ok: libc::c_int =
        ((*dnsmasq_daemon).options[(40 as libc::c_int as
                                        libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                                       as usize] &
             (1 as libc::c_uint) <<
                 (40 as libc::c_int as
                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       as
                                                       libc::c_ulong).wrapping_mul(8
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong))
             != 0) as libc::c_int;
    let mut dhcp_ok: libc::c_int = 1 as libc::c_int;
    let mut auth_dns: libc::c_int = 0 as libc::c_int;
    let mut is_label: libc::c_int = 0 as libc::c_int;
    let mut tmp: *mut iname = 0 as *mut iname;
    if indextoname((*param).fd, if_index, ifr.ifr_ifrn.ifrn_name.as_mut_ptr())
           == 0 ||
           ioctl((*param).fd, 0x8913 as libc::c_int as libc::c_ulong,
                 &mut ifr as *mut ifreq) == -(1 as libc::c_int) {
        return 0 as libc::c_int
    }
    loopback =
        ifr.ifr_ifru.ifru_flags as libc::c_int & IFF_LOOPBACK as libc::c_int;
    if loopback != 0 { dhcp_ok = 0 as libc::c_int }
    if ioctl((*param).fd, 0x8921 as libc::c_int as libc::c_ulong,
             &mut ifr as *mut ifreq) != -(1 as libc::c_int) {
        mtu = ifr.ifr_ifru.ifru_mtu
    }
    if label.is_null() {
        label = ifr.ifr_ifrn.ifrn_name.as_mut_ptr()
    } else { is_label = strcmp(label, ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) }
    /* maintain a list of all addresses on all interfaces for --local-service option */
    if (*dnsmasq_daemon).options[(49 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (49 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        let mut al: *mut addrlist = 0 as *mut addrlist;
        if !(*param).spare.is_null() {
            al = (*param).spare;
            (*param).spare = (*al).next
        } else {
            al =
                whine_malloc(::std::mem::size_of::<addrlist>() as
                                 libc::c_ulong) as *mut addrlist
        }
        if !al.is_null() {
            (*al).next = (*dnsmasq_daemon).interface_addrs;
            (*dnsmasq_daemon).interface_addrs = al;
            (*al).prefixlen = prefixlen;
            if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int {
                (*al).addr.addr4 = (*addr).in_0.sin_addr;
                (*al).flags = 0 as libc::c_int
            } else {
                (*al).addr.addr6 = (*addr).in6.sin6_addr;
                (*al).flags = 2 as libc::c_int
            }
        }
    }
    if (*addr).sa.sa_family as libc::c_int != 10 as libc::c_int ||
           ({
                let mut __a: *const in6_addr =
                    &mut (*addr).in6.sin6_addr as *mut in6_addr as
                        *const in6_addr;
                ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                     __bswap_32(0xffc00000 as libc::c_uint) ==
                     __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
            }) == 0 {
        let mut int_name: *mut interface_name = 0 as *mut interface_name;
        let mut al_0: *mut addrlist = 0 as *mut addrlist;
        let mut zone: *mut auth_zone = 0 as *mut auth_zone;
        let mut name: *mut auth_name_list = 0 as *mut auth_name_list;
        /* Find subnets in auth_zones */
        zone = (*dnsmasq_daemon).auth_zones;
        while !zone.is_null() {
            name = (*zone).interface_names;
            while !name.is_null() {
                if wildcard_match((*name).name, label) != 0 {
                    if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int
                           && (*name).flags & 2 as libc::c_int != 0 {
                        if !(*param).spare.is_null() {
                            al_0 = (*param).spare;
                            (*param).spare = (*al_0).next
                        } else {
                            al_0 =
                                whine_malloc(::std::mem::size_of::<addrlist>()
                                                 as libc::c_ulong) as
                                    *mut addrlist
                        }
                        if !al_0.is_null() {
                            (*al_0).next = (*zone).subnet;
                            (*zone).subnet = al_0;
                            (*al_0).prefixlen = prefixlen;
                            (*al_0).addr.addr4 = (*addr).in_0.sin_addr;
                            (*al_0).flags = 0 as libc::c_int
                        }
                    }
                    if (*addr).sa.sa_family as libc::c_int ==
                           10 as libc::c_int &&
                           (*name).flags & 1 as libc::c_int != 0 {
                        if !(*param).spare.is_null() {
                            al_0 = (*param).spare;
                            (*param).spare = (*al_0).next
                        } else {
                            al_0 =
                                whine_malloc(::std::mem::size_of::<addrlist>()
                                                 as libc::c_ulong) as
                                    *mut addrlist
                        }
                        if !al_0.is_null() {
                            (*al_0).next = (*zone).subnet;
                            (*zone).subnet = al_0;
                            (*al_0).prefixlen = prefixlen;
                            (*al_0).addr.addr6 = (*addr).in6.sin6_addr;
                            (*al_0).flags = 2 as libc::c_int
                        }
                    }
                }
                name = (*name).next
            }
            zone = (*zone).next
        }
        /* Update addresses from interface_names. These are a set independent
	 of the set we're listening on. */
        int_name = (*dnsmasq_daemon).int_names;
        while !int_name.is_null() {
            if strncmp(label, (*int_name).intr,
                       16 as libc::c_int as libc::c_ulong) == 0 as libc::c_int
                   &&
                   ((*addr).sa.sa_family as libc::c_int == (*int_name).family
                        || (*int_name).family == 0 as libc::c_int) {
                if !(*param).spare.is_null() {
                    al_0 = (*param).spare;
                    (*param).spare = (*al_0).next
                } else {
                    al_0 =
                        whine_malloc(::std::mem::size_of::<addrlist>() as
                                         libc::c_ulong) as *mut addrlist
                }
                if !al_0.is_null() {
                    (*al_0).next = (*int_name).addr;
                    (*int_name).addr = al_0;
                    if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int
                       {
                        (*al_0).addr.addr4 = (*addr).in_0.sin_addr;
                        (*al_0).flags = 0 as libc::c_int
                    } else {
                        (*al_0).addr.addr6 = (*addr).in6.sin6_addr;
                        (*al_0).flags = 2 as libc::c_int;
                        /* Privacy addresses and addresses still undergoing DAD and deprecated addresses
		       don't appear in forward queries, but will in reverse ones. */
                        if iface_flags & 4 as libc::c_int == 0 ||
                               iface_flags &
                                   (2 as libc::c_int | 1 as libc::c_int) != 0
                           {
                            (*al_0).flags |= 4 as libc::c_int
                        }
                    }
                }
            }
            int_name = (*int_name).next
        }
    }
    /* check whether the interface IP has been added already 
     we call this routine multiple times. */
    iface = (*dnsmasq_daemon).interfaces; /* for garbage collection */
    while !iface.is_null() {
        if sockaddr_isequal(&mut (*iface).addr, addr) != 0 &&
               (*iface).index == if_index {
            (*iface).dad =
                (iface_flags & 1 as libc::c_int != 0) as libc::c_int;
            (*iface).found = 1 as libc::c_int;
            (*iface).netmask = netmask;
            return 1 as libc::c_int
        }
        iface = (*iface).next
    }
    /* If we are restricting the set of interfaces to use, make
     sure that loopback interfaces are in that set. */
    if !(*dnsmasq_daemon).if_names.is_null() && loopback != 0 {
        let mut lo: *mut iname = 0 as *mut iname;
        lo = (*dnsmasq_daemon).if_names;
        while !lo.is_null() {
            if !(*lo).name.is_null() &&
                   strcmp((*lo).name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) ==
                       0 as libc::c_int {
                break ;
            }
            lo = (*lo).next
        }
        if lo.is_null() &&
               {
                   lo =
                       whine_malloc(::std::mem::size_of::<iname>() as
                                        libc::c_ulong) as *mut iname;
                   !lo.is_null()
               } {
            (*lo).name =
                whine_malloc(strlen(ifr.ifr_ifrn.ifrn_name.as_mut_ptr()).wrapping_add(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong))
                    as *mut libc::c_char;
            if !(*lo).name.is_null() {
                strcpy((*lo).name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
                (*lo).used = 1 as libc::c_int;
                (*lo).next = (*dnsmasq_daemon).if_names;
                (*dnsmasq_daemon).if_names = lo
            } else { free(lo as *mut libc::c_void); }
        }
    }
    if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int &&
           iface_check(2 as libc::c_int,
                       &mut (*addr).in_0.sin_addr as *mut in_addr as
                           *mut all_addr, label, &mut auth_dns) == 0 {
        return 1 as libc::c_int
    }
    if (*addr).sa.sa_family as libc::c_int == 10 as libc::c_int &&
           iface_check(10 as libc::c_int,
                       &mut (*addr).in6.sin6_addr as *mut in6_addr as
                           *mut all_addr, label, &mut auth_dns) == 0 {
        return 1 as libc::c_int
    }
    /* No DHCP where we're doing auth DNS. */
    if auth_dns != 0 {
        tftp_ok = 0 as libc::c_int;
        dhcp_ok = 0 as libc::c_int
    } else {
        tmp = (*dnsmasq_daemon).dhcp_except;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                tftp_ok = 0 as libc::c_int;
                dhcp_ok = 0 as libc::c_int
            }
            tmp = (*tmp).next
        }
    }
    if !(*dnsmasq_daemon).tftp_interfaces.is_null() {
        /* dedicated tftp interface list */
        tftp_ok = 0 as libc::c_int;
        tmp = (*dnsmasq_daemon).tftp_interfaces;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                tftp_ok = 1 as libc::c_int
            }
            tmp = (*tmp).next
        }
    }
    /* add to list */
    iface =
        whine_malloc(::std::mem::size_of::<irec>() as libc::c_ulong) as
            *mut irec; /* dummy */
    if !iface.is_null() {
        (*iface).addr = *addr;
        (*iface).netmask = netmask;
        (*iface).tftp_ok = tftp_ok;
        (*iface).dhcp_ok = dhcp_ok;
        (*iface).dns_auth = auth_dns;
        (*iface).mtu = mtu;
        (*iface).dad = (iface_flags & 1 as libc::c_int != 0) as libc::c_int;
        (*iface).found = 1 as libc::c_int;
        (*iface).warned = 0 as libc::c_int;
        (*iface).multicast_done = (*iface).warned;
        (*iface).done = (*iface).multicast_done;
        (*iface).index = if_index;
        (*iface).label = is_label;
        (*iface).name =
            whine_malloc(strlen(ifr.ifr_ifrn.ifrn_name.as_mut_ptr()).wrapping_add(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong))
                as *mut libc::c_char;
        if !(*iface).name.is_null() {
            strcpy((*iface).name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
            (*iface).next = (*dnsmasq_daemon).interfaces;
            (*dnsmasq_daemon).interfaces = iface;
            return 1 as libc::c_int
        }
        free(iface as *mut libc::c_void);
    }
    *__errno_location() = 12 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn iface_allowed_v6(mut local: *mut in6_addr,
                                      mut prefix: libc::c_int,
                                      mut scope: libc::c_int,
                                      mut if_index: libc::c_int,
                                      mut flags: libc::c_int,
                                      mut preferred: libc::c_int,
                                      mut valid: libc::c_int,
                                      mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut addr: mysockaddr =
        mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut netmask: in_addr = in_addr{s_addr: 0,};
    netmask.s_addr = 0 as libc::c_int as in_addr_t;
    /* warning */
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in6.sin6_family = 10 as libc::c_int as sa_family_t;
    addr.in6.sin6_addr = *local;
    addr.in6.sin6_port = __bswap_16((*dnsmasq_daemon).port as __uint16_t);
    /* FreeBSD insists this is zero for non-linklocal addresses */
    if ({
            let mut __a: *const in6_addr = local as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                 __bswap_32(0xffc00000 as libc::c_uint) ==
                 __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
        }) != 0 {
        addr.in6.sin6_scope_id = if_index as uint32_t
    } else { addr.in6.sin6_scope_id = 0 as libc::c_int as uint32_t }
    return iface_allowed(vparam as *mut iface_param, if_index,
                         0 as *mut libc::c_char, &mut addr, netmask, prefix,
                         flags);
}
unsafe extern "C" fn iface_allowed_v4(mut local: in_addr,
                                      mut if_index: libc::c_int,
                                      mut label: *mut libc::c_char,
                                      mut netmask: in_addr,
                                      mut broadcast: in_addr,
                                      mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut addr: mysockaddr =
        mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut prefix: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    /* warning */
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in_0.sin_family = 2 as libc::c_int as sa_family_t;
    addr.in_0.sin_addr = local;
    addr.in_0.sin_port = __bswap_16((*dnsmasq_daemon).port as __uint16_t);
    /* determine prefix length from netmask */
    prefix = 32 as libc::c_int;
    bit = 1 as libc::c_int;
    while bit as libc::c_uint & __bswap_32(netmask.s_addr) ==
              0 as libc::c_int as libc::c_uint && prefix != 0 as libc::c_int {
        bit = bit << 1 as libc::c_int;
        prefix -= 1
    }
    return iface_allowed(vparam as *mut iface_param, if_index, label,
                         &mut addr, netmask, prefix, 0 as libc::c_int);
}
/*
 * Clean old interfaces no longer found.
 */
unsafe extern "C" fn clean_interfaces() {
    let mut iface: *mut irec = 0 as *mut irec;
    let mut up: *mut *mut irec = &mut (*dnsmasq_daemon).interfaces;
    iface = *up;
    while !iface.is_null() {
        if (*iface).found == 0 && (*iface).done == 0 {
            *up = (*iface).next;
            free((*iface).name as *mut libc::c_void);
            free(iface as *mut libc::c_void);
        } else { up = &mut (*iface).next }
        iface = *up
    };
}
/* * Release listener if no other interface needs it.
 *
 * @return 1 if released, 0 if still required
 */
unsafe extern "C" fn release_listener(mut l: *mut listener) -> libc::c_int {
    if (*l).used > 1 as libc::c_int {
        let mut iface: *mut irec = 0 as *mut irec;
        iface = (*dnsmasq_daemon).interfaces;
        while !iface.is_null() {
            if (*iface).done != 0 &&
                   sockaddr_isequal(&mut (*l).addr, &mut (*iface).addr) != 0 {
                if (*iface).found != 0 {
                    /* update listener to point to active interface instead */
                    if (*(*l).iface).found == 0 { (*l).iface = iface }
                } else { (*l).used -= 1; (*iface).done = 0 as libc::c_int }
            }
            iface = (*iface).next
        }
        /* Someone is still using this listener, skip its deletion */
        if (*l).used > 0 as libc::c_int { return 0 as libc::c_int }
    }
    if (*(*l).iface).done != 0 {
        let mut port: libc::c_int = 0;
        port =
            prettyprint_addr(&mut (*(*l).iface).addr,
                             (*dnsmasq_daemon).addrbuff);
        my_syslog(7 as libc::c_int,
                  b"stopped listening on %s(#%d): %s port %d\x00" as *const u8
                      as *const libc::c_char, (*(*l).iface).name,
                  (*(*l).iface).index, (*dnsmasq_daemon).addrbuff, port);
        /* In case it ever returns */
        (*(*l).iface).done = 0 as libc::c_int
    }
    if (*l).fd != -(1 as libc::c_int) { close((*l).fd); }
    if (*l).tcpfd != -(1 as libc::c_int) { close((*l).tcpfd); }
    if (*l).tftpfd != -(1 as libc::c_int) { close((*l).tftpfd); }
    free(l as *mut libc::c_void);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn enumerate_interfaces(mut reset: libc::c_int)
 -> libc::c_int {
    static mut spare: *mut addrlist = 0 as *const addrlist as *mut addrlist;
    static mut done: libc::c_int = 0 as libc::c_int;
    let mut param: iface_param =
        iface_param{spare: 0 as *mut addrlist, fd: 0,};
    let mut errsave: libc::c_int = 0;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut addr: *mut addrlist = 0 as *mut addrlist;
    let mut tmp: *mut addrlist = 0 as *mut addrlist;
    let mut intname: *mut interface_name = 0 as *mut interface_name;
    let mut iface: *mut irec = 0 as *mut irec;
    let mut zone: *mut auth_zone = 0 as *mut auth_zone;
    /* Do this max once per select cycle  - also inhibits netlink socket use
   in TCP child processes. */
    if reset != 0 { done = 0 as libc::c_int; return 1 as libc::c_int }
    if done != 0 { return 1 as libc::c_int }
    done = 1 as libc::c_int;
    param.fd =
        socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
    if param.fd == -(1 as libc::c_int) { return 0 as libc::c_int }
    /* Mark interfaces for garbage collection */
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        (*iface).found = 0 as libc::c_int;
        iface = (*iface).next
    }
    /* remove addresses stored against interface_names */
    intname = (*dnsmasq_daemon).int_names;
    while !intname.is_null() {
        addr = (*intname).addr;
        while !addr.is_null() {
            tmp = (*addr).next;
            (*addr).next = spare;
            spare = addr;
            addr = tmp
        }
        (*intname).addr = 0 as *mut addrlist;
        intname = (*intname).next
    }
    /* Remove list of addresses of local interfaces */
    addr = (*dnsmasq_daemon).interface_addrs;
    while !addr.is_null() {
        tmp = (*addr).next;
        (*addr).next = spare;
        spare = addr;
        addr = tmp
    }
    (*dnsmasq_daemon).interface_addrs = 0 as *mut addrlist;
    /* remove addresses stored against auth_zone subnets, but not 
   ones configured as address literals */
    zone = (*dnsmasq_daemon).auth_zones;
    while !zone.is_null() {
        if !(*zone).interface_names.is_null() {
            let mut up: *mut *mut addrlist = 0 as *mut *mut addrlist;
            up = &mut (*zone).subnet;
            addr = (*zone).subnet;
            while !addr.is_null() {
                tmp = (*addr).next;
                if (*addr).flags & 1 as libc::c_int != 0 {
                    up = &mut (*addr).next
                } else {
                    *up = (*addr).next;
                    (*addr).next = spare;
                    spare = addr
                }
                addr = tmp
            }
        }
        zone = (*zone).next
    }
    param.spare = spare;
    ret =
        iface_enumerate(10 as libc::c_int,
                        &mut param as *mut iface_param as *mut libc::c_void,
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
                                                               libc::c_int>>(Some(iface_allowed_v6
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
    if ret != 0 {
        ret =
            iface_enumerate(2 as libc::c_int,
                            &mut param as *mut iface_param as
                                *mut libc::c_void,
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                    in_addr,
                                                                                _:
                                                                                    libc::c_int,
                                                                                _:
                                                                                    *mut libc::c_char,
                                                                                _:
                                                                                    in_addr,
                                                                                _:
                                                                                    in_addr,
                                                                                _:
                                                                                    *mut libc::c_void)
                                                               ->
                                                                   libc::c_int>,
                                                    Option<unsafe extern "C" fn()
                                                               ->
                                                                   libc::c_int>>(Some(iface_allowed_v4
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   in_addr,
                                                                                                               _:
                                                                                                                   libc::c_int,
                                                                                                               _:
                                                                                                                   *mut libc::c_char,
                                                                                                               _:
                                                                                                                   in_addr,
                                                                                                               _:
                                                                                                                   in_addr,
                                                                                                               _:
                                                                                                                   *mut libc::c_void)
                                                                                              ->
                                                                                                  libc::c_int)))
    }
    errsave = *__errno_location();
    close(param.fd);
    if (*dnsmasq_daemon).options[(39 as libc::c_int as
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
        /* Garbage-collect listeners listening on addresses that no longer exist.
	 Does nothing when not binding interfaces or for listeners on localhost, 
	 since the ->iface field is NULL. Note that this needs the protections
	 against reentrancy, hence it's here.  It also means there's a possibility,
	 in OPT_CLEVERBIND mode, that at listener will just disappear after
	 a call to enumerate_interfaces, this is checked OK on all calls. */
        let mut l: *mut listener = 0 as *mut listener;
        let mut tmp_0: *mut listener = 0 as *mut listener;
        let mut up_0: *mut *mut listener = 0 as *mut *mut listener;
        let mut freed: libc::c_int = 0 as libc::c_int;
        up_0 = &mut (*dnsmasq_daemon).listeners;
        l = (*dnsmasq_daemon).listeners;
        while !l.is_null() {
            tmp_0 = (*l).next;
            if (*l).iface.is_null() || (*(*l).iface).found != 0 {
                up_0 = &mut (*l).next
            } else if release_listener(l) != 0 {
                *up_0 = tmp_0;
                freed = 1 as libc::c_int
            }
            l = tmp_0
        }
        if freed != 0 { clean_interfaces(); }
    }
    *__errno_location() = errsave;
    spare = param.spare;
    return ret;
}
/* set NONBLOCK bit on fd: See Stevens 16.6 */
#[no_mangle]
pub unsafe extern "C" fn fix_fd(mut fd: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    flags = fcntl(fd, 3 as libc::c_int);
    if flags == -(1 as libc::c_int) ||
           fcntl(fd, 4 as libc::c_int, flags | 0o4000 as libc::c_int) ==
               -(1 as libc::c_int) {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn make_sock(mut addr: *mut mysockaddr,
                               mut type_0: libc::c_int,
                               mut dienow: libc::c_int) -> libc::c_int {
    let mut port: libc::c_int = 0;
    let mut errsave: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_block: u64;
    let mut family: libc::c_int = (*addr).sa.sa_family as libc::c_int;
    let mut fd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut opt: libc::c_int = 1 as libc::c_int;
    fd = socket(family, type_0, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        port = 0;
        errsave = 0;
        s = 0 as *mut libc::c_char;
        /* No error if the kernel just doesn't support this IP flavour */
        if *__errno_location() == 93 as libc::c_int ||
               *__errno_location() == 97 as libc::c_int ||
               *__errno_location() == 22 as libc::c_int {
            return -(1 as libc::c_int)
        }
    } else if !(setsockopt(fd, 1 as libc::c_int, 2 as libc::c_int,
                           &mut opt as *mut libc::c_int as
                               *const libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as socklen_t) ==
                    -(1 as libc::c_int) || fix_fd(fd) == 0) {
        if !(family == 10 as libc::c_int &&
                 setsockopt(fd, IPPROTO_IPV6 as libc::c_int,
                            26 as libc::c_int,
                            &mut opt as *mut libc::c_int as
                                *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as
                                libc::c_ulong as socklen_t) ==
                     -(1 as libc::c_int)) {
            rc =
                bind(fd,
                     __CONST_SOCKADDR_ARG{__sockaddr__:
                                              addr as *mut sockaddr,},
                     sa_len(addr) as socklen_t);
            if !(rc == -(1 as libc::c_int)) {
                if type_0 == SOCK_STREAM as libc::c_int {
                    let mut qlen: libc::c_int = 5 as libc::c_int;
                    setsockopt(fd, IPPROTO_TCP as libc::c_int,
                               23 as libc::c_int,
                               &mut qlen as *mut libc::c_int as
                                   *const libc::c_void,
                               ::std::mem::size_of::<libc::c_int>() as
                                   libc::c_ulong as socklen_t);
                    if listen(fd, 32 as libc::c_int) == -(1 as libc::c_int) {
                        current_block = 4055993212646746884;
                    } else { current_block = 11459959175219260272; }
                } else if family == 2 as libc::c_int {
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
                           == 0 {
                        if setsockopt(fd, IPPROTO_IP as libc::c_int,
                                      8 as libc::c_int,
                                      &mut opt as *mut libc::c_int as
                                          *const libc::c_void,
                                      ::std::mem::size_of::<libc::c_int>() as
                                          libc::c_ulong as socklen_t) ==
                               -(1 as libc::c_int) {
                            current_block = 4055993212646746884;
                        } else { current_block = 11459959175219260272; }
                    } else { current_block = 11459959175219260272; }
                } else if set_ipv6pktinfo(fd) == 0 {
                    current_block = 4055993212646746884;
                } else { current_block = 11459959175219260272; }
                match current_block {
                    4055993212646746884 => { }
                    _ => { return fd }
                }
            }
        }
    }
    errsave = *__errno_location();
    port = prettyprint_addr(addr, (*dnsmasq_daemon).addrbuff);
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
           == 0 &&
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
               == 0 {
        sprintf((*dnsmasq_daemon).addrbuff,
                b"port %d\x00" as *const u8 as *const libc::c_char, port);
    }
    s =
        b"failed to create listening socket for %s: %s\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char;
    if fd != -(1 as libc::c_int) { close(fd); }
    *__errno_location() = errsave;
    if dienow != 0 {
        /* failure to bind addresses given by --listen-address at this point
	     is OK if we're doing bind-dynamic */
        if (*dnsmasq_daemon).options[(39 as libc::c_int as
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
               == 0 {
            die(s, (*dnsmasq_daemon).addrbuff, 2 as libc::c_int);
        }
    } else {
        my_syslog(4 as libc::c_int, s, (*dnsmasq_daemon).addrbuff,
                  strerror(*__errno_location()));
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn set_ipv6pktinfo(mut fd: libc::c_int) -> libc::c_int {
    let mut opt: libc::c_int = 1 as libc::c_int;
    /* The API changed around Linux 2.6.14 but the old ABI is still supported:
     handle all combinations of headers and kernel.
     OpenWrt note that this fixes the problem addressed by your very broken patch. */
    (*dnsmasq_daemon).v6pktinfo = 50 as libc::c_int;
    if setsockopt(fd, IPPROTO_IPV6 as libc::c_int, 49 as libc::c_int,
                  &mut opt as *mut libc::c_int as *const libc::c_void,
                  ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                      socklen_t) != -(1 as libc::c_int) {
        return 1 as libc::c_int
    } else {
        if *__errno_location() == 92 as libc::c_int &&
               setsockopt(fd, IPPROTO_IPV6 as libc::c_int, 2 as libc::c_int,
                          &mut opt as *mut libc::c_int as *const libc::c_void,
                          ::std::mem::size_of::<libc::c_int>() as
                              libc::c_ulong as socklen_t) !=
                   -(1 as libc::c_int) {
            (*dnsmasq_daemon).v6pktinfo = 2 as libc::c_int;
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/* Find the interface on which a TCP connection arrived, if possible, or zero otherwise. */
#[no_mangle]
pub unsafe extern "C" fn tcp_interface(mut fd: libc::c_int,
                                       mut af: libc::c_int) -> libc::c_int {
    /* suppress potential unused warning */
    let mut if_index: libc::c_int = 0 as libc::c_int;
    let mut opt: libc::c_int = 1 as libc::c_int;
    let mut cmptr: *mut cmsghdr = 0 as *mut cmsghdr;
    let mut msg: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut len: socklen_t = 0;
    /* use mshdr so that the CMSDG_* macros are available */
    msg.msg_control = (*dnsmasq_daemon).packet as *mut libc::c_void;
    len = (*dnsmasq_daemon).packet_buff_sz as socklen_t;
    msg.msg_controllen = len as size_t;
    /* we overwrote the buffer... */
    (*dnsmasq_daemon).srv_save = 0 as *mut server;
    if af == 2 as libc::c_int {
        if setsockopt(fd, IPPROTO_IP as libc::c_int, 8 as libc::c_int,
                      &mut opt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) != -(1 as libc::c_int) &&
               getsockopt(fd, IPPROTO_IP as libc::c_int, 9 as libc::c_int,
                          msg.msg_control, &mut len) != -(1 as libc::c_int) {
            msg.msg_controllen = len as size_t;
            cmptr =
                if msg.msg_controllen >=
                       ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                    msg.msg_control as *mut cmsghdr
                } else { 0 as *mut cmsghdr };
            while !cmptr.is_null() {
                if (*cmptr).cmsg_level == IPPROTO_IP as libc::c_int &&
                       (*cmptr).cmsg_type == 8 as libc::c_int {
                    let mut p: C2RustUnnamed_13 =
                        C2RustUnnamed_13{c: 0 as *mut libc::c_uchar,};
                    p.c = (*cmptr).__cmsg_data.as_mut_ptr();
                    if_index = (*p.p).ipi_ifindex
                }
                cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            }
        }
    } else if set_ipv6pktinfo(fd) != 0 &&
                  getsockopt(fd, IPPROTO_IPV6 as libc::c_int,
                             6 as libc::c_int, msg.msg_control, &mut len) !=
                      -(1 as libc::c_int) {
        msg.msg_controllen = len as size_t;
        cmptr =
            if msg.msg_controllen >=
                   ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                msg.msg_control as *mut cmsghdr
            } else { 0 as *mut cmsghdr };
        while !cmptr.is_null() {
            if (*cmptr).cmsg_level == IPPROTO_IPV6 as libc::c_int &&
                   (*cmptr).cmsg_type == (*dnsmasq_daemon).v6pktinfo {
                let mut p_0: C2RustUnnamed_12 =
                    C2RustUnnamed_12{c: 0 as *mut libc::c_uchar,};
                p_0.c = (*cmptr).__cmsg_data.as_mut_ptr();
                if_index = (*p_0.p).ipi6_ifindex as libc::c_int
            }
            cmptr = __cmsg_nxthdr(&mut msg, cmptr)
        }
    }
    /* Only the RFC-2292 API has the ability to find the interface for TCP connections,
	 it was removed in RFC-3542 !!!! 

	 Fortunately, Linux kept the 2292 ABI when it moved to 3542. The following code always
	 uses the old ABI, and should work with pre- and post-3542 kernel headers */
    /* Linux */
    return if_index;
}
unsafe extern "C" fn create_listeners(mut addr: *mut mysockaddr,
                                      mut do_tftp: libc::c_int,
                                      mut dienow: libc::c_int)
 -> *mut listener {
    let mut l: *mut listener = 0 as *mut listener;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut tcpfd: libc::c_int = -(1 as libc::c_int);
    let mut tftpfd: libc::c_int = -(1 as libc::c_int);
    if (*dnsmasq_daemon).port != 0 as libc::c_int {
        fd = make_sock(addr, SOCK_DGRAM as libc::c_int, dienow);
        tcpfd = make_sock(addr, SOCK_STREAM as libc::c_int, dienow)
    }
    if do_tftp != 0 {
        if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int {
            /* port must be restored to DNS port for TCP code */
            let mut save: libc::c_short =
                (*addr).in_0.sin_port as libc::c_short;
            (*addr).in_0.sin_port =
                __bswap_16(69 as libc::c_int as __uint16_t);
            tftpfd = make_sock(addr, SOCK_DGRAM as libc::c_int, dienow);
            (*addr).in_0.sin_port = save as in_port_t
        } else {
            let mut save_0: libc::c_short =
                (*addr).in6.sin6_port as libc::c_short;
            (*addr).in6.sin6_port =
                __bswap_16(69 as libc::c_int as __uint16_t);
            tftpfd = make_sock(addr, SOCK_DGRAM as libc::c_int, dienow);
            (*addr).in6.sin6_port = save_0 as in_port_t
        }
    }
    if fd != -(1 as libc::c_int) || tcpfd != -(1 as libc::c_int) ||
           tftpfd != -(1 as libc::c_int) {
        l =
            safe_malloc(::std::mem::size_of::<listener>() as libc::c_ulong) as
                *mut listener;
        (*l).next = 0 as *mut listener;
        (*l).fd = fd;
        (*l).tcpfd = tcpfd;
        (*l).tftpfd = tftpfd;
        (*l).addr = *addr;
        (*l).used = 1 as libc::c_int;
        (*l).iface = 0 as *mut irec
    }
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn create_wildcard_listeners() {
    let mut addr: mysockaddr =
        mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut l: *mut listener = 0 as *mut listener;
    let mut l6: *mut listener = 0 as *mut listener;
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in_0.sin_family = 2 as libc::c_int as sa_family_t;
    addr.in_0.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    addr.in_0.sin_port = __bswap_16((*dnsmasq_daemon).port as __uint16_t);
    l =
        create_listeners(&mut addr,
                         ((*dnsmasq_daemon).options[(40 as libc::c_int as
                                                         libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_ulong))
                                                        as usize] &
                              (1 as libc::c_uint) <<
                                  (40 as libc::c_int as
                                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong))
                              != 0) as libc::c_int, 1 as libc::c_int);
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in6.sin6_family = 10 as libc::c_int as sa_family_t;
    addr.in6.sin6_addr = in6addr_any;
    addr.in6.sin6_port = __bswap_16((*dnsmasq_daemon).port as __uint16_t);
    l6 =
        create_listeners(&mut addr,
                         ((*dnsmasq_daemon).options[(40 as libc::c_int as
                                                         libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_ulong))
                                                        as usize] &
                              (1 as libc::c_uint) <<
                                  (40 as libc::c_int as
                                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong))
                              != 0) as libc::c_int, 1 as libc::c_int);
    if !l.is_null() { (*l).next = l6 } else { l = l6 }
    (*dnsmasq_daemon).listeners = l;
}
unsafe extern "C" fn find_listener(mut addr: *mut mysockaddr)
 -> *mut listener {
    let mut l: *mut listener = 0 as *mut listener;
    l = (*dnsmasq_daemon).listeners;
    while !l.is_null() {
        if sockaddr_isequal(&mut (*l).addr, addr) != 0 { return l }
        l = (*l).next
    }
    return 0 as *mut listener;
}
#[no_mangle]
pub unsafe extern "C" fn create_bound_listeners(mut dienow: libc::c_int) {
    let mut new: *mut listener = 0 as *mut listener;
    let mut iface: *mut irec = 0 as *mut irec;
    let mut if_tmp: *mut iname = 0 as *mut iname;
    let mut existing: *mut listener = 0 as *mut listener;
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).done == 0 && (*iface).dad == 0 && (*iface).found != 0 {
            existing = find_listener(&mut (*iface).addr);
            if !existing.is_null() {
                (*iface).done = 1 as libc::c_int;
                (*existing).used += 1
                /* increase usage counter */
            } else {
                new =
                    create_listeners(&mut (*iface).addr, (*iface).tftp_ok,
                                     dienow);
                if !new.is_null() {
                    (*new).iface = iface;
                    (*new).next = (*dnsmasq_daemon).listeners;
                    (*dnsmasq_daemon).listeners = new;
                    (*iface).done = 1 as libc::c_int;
                    /* Don't log the initial set of listen addresses created
               at startup, since this is happening before the logging
               system is initialised and the sign-on printed. */
                    if dienow == 0 {
                        let mut port: libc::c_int =
                            prettyprint_addr(&mut (*iface).addr,
                                             (*dnsmasq_daemon).addrbuff);
                        my_syslog(7 as libc::c_int,
                                  b"listening on %s(#%d): %s port %d\x00" as
                                      *const u8 as *const libc::c_char,
                                  (*iface).name, (*iface).index,
                                  (*dnsmasq_daemon).addrbuff, port);
                    }
                }
            }
        }
        iface = (*iface).next
    }
    /* Check for --listen-address options that haven't been used because there's
     no interface with a matching address. These may be valid: eg it's possible
     to listen on 127.0.1.1 even if the loopback interface is 127.0.0.1

     If the address isn't valid the bind() will fail and we'll die() 
     (except in bind-dynamic mode, when we'll complain but keep trying.)

     The resulting listeners have the ->iface field NULL, and this has to be
     handled by the DNS and TFTP code. It disables --localise-queries processing
     (no netmask) and some MTU login the tftp code. */
    if_tmp = (*dnsmasq_daemon).if_addrs;
    while !if_tmp.is_null() {
        if (*if_tmp).used == 0 &&
               {
                   new =
                       create_listeners(&mut (*if_tmp).addr,
                                        ((*dnsmasq_daemon).options[(40 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                         as
                                                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         as
                                                                                                                                         libc::c_ulong))
                                                                       as
                                                                       usize]
                                             &
                                             (1 as libc::c_uint) <<
                                                 (40 as libc::c_int as
                                                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                             != 0) as libc::c_int, dienow);
                   !new.is_null()
               } {
            (*new).next = (*dnsmasq_daemon).listeners;
            (*dnsmasq_daemon).listeners = new;
            if dienow == 0 {
                let mut port_0: libc::c_int =
                    prettyprint_addr(&mut (*if_tmp).addr,
                                     (*dnsmasq_daemon).addrbuff);
                my_syslog(7 as libc::c_int,
                          b"listening on %s port %d\x00" as *const u8 as
                              *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                          port_0);
            }
        }
        if_tmp = (*if_tmp).next
    };
}
/* In --bind-interfaces, the only access control is the addresses we're listening on. 
   There's nothing to avoid a query to the address of an internal interface arriving via
   an external interface where we don't want to accept queries, except that in the usual 
   case the addresses of internal interfaces are RFC1918. When bind-interfaces in use, 
   and we listen on an address that looks like it's probably globally routeable, shout.

   The fix is to use --bind-dynamic, which actually checks the arrival interface too.
   Tough if your platform doesn't support this.

   Note that checking the arrival interface is supported in the standard IPv6 API and
   always done, so we don't warn about any IPv6 addresses here.
*/
#[no_mangle]
pub unsafe extern "C" fn warn_bound_listeners() {
    let mut iface: *mut irec = 0 as *mut irec;
    let mut advice: libc::c_int = 0 as libc::c_int;
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).dns_auth == 0 {
            if (*iface).addr.sa.sa_family as libc::c_int == 2 as libc::c_int {
                if private_net((*iface).addr.in_0.sin_addr, 1 as libc::c_int)
                       == 0 {
                    inet_ntop(2 as libc::c_int,
                              &mut (*iface).addr.in_0.sin_addr as *mut in_addr
                                  as *const libc::c_void,
                              (*dnsmasq_daemon).addrbuff,
                              46 as libc::c_int as socklen_t);
                    advice = 1 as libc::c_int;
                    (*iface).warned = advice;
                    my_syslog(4 as libc::c_int,
                              b"LOUD WARNING: listening on %s may accept requests via interfaces other than %s\x00"
                                  as *const u8 as *const libc::c_char,
                              (*dnsmasq_daemon).addrbuff, (*iface).name);
                }
            }
        }
        iface = (*iface).next
    }
    if advice != 0 {
        my_syslog(4 as libc::c_int,
                  b"LOUD WARNING: use --bind-dynamic rather than --bind-interfaces to avoid DNS amplification attacks via these interface(s)\x00"
                      as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn warn_wild_labels() {
    let mut iface: *mut irec = 0 as *mut irec;
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).found != 0 && !(*iface).name.is_null() &&
               (*iface).label != 0 {
            my_syslog(4 as libc::c_int,
                      b"warning: using interface %s instead\x00" as *const u8
                          as *const libc::c_char, (*iface).name);
        }
        iface = (*iface).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn warn_int_names() {
    let mut intname: *mut interface_name = 0 as *mut interface_name;
    intname = (*dnsmasq_daemon).int_names;
    while !intname.is_null() {
        if (*intname).addr.is_null() {
            my_syslog(4 as libc::c_int,
                      b"warning: no addresses found for interface %s\x00" as
                          *const u8 as *const libc::c_char, (*intname).intr);
        }
        intname = (*intname).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn is_dad_listeners() -> libc::c_int {
    let mut iface: *mut irec = 0 as *mut irec;
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
           != 0 {
        iface = (*dnsmasq_daemon).interfaces;
        while !iface.is_null() {
            if (*iface).dad != 0 && (*iface).done == 0 {
                return 1 as libc::c_int
            }
            iface = (*iface).next
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn join_multicast(mut dienow: libc::c_int) {
    let mut iface: *mut irec = 0 as *mut irec;
    let mut tmp: *mut irec = 0 as *mut irec;
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).addr.sa.sa_family as libc::c_int == 10 as libc::c_int &&
               (*iface).dhcp_ok != 0 && (*iface).multicast_done == 0 {
            /* There's an irec per address but we only want to join for multicast 
	   once per interface. Weed out duplicates. */
            tmp = (*dnsmasq_daemon).interfaces;
            while !tmp.is_null() {
                if (*tmp).multicast_done != 0 &&
                       (*tmp).index == (*iface).index {
                    break ;
                }
                tmp = (*tmp).next
            }
            (*iface).multicast_done = 1 as libc::c_int;
            if tmp.is_null() {
                let mut mreq: ipv6_mreq =
                    ipv6_mreq{ipv6mr_multiaddr:
                                  in6_addr{__in6_u:
                                               C2RustUnnamed{__u6_addr8:
                                                                 [0; 16],},},
                              ipv6mr_interface: 0,};
                let mut err: libc::c_int = 0 as libc::c_int;
                mreq.ipv6mr_interface = (*iface).index as libc::c_uint;
                inet_pton(10 as libc::c_int,
                          b"FF02::1:2\x00" as *const u8 as
                              *const libc::c_char,
                          &mut mreq.ipv6mr_multiaddr as *mut in6_addr as
                              *mut libc::c_void);
                if ((*dnsmasq_daemon).doing_dhcp6 != 0 ||
                        !(*dnsmasq_daemon).relay6.is_null()) &&
                       setsockopt((*dnsmasq_daemon).dhcp6fd,
                                  IPPROTO_IPV6 as libc::c_int,
                                  20 as libc::c_int,
                                  &mut mreq as *mut ipv6_mreq as
                                      *const libc::c_void,
                                  ::std::mem::size_of::<ipv6_mreq>() as
                                      libc::c_ulong as socklen_t) ==
                           -(1 as libc::c_int) {
                    err = *__errno_location()
                }
                inet_pton(10 as libc::c_int,
                          b"FF05::1:3\x00" as *const u8 as
                              *const libc::c_char,
                          &mut mreq.ipv6mr_multiaddr as *mut in6_addr as
                              *mut libc::c_void);
                if (*dnsmasq_daemon).doing_dhcp6 != 0 &&
                       setsockopt((*dnsmasq_daemon).dhcp6fd,
                                  IPPROTO_IPV6 as libc::c_int,
                                  20 as libc::c_int,
                                  &mut mreq as *mut ipv6_mreq as
                                      *const libc::c_void,
                                  ::std::mem::size_of::<ipv6_mreq>() as
                                      libc::c_ulong as socklen_t) ==
                           -(1 as libc::c_int) {
                    err = *__errno_location()
                }
                inet_pton(10 as libc::c_int,
                          b"FF02::2\x00" as *const u8 as *const libc::c_char,
                          &mut mreq.ipv6mr_multiaddr as *mut in6_addr as
                              *mut libc::c_void);
                if (*dnsmasq_daemon).doing_ra != 0 &&
                       setsockopt((*dnsmasq_daemon).icmp6fd,
                                  IPPROTO_IPV6 as libc::c_int,
                                  20 as libc::c_int,
                                  &mut mreq as *mut ipv6_mreq as
                                      *const libc::c_void,
                                  ::std::mem::size_of::<ipv6_mreq>() as
                                      libc::c_ulong as socklen_t) ==
                           -(1 as libc::c_int) {
                    err = *__errno_location()
                }
                if err != 0 {
                    let mut s: *mut libc::c_char =
                        b"interface %s failed to join DHCPv6 multicast group: %s\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char;
                    *__errno_location() = err;
                    if *__errno_location() == 12 as libc::c_int {
                        my_syslog(3 as libc::c_int,
                                  b"try increasing /proc/sys/net/core/optmem_max\x00"
                                      as *const u8 as *const libc::c_char);
                    }
                    if dienow != 0 {
                        die(s, (*iface).name, 2 as libc::c_int);
                    } else {
                        my_syslog(3 as libc::c_int, s, (*iface).name,
                                  strerror(*__errno_location()));
                    }
                }
            }
        }
        iface = (*iface).next
    };
}
/* return a UDP socket bound to a random port, have to cope with straying into
   occupied port nos and reserved ones. */
#[no_mangle]
pub unsafe extern "C" fn random_sock(mut family: libc::c_int) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    fd = socket(family, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
    if fd != -(1 as libc::c_int) {
        let mut addr: mysockaddr =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        let mut ports_avail: libc::c_uint =
            ((*dnsmasq_daemon).max_port as libc::c_ushort as libc::c_int -
                 (*dnsmasq_daemon).min_port as libc::c_ushort as libc::c_int +
                 1 as libc::c_int) as libc::c_uint;
        let mut tries: libc::c_int =
            if ports_avail < 30 as libc::c_int as libc::c_uint {
                (3 as libc::c_int as libc::c_uint).wrapping_mul(ports_avail)
            } else { 100 as libc::c_int as libc::c_uint } as libc::c_int;
        memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        addr.sa.sa_family = family as sa_family_t;
        /* don't loop forever if all ports in use. */
        if fix_fd(fd) != 0 {
            loop  {
                let fresh6 = tries;
                tries = tries - 1;
                if !(fresh6 != 0) { break ; }
                let mut port: libc::c_ushort =
                    __bswap_16(((*dnsmasq_daemon).min_port +
                                    rand16() as libc::c_int %
                                        ports_avail as libc::c_ushort as
                                            libc::c_int) as __uint16_t);
                if family == 2 as libc::c_int {
                    addr.in_0.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
                    addr.in_0.sin_port = port
                } else {
                    addr.in6.sin6_addr = in6addr_any;
                    addr.in6.sin6_port = port
                }
                if bind(fd,
                        __CONST_SOCKADDR_ARG{__sockaddr__:
                                                 &mut addr as *mut mysockaddr
                                                     as *mut sockaddr,},
                        sa_len(&mut addr) as socklen_t) == 0 as libc::c_int {
                    return fd
                }
                if *__errno_location() != 98 as libc::c_int &&
                       *__errno_location() != 13 as libc::c_int {
                    break ;
                }
            }
        }
        close(fd);
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn local_bind(mut fd: libc::c_int,
                                    mut addr: *mut mysockaddr,
                                    mut intname: *mut libc::c_char,
                                    mut ifindex: libc::c_uint,
                                    mut is_tcp: libc::c_int) -> libc::c_int {
    let mut addr_copy: mysockaddr = *addr;
    let mut port: libc::c_ushort = 0;
    let mut tries: libc::c_int = 1 as libc::c_int;
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut ports_avail: libc::c_uint =
        ((*dnsmasq_daemon).max_port as libc::c_ushort as libc::c_int -
             (*dnsmasq_daemon).min_port as libc::c_ushort as libc::c_int +
             1 as libc::c_int) as libc::c_uint;
    if addr_copy.sa.sa_family as libc::c_int == 2 as libc::c_int {
        port = addr_copy.in_0.sin_port
    } else { port = addr_copy.in6.sin6_port }
    /* cannot set source _port_ for TCP connections. */
    if is_tcp != 0 { port = 0 as libc::c_int as libc::c_ushort }
    /* Bind a random port within the range given by min-port and max-port */
    if port as libc::c_int == 0 as libc::c_int {
        tries =
            if ports_avail < 30 as libc::c_int as libc::c_uint {
                (3 as libc::c_int as libc::c_uint).wrapping_mul(ports_avail)
            } else { 100 as libc::c_int as libc::c_uint } as libc::c_int;
        port =
            __bswap_16(((*dnsmasq_daemon).min_port +
                            rand16() as libc::c_int %
                                ports_avail as libc::c_ushort as libc::c_int)
                           as __uint16_t)
    }
    loop  {
        let fresh7 = tries;
        tries = tries - 1;
        if !(fresh7 != 0) { break ; }
        if addr_copy.sa.sa_family as libc::c_int == 2 as libc::c_int {
            addr_copy.in_0.sin_port = port
        } else { addr_copy.in6.sin6_port = port }
        if bind(fd,
                __CONST_SOCKADDR_ARG{__sockaddr__:
                                         &mut addr_copy as *mut mysockaddr as
                                             *mut sockaddr,},
                sa_len(&mut addr_copy) as socklen_t) != -(1 as libc::c_int) {
            done = 1 as libc::c_int;
            break ;
        } else {
            if *__errno_location() != 98 as libc::c_int &&
                   *__errno_location() != 13 as libc::c_int {
                return 0 as libc::c_int
            }
            port =
                __bswap_16(((*dnsmasq_daemon).min_port +
                                rand16() as libc::c_int %
                                    ports_avail as libc::c_ushort as
                                        libc::c_int) as __uint16_t)
        }
    }
    if done == 0 { return 0 as libc::c_int }
    if is_tcp == 0 && ifindex > 0 as libc::c_int as libc::c_uint {
        if addr_copy.sa.sa_family as libc::c_int == 2 as libc::c_int {
            let mut ifindex_opt: uint32_t = __bswap_32(ifindex);
            return (setsockopt(fd, IPPROTO_IP as libc::c_int,
                               50 as libc::c_int,
                               &mut ifindex_opt as *mut uint32_t as
                                   *const libc::c_void,
                               ::std::mem::size_of::<uint32_t>() as
                                   libc::c_ulong as socklen_t) ==
                        0 as libc::c_int) as libc::c_int
        }
        if addr_copy.sa.sa_family as libc::c_int == 10 as libc::c_int {
            let mut ifindex_opt_0: uint32_t = __bswap_32(ifindex);
            return (setsockopt(fd, IPPROTO_IPV6 as libc::c_int,
                               76 as libc::c_int,
                               &mut ifindex_opt_0 as *mut uint32_t as
                                   *const libc::c_void,
                               ::std::mem::size_of::<uint32_t>() as
                                   libc::c_ulong as socklen_t) ==
                        0 as libc::c_int) as libc::c_int
        }
    }
    /* suppress potential unused warning */
    if *intname.offset(0 as libc::c_int as isize) as libc::c_int !=
           0 as libc::c_int &&
           setsockopt(fd, 1 as libc::c_int, 25 as libc::c_int,
                      intname as *const libc::c_void,
                      16 as libc::c_int as socklen_t) == -(1 as libc::c_int) {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn allocate_sfd(mut addr: *mut mysockaddr,
                                  mut intname: *mut libc::c_char)
 -> *mut serverfd {
    let mut sfd: *mut serverfd = 0 as *mut serverfd;
    let mut ifindex: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut errsave: libc::c_int = 0;
    let mut opt: libc::c_int = 1 as libc::c_int;
    /* when using random ports, servers which would otherwise use
     the INADDR_ANY/port0 socket have sfd set to NULL */
    if (*dnsmasq_daemon).osport == 0 &&
           *intname.offset(0 as libc::c_int as isize) as libc::c_int ==
               0 as libc::c_int {
        *__errno_location() =
            0 as
                libc::c_int; /* index == 0 when not binding to an interface */
        if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int &&
               (*addr).in_0.sin_addr.s_addr == 0 as libc::c_int as in_addr_t
               &&
               (*addr).in_0.sin_port as libc::c_int ==
                   __bswap_16(0 as libc::c_int as __uint16_t) as libc::c_int {
            return 0 as *mut serverfd
        }
        if (*addr).sa.sa_family as libc::c_int == 10 as libc::c_int &&
               memcmp(&mut (*addr).in6.sin6_addr as *mut in6_addr as
                          *const libc::c_void,
                      &in6addr_any as *const in6_addr as *const libc::c_void,
                      ::std::mem::size_of::<in6_addr>() as libc::c_ulong) ==
                   0 as libc::c_int &&
               (*addr).in6.sin6_port as libc::c_int ==
                   __bswap_16(0 as libc::c_int as __uint16_t) as libc::c_int {
            return 0 as *mut serverfd
        }
    }
    if !intname.is_null() &&
           strlen(intname) != 0 as libc::c_int as libc::c_ulong {
        ifindex = if_nametoindex(intname)
    }
    /* may have a suitable one already */
    sfd = (*dnsmasq_daemon).sfds;
    while !sfd.is_null() {
        if sockaddr_isequal(&mut (*sfd).source_addr, addr) != 0 &&
               strcmp(intname, (*sfd).interface.as_mut_ptr()) ==
                   0 as libc::c_int && ifindex == (*sfd).ifindex {
            return sfd
        }
        sfd = (*sfd).next
    }
    /* need to make a new one. */
    *__errno_location() = 12 as libc::c_int; /* in case malloc fails. */
    sfd =
        whine_malloc(::std::mem::size_of::<serverfd>() as libc::c_ulong) as
            *mut serverfd; /* save error from bind/setsockopt. */
    if sfd.is_null() { return 0 as *mut serverfd }
    (*sfd).fd =
        socket((*addr).sa.sa_family as libc::c_int, SOCK_DGRAM as libc::c_int,
               0 as libc::c_int);
    if (*sfd).fd == -(1 as libc::c_int) {
        free(sfd as *mut libc::c_void);
        return 0 as *mut serverfd
    }
    if (*addr).sa.sa_family as libc::c_int == 10 as libc::c_int &&
           setsockopt((*sfd).fd, IPPROTO_IPV6 as libc::c_int,
                      26 as libc::c_int,
                      &mut opt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) ||
           local_bind((*sfd).fd, addr, intname, ifindex, 0 as libc::c_int) ==
               0 || fix_fd((*sfd).fd) == 0 {
        errsave = *__errno_location();
        close((*sfd).fd);
        free(sfd as *mut libc::c_void);
        *__errno_location() = errsave;
        return 0 as *mut serverfd
    }
    safe_strncpy((*sfd).interface.as_mut_ptr(), intname,
                 ::std::mem::size_of::<[libc::c_char; 17]>() as
                     libc::c_ulong);
    (*sfd).source_addr = *addr;
    (*sfd).next = (*dnsmasq_daemon).sfds;
    (*sfd).ifindex = ifindex;
    (*sfd).preallocated = 0 as libc::c_int as libc::c_uint;
    (*dnsmasq_daemon).sfds = sfd;
    return sfd;
}
/* create upstream sockets during startup, before root is dropped which may be needed
   this allows query_port to be a low port and interface binding */
#[no_mangle]
pub unsafe extern "C" fn pre_allocate_sfds() {
    let mut srv: *mut server = 0 as *mut server;
    let mut sfd: *mut serverfd = 0 as *mut serverfd;
    if (*dnsmasq_daemon).query_port != 0 as libc::c_int {
        let mut addr: mysockaddr =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        addr.in_0.sin_family = 2 as libc::c_int as sa_family_t;
        addr.in_0.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
        addr.in_0.sin_port =
            __bswap_16((*dnsmasq_daemon).query_port as __uint16_t);
        sfd =
            allocate_sfd(&mut addr,
                         b"\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char);
        if !sfd.is_null() {
            (*sfd).preallocated = 1 as libc::c_int as libc::c_uint
        }
        memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        addr.in6.sin6_family = 10 as libc::c_int as sa_family_t;
        addr.in6.sin6_addr = in6addr_any;
        addr.in6.sin6_port =
            __bswap_16((*dnsmasq_daemon).query_port as __uint16_t);
        sfd =
            allocate_sfd(&mut addr,
                         b"\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char);
        if !sfd.is_null() {
            (*sfd).preallocated = 1 as libc::c_int as libc::c_uint
        }
    }
    srv = (*dnsmasq_daemon).servers;
    while !srv.is_null() {
        if (*srv).flags &
               (4 as libc::c_int | 2 as libc::c_int | 1024 as libc::c_int |
                    2048 as libc::c_int) == 0 &&
               allocate_sfd(&mut (*srv).source_addr,
                            (*srv).interface.as_mut_ptr()).is_null() &&
               *__errno_location() != 0 as libc::c_int &&
               (*dnsmasq_daemon).options[(13 as libc::c_int as
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
                   != 0 {
            prettyprint_addr(&mut (*srv).source_addr,
                             (*dnsmasq_daemon).namebuff);
            if (*srv).interface[0 as libc::c_int as usize] as libc::c_int !=
                   0 as libc::c_int {
                strcat((*dnsmasq_daemon).namebuff,
                       b" \x00" as *const u8 as *const libc::c_char);
                strcat((*dnsmasq_daemon).namebuff,
                       (*srv).interface.as_mut_ptr());
            }
            die(b"failed to bind server socket for %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).namebuff, 2 as libc::c_int);
        }
        srv = (*srv).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn mark_servers(mut flag: libc::c_int) {
    let mut serv: *mut server = 0 as *mut server;
    /* mark everything with argument flag */
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if (*serv).flags & flag != 0 { (*serv).flags |= 256 as libc::c_int }
        /* Give looped servers another chance */
        (*serv).flags &= !(8192 as libc::c_int);
        serv = (*serv).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn cleanup_servers() {
    let mut serv: *mut server = 0 as *mut server;
    let mut tmp: *mut server = 0 as *mut server;
    let mut up: *mut *mut server = 0 as *mut *mut server;
    /* unlink and free anything still marked. */
    serv = (*dnsmasq_daemon).servers;
    up = &mut (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        tmp = (*serv).next;
        if (*serv).flags & 256 as libc::c_int != 0 {
            server_gone(serv);
            *up = (*serv).next;
            if !(*serv).domain.is_null() {
                free((*serv).domain as *mut libc::c_void);
            }
            free(serv as *mut libc::c_void);
        } else { up = &mut (*serv).next }
        serv = tmp
    }
    /* Now we have a new set of servers, test for loops. */
    loop_send_probes();
}
#[no_mangle]
pub unsafe extern "C" fn add_update_server(mut flags: libc::c_int,
                                           mut addr: *mut mysockaddr,
                                           mut source_addr: *mut mysockaddr,
                                           mut interface: *const libc::c_char,
                                           mut domain: *const libc::c_char) {
    let mut serv: *mut server = 0 as *mut server;
    let mut next: *mut server = 0 as *mut server;
    let mut domain_str: *mut libc::c_char = 0 as *mut libc::c_char;
    /* See if there is a suitable candidate, and unmark */
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if (*serv).flags & 256 as libc::c_int != 0 {
            if !domain.is_null() {
                if !((*serv).flags & 8 as libc::c_int == 0 ||
                         hostname_isequal(domain, (*serv).domain) == 0) {
                    break ;
                }
            } else if !((*serv).flags & 8 as libc::c_int != 0) { break ; }
        }
        serv = (*serv).next
    }
    if !serv.is_null() {
        domain_str = (*serv).domain;
        next = (*serv).next
    } else {
        serv =
            whine_malloc(::std::mem::size_of::<server>() as libc::c_ulong) as
                *mut server;
        if !serv.is_null() {
            /* Not found, create a new one. */
            if !domain.is_null() &&
                   {
                       domain_str =
                           whine_malloc(strlen(domain).wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong))
                               as *mut libc::c_char;
                       domain_str.is_null()
                   } {
                free(serv as *mut libc::c_void);
                serv = 0 as *mut server
            } else {
                let mut s: *mut server = 0 as *mut server;
                /* Add to the end of the chain, for order */
                if (*dnsmasq_daemon).servers.is_null() {
                    (*dnsmasq_daemon).servers = serv
                } else {
                    s = (*dnsmasq_daemon).servers;
                    while !(*s).next.is_null() { s = (*s).next }
                    (*s).next = serv
                }
                if !domain.is_null() { strcpy(domain_str, domain); }
            }
        }
    }
    if !serv.is_null() {
        memset(serv as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<server>() as libc::c_ulong);
        (*serv).flags = flags;
        (*serv).domain = domain_str;
        (*serv).next = next;
        (*serv).failed_queries = 0 as libc::c_int as libc::c_uint;
        (*serv).queries = (*serv).failed_queries;
        (*serv).uid = rand32();
        if !domain.is_null() { (*serv).flags |= 8 as libc::c_int }
        if !interface.is_null() {
            safe_strncpy((*serv).interface.as_mut_ptr(), interface,
                         ::std::mem::size_of::<[libc::c_char; 17]>() as
                             libc::c_ulong);
        }
        if !addr.is_null() { (*serv).addr = *addr }
        if !source_addr.is_null() { (*serv).source_addr = *source_addr }
    };
}
#[no_mangle]
pub unsafe extern "C" fn check_servers() {
    let mut iface: *mut irec = 0 as *mut irec;
    let mut serv: *mut server = 0 as *mut server;
    let mut sfd: *mut serverfd = 0 as *mut serverfd;
    let mut tmp: *mut serverfd = 0 as *mut serverfd;
    let mut up: *mut *mut serverfd = 0 as *mut *mut serverfd;
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0;
    let mut locals: libc::c_int = 0 as libc::c_int;
    /* interface may be new since startup */
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
           == 0 {
        enumerate_interfaces(0 as libc::c_int);
    }
    /* don't garbage collect pre-allocated sfds. */
    sfd = (*dnsmasq_daemon).sfds;
    while !sfd.is_null() {
        (*sfd).used = (*sfd).preallocated;
        sfd = (*sfd).next
    }
    let mut current_block_37: u64;
    count = 0 as libc::c_int;
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if (*serv).flags &
               (4 as libc::c_int | 2 as libc::c_int | 1024 as libc::c_int |
                    2048 as libc::c_int) == 0 {
            /* Init edns_pktsz for newly created server records. */
            if (*serv).edns_pktsz == 0 as libc::c_int {
                (*serv).edns_pktsz =
                    (*dnsmasq_daemon).edns_pktsz as libc::c_int
            }
            port =
                prettyprint_addr(&mut (*serv).addr,
                                 (*dnsmasq_daemon).namebuff);
            /* 0.0.0.0 is nothing, the stack treats it like 127.0.0.1 */
            if (*serv).addr.sa.sa_family as libc::c_int == 2 as libc::c_int &&
                   (*serv).addr.in_0.sin_addr.s_addr ==
                       0 as libc::c_int as libc::c_uint {
                (*serv).flags |= 256 as libc::c_int;
                current_block_37 = 8515828400728868193;
            } else {
                iface = (*dnsmasq_daemon).interfaces;
                while !iface.is_null() {
                    if sockaddr_isequal(&mut (*serv).addr, &mut (*iface).addr)
                           != 0 {
                        break ;
                    }
                    iface = (*iface).next
                }
                if !iface.is_null() {
                    my_syslog(4 as libc::c_int,
                              b"ignoring nameserver %s - local interface\x00"
                                  as *const u8 as *const libc::c_char,
                              (*dnsmasq_daemon).namebuff);
                    (*serv).flags |= 256 as libc::c_int;
                    current_block_37 = 8515828400728868193;
                } else if (*serv).sfd.is_null() &&
                              {
                                  (*serv).sfd =
                                      allocate_sfd(&mut (*serv).source_addr,
                                                   (*serv).interface.as_mut_ptr());
                                  (*serv).sfd.is_null()
                              } && *__errno_location() != 0 as libc::c_int {
                    my_syslog(4 as libc::c_int,
                              b"ignoring nameserver %s - cannot make/bind socket: %s\x00"
                                  as *const u8 as *const libc::c_char,
                              (*dnsmasq_daemon).namebuff,
                              strerror(*__errno_location()));
                    (*serv).flags |= 256 as libc::c_int;
                    current_block_37 = 8515828400728868193;
                } else {
                    if !(*serv).sfd.is_null() {
                        (*(*serv).sfd).used = 1 as libc::c_int as libc::c_uint
                    }
                    current_block_37 = 3437258052017859086;
                }
            }
        } else { current_block_37 = 3437258052017859086; }
        match current_block_37 {
            3437258052017859086 => {
                if (*serv).flags & 2048 as libc::c_int == 0 &&
                       (*serv).flags & 4 as libc::c_int == 0 {
                    count += 1;
                    if !(count > 30 as libc::c_int) {
                        if (*serv).flags &
                               (8 as libc::c_int | 32 as libc::c_int |
                                    1024 as libc::c_int) != 0 {
                            let mut s1: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut s2: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut s3: *mut libc::c_char =
                                b"\x00" as *const u8 as *const libc::c_char as
                                    *mut libc::c_char;
                            if (*serv).flags & 8 as libc::c_int == 0 {
                                s1 =
                                    b"unqualified\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char;
                                s2 =
                                    b"names\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char
                            } else if strlen((*serv).domain) ==
                                          0 as libc::c_int as libc::c_ulong {
                                s1 =
                                    b"default\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char;
                                s2 =
                                    b"\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char
                            } else {
                                s1 =
                                    b"domain\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char;
                                s2 = (*serv).domain
                            }
                            if (*serv).flags & 2 as libc::c_int != 0 {
                                count -= 1;
                                locals += 1;
                                if locals <= 8 as libc::c_int {
                                    my_syslog(6 as libc::c_int,
                                              b"using only locally-known addresses for %s %s\x00"
                                                  as *const u8 as
                                                  *const libc::c_char, s1,
                                              s2);
                                }
                            } else if (*serv).flags & 1024 as libc::c_int != 0
                             {
                                my_syslog(6 as libc::c_int,
                                          b"using standard nameservers for %s %s\x00"
                                              as *const u8 as
                                              *const libc::c_char, s1, s2);
                            } else {
                                my_syslog(6 as libc::c_int,
                                          b"using nameserver %s#%d for %s %s %s\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          (*dnsmasq_daemon).namebuff, port,
                                          s1, s2, s3);
                            }
                        } else if (*serv).flags & 8192 as libc::c_int != 0 {
                            my_syslog(6 as libc::c_int,
                                      b"NOT using nameserver %s#%d - query loop detected\x00"
                                          as *const u8 as *const libc::c_char,
                                      (*dnsmasq_daemon).namebuff, port);
                        } else if (*serv).interface[0 as libc::c_int as usize]
                                      as libc::c_int != 0 as libc::c_int {
                            my_syslog(6 as libc::c_int,
                                      b"using nameserver %s#%d(via %s)\x00" as
                                          *const u8 as *const libc::c_char,
                                      (*dnsmasq_daemon).namebuff, port,
                                      (*serv).interface.as_mut_ptr());
                        } else {
                            my_syslog(6 as libc::c_int,
                                      b"using nameserver %s#%d\x00" as
                                          *const u8 as *const libc::c_char,
                                      (*dnsmasq_daemon).namebuff, port);
                        }
                    }
                }
            }
            _ => { }
        }
        serv = (*serv).next
    }
    if locals > 8 as libc::c_int {
        my_syslog(6 as libc::c_int,
                  b"using %d more local addresses\x00" as *const u8 as
                      *const libc::c_char, locals - 8 as libc::c_int);
    }
    if count - 1 as libc::c_int > 30 as libc::c_int {
        my_syslog(6 as libc::c_int,
                  b"using %d more nameservers\x00" as *const u8 as
                      *const libc::c_char,
                  count - 30 as libc::c_int - 1 as libc::c_int);
    }
    /* Do we need a socket set? */
    /* Remove unused sfds */
    sfd = (*dnsmasq_daemon).sfds;
    up = &mut (*dnsmasq_daemon).sfds;
    while !sfd.is_null() {
        tmp = (*sfd).next;
        if (*sfd).used == 0 {
            *up = (*sfd).next;
            close((*sfd).fd);
            free(sfd as *mut libc::c_void);
        } else { up = &mut (*sfd).next }
        sfd = tmp
    }
    cleanup_servers();
}
/* Return zero if no servers found, in that case we keep polling.
   This is a protection against an update-time/write race on resolv.conf */
#[no_mangle]
pub unsafe extern "C" fn reload_servers(mut fname: *mut libc::c_char)
 -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gotone: libc::c_int = 0 as libc::c_int;
    /* buff happens to be MAXDNAME long... */
    f = fopen(fname, b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        my_syslog(3 as libc::c_int,
                  b"failed to read %s: %s\x00" as *const u8 as
                      *const libc::c_char, fname,
                  strerror(*__errno_location()));
        return 0 as libc::c_int
    }
    mark_servers(1 as libc::c_int);
    loop  {
        line = fgets((*dnsmasq_daemon).namebuff, 1025 as libc::c_int, f);
        if line.is_null() { break ; }
        let mut addr: mysockaddr =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        let mut source_addr: mysockaddr =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        let mut token: *mut libc::c_char =
            strtok(line, b" \t\n\r\x00" as *const u8 as *const libc::c_char);
        if token.is_null() { continue ; }
        if strcmp(token,
                  b"nameserver\x00" as *const u8 as *const libc::c_char) !=
               0 as libc::c_int &&
               strcmp(token,
                      b"server\x00" as *const u8 as *const libc::c_char) !=
                   0 as libc::c_int {
            continue ;
        }
        token =
            strtok(0 as *mut libc::c_char,
                   b" \t\n\r\x00" as *const u8 as *const libc::c_char);
        if token.is_null() { continue ; }
        memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        memset(&mut source_addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        addr.in_0.sin_addr.s_addr = inet_addr(token);
        if addr.in_0.sin_addr.s_addr != -(1 as libc::c_int) as in_addr_t {
            addr.in_0.sin_family = 2 as libc::c_int as sa_family_t;
            source_addr.in_0.sin_family = addr.in_0.sin_family;
            addr.in_0.sin_port = __bswap_16(53 as libc::c_int as __uint16_t);
            source_addr.in_0.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
            source_addr.in_0.sin_port =
                __bswap_16((*dnsmasq_daemon).query_port as __uint16_t)
        } else {
            let mut scope_index: libc::c_int = 0 as libc::c_int;
            let mut scope_id: *mut libc::c_char = strchr(token, '%' as i32);
            if !scope_id.is_null() {
                let fresh8 = scope_id;
                scope_id = scope_id.offset(1);
                *fresh8 = 0 as libc::c_int as libc::c_char;
                scope_index = if_nametoindex(scope_id) as libc::c_int
            }
            if !(inet_pton(10 as libc::c_int, token,
                           &mut addr.in6.sin6_addr as *mut in6_addr as
                               *mut libc::c_void) > 0 as libc::c_int) {
                continue ;
            }
            addr.in6.sin6_family = 10 as libc::c_int as sa_family_t;
            source_addr.in6.sin6_family = addr.in6.sin6_family;
            addr.in6.sin6_flowinfo = 0 as libc::c_int as uint32_t;
            source_addr.in6.sin6_flowinfo = addr.in6.sin6_flowinfo;
            addr.in6.sin6_port = __bswap_16(53 as libc::c_int as __uint16_t);
            addr.in6.sin6_scope_id = scope_index as uint32_t;
            source_addr.in6.sin6_addr = in6addr_any;
            source_addr.in6.sin6_port =
                __bswap_16((*dnsmasq_daemon).query_port as __uint16_t);
            source_addr.in6.sin6_scope_id = 0 as libc::c_int as uint32_t
        }
        add_update_server(1 as libc::c_int, &mut addr, &mut source_addr,
                          0 as *const libc::c_char, 0 as *const libc::c_char);
        gotone = 1 as libc::c_int
    }
    fclose(f);
    cleanup_servers();
    return gotone;
}
/* Called when addresses are added or deleted from an interface */
#[no_mangle]
pub unsafe extern "C" fn newaddress(mut now: time_t) {
    if (*dnsmasq_daemon).options[(39 as libc::c_int as
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
           != 0 ||
           (*dnsmasq_daemon).options[(49 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (49 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 || (*dnsmasq_daemon).doing_dhcp6 != 0 ||
           !(*dnsmasq_daemon).relay6.is_null() ||
           (*dnsmasq_daemon).doing_ra != 0 {
        enumerate_interfaces(0 as libc::c_int);
    }
    if (*dnsmasq_daemon).options[(39 as libc::c_int as
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
        create_bound_listeners(0 as libc::c_int);
    }
    if (*dnsmasq_daemon).doing_dhcp6 != 0 ||
           !(*dnsmasq_daemon).relay6.is_null() ||
           (*dnsmasq_daemon).doing_ra != 0 {
        join_multicast(0 as libc::c_int);
    }
    if (*dnsmasq_daemon).doing_dhcp6 != 0 || (*dnsmasq_daemon).doing_ra != 0 {
        dhcp_construct_contexts(now);
    }
    if (*dnsmasq_daemon).doing_dhcp6 != 0 { lease_find_interfaces(now); };
}
