#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types,
           ptr_wrapping_offset_from, register_tool)]
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:19"]
pub mod types_h {
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "72:1"]
    pub type __intmax_t = libc::c_long;
    #[c2rust::src_loc = "73:1"]
    pub type __uintmax_t = libc::c_ulong;
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "149:1"]
    pub type __ino64_t = libc::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "180:1"]
    pub type __blkcnt64_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src =
  "/usr/lib/llvm-10/lib/clang/10.0.0/include/stddef.h:19"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/types/struct_timespec.h:19"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/types/struct_iovec.h:19"]
pub mod struct_iovec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:8"]
    pub struct iovec {
        pub iov_base: *mut libc::c_void,
        pub iov_len: size_t,
    }
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/socket.h:19"]
pub mod socket_h {
    #[c2rust::src_loc = "33:1"]
    pub type socklen_t = __socklen_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "257:8"]
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
    #[c2rust::src_loc = "275:8"]
    pub struct cmsghdr {
        pub cmsg_len: size_t,
        pub cmsg_level: libc::c_int,
        pub cmsg_type: libc::c_int,
        pub __cmsg_data: [libc::c_uchar; 0],
    }
    #[inline]
    #[c2rust::src_loc = "311:1"]
    pub unsafe extern "C" fn __cmsg_nxthdr(mut __mhdr: *mut msghdr,
                                           mut __cmsg: *mut cmsghdr)
     -> *mut cmsghdr {
        if (*__cmsg).cmsg_len <
               ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
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
                    *mut libc::c_uchar).offset((*__mhdr).msg_controllen as
                                                   isize) ||
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
    use super::types_h::__socklen_t;
    use super::struct_iovec_h::iovec;
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dnsmasq.h:19"]
pub mod dnsmasq_h {
    #[c2rust::src_loc = "68:1"]
    pub type u8_0 = libc::c_uchar;
    #[c2rust::src_loc = "69:1"]
    pub type u16_0 = libc::c_ushort;
    use super::dns_protocol_h::dns_header;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "1223:1"]
        pub fn skip_name(ansp: *mut libc::c_uchar, header: *mut dns_header,
                         plen: size_t, extrabytes: libc::c_int)
         -> *mut libc::c_uchar;
        #[no_mangle]
        #[c2rust::src_loc = "1291:1"]
        pub fn whine_malloc(size: size_t) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/mnt/d/projects/dnsmasq-2.84/src/dns-protocol.h:19"]
