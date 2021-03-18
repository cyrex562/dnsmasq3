#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn recvmsg(__fd: libc::c_int, __message: *mut msghdr,
               __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                  __optname: libc::c_int, __optval: *const libc::c_void,
                  __optlen: socklen_t) -> libc::c_int;
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
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn cache_find_by_name(crecp: *mut crec, name: *mut libc::c_char,
                          now: time_t, prot: libc::c_uint) -> *mut crec;
    #[no_mangle]
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn whine_malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn hostname_isequal(a: *const libc::c_char, b: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn is_same_net(a: in_addr, b: in_addr, mask: in_addr) -> libc::c_int;
    #[no_mangle]
    fn is_same_net6(a: *mut in6_addr, b: *mut in6_addr,
                    prefixlen: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn setaddr6part(addr: *mut in6_addr, host: u64_0);
    #[no_mangle]
    fn prettyprint_time(buf: *mut libc::c_char, t: libc::c_uint);
    #[no_mangle]
    fn memcmp_masked(a: *mut libc::c_uchar, b: *mut libc::c_uchar,
                     len: libc::c_int, mask: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn expand_buf(iov: *mut iovec, size: size_t) -> libc::c_int;
    #[no_mangle]
    fn print_mac(buff: *mut libc::c_char, mac: *mut libc::c_uchar,
                 len: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn die(message: *mut libc::c_char, arg1: *mut libc::c_char,
           exit_code: libc::c_int) -> !;
    #[no_mangle]
    fn my_syslog(priority: libc::c_int, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn config_find_by_address6(configs: *mut dhcp_config, net: *mut in6_addr,
                               prefix: libc::c_int, addr: *mut in6_addr)
     -> *mut dhcp_config;
    #[no_mangle]
    fn indextoname(fd: libc::c_int, index: libc::c_int,
                   name: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn config_find_by_address(configs: *mut dhcp_config, addr: in_addr)
     -> *mut dhcp_config;
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
pub type C2RustUnnamed_1 = libc::c_uint;
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
pub struct opttab_t {
    pub name: *mut libc::c_char,
    pub val: u16_0,
    pub size: u16_0,
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
pub unsafe extern "C" fn dhcp_common_init() {
    /* These each hold a DHCP option max size 255
     and get a terminating zero added */
    (*dnsmasq_daemon).dhcp_buff =
        safe_malloc(256 as libc::c_int as size_t) as *mut libc::c_char;
    (*dnsmasq_daemon).dhcp_buff2 =
        safe_malloc(256 as libc::c_int as size_t) as *mut libc::c_char;
    (*dnsmasq_daemon).dhcp_buff3 =
        safe_malloc(256 as libc::c_int as size_t) as *mut libc::c_char;
    /* dhcp_packet is used by v4 and v6, outpacket only by v6 
     sizeof(struct dhcp_packet) is as good an initial size as any,
     even for v6 */
    expand_buf(&mut (*dnsmasq_daemon).dhcp_packet,
               ::std::mem::size_of::<dhcp_packet>() as libc::c_ulong);
    if !(*dnsmasq_daemon).dhcp6.is_null() {
        expand_buf(&mut (*dnsmasq_daemon).outpacket,
                   ::std::mem::size_of::<dhcp_packet>() as libc::c_ulong);
    };
}
#[no_mangle]
pub unsafe extern "C" fn recv_dhcp_packet(mut fd: libc::c_int,
                                          mut msg: *mut msghdr) -> ssize_t {
    let mut sz: ssize_t = 0;
    let mut new_sz: ssize_t = 0;
    loop  {
        (*msg).msg_flags = 0 as libc::c_int;
        loop  {
            sz =
                recvmsg(fd, msg,
                        MSG_PEEK as libc::c_int | MSG_TRUNC as libc::c_int);
            if !(sz == -(1 as libc::c_int) as libc::c_long &&
                     *__errno_location() == 4 as libc::c_int) {
                break ;
            }
        }
        if sz == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t
        }
        if (*msg).msg_flags & MSG_TRUNC as libc::c_int == 0 { break ; }
        /* Very new Linux kernels return the actual size needed, 
	 older ones always return truncated size */
        if sz as size_t == (*(*msg).msg_iov).iov_len {
            if expand_buf((*msg).msg_iov,
                          (sz + 100 as libc::c_int as libc::c_long) as size_t)
                   == 0 {
                return -(1 as libc::c_int) as ssize_t
            }
        } else { expand_buf((*msg).msg_iov, sz as size_t); break ; }
    }
    loop  {
        new_sz = recvmsg(fd, msg, 0 as libc::c_int);
        if !(new_sz == -(1 as libc::c_int) as libc::c_long &&
                 *__errno_location() == 4 as libc::c_int) {
            break ;
        }
    }
    /* Some kernels seem to ignore MSG_PEEK, and dequeue the packet anyway. 
     If that happens we get EAGAIN here because the socket is non-blocking.
     Use the result of the original testing recvmsg as long as the buffer
     was big enough. There's a small race here that may lose the odd packet,
     but it's UDP anyway. */
    if new_sz == -(1 as libc::c_int) as libc::c_long &&
           (*__errno_location() == 11 as libc::c_int ||
                *__errno_location() == 11 as libc::c_int) {
        new_sz = sz
    }
    return if (*msg).msg_flags & MSG_TRUNC as libc::c_int != 0 {
               -(1 as libc::c_int) as libc::c_long
           } else { new_sz };
}
#[no_mangle]
pub unsafe extern "C" fn run_tag_if(mut tags: *mut dhcp_netid)
 -> *mut dhcp_netid {
    let mut exprs: *mut tag_if = 0 as *mut tag_if;
    let mut list: *mut dhcp_netid_list = 0 as *mut dhcp_netid_list;
    exprs = (*dnsmasq_daemon).tag_if;
    while !exprs.is_null() {
        if match_netid((*exprs).tag, tags, 1 as libc::c_int) != 0 {
            list = (*exprs).set;
            while !list.is_null() {
                (*(*list).list).next = tags;
                tags = (*list).list;
                list = (*list).next
            }
        }
        exprs = (*exprs).next
    }
    return tags;
}
#[no_mangle]
pub unsafe extern "C" fn option_filter(mut tags: *mut dhcp_netid,
                                       mut context_tags: *mut dhcp_netid,
                                       mut opts: *mut dhcp_opt)
 -> *mut dhcp_netid {
    let mut tagif: *mut dhcp_netid = run_tag_if(tags);
    let mut opt: *mut dhcp_opt = 0 as *mut dhcp_opt;
    let mut tmp: *mut dhcp_opt = 0 as *mut dhcp_opt;
    /* flag options which are valid with the current tag set (sans context tags) */
    opt = opts;
    while !opt.is_null() {
        (*opt).flags &= !(4096 as libc::c_int);
        if (*opt).flags &
               (4 as libc::c_int | 256 as libc::c_int | 2048 as libc::c_int)
               == 0 && match_netid((*opt).netid, tagif, 0 as libc::c_int) != 0
           {
            (*opt).flags |= 4096 as libc::c_int
        }
        opt = (*opt).next
    }
    /* now flag options which are valid, including the context tags,
     otherwise valid options are inhibited if we found a higher priority one above */
    if !context_tags.is_null() {
        let mut last_tag: *mut dhcp_netid = 0 as *mut dhcp_netid;
        last_tag = context_tags;
        while !(*last_tag).next.is_null() { last_tag = (*last_tag).next }
        (*last_tag).next = tags;
        tagif = run_tag_if(context_tags);
        /* reset stuff with tag:!<tag> which now matches. */
        opt = opts;
        while !opt.is_null() {
            if (*opt).flags &
                   (4 as libc::c_int | 256 as libc::c_int |
                        2048 as libc::c_int) == 0 &&
                   (*opt).flags & 4096 as libc::c_int != 0 &&
                   match_netid((*opt).netid, tagif, 0 as libc::c_int) == 0 {
                (*opt).flags &= !(4096 as libc::c_int)
            }
            opt = (*opt).next
        }
        opt = opts;
        while !opt.is_null() {
            if (*opt).flags &
                   (4 as libc::c_int | 256 as libc::c_int |
                        2048 as libc::c_int | 4096 as libc::c_int) == 0 &&
                   match_netid((*opt).netid, tagif, 0 as libc::c_int) != 0 {
                let mut tmp_0: *mut dhcp_opt = 0 as *mut dhcp_opt;
                tmp_0 = opts;
                while !tmp_0.is_null() {
                    if (*tmp_0).opt == (*opt).opt && !(*opt).netid.is_null()
                           && (*tmp_0).flags & 4096 as libc::c_int != 0 {
                        break ;
                    }
                    tmp_0 = (*tmp_0).next
                }
                if tmp_0.is_null() { (*opt).flags |= 4096 as libc::c_int }
            }
            opt = (*opt).next
        }
    }
    /* now flag untagged options which are not overridden by tagged ones */
    opt = opts;
    while !opt.is_null() {
        if (*opt).flags &
               (4 as libc::c_int | 256 as libc::c_int | 2048 as libc::c_int |
                    4096 as libc::c_int) == 0 && (*opt).netid.is_null() {
            tmp = opts;
            while !tmp.is_null() {
                if (*tmp).opt == (*opt).opt &&
                       (*tmp).flags & 4096 as libc::c_int != 0 {
                    break ;
                }
                tmp = (*tmp).next
            }
            if tmp.is_null() {
                (*opt).flags |= 4096 as libc::c_int
            } else if (*tmp).netid.is_null() {
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              4 as libc::c_int,
                          b"Ignoring duplicate dhcp-option %d\x00" as
                              *const u8 as *const libc::c_char, (*tmp).opt);
            }
        }
        opt = (*opt).next
    }
    /* Finally, eliminate duplicate options later in the chain, and therefore earlier in the config file. */
    opt = opts;
    while !opt.is_null() {
        if (*opt).flags & 4096 as libc::c_int != 0 {
            tmp = (*opt).next;
            while !tmp.is_null() {
                if (*tmp).opt == (*opt).opt {
                    (*tmp).flags &= !(4096 as libc::c_int)
                }
                tmp = (*tmp).next
            }
        }
        opt = (*opt).next
    }
    return tagif;
}
/* Is every member of check matched by a member of pool? 
   If tagnotneeded, untagged is OK */
#[no_mangle]
pub unsafe extern "C" fn match_netid(mut check: *mut dhcp_netid,
                                     mut pool: *mut dhcp_netid,
                                     mut tagnotneeded: libc::c_int)
 -> libc::c_int {
    let mut tmp1: *mut dhcp_netid = 0 as *mut dhcp_netid;
    if check.is_null() && tagnotneeded == 0 { return 0 as libc::c_int }
    while !check.is_null() {
        /* '#' for not is for backwards compat. */
        if *(*check).net.offset(0 as libc::c_int as isize) as libc::c_int !=
               '!' as i32 &&
               *(*check).net.offset(0 as libc::c_int as isize) as libc::c_int
                   != '#' as i32 {
            tmp1 = pool;
            while !tmp1.is_null() {
                if strcmp((*check).net, (*tmp1).net) == 0 as libc::c_int {
                    break ;
                }
                tmp1 = (*tmp1).next
            }
            if tmp1.is_null() { return 0 as libc::c_int }
        } else {
            tmp1 = pool;
            while !tmp1.is_null() {
                if strcmp((*check).net.offset(1 as libc::c_int as isize),
                          (*tmp1).net) == 0 as libc::c_int {
                    return 0 as libc::c_int
                }
                tmp1 = (*tmp1).next
            }
        }
        check = (*check).next
    }
    return 1 as libc::c_int;
}
/* return domain or NULL if none. */
#[no_mangle]
pub unsafe extern "C" fn strip_hostname(mut hostname: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut dot: *mut libc::c_char =
        strchr(hostname, '.' as i32); /* truncate */
    if dot.is_null() { return 0 as *mut libc::c_char }
    *dot = 0 as libc::c_int as libc::c_char;
    if strlen(dot.offset(1 as libc::c_int as isize)) !=
           0 as libc::c_int as libc::c_ulong {
        return dot.offset(1 as libc::c_int as isize)
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn log_tags(mut netid: *mut dhcp_netid,
                                  mut xid: u32_0) {
    if !netid.is_null() &&
           (*dnsmasq_daemon).options[(28 as libc::c_int as
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
        let mut s: *mut libc::c_char = (*dnsmasq_daemon).namebuff;
        *s = 0 as libc::c_int as libc::c_char;
        while !netid.is_null() {
            /* kill dupes. */
            let mut n: *mut dhcp_netid = 0 as *mut dhcp_netid;
            n = (*netid).next;
            while !n.is_null() {
                if strcmp((*netid).net, (*n).net) == 0 as libc::c_int {
                    break ;
                }
                n = (*n).next
            }
            if n.is_null() {
                strncat(s, (*netid).net,
                        ((1025 as libc::c_int - 1 as libc::c_int) as
                             libc::c_ulong).wrapping_sub(strlen(s)));
                if !(*netid).next.is_null() {
                    strncat(s, b", \x00" as *const u8 as *const libc::c_char,
                            ((1025 as libc::c_int - 1 as libc::c_int) as
                                 libc::c_ulong).wrapping_sub(strlen(s)));
                }
            }
            netid = (*netid).next
        }
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"%u tags: %s\x00" as *const u8 as *const libc::c_char, xid,
                  s);
    };
}
#[no_mangle]
pub unsafe extern "C" fn match_bytes(mut o: *mut dhcp_opt,
                                     mut p: *mut libc::c_uchar,
                                     mut len: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*o).len > len { return 0 as libc::c_int }
    if (*o).len == 0 as libc::c_int { return 1 as libc::c_int }
    if (*o).flags & 512 as libc::c_int != 0 {
        if memcmp_masked((*o).val, p, (*o).len, (*o).u.wildcard_mask) != 0 {
            return 1 as libc::c_int
        }
    } else {
        i = 0 as libc::c_int;
        while i <= len - (*o).len {
            if memcmp((*o).val as *const libc::c_void,
                      p.offset(i as isize) as *const libc::c_void,
                      (*o).len as libc::c_ulong) == 0 as libc::c_int {
                return 1 as libc::c_int
            }
            if (*o).flags & 2 as libc::c_int != 0 {
                i += 1
            } else { i += (*o).len }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn config_has_mac(mut config: *mut dhcp_config,
                                        mut hwaddr: *mut libc::c_uchar,
                                        mut len: libc::c_int,
                                        mut type_0: libc::c_int)
 -> libc::c_int {
    let mut conf_addr: *mut hwaddr_config = 0 as *mut hwaddr_config;
    conf_addr = (*config).hwaddr;
    while !conf_addr.is_null() {
        if (*conf_addr).wildcard_mask == 0 as libc::c_int as libc::c_uint &&
               (*conf_addr).hwaddr_len == len &&
               ((*conf_addr).hwaddr_type == type_0 ||
                    (*conf_addr).hwaddr_type == 0 as libc::c_int) &&
               memcmp((*conf_addr).hwaddr.as_mut_ptr() as *const libc::c_void,
                      hwaddr as *const libc::c_void, len as libc::c_ulong) ==
                   0 as libc::c_int {
            return 1 as libc::c_int
        }
        conf_addr = (*conf_addr).next
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_config_in_context(mut context: *mut dhcp_context,
                                          mut config: *mut dhcp_config)
 -> libc::c_int {
    if context.is_null() {
        /* called via find_config() from lease_update_from_configs() */
        return 1 as libc::c_int
    }
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 17 as libc::c_int != 0 {
        let mut addr_list: *mut addrlist = 0 as *mut addrlist;
        if (*config).flags & 4096 as libc::c_int as libc::c_uint == 0 {
            return 1 as libc::c_int
        }
        while !context.is_null() {
            addr_list = (*config).addr6;
            while !addr_list.is_null() {
                if (*addr_list).flags & 16 as libc::c_int != 0 &&
                       (*context).prefix == 64 as libc::c_int {
                    return 1 as libc::c_int
                }
                if is_same_net6(&mut (*addr_list).addr.addr6,
                                &mut (*context).start6, (*context).prefix) !=
                       0 {
                    return 1 as libc::c_int
                }
                addr_list = (*addr_list).next
            }
            context = (*context).current
        }
    } else {
        if (*config).flags & 32 as libc::c_int as libc::c_uint == 0 {
            return 1 as libc::c_int
        }
        while !context.is_null() {
            if (*config).flags & 32 as libc::c_int as libc::c_uint != 0 &&
                   is_same_net((*config).addr, (*context).start,
                               (*context).netmask) != 0 {
                return 1 as libc::c_int
            }
            context = (*context).current
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_config_match(mut configs: *mut dhcp_config,
                                       mut context: *mut dhcp_context,
                                       mut clid: *mut libc::c_uchar,
                                       mut clid_len: libc::c_int,
                                       mut hwaddr: *mut libc::c_uchar,
                                       mut hw_len: libc::c_int,
                                       mut hw_type: libc::c_int,
                                       mut hostname: *mut libc::c_char,
                                       mut tags: *mut dhcp_netid,
                                       mut tag_not_needed: libc::c_int)
 -> *mut dhcp_config {
    let mut count: libc::c_int = 0;
    let mut new: libc::c_int = 0;
    let mut config: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut candidate: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut conf_addr: *mut hwaddr_config = 0 as *mut hwaddr_config;
    if !clid.is_null() {
        config = configs;
        while !config.is_null() {
            if (*config).flags & 2 as libc::c_int as libc::c_uint != 0 {
                if (*config).clid_len == clid_len &&
                       memcmp((*config).clid as *const libc::c_void,
                              clid as *const libc::c_void,
                              clid_len as libc::c_ulong) == 0 as libc::c_int
                       && is_config_in_context(context, config) != 0 &&
                       match_netid((*config).filter, tags, tag_not_needed) !=
                           0 {
                    return config
                }
                /* dhcpcd prefixes ASCII client IDs by zero which is wrong, but we try and
	     cope with that here. This is IPv4 only. context==NULL implies IPv4, 
	     see lease_update_from_configs() */
                if (context.is_null() ||
                        (*context).flags as libc::c_uint &
                            (1 as libc::c_uint) << 17 as libc::c_int == 0) &&
                       *clid as libc::c_int == 0 as libc::c_int &&
                       (*config).clid_len == clid_len - 1 as libc::c_int &&
                       memcmp((*config).clid as *const libc::c_void,
                              clid.offset(1 as libc::c_int as isize) as
                                  *const libc::c_void,
                              (clid_len - 1 as libc::c_int) as libc::c_ulong)
                           == 0 as libc::c_int &&
                       is_config_in_context(context, config) != 0 &&
                       match_netid((*config).filter, tags, tag_not_needed) !=
                           0 {
                    return config
                }
            }
            config = (*config).next
        }
    }
    if !hwaddr.is_null() {
        config = configs;
        while !config.is_null() {
            if config_has_mac(config, hwaddr, hw_len, hw_type) != 0 &&
                   is_config_in_context(context, config) != 0 &&
                   match_netid((*config).filter, tags, tag_not_needed) != 0 {
                return config
            }
            config = (*config).next
        }
    }
    if !hostname.is_null() && !context.is_null() {
        config = configs;
        while !config.is_null() {
            if (*config).flags & 16 as libc::c_int as libc::c_uint != 0 &&
                   hostname_isequal((*config).hostname, hostname) != 0 &&
                   is_config_in_context(context, config) != 0 &&
                   match_netid((*config).filter, tags, tag_not_needed) != 0 {
                return config
            }
            config = (*config).next
        }
    }
    if hwaddr.is_null() { return 0 as *mut dhcp_config }
    /* use match with fewest wildcard octets */
    candidate = 0 as *mut dhcp_config;
    count = 0 as libc::c_int;
    config = configs;
    while !config.is_null() {
        if is_config_in_context(context, config) != 0 &&
               match_netid((*config).filter, tags, tag_not_needed) != 0 {
            conf_addr = (*config).hwaddr;
            while !conf_addr.is_null() {
                if (*conf_addr).wildcard_mask !=
                       0 as libc::c_int as libc::c_uint &&
                       (*conf_addr).hwaddr_len == hw_len &&
                       ((*conf_addr).hwaddr_type == hw_type ||
                            (*conf_addr).hwaddr_type == 0 as libc::c_int) &&
                       {
                           new =
                               memcmp_masked((*conf_addr).hwaddr.as_mut_ptr(),
                                             hwaddr, hw_len,
                                             (*conf_addr).wildcard_mask);
                           (new) > count
                       } {
                    count = new;
                    candidate = config
                }
                conf_addr = (*conf_addr).next
            }
        }
        config = (*config).next
    }
    return candidate;
}
/* Find tagged configs first. */
#[no_mangle]
pub unsafe extern "C" fn find_config(mut configs: *mut dhcp_config,
                                     mut context: *mut dhcp_context,
                                     mut clid: *mut libc::c_uchar,
                                     mut clid_len: libc::c_int,
                                     mut hwaddr: *mut libc::c_uchar,
                                     mut hw_len: libc::c_int,
                                     mut hw_type: libc::c_int,
                                     mut hostname: *mut libc::c_char,
                                     mut tags: *mut dhcp_netid)
 -> *mut dhcp_config {
    let mut ret: *mut dhcp_config =
        find_config_match(configs, context, clid, clid_len, hwaddr, hw_len,
                          hw_type, hostname, tags, 0 as libc::c_int);
    if ret.is_null() {
        ret =
            find_config_match(configs, context, clid, clid_len, hwaddr,
                              hw_len, hw_type, hostname, tags,
                              1 as libc::c_int)
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn dhcp_update_configs(mut configs: *mut dhcp_config) {
    /* Some people like to keep all static IP addresses in /etc/hosts.
     This goes through /etc/hosts and sets static addresses for any DHCP config
     records which don't have an address and whose name matches. 
     We take care to maintain the invariant that any IP address can appear
     in at most one dhcp-host. Since /etc/hosts can be re-read by SIGHUP, 
     restore the status-quo ante first. */
    let mut config: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut conf_tmp: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut crec: *mut crec = 0 as *mut crec;
    let mut prot: libc::c_int = 2 as libc::c_int;
    config = configs;
    while !config.is_null() {
        if (*config).flags & 512 as libc::c_int as libc::c_uint != 0 {
            (*config).flags &=
                !(32 as libc::c_int | 512 as libc::c_int) as libc::c_uint
        }
        if (*config).flags & 16384 as libc::c_int as libc::c_uint != 0 {
            (*config).flags &=
                !(4096 as libc::c_int | 16384 as libc::c_int) as libc::c_uint
        }
        config = (*config).next
    }
    loop  {
        if (*dnsmasq_daemon).port != 0 as libc::c_int {
            let mut current_block_27: u64;
            config = configs;
            while !config.is_null() {
                let mut conflags: libc::c_int = 32 as libc::c_int;
                let mut cacheflags: libc::c_int =
                    ((1 as libc::c_uint) << 7 as libc::c_int) as libc::c_int;
                if prot == 10 as libc::c_int {
                    conflags = 4096 as libc::c_int;
                    cacheflags =
                        ((1 as libc::c_uint) << 8 as libc::c_int) as
                            libc::c_int
                }
                if (*config).flags & conflags as libc::c_uint == 0 &&
                       (*config).flags & 16 as libc::c_int as libc::c_uint !=
                           0 &&
                       {
                           crec =
                               cache_find_by_name(0 as *mut crec,
                                                  (*config).hostname,
                                                  0 as libc::c_int as time_t,
                                                  cacheflags as libc::c_uint);
                           !crec.is_null()
                       } &&
                       (*crec).flags & (1 as libc::c_uint) << 6 as libc::c_int
                           != 0 {
                    if !cache_find_by_name(crec, (*config).hostname,
                                           0 as libc::c_int as time_t,
                                           cacheflags as
                                               libc::c_uint).is_null() {
                        /* use primary (first) address */
                        while !crec.is_null() &&
                                  (*crec).flags &
                                      (1 as libc::c_uint) << 2 as libc::c_int
                                      == 0 {
                            crec =
                                cache_find_by_name(crec, (*config).hostname,
                                                   0 as libc::c_int as time_t,
                                                   cacheflags as libc::c_uint)
                        } /* should be never */
                        if crec.is_null() {
                            current_block_27 = 3640593987805443782;
                        } else {
                            inet_ntop(prot,
                                      &mut (*crec).addr as *mut all_addr as
                                          *const libc::c_void,
                                      (*dnsmasq_daemon).addrbuff,
                                      46 as libc::c_int as socklen_t);
                            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                          4 as libc::c_int,
                                      b"%s has more than one address in hostsfile, using %s for DHCP\x00"
                                          as *const u8 as *const libc::c_char,
                                      (*config).hostname,
                                      (*dnsmasq_daemon).addrbuff);
                            current_block_27 = 1109700713171191020;
                        }
                    } else { current_block_27 = 1109700713171191020; }
                    match current_block_27 {
                        3640593987805443782 => { }
                        _ => {
                            if prot == 2 as libc::c_int &&
                                   {
                                       conf_tmp =
                                           config_find_by_address(configs,
                                                                  (*crec).addr.addr4);
                                       (conf_tmp.is_null()) ||
                                           conf_tmp == config
                                   } {
                                (*config).addr = (*crec).addr.addr4;
                                (*config).flags |=
                                    (32 as libc::c_int | 512 as libc::c_int)
                                        as libc::c_uint
                            } else if prot == 10 as libc::c_int &&
                                          {
                                              conf_tmp =
                                                  config_find_by_address6(configs,
                                                                          0 as
                                                                              *mut in6_addr,
                                                                          0 as
                                                                              libc::c_int,
                                                                          &mut (*crec).addr.addr6);
                                              (conf_tmp.is_null()) ||
                                                  conf_tmp == config
                                          } {
                                /* host must have exactly one address if comming from /etc/hosts. */
                                if (*config).addr6.is_null() &&
                                       {
                                           (*config).addr6 =
                                               whine_malloc(::std::mem::size_of::<addrlist>()
                                                                as
                                                                libc::c_ulong)
                                                   as *mut addrlist;
                                           !(*config).addr6.is_null()
                                       } {
                                    (*(*config).addr6).next =
                                        0 as *mut addrlist;
                                    (*(*config).addr6).flags =
                                        0 as libc::c_int
                                }
                                if !(*config).addr6.is_null() &&
                                       (*(*config).addr6).next.is_null() &&
                                       (*(*config).addr6).flags &
                                           (16 as libc::c_int |
                                                8 as libc::c_int) == 0 {
                                    memcpy(&mut (*(*config).addr6).addr.addr6
                                               as *mut in6_addr as
                                               *mut libc::c_void,
                                           &mut (*crec).addr.addr6 as
                                               *mut in6_addr as
                                               *const libc::c_void,
                                           16 as libc::c_int as
                                               libc::c_ulong);
                                    (*config).flags |=
                                        (4096 as libc::c_int |
                                             16384 as libc::c_int) as
                                            libc::c_uint
                                }
                            } else {
                                inet_ntop(prot,
                                          &mut (*crec).addr as *mut all_addr
                                              as *const libc::c_void,
                                          (*dnsmasq_daemon).addrbuff,
                                          46 as libc::c_int as socklen_t);
                                my_syslog((3 as libc::c_int) <<
                                              3 as libc::c_int |
                                              4 as libc::c_int,
                                          b"duplicate IP address %s (%s) in dhcp-config directive\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          (*dnsmasq_daemon).addrbuff,
                                          (*config).hostname);
                            }
                        }
                    }
                }
                config = (*config).next
            }
        }
        if !(prot == 2 as libc::c_int) { break ; }
        prot = 10 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn whichdevice() -> *mut libc::c_char {
    /* If we are doing DHCP on exactly one interface, and running linux, do SO_BINDTODEVICE
     to that device. This is for the use case of  (eg) OpenStack, which runs a new
     dnsmasq instance for each VLAN interface it creates. Without the BINDTODEVICE, 
     individual processes don't always see the packets they should.
     SO_BINDTODEVICE is only available Linux. 

     Note that if wildcards are used in --interface, or --interface is not used at all,
     or a configured interface doesn't yet exist, then more interfaces may arrive later, 
     so we can't safely assert there is only one interface and proceed.
*/
    let mut iface: *mut irec = 0 as *mut irec;
    let mut found: *mut irec = 0 as *mut irec;
    let mut if_tmp: *mut iname = 0 as *mut iname;
    if (*dnsmasq_daemon).if_names.is_null() { return 0 as *mut libc::c_char }
    if_tmp = (*dnsmasq_daemon).if_names;
    while !if_tmp.is_null() {
        if !(*if_tmp).name.is_null() &&
               ((*if_tmp).used == 0 ||
                    !strchr((*if_tmp).name, '*' as i32).is_null()) {
            return 0 as *mut libc::c_char
        }
        if_tmp = (*if_tmp).next
    }
    found = 0 as *mut irec;
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).dhcp_ok != 0 {
            if found.is_null() {
                found = iface
            } else if strcmp((*found).name, (*iface).name) != 0 as libc::c_int
             {
                return 0 as *mut libc::c_char
            }
            /* more than one. */
        }
        iface = (*iface).next
    }
    if !found.is_null() { return (*found).name }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn bindtodevice(mut device: *mut libc::c_char,
                                      mut fd: libc::c_int) {
    let mut len: size_t =
        strlen(device).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if len > 16 as libc::c_int as libc::c_ulong {
        len = 16 as libc::c_int as size_t
    }
    /* only allowed by root. */
    if setsockopt(fd, 1 as libc::c_int, 25 as libc::c_int,
                  device as *const libc::c_void, len as socklen_t) ==
           -(1 as libc::c_int) && *__errno_location() != 1 as libc::c_int {
        die(b"failed to set SO_BINDTODEVICE on DHCP socket: %s\x00" as
                *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 2 as libc::c_int);
    };
}
static mut opttab: [opttab_t; 74] =
    [{
         let mut init =
             opttab_t{name:
                          b"netmask\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 1 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"time-offset\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 2 as libc::c_int as u16_0,
                      size: 4 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"router\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 3 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"dns-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 6 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"log-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 7 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"lpr-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 9 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"hostname\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 12 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x1000 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"boot-file-size\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 13 as libc::c_int as u16_0,
                      size:
                          (2 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"domain-name\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 15 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"swap-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 16 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"root-path\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 17 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"extension-path\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 18 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"ip-forward-enable\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 19 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"non-local-source-routing\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 20 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"policy-filter\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 21 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"max-datagram-reassembly\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 22 as libc::c_int as u16_0,
                      size:
                          (2 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"default-ttl\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 23 as libc::c_int as u16_0,
                      size:
                          (1 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"mtu\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 26 as libc::c_int as u16_0,
                      size:
                          (2 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"all-subnets-local\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 27 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"broadcast\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 28 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x8000 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"router-discovery\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 31 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"router-solicitation\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 32 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"static-route\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 33 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"trailer-encapsulation\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 34 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"arp-timeout\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 35 as libc::c_int as u16_0,
                      size:
                          (4 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"ethernet-encap\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 36 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"tcp-ttl\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 37 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"tcp-keepalive\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 38 as libc::c_int as u16_0,
                      size:
                          (4 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis-domain\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 40 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 41 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"ntp-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 42 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"vendor-encap\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 43 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"netbios-ns\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 44 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"netbios-dd\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 45 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"netbios-nodetype\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 46 as libc::c_int as u16_0,
                      size: 1 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"netbios-scope\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 47 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"x-windows-fs\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 48 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"x-windows-dm\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 49 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"requested-address\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 50 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x8000 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"lease-time\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 51 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x200 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"option-overload\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 52 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"message-type\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 53 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"server-identifier\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 54 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x8000 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"parameter-request\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 55 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"message\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 56 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"max-message-size\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 57 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"T1\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 58 as libc::c_int as u16_0,
                      size: 0x200 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"T2\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 59 as libc::c_int as u16_0,
                      size: 0x200 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"vendor-class\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 60 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"client-id\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 61 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis+-domain\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 64 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis+-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 65 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"tftp-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 66 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"bootfile-name\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 67 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"mobile-ip-home\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 68 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"smtp-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 69 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"pop3-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 70 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nntp-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 71 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"irc-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 74 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"user-class\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 77 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"rapid-commit\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 80 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"FQDN\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 81 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"agent-id\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 82 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"client-arch\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 93 as libc::c_int as u16_0,
                      size:
                          (2 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"client-interface-id\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 94 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"client-machine-id\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 97 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"subnet-select\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 118 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"domain-search\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 119 as libc::c_int as u16_0,
                      size: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"sip-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 120 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"classless-static-route\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 121 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"vendor-id-encap\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 125 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"tftp-server-address\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 150 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"server-ip-address\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 255 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name: 0 as *const libc::c_char as *mut libc::c_char,
                      val: 0 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     }];
static mut opttab6: [opttab_t; 28] =
    [{
         let mut init =
             opttab_t{name:
                          b"client-id\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 1 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"server-id\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 2 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"ia-na\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 3 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"ia-ta\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 4 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"iaaddr\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 5 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"oro\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 6 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"preference\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 7 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x400 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"unicast\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char,
                      val: 12 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"status\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 13 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"rapid-commit\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 14 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"user-class\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 15 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x800 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"vendor-class\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 16 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x800 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"vendor-opts\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 17 as libc::c_int as u16_0,
                      size: 0x2000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"sip-server-domain\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 21 as libc::c_int as u16_0,
                      size: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"sip-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 22 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"dns-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 23 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"domain-search\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 24 as libc::c_int as u16_0,
                      size: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 27 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis+-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 28 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis-domain\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 29 as libc::c_int as u16_0,
                      size: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"nis+-domain\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 30 as libc::c_int as u16_0,
                      size: 0x4000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"sntp-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 31 as libc::c_int as u16_0,
                      size: 0x8000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"information-refresh-time\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 32 as libc::c_int as u16_0,
                      size: 0x200 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"FQDN\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                      val: 39 as libc::c_int as u16_0,
                      size:
                          (0x2000 as libc::c_int | 0x4000 as libc::c_int) as
                              u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"ntp-server\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 56 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"bootfile-url\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 59 as libc::c_int as u16_0,
                      size: 0x1000 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name:
                          b"bootfile-param\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                      val: 60 as libc::c_int as u16_0,
                      size: 0x800 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             opttab_t{name: 0 as *const libc::c_char as *mut libc::c_char,
                      val: 0 as libc::c_int as u16_0,
                      size: 0 as libc::c_int as u16_0,};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn display_opts() {
    let mut i: libc::c_int = 0;
    printf(b"Known DHCP options:\n\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while !opttab[i as usize].name.is_null() {
        if opttab[i as usize].size as libc::c_int & 0x2000 as libc::c_int == 0
           {
            printf(b"%3d %s\n\x00" as *const u8 as *const libc::c_char,
                   opttab[i as usize].val as libc::c_int,
                   opttab[i as usize].name);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn display_opts6() {
    let mut i: libc::c_int = 0;
    printf(b"Known DHCPv6 options:\n\x00" as *const u8 as
               *const libc::c_char);
    i = 0 as libc::c_int;
    while !opttab6[i as usize].name.is_null() {
        if opttab6[i as usize].size as libc::c_int & 0x2000 as libc::c_int ==
               0 {
            printf(b"%3d %s\n\x00" as *const u8 as *const libc::c_char,
                   opttab6[i as usize].val as libc::c_int,
                   opttab6[i as usize].name);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn lookup_dhcp_opt(mut prot: libc::c_int,
                                         mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut t: *const opttab_t = 0 as *const opttab_t;
    let mut i: libc::c_int = 0;
    if prot == 10 as libc::c_int {
        t = opttab6.as_ptr()
    } else { t = opttab.as_ptr() }
    i = 0 as libc::c_int;
    while !(*t.offset(i as isize)).name.is_null() {
        if strcasecmp((*t.offset(i as isize)).name, name) == 0 as libc::c_int
           {
            return (*t.offset(i as isize)).val as libc::c_int
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn lookup_dhcp_len(mut prot: libc::c_int,
                                         mut val: libc::c_int)
 -> libc::c_int {
    let mut t: *const opttab_t = 0 as *const opttab_t;
    let mut i: libc::c_int = 0;
    if prot == 10 as libc::c_int {
        t = opttab6.as_ptr()
    } else { t = opttab.as_ptr() }
    i = 0 as libc::c_int;
    while !(*t.offset(i as isize)).name.is_null() {
        if val == (*t.offset(i as isize)).val as libc::c_int {
            return (*t.offset(i as isize)).size as libc::c_int &
                       !(0x400 as libc::c_int)
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn option_string(mut prot: libc::c_int,
                                       mut opt: libc::c_uint,
                                       mut val: *mut libc::c_uchar,
                                       mut opt_len: libc::c_int,
                                       mut buf: *mut libc::c_char,
                                       mut buf_len: libc::c_int)
 -> *mut libc::c_char {
    let mut o: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nodecode: libc::c_int = 0 as libc::c_int;
    let mut ot: *const opttab_t = opttab.as_ptr();
    if prot == 10 as libc::c_int { ot = opttab6.as_ptr() }
    o = 0 as libc::c_int;
    while !(*ot.offset(o as isize)).name.is_null() {
        if (*ot.offset(o as isize)).val as libc::c_uint == opt {
            if !buf.is_null() {
                memset(buf as *mut libc::c_void, 0 as libc::c_int,
                       buf_len as libc::c_ulong);
                if (*ot.offset(o as isize)).size as libc::c_int &
                       0x8000 as libc::c_int != 0 {
                    let mut addr: all_addr =
                        all_addr{addr4: in_addr{s_addr: 0,},};
                    let mut addr_len: libc::c_int = 4 as libc::c_int;
                    if prot == 10 as libc::c_int {
                        addr_len = 16 as libc::c_int
                    }
                    *buf.offset(0 as libc::c_int as isize) =
                        0 as libc::c_int as libc::c_char;
                    i = 0 as libc::c_int;
                    while i <= opt_len - addr_len {
                        if i != 0 as libc::c_int {
                            strncat(buf,
                                    b", \x00" as *const u8 as
                                        *const libc::c_char,
                                    (buf_len as
                                         libc::c_ulong).wrapping_sub(strlen(buf)));
                        }
                        /* align */
                        memcpy(&mut addr as *mut all_addr as
                                   *mut libc::c_void,
                               &mut *val.offset(i as isize) as
                                   *mut libc::c_uchar as *const libc::c_void,
                               addr_len as libc::c_ulong);
                        inet_ntop(prot,
                                  &mut *val.offset(i as isize) as
                                      *mut libc::c_uchar as
                                      *const libc::c_void,
                                  (*dnsmasq_daemon).addrbuff,
                                  46 as libc::c_int as socklen_t);
                        strncat(buf, (*dnsmasq_daemon).addrbuff,
                                (buf_len as
                                     libc::c_ulong).wrapping_sub(strlen(buf)));
                        i += addr_len
                    }
                } else if (*ot.offset(o as isize)).size as libc::c_int &
                              0x1000 as libc::c_int != 0 {
                    i = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    while i < opt_len && j < buf_len {
                        let mut c: libc::c_char =
                            *val.offset(i as isize) as libc::c_char;
                        if *(*__ctype_b_loc()).offset(c as libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISprint as libc::c_int as libc::c_ushort as
                                   libc::c_int != 0 {
                            let fresh6 = j;
                            j = j + 1;
                            *buf.offset(fresh6 as isize) = c
                        }
                        i += 1
                    }
                } else if (*ot.offset(o as isize)).size as libc::c_int &
                              0x4000 as libc::c_int != 0 &&
                              prot == 10 as libc::c_int {
                    i = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    while i < opt_len &&
                              *val.offset(i as isize) as libc::c_int !=
                                  0 as libc::c_int {
                        let mut k: libc::c_int = 0;
                        let mut l: libc::c_int =
                            i + *val.offset(i as isize) as libc::c_int +
                                1 as libc::c_int;
                        k = i + 1 as libc::c_int;
                        while k < opt_len && k < l && j < buf_len {
                            let mut c_0: libc::c_char =
                                *val.offset(k as isize) as libc::c_char;
                            if *(*__ctype_b_loc()).offset(c_0 as libc::c_int
                                                              as isize) as
                                   libc::c_int &
                                   _ISprint as libc::c_int as libc::c_ushort
                                       as libc::c_int != 0 {
                                let fresh7 = j;
                                j = j + 1;
                                *buf.offset(fresh7 as isize) = c_0
                            }
                            k += 1
                        }
                        i = l;
                        if *val.offset(i as isize) as libc::c_int !=
                               0 as libc::c_int && j < buf_len {
                            let fresh8 = j;
                            j = j + 1;
                            *buf.offset(fresh8 as isize) =
                                '.' as i32 as libc::c_char
                        }
                    }
                } else if (*ot.offset(o as isize)).size as libc::c_int &
                              0x800 as libc::c_int != 0 {
                    let mut k_0: libc::c_int = 0;
                    let mut len: libc::c_int = 0;
                    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    i = 0 as libc::c_int;
                    j = 0 as libc::c_int;
                    loop  {
                        p =
                            &mut *val.offset(i as isize) as
                                *mut libc::c_uchar;
                        let mut t_cp: *mut libc::c_uchar = p;
                        len =
                            (*t_cp.offset(0 as libc::c_int as isize) as u16_0
                                 as libc::c_int) << 8 as libc::c_int |
                                *t_cp.offset(1 as libc::c_int as isize) as
                                    u16_0 as libc::c_int;
                        p = p.offset(2 as libc::c_int as isize);
                        k_0 = 0 as libc::c_int;
                        while k_0 < len && j < buf_len {
                            let fresh9 = p;
                            p = p.offset(1);
                            let mut c_1: libc::c_char =
                                *fresh9 as libc::c_char;
                            if *(*__ctype_b_loc()).offset(c_1 as libc::c_int
                                                              as isize) as
                                   libc::c_int &
                                   _ISprint as libc::c_int as libc::c_ushort
                                       as libc::c_int != 0 {
                                let fresh10 = j;
                                j = j + 1;
                                *buf.offset(fresh10 as isize) = c_1
                            }
                            k_0 += 1
                        }
                        i += len + 2 as libc::c_int;
                        if i >= opt_len { break ; }
                        if j < buf_len {
                            let fresh11 = j;
                            j = j + 1;
                            *buf.offset(fresh11 as isize) =
                                ',' as i32 as libc::c_char
                        }
                    }
                } else if (*ot.offset(o as isize)).size as libc::c_int &
                              (0x400 as libc::c_int | 0x200 as libc::c_int) !=
                              0 && opt_len != 0 as libc::c_int {
                    let mut dec: libc::c_uint =
                        0 as libc::c_int as libc::c_uint;
                    i = 0 as libc::c_int;
                    while i < opt_len {
                        dec =
                            dec << 8 as libc::c_int |
                                *val.offset(i as isize) as libc::c_uint;
                        i += 1
                    }
                    if (*ot.offset(o as isize)).size as libc::c_int &
                           0x200 as libc::c_int != 0 {
                        prettyprint_time(buf, dec);
                    } else {
                        sprintf(buf,
                                b"%u\x00" as *const u8 as *const libc::c_char,
                                dec);
                    }
                } else { nodecode = 1 as libc::c_int }
            }
            break ;
        } else { o += 1 }
    }
    if opt_len != 0 as libc::c_int && !buf.is_null() &&
           ((*ot.offset(o as isize)).name.is_null() || nodecode != 0) {
        let mut trunc: libc::c_int = 0 as libc::c_int;
        if opt_len > 14 as libc::c_int {
            trunc = 1 as libc::c_int;
            opt_len = 14 as libc::c_int
        }
        print_mac(buf, val, opt_len);
        if trunc != 0 {
            strncat(buf, b"...\x00" as *const u8 as *const libc::c_char,
                    (buf_len as libc::c_ulong).wrapping_sub(strlen(buf)));
        }
    }
    return if !(*ot.offset(o as isize)).name.is_null() {
               (*ot.offset(o as isize)).name as *const libc::c_char
           } else { b"\x00" as *const u8 as *const libc::c_char } as
               *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn log_context(mut family: libc::c_int,
                                     mut context: *mut dhcp_context) {
    /* We don't handle compressed rfc1035 names, so no good in IPv4 land */
    /* Cannot use dhcp_buff* for RA contexts */
    let mut start: *mut libc::c_void =
        &mut (*context).start as *mut in_addr as *mut libc::c_void;
    let mut end: *mut libc::c_void =
        &mut (*context).end as *mut in_addr as *mut libc::c_void;
    let mut template: *mut libc::c_char =
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut p: *mut libc::c_char = (*dnsmasq_daemon).namebuff;
    *p = 0 as libc::c_int as libc::c_char;
    if family == 10 as libc::c_int {
        let mut subnet: in6_addr = (*context).start6;
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 10 as libc::c_int == 0 {
            setaddr6part(&mut subnet, 0 as libc::c_int as u64_0);
        }
        inet_ntop(10 as libc::c_int,
                  &mut subnet as *mut in6_addr as *const libc::c_void,
                  (*dnsmasq_daemon).addrbuff, 46 as libc::c_int as socklen_t);
        start = &mut (*context).start6 as *mut in6_addr as *mut libc::c_void;
        end = &mut (*context).end6 as *mut in6_addr as *mut libc::c_void
    }
    if family != 2 as libc::c_int &&
           (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 9 as libc::c_int != 0 {
        strcpy((*dnsmasq_daemon).namebuff,
               b", prefix deprecated\x00" as *const u8 as
                   *const libc::c_char);
    } else {
        p =
            p.offset(sprintf(p,
                             b", lease time \x00" as *const u8 as
                                 *const libc::c_char) as isize);
        prettyprint_time(p, (*context).lease_time);
        p = p.offset(strlen(p) as isize)
    }
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 11 as libc::c_int != 0 {
        let mut ifrn_name: [libc::c_char; 16] = [0; 16];
        template = p;
        p =
            p.offset(sprintf(p, b", \x00" as *const u8 as *const libc::c_char)
                         as isize);
        if indextoname((*dnsmasq_daemon).icmp6fd, (*context).if_index,
                       ifrn_name.as_mut_ptr()) != 0 {
            sprintf(p, b"%s for %s\x00" as *const u8 as *const libc::c_char,
                    if (*context).flags as libc::c_uint &
                           (1 as libc::c_uint) << 16 as libc::c_int != 0 {
                        b"old prefix\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"constructed\x00" as *const u8 as *const libc::c_char
                    }, ifrn_name.as_mut_ptr());
        }
    } else if (*context).flags as libc::c_uint &
                  (1 as libc::c_uint) << 10 as libc::c_int != 0 &&
                  (*context).flags as libc::c_uint &
                      (1 as libc::c_uint) << 7 as libc::c_int == 0 {
        template = p;
        p =
            p.offset(sprintf(p, b", \x00" as *const u8 as *const libc::c_char)
                         as isize);
        sprintf(p, b"template for %s\x00" as *const u8 as *const libc::c_char,
                (*context).template_interface);
    }
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 16 as libc::c_int == 0 &&
           ((*context).flags as libc::c_uint &
                (1 as libc::c_uint) << 8 as libc::c_int != 0 ||
                family == 2 as libc::c_int) {
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 7 as libc::c_int != 0 {
            if (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 10 as libc::c_int != 0 {
                strncpy((*dnsmasq_daemon).dhcp_buff,
                        (*context).template_interface,
                        256 as libc::c_int as libc::c_ulong);
            } else {
                strcpy((*dnsmasq_daemon).dhcp_buff,
                       (*dnsmasq_daemon).addrbuff);
            }
        } else {
            inet_ntop(family, start, (*dnsmasq_daemon).dhcp_buff,
                      256 as libc::c_int as socklen_t);
        }
        inet_ntop(family, end, (*dnsmasq_daemon).dhcp_buff3,
                  256 as libc::c_int as socklen_t);
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  if (*context).flags as libc::c_uint &
                         (1 as libc::c_uint) << 7 as libc::c_int != 0 {
                      b"%s stateless on %s%.0s%.0s%s\x00" as *const u8 as
                          *const libc::c_char
                  } else if (*context).flags as libc::c_uint &
                                (1 as libc::c_uint) << 0 as libc::c_int != 0 {
                      b"%s, static leases only on %.0s%s%s%.0s\x00" as
                          *const u8 as *const libc::c_char
                  } else if (*context).flags as libc::c_uint &
                                (1 as libc::c_uint) << 3 as libc::c_int != 0 {
                      b"%s, proxy on subnet %.0s%s%.0s%.0s\x00" as *const u8
                          as *const libc::c_char
                  } else {
                      b"%s, IP range %s -- %s%s%.0s\x00" as *const u8 as
                          *const libc::c_char
                  },
                  if family != 2 as libc::c_int {
                      b"DHCPv6\x00" as *const u8 as *const libc::c_char
                  } else { b"DHCP\x00" as *const u8 as *const libc::c_char },
                  (*dnsmasq_daemon).dhcp_buff, (*dnsmasq_daemon).dhcp_buff3,
                  (*dnsmasq_daemon).namebuff, template);
    }
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 10 as libc::c_int != 0 {
        strcpy((*dnsmasq_daemon).addrbuff, (*context).template_interface);
        template =
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if (*context).flags as libc::c_uint &
           (1 as libc::c_uint) << 6 as libc::c_int != 0 &&
           (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 16 as libc::c_int == 0 {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"DHCPv4-derived IPv6 names on %s%s\x00" as *const u8 as
                      *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                  template);
    }
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
               != 0 &&
               (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 8 as libc::c_int != 0 &&
               family == 10 as libc::c_int {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"router advertisement on %s%s\x00" as *const u8 as
                      *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                  template);
    };
}
#[no_mangle]
pub unsafe extern "C" fn log_relay(mut family: libc::c_int,
                                   mut relay: *mut dhcp_relay) {
    inet_ntop(family,
              &mut (*relay).local as *mut all_addr as *const libc::c_void,
              (*dnsmasq_daemon).addrbuff, 46 as libc::c_int as socklen_t);
    inet_ntop(family,
              &mut (*relay).server as *mut all_addr as *const libc::c_void,
              (*dnsmasq_daemon).namebuff, 46 as libc::c_int as socklen_t);
    if !(*relay).interface.is_null() {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"DHCP relay from %s to %s via %s\x00" as *const u8 as
                      *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                  (*dnsmasq_daemon).namebuff, (*relay).interface);
    } else {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"DHCP relay from %s to %s\x00" as *const u8 as
                      *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                  (*dnsmasq_daemon).namebuff);
    };
}
