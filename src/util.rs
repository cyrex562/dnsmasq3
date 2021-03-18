#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
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
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
                  __delimiter: libc::c_int, __stream: *mut FILE) -> __ssize_t;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn if_indextoname(__ifindex: libc::c_uint, __ifname: *mut libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strtoll(_: *const libc::c_char, _: *mut *mut libc::c_char,
               _: libc::c_int) -> libc::c_longlong;
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
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
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
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn dirfd(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn die(message: *mut libc::c_char, arg1: *mut libc::c_char,
           exit_code: libc::c_int) -> !;
    #[no_mangle]
    fn my_syslog(priority: libc::c_int, format: *const libc::c_char, _: ...);
    #[no_mangle]
    fn fix_fd(fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn uname(__name: *mut utsname) -> libc::c_int;
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
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __blkcnt64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type __gwchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub union mysockaddr {
    pub sa: sockaddr,
    pub in_0: sockaddr_in,
    pub in6: sockaddr_in6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
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
unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
    return __x;
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
/* The SURF random number generator was taken from djbdns-1.05, by 
   Daniel J Bernstein, which is public domain. */
/* SURF random number generator */
static mut seed: [u32_0; 32] = [0; 32];
static mut in_0: [u32_0; 12] = [0; 12];
static mut out: [u32_0; 8] = [0; 8];
static mut outleft: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn rand_init() {
    let mut fd: libc::c_int =
        open(b"/dev/urandom\x00" as *const u8 as *const libc::c_char,
             0 as libc::c_int);
    if fd == -(1 as libc::c_int) ||
           read_write(fd, &mut seed as *mut [u32_0; 32] as *mut libc::c_uchar,
                      ::std::mem::size_of::<[u32_0; 32]>() as libc::c_ulong as
                          libc::c_int, 1 as libc::c_int) == 0 ||
           read_write(fd, &mut in_0 as *mut [u32_0; 12] as *mut libc::c_uchar,
                      ::std::mem::size_of::<[u32_0; 12]>() as libc::c_ulong as
                          libc::c_int, 1 as libc::c_int) == 0 {
        die(b"failed to seed the random number generator: %s\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 5 as libc::c_int);
    }
    close(fd);
}
unsafe extern "C" fn surf() {
    let mut t: [u32_0; 12] = [0; 12];
    let mut x: u32_0 = 0;
    let mut sum: u32_0 = 0 as libc::c_int as u32_0;
    let mut r: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut loop_0: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 12 as libc::c_int {
        t[i as usize] =
            in_0[i as usize] ^ seed[(12 as libc::c_int + i) as usize];
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        out[i as usize] = seed[(24 as libc::c_int + i) as usize];
        i += 1
    }
    x = t[11 as libc::c_int as usize];
    loop_0 = 0 as libc::c_int;
    while loop_0 < 2 as libc::c_int {
        r = 0 as libc::c_int;
        while r < 16 as libc::c_int {
            sum =
                (sum as libc::c_uint).wrapping_add(0x9e3779b9 as libc::c_uint)
                    as u32_0 as u32_0;
            t[0 as libc::c_int as usize] =
                (t[0 as libc::c_int as usize] as
                     libc::c_uint).wrapping_add((x ^
                                                     seed[0 as libc::c_int as
                                                              usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 5 as libc::c_int |
                                                         x >>
                                                             32 as libc::c_int
                                                                 -
                                                                 5 as
                                                                     libc::c_int))
                    as u32_0 as u32_0;
            x = t[0 as libc::c_int as usize];
            t[1 as libc::c_int as usize] =
                (t[1 as libc::c_int as usize] as
                     libc::c_uint).wrapping_add((x ^
                                                     seed[1 as libc::c_int as
                                                              usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 7 as libc::c_int |
                                                         x >>
                                                             32 as libc::c_int
                                                                 -
                                                                 7 as
                                                                     libc::c_int))
                    as u32_0 as u32_0;
            x = t[1 as libc::c_int as usize];
            t[2 as libc::c_int as usize] =
                (t[2 as libc::c_int as usize] as
                     libc::c_uint).wrapping_add((x ^
                                                     seed[2 as libc::c_int as
                                                              usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 9 as libc::c_int |
                                                         x >>
                                                             32 as libc::c_int
                                                                 -
                                                                 9 as
                                                                     libc::c_int))
                    as u32_0 as u32_0;
            x = t[2 as libc::c_int as usize];
            t[3 as libc::c_int as usize] =
                (t[3 as libc::c_int as usize] as
                     libc::c_uint).wrapping_add((x ^
                                                     seed[3 as libc::c_int as
                                                              usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 13 as libc::c_int |
                                                         x >>
                                                             32 as libc::c_int
                                                                 -
                                                                 13 as
                                                                     libc::c_int))
                    as u32_0 as u32_0;
            x = t[3 as libc::c_int as usize];
            t[4 as libc::c_int as usize] =
                (t[4 as libc::c_int as usize] as
                     libc::c_uint).wrapping_add((x ^
                                                     seed[4 as libc::c_int as
                                                              usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 5 as libc::c_int |
                                                         x >>
                                                             32 as libc::c_int
                                                                 -
                                                                 5 as
                                                                     libc::c_int))
                    as u32_0 as u32_0;
            x = t[4 as libc::c_int as usize];
            t[5 as libc::c_int as usize] =
                (t[5 as libc::c_int as usize] as
                     libc::c_uint).wrapping_add((x ^
                                                     seed[5 as libc::c_int as
                                                              usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 7 as libc::c_int |
                                                         x >>
                                                             32 as libc::c_int
                                                                 -
                                                                 7 as
                                                                     libc::c_int))
                    as u32_0 as u32_0;
            x = t[5 as libc::c_int as usize];
            t[6 as libc::c_int as usize] =
                (t[6 as libc::c_int as usize] as
                     libc::c_uint).wrapping_add((x ^
                                                     seed[6 as libc::c_int as
                                                              usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 9 as libc::c_int |
                                                         x >>
                                                             32 as libc::c_int
                                                                 -
                                                                 9 as
                                                                     libc::c_int))
                    as u32_0 as u32_0;
            x = t[6 as libc::c_int as usize];
            t[7 as libc::c_int as usize] =
                (t[7 as libc::c_int as usize] as
                     libc::c_uint).wrapping_add((x ^
                                                     seed[7 as libc::c_int as
                                                              usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 13 as libc::c_int |
                                                         x >>
                                                             32 as libc::c_int
                                                                 -
                                                                 13 as
                                                                     libc::c_int))
                    as u32_0 as u32_0;
            x = t[7 as libc::c_int as usize];
            t[8 as libc::c_int as usize] =
                (t[8 as libc::c_int as usize] as
                     libc::c_uint).wrapping_add((x ^
                                                     seed[8 as libc::c_int as
                                                              usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 5 as libc::c_int |
                                                         x >>
                                                             32 as libc::c_int
                                                                 -
                                                                 5 as
                                                                     libc::c_int))
                    as u32_0 as u32_0;
            x = t[8 as libc::c_int as usize];
            t[9 as libc::c_int as usize] =
                (t[9 as libc::c_int as usize] as
                     libc::c_uint).wrapping_add((x ^
                                                     seed[9 as libc::c_int as
                                                              usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 7 as libc::c_int |
                                                         x >>
                                                             32 as libc::c_int
                                                                 -
                                                                 7 as
                                                                     libc::c_int))
                    as u32_0 as u32_0;
            x = t[9 as libc::c_int as usize];
            t[10 as libc::c_int as usize] =
                (t[10 as libc::c_int as usize] as
                     libc::c_uint).wrapping_add((x ^
                                                     seed[10 as libc::c_int as
                                                              usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 9 as libc::c_int |
                                                         x >>
                                                             32 as libc::c_int
                                                                 -
                                                                 9 as
                                                                     libc::c_int))
                    as u32_0 as u32_0;
            x = t[10 as libc::c_int as usize];
            t[11 as libc::c_int as usize] =
                (t[11 as libc::c_int as usize] as
                     libc::c_uint).wrapping_add((x ^
                                                     seed[11 as libc::c_int as
                                                              usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 13 as libc::c_int |
                                                         x >>
                                                             32 as libc::c_int
                                                                 -
                                                                 13 as
                                                                     libc::c_int))
                    as u32_0 as u32_0;
            x = t[11 as libc::c_int as usize];
            r += 1
        }
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            out[i as usize] ^= t[(i + 4 as libc::c_int) as usize];
            i += 1
        }
        loop_0 += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn rand16() -> libc::c_ushort {
    if outleft == 0 {
        in_0[0 as libc::c_int as usize] =
            in_0[0 as libc::c_int as usize].wrapping_add(1);
        if in_0[0 as libc::c_int as usize] == 0 {
            in_0[1 as libc::c_int as usize] =
                in_0[1 as libc::c_int as usize].wrapping_add(1);
            if in_0[1 as libc::c_int as usize] == 0 {
                in_0[2 as libc::c_int as usize] =
                    in_0[2 as libc::c_int as usize].wrapping_add(1);
                if in_0[2 as libc::c_int as usize] == 0 {
                    in_0[3 as libc::c_int as usize] =
                        in_0[3 as libc::c_int as usize].wrapping_add(1)
                }
            }
        }
        surf();
        outleft = 8 as libc::c_int
    }
    outleft -= 1;
    return out[outleft as usize] as libc::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn rand32() -> u32_0 {
    if outleft == 0 {
        in_0[0 as libc::c_int as usize] =
            in_0[0 as libc::c_int as usize].wrapping_add(1);
        if in_0[0 as libc::c_int as usize] == 0 {
            in_0[1 as libc::c_int as usize] =
                in_0[1 as libc::c_int as usize].wrapping_add(1);
            if in_0[1 as libc::c_int as usize] == 0 {
                in_0[2 as libc::c_int as usize] =
                    in_0[2 as libc::c_int as usize].wrapping_add(1);
                if in_0[2 as libc::c_int as usize] == 0 {
                    in_0[3 as libc::c_int as usize] =
                        in_0[3 as libc::c_int as usize].wrapping_add(1)
                }
            }
        }
        surf();
        outleft = 8 as libc::c_int
    }
    outleft -= 1;
    return out[outleft as usize];
}
#[no_mangle]
pub unsafe extern "C" fn rand64() -> u64_0 {
    static mut outleft_0: libc::c_int = 0 as libc::c_int;
    if outleft_0 < 2 as libc::c_int {
        in_0[0 as libc::c_int as usize] =
            in_0[0 as libc::c_int as usize].wrapping_add(1);
        if in_0[0 as libc::c_int as usize] == 0 {
            in_0[1 as libc::c_int as usize] =
                in_0[1 as libc::c_int as usize].wrapping_add(1);
            if in_0[1 as libc::c_int as usize] == 0 {
                in_0[2 as libc::c_int as usize] =
                    in_0[2 as libc::c_int as usize].wrapping_add(1);
                if in_0[2 as libc::c_int as usize] == 0 {
                    in_0[3 as libc::c_int as usize] =
                        in_0[3 as libc::c_int as usize].wrapping_add(1)
                }
            }
        }
        surf();
        outleft_0 = 8 as libc::c_int
    }
    outleft_0 -= 2 as libc::c_int;
    return (out[(outleft_0 + 1 as libc::c_int) as usize] as
                u64_0).wrapping_add((out[outleft_0 as usize] as u64_0) <<
                                        32 as libc::c_int);
}
/* returns 2 if names is OK but contains one or more underscores */
unsafe extern "C" fn check_name(mut in_1: *mut libc::c_char) -> libc::c_int {
    /* remove trailing . 
     also fail empty string and label > 63 chars */
    let mut dotgap: size_t = 0 as libc::c_int as size_t;
    let mut l: size_t = strlen(in_1);
    let mut c: libc::c_char = 0;
    let mut nowhite: libc::c_int = 0 as libc::c_int;
    let mut hasuscore: libc::c_int = 0 as libc::c_int;
    if l == 0 as libc::c_int as libc::c_ulong ||
           l > 1025 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    }
    if *in_1.offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                        isize) as libc::c_int == '.' as i32 {
        *in_1.offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                         isize) = 0 as libc::c_int as libc::c_char;
        nowhite = 1 as libc::c_int
    }
    loop  {
        c = *in_1;
        if !(c != 0) { break ; }
        if c as libc::c_int == '.' as i32 {
            dotgap = 0 as libc::c_int as size_t
        } else {
            dotgap = dotgap.wrapping_add(1);
            if dotgap > 63 as libc::c_int as libc::c_ulong {
                return 0 as libc::c_int
            } else {
                if c as libc::c_uchar as libc::c_int & !(0x7f as libc::c_int)
                       == 0 as libc::c_int &&
                       *(*__ctype_b_loc()).offset(c as libc::c_uchar as
                                                      libc::c_int as isize) as
                           libc::c_int &
                           _IScntrl as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                    /* iscntrl only gives expected results for ascii */
                    return 0 as libc::c_int
                } else {
                    if !(c as libc::c_uchar as libc::c_int &
                             !(0x7f as libc::c_int) == 0 as libc::c_int) {
                        return 0 as libc::c_int
                    } else {
                        if c as libc::c_int != ' ' as i32 {
                            nowhite = 1 as libc::c_int;
                            if c as libc::c_int == '_' as i32 {
                                hasuscore = 1 as libc::c_int
                            }
                        }
                    }
                }
            }
        }
        in_1 = in_1.offset(1)
    }
    if nowhite == 0 { return 0 as libc::c_int }
    return if hasuscore != 0 { 2 as libc::c_int } else { 1 as libc::c_int };
}
/* Hostnames have a more limited valid charset than domain names
   so check for legal char a-z A-Z 0-9 - _ 
   Note that this may receive a FQDN, so only check the first label 
   for the tighter criteria. */
#[no_mangle]
pub unsafe extern "C" fn legal_hostname(mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut first: libc::c_int = 0;
    if check_name(name) == 0 { return 0 as libc::c_int }
    first = 1 as libc::c_int;
    loop  {
        c = *name;
        if !(c != 0) { break ; }
        /* check for legal char a-z A-Z 0-9 - _ . */
        if !(c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32
                 ||
                 c as libc::c_int >= 'a' as i32 &&
                     c as libc::c_int <= 'z' as i32 ||
                 c as libc::c_int >= '0' as i32 &&
                     c as libc::c_int <= '9' as i32) {
            if !(first == 0 &&
                     (c as libc::c_int == '-' as i32 ||
                          c as libc::c_int == '_' as i32)) {
                /* end of hostname part */
                if c as libc::c_int == '.' as i32 { return 1 as libc::c_int }
                return 0 as libc::c_int
            }
        }
        name = name.offset(1);
        first = 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn canonicalise(mut in_1: *mut libc::c_char,
                                      mut nomem: *mut libc::c_int)
 -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rc: libc::c_int = 0;
    if !nomem.is_null() { *nomem = 0 as libc::c_int }
    rc = check_name(in_1);
    if rc == 0 { return 0 as *mut libc::c_char }
    ret =
        whine_malloc(strlen(in_1).wrapping_add(1 as libc::c_int as
                                                   libc::c_ulong)) as
            *mut libc::c_char;
    if !ret.is_null() {
        strcpy(ret, in_1);
    } else if !nomem.is_null() { *nomem = 1 as libc::c_int }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn do_rfc1035_name(mut p: *mut libc::c_uchar,
                                         mut sval: *mut libc::c_char,
                                         mut limit: *mut libc::c_char)
 -> *mut libc::c_uchar {
    let mut j: libc::c_int = 0;
    while !sval.is_null() && *sval as libc::c_int != 0 {
        let fresh6 = p;
        p = p.offset(1);
        let mut cp: *mut libc::c_uchar = fresh6;
        if !limit.is_null() && p > limit as *mut libc::c_uchar {
            return 0 as *mut libc::c_uchar
        }
        j = 0 as libc::c_int;
        while *sval as libc::c_int != 0 && *sval as libc::c_int != '.' as i32
              {
            if !limit.is_null() &&
                   p.offset(1 as libc::c_int as isize) >
                       limit as *mut libc::c_uchar {
                return 0 as *mut libc::c_uchar
            }
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = *sval as libc::c_uchar;
            sval = sval.offset(1);
            j += 1
        }
        *cp = j as libc::c_uchar;
        if *sval != 0 { sval = sval.offset(1) }
    }
    return p;
}
/* for use during startup */
#[no_mangle]
pub unsafe extern "C" fn safe_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void =
        calloc(1 as libc::c_int as libc::c_ulong, size);
    if ret.is_null() {
        die(b"could not get memory\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 4 as libc::c_int);
    }
    return ret;
}
/* Ensure limited size string is always terminated.
 * Can be replaced by (void)strlcpy() on some platforms */
#[no_mangle]
pub unsafe extern "C" fn safe_strncpy(mut dest: *mut libc::c_char,
                                      mut src: *const libc::c_char,
                                      mut size: size_t) {
    if size != 0 as libc::c_int as libc::c_ulong {
        *dest.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                         isize) = '\u{0}' as i32 as libc::c_char;
        strncpy(dest, src,
                size.wrapping_sub(1 as libc::c_int as libc::c_ulong));
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_pipe(mut fd: *mut libc::c_int,
                                   mut read_noblock: libc::c_int) {
    if pipe(fd) == -(1 as libc::c_int) ||
           fix_fd(*fd.offset(1 as libc::c_int as isize)) == 0 ||
           read_noblock != 0 &&
               fix_fd(*fd.offset(0 as libc::c_int as isize)) == 0 {
        die(b"cannot create pipe: %s\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char, 0 as *mut libc::c_char,
            5 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn whine_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void =
        calloc(1 as libc::c_int as libc::c_ulong, size);
    if ret.is_null() {
        my_syslog(3 as libc::c_int,
                  b"failed to allocate %d bytes\x00" as *const u8 as
                      *const libc::c_char, size as libc::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn sockaddr_isequal(mut s1: *mut mysockaddr,
                                          mut s2: *mut mysockaddr)
 -> libc::c_int {
    if (*s1).sa.sa_family as libc::c_int == (*s2).sa.sa_family as libc::c_int
       {
        if (*s1).sa.sa_family as libc::c_int == 2 as libc::c_int &&
               (*s1).in_0.sin_port as libc::c_int ==
                   (*s2).in_0.sin_port as libc::c_int &&
               (*s1).in_0.sin_addr.s_addr == (*s2).in_0.sin_addr.s_addr {
            return 1 as libc::c_int
        }
        if (*s1).sa.sa_family as libc::c_int == 10 as libc::c_int &&
               (*s1).in6.sin6_port as libc::c_int ==
                   (*s2).in6.sin6_port as libc::c_int &&
               (*s1).in6.sin6_scope_id == (*s2).in6.sin6_scope_id &&
               ({
                    let mut __a: *const in6_addr =
                        &mut (*s1).in6.sin6_addr as *mut in6_addr as
                            *const in6_addr;
                    let mut __b: *const in6_addr =
                        &mut (*s2).in6.sin6_addr as *mut in6_addr as
                            *const in6_addr;
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
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sa_len(mut addr: *mut mysockaddr) -> libc::c_int {
    if (*addr).sa.sa_family as libc::c_int == 10 as libc::c_int {
        return ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as
                   libc::c_int
    } else {
        return ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                   libc::c_int
    };
}
/* don't use strcasecmp and friends here - they may be messed up by LOCALE */
#[no_mangle]
pub unsafe extern "C" fn hostname_isequal(mut a: *const libc::c_char,
                                          mut b: *const libc::c_char)
 -> libc::c_int {
    let mut c1: libc::c_uint = 0;
    let mut c2: libc::c_uint = 0;
    loop  {
        let fresh8 = a;
        a = a.offset(1);
        c1 = *fresh8 as libc::c_uchar as libc::c_uint;
        let fresh9 = b;
        b = b.offset(1);
        c2 = *fresh9 as libc::c_uchar as libc::c_uint;
        if c1 >= 'A' as i32 as libc::c_uint &&
               c1 <= 'Z' as i32 as libc::c_uint {
            c1 = c1.wrapping_add(('a' as i32 - 'A' as i32) as libc::c_uint)
        }
        if c2 >= 'A' as i32 as libc::c_uint &&
               c2 <= 'Z' as i32 as libc::c_uint {
            c2 = c2.wrapping_add(('a' as i32 - 'A' as i32) as libc::c_uint)
        }
        if c1 != c2 { return 0 as libc::c_int }
        if !(c1 != 0) { break ; }
    }
    return 1 as libc::c_int;
}
/* is b equal to or a subdomain of a return 2 for equal, 1 for subdomain */
#[no_mangle]
pub unsafe extern "C" fn hostname_issubdomain(mut a: *mut libc::c_char,
                                              mut b: *mut libc::c_char)
 -> libc::c_int {
    let mut ap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c1: libc::c_uint = 0;
    let mut c2: libc::c_uint = 0;
    /* move to the end */
    ap = a;
    while *ap != 0 { ap = ap.offset(1) }
    bp = b;
    while *bp != 0 { bp = bp.offset(1) }
    /* a shorter than b or a empty. */
    if (bp.wrapping_offset_from(b) as libc::c_long) <
           ap.wrapping_offset_from(a) as libc::c_long || ap == a {
        return 0 as libc::c_int
    }
    loop  {
        ap = ap.offset(-1);
        c1 = *ap as libc::c_uchar as libc::c_uint;
        bp = bp.offset(-1);
        c2 = *bp as libc::c_uchar as libc::c_uint;
        if c1 >= 'A' as i32 as libc::c_uint &&
               c1 <= 'Z' as i32 as libc::c_uint {
            c1 = c1.wrapping_add(('a' as i32 - 'A' as i32) as libc::c_uint)
        }
        if c2 >= 'A' as i32 as libc::c_uint &&
               c2 <= 'Z' as i32 as libc::c_uint {
            c2 = c2.wrapping_add(('a' as i32 - 'A' as i32) as libc::c_uint)
        }
        if c1 != c2 { return 0 as libc::c_int }
        if !(ap != a) { break ; }
    }
    if bp == b { return 2 as libc::c_int }
    bp = bp.offset(-1);
    if *bp as libc::c_int == '.' as i32 { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dnsmasq_time() -> time_t {
    return time(0 as *mut time_t);
}
#[no_mangle]
pub unsafe extern "C" fn netmask_length(mut mask: in_addr) -> libc::c_int {
    let mut zero_count: libc::c_int = 0 as libc::c_int;
    while 0 as libc::c_int as libc::c_uint ==
              mask.s_addr & 0x1 as libc::c_int as libc::c_uint &&
              zero_count < 32 as libc::c_int {
        mask.s_addr >>= 1 as libc::c_int;
        zero_count += 1
    }
    return 32 as libc::c_int - zero_count;
}
#[no_mangle]
pub unsafe extern "C" fn is_same_net(mut a: in_addr, mut b: in_addr,
                                     mut mask: in_addr) -> libc::c_int {
    return (a.s_addr & mask.s_addr == b.s_addr & mask.s_addr) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_same_net6(mut a: *mut in6_addr,
                                      mut b: *mut in6_addr,
                                      mut prefixlen: libc::c_int)
 -> libc::c_int {
    let mut pfbytes: libc::c_int = prefixlen >> 3 as libc::c_int;
    let mut pfbits: libc::c_int = prefixlen & 7 as libc::c_int;
    if memcmp(&mut (*a).__in6_u.__u6_addr8 as *mut [uint8_t; 16] as
                  *const libc::c_void,
              &mut (*b).__in6_u.__u6_addr8 as *mut [uint8_t; 16] as
                  *const libc::c_void, pfbytes as libc::c_ulong) !=
           0 as libc::c_int {
        return 0 as libc::c_int
    }
    if pfbits == 0 as libc::c_int ||
           (*a).__in6_u.__u6_addr8[pfbytes as usize] as libc::c_int >>
               8 as libc::c_int - pfbits ==
               (*b).__in6_u.__u6_addr8[pfbytes as usize] as libc::c_int >>
                   8 as libc::c_int - pfbits {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* return least significant 64 bits if IPv6 address */
#[no_mangle]
pub unsafe extern "C" fn addr6part(mut addr: *mut in6_addr) -> u64_0 {
    let mut i: libc::c_int = 0;
    let mut ret: u64_0 = 0 as libc::c_int as u64_0;
    i = 8 as libc::c_int;
    while i < 16 as libc::c_int {
        ret =
            (ret <<
                 8 as
                     libc::c_int).wrapping_add((*addr).__in6_u.__u6_addr8[i as
                                                                              usize]
                                                   as libc::c_ulonglong);
        i += 1
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn setaddr6part(mut addr: *mut in6_addr,
                                      mut host: u64_0) {
    let mut i: libc::c_int = 0;
    i = 15 as libc::c_int;
    while i >= 8 as libc::c_int {
        (*addr).__in6_u.__u6_addr8[i as usize] = host as uint8_t;
        host = host >> 8 as libc::c_int;
        i -= 1
    };
}
/* returns port number from address */
#[no_mangle]
pub unsafe extern "C" fn prettyprint_addr(mut addr: *mut mysockaddr,
                                          mut buf: *mut libc::c_char)
 -> libc::c_int {
    let mut port: libc::c_int = 0 as libc::c_int;
    if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int {
        inet_ntop(2 as libc::c_int,
                  &mut (*addr).in_0.sin_addr as *mut in_addr as
                      *const libc::c_void, buf,
                  46 as libc::c_int as socklen_t);
        port = __bswap_16((*addr).in_0.sin_port) as libc::c_int
    } else if (*addr).sa.sa_family as libc::c_int == 10 as libc::c_int {
        let mut name: [libc::c_char; 16] = [0; 16];
        inet_ntop(10 as libc::c_int,
                  &mut (*addr).in6.sin6_addr as *mut in6_addr as
                      *const libc::c_void, buf,
                  46 as libc::c_int as socklen_t);
        if (*addr).in6.sin6_scope_id != 0 as libc::c_int as libc::c_uint &&
               !if_indextoname((*addr).in6.sin6_scope_id,
                               name.as_mut_ptr()).is_null() &&
               strlen(buf).wrapping_add(strlen(name.as_mut_ptr())).wrapping_add(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                   <= 46 as libc::c_int as libc::c_ulong {
            strcat(buf, b"%\x00" as *const u8 as *const libc::c_char);
            strcat(buf, name.as_mut_ptr());
        }
        port = __bswap_16((*addr).in6.sin6_port) as libc::c_int
    }
    return port;
}
#[no_mangle]
pub unsafe extern "C" fn prettyprint_time(mut buf: *mut libc::c_char,
                                          mut t: libc::c_uint) {
    if t == 0xffffffff as libc::c_uint {
        sprintf(buf, b"infinite\x00" as *const u8 as *const libc::c_char);
    } else {
        let mut x: libc::c_uint = 0;
        let mut p: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        x = t.wrapping_div(86400 as libc::c_int as libc::c_uint);
        if x != 0 {
            p =
                p.wrapping_add(sprintf(&mut *buf.offset(p as isize) as
                                           *mut libc::c_char,
                                       b"%ud\x00" as *const u8 as
                                           *const libc::c_char, x) as
                                   libc::c_uint)
        }
        x =
            t.wrapping_div(3600 as libc::c_int as
                               libc::c_uint).wrapping_rem(24 as libc::c_int as
                                                              libc::c_uint);
        if x != 0 {
            p =
                p.wrapping_add(sprintf(&mut *buf.offset(p as isize) as
                                           *mut libc::c_char,
                                       b"%uh\x00" as *const u8 as
                                           *const libc::c_char, x) as
                                   libc::c_uint)
        }
        x =
            t.wrapping_div(60 as libc::c_int as
                               libc::c_uint).wrapping_rem(60 as libc::c_int as
                                                              libc::c_uint);
        if x != 0 {
            p =
                p.wrapping_add(sprintf(&mut *buf.offset(p as isize) as
                                           *mut libc::c_char,
                                       b"%um\x00" as *const u8 as
                                           *const libc::c_char, x) as
                                   libc::c_uint)
        }
        x = t.wrapping_rem(60 as libc::c_int as libc::c_uint);
        if x != 0 {
            p =
                p.wrapping_add(sprintf(&mut *buf.offset(p as isize) as
                                           *mut libc::c_char,
                                       b"%us\x00" as *const u8 as
                                           *const libc::c_char, x) as
                                   libc::c_uint)
        }
    };
}
/* in may equal out, when maxlen may be -1 (No max len). 
   Return -1 for extraneous no-hex chars found. */
#[no_mangle]
pub unsafe extern "C" fn parse_hex(mut in_1: *mut libc::c_char,
                                   mut out_0: *mut libc::c_uchar,
                                   mut maxlen: libc::c_int,
                                   mut wildcard_mask: *mut libc::c_uint,
                                   mut mac_type: *mut libc::c_int)
 -> libc::c_int {
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut mask: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    if !mac_type.is_null() { *mac_type = 0 as libc::c_int }
    while done == 0 && (maxlen == -(1 as libc::c_int) || i < maxlen) {
        r = in_1;
        while *r as libc::c_int != 0 as libc::c_int &&
                  *r as libc::c_int != ':' as i32 &&
                  *r as libc::c_int != '-' as i32 &&
                  *r as libc::c_int != ' ' as i32 {
            if *r as libc::c_int != '*' as i32 &&
                   *(*__ctype_b_loc()).offset(*r as libc::c_uchar as
                                                  libc::c_int as isize) as
                       libc::c_int &
                       _ISxdigit as libc::c_int as libc::c_ushort as
                           libc::c_int == 0 {
                return -(1 as libc::c_int)
            }
            r = r.offset(1)
        }
        if *r as libc::c_int == 0 as libc::c_int { done = 1 as libc::c_int }
        if r != in_1 {
            if *r as libc::c_int == '-' as i32 && i == 0 as libc::c_int &&
                   !mac_type.is_null() {
                *r = 0 as libc::c_int as libc::c_char;
                *mac_type =
                    strtol(in_1, 0 as *mut *mut libc::c_char,
                           16 as libc::c_int) as libc::c_int;
                mac_type = 0 as *mut libc::c_int
            } else {
                *r = 0 as libc::c_int as libc::c_char;
                if strcmp(in_1, b"*\x00" as *const u8 as *const libc::c_char)
                       == 0 as libc::c_int {
                    mask = mask << 1 as libc::c_int | 1 as libc::c_int;
                    i += 1
                } else {
                    let mut j: libc::c_int = 0;
                    let mut bytes: libc::c_int =
                        ((1 as libc::c_int as libc::c_long +
                              r.wrapping_offset_from(in_1) as libc::c_long) /
                             2 as libc::c_int as libc::c_long) as libc::c_int;
                    j = 0 as libc::c_int;
                    while j < bytes {
                        let mut sav: libc::c_char = 0;
                        sav = sav;
                        if j < bytes - 1 as libc::c_int {
                            sav =
                                *in_1.offset(((j + 1 as libc::c_int) *
                                                  2 as libc::c_int) as isize);
                            *in_1.offset(((j + 1 as libc::c_int) *
                                              2 as libc::c_int) as isize) =
                                0 as libc::c_int as libc::c_char
                        }
                        /* checks above allow mix of hexdigit and *, which
			 is illegal. */
                        if !strchr(&mut *in_1.offset((j * 2 as libc::c_int) as
                                                         isize),
                                   '*' as i32).is_null() {
                            return -(1 as libc::c_int)
                        }
                        *out_0.offset(i as isize) =
                            strtol(&mut *in_1.offset((j * 2 as libc::c_int) as
                                                         isize),
                                   0 as *mut *mut libc::c_char,
                                   16 as libc::c_int) as libc::c_uchar;
                        mask = mask << 1 as libc::c_int;
                        i += 1;
                        if i == maxlen { break ; }
                        if j < bytes - 1 as libc::c_int {
                            *in_1.offset(((j + 1 as libc::c_int) *
                                              2 as libc::c_int) as isize) =
                                sav
                        }
                        j += 1
                    }
                }
            }
        }
        in_1 = r.offset(1 as libc::c_int as isize)
    }
    if !wildcard_mask.is_null() { *wildcard_mask = mask as libc::c_uint }
    return i;
}
/* return 0 for no match, or (no matched octets) + 1 */
#[no_mangle]
pub unsafe extern "C" fn memcmp_masked(mut a: *mut libc::c_uchar,
                                       mut b: *mut libc::c_uchar,
                                       mut len: libc::c_int,
                                       mut mask: libc::c_uint)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    count = 1 as libc::c_int;
    i = len - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if mask & 1 as libc::c_int as libc::c_uint == 0 {
            if *a.offset(i as isize) as libc::c_int ==
                   *b.offset(i as isize) as libc::c_int {
                count += 1
            } else { return 0 as libc::c_int }
        }
        i -= 1;
        mask = mask >> 1 as libc::c_int
    }
    return count;
}
/* _note_ may copy buffer */
#[no_mangle]
pub unsafe extern "C" fn expand_buf(mut iov: *mut iovec, mut size: size_t)
 -> libc::c_int {
    let mut new: *mut libc::c_void = 0 as *mut libc::c_void;
    if size <= (*iov).iov_len { return 1 as libc::c_int }
    new = whine_malloc(size);
    if new.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as libc::c_int
    }
    if !(*iov).iov_base.is_null() {
        memcpy(new, (*iov).iov_base, (*iov).iov_len);
        free((*iov).iov_base);
    }
    (*iov).iov_base = new;
    (*iov).iov_len = size;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn print_mac(mut buff: *mut libc::c_char,
                                   mut mac: *mut libc::c_uchar,
                                   mut len: libc::c_int)
 -> *mut libc::c_char {
    let mut p: *mut libc::c_char = buff;
    let mut i: libc::c_int = 0;
    if len == 0 as libc::c_int {
        sprintf(p, b"<null>\x00" as *const u8 as *const libc::c_char);
    } else {
        i = 0 as libc::c_int;
        while i < len {
            p =
                p.offset(sprintf(p,
                                 b"%.2x%s\x00" as *const u8 as
                                     *const libc::c_char,
                                 *mac.offset(i as isize) as libc::c_int,
                                 if i == len - 1 as libc::c_int {
                                     b"\x00" as *const u8 as
                                         *const libc::c_char
                                 } else {
                                     b":\x00" as *const u8 as
                                         *const libc::c_char
                                 }) as isize);
            i += 1
        }
    }
    return buff;
}
/* rc is return from sendto and friends.
   Return 1 if we should retry.
   Set errno to zero if we succeeded. */
#[no_mangle]
pub unsafe extern "C" fn retry_send(mut rc: ssize_t) -> libc::c_int {
    static mut retries: libc::c_int = 0 as libc::c_int;
    let mut waiter: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    if rc != -(1 as libc::c_int) as libc::c_long {
        retries = 0 as libc::c_int;
        *__errno_location() = 0 as libc::c_int;
        return 0 as libc::c_int
    }
    /* Linux kernels can return EAGAIN in perpetuity when calling
     sendmsg() and the relevant interface has gone. Here we loop
     retrying in EAGAIN for 1 second max, to avoid this hanging 
     dnsmasq. */
    if *__errno_location() == 11 as libc::c_int ||
           *__errno_location() == 11 as libc::c_int {
        waiter.tv_sec = 0 as libc::c_int as __time_t;
        waiter.tv_nsec = 10000 as libc::c_int as __syscall_slong_t;
        nanosleep(&mut waiter, 0 as *mut timespec);
        let fresh10 = retries;
        retries = retries + 1;
        if fresh10 < 1000 as libc::c_int { return 1 as libc::c_int }
    }
    retries = 0 as libc::c_int;
    if *__errno_location() == 4 as libc::c_int { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn read_write(mut fd: libc::c_int,
                                    mut packet: *mut libc::c_uchar,
                                    mut size: libc::c_int,
                                    mut rw: libc::c_int) -> libc::c_int {
    let mut n: ssize_t = 0;
    let mut done: ssize_t = 0;
    done = 0 as libc::c_int as ssize_t;
    while done < size as libc::c_long {
        loop  {
            if rw != 0 {
                n =
                    read(fd,
                         &mut *packet.offset(done as isize) as
                             *mut libc::c_uchar as *mut libc::c_void,
                         (size as libc::c_long - done) as size_t)
            } else {
                n =
                    write(fd,
                          &mut *packet.offset(done as isize) as
                              *mut libc::c_uchar as *const libc::c_void,
                          (size as libc::c_long - done) as size_t)
            }
            if n == 0 as libc::c_int as libc::c_long {
                return 0 as libc::c_int
            }
            if !(retry_send(n) != 0 ||
                     *__errno_location() == 12 as libc::c_int ||
                     *__errno_location() == 105 as libc::c_int) {
                break ;
            }
        }
        if *__errno_location() != 0 as libc::c_int { return 0 as libc::c_int }
        done += n
    }
    return 1 as libc::c_int;
}
/* close all fds except STDIN, STDOUT and STDERR, spare1, spare2 and spare3 */
#[no_mangle]
pub unsafe extern "C" fn close_fds(mut max_fd: libc::c_long,
                                   mut spare1: libc::c_int,
                                   mut spare2: libc::c_int,
                                   mut spare3: libc::c_int) {
    /* On Linux, use the /proc/ filesystem to find which files
     are actually open, rather than iterate over the whole space,
     for efficiency reasons. If this fails we drop back to the dumb code. */
    let mut d: *mut DIR = 0 as *mut DIR;
    d = opendir(b"/proc/self/fd\x00" as *const u8 as *const libc::c_char);
    if !d.is_null() {
        let mut de: *mut dirent = 0 as *mut dirent;
        loop  {
            de = readdir(d);
            if de.is_null() { break ; }
            let mut fd: libc::c_long = 0;
            let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
            *__errno_location() = 0 as libc::c_int;
            fd = strtol((*de).d_name.as_mut_ptr(), &mut e, 10 as libc::c_int);
            if *__errno_location() != 0 as libc::c_int || e.is_null() ||
                   *e as libc::c_int != 0 || fd == dirfd(d) as libc::c_long ||
                   fd == 1 as libc::c_int as libc::c_long ||
                   fd == 2 as libc::c_int as libc::c_long ||
                   fd == 0 as libc::c_int as libc::c_long ||
                   fd == spare1 as libc::c_long ||
                   fd == spare2 as libc::c_long ||
                   fd == spare3 as libc::c_long {
                continue ;
            }
            close(fd as libc::c_int);
        }
        closedir(d);
        return
    }
    /* fallback, dumb code. */
    max_fd -= 1;
    while max_fd >= 0 as libc::c_int as libc::c_long {
        if max_fd != 1 as libc::c_int as libc::c_long &&
               max_fd != 2 as libc::c_int as libc::c_long &&
               max_fd != 0 as libc::c_int as libc::c_long &&
               max_fd != spare1 as libc::c_long &&
               max_fd != spare2 as libc::c_long &&
               max_fd != spare3 as libc::c_long {
            close(max_fd as libc::c_int);
        }
        max_fd -= 1
    };
}
/* Basically match a string value against a wildcard pattern.  */
#[no_mangle]
pub unsafe extern "C" fn wildcard_match(mut wildcard: *const libc::c_char,
                                        mut match_0: *const libc::c_char)
 -> libc::c_int {
    while *wildcard as libc::c_int != 0 && *match_0 as libc::c_int != 0 {
        if *wildcard as libc::c_int == '*' as i32 { return 1 as libc::c_int }
        if *wildcard as libc::c_int != *match_0 as libc::c_int {
            return 0 as libc::c_int
        }
        wildcard = wildcard.offset(1);
        match_0 = match_0.offset(1)
    }
    return (*wildcard as libc::c_int == *match_0 as libc::c_int) as
               libc::c_int;
}
/* The same but comparing a maximum of NUM characters, like strncmp.  */
#[no_mangle]
pub unsafe extern "C" fn wildcard_matchn(mut wildcard: *const libc::c_char,
                                         mut match_0: *const libc::c_char,
                                         mut num: libc::c_int)
 -> libc::c_int {
    while *wildcard as libc::c_int != 0 && *match_0 as libc::c_int != 0 &&
              num != 0 {
        if *wildcard as libc::c_int == '*' as i32 { return 1 as libc::c_int }
        if *wildcard as libc::c_int != *match_0 as libc::c_int {
            return 0 as libc::c_int
        }
        wildcard = wildcard.offset(1);
        match_0 = match_0.offset(1);
        num -= 1
    }
    return (num == 0 || *wildcard as libc::c_int == *match_0 as libc::c_int)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kernel_version() -> libc::c_int {
    let mut utsname: utsname =
        utsname{sysname: [0; 65],
                nodename: [0; 65],
                release: [0; 65],
                version: [0; 65],
                machine: [0; 65],
                domainname: [0; 65],};
    let mut version: libc::c_int = 0;
    let mut split: *mut libc::c_char = 0 as *mut libc::c_char;
    if uname(&mut utsname) < 0 as libc::c_int {
        die(b"failed to find kernel version: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 5 as libc::c_int);
    }
    split =
        strtok(utsname.release.as_mut_ptr(),
               b".\x00" as *const u8 as *const libc::c_char);
    version = if !split.is_null() { atoi(split) } else { 0 as libc::c_int };
    split =
        strtok(0 as *mut libc::c_char,
               b".\x00" as *const u8 as *const libc::c_char);
    version =
        version * 256 as libc::c_int +
            (if !split.is_null() { atoi(split) } else { 0 as libc::c_int });
    split =
        strtok(0 as *mut libc::c_char,
               b".\x00" as *const u8 as *const libc::c_char);
    return version * 256 as libc::c_int +
               (if !split.is_null() {
                    atoi(split)
                } else { 0 as libc::c_int });
}