pub mod dns_protocol_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:8"]
    pub struct dns_header {
        pub id: u16_0,
        pub hb3: u8_0,
        pub hb4: u8_0,
        pub qdcount: u16_0,
        pub ancount: u16_0,
        pub nscount: u16_0,
        pub arcount: u16_0,
    }
    #[c2rust::src_loc = "67:9"]
    pub const T_DNAME: libc::c_int = 39 as libc::c_int;
    #[c2rust::src_loc = "64:9"]
    pub const T_SRV: libc::c_int = 33 as libc::c_int;
    #[c2rust::src_loc = "66:9"]
    pub const T_KX: libc::c_int = 36 as libc::c_int;
    #[c2rust::src_loc = "63:9"]
    pub const T_NXT: libc::c_int = 30 as libc::c_int;
    #[c2rust::src_loc = "61:9"]
    pub const T_PX: libc::c_int = 26 as libc::c_int;
    #[c2rust::src_loc = "60:9"]
    pub const T_SIG: libc::c_int = 24 as libc::c_int;
    #[c2rust::src_loc = "59:9"]
    pub const T_RT: libc::c_int = 21 as libc::c_int;
    #[c2rust::src_loc = "58:9"]
    pub const T_AFSDB: libc::c_int = 18 as libc::c_int;
    #[c2rust::src_loc = "57:9"]
    pub const T_RP: libc::c_int = 17 as libc::c_int;
    #[c2rust::src_loc = "55:9"]
    pub const T_MX: libc::c_int = 15 as libc::c_int;
    #[c2rust::src_loc = "54:9"]
    pub const T_MINFO: libc::c_int = 14 as libc::c_int;
    #[c2rust::src_loc = "53:9"]
    pub const T_PTR: libc::c_int = 12 as libc::c_int;
    #[c2rust::src_loc = "52:9"]
    pub const T_MR: libc::c_int = 9 as libc::c_int;
    #[c2rust::src_loc = "51:9"]
    pub const T_MG: libc::c_int = 8 as libc::c_int;
    #[c2rust::src_loc = "50:9"]
    pub const T_MB: libc::c_int = 7 as libc::c_int;
    #[c2rust::src_loc = "49:9"]
    pub const T_SOA: libc::c_int = 6 as libc::c_int;
    #[c2rust::src_loc = "48:9"]
    pub const T_CNAME: libc::c_int = 5 as libc::c_int;
    #[c2rust::src_loc = "47:9"]
    pub const T_MF: libc::c_int = 4 as libc::c_int;
    #[c2rust::src_loc = "46:9"]
    pub const T_MD: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const T_NS: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "39:9"]
    pub const C_IN: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "70:9"]
    pub const T_RRSIG: libc::c_int = 46 as libc::c_int;
    #[c2rust::src_loc = "73:9"]
    pub const T_NSEC3: libc::c_int = 50 as libc::c_int;
    #[c2rust::src_loc = "71:9"]
    pub const T_NSEC: libc::c_int = 47 as libc::c_int;
    #[c2rust::src_loc = "68:9"]
    pub const T_OPT: libc::c_int = 41 as libc::c_int;
    use super::dnsmasq_h::{u16_0, u8_0};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stat.h:19"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
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
    #[c2rust::src_loc = "119:8"]
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
    #[c2rust::src_loc = "38:10"]
    pub const _STAT_VER_LINUX: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const _STAT_VER: libc::c_int = _STAT_VER_LINUX;
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t, __ino64_t, __blkcnt64_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h:19"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
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
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    #[c2rust::src_loc = "111:9"]
    pub const _IO_EOF_SEEN: libc::c_int = 0x10 as libc::c_int;
    #[c2rust::src_loc = "114:9"]
    pub const _IO_ERR_SEEN: libc::c_int = 0x20 as libc::c_int;
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/FILE.h:19"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdlib.h:19"]
pub mod stdlib_h {
    #[c2rust::src_loc = "808:1"]
    pub type __compar_fn_t
        =
        Option<unsafe extern "C" fn(_: *const libc::c_void,
                                    _: *const libc::c_void) -> libc::c_int>;
    #[inline]
    #[c2rust::src_loc = "360:1"]
    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char)
     -> libc::c_int {
        return strtol(__nptr,
                      NULL as *mut libc::c_void as *mut *mut libc::c_char,
                      10 as libc::c_int) as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "365:1"]
    pub unsafe extern "C" fn atol(mut __nptr: *const libc::c_char)
     -> libc::c_long {
        return strtol(__nptr,
                      NULL as *mut libc::c_void as *mut *mut libc::c_char,
                      10 as libc::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "372:15"]
    pub unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char)
     -> libc::c_longlong {
        return strtoll(__nptr,
                       NULL as *mut libc::c_void as *mut *mut libc::c_char,
                       10 as libc::c_int);
    }
    use super::stddef_h::NULL;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "117:15"]
        pub fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char)
         -> libc::c_double;
        #[no_mangle]
        #[c2rust::src_loc = "176:17"]
        pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
                      _: libc::c_int) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "200:22"]
        pub fn strtoll(_: *const libc::c_char, _: *mut *mut libc::c_char,
                       _: libc::c_int) -> libc::c_longlong;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdint.h:19"]
