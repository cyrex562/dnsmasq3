
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
pub struct iface_param {
    pub current: *mut dhcp_context,
    pub relay: *mut dhcp_relay,
    pub relay_local: in_addr,
    pub ind: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_param {
    pub ind: libc::c_int,
    pub matched: libc::c_int,
    pub netmask: in_addr,
    pub broadcast: in_addr,
    pub addr: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub c: *mut libc::c_uchar,
    pub p: *mut in_pktinfo,
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
unsafe extern "C" fn make_fd(mut port: libc::c_int) -> libc::c_int {
    let mut fd: libc::c_int =
        socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int,
               IPPROTO_UDP as libc::c_int);
    let mut saddr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut oneopt: libc::c_int = 1 as libc::c_int;
    let mut mtu: libc::c_int = 0 as libc::c_int;
    let mut tos: libc::c_int = 0xc0 as libc::c_int;
    if fd == -(1 as libc::c_int) {
        die(b"cannot create DHCP socket: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 2 as libc::c_int);
    }
    if fix_fd(fd) == 0 ||
           setsockopt(fd, IPPROTO_IP as libc::c_int, 10 as libc::c_int,
                      &mut mtu as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) ||
           setsockopt(fd, IPPROTO_IP as libc::c_int, 1 as libc::c_int,
                      &mut tos as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) ||
           setsockopt(fd, IPPROTO_IP as libc::c_int, 8 as libc::c_int,
                      &mut oneopt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) ||
           setsockopt(fd, 1 as libc::c_int, 6 as libc::c_int,
                      &mut oneopt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) {
        die(b"failed to set options on DHCP socket: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 2 as libc::c_int);
    }
    /* When bind-interfaces is set, there might be more than one dnsmasq
     instance binding port 67. That's OK if they serve different networks.
     Need to set REUSEADDR|REUSEPORT to make this possible.
     Handle the case that REUSEPORT is defined, but the kernel doesn't 
     support it. This handles the introduction of REUSEPORT on Linux. */
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
               != 0 {
        let mut rc: libc::c_int = 0 as libc::c_int;
        rc =
            setsockopt(fd, 1 as libc::c_int, 15 as libc::c_int,
                       &mut oneopt as *mut libc::c_int as *const libc::c_void,
                       ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                           as socklen_t);
        if rc == -(1 as libc::c_int) &&
               *__errno_location() == 92 as libc::c_int {
            rc = 0 as libc::c_int
        }
        if rc != -(1 as libc::c_int) {
            rc =
                setsockopt(fd, 1 as libc::c_int, 2 as libc::c_int,
                           &mut oneopt as *mut libc::c_int as
                               *const libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as socklen_t)
        }
        if rc == -(1 as libc::c_int) {
            die(b"failed to set SO_REUSE{ADDR|PORT} on DHCP socket: %s\x00" as
                    *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char, 2 as libc::c_int);
        }
    }
    memset(&mut saddr as *mut sockaddr_in as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
    saddr.sin_family = 2 as libc::c_int as sa_family_t;
    saddr.sin_port = __bswap_16(port as __uint16_t);
    saddr.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    if bind(fd,
            __CONST_SOCKADDR_ARG{__sockaddr__:
                                     &mut saddr as *mut sockaddr_in as
                                         *mut sockaddr,},
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                socklen_t) != 0 {
        die(b"failed to bind DHCP server socket: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 2 as libc::c_int);
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn dhcp_init() {
    (*dnsmasq_daemon).dhcpfd = make_fd((*dnsmasq_daemon).dhcp_server_port);
    if (*dnsmasq_daemon).enable_pxe != 0 {
        (*dnsmasq_daemon).pxefd = make_fd(4011 as libc::c_int)
    } else { (*dnsmasq_daemon).pxefd = -(1 as libc::c_int) };
}
#[no_mangle]
pub unsafe extern "C" fn dhcp_packet(mut now: time_t,
                                     mut pxe_fd: libc::c_int) {
    let mut fd: libc::c_int =
        if pxe_fd != 0 {
            (*dnsmasq_daemon).pxefd
        } else { (*dnsmasq_daemon).dhcpfd };
    let mut mess: *mut dhcp_packet = 0 as *mut dhcp_packet;
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut relay: *mut dhcp_relay = 0 as *mut dhcp_relay;
    let mut is_relay_reply: libc::c_int = 0 as libc::c_int;
    let mut tmp: *mut iname = 0 as *mut iname;
    let mut ifr: ifreq =
        ifreq{ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_2{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut msg: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut dest: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut cmptr: *mut cmsghdr = 0 as *mut cmsghdr;
    let mut iov: iovec = iovec{iov_base: 0 as *mut libc::c_void, iov_len: 0,};
    let mut sz: ssize_t = 0;
    let mut iface_index: libc::c_int = 0 as libc::c_int;
    let mut unicast_dest: libc::c_int = 0 as libc::c_int;
    let mut is_inform: libc::c_int = 0 as libc::c_int;
    let mut loopback: libc::c_int = 0 as libc::c_int;
    let mut rcvd_iface_index: libc::c_int = 0;
    let mut iface_addr: in_addr = in_addr{s_addr: 0,};
    let mut parm: iface_param =
        iface_param{current: 0 as *mut dhcp_context,
                    relay: 0 as *mut dhcp_relay,
                    relay_local: in_addr{s_addr: 0,},
                    ind: 0,};
    let mut recvtime: time_t = now;
    let mut arp_req: arpreq =
        arpreq{arp_pa: sockaddr{sa_family: 0, sa_data: [0; 14],},
               arp_ha: sockaddr{sa_family: 0, sa_data: [0; 14],},
               arp_flags: 0,
               arp_netmask: sockaddr{sa_family: 0, sa_data: [0; 14],},
               arp_dev: [0; 16],};
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut control_u: C2RustUnnamed_13 =
        C2RustUnnamed_13{align:
                             cmsghdr{cmsg_len: 0,
                                     cmsg_level: 0,
                                     cmsg_type: 0,
                                     __cmsg_data: [],},};
    let mut bridge: *mut dhcp_bridge = 0 as *mut dhcp_bridge;
    let mut alias: *mut dhcp_bridge = 0 as *mut dhcp_bridge;
    msg.msg_controllen =
        ::std::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong;
    msg.msg_control = control_u.control.as_mut_ptr() as *mut libc::c_void;
    msg.msg_name = &mut dest as *mut sockaddr_in as *mut libc::c_void;
    msg.msg_namelen =
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    msg.msg_iov = &mut (*dnsmasq_daemon).dhcp_packet;
    msg.msg_iovlen = 1 as libc::c_int as size_t;
    sz = recv_dhcp_packet(fd, &mut msg);
    if sz == -(1 as libc::c_int) as libc::c_long ||
           sz <
               (::std::mem::size_of::<dhcp_packet>() as
                    libc::c_ulong).wrapping_sub(::std::mem::size_of::<[u8_0; 312]>()
                                                    as libc::c_ulong) as
                   ssize_t {
        return
    }
    if ioctl(fd, 0x8906 as libc::c_int as libc::c_ulong,
             &mut tv as *mut timeval) == 0 as libc::c_int {
        recvtime = tv.tv_sec
    }
    if msg.msg_controllen >= ::std::mem::size_of::<cmsghdr>() as libc::c_ulong
       {
        cmptr =
            if msg.msg_controllen >=
                   ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                msg.msg_control as *mut cmsghdr
            } else { 0 as *mut cmsghdr };
        while !cmptr.is_null() {
            if (*cmptr).cmsg_level == IPPROTO_IP as libc::c_int &&
                   (*cmptr).cmsg_type == 8 as libc::c_int {
                let mut p: C2RustUnnamed_14 =
                    C2RustUnnamed_14{c: 0 as *mut libc::c_uchar,};
                p.c = (*cmptr).__cmsg_data.as_mut_ptr();
                iface_index = (*p.p).ipi_ifindex;
                if (*p.p).ipi_addr.s_addr != 0xffffffff as libc::c_uint {
                    unicast_dest = 1 as libc::c_int
                }
            }
            cmptr = __cmsg_nxthdr(&mut msg, cmptr)
        }
    }
    if indextoname((*dnsmasq_daemon).dhcpfd, iface_index,
                   ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) == 0 ||
           ioctl((*dnsmasq_daemon).dhcpfd,
                 0x8913 as libc::c_int as libc::c_ulong,
                 &mut ifr as *mut ifreq) != 0 as libc::c_int {
        return
    }
    mess = (*dnsmasq_daemon).dhcp_packet.iov_base as *mut dhcp_packet;
    loopback =
        ((*mess).giaddr.s_addr == 0 &&
             ifr.ifr_ifru.ifru_flags as libc::c_int &
                 IFF_LOOPBACK as libc::c_int != 0) as libc::c_int;
    /* ARP fiddling uses original interface even if we pretend to use a different one. */
    safe_strncpy(arp_req.arp_dev.as_mut_ptr(),
                 ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 16]>() as
                     libc::c_ulong);
    /* If the interface on which the DHCP request was received is an
     alias of some other interface (as specified by the
     --bridge-interface option), change ifr.ifr_name so that we look
     for DHCP contexts associated with the aliased interface instead
     of with the aliasing one. */
    rcvd_iface_index = iface_index;
    bridge = (*dnsmasq_daemon).bridges;
    while !bridge.is_null() {
        alias = (*bridge).alias;
        while !alias.is_null() {
            if wildcard_matchn((*alias).iface.as_mut_ptr(),
                               ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                               16 as libc::c_int) != 0 {
                iface_index =
                    if_nametoindex((*bridge).iface.as_mut_ptr()) as
                        libc::c_int;
                if iface_index == 0 {
                    my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                  4 as libc::c_int,
                              b"unknown interface %s in bridge-interface\x00"
                                  as *const u8 as *const libc::c_char,
                              (*bridge).iface.as_mut_ptr());
                    return
                } else {
                    safe_strncpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                                 (*bridge).iface.as_mut_ptr(),
                                 ::std::mem::size_of::<[libc::c_char; 16]>()
                                     as libc::c_ulong);
                    break ;
                }
            } else { alias = (*alias).next }
        }
        if !alias.is_null() { break ; }
        bridge = (*bridge).next
    }
    relay =
        relay_reply4((*dnsmasq_daemon).dhcp_packet.iov_base as
                         *mut dhcp_packet,
                     ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
    if !relay.is_null() {
        /* Reply from server, using us as relay. */
        rcvd_iface_index = (*relay).iface_index;
        if indextoname((*dnsmasq_daemon).dhcpfd, rcvd_iface_index,
                       ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) == 0 {
            return
        }
        is_relay_reply = 1 as libc::c_int;
        iov.iov_len = sz as size_t;
        safe_strncpy(arp_req.arp_dev.as_mut_ptr(),
                     ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 16]>() as
                         libc::c_ulong);
    } else {
        ifr.ifr_ifru.ifru_addr.sa_family = 2 as libc::c_int as sa_family_t;
        if ioctl((*dnsmasq_daemon).dhcpfd,
                 0x8915 as libc::c_int as libc::c_ulong,
                 &mut ifr as *mut ifreq) != -(1 as libc::c_int) {
            iface_addr =
                (*(&mut ifr.ifr_ifru.ifru_addr as *mut sockaddr as
                       *mut sockaddr_in)).sin_addr
        } else {
            if iface_check(2 as libc::c_int, 0 as *mut all_addr,
                           ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                           0 as *mut libc::c_int) != 0 {
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              4 as libc::c_int,
                          b"DHCP packet received on %s which has no address\x00"
                              as *const u8 as *const libc::c_char,
                          ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
            }
            return
        }
        tmp = (*dnsmasq_daemon).dhcp_except;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                return
            }
            tmp = (*tmp).next
        }
        /* unlinked contexts/relays are marked by context->current == context */
        context = (*dnsmasq_daemon).dhcp;
        while !context.is_null() {
            (*context).current = context;
            context = (*context).next
        }
        relay = (*dnsmasq_daemon).relay4;
        while !relay.is_null() {
            (*relay).current = relay;
            relay = (*relay).next
        }
        parm.current = 0 as *mut dhcp_context;
        parm.relay = 0 as *mut dhcp_relay;
        parm.relay_local.s_addr = 0 as libc::c_int as in_addr_t;
        parm.ind = iface_index;
        if iface_check(2 as libc::c_int,
                       &mut iface_addr as *mut in_addr as *mut all_addr,
                       ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                       0 as *mut libc::c_int) == 0 {
            /* If we failed to match the primary address of the interface, see if we've got a --listen-address
	     for a secondary */
            let mut match_0: match_param =
                match_param{ind: 0,
                            matched: 0,
                            netmask: in_addr{s_addr: 0,},
                            broadcast: in_addr{s_addr: 0,},
                            addr: in_addr{s_addr: 0,},};
            match_0.matched = 0 as libc::c_int;
            match_0.ind = iface_index;
            if (*dnsmasq_daemon).if_addrs.is_null() ||
                   iface_enumerate(2 as libc::c_int,
                                   &mut match_0 as *mut match_param as
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
                                                                          libc::c_int>>(Some(check_listen_addrs
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
                       == 0 || match_0.matched == 0 {
                return
            }
            iface_addr = match_0.addr;
            /* make sure secondary address gets priority in case
	     there is more than one address on the interface in the same subnet */
            complete_context(match_0.addr, iface_index,
                             0 as *mut libc::c_char, match_0.netmask,
                             match_0.broadcast,
                             &mut parm as *mut iface_param as
                                 *mut libc::c_void);
        }
        if iface_enumerate(2 as libc::c_int,
                           &mut parm as *mut iface_param as *mut libc::c_void,
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
                                                                  libc::c_int>>(Some(complete_context
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
               == 0 {
            return
        }
        /* We're relaying this request */
        if parm.relay_local.s_addr != 0 as libc::c_int as libc::c_uint &&
               relay_upstream4(parm.relay, mess, sz as size_t, iface_index) !=
                   0 {
            return
        }
        /* May have configured relay, but not DHCP server */
        if (*dnsmasq_daemon).dhcp.is_null() {
            return
        } /* lose any expired leases */
        lease_prune(0 as *mut dhcp_lease, now);
        iov.iov_len =
            dhcp_reply(parm.current, ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                       iface_index, sz as size_t, now, unicast_dest, loopback,
                       &mut is_inform, pxe_fd, iface_addr, recvtime);
        lease_update_file(now);
        lease_update_dns(0 as libc::c_int);
        if iov.iov_len == 0 as libc::c_int as libc::c_ulong { return }
    }
    msg.msg_name = &mut dest as *mut sockaddr_in as *mut libc::c_void;
    msg.msg_namelen =
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    msg.msg_control = 0 as *mut libc::c_void;
    msg.msg_controllen = 0 as libc::c_int as size_t;
    msg.msg_iov = &mut iov;
    iov.iov_base = (*dnsmasq_daemon).dhcp_packet.iov_base;
    /* packet buffer may have moved */
    mess = (*dnsmasq_daemon).dhcp_packet.iov_base as *mut dhcp_packet;
    if pxe_fd != 0 {
        if (*mess).ciaddr.s_addr != 0 as libc::c_int as libc::c_uint {
            dest.sin_addr = (*mess).ciaddr
        }
    } else if (*mess).giaddr.s_addr != 0 && is_relay_reply == 0 {
        /* Send to BOOTP relay  */
        dest.sin_port =
            __bswap_16((*dnsmasq_daemon).dhcp_server_port as __uint16_t);
        dest.sin_addr = (*mess).giaddr
    } else if (*mess).ciaddr.s_addr != 0 {
        /* If the client's idea of its own address tallys with
	 the source address in the request packet, we believe the
	 source port too, and send back to that.  If we're replying 
	 to a DHCPINFORM, trust the source address always. */
        if is_inform == 0 && dest.sin_addr.s_addr != (*mess).ciaddr.s_addr ||
               dest.sin_port as libc::c_int == 0 as libc::c_int ||
               dest.sin_addr.s_addr == 0 as libc::c_int as libc::c_uint ||
               is_relay_reply != 0 {
            dest.sin_port =
                __bswap_16((*dnsmasq_daemon).dhcp_client_port as __uint16_t);
            dest.sin_addr = (*mess).ciaddr
        }
    } else {
        /* fill cmsg for outbound interface (both broadcast & unicast) */
        let mut pkt: *mut in_pktinfo = 0 as *mut in_pktinfo;
        msg.msg_control = control_u.control.as_mut_ptr() as *mut libc::c_void;
        msg.msg_controllen =
            ::std::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong;
        cmptr =
            if msg.msg_controllen >=
                   ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                msg.msg_control as *mut cmsghdr
            } else { 0 as *mut cmsghdr };
        pkt = (*cmptr).__cmsg_data.as_mut_ptr() as *mut in_pktinfo;
        (*pkt).ipi_ifindex = rcvd_iface_index;
        (*pkt).ipi_spec_dst.s_addr = 0 as libc::c_int as in_addr_t;
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
        (*cmptr).cmsg_type = 8 as libc::c_int;
        if __bswap_16((*mess).flags) as libc::c_int & 0x8000 as libc::c_int !=
               0 || (*mess).hlen as libc::c_int == 0 as libc::c_int ||
               (*mess).hlen as libc::c_ulong >
                   ::std::mem::size_of::<[libc::c_char; 14]>() as
                       libc::c_ulong ||
               (*mess).htype as libc::c_int == 0 as libc::c_int {
            /* broadcast to 255.255.255.255 (or mac address invalid) */
            dest.sin_addr.s_addr = 0xffffffff as libc::c_uint;
            dest.sin_port =
                __bswap_16((*dnsmasq_daemon).dhcp_client_port as __uint16_t)
        } else {
            /* unicast to unconfigured client. Inject mac address direct into ARP cache.
          struct sockaddr limits size to 14 bytes. */
            dest.sin_addr = (*mess).yiaddr;
            dest.sin_port =
                __bswap_16((*dnsmasq_daemon).dhcp_client_port as __uint16_t);
            memcpy(&mut arp_req.arp_pa as *mut sockaddr as *mut libc::c_void,
                   &mut dest as *mut sockaddr_in as *const libc::c_void,
                   ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
            arp_req.arp_ha.sa_family = (*mess).htype as sa_family_t;
            memcpy(arp_req.arp_ha.sa_data.as_mut_ptr() as *mut libc::c_void,
                   (*mess).chaddr.as_mut_ptr() as *const libc::c_void,
                   (*mess).hlen as libc::c_ulong);
            /* interface name already copied in */
            arp_req.arp_flags = 0x2 as libc::c_int;
            if ioctl((*dnsmasq_daemon).dhcpfd,
                     0x8955 as libc::c_int as libc::c_ulong,
                     &mut arp_req as *mut arpreq) == -(1 as libc::c_int) {
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              3 as libc::c_int,
                          b"ARP-cache injection failed: %s\x00" as *const u8
                              as *const libc::c_char,
                          strerror(*__errno_location()));
            }
        }
    }
    while retry_send(sendmsg(fd, &mut msg, 0 as libc::c_int)) != 0 { }
    /* This can fail when, eg, iptables DROPS destination 255.255.255.255 */
    if *__errno_location() != 0 as libc::c_int {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 4 as libc::c_int,
                  b"Error sending DHCP packet to %s: %s\x00" as *const u8 as
                      *const libc::c_char, inet_ntoa(dest.sin_addr),
                  strerror(*__errno_location()));
    };
}
/* check against secondary interface addresses */
unsafe extern "C" fn check_listen_addrs(mut local: in_addr,
                                        mut if_index: libc::c_int,
                                        mut label: *mut libc::c_char,
                                        mut netmask: in_addr,
                                        mut broadcast: in_addr,
                                        mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut param: *mut match_param = vparam as *mut match_param;
    let mut tmp: *mut iname = 0 as *mut iname;
    if if_index == (*param).ind {
        tmp = (*dnsmasq_daemon).if_addrs;
        while !tmp.is_null() {
            if (*tmp).addr.sa.sa_family as libc::c_int == 2 as libc::c_int &&
                   (*tmp).addr.in_0.sin_addr.s_addr == local.s_addr {
                (*param).matched = 1 as libc::c_int;
                (*param).addr = local;
                (*param).netmask = netmask;
                (*param).broadcast = broadcast;
                break ;
            } else { tmp = (*tmp).next }
        }
    }
    return 1 as libc::c_int;
}
/* This is a complex routine: it gets called with each (address,netmask,broadcast) triple 
   of each interface (and any relay address) and does the  following things:

   1) Discards stuff for interfaces other than the one on which a DHCP packet just arrived.
   2) Fills in any netmask and broadcast addresses which have not been explicitly configured.
   3) Fills in local (this host) and router (this host or relay) addresses.
   4) Links contexts which are valid for hosts directly connected to the arrival interface on ->current.

   Note that the current chain may be superseded later for configured hosts or those coming via gateways. */
unsafe extern "C" fn guess_range_netmask(mut addr: in_addr,
                                         mut netmask: in_addr) {
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    context = (*dnsmasq_daemon).dhcp;
    while !context.is_null() {
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 1 as libc::c_int == 0 &&
               (is_same_net(addr, (*context).start, netmask) != 0 ||
                    is_same_net(addr, (*context).end, netmask) != 0) {
            if (*context).netmask.s_addr != netmask.s_addr &&
                   !(is_same_net(addr, (*context).start, netmask) != 0 &&
                         is_same_net(addr, (*context).end, netmask) != 0) {
                strcpy((*dnsmasq_daemon).dhcp_buff,
                       inet_ntoa((*context).start));
                strcpy((*dnsmasq_daemon).dhcp_buff2,
                       inet_ntoa((*context).end));
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              4 as libc::c_int,
                          b"DHCP range %s -- %s is not consistent with netmask %s\x00"
                              as *const u8 as *const libc::c_char,
                          (*dnsmasq_daemon).dhcp_buff,
                          (*dnsmasq_daemon).dhcp_buff2, inet_ntoa(netmask));
            }
            (*context).netmask = netmask
        }
        context = (*context).next
    };
}
unsafe extern "C" fn complete_context(mut local: in_addr,
                                      mut if_index: libc::c_int,
                                      mut label: *mut libc::c_char,
                                      mut netmask: in_addr,
                                      mut broadcast: in_addr,
                                      mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut relay: *mut dhcp_relay = 0 as *mut dhcp_relay;
    let mut param: *mut iface_param = vparam as *mut iface_param;
    let mut share: *mut shared_network = 0 as *mut shared_network;
    let mut current_block_14: u64;
    share = (*dnsmasq_daemon).shared_networks;
    while !share.is_null() {
        if !((*share).shared_addr.s_addr == 0 as libc::c_int as libc::c_uint)
           {
            if (*share).if_index != 0 as libc::c_int {
                if (*share).if_index != if_index {
                    current_block_14 = 17778012151635330486;
                } else { current_block_14 = 13536709405535804910; }
            } else if (*share).match_addr.s_addr != local.s_addr {
                current_block_14 = 17778012151635330486;
            } else { current_block_14 = 13536709405535804910; }
            match current_block_14 {
                17778012151635330486 => { }
                _ => {
                    context = (*dnsmasq_daemon).dhcp;
                    while !context.is_null() {
                        if (*context).netmask.s_addr !=
                               0 as libc::c_int as libc::c_uint &&
                               is_same_net((*share).shared_addr,
                                           (*context).start,
                                           (*context).netmask) != 0 &&
                               is_same_net((*share).shared_addr,
                                           (*context).end, (*context).netmask)
                                   != 0 {
                            /* link it onto the current chain if we've not seen it before */
                            if (*context).current == context {
                                /* For a shared network, we have no way to guess what the default route should be. */
                                (*context).router.s_addr =
                                    0 as libc::c_int as
                                        in_addr_t; /* Use configured address for Server Identifier */
                                (*context).local = local;
                                (*context).current = (*param).current;
                                (*param).current = context
                            }
                            if (*context).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 2 as libc::c_int ==
                                   0 {
                                (*context).broadcast.s_addr =
                                    (*context).start.s_addr |
                                        !(*context).netmask.s_addr
                            }
                        }
                        context = (*context).next
                    }
                }
            }
        }
        share = (*share).next
    }
    guess_range_netmask(local, netmask);
    context = (*dnsmasq_daemon).dhcp;
    while !context.is_null() {
        if (*context).netmask.s_addr != 0 as libc::c_int as libc::c_uint &&
               is_same_net(local, (*context).start, (*context).netmask) != 0
               && is_same_net(local, (*context).end, (*context).netmask) != 0
           {
            /* link it onto the current chain if we've not seen it before */
            if if_index == (*param).ind && (*context).current == context {
                (*context).router = local;
                (*context).local = local;
                (*context).current = (*param).current;
                (*param).current = context
            }
            if (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 2 as libc::c_int == 0 {
                if is_same_net(broadcast, (*context).start,
                               (*context).netmask) != 0 {
                    (*context).broadcast = broadcast
                } else {
                    (*context).broadcast.s_addr =
                        (*context).start.s_addr | !(*context).netmask.s_addr
                }
            }
        }
        context = (*context).next
    }
    relay = (*dnsmasq_daemon).relay4;
    while !relay.is_null() {
        if if_index == (*param).ind &&
               (*relay).local.addr4.s_addr == local.s_addr &&
               (*relay).current == relay &&
               ((*param).relay_local.s_addr ==
                    0 as libc::c_int as libc::c_uint ||
                    (*param).relay_local.s_addr == local.s_addr) {
            (*relay).current = (*param).relay;
            (*param).relay = relay;
            (*param).relay_local = local
        }
        relay = (*relay).next
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn address_available(mut context: *mut dhcp_context,
                                           mut taddr: in_addr,
                                           mut netids: *mut dhcp_netid)
 -> *mut dhcp_context {
    /* Check is an address is OK for this network, check all
     possible ranges. Make sure that the address isn't in use
     by the server itself. */
    let mut start: libc::c_uint = 0;
    let mut end: libc::c_uint = 0;
    let mut addr: libc::c_uint = __bswap_32(taddr.s_addr);
    let mut tmp: *mut dhcp_context = 0 as *mut dhcp_context;
    tmp = context;
    while !tmp.is_null() {
        if taddr.s_addr == (*context).router.s_addr {
            return 0 as *mut dhcp_context
        }
        tmp = (*tmp).current
    }
    tmp = context;
    while !tmp.is_null() {
        start = __bswap_32((*tmp).start.s_addr);
        end = __bswap_32((*tmp).end.s_addr);
        if (*tmp).flags as libc::c_uint &
               ((1 as libc::c_uint) << 0 as libc::c_int |
                    (1 as libc::c_uint) << 3 as libc::c_int) == 0 &&
               addr >= start && addr <= end &&
               match_netid((*tmp).filter, netids, 1 as libc::c_int) != 0 {
            return tmp
        }
        tmp = (*tmp).current
    }
    return 0 as *mut dhcp_context;
}
#[no_mangle]
pub unsafe extern "C" fn narrow_context(mut context: *mut dhcp_context,
                                        mut taddr: in_addr,
                                        mut netids: *mut dhcp_netid)
 -> *mut dhcp_context {
    /* We start of with a set of possible contexts, all on the current physical interface.
     These are chained on ->current.
     Here we have an address, and return the actual context corresponding to that
     address. Note that none may fit, if the address came a dhcp-host and is outside
     any dhcp-range. In that case we return a static range if possible, or failing that,
     any context on the correct subnet. (If there's more than one, this is a dodgy 
     configuration: maybe there should be a warning.) */
    let mut tmp: *mut dhcp_context = 0 as *mut dhcp_context;
    tmp = address_available(context, taddr, netids);
    if tmp.is_null() {
        tmp = context;
        while !tmp.is_null() {
            if match_netid((*tmp).filter, netids, 1 as libc::c_int) != 0 &&
                   is_same_net(taddr, (*tmp).start, (*tmp).netmask) != 0 &&
                   (*tmp).flags as libc::c_uint &
                       (1 as libc::c_uint) << 0 as libc::c_int != 0 {
                break ;
            }
            tmp = (*tmp).current
        }
        if tmp.is_null() {
            tmp = context;
            while !tmp.is_null() {
                if match_netid((*tmp).filter, netids, 1 as libc::c_int) != 0
                       &&
                       is_same_net(taddr, (*tmp).start, (*tmp).netmask) != 0
                       &&
                       (*tmp).flags as libc::c_uint &
                           (1 as libc::c_uint) << 3 as libc::c_int == 0 {
                    break ;
                }
                tmp = (*tmp).current
            }
        }
    }
    /* Only one context allowed now */
    if !tmp.is_null() { (*tmp).current = 0 as *mut dhcp_context }
    return tmp;
}
#[no_mangle]
pub unsafe extern "C" fn config_find_by_address(mut configs: *mut dhcp_config,
                                                mut addr: in_addr)
 -> *mut dhcp_config {
    let mut config: *mut dhcp_config = 0 as *mut dhcp_config;
    config = configs;
    while !config.is_null() {
        if (*config).flags & 32 as libc::c_int as libc::c_uint != 0 &&
               (*config).addr.s_addr == addr.s_addr {
            return config
        }
        config = (*config).next
    }
    return 0 as *mut dhcp_config;
}
/* Check if and address is in use by sending ICMP ping.
   This wrapper handles a cache and load-limiting.
   Return is NULL is address in use, or a pointer to a cache entry
   recording that it isn't. */
#[no_mangle]
pub unsafe extern "C" fn do_icmp_ping(mut now: time_t, mut addr: in_addr,
                                      mut hash: libc::c_uint,
                                      mut loopback: libc::c_int)
 -> *mut ping_result {
    static mut dummy: ping_result =
        ping_result{addr: in_addr{s_addr: 0,},
                    time: 0,
                    hash: 0,
                    next: 0 as *const ping_result as *mut ping_result,};
    let mut r: *mut ping_result = 0 as *mut ping_result;
    let mut victim: *mut ping_result = 0 as *mut ping_result;
    let mut count: libc::c_int = 0;
    let mut max: libc::c_int =
        (0.6f64 *
             (30 as libc::c_int as libc::c_float /
                  3 as libc::c_int as libc::c_float) as libc::c_double) as
            libc::c_int;
    /* check if we failed to ping addr sometime in the last
     PING_CACHE_TIME seconds. If so, assume the same situation still exists.
     This avoids problems when a stupid client bangs
     on us repeatedly. As a final check, if we did more
     than 60% of the possible ping checks in the last 
     PING_CACHE_TIME, we are in high-load mode, so don't do any more. */
    count = 0 as libc::c_int; /* old record */
    r = (*dnsmasq_daemon).ping_results;
    while !r.is_null() {
        if difftime(now, (*r).time) >
               30 as libc::c_int as libc::c_float as libc::c_double {
            victim = r
        } else { count += 1; if (*r).addr.s_addr == addr.s_addr { return r } }
        r = (*r).next
    }
    /* didn't find cached entry */
    if count >= max ||
           (*dnsmasq_daemon).options[(21 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (21 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 || loopback != 0 {
        /* overloaded, or configured not to check, loopback interface, return "not in use" */
        dummy.hash = hash; /* address in use. */
        return &mut dummy
    } else if icmp_ping(addr) != 0 {
        return 0 as *mut ping_result
    } else {
        /* at this point victim may hold an expired record */
        if victim.is_null() {
            victim =
                whine_malloc(::std::mem::size_of::<ping_result>() as
                                 libc::c_ulong) as *mut ping_result;
            if !victim.is_null() {
                (*victim).next = (*dnsmasq_daemon).ping_results;
                (*dnsmasq_daemon).ping_results = victim
            }
        }
        /* record that this address is OK for 30s 
	 without more ping checks */
        if !victim.is_null() {
            (*victim).addr = addr;
            (*victim).time = now;
            (*victim).hash = hash
        }
        return victim
    };
}
#[no_mangle]
pub unsafe extern "C" fn address_allocate(mut context: *mut dhcp_context,
                                          mut addrp: *mut in_addr,
                                          mut hwaddr: *mut libc::c_uchar,
                                          mut hw_len: libc::c_int,
                                          mut netids: *mut dhcp_netid,
                                          mut now: time_t,
                                          mut loopback: libc::c_int)
 -> libc::c_int {
    /* Find a free address: exclude anything in use and anything allocated to
     a particular hwaddr/clientid/hostname in our configuration.
     Try to return from contexts which match netids first. */
    let mut start: in_addr = in_addr{s_addr: 0,};
    let mut addr: in_addr = in_addr{s_addr: 0,};
    let mut c: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut d: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut i: libc::c_int = 0;
    let mut pass: libc::c_int = 0;
    let mut j: libc::c_uint = 0;
    /* hash hwaddr: use the SDBM hashing algorithm.  Seems to give good
     dispersal even with similarly-valued "strings". */
    j = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < hw_len {
        j =
            (*hwaddr.offset(i as isize) as
                 libc::c_uint).wrapping_add(j <<
                                                6 as
                                                    libc::c_int).wrapping_add(j
                                                                                  <<
                                                                                  16
                                                                                      as
                                                                                      libc::c_int).wrapping_sub(j);
        i += 1
    }
    /* j == 0 is marker */
    if j == 0 as libc::c_int as libc::c_uint {
        j = 1 as libc::c_int as libc::c_uint
    }
    pass = 0 as libc::c_int;
    while pass <= 1 as libc::c_int {
        c = context;
        while !c.is_null() {
            if !((*c).flags as libc::c_uint &
                     ((1 as libc::c_uint) << 0 as libc::c_int |
                          (1 as libc::c_uint) << 3 as libc::c_int) != 0) {
                if !(match_netid((*c).filter, netids, pass) == 0) {
                    if (*dnsmasq_daemon).options[(34 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (34 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           != 0 {
                        /* seed is largest extant lease addr in this context */
                        start = lease_find_max_addr(c)
                    } else {
                        /* pick a seed based on hwaddr */
                        start.s_addr =
                            __bswap_32(__bswap_32((*c).start.s_addr).wrapping_add(j.wrapping_add((*c).addr_epoch).wrapping_rem((1
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_uint).wrapping_add(__bswap_32((*c).end.s_addr)).wrapping_sub(__bswap_32((*c).start.s_addr)))))
                    }
                    /* iterate until we find a free address. */
                    addr = start;
                    loop  {
                        /* eliminate addresses in use by the server. */
                        d = context;
                        while !d.is_null() {
                            if addr.s_addr == (*d).router.s_addr { break ; }
                            d = (*d).current
                        }
                        /* Addresses which end in .255 and .0 are broken in Windows even when using 
	       supernetting. ie dhcp-range=192.168.0.1,192.168.1.254,255,255,254.0
	       then 192.168.0.255 is a valid IP address, but not for Windows as it's
	       in the class C range. See  KB281579. We therefore don't allocate these 
	       addresses to avoid hard-to-diagnose problems. Thanks Bill. */
                        if d.is_null() && lease_find_by_addr(addr).is_null()
                               &&
                               config_find_by_address((*dnsmasq_daemon).dhcp_conf,
                                                      addr).is_null() &&
                               (!(__bswap_32(addr.s_addr) &
                                      0xe0000000 as libc::c_uint ==
                                      0xc0000000 as libc::c_uint) ||
                                    __bswap_32(addr.s_addr) &
                                        0xff as libc::c_int as libc::c_uint !=
                                        0xff as libc::c_int as libc::c_uint &&
                                        __bswap_32(addr.s_addr) &
                                            0xff as libc::c_int as
                                                libc::c_uint !=
                                            0 as libc::c_int as libc::c_uint)
                           {
                            /* in consec-ip mode, skip addresses equal to
		   the number of addresses rejected by clients. This
		   should avoid the same client being offered the same
		   address after it has rjected it. */
                            if (*dnsmasq_daemon).options[(34 as libc::c_int as
                                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                               as
                                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_ulong))
                                                             as usize] &
                                   (1 as libc::c_uint) <<
                                       (34 as libc::c_int as
                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong))
                                   != 0 && (*c).addr_epoch != 0 {
                                (*c).addr_epoch =
                                    (*c).addr_epoch.wrapping_sub(1)
                            } else {
                                let mut r: *mut ping_result =
                                    0 as *mut ping_result;
                                r = do_icmp_ping(now, addr, j, loopback);
                                if !r.is_null() {
                                    /* consec-ip mode: we offered this address for another client
			   (different hash) recently, don't offer it to this one. */
                                    if (*dnsmasq_daemon).options[(34 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                       as
                                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                       as
                                                                                                                                       libc::c_ulong))
                                                                     as usize]
                                           &
                                           (1 as libc::c_uint) <<
                                               (34 as libc::c_int as
                                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong))
                                           == 0 || (*r).hash == j {
                                        *addrp = addr;
                                        return 1 as libc::c_int
                                    }
                                } else if (*dnsmasq_daemon).options[(34 as
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
                                                  (34 as libc::c_int as
                                                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                        as
                                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_ulong))
                                              == 0 {
                                    (*c).addr_epoch =
                                        (*c).addr_epoch.wrapping_add(1)
                                }
                            }
                        }
                        addr.s_addr =
                            __bswap_32(__bswap_32(addr.s_addr).wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint));
                        if addr.s_addr ==
                               __bswap_32(__bswap_32((*c).end.s_addr).wrapping_add(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint))
                           {
                            addr = (*c).start
                        }
                        if !(addr.s_addr != start.s_addr) { break ; }
                    }
                }
            }
            c = (*c).current
        }
        pass += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dhcp_read_ethers() {
    let mut f: *mut FILE =
        fopen(b"/etc/ethers\x00" as *const u8 as *const libc::c_char,
              b"r\x00" as *const u8 as *const libc::c_char);
    let mut flags: libc::c_uint = 0;
    let mut buff: *mut libc::c_char = (*dnsmasq_daemon).namebuff;
    let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addr: in_addr = in_addr{s_addr: 0,};
    let mut hwaddr: [libc::c_uchar; 6] = [0; 6];
    let mut up: *mut *mut dhcp_config = 0 as *mut *mut dhcp_config;
    let mut tmp: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut config: *mut dhcp_config = 0 as *mut dhcp_config;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut lineno: libc::c_int = 0 as libc::c_int;
    /* address in use: perturb address selection so that we are
			   less likely to try this address again. */
    addr.s_addr = 0 as libc::c_int as in_addr_t; /* eliminate warning */
    if f.is_null() {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 3 as libc::c_int,
                  b"failed to read %s: %s\x00" as *const u8 as
                      *const libc::c_char,
                  b"/etc/ethers\x00" as *const u8 as *const libc::c_char,
                  strerror(*__errno_location()));
        return
    }
    /* This can be called again on SIGHUP, so remove entries created last time round. */
    up = &mut (*dnsmasq_daemon).dhcp_conf;
    config = (*dnsmasq_daemon).dhcp_conf;
    while !config.is_null() {
        tmp = (*config).next;
        if (*config).flags & 256 as libc::c_int as libc::c_uint != 0 {
            *up = tmp;
            /* cannot have a clid */
            if (*config).flags & 16 as libc::c_int as libc::c_uint != 0 {
                free((*config).hostname as *mut libc::c_void);
            }
            free((*config).hwaddr as *mut libc::c_void);
            free(config as *mut libc::c_void);
        } else { up = &mut (*config).next }
        config = tmp
    }
    while !fgets(buff, 1025 as libc::c_int, f).is_null() {
        let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
        lineno += 1;
        while strlen(buff) > 0 as libc::c_int as libc::c_ulong &&
                  *(*__ctype_b_loc()).offset(*buff.offset(strlen(buff).wrapping_sub(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong)
                                                              as isize) as
                                                 libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
            *buff.offset(strlen(buff).wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong) as
                             isize) = 0 as libc::c_int as libc::c_char
        }
        if *buff as libc::c_int == '#' as i32 ||
               *buff as libc::c_int == '+' as i32 ||
               *buff as libc::c_int == 0 as libc::c_int {
            continue ;
        }
        ip = buff;
        while *ip as libc::c_int != 0 &&
                  *(*__ctype_b_loc()).offset(*ip as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      == 0 {
            ip = ip.offset(1)
        }
        while *ip as libc::c_int != 0 &&
                  *(*__ctype_b_loc()).offset(*ip as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
            *ip = 0 as libc::c_int as libc::c_char;
            ip = ip.offset(1)
        }
        if *ip == 0 ||
               parse_hex(buff, hwaddr.as_mut_ptr(), 6 as libc::c_int,
                         0 as *mut libc::c_uint, 0 as *mut libc::c_int) !=
                   6 as libc::c_int {
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          3 as libc::c_int,
                      b"bad line at %s line %d\x00" as *const u8 as
                          *const libc::c_char,
                      b"/etc/ethers\x00" as *const u8 as *const libc::c_char,
                      lineno);
        } else {
            /* check for name or dotted-quad */
            cp = ip;
            while *cp != 0 {
                if !(*cp as libc::c_int == '.' as i32 ||
                         *cp as libc::c_int >= '0' as i32 &&
                             *cp as libc::c_int <= '9' as i32) {
                    break ;
                }
                cp = cp.offset(1)
            }
            if *cp == 0 {
                addr.s_addr = inet_addr(ip);
                if addr.s_addr == -(1 as libc::c_int) as in_addr_t {
                    my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                  3 as libc::c_int,
                              b"bad address at %s line %d\x00" as *const u8 as
                                  *const libc::c_char,
                              b"/etc/ethers\x00" as *const u8 as
                                  *const libc::c_char, lineno);
                    continue ;
                } else {
                    flags = 32 as libc::c_int as libc::c_uint;
                    config = (*dnsmasq_daemon).dhcp_conf;
                    while !config.is_null() {
                        if (*config).flags & 32 as libc::c_int as libc::c_uint
                               != 0 && (*config).addr.s_addr == addr.s_addr {
                            break ;
                        }
                        config = (*config).next
                    }
                }
            } else {
                let mut nomem: libc::c_int = 0;
                host = canonicalise(ip, &mut nomem);
                if host.is_null() || legal_hostname(host) == 0 {
                    if nomem == 0 {
                        my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                      3 as libc::c_int,
                                  b"bad name at %s line %d\x00" as *const u8
                                      as *const libc::c_char,
                                  b"/etc/ethers\x00" as *const u8 as
                                      *const libc::c_char, lineno);
                    }
                    free(host as *mut libc::c_void);
                    continue ;
                } else {
                    flags = 16 as libc::c_int as libc::c_uint;
                    config = (*dnsmasq_daemon).dhcp_conf;
                    while !config.is_null() {
                        if (*config).flags & 16 as libc::c_int as libc::c_uint
                               != 0 &&
                               hostname_isequal((*config).hostname, host) != 0
                           {
                            break ;
                        }
                        config = (*config).next
                    }
                }
            }
            if !config.is_null() &&
                   (*config).flags & 256 as libc::c_int as libc::c_uint != 0 {
                my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                              3 as libc::c_int,
                          b"ignoring %s line %d, duplicate name or IP address\x00"
                              as *const u8 as *const libc::c_char,
                          b"/etc/ethers\x00" as *const u8 as
                              *const libc::c_char, lineno);
            } else {
                if config.is_null() {
                    config = (*dnsmasq_daemon).dhcp_conf;
                    while !config.is_null() {
                        let mut conf_addr: *mut hwaddr_config =
                            (*config).hwaddr;
                        if !conf_addr.is_null() && (*conf_addr).next.is_null()
                               &&
                               (*conf_addr).wildcard_mask ==
                                   0 as libc::c_int as libc::c_uint &&
                               (*conf_addr).hwaddr_len == 6 as libc::c_int &&
                               ((*conf_addr).hwaddr_type == 1 as libc::c_int
                                    ||
                                    (*conf_addr).hwaddr_type ==
                                        0 as libc::c_int) &&
                               memcmp((*conf_addr).hwaddr.as_mut_ptr() as
                                          *const libc::c_void,
                                      hwaddr.as_mut_ptr() as
                                          *const libc::c_void,
                                      6 as libc::c_int as libc::c_ulong) ==
                                   0 as libc::c_int {
                            break ;
                        }
                        config = (*config).next
                    }
                    if config.is_null() {
                        config =
                            whine_malloc(::std::mem::size_of::<dhcp_config>()
                                             as libc::c_ulong) as
                                *mut dhcp_config;
                        if config.is_null() { continue ; }
                        (*config).flags = 256 as libc::c_int as libc::c_uint;
                        (*config).hwaddr = 0 as *mut hwaddr_config;
                        (*config).domain = 0 as *mut libc::c_char;
                        (*config).netid = 0 as *mut dhcp_netid_list;
                        (*config).next = (*dnsmasq_daemon).dhcp_conf;
                        (*dnsmasq_daemon).dhcp_conf = config
                    }
                    (*config).flags |= flags;
                    if flags & 16 as libc::c_int as libc::c_uint != 0 {
                        (*config).hostname = host;
                        host = 0 as *mut libc::c_char
                    }
                    if flags & 32 as libc::c_int as libc::c_uint != 0 {
                        (*config).addr = addr
                    }
                }
                (*config).flags |= 128 as libc::c_int as libc::c_uint;
                if (*config).hwaddr.is_null() {
                    (*config).hwaddr =
                        whine_malloc(::std::mem::size_of::<hwaddr_config>() as
                                         libc::c_ulong) as *mut hwaddr_config
                }
                if !(*config).hwaddr.is_null() {
                    memcpy((*(*config).hwaddr).hwaddr.as_mut_ptr() as
                               *mut libc::c_void,
                           hwaddr.as_mut_ptr() as *const libc::c_void,
                           6 as libc::c_int as libc::c_ulong);
                    (*(*config).hwaddr).hwaddr_len = 6 as libc::c_int;
                    (*(*config).hwaddr).hwaddr_type = 1 as libc::c_int;
                    (*(*config).hwaddr).wildcard_mask =
                        0 as libc::c_int as libc::c_uint;
                    (*(*config).hwaddr).next = 0 as *mut hwaddr_config
                }
                count += 1;
                free(host as *mut libc::c_void);
            }
        }
    }
    fclose(f);
    my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
              b"read %s - %d addresses\x00" as *const u8 as
                  *const libc::c_char,
              b"/etc/ethers\x00" as *const u8 as *const libc::c_char, count);
}
/* If we've not found a hostname any other way, try and see if there's one in /etc/hosts
   for this address. If it has a domain part, that must match the set domain and
   it gets stripped. The set of legal domain names is bigger than the set of legal hostnames
   so check here that the domain name is legal as a hostname. 
   NOTE: we're only allowed to overwrite daemon->dhcp_buff if we succeed. */
#[no_mangle]
pub unsafe extern "C" fn host_from_dns(mut addr: in_addr)
 -> *mut libc::c_char {
    let mut lookup: *mut crec = 0 as *mut crec; /* DNS disabled. */
    if (*dnsmasq_daemon).port == 0 as libc::c_int {
        return 0 as *mut libc::c_char
    }
    lookup =
        cache_find_by_addr(0 as *mut crec,
                           &mut addr as *mut in_addr as *mut all_addr,
                           0 as libc::c_int as time_t,
                           (1 as libc::c_uint) << 7 as libc::c_int);
    if !lookup.is_null() &&
           (*lookup).flags & (1 as libc::c_uint) << 6 as libc::c_int != 0 {
        let mut dot: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut hostname: *mut libc::c_char = cache_get_name(lookup);
        dot = strchr(hostname, '.' as i32);
        if !dot.is_null() &&
               strlen(dot.offset(1 as libc::c_int as isize)) !=
                   0 as libc::c_int as libc::c_ulong {
            let mut d2: *mut libc::c_char = get_domain(addr);
            if d2.is_null() ||
                   hostname_isequal(dot.offset(1 as libc::c_int as isize), d2)
                       == 0 {
                return 0 as *mut libc::c_char
            }
            /* wrong domain */
        }
        if legal_hostname(hostname) == 0 { return 0 as *mut libc::c_char }
        safe_strncpy((*dnsmasq_daemon).dhcp_buff, hostname,
                     256 as libc::c_int as size_t);
        strip_hostname((*dnsmasq_daemon).dhcp_buff);
        return (*dnsmasq_daemon).dhcp_buff
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn relay_upstream4(mut relay: *mut dhcp_relay,
                                     mut mess: *mut dhcp_packet,
                                     mut sz: size_t,
                                     mut iface_index: libc::c_int)
 -> libc::c_int {
    /* ->local is same value for all relays on ->current chain */
    let mut from: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
    if (*mess).op as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int
    }
    /* source address == relay address */
    from.addr4 = (*relay).local.addr4;
    /* already gatewayed ? */
    if (*mess).giaddr.s_addr != 0 {
        /* if so check if by us, to stomp on loops. */
        if (*mess).giaddr.s_addr == (*relay).local.addr4.s_addr {
            return 1 as libc::c_int
        }
    } else {
        /* plug in our address */
        (*mess).giaddr.s_addr = (*relay).local.addr4.s_addr
    }
    let fresh6 = (*mess).hops;
    (*mess).hops = (*mess).hops.wrapping_add(1);
    if fresh6 as libc::c_int > 20 as libc::c_int { return 1 as libc::c_int }
    while !relay.is_null() {
        let mut to: mysockaddr =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        to.sa.sa_family = 2 as libc::c_int as sa_family_t;
        to.in_0.sin_addr = (*relay).server.addr4;
        to.in_0.sin_port =
            __bswap_16((*dnsmasq_daemon).dhcp_server_port as __uint16_t);
        send_from((*dnsmasq_daemon).dhcpfd, 0 as libc::c_int,
                  mess as *mut libc::c_char, sz, &mut to, &mut from,
                  0 as libc::c_int as libc::c_uint);
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
            inet_ntop(2 as libc::c_int,
                      &mut (*relay).local as *mut all_addr as
                          *const libc::c_void, (*dnsmasq_daemon).addrbuff,
                      46 as libc::c_int as socklen_t);
            my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                          6 as libc::c_int,
                      b"DHCP relay %s -> %s\x00" as *const u8 as
                          *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                      inet_ntoa((*relay).server.addr4));
        }
        /* Save this for replies */
        (*relay).iface_index = iface_index;
        relay = (*relay).current
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn relay_reply4(mut mess: *mut dhcp_packet,
                                  mut arrival_interface: *mut libc::c_char)
 -> *mut dhcp_relay {
    let mut relay: *mut dhcp_relay = 0 as *mut dhcp_relay;
    if (*mess).giaddr.s_addr == 0 as libc::c_int as libc::c_uint ||
           (*mess).op as libc::c_int != 2 as libc::c_int {
        return 0 as *mut dhcp_relay
    }
    relay = (*dnsmasq_daemon).relay4;
    while !relay.is_null() {
        if (*mess).giaddr.s_addr == (*relay).local.addr4.s_addr {
            if (*relay).interface.is_null() ||
                   wildcard_match((*relay).interface, arrival_interface) != 0
               {
                return if (*relay).iface_index != 0 as libc::c_int {
                           relay
                       } else { 0 as *mut dhcp_relay }
            }
        }
        relay = (*relay).next
    }
    return 0 as *mut dhcp_relay;
}
