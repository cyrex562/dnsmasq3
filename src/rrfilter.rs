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
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
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
    fn skip_name(ansp: *mut libc::c_uchar, header: *mut dns_header,
                 plen: size_t, extrabytes: libc::c_int) -> *mut libc::c_uchar;
    #[no_mangle]
    fn whine_malloc(size: size_t) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
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
pub type u8_0 = libc::c_uchar;
pub type u16_0 = libc::c_ushort;
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
/* Code to safely remove RRs from a DNS answer */
/* Go through a domain name, find "pointers" and fix them up based on how many bytes
   we've chopped out of the packet, or check they don't point into an elided part.  */
unsafe extern "C" fn check_name(mut namep: *mut *mut libc::c_uchar,
                                mut header: *mut dns_header, mut plen: size_t,
                                mut fixup: libc::c_int,
                                mut rrs: *mut *mut libc::c_uchar,
                                mut rr_count: libc::c_int) -> libc::c_int {
    let mut ansp: *mut libc::c_uchar = *namep;
    loop  {
        let mut label_type: libc::c_uint = 0;
        if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                  libc::c_long + 1 as libc::c_int as libc::c_long) as size_t
                 <= plen) {
            return 0 as libc::c_int
        }
        label_type =
            (*ansp as libc::c_int & 0xc0 as libc::c_int) as libc::c_uint;
        if label_type == 0xc0 as libc::c_int as libc::c_uint {
            /* pointer for compression. */
            let mut offset: libc::c_uint = 0;
            let mut i: libc::c_int = 0;
            let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + 2 as libc::c_int as libc::c_long) as
                     size_t <= plen) {
                return 0 as libc::c_int
            }
            let fresh6 = ansp;
            ansp = ansp.offset(1);
            offset =
                ((*fresh6 as libc::c_int & 0x3f as libc::c_int) <<
                     8 as libc::c_int) as libc::c_uint;
            let fresh7 = ansp;
            ansp = ansp.offset(1);
            offset |= *fresh7 as libc::c_uint;
            p = (header as *mut libc::c_uchar).offset(offset as isize);
            i = 0 as libc::c_int;
            while i < rr_count {
                if p < *rrs.offset(i as isize) { break ; }
                if i & 1 as libc::c_int != 0 {
                    offset =
                        (offset as libc::c_long -
                             (*rrs.offset(i as
                                              isize)).wrapping_offset_from(*rrs.offset((i
                                                                                            -
                                                                                            1
                                                                                                as
                                                                                                libc::c_int)
                                                                                           as
                                                                                           isize))
                                 as libc::c_long) as libc::c_uint
                }
                i += 1
            }
            /* does the pointer end up in an elided RR? */
            if i & 1 as libc::c_int != 0 { return 0 as libc::c_int }
            /* No, scale the pointer */
            if fixup != 0 {
                ansp =
                    ansp.offset(-(2 as libc::c_int as isize)); /* reserved */
                let fresh8 = ansp;
                ansp = ansp.offset(1);
                *fresh8 =
                    (offset >> 8 as libc::c_int |
                         0xc0 as libc::c_int as libc::c_uint) as
                        libc::c_uchar;
                let fresh9 = ansp;
                ansp = ansp.offset(1);
                *fresh9 =
                    (offset & 0xff as libc::c_int as libc::c_uint) as
                        libc::c_uchar
            }
            break ;
        } else if label_type == 0x80 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        } else if label_type == 0x40 as libc::c_int as libc::c_uint {
            /* Extended label type */
            let mut count: libc::c_uint =
                0; /* we only understand bitstrings */
            if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + 2 as libc::c_int as libc::c_long) as
                     size_t <= plen) {
                return 0 as libc::c_int
            } /* Bits in bitstring */
            let fresh10 = ansp;
            ansp = ansp.offset(1);
            if *fresh10 as libc::c_int & 0x3f as libc::c_int !=
                   1 as libc::c_int {
                return 0 as libc::c_int
            }
            let fresh11 = ansp;
            ansp = ansp.offset(1);
            count = *fresh11 as libc::c_uint;
            if count == 0 as libc::c_int as libc::c_uint {
                /* count == 0 means 256 bits */
                ansp = ansp.offset(32 as libc::c_int as isize)
            } else {
                ansp =
                    ansp.offset((count.wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint) >>
                                     3 as
                                         libc::c_int).wrapping_add(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                    as isize)
            }
        } else {
            /* label type == 0 Bottom six bits is length */
            let fresh12 = ansp;
            ansp = ansp.offset(1);
            let mut len: libc::c_uint =
                (*fresh12 as libc::c_int & 0x3f as libc::c_int) as
                    libc::c_uint;
            if if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar)
                         as libc::c_long + len as libc::c_long) as size_t <=
                        plen) {
                   0 as libc::c_int
               } else { ansp = ansp.offset(len as isize); 1 as libc::c_int }
                   == 0 {
                return 0 as libc::c_int
            }
            if len == 0 as libc::c_int as libc::c_uint { break ; }
            /* zero length label marks the end. */
        }
    }
    *namep = ansp;
    return 1 as libc::c_int;
}
/* Go through RRs and check or fixup the domain names contained within */
unsafe extern "C" fn check_rrs(mut p: *mut libc::c_uchar,
                               mut header: *mut dns_header, mut plen: size_t,
                               mut fixup: libc::c_int,
                               mut rrs: *mut *mut libc::c_uchar,
                               mut rr_count: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0; /* TTL */
    let mut j: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut class: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    let mut pp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    i = 0 as libc::c_int;
    while i <
              __bswap_16((*header).ancount) as libc::c_int +
                  __bswap_16((*header).nscount) as libc::c_int +
                  __bswap_16((*header).arcount) as libc::c_int {
        pp = p;
        p = skip_name(p, header, plen, 10 as libc::c_int);
        if p.is_null() { return 0 as libc::c_int }
        let mut t_cp: *mut libc::c_uchar = p;
        type_0 =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_0: *mut libc::c_uchar = p;
        class =
            (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        p = p.offset(4 as libc::c_int as isize);
        let mut t_cp_1: *mut libc::c_uchar = p;
        rdlen =
            (*t_cp_1.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_1.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        /* If this RR is to be elided, don't fix up its contents */
        j = 0 as libc::c_int;
        while j < rr_count {
            if *rrs.offset(j as isize) == pp { break ; }
            j += 2 as libc::c_int
        }
        if j >= rr_count {
            /* fixup name of RR */
            if check_name(&mut pp, header, plen, fixup, rrs, rr_count) == 0 {
                return 0 as libc::c_int
            }
            if class == 1 as libc::c_int {
                let mut d: *mut u16_0 = 0 as *mut u16_0;
                pp = p;
                d = rrfilter_desc(type_0);
                while *d as libc::c_int !=
                          -(1 as libc::c_int) as u16_0 as libc::c_int {
                    if *d as libc::c_int != 0 as libc::c_int {
                        pp = pp.offset(*d as libc::c_int as isize)
                    } else if check_name(&mut pp, header, plen, fixup, rrs,
                                         rr_count) == 0 {
                        return 0 as libc::c_int
                    }
                    d = d.offset(1)
                }
            }
        }
        if if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                     libc::c_long + rdlen as libc::c_long) as size_t <= plen)
              {
               0 as libc::c_int
           } else { p = p.offset(rdlen as isize); 1 as libc::c_int } == 0 {
            return 0 as libc::c_int
        }
        i += 1
    }
    return 1 as libc::c_int;
}
/* mode is 0 to remove EDNS0, 1 to filter DNSSEC RRs */
#[no_mangle]
pub unsafe extern "C" fn rrfilter(mut header: *mut dns_header,
                                  mut plen: size_t, mut mode: libc::c_int)
 -> size_t {
    static mut rrs: *mut *mut libc::c_uchar =
        0 as *const *mut libc::c_uchar as *mut *mut libc::c_uchar;
    static mut rr_sz: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_uchar =
        header.offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    let mut qtype: libc::c_int = 0;
    let mut qclass: libc::c_int = 0;
    let mut rr_found: libc::c_int = 0;
    let mut chop_an: libc::c_int = 0;
    let mut chop_ns: libc::c_int = 0;
    let mut chop_ar: libc::c_int = 0;
    if __bswap_16((*header).qdcount) as libc::c_int != 1 as libc::c_int ||
           { p = skip_name(p, header, plen, 4 as libc::c_int); p.is_null() } {
        return plen
    }
    let mut t_cp: *mut libc::c_uchar = p;
    qtype =
        (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int) <<
            8 as libc::c_int |
            *t_cp.offset(1 as libc::c_int as isize) as u16_0 as libc::c_int;
    p = p.offset(2 as libc::c_int as isize);
    let mut t_cp_0: *mut libc::c_uchar = p;
    qclass =
        (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int) <<
            8 as libc::c_int |
            *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as libc::c_int;
    p = p.offset(2 as libc::c_int as isize);
    let mut current_block_36: u64;
    /* First pass, find pointers to start and end of all the records we wish to elide:
     records added for DNSSEC, unless explicitly queried for */
    rr_found = 0 as libc::c_int; /* TTL */
    chop_ns = 0 as libc::c_int;
    chop_an = 0 as libc::c_int;
    chop_ar = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i <
              __bswap_16((*header).ancount) as libc::c_int +
                  __bswap_16((*header).nscount) as libc::c_int +
                  __bswap_16((*header).arcount) as libc::c_int {
        let mut pstart: *mut libc::c_uchar = p;
        let mut type_0: libc::c_int = 0;
        let mut class: libc::c_int = 0;
        p = skip_name(p, header, plen, 10 as libc::c_int);
        if p.is_null() { return plen }
        let mut t_cp_1: *mut libc::c_uchar = p;
        type_0 =
            (*t_cp_1.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_1.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_2: *mut libc::c_uchar = p;
        class =
            (*t_cp_2.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_2.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        p = p.offset(4 as libc::c_int as isize);
        let mut t_cp_3: *mut libc::c_uchar = p;
        rdlen =
            (*t_cp_3.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_3.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        if if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                     libc::c_long + rdlen as libc::c_long) as size_t <= plen)
              {
               0 as libc::c_int
           } else { p = p.offset(rdlen as isize); 1 as libc::c_int } == 0 {
            return plen
        }
        /* Don't remove the answer. */
        if !(i < __bswap_16((*header).ancount) as libc::c_int &&
                 type_0 == qtype && class == qclass) {
            if mode == 0 as libc::c_int {
                /* EDNS */
                /* EDNS mode, remove T_OPT from additional section only */
                if i <
                       __bswap_16((*header).nscount) as libc::c_int +
                           __bswap_16((*header).ancount) as libc::c_int ||
                       type_0 != 41 as libc::c_int {
                    current_block_36 = 2979737022853876585;
                } else { current_block_36 = 10692455896603418738; }
            } else if type_0 != 47 as libc::c_int &&
                          type_0 != 50 as libc::c_int &&
                          type_0 != 46 as libc::c_int {
                current_block_36 = 2979737022853876585;
            } else { current_block_36 = 10692455896603418738; }
            match current_block_36 {
                2979737022853876585 => { }
                _ => {
                    if expand_workspace(&mut rrs, &mut rr_sz,
                                        rr_found + 1 as libc::c_int) == 0 {
                        return plen
                    }
                    let fresh13 = rr_found;
                    rr_found = rr_found + 1;
                    let ref mut fresh14 = *rrs.offset(fresh13 as isize);
                    *fresh14 = pstart;
                    let fresh15 = rr_found;
                    rr_found = rr_found + 1;
                    let ref mut fresh16 = *rrs.offset(fresh15 as isize);
                    *fresh16 = p;
                    if i < __bswap_16((*header).ancount) as libc::c_int {
                        chop_an += 1
                    } else if i <
                                  __bswap_16((*header).nscount) as libc::c_int
                                      +
                                      __bswap_16((*header).ancount) as
                                          libc::c_int {
                        chop_ns += 1
                    } else { chop_ar += 1 }
                }
            }
        }
        /* DNSSEC mode, remove SIGs and NSECs from all three sections. */
        i += 1
    }
    /* Nothing to do. */
    if rr_found == 0 as libc::c_int { return plen }
    /* Second pass, look for pointers in names in the records we're keeping and make sure they don't
     point to records we're going to elide. This is theoretically possible, but unlikely. If
     it happens, we give up and leave the answer unchanged. */
    p = header.offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
    /* question first */
    if check_name(&mut p, header, plen, 0 as libc::c_int, rrs, rr_found) == 0
       {
        return plen
    } /* qclass, qtype */
    p = p.offset(4 as libc::c_int as isize);
    /* Now answers and NS */
    if check_rrs(p, header, plen, 0 as libc::c_int, rrs, rr_found) == 0 {
        return plen
    }
    /* Third pass, actually fix up pointers in the records */
    p =
        header.offset(1 as libc::c_int as isize) as
            *mut libc::c_uchar; /* qclass, qtype */
    check_name(&mut p, header, plen, 1 as libc::c_int, rrs, rr_found);
    p = p.offset(4 as libc::c_int as isize);
    check_rrs(p, header, plen, 1 as libc::c_int, rrs, rr_found);
    /* Fourth pass, elide records */
    p = *rrs.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < rr_found {
        let mut start: *mut libc::c_uchar = *rrs.offset(i as isize);
        let mut end: *mut libc::c_uchar =
            if i != rr_found - 1 as libc::c_int {
                *rrs.offset((i + 1 as libc::c_int) as isize)
            } else { (header as *mut libc::c_uchar).offset(plen as isize) };
        memmove(p as *mut libc::c_void, start as *const libc::c_void,
                end.wrapping_offset_from(start) as libc::c_long as
                    libc::c_ulong);
        p =
            p.offset(end.wrapping_offset_from(start) as libc::c_long as
                         isize);
        i += 2 as libc::c_int
    }
    plen =
        p.wrapping_offset_from(header as *mut libc::c_uchar) as libc::c_long
            as size_t;
    (*header).ancount =
        __bswap_16((__bswap_16((*header).ancount) as libc::c_int - chop_an) as
                       __uint16_t);
    (*header).nscount =
        __bswap_16((__bswap_16((*header).nscount) as libc::c_int - chop_ns) as
                       __uint16_t);
    (*header).arcount =
        __bswap_16((__bswap_16((*header).arcount) as libc::c_int - chop_ar) as
                       __uint16_t);
    return plen;
}
/* This is used in the DNSSEC code too, hence it's exported */
#[no_mangle]
pub unsafe extern "C" fn rrfilter_desc(mut type_0: libc::c_int)
 -> *mut u16_0 {
    /* List of RRtypes which include domains in the data.
     0 -> domain
     integer -> no. of plain bytes
     -1 -> end

     zero is not a valid RRtype, so the final entry is returned for
     anything which needs no mangling.
  */
    static mut rr_desc: [u16_0; 73] =
        [2 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, 3 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         4 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, 5 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         6 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         7 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, 8 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         9 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, 12 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         14 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         15 as libc::c_int as u16_0, 2 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         17 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         18 as libc::c_int as u16_0, 2 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         21 as libc::c_int as u16_0, 2 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         24 as libc::c_int as u16_0, 18 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         26 as libc::c_int as u16_0, 2 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, 30 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         36 as libc::c_int as u16_0, 2 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         33 as libc::c_int as u16_0, 6 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         39 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0];
    let mut p: *mut u16_0 = rr_desc.as_mut_ptr();
    while *p as libc::c_int != type_0 && *p as libc::c_int != 0 as libc::c_int
          {
        loop  {
            let fresh17 = p;
            p = p.offset(1);
            if !(*fresh17 as libc::c_int !=
                     -(1 as libc::c_int) as u16_0 as libc::c_int) {
                break ;
            }
        }
    }
    return p.offset(1 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn expand_workspace(mut wkspc:
                                              *mut *mut *mut libc::c_uchar,
                                          mut szp: *mut libc::c_int,
                                          mut new: libc::c_int)
 -> libc::c_int {
    let mut p: *mut *mut libc::c_uchar = 0 as *mut *mut libc::c_uchar;
    let mut old: libc::c_int = *szp;
    if old >= new + 1 as libc::c_int { return 1 as libc::c_int }
    if new >= 100 as libc::c_int { return 0 as libc::c_int }
    new += 5 as libc::c_int;
    p =
        whine_malloc((new as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_uchar>()
                                                          as libc::c_ulong))
            as *mut *mut libc::c_uchar;
    if p.is_null() { return 0 as libc::c_int }
    if old != 0 as libc::c_int && !(*wkspc).is_null() {
        memcpy(p as *mut libc::c_void, *wkspc as *const libc::c_void,
               (old as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_uchar>()
                                                    as libc::c_ulong));
        free(*wkspc as *mut libc::c_void);
    }
    *wkspc = p;
    *szp = new;
    return 1 as libc::c_int;
}