pub mod stdint_h {
    #[c2rust::src_loc = "101:1"]
    pub type intmax_t = __intmax_t;
    #[c2rust::src_loc = "102:1"]
    pub type uintmax_t = __uintmax_t;
    use super::types_h::{__intmax_t, __uintmax_t};
}
#[c2rust::header_src = "/usr/include/inttypes.h:19"]
pub mod inttypes_h {
    #[c2rust::src_loc = "34:1"]
    pub type __gwchar_t = libc::c_int;
    #[inline]
    #[c2rust::src_loc = "323:1"]
    pub unsafe extern "C" fn strtoimax(mut nptr: *const libc::c_char,
                                       mut endptr: *mut *mut libc::c_char,
                                       mut base: libc::c_int) -> intmax_t {
        return __strtol_internal(nptr, endptr, base, 0 as libc::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "335:1"]
    pub unsafe extern "C" fn strtoumax(mut nptr: *const libc::c_char,
                                       mut endptr: *mut *mut libc::c_char,
                                       mut base: libc::c_int) -> uintmax_t {
        return __strtoul_internal(nptr, endptr, base, 0 as libc::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "347:1"]
    pub unsafe extern "C" fn wcstoimax(mut nptr: *const __gwchar_t,
                                       mut endptr: *mut *mut __gwchar_t,
                                       mut base: libc::c_int) -> intmax_t {
        return __wcstol_internal(nptr, endptr, base, 0 as libc::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "361:1"]
    pub unsafe extern "C" fn wcstoumax(mut nptr: *const __gwchar_t,
                                       mut endptr: *mut *mut __gwchar_t,
                                       mut base: libc::c_int) -> uintmax_t {
        return __wcstoul_internal(nptr, endptr, base, 0 as libc::c_int);
    }
    use super::stdint_h::{intmax_t, uintmax_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "318:1"]
        pub fn __strtol_internal(__nptr: *const libc::c_char,
                                 __endptr: *mut *mut libc::c_char,
                                 __base: libc::c_int, __group: libc::c_int)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "330:1"]
        pub fn __strtoul_internal(__nptr: *const libc::c_char,
                                  __endptr: *mut *mut libc::c_char,
                                  __base: libc::c_int, __group: libc::c_int)
         -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "342:1"]
        pub fn __wcstol_internal(__nptr: *const __gwchar_t,
                                 __endptr: *mut *mut __gwchar_t,
                                 __base: libc::c_int, __group: libc::c_int)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "354:1"]
        pub fn __wcstoul_internal(__nptr: *const __gwchar_t,
                                  __endptr: *mut *mut __gwchar_t,
                                  __base: libc::c_int, __group: libc::c_int)
         -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:19"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::internal::__va_list_tag;
    use super::stddef_h::size_t;
    use super::types_h::__ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "137:14"]
        pub static mut stdin: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "341:12"]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
                        _: ::std::ffi::VaList) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "486:1"]
        pub fn getc(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "603:1"]
        pub fn __getdelim(__lineptr: *mut *mut libc::c_char, __n: *mut size_t,
                          __delimiter: libc::c_int, __stream: *mut FILE)
         -> __ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "858:1"]
        pub fn __uflow(_: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "859:1"]
        pub fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/byteswap.h:19"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                    |
                    (__bsx as libc::c_int & 0xff as libc::c_int) <<
                        8 as libc::c_int) as __uint16_t;
    }
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
                   (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
                   (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
                   (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "69:15"]
    pub unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
        return ((__bsx as libc::c_ulonglong &
                     0xff00000000000000 as libc::c_ulonglong) >>
                    56 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff000000000000 as libc::c_ulonglong) >>
                        40 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff0000000000 as libc::c_ulonglong) >>
                        24 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff00000000 as libc::c_ulonglong) >>
                        8 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff000000 as libc::c_ulonglong) << 8 as libc::c_int
                    |
                    (__bsx as libc::c_ulonglong &
                         0xff0000 as libc::c_ulonglong) << 24 as libc::c_int |
                    (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong)
                        << 40 as libc::c_int |
                    (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong)
                        << 56 as libc::c_int) as __uint64_t;
    }
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/uintn-identity.h:19"]
pub mod uintn_identity_h {
    #[inline]
    #[c2rust::src_loc = "32:1"]
    pub unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t)
     -> __uint16_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "38:1"]
    pub unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t)
     -> __uint32_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "44:1"]
    pub unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t)
     -> __uint64_t {
        return __x;
    }
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/sys/stat.h:19"]
pub mod sys_stat_h {
    #[inline]
    #[c2rust::src_loc = "452:1"]
    pub unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                                  mut __statbuf: *mut stat) -> libc::c_int {
        return __xstat(_STAT_VER, __path, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "466:1"]
    pub unsafe extern "C" fn fstat(mut __fd: libc::c_int,
                                   mut __statbuf: *mut stat) -> libc::c_int {
        return __fxstat(_STAT_VER, __fd, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "501:1"]
    pub unsafe extern "C" fn stat64(mut __path: *const libc::c_char,
                                    mut __statbuf: *mut stat64)
     -> libc::c_int {
        return __xstat64(_STAT_VER, __path, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "515:1"]
    pub unsafe extern "C" fn fstat64(mut __fd: libc::c_int,
                                     mut __statbuf: *mut stat64)
     -> libc::c_int {
        return __fxstat64(_STAT_VER, __fd, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "473:1"]
    pub unsafe extern "C" fn fstatat(mut __fd: libc::c_int,
                                     mut __filename: *const libc::c_char,
                                     mut __statbuf: *mut stat,
                                     mut __flag: libc::c_int) -> libc::c_int {
        return __fxstatat(_STAT_VER, __fd, __filename, __statbuf, __flag);
    }
    #[inline]
    #[c2rust::src_loc = "522:1"]
    pub unsafe extern "C" fn fstatat64(mut __fd: libc::c_int,
                                       mut __filename: *const libc::c_char,
                                       mut __statbuf: *mut stat64,
                                       mut __flag: libc::c_int)
     -> libc::c_int {
        return __fxstatat64(_STAT_VER, __fd, __filename, __statbuf, __flag);
    }
    #[inline]
    #[c2rust::src_loc = "459:1"]
    pub unsafe extern "C" fn lstat(mut __path: *const libc::c_char,
                                   mut __statbuf: *mut stat) -> libc::c_int {
        return __lxstat(_STAT_VER, __path, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "508:1"]
    pub unsafe extern "C" fn lstat64(mut __path: *const libc::c_char,
                                     mut __statbuf: *mut stat64)
     -> libc::c_int {
        return __lxstat64(_STAT_VER, __path, __statbuf);
    }
    #[inline]
    #[c2rust::src_loc = "482:1"]
    pub unsafe extern "C" fn mknod(mut __path: *const libc::c_char,
                                   mut __mode: __mode_t, mut __dev: __dev_t)
     -> libc::c_int {
        return __xmknod(_MKNOD_VER, __path, __mode, &mut __dev);
    }
    #[c2rust::src_loc = "390:10"]
    pub const _MKNOD_VER: libc::c_int = 0 as libc::c_int;
    #[inline]
    #[c2rust::src_loc = "490:1"]
    pub unsafe extern "C" fn mknodat(mut __fd: libc::c_int,
                                     mut __path: *const libc::c_char,
                                     mut __mode: __mode_t, mut __dev: __dev_t)
     -> libc::c_int {
        return __xmknodat(_MKNOD_VER, __fd, __path, __mode, &mut __dev);
    }
    use super::stat_h::{stat, _STAT_VER_LINUX, _STAT_VER, stat64};
    use super::types_h::{__mode_t, __dev_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "409:1"]
        pub fn __xstat(__ver: libc::c_int, __filename: *const libc::c_char,
                       __stat_buf: *mut stat) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "406:1"]
        pub fn __fxstat(__ver: libc::c_int, __fildes: libc::c_int,
                        __stat_buf: *mut stat) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "430:1"]
        pub fn __xstat64(__ver: libc::c_int, __filename: *const libc::c_char,
                         __stat_buf: *mut stat64) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "428:1"]
        pub fn __fxstat64(__ver: libc::c_int, __fildes: libc::c_int,
                          __stat_buf: *mut stat64) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "415:1"]
        pub fn __fxstatat(__ver: libc::c_int, __fildes: libc::c_int,
                          __filename: *const libc::c_char,
                          __stat_buf: *mut stat, __flag: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "434:1"]
        pub fn __fxstatat64(__ver: libc::c_int, __fildes: libc::c_int,
                            __filename: *const libc::c_char,
                            __stat_buf: *mut stat64, __flag: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "412:1"]
        pub fn __lxstat(__ver: libc::c_int, __filename: *const libc::c_char,
                        __stat_buf: *mut stat) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn __lxstat64(__ver: libc::c_int, __filename: *const libc::c_char,
                          __stat_buf: *mut stat64) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "438:1"]
        pub fn __xmknod(__ver: libc::c_int, __path: *const libc::c_char,
                        __mode: __mode_t, __dev: *mut __dev_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "441:1"]
        pub fn __xmknodat(__ver: libc::c_int, __fd: libc::c_int,
                          __path: *const libc::c_char, __mode: __mode_t,
                          __dev: *mut __dev_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:19"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdio.h:19"]
