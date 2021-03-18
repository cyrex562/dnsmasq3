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
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
                  __delimiter: libc::c_int, __stream: *mut FILE) -> __ssize_t;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int,
              __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
               __len: socklen_t) -> libc::c_int;
    #[no_mangle]
    fn getpeername(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                   __len: *mut socklen_t) -> libc::c_int;
    #[no_mangle]
    fn sendto(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t,
              __flags: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
              __addr_len: socklen_t) -> ssize_t;
    #[no_mangle]
    fn recvfrom(__fd: libc::c_int, __buf: *mut libc::c_void, __n: size_t,
                __flags: libc::c_int, __addr: __SOCKADDR_ARG,
                __addr_len: *mut socklen_t) -> ssize_t;
    #[no_mangle]
    fn sendmsg(__fd: libc::c_int, __message: *const msghdr,
               __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn recvmsg(__fd: libc::c_int, __message: *mut msghdr,
               __flags: libc::c_int) -> ssize_t;
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
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
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
    fn check_for_local_domain(name: *mut libc::c_char, now: time_t)
     -> libc::c_int;
    #[no_mangle]
    fn resize_packet(header: *mut dns_header, plen: size_t,
                     pheader: *mut libc::c_uchar, hlen: size_t) -> size_t;
    #[no_mangle]
    fn answer_auth(header: *mut dns_header, limit: *mut libc::c_char,
                   qlen: size_t, now: time_t, peer_addr: *mut mysockaddr,
                   local_query: libc::c_int, do_bit: libc::c_int,
                   have_pseudoheader: libc::c_int) -> size_t;
    #[no_mangle]
    fn in_zone(zone: *mut auth_zone, name: *mut libc::c_char,
               cut: *mut *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn check_for_bogus_wildcard(header: *mut dns_header, qlen: size_t,
                                name: *mut libc::c_char,
                                baddr: *mut bogus_addr, now: time_t)
     -> libc::c_int;
    #[no_mangle]
    fn hash_questions(header: *mut dns_header, plen: size_t,
                      name: *mut libc::c_char) -> *mut libc::c_uchar;
    #[no_mangle]
    fn rand16() -> libc::c_ushort;
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
    fn is_same_net(a: in_addr, b: in_addr, mask: in_addr) -> libc::c_int;
    #[no_mangle]
    fn is_same_net6(a: *mut in6_addr, b: *mut in6_addr,
                    prefixlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn retry_send(rc: ssize_t) -> libc::c_int;
    #[no_mangle]
    fn prettyprint_addr(addr: *mut mysockaddr, buf: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn read_write(fd: libc::c_int, packet: *mut libc::c_uchar,
                  size: libc::c_int, rw: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn my_syslog(priority: libc::c_int, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn check_log_writer(force: libc::c_int);
    #[no_mangle]
    fn answer_request(header: *mut dns_header, limit: *mut libc::c_char,
                      qlen: size_t, local_addr: in_addr,
                      local_netmask: in_addr, now: time_t,
                      ad_reqd: libc::c_int, do_bit: libc::c_int,
                      have_pseudoheader: libc::c_int) -> size_t;
    #[no_mangle]
    fn extract_addresses(header: *mut dns_header, qlen: size_t,
                         name: *mut libc::c_char, now: time_t,
                         ipsets: *mut *mut libc::c_char, is_sign: libc::c_int,
                         check_rebind: libc::c_int,
                         no_cache_dnssec: libc::c_int, secure: libc::c_int,
                         doctored: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn setup_reply(header: *mut dns_header, qlen: size_t,
                   addrp: *mut all_addr, flags: libc::c_uint,
                   ttl: libc::c_ulong) -> size_t;
    #[no_mangle]
    fn extract_request(header: *mut dns_header, qlen: size_t,
                       name: *mut libc::c_char, typep: *mut libc::c_ushort)
     -> libc::c_uint;
    #[no_mangle]
    fn querystr(desc: *mut libc::c_char, type_0: libc::c_ushort)
     -> *mut libc::c_char;
    #[no_mangle]
    fn log_query(flags: libc::c_uint, name: *mut libc::c_char,
                 addr: *mut all_addr, arg: *mut libc::c_char);
    #[no_mangle]
    static mut dnsmasq_daemon: *mut dnsmasq_daemon;
    #[no_mangle]
    fn random_sock(family: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn check_for_ignored_address(header: *mut dns_header, qlen: size_t,
                                 baddr: *mut bogus_addr) -> libc::c_int;
    #[no_mangle]
    fn enumerate_interfaces(reset: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn label_exception(index: libc::c_int, family: libc::c_int,
                       addr: *mut all_addr) -> libc::c_int;
    #[no_mangle]
    fn loopback_exception(fd: libc::c_int, family: libc::c_int,
                          addr: *mut all_addr, name: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn iface_check(family: libc::c_int, addr: *mut all_addr,
                   name: *mut libc::c_char, auth: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn indextoname(fd: libc::c_int, index: libc::c_int,
                   name: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn local_bind(fd: libc::c_int, addr: *mut mysockaddr,
                  intname: *mut libc::c_char, ifindex: libc::c_uint,
                  is_tcp: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn detect_loop(query: *mut libc::c_char, type_0: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn find_pseudoheader(header: *mut dns_header, plen: size_t,
                         len: *mut size_t, p: *mut *mut libc::c_uchar,
                         is_sign: *mut libc::c_int, is_last: *mut libc::c_int)
     -> *mut libc::c_uchar;
    #[no_mangle]
    fn add_do_bit(header: *mut dns_header, plen: size_t,
                  limit: *mut libc::c_uchar) -> size_t;
    #[no_mangle]
    fn add_edns0_config(header: *mut dns_header, plen: size_t,
                        limit: *mut libc::c_uchar, source: *mut mysockaddr,
                        now: time_t, check_subnet: *mut libc::c_int,
                        cacheable: *mut libc::c_int) -> size_t;
    #[no_mangle]
    fn add_pseudoheader(header: *mut dns_header, plen: size_t,
                        limit: *mut libc::c_uchar, udp_sz: libc::c_ushort,
                        optno: libc::c_int, opt: *mut libc::c_uchar,
                        optlen: size_t, set_do: libc::c_int,
                        replace: libc::c_int) -> size_t;
    #[no_mangle]
    fn rrfilter(header: *mut dns_header, plen: size_t, mode: libc::c_int)
     -> size_t;
    #[no_mangle]
    fn check_source(header: *mut dns_header, plen: size_t,
                    pseudoheader: *mut libc::c_uchar, peer: *mut mysockaddr)
     -> libc::c_int;
    #[no_mangle]
    fn dump_packet(mask: libc::c_int, packet: *mut libc::c_void, len: size_t,
                   src: *mut mysockaddr, dst: *mut mysockaddr);
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
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_TRYHARD: C2RustUnnamed = 4;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
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
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_1 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_1 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_1 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_1 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_1 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_1 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_1 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_1 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_1 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_1 = 92;
pub const IPPROTO_AH: C2RustUnnamed_1 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_1 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_1 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_1 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_1 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_1 = 33;
pub const IPPROTO_TP: C2RustUnnamed_1 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_1 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_1 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_1 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_1 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_1 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_1 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_1 = 1;
pub const IPPROTO_IP: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_pktinfo {
    pub ipi6_addr: in6_addr,
    pub ipi6_ifindex: libc::c_uint,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const __METRIC_MAX: C2RustUnnamed_2 = 20;
pub const METRIC_LEASES_PRUNED_6: C2RustUnnamed_2 = 19;
pub const METRIC_LEASES_ALLOCATED_6: C2RustUnnamed_2 = 18;
pub const METRIC_LEASES_PRUNED_4: C2RustUnnamed_2 = 17;
pub const METRIC_LEASES_ALLOCATED_4: C2RustUnnamed_2 = 16;
pub const METRIC_NOANSWER: C2RustUnnamed_2 = 15;
pub const METRIC_DHCPREQUEST: C2RustUnnamed_2 = 14;
pub const METRIC_DHCPRELEASE: C2RustUnnamed_2 = 13;
pub const METRIC_DHCPOFFER: C2RustUnnamed_2 = 12;
pub const METRIC_DHCPNAK: C2RustUnnamed_2 = 11;
pub const METRIC_DHCPINFORM: C2RustUnnamed_2 = 10;
pub const METRIC_DHCPDISCOVER: C2RustUnnamed_2 = 9;
pub const METRIC_DHCPDECLINE: C2RustUnnamed_2 = 8;
pub const METRIC_DHCPACK: C2RustUnnamed_2 = 7;
pub const METRIC_PXE: C2RustUnnamed_2 = 6;
pub const METRIC_BOOTP: C2RustUnnamed_2 = 5;
pub const METRIC_DNS_LOCAL_ANSWERED: C2RustUnnamed_2 = 4;
pub const METRIC_DNS_AUTH_ANSWERED: C2RustUnnamed_2 = 3;
pub const METRIC_DNS_QUERIES_FORWARDED: C2RustUnnamed_2 = 2;
pub const METRIC_DNS_CACHE_LIVE_FREED: C2RustUnnamed_2 = 1;
pub const METRIC_DNS_CACHE_INSERTED: C2RustUnnamed_2 = 0;
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type u32_0 = libc::c_uint;
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
    pub ifr_ifrn: C2RustUnnamed_4,
    pub ifr_ifru: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
pub union C2RustUnnamed_4 {
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
    pub cname: C2RustUnnamed_9,
    pub key: C2RustUnnamed_8,
    pub ds: C2RustUnnamed_7,
    pub srv: C2RustUnnamed_6,
    pub log: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub keytag: libc::c_ushort,
    pub algo: libc::c_ushort,
    pub digest: libc::c_ushort,
    pub rcode: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
pub struct C2RustUnnamed_7 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub flags: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub target: C2RustUnnamed_10,
    pub uid: libc::c_uint,
    pub is_name_ptr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
    pub name: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
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
    pub u: C2RustUnnamed_12,
    pub val: *mut libc::c_uchar,
    pub netid: *mut dhcp_netid,
    pub next: *mut dhcp_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
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
pub union C2RustUnnamed_13 {
    pub align: cmsghdr,
    pub control: [libc::c_char; 32],
    pub control6: [libc::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub c: *mut libc::c_uchar,
    pub p: *mut in6_pktinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub c: *mut libc::c_uchar,
    pub p: *mut in_pktinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub align: cmsghdr,
    pub control6: [libc::c_char; 40],
    pub control: [libc::c_char; 32],
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
/* Send a UDP packet with its source address set as "source" 
   unless nowild is true, when we just send it with the kernel default */
#[no_mangle]
pub unsafe extern "C" fn send_from(mut fd: libc::c_int,
                                   mut nowild: libc::c_int,
                                   mut packet: *mut libc::c_char,
                                   mut len: size_t, mut to: *mut mysockaddr,
                                   mut source: *mut all_addr,
                                   mut iface: libc::c_uint) -> libc::c_int {
    let mut msg: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags:
                   0,}; /* Need iface for IPv6 to handle link-local addrs */
    let mut iov: [iovec; 1] =
        [iovec{iov_base: 0 as *mut libc::c_void, iov_len: 0,}; 1];
    let mut control_u: C2RustUnnamed_13 =
        C2RustUnnamed_13{align:
                             cmsghdr{cmsg_len: 0,
                                     cmsg_level: 0,
                                     cmsg_type: 0,
                                     __cmsg_data: [],},};
    iov[0 as libc::c_int as usize].iov_base = packet as *mut libc::c_void;
    iov[0 as libc::c_int as usize].iov_len = len;
    msg.msg_control = 0 as *mut libc::c_void;
    msg.msg_controllen = 0 as libc::c_int as size_t;
    msg.msg_flags = 0 as libc::c_int;
    msg.msg_name = to as *mut libc::c_void;
    msg.msg_namelen = sa_len(to) as socklen_t;
    msg.msg_iov = iov.as_mut_ptr();
    msg.msg_iovlen = 1 as libc::c_int as size_t;
    if nowild == 0 {
        let mut cmptr: *mut cmsghdr = 0 as *mut cmsghdr;
        msg.msg_control =
            &mut control_u as *mut C2RustUnnamed_13 as *mut libc::c_void;
        msg.msg_controllen =
            ::std::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong;
        cmptr =
            if msg.msg_controllen >=
                   ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                msg.msg_control as *mut cmsghdr
            } else { 0 as *mut cmsghdr };
        if (*to).sa.sa_family as libc::c_int == 2 as libc::c_int {
            let mut p: in_pktinfo =
                in_pktinfo{ipi_ifindex: 0,
                           ipi_spec_dst: in_addr{s_addr: 0,},
                           ipi_addr: in_addr{s_addr: 0,},};
            p.ipi_ifindex = 0 as libc::c_int;
            p.ipi_spec_dst = (*source).addr4;
            msg.msg_controllen =
                ((::std::mem::size_of::<in_pktinfo>() as
                      libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>()
                                                      as
                                                      libc::c_ulong).wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)
                     &
                     !(::std::mem::size_of::<size_t>() as
                           libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong)).wrapping_add((::std::mem::size_of::<cmsghdr>()
                                                                                             as
                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>()
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
                                                                                                                                  libc::c_ulong));
            memcpy((*cmptr).__cmsg_data.as_mut_ptr() as *mut libc::c_void,
                   &mut p as *mut in_pktinfo as *const libc::c_void,
                   ::std::mem::size_of::<in_pktinfo>() as libc::c_ulong);
            (*cmptr).cmsg_len =
                ((::std::mem::size_of::<cmsghdr>() as
                      libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>()
                                                      as
                                                      libc::c_ulong).wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)
                     &
                     !(::std::mem::size_of::<size_t>() as
                           libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong)).wrapping_add(::std::mem::size_of::<in_pktinfo>()
                                                                                            as
                                                                                            libc::c_ulong);
            (*cmptr).cmsg_level = IPPROTO_IP as libc::c_int;
            (*cmptr).cmsg_type = 8 as libc::c_int
        } else {
            let mut p_0: in6_pktinfo =
                in6_pktinfo{ipi6_addr:
                                in6_addr{__in6_u:
                                             C2RustUnnamed_0{__u6_addr8:
                                                                 [0; 16],},},
                            ipi6_ifindex: 0,};
            p_0.ipi6_ifindex = iface;
            p_0.ipi6_addr = (*source).addr6;
            msg.msg_controllen =
                ((::std::mem::size_of::<in6_pktinfo>() as
                      libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>()
                                                      as
                                                      libc::c_ulong).wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)
                     &
                     !(::std::mem::size_of::<size_t>() as
                           libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong)).wrapping_add((::std::mem::size_of::<cmsghdr>()
                                                                                             as
                                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>()
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
                                                                                                                                  libc::c_ulong));
            memcpy((*cmptr).__cmsg_data.as_mut_ptr() as *mut libc::c_void,
                   &mut p_0 as *mut in6_pktinfo as *const libc::c_void,
                   ::std::mem::size_of::<in6_pktinfo>() as libc::c_ulong);
            (*cmptr).cmsg_len =
                ((::std::mem::size_of::<cmsghdr>() as
                      libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>()
                                                      as
                                                      libc::c_ulong).wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong)
                     &
                     !(::std::mem::size_of::<size_t>() as
                           libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong)).wrapping_add(::std::mem::size_of::<in6_pktinfo>()
                                                                                            as
                                                                                            libc::c_ulong);
            (*cmptr).cmsg_type = (*dnsmasq_daemon).v6pktinfo;
            (*cmptr).cmsg_level = IPPROTO_IPV6 as libc::c_int
        }
    }
    while retry_send(sendmsg(fd, &mut msg, 0 as libc::c_int)) != 0 { }
    if *__errno_location() != 0 as libc::c_int {
        /* If interface is still in DAD, EINVAL results - ignore that. */
        if *__errno_location() != 22 as libc::c_int {
            my_syslog(3 as libc::c_int,
                      b"failed to send packet: %s\x00" as *const u8 as
                          *const libc::c_char, strerror(*__errno_location()));
        }
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn search_servers(mut now: time_t,
                                    mut addrpp: *mut *mut all_addr,
                                    mut qtype: libc::c_uint,
                                    mut qdomain: *mut libc::c_char,
                                    mut type_0: *mut libc::c_int,
                                    mut domain: *mut *mut libc::c_char,
                                    mut norebind: *mut libc::c_int)
 -> libc::c_uint {
    /* If the query ends in the domain in one of our servers, set
     domain to point to that name. We find the largest match to allow both
     domain.org and sub.domain.org to exist. */
    let mut namelen: libc::c_uint = strlen(qdomain) as libc::c_uint;
    let mut matchlen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut serv: *mut server = 0 as *mut server;
    let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    static mut zero: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut current_block_45: u64;
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if !(qtype == (1 as libc::c_uint) << 15 as libc::c_int &&
                 (*serv).flags & 16384 as libc::c_int == 0) {
            /* domain matches take priority over NODOTS matches */
            if (*serv).flags & 32 as libc::c_int != 0 &&
                   *type_0 != 8 as libc::c_int &&
                   strchr(qdomain, '.' as i32).is_null() &&
                   namelen != 0 as libc::c_int as libc::c_uint {
                let mut sflag: libc::c_uint =
                    if (*serv).addr.sa.sa_family as libc::c_int ==
                           2 as libc::c_int {
                        ((1 as libc::c_uint)) << 7 as libc::c_int
                    } else { ((1 as libc::c_uint)) << 8 as libc::c_int };
                *type_0 = 32 as libc::c_int;
                if (*serv).flags & 2048 as libc::c_int != 0 &&
                       !norebind.is_null() {
                    *norebind = 1 as libc::c_int
                } else if (*serv).flags & 2 as libc::c_int != 0 {
                    flags = (1 as libc::c_uint) << 10 as libc::c_int
                } else if (*serv).flags & 4 as libc::c_int != 0 {
                    /* literal address = '#' -> return all-zero address for IPv4 and IPv6 */
                    if (*serv).flags & 1024 as libc::c_int != 0 &&
                           qtype &
                               ((1 as libc::c_uint) << 8 as libc::c_int |
                                    (1 as libc::c_uint) << 7 as libc::c_int)
                               != 0 {
                        memset(&mut zero as *mut all_addr as
                                   *mut libc::c_void, 0 as libc::c_int,
                               ::std::mem::size_of::<all_addr>() as
                                   libc::c_ulong);
                        flags = qtype;
                        *addrpp = &mut zero
                    } else if sflag & qtype != 0 {
                        flags = sflag;
                        if (*serv).addr.sa.sa_family as libc::c_int ==
                               2 as libc::c_int {
                            *addrpp =
                                &mut (*serv).addr.in_0.sin_addr as
                                    *mut in_addr as *mut all_addr
                        } else {
                            *addrpp =
                                &mut (*serv).addr.in6.sin6_addr as
                                    *mut in6_addr as *mut all_addr
                        }
                    } else if flags == 0 ||
                                  flags &
                                      (1 as libc::c_uint) << 10 as libc::c_int
                                      != 0 {
                        flags = (1 as libc::c_uint) << 20 as libc::c_int
                    }
                }
            } else if (*serv).flags & 8 as libc::c_int != 0 {
                let mut domainlen: libc::c_uint =
                    strlen((*serv).domain) as libc::c_uint;
                let mut matchstart: *mut libc::c_char =
                    qdomain.offset(namelen as
                                       isize).offset(-(domainlen as isize));
                if namelen >= domainlen &&
                       hostname_isequal(matchstart, (*serv).domain) != 0 &&
                       (domainlen == 0 as libc::c_int as libc::c_uint ||
                            namelen == domainlen ||
                            *matchstart.offset(-(1 as libc::c_int as isize))
                                as libc::c_int == '.' as i32) {
                    if (*serv).flags & 2048 as libc::c_int != 0 &&
                           !norebind.is_null() {
                        *norebind = 1 as libc::c_int
                    } else {
                        let mut sflag_0: libc::c_uint =
                            if (*serv).addr.sa.sa_family as libc::c_int ==
                                   2 as libc::c_int {
                                ((1 as libc::c_uint)) << 7 as libc::c_int
                            } else {
                                ((1 as libc::c_uint)) << 8 as libc::c_int
                            };
                        /* implement priority rules for --address and --server for same domain.
		   --address wins if the address is for the correct AF
		   --server wins otherwise. */
                        if domainlen != 0 as libc::c_int as libc::c_uint &&
                               domainlen == matchlen {
                            if (*serv).flags & 4 as libc::c_int != 0 {
                                if sflag_0 & qtype == 0 &&
                                       flags ==
                                           0 as libc::c_int as libc::c_uint {
                                    current_block_45 = 4644295000439058019;
                                } else {
                                    current_block_45 = 6450636197030046351;
                                }
                            } else if flags &
                                          ((1 as libc::c_uint) <<
                                               7 as libc::c_int |
                                               (1 as libc::c_uint) <<
                                                   8 as libc::c_int) != 0 {
                                current_block_45 = 4644295000439058019;
                            } else { current_block_45 = 6450636197030046351; }
                        } else { current_block_45 = 6450636197030046351; }
                        match current_block_45 {
                            4644295000439058019 => { }
                            _ => {
                                if domainlen >= matchlen {
                                    *type_0 =
                                        (*serv).flags &
                                            (8 as libc::c_int |
                                                 1024 as libc::c_int |
                                                 2048 as libc::c_int |
                                                 16384 as libc::c_int);
                                    *domain = (*serv).domain;
                                    matchlen = domainlen;
                                    if (*serv).flags & 2 as libc::c_int != 0 {
                                        flags =
                                            (1 as libc::c_uint) <<
                                                10 as libc::c_int
                                    } else if (*serv).flags & 4 as libc::c_int
                                                  != 0 {
                                        /* literal address = '#' -> return all-zero address for IPv4 and IPv6 */
                                        if (*serv).flags & 1024 as libc::c_int
                                               != 0 &&
                                               qtype &
                                                   ((1 as libc::c_uint) <<
                                                        8 as libc::c_int |
                                                        (1 as libc::c_uint) <<
                                                            7 as libc::c_int)
                                                   != 0 {
                                            memset(&mut zero as *mut all_addr
                                                       as *mut libc::c_void,
                                                   0 as libc::c_int,
                                                   ::std::mem::size_of::<all_addr>()
                                                       as libc::c_ulong);
                                            flags = qtype;
                                            *addrpp = &mut zero
                                        } else if sflag_0 & qtype != 0 {
                                            flags = sflag_0;
                                            if (*serv).addr.sa.sa_family as
                                                   libc::c_int ==
                                                   2 as libc::c_int {
                                                *addrpp =
                                                    &mut (*serv).addr.in_0.sin_addr
                                                        as *mut in_addr as
                                                        *mut all_addr
                                            } else {
                                                *addrpp =
                                                    &mut (*serv).addr.in6.sin6_addr
                                                        as *mut in6_addr as
                                                        *mut all_addr
                                            }
                                        } else if flags == 0 ||
                                                      flags &
                                                          (1 as libc::c_uint)
                                                              <<
                                                              10 as
                                                                  libc::c_int
                                                          != 0 {
                                            flags =
                                                (1 as libc::c_uint) <<
                                                    20 as libc::c_int
                                        }
                                    } else {
                                        flags =
                                            0 as libc::c_int as libc::c_uint
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        serv = (*serv).next
    }
    if flags == 0 as libc::c_int as libc::c_uint &&
           qtype &
               ((1 as libc::c_uint) << 19 as libc::c_int |
                    (1 as libc::c_uint) << 15 as libc::c_int) == 0 &&
           (*dnsmasq_daemon).options[(12 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (12 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 && strchr(qdomain, '.' as i32).is_null() &&
           namelen != 0 as libc::c_int as libc::c_uint {
        /* don't forward A or AAAA queries for simple names, except the empty name */
        flags = (1 as libc::c_uint) << 20 as libc::c_int
    }
    if flags == (1 as libc::c_uint) << 10 as libc::c_int &&
           check_for_local_domain(qdomain, now) != 0 {
        flags = (1 as libc::c_uint) << 20 as libc::c_int
    }
    if flags != 0 {
        if flags == (1 as libc::c_uint) << 10 as libc::c_int ||
               flags == (1 as libc::c_uint) << 20 as libc::c_int {
            log_query(flags | qtype | (1 as libc::c_uint) << 5 as libc::c_int
                          | (1 as libc::c_uint) << 13 as libc::c_int |
                          (1 as libc::c_uint) << 3 as libc::c_int, qdomain,
                      0 as *mut all_addr, 0 as *mut libc::c_char);
        } else {
            /* handle F_IPV4 and F_IPV6 set on ANY query to 0.0.0.0/:: domain. */
            if flags & (1 as libc::c_uint) << 7 as libc::c_int != 0 {
                log_query((flags | (1 as libc::c_uint) << 13 as libc::c_int |
                               (1 as libc::c_uint) << 3 as libc::c_int) &
                              !((1 as libc::c_uint) << 8 as libc::c_int),
                          qdomain, *addrpp,
                          0 as
                              *mut libc::c_char); /* use normal servers for this domain */
            }
            if flags & (1 as libc::c_uint) << 8 as libc::c_int != 0 {
                log_query((flags | (1 as libc::c_uint) << 13 as libc::c_int |
                               (1 as libc::c_uint) << 3 as libc::c_int) &
                              !((1 as libc::c_uint) << 7 as libc::c_int),
                          qdomain, *addrpp, 0 as *mut libc::c_char);
            }
        }
    } else if *type_0 & 1024 as libc::c_int != 0 {
        *type_0 = 0 as libc::c_int;
        *domain = 0 as *mut libc::c_char
    }
    return flags;
}
unsafe extern "C" fn forward_query(mut udpfd: libc::c_int,
                                   mut udpaddr: *mut mysockaddr,
                                   mut dst_addr: *mut all_addr,
                                   mut dst_iface: libc::c_uint,
                                   mut header: *mut dns_header,
                                   mut plen: size_t, mut now: time_t,
                                   mut forward: *mut frec,
                                   mut ad_reqd: libc::c_int,
                                   mut do_bit: libc::c_int) -> libc::c_int {
    let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: libc::c_int = 16384 as libc::c_int;
    let mut norebind: libc::c_int = 0 as libc::c_int;
    let mut addrp: *mut all_addr = 0 as *mut all_addr;
    let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut fwd_flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut start: *mut server = 0 as *mut server;
    let mut hash: *mut libc::c_void =
        hash_questions(header, plen, (*dnsmasq_daemon).namebuff) as
            *mut libc::c_void;
    let mut gotname: libc::c_uint =
        extract_request(header, plen, (*dnsmasq_daemon).namebuff,
                        0 as *mut libc::c_ushort);
    let mut oph: *mut libc::c_uchar =
        find_pseudoheader(header, plen, 0 as *mut size_t,
                          0 as *mut *mut libc::c_uchar, 0 as *mut libc::c_int,
                          0 as *mut libc::c_int);
    if (*header).hb4 as libc::c_int & 0x10 as libc::c_int != 0 {
        fwd_flags |= 2 as libc::c_int as libc::c_uint
    }
    if ad_reqd != 0 { fwd_flags |= 32 as libc::c_int as libc::c_uint }
    if !oph.is_null() { fwd_flags |= 1024 as libc::c_int as libc::c_uint }
    /* may be no servers available. */
    if !forward.is_null() ||
           {
               forward =
                   lookup_frec_by_sender(__bswap_16((*header).id), udpaddr,
                                         hash);
               !forward.is_null()
           } {
        /* If we didn't get an answer advertising a maximal packet in EDNS,
	 fall back to 1280, which should work everywhere on IPv6.
	 If that generates an answer, it will become the new default
	 for this server */
        (*forward).flags |= 256 as libc::c_int;
        /* retry on existing query, send to all available servers  */
        domain = (*(*forward).sentto).domain; /* at end of list, recycle */
        (*(*forward).sentto).failed_queries =
            (*(*forward).sentto).failed_queries.wrapping_add(1);
        if (*dnsmasq_daemon).options[(7 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (7 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
            (*forward).forwardall = 1 as libc::c_int;
            (*dnsmasq_daemon).last_server = 0 as *mut server
        }
        type_0 =
            (*(*forward).sentto).flags &
                (8 as libc::c_int | 32 as libc::c_int);
        start = (*(*forward).sentto).next;
        if start.is_null() { start = (*dnsmasq_daemon).servers }
        (*header).id = __bswap_16((*forward).new_id)
    } else {
        /* Query from new source, but the same query may be in progress
	 from another source. If so, just add this client to the
	 list that will get the reply.*/
        if (*dnsmasq_daemon).options[(32 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (32 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 &&
               (*dnsmasq_daemon).options[(54 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (54 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   == 0 &&
               {
                   forward = lookup_frec_by_query(hash, fwd_flags);
                   !forward.is_null()
               } {
            /* Note whine_malloc() zeros memory. */
            if (*dnsmasq_daemon).free_frec_src.is_null() &&
                   (*dnsmasq_daemon).frec_src_count <
                       (*dnsmasq_daemon).ftabsize &&
                   {
                       (*dnsmasq_daemon).free_frec_src =
                           whine_malloc(::std::mem::size_of::<frec_src>() as
                                            libc::c_ulong) as *mut frec_src;
                       !(*dnsmasq_daemon).free_frec_src.is_null()
                   } {
                (*dnsmasq_daemon).frec_src_count += 1;
                (*(*dnsmasq_daemon).free_frec_src).next = 0 as *mut frec_src
            }
            /* If we've been spammed with many duplicates, just drop the query. */
            if !(*dnsmasq_daemon).free_frec_src.is_null() {
                let mut new: *mut frec_src = (*dnsmasq_daemon).free_frec_src;
                (*dnsmasq_daemon).free_frec_src = (*new).next;
                (*new).next = (*forward).frec_src.next;
                (*forward).frec_src.next = new;
                (*new).orig_id = __bswap_16((*header).id);
                (*new).source = *udpaddr;
                (*new).dest = *dst_addr;
                (*new).log_id = (*dnsmasq_daemon).log_id as libc::c_uint;
                (*new).iface = dst_iface;
                (*new).fd = udpfd
            }
            return 1 as libc::c_int
        }
        if gotname != 0 {
            flags =
                search_servers(now, &mut addrp, gotname,
                               (*dnsmasq_daemon).namebuff, &mut type_0,
                               &mut domain, &mut norebind)
        }
        type_0 &= !(16384 as libc::c_int);
        if !(*dnsmasq_daemon).servers.is_null() && flags == 0 {
            forward = get_new_frec(now, 0 as *mut libc::c_int, 0 as *mut frec)
        }
        /* table full - flags == 0, return REFUSED */
        if !forward.is_null() {
            (*forward).frec_src.source = *udpaddr;
            (*forward).frec_src.orig_id = __bswap_16((*header).id);
            (*forward).frec_src.dest = *dst_addr;
            (*forward).frec_src.iface = dst_iface;
            (*forward).frec_src.next = 0 as *mut frec_src;
            (*forward).frec_src.fd = udpfd;
            (*forward).new_id = get_id();
            memcpy((*forward).hash.as_mut_ptr() as *mut libc::c_void, hash,
                   32 as libc::c_int as libc::c_ulong);
            (*forward).forwardall = 0 as libc::c_int;
            (*forward).flags = fwd_flags as libc::c_int;
            if norebind != 0 { (*forward).flags |= 1 as libc::c_int }
            if (*header).hb4 as libc::c_int & 0x10 as libc::c_int != 0 {
                (*forward).flags |= 2 as libc::c_int
            }
            if ad_reqd != 0 { (*forward).flags |= 32 as libc::c_int }
            (*header).id = __bswap_16((*forward).new_id);
            /* In strict_order mode, always try servers in the order 
	     specified in resolv.conf, if a domain is given 
	     always try all the available servers,
	     otherwise, use the one last known to work. */
            if type_0 == 0 as libc::c_int {
                if (*dnsmasq_daemon).options[(7 as libc::c_int as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &
                       (1 as libc::c_uint) <<
                           (7 as libc::c_int as
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                       != 0 {
                    start = (*dnsmasq_daemon).servers
                } else {
                    start = (*dnsmasq_daemon).last_server;
                    if start.is_null() ||
                           {
                               let fresh6 = (*dnsmasq_daemon).forwardcount;
                               (*dnsmasq_daemon).forwardcount =
                                   (*dnsmasq_daemon).forwardcount + 1;
                               (fresh6) > 50 as libc::c_int
                           } ||
                           difftime(now, (*dnsmasq_daemon).forwardtime) >
                               20 as libc::c_int as libc::c_double {
                        start = (*dnsmasq_daemon).servers;
                        (*forward).forwardall = 1 as libc::c_int;
                        (*dnsmasq_daemon).forwardcount = 0 as libc::c_int;
                        (*dnsmasq_daemon).forwardtime = now
                    }
                }
            } else {
                start = (*dnsmasq_daemon).servers;
                if (*dnsmasq_daemon).options[(7 as libc::c_int as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &
                       (1 as libc::c_uint) <<
                           (7 as libc::c_int as
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                       == 0 {
                    (*forward).forwardall = 1 as libc::c_int
                }
            }
        }
    }
    /* check for send errors here (no route to host) 
     if we fail to send to all nameservers, send back an error
     packet straight away (helps modem users when offline)  */
    if flags == 0 && !forward.is_null() {
        let mut firstsentto: *mut server = start;
        let mut subnet: libc::c_int = 0;
        let mut cacheable: libc::c_int = 0;
        let mut forwarded: libc::c_int = 0 as libc::c_int;
        let mut edns0_len: size_t = 0;
        let mut pheader: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        /* cancel */
        (*forward).frec_src.log_id = (*dnsmasq_daemon).log_id as libc::c_uint;
        plen =
            add_edns0_config(header, plen,
                             (header as
                                  *mut libc::c_uchar).offset(512 as
                                                                 libc::c_int
                                                                 as isize),
                             &mut (*forward).frec_src.source, now,
                             &mut subnet, &mut cacheable);
        if subnet != 0 { (*forward).flags |= 4 as libc::c_int }
        if cacheable == 0 { (*forward).flags |= 2048 as libc::c_int }
        if !find_pseudoheader(header, plen, &mut edns0_len, &mut pheader,
                              0 as *mut libc::c_int,
                              0 as *mut libc::c_int).is_null() {
            /* If a query is retried, use the log_id for the retry when logging the answer. */
            /* If there wasn't a PH before, and there is now, we added it. */
            if oph.is_null() { (*forward).flags |= 128 as libc::c_int }
            /* If we're sending an EDNS0 with any options, we can't recreate the query from a reply. */
            if edns0_len > 11 as libc::c_int as libc::c_ulong {
                (*forward).flags |= 512 as libc::c_int
            }
            /* Reduce udp size on retransmits. */
            if (*forward).flags & 256 as libc::c_int != 0 {
                let mut t_s: u16_0 = 1280 as libc::c_int as u16_0;
                let mut t_cp: *mut libc::c_uchar = pheader;
                let fresh7 = t_cp;
                t_cp = t_cp.offset(1);
                *fresh7 =
                    (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
                *t_cp = t_s as libc::c_uchar;
                pheader = pheader.offset(2 as libc::c_int as isize)
            }
        }
        loop 
             /* only send to servers dealing with our domain.
	     domain may be NULL, in which case server->domain 
	     must be NULL also. */
             {
            if type_0 ==
                   (*start).flags & (8 as libc::c_int | 32 as libc::c_int) &&
                   (type_0 != 8 as libc::c_int ||
                        hostname_isequal(domain, (*start).domain) != 0) &&
                   (*start).flags & (4 as libc::c_int | 8192 as libc::c_int)
                       == 0 {
                let mut fd: libc::c_int = 0;
                /* find server socket to use, may need to get random one. */
                if !(*start).sfd.is_null() {
                    fd = (*(*start).sfd).fd
                } else if (*start).addr.sa.sa_family as libc::c_int ==
                              10 as libc::c_int {
                    if (*forward).rfd6.is_null() &&
                           {
                               (*forward).rfd6 =
                                   allocate_rfd(10 as libc::c_int);
                               (*forward).rfd6.is_null()
                           } {
                        break ;
                    }
                    (*dnsmasq_daemon).rfd_save = (*forward).rfd6;
                    fd = (*(*forward).rfd6).fd
                } else {
                    if (*forward).rfd4.is_null() &&
                           {
                               (*forward).rfd4 =
                                   allocate_rfd(2 as libc::c_int);
                               (*forward).rfd4.is_null()
                           } {
                        break ;
                    }
                    (*dnsmasq_daemon).rfd_save = (*forward).rfd4;
                    fd = (*(*forward).rfd4).fd
                }
                if retry_send(sendto(fd,
                                     header as *mut libc::c_char as
                                         *const libc::c_void, plen,
                                     0 as libc::c_int,
                                     __CONST_SOCKADDR_ARG{__sockaddr__:
                                                              &mut (*start).addr.sa,},
                                     sa_len(&mut (*start).addr) as socklen_t))
                       != 0 {
                    continue ;
                }
                if *__errno_location() == 0 as libc::c_int {
                    dump_packet(0x4 as libc::c_int,
                                header as *mut libc::c_void, plen,
                                0 as *mut mysockaddr, &mut (*start).addr);
                    /* Keep info in case we want to re-send this packet */
                    (*dnsmasq_daemon).srv_save = start;
                    (*dnsmasq_daemon).packet_len = plen;
                    if gotname == 0 {
                        strcpy((*dnsmasq_daemon).namebuff,
                               b"query\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    if (*start).addr.sa.sa_family as libc::c_int ==
                           2 as libc::c_int {
                        log_query((1 as libc::c_uint) << 18 as libc::c_int |
                                      (1 as libc::c_uint) << 7 as libc::c_int
                                      |
                                      (1 as libc::c_uint) << 3 as libc::c_int,
                                  (*dnsmasq_daemon).namebuff,
                                  &mut (*start).addr.in_0.sin_addr as
                                      *mut in_addr as *mut all_addr,
                                  0 as *mut libc::c_char);
                    } else {
                        log_query((1 as libc::c_uint) << 18 as libc::c_int |
                                      (1 as libc::c_uint) << 8 as libc::c_int
                                      |
                                      (1 as libc::c_uint) << 3 as libc::c_int,
                                  (*dnsmasq_daemon).namebuff,
                                  &mut (*start).addr.in6.sin6_addr as
                                      *mut in6_addr as *mut all_addr,
                                  0 as *mut libc::c_char);
                    }
                    (*start).queries = (*start).queries.wrapping_add(1);
                    forwarded = 1 as libc::c_int;
                    (*forward).sentto = start;
                    if (*forward).forwardall == 0 { break ; }
                    (*forward).forwardall += 1
                }
            }
            start = (*start).next;
            if start.is_null() { start = (*dnsmasq_daemon).servers }
            if start == firstsentto { break ; }
        }
        if forwarded != 0 { return 1 as libc::c_int }
        (*header).id = __bswap_16((*forward).frec_src.orig_id);
        free_frec(forward);
    }
    /* could not send on, prepare to return */
    /* could not send on, return empty answer or address if known for whole domain */
    if udpfd != -(1 as libc::c_int) {
        plen =
            setup_reply(header, plen, addrp, flags,
                        (*dnsmasq_daemon).local_ttl);
        if !oph.is_null() {
            plen =
                add_pseudoheader(header, plen,
                                 (header as
                                      *mut libc::c_uchar).offset(512 as
                                                                     libc::c_int
                                                                     as
                                                                     isize),
                                 (*dnsmasq_daemon).edns_pktsz,
                                 0 as libc::c_int, 0 as *mut libc::c_uchar,
                                 0 as libc::c_int as size_t, do_bit,
                                 0 as libc::c_int)
        }
        send_from(udpfd,
                  ((*dnsmasq_daemon).options[(13 as libc::c_int as
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
                           != 0) as libc::c_int, header as *mut libc::c_char,
                  plen, udpaddr, dst_addr, dst_iface);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn process_reply(mut header: *mut dns_header,
                                   mut now: time_t, mut server: *mut server,
                                   mut n: size_t,
                                   mut check_rebind: libc::c_int,
                                   mut no_cache: libc::c_int,
                                   mut cache_secure: libc::c_int,
                                   mut bogusanswer: libc::c_int,
                                   mut ad_reqd: libc::c_int,
                                   mut do_bit: libc::c_int,
                                   mut added_pheader: libc::c_int,
                                   mut check_subnet: libc::c_int,
                                   mut query_source: *mut mysockaddr)
 -> size_t {
    let mut pheader: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sizep: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sets: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut munged: libc::c_int = 0 as libc::c_int;
    let mut is_sign: libc::c_int = 0;
    let mut rcode: libc::c_uint =
        ((*header).hb4 as libc::c_int & 0xf as libc::c_int) as libc::c_uint;
    let mut plen: size_t = 0;
    if !(*dnsmasq_daemon).ipsets.is_null() &&
           extract_request(header, n, (*dnsmasq_daemon).namebuff,
                           0 as *mut libc::c_ushort) != 0 {
        /* Similar algorithm to search_servers. */
        let mut ipset_pos: *mut ipsets = 0 as *mut ipsets;
        let mut namelen: libc::c_uint =
            strlen((*dnsmasq_daemon).namebuff) as libc::c_uint;
        let mut matchlen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        ipset_pos = (*dnsmasq_daemon).ipsets;
        while !ipset_pos.is_null() {
            let mut domainlen: libc::c_uint =
                strlen((*ipset_pos).domain) as libc::c_uint;
            let mut matchstart: *mut libc::c_char =
                (*dnsmasq_daemon).namebuff.offset(namelen as
                                                      isize).offset(-(domainlen
                                                                          as
                                                                          isize));
            if namelen >= domainlen &&
                   hostname_isequal(matchstart, (*ipset_pos).domain) != 0 &&
                   (domainlen == 0 as libc::c_int as libc::c_uint ||
                        namelen == domainlen ||
                        *matchstart.offset(-(1 as libc::c_int as isize)) as
                            libc::c_int == '.' as i32) &&
                   domainlen >= matchlen {
                matchlen = domainlen;
                sets = (*ipset_pos).sets
            }
            ipset_pos = (*ipset_pos).next
        }
    }
    pheader =
        find_pseudoheader(header, n, &mut plen, &mut sizep, &mut is_sign,
                          0 as *mut libc::c_int);
    if !pheader.is_null() {
        /* Get extended RCODE. */
        rcode |=
            ((*sizep.offset(2 as libc::c_int as isize) as libc::c_int) <<
                 4 as libc::c_int) as libc::c_uint;
        if check_subnet != 0 &&
               check_source(header, plen, pheader, query_source) == 0 {
            my_syslog(4 as libc::c_int,
                      b"discarding DNS reply: subnet option mismatch\x00" as
                          *const u8 as *const libc::c_char);
            return 0 as libc::c_int as size_t
        }
        if is_sign == 0 {
            if added_pheader != 0 {
                /* client didn't send EDNS0, we added one, strip it off before returning answer. */
                n = rrfilter(header, n, 0 as libc::c_int);
                pheader = 0 as *mut libc::c_uchar
            } else {
                let mut udpsz: libc::c_ushort = 0;
                /* If upstream is advertising a larger UDP packet size
		 than we allow, trim it so that we don't get overlarge
		 requests for the client. We can't do this for signed packets. */
                let mut t_cp: *mut libc::c_uchar = sizep;
                udpsz =
                    ((*t_cp.offset(0 as libc::c_int as isize) as u16_0 as
                          libc::c_int) << 8 as libc::c_int |
                         *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                             libc::c_int) as libc::c_ushort;
                sizep = sizep.offset(2 as libc::c_int as isize);
                if udpsz as libc::c_int >
                       (*dnsmasq_daemon).edns_pktsz as libc::c_int {
                    sizep = sizep.offset(-(2 as libc::c_int as isize));
                    let mut t_s: u16_0 = (*dnsmasq_daemon).edns_pktsz;
                    let mut t_cp_0: *mut libc::c_uchar = sizep;
                    let fresh8 = t_cp_0;
                    t_cp_0 = t_cp_0.offset(1);
                    *fresh8 =
                        (t_s as libc::c_int >> 8 as libc::c_int) as
                            libc::c_uchar;
                    *t_cp_0 = t_s as libc::c_uchar;
                    sizep = sizep.offset(2 as libc::c_int as isize)
                }
            }
        }
    }
    /* RFC 4035 sect 4.6 para 3 */
    if is_sign == 0 &&
           (*dnsmasq_daemon).options[(33 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (33 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !(0x20 as libc::c_int)) as u8_0
    }
    if ((*header).hb3 as libc::c_int & 0x78 as libc::c_int) >>
           3 as libc::c_int != 0 as libc::c_int {
        return resize_packet(header, n, pheader, plen)
    }
    if rcode != 0 as libc::c_int as libc::c_uint &&
           rcode != 3 as libc::c_int as libc::c_uint {
        let mut a: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
        a.log.rcode = rcode as libc::c_ushort;
        log_query((1 as libc::c_uint) << 16 as libc::c_int |
                      (1 as libc::c_uint) << 29 as libc::c_int,
                  b"error\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut a, 0 as *mut libc::c_char);
        return resize_packet(header, n, pheader, plen)
    }
    /* Complain loudly if the upstream server is non-recursive. */
    if (*header).hb4 as libc::c_int & 0x80 as libc::c_int == 0 &&
           rcode == 0 as libc::c_int as libc::c_uint && !server.is_null() &&
           (*server).flags & 64 as libc::c_int == 0 {
        prettyprint_addr(&mut (*server).addr, (*dnsmasq_daemon).namebuff);
        my_syslog(4 as libc::c_int,
                  b"nameserver %s refused to do a recursive query\x00" as
                      *const u8 as *const libc::c_char,
                  (*dnsmasq_daemon).namebuff);
        if (*dnsmasq_daemon).options[(2 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (2 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
            (*server).flags |= 64 as libc::c_int
        }
    }
    if !(*dnsmasq_daemon).bogus_addr.is_null() &&
           rcode != 3 as libc::c_int as libc::c_uint &&
           check_for_bogus_wildcard(header, n, (*dnsmasq_daemon).namebuff,
                                    (*dnsmasq_daemon).bogus_addr, now) != 0 {
        munged = 1 as libc::c_int;
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !(0xf as libc::c_int) |
                 3 as libc::c_int) as u8_0;
        (*header).hb3 =
            ((*header).hb3 as libc::c_int & !(0x4 as libc::c_int)) as u8_0;
        cache_secure = 0 as libc::c_int
    } else {
        let mut doctored: libc::c_int = 0 as libc::c_int;
        if rcode == 3 as libc::c_int as libc::c_uint &&
               extract_request(header, n, (*dnsmasq_daemon).namebuff,
                               0 as *mut libc::c_ushort) != 0 &&
               check_for_local_domain((*dnsmasq_daemon).namebuff, now) != 0 {
            /* if we forwarded a query for a locally known name (because it was for 
	     an unknown type) and the answer is NXDOMAIN, convert that to NODATA,
	     since we know that the domain exists, even if upstream doesn't */
            munged = 1 as libc::c_int;
            (*header).hb3 =
                ((*header).hb3 as libc::c_int | 0x4 as libc::c_int) as u8_0;
            (*header).hb4 =
                ((*header).hb4 as libc::c_int & !(0xf as libc::c_int) |
                     0 as libc::c_int) as u8_0;
            cache_secure = 0 as libc::c_int
        }
        if extract_addresses(header, n, (*dnsmasq_daemon).namebuff, now, sets,
                             is_sign, check_rebind, no_cache, cache_secure,
                             &mut doctored) != 0 {
            my_syslog(4 as libc::c_int,
                      b"possible DNS-rebind attack detected: %s\x00" as
                          *const u8 as *const libc::c_char,
                      (*dnsmasq_daemon).namebuff);
            munged = 1 as libc::c_int;
            cache_secure = 0 as libc::c_int
        }
        if doctored != 0 { cache_secure = 0 as libc::c_int }
    }
    /* do this after extract_addresses. Ensure NODATA reply and remove
     nameserver info. */
    if munged != 0 {
        (*header).ancount = __bswap_16(0 as libc::c_int as __uint16_t);
        (*header).nscount = __bswap_16(0 as libc::c_int as __uint16_t);
        (*header).arcount = __bswap_16(0 as libc::c_int as __uint16_t);
        (*header).hb3 =
            ((*header).hb3 as libc::c_int & !(0x2 as libc::c_int)) as u8_0
    }
    /* the bogus-nxdomain stuff, doctor and NXDOMAIN->NODATA munging can all elide
     sections of the packet. Find the new length here and put back pseudoheader
     if it was removed. */
    return resize_packet(header, n, pheader, plen);
}
/* sets new last_server */
#[no_mangle]
pub unsafe extern "C" fn reply_query(mut fd: libc::c_int,
                                     mut family: libc::c_int,
                                     mut now: time_t) {
    /* packet from peer server, extract data for cache, and send to
     original requester */
    let mut header: *mut dns_header = 0 as *mut dns_header;
    let mut serveraddr: mysockaddr =
        mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut forward: *mut frec = 0 as *mut frec;
    let mut addrlen: socklen_t =
        ::std::mem::size_of::<mysockaddr>() as libc::c_ulong as socklen_t;
    let mut n: ssize_t =
        recvfrom(fd, (*dnsmasq_daemon).packet as *mut libc::c_void,
                 (*dnsmasq_daemon).packet_buff_sz as size_t, 0 as libc::c_int,
                 __SOCKADDR_ARG{__sockaddr__:
                                    &mut serveraddr.sa as *mut sockaddr,},
                 &mut addrlen);
    let mut nn: size_t = 0;
    let mut server: *mut server = 0 as *mut server;
    let mut hash: *mut libc::c_void = 0 as *mut libc::c_void;
    /* packet buffer overwritten */
    (*dnsmasq_daemon).srv_save = 0 as *mut server;
    /* Determine the address of the server replying  so that we can mark that as good */
    serveraddr.sa.sa_family = family as sa_family_t;
    if serveraddr.sa.sa_family as libc::c_int == 10 as libc::c_int {
        serveraddr.in6.sin6_flowinfo = 0 as libc::c_int as uint32_t
    }
    header = (*dnsmasq_daemon).packet as *mut dns_header;
    if n <
           ::std::mem::size_of::<dns_header>() as libc::c_ulong as libc::c_int
               as libc::c_long ||
           (*header).hb3 as libc::c_int & 0x80 as libc::c_int == 0 {
        return
    }
    /* spoof check: answer must come from known server, */
    server = (*dnsmasq_daemon).servers;
    while !server.is_null() {
        if (*server).flags & (4 as libc::c_int | 2 as libc::c_int) == 0 &&
               sockaddr_isequal(&mut (*server).addr, &mut serveraddr) != 0 {
            break ;
        }
        server = (*server).next
    }
    if server.is_null() { return }
    /* If sufficient time has elapsed, try and expand UDP buffer size again. */
    if difftime(now, (*server).pktsz_reduced) >
           60 as libc::c_int as libc::c_double {
        (*server).edns_pktsz = (*dnsmasq_daemon).edns_pktsz as libc::c_int
    }
    hash =
        hash_questions(header, n as size_t, (*dnsmasq_daemon).namebuff) as
            *mut libc::c_void;
    forward = lookup_frec(__bswap_16((*header).id), fd, family, hash);
    if forward.is_null() { return }
    dump_packet(if (*forward).flags & (8 as libc::c_int | 16 as libc::c_int)
                       != 0 {
                    0x20 as libc::c_int
                } else { 0x8 as libc::c_int }, header as *mut libc::c_void,
                n as size_t, &mut serveraddr, 0 as *mut mysockaddr);
    /* log_query gets called indirectly all over the place, so 
     pass these in global variables - sorry. */
    (*dnsmasq_daemon).log_display_id =
        (*forward).frec_src.log_id as libc::c_int;
    (*dnsmasq_daemon).log_source_addr = &mut (*forward).frec_src.source;
    if !(*dnsmasq_daemon).ignore_addr.is_null() &&
           (*header).hb4 as libc::c_int & 0xf as libc::c_int ==
               0 as libc::c_int &&
           check_for_ignored_address(header, n as size_t,
                                     (*dnsmasq_daemon).ignore_addr) != 0 {
        return
    }
    /* Note: if we send extra options in the EDNS0 header, we can't recreate
     the query from the reply. */
    if ((*header).hb4 as libc::c_int & 0xf as libc::c_int == 5 as libc::c_int
            ||
            (*header).hb4 as libc::c_int & 0xf as libc::c_int ==
                2 as libc::c_int) && (*forward).forwardall == 0 as libc::c_int
           && (*forward).flags & 512 as libc::c_int == 0 {
        /* for broken servers, attempt to send to another one. */
        let mut pheader: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut plen: size_t = 0;
        let mut is_sign: libc::c_int = 0;
        /* In strict order mode, there must be a server later in the chain
	 left to send to, otherwise without the forwardall mechanism,
	 code further on will cycle around the list forwever if they
	 all return REFUSED. Note that server is always non-NULL before 
	 this executes. */
        if (*dnsmasq_daemon).options[(7 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (7 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
            server = (*(*forward).sentto).next;
            while !server.is_null() {
                if (*server).flags &
                       (4 as libc::c_int | 8 as libc::c_int |
                            32 as libc::c_int | 2 as libc::c_int |
                            8192 as libc::c_int) == 0 {
                    break ;
                }
                server = (*server).next
            }
        }
        /* recreate query from reply */
        pheader =
            find_pseudoheader(header, n as size_t, &mut plen,
                              0 as *mut *mut libc::c_uchar, &mut is_sign,
                              0 as *mut libc::c_int);
        if is_sign == 0 && !server.is_null() {
            (*header).ancount = __bswap_16(0 as libc::c_int as __uint16_t);
            (*header).nscount = __bswap_16(0 as libc::c_int as __uint16_t);
            (*header).arcount = __bswap_16(0 as libc::c_int as __uint16_t);
            nn = resize_packet(header, n as size_t, pheader, plen);
            if nn != 0 {
                (*header).hb3 =
                    ((*header).hb3 as libc::c_int &
                         !(0x80 as libc::c_int | 0x4 as libc::c_int |
                               0x2 as libc::c_int)) as u8_0;
                (*header).hb4 =
                    ((*header).hb4 as libc::c_int &
                         !(0x80 as libc::c_int | 0xf as libc::c_int |
                               0x10 as libc::c_int | 0x20 as libc::c_int)) as
                        u8_0;
                if (*forward).flags & 2 as libc::c_int != 0 {
                    (*header).hb4 =
                        ((*header).hb4 as libc::c_int | 0x10 as libc::c_int)
                            as u8_0
                }
                if (*forward).flags & 32 as libc::c_int != 0 {
                    (*header).hb4 =
                        ((*header).hb4 as libc::c_int | 0x20 as libc::c_int)
                            as u8_0
                }
                if (*forward).flags & 64 as libc::c_int != 0 {
                    add_do_bit(header, nn, pheader.offset(plen as isize));
                }
                forward_query(-(1 as libc::c_int), 0 as *mut mysockaddr,
                              0 as *mut all_addr,
                              0 as libc::c_int as libc::c_uint, header, nn,
                              now, forward,
                              (*forward).flags & 32 as libc::c_int,
                              (*forward).flags & 64 as libc::c_int);
                return
            }
        }
    }
    server = (*forward).sentto;
    if (*(*forward).sentto).flags & (8 as libc::c_int | 32 as libc::c_int) ==
           0 as libc::c_int {
        if (*header).hb4 as libc::c_int & 0xf as libc::c_int ==
               5 as libc::c_int {
            server = 0 as *mut server
        } else {
            let mut last_server: *mut server = 0 as *mut server;
            /* find good server by address if possible, otherwise assume the last one we sent to */
            last_server = (*dnsmasq_daemon).servers;
            while !last_server.is_null() {
                if (*last_server).flags &
                       (4 as libc::c_int | 8 as libc::c_int |
                            32 as libc::c_int | 2 as libc::c_int) == 0 &&
                       sockaddr_isequal(&mut (*last_server).addr,
                                        &mut serveraddr) != 0 {
                    server = last_server;
                    break ;
                } else { last_server = (*last_server).next }
            }
        }
        if (*dnsmasq_daemon).options[(23 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (23 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
            (*dnsmasq_daemon).last_server = server
        }
    }
    /* We tried resending to this server with a smaller maximum size and got an answer.
     Make that permanent. To avoid reduxing the packet size for a single dropped packet,
     only do this when we get a truncated answer, or one larger than the safe size. */
    if (*(*forward).sentto).edns_pktsz > 1280 as libc::c_int &&
           (*forward).flags & 256 as libc::c_int != 0 &&
           ((*header).hb3 as libc::c_int & 0x2 as libc::c_int != 0 ||
                n >= 1280 as libc::c_int as libc::c_long) {
        (*(*forward).sentto).edns_pktsz = 1280 as libc::c_int;
        (*(*forward).sentto).pktsz_reduced = now;
        prettyprint_addr(&mut (*(*forward).sentto).addr,
                         (*dnsmasq_daemon).addrbuff);
        my_syslog(4 as libc::c_int,
                  b"reducing DNS packet size for nameserver %s to %d\x00" as
                      *const u8 as *const libc::c_char,
                  (*dnsmasq_daemon).addrbuff, 1280 as libc::c_int);
    }
    /* If the answer is an error, keep the forward record in place in case
     we get a good reply from another server. Kill it when we've
     had replies from all to avoid filling the forwarding table when
     everything is broken */
    if (*forward).forwardall == 0 as libc::c_int ||
           {
               (*forward).forwardall -= 1;
               ((*forward).forwardall) == 1 as libc::c_int
           } ||
           (*header).hb4 as libc::c_int & 0xf as libc::c_int !=
               5 as libc::c_int {
        let mut check_rebind: libc::c_int = 0 as libc::c_int;
        let mut no_cache_dnssec: libc::c_int = 0 as libc::c_int;
        let mut cache_secure: libc::c_int = 0 as libc::c_int;
        let mut bogusanswer: libc::c_int = 0 as libc::c_int;
        if (*dnsmasq_daemon).options[(31 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (31 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
            check_rebind =
                ((*forward).flags & 1 as libc::c_int == 0) as libc::c_int
        }
        /* cancel */
        if (*header).hb4 as libc::c_int & 0x10 as libc::c_int != 0 ||
               (*forward).flags & 2 as libc::c_int != 0 {
            no_cache_dnssec = 1 as libc::c_int
        }
        if (*forward).flags & 2 as libc::c_int != 0 {
            (*header).hb4 =
                ((*header).hb4 as libc::c_int | 0x10 as libc::c_int) as u8_0
        } else {
            (*header).hb4 =
                ((*header).hb4 as libc::c_int & !(0x10 as libc::c_int)) as
                    u8_0
        }
        if (*forward).flags & 2048 as libc::c_int != 0 {
            no_cache_dnssec = 1 as libc::c_int
        }
        nn =
            process_reply(header, now, (*forward).sentto, n as size_t,
                          check_rebind, no_cache_dnssec, cache_secure,
                          bogusanswer, (*forward).flags & 32 as libc::c_int,
                          (*forward).flags & 64 as libc::c_int,
                          (*forward).flags & 128 as libc::c_int,
                          (*forward).flags & 4 as libc::c_int,
                          &mut (*forward).frec_src.source);
        if nn != 0 {
            let mut src: *mut frec_src = 0 as *mut frec_src;
            (*header).id = __bswap_16((*forward).frec_src.orig_id);
            /*   Don't cache replies where DNSSEC validation was turned off, either
	   the upstream server told us so, or the original query specified it.  */
            /* restore CD bit to the value in the query */
            /* Never cache answers which are contingent on the source or MAC address EDSN0 option,
	 since the cache is ignorant of such things. */
            (*header).hb4 =
                ((*header).hb4 as libc::c_int | 0x80 as libc::c_int) as
                    u8_0; /* recursion if available */
            src = &mut (*forward).frec_src; /* default if no EDNS0 */
            while !src.is_null() {
                (*header).id = __bswap_16((*src).orig_id);
                dump_packet(0x2 as libc::c_int,
                            (*dnsmasq_daemon).packet as *mut libc::c_void, nn,
                            0 as *mut mysockaddr, &mut (*src).source);
                send_from((*src).fd,
                          ((*dnsmasq_daemon).options[(13 as libc::c_int as
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
                                   != 0) as libc::c_int,
                          (*dnsmasq_daemon).packet, nn, &mut (*src).source,
                          &mut (*src).dest, (*src).iface);
                if (*dnsmasq_daemon).options[(51 as libc::c_int as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &
                       (1 as libc::c_uint) <<
                           (51 as libc::c_int as
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                       != 0 &&
                       src != &mut (*forward).frec_src as *mut frec_src {
                    (*dnsmasq_daemon).log_display_id =
                        (*src).log_id as libc::c_int;
                    (*dnsmasq_daemon).log_source_addr = &mut (*src).source;
                    log_query((1 as libc::c_uint) << 16 as libc::c_int,
                              b"query\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char, 0 as *mut all_addr,
                              b"duplicate\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char);
                }
                src = (*src).next
            }
        }
        free_frec(forward);
    };
}
#[no_mangle]
pub unsafe extern "C" fn receive_query(mut listen: *mut listener,
                                       mut now: time_t) {
    let mut header: *mut dns_header =
        (*dnsmasq_daemon).packet as *mut dns_header;
    let mut source_addr: mysockaddr =
        mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut pheader: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut type_0: libc::c_ushort = 0;
    let mut udp_size: libc::c_ushort = 512 as libc::c_int as libc::c_ushort;
    let mut dst_addr: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut netmask: in_addr = in_addr{s_addr: 0,};
    let mut dst_addr_4: in_addr = in_addr{s_addr: 0,};
    let mut m: size_t = 0;
    let mut n: ssize_t = 0;
    let mut if_index: libc::c_int = 0 as libc::c_int;
    let mut auth_dns: libc::c_int = 0 as libc::c_int;
    let mut do_bit: libc::c_int = 0 as libc::c_int;
    let mut have_pseudoheader: libc::c_int = 0 as libc::c_int;
    let mut local_auth: libc::c_int = 0 as libc::c_int;
    let mut iov: [iovec; 1] =
        [iovec{iov_base: 0 as *mut libc::c_void, iov_len: 0,}; 1];
    let mut msg: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut cmptr: *mut cmsghdr = 0 as *mut cmsghdr;
    let mut control_u: C2RustUnnamed_16 =
        C2RustUnnamed_16{align:
                             cmsghdr{cmsg_len: 0,
                                     cmsg_level: 0,
                                     cmsg_type: 0,
                                     __cmsg_data: [],},};
    let mut family: libc::c_int = (*listen).addr.sa.sa_family as libc::c_int;
    /* Can always get recvd interface for IPv6 */
    let mut check_dst: libc::c_int =
        ((*dnsmasq_daemon).options[(13 as libc::c_int as
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
             == 0 || family == 10 as libc::c_int) as libc::c_int;
    /* packet buffer overwritten */
    (*dnsmasq_daemon).srv_save = 0 as *mut server;
    dst_addr.addr4.s_addr = 0 as libc::c_int as in_addr_t;
    dst_addr_4.s_addr = dst_addr.addr4.s_addr;
    netmask.s_addr = 0 as libc::c_int as in_addr_t;
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
           != 0 && !(*listen).iface.is_null() {
        auth_dns = (*(*listen).iface).dns_auth;
        if family == 2 as libc::c_int {
            dst_addr.addr4 = (*(*listen).iface).addr.in_0.sin_addr;
            dst_addr_4 = dst_addr.addr4;
            netmask = (*(*listen).iface).netmask
        }
    }
    iov[0 as libc::c_int as usize].iov_base =
        (*dnsmasq_daemon).packet as *mut libc::c_void;
    iov[0 as libc::c_int as usize].iov_len =
        (*dnsmasq_daemon).edns_pktsz as size_t;
    msg.msg_control = control_u.control.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen =
        ::std::mem::size_of::<C2RustUnnamed_16>() as libc::c_ulong;
    msg.msg_flags = 0 as libc::c_int;
    msg.msg_name = &mut source_addr as *mut mysockaddr as *mut libc::c_void;
    msg.msg_namelen =
        ::std::mem::size_of::<mysockaddr>() as libc::c_ulong as socklen_t;
    msg.msg_iov = iov.as_mut_ptr();
    msg.msg_iovlen = 1 as libc::c_int as size_t;
    n = recvmsg((*listen).fd, &mut msg, 0 as libc::c_int);
    if n == -(1 as libc::c_int) as libc::c_long { return }
    if n <
           ::std::mem::size_of::<dns_header>() as libc::c_ulong as libc::c_int
               as libc::c_long ||
           msg.msg_flags & MSG_TRUNC as libc::c_int != 0 ||
           (*header).hb3 as libc::c_int & 0x80 as libc::c_int != 0 {
        return
    }
    /* Clear buffer beyond request to avoid risk of
     information disclosure. */
    memset((*dnsmasq_daemon).packet.offset(n as isize) as *mut libc::c_void,
           0 as libc::c_int,
           ((*dnsmasq_daemon).edns_pktsz as libc::c_long - n) as
               libc::c_ulong);
    source_addr.sa.sa_family = family as sa_family_t;
    if family == 2 as libc::c_int {
        /* Source-port == 0 is an error, we can't send back to that. 
	  http://www.ietf.org/mail-archive/web/dnsop/current/msg11441.html */
        if source_addr.in_0.sin_port as libc::c_int == 0 as libc::c_int {
            return
        }
    } else {
        /* Source-port == 0 is an error, we can't send back to that. */
        if source_addr.in6.sin6_port as libc::c_int == 0 as libc::c_int {
            return
        }
        source_addr.in6.sin6_flowinfo = 0 as libc::c_int as uint32_t
    }
    /* We can be configured to only accept queries from at-most-one-hop-away addresses. */
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
        let mut addr: *mut addrlist = 0 as *mut addrlist;
        if family == 10 as libc::c_int {
            addr = (*dnsmasq_daemon).interface_addrs;
            while !addr.is_null() {
                if (*addr).flags & 2 as libc::c_int != 0 &&
                       is_same_net6(&mut (*addr).addr.addr6,
                                    &mut source_addr.in6.sin6_addr,
                                    (*addr).prefixlen) != 0 {
                    break ;
                }
                addr = (*addr).next
            }
        } else {
            let mut netmask_0: in_addr = in_addr{s_addr: 0,};
            addr = (*dnsmasq_daemon).interface_addrs;
            while !addr.is_null() {
                netmask_0.s_addr =
                    __bswap_32((!(0 as libc::c_int as in_addr_t)) <<
                                   32 as libc::c_int - (*addr).prefixlen);
                if (*addr).flags & 2 as libc::c_int == 0 &&
                       is_same_net((*addr).addr.addr4,
                                   source_addr.in_0.sin_addr, netmask_0) != 0
                   {
                    break ;
                }
                addr = (*addr).next
            }
        }
        if addr.is_null() {
            static mut warned: libc::c_int = 0 as libc::c_int;
            if warned == 0 {
                my_syslog(4 as libc::c_int,
                          b"Ignoring query from non-local network\x00" as
                              *const u8 as *const libc::c_char);
                warned = 1 as libc::c_int
            }
            return
        }
    }
    if check_dst != 0 {
        let mut ifr: ifreq =
            ifreq{ifr_ifrn: C2RustUnnamed_4{ifrn_name: [0; 16],},
                  ifr_ifru:
                      C2RustUnnamed_3{ifru_addr:
                                          sockaddr{sa_family: 0,
                                                   sa_data: [0; 14],},},};
        if msg.msg_controllen <
               ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
            return
        }
        if family == 2 as libc::c_int {
            cmptr =
                if msg.msg_controllen >=
                       ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                    msg.msg_control as *mut cmsghdr
                } else { 0 as *mut cmsghdr };
            while !cmptr.is_null() {
                if (*cmptr).cmsg_level == IPPROTO_IP as libc::c_int &&
                       (*cmptr).cmsg_type == 8 as libc::c_int {
                    let mut p: C2RustUnnamed_15 =
                        C2RustUnnamed_15{c: 0 as *mut libc::c_uchar,};
                    p.c = (*cmptr).__cmsg_data.as_mut_ptr();
                    dst_addr.addr4 = (*p.p).ipi_spec_dst;
                    dst_addr_4 = dst_addr.addr4;
                    if_index = (*p.p).ipi_ifindex
                }
                cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            }
        }
        if family == 10 as libc::c_int {
            cmptr =
                if msg.msg_controllen >=
                       ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                    msg.msg_control as *mut cmsghdr
                } else { 0 as *mut cmsghdr };
            while !cmptr.is_null() {
                if (*cmptr).cmsg_level == IPPROTO_IPV6 as libc::c_int &&
                       (*cmptr).cmsg_type == (*dnsmasq_daemon).v6pktinfo {
                    let mut p_0: C2RustUnnamed_14 =
                        C2RustUnnamed_14{c: 0 as *mut libc::c_uchar,};
                    p_0.c = (*cmptr).__cmsg_data.as_mut_ptr();
                    dst_addr.addr6 = (*p_0.p).ipi6_addr;
                    if_index = (*p_0.p).ipi6_ifindex as libc::c_int
                }
                cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            }
        }
        /* enforce available interface configuration */
        if indextoname((*listen).fd, if_index,
                       ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) == 0 {
            return
        }
        if iface_check(family, &mut dst_addr,
                       ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), &mut auth_dns) ==
               0 {
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
                enumerate_interfaces(0 as libc::c_int);
            }
            if loopback_exception((*listen).fd, family, &mut dst_addr,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) == 0 &&
                   label_exception(if_index, family, &mut dst_addr) == 0 {
                return
            }
        }
        if family == 2 as libc::c_int &&
               (*dnsmasq_daemon).options[(18 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (18 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
            let mut iface: *mut irec = 0 as *mut irec;
            /* get the netmask of the interface which has the address we were sent to.
	     This is no necessarily the interface we arrived on. */
            iface = (*dnsmasq_daemon).interfaces;
            while !iface.is_null() {
                if (*iface).addr.sa.sa_family as libc::c_int ==
                       2 as libc::c_int &&
                       (*iface).addr.in_0.sin_addr.s_addr == dst_addr_4.s_addr
                   {
                    break ;
                }
                iface = (*iface).next
            }
            /* interface may be new */
            if iface.is_null() &&
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
                enumerate_interfaces(0 as libc::c_int);
            }
            iface = (*dnsmasq_daemon).interfaces;
            while !iface.is_null() {
                if (*iface).addr.sa.sa_family as libc::c_int ==
                       2 as libc::c_int &&
                       (*iface).addr.in_0.sin_addr.s_addr == dst_addr_4.s_addr
                   {
                    break ;
                }
                iface = (*iface).next
            }
            /* If we failed, abandon localisation */
            if !iface.is_null() {
                netmask = (*iface).netmask
            } else { dst_addr_4.s_addr = 0 as libc::c_int as in_addr_t }
        }
    }
    /* log_query gets called indirectly all over the place, so 
     pass these in global variables - sorry. */
    (*dnsmasq_daemon).log_id += 1;
    (*dnsmasq_daemon).log_display_id = (*dnsmasq_daemon).log_id;
    (*dnsmasq_daemon).log_source_addr = &mut source_addr;
    dump_packet(0x1 as libc::c_int,
                (*dnsmasq_daemon).packet as *mut libc::c_void, n as size_t,
                &mut source_addr, 0 as *mut mysockaddr);
    if extract_request(header, n as size_t, (*dnsmasq_daemon).namebuff,
                       &mut type_0) != 0 {
        let mut zone: *mut auth_zone = 0 as *mut auth_zone;
        let mut types: *mut libc::c_char =
            querystr(if auth_dns != 0 {
                         b"auth\x00" as *const u8 as *const libc::c_char
                     } else {
                         b"query\x00" as *const u8 as *const libc::c_char
                     } as *mut libc::c_char, type_0);
        if family == 2 as libc::c_int {
            log_query((1 as libc::c_uint) << 19 as libc::c_int |
                          (1 as libc::c_uint) << 7 as libc::c_int |
                          (1 as libc::c_uint) << 3 as libc::c_int,
                      (*dnsmasq_daemon).namebuff,
                      &mut source_addr.in_0.sin_addr as *mut in_addr as
                          *mut all_addr, types);
        } else {
            log_query((1 as libc::c_uint) << 19 as libc::c_int |
                          (1 as libc::c_uint) << 8 as libc::c_int |
                          (1 as libc::c_uint) << 3 as libc::c_int,
                      (*dnsmasq_daemon).namebuff,
                      &mut source_addr.in6.sin6_addr as *mut in6_addr as
                          *mut all_addr, types);
        }
        /* find queries for zones we're authoritative for, and answer them directly */
        if auth_dns == 0 &&
               (*dnsmasq_daemon).options[(18 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (18 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   == 0 {
            zone = (*dnsmasq_daemon).auth_zones;
            while !zone.is_null() {
                if in_zone(zone, (*dnsmasq_daemon).namebuff,
                           0 as *mut *mut libc::c_char) != 0 {
                    auth_dns = 1 as libc::c_int;
                    local_auth = 1 as libc::c_int;
                    break ;
                } else { zone = (*zone).next }
            }
        }
        /* Check for forwarding loop */
        if detect_loop((*dnsmasq_daemon).namebuff, type_0 as libc::c_int) != 0
           {
            return
        }
    }
    if !find_pseudoheader(header, n as size_t, 0 as *mut size_t, &mut pheader,
                          0 as *mut libc::c_int,
                          0 as *mut libc::c_int).is_null() {
        let mut flags: libc::c_ushort = 0;
        have_pseudoheader = 1 as libc::c_int;
        let mut t_cp: *mut libc::c_uchar = pheader;
        udp_size =
            ((*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                 << 8 as libc::c_int |
                 *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                     libc::c_int) as libc::c_ushort;
        pheader = pheader.offset(2 as libc::c_int as isize);
        /* Sanity check - can't reduce below default. RFC 6891 6.2.3 */
        pheader = pheader.offset(2 as libc::c_int as isize); /* ext_rcode */
        let mut t_cp_0: *mut libc::c_uchar = pheader; /* do bit */
        flags =
            ((*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                  libc::c_int) << 8 as libc::c_int |
                 *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                     libc::c_int) as libc::c_ushort;
        pheader = pheader.offset(2 as libc::c_int as isize);
        if flags as libc::c_int & 0x8000 as libc::c_int != 0 {
            do_bit = 1 as libc::c_int
        }
        if udp_size as libc::c_int >
               (*dnsmasq_daemon).edns_pktsz as libc::c_int {
            udp_size = (*dnsmasq_daemon).edns_pktsz
        } else if (udp_size as libc::c_int) < 512 as libc::c_int {
            udp_size = 512 as libc::c_int as libc::c_ushort
        }
    }
    if auth_dns != 0 {
        m =
            answer_auth(header,
                        (header as
                             *mut libc::c_char).offset(udp_size as libc::c_int
                                                           as isize),
                        n as size_t, now, &mut source_addr, local_auth,
                        do_bit, have_pseudoheader);
        if m >= 1 as libc::c_int as libc::c_ulong {
            send_from((*listen).fd,
                      ((*dnsmasq_daemon).options[(13 as libc::c_int as
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
                               != 0) as libc::c_int,
                      header as *mut libc::c_char, m, &mut source_addr,
                      &mut dst_addr, if_index as libc::c_uint);
            (*dnsmasq_daemon).metrics[METRIC_DNS_AUTH_ANSWERED as libc::c_int
                                          as usize] =
                (*dnsmasq_daemon).metrics[METRIC_DNS_AUTH_ANSWERED as
                                              libc::c_int as
                                              usize].wrapping_add(1)
        }
    } else {
        let mut ad_reqd: libc::c_int = do_bit;
        /* If the client provides an EDNS0 UDP size, use that to limit our reply.
	 (bounded by the maximum configured). If no EDNS0, then it
	 defaults to 512 */
        /* RFC 6840 5.7 */
        if (*header).hb4 as libc::c_int & 0x20 as libc::c_int != 0 {
            ad_reqd = 1 as libc::c_int
        }
        m =
            answer_request(header,
                           (header as
                                *mut libc::c_char).offset(udp_size as
                                                              libc::c_int as
                                                              isize),
                           n as size_t, dst_addr_4, netmask, now, ad_reqd,
                           do_bit, have_pseudoheader);
        if m >= 1 as libc::c_int as libc::c_ulong {
            send_from((*listen).fd,
                      ((*dnsmasq_daemon).options[(13 as libc::c_int as
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
                               != 0) as libc::c_int,
                      header as *mut libc::c_char, m, &mut source_addr,
                      &mut dst_addr, if_index as libc::c_uint);
            (*dnsmasq_daemon).metrics[METRIC_DNS_LOCAL_ANSWERED as libc::c_int
                                          as usize] =
                (*dnsmasq_daemon).metrics[METRIC_DNS_LOCAL_ANSWERED as
                                              libc::c_int as
                                              usize].wrapping_add(1)
        } else if forward_query((*listen).fd, &mut source_addr, &mut dst_addr,
                                if_index as libc::c_uint, header, n as size_t,
                                now, 0 as *mut frec, ad_reqd, do_bit) != 0 {
            (*dnsmasq_daemon).metrics[METRIC_DNS_QUERIES_FORWARDED as
                                          libc::c_int as usize] =
                (*dnsmasq_daemon).metrics[METRIC_DNS_QUERIES_FORWARDED as
                                              libc::c_int as
                                              usize].wrapping_add(1)
        } else {
            (*dnsmasq_daemon).metrics[METRIC_DNS_LOCAL_ANSWERED as libc::c_int
                                          as usize] =
                (*dnsmasq_daemon).metrics[METRIC_DNS_LOCAL_ANSWERED as
                                              libc::c_int as
                                              usize].wrapping_add(1)
        }
    };
}
/* The daemon forks before calling this: it should deal with one connection,
   blocking as necessary, and then return. Note, need to be a bit careful
   about resources for debug mode, when the fork is suppressed: that's
   done by the caller. */
#[no_mangle]
pub unsafe extern "C" fn tcp_request(mut confd: libc::c_int, mut now: time_t,
                                     mut local_addr: *mut mysockaddr,
                                     mut netmask: in_addr,
                                     mut auth_dns: libc::c_int)
 -> *mut libc::c_uchar {
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut norebind: libc::c_int = 0 as libc::c_int;
    let mut local_auth: libc::c_int = 0 as libc::c_int;
    let mut checking_disabled: libc::c_int = 0;
    let mut do_bit: libc::c_int = 0;
    let mut added_pheader: libc::c_int = 0 as libc::c_int;
    let mut have_pseudoheader: libc::c_int = 0 as libc::c_int;
    let mut check_subnet: libc::c_int = 0;
    let mut cacheable: libc::c_int = 0;
    let mut no_cache_dnssec: libc::c_int = 0 as libc::c_int;
    let mut cache_secure: libc::c_int = 0 as libc::c_int;
    let mut bogusanswer: libc::c_int = 0 as libc::c_int;
    let mut m: size_t = 0;
    let mut qtype: libc::c_ushort = 0;
    let mut gotname: libc::c_uint = 0;
    let mut c1: libc::c_uchar = 0;
    let mut c2: libc::c_uchar = 0;
    /* Max TCP packet + slop + size */
    let mut packet: *mut libc::c_uchar =
        whine_malloc(((65536 as libc::c_int + 1025 as libc::c_int +
                           10 as libc::c_int) as
                          libc::c_ulong).wrapping_add(::std::mem::size_of::<u16_0>()
                                                          as libc::c_ulong))
            as *mut libc::c_uchar;
    let mut payload: *mut libc::c_uchar =
        &mut *packet.offset(2 as libc::c_int as isize) as *mut libc::c_uchar;
    /* largest field in header is 16-bits, so this is still sufficiently aligned */
    let mut header: *mut dns_header = payload as *mut dns_header;
    let mut length: *mut u16_0 = packet as *mut u16_0;
    let mut last_server: *mut server = 0 as *mut server;
    let mut dst_addr_4: in_addr = in_addr{s_addr: 0,};
    let mut peer_addr: mysockaddr =
        mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut peer_len: socklen_t =
        ::std::mem::size_of::<mysockaddr>() as libc::c_ulong as socklen_t;
    let mut query_count: libc::c_int = 0 as libc::c_int;
    let mut pheader: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut mark: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut have_mark: libc::c_int = 0 as libc::c_int;
    if getpeername(confd,
                   __SOCKADDR_ARG{__sockaddr__:
                                      &mut peer_addr as *mut mysockaddr as
                                          *mut sockaddr,}, &mut peer_len) ==
           -(1 as libc::c_int) {
        return packet
    }
    /* We can be configured to only accept queries from at-most-one-hop-away addresses. */
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
        let mut addr: *mut addrlist = 0 as *mut addrlist;
        if peer_addr.sa.sa_family as libc::c_int == 10 as libc::c_int {
            addr = (*dnsmasq_daemon).interface_addrs;
            while !addr.is_null() {
                if (*addr).flags & 2 as libc::c_int != 0 &&
                       is_same_net6(&mut (*addr).addr.addr6,
                                    &mut peer_addr.in6.sin6_addr,
                                    (*addr).prefixlen) != 0 {
                    break ;
                }
                addr = (*addr).next
            }
        } else {
            let mut netmask_0: in_addr = in_addr{s_addr: 0,};
            addr = (*dnsmasq_daemon).interface_addrs;
            while !addr.is_null() {
                netmask_0.s_addr =
                    __bswap_32((!(0 as libc::c_int as in_addr_t)) <<
                                   32 as libc::c_int - (*addr).prefixlen);
                if (*addr).flags & 2 as libc::c_int == 0 &&
                       is_same_net((*addr).addr.addr4,
                                   peer_addr.in_0.sin_addr, netmask_0) != 0 {
                    break ;
                }
                addr = (*addr).next
            }
        }
        if addr.is_null() {
            my_syslog(4 as libc::c_int,
                      b"Ignoring query from non-local network\x00" as
                          *const u8 as *const libc::c_char);
            return packet
        }
    }
    loop  {
        if query_count == 100 as libc::c_int || packet.is_null() ||
               read_write(confd, &mut c1, 1 as libc::c_int, 1 as libc::c_int)
                   == 0 ||
               read_write(confd, &mut c2, 1 as libc::c_int, 1 as libc::c_int)
                   == 0 ||
               {
                   size =
                       ((c1 as libc::c_int) << 8 as libc::c_int |
                            c2 as libc::c_int) as size_t;
                   (size) == 0
               } ||
               read_write(confd, payload, size as libc::c_int,
                          1 as libc::c_int) == 0 {
            return packet
        }
        if size <
               ::std::mem::size_of::<dns_header>() as libc::c_ulong as
                   libc::c_int as libc::c_ulong {
            continue ;
        }
        /* Clear buffer beyond request to avoid risk of
	 information disclosure. */
        memset(payload.offset(size as isize) as *mut libc::c_void,
               0 as libc::c_int,
               (65536 as libc::c_int as libc::c_ulong).wrapping_sub(size));
        query_count += 1;
        /* log_query gets called indirectly all over the place, so 
	 pass these in global variables - sorry. */
        (*dnsmasq_daemon).log_id += 1;
        (*dnsmasq_daemon).log_display_id = (*dnsmasq_daemon).log_id;
        (*dnsmasq_daemon).log_source_addr = &mut peer_addr;
        /* save state of "cd" flag in query */
        checking_disabled =
            (*header).hb4 as libc::c_int & 0x10 as libc::c_int;
        if checking_disabled != 0 { no_cache_dnssec = 1 as libc::c_int }
        gotname =
            extract_request(header, size as libc::c_uint as size_t,
                            (*dnsmasq_daemon).namebuff, &mut qtype);
        if gotname != 0 {
            let mut zone: *mut auth_zone = 0 as *mut auth_zone;
            let mut types: *mut libc::c_char =
                querystr(if auth_dns != 0 {
                             b"auth\x00" as *const u8 as *const libc::c_char
                         } else {
                             b"query\x00" as *const u8 as *const libc::c_char
                         } as *mut libc::c_char, qtype);
            if peer_addr.sa.sa_family as libc::c_int == 2 as libc::c_int {
                log_query((1 as libc::c_uint) << 19 as libc::c_int |
                              (1 as libc::c_uint) << 7 as libc::c_int |
                              (1 as libc::c_uint) << 3 as libc::c_int,
                          (*dnsmasq_daemon).namebuff,
                          &mut peer_addr.in_0.sin_addr as *mut in_addr as
                              *mut all_addr, types);
            } else {
                log_query((1 as libc::c_uint) << 19 as libc::c_int |
                              (1 as libc::c_uint) << 8 as libc::c_int |
                              (1 as libc::c_uint) << 3 as libc::c_int,
                          (*dnsmasq_daemon).namebuff,
                          &mut peer_addr.in6.sin6_addr as *mut in6_addr as
                              *mut all_addr, types);
            }
            /* find queries for zones we're authoritative for, and answer them directly */
            if auth_dns == 0 &&
                   (*dnsmasq_daemon).options[(18 as libc::c_int as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &
                       (1 as libc::c_uint) <<
                           (18 as libc::c_int as
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                       == 0 {
                zone = (*dnsmasq_daemon).auth_zones;
                while !zone.is_null() {
                    if in_zone(zone, (*dnsmasq_daemon).namebuff,
                               0 as *mut *mut libc::c_char) != 0 {
                        auth_dns = 1 as libc::c_int;
                        local_auth = 1 as libc::c_int;
                        break ;
                    } else { zone = (*zone).next }
                }
            }
        }
        if (*local_addr).sa.sa_family as libc::c_int == 2 as libc::c_int {
            dst_addr_4 = (*local_addr).in_0.sin_addr
        } else { dst_addr_4.s_addr = 0 as libc::c_int as in_addr_t }
        do_bit = 0 as libc::c_int;
        if !find_pseudoheader(header, size, 0 as *mut size_t, &mut pheader,
                              0 as *mut libc::c_int,
                              0 as *mut libc::c_int).is_null() {
            let mut flags: libc::c_ushort = 0;
            have_pseudoheader = 1 as libc::c_int;
            /* do bit */
            pheader =
                pheader.offset(4 as libc::c_int as
                                   isize); /* udp_size, ext_rcode */
            let mut t_cp: *mut libc::c_uchar = pheader;
            flags =
                ((*t_cp.offset(0 as libc::c_int as isize) as u16_0 as
                      libc::c_int) << 8 as libc::c_int |
                     *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                         libc::c_int) as libc::c_ushort;
            pheader = pheader.offset(2 as libc::c_int as isize);
            if flags as libc::c_int & 0x8000 as libc::c_int != 0 {
                do_bit = 1 as libc::c_int
            }
        }
        if auth_dns != 0 {
            m =
                answer_auth(header,
                            (header as
                                 *mut libc::c_char).offset(65536 as
                                                               libc::c_int as
                                                               isize), size,
                            now, &mut peer_addr, local_auth, do_bit,
                            have_pseudoheader)
        } else {
            let mut ad_reqd: libc::c_int = do_bit;
            /* RFC 6840 5.7 */
            if (*header).hb4 as libc::c_int & 0x20 as libc::c_int != 0 {
                ad_reqd = 1 as libc::c_int
            }
            /* m > 0 if answered from cache */
            m =
                answer_request(header,
                               (header as
                                    *mut libc::c_char).offset(65536 as
                                                                  libc::c_int
                                                                  as isize),
                               size, dst_addr_4, netmask, now, ad_reqd,
                               do_bit, have_pseudoheader);
            /* Do this by steam now we're not in the select() loop */
            check_log_writer(1 as libc::c_int);
            if m == 0 as libc::c_int as libc::c_ulong {
                let mut flags_0: libc::c_uint =
                    0 as libc::c_int as libc::c_uint;
                let mut addrp: *mut all_addr = 0 as *mut all_addr;
                let mut type_0: libc::c_int = 16384 as libc::c_int;
                let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut oph: *mut libc::c_uchar =
                    find_pseudoheader(header, size, 0 as *mut size_t,
                                      0 as *mut *mut libc::c_uchar,
                                      0 as *mut libc::c_int,
                                      0 as *mut libc::c_int);
                size =
                    add_edns0_config(header, size,
                                     (header as
                                          *mut libc::c_uchar).offset(65536 as
                                                                         libc::c_int
                                                                         as
                                                                         isize),
                                     &mut peer_addr, now, &mut check_subnet,
                                     &mut cacheable);
                if gotname != 0 {
                    flags_0 =
                        search_servers(now, &mut addrp, gotname,
                                       (*dnsmasq_daemon).namebuff,
                                       &mut type_0, &mut domain,
                                       &mut norebind)
                }
                /* Check if we added a pheader on forwarding - may need to
		 strip it from the reply. */
                if oph.is_null() &&
                       !find_pseudoheader(header, size, 0 as *mut size_t,
                                          0 as *mut *mut libc::c_uchar,
                                          0 as *mut libc::c_int,
                                          0 as *mut libc::c_int).is_null() {
                    added_pheader = 1 as libc::c_int
                }
                type_0 &= !(16384 as libc::c_int);
                if type_0 != 0 as libc::c_int ||
                       (*dnsmasq_daemon).options[(7 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (7 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           != 0 || (*dnsmasq_daemon).last_server.is_null() {
                    last_server = (*dnsmasq_daemon).servers
                } else { last_server = (*dnsmasq_daemon).last_server }
                if flags_0 == 0 && !last_server.is_null() {
                    let mut firstsendto: *mut server = 0 as *mut server;
                    let mut hash: [libc::c_uchar; 32] = [0; 32];
                    memcpy(hash.as_mut_ptr() as *mut libc::c_void,
                           hash_questions(header,
                                          size as libc::c_uint as size_t,
                                          (*dnsmasq_daemon).namebuff) as
                               *const libc::c_void,
                           32 as libc::c_int as libc::c_ulong);
                    let mut current_block_93: u64;
                    's_446:
                        loop 
                             /* Loop round available servers until we succeed in connecting to one.
		     Note that this code subtly ensures that consecutive queries on this connection
		     which can go to the same server, do so. */
                             {
                            let mut data_sent: libc::c_int = 0 as libc::c_int;
                            if firstsendto.is_null() {
                                firstsendto = last_server
                            } else {
                                last_server = (*last_server).next;
                                if last_server.is_null() {
                                    last_server = (*dnsmasq_daemon).servers
                                }
                                if last_server == firstsendto { break ; }
                            }
                            /* server for wrong domain */
                            if type_0 !=
                                   (*last_server).flags &
                                       (8 as libc::c_int | 32 as libc::c_int)
                                   ||
                                   type_0 == 8 as libc::c_int &&
                                       hostname_isequal(domain,
                                                        (*last_server).domain)
                                           == 0 ||
                                   (*last_server).flags &
                                       (4 as libc::c_int |
                                            8192 as libc::c_int) != 0 {
                                continue ;
                            }
                            loop  {
                                *length = __bswap_16(size as __uint16_t);
                                if (*last_server).tcpfd == -(1 as libc::c_int)
                                   {
                                    (*last_server).tcpfd =
                                        socket((*last_server).addr.sa.sa_family
                                                   as libc::c_int,
                                               SOCK_STREAM as libc::c_int,
                                               0 as libc::c_int);
                                    if (*last_server).tcpfd ==
                                           -(1 as libc::c_int) {
                                        continue 's_446 ;
                                    }
                                    if local_bind((*last_server).tcpfd,
                                                  &mut (*last_server).source_addr,
                                                  (*last_server).interface.as_mut_ptr(),
                                                  0 as libc::c_int as
                                                      libc::c_uint,
                                                  1 as libc::c_int) == 0 {
                                        close((*last_server).tcpfd);
                                        (*last_server).tcpfd =
                                            -(1 as libc::c_int);
                                        continue 's_446 ;
                                    } else {
                                        while retry_send(sendto((*last_server).tcpfd,
                                                                packet as
                                                                    *const libc::c_void,
                                                                size.wrapping_add(::std::mem::size_of::<u16_0>()
                                                                                      as
                                                                                      libc::c_ulong),
                                                                MSG_FASTOPEN
                                                                    as
                                                                    libc::c_int,
                                                                __CONST_SOCKADDR_ARG{__sockaddr__:
                                                                                         &mut (*last_server).addr.sa,},
                                                                sa_len(&mut (*last_server).addr)
                                                                    as
                                                                    socklen_t))
                                                  != 0 {
                                        }
                                        if *__errno_location() ==
                                               0 as libc::c_int {
                                            data_sent = 1 as libc::c_int
                                        }
                                        if data_sent == 0 &&
                                               connect((*last_server).tcpfd,
                                                       __CONST_SOCKADDR_ARG{__sockaddr__:
                                                                                &mut (*last_server).addr.sa,},
                                                       sa_len(&mut (*last_server).addr)
                                                           as socklen_t) ==
                                                   -(1 as libc::c_int) {
                                            close((*last_server).tcpfd);
                                            (*last_server).tcpfd =
                                                -(1 as libc::c_int);
                                            continue 's_446 ;
                                        } else {
                                            (*last_server).flags &=
                                                !(32768 as libc::c_int)
                                        }
                                    }
                                }
                                /* get query name again for logging - may have been overwritten */
                                gotname =
                                    extract_request(header,
                                                    size as libc::c_uint as
                                                        size_t,
                                                    (*dnsmasq_daemon).namebuff,
                                                    &mut qtype);
                                if gotname == 0 {
                                    strcpy((*dnsmasq_daemon).namebuff,
                                           b"query\x00" as *const u8 as
                                               *const libc::c_char);
                                }
                                if data_sent == 0 &&
                                       read_write((*last_server).tcpfd,
                                                  packet,
                                                  size.wrapping_add(::std::mem::size_of::<u16_0>()
                                                                        as
                                                                        libc::c_ulong)
                                                      as libc::c_int,
                                                  0 as libc::c_int) == 0 ||
                                       read_write((*last_server).tcpfd,
                                                  &mut c1, 1 as libc::c_int,
                                                  1 as libc::c_int) == 0 ||
                                       read_write((*last_server).tcpfd,
                                                  &mut c2, 1 as libc::c_int,
                                                  1 as libc::c_int) == 0 ||
                                       read_write((*last_server).tcpfd,
                                                  payload,
                                                  (c1 as libc::c_int) <<
                                                      8 as libc::c_int |
                                                      c2 as libc::c_int,
                                                  1 as libc::c_int) == 0 {
                                    close((*last_server).tcpfd);
                                    (*last_server).tcpfd =
                                        -(1 as libc::c_int);
                                    /* We get data then EOF, reopen connection to same server,
			     else try next. This avoids DoS from a server which accepts
			     connections and then closes them. */
                                    if !((*last_server).flags &
                                             32768 as libc::c_int != 0) {
                                        continue 's_446 ;
                                    }
                                } else {
                                    (*last_server).flags |=
                                        32768 as libc::c_int;
                                    m =
                                        ((c1 as libc::c_int) <<
                                             8 as libc::c_int |
                                             c2 as libc::c_int) as size_t;
                                    if (*last_server).addr.sa.sa_family as
                                           libc::c_int == 2 as libc::c_int {
                                        log_query((1 as libc::c_uint) <<
                                                      18 as libc::c_int |
                                                      (1 as libc::c_uint) <<
                                                          7 as libc::c_int |
                                                      (1 as libc::c_uint) <<
                                                          3 as libc::c_int,
                                                  (*dnsmasq_daemon).namebuff,
                                                  &mut (*last_server).addr.in_0.sin_addr
                                                      as *mut in_addr as
                                                      *mut all_addr,
                                                  0 as *mut libc::c_char);
                                    } else {
                                        log_query((1 as libc::c_uint) <<
                                                      18 as libc::c_int |
                                                      (1 as libc::c_uint) <<
                                                          8 as libc::c_int |
                                                      (1 as libc::c_uint) <<
                                                          3 as libc::c_int,
                                                  (*dnsmasq_daemon).namebuff,
                                                  &mut (*last_server).addr.in6.sin6_addr
                                                      as *mut in6_addr as
                                                      *mut all_addr,
                                                  0 as *mut libc::c_char);
                                    }
                                    /* restore CD bit to the value in the query */
                                    if checking_disabled != 0 {
                                        (*header).hb4 =
                                            ((*header).hb4 as libc::c_int |
                                                 0x10 as libc::c_int) as u8_0
                                    } else {
                                        (*header).hb4 =
                                            ((*header).hb4 as libc::c_int &
                                                 !(0x10 as libc::c_int)) as
                                                u8_0
                                    }
                                    /* There's no point in updating the cache, since this process will exit and
			 lose the information after a few queries. We make this call for the alias and 
			 bogus-nxdomain side-effects. */
		      /* If the crc of the question section doesn't match the crc we sent, then
			 someone might be attempting to insert bogus values into the cache by 
			 sending replies containing questions and bogus answers. */
                                    if memcmp(hash.as_mut_ptr() as
                                                  *const libc::c_void,
                                              hash_questions(header,
                                                             m as libc::c_uint
                                                                 as size_t,
                                                             (*dnsmasq_daemon).namebuff)
                                                  as *const libc::c_void,
                                              32 as libc::c_int as
                                                  libc::c_ulong) !=
                                           0 as libc::c_int {
                                        current_block_93 =
                                            9430418855388998878;
                                        break ;
                                    } else {
                                        current_block_93 =
                                            7079180960716815705;
                                        break ;
                                    }
                                }
                            }
                            match current_block_93 {
                                9430418855388998878 => {
                                    m = 0 as libc::c_int as size_t;
                                    break ;
                                }
                                _ => {
                                    /* Never cache answers which are contingent on the source or MAC address EDSN0 option,
			 since the cache is ignorant of such things. */
                                    if cacheable == 0 {
                                        no_cache_dnssec = 1 as libc::c_int
                                    }
                                    m =
                                        process_reply(header, now,
                                                      last_server,
                                                      m as libc::c_uint as
                                                          size_t,
                                                      ((*dnsmasq_daemon).options[(31
                                                                                      as
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
                                                           (1 as libc::c_uint)
                                                               <<
                                                               (31 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_ulong))
                                                           != 0 &&
                                                           norebind == 0) as
                                                          libc::c_int,
                                                      no_cache_dnssec,
                                                      cache_secure,
                                                      bogusanswer, ad_reqd,
                                                      do_bit, added_pheader,
                                                      check_subnet,
                                                      &mut peer_addr);
                                    break ;
                                }
                            }
                        }
                }
                /* In case of local answer or no connections made. */
                if m == 0 as libc::c_int as libc::c_ulong {
                    m =
                        setup_reply(header, size as libc::c_uint as size_t,
                                    addrp, flags_0,
                                    (*dnsmasq_daemon).local_ttl);
                    if have_pseudoheader != 0 {
                        m =
                            add_pseudoheader(header, m,
                                             (header as
                                                  *mut libc::c_uchar).offset(65536
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize),
                                             (*dnsmasq_daemon).edns_pktsz,
                                             0 as libc::c_int,
                                             0 as *mut libc::c_uchar,
                                             0 as libc::c_int as size_t,
                                             do_bit, 0 as libc::c_int)
                    }
                }
            }
        }
        check_log_writer(1 as libc::c_int);
        *length = __bswap_16(m as __uint16_t);
        if m == 0 as libc::c_int as libc::c_ulong ||
               read_write(confd, packet,
                          m.wrapping_add(::std::mem::size_of::<u16_0>() as
                                             libc::c_ulong) as libc::c_int,
                          0 as libc::c_int) == 0 {
            return packet
        }
    };
}
unsafe extern "C" fn allocate_frec(mut now: time_t) -> *mut frec {
    let mut f: *mut frec = 0 as *mut frec;
    f =
        whine_malloc(::std::mem::size_of::<frec>() as libc::c_ulong) as
            *mut frec;
    if !f.is_null() {
        (*f).next = (*dnsmasq_daemon).frec_list;
        (*f).time = now;
        (*f).sentto = 0 as *mut server;
        (*f).rfd4 = 0 as *mut randfd;
        (*f).flags = 0 as libc::c_int;
        (*f).rfd6 = 0 as *mut randfd;
        (*dnsmasq_daemon).frec_list = f
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn allocate_rfd(mut family: libc::c_int)
 -> *mut randfd {
    static mut finger: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    /* limit the number of sockets we have open to avoid starvation of 
     (eg) TFTP. Once we have a reasonable number, randomness should be OK */
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if (*dnsmasq_daemon).randomsocks[i as usize].refcount as libc::c_int
               == 0 as libc::c_int {
            (*dnsmasq_daemon).randomsocks[i as usize].fd =
                random_sock(family);
            if (*dnsmasq_daemon).randomsocks[i as usize].fd ==
                   -(1 as libc::c_int) {
                break ;
            }
            (*dnsmasq_daemon).randomsocks[i as usize].refcount =
                1 as libc::c_int as libc::c_ushort;
            (*dnsmasq_daemon).randomsocks[i as usize].family =
                family as libc::c_ushort;
            return &mut *(*dnsmasq_daemon).randomsocks.as_mut_ptr().offset(i
                                                                               as
                                                                               isize)
                       as *mut randfd
        } else { i += 1 }
    }
    /* No free ones or cannot get new socket, grab an existing one */
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        let mut j: libc::c_int = (i + finger) % 64 as libc::c_int;
        if (*dnsmasq_daemon).randomsocks[j as usize].refcount as libc::c_int
               != 0 as libc::c_int &&
               (*dnsmasq_daemon).randomsocks[j as usize].family as libc::c_int
                   == family &&
               (*dnsmasq_daemon).randomsocks[j as usize].refcount as
                   libc::c_int != 0xffff as libc::c_int {
            finger = j;
            (*dnsmasq_daemon).randomsocks[j as usize].refcount =
                (*dnsmasq_daemon).randomsocks[j as
                                                  usize].refcount.wrapping_add(1);
            return &mut *(*dnsmasq_daemon).randomsocks.as_mut_ptr().offset(j
                                                                               as
                                                                               isize)
                       as *mut randfd
        }
        i += 1
    }
    return 0 as *mut randfd;
    /* doom */
}
#[no_mangle]
pub unsafe extern "C" fn free_rfd(mut rfd: *mut randfd) {
    if !rfd.is_null() &&
           {
               (*rfd).refcount = (*rfd).refcount.wrapping_sub(1);
               ((*rfd).refcount as libc::c_int) == 0 as libc::c_int
           } {
        close((*rfd).fd);
    };
}
unsafe extern "C" fn free_frec(mut f: *mut frec) {
    let mut last: *mut frec_src = 0 as *mut frec_src;
    /* add back to freelist if not the record builtin to every frec. */
    last = (*f).frec_src.next;
    while !last.is_null() && !(*last).next.is_null() { last = (*last).next }
    if !last.is_null() {
        (*last).next = (*dnsmasq_daemon).free_frec_src;
        (*dnsmasq_daemon).free_frec_src = (*f).frec_src.next
    }
    (*f).frec_src.next = 0 as *mut frec_src;
    free_rfd((*f).rfd4);
    (*f).rfd4 = 0 as *mut randfd;
    (*f).sentto = 0 as *mut server;
    (*f).flags = 0 as libc::c_int;
    free_rfd((*f).rfd6);
    (*f).rfd6 = 0 as *mut randfd;
}
/* if wait==NULL return a free or older than TIMEOUT record.
   else return *wait zero if one available, or *wait is delay to
   when the oldest in-use record will expire. Impose an absolute
   limit of 4*TIMEOUT before we wipe things (for random sockets).
   If force is non-NULL, always return a result, even if we have
   to allocate above the limit, and never free the record pointed
   to by the force argument. */
#[no_mangle]
pub unsafe extern "C" fn get_new_frec(mut now: time_t,
                                      mut wait: *mut libc::c_int,
                                      mut force: *mut frec) -> *mut frec {
    let mut f: *mut frec = 0 as *mut frec;
    let mut oldest: *mut frec = 0 as *mut frec;
    let mut target: *mut frec = 0 as *mut frec;
    let mut count: libc::c_int = 0;
    if !wait.is_null() { *wait = 0 as libc::c_int }
    f = (*dnsmasq_daemon).frec_list;
    oldest = 0 as *mut frec;
    target = 0 as *mut frec;
    count = 0 as libc::c_int;
    while !f.is_null() {
        if (*f).sentto.is_null() {
            target = f
        } else {
            if difftime(now, (*f).time) >=
                   (4 as libc::c_int * 10 as libc::c_int) as libc::c_double {
                free_frec(f);
                target = f
            }
            if oldest.is_null() ||
                   difftime((*f).time, (*oldest).time) <=
                       0 as libc::c_int as libc::c_double {
                oldest = f
            }
        }
        f = (*f).next;
        count += 1
    }
    if !target.is_null() { (*target).time = now; return target }
    /* can't find empty one, use oldest if there is one
     and it's older than timeout */
    if force.is_null() && !oldest.is_null() &&
           difftime(now, (*oldest).time) as libc::c_int >= 10 as libc::c_int {
        /* keep stuff for twice timeout if we can by allocating a new
	 record instead */
        if difftime(now, (*oldest).time) <
               (2 as libc::c_int * 10 as libc::c_int) as libc::c_double &&
               count <= (*dnsmasq_daemon).ftabsize &&
               { f = allocate_frec(now); !f.is_null() } {
            return f
        }
        if wait.is_null() { free_frec(oldest); (*oldest).time = now }
        return oldest
    }
    /* none available, calculate time 'till oldest record expires */
    if force.is_null() && count > (*dnsmasq_daemon).ftabsize {
        static mut last_log: time_t = 0 as libc::c_int as time_t;
        if !oldest.is_null() && !wait.is_null() {
            *wait =
                ((*oldest).time + 10 as libc::c_int as time_t - now) as
                    libc::c_int
        }
        if difftime(now, last_log) as libc::c_int > 5 as libc::c_int {
            last_log = now;
            my_syslog(4 as libc::c_int,
                      b"Maximum number of concurrent DNS queries reached (max: %d)\x00"
                          as *const u8 as *const libc::c_char,
                      (*dnsmasq_daemon).ftabsize);
        }
        return 0 as *mut frec
    }
    f = allocate_frec(now);
    if f.is_null() && !wait.is_null() {
        /* wait one second on malloc failure */
        *wait = 1 as libc::c_int
    }
    return f;
    /* OK if malloc fails and this is NULL */
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
unsafe extern "C" fn lookup_frec(mut id: libc::c_ushort, mut fd: libc::c_int,
                                 mut family: libc::c_int,
                                 mut hash: *mut libc::c_void) -> *mut frec {
    let mut f: *mut frec = 0 as *mut frec;
    f = (*dnsmasq_daemon).frec_list;
    while !f.is_null() {
        if !(*f).sentto.is_null() &&
               (*f).new_id as libc::c_int == id as libc::c_int &&
               memcmp(hash, (*f).hash.as_mut_ptr() as *const libc::c_void,
                      32 as libc::c_int as libc::c_ulong) == 0 as libc::c_int
           {
            /* sent from random port */
            if family == 2 as libc::c_int && !(*f).rfd4.is_null() &&
                   (*(*f).rfd4).fd == fd {
                return f
            }
            if family == 10 as libc::c_int && !(*f).rfd6.is_null() &&
                   (*(*f).rfd6).fd == fd {
                return f
            }
            /* sent to upstream from bound socket. */
            if !(*(*f).sentto).sfd.is_null() && (*(*(*f).sentto).sfd).fd == fd
               {
                return f
            }
        }
        f = (*f).next
    }
    return 0 as *mut frec;
}
unsafe extern "C" fn lookup_frec_by_sender(mut id: libc::c_ushort,
                                           mut addr: *mut mysockaddr,
                                           mut hash: *mut libc::c_void)
 -> *mut frec {
    let mut f: *mut frec = 0 as *mut frec;
    let mut src: *mut frec_src = 0 as *mut frec_src;
    f = (*dnsmasq_daemon).frec_list;
    while !f.is_null() {
        if !(*f).sentto.is_null() &&
               (*f).flags & (8 as libc::c_int | 16 as libc::c_int) == 0 &&
               memcmp(hash, (*f).hash.as_mut_ptr() as *const libc::c_void,
                      32 as libc::c_int as libc::c_ulong) == 0 as libc::c_int
           {
            src = &mut (*f).frec_src;
            while !src.is_null() {
                if (*src).orig_id as libc::c_int == id as libc::c_int &&
                       sockaddr_isequal(&mut (*src).source, addr) != 0 {
                    return f
                }
                src = (*src).next
            }
        }
        f = (*f).next
    }
    return 0 as *mut frec;
}
unsafe extern "C" fn lookup_frec_by_query(mut hash: *mut libc::c_void,
                                          mut flags: libc::c_uint)
 -> *mut frec {
    let mut f: *mut frec = 0 as *mut frec;
    /* FREC_DNSKEY and FREC_DS_QUERY are never set in flags, so the test below 
     ensures that no frec created for internal DNSSEC query can be returned here.
     
     Similarly FREC_NO_CACHE is never set in flags, so a query which is
     contigent on a particular source address EDNS0 option will never be matched. */
    f = (*dnsmasq_daemon).frec_list;
    while !f.is_null() {
        if !(*f).sentto.is_null() &&
               ((*f).flags &
                    (2 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int
                         | 1024 as libc::c_int | 8 as libc::c_int |
                         16 as libc::c_int | 2048 as libc::c_int)) as
                   libc::c_uint == flags &&
               memcmp(hash, (*f).hash.as_mut_ptr() as *const libc::c_void,
                      32 as libc::c_int as libc::c_ulong) == 0 as libc::c_int
           {
            return f
        }
        f = (*f).next
    }
    return 0 as *mut frec;
}
/* Send query packet again, if we can. */
#[no_mangle]
pub unsafe extern "C" fn resend_query() {
    if !(*dnsmasq_daemon).srv_save.is_null() {
        let mut fd: libc::c_int = 0;
        if !(*(*dnsmasq_daemon).srv_save).sfd.is_null() {
            fd = (*(*(*dnsmasq_daemon).srv_save).sfd).fd
        } else if !(*dnsmasq_daemon).rfd_save.is_null() &&
                      (*(*dnsmasq_daemon).rfd_save).refcount as libc::c_int !=
                          0 as libc::c_int {
            fd = (*(*dnsmasq_daemon).rfd_save).fd
        } else { return }
        while retry_send(sendto(fd,
                                (*dnsmasq_daemon).packet as
                                    *const libc::c_void,
                                (*dnsmasq_daemon).packet_len,
                                0 as libc::c_int,
                                __CONST_SOCKADDR_ARG{__sockaddr__:
                                                         &mut (*(*dnsmasq_daemon).srv_save).addr.sa,},
                                sa_len(&mut (*(*dnsmasq_daemon).srv_save).addr)
                                    as socklen_t)) != 0 {
        }
    };
}
/* A server record is going away, remove references to it */
#[no_mangle]
pub unsafe extern "C" fn server_gone(mut server: *mut server) {
    let mut f: *mut frec = 0 as *mut frec;
    f = (*dnsmasq_daemon).frec_list;
    while !f.is_null() {
        if !(*f).sentto.is_null() && (*f).sentto == server { free_frec(f); }
        f = (*f).next
    }
    if (*dnsmasq_daemon).last_server == server {
        (*dnsmasq_daemon).last_server = 0 as *mut server
    }
    if (*dnsmasq_daemon).srv_save == server {
        (*dnsmasq_daemon).srv_save = 0 as *mut server
    };
}
/* return unique random ids. */
unsafe extern "C" fn get_id() -> libc::c_ushort {
    let mut ret: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let mut f: *mut frec = 0 as *mut frec;
    loop  {
        ret = rand16();
        /* ensure id is unique. */
        f = (*dnsmasq_daemon).frec_list;
        while !f.is_null() {
            if !(*f).sentto.is_null() &&
                   (*f).new_id as libc::c_int == ret as libc::c_int {
                break ;
            }
            f = (*f).next
        }
        if f.is_null() { return ret }
    };
}
