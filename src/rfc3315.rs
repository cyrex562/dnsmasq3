#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> libc::c_int;
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
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn reset_counter();
    #[no_mangle]
    fn match_bytes(o: *mut dhcp_opt, p: *mut libc::c_uchar, len: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn strip_hostname(hostname: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn find_config(configs: *mut dhcp_config, context: *mut dhcp_context,
                   clid: *mut libc::c_uchar, clid_len: libc::c_int,
                   hwaddr: *mut libc::c_uchar, hw_len: libc::c_int,
                   hw_type: libc::c_int, hostname: *mut libc::c_char,
                   filter: *mut dhcp_netid) -> *mut dhcp_config;
    #[no_mangle]
    fn put_opt6_char(val: libc::c_uint);
    #[no_mangle]
    fn run_tag_if(tags: *mut dhcp_netid) -> *mut dhcp_netid;
    #[no_mangle]
    fn match_netid(check: *mut dhcp_netid, pool: *mut dhcp_netid,
                   tagnotneeded: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn expand(headroom: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn option_filter(tags: *mut dhcp_netid, context_tags: *mut dhcp_netid,
                     opts: *mut dhcp_opt) -> *mut dhcp_netid;
    #[no_mangle]
    fn put_opt6_long(val: libc::c_uint);
    #[no_mangle]
    fn put_opt6_short(val: libc::c_uint);
    #[no_mangle]
    fn put_opt6_string(s: *mut libc::c_char);
    #[no_mangle]
    fn log_tags(netid: *mut dhcp_netid, xid: u32_0);
    #[no_mangle]
    fn save_counter(newval: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn put_opt6(data: *mut libc::c_void, len: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn end_opt6(container: libc::c_int);
    #[no_mangle]
    fn new_opt6(opt: libc::c_int) -> libc::c_int;
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
    fn get_domain6(addr: *mut in6_addr) -> *mut libc::c_char;
    #[no_mangle]
    fn rand16() -> libc::c_ushort;
    #[no_mangle]
    fn legal_hostname(name: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn do_rfc1035_name(p: *mut libc::c_uchar, sval: *mut libc::c_char,
                       limit: *mut libc::c_char) -> *mut libc::c_uchar;
    #[no_mangle]
    fn hostname_isequal(a: *const libc::c_char, b: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn is_same_net6(a: *mut in6_addr, b: *mut in6_addr,
                    prefixlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn addr6part(addr: *mut in6_addr) -> u64_0;
    #[no_mangle]
    fn setaddr6part(addr: *mut in6_addr, host: u64_0);
    #[no_mangle]
    fn prettyprint_time(buf: *mut libc::c_char, t: libc::c_uint);
    #[no_mangle]
    fn memcmp_masked(a: *mut libc::c_uchar, b: *mut libc::c_uchar,
                     len: libc::c_int, mask: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn print_mac(buff: *mut libc::c_char, mac: *mut libc::c_uchar,
                 len: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn wildcard_match(wildcard: *const libc::c_char,
                      match_0: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn my_syslog(priority: libc::c_int, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn option_string(prot: libc::c_int, opt: libc::c_uint,
                     val: *mut libc::c_uchar, opt_len: libc::c_int,
                     buf: *mut libc::c_char, buf_len: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn send_from(fd: libc::c_int, nowild: libc::c_int,
                 packet: *mut libc::c_char, len: size_t, to: *mut mysockaddr,
                 source: *mut all_addr, iface: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn lease6_allocate(addrp: *mut in6_addr, lease_type: libc::c_int)
     -> *mut dhcp_lease;
    #[no_mangle]
    fn lease6_find(clid: *mut libc::c_uchar, clid_len: libc::c_int,
                   lease_type: libc::c_int, iaid: libc::c_uint,
                   addr: *mut in6_addr) -> *mut dhcp_lease;
    #[no_mangle]
    fn lease6_reset();
    #[no_mangle]
    fn lease6_find_by_client(first: *mut dhcp_lease, lease_type: libc::c_int,
                             clid: *mut libc::c_uchar, clid_len: libc::c_int,
                             iaid: libc::c_uint) -> *mut dhcp_lease;
    #[no_mangle]
    fn lease6_find_by_addr(net: *mut in6_addr, prefix: libc::c_int,
                           addr: u64_0) -> *mut dhcp_lease;
    #[no_mangle]
    fn lease_set_iaid(lease: *mut dhcp_lease, iaid: libc::c_uint);
    #[no_mangle]
    fn lease_set_hwaddr(lease: *mut dhcp_lease, hwaddr: *const libc::c_uchar,
                        clid: *const libc::c_uchar, hw_len: libc::c_int,
                        hw_type: libc::c_int, clid_len: libc::c_int,
                        now: time_t, force: libc::c_int);
    #[no_mangle]
    fn lease_set_hostname(lease: *mut dhcp_lease, name: *const libc::c_char,
                          auth: libc::c_int, domain: *mut libc::c_char,
                          config_domain: *mut libc::c_char);
    #[no_mangle]
    fn lease_set_expires(lease: *mut dhcp_lease, len: libc::c_uint,
                         now: time_t);
    #[no_mangle]
    fn lease_set_interface(lease: *mut dhcp_lease, interface: libc::c_int,
                           now: time_t);
    #[no_mangle]
    fn lease_prune(target: *mut dhcp_lease, now: time_t);
    #[no_mangle]
    fn lease_add_extradata(lease: *mut dhcp_lease, data: *mut libc::c_uchar,
                           len: libc::c_uint, delim: libc::c_int);
    #[no_mangle]
    fn address6_allocate(context: *mut dhcp_context, clid: *mut libc::c_uchar,
                         clid_len: libc::c_int, temp_addr: libc::c_int,
                         iaid: libc::c_uint, serial: libc::c_int,
                         netids: *mut dhcp_netid, plain_range: libc::c_int,
                         ans: *mut in6_addr) -> *mut dhcp_context;
    #[no_mangle]
    fn address6_available(context: *mut dhcp_context, taddr: *mut in6_addr,
                          netids: *mut dhcp_netid, plain_range: libc::c_int)
     -> *mut dhcp_context;
    #[no_mangle]
    fn address6_valid(context: *mut dhcp_context, taddr: *mut in6_addr,
                      netids: *mut dhcp_netid, plain_range: libc::c_int)
     -> *mut dhcp_context;
    #[no_mangle]
    fn get_client_mac(client: *mut in6_addr, iface: libc::c_int,
                      mac: *mut libc::c_uchar, maclenp: *mut libc::c_uint,
                      mactypep: *mut libc::c_uint, now: time_t);
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
pub type u16_0 = libc::c_ushort;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
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
    pub cname: C2RustUnnamed_5,
    pub key: C2RustUnnamed_4,
    pub ds: C2RustUnnamed_3,
    pub srv: C2RustUnnamed_2,
    pub log: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub keytag: libc::c_ushort,
    pub algo: libc::c_ushort,
    pub digest: libc::c_ushort,
    pub rcode: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub flags: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub target: C2RustUnnamed_6,
    pub uid: libc::c_uint,
    pub is_name_ptr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
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
    pub name: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
    pub u: C2RustUnnamed_8,
    pub val: *mut libc::c_uchar,
    pub netid: *mut dhcp_netid,
    pub next: *mut dhcp_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
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
pub struct state {
    pub clid: *mut libc::c_uchar,
    pub clid_len: libc::c_int,
    pub ia_type: libc::c_int,
    pub interface: libc::c_int,
    pub hostname_auth: libc::c_int,
    pub lease_allocate: libc::c_int,
    pub client_hostname: *mut libc::c_char,
    pub hostname: *mut libc::c_char,
    pub domain: *mut libc::c_char,
    pub send_domain: *mut libc::c_char,
    pub context: *mut dhcp_context,
    pub link_address: *mut in6_addr,
    pub fallback: *mut in6_addr,
    pub ll_addr: *mut in6_addr,
    pub ula_addr: *mut in6_addr,
    pub xid: libc::c_uint,
    pub fqdn_flags: libc::c_uint,
    pub iaid: libc::c_uint,
    pub iface_name: *mut libc::c_char,
    pub packet_options: *mut libc::c_void,
    pub end: *mut libc::c_void,
    pub tags: *mut dhcp_netid,
    pub context_tags: *mut dhcp_netid,
    pub mac: [libc::c_uchar; 16],
    pub mac_len: libc::c_uint,
    pub mac_type: libc::c_uint,
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
#[no_mangle]
pub unsafe extern "C" fn dhcp6_reply(mut context: *mut dhcp_context,
                                     mut interface: libc::c_int,
                                     mut iface_name: *mut libc::c_char,
                                     mut fallback: *mut in6_addr,
                                     mut ll_addr: *mut in6_addr,
                                     mut ula_addr: *mut in6_addr,
                                     mut sz: size_t,
                                     mut client_addr: *mut in6_addr,
                                     mut now: time_t) -> libc::c_ushort {
    let mut vendor: *mut dhcp_vendor = 0 as *mut dhcp_vendor;
    let mut msg_type: libc::c_int = 0;
    let mut state: state =
        state{clid: 0 as *mut libc::c_uchar,
              clid_len: 0,
              ia_type: 0,
              interface: 0,
              hostname_auth: 0,
              lease_allocate: 0,
              client_hostname: 0 as *mut libc::c_char,
              hostname: 0 as *mut libc::c_char,
              domain: 0 as *mut libc::c_char,
              send_domain: 0 as *mut libc::c_char,
              context: 0 as *mut dhcp_context,
              link_address: 0 as *mut in6_addr,
              fallback: 0 as *mut in6_addr,
              ll_addr: 0 as *mut in6_addr,
              ula_addr: 0 as *mut in6_addr,
              xid: 0,
              fqdn_flags: 0,
              iaid: 0,
              iface_name: 0 as *mut libc::c_char,
              packet_options: 0 as *mut libc::c_void,
              end: 0 as *mut libc::c_void,
              tags: 0 as *mut dhcp_netid,
              context_tags: 0 as *mut dhcp_netid,
              mac: [0; 16],
              mac_len: 0,
              mac_type: 0,};
    if sz <= 4 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_ushort
    }
    msg_type =
        *((*dnsmasq_daemon).dhcp_packet.iov_base as *mut libc::c_uchar) as
            libc::c_int;
    /* Mark these so we only match each at most once, to avoid tangled linked lists */
    vendor = (*dnsmasq_daemon).dhcp_vendors;
    while !vendor.is_null() {
        (*vendor).netid.next = &mut (*vendor).netid;
        vendor = (*vendor).next
    }
    reset_counter();
    state.context = context;
    state.interface = interface;
    state.iface_name = iface_name;
    state.fallback = fallback;
    state.ll_addr = ll_addr;
    state.ula_addr = ula_addr;
    state.mac_len = 0 as libc::c_int as libc::c_uint;
    state.tags = 0 as *mut dhcp_netid;
    state.link_address = 0 as *mut in6_addr;
    if dhcp6_maybe_relay(&mut state, (*dnsmasq_daemon).dhcp_packet.iov_base,
                         sz, client_addr,
                         (*(client_addr as
                                *const uint8_t).offset(0 as libc::c_int as
                                                           isize) as
                              libc::c_int == 0xff as libc::c_int) as
                             libc::c_int, now) != 0 {
        return if msg_type == 12 as libc::c_int {
                   547 as libc::c_int
               } else { 546 as libc::c_int } as libc::c_ushort
    }
    return 0 as libc::c_int as libc::c_ushort;
}
/* This cost me blood to write, it will probably cost you blood to understand - srk. */
unsafe extern "C" fn dhcp6_maybe_relay(mut state: *mut state,
                                       mut inbuff: *mut libc::c_void,
                                       mut sz: size_t,
                                       mut client_addr: *mut in6_addr,
                                       mut is_unicast: libc::c_int,
                                       mut now: time_t) -> libc::c_int {
    let mut end: *mut libc::c_void = inbuff.offset(sz as isize);
    let mut opts: *mut libc::c_void =
        inbuff.offset(34 as libc::c_int as isize);
    let mut msg_type: libc::c_int =
        *(inbuff as *mut libc::c_uchar) as libc::c_int;
    let mut outmsgtypep: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut opt: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut vendor: *mut dhcp_vendor = 0 as *mut dhcp_vendor;
    /* if not an encapsulated relayed message, just do the stuff */
    if msg_type != 12 as libc::c_int {
        /* if link_address != NULL if points to the link address field of the 
	 innermost nested RELAYFORW message, which is where we find the
	 address of the network on which we can allocate an address.
	 Recalculate the available contexts using that information. 

      link_address == NULL means there's no relay in use, so we try and find the client's 
      MAC address from the local ND cache. */
        if (*state).link_address.is_null() {
            get_client_mac(client_addr, (*state).interface,
                           (*state).mac.as_mut_ptr(), &mut (*state).mac_len,
                           &mut (*state).mac_type, now);
        } else {
            let mut c: *mut dhcp_context = 0 as *mut dhcp_context;
            let mut share: *mut shared_network = 0 as *mut shared_network;
            (*state).context = 0 as *mut dhcp_context;
            if ({
                    let mut __a: *const in6_addr =
                        (*state).link_address as *const in6_addr;
                    ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                         0 as libc::c_int as libc::c_uint &&
                         (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize]
                             == 0 as libc::c_int as libc::c_uint &&
                         (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize]
                             == 0 as libc::c_int as libc::c_uint &&
                         (*__a).__in6_u.__u6_addr32[3 as libc::c_int as usize]
                             == __bswap_32(1 as libc::c_int as __uint32_t)) as
                        libc::c_int
                }) == 0 &&
                   ({
                        let mut __a: *const in6_addr =
                            (*state).link_address as *const in6_addr;
                        ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                             & __bswap_32(0xffc00000 as libc::c_uint) ==
                             __bswap_32(0xfe800000 as libc::c_uint)) as
                            libc::c_int
                    }) == 0 &&
                   !(*((*state).link_address as
                           *const uint8_t).offset(0 as libc::c_int as isize)
                         as libc::c_int == 0xff as libc::c_int) {
                c = (*dnsmasq_daemon).dhcp6;
                while !c.is_null() {
                    share = (*dnsmasq_daemon).shared_networks;
                    while !share.is_null() {
                        if !((*share).shared_addr.s_addr !=
                                 0 as libc::c_int as libc::c_uint) {
                            if !((*share).if_index != 0 as libc::c_int ||
                                     ({
                                          let mut __a: *const in6_addr =
                                              (*state).link_address as
                                                  *const in6_addr;
                                          let mut __b: *const in6_addr =
                                              &mut (*share).match_addr6 as
                                                  *mut in6_addr as
                                                  *const in6_addr;
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
                                      }) == 0) {
                                if (*c).flags as libc::c_uint &
                                       (1 as libc::c_uint) << 8 as libc::c_int
                                       != 0 &&
                                       (*c).flags as libc::c_uint &
                                           ((1 as libc::c_uint) <<
                                                10 as libc::c_int |
                                                (1 as libc::c_uint) <<
                                                    16 as libc::c_int) == 0 &&
                                       is_same_net6(&mut (*share).shared_addr6,
                                                    &mut (*c).start6,
                                                    (*c).prefix) != 0 &&
                                       is_same_net6(&mut (*share).shared_addr6,
                                                    &mut (*c).end6,
                                                    (*c).prefix) != 0 {
                                    break ;
                                }
                            }
                        }
                        share = (*share).next
                    }
                    if !share.is_null() ||
                           (*c).flags as libc::c_uint &
                               (1 as libc::c_uint) << 8 as libc::c_int != 0 &&
                               (*c).flags as libc::c_uint &
                                   ((1 as libc::c_uint) << 10 as libc::c_int |
                                        (1 as libc::c_uint) <<
                                            16 as libc::c_int) == 0 &&
                               is_same_net6((*state).link_address,
                                            &mut (*c).start6, (*c).prefix) !=
                                   0 &&
                               is_same_net6((*state).link_address,
                                            &mut (*c).end6, (*c).prefix) != 0
                       {
                        (*c).valid = 0xffffffff as libc::c_uint;
                        (*c).preferred = (*c).valid;
                        (*c).current = (*state).context;
                        (*state).context = c
                    }
                    c = (*c).next
                }
            }
            if (*state).context.is_null() {
                inet_ntop(10 as libc::c_int,
                          (*state).link_address as *const libc::c_void,
                          (*dnsmasq_daemon).addrbuff,
                          46 as libc::c_int as socklen_t);
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              4 as libc::c_int,
                          b"no address range available for DHCPv6 request from relay at %s\x00"
                              as *const u8 as *const libc::c_char,
                          (*dnsmasq_daemon).addrbuff);
                return 0 as libc::c_int
            }
        }
        if (*state).context.is_null() {
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          4 as libc::c_int,
                      b"no address range available for DHCPv6 request via %s\x00"
                          as *const u8 as *const libc::c_char,
                      (*state).iface_name);
            return 0 as libc::c_int
        }
        return dhcp6_no_relay(state, msg_type, inbuff, sz, is_unicast, now)
    }
    /* must have at least msg_type+hopcount+link_address+peer_address+minimal size option
     which is               1   +    1   +    16      +     16     + 2 + 2 = 38 */
    if sz < 38 as libc::c_int as libc::c_ulong { return 0 as libc::c_int }
    /* copy header stuff into reply message and set type to reply */
    outmsgtypep =
        put_opt6(inbuff, 34 as libc::c_int as size_t) as *mut libc::c_uchar;
    if outmsgtypep.is_null() { return 0 as libc::c_int }
    *outmsgtypep = 13 as libc::c_int as libc::c_uchar;
    let mut current_block_36: u64;
    /* look for relay options and set tags if found. */
    vendor = (*dnsmasq_daemon).dhcp_vendors;
    while !vendor.is_null() {
        let mut mopt: libc::c_int = 0;
        if (*vendor).match_type == 5 as libc::c_int {
            mopt = 38 as libc::c_int;
            current_block_36 = 2543120759711851213;
        } else if (*vendor).match_type == 4 as libc::c_int {
            mopt = 37 as libc::c_int;
            current_block_36 = 2543120759711851213;
        } else { current_block_36 = 4090602189656566074; }
        match current_block_36 {
            2543120759711851213 => {
                opt =
                    opt6_find(opts, end, mopt as libc::c_uint,
                              1 as libc::c_int as libc::c_uint);
                if !opt.is_null() &&
                       (*vendor).len ==
                           opt6_uint(opt as *mut libc::c_uchar,
                                     -(2 as libc::c_int), 2 as libc::c_int) as
                               libc::c_int &&
                       memcmp((*vendor).data as *const libc::c_void,
                              &mut *(opt as
                                         *mut libc::c_uchar).offset((4 as
                                                                         libc::c_int
                                                                         +
                                                                         0 as
                                                                             libc::c_int)
                                                                        as
                                                                        isize)
                                  as *mut libc::c_uchar as *mut libc::c_void,
                              (*vendor).len as libc::c_ulong) ==
                           0 as libc::c_int &&
                       (*vendor).netid.next !=
                           &mut (*vendor).netid as *mut dhcp_netid {
                    (*vendor).netid.next = (*state).tags;
                    (*state).tags = &mut (*vendor).netid;
                    break ;
                }
            }
            _ => { }
        }
        vendor = (*vendor).next
    }
    /* RFC-6939 */
    opt =
        opt6_find(opts, end, 79 as libc::c_int as libc::c_uint,
                  3 as libc::c_int as libc::c_uint);
    if !opt.is_null() {
        if opt6_uint(opt as *mut libc::c_uchar, -(2 as libc::c_int),
                     2 as libc::c_int) as libc::c_int - 2 as libc::c_int >
               16 as libc::c_int {
            return 0 as libc::c_int
        }
        (*state).mac_type =
            opt6_uint(opt as *mut libc::c_uchar, 0 as libc::c_int,
                      2 as libc::c_int);
        (*state).mac_len =
            (opt6_uint(opt as *mut libc::c_uchar, -(2 as libc::c_int),
                       2 as libc::c_int) as libc::c_int - 2 as libc::c_int) as
                libc::c_uint;
        memcpy(&mut *(*state).mac.as_mut_ptr().offset(0 as libc::c_int as
                                                          isize) as
                   *mut libc::c_uchar as *mut libc::c_void,
               &mut *(opt as
                          *mut libc::c_uchar).offset((4 as libc::c_int +
                                                          2 as libc::c_int) as
                                                         isize) as
                   *mut libc::c_uchar as *mut libc::c_void,
               (*state).mac_len as libc::c_ulong);
    }
    opt = opts;
    while !opt.is_null() {
        if (&mut *(opt as
                       *mut libc::c_uchar).offset((4 as libc::c_int +
                                                       0 as libc::c_int) as
                                                      isize) as
                *mut libc::c_uchar as
                *mut libc::c_void).offset(opt6_uint(opt as *mut libc::c_uchar,
                                                    -(2 as libc::c_int),
                                                    2 as libc::c_int) as
                                              libc::c_int as isize) > end {
            return 0 as libc::c_int
        }
        /* Don't copy MAC address into reply. */
        if opt6_uint(opt as *mut libc::c_uchar, -(4 as libc::c_int),
                     2 as libc::c_int) != 79 as libc::c_int as libc::c_uint {
            let mut o: libc::c_int =
                new_opt6(opt6_uint(opt as *mut libc::c_uchar,
                                   -(4 as libc::c_int), 2 as libc::c_int) as
                             libc::c_int);
            if opt6_uint(opt as *mut libc::c_uchar, -(4 as libc::c_int),
                         2 as libc::c_int) == 9 as libc::c_int as libc::c_uint
               {
                let mut align: in6_addr =
                    in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
                /* the packet data is unaligned, copy to aligned storage */
                memcpy(&mut align as *mut in6_addr as *mut libc::c_void,
                       inbuff.offset(2 as libc::c_int as isize),
                       16 as libc::c_int as libc::c_ulong);
                (*state).link_address = &mut align;
                /* zero is_unicast since that is now known to refer to the 
		 relayed packet, not the original sent by the client */
                if dhcp6_maybe_relay(state,
                                     &mut *(opt as
                                                *mut libc::c_uchar).offset((4
                                                                                as
                                                                                libc::c_int
                                                                                +
                                                                                0
                                                                                    as
                                                                                    libc::c_int)
                                                                               as
                                                                               isize)
                                         as *mut libc::c_uchar as
                                         *mut libc::c_void,
                                     opt6_uint(opt as *mut libc::c_uchar,
                                               -(2 as libc::c_int),
                                               2 as libc::c_int) as
                                         libc::c_int as size_t, client_addr,
                                     0 as libc::c_int, now) == 0 {
                    return 0 as libc::c_int
                }
            } else {
                put_opt6(&mut *(opt as
                                    *mut libc::c_uchar).offset((4 as
                                                                    libc::c_int
                                                                    +
                                                                    0 as
                                                                        libc::c_int)
                                                                   as isize)
                             as *mut libc::c_uchar as *mut libc::c_void,
                         opt6_uint(opt as *mut libc::c_uchar,
                                   -(2 as libc::c_int), 2 as libc::c_int) as
                             libc::c_int as
                             size_t); /* default to send if we receive no FQDN option */
            }
            end_opt6(o);
        }
        opt = opt6_next(opt, end)
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn dhcp6_no_relay(mut state: *mut state,
                                    mut msg_type: libc::c_int,
                                    mut inbuff: *mut libc::c_void,
                                    mut sz: size_t,
                                    mut is_unicast: libc::c_int,
                                    mut now: time_t) -> libc::c_int {
    let mut opt: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut o: libc::c_int = 0;
    let mut o1: libc::c_int = 0;
    let mut start_opts: libc::c_int = 0;
    let mut opt_cfg: *mut dhcp_opt = 0 as *mut dhcp_opt;
    let mut tagif: *mut dhcp_netid = 0 as *mut dhcp_netid;
    let mut config: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut known_id: dhcp_netid =
        dhcp_netid{net: 0 as *mut libc::c_char, next: 0 as *mut dhcp_netid,};
    let mut iface_id: dhcp_netid =
        dhcp_netid{net: 0 as *mut libc::c_char, next: 0 as *mut dhcp_netid,};
    let mut v6_id: dhcp_netid =
        dhcp_netid{net: 0 as *mut libc::c_char, next: 0 as *mut dhcp_netid,};
    let mut outmsgtypep: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vendor: *mut dhcp_vendor = 0 as *mut dhcp_vendor;
    let mut context_tmp: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut mac_opt: *mut dhcp_mac = 0 as *mut dhcp_mac;
    let mut ignore: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    (*state).packet_options = inbuff.offset(4 as libc::c_int as isize);
    (*state).end = inbuff.offset(sz as isize);
    (*state).clid = 0 as *mut libc::c_uchar;
    (*state).clid_len = 0 as libc::c_int;
    (*state).lease_allocate = 0 as libc::c_int;
    (*state).context_tags = 0 as *mut dhcp_netid;
    (*state).domain = 0 as *mut libc::c_char;
    (*state).send_domain = 0 as *mut libc::c_char;
    (*state).hostname_auth = 0 as libc::c_int;
    (*state).hostname = 0 as *mut libc::c_char;
    (*state).client_hostname = 0 as *mut libc::c_char;
    (*state).fqdn_flags = 0x1 as libc::c_int as libc::c_uint;
    /* set tag with name == interface */
    iface_id.net = (*state).iface_name;
    iface_id.next = (*state).tags;
    (*state).tags = &mut iface_id;
    /* set tag "dhcpv6" */
    v6_id.net =
        b"dhcpv6\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    v6_id.next = (*state).tags;
    (*state).tags = &mut v6_id;
    /* copy over transaction-id, and save pointer to message type */
    outmsgtypep =
        put_opt6(inbuff, 4 as libc::c_int as size_t) as *mut libc::c_uchar;
    if outmsgtypep.is_null() { return 0 as libc::c_int }
    start_opts = save_counter(-(1 as libc::c_int));
    (*state).xid =
        (*outmsgtypep.offset(3 as libc::c_int as isize) as libc::c_int |
             (*outmsgtypep.offset(2 as libc::c_int as isize) as libc::c_int)
                 << 8 as libc::c_int |
             (*outmsgtypep.offset(1 as libc::c_int as isize) as libc::c_int)
                 << 16 as libc::c_int) as libc::c_uint;
    /* We're going to be linking tags from all context we use. 
     mark them as unused so we don't link one twice and break the list */
    context_tmp = (*state).context;
    while !context_tmp.is_null() {
        (*context_tmp).netid.next = &mut (*context_tmp).netid;
        if (*dnsmasq_daemon).options[(28 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (28 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
            inet_ntop(10 as libc::c_int,
                      &mut (*context_tmp).start6 as *mut in6_addr as
                          *const libc::c_void, (*dnsmasq_daemon).dhcp_buff,
                      46 as libc::c_int as socklen_t);
            inet_ntop(10 as libc::c_int,
                      &mut (*context_tmp).end6 as *mut in6_addr as
                          *const libc::c_void, (*dnsmasq_daemon).dhcp_buff2,
                      46 as libc::c_int as socklen_t);
            if (*context_tmp).flags as libc::c_uint &
                   (1 as libc::c_uint) << 0 as libc::c_int != 0 {
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              6 as libc::c_int,
                          b"%u available DHCPv6 subnet: %s/%d\x00" as
                              *const u8 as *const libc::c_char, (*state).xid,
                          (*dnsmasq_daemon).dhcp_buff, (*context_tmp).prefix);
            } else {
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              6 as libc::c_int,
                          b"%u available DHCP range: %s -- %s\x00" as
                              *const u8 as *const libc::c_char, (*state).xid,
                          (*dnsmasq_daemon).dhcp_buff,
                          (*dnsmasq_daemon).dhcp_buff2);
            }
        }
        context_tmp = (*context_tmp).current
    }
    opt =
        opt6_find((*state).packet_options, (*state).end,
                  1 as libc::c_int as libc::c_uint,
                  1 as libc::c_int as libc::c_uint);
    if !opt.is_null() {
        (*state).clid =
            &mut *(opt as
                       *mut libc::c_uchar).offset((4 as libc::c_int +
                                                       0 as libc::c_int) as
                                                      isize) as
                *mut libc::c_uchar as *mut libc::c_void as *mut libc::c_uchar;
        (*state).clid_len =
            opt6_uint(opt as *mut libc::c_uchar, -(2 as libc::c_int),
                      2 as libc::c_int) as libc::c_int;
        o = new_opt6(1 as libc::c_int);
        put_opt6((*state).clid as *mut libc::c_void,
                 (*state).clid_len as size_t);
        end_opt6(o);
    } else if msg_type != 11 as libc::c_int { return 0 as libc::c_int }
    /* server-id must match except for SOLICIT, CONFIRM and REBIND messages */
    if msg_type != 1 as libc::c_int && msg_type != 4 as libc::c_int &&
           msg_type != 11 as libc::c_int && msg_type != 6 as libc::c_int &&
           {
               opt =
                   opt6_find((*state).packet_options, (*state).end,
                             2 as libc::c_int as libc::c_uint,
                             1 as libc::c_int as libc::c_uint);
               (opt.is_null() ||
                    opt6_uint(opt as *mut libc::c_uchar, -(2 as libc::c_int),
                              2 as libc::c_int) as libc::c_int !=
                        (*dnsmasq_daemon).duid_len) ||
                   memcmp(&mut *(opt as
                                     *mut libc::c_uchar).offset((4 as
                                                                     libc::c_int
                                                                     +
                                                                     0 as
                                                                         libc::c_int)
                                                                    as isize)
                              as *mut libc::c_uchar as *mut libc::c_void,
                          (*dnsmasq_daemon).duid as *const libc::c_void,
                          (*dnsmasq_daemon).duid_len as libc::c_ulong) !=
                       0 as libc::c_int
           } {
        return 0 as libc::c_int
    }
    o = new_opt6(2 as libc::c_int);
    put_opt6((*dnsmasq_daemon).duid as *mut libc::c_void,
             (*dnsmasq_daemon).duid_len as size_t);
    end_opt6(o);
    if is_unicast != 0 &&
           (msg_type == 3 as libc::c_int || msg_type == 5 as libc::c_int ||
                msg_type == 8 as libc::c_int || msg_type == 9 as libc::c_int)
       {
        *outmsgtypep = 7 as libc::c_int as libc::c_uchar;
        o1 = new_opt6(13 as libc::c_int);
        put_opt6_short(5 as libc::c_int as libc::c_uint);
        put_opt6_string(b"Use multicast\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
        end_opt6(o1);
        return 1 as libc::c_int
    }
    let mut current_block_64: u64;
    /* match vendor and user class options */
    vendor = (*dnsmasq_daemon).dhcp_vendors;
    while !vendor.is_null() {
        let mut mopt: libc::c_int = 0;
        if (*vendor).match_type == 1 as libc::c_int {
            mopt = 16 as libc::c_int;
            current_block_64 = 6560072651652764009;
        } else if (*vendor).match_type == 2 as libc::c_int {
            mopt = 15 as libc::c_int;
            current_block_64 = 6560072651652764009;
        } else { current_block_64 = 17747245473264231573; }
        match current_block_64 {
            6560072651652764009 => {
                opt =
                    opt6_find((*state).packet_options, (*state).end,
                              mopt as libc::c_uint,
                              2 as libc::c_int as libc::c_uint);
                if !opt.is_null() {
                    let mut enc_opt: *mut libc::c_void =
                        0 as *mut libc::c_void;
                    let mut enc_end: *mut libc::c_void =
                        &mut *(opt as
                                   *mut libc::c_uchar).offset((4 as
                                                                   libc::c_int
                                                                   +
                                                                   (opt6_uint
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *mut libc::c_uchar,
                                                                                             _:
                                                                                                 libc::c_int,
                                                                                             _:
                                                                                                 libc::c_int)
                                                                            ->
                                                                                libc::c_uint)(opt
                                                                                                  as
                                                                                                  *mut libc::c_uchar,
                                                                                              -(2
                                                                                                    as
                                                                                                    libc::c_int),
                                                                                              2
                                                                                                  as
                                                                                                  libc::c_int)
                                                                       as
                                                                       libc::c_int)
                                                                  as isize) as
                            *mut libc::c_uchar as *mut libc::c_void;
                    let mut offset: libc::c_int = 0 as libc::c_int;
                    if mopt == 16 as libc::c_int {
                        if (opt6_uint(opt as *mut libc::c_uchar,
                                      -(2 as libc::c_int), 2 as libc::c_int)
                                as libc::c_int) < 4 as libc::c_int {
                            current_block_64 = 17747245473264231573;
                        } else if (*vendor).enterprise !=
                                      opt6_uint(opt as *mut libc::c_uchar,
                                                0 as libc::c_int,
                                                4 as libc::c_int) {
                            current_block_64 = 17747245473264231573;
                        } else {
                            offset = 4 as libc::c_int;
                            current_block_64 = 307447392441238883;
                        }
                    } else { current_block_64 = 307447392441238883; }
                    match current_block_64 {
                        17747245473264231573 => { }
                        _ => {
                            /* Note that format if user/vendor classes is different to DHCP options - no option types. */
                            enc_opt =
                                &mut *(opt as
                                           *mut libc::c_uchar).offset((4 as
                                                                           libc::c_int
                                                                           +
                                                                           offset)
                                                                          as
                                                                          isize)
                                    as *mut libc::c_uchar as
                                    *mut libc::c_void;
                            while !enc_opt.is_null() {
                                i = 0 as libc::c_int;
                                while i <=
                                          opt6_uint(enc_opt as
                                                        *mut libc::c_uchar,
                                                    -(4 as libc::c_int),
                                                    2 as libc::c_int) as
                                              libc::c_int - (*vendor).len {
                                    if memcmp((*vendor).data as
                                                  *const libc::c_void,
                                              &mut *(enc_opt as
                                                         *mut libc::c_uchar).offset((2
                                                                                         as
                                                                                         libc::c_int
                                                                                         +
                                                                                         i)
                                                                                        as
                                                                                        isize)
                                                  as *mut libc::c_uchar as
                                                  *mut libc::c_void,
                                              (*vendor).len as libc::c_ulong)
                                           == 0 as libc::c_int {
                                        (*vendor).netid.next = (*state).tags;
                                        (*state).tags = &mut (*vendor).netid;
                                        break ;
                                    } else { i += 1 }
                                }
                                enc_opt =
                                    opt6_next(enc_opt.offset(-(2 as
                                                                   libc::c_int
                                                                   as isize)),
                                              enc_end)
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        vendor = (*vendor).next
    }
    if (*dnsmasq_daemon).options[(28 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (28 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 &&
           {
               opt =
                   opt6_find((*state).packet_options, (*state).end,
                             16 as libc::c_int as libc::c_uint,
                             4 as libc::c_int as libc::c_uint);
               !opt.is_null()
           } {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"%u vendor class: %u\x00" as *const u8 as
                      *const libc::c_char, (*state).xid,
                  opt6_uint(opt as *mut libc::c_uchar, 0 as libc::c_int,
                            4 as libc::c_int));
    }
    let mut current_block_78: u64;
    /* dhcp-match. If we have hex-and-wildcards, look for a left-anchored match.
     Otherwise assume the option is an array, and look for a matching element. 
     If no data given, existence of the option is enough. This code handles 
     V-I opts too. */
    opt_cfg = (*dnsmasq_daemon).dhcp_match6;
    while !opt_cfg.is_null() {
        let mut match_0: libc::c_int = 0 as libc::c_int;
        if (*opt_cfg).flags & 2048 as libc::c_int != 0 {
            opt =
                opt6_find((*state).packet_options, (*state).end,
                          17 as libc::c_int as libc::c_uint,
                          4 as libc::c_int as libc::c_uint);
            while !opt.is_null() {
                let mut vopt: *mut libc::c_void = 0 as *mut libc::c_void;
                let mut vend: *mut libc::c_void =
                    &mut *(opt as
                               *mut libc::c_uchar).offset((4 as libc::c_int +
                                                               (opt6_uint as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut libc::c_uchar,
                                                                                         _:
                                                                                             libc::c_int,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            libc::c_uint)(opt
                                                                                              as
                                                                                              *mut libc::c_uchar,
                                                                                          -(2
                                                                                                as
                                                                                                libc::c_int),
                                                                                          2
                                                                                              as
                                                                                              libc::c_int)
                                                                   as
                                                                   libc::c_int)
                                                              as isize) as
                        *mut libc::c_uchar as *mut libc::c_void;
                vopt =
                    opt6_find(&mut *(opt as
                                         *mut libc::c_uchar).offset((4 as
                                                                         libc::c_int
                                                                         +
                                                                         4 as
                                                                             libc::c_int)
                                                                        as
                                                                        isize)
                                  as *mut libc::c_uchar as *mut libc::c_void,
                              vend, (*opt_cfg).opt as libc::c_uint,
                              0 as libc::c_int as libc::c_uint);
                while !vopt.is_null() {
                    match_0 =
                        match_bytes(opt_cfg,
                                    &mut *(vopt as
                                               *mut libc::c_uchar).offset((4
                                                                               as
                                                                               libc::c_int
                                                                               +
                                                                               0
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              isize)
                                        as *mut libc::c_uchar as
                                        *mut libc::c_void as
                                        *mut libc::c_uchar,
                                    opt6_uint(vopt as *mut libc::c_uchar,
                                              -(2 as libc::c_int),
                                              2 as libc::c_int) as
                                        libc::c_int);
                    if match_0 != 0 { break ; }
                    vopt =
                        opt6_find(opt6_next(vopt, vend), vend,
                                  (*opt_cfg).opt as libc::c_uint,
                                  0 as libc::c_int as libc::c_uint)
                }
                opt =
                    opt6_find(opt6_next(opt, (*state).end), (*state).end,
                              17 as libc::c_int as libc::c_uint,
                              4 as libc::c_int as libc::c_uint)
            }
            if match_0 != 0 { break ; }
            current_block_78 = 2616667235040759262;
        } else {
            opt =
                opt6_find((*state).packet_options, (*state).end,
                          (*opt_cfg).opt as libc::c_uint,
                          1 as libc::c_int as libc::c_uint);
            if opt.is_null() {
                current_block_78 = 5793491756164225964;
            } else {
                match_0 =
                    match_bytes(opt_cfg,
                                &mut *(opt as
                                           *mut libc::c_uchar).offset((4 as
                                                                           libc::c_int
                                                                           +
                                                                           0
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          isize)
                                    as *mut libc::c_uchar as *mut libc::c_void
                                    as *mut libc::c_uchar,
                                opt6_uint(opt as *mut libc::c_uchar,
                                          -(2 as libc::c_int),
                                          2 as libc::c_int) as libc::c_int);
                current_block_78 = 2616667235040759262;
            }
        }
        match current_block_78 {
            2616667235040759262 => {
                if match_0 != 0 {
                    (*(*opt_cfg).netid).next = (*state).tags;
                    (*state).tags = (*opt_cfg).netid
                }
            }
            _ => { }
        }
        opt_cfg = (*opt_cfg).next
    }
    if (*state).mac_len != 0 as libc::c_int as libc::c_uint {
        if (*dnsmasq_daemon).options[(28 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (28 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
            print_mac((*dnsmasq_daemon).dhcp_buff, (*state).mac.as_mut_ptr(),
                      (*state).mac_len as libc::c_int);
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          6 as libc::c_int,
                      b"%u client MAC address: %s\x00" as *const u8 as
                          *const libc::c_char, (*state).xid,
                      (*dnsmasq_daemon).dhcp_buff);
        }
        mac_opt = (*dnsmasq_daemon).dhcp_macs;
        while !mac_opt.is_null() {
            if (*mac_opt).hwaddr_len as libc::c_uint == (*state).mac_len &&
                   ((*mac_opt).hwaddr_type as libc::c_uint ==
                        (*state).mac_type ||
                        (*mac_opt).hwaddr_type == 0 as libc::c_int) &&
                   memcmp_masked((*mac_opt).hwaddr.as_mut_ptr(),
                                 (*state).mac.as_mut_ptr(),
                                 (*state).mac_len as libc::c_int,
                                 (*mac_opt).mask) != 0 {
                (*mac_opt).netid.next = (*state).tags;
                (*state).tags = &mut (*mac_opt).netid
            }
            mac_opt = (*mac_opt).next
        }
    }
    opt =
        opt6_find((*state).packet_options, (*state).end,
                  39 as libc::c_int as libc::c_uint,
                  1 as libc::c_int as libc::c_uint);
    if !opt.is_null() {
        /* RFC4704 refers */
        let mut len: libc::c_int =
            opt6_uint(opt as *mut libc::c_uchar, -(2 as libc::c_int),
                      2 as libc::c_int) as libc::c_int - 1 as libc::c_int;
        (*state).fqdn_flags =
            opt6_uint(opt as *mut libc::c_uchar, 0 as libc::c_int,
                      1 as libc::c_int);
        /* Always force update, since the client has no way to do it itself. */
        if (*dnsmasq_daemon).options[(36 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (36 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 &&
               (*state).fqdn_flags & 0x1 as libc::c_int as libc::c_uint == 0 {
            (*state).fqdn_flags |= 0x3 as libc::c_int as libc::c_uint
        }
        (*state).fqdn_flags &= !(0x4 as libc::c_int) as libc::c_uint;
        if len != 0 as libc::c_int && len < 255 as libc::c_int {
            let mut pp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut op: *mut libc::c_uchar =
                &mut *(opt as
                           *mut libc::c_uchar).offset((4 as libc::c_int +
                                                           1 as libc::c_int)
                                                          as isize) as
                    *mut libc::c_uchar as *mut libc::c_void as
                    *mut libc::c_uchar;
            let mut pq: *mut libc::c_char = (*dnsmasq_daemon).dhcp_buff;
            pp = op;
            while *op as libc::c_int != 0 as libc::c_int &&
                      (op.offset(*op as libc::c_int as
                                     isize).wrapping_offset_from(pp) as
                           libc::c_long) < len as libc::c_long {
                memcpy(pq as *mut libc::c_void,
                       op.offset(1 as libc::c_int as isize) as
                           *const libc::c_void, *op as libc::c_ulong);
                pq = pq.offset(*op as libc::c_int as isize);
                op =
                    op.offset((*op as libc::c_int + 1 as libc::c_int) as
                                  isize);
                let fresh6 = pq;
                pq = pq.offset(1);
                *fresh6 = '.' as i32 as libc::c_char
            }
            if pq != (*dnsmasq_daemon).dhcp_buff { pq = pq.offset(-1) }
            *pq = 0 as libc::c_int as libc::c_char;
            if legal_hostname((*dnsmasq_daemon).dhcp_buff) != 0 {
                let mut m: *mut dhcp_match_name = 0 as *mut dhcp_match_name;
                let mut nl: size_t = strlen((*dnsmasq_daemon).dhcp_buff);
                (*state).client_hostname = (*dnsmasq_daemon).dhcp_buff;
                if (*dnsmasq_daemon).options[(28 as libc::c_int as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &
                       (1 as libc::c_uint) <<
                           (28 as libc::c_int as
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                       != 0 {
                    my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                  6 as libc::c_int,
                              b"%u client provides name: %s\x00" as *const u8
                                  as *const libc::c_char, (*state).xid,
                              (*state).client_hostname);
                }
                m = (*dnsmasq_daemon).dhcp_name_match;
                while !m.is_null() {
                    let mut ml: size_t = strlen((*m).name);
                    let mut save: libc::c_char =
                        0 as libc::c_int as libc::c_char;
                    if !(nl < ml) {
                        if nl > ml {
                            save =
                                *(*state).client_hostname.offset(ml as isize);
                            *(*state).client_hostname.offset(ml as isize) =
                                0 as libc::c_int as libc::c_char
                        }
                        if hostname_isequal((*state).client_hostname,
                                            (*m).name) != 0 &&
                               (save as libc::c_int == 0 as libc::c_int ||
                                    (*m).wildcard != 0) {
                            (*(*m).netid).next = (*state).tags;
                            (*state).tags = (*m).netid
                        }
                        if save as libc::c_int != 0 as libc::c_int {
                            *(*state).client_hostname.offset(ml as isize) =
                                save
                        }
                    }
                    m = (*m).next
                }
            }
        }
    }
    if !(*state).clid.is_null() &&
           {
               config =
                   find_config((*dnsmasq_daemon).dhcp_conf, (*state).context,
                               (*state).clid, (*state).clid_len,
                               (*state).mac.as_mut_ptr(),
                               (*state).mac_len as libc::c_int,
                               (*state).mac_type as libc::c_int,
                               0 as *mut libc::c_char,
                               run_tag_if((*state).tags));
               !config.is_null()
           } &&
           (!config.is_null() &&
                (*config).flags & 16 as libc::c_int as libc::c_uint != 0) {
        (*state).hostname = (*config).hostname;
        (*state).domain = (*config).domain;
        (*state).hostname_auth = 1 as libc::c_int
    } else if !(*state).client_hostname.is_null() {
        (*state).domain = strip_hostname((*state).client_hostname);
        if strlen((*state).client_hostname) !=
               0 as libc::c_int as libc::c_ulong {
            (*state).hostname = (*state).client_hostname;
            if config.is_null() {
                /* Search again now we have a hostname. 
		 Only accept configs without CLID here, (it won't match)
		 to avoid impersonation by name. */
                let mut new: *mut dhcp_config =
                    find_config((*dnsmasq_daemon).dhcp_conf, (*state).context,
                                0 as *mut libc::c_uchar, 0 as libc::c_int,
                                0 as *mut libc::c_uchar, 0 as libc::c_int,
                                0 as libc::c_int, (*state).hostname,
                                run_tag_if((*state).tags));
                if !new.is_null() &&
                       !(!new.is_null() &&
                             (*new).flags & 2 as libc::c_int as libc::c_uint
                                 != 0) && (*new).hwaddr.is_null() {
                    config = new
                }
            }
        }
    }
    if !config.is_null() {
        let mut list: *mut dhcp_netid_list = 0 as *mut dhcp_netid_list;
        list = (*config).netid;
        while !list.is_null() {
            (*(*list).list).next = (*state).tags;
            (*state).tags = (*list).list;
            list = (*list).next
        }
        /* set "known" tag for known hosts */
        known_id.net =
            b"known\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        known_id.next = (*state).tags;
        (*state).tags = &mut known_id;
        if !config.is_null() &&
               (*config).flags & 1 as libc::c_int as libc::c_uint != 0 {
            ignore = 1 as libc::c_int as libc::c_uint
        }
    } else if !(*state).clid.is_null() &&
                  !find_config((*dnsmasq_daemon).dhcp_conf,
                               0 as *mut dhcp_context, (*state).clid,
                               (*state).clid_len, (*state).mac.as_mut_ptr(),
                               (*state).mac_len as libc::c_int,
                               (*state).mac_type as libc::c_int,
                               0 as *mut libc::c_char,
                               run_tag_if((*state).tags)).is_null() {
        known_id.net =
            b"known-othernet\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        known_id.next = (*state).tags;
        (*state).tags = &mut known_id
    }
    tagif = run_tag_if((*state).tags);
    /* if all the netids in the ignore list are present, ignore this client */
    if !(*dnsmasq_daemon).dhcp_ignore.is_null() {
        let mut id_list: *mut dhcp_netid_list = 0 as *mut dhcp_netid_list;
        id_list = (*dnsmasq_daemon).dhcp_ignore;
        while !id_list.is_null() {
            if match_netid((*id_list).list, tagif, 0 as libc::c_int) != 0 {
                ignore = 1 as libc::c_int as libc::c_uint
            }
            id_list = (*id_list).next
        }
    }
    /* if all the netids in the ignore_name list are present, ignore client-supplied name */
    if (*state).hostname_auth == 0 {
        let mut id_list_0: *mut dhcp_netid_list = 0 as *mut dhcp_netid_list;
        id_list_0 = (*dnsmasq_daemon).dhcp_ignore_names;
        while !id_list_0.is_null() {
            if (*id_list_0).list.is_null() ||
                   match_netid((*id_list_0).list, tagif, 0 as libc::c_int) !=
                       0 {
                break ;
            }
            id_list_0 = (*id_list_0).next
        }
        if !id_list_0.is_null() { (*state).hostname = 0 as *mut libc::c_char }
    }
    let mut address_assigned: libc::c_int = 0;
    let mut solicit_tags: *mut dhcp_netid = 0 as *mut dhcp_netid;
    let mut c: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut current_block_486: u64;
    match msg_type {
        1 => {
            address_assigned = 0 as libc::c_int;
            /* tags without all prefix-class tags */
            solicit_tags = 0 as *mut dhcp_netid;
            c = 0 as *mut dhcp_context;
            *outmsgtypep = 2 as libc::c_int as libc::c_uchar;
            if !opt6_find((*state).packet_options, (*state).end,
                          14 as libc::c_int as libc::c_uint,
                          0 as libc::c_int as libc::c_uint).is_null() {
                *outmsgtypep = 7 as libc::c_int as libc::c_uchar;
                (*state).lease_allocate = 1 as libc::c_int;
                o = new_opt6(14 as libc::c_int);
                end_opt6(o);
            }
            log6_quiet(state,
                       b"DHCPSOLICIT\x00" as *const u8 as *const libc::c_char
                           as *mut libc::c_char, 0 as *mut in6_addr,
                       if ignore != 0 {
                           b"ignored\x00" as *const u8 as *const libc::c_char
                       } else { 0 as *const libc::c_char } as
                           *mut libc::c_char);
            current_block_486 = 15319502457978536222;
        }
        3 => {
            let mut address_assigned_0: libc::c_int = 0 as libc::c_int;
            let mut start: libc::c_int = save_counter(-(1 as libc::c_int));
            /* set reply message type */
            *outmsgtypep = 7 as libc::c_int as libc::c_uchar;
            (*state).lease_allocate = 1 as libc::c_int;
            log6_quiet(state,
                       b"DHCPREQUEST\x00" as *const u8 as *const libc::c_char
                           as *mut libc::c_char, 0 as *mut in6_addr,
                       if ignore != 0 {
                           b"ignored\x00" as *const u8 as *const libc::c_char
                       } else { 0 as *const libc::c_char } as
                           *mut libc::c_char);
            if ignore != 0 { return 0 as libc::c_int }
            opt = (*state).packet_options;
            loop  {
                if opt.is_null() {
                    current_block_486 = 309319537768397308;
                    break ;
                }
                let mut ia_option_0: *mut libc::c_void =
                    0 as *mut libc::c_void;
                let mut ia_end_0: *mut libc::c_void = 0 as *mut libc::c_void;
                let mut min_time_0: libc::c_uint = 0xffffffff as libc::c_uint;
                let mut t1cntr_0: libc::c_int = 0;
                if !(check_ia(state, opt, &mut ia_end_0, &mut ia_option_0) ==
                         0) {
                    if ia_option_0.is_null() {
                        /* If we get a request with an IA_*A without addresses, treat it exactly like
		    a SOLICT with rapid commit set. */
                        save_counter(start);
                        current_block_486 = 15319502457978536222;
                        break ;
                    } else {
                        o = build_ia(state, &mut t1cntr_0);
                        while !ia_option_0.is_null() {
                            let mut req_addr_0: in6_addr =
                                in6_addr{__in6_u:
                                             C2RustUnnamed{__u6_addr8:
                                                               [0; 16],},};
                            let mut dynamic: *mut dhcp_context =
                                0 as *mut dhcp_context;
                            let mut c_0: *mut dhcp_context =
                                0 as *mut dhcp_context;
                            let mut lease_time_0: libc::c_uint = 0;
                            let mut config_ok: libc::c_int = 0 as libc::c_int;
                            /* align. */
                            memcpy(&mut req_addr_0 as *mut in6_addr as
                                       *mut libc::c_void,
                                   &mut *(ia_option_0 as
                                              *mut libc::c_uchar).offset((4 as
                                                                              libc::c_int
                                                                              +
                                                                              0
                                                                                  as
                                                                                  libc::c_int)
                                                                             as
                                                                             isize)
                                       as *mut libc::c_uchar as
                                       *mut libc::c_void,
                                   16 as libc::c_int as libc::c_ulong);
                            c_0 =
                                address6_valid((*state).context,
                                               &mut req_addr_0, tagif,
                                               1 as libc::c_int);
                            if !c_0.is_null() {
                                config_ok =
                                    (config_implies(config, c_0,
                                                    &mut req_addr_0) !=
                                         0 as *mut libc::c_void as
                                             *mut addrlist) as libc::c_int
                            }
                            dynamic =
                                address6_available((*state).context,
                                                   &mut req_addr_0, tagif,
                                                   1 as libc::c_int);
                            if !dynamic.is_null() || !c_0.is_null() {
                                if dynamic.is_null() && config_ok == 0 {
                                    /* Static range, not configured. */
                                    o1 = new_opt6(13 as libc::c_int);
                                    put_opt6_short(2 as libc::c_int as
                                                       libc::c_uint);
                                    put_opt6_string(b"address unavailable\x00"
                                                        as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char);
                                    end_opt6(o1);
                                } else if check_address(state,
                                                        &mut req_addr_0) == 0
                                 {
                                    /* Address leased to another DUID/IAID */
                                    o1 = new_opt6(13 as libc::c_int);
                                    put_opt6_short(1 as libc::c_int as
                                                       libc::c_uint);
                                    put_opt6_string(b"address in use\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char);
                                    end_opt6(o1);
                                } else {
                                    if dynamic.is_null() { dynamic = c_0 }
                                    lease_time_0 = (*dynamic).lease_time;
                                    if config_ok != 0 &&
                                           (!config.is_null() &&
                                                (*config).flags &
                                                    8 as libc::c_int as
                                                        libc::c_uint != 0) {
                                        lease_time_0 = (*config).lease_time
                                    }
                                    add_address(state, dynamic, lease_time_0,
                                                ia_option_0, &mut min_time_0,
                                                &mut req_addr_0, now);
                                    get_context_tag(state, dynamic);
                                    address_assigned_0 = 1 as libc::c_int
                                }
                            } else {
                                /* requested address not on the correct link */
                                o1 = new_opt6(13 as libc::c_int);
                                put_opt6_short(4 as libc::c_int as
                                                   libc::c_uint);
                                put_opt6_string(b"not on link\x00" as
                                                    *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char);
                                end_opt6(o1);
                            }
                            ia_option_0 =
                                opt6_find(opt6_next(ia_option_0, ia_end_0),
                                          ia_end_0,
                                          5 as libc::c_int as libc::c_uint,
                                          24 as libc::c_int as libc::c_uint)
                        }
                        end_ia(t1cntr_0, min_time_0, 0 as libc::c_int);
                        end_opt6(o);
                    }
                }
                opt = opt6_next(opt, (*state).end)
            }
            match current_block_486 {
                15319502457978536222 => { }
                _ => {
                    if address_assigned_0 != 0 {
                        o1 = new_opt6(13 as libc::c_int);
                        put_opt6_short(0 as libc::c_int as libc::c_uint);
                        put_opt6_string(b"success\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char);
                        end_opt6(o1);
                    } else {
                        /* no address, return error */
                        o1 = new_opt6(13 as libc::c_int);
                        put_opt6_short(2 as libc::c_int as libc::c_uint);
                        put_opt6_string(b"no addresses available\x00" as
                                            *const u8 as *const libc::c_char
                                            as *mut libc::c_char);
                        end_opt6(o1);
                        log6_packet(state,
                                    b"DHCPREPLY\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char, 0 as *mut in6_addr,
                                    b"no addresses available\x00" as *const u8
                                        as *const libc::c_char as
                                        *mut libc::c_char);
                    }
                    tagif = add_options(state, 0 as libc::c_int);
                    current_block_486 = 14838758841813985983;
                }
            }
        }
        5 => {
            /* set reply message type */
            *outmsgtypep = 7 as libc::c_int as libc::c_uchar;
            log6_quiet(state,
                       b"DHCPRENEW\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char, 0 as *mut in6_addr,
                       0 as *mut libc::c_char);
            opt = (*state).packet_options;
            while !opt.is_null() {
                let mut ia_option_1: *mut libc::c_void =
                    0 as *mut libc::c_void;
                let mut ia_end_1: *mut libc::c_void = 0 as *mut libc::c_void;
                let mut min_time_1: libc::c_uint = 0xffffffff as libc::c_uint;
                let mut t1cntr_1: libc::c_int = 0;
                let mut iacntr: libc::c_int = 0;
                if !(check_ia(state, opt, &mut ia_end_1, &mut ia_option_1) ==
                         0) {
                    o = build_ia(state, &mut t1cntr_1);
                    iacntr = save_counter(-(1 as libc::c_int));
                    while !ia_option_1.is_null() {
                        let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
                        let mut req_addr_1: in6_addr =
                            in6_addr{__in6_u:
                                         C2RustUnnamed{__u6_addr8:
                                                           [0; 16],},};
                        let mut preferred_time: libc::c_uint =
                            opt6_uint(ia_option_1 as *mut libc::c_uchar,
                                      16 as libc::c_int, 4 as libc::c_int);
                        let mut valid_time: libc::c_uint =
                            opt6_uint(ia_option_1 as *mut libc::c_uchar,
                                      20 as libc::c_int, 4 as libc::c_int);
                        let mut message: *mut libc::c_char =
                            0 as *mut libc::c_char;
                        let mut this_context: *mut dhcp_context =
                            0 as *mut dhcp_context;
                        memcpy(&mut req_addr_1 as *mut in6_addr as
                                   *mut libc::c_void,
                               &mut *(ia_option_1 as
                                          *mut libc::c_uchar).offset((4 as
                                                                          libc::c_int
                                                                          +
                                                                          0 as
                                                                              libc::c_int)
                                                                         as
                                                                         isize)
                                   as *mut libc::c_uchar as *mut libc::c_void,
                               16 as libc::c_int as libc::c_ulong);
                        lease =
                            lease6_find((*state).clid, (*state).clid_len,
                                        if (*state).ia_type ==
                                               3 as libc::c_int {
                                            32 as libc::c_int
                                        } else { 64 as libc::c_int },
                                        (*state).iaid, &mut req_addr_1);
                        if lease.is_null() {
                            /* If the server cannot find a client entry for the IA the server
		       returns the IA containing no addresses with a Status Code option set
		       to NoBinding in the Reply message. */
                            save_counter(iacntr);
                            t1cntr_1 = 0 as libc::c_int;
                            log6_packet(state,
                                        b"DHCPREPLY\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        &mut req_addr_1,
                                        b"lease not found\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char);
                            o1 = new_opt6(13 as libc::c_int);
                            put_opt6_short(3 as libc::c_int as libc::c_uint);
                            put_opt6_string(b"no binding found\x00" as
                                                *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char);
                            end_opt6(o1);
                            valid_time = 0 as libc::c_int as libc::c_uint;
                            preferred_time = valid_time;
                            break ;
                        } else {
                            this_context =
                                address6_available((*state).context,
                                                   &mut req_addr_1, tagif,
                                                   1 as libc::c_int);
                            if !this_context.is_null() ||
                                   {
                                       this_context =
                                           address6_valid((*state).context,
                                                          &mut req_addr_1,
                                                          tagif,
                                                          1 as libc::c_int);
                                       !this_context.is_null()
                                   } {
                                let mut lease_time_1: libc::c_uint = 0;
                                get_context_tag(state, this_context);
                                if !config_implies(config, this_context,
                                                   &mut req_addr_1).is_null()
                                       &&
                                       (!config.is_null() &&
                                            (*config).flags &
                                                8 as libc::c_int as
                                                    libc::c_uint != 0) {
                                    lease_time_1 = (*config).lease_time
                                } else {
                                    lease_time_1 = (*this_context).lease_time
                                }
                                calculate_times(this_context, &mut min_time_1,
                                                &mut valid_time,
                                                &mut preferred_time,
                                                lease_time_1);
                                lease_set_expires(lease, valid_time, now);
                                /* Update MAC record in case it's new information. */
                                if (*state).mac_len !=
                                       0 as libc::c_int as libc::c_uint {
                                    lease_set_hwaddr(lease,
                                                     (*state).mac.as_mut_ptr(),
                                                     (*state).clid,
                                                     (*state).mac_len as
                                                         libc::c_int,
                                                     (*state).mac_type as
                                                         libc::c_int,
                                                     (*state).clid_len, now,
                                                     0 as libc::c_int);
                                }
                                if (*state).ia_type == 3 as libc::c_int &&
                                       !(*state).hostname.is_null() {
                                    let mut addr_domain: *mut libc::c_char =
                                        get_domain6(&mut req_addr_1);
                                    if (*state).send_domain.is_null() {
                                        (*state).send_domain = addr_domain
                                    }
                                    lease_set_hostname(lease,
                                                       (*state).hostname,
                                                       (*state).hostname_auth,
                                                       addr_domain,
                                                       (*state).domain);
                                    message = (*state).hostname
                                }
                                if preferred_time ==
                                       0 as libc::c_int as libc::c_uint {
                                    message =
                                        b"deprecated\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char
                                }
                            } else {
                                valid_time = 0 as libc::c_int as libc::c_uint;
                                preferred_time = valid_time;
                                message =
                                    b"address invalid\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char
                            }
                            if !message.is_null() &&
                                   message != (*state).hostname {
                                log6_packet(state,
                                            b"DHCPREPLY\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char,
                                            &mut req_addr_1, message);
                            } else {
                                log6_quiet(state,
                                           b"DHCPREPLY\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char,
                                           &mut req_addr_1, message);
                            }
                            o1 = new_opt6(5 as libc::c_int);
                            put_opt6(&mut req_addr_1 as *mut in6_addr as
                                         *mut libc::c_void,
                                     ::std::mem::size_of::<in6_addr>() as
                                         libc::c_ulong);
                            put_opt6_long(preferred_time);
                            put_opt6_long(valid_time);
                            end_opt6(o1);
                            ia_option_1 =
                                opt6_find(opt6_next(ia_option_1, ia_end_1),
                                          ia_end_1,
                                          5 as libc::c_int as libc::c_uint,
                                          24 as libc::c_int as libc::c_uint)
                        }
                    }
                    end_ia(t1cntr_1, min_time_1, 1 as libc::c_int);
                    end_opt6(o);
                }
                opt = opt6_next(opt, (*state).end)
            }
            tagif = add_options(state, 0 as libc::c_int);
            current_block_486 = 14838758841813985983;
        }
        4 => {
            let mut good_addr: libc::c_int = 0 as libc::c_int;
            /* set reply message type */
            *outmsgtypep = 7 as libc::c_int as libc::c_uchar;
            log6_quiet(state,
                       b"DHCPCONFIRM\x00" as *const u8 as *const libc::c_char
                           as *mut libc::c_char, 0 as *mut in6_addr,
                       0 as *mut libc::c_char);
            opt = (*state).packet_options;
            while !opt.is_null() {
                let mut ia_option_2: *mut libc::c_void =
                    0 as *mut libc::c_void;
                let mut ia_end_2: *mut libc::c_void = 0 as *mut libc::c_void;
                check_ia(state, opt, &mut ia_end_2, &mut ia_option_2);
                while !ia_option_2.is_null() {
                    let mut req_addr_2: in6_addr =
                        in6_addr{__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},};
                    /* alignment */
                    memcpy(&mut req_addr_2 as *mut in6_addr as
                               *mut libc::c_void,
                           &mut *(ia_option_2 as
                                      *mut libc::c_uchar).offset((4 as
                                                                      libc::c_int
                                                                      +
                                                                      0 as
                                                                          libc::c_int)
                                                                     as isize)
                               as *mut libc::c_uchar as *mut libc::c_void,
                           16 as libc::c_int as libc::c_ulong);
                    if address6_valid((*state).context, &mut req_addr_2,
                                      tagif, 1 as libc::c_int).is_null() {
                        o1 = new_opt6(13 as libc::c_int);
                        put_opt6_short(4 as libc::c_int as libc::c_uint);
                        put_opt6_string(b"confirm failed\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char);
                        end_opt6(o1);
                        log6_quiet(state,
                                   b"DHCPREPLY\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, &mut req_addr_2,
                                   b"confirm failed\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char);
                        return 1 as libc::c_int
                    }
                    good_addr = 1 as libc::c_int;
                    log6_quiet(state,
                               b"DHCPREPLY\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               &mut req_addr_2, (*state).hostname);
                    ia_option_2 =
                        opt6_find(opt6_next(ia_option_2, ia_end_2), ia_end_2,
                                  5 as libc::c_int as libc::c_uint,
                                  24 as libc::c_int as libc::c_uint)
                }
                opt = opt6_next(opt, (*state).end)
            }
            /* No addresses, no reply: RFC 3315 18.2.2 */
            if good_addr == 0 { return 0 as libc::c_int }
            o1 = new_opt6(13 as libc::c_int);
            put_opt6_short(0 as libc::c_int as libc::c_uint);
            put_opt6_string(b"all addresses still on link\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char);
            end_opt6(o1);
            current_block_486 = 14838758841813985983;
        }
        11 => {
            /* We can't discriminate contexts based on address, as we don't know it.
	   If there is only one possible context, we can use its tags */
            if !(*state).context.is_null() &&
                   !(*(*state).context).netid.net.is_null() &&
                   (*(*state).context).current.is_null() {
                (*(*state).context).netid.next = 0 as *mut dhcp_netid;
                (*state).context_tags = &mut (*(*state).context).netid
            }
            /* Similarly, we can't determine domain from address, but if the FQDN is
	   given in --dhcp-host, we can use that, and failing that we can use the 
	   unqualified configured domain, if any. */
            if (*state).hostname_auth != 0 {
                (*state).send_domain = (*state).domain
            } else { (*state).send_domain = get_domain6(0 as *mut in6_addr) }
            log6_quiet(state,
                       b"DHCPINFORMATION-REQUEST\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       0 as *mut in6_addr,
                       if ignore != 0 {
                           b"ignored\x00" as *const u8 as *const libc::c_char
                       } else { (*state).hostname as *const libc::c_char } as
                           *mut libc::c_char);
            if ignore != 0 { return 0 as libc::c_int }
            *outmsgtypep = 7 as libc::c_int as libc::c_uchar;
            tagif = add_options(state, 1 as libc::c_int);
            current_block_486 = 14838758841813985983;
        }
        8 => {
            /* set reply message type */
            *outmsgtypep = 7 as libc::c_int as libc::c_uchar;
            log6_quiet(state,
                       b"DHCPRELEASE\x00" as *const u8 as *const libc::c_char
                           as *mut libc::c_char, 0 as *mut in6_addr,
                       0 as *mut libc::c_char);
            opt = (*state).packet_options;
            while !opt.is_null() {
                let mut ia_option_3: *mut libc::c_void =
                    0 as *mut libc::c_void;
                let mut ia_end_3: *mut libc::c_void = 0 as *mut libc::c_void;
                let mut made_ia: libc::c_int = 0 as libc::c_int;
                check_ia(state, opt, &mut ia_end_3, &mut ia_option_3);
                while !ia_option_3.is_null() {
                    let mut lease_0: *mut dhcp_lease = 0 as *mut dhcp_lease;
                    let mut addr_0: in6_addr =
                        in6_addr{__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},};
                    /* align */
                    memcpy(&mut addr_0 as *mut in6_addr as *mut libc::c_void,
                           &mut *(ia_option_3 as
                                      *mut libc::c_uchar).offset((4 as
                                                                      libc::c_int
                                                                      +
                                                                      0 as
                                                                          libc::c_int)
                                                                     as isize)
                               as *mut libc::c_uchar as *mut libc::c_void,
                           16 as libc::c_int as libc::c_ulong);
                    lease_0 =
                        lease6_find((*state).clid, (*state).clid_len,
                                    if (*state).ia_type == 3 as libc::c_int {
                                        32 as libc::c_int
                                    } else { 64 as libc::c_int },
                                    (*state).iaid, &mut addr_0);
                    if !lease_0.is_null() {
                        lease_prune(lease_0, now);
                    } else {
                        if made_ia == 0 {
                            o = new_opt6((*state).ia_type);
                            put_opt6_long((*state).iaid);
                            if (*state).ia_type == 3 as libc::c_int {
                                put_opt6_long(0 as libc::c_int as
                                                  libc::c_uint);
                                put_opt6_long(0 as libc::c_int as
                                                  libc::c_uint);
                            }
                            made_ia = 1 as libc::c_int
                        }
                        o1 = new_opt6(5 as libc::c_int);
                        put_opt6(&mut addr_0 as *mut in6_addr as
                                     *mut libc::c_void,
                                 16 as libc::c_int as size_t);
                        put_opt6_long(0 as libc::c_int as libc::c_uint);
                        put_opt6_long(0 as libc::c_int as libc::c_uint);
                        end_opt6(o1);
                    }
                    ia_option_3 =
                        opt6_find(opt6_next(ia_option_3, ia_end_3), ia_end_3,
                                  5 as libc::c_int as libc::c_uint,
                                  24 as libc::c_int as libc::c_uint)
                }
                if made_ia != 0 {
                    o1 = new_opt6(13 as libc::c_int);
                    put_opt6_short(3 as libc::c_int as libc::c_uint);
                    put_opt6_string(b"no binding found\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char);
                    end_opt6(o1);
                    end_opt6(o);
                }
                opt = opt6_next(opt, (*state).end)
            }
            o1 = new_opt6(13 as libc::c_int);
            put_opt6_short(0 as libc::c_int as libc::c_uint);
            put_opt6_string(b"release received\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char);
            end_opt6(o1);
            current_block_486 = 14838758841813985983;
        }
        9 => {
            /* set reply message type */
            *outmsgtypep = 7 as libc::c_int as libc::c_uchar;
            log6_quiet(state,
                       b"DHCPDECLINE\x00" as *const u8 as *const libc::c_char
                           as *mut libc::c_char, 0 as *mut in6_addr,
                       0 as *mut libc::c_char);
            opt = (*state).packet_options;
            while !opt.is_null() {
                let mut ia_option_4: *mut libc::c_void =
                    0 as *mut libc::c_void;
                let mut ia_end_4: *mut libc::c_void = 0 as *mut libc::c_void;
                let mut made_ia_0: libc::c_int = 0 as libc::c_int;
                check_ia(state, opt, &mut ia_end_4, &mut ia_option_4);
                while !ia_option_4.is_null() {
                    let mut lease_1: *mut dhcp_lease = 0 as *mut dhcp_lease;
                    let mut addr_1: in6_addr =
                        in6_addr{__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},};
                    let mut addr_list: *mut addrlist = 0 as *mut addrlist;
                    /* align */
                    memcpy(&mut addr_1 as *mut in6_addr as *mut libc::c_void,
                           &mut *(ia_option_4 as
                                      *mut libc::c_uchar).offset((4 as
                                                                      libc::c_int
                                                                      +
                                                                      0 as
                                                                          libc::c_int)
                                                                     as isize)
                               as *mut libc::c_uchar as *mut libc::c_void,
                           16 as libc::c_int as libc::c_ulong);
                    addr_list =
                        config_implies(config, (*state).context, &mut addr_1);
                    if !addr_list.is_null() {
                        prettyprint_time((*dnsmasq_daemon).dhcp_buff3,
                                         600 as libc::c_int as libc::c_uint);
                        inet_ntop(10 as libc::c_int,
                                  &mut addr_1 as *mut in6_addr as
                                      *const libc::c_void,
                                  (*dnsmasq_daemon).addrbuff,
                                  46 as libc::c_int as socklen_t);
                        my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                      4 as libc::c_int,
                                  b"disabling DHCP static address %s for %s\x00"
                                      as *const u8 as *const libc::c_char,
                                  (*dnsmasq_daemon).addrbuff,
                                  (*dnsmasq_daemon).dhcp_buff3);
                        (*addr_list).flags |= 32 as libc::c_int;
                        (*addr_list).decline_time = now
                    } else {
                        /* make sure this host gets a different address next time. */
                        context_tmp = (*state).context;
                        while !context_tmp.is_null() {
                            (*context_tmp).addr_epoch =
                                (*context_tmp).addr_epoch.wrapping_add(1);
                            context_tmp = (*context_tmp).current
                        }
                    }
                    lease_1 =
                        lease6_find((*state).clid, (*state).clid_len,
                                    if (*state).ia_type == 3 as libc::c_int {
                                        32 as libc::c_int
                                    } else { 64 as libc::c_int },
                                    (*state).iaid, &mut addr_1);
                    if !lease_1.is_null() {
                        lease_prune(lease_1, now);
                    } else {
                        if made_ia_0 == 0 {
                            o = new_opt6((*state).ia_type);
                            put_opt6_long((*state).iaid);
                            if (*state).ia_type == 3 as libc::c_int {
                                put_opt6_long(0 as libc::c_int as
                                                  libc::c_uint);
                                put_opt6_long(0 as libc::c_int as
                                                  libc::c_uint);
                            }
                            made_ia_0 = 1 as libc::c_int
                        }
                        o1 = new_opt6(5 as libc::c_int);
                        put_opt6(&mut addr_1 as *mut in6_addr as
                                     *mut libc::c_void,
                                 16 as libc::c_int as size_t);
                        put_opt6_long(0 as libc::c_int as libc::c_uint);
                        put_opt6_long(0 as libc::c_int as libc::c_uint);
                        end_opt6(o1);
                    }
                    ia_option_4 =
                        opt6_find(opt6_next(ia_option_4, ia_end_4), ia_end_4,
                                  5 as libc::c_int as libc::c_uint,
                                  24 as libc::c_int as libc::c_uint)
                }
                if made_ia_0 != 0 {
                    o1 = new_opt6(13 as libc::c_int);
                    put_opt6_short(3 as libc::c_int as libc::c_uint);
                    put_opt6_string(b"no binding found\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char);
                    end_opt6(o1);
                    end_opt6(o);
                }
                opt = opt6_next(opt, (*state).end)
            }
            /* We must answer with 'success' in global section anyway */
            o1 = new_opt6(13 as libc::c_int);
            put_opt6_short(0 as libc::c_int as libc::c_uint);
            put_opt6_string(b"success\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char);
            end_opt6(o1);
            current_block_486 = 14838758841813985983;
        }
        _ => { return 0 as libc::c_int }
    }
    match current_block_486 {
        15319502457978536222 => {
            solicit_tags = tagif;
            if ignore != 0 { return 0 as libc::c_int }
            /* reset USED bits in leases */
            lease6_reset();
            /* Can use configured address max once per prefix */
            c = (*state).context;
            while !c.is_null() {
                (*c).flags =
                    ((*c).flags as libc::c_uint &
                         !((1 as libc::c_uint) << 14 as libc::c_int)) as
                        libc::c_int;
                c = (*c).current
            }
            let mut current_block_242: u64;
            opt = (*state).packet_options;
            while !opt.is_null() {
                let mut ia_option: *mut libc::c_void = 0 as *mut libc::c_void;
                let mut ia_end: *mut libc::c_void = 0 as *mut libc::c_void;
                let mut min_time: libc::c_uint = 0xffffffff as libc::c_uint;
                let mut t1cntr: libc::c_int = 0;
                let mut ia_counter: libc::c_int = 0;
                /* set unless we're sending a particular prefix-class, when we
	       want only dhcp-ranges with the correct tags set and not those without any tags. */
                let mut plain_range: libc::c_int = 1 as libc::c_int;
                let mut lease_time: u32_0 = 0;
                let mut ltmp: *mut dhcp_lease = 0 as *mut dhcp_lease;
                let mut req_addr: in6_addr =
                    in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
                let mut addr: in6_addr =
                    in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
                if !(check_ia(state, opt, &mut ia_end, &mut ia_option) == 0) {
                    /* reset USED bits in contexts - one address per prefix per IAID */
                    c = (*state).context;
                    while !c.is_null() {
                        (*c).flags =
                            ((*c).flags as libc::c_uint &
                                 !((1 as libc::c_uint) << 15 as libc::c_int))
                                as libc::c_int;
                        c = (*c).current
                    }
                    o = build_ia(state, &mut t1cntr);
                    if address_assigned != 0 {
                        address_assigned = 2 as libc::c_int
                    }
                    let mut current_block_206: u64;
                    ia_counter = 0 as libc::c_int;
                    while !ia_option.is_null() {
                        /* worry about alignment here. */
                        memcpy(&mut req_addr as *mut in6_addr as
                                   *mut libc::c_void,
                               &mut *(ia_option as
                                          *mut libc::c_uchar).offset((4 as
                                                                          libc::c_int
                                                                          +
                                                                          0 as
                                                                              libc::c_int)
                                                                         as
                                                                         isize)
                                   as *mut libc::c_uchar as *mut libc::c_void,
                               16 as libc::c_int as libc::c_ulong);
                        c =
                            address6_valid((*state).context, &mut req_addr,
                                           solicit_tags, plain_range);
                        if !c.is_null() {
                            lease_time = (*c).lease_time;
                            /* If the client asks for an address on the same network as a configured address, 
		       offer the configured address instead, to make moving to newly-configured
		       addresses automatic. */
                            if (*c).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 14 as libc::c_int ==
                                   0 &&
                                   config_valid(config, c, &mut addr, state,
                                                now) != 0 {
                                req_addr =
                                    addr; /* address leased elsewhere */
                                mark_config_used(c, &mut addr);
                                if !config.is_null() &&
                                       (*config).flags &
                                           8 as libc::c_int as libc::c_uint !=
                                           0 {
                                    lease_time = (*config).lease_time
                                }
                                current_block_206 = 1851490986684842406;
                            } else {
                                c =
                                    address6_available((*state).context,
                                                       &mut req_addr,
                                                       solicit_tags,
                                                       plain_range);
                                if c.is_null() {
                                    current_block_206 = 9350489878244555550;
                                } else if check_address(state, &mut req_addr)
                                              == 0 {
                                    current_block_206 = 9350489878244555550;
                                } else {
                                    current_block_206 = 1851490986684842406;
                                }
                            }
                            match current_block_206 {
                                9350489878244555550 => { }
                                _ => {
                                    /* add address to output packet */
                                    add_address(state, c, lease_time,
                                                ia_option, &mut min_time,
                                                &mut req_addr,
                                                now); /* not an address we're allowed */
                                    mark_context_used(state, &mut req_addr);
                                    get_context_tag(state, c);
                                    address_assigned = 1 as libc::c_int
                                }
                            }
                        }
                        ia_counter += 1;
                        ia_option =
                            opt6_find(opt6_next(ia_option, ia_end), ia_end,
                                      5 as libc::c_int as libc::c_uint,
                                      24 as libc::c_int as libc::c_uint)
                    }
                    /* Suggest configured address(es) */
                    c = (*state).context;
                    while !c.is_null() {
                        if (*c).flags as libc::c_uint &
                               (1 as libc::c_uint) << 14 as libc::c_int == 0
                               &&
                               match_netid((*c).filter, solicit_tags,
                                           plain_range) != 0 &&
                               config_valid(config, c, &mut addr, state, now)
                                   != 0 {
                            mark_config_used((*state).context, &mut addr);
                            if !config.is_null() &&
                                   (*config).flags &
                                       8 as libc::c_int as libc::c_uint != 0 {
                                lease_time = (*config).lease_time
                            } else { lease_time = (*c).lease_time }
                            /* add address to output packet */
                            add_address(state, c, lease_time,
                                        0 as *mut libc::c_void, &mut min_time,
                                        &mut addr, now);
                            mark_context_used(state, &mut addr);
                            get_context_tag(state, c);
                            address_assigned = 1 as libc::c_int
                        }
                        c = (*c).current
                    }
                    /* return addresses for existing leases */
                    ltmp = 0 as *mut dhcp_lease;
                    loop  {
                        ltmp =
                            lease6_find_by_client(ltmp,
                                                  if (*state).ia_type ==
                                                         3 as libc::c_int {
                                                      32 as libc::c_int
                                                  } else {
                                                      64 as libc::c_int
                                                  }, (*state).clid,
                                                  (*state).clid_len,
                                                  (*state).iaid);
                        if ltmp.is_null() { break ; }
                        req_addr = (*ltmp).addr6;
                        c =
                            address6_available((*state).context,
                                               &mut req_addr, solicit_tags,
                                               plain_range);
                        if !c.is_null() {
                            add_address(state, c, (*c).lease_time,
                                        0 as *mut libc::c_void, &mut min_time,
                                        &mut req_addr, now);
                            mark_context_used(state, &mut req_addr);
                            get_context_tag(state, c);
                            address_assigned = 1 as libc::c_int
                        }
                    }
                    loop 
                         /* Return addresses for all valid contexts which don't yet have one */
                         {
                        c =
                            address6_allocate((*state).context, (*state).clid,
                                              (*state).clid_len,
                                              ((*state).ia_type ==
                                                   4 as libc::c_int) as
                                                  libc::c_int, (*state).iaid,
                                              ia_counter, solicit_tags,
                                              plain_range, &mut addr);
                        if c.is_null() { break ; }
                        add_address(state, c, (*c).lease_time,
                                    0 as *mut libc::c_void, &mut min_time,
                                    &mut addr, now);
                        mark_context_used(state, &mut addr);
                        get_context_tag(state, c);
                        address_assigned = 1 as libc::c_int
                    }
                    if address_assigned != 1 as libc::c_int {
                        /* If the server will not assign any addresses to any IAs in a
		   subsequent Request from the client, the server MUST send an Advertise
		   message to the client that doesn't include any IA options. */
                        if (*state).lease_allocate == 0 {
                            save_counter(o);
                            current_block_242 = 13164310931121142693;
                        } else {
                            /* If the server cannot assign any addresses to an IA in the message
		   from the client, the server MUST include the IA in the Reply message
		   with no addresses in the IA and a Status Code option in the IA
		   containing status code NoAddrsAvail. */
                            o1 = new_opt6(13 as libc::c_int);
                            put_opt6_short(2 as libc::c_int as libc::c_uint);
                            put_opt6_string(b"address unavailable\x00" as
                                                *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char);
                            end_opt6(o1);
                            current_block_242 = 15605369199999130895;
                        }
                    } else { current_block_242 = 15605369199999130895; }
                    match current_block_242 {
                        13164310931121142693 => { }
                        _ => {
                            end_ia(t1cntr, min_time, 0 as libc::c_int);
                            end_opt6(o);
                        }
                    }
                }
                opt = opt6_next(opt, (*state).end)
            }
            if address_assigned != 0 {
                o1 = new_opt6(13 as libc::c_int);
                put_opt6_short(0 as libc::c_int as libc::c_uint);
                put_opt6_string(b"success\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char);
                end_opt6(o1);
                /* If --dhcp-authoritative is set, we can tell client not to wait for
	       other possible servers */
                o = new_opt6(7 as libc::c_int);
                put_opt6_char(if (*dnsmasq_daemon).options[(17 as libc::c_int
                                                                as
                                                                libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                 as
                                                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_ulong))
                                                               as usize] &
                                     (1 as libc::c_uint) <<
                                         (17 as libc::c_int as
                                              libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                     != 0 {
                                  255 as libc::c_int
                              } else { 0 as libc::c_int } as libc::c_uint);
                end_opt6(o);
                tagif = add_options(state, 0 as libc::c_int)
            } else {
                /* no address, return error */
                o1 = new_opt6(13 as libc::c_int);
                put_opt6_short(2 as libc::c_int as libc::c_uint);
                put_opt6_string(b"no addresses available\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char);
                end_opt6(o1);
                /* Some clients will ask repeatedly when we're not giving
	       out addresses because we're in stateless mode. Avoid spamming
	       the log in that case. */
                c = (*state).context;
                while !c.is_null() {
                    if (*c).flags as libc::c_uint &
                           (1 as libc::c_uint) << 7 as libc::c_int == 0 {
                        log6_packet(state,
                                    if (*state).lease_allocate != 0 {
                                        b"DHCPREPLY\x00" as *const u8 as
                                            *const libc::c_char
                                    } else {
                                        b"DHCPADVERTISE\x00" as *const u8 as
                                            *const libc::c_char
                                    } as *mut libc::c_char,
                                    0 as *mut in6_addr,
                                    b"no addresses available\x00" as *const u8
                                        as *const libc::c_char as
                                        *mut libc::c_char);
                        break ;
                    } else { c = (*c).current }
                }
            }
        }
        _ => { }
    }
    log_tags(tagif, (*state).xid);
    log6_opts(0 as libc::c_int, (*state).xid,
              (*dnsmasq_daemon).outpacket.iov_base.offset(start_opts as
                                                              isize),
              (*dnsmasq_daemon).outpacket.iov_base.offset(save_counter(-(1 as
                                                                             libc::c_int))
                                                              as isize));
    return 1 as libc::c_int;
}
unsafe extern "C" fn add_options(mut state: *mut state,
                                 mut do_refresh: libc::c_int)
 -> *mut dhcp_netid {
    let mut oro: *mut libc::c_void = 0 as *mut libc::c_void;
    /* filter options based on tags, those we want get DHOPT_TAGOK bit set */
    let mut tagif: *mut dhcp_netid =
        option_filter((*state).tags, (*state).context_tags,
                      (*dnsmasq_daemon).dhcp_opts6);
    let mut opt_cfg: *mut dhcp_opt = 0 as *mut dhcp_opt;
    let mut done_dns: libc::c_int = 0 as libc::c_int;
    let mut done_refresh: libc::c_int = (do_refresh == 0) as libc::c_int;
    let mut do_encap: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut o: libc::c_int = 0;
    let mut o1: libc::c_int = 0;
    oro =
        opt6_find((*state).packet_options, (*state).end,
                  6 as libc::c_int as libc::c_uint,
                  0 as libc::c_int as libc::c_uint);
    let mut current_block_45: u64;
    opt_cfg = (*dnsmasq_daemon).dhcp_opts6;
    while !opt_cfg.is_null() {
        /* netids match and not encapsulated? */
        if !((*opt_cfg).flags & 4096 as libc::c_int == 0) {
            if (*opt_cfg).flags & 16 as libc::c_int == 0 && !oro.is_null() {
                i = 0 as libc::c_int;
                while i <
                          opt6_uint(oro as *mut libc::c_uchar,
                                    -(2 as libc::c_int), 2 as libc::c_int) as
                              libc::c_int - 1 as libc::c_int {
                    if opt6_uint(oro as *mut libc::c_uchar, i,
                                 2 as libc::c_int) ==
                           (*opt_cfg).opt as libc::c_uint {
                        break ;
                    }
                    i += 2 as libc::c_int
                }
                /* option not requested */
                if i >=
                       opt6_uint(oro as *mut libc::c_uchar,
                                 -(2 as libc::c_int), 2 as libc::c_int) as
                           libc::c_int - 1 as libc::c_int {
                    current_block_45 = 735147466149431745;
                } else { current_block_45 = 11050875288958768710; }
            } else { current_block_45 = 11050875288958768710; }
            match current_block_45 {
                735147466149431745 => { }
                _ => {
                    if (*opt_cfg).opt == 32 as libc::c_int {
                        done_refresh = 1 as libc::c_int
                    }
                    if (*opt_cfg).opt == 23 as libc::c_int {
                        done_dns = 1 as libc::c_int
                    }
                    if (*opt_cfg).flags & 8192 as libc::c_int != 0 {
                        let mut len: libc::c_int = 0;
                        let mut j: libc::c_int = 0;
                        let mut a: *mut in6_addr = 0 as *mut in6_addr;
                        a = (*opt_cfg).val as *mut in6_addr;
                        len = (*opt_cfg).len;
                        j = 0 as libc::c_int;
                        while j < (*opt_cfg).len {
                            if *(a as
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
                                   ({
                                        let mut __a: *const in6_addr =
                                            (*state).ula_addr as
                                                *const in6_addr;
                                        ((*__a).__in6_u.__u6_addr32[0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                             ==
                                             0 as libc::c_int as libc::c_uint
                                             &&
                                             (*__a).__in6_u.__u6_addr32[1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                 ==
                                                 0 as libc::c_int as
                                                     libc::c_uint &&
                                             (*__a).__in6_u.__u6_addr32[2 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                 ==
                                                 0 as libc::c_int as
                                                     libc::c_uint &&
                                             (*__a).__in6_u.__u6_addr32[3 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                 ==
                                                 0 as libc::c_int as
                                                     libc::c_uint) as
                                            libc::c_int
                                    }) != 0 ||
                                   *(a as
                                         *const uint32_t).offset(0 as
                                                                     libc::c_int
                                                                     as isize)
                                       ==
                                       __bswap_32(0xfe800000 as libc::c_uint)
                                       &&
                                       *(a as
                                             *const uint32_t).offset(1 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                           == 0 as libc::c_int as libc::c_uint
                                       &&
                                       *(a as
                                             *const uint32_t).offset(2 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                           == 0 as libc::c_int as libc::c_uint
                                       &&
                                       *(a as
                                             *const uint32_t).offset(3 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                           == 0 as libc::c_int as libc::c_uint
                                       &&
                                       ({
                                            let mut __a: *const in6_addr =
                                                (*state).ll_addr as
                                                    *const in6_addr;
                                            ((*__a).__in6_u.__u6_addr32[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                 ==
                                                 0 as libc::c_int as
                                                     libc::c_uint &&
                                                 (*__a).__in6_u.__u6_addr32[1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                     ==
                                                     0 as libc::c_int as
                                                         libc::c_uint &&
                                                 (*__a).__in6_u.__u6_addr32[2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                     ==
                                                     0 as libc::c_int as
                                                         libc::c_uint &&
                                                 (*__a).__in6_u.__u6_addr32[3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                     ==
                                                     0 as libc::c_int as
                                                         libc::c_uint) as
                                                libc::c_int
                                        }) != 0 {
                                len -= 16 as libc::c_int
                            }
                            j += 16 as libc::c_int;
                            a = a.offset(1)
                        }
                        if len != 0 as libc::c_int {
                            o = new_opt6((*opt_cfg).opt);
                            a = (*opt_cfg).val as *mut in6_addr;
                            j = 0 as libc::c_int;
                            while j < (*opt_cfg).len {
                                let mut p: *mut in6_addr = 0 as *mut in6_addr;
                                if ({
                                        let mut __a: *const in6_addr =
                                            a as *const in6_addr;
                                        ((*__a).__in6_u.__u6_addr32[0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                             ==
                                             0 as libc::c_int as libc::c_uint
                                             &&
                                             (*__a).__in6_u.__u6_addr32[1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                 ==
                                                 0 as libc::c_int as
                                                     libc::c_uint &&
                                             (*__a).__in6_u.__u6_addr32[2 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                 ==
                                                 0 as libc::c_int as
                                                     libc::c_uint &&
                                             (*__a).__in6_u.__u6_addr32[3 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                 ==
                                                 0 as libc::c_int as
                                                     libc::c_uint) as
                                            libc::c_int
                                    }) != 0 {
                                    if add_local_addrs((*state).context) == 0
                                       {
                                        p = (*state).fallback
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
                                                    *const uint32_t).offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                                  ==
                                                  0 as libc::c_int as
                                                      libc::c_uint &&
                                              *(a as
                                                    *const uint32_t).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                                  ==
                                                  0 as libc::c_int as
                                                      libc::c_uint &&
                                              *(a as
                                                    *const uint32_t).offset(3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                                  ==
                                                  0 as libc::c_int as
                                                      libc::c_uint {
                                    if ({
                                            let mut __a: *const in6_addr =
                                                (*state).ula_addr as
                                                    *const in6_addr;
                                            ((*__a).__in6_u.__u6_addr32[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                 ==
                                                 0 as libc::c_int as
                                                     libc::c_uint &&
                                                 (*__a).__in6_u.__u6_addr32[1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                     ==
                                                     0 as libc::c_int as
                                                         libc::c_uint &&
                                                 (*__a).__in6_u.__u6_addr32[2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                     ==
                                                     0 as libc::c_int as
                                                         libc::c_uint &&
                                                 (*__a).__in6_u.__u6_addr32[3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                     ==
                                                     0 as libc::c_int as
                                                         libc::c_uint) as
                                                libc::c_int
                                        }) == 0 {
                                        p = (*state).ula_addr
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
                                                    *const uint32_t).offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                                  ==
                                                  0 as libc::c_int as
                                                      libc::c_uint &&
                                              *(a as
                                                    *const uint32_t).offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                                  ==
                                                  0 as libc::c_int as
                                                      libc::c_uint &&
                                              *(a as
                                                    *const uint32_t).offset(3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                                  ==
                                                  0 as libc::c_int as
                                                      libc::c_uint {
                                    if ({
                                            let mut __a: *const in6_addr =
                                                (*state).ll_addr as
                                                    *const in6_addr;
                                            ((*__a).__in6_u.__u6_addr32[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                 ==
                                                 0 as libc::c_int as
                                                     libc::c_uint &&
                                                 (*__a).__in6_u.__u6_addr32[1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                     ==
                                                     0 as libc::c_int as
                                                         libc::c_uint &&
                                                 (*__a).__in6_u.__u6_addr32[2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                     ==
                                                     0 as libc::c_int as
                                                         libc::c_uint &&
                                                 (*__a).__in6_u.__u6_addr32[3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                     ==
                                                     0 as libc::c_int as
                                                         libc::c_uint) as
                                                libc::c_int
                                        }) == 0 {
                                        p = (*state).ll_addr
                                    }
                                } else { p = a }
                                if !p.is_null() {
                                    if (*opt_cfg).opt == 56 as libc::c_int {
                                        if *(p as
                                                 *const uint8_t).offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                               as libc::c_int ==
                                               0xff as libc::c_int {
                                            o1 = new_opt6(2 as libc::c_int)
                                        } else {
                                            o1 = new_opt6(1 as libc::c_int)
                                        }
                                        put_opt6(p as *mut libc::c_void,
                                                 16 as libc::c_int as size_t);
                                        end_opt6(o1);
                                    } else {
                                        put_opt6(p as *mut libc::c_void,
                                                 16 as libc::c_int as size_t);
                                    }
                                }
                                j += 16 as libc::c_int;
                                a = a.offset(1)
                            }
                            end_opt6(o);
                        }
                    } else {
                        o = new_opt6((*opt_cfg).opt);
                        if !(*opt_cfg).val.is_null() {
                            put_opt6((*opt_cfg).val as *mut libc::c_void,
                                     (*opt_cfg).len as size_t);
                        }
                        end_opt6(o);
                    }
                }
            }
        }
        opt_cfg = (*opt_cfg).next
    }
    if (*dnsmasq_daemon).port == 53 as libc::c_int && done_dns == 0 {
        o = new_opt6(23 as libc::c_int);
        if add_local_addrs((*state).context) == 0 {
            put_opt6((*state).fallback as *mut libc::c_void,
                     16 as libc::c_int as size_t);
        }
        end_opt6(o);
    }
    if !(*state).context.is_null() && done_refresh == 0 {
        let mut c: *mut dhcp_context = 0 as *mut dhcp_context;
        let mut lease_time: libc::c_uint = 0xffffffff as libc::c_uint;
        /* Find the smallest lease tie of all contexts,
	 subject to the RFC-4242 stipulation that this must not 
	 be less than 600. */
        c = (*state).context;
        while !c.is_null() {
            if (*c).lease_time < lease_time {
                if (*c).lease_time < 600 as libc::c_int as libc::c_uint {
                    lease_time = 600 as libc::c_int as libc::c_uint
                } else { lease_time = (*c).lease_time }
            }
            c = (*c).next
        }
        o = new_opt6(32 as libc::c_int);
        put_opt6_long(lease_time);
        end_opt6(o);
    }
    /* handle vendor-identifying vendor-encapsulated options,
       dhcp-option = vi-encap:13,17,....... */
    opt_cfg = (*dnsmasq_daemon).dhcp_opts6;
    while !opt_cfg.is_null() {
        (*opt_cfg).flags &= !(64 as libc::c_int);
        opt_cfg = (*opt_cfg).next
    }
    if !oro.is_null() {
        i = 0 as libc::c_int;
        while i <
                  opt6_uint(oro as *mut libc::c_uchar, -(2 as libc::c_int),
                            2 as libc::c_int) as libc::c_int -
                      1 as libc::c_int {
            if opt6_uint(oro as *mut libc::c_uchar, i, 2 as libc::c_int) ==
                   17 as libc::c_int as libc::c_uint {
                do_encap = 1 as libc::c_int
            }
            i += 2 as libc::c_int
        }
    }
    opt_cfg = (*dnsmasq_daemon).dhcp_opts6;
    while !opt_cfg.is_null() {
        if (*opt_cfg).flags & 2048 as libc::c_int != 0 {
            let mut found: libc::c_int = 0 as libc::c_int;
            let mut oc: *mut dhcp_opt = 0 as *mut dhcp_opt;
            if !((*opt_cfg).flags & 64 as libc::c_int != 0) {
                oc = (*dnsmasq_daemon).dhcp_opts6;
                while !oc.is_null() {
                    (*oc).flags &= !(8 as libc::c_int);
                    if !((*oc).flags & 2048 as libc::c_int == 0 ||
                             (*opt_cfg).u.encap != (*oc).u.encap) {
                        (*oc).flags |= 64 as libc::c_int;
                        if match_netid((*oc).netid, tagif, 1 as libc::c_int)
                               != 0 {
                            /* option requested/forced? */
                            if oro.is_null() || do_encap != 0 ||
                                   (*oc).flags & 16 as libc::c_int != 0 {
                                (*oc).flags |= 8 as libc::c_int;
                                found = 1 as libc::c_int
                            }
                        }
                    }
                    oc = (*oc).next
                }
                if found != 0 {
                    o = new_opt6(17 as libc::c_int);
                    put_opt6_long((*opt_cfg).u.encap as libc::c_uint);
                    oc = (*dnsmasq_daemon).dhcp_opts6;
                    while !oc.is_null() {
                        if (*oc).flags & 8 as libc::c_int != 0 {
                            o1 = new_opt6((*oc).opt);
                            put_opt6((*oc).val as *mut libc::c_void,
                                     (*oc).len as size_t);
                            end_opt6(o1);
                        }
                        oc = (*oc).next
                    }
                    end_opt6(o);
                }
            }
        }
        opt_cfg = (*opt_cfg).next
    }
    if !(*state).hostname.is_null() {
        let mut p_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut len_0: size_t = strlen((*state).hostname);
        if !(*state).send_domain.is_null() {
            len_0 =
                (len_0 as
                     libc::c_ulong).wrapping_add(strlen((*state).send_domain).wrapping_add(2
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong))
                    as size_t as size_t
        }
        o = new_opt6(39 as libc::c_int);
        p_0 =
            expand(len_0.wrapping_add(2 as libc::c_int as libc::c_ulong)) as
                *mut libc::c_uchar;
        if !p_0.is_null() {
            let fresh7 = p_0;
            p_0 = p_0.offset(1);
            *fresh7 = (*state).fqdn_flags as libc::c_uchar;
            p_0 =
                do_rfc1035_name(p_0, (*state).hostname,
                                0 as *mut libc::c_char);
            if !(*state).send_domain.is_null() {
                p_0 =
                    do_rfc1035_name(p_0, (*state).send_domain,
                                    0 as *mut libc::c_char);
                *p_0 = 0 as libc::c_int as libc::c_uchar
            }
        }
        end_opt6(o);
    }
    /* logging */
    if (*dnsmasq_daemon).options[(28 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (28 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 && !oro.is_null() {
        let mut q: *mut libc::c_char = (*dnsmasq_daemon).namebuff;
        i = 0 as libc::c_int;
        while i <
                  opt6_uint(oro as *mut libc::c_uchar, -(2 as libc::c_int),
                            2 as libc::c_int) as libc::c_int -
                      1 as libc::c_int {
            let mut s: *mut libc::c_char =
                option_string(10 as libc::c_int,
                              opt6_uint(oro as *mut libc::c_uchar, i,
                                        2 as libc::c_int),
                              0 as *mut libc::c_uchar, 0 as libc::c_int,
                              0 as *mut libc::c_char, 0 as libc::c_int);
            q =
                q.offset(snprintf(q,
                                  (1025 as libc::c_int as libc::c_long -
                                       q.wrapping_offset_from((*dnsmasq_daemon).namebuff)
                                           as libc::c_long) as libc::c_ulong,
                                  b"%d%s%s%s\x00" as *const u8 as
                                      *const libc::c_char,
                                  opt6_uint(oro as *mut libc::c_uchar, i,
                                            2 as libc::c_int),
                                  if strlen(s) !=
                                         0 as libc::c_int as libc::c_ulong {
                                      b":\x00" as *const u8 as
                                          *const libc::c_char
                                  } else {
                                      b"\x00" as *const u8 as
                                          *const libc::c_char
                                  }, s,
                                  if i >
                                         opt6_uint(oro as *mut libc::c_uchar,
                                                   -(2 as libc::c_int),
                                                   2 as libc::c_int) as
                                             libc::c_int - 3 as libc::c_int {
                                      b"\x00" as *const u8 as
                                          *const libc::c_char
                                  } else {
                                      b", \x00" as *const u8 as
                                          *const libc::c_char
                                  }) as isize);
            if i >
                   opt6_uint(oro as *mut libc::c_uchar, -(2 as libc::c_int),
                             2 as libc::c_int) as libc::c_int -
                       3 as libc::c_int ||
                   q.wrapping_offset_from((*dnsmasq_daemon).namebuff) as
                       libc::c_long > 40 as libc::c_int as libc::c_long {
                q = (*dnsmasq_daemon).namebuff;
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              6 as libc::c_int,
                          b"%u requested options: %s\x00" as *const u8 as
                              *const libc::c_char, (*state).xid,
                          (*dnsmasq_daemon).namebuff);
            }
            i += 2 as libc::c_int
        }
    }
    return tagif;
}
unsafe extern "C" fn add_local_addrs(mut context: *mut dhcp_context)
 -> libc::c_int {
    let mut done: libc::c_int = 0 as libc::c_int;
    while !context.is_null() {
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 15 as libc::c_int != 0 &&
               ({
                    let mut __a: *const in6_addr =
                        &mut (*context).local6 as *mut in6_addr as
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
                }) == 0 {
            /* squash duplicates */
            let mut c: *mut dhcp_context = 0 as *mut dhcp_context;
            c = (*context).current;
            while !c.is_null() {
                if (*c).flags as libc::c_uint &
                       (1 as libc::c_uint) << 15 as libc::c_int != 0 &&
                       ({
                            let mut __a: *const in6_addr =
                                &mut (*context).local6 as *mut in6_addr as
                                    *const in6_addr;
                            let mut __b: *const in6_addr =
                                &mut (*c).local6 as *mut in6_addr as
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
                    break ;
                }
                c = (*c).current
            }
            if c.is_null() {
                done = 1 as libc::c_int;
                put_opt6(&mut (*context).local6 as *mut in6_addr as
                             *mut libc::c_void, 16 as libc::c_int as size_t);
            }
        }
        context = (*context).current
    }
    return done;
}
unsafe extern "C" fn get_context_tag(mut state: *mut state,
                                     mut context: *mut dhcp_context) {
    /* get tags from context if we've not used it before */
    if (*context).netid.next == &mut (*context).netid as *mut dhcp_netid &&
           !(*context).netid.net.is_null() {
        (*context).netid.next = (*state).context_tags;
        (*state).context_tags = &mut (*context).netid;
        if (*state).hostname_auth == 0 {
            let mut id_list: *mut dhcp_netid_list = 0 as *mut dhcp_netid_list;
            id_list = (*dnsmasq_daemon).dhcp_ignore_names;
            while !id_list.is_null() {
                if (*id_list).list.is_null() ||
                       match_netid((*id_list).list, &mut (*context).netid,
                                   0 as libc::c_int) != 0 {
                    break ;
                }
                id_list = (*id_list).next
            }
            if !id_list.is_null() {
                (*state).hostname = 0 as *mut libc::c_char
            }
        }
    };
}
unsafe extern "C" fn check_ia(mut state: *mut state,
                              mut opt: *mut libc::c_void,
                              mut endp: *mut *mut libc::c_void,
                              mut ia_option: *mut *mut libc::c_void)
 -> libc::c_int {
    (*state).ia_type =
        opt6_uint(opt as *mut libc::c_uchar, -(4 as libc::c_int),
                  2 as libc::c_int) as libc::c_int;
    *ia_option = 0 as *mut libc::c_void;
    if (*state).ia_type != 3 as libc::c_int &&
           (*state).ia_type != 4 as libc::c_int {
        return 0 as libc::c_int
    }
    if (*state).ia_type == 3 as libc::c_int &&
           (opt6_uint(opt as *mut libc::c_uchar, -(2 as libc::c_int),
                      2 as libc::c_int) as libc::c_int) < 12 as libc::c_int {
        return 0 as libc::c_int
    }
    if (*state).ia_type == 4 as libc::c_int &&
           (opt6_uint(opt as *mut libc::c_uchar, -(2 as libc::c_int),
                      2 as libc::c_int) as libc::c_int) < 4 as libc::c_int {
        return 0 as libc::c_int
    }
    *endp =
        &mut *(opt as
                   *mut libc::c_uchar).offset((4 as libc::c_int +
                                                   (opt6_uint as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut libc::c_uchar,
                                                                             _:
                                                                                 libc::c_int,
                                                                             _:
                                                                                 libc::c_int)
                                                            ->
                                                                libc::c_uint)(opt
                                                                                  as
                                                                                  *mut libc::c_uchar,
                                                                              -(2
                                                                                    as
                                                                                    libc::c_int),
                                                                              2
                                                                                  as
                                                                                  libc::c_int)
                                                       as libc::c_int) as
                                                  isize) as *mut libc::c_uchar
            as *mut libc::c_void;
    (*state).iaid =
        opt6_uint(opt as *mut libc::c_uchar, 0 as libc::c_int,
                  4 as libc::c_int);
    *ia_option =
        opt6_find(&mut *(opt as
                             *mut libc::c_uchar).offset((4 as libc::c_int +
                                                             (if (*state).ia_type
                                                                     ==
                                                                     3 as
                                                                         libc::c_int
                                                                 {
                                                                  12 as
                                                                      libc::c_int
                                                              } else {
                                                                  4 as
                                                                      libc::c_int
                                                              })) as isize) as
                      *mut libc::c_uchar as *mut libc::c_void, *endp,
                  5 as libc::c_int as libc::c_uint,
                  24 as libc::c_int as libc::c_uint);
    return 1 as libc::c_int;
}
unsafe extern "C" fn build_ia(mut state: *mut state,
                              mut t1cntr: *mut libc::c_int) -> libc::c_int {
    let mut o: libc::c_int = new_opt6((*state).ia_type);
    put_opt6_long((*state).iaid);
    *t1cntr = 0 as libc::c_int;
    if (*state).ia_type == 3 as libc::c_int {
        /* save pointer */
        *t1cntr = save_counter(-(1 as libc::c_int));
        /* so we can fill these in later */
        put_opt6_long(0 as libc::c_int as libc::c_uint);
        put_opt6_long(0 as libc::c_int as libc::c_uint);
    }
    return o;
}
unsafe extern "C" fn end_ia(mut t1cntr: libc::c_int,
                            mut min_time: libc::c_uint,
                            mut do_fuzz: libc::c_int) {
    if t1cntr != 0 as libc::c_int {
        /* go back and fill in fields in IA_NA option */
        let mut sav: libc::c_int = save_counter(t1cntr);
        let mut t1: libc::c_uint = 0;
        let mut t2: libc::c_uint = 0;
        let mut fuzz: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        if do_fuzz != 0 {
            fuzz = rand16() as libc::c_uint;
            while fuzz >
                      min_time.wrapping_div(16 as libc::c_int as libc::c_uint)
                  {
                fuzz = fuzz.wrapping_div(2 as libc::c_int as libc::c_uint)
            }
        }
        t1 =
            if min_time == 0xffffffff as libc::c_uint {
                0xffffffff as libc::c_uint
            } else {
                min_time.wrapping_div(2 as libc::c_int as
                                          libc::c_uint).wrapping_sub(fuzz)
            };
        t2 =
            if min_time == 0xffffffff as libc::c_uint {
                0xffffffff as libc::c_uint
            } else {
                min_time.wrapping_div(8 as libc::c_int as
                                          libc::c_uint).wrapping_mul(7 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint).wrapping_sub(fuzz)
            };
        put_opt6_long(t1);
        put_opt6_long(t2);
        save_counter(sav);
    };
}
unsafe extern "C" fn add_address(mut state: *mut state,
                                 mut context: *mut dhcp_context,
                                 mut lease_time: libc::c_uint,
                                 mut ia_option: *mut libc::c_void,
                                 mut min_time: *mut libc::c_uint,
                                 mut addr: *mut in6_addr, mut now: time_t) {
    let mut valid_time: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut preferred_time: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut o: libc::c_int = new_opt6(5 as libc::c_int);
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    /* get client requested times */
    if !ia_option.is_null() {
        preferred_time =
            opt6_uint(ia_option as *mut libc::c_uchar, 16 as libc::c_int,
                      4 as libc::c_int);
        valid_time =
            opt6_uint(ia_option as *mut libc::c_uchar, 20 as libc::c_int,
                      4 as libc::c_int)
    }
    calculate_times(context, min_time, &mut valid_time, &mut preferred_time,
                    lease_time);
    put_opt6(addr as *mut libc::c_void,
             ::std::mem::size_of::<in6_addr>() as libc::c_ulong);
    put_opt6_long(preferred_time);
    put_opt6_long(valid_time);
    end_opt6(o);
    if (*state).lease_allocate != 0 {
        update_leases(state, context, addr, valid_time, now);
    }
    lease =
        lease6_find_by_addr(addr, 128 as libc::c_int,
                            0 as libc::c_int as u64_0);
    if !lease.is_null() { (*lease).flags |= 16 as libc::c_int }
    /* get tags from context if we've not used it before */
    if (*context).netid.next == &mut (*context).netid as *mut dhcp_netid &&
           !(*context).netid.net.is_null() {
        (*context).netid.next = (*state).context_tags;
        (*state).context_tags = &mut (*context).netid;
        if (*state).hostname_auth == 0 {
            let mut id_list: *mut dhcp_netid_list = 0 as *mut dhcp_netid_list;
            id_list = (*dnsmasq_daemon).dhcp_ignore_names;
            while !id_list.is_null() {
                if (*id_list).list.is_null() ||
                       match_netid((*id_list).list, &mut (*context).netid,
                                   0 as libc::c_int) != 0 {
                    break ;
                }
                id_list = (*id_list).next
            }
            if !id_list.is_null() {
                (*state).hostname = 0 as *mut libc::c_char
            }
        }
    }
    log6_quiet(state,
               if (*state).lease_allocate != 0 {
                   b"DHCPREPLY\x00" as *const u8 as *const libc::c_char
               } else {
                   b"DHCPADVERTISE\x00" as *const u8 as *const libc::c_char
               } as *mut libc::c_char, addr, (*state).hostname);
}
unsafe extern "C" fn mark_context_used(mut state: *mut state,
                                       mut addr: *mut in6_addr) {
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    /* Mark that we have an address for this prefix. */
    context = (*state).context;
    while !context.is_null() {
        if is_same_net6(addr, &mut (*context).start6, (*context).prefix) != 0
           {
            (*context).flags =
                ((*context).flags as libc::c_uint |
                     (1 as libc::c_uint) << 15 as libc::c_int) as libc::c_int
        }
        context = (*context).current
    };
}
unsafe extern "C" fn mark_config_used(mut context: *mut dhcp_context,
                                      mut addr: *mut in6_addr) {
    while !context.is_null() {
        if is_same_net6(addr, &mut (*context).start6, (*context).prefix) != 0
           {
            (*context).flags =
                ((*context).flags as libc::c_uint |
                     (1 as libc::c_uint) << 14 as libc::c_int) as libc::c_int
        }
        context = (*context).current
    };
}
/* make sure address not leased to another CLID/IAID */
unsafe extern "C" fn check_address(mut state: *mut state,
                                   mut addr: *mut in6_addr) -> libc::c_int {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease =
        lease6_find_by_addr(addr, 128 as libc::c_int,
                            0 as libc::c_int as u64_0);
    if lease.is_null() { return 1 as libc::c_int }
    if (*lease).clid_len != (*state).clid_len ||
           memcmp((*lease).clid as *const libc::c_void,
                  (*state).clid as *const libc::c_void,
                  (*state).clid_len as libc::c_ulong) != 0 as libc::c_int ||
           (*lease).iaid != (*state).iaid {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* return true of *addr could have been generated from config. */
unsafe extern "C" fn config_implies(mut config: *mut dhcp_config,
                                    mut context: *mut dhcp_context,
                                    mut addr: *mut in6_addr)
 -> *mut addrlist {
    let mut prefix: libc::c_int = 0;
    let mut wild_addr: in6_addr =
        in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    let mut addr_list: *mut addrlist = 0 as *mut addrlist;
    if config.is_null() ||
           (*config).flags & 4096 as libc::c_int as libc::c_uint == 0 {
        return 0 as *mut addrlist
    }
    let mut current_block_9: u64;
    addr_list = (*config).addr6;
    while !addr_list.is_null() {
        prefix =
            if (*addr_list).flags & 8 as libc::c_int != 0 {
                (*addr_list).prefixlen
            } else { 128 as libc::c_int };
        wild_addr = (*addr_list).addr.addr6;
        if (*addr_list).flags & 16 as libc::c_int != 0 &&
               (*context).prefix == 64 as libc::c_int {
            wild_addr = (*context).start6;
            setaddr6part(&mut wild_addr,
                         addr6part(&mut (*addr_list).addr.addr6));
            current_block_9 = 7746791466490516765;
        } else if is_same_net6(&mut (*context).start6, addr,
                               (*context).prefix) == 0 {
            current_block_9 = 17179679302217393232;
        } else { current_block_9 = 7746791466490516765; }
        match current_block_9 {
            7746791466490516765 => {
                if is_same_net6(&mut wild_addr, addr, prefix) != 0 {
                    return addr_list
                }
            }
            _ => { }
        }
        addr_list = (*addr_list).next
    }
    return 0 as *mut addrlist;
}
unsafe extern "C" fn config_valid(mut config: *mut dhcp_config,
                                  mut context: *mut dhcp_context,
                                  mut addr: *mut in6_addr,
                                  mut state: *mut state, mut now: time_t)
 -> libc::c_int {
    let mut addrpart: u64_0 = 0;
    let mut i: u64_0 = 0;
    let mut addresses: u64_0 = 0;
    let mut addr_list: *mut addrlist = 0 as *mut addrlist;
    if config.is_null() ||
           (*config).flags & 4096 as libc::c_int as libc::c_uint == 0 {
        return 0 as libc::c_int
    }
    let mut current_block_14: u64;
    addr_list = (*config).addr6;
    while !addr_list.is_null() {
        if (*addr_list).flags & 32 as libc::c_int == 0 ||
               difftime(now, (*addr_list).decline_time) >=
                   600 as libc::c_int as libc::c_float as libc::c_double {
            addrpart = addr6part(&mut (*addr_list).addr.addr6);
            addresses = 1 as libc::c_int as u64_0;
            if (*addr_list).flags & 8 as libc::c_int != 0 {
                addresses =
                    (1 as libc::c_int as u64_0) <<
                        128 as libc::c_int - (*addr_list).prefixlen
            }
            if (*addr_list).flags & 16 as libc::c_int != 0 {
                if (*context).prefix != 64 as libc::c_int {
                    current_block_14 = 10680521327981672866;
                } else {
                    *addr = (*context).start6;
                    current_block_14 = 3512920355445576850;
                }
            } else if is_same_net6(&mut (*context).start6,
                                   &mut (*addr_list).addr.addr6,
                                   (*context).prefix) != 0 {
                *addr = (*addr_list).addr.addr6;
                current_block_14 = 3512920355445576850;
            } else { current_block_14 = 10680521327981672866; }
            match current_block_14 {
                10680521327981672866 => { }
                _ => {
                    i = 0 as libc::c_int as u64_0;
                    while i < addresses {
                        setaddr6part(addr, addrpart.wrapping_add(i));
                        if check_address(state, addr) != 0 {
                            return 1 as libc::c_int
                        }
                        i = i.wrapping_add(1)
                    }
                }
            }
        }
        addr_list = (*addr_list).next
    }
    return 0 as libc::c_int;
}
/* Calculate valid and preferred times to send in leases/renewals. 

   Inputs are:

   *valid_timep, *preferred_timep - requested times from IAADDR options.
   context->valid, context->preferred - times associated with subnet address on local interface.
   context->flags | CONTEXT_DEPRECATE - "deprecated" flag in dhcp-range.
   lease_time - configured time for context for individual client.
   *min_time - smallest valid time sent so far.

   Outputs are :
   
   *valid_timep, *preferred_timep - times to be send in IAADDR option.
   *min_time - smallest valid time sent so far, to calculate T1 and T2.
   
   */
unsafe extern "C" fn calculate_times(mut context: *mut dhcp_context,
                                     mut min_time: *mut libc::c_uint,
                                     mut valid_timep: *mut libc::c_uint,
                                     mut preferred_timep: *mut libc::c_uint,
                                     mut lease_time: libc::c_uint) {
    let mut req_preferred: libc::c_uint = *preferred_timep;
    let mut req_valid: libc::c_uint = *valid_timep;
    let mut valid_time: libc::c_uint = lease_time;
    let mut preferred_time: libc::c_uint = lease_time;
    /* RFC 3315: "A server ignores the lifetimes set
     by the client if the preferred lifetime is greater than the valid
     lifetime. */
    if req_preferred <= req_valid {
        if req_preferred != 0 as libc::c_int as libc::c_uint {
            /* 0 == "no preference from client" */
            if req_preferred < 120 as libc::c_uint {
                req_preferred = 120 as libc::c_uint
            } /* sanity */
            if req_preferred < preferred_time {
                preferred_time = req_preferred
            }
        }
        if req_valid != 0 as libc::c_int as libc::c_uint {
            /* 0 == "no preference from client" */
            if req_valid < 120 as libc::c_uint {
                req_valid = 120 as libc::c_uint
            } /* sanity */
            if req_valid < valid_time { valid_time = req_valid }
        }
    }
    /* deprecate (preferred == 0) which configured, or when local address 
     is deprecated */
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 9 as libc::c_int != 0 ||
           (*context).preferred == 0 as libc::c_int as libc::c_uint {
        preferred_time = 0 as libc::c_int as libc::c_uint
    }
    if preferred_time != 0 as libc::c_int as libc::c_uint &&
           preferred_time < *min_time {
        *min_time = preferred_time
    }
    if valid_time != 0 as libc::c_int as libc::c_uint &&
           valid_time < *min_time {
        *min_time = valid_time
    }
    *valid_timep = valid_time;
    *preferred_timep = preferred_time;
}
unsafe extern "C" fn update_leases(mut state: *mut state,
                                   mut context: *mut dhcp_context,
                                   mut addr: *mut in6_addr,
                                   mut lease_time: libc::c_uint,
                                   mut now: time_t) {
    let mut lease: *mut dhcp_lease =
        lease6_find_by_addr(addr, 128 as libc::c_int,
                            0 as libc::c_int as u64_0);
    let mut tagif: *mut dhcp_netid = run_tag_if((*state).tags);
    if lease.is_null() {
        lease =
            lease6_allocate(addr,
                            if (*state).ia_type == 3 as libc::c_int {
                                32 as libc::c_int
                            } else { 64 as libc::c_int })
    }
    if !lease.is_null() {
        lease_set_expires(lease, lease_time, now);
        lease_set_iaid(lease, (*state).iaid);
        lease_set_hwaddr(lease, (*state).mac.as_mut_ptr(), (*state).clid,
                         (*state).mac_len as libc::c_int,
                         (*state).mac_type as libc::c_int, (*state).clid_len,
                         now, 0 as libc::c_int);
        lease_set_interface(lease, (*state).interface, now);
        if !(*state).hostname.is_null() &&
               (*state).ia_type == 3 as libc::c_int {
            let mut addr_domain: *mut libc::c_char = get_domain6(addr);
            if (*state).send_domain.is_null() {
                (*state).send_domain = addr_domain
            }
            lease_set_hostname(lease, (*state).hostname,
                               (*state).hostname_auth, addr_domain,
                               (*state).domain);
        }
        if !(*dnsmasq_daemon).lease_change_command.is_null() {
            let mut class_opt: *mut libc::c_void = 0 as *mut libc::c_void;
            (*lease).flags |= 2 as libc::c_int;
            free((*lease).extradata as *mut libc::c_void);
            (*lease).extradata = 0 as *mut libc::c_uchar;
            (*lease).extradata_len = 0 as libc::c_int as libc::c_uint;
            (*lease).extradata_size = (*lease).extradata_len;
            (*lease).vendorclass_count = 0 as libc::c_int;
            class_opt =
                opt6_find((*state).packet_options, (*state).end,
                          16 as libc::c_int as libc::c_uint,
                          4 as libc::c_int as libc::c_uint);
            if !class_opt.is_null() {
                let mut enc_opt: *mut libc::c_void = 0 as *mut libc::c_void;
                let mut enc_end: *mut libc::c_void =
                    &mut *(class_opt as
                               *mut libc::c_uchar).offset((4 as libc::c_int +
                                                               (opt6_uint as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut libc::c_uchar,
                                                                                         _:
                                                                                             libc::c_int,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            libc::c_uint)(class_opt
                                                                                              as
                                                                                              *mut libc::c_uchar,
                                                                                          -(2
                                                                                                as
                                                                                                libc::c_int),
                                                                                          2
                                                                                              as
                                                                                              libc::c_int)
                                                                   as
                                                                   libc::c_int)
                                                              as isize) as
                        *mut libc::c_uchar as *mut libc::c_void;
                (*lease).vendorclass_count += 1;
                /* send enterprise number first  */
                sprintf((*dnsmasq_daemon).dhcp_buff2,
                        b"%u\x00" as *const u8 as *const libc::c_char,
                        opt6_uint(class_opt as *mut libc::c_uchar,
                                  0 as libc::c_int, 4 as libc::c_int));
                lease_add_extradata(lease,
                                    (*dnsmasq_daemon).dhcp_buff2 as
                                        *mut libc::c_uchar,
                                    strlen((*dnsmasq_daemon).dhcp_buff2) as
                                        libc::c_uint, 0 as libc::c_int);
                if opt6_uint(class_opt as *mut libc::c_uchar,
                             -(2 as libc::c_int), 2 as libc::c_int) as
                       libc::c_int >= 6 as libc::c_int {
                    enc_opt =
                        &mut *(class_opt as
                                   *mut libc::c_uchar).offset((4 as
                                                                   libc::c_int
                                                                   +
                                                                   4 as
                                                                       libc::c_int)
                                                                  as isize) as
                            *mut libc::c_uchar as *mut libc::c_void;
                    while !enc_opt.is_null() {
                        (*lease).vendorclass_count += 1;
                        lease_add_extradata(lease,
                                            &mut *(enc_opt as
                                                       *mut libc::c_uchar).offset((4
                                                                                       as
                                                                                       libc::c_int
                                                                                       +
                                                                                       0
                                                                                           as
                                                                                           libc::c_int)
                                                                                      as
                                                                                      isize)
                                                as *mut libc::c_uchar as
                                                *mut libc::c_void as
                                                *mut libc::c_uchar,
                                            opt6_uint(enc_opt as
                                                          *mut libc::c_uchar,
                                                      -(2 as libc::c_int),
                                                      2 as libc::c_int) as
                                                libc::c_int as libc::c_uint,
                                            0 as libc::c_int);
                        enc_opt = opt6_next(enc_opt, enc_end)
                    }
                }
            }
            lease_add_extradata(lease,
                                (*state).client_hostname as
                                    *mut libc::c_uchar,
                                if !(*state).client_hostname.is_null() {
                                    strlen((*state).client_hostname)
                                } else { 0 as libc::c_int as libc::c_ulong }
                                    as libc::c_uint, 0 as libc::c_int);
            /* space-concat tag set */
            if tagif.is_null() && (*context).netid.net.is_null() {
                lease_add_extradata(lease, 0 as *mut libc::c_uchar,
                                    0 as libc::c_int as libc::c_uint,
                                    0 as libc::c_int);
            } else {
                if !(*context).netid.net.is_null() {
                    lease_add_extradata(lease,
                                        (*context).netid.net as
                                            *mut libc::c_uchar,
                                        strlen((*context).netid.net) as
                                            libc::c_uint,
                                        if !tagif.is_null() {
                                            ' ' as i32
                                        } else { 0 as libc::c_int });
                }
                if !tagif.is_null() {
                    let mut n: *mut dhcp_netid = 0 as *mut dhcp_netid;
                    n = tagif;
                    while !n.is_null() {
                        let mut n1: *mut dhcp_netid = 0 as *mut dhcp_netid;
                        /* kill dupes */
                        n1 = (*n).next;
                        while !n1.is_null() {
                            if strcmp((*n).net, (*n1).net) == 0 as libc::c_int
                               {
                                break ;
                            }
                            n1 = (*n1).next
                        }
                        if n1.is_null() {
                            lease_add_extradata(lease,
                                                (*n).net as
                                                    *mut libc::c_uchar,
                                                strlen((*n).net) as
                                                    libc::c_uint,
                                                if !(*n).next.is_null() {
                                                    ' ' as i32
                                                } else { 0 as libc::c_int });
                        }
                        n = (*n).next
                    }
                }
            }
            if !(*state).link_address.is_null() {
                inet_ntop(10 as libc::c_int,
                          (*state).link_address as *const libc::c_void,
                          (*dnsmasq_daemon).addrbuff,
                          46 as libc::c_int as socklen_t);
            }
            lease_add_extradata(lease,
                                (*dnsmasq_daemon).addrbuff as
                                    *mut libc::c_uchar,
                                if !(*state).link_address.is_null() {
                                    strlen((*dnsmasq_daemon).addrbuff)
                                } else { 0 as libc::c_int as libc::c_ulong }
                                    as libc::c_uint, 0 as libc::c_int);
            class_opt =
                opt6_find((*state).packet_options, (*state).end,
                          15 as libc::c_int as libc::c_uint,
                          2 as libc::c_int as libc::c_uint);
            if !class_opt.is_null() {
                let mut enc_opt_0: *mut libc::c_void = 0 as *mut libc::c_void;
                let mut enc_end_0: *mut libc::c_void =
                    &mut *(class_opt as
                               *mut libc::c_uchar).offset((4 as libc::c_int +
                                                               (opt6_uint as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut libc::c_uchar,
                                                                                         _:
                                                                                             libc::c_int,
                                                                                         _:
                                                                                             libc::c_int)
                                                                        ->
                                                                            libc::c_uint)(class_opt
                                                                                              as
                                                                                              *mut libc::c_uchar,
                                                                                          -(2
                                                                                                as
                                                                                                libc::c_int),
                                                                                          2
                                                                                              as
                                                                                              libc::c_int)
                                                                   as
                                                                   libc::c_int)
                                                              as isize) as
                        *mut libc::c_uchar as *mut libc::c_void;
                enc_opt_0 =
                    &mut *(class_opt as
                               *mut libc::c_uchar).offset((4 as libc::c_int +
                                                               0 as
                                                                   libc::c_int)
                                                              as isize) as
                        *mut libc::c_uchar as *mut libc::c_void;
                while !enc_opt_0.is_null() {
                    lease_add_extradata(lease,
                                        &mut *(enc_opt_0 as
                                                   *mut libc::c_uchar).offset((4
                                                                                   as
                                                                                   libc::c_int
                                                                                   +
                                                                                   0
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  isize)
                                            as *mut libc::c_uchar as
                                            *mut libc::c_void as
                                            *mut libc::c_uchar,
                                        opt6_uint(enc_opt_0 as
                                                      *mut libc::c_uchar,
                                                  -(2 as libc::c_int),
                                                  2 as libc::c_int) as
                                            libc::c_int as libc::c_uint,
                                        0 as libc::c_int);
                    enc_opt_0 = opt6_next(enc_opt_0, enc_end_0)
                }
            }
        }
    };
}
unsafe extern "C" fn log6_opts(mut nest: libc::c_int, mut xid: libc::c_uint,
                               mut start_opts: *mut libc::c_void,
                               mut end_opts: *mut libc::c_void) {
    let mut opt: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut desc: *mut libc::c_char =
        if nest != 0 {
            b"nest\x00" as *const u8 as *const libc::c_char
        } else { b"sent\x00" as *const u8 as *const libc::c_char } as
            *mut libc::c_char;
    if (*dnsmasq_daemon).options[(28 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (28 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 || start_opts == end_opts {
        return
    }
    opt = start_opts;
    while !opt.is_null() {
        let mut type_0: libc::c_int =
            opt6_uint(opt as *mut libc::c_uchar, -(4 as libc::c_int),
                      2 as libc::c_int) as libc::c_int;
        let mut ia_options: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut optname: *mut libc::c_char = 0 as *mut libc::c_char;
        if type_0 == 3 as libc::c_int {
            sprintf((*dnsmasq_daemon).namebuff,
                    b"IAID=%u T1=%u T2=%u\x00" as *const u8 as
                        *const libc::c_char,
                    opt6_uint(opt as *mut libc::c_uchar, 0 as libc::c_int,
                              4 as libc::c_int),
                    opt6_uint(opt as *mut libc::c_uchar, 4 as libc::c_int,
                              4 as libc::c_int),
                    opt6_uint(opt as *mut libc::c_uchar, 8 as libc::c_int,
                              4 as libc::c_int));
            optname =
                b"ia-na\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            ia_options =
                &mut *(opt as
                           *mut libc::c_uchar).offset((4 as libc::c_int +
                                                           12 as libc::c_int)
                                                          as isize) as
                    *mut libc::c_uchar as *mut libc::c_void
        } else if type_0 == 4 as libc::c_int {
            sprintf((*dnsmasq_daemon).namebuff,
                    b"IAID=%u\x00" as *const u8 as *const libc::c_char,
                    opt6_uint(opt as *mut libc::c_uchar, 0 as libc::c_int,
                              4 as libc::c_int));
            optname =
                b"ia-ta\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            ia_options =
                &mut *(opt as
                           *mut libc::c_uchar).offset((4 as libc::c_int +
                                                           4 as libc::c_int)
                                                          as isize) as
                    *mut libc::c_uchar as *mut libc::c_void
        } else if type_0 == 5 as libc::c_int {
            let mut addr: in6_addr =
                in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
            /* align */
            memcpy(&mut addr as *mut in6_addr as *mut libc::c_void,
                   &mut *(opt as
                              *mut libc::c_uchar).offset((4 as libc::c_int +
                                                              0 as
                                                                  libc::c_int)
                                                             as isize) as
                       *mut libc::c_uchar as *mut libc::c_void,
                   16 as libc::c_int as libc::c_ulong);
            inet_ntop(10 as libc::c_int,
                      &mut addr as *mut in6_addr as *const libc::c_void,
                      (*dnsmasq_daemon).addrbuff,
                      46 as libc::c_int as socklen_t);
            sprintf((*dnsmasq_daemon).namebuff,
                    b"%s PL=%u VL=%u\x00" as *const u8 as *const libc::c_char,
                    (*dnsmasq_daemon).addrbuff,
                    opt6_uint(opt as *mut libc::c_uchar, 16 as libc::c_int,
                              4 as libc::c_int),
                    opt6_uint(opt as *mut libc::c_uchar, 20 as libc::c_int,
                              4 as libc::c_int));
            optname =
                b"iaaddr\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            ia_options =
                &mut *(opt as
                           *mut libc::c_uchar).offset((4 as libc::c_int +
                                                           24 as libc::c_int)
                                                          as isize) as
                    *mut libc::c_uchar as *mut libc::c_void
        } else if type_0 == 13 as libc::c_int {
            let mut len: libc::c_int =
                sprintf((*dnsmasq_daemon).namebuff,
                        b"%u \x00" as *const u8 as *const libc::c_char,
                        opt6_uint(opt as *mut libc::c_uchar, 0 as libc::c_int,
                                  2 as libc::c_int));
            memcpy((*dnsmasq_daemon).namebuff.offset(len as isize) as
                       *mut libc::c_void,
                   &mut *(opt as
                              *mut libc::c_uchar).offset((4 as libc::c_int +
                                                              2 as
                                                                  libc::c_int)
                                                             as isize) as
                       *mut libc::c_uchar as *mut libc::c_void,
                   (opt6_uint(opt as *mut libc::c_uchar, -(2 as libc::c_int),
                              2 as libc::c_int) as libc::c_int -
                        2 as libc::c_int) as libc::c_ulong);
            *(*dnsmasq_daemon).namebuff.offset((len +
                                                    opt6_uint(opt as
                                                                  *mut libc::c_uchar,
                                                              -(2 as
                                                                    libc::c_int),
                                                              2 as
                                                                  libc::c_int)
                                                        as libc::c_int -
                                                    2 as libc::c_int) as
                                                   isize) =
                0 as libc::c_int as libc::c_char;
            optname =
                b"status\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else {
            /* account for flag byte on FQDN */
            let mut offset: libc::c_int =
                if type_0 == 39 as libc::c_int {
                    1 as libc::c_int
                } else { 0 as libc::c_int };
            optname =
                option_string(10 as libc::c_int, type_0 as libc::c_uint,
                              &mut *(opt as
                                         *mut libc::c_uchar).offset((4 as
                                                                         libc::c_int
                                                                         +
                                                                         offset)
                                                                        as
                                                                        isize)
                                  as *mut libc::c_uchar as *mut libc::c_void
                                  as *mut libc::c_uchar,
                              opt6_uint(opt as *mut libc::c_uchar,
                                        -(2 as libc::c_int), 2 as libc::c_int)
                                  as libc::c_int - offset,
                              (*dnsmasq_daemon).namebuff, 1025 as libc::c_int)
        }
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"%u %s size:%3d option:%3d %s  %s\x00" as *const u8 as
                      *const libc::c_char, xid, desc,
                  opt6_uint(opt as *mut libc::c_uchar, -(2 as libc::c_int),
                            2 as libc::c_int) as libc::c_int, type_0, optname,
                  (*dnsmasq_daemon).namebuff);
        if !ia_options.is_null() {
            log6_opts(1 as libc::c_int, xid, ia_options,
                      &mut *(opt as
                                 *mut libc::c_uchar).offset((4 as libc::c_int
                                                                 +
                                                                 (opt6_uint as
                                                                      unsafe extern "C" fn(_:
                                                                                               *mut libc::c_uchar,
                                                                                           _:
                                                                                               libc::c_int,
                                                                                           _:
                                                                                               libc::c_int)
                                                                          ->
                                                                              libc::c_uint)(opt
                                                                                                as
                                                                                                *mut libc::c_uchar,
                                                                                            -(2
                                                                                                  as
                                                                                                  libc::c_int),
                                                                                            2
                                                                                                as
                                                                                                libc::c_int)
                                                                     as
                                                                     libc::c_int)
                                                                as isize) as
                          *mut libc::c_uchar as *mut libc::c_void);
        }
        opt = opt6_next(opt, end_opts)
    };
}
unsafe extern "C" fn log6_quiet(mut state: *mut state,
                                mut type_0: *mut libc::c_char,
                                mut addr: *mut in6_addr,
                                mut string: *mut libc::c_char) {
    if (*dnsmasq_daemon).options[(28 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (28 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 ||
           (*dnsmasq_daemon).options[(43 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (43 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
        log6_packet(state, type_0, addr, string);
    };
}
unsafe extern "C" fn log6_packet(mut state: *mut state,
                                 mut type_0: *mut libc::c_char,
                                 mut addr: *mut in6_addr,
                                 mut string: *mut libc::c_char) {
    let mut clid_len: libc::c_int = (*state).clid_len;
    /* avoid buffer overflow */
    if clid_len > 100 as libc::c_int { clid_len = 100 as libc::c_int }
    print_mac((*dnsmasq_daemon).namebuff, (*state).clid, clid_len);
    if !addr.is_null() {
        inet_ntop(10 as libc::c_int, addr as *const libc::c_void,
                  (*dnsmasq_daemon).dhcp_buff2,
                  (256 as libc::c_int - 1 as libc::c_int) as socklen_t);
        strcat((*dnsmasq_daemon).dhcp_buff2,
               b" \x00" as *const u8 as *const libc::c_char);
    } else {
        *(*dnsmasq_daemon).dhcp_buff2.offset(0 as libc::c_int as isize) =
            0 as libc::c_int as libc::c_char
    }
    if (*dnsmasq_daemon).options[(28 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (28 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"%u %s(%s) %s%s %s\x00" as *const u8 as
                      *const libc::c_char, (*state).xid, type_0,
                  (*state).iface_name, (*dnsmasq_daemon).dhcp_buff2,
                  (*dnsmasq_daemon).namebuff,
                  if !string.is_null() {
                      string as *const libc::c_char
                  } else { b"\x00" as *const u8 as *const libc::c_char });
    } else {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"%s(%s) %s%s %s\x00" as *const u8 as *const libc::c_char,
                  type_0, (*state).iface_name, (*dnsmasq_daemon).dhcp_buff2,
                  (*dnsmasq_daemon).namebuff,
                  if !string.is_null() {
                      string as *const libc::c_char
                  } else { b"\x00" as *const u8 as *const libc::c_char });
    };
}
unsafe extern "C" fn opt6_find(mut opts: *mut libc::c_void,
                               mut end: *mut libc::c_void,
                               mut search: libc::c_uint,
                               mut minsize: libc::c_uint)
 -> *mut libc::c_void {
    let mut opt: u16_0 = 0;
    let mut opt_len: u16_0 = 0;
    let mut start: *mut libc::c_void = 0 as *mut libc::c_void;
    if opts.is_null() { return 0 as *mut libc::c_void }
    loop  {
        if (end.wrapping_offset_from(opts) as libc::c_long) <
               4 as libc::c_int as libc::c_long {
            return 0 as *mut libc::c_void
        }
        start = opts;
        let mut t_cp: *mut libc::c_uchar = opts as *mut libc::c_uchar;
        opt =
            ((*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                 << 8 as libc::c_int |
                 *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                     libc::c_int) as u16_0;
        opts = opts.offset(2 as libc::c_int as isize);
        let mut t_cp_0: *mut libc::c_uchar = opts as *mut libc::c_uchar;
        opt_len =
            ((*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                  libc::c_int) << 8 as libc::c_int |
                 *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                     libc::c_int) as u16_0;
        opts = opts.offset(2 as libc::c_int as isize);
        if opt_len as libc::c_long >
               end.wrapping_offset_from(opts) as libc::c_long {
            return 0 as *mut libc::c_void
        }
        if opt as libc::c_uint == search && opt_len as libc::c_uint >= minsize
           {
            return start
        }
        opts = opts.offset(opt_len as libc::c_int as isize)
    };
}
unsafe extern "C" fn opt6_next(mut opts: *mut libc::c_void,
                               mut end: *mut libc::c_void)
 -> *mut libc::c_void {
    let mut opt_len: u16_0 = 0;
    if (end.wrapping_offset_from(opts) as libc::c_long) <
           4 as libc::c_int as libc::c_long {
        return 0 as *mut libc::c_void
    }
    opts = opts.offset(2 as libc::c_int as isize);
    let mut t_cp: *mut libc::c_uchar = opts as *mut libc::c_uchar;
    opt_len =
        ((*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int) <<
             8 as libc::c_int |
             *t_cp.offset(1 as libc::c_int as isize) as u16_0 as libc::c_int)
            as u16_0;
    opts = opts.offset(2 as libc::c_int as isize);
    if opt_len as libc::c_long >=
           end.wrapping_offset_from(opts) as libc::c_long {
        return 0 as *mut libc::c_void
    }
    return opts.offset(opt_len as libc::c_int as isize);
}
unsafe extern "C" fn opt6_uint(mut opt: *mut libc::c_uchar,
                               mut offset: libc::c_int, mut size: libc::c_int)
 -> libc::c_uint {
    /* this worries about unaligned data and byte order */
    let mut ret: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_uchar =
        &mut *opt.offset((4 as libc::c_int + offset) as isize) as
            *mut libc::c_uchar as *mut libc::c_void as *mut libc::c_uchar;
    i = 0 as libc::c_int;
    while i < size {
        let fresh8 = p;
        p = p.offset(1);
        ret = ret << 8 as libc::c_int | *fresh8 as libc::c_uint;
        i += 1
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn relay_upstream6(mut relay: *mut dhcp_relay,
                                         mut sz: ssize_t,
                                         mut peer_address: *mut in6_addr,
                                         mut scope_id: u32_0,
                                         mut now: time_t) {
    /* ->local is same value for all relays on ->current chain */
    let mut from: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut header: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut inbuff: *mut libc::c_uchar =
        (*dnsmasq_daemon).dhcp_packet.iov_base as *mut libc::c_uchar;
    let mut msg_type: libc::c_int = *inbuff as libc::c_int;
    let mut hopcount: libc::c_int = 0;
    let mut multicast: in6_addr =
        in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    let mut maclen: libc::c_uint = 0;
    let mut mactype: libc::c_uint = 0;
    let mut mac: [libc::c_uchar; 16] = [0; 16];
    inet_pton(10 as libc::c_int,
              b"FF05::1:3\x00" as *const u8 as *const libc::c_char,
              &mut multicast as *mut in6_addr as *mut libc::c_void);
    get_client_mac(peer_address, scope_id as libc::c_int, mac.as_mut_ptr(),
                   &mut maclen, &mut mactype, now);
    /* source address == relay address */
    from.addr6 = (*relay).local.addr6;
    /* Get hop count from nested relayed message */
    if msg_type == 12 as libc::c_int {
        hopcount =
            *inbuff.offset(1 as libc::c_int as isize) as libc::c_int +
                1 as libc::c_int
    } else { hopcount = 0 as libc::c_int }
    /* RFC 3315 HOP_COUNT_LIMIT */
    if hopcount > 32 as libc::c_int { return }
    reset_counter();
    header =
        put_opt6(0 as *mut libc::c_void, 34 as libc::c_int as size_t) as
            *mut libc::c_uchar;
    if !header.is_null() {
        let mut o: libc::c_int = 0;
        *header.offset(0 as libc::c_int as isize) =
            12 as libc::c_int as libc::c_uchar;
        *header.offset(1 as libc::c_int as isize) = hopcount as libc::c_uchar;
        memcpy(&mut *header.offset(2 as libc::c_int as isize) as
                   *mut libc::c_uchar as *mut libc::c_void,
               &mut (*relay).local.addr6 as *mut in6_addr as
                   *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
        memcpy(&mut *header.offset(18 as libc::c_int as isize) as
                   *mut libc::c_uchar as *mut libc::c_void,
               peer_address as *const libc::c_void,
               16 as libc::c_int as libc::c_ulong);
        /* RFC-6939 */
        if maclen != 0 as libc::c_int as libc::c_uint {
            o = new_opt6(79 as libc::c_int);
            put_opt6_short(mactype);
            put_opt6(mac.as_mut_ptr() as *mut libc::c_void, maclen as size_t);
            end_opt6(o);
        }
        o = new_opt6(9 as libc::c_int);
        put_opt6(inbuff as *mut libc::c_void, sz as size_t);
        end_opt6(o);
        while !relay.is_null() {
            let mut to: mysockaddr =
                mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
            to.sa.sa_family = 10 as libc::c_int as sa_family_t;
            to.in6.sin6_addr = (*relay).server.addr6;
            to.in6.sin6_port = __bswap_16(547 as libc::c_int as __uint16_t);
            to.in6.sin6_flowinfo = 0 as libc::c_int as uint32_t;
            to.in6.sin6_scope_id = 0 as libc::c_int as uint32_t;
            if ({
                    let mut __a: *const in6_addr =
                        &mut (*relay).server.addr6 as *mut in6_addr as
                            *const in6_addr;
                    let mut __b: *const in6_addr =
                        &mut multicast as *mut in6_addr as *const in6_addr;
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
                let mut multicast_iface: libc::c_int = 0;
                if (*relay).interface.is_null() ||
                       !strchr((*relay).interface, '*' as i32).is_null() ||
                       {
                           multicast_iface =
                               if_nametoindex((*relay).interface) as
                                   libc::c_int;
                           (multicast_iface) == 0 as libc::c_int
                       } ||
                       setsockopt((*dnsmasq_daemon).dhcp6fd,
                                  IPPROTO_IPV6 as libc::c_int,
                                  17 as libc::c_int,
                                  &mut multicast_iface as *mut libc::c_int as
                                      *const libc::c_void,
                                  ::std::mem::size_of::<libc::c_int>() as
                                      libc::c_ulong as socklen_t) ==
                           -(1 as libc::c_int) {
                    my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                  3 as libc::c_int,
                              b"Cannot multicast to DHCPv6 server without correct interface\x00"
                                  as *const u8 as *const libc::c_char);
                }
            }
            send_from((*dnsmasq_daemon).dhcp6fd, 0 as libc::c_int,
                      (*dnsmasq_daemon).outpacket.iov_base as
                          *mut libc::c_char,
                      save_counter(-(1 as libc::c_int)) as size_t, &mut to,
                      &mut from, 0 as libc::c_int as libc::c_uint);
            if (*dnsmasq_daemon).options[(28 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (28 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
                inet_ntop(10 as libc::c_int,
                          &mut (*relay).local as *mut all_addr as
                              *const libc::c_void, (*dnsmasq_daemon).addrbuff,
                          46 as libc::c_int as socklen_t);
                inet_ntop(10 as libc::c_int,
                          &mut (*relay).server as *mut all_addr as
                              *const libc::c_void, (*dnsmasq_daemon).namebuff,
                          46 as libc::c_int as socklen_t);
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              6 as libc::c_int,
                          b"DHCP relay %s -> %s\x00" as *const u8 as
                              *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                          (*dnsmasq_daemon).namebuff);
            }
            /* Save this for replies */
            (*relay).iface_index = scope_id as libc::c_int;
            relay = (*relay).current
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn relay_reply6(mut peer: *mut sockaddr_in6,
                                      mut sz: ssize_t,
                                      mut arrival_interface:
                                          *mut libc::c_char)
 -> libc::c_ushort {
    let mut relay: *mut dhcp_relay = 0 as *mut dhcp_relay;
    let mut link: in6_addr =
        in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    let mut inbuff: *mut libc::c_uchar =
        (*dnsmasq_daemon).dhcp_packet.iov_base as *mut libc::c_uchar;
    /* must have at least msg_type+hopcount+link_address+peer_address+minimal size option
     which is               1   +    1   +    16      +     16     + 2 + 2 = 38 */
    if sz < 38 as libc::c_int as libc::c_long ||
           *inbuff as libc::c_int != 13 as libc::c_int {
        return 0 as libc::c_int as libc::c_ushort
    }
    memcpy(&mut link as *mut in6_addr as *mut libc::c_void,
           &mut *inbuff.offset(2 as libc::c_int as isize) as
               *mut libc::c_uchar as *const libc::c_void,
           16 as libc::c_int as libc::c_ulong);
    relay = (*dnsmasq_daemon).relay6;
    while !relay.is_null() {
        if ({
                let mut __a: *const in6_addr =
                    &mut link as *mut in6_addr as *const in6_addr;
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
            }) != 0 &&
               ((*relay).interface.is_null() ||
                    wildcard_match((*relay).interface, arrival_interface) !=
                        0) {
            break ;
        }
        relay = (*relay).next
    }
    reset_counter();
    if !relay.is_null() {
        let mut opt: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut opts: *mut libc::c_void =
            inbuff.offset(34 as libc::c_int as isize) as *mut libc::c_void;
        let mut end: *mut libc::c_void =
            inbuff.offset(sz as isize) as *mut libc::c_void;
        opt = opts;
        while !opt.is_null() {
            if opt6_uint(opt as *mut libc::c_uchar, -(4 as libc::c_int),
                         2 as libc::c_int) == 9 as libc::c_int as libc::c_uint
                   &&
                   opt6_uint(opt as *mut libc::c_uchar, -(2 as libc::c_int),
                             2 as libc::c_int) as libc::c_int >
                       0 as libc::c_int {
                let mut encap_type: libc::c_int =
                    *(&mut *(opt as
                                 *mut libc::c_uchar).offset((4 as libc::c_int
                                                                 +
                                                                 0 as
                                                                     libc::c_int)
                                                                as isize) as
                          *mut libc::c_uchar as *mut libc::c_void as
                          *mut libc::c_uchar) as libc::c_int;
                put_opt6(&mut *(opt as
                                    *mut libc::c_uchar).offset((4 as
                                                                    libc::c_int
                                                                    +
                                                                    0 as
                                                                        libc::c_int)
                                                                   as isize)
                             as *mut libc::c_uchar as *mut libc::c_void,
                         opt6_uint(opt as *mut libc::c_uchar,
                                   -(2 as libc::c_int), 2 as libc::c_int) as
                             libc::c_int as size_t);
                memcpy(&mut (*peer).sin6_addr as *mut in6_addr as
                           *mut libc::c_void,
                       &mut *inbuff.offset(18 as libc::c_int as isize) as
                           *mut libc::c_uchar as *const libc::c_void,
                       16 as libc::c_int as libc::c_ulong);
                (*peer).sin6_scope_id = (*relay).iface_index as uint32_t;
                return if encap_type == 13 as libc::c_int {
                           547 as libc::c_int
                       } else { 546 as libc::c_int } as libc::c_ushort
            }
            opt = opt6_next(opt, end)
        }
    }
    return 0 as libc::c_int as libc::c_ushort;
}