pub mod bits_stdio_h {
    #[inline]
    #[c2rust::src_loc = "38:1"]
    pub unsafe extern "C" fn vprintf(mut __fmt: *const libc::c_char,
                                     mut __arg: ::std::ffi::VaList)
     -> libc::c_int {
        return vfprintf(stdout, __fmt, __arg.as_va_list());
    }
    #[inline]
    #[c2rust::src_loc = "46:1"]
    pub unsafe extern "C" fn getchar() -> libc::c_int { return getc(stdin); }
    #[inline]
    #[c2rust::src_loc = "65:1"]
    pub unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE)
     -> libc::c_int {
        return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as
                      libc::c_int as libc::c_long != 0 {
                   __uflow(__fp)
               } else {
                   let fresh0 = (*__fp)._IO_read_ptr;
                   (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
                   *(fresh0 as *mut libc::c_uchar) as libc::c_int
               };
    }
    #[inline]
    #[c2rust::src_loc = "72:1"]
    pub unsafe extern "C" fn getchar_unlocked() -> libc::c_int {
        return if ((*stdin)._IO_read_ptr >= (*stdin)._IO_read_end) as
                      libc::c_int as libc::c_long != 0 {
                   __uflow(stdin)
               } else {
                   let fresh1 = (*stdin)._IO_read_ptr;
                   (*stdin)._IO_read_ptr = (*stdin)._IO_read_ptr.offset(1);
                   *(fresh1 as *mut libc::c_uchar) as libc::c_int
               };
    }
    #[inline]
    #[c2rust::src_loc = "55:1"]
    pub unsafe extern "C" fn fgetc_unlocked(mut __fp: *mut FILE)
     -> libc::c_int {
        return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as
                      libc::c_int as libc::c_long != 0 {
                   __uflow(__fp)
               } else {
                   let fresh2 = (*__fp)._IO_read_ptr;
                   (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
                   *(fresh2 as *mut libc::c_uchar) as libc::c_int
               };
    }
    #[inline]
    #[c2rust::src_loc = "81:1"]
    pub unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
        return putc(__c, stdout);
    }
    #[inline]
    #[c2rust::src_loc = "90:1"]
    pub unsafe extern "C" fn fputc_unlocked(mut __c: libc::c_int,
                                            mut __stream: *mut FILE)
     -> libc::c_int {
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
    #[c2rust::src_loc = "100:1"]
    pub unsafe extern "C" fn putc_unlocked(mut __c: libc::c_int,
                                           mut __stream: *mut FILE)
     -> libc::c_int {
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
    #[c2rust::src_loc = "107:1"]
    pub unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int)
     -> libc::c_int {
        return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as
                      libc::c_int as libc::c_long != 0 {
                   __overflow(stdout, __c as libc::c_uchar as libc::c_int)
               } else {
                   let fresh5 = (*stdout)._IO_write_ptr;
                   (*stdout)._IO_write_ptr =
                       (*stdout)._IO_write_ptr.offset(1);
                   *fresh5 = __c as libc::c_char;
                   *fresh5 as libc::c_uchar as libc::c_int
               };
    }
    #[inline]
    #[c2rust::src_loc = "117:1"]
    pub unsafe extern "C" fn getline(mut __lineptr: *mut *mut libc::c_char,
                                     mut __n: *mut size_t,
                                     mut __stream: *mut FILE) -> __ssize_t {
        return __getdelim(__lineptr, __n, '\n' as i32, __stream);
    }
    #[inline]
    #[c2rust::src_loc = "127:1"]
    pub unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE)
     -> libc::c_int {
        return ((*__stream)._flags & _IO_EOF_SEEN != 0 as libc::c_int) as
                   libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "134:1"]
    pub unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE)
     -> libc::c_int {
        return ((*__stream)._flags & _IO_ERR_SEEN != 0 as libc::c_int) as
                   libc::c_int;
    }
    use super::internal::__va_list_tag;
    use super::stdio_h::{vfprintf, stdout, getc, stdin, __uflow, putc,
                         __overflow, __getdelim};
    use super::FILE_h::FILE;
    use super::stddef_h::size_t;
    use super::types_h::__ssize_t;
    use super::struct_FILE_h::{_IO_EOF_SEEN, _IO_ERR_SEEN};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/stdlib-float.h:19"]
