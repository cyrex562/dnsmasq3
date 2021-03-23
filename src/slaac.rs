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
    fn sendto(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t,
              __flags: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
              __addr_len: socklen_t) -> ssize_t;
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
    fn rand16() -> libc::c_ushort;
    #[no_mangle]
    fn whine_malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn my_syslog(priority: libc::c_int, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn lease_update_dns(force: libc::c_int);
    #[no_mangle]
    fn reset_counter();
    #[no_mangle]
    fn save_counter(newval: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn expand(headroom: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn ra_start_unsolicited(now: time_t, context: *mut dhcp_context);
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
pub const IPPROTO_MH: C2RustUnnamed_0 = 135;
pub const IPPROTO_DSTOPTS: C2RustUnnamed_0 = 60;
pub const IPPROTO_NONE: C2RustUnnamed_0 = 59;
pub const IPPROTO_ICMPV6: C2RustUnnamed_0 = 58;
pub const IPPROTO_FRAGMENT: C2RustUnnamed_0 = 44;
pub const IPPROTO_ROUTING: C2RustUnnamed_0 = 43;
pub const IPPROTO_HOPOPTS: C2RustUnnamed_0 = 0;
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
pub type u32_0 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ping_packet {
    pub type_0: u8_0,
    pub code: u8_0,
    pub checksum: u16_0,
    pub identifier: u16_0,
    pub sequence_no: u16_0,
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
unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
    return __x;
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
unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
    return __x;
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
static mut ping_id: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn slaac_add_addrs(mut lease: *mut dhcp_lease,
                                         mut now: time_t,
                                         mut force: libc::c_int) {
    let mut slaac: *mut slaac_address = 0 as *mut slaac_address;
    let mut old: *mut slaac_address = 0 as *mut slaac_address;
    let mut up: *mut *mut slaac_address = 0 as *mut *mut slaac_address;
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut dns_dirty: libc::c_int = 0 as libc::c_int;
    if (*lease).flags & 128 as libc::c_int == 0 ||
           (*lease).flags & (64 as libc::c_int | 32 as libc::c_int) != 0 ||
           (*lease).last_interface == 0 as libc::c_int ||
           (*lease).hostname.is_null() {
        return
    }
    old = (*lease).slaac_address;
    (*lease).slaac_address = 0 as *mut slaac_address;
    let mut current_block_31: u64;
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 6 as libc::c_int != 0 &&
               (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 16 as libc::c_int == 0 &&
               (*lease).last_interface == (*context).if_index {
            let mut addr: in6_addr = (*context).start6;
            if (*lease).hwaddr_len == 6 as libc::c_int &&
                   ((*lease).hwaddr_type == 1 as libc::c_int ||
                        (*lease).hwaddr_type == 6 as libc::c_int) {
                /* convert MAC address to EUI-64 */
                memcpy(&mut *addr.__in6_u.__u6_addr8.as_mut_ptr().offset(8 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                           as *mut uint8_t as *mut libc::c_void,
                       (*lease).hwaddr.as_mut_ptr() as *const libc::c_void,
                       3 as libc::c_int as libc::c_ulong);
                memcpy(&mut *addr.__in6_u.__u6_addr8.as_mut_ptr().offset(13 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                           as *mut uint8_t as *mut libc::c_void,
                       &mut *(*lease).hwaddr.as_mut_ptr().offset(3 as
                                                                     libc::c_int
                                                                     as isize)
                           as *mut libc::c_uchar as *const libc::c_void,
                       3 as libc::c_int as libc::c_ulong);
                addr.__in6_u.__u6_addr8[11 as libc::c_int as usize] =
                    0xff as libc::c_int as uint8_t;
                addr.__in6_u.__u6_addr8[12 as libc::c_int as usize] =
                    0xfe as libc::c_int as uint8_t;
                current_block_31 = 12039483399334584727;
            } else if (*lease).hwaddr_len == 8 as libc::c_int &&
                          (*lease).hwaddr_type == 27 as libc::c_int {
                memcpy(&mut *addr.__in6_u.__u6_addr8.as_mut_ptr().offset(8 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                           as *mut uint8_t as *mut libc::c_void,
                       (*lease).hwaddr.as_mut_ptr() as *const libc::c_void,
                       8 as libc::c_int as libc::c_ulong);
                current_block_31 = 12039483399334584727;
            } else if (*lease).clid_len == 9 as libc::c_int &&
                          *(*lease).clid.offset(0 as libc::c_int as isize) as
                              libc::c_int == 27 as libc::c_int &&
                          (*lease).hwaddr_type == 24 as libc::c_int {
                /* firewire has EUI-64 identifier as clid */
                memcpy(&mut *addr.__in6_u.__u6_addr8.as_mut_ptr().offset(8 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                           as *mut uint8_t as *mut libc::c_void,
                       &mut *(*lease).clid.offset(1 as libc::c_int as isize)
                           as *mut libc::c_uchar as *const libc::c_void,
                       8 as libc::c_int as libc::c_ulong);
                current_block_31 = 12039483399334584727;
            } else { current_block_31 = 6873731126896040597; }
            match current_block_31 {
                6873731126896040597 => { }
                _ => {
                    addr.__in6_u.__u6_addr8[8 as libc::c_int as usize] =
                        (addr.__in6_u.__u6_addr8[8 as libc::c_int as usize] as
                             libc::c_int ^ 0x2 as libc::c_int) as uint8_t;
                    /* check if we already have this one */
                    up = &mut old;
                    slaac = old;
                    while !slaac.is_null() {
                        if ({
                                let mut __a: *const in6_addr =
                                    &mut addr as *mut in6_addr as
                                        *const in6_addr;
                                let mut __b: *const in6_addr =
                                    &mut (*slaac).addr as *mut in6_addr as
                                        *const in6_addr;
                                ((*__a).__in6_u.__u6_addr32[0 as libc::c_int
                                                                as usize] ==
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
                            *up = (*slaac).next;
                            /* recheck when DHCPv4 goes through init-reboot */
                            if force != 0 {
                                (*slaac).ping_time = now;
                                (*slaac).backoff = 1 as libc::c_int;
                                dns_dirty = 1 as libc::c_int
                            }
                            break ;
                        } else {
                            up = &mut (*slaac).next;
                            slaac = (*slaac).next
                        }
                    }
                    /* No, make new one */
                    if slaac.is_null() &&
                           {
                               slaac =
                                   whine_malloc(::std::mem::size_of::<slaac_address>()
                                                    as libc::c_ulong) as
                                       *mut slaac_address;
                               !slaac.is_null()
                           } {
                        (*slaac).ping_time = now;
                        (*slaac).backoff = 1 as libc::c_int;
                        (*slaac).addr = addr;
                        /* Do RA's to prod it */
                        ra_start_unsolicited(now, context);
                    }
                    if !slaac.is_null() {
                        (*slaac).next = (*lease).slaac_address;
                        (*lease).slaac_address = slaac
                    }
                }
            }
        }
        context = (*context).next
    }
    if !old.is_null() || dns_dirty != 0 {
        lease_update_dns(1 as libc::c_int);
    }
    /* Free any no reused */
    while !old.is_null() {
        slaac = (*old).next;
        free(old as *mut libc::c_void);
        old = slaac
    };
}
#[no_mangle]
pub unsafe extern "C" fn periodic_slaac(mut now: time_t,
                                        mut leases: *mut dhcp_lease)
 -> time_t {
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut slaac: *mut slaac_address = 0 as *mut slaac_address;
    let mut next_event: time_t = 0 as libc::c_int as time_t;
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 6 as libc::c_int != 0 &&
               (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 16 as libc::c_int == 0 {
            break ;
        }
        context = (*context).next
    }
    /* nothing configured */
    if context.is_null() { return 0 as libc::c_int as time_t }
    while ping_id == 0 as libc::c_int { ping_id = rand16() as libc::c_int }
    lease = leases;
    while !lease.is_null() {
        let mut current_block_26: u64;
        slaac = (*lease).slaac_address;
        while !slaac.is_null() {
            /* confirmed or given up? */
            if !((*slaac).backoff == 0 as libc::c_int ||
                     (*slaac).ping_time == 0 as libc::c_int as libc::c_long) {
                if difftime((*slaac).ping_time, now) <= 0.0f64 {
                    let mut ping: *mut ping_packet =
                        0 as *mut ping_packet; /* Give up */
                    let mut addr: sockaddr_in6 =
                        sockaddr_in6{sin6_family: 0,
                                     sin6_port: 0,
                                     sin6_flowinfo: 0,
                                     sin6_addr:
                                         in6_addr{__in6_u:
                                                      C2RustUnnamed{__u6_addr8:
                                                                        [0;
                                                                            16],},},
                                     sin6_scope_id: 0,}; /* 0 - 3 */
                    reset_counter(); /* 0 - 15 */
                    ping =
                        expand(::std::mem::size_of::<ping_packet>() as
                                   libc::c_ulong) as *mut ping_packet;
                    if ping.is_null() {
                        current_block_26 = 12209867499936983673;
                    } else {
                        (*ping).type_0 = 128 as libc::c_int as u8_0;
                        (*ping).code = 0 as libc::c_int as u8_0;
                        (*ping).identifier = ping_id as u16_0;
                        (*ping).sequence_no = (*slaac).backoff as u16_0;
                        memset(&mut addr as *mut sockaddr_in6 as
                                   *mut libc::c_void, 0 as libc::c_int,
                               ::std::mem::size_of::<sockaddr_in6>() as
                                   libc::c_ulong);
                        addr.sin6_family = 10 as libc::c_int as sa_family_t;
                        addr.sin6_port =
                            __bswap_16(IPPROTO_ICMPV6 as libc::c_int as
                                           __uint16_t);
                        addr.sin6_addr = (*slaac).addr;
                        if sendto((*dnsmasq_daemon).icmp6fd,
                                  (*dnsmasq_daemon).outpacket.iov_base,
                                  save_counter(-(1 as libc::c_int)) as size_t,
                                  0 as libc::c_int,
                                  __CONST_SOCKADDR_ARG{__sockaddr__:
                                                           &mut addr as
                                                               *mut sockaddr_in6
                                                               as
                                                               *mut sockaddr,},
                                  ::std::mem::size_of::<sockaddr_in6>() as
                                      libc::c_ulong as socklen_t) ==
                               -(1 as libc::c_int) as libc::c_long &&
                               *__errno_location() == 113 as libc::c_int &&
                               (*slaac).backoff == 12 as libc::c_int {
                            (*slaac).ping_time = 0 as libc::c_int as time_t
                        } else {
                            (*slaac).ping_time +=
                                (((1 as libc::c_int) <<
                                      (*slaac).backoff - 1 as libc::c_int) +
                                     rand16() as libc::c_int /
                                         21785 as libc::c_int) as
                                    libc::c_long;
                            if (*slaac).backoff > 4 as libc::c_int {
                                (*slaac).ping_time +=
                                    (rand16() as libc::c_int /
                                         4000 as libc::c_int) as libc::c_long
                            }
                            if (*slaac).backoff < 12 as libc::c_int {
                                (*slaac).backoff += 1
                            }
                        }
                        current_block_26 = 3275366147856559585;
                    }
                } else { current_block_26 = 3275366147856559585; }
                match current_block_26 {
                    12209867499936983673 => { }
                    _ => {
                        if (*slaac).ping_time !=
                               0 as libc::c_int as libc::c_long &&
                               (next_event == 0 as libc::c_int as libc::c_long
                                    ||
                                    difftime(next_event, (*slaac).ping_time)
                                        >= 0.0f64) {
                            next_event = (*slaac).ping_time
                        }
                    }
                }
            }
            slaac = (*slaac).next
        }
        lease = (*lease).next
    }
    return next_event;
}
#[no_mangle]
pub unsafe extern "C" fn slaac_ping_reply(mut sender: *mut in6_addr,
                                          mut packet: *mut libc::c_uchar,
                                          mut interface: *mut libc::c_char,
                                          mut leases: *mut dhcp_lease) {
    let mut lease: *mut dhcp_lease = 0 as *mut dhcp_lease;
    let mut slaac: *mut slaac_address = 0 as *mut slaac_address;
    let mut ping: *mut ping_packet = packet as *mut ping_packet;
    let mut gotone: libc::c_int = 0 as libc::c_int;
    if (*ping).identifier as libc::c_int == ping_id {
        lease = leases;
        while !lease.is_null() {
            slaac = (*lease).slaac_address;
            while !slaac.is_null() {
                if (*slaac).backoff != 0 as libc::c_int &&
                       ({
                            let mut __a: *const in6_addr =
                                sender as *const in6_addr;
                            let mut __b: *const in6_addr =
                                &mut (*slaac).addr as *mut in6_addr as
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
                    (*slaac).backoff = 0 as libc::c_int;
                    gotone = 1 as libc::c_int;
                    inet_ntop(10 as libc::c_int,
                              sender as *const libc::c_void,
                              (*dnsmasq_daemon).addrbuff,
                              46 as libc::c_int as socklen_t);
                    if (*dnsmasq_daemon).options[(43 as libc::c_int as
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
                        my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                      6 as libc::c_int,
                                  b"SLAAC-CONFIRM(%s) %s %s\x00" as *const u8
                                      as *const libc::c_char, interface,
                                  (*dnsmasq_daemon).addrbuff,
                                  (*lease).hostname);
                    }
                }
                slaac = (*slaac).next
            }
            lease = (*lease).next
        }
    }
    lease_update_dns(gotone);
}
