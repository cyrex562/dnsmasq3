#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types,
           register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ftruncate(__fd: libc::c_int, __length: __off64_t) -> libc::c_int;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn rewind(__stream: *mut FILE);
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char)
     -> *mut FILE;
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
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char,
               _: libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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
    static mut dnsmasq_daemon: *mut dnsmasq_daemon;
    #[no_mangle]
    fn cache_add_dhcp_entry(host_name: *mut libc::c_char, prot: libc::c_int,
                            host_address: *mut all_addr, ttd: time_t);
    #[no_mangle]
    fn cache_unhash_dhcp();
    #[no_mangle]
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn get_domain(addr: in_addr) -> *mut libc::c_char;
    #[no_mangle]
    fn get_domain6(addr: *mut in6_addr) -> *mut libc::c_char;
    #[no_mangle]
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn whine_malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn hostname_isequal(a: *const libc::c_char, b: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn netmask_length(mask: in_addr) -> libc::c_int;
    #[no_mangle]
    fn is_same_net(a: in_addr, b: in_addr, mask: in_addr) -> libc::c_int;
    #[no_mangle]
    fn is_same_net6(a: *mut in6_addr, b: *mut in6_addr,
                    prefixlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn addr6part(addr: *mut in6_addr) -> u64_0;
    #[no_mangle]
    fn parse_hex(in_0: *mut libc::c_char, out: *mut libc::c_uchar,
                 maxlen: libc::c_int, wildcard_mask: *mut libc::c_uint,
                 mac_type: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn die(message: *mut libc::c_char, arg1: *mut libc::c_char,
           exit_code: libc::c_int) -> !;
    #[no_mangle]
    fn my_syslog(priority: libc::c_int, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn host_from_dns(addr: in_addr) -> *mut libc::c_char;
    #[no_mangle]
    fn send_alarm(event: time_t, now: time_t);
    #[no_mangle]
    fn periodic_ra(now: time_t) -> time_t;
    #[no_mangle]
    fn periodic_slaac(now: time_t, leases_0: *mut dhcp_lease) -> time_t;
    #[no_mangle]
    fn slaac_add_addrs(lease: *mut dhcp_lease, now: time_t,
                       force: libc::c_int);
    #[no_mangle]
    fn slaac_ping_reply(sender: *mut in6_addr, packet: *mut libc::c_uchar,
                        interface: *mut libc::c_char,
                        leases_0: *mut dhcp_lease);
    #[no_mangle]
    fn make_duid(now: time_t);
    #[no_mangle]
    fn find_config(configs: *mut dhcp_config, context: *mut dhcp_context,
                   clid: *mut libc::c_uchar, clid_len: libc::c_int,
                   hwaddr: *mut libc::c_uchar, hw_len: libc::c_int,
                   hw_type: libc::c_int, hostname: *mut libc::c_char,
                   filter: *mut dhcp_netid) -> *mut dhcp_config;
    #[no_mangle]
    fn queue_script(action: libc::c_int, lease: *mut dhcp_lease,
                    hostname: *mut libc::c_char, now: time_t);
    #[no_mangle]
    fn iface_enumerate(family: libc::c_int, parm: *mut libc::c_void,
                       callback:
                           Option<unsafe extern "C" fn() -> libc::c_int>)
     -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
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
pub const __METRIC_MAX: C2RustUnnamed_0 = 20;
pub const METRIC_LEASES_PRUNED_6: C2RustUnnamed_0 = 19;
pub const METRIC_LEASES_ALLOCATED_6: C2RustUnnamed_0 = 18;
pub const METRIC_LEASES_PRUNED_4: C2RustUnnamed_0 = 17;
pub const METRIC_LEASES_ALLOCATED_4: C2RustUnnamed_0 = 16;
pub const METRIC_NOANSWER: C2RustUnnamed_0 = 15;
pub const METRIC_DHCPREQUEST: C2RustUnnamed_0 = 14;
pub const METRIC_DHCPRELEASE: C2RustUnnamed_0 = 13;
pub const METRIC_DHCPOFFER: C2RustUnnamed_0 = 12;
pub const METRIC_DHCPNAK: C2RustUnnamed_0 = 11;
pub const METRIC_DHCPINFORM: C2RustUnnamed_0 = 10;
pub const METRIC_DHCPDISCOVER: C2RustUnnamed_0 = 9;
pub const METRIC_DHCPDECLINE: C2RustUnnamed_0 = 8;
pub const METRIC_DHCPACK: C2RustUnnamed_0 = 7;
pub const METRIC_PXE: C2RustUnnamed_0 = 6;
pub const METRIC_BOOTP: C2RustUnnamed_0 = 5;
pub const METRIC_DNS_LOCAL_ANSWERED: C2RustUnnamed_0 = 4;
pub const METRIC_DNS_AUTH_ANSWERED: C2RustUnnamed_0 = 3;
pub const METRIC_DNS_QUERIES_FORWARDED: C2RustUnnamed_0 = 2;
pub const METRIC_DNS_CACHE_LIVE_FREED: C2RustUnnamed_0 = 1;
pub const METRIC_DNS_CACHE_INSERTED: C2RustUnnamed_0 = 0;
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
pub type va_list = __builtin_va_list;
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
static mut leases: *mut dhcp_lease =
    0 as *const dhcp_lease as *mut dhcp_lease;
static mut old_leases: *mut dhcp_lease =
    0 as *const dhcp_lease as *mut dhcp_lease;
static mut dns_dirty: libc::c_int = 0;
static mut file_dirty: libc::c_int = 0;
static mut leases_left: libc::c_int = 0;
unsafe extern "C" fn read_leases(mut now: time_t, mut leasestream: *mut FILE)
 -> libc::c_int {
    let mut ei: libc::c_ulong = 0;
    let mut addr: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut clid_len: libc::c_int = 0;
    let mut hw_len: libc::c_int = 0;
    let mut hw_type: libc::c_int = 0;
    let mut items: libc::c_int = 0;
    let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
    *(*dnsmasq_daemon).dhcp_buff2 = '\u{0}' as i32 as libc::c_char;
    *(*dnsmasq_daemon).dhcp_buff3 = *(*dnsmasq_daemon).dhcp_buff2;
    loop 
         /* client-id max length is 255 which is 255*2 digits + 254 colons
     borrow DNS packet buffer which is always larger than 1000 bytes

     Check various buffers are big enough for the code below */
         {
        items =
            fscanf(leasestream,
                   b"%255s %255s\x00" as *const u8 as *const libc::c_char,
                   (*dnsmasq_daemon).dhcp_buff3,
                   (*dnsmasq_daemon).dhcp_buff2);
        if !(items == 2 as libc::c_int) { break ; }
        *(*dnsmasq_daemon).packet = '\u{0}' as i32 as libc::c_char;
        *(*dnsmasq_daemon).dhcp_buff = *(*dnsmasq_daemon).packet;
        *(*dnsmasq_daemon).namebuff = *(*dnsmasq_daemon).dhcp_buff;
        clid_len = 0 as libc::c_int;
        hw_type = clid_len;
        hw_len = hw_type;
        if strcmp((*dnsmasq_daemon).dhcp_buff3,
                  b"duid\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            (*dnsmasq_daemon).duid_len =
                parse_hex((*dnsmasq_daemon).dhcp_buff2,
                          (*dnsmasq_daemon).dhcp_buff2 as *mut libc::c_uchar,
                          130 as libc::c_int, 0 as *mut libc::c_uint,
                          0 as *mut libc::c_int);
            if (*dnsmasq_daemon).duid_len < 0 as libc::c_int {
                return 0 as libc::c_int
            }
            (*dnsmasq_daemon).duid =
                safe_malloc((*dnsmasq_daemon).duid_len as size_t) as
                    *mut libc::c_uchar;
            memcpy((*dnsmasq_daemon).duid as *mut libc::c_void,
                   (*dnsmasq_daemon).dhcp_buff2 as *const libc::c_void,
                   (*dnsmasq_daemon).duid_len as libc::c_ulong);
        } else if fscanf(leasestream,
                         b" %64s %255s %764s\x00" as *const u8 as
                             *const libc::c_char, (*dnsmasq_daemon).namebuff,
                         (*dnsmasq_daemon).dhcp_buff,
                         (*dnsmasq_daemon).packet) != 3 as libc::c_int {
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          4 as libc::c_int,
                      b"ignoring invalid line in lease database: %s %s %s %s ...\x00"
                          as *const u8 as *const libc::c_char,
                      (*dnsmasq_daemon).dhcp_buff3,
                      (*dnsmasq_daemon).dhcp_buff2,
                      (*dnsmasq_daemon).namebuff,
                      (*dnsmasq_daemon).dhcp_buff);
        } else {
            if inet_pton(2 as libc::c_int, (*dnsmasq_daemon).namebuff,
                         &mut addr.addr4 as *mut in_addr as *mut libc::c_void)
                   != 0 {
                lease = lease4_allocate(addr.addr4);
                if !lease.is_null() { domain = get_domain((*lease).addr) }
                hw_len =
                    parse_hex((*dnsmasq_daemon).dhcp_buff2,
                              (*dnsmasq_daemon).dhcp_buff2 as
                                  *mut libc::c_uchar, 16 as libc::c_int,
                              0 as *mut libc::c_uint, &mut hw_type);
                /* For backwards compatibility, no explicit MAC address type means ether. */
                if hw_type == 0 as libc::c_int && hw_len != 0 as libc::c_int {
                    hw_type = 1 as libc::c_int
                }
            } else if inet_pton(10 as libc::c_int, (*dnsmasq_daemon).namebuff,
                                &mut addr.addr6 as *mut in6_addr as
                                    *mut libc::c_void) != 0 {
                let mut s: *mut libc::c_char = (*dnsmasq_daemon).dhcp_buff2;
                let mut lease_type: libc::c_int = 32 as libc::c_int;
                if *s.offset(0 as libc::c_int as isize) as libc::c_int ==
                       'T' as i32 {
                    lease_type = 64 as libc::c_int;
                    s = s.offset(1)
                }
                lease = lease6_allocate(&mut addr.addr6, lease_type);
                if !lease.is_null() {
                    lease_set_iaid(lease,
                                   strtoul(s, 0 as *mut *mut libc::c_char,
                                           10 as libc::c_int) as
                                       libc::c_uint);
                    domain = get_domain6(&mut (*lease).addr6)
                }
            } else {
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              4 as libc::c_int,
                          b"ignoring invalid line in lease database, bad address: %s\x00"
                              as *const u8 as *const libc::c_char,
                          (*dnsmasq_daemon).namebuff);
                continue ;
            }
            if lease.is_null() {
                die(b"too many stored leases\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_char, 5 as libc::c_int);
            }
            if strcmp((*dnsmasq_daemon).packet,
                      b"*\x00" as *const u8 as *const libc::c_char) !=
                   0 as libc::c_int {
                clid_len =
                    parse_hex((*dnsmasq_daemon).packet,
                              (*dnsmasq_daemon).packet as *mut libc::c_uchar,
                              255 as libc::c_int, 0 as *mut libc::c_uint,
                              0 as *mut libc::c_int)
            }
            lease_set_hwaddr(lease,
                             (*dnsmasq_daemon).dhcp_buff2 as
                                 *mut libc::c_uchar,
                             (*dnsmasq_daemon).packet as *mut libc::c_uchar,
                             hw_len, hw_type, clid_len, now,
                             0 as libc::c_int);
            if strcmp((*dnsmasq_daemon).dhcp_buff,
                      b"*\x00" as *const u8 as *const libc::c_char) !=
                   0 as libc::c_int {
                lease_set_hostname(lease, (*dnsmasq_daemon).dhcp_buff,
                                   0 as libc::c_int, domain,
                                   0 as *mut libc::c_char);
            }
            ei = atol((*dnsmasq_daemon).dhcp_buff3) as libc::c_ulong;
            /* strictly time_t is opaque, but this hack should work on all sane systems,
	   even when sizeof(time_t) == 8 */
            (*lease).expires = ei as time_t;
            /* set these correctly: the "old" events are generated later from
	   the startup synthesised SIGHUP. */
            (*lease).flags &= !(1 as libc::c_int | 2 as libc::c_int);
            *(*dnsmasq_daemon).dhcp_buff2 = '\u{0}' as i32 as libc::c_char;
            *(*dnsmasq_daemon).dhcp_buff3 = *(*dnsmasq_daemon).dhcp_buff2
        }
    }
    return (items == 0 as libc::c_int || items == -(1 as libc::c_int)) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lease_init(mut now: time_t) {
    let mut leasestream: *mut FILE = 0 as *mut FILE;
    leases_left = (*dnsmasq_daemon).dhcp_max;
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
           != 0 {
        /* run "<lease_change_script> init" once to get the
	 initial state of the database. If leasefile-ro is
	 set without a script, we just do without any
	 lease database. */
        if !(*dnsmasq_daemon).lease_change_command.is_null() {
            strcpy((*dnsmasq_daemon).dhcp_buff,
                   (*dnsmasq_daemon).lease_change_command);
            strcat((*dnsmasq_daemon).dhcp_buff,
                   b" init\x00" as *const u8 as *const libc::c_char);
            leasestream =
                popen((*dnsmasq_daemon).dhcp_buff,
                      b"r\x00" as *const u8 as *const libc::c_char)
        } else {
            dns_dirty = 0 as libc::c_int;
            file_dirty = dns_dirty;
            return
        }
    } else {
        /* NOTE: need a+ mode to create file if it doesn't exist */
        (*dnsmasq_daemon).lease_stream =
            fopen((*dnsmasq_daemon).lease_file,
                  b"a+\x00" as *const u8 as *const libc::c_char);
        leasestream = (*dnsmasq_daemon).lease_stream;
        if leasestream.is_null() {
            die(b"cannot open or create lease file %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).lease_file, 3 as libc::c_int);
        }
        /* a+ mode leaves pointer at end. */
        rewind(leasestream);
    }
    if !leasestream.is_null() {
        if read_leases(now, leasestream) == 0 {
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          3 as libc::c_int,
                      b"failed to parse lease database cleanly\x00" as
                          *const u8 as *const libc::c_char);
        }
        if ferror(leasestream) != 0 {
            die(b"failed to read lease file %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).lease_file, 3 as libc::c_int);
        }
    }
    if (*dnsmasq_daemon).lease_stream.is_null() {
        let mut rc: libc::c_int = 0 as libc::c_int;
        /* shell returns 127 for "command not found", 126 for bad permissions. */
        if leasestream.is_null() ||
               { rc = pclose(leasestream); (rc) == -(1 as libc::c_int) } ||
               (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                   127 as libc::c_int ||
               (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                   126 as libc::c_int {
            if (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                   127 as libc::c_int {
                *__errno_location() = 2 as libc::c_int
            } else if (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                          126 as libc::c_int {
                *__errno_location() = 13 as libc::c_int
            }
            die(b"cannot run lease-init script %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).lease_change_command, 3 as libc::c_int);
        }
        if (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int !=
               0 as libc::c_int {
            sprintf((*dnsmasq_daemon).dhcp_buff,
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    (rc & 0xff00 as libc::c_int) >> 8 as libc::c_int);
            die(b"lease-init script returned exit code %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).dhcp_buff,
                ((rc & 0xff00 as libc::c_int) >> 8 as libc::c_int) +
                    10 as libc::c_int);
        }
    }
    /* Some leases may have expired */
    file_dirty = 0 as libc::c_int;
    lease_prune(0 as *mut dhcp_lease, now);
    dns_dirty = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lease_update_from_configs() {
    /* changes to the config may change current leases. */
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut config: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) != 0) {
            config =
                find_config((*dnsmasq_daemon).dhcp_conf,
                            0 as *mut dhcp_context, (*lease).clid,
                            (*lease).clid_len, (*lease).hwaddr.as_mut_ptr(),
                            (*lease).hwaddr_len, (*lease).hwaddr_type,
                            0 as *mut libc::c_char, 0 as *mut dhcp_netid);
            if !config.is_null() &&
                   (*config).flags & 16 as libc::c_int as libc::c_uint != 0 &&
                   ((*config).flags & 32 as libc::c_int as libc::c_uint == 0
                        || (*config).addr.s_addr == (*lease).addr.s_addr) {
                lease_set_hostname(lease, (*config).hostname,
                                   1 as libc::c_int,
                                   get_domain((*lease).addr),
                                   0 as *mut libc::c_char);
            } else {
                name = host_from_dns((*lease).addr);
                if !name.is_null() {
                    lease_set_hostname(lease, name, 1 as libc::c_int,
                                       get_domain((*lease).addr),
                                       0 as *mut libc::c_char);
                }
            }
        }
        lease = (*lease).next
    };
    /* updates auth flag only */
}
unsafe extern "C" fn ourprintf(mut errp: *mut libc::c_int,
                               mut format: *mut libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if *errp == 0 &&
           vfprintf((*dnsmasq_daemon).lease_stream, format, ap.as_va_list()) <
               0 as libc::c_int {
        *errp = *__errno_location()
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_update_file(mut now: time_t) {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut next_event: time_t = 0;
    let mut i: libc::c_int = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    if file_dirty != 0 as libc::c_int &&
           !(*dnsmasq_daemon).lease_stream.is_null() {
        *__errno_location() = 0 as libc::c_int;
        rewind((*dnsmasq_daemon).lease_stream);
        if *__errno_location() != 0 as libc::c_int ||
               ftruncate(fileno((*dnsmasq_daemon).lease_stream),
                         0 as libc::c_int as __off64_t) != 0 as libc::c_int {
            err = *__errno_location()
        }
        lease = leases;
        while !lease.is_null() {
            if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) !=
                     0) {
                ourprintf(&mut err as *mut libc::c_int,
                          b"%lu \x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          (*lease).expires as libc::c_ulong);
                if (*lease).hwaddr_type != 1 as libc::c_int ||
                       (*lease).hwaddr_len == 0 as libc::c_int {
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%.2x-\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char, (*lease).hwaddr_type);
                }
                i = 0 as libc::c_int;
                while i < (*lease).hwaddr_len {
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%.2x\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char,
                              (*lease).hwaddr[i as usize] as libc::c_int);
                    if i != (*lease).hwaddr_len - 1 as libc::c_int {
                        ourprintf(&mut err as *mut libc::c_int,
                                  b":\x00" as *const u8 as *const libc::c_char
                                      as *mut libc::c_char);
                    }
                    i += 1
                }
                inet_ntop(2 as libc::c_int,
                          &mut (*lease).addr as *mut in_addr as
                              *const libc::c_void, (*dnsmasq_daemon).addrbuff,
                          46 as libc::c_int as socklen_t);
                ourprintf(&mut err as *mut libc::c_int,
                          b" %s \x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, (*dnsmasq_daemon).addrbuff);
                ourprintf(&mut err as *mut libc::c_int,
                          b"%s \x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          if !(*lease).hostname.is_null() {
                              (*lease).hostname as *const libc::c_char
                          } else {
                              b"*\x00" as *const u8 as *const libc::c_char
                          });
                if !(*lease).clid.is_null() &&
                       (*lease).clid_len != 0 as libc::c_int {
                    i = 0 as libc::c_int;
                    while i < (*lease).clid_len - 1 as libc::c_int {
                        ourprintf(&mut err as *mut libc::c_int,
                                  b"%.2x:\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                                  *(*lease).clid.offset(i as isize) as
                                      libc::c_int);
                        i += 1
                    }
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%.2x\n\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              *(*lease).clid.offset(i as isize) as
                                  libc::c_int);
                } else {
                    ourprintf(&mut err as *mut libc::c_int,
                              b"*\n\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char);
                }
            }
            lease = (*lease).next
        }
        if !(*dnsmasq_daemon).duid.is_null() {
            ourprintf(&mut err as *mut libc::c_int,
                      b"duid \x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
            i = 0 as libc::c_int;
            while i < (*dnsmasq_daemon).duid_len - 1 as libc::c_int {
                ourprintf(&mut err as *mut libc::c_int,
                          b"%.2x:\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          *(*dnsmasq_daemon).duid.offset(i as isize) as
                              libc::c_int);
                i += 1
            }
            ourprintf(&mut err as *mut libc::c_int,
                      b"%.2x\n\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      *(*dnsmasq_daemon).duid.offset(i as isize) as
                          libc::c_int);
            lease = leases;
            while !lease.is_null() {
                if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int)
                         == 0) {
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%lu \x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char,
                              (*lease).expires as libc::c_ulong);
                    inet_ntop(10 as libc::c_int,
                              &mut (*lease).addr6 as *mut in6_addr as
                                  *const libc::c_void,
                              (*dnsmasq_daemon).addrbuff,
                              46 as libc::c_int as socklen_t);
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%s%u %s \x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              if (*lease).flags & 64 as libc::c_int != 0 {
                                  b"T\x00" as *const u8 as *const libc::c_char
                              } else {
                                  b"\x00" as *const u8 as *const libc::c_char
                              }, (*lease).iaid, (*dnsmasq_daemon).addrbuff);
                    ourprintf(&mut err as *mut libc::c_int,
                              b"%s \x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char,
                              if !(*lease).hostname.is_null() {
                                  (*lease).hostname as *const libc::c_char
                              } else {
                                  b"*\x00" as *const u8 as *const libc::c_char
                              });
                    if !(*lease).clid.is_null() &&
                           (*lease).clid_len != 0 as libc::c_int {
                        i = 0 as libc::c_int;
                        while i < (*lease).clid_len - 1 as libc::c_int {
                            ourprintf(&mut err as *mut libc::c_int,
                                      b"%.2x:\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char,
                                      *(*lease).clid.offset(i as isize) as
                                          libc::c_int);
                            i += 1
                        }
                        ourprintf(&mut err as *mut libc::c_int,
                                  b"%.2x\n\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                                  *(*lease).clid.offset(i as isize) as
                                      libc::c_int);
                    } else {
                        ourprintf(&mut err as *mut libc::c_int,
                                  b"*\n\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char);
                    }
                }
                lease = (*lease).next
            }
        }
        if fflush((*dnsmasq_daemon).lease_stream) != 0 as libc::c_int ||
               fsync(fileno((*dnsmasq_daemon).lease_stream)) <
                   0 as libc::c_int {
            err = *__errno_location()
        }
        if err == 0 { file_dirty = 0 as libc::c_int }
    }
    /* Set alarm for when the first lease expires. */
    next_event = 0 as libc::c_int as time_t;
    /* do timed RAs and determine when the next is, also pings to potential SLAAC addresses */
    if (*dnsmasq_daemon).doing_ra != 0 {
        let mut event: time_t = 0;
        event = periodic_slaac(now, leases);
        if event != 0 as libc::c_int as libc::c_long {
            if next_event == 0 as libc::c_int as libc::c_long ||
                   difftime(next_event, event) > 0.0f64 {
                next_event = event
            }
        }
        event = periodic_ra(now);
        if event != 0 as libc::c_int as libc::c_long {
            if next_event == 0 as libc::c_int as libc::c_long ||
                   difftime(next_event, event) > 0.0f64 {
                next_event = event
            }
        }
    }
    lease = leases;
    while !lease.is_null() {
        if (*lease).expires != 0 as libc::c_int as libc::c_long &&
               (next_event == 0 as libc::c_int as libc::c_long ||
                    difftime(next_event, (*lease).expires) > 0.0f64) {
            next_event = (*lease).expires
        }
        lease = (*lease).next
    }
    if err != 0 {
        if next_event == 0 as libc::c_int as libc::c_long ||
               difftime(next_event, 60 as libc::c_int as libc::c_long + now) >
                   0.0f64 {
            next_event = 60 as libc::c_int as libc::c_long + now
        }
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 3 as libc::c_int,
                  b"failed to write %s: %s (retry in %us)\x00" as *const u8 as
                      *const libc::c_char, (*dnsmasq_daemon).lease_file,
                  strerror(err), difftime(next_event, now) as libc::c_uint);
    }
    send_alarm(next_event, now);
}
unsafe extern "C" fn find_interface_v4(mut local: in_addr,
                                       mut if_index: libc::c_int,
                                       mut label: *mut libc::c_char,
                                       mut netmask: in_addr,
                                       mut broadcast: in_addr,
                                       mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut prefix: libc::c_int = netmask_length(netmask);
    lease = leases;
    while !lease.is_null() {
        if (*lease).flags & (64 as libc::c_int | 32 as libc::c_int) == 0 &&
               is_same_net(local, (*lease).addr, netmask) != 0 &&
               prefix > (*lease).new_prefixlen {
            (*lease).new_interface = if_index;
            (*lease).new_prefixlen = prefix
        }
        lease = (*lease).next
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn find_interface_v6(mut local: *mut in6_addr,
                                       mut prefix: libc::c_int,
                                       mut scope: libc::c_int,
                                       mut if_index: libc::c_int,
                                       mut flags: libc::c_int,
                                       mut preferred: libc::c_int,
                                       mut valid: libc::c_int,
                                       mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        if (*lease).flags & (64 as libc::c_int | 32 as libc::c_int) != 0 {
            if is_same_net6(local, &mut (*lease).addr6, prefix) != 0 &&
                   prefix > (*lease).new_prefixlen {
                /* save prefix length for comparison, as we might get shorter matching
         * prefix in upcoming netlink GETADDR responses
         * */
                (*lease).new_interface = if_index;
                (*lease).new_prefixlen = prefix
            }
        }
        lease = (*lease).next
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lease_ping_reply(mut sender: *mut in6_addr,
                                          mut packet: *mut libc::c_uchar,
                                          mut interface: *mut libc::c_char) {
    /* We may be doing RA but not DHCPv4, in which case the lease
     database may not exist and we have nothing to do anyway */
    if !(*dnsmasq_daemon).dhcp.is_null() {
        slaac_ping_reply(sender, packet, interface, leases);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_update_slaac(mut now: time_t) {
    /* Called when we construct a new RA-names context, to add putative
     new SLAAC addresses to existing leases. */
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    if !(*dnsmasq_daemon).dhcp.is_null() {
        lease = leases;
        while !lease.is_null() {
            slaac_add_addrs(lease, now, 0 as libc::c_int);
            lease = (*lease).next
        }
    };
}
/* Find interfaces associated with leases at start-up. This gets updated as
   we do DHCP transactions, but information about directly-connected subnets
   is useful from scrips and necessary for determining SLAAC addresses from
   start-time. */
#[no_mangle]
pub unsafe extern "C" fn lease_find_interfaces(mut now: time_t) {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        (*lease).new_interface = 0 as libc::c_int;
        (*lease).new_prefixlen = (*lease).new_interface;
        lease = (*lease).next
    }
    iface_enumerate(2 as libc::c_int,
                    &mut now as *mut time_t as *mut libc::c_void,
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
                                                       -> libc::c_int>,
                                            Option<unsafe extern "C" fn()
                                                       ->
                                                           libc::c_int>>(Some(find_interface_v4
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
                                                                                          libc::c_int)));
    iface_enumerate(10 as libc::c_int,
                    &mut now as *mut time_t as *mut libc::c_void,
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
                                                           libc::c_int>>(Some(find_interface_v6
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
    lease = leases;
    while !lease.is_null() {
        if (*lease).new_interface != 0 as libc::c_int {
            lease_set_interface(lease, (*lease).new_interface, now);
        }
        lease = (*lease).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_make_duid(mut now: time_t) {
    /* If we're not doing DHCPv6, and there are not v6 leases, don't add the DUID to the database */
    if (*dnsmasq_daemon).duid.is_null() && (*dnsmasq_daemon).doing_dhcp6 != 0
       {
        file_dirty = 1 as libc::c_int;
        make_duid(now);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_update_dns(mut force: libc::c_int) {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    if (*dnsmasq_daemon).port != 0 as libc::c_int &&
           (dns_dirty != 0 || force != 0) {
        /* force transfer to authoritative secondaries */
        (*dnsmasq_daemon).soa_sn =
            (*dnsmasq_daemon).soa_sn.wrapping_add(1); /* unlink */
        cache_unhash_dhcp();
        lease = leases;
        while !lease.is_null() {
            let mut prot: libc::c_int = 2 as libc::c_int;
            if (*lease).flags & (64 as libc::c_int | 32 as libc::c_int) != 0 {
                prot = 10 as libc::c_int
            } else if !(*lease).hostname.is_null() || !(*lease).fqdn.is_null()
             {
                let mut slaac: *mut slaac_address = 0 as *mut slaac_address;
                slaac = (*lease).slaac_address;
                while !slaac.is_null() {
                    if (*slaac).backoff == 0 as libc::c_int {
                        if !(*lease).fqdn.is_null() {
                            cache_add_dhcp_entry((*lease).fqdn,
                                                 10 as libc::c_int,
                                                 &mut (*slaac).addr as
                                                     *mut in6_addr as
                                                     *mut all_addr,
                                                 (*lease).expires);
                        }
                        if (*dnsmasq_daemon).options[(20 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (20 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               == 0 && !(*lease).hostname.is_null() {
                            cache_add_dhcp_entry((*lease).hostname,
                                                 10 as libc::c_int,
                                                 &mut (*slaac).addr as
                                                     *mut in6_addr as
                                                     *mut all_addr,
                                                 (*lease).expires);
                        }
                    }
                    slaac = (*slaac).next
                }
            }
            if !(*lease).fqdn.is_null() {
                cache_add_dhcp_entry((*lease).fqdn, prot,
                                     if prot == 2 as libc::c_int {
                                         &mut (*lease).addr as *mut in_addr as
                                             *mut all_addr
                                     } else {
                                         &mut (*lease).addr6 as *mut in6_addr
                                             as *mut all_addr
                                     }, (*lease).expires);
            }
            if (*dnsmasq_daemon).options[(20 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (20 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   == 0 && !(*lease).hostname.is_null() {
                cache_add_dhcp_entry((*lease).hostname, prot,
                                     if prot == 2 as libc::c_int {
                                         &mut (*lease).addr as *mut in_addr as
                                             *mut all_addr
                                     } else {
                                         &mut (*lease).addr6 as *mut in6_addr
                                             as *mut all_addr
                                     }, (*lease).expires);
            }
            lease = (*lease).next
        }
        dns_dirty = 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_prune(mut target: *mut dhcp_lease,
                                     mut now: time_t) {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut tmp: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut up: *mut *mut dhcp_lease = 0 as *mut *mut dhcp_lease;
    lease = leases;
    up = &mut leases;
    while !lease.is_null() {
        tmp = (*lease).next;
        if (*lease).expires != 0 as libc::c_int as libc::c_long &&
               difftime(now, (*lease).expires) >=
                   0 as libc::c_int as libc::c_double || lease == target {
            file_dirty = 1 as libc::c_int;
            if !(*lease).hostname.is_null() { dns_dirty = 1 as libc::c_int }
            (*dnsmasq_daemon).metrics[if (*lease).addr.s_addr != 0 {
                                          METRIC_LEASES_PRUNED_4 as
                                              libc::c_int
                                      } else {
                                          METRIC_LEASES_PRUNED_6 as
                                              libc::c_int
                                      } as usize] =
                (*dnsmasq_daemon).metrics[if (*lease).addr.s_addr != 0 {
                                              METRIC_LEASES_PRUNED_4 as
                                                  libc::c_int
                                          } else {
                                              METRIC_LEASES_PRUNED_6 as
                                                  libc::c_int
                                          } as usize].wrapping_add(1);
            *up = (*lease).next;
            /* Put on old_leases list 'till we
	     can run the script */
            (*lease).next = old_leases;
            old_leases = lease;
            leases_left += 1
        } else { up = &mut (*lease).next }
        lease = tmp
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_find_by_client(mut hwaddr: *mut libc::c_uchar,
                                              mut hw_len: libc::c_int,
                                              mut hw_type: libc::c_int,
                                              mut clid: *mut libc::c_uchar,
                                              mut clid_len: libc::c_int)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    if !clid.is_null() {
        lease = leases;
        while !lease.is_null() {
            if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) !=
                     0) {
                if !(*lease).clid.is_null() && clid_len == (*lease).clid_len
                       &&
                       memcmp(clid as *const libc::c_void,
                              (*lease).clid as *const libc::c_void,
                              clid_len as libc::c_ulong) == 0 as libc::c_int {
                    return lease
                }
            }
            lease = (*lease).next
        }
    }
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) != 0) {
            if ((*lease).clid.is_null() || clid.is_null()) &&
                   hw_len != 0 as libc::c_int && (*lease).hwaddr_len == hw_len
                   && (*lease).hwaddr_type == hw_type &&
                   memcmp(hwaddr as *const libc::c_void,
                          (*lease).hwaddr.as_mut_ptr() as *const libc::c_void,
                          hw_len as libc::c_ulong) == 0 as libc::c_int {
                return lease
            }
        }
        lease = (*lease).next
    }
    return 0 as *mut dhcp_lease;
}
#[no_mangle]
pub unsafe extern "C" fn lease_find_by_addr(mut addr: in_addr)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) != 0) {
            if (*lease).addr.s_addr == addr.s_addr { return lease }
        }
        lease = (*lease).next
    }
    return 0 as *mut dhcp_lease;
}
/* find address for {CLID, IAID, address} */
#[no_mangle]
pub unsafe extern "C" fn lease6_find(mut clid: *mut libc::c_uchar,
                                     mut clid_len: libc::c_int,
                                     mut lease_type: libc::c_int,
                                     mut iaid: libc::c_uint,
                                     mut addr: *mut in6_addr)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & lease_type == 0 || (*lease).iaid != iaid) {
            if !(({
                      let mut __a: *const in6_addr =
                          &mut (*lease).addr6 as *mut in6_addr as
                              *const in6_addr;
                      let mut __b: *const in6_addr = addr as *const in6_addr;
                      ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                           ==
                           (*__b).__in6_u.__u6_addr32[0 as libc::c_int as
                                                          usize] &&
                           (*__a).__in6_u.__u6_addr32[1 as libc::c_int as
                                                          usize] ==
                               (*__b).__in6_u.__u6_addr32[1 as libc::c_int as
                                                              usize] &&
                           (*__a).__in6_u.__u6_addr32[2 as libc::c_int as
                                                          usize] ==
                               (*__b).__in6_u.__u6_addr32[2 as libc::c_int as
                                                              usize] &&
                           (*__a).__in6_u.__u6_addr32[3 as libc::c_int as
                                                          usize] ==
                               (*__b).__in6_u.__u6_addr32[3 as libc::c_int as
                                                              usize]) as
                          libc::c_int
                  }) == 0) {
                if !(clid_len != (*lease).clid_len ||
                         memcmp(clid as *const libc::c_void,
                                (*lease).clid as *const libc::c_void,
                                clid_len as libc::c_ulong) !=
                             0 as libc::c_int) {
                    return lease
                }
            }
        }
        lease = (*lease).next
    }
    return 0 as *mut dhcp_lease;
}
/* reset "USED flags */
#[no_mangle]
pub unsafe extern "C" fn lease6_reset() {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        (*lease).flags &= !(16 as libc::c_int);
        lease = (*lease).next
    };
}
/* enumerate all leases belonging to {CLID, IAID} */
#[no_mangle]
pub unsafe extern "C" fn lease6_find_by_client(mut first: *mut dhcp_lease,
                                               mut lease_type: libc::c_int,
                                               mut clid: *mut libc::c_uchar,
                                               mut clid_len: libc::c_int,
                                               mut iaid: libc::c_uint)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    if first.is_null() { first = leases } else { first = (*first).next }
    lease = first;
    while !lease.is_null() {
        if !((*lease).flags & 16 as libc::c_int != 0) {
            if !((*lease).flags & lease_type == 0 || (*lease).iaid != iaid) {
                if !(clid_len != (*lease).clid_len ||
                         memcmp(clid as *const libc::c_void,
                                (*lease).clid as *const libc::c_void,
                                clid_len as libc::c_ulong) !=
                             0 as libc::c_int) {
                    return lease
                }
            }
        }
        lease = (*lease).next
    }
    return 0 as *mut dhcp_lease;
}
#[no_mangle]
pub unsafe extern "C" fn lease6_find_by_addr(mut net: *mut in6_addr,
                                             mut prefix: libc::c_int,
                                             mut addr: u64_0)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) == 0) {
            if is_same_net6(&mut (*lease).addr6, net, prefix) != 0 &&
                   (prefix == 128 as libc::c_int ||
                        addr6part(&mut (*lease).addr6) == addr) {
                return lease
            }
        }
        lease = (*lease).next
    }
    return 0 as *mut dhcp_lease;
}
/* Find largest assigned address in context */
#[no_mangle]
pub unsafe extern "C" fn lease_find_max_addr6(mut context: *mut dhcp_context)
 -> u64_0 {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut addr: u64_0 = addr6part(&mut (*context).start6);
    if (*context).flags as libc::c_uint &
           ((1 as libc::c_uint) << 0 as libc::c_int |
                (1 as libc::c_uint) << 3 as libc::c_int) == 0 {
        lease = leases;
        while !lease.is_null() {
            if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) ==
                     0) {
                if is_same_net6(&mut (*lease).addr6, &mut (*context).start6,
                                64 as libc::c_int) != 0 &&
                       addr6part(&mut (*lease).addr6) >
                           addr6part(&mut (*context).start6) &&
                       addr6part(&mut (*lease).addr6) <=
                           addr6part(&mut (*context).end6) &&
                       addr6part(&mut (*lease).addr6) > addr {
                    addr = addr6part(&mut (*lease).addr6)
                }
            }
            lease = (*lease).next
        }
    }
    return addr;
}
/* Find largest assigned address in context */
#[no_mangle]
pub unsafe extern "C" fn lease_find_max_addr(mut context: *mut dhcp_context)
 -> in_addr {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease; /* illegal value */
    let mut addr: in_addr = (*context).start;
    if (*context).flags as libc::c_uint &
           ((1 as libc::c_uint) << 0 as libc::c_int |
                (1 as libc::c_uint) << 3 as libc::c_int) == 0 {
        lease = leases;
        while !lease.is_null() {
            if !((*lease).flags & (64 as libc::c_int | 32 as libc::c_int) !=
                     0) {
                if __bswap_32((*lease).addr.s_addr) >
                       __bswap_32((*context).start.s_addr) &&
                       __bswap_32((*lease).addr.s_addr) <=
                           __bswap_32((*context).end.s_addr) &&
                       __bswap_32((*lease).addr.s_addr) >
                           __bswap_32(addr.s_addr) {
                    addr = (*lease).addr
                }
            }
            lease = (*lease).next
        }
    }
    return addr;
}
unsafe extern "C" fn lease_allocate() -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    if leases_left == 0 ||
           {
               lease =
                   whine_malloc(::std::mem::size_of::<dhcp_lease>() as
                                    libc::c_ulong) as *mut dhcp_lease;
               lease.is_null()
           } {
        return 0 as *mut dhcp_lease
    }
    memset(lease as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<dhcp_lease>() as libc::c_ulong);
    (*lease).flags = 1 as libc::c_int;
    (*lease).expires = 1 as libc::c_int as time_t;
    (*lease).hwaddr_len = 256 as libc::c_int;
    (*lease).next = leases;
    leases = lease;
    file_dirty = 1 as libc::c_int;
    leases_left -= 1;
    return lease;
}
#[no_mangle]
pub unsafe extern "C" fn lease4_allocate(mut addr: in_addr)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = lease_allocate();
    if !lease.is_null() {
        (*lease).addr = addr;
        (*dnsmasq_daemon).metrics[METRIC_LEASES_ALLOCATED_4 as libc::c_int as
                                      usize] =
            (*dnsmasq_daemon).metrics[METRIC_LEASES_ALLOCATED_4 as libc::c_int
                                          as usize].wrapping_add(1)
    }
    return lease;
}
#[no_mangle]
pub unsafe extern "C" fn lease6_allocate(mut addrp: *mut in6_addr,
                                         mut lease_type: libc::c_int)
 -> *mut dhcp_lease {
    let mut lease: *mut dhcp_lease = lease_allocate();
    if !lease.is_null() {
        (*lease).addr6 = *addrp;
        (*lease).flags |= lease_type;
        (*lease).iaid = 0 as libc::c_int as libc::c_uint;
        (*dnsmasq_daemon).metrics[METRIC_LEASES_ALLOCATED_6 as libc::c_int as
                                      usize] =
            (*dnsmasq_daemon).metrics[METRIC_LEASES_ALLOCATED_6 as libc::c_int
                                          as usize].wrapping_add(1)
    }
    return lease;
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_expires(mut lease: *mut dhcp_lease,
                                           mut len: libc::c_uint,
                                           mut now: time_t) {
    let mut exp: time_t = 0;
    if len == 0xffffffff as libc::c_uint {
        exp = 0 as libc::c_int as time_t;
        len = 0 as libc::c_int as libc::c_uint
    } else {
        exp = now + len as time_t;
        /* Check for 2038 overflow. Make the lease
	 infinite in that case, as the least disruptive
	 thing we can do. */
        if difftime(exp, now) <= 0.0f64 { exp = 0 as libc::c_int as time_t }
    }
    if exp != (*lease).expires {
        dns_dirty = 1 as libc::c_int;
        (*lease).expires = exp;
        (*lease).flags |= 4 as libc::c_int | 256 as libc::c_int;
        file_dirty = 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_iaid(mut lease: *mut dhcp_lease,
                                        mut iaid: libc::c_uint) {
    if (*lease).iaid != iaid {
        (*lease).iaid = iaid;
        (*lease).flags |= 2 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_hwaddr(mut lease: *mut dhcp_lease,
                                          mut hwaddr: *const libc::c_uchar,
                                          mut clid: *const libc::c_uchar,
                                          mut hw_len: libc::c_int,
                                          mut hw_type: libc::c_int,
                                          mut clid_len: libc::c_int,
                                          mut now: time_t,
                                          mut force: libc::c_int) {
    let mut change: libc::c_int = force;
    (*lease).flags |= 128 as libc::c_int;
    if hw_len != (*lease).hwaddr_len || hw_type != (*lease).hwaddr_type ||
           hw_len != 0 as libc::c_int &&
               memcmp((*lease).hwaddr.as_mut_ptr() as *const libc::c_void,
                      hwaddr as *const libc::c_void, hw_len as libc::c_ulong)
                   != 0 as libc::c_int {
        if hw_len != 0 as libc::c_int {
            memcpy((*lease).hwaddr.as_mut_ptr() as *mut libc::c_void,
                   hwaddr as *const libc::c_void, hw_len as libc::c_ulong);
        }
        (*lease).hwaddr_len = hw_len;
        (*lease).hwaddr_type = hw_type;
        (*lease).flags |= 2 as libc::c_int;
        file_dirty = 1 as libc::c_int
        /* run script on change */
    }
    /* only update clid when one is available, stops packets
     without a clid removing the record. Lease init uses
     clid_len == 0 for no clid. */
    if clid_len != 0 as libc::c_int && !clid.is_null() {
        if (*lease).clid.is_null() { (*lease).clid_len = 0 as libc::c_int }
        if (*lease).clid_len != clid_len {
            (*lease).flags |= 4 as libc::c_int;
            file_dirty = 1 as libc::c_int;
            free((*lease).clid as *mut libc::c_void);
            (*lease).clid =
                whine_malloc(clid_len as size_t) as *mut libc::c_uchar;
            if (*lease).clid.is_null() { return }
            change = 1 as libc::c_int
        } else if memcmp((*lease).clid as *const libc::c_void,
                         clid as *const libc::c_void,
                         clid_len as libc::c_ulong) != 0 as libc::c_int {
            (*lease).flags |= 4 as libc::c_int;
            file_dirty = 1 as libc::c_int;
            change = 1 as libc::c_int
        }
        (*lease).clid_len = clid_len;
        memcpy((*lease).clid as *mut libc::c_void,
               clid as *const libc::c_void, clid_len as libc::c_ulong);
    }
    if change != 0 { slaac_add_addrs(lease, now, force); };
}
unsafe extern "C" fn kill_name(mut lease: *mut dhcp_lease) {
    /* run script to say we lost our old name */
    /* this shouldn't happen unless updates are very quick and the
     script very slow, we just avoid a memory leak if it does. */
    free((*lease).old_hostname as *mut libc::c_void);
    /* If we know the fqdn, pass that. The helper will derive the
     unqualified name from it, free the unqualified name here. */
    if !(*lease).fqdn.is_null() {
        (*lease).old_hostname = (*lease).fqdn;
        free((*lease).hostname as *mut libc::c_void);
    } else { (*lease).old_hostname = (*lease).hostname }
    (*lease).fqdn = 0 as *mut libc::c_char;
    (*lease).hostname = (*lease).fqdn;
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_hostname(mut lease: *mut dhcp_lease,
                                            mut name: *const libc::c_char,
                                            mut auth: libc::c_int,
                                            mut domain: *mut libc::c_char,
                                            mut config_domain:
                                                *mut libc::c_char) {
    let mut lease_tmp: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut new_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_fqdn: *mut libc::c_char = 0 as *mut libc::c_char;
    if !config_domain.is_null() &&
           (domain.is_null() || hostname_isequal(domain, config_domain) == 0)
       {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 4 as libc::c_int,
                  b"Ignoring domain %s for DHCP host name %s\x00" as *const u8
                      as *const libc::c_char, config_domain, name);
    }
    if !(*lease).hostname.is_null() && !name.is_null() &&
           hostname_isequal((*lease).hostname, name) != 0 {
        if auth != 0 { (*lease).flags |= 8 as libc::c_int }
        return
    }
    if name.is_null() && (*lease).hostname.is_null() { return }
    /* If a machine turns up on a new net without dropping the old lease,
     or two machines claim the same name, then we end up with two interfaces with
     the same name. Check for that here and remove the name from the old lease.
     Note that IPv6 leases are different. All the leases to the same DUID are 
     allowed the same name.

     Don't allow a name from the client to override a name from dnsmasq config. */
    if !name.is_null() {
        new_name =
            whine_malloc(strlen(name).wrapping_add(1 as libc::c_int as
                                                       libc::c_ulong)) as
                *mut libc::c_char;
        if !new_name.is_null() {
            strcpy(new_name, name);
            if !domain.is_null() &&
                   {
                       new_fqdn =
                           whine_malloc(strlen(new_name).wrapping_add(strlen(domain)).wrapping_add(2
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                               as *mut libc::c_char;
                       !new_fqdn.is_null()
                   } {
                strcpy(new_fqdn, name);
                strcat(new_fqdn,
                       b".\x00" as *const u8 as *const libc::c_char);
                strcat(new_fqdn, domain);
            }
        }
        let mut current_block_23: u64;
        /* Depending on mode, we check either unqualified name or FQDN. */
        lease_tmp = leases;
        while !lease_tmp.is_null() {
            if (*dnsmasq_daemon).options[(20 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (20 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
                if new_fqdn.is_null() || (*lease_tmp).fqdn.is_null() ||
                       hostname_isequal((*lease_tmp).fqdn, new_fqdn) == 0 {
                    current_block_23 = 17833034027772472439;
                } else { current_block_23 = 11307063007268554308; }
            } else if new_name.is_null() || (*lease_tmp).hostname.is_null() ||
                          hostname_isequal((*lease_tmp).hostname, new_name) ==
                              0 {
                current_block_23 = 17833034027772472439;
            } else { current_block_23 = 11307063007268554308; }
            match current_block_23 {
                11307063007268554308 => {
                    if (*lease).flags &
                           (64 as libc::c_int | 32 as libc::c_int) != 0 {
                        if (*lease_tmp).flags &
                               (64 as libc::c_int | 32 as libc::c_int) == 0 {
                            current_block_23 = 17833034027772472439;
                        } else if (*lease).clid_len == (*lease_tmp).clid_len
                                      && !(*lease).clid.is_null() &&
                                      !(*lease_tmp).clid.is_null() &&
                                      memcmp((*lease).clid as
                                                 *const libc::c_void,
                                             (*lease_tmp).clid as
                                                 *const libc::c_void,
                                             (*lease).clid_len as
                                                 libc::c_ulong) ==
                                          0 as libc::c_int {
                            current_block_23 = 17833034027772472439;
                        } else { current_block_23 = 1608152415753874203; }
                    } else if (*lease_tmp).flags &
                                  (64 as libc::c_int | 32 as libc::c_int) != 0
                     {
                        current_block_23 = 17833034027772472439;
                    } else { current_block_23 = 1608152415753874203; }
                    match current_block_23 {
                        17833034027772472439 => { }
                        _ => {
                            if (*lease_tmp).flags & 8 as libc::c_int != 0 &&
                                   auth == 0 {
                                free(new_name as *mut libc::c_void);
                                free(new_fqdn as *mut libc::c_void);
                                return
                            }
                            kill_name(lease_tmp);
                            break ;
                        }
                    }
                }
                _ => { }
            }
            lease_tmp = (*lease_tmp).next
        }
    }
    if !(*lease).hostname.is_null() { kill_name(lease); }
    (*lease).hostname = new_name;
    (*lease).fqdn = new_fqdn;
    if auth != 0 { (*lease).flags |= 8 as libc::c_int }
    file_dirty = 1 as libc::c_int;
    dns_dirty = 1 as libc::c_int;
    (*lease).flags |= 2 as libc::c_int;
    /* another lease for the same DUID is OK for IPv6 */
    /* run script on change */
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_interface(mut lease: *mut dhcp_lease,
                                             mut interface: libc::c_int,
                                             mut now: time_t) {
    if (*lease).last_interface == interface { return }
    (*lease).last_interface = interface;
    (*lease).flags |= 2 as libc::c_int;
    slaac_add_addrs(lease, now, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rerun_scripts() {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    lease = leases;
    while !lease.is_null() {
        (*lease).flags |= 2 as libc::c_int;
        lease = (*lease).next
    };
}
/* deleted leases get transferred to the old_leases list.
   remove them here, after calling the lease change
   script. Also run the lease change script on new/modified leases.

   Return zero if nothing to do. */
#[no_mangle]
pub unsafe extern "C" fn do_script_run(mut now: time_t) -> libc::c_int {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    if !old_leases.is_null() {
        lease = old_leases;
        /* If the lease still has an old_hostname, do the "old" action on that first */
        if !(*lease).old_hostname.is_null() {
            queue_script(2 as libc::c_int, lease, (*lease).old_hostname, now);
            free((*lease).old_hostname as *mut libc::c_void);
            (*lease).old_hostname = 0 as *mut libc::c_char;
            return 1 as libc::c_int
        } else {
            let mut slaac: *mut slaac_address = 0 as *mut slaac_address;
            let mut tmp: *mut slaac_address = 0 as *mut slaac_address;
            slaac = (*lease).slaac_address;
            while !slaac.is_null() {
                tmp = (*slaac).next;
                free(slaac as *mut libc::c_void);
                slaac = tmp
            }
            kill_name(lease);
            queue_script(1 as libc::c_int, lease, (*lease).old_hostname, now);
            old_leases = (*lease).next;
            free((*lease).old_hostname as *mut libc::c_void);
            free((*lease).clid as *mut libc::c_void);
            free((*lease).extradata as *mut libc::c_void);
            free(lease as *mut libc::c_void);
            return 1 as libc::c_int
        }
    }
    /* make sure we announce the loss of a hostname before its new location. */
    lease = leases;
    while !lease.is_null() {
        if !(*lease).old_hostname.is_null() {
            queue_script(2 as libc::c_int, lease, (*lease).old_hostname, now);
            free((*lease).old_hostname as *mut libc::c_void);
            (*lease).old_hostname = 0 as *mut libc::c_char;
            return 1 as libc::c_int
        }
        lease = (*lease).next
    }
    lease = leases;
    while !lease.is_null() {
        if (*lease).flags & (1 as libc::c_int | 2 as libc::c_int) != 0 ||
               (*lease).flags & 4 as libc::c_int != 0 &&
                   (*dnsmasq_daemon).options[(22 as libc::c_int as
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
                       != 0 ||
               (*lease).flags & 256 as libc::c_int != 0 &&
                   (*dnsmasq_daemon).options[(61 as libc::c_int as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &
                       (1 as libc::c_uint) <<
                           (61 as libc::c_int as
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                       != 0 {
            queue_script(if (*lease).flags & 1 as libc::c_int != 0 {
                             4 as libc::c_int
                         } else { 3 as libc::c_int }, lease,
                         if !(*lease).fqdn.is_null() {
                             (*lease).fqdn
                         } else { (*lease).hostname }, now);
            (*lease).flags &=
                !(1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int |
                      256 as libc::c_int);
            /* this is used for the "add" call, then junked, since they're not in the database */
            free((*lease).extradata as *mut libc::c_void);
            (*lease).extradata = 0 as *mut libc::c_uchar;
            return 1 as libc::c_int
        }
        lease = (*lease).next
    }
    return 0 as libc::c_int;
    /* nothing to do */
}
/* delim == -1 -> delim = 0, but embedded 0s, creating extra records, are OK. */
#[no_mangle]
pub unsafe extern "C" fn lease_add_extradata(mut lease: *mut dhcp_lease,
                                             mut data: *mut libc::c_uchar,
                                             mut len: libc::c_uint,
                                             mut delim: libc::c_int) {
    let mut i: libc::c_uint = 0;
    if delim == -(1 as libc::c_int) {
        delim = 0 as libc::c_int
    } else {
        /* check for embedded NULLs */
        i = 0 as libc::c_int as libc::c_uint;
        while i < len {
            if *data.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                len = i;
                break ;
            } else { i = i.wrapping_add(1) }
        }
    }
    if (*lease).extradata_size.wrapping_sub((*lease).extradata_len) <
           len.wrapping_add(1 as libc::c_int as libc::c_uint) {
        let mut newsz: size_t =
            (*lease).extradata_len.wrapping_add(len).wrapping_add(100 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                as size_t;
        let mut new: *mut libc::c_uchar =
            whine_malloc(newsz) as *mut libc::c_uchar;
        if new.is_null() { return }
        if !(*lease).extradata.is_null() {
            memcpy(new as *mut libc::c_void,
                   (*lease).extradata as *const libc::c_void,
                   (*lease).extradata_len as libc::c_ulong);
            free((*lease).extradata as *mut libc::c_void);
        }
        (*lease).extradata = new;
        (*lease).extradata_size = newsz as libc::c_uint
    }
    if len != 0 as libc::c_int as libc::c_uint {
        memcpy((*lease).extradata.offset((*lease).extradata_len as isize) as
                   *mut libc::c_void, data as *const libc::c_void,
               len as libc::c_ulong);
    }
    *(*lease).extradata.offset((*lease).extradata_len.wrapping_add(len) as
                                   isize) = delim as libc::c_uchar;
    (*lease).extradata_len =
        (*lease).extradata_len.wrapping_add(len.wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint));
}