pub mod stdlib_float_h {
    #[inline]
    #[c2rust::src_loc = "24:1"]
    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char)
     -> libc::c_double {
        return strtod(__nptr,
                      NULL as *mut libc::c_void as *mut *mut libc::c_char);
    }
    use super::stdlib_h::strtod;
    use super::stddef_h::NULL;
}
#[c2rust::header_src =
  "/usr/include/x86_64-linux-gnu/bits/stdlib-bsearch.h:19"]
pub mod stdlib_bsearch_h {
    #[inline]
    #[c2rust::src_loc = "19:1"]
    pub unsafe extern "C" fn bsearch(mut __key: *const libc::c_void,
                                     mut __base: *const libc::c_void,
                                     mut __nmemb: size_t, mut __size: size_t,
                                     mut __compar: __compar_fn_t)
     -> *mut libc::c_void {
        let mut __l: size_t = 0;
        let mut __u: size_t = 0;
        let mut __idx: size_t = 0;
        let mut __p = 0 as *const libc::c_void;
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
                                                     isize) as
                    *mut libc::c_void;
            __comparison =
                Some(__compar.expect("non-null function pointer")).expect("non-null function pointer")(__key,
                                                                                                       __p);
            if __comparison < 0 as libc::c_int {
                __u = __idx
            } else if __comparison > 0 as libc::c_int {
                __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong)
            } else { return __p as *mut libc::c_void }
        }
        return NULL as *mut libc::c_void;
    }
    use super::stddef_h::{size_t, NULL};
    use super::stdlib_h::__compar_fn_t;
}
#[c2rust::header_src = "/usr/include/ctype.h:19"]
pub mod ctype_h {
    #[inline]
    #[c2rust::src_loc = "206:1"]
    pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
                   *(*__ctype_tolower_loc()).offset(__c as isize)
               } else { __c };
    }
    #[inline]
    #[c2rust::src_loc = "212:1"]
    pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
                   *(*__ctype_toupper_loc()).offset(__c as isize)
               } else { __c };
    }
    use super::types_h::__int32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
    }
}
pub use self::internal::__va_list_tag;
pub use self::types_h::{__uint16_t, __int32_t, __uint32_t, __uint64_t,
                        __intmax_t, __uintmax_t, __dev_t, __uid_t, __gid_t,
                        __ino_t, __ino64_t, __mode_t, __nlink_t, __off_t,
                        __off64_t, __time_t, __blksize_t, __blkcnt_t,
                        __blkcnt64_t, __ssize_t, __syscall_slong_t,
                        __socklen_t};
