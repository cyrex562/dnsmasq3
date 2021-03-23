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
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
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
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
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
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                 __oact: *mut sigaction) -> libc::c_int;
    #[no_mangle]
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn _exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn setuid(__uid: __uid_t) -> libc::c_int;
    #[no_mangle]
    fn setgid(__gid: __gid_t) -> libc::c_int;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    fn __uflow(_: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char)
     -> libc::c_double;
    #[no_mangle]
    fn send_event(fd: libc::c_int, event: libc::c_int, data: libc::c_int,
                  msg: *mut libc::c_char);
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn setgroups(__n: size_t, __groups: *const __gid_t) -> libc::c_int;
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
    fn legal_hostname(name: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn whine_malloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn read_write(fd: libc::c_int, packet: *mut libc::c_uchar,
                  size: libc::c_int, rw: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn close_fds(max_fd: libc::c_long, spare1: libc::c_int,
                 spare2: libc::c_int, spare3: libc::c_int);
    #[no_mangle]
    fn indextoname(fd: libc::c_int, index: libc::c_int,
                   name: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fix_fd(fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strtoll(_: *const libc::c_char, _: *mut *mut libc::c_char,
               _: libc::c_int) -> libc::c_longlong;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn setenv(__name: *const libc::c_char, __value: *const libc::c_char,
              __replace: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __blkcnt64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ino_t = __ino64_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_9,
    pub _timer: C2RustUnnamed_8,
    pub _rt: C2RustUnnamed_7,
    pub _sigchld: C2RustUnnamed_6,
    pub _sigfault: C2RustUnnamed_3,
    pub _sigpoll: C2RustUnnamed_2,
    pub _sigsys: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _addr_bnd: C2RustUnnamed_5,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _: *mut siginfo_t,
                                                  _: *mut libc::c_void)
                                 -> ()>,
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
    pub cname: C2RustUnnamed_15,
    pub key: C2RustUnnamed_14,
    pub ds: C2RustUnnamed_13,
    pub srv: C2RustUnnamed_12,
    pub log: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub keytag: libc::c_ushort,
    pub algo: libc::c_ushort,
    pub digest: libc::c_ushort,
    pub rcode: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
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
pub struct C2RustUnnamed_13 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
    pub digest: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub keydata: *mut blockdata,
    pub keylen: libc::c_ushort,
    pub flags: libc::c_ushort,
    pub keytag: libc::c_ushort,
    pub algo: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub target: C2RustUnnamed_16,
    pub uid: libc::c_uint,
    pub is_name_ptr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
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
    pub name: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
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
    pub u: C2RustUnnamed_18,
    pub val: *mut libc::c_uchar,
    pub netid: *mut dhcp_netid,
    pub next: *mut dhcp_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
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
pub struct script_data {
    pub flags: libc::c_int,
    pub action: libc::c_int,
    pub hwaddr_len: libc::c_int,
    pub hwaddr_type: libc::c_int,
    pub clid_len: libc::c_int,
    pub hostname_len: libc::c_int,
    pub ed_len: libc::c_int,
    pub addr: in_addr,
    pub giaddr: in_addr,
    pub remaining_time: libc::c_uint,
    pub expires: time_t,
    pub file_len: off_t,
    pub addr6: in6_addr,
    pub vendorclass_count: libc::c_int,
    pub iaid: libc::c_uint,
    pub hwaddr: [libc::c_uchar; 16],
    pub interface: [libc::c_char; 16],
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
unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
    return __x;
}
#[inline]
unsafe extern "C" fn stat64(mut __path: *const libc::c_char,
                            mut __statbuf: *mut stat64) -> libc::c_int {
    return __xstat64(1 as libc::c_int, __path, __statbuf);
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
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char)
 -> libc::c_longlong {
    return strtoll(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                   10 as libc::c_int);
}
static mut buf: *mut script_data =
    0 as *const script_data as *mut script_data;
static mut bytes_in_buf: size_t = 0 as libc::c_int as size_t;
static mut buf_size: size_t = 0 as libc::c_int as size_t;
#[no_mangle]
pub unsafe extern "C" fn create_helper(mut event_fd: libc::c_int,
                                       mut err_fd: libc::c_int,
                                       mut uid: uid_t, mut gid: gid_t,
                                       mut max_fd: libc::c_long)
 -> libc::c_int {
    let mut pid: pid_t = 0;
    let mut i: libc::c_int = 0;
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    let mut sigact: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_10{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    let mut alloc_buff: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* create the pipe through which the main program sends us commands,
     then fork our process. */
    if pipe(pipefd.as_mut_ptr()) == -(1 as libc::c_int) ||
           fix_fd(pipefd[1 as libc::c_int as usize]) == 0 ||
           {
               pid = fork(); /* close reader side */
               (pid) == -(1 as libc::c_int)
           } {
        send_event(err_fd, 10 as libc::c_int, *__errno_location(),
                   0 as *mut libc::c_char);
        _exit(0 as libc::c_int);
    }
    if pid != 0 as libc::c_int {
        close(pipefd[0 as libc::c_int as usize]);
        return pipefd[1 as libc::c_int as usize]
    }
    /* ignore SIGTERM and SIGINT, so that we can clean up when the main process gets hit
     and SIGALRM so that we can use sleep() */
    sigact.__sigaction_handler.sa_handler =
        ::std::mem::transmute::<libc::intptr_t,
                                __sighandler_t>(1 as libc::c_int as
                                                    libc::intptr_t);
    sigact.sa_flags = 0 as libc::c_int;
    sigemptyset(&mut sigact.sa_mask);
    sigaction(15 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    sigaction(14 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    sigaction(2 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    if (*dnsmasq_daemon).options[(6 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (6 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 && uid != 0 as libc::c_int as libc::c_uint {
        let mut dummy: gid_t = 0;
        if setgroups(0 as libc::c_int as size_t, &mut dummy) ==
               -(1 as libc::c_int) || setgid(gid) == -(1 as libc::c_int) ||
               setuid(uid) == -(1 as libc::c_int) {
            if (*dnsmasq_daemon).options[(16 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (16 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
                /* send error to daemon process if no-fork */
                send_event(event_fd, 11 as libc::c_int, *__errno_location(),
                           (*dnsmasq_daemon).scriptuser);
            } else {
                /* kill daemon */
                send_event(event_fd, 16 as libc::c_int, 0 as libc::c_int,
                           0 as *mut libc::c_char);
                /* return error */
                send_event(err_fd, 11 as libc::c_int, *__errno_location(),
                           (*dnsmasq_daemon).scriptuser);
            }
            _exit(0 as libc::c_int);
        }
    }
    /* close all the sockets etc, we don't need them here. 
     Don't close err_fd, in case the lua-init fails.
     Note that we have to do this before lua init
     so we don't close any lua fds. */
    close_fds(max_fd, pipefd[0 as libc::c_int as usize], event_fd, err_fd);
    /* All init done, close our copy of the error pipe, so that main process can return */
    if err_fd != -(1 as libc::c_int) { close(err_fd); }
    loop 
         /* loop here */
         {
        let mut data: script_data =
            script_data{flags: 0,
                        action: 0,
                        hwaddr_len: 0,
                        hwaddr_type: 0,
                        clid_len: 0,
                        hostname_len: 0,
                        ed_len: 0,
                        addr: in_addr{s_addr: 0,},
                        giaddr: in_addr{s_addr: 0,},
                        remaining_time: 0,
                        expires: 0,
                        file_len: 0,
                        addr6:
                            in6_addr{__in6_u:
                                         C2RustUnnamed{__u6_addr8:
                                                           [0; 16],},},
                        vendorclass_count: 0,
                        iaid: 0,
                        hwaddr: [0; 16],
                        interface: [0; 16],};
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut action_str: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut buf_0: *mut libc::c_uchar =
            (*dnsmasq_daemon).namebuff as *mut libc::c_uchar;
        let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut extradata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut is6: libc::c_int = 0;
        let mut err: libc::c_int = 0 as libc::c_int;
        let mut pipeout: [libc::c_int; 2] = [0; 2];
        /* Free rarely-allocated memory from previous iteration. */
        if !alloc_buff.is_null() {
            free(alloc_buff as *mut libc::c_void);
            alloc_buff = 0 as *mut libc::c_uchar
        }
        /* we read zero bytes when pipe closed: this is our signal to exit */
        if read_write(pipefd[0 as libc::c_int as usize],
                      &mut data as *mut script_data as *mut libc::c_uchar,
                      ::std::mem::size_of::<script_data>() as libc::c_ulong as
                          libc::c_int, 1 as libc::c_int) == 0 {
            _exit(0 as libc::c_int);
        }
        is6 =
            (data.flags & (64 as libc::c_int | 32 as libc::c_int) != 0) as
                libc::c_int;
        if data.action == 1 as libc::c_int {
            action_str =
                b"del\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else if data.action == 4 as libc::c_int {
            action_str =
                b"add\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else if data.action == 3 as libc::c_int ||
                      data.action == 2 as libc::c_int {
            action_str =
                b"old\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else if data.action == 5 as libc::c_int {
            action_str =
                b"tftp\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            is6 = (data.flags != 2 as libc::c_int) as libc::c_int
        } else if data.action == 6 as libc::c_int {
            action_str =
                b"arp-add\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            is6 = (data.flags != 2 as libc::c_int) as libc::c_int
        } else {
            if !(data.action == 7 as libc::c_int) { continue ; }
            action_str =
                b"arp-del\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            is6 = (data.flags != 2 as libc::c_int) as libc::c_int;
            data.action = 6 as libc::c_int
        }
        /* stringify MAC into dhcp_buff */
        p = (*dnsmasq_daemon).dhcp_buff;
        if data.hwaddr_type != 1 as libc::c_int ||
               data.hwaddr_len == 0 as libc::c_int {
            p =
                p.offset(sprintf(p,
                                 b"%.2x-\x00" as *const u8 as
                                     *const libc::c_char, data.hwaddr_type) as
                             isize)
        }
        i = 0 as libc::c_int;
        while i < data.hwaddr_len && i < 16 as libc::c_int {
            p =
                p.offset(sprintf(p,
                                 b"%.2x\x00" as *const u8 as
                                     *const libc::c_char,
                                 data.hwaddr[i as usize] as libc::c_int) as
                             isize);
            if i != data.hwaddr_len - 1 as libc::c_int {
                p =
                    p.offset(sprintf(p,
                                     b":\x00" as *const u8 as
                                         *const libc::c_char) as isize)
            }
            i += 1
        }
        /* supplied data may just exceed normal buffer (unlikely) */
        if data.hostname_len + data.ed_len + data.clid_len >
               1025 as libc::c_int &&
               {
                   buf_0 =
                       malloc((data.hostname_len + data.ed_len +
                                   data.clid_len) as libc::c_ulong) as
                           *mut libc::c_uchar;
                   alloc_buff = buf_0;
                   alloc_buff.is_null()
               } {
            continue ;
        }
        if read_write(pipefd[0 as libc::c_int as usize], buf_0,
                      data.hostname_len + data.ed_len + data.clid_len,
                      1 as libc::c_int) == 0 {
            continue ;
        }
        /* CLID into packet */
        p = (*dnsmasq_daemon).packet;
        i = 0 as libc::c_int;
        while i < data.clid_len {
            p =
                p.offset(sprintf(p,
                                 b"%.2x\x00" as *const u8 as
                                     *const libc::c_char,
                                 *buf_0.offset(i as isize) as libc::c_int) as
                             isize);
            if i != data.clid_len - 1 as libc::c_int {
                p =
                    p.offset(sprintf(p,
                                     b":\x00" as *const u8 as
                                         *const libc::c_char) as isize)
            }
            i += 1
        }
        if is6 != 0 {
            /* or IAID and server DUID for IPv6 */
            sprintf((*dnsmasq_daemon).dhcp_buff3,
                    b"%s%u\x00" as *const u8 as *const libc::c_char,
                    if data.flags & 64 as libc::c_int != 0 {
                        b"T\x00" as *const u8 as *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char },
                    data.iaid);
            p = (*dnsmasq_daemon).dhcp_packet.iov_base as *mut libc::c_char;
            i = 0 as libc::c_int;
            while i < (*dnsmasq_daemon).duid_len {
                p =
                    p.offset(sprintf(p,
                                     b"%.2x\x00" as *const u8 as
                                         *const libc::c_char,
                                     *(*dnsmasq_daemon).duid.offset(i as
                                                                        isize)
                                         as libc::c_int) as isize);
                if i != (*dnsmasq_daemon).duid_len - 1 as libc::c_int {
                    p =
                        p.offset(sprintf(p,
                                         b":\x00" as *const u8 as
                                             *const libc::c_char) as isize)
                }
                i += 1
            }
        }
        buf_0 = buf_0.offset(data.clid_len as isize);
        if data.hostname_len != 0 as libc::c_int {
            let mut dot: *mut libc::c_char = 0 as *mut libc::c_char;
            hostname = buf_0 as *mut libc::c_char;
            *hostname.offset((data.hostname_len - 1 as libc::c_int) as isize)
                = 0 as libc::c_int as libc::c_char;
            if data.action != 5 as libc::c_int {
                if legal_hostname(hostname) == 0 {
                    hostname = 0 as *mut libc::c_char
                } else {
                    dot = strchr(hostname, '.' as i32);
                    if !dot.is_null() {
                        domain = dot.offset(1 as libc::c_int as isize);
                        *dot = 0 as libc::c_int as libc::c_char
                    }
                }
            }
        }
        extradata = buf_0.offset(data.hostname_len as isize);
        if is6 == 0 {
            inet_ntop(2 as libc::c_int,
                      &mut data.addr as *mut in_addr as *const libc::c_void,
                      (*dnsmasq_daemon).addrbuff,
                      46 as libc::c_int as socklen_t);
        } else {
            inet_ntop(10 as libc::c_int,
                      &mut data.addr6 as *mut in6_addr as *const libc::c_void,
                      (*dnsmasq_daemon).addrbuff,
                      46 as libc::c_int as socklen_t);
        }
        /* file length */
        if data.action == 5 as libc::c_int {
            sprintf(if is6 != 0 {
                        (*dnsmasq_daemon).packet
                    } else { (*dnsmasq_daemon).dhcp_buff },
                    b"%lu\x00" as *const u8 as *const libc::c_char,
                    data.file_len as libc::c_ulong);
        }
        /* no script, just lua */
        if (*dnsmasq_daemon).lease_change_command.is_null() { continue ; }
        /* Pipe to capture stdout and stderr from script */
        if (*dnsmasq_daemon).options[(6 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (6 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 && pipe(pipeout.as_mut_ptr()) == -(1 as libc::c_int) {
            continue ;
        }
        loop 
             /* possible fork errors are all temporary resource problems */
             {
            pid = fork();
            if !(pid == -(1 as libc::c_int) &&
                     (*__errno_location() == 11 as libc::c_int ||
                          *__errno_location() == 12 as libc::c_int)) {
                break ;
            }
            sleep(2 as libc::c_int as libc::c_uint);
        }
        if pid == -(1 as libc::c_int) {
            if (*dnsmasq_daemon).options[(6 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (6 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   == 0 {
                close(pipeout[0 as libc::c_int as usize]);
                close(pipeout[1 as libc::c_int as usize]);
            }
        } else if pid != 0 as libc::c_int {
            if (*dnsmasq_daemon).options[(6 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (6 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   == 0 {
                let mut fp: *mut FILE = 0 as *mut FILE;
                close(pipeout[1 as libc::c_int as usize]);
                /* wait for child to complete */
                /* Read lines sent to stdout/err by the script and pass them back to be logged */
                fp =
                    fdopen(pipeout[0 as libc::c_int as usize],
                           b"r\x00" as *const u8 as *const libc::c_char);
                if fp.is_null() {
                    close(pipeout[0 as libc::c_int as usize]);
                } else {
                    while !fgets((*dnsmasq_daemon).packet,
                                 (*dnsmasq_daemon).packet_buff_sz,
                                 fp).is_null() {
                        /* do not include new lines, log will append them */
                        let mut len: size_t =
                            strlen((*dnsmasq_daemon).packet);
                        if len > 0 as libc::c_int as libc::c_ulong {
                            len = len.wrapping_sub(1);
                            if *(*dnsmasq_daemon).packet.offset(len as isize)
                                   as libc::c_int == '\n' as i32 {
                                *(*dnsmasq_daemon).packet.offset(len as isize)
                                    = 0 as libc::c_int as libc::c_char
                            }
                        }
                        send_event(event_fd, 25 as libc::c_int,
                                   0 as libc::c_int,
                                   (*dnsmasq_daemon).packet);
                    }
                    fclose(fp);
                }
            }
            loop 
                 /* reap our children's children, if necessary */
                 {
                let mut status: libc::c_int = 0;
                let mut rc: pid_t = wait(&mut status);
                if rc == pid {
                    /* On error send event back to main process for logging */
                    if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as
                           libc::c_schar as libc::c_int >> 1 as libc::c_int >
                           0 as libc::c_int {
                        send_event(event_fd, 8 as libc::c_int,
                                   status & 0x7f as libc::c_int,
                                   0 as *mut libc::c_char);
                    } else if status & 0x7f as libc::c_int == 0 as libc::c_int
                                  &&
                                  (status & 0xff00 as libc::c_int) >>
                                      8 as libc::c_int != 0 as libc::c_int {
                        send_event(event_fd, 7 as libc::c_int,
                                   (status & 0xff00 as libc::c_int) >>
                                       8 as libc::c_int,
                                   0 as *mut libc::c_char);
                    }
                    break ;
                } else if rc == -(1 as libc::c_int) &&
                              *__errno_location() != 4 as libc::c_int {
                    break ;
                }
            }
        } else {
            if (*dnsmasq_daemon).options[(6 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (6 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   == 0 {
                /* map stdout/stderr of script to pipeout */
                close(pipeout[0 as libc::c_int as usize]);
                dup2(pipeout[1 as libc::c_int as usize], 1 as libc::c_int);
                dup2(pipeout[1 as libc::c_int as usize], 2 as libc::c_int);
                close(pipeout[1 as libc::c_int as usize]);
            }
            if data.action != 5 as libc::c_int &&
                   data.action != 6 as libc::c_int {
                my_setenv(b"DNSMASQ_IAID\x00" as *const u8 as
                              *const libc::c_char,
                          if is6 != 0 {
                              (*dnsmasq_daemon).dhcp_buff3
                          } else { 0 as *mut libc::c_char }, &mut err);
                my_setenv(b"DNSMASQ_SERVER_DUID\x00" as *const u8 as
                              *const libc::c_char,
                          if is6 != 0 {
                              (*dnsmasq_daemon).dhcp_packet.iov_base
                          } else { 0 as *mut libc::c_void } as
                              *const libc::c_char, &mut err);
                my_setenv(b"DNSMASQ_MAC\x00" as *const u8 as
                              *const libc::c_char,
                          if is6 != 0 && data.hwaddr_len != 0 as libc::c_int {
                              (*dnsmasq_daemon).dhcp_buff
                          } else { 0 as *mut libc::c_char }, &mut err);
                my_setenv(b"DNSMASQ_CLIENT_ID\x00" as *const u8 as
                              *const libc::c_char,
                          if is6 == 0 && data.clid_len != 0 as libc::c_int {
                              (*dnsmasq_daemon).packet
                          } else { 0 as *mut libc::c_char }, &mut err);
                my_setenv(b"DNSMASQ_INTERFACE\x00" as *const u8 as
                              *const libc::c_char,
                          if strlen(data.interface.as_mut_ptr()) !=
                                 0 as libc::c_int as libc::c_ulong {
                              data.interface.as_mut_ptr()
                          } else { 0 as *mut libc::c_char }, &mut err);
                sprintf((*dnsmasq_daemon).dhcp_buff2,
                        b"%lu\x00" as *const u8 as *const libc::c_char,
                        data.expires as libc::c_ulong);
                my_setenv(b"DNSMASQ_LEASE_EXPIRES\x00" as *const u8 as
                              *const libc::c_char,
                          (*dnsmasq_daemon).dhcp_buff2, &mut err);
                my_setenv(b"DNSMASQ_DOMAIN\x00" as *const u8 as
                              *const libc::c_char, domain, &mut err);
                end = extradata.offset(data.ed_len as isize);
                buf_0 = extradata;
                if is6 == 0 {
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_VENDOR_CLASS\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err)
                } else if data.vendorclass_count != 0 as libc::c_int {
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_VENDOR_CLASS_ID\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    i = 0 as libc::c_int;
                    while i < data.vendorclass_count - 1 as libc::c_int {
                        sprintf((*dnsmasq_daemon).dhcp_buff2,
                                b"DNSMASQ_VENDOR_CLASS%i\x00" as *const u8 as
                                    *const libc::c_char, i);
                        buf_0 =
                            grab_extradata(buf_0, end,
                                           (*dnsmasq_daemon).dhcp_buff2,
                                           &mut err);
                        i += 1
                    }
                }
                buf_0 =
                    grab_extradata(buf_0, end,
                                   b"DNSMASQ_SUPPLIED_HOSTNAME\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char, &mut err);
                if is6 == 0 {
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_CPEWAN_OUI\x00" as *const u8
                                           as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_CPEWAN_SERIAL\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_CPEWAN_CLASS\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_CIRCUIT_ID\x00" as *const u8
                                           as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_SUBSCRIBER_ID\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_REMOTE_ID\x00" as *const u8
                                           as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_REQUESTED_OPTIONS\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err)
                }
                buf_0 =
                    grab_extradata(buf_0, end,
                                   b"DNSMASQ_TAGS\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, &mut err);
                if is6 != 0 {
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_RELAY_ADDRESS\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err)
                } else {
                    my_setenv(b"DNSMASQ_RELAY_ADDRESS\x00" as *const u8 as
                                  *const libc::c_char,
                              if data.giaddr.s_addr !=
                                     0 as libc::c_int as libc::c_uint {
                                  inet_ntoa(data.giaddr)
                              } else { 0 as *mut libc::c_char }, &mut err);
                }
                i = 0 as libc::c_int;
                while !buf_0.is_null() {
                    sprintf((*dnsmasq_daemon).dhcp_buff2,
                            b"DNSMASQ_USER_CLASS%i\x00" as *const u8 as
                                *const libc::c_char, i);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       (*dnsmasq_daemon).dhcp_buff2,
                                       &mut err);
                    i += 1
                }
                sprintf((*dnsmasq_daemon).dhcp_buff2,
                        b"%u\x00" as *const u8 as *const libc::c_char,
                        data.remaining_time);
                my_setenv(b"DNSMASQ_TIME_REMAINING\x00" as *const u8 as
                              *const libc::c_char,
                          if data.action != 1 as libc::c_int &&
                                 data.remaining_time !=
                                     0 as libc::c_int as libc::c_uint {
                              (*dnsmasq_daemon).dhcp_buff2
                          } else { 0 as *mut libc::c_char }, &mut err);
                my_setenv(b"DNSMASQ_OLD_HOSTNAME\x00" as *const u8 as
                              *const libc::c_char,
                          if data.action == 2 as libc::c_int {
                              hostname
                          } else { 0 as *mut libc::c_char }, &mut err);
                if data.action == 2 as libc::c_int {
                    hostname = 0 as *mut libc::c_char
                }
                my_setenv(b"DNSMASQ_LOG_DHCP\x00" as *const u8 as
                              *const libc::c_char,
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
                              b"1\x00" as *const u8 as *const libc::c_char
                          } else { 0 as *const libc::c_char }, &mut err);
            }
            /* we need to have the event_fd around if exec fails */
            i = fcntl(event_fd, 1 as libc::c_int);
            if i != -(1 as libc::c_int) {
                fcntl(event_fd, 2 as libc::c_int, i | 1 as libc::c_int);
            }
            close(pipefd[0 as libc::c_int as usize]);
            p = strrchr((*dnsmasq_daemon).lease_change_command, '/' as i32);
            if err == 0 as libc::c_int {
                execl((*dnsmasq_daemon).lease_change_command,
                      if !p.is_null() {
                          p.offset(1 as libc::c_int as isize)
                      } else { (*dnsmasq_daemon).lease_change_command },
                      action_str,
                      if is6 != 0 && data.action != 6 as libc::c_int {
                          (*dnsmasq_daemon).packet
                      } else { (*dnsmasq_daemon).dhcp_buff },
                      (*dnsmasq_daemon).addrbuff, hostname,
                      0 as *mut libc::c_void as *mut libc::c_char);
                err = *__errno_location()
            }
            /* failed, send event so the main process logs the problem */
            send_event(event_fd, 9 as libc::c_int, err,
                       0 as *mut libc::c_char);
            _exit(0 as libc::c_int);
        }
    };
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
/* This file has code to fork a helper process which receives data via a pipe 
   shared with the main process and which is responsible for calling a script when
   DHCP leases change.

   The helper process is forked before the main process drops root, so it retains root 
   privs to pass on to the script. For this reason it tries to be paranoid about 
   data received from the main process, in case that has been compromised. We don't
   want the helper to give an attacker root. In particular, the script to be run is
   not settable via the pipe, once the fork has taken place it is not alterable by the 
   main process.
*/
unsafe extern "C" fn my_setenv(mut name: *const libc::c_char,
                               mut value: *const libc::c_char,
                               mut error: *mut libc::c_int) {
    if *error == 0 as libc::c_int {
        if value.is_null() {
            unsetenv(name);
        } else if setenv(name, value, 1 as libc::c_int) != 0 as libc::c_int {
            *error = *__errno_location()
        }
    };
}
unsafe extern "C" fn grab_extradata(mut buf_0: *mut libc::c_uchar,
                                    mut end: *mut libc::c_uchar,
                                    mut env: *mut libc::c_char,
                                    mut err: *mut libc::c_int)
 -> *mut libc::c_uchar {
    let mut next: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    if !buf_0.is_null() && buf_0 != end {
        next = buf_0;
        loop  {
            if next == end {
                next = 0 as *mut libc::c_uchar;
                break ;
            } else {
                if *next as libc::c_int == 0 as libc::c_int { break ; }
                next = next.offset(1)
            }
        }
        if !next.is_null() && next != buf_0 {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            /* No "=" in value */
            p = strchr(buf_0 as *mut libc::c_char, '=' as i32);
            if !p.is_null() { *p = 0 as libc::c_int as libc::c_char }
            val = buf_0 as *mut libc::c_char
        }
    }
    my_setenv(env, val, err);
    return if !next.is_null() {
               next.offset(1 as libc::c_int as isize)
           } else { 0 as *mut libc::c_uchar };
}
unsafe extern "C" fn buff_alloc(mut size: size_t) {
    if size > buf_size {
        let mut new: *mut script_data = 0 as *mut script_data;
        /* start with reasonable size, will almost never need extending. */
        if size <
               (::std::mem::size_of::<script_data>() as
                    libc::c_ulong).wrapping_add(200 as libc::c_int as
                                                    libc::c_ulong) {
            size =
                (::std::mem::size_of::<script_data>() as
                     libc::c_ulong).wrapping_add(200 as libc::c_int as
                                                     libc::c_ulong)
        }
        new = whine_malloc(size) as *mut script_data;
        if new.is_null() { return }
        if !buf.is_null() { free(buf as *mut libc::c_void); }
        buf = new;
        buf_size = size
    };
}
/* pack up lease data into a buffer */
#[no_mangle]
pub unsafe extern "C" fn queue_script(mut action: libc::c_int,
                                      mut lease: *mut dhcp_lease,
                                      mut hostname: *mut libc::c_char,
                                      mut now: time_t) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut hostname_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut clid_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ed_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut fd: libc::c_int = (*dnsmasq_daemon).dhcpfd;
    if (*dnsmasq_daemon).dhcp.is_null() { fd = (*dnsmasq_daemon).dhcp6fd }
    /* no script */
    if (*dnsmasq_daemon).helperfd == -(1 as libc::c_int) { return }
    if !(*lease).extradata.is_null() { ed_len = (*lease).extradata_len }
    if !(*lease).clid.is_null() {
        clid_len = (*lease).clid_len as libc::c_uint
    }
    if !hostname.is_null() {
        hostname_len =
            strlen(hostname).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_uint
    }
    buff_alloc((::std::mem::size_of::<script_data>() as
                    libc::c_ulong).wrapping_add(clid_len as
                                                    libc::c_ulong).wrapping_add(ed_len
                                                                                    as
                                                                                    libc::c_ulong).wrapping_add(hostname_len
                                                                                                                    as
                                                                                                                    libc::c_ulong));
    (*buf).action = action;
    (*buf).flags = (*lease).flags;
    (*buf).vendorclass_count = (*lease).vendorclass_count;
    (*buf).addr6 = (*lease).addr6;
    (*buf).iaid = (*lease).iaid;
    (*buf).hwaddr_len = (*lease).hwaddr_len;
    (*buf).hwaddr_type = (*lease).hwaddr_type;
    (*buf).clid_len = clid_len as libc::c_int;
    (*buf).ed_len = ed_len as libc::c_int;
    (*buf).hostname_len = hostname_len as libc::c_int;
    (*buf).addr = (*lease).addr;
    (*buf).giaddr = (*lease).giaddr;
    memcpy((*buf).hwaddr.as_mut_ptr() as *mut libc::c_void,
           (*lease).hwaddr.as_mut_ptr() as *const libc::c_void,
           16 as libc::c_int as libc::c_ulong);
    if indextoname(fd, (*lease).last_interface, (*buf).interface.as_mut_ptr())
           == 0 {
        (*buf).interface[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char
    }
    (*buf).expires = (*lease).expires;
    if (*lease).expires != 0 as libc::c_int as libc::c_long {
        (*buf).remaining_time =
            difftime((*lease).expires, now) as libc::c_uint
    } else { (*buf).remaining_time = 0 as libc::c_int as libc::c_uint }
    p = buf.offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
    if clid_len != 0 as libc::c_int as libc::c_uint {
        memcpy(p as *mut libc::c_void, (*lease).clid as *const libc::c_void,
               clid_len as libc::c_ulong);
        p = p.offset(clid_len as isize)
    }
    if hostname_len != 0 as libc::c_int as libc::c_uint {
        memcpy(p as *mut libc::c_void, hostname as *const libc::c_void,
               hostname_len as libc::c_ulong);
        p = p.offset(hostname_len as isize)
    }
    if ed_len != 0 as libc::c_int as libc::c_uint {
        memcpy(p as *mut libc::c_void,
               (*lease).extradata as *const libc::c_void,
               ed_len as libc::c_ulong);
        p = p.offset(ed_len as isize)
    }
    bytes_in_buf =
        p.wrapping_offset_from(buf as *mut libc::c_uchar) as libc::c_long as
            size_t;
}
/* This nastily re-uses DHCP-fields for TFTP stuff */
#[no_mangle]
pub unsafe extern "C" fn queue_tftp(mut file_len: off_t,
                                    mut filename: *mut libc::c_char,
                                    mut peer: *mut mysockaddr) {
    let mut filename_len: libc::c_uint = 0;
    /* no script */
    if (*dnsmasq_daemon).helperfd == -(1 as libc::c_int) { return }
    filename_len =
        strlen(filename).wrapping_add(1 as libc::c_int as libc::c_ulong) as
            libc::c_uint;
    buff_alloc((::std::mem::size_of::<script_data>() as
                    libc::c_ulong).wrapping_add(filename_len as
                                                    libc::c_ulong));
    memset(buf as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<script_data>() as libc::c_ulong);
    (*buf).action = 5 as libc::c_int;
    (*buf).hostname_len = filename_len as libc::c_int;
    (*buf).file_len = file_len;
    (*buf).flags = (*peer).sa.sa_family as libc::c_int;
    if (*buf).flags == 2 as libc::c_int {
        (*buf).addr = (*peer).in_0.sin_addr
    } else { (*buf).addr6 = (*peer).in6.sin6_addr }
    memcpy(buf.offset(1 as libc::c_int as isize) as *mut libc::c_uchar as
               *mut libc::c_void, filename as *const libc::c_void,
           filename_len as libc::c_ulong);
    bytes_in_buf =
        (::std::mem::size_of::<script_data>() as
             libc::c_ulong).wrapping_add(filename_len as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn queue_arp(mut action: libc::c_int,
                                   mut mac: *mut libc::c_uchar,
                                   mut maclen: libc::c_int,
                                   mut family: libc::c_int,
                                   mut addr: *mut all_addr) {
    /* no script */
    if (*dnsmasq_daemon).helperfd == -(1 as libc::c_int) { return }
    buff_alloc(::std::mem::size_of::<script_data>() as libc::c_ulong);
    memset(buf as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<script_data>() as libc::c_ulong);
    (*buf).action = action;
    (*buf).hwaddr_len = maclen;
    (*buf).hwaddr_type = 1 as libc::c_int;
    (*buf).flags = family;
    if (*buf).flags == 2 as libc::c_int {
        (*buf).addr = (*addr).addr4
    } else { (*buf).addr6 = (*addr).addr6 }
    memcpy((*buf).hwaddr.as_mut_ptr() as *mut libc::c_void,
           mac as *const libc::c_void, maclen as libc::c_ulong);
    bytes_in_buf = ::std::mem::size_of::<script_data>() as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn helper_buf_empty() -> libc::c_int {
    return (bytes_in_buf == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn helper_write() {
    let mut rc: ssize_t = 0;
    if bytes_in_buf == 0 as libc::c_int as libc::c_ulong { return }
    rc =
        write((*dnsmasq_daemon).helperfd, buf as *const libc::c_void,
              bytes_in_buf);
    if rc != -(1 as libc::c_int) as libc::c_long {
        if bytes_in_buf != rc as size_t {
            memmove(buf as *mut libc::c_void,
                    buf.offset(rc as isize) as *const libc::c_void,
                    bytes_in_buf.wrapping_sub(rc as libc::c_ulong));
        }
        bytes_in_buf =
            (bytes_in_buf as libc::c_ulong).wrapping_sub(rc as libc::c_ulong)
                as size_t as size_t
    } else {
        if *__errno_location() == 11 as libc::c_int ||
               *__errno_location() == 4 as libc::c_int {
            return
        }
        bytes_in_buf = 0 as libc::c_int as size_t
    };
}