pub use self::stddef_h::{size_t, NULL};
pub use self::struct_timespec_h::timespec;
pub use self::struct_iovec_h::iovec;
pub use self::socket_h::{socklen_t, msghdr, cmsghdr, __cmsg_nxthdr};
pub use self::dnsmasq_h::{u8_0, u16_0, skip_name, whine_malloc};
pub use self::dns_protocol_h::{dns_header, T_DNAME, T_SRV, T_KX, T_NXT, T_PX,
                               T_SIG, T_RT, T_AFSDB, T_RP, T_MX, T_MINFO,
                               T_PTR, T_MR, T_MG, T_MB, T_SOA, T_CNAME, T_MF,
                               T_MD, T_NS, C_IN, T_RRSIG, T_NSEC3, T_NSEC,
                               T_OPT};
pub use self::stat_h::{stat, stat64, _STAT_VER_LINUX, _STAT_VER};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_EOF_SEEN,
                              _IO_ERR_SEEN, _IO_wide_data, _IO_codecvt,
                              _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdlib_h::{__compar_fn_t, atoi, atol, atoll, strtod, strtol,
                         strtoll, free};
pub use self::stdint_h::{intmax_t, uintmax_t};
pub use self::inttypes_h::{__gwchar_t, strtoimax, strtoumax, wcstoimax,
                           wcstoumax, __strtol_internal, __strtoul_internal,
                           __wcstol_internal, __wcstoul_internal};
use self::stdio_h::{stdin, stdout, vfprintf, getc, putc, __getdelim, __uflow,
                    __overflow};
pub use self::byteswap_h::{__bswap_16, __bswap_32, __bswap_64};
pub use self::uintn_identity_h::{__uint16_identity, __uint32_identity,
                                 __uint64_identity};
pub use self::sys_stat_h::{stat, fstat, stat64, fstat64, fstatat, fstatat64,
                           lstat, lstat64, mknod, _MKNOD_VER, mknodat,
                           __xstat, __fxstat, __xstat64, __fxstat64,
                           __fxstatat, __fxstatat64, __lxstat, __lxstat64,
                           __xmknod, __xmknodat};
use self::string_h::{memcpy, memmove};
pub use self::bits_stdio_h::{vprintf, getchar, getc_unlocked,
                             getchar_unlocked, fgetc_unlocked, putchar,
                             fputc_unlocked, putc_unlocked, putchar_unlocked,
                             getline, feof_unlocked, ferror_unlocked};
pub use self::stdlib_float_h::atof;
pub use self::stdlib_bsearch_h::bsearch;
pub use self::ctype_h::{tolower, toupper, __ctype_tolower_loc,
                        __ctype_toupper_loc};
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
#[c2rust::src_loc = "23:1"]
unsafe extern "C" fn check_name(mut namep: *mut *mut libc::c_uchar,
                                mut header: *mut dns_header, mut plen: size_t,
                                mut fixup: libc::c_int,
                                mut rrs: *mut *mut libc::c_uchar,
                                mut rr_count: libc::c_int) -> libc::c_int {
    let mut ansp = *namep;
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
            let mut p = 0 as *mut libc::c_uchar;
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
            let mut len =
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
#[c2rust::src_loc = "109:1"]
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
    let mut pp = 0 as *mut libc::c_uchar;
    i = 0 as libc::c_int;
    while i <
              __bswap_16((*header).ancount) as libc::c_int +
                  __bswap_16((*header).nscount) as libc::c_int +
                  __bswap_16((*header).arcount) as libc::c_int {
        pp = p;
        p = skip_name(p, header, plen, 10 as libc::c_int);
        if p.is_null() { return 0 as libc::c_int }
        let mut t_cp = p;
        type_0 =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_0 = p;
        class =
            (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        p = p.offset(4 as libc::c_int as isize);
        let mut t_cp_1 = p;
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
            if class == C_IN {
                let mut d = 0 as *mut u16_0;
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
#[c2rust::src_loc = "160:1"]
pub unsafe extern "C" fn rrfilter(mut header: *mut dns_header,
                                  mut plen: size_t, mut mode: libc::c_int)
 -> size_t {
    static mut rrs: *mut *mut libc::c_uchar =
        0 as *const *mut libc::c_uchar as *mut *mut libc::c_uchar;
    static mut rr_sz: libc::c_int = 0 as libc::c_int;
    let mut p =
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
    let mut t_cp = p;
    qtype =
        (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int) <<
            8 as libc::c_int |
            *t_cp.offset(1 as libc::c_int as isize) as u16_0 as libc::c_int;
    p = p.offset(2 as libc::c_int as isize);
    let mut t_cp_0 = p;
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
        let mut pstart = p;
        let mut type_0: libc::c_int = 0;
        let mut class: libc::c_int = 0;
        p = skip_name(p, header, plen, 10 as libc::c_int);
        if p.is_null() { return plen }
        let mut t_cp_1 = p;
        type_0 =
            (*t_cp_1.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_1.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_2 = p;
        class =
            (*t_cp_2.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_2.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        p = p.offset(4 as libc::c_int as isize);
        let mut t_cp_3 = p;
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
                       type_0 != T_OPT {
                    current_block_36 = 2979737022853876585;
                } else { current_block_36 = 10692455896603418738; }
            } else if type_0 != T_NSEC && type_0 != T_NSEC3 &&
                          type_0 != T_RRSIG {
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
        let mut start = *rrs.offset(i as isize);
        let mut end =
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
#[c2rust::src_loc = "269:1"]
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
        [T_NS as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_MD as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         T_MF as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_CNAME as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         T_SOA as u16_0, 0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_MB as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         T_MG as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_MR as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         T_PTR as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_MINFO as u16_0,
         0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_MX as u16_0,
         2 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_RP as u16_0,
         0 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_AFSDB as u16_0,
         2 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_RT as u16_0,
         2 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_SIG as u16_0,
         18 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_PX as u16_0,
         2 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         T_NXT as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_KX as u16_0,
         2 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_SRV as u16_0,
         6 as libc::c_int as u16_0, 0 as libc::c_int as u16_0,
         -(1 as libc::c_int) as u16_0, T_DNAME as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0,
         0 as libc::c_int as u16_0, -(1 as libc::c_int) as u16_0];
    let mut p = rr_desc.as_mut_ptr();
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
#[c2rust::src_loc = "313:1"]
pub unsafe extern "C" fn expand_workspace(mut wkspc:
                                              *mut *mut *mut libc::c_uchar,
                                          mut szp: *mut libc::c_int,
                                          mut new: libc::c_int)
 -> libc::c_int {
    let mut p = 0 as *mut *mut libc::c_uchar;
    let mut old = *szp;
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
