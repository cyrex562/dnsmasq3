
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
    pub fallback: in6_addr,
    pub relay_local: in6_addr,
    pub ll_addr: in6_addr,
    pub ula_addr: in6_addr,
    pub ind: libc::c_int,
    pub addr_match: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub c: *mut libc::c_uchar,
    pub p: *mut in6_pktinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub align: cmsghdr,
    pub control6: [libc::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cparam {
    pub now: time_t,
    pub newone: libc::c_int,
    pub newname: libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn dhcp6_init() {
    let mut fd: libc::c_int = 0;
    let mut saddr: sockaddr_in6 =
        sockaddr_in6{sin6_family: 0,
                     sin6_port: 0,
                     sin6_flowinfo: 0,
                     sin6_addr:
                         in6_addr{__in6_u:
                                      C2RustUnnamed{__u6_addr8: [0; 16],},},
                     sin6_scope_id: 0,};
    let mut class: libc::c_int = 0xc0 as libc::c_int;
    let mut oneopt: libc::c_int = 1 as libc::c_int;
    fd =
        socket(10 as libc::c_int, SOCK_DGRAM as libc::c_int,
               IPPROTO_UDP as libc::c_int);
    if fd == -(1 as libc::c_int) ||
           setsockopt(fd, IPPROTO_IPV6 as libc::c_int, 67 as libc::c_int,
                      &mut class as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) ||
           setsockopt(fd, IPPROTO_IPV6 as libc::c_int, 26 as libc::c_int,
                      &mut oneopt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) || fix_fd(fd) == 0
           || set_ipv6pktinfo(fd) == 0 {
        die(b"cannot create DHCPv6 socket: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 2 as libc::c_int);
    }
    /* When bind-interfaces is set, there might be more than one dnsmasq
     instance binding port 547. That's OK if they serve different networks.
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
            die(b"failed to set SO_REUSE{ADDR|PORT} on DHCPv6 socket: %s\x00"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char, 2 as libc::c_int);
        }
    }
    memset(&mut saddr as *mut sockaddr_in6 as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong);
    saddr.sin6_family = 10 as libc::c_int as sa_family_t;
    saddr.sin6_addr = in6addr_any;
    saddr.sin6_port = __bswap_16(547 as libc::c_int as __uint16_t);
    if bind(fd,
            __CONST_SOCKADDR_ARG{__sockaddr__:
                                     &mut saddr as *mut sockaddr_in6 as
                                         *mut sockaddr,},
            ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as
                socklen_t) != 0 {
        die(b"failed to bind DHCPv6 server socket: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 2 as libc::c_int);
    }
    (*dnsmasq_daemon).dhcp6fd = fd;
}
#[no_mangle]
pub unsafe extern "C" fn dhcp6_packet(mut now: time_t) {
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut relay: *mut dhcp_relay = 0 as *mut dhcp_relay;
    let mut parm: iface_param =
        iface_param{current: 0 as *mut dhcp_context,
                    relay: 0 as *mut dhcp_relay,
                    fallback:
                        in6_addr{__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},},
                    relay_local:
                        in6_addr{__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},},
                    ll_addr:
                        in6_addr{__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},},
                    ula_addr:
                        in6_addr{__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},},
                    ind: 0,
                    addr_match: 0,};
    let mut cmptr: *mut cmsghdr = 0 as *mut cmsghdr;
    let mut msg: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut if_index: libc::c_int = 0 as libc::c_int;
    let mut control_u: C2RustUnnamed_13 =
        C2RustUnnamed_13{align:
                             cmsghdr{cmsg_len: 0,
                                     cmsg_level: 0,
                                     cmsg_type: 0,
                                     __cmsg_data: [],},};
    let mut from: sockaddr_in6 =
        sockaddr_in6{sin6_family: 0,
                     sin6_port: 0,
                     sin6_flowinfo: 0,
                     sin6_addr:
                         in6_addr{__in6_u:
                                      C2RustUnnamed{__u6_addr8: [0; 16],},},
                     sin6_scope_id: 0,};
    let mut sz: ssize_t = 0;
    let mut ifr: ifreq =
        ifreq{ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_2{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut tmp: *mut iname = 0 as *mut iname;
    let mut port: libc::c_ushort = 0;
    let mut dst_addr: in6_addr =
        in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    memset(&mut dst_addr as *mut in6_addr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<in6_addr>() as libc::c_ulong);
    msg.msg_control = control_u.control6.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen =
        ::std::mem::size_of::<C2RustUnnamed_13>() as libc::c_ulong;
    msg.msg_flags = 0 as libc::c_int;
    msg.msg_name = &mut from as *mut sockaddr_in6 as *mut libc::c_void;
    msg.msg_namelen =
        ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as socklen_t;
    msg.msg_iov = &mut (*dnsmasq_daemon).dhcp_packet;
    msg.msg_iovlen = 1 as libc::c_int as size_t;
    sz = recv_dhcp_packet((*dnsmasq_daemon).dhcp6fd, &mut msg);
    if sz == -(1 as libc::c_int) as libc::c_long { return }
    cmptr =
        if msg.msg_controllen >=
               ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
            msg.msg_control as *mut cmsghdr
        } else { 0 as *mut cmsghdr };
    while !cmptr.is_null() {
        if (*cmptr).cmsg_level == IPPROTO_IPV6 as libc::c_int &&
               (*cmptr).cmsg_type == (*dnsmasq_daemon).v6pktinfo {
            let mut p: C2RustUnnamed_12 =
                C2RustUnnamed_12{c: 0 as *mut libc::c_uchar,};
            p.c = (*cmptr).__cmsg_data.as_mut_ptr();
            if_index = (*p.p).ipi6_ifindex as libc::c_int;
            dst_addr = (*p.p).ipi6_addr
        }
        cmptr = __cmsg_nxthdr(&mut msg, cmptr)
    }
    if indextoname((*dnsmasq_daemon).dhcp6fd, if_index,
                   ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) == 0 {
        return
    }
    port = relay_reply6(&mut from, sz, ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
    if port as libc::c_int != 0 as libc::c_int {
        from.sin6_port = __bswap_16(port);
        while retry_send(sendto((*dnsmasq_daemon).dhcp6fd,
                                (*dnsmasq_daemon).outpacket.iov_base,
                                save_counter(-(1 as libc::c_int)) as size_t,
                                0 as libc::c_int,
                                __CONST_SOCKADDR_ARG{__sockaddr__:
                                                         &mut from as
                                                             *mut sockaddr_in6
                                                             as
                                                             *mut sockaddr,},
                                ::std::mem::size_of::<sockaddr_in6>() as
                                    libc::c_ulong as socklen_t)) != 0 {
        }
    } else {
        let mut bridge: *mut dhcp_bridge = 0 as *mut dhcp_bridge;
        let mut alias: *mut dhcp_bridge = 0 as *mut dhcp_bridge;
        tmp = (*dnsmasq_daemon).if_except;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                return
            }
            tmp = (*tmp).next
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
        parm.current = 0 as *mut dhcp_context;
        parm.relay = 0 as *mut dhcp_relay;
        memset(&mut parm.relay_local as *mut in6_addr as *mut libc::c_void,
               0 as libc::c_int, 16 as libc::c_int as libc::c_ulong);
        parm.ind = if_index;
        parm.addr_match = 0 as libc::c_int;
        memset(&mut parm.fallback as *mut in6_addr as *mut libc::c_void,
               0 as libc::c_int, 16 as libc::c_int as libc::c_ulong);
        memset(&mut parm.ll_addr as *mut in6_addr as *mut libc::c_void,
               0 as libc::c_int, 16 as libc::c_int as libc::c_ulong);
        memset(&mut parm.ula_addr as *mut in6_addr as *mut libc::c_void,
               0 as libc::c_int, 16 as libc::c_int as libc::c_ulong);
        /* If the interface on which the DHCPv6 request was received is
         an alias of some other interface (as specified by the
         --bridge-interface option), change parm.ind so that we look
         for DHCPv6 contexts associated with the aliased interface
         instead of with the aliasing one. */
        bridge = (*dnsmasq_daemon).bridges;
        while !bridge.is_null() {
            alias = (*bridge).alias;
            while !alias.is_null() {
                if wildcard_matchn((*alias).iface.as_mut_ptr(),
                                   ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                                   16 as libc::c_int) != 0 {
                    parm.ind =
                        if_nametoindex((*bridge).iface.as_mut_ptr()) as
                            libc::c_int;
                    if parm.ind == 0 {
                        my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                      4 as libc::c_int,
                                  b"unknown interface %s in bridge-interface\x00"
                                      as *const u8 as *const libc::c_char,
                                  (*bridge).iface.as_mut_ptr());
                        return
                    }
                    break ;
                } else { alias = (*alias).next }
            }
            if !alias.is_null() { break ; }
            bridge = (*bridge).next
        }
        context = (*dnsmasq_daemon).dhcp6;
        while !context.is_null() {
            if ({
                    let mut __a: *const in6_addr =
                        &mut (*context).start6 as *mut in6_addr as
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
                }) != 0 && (*context).prefix == 0 as libc::c_int {
                /* wildcard context for DHCP-stateless only */
                parm.current = context;
                (*context).current = 0 as *mut dhcp_context
            } else {
                /* unlinked contexts are marked by context->current == context */
                (*context).current = context;
                memset(&mut (*context).local6 as *mut in6_addr as
                           *mut libc::c_void, 0 as libc::c_int,
                       16 as libc::c_int as libc::c_ulong);
            }
            context = (*context).next
        }
        relay = (*dnsmasq_daemon).relay6;
        while !relay.is_null() {
            (*relay).current = relay;
            relay = (*relay).next
        }
        if iface_enumerate(10 as libc::c_int,
                           &mut parm as *mut iface_param as *mut libc::c_void,
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
                                                                                   libc::c_uint,
                                                                               _:
                                                                                   libc::c_uint,
                                                                               _:
                                                                                   *mut libc::c_void)
                                                              -> libc::c_int>,
                                                   Option<unsafe extern "C" fn()
                                                              ->
                                                                  libc::c_int>>(Some(complete_context6
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
                                                                                                                  libc::c_uint,
                                                                                                              _:
                                                                                                                  libc::c_uint,
                                                                                                              _:
                                                                                                                  *mut libc::c_void)
                                                                                             ->
                                                                                                 libc::c_int)))
               == 0 {
            return
        }
        if !(*dnsmasq_daemon).if_names.is_null() ||
               !(*dnsmasq_daemon).if_addrs.is_null() {
            tmp = (*dnsmasq_daemon).if_names;
            while !tmp.is_null() {
                if !(*tmp).name.is_null() &&
                       wildcard_match((*tmp).name,
                                      ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) !=
                           0 {
                    break ;
                }
                tmp = (*tmp).next
            }
            if tmp.is_null() && parm.addr_match == 0 { return }
        }
        if !parm.relay.is_null() {
            /* Ignore requests sent to the ALL_SERVERS multicast address for relay when
	     we're listening there for DHCPv6 server reasons. */
            let mut all_servers: in6_addr =
                in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
            inet_pton(10 as libc::c_int,
                      b"FF05::1:3\x00" as *const u8 as *const libc::c_char,
                      &mut all_servers as *mut in6_addr as *mut libc::c_void);
            if ({
                    let mut __a: *const in6_addr =
                        &mut dst_addr as *mut in6_addr as *const in6_addr;
                    let mut __b: *const in6_addr =
                        &mut all_servers as *mut in6_addr as *const in6_addr;
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
                }) == 0 {
                relay_upstream6(parm.relay, sz, &mut from.sin6_addr,
                                from.sin6_scope_id, now);
            }
            return
        }
        /* May have configured relay, but not DHCP server */
        if (*dnsmasq_daemon).doing_dhcp6 == 0 {
            return
        } /* lose any expired leases */
        lease_prune(0 as *mut dhcp_lease, now);
        port =
            dhcp6_reply(parm.current, if_index,
                        ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                        &mut parm.fallback, &mut parm.ll_addr,
                        &mut parm.ula_addr, sz as size_t, &mut from.sin6_addr,
                        now);
        /* The port in the source address of the original request should
	 be correct, but at least once client sends from the server port,
	 so we explicitly send to the client port to a client, and the
	 server port to a relay. */
        if port as libc::c_int != 0 as libc::c_int {
            from.sin6_port = __bswap_16(port);
            while retry_send(sendto((*dnsmasq_daemon).dhcp6fd,
                                    (*dnsmasq_daemon).outpacket.iov_base,
                                    save_counter(-(1 as libc::c_int)) as
                                        size_t, 0 as libc::c_int,
                                    __CONST_SOCKADDR_ARG{__sockaddr__:
                                                             &mut from as
                                                                 *mut sockaddr_in6
                                                                 as
                                                                 *mut sockaddr,},
                                    ::std::mem::size_of::<sockaddr_in6>() as
                                        libc::c_ulong as socklen_t)) != 0 {
            }
        }
        /* These need to be called _after_ we send DHCPv6 packet, since lease_update_file()
	 may trigger sending an RA packet, which overwrites our buffer. */
        lease_update_file(now);
        lease_update_dns(0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_client_mac(mut client: *mut in6_addr,
                                        mut iface: libc::c_int,
                                        mut mac: *mut libc::c_uchar,
                                        mut maclenp: *mut libc::c_uint,
                                        mut mactypep: *mut libc::c_uint,
                                        mut now: time_t) {
    /* Receiving a packet from a host does not populate the neighbour
     cache, so we send a neighbour discovery request if we can't 
     find the sender. Repeat a few times in case of packet loss. */
    let mut neigh: neigh_packet =
        neigh_packet{type_0: 0,
                     code: 0,
                     checksum: 0,
                     reserved: 0,
                     target:
                         in6_addr{__in6_u:
                                      C2RustUnnamed{__u6_addr8: [0; 16],},},};
    let mut addr: mysockaddr =
        mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut i: libc::c_int = 0;
    let mut maclen: libc::c_int = 0;
    neigh.type_0 = 135 as libc::c_int as u8_0;
    neigh.code = 0 as libc::c_int as u8_0;
    neigh.reserved = 0 as libc::c_int as u16_0;
    neigh.target = *client;
    /* RFC4443 section-2.3: checksum has to be zero to be calculated */
    neigh.checksum = 0 as libc::c_int as u16_0; /* 100ms */
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in6.sin6_family = 10 as libc::c_int as sa_family_t;
    addr.in6.sin6_port =
        __bswap_16(IPPROTO_ICMPV6 as libc::c_int as __uint16_t);
    addr.in6.sin6_addr = *client;
    addr.in6.sin6_scope_id = iface as uint32_t;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        let mut ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
        maclen = find_mac(&mut addr, mac, 0 as libc::c_int, now);
        if maclen != 0 as libc::c_int { break ; }
        sendto((*dnsmasq_daemon).icmp6fd,
               &mut neigh as *mut neigh_packet as *const libc::c_void,
               ::std::mem::size_of::<neigh_packet>() as libc::c_ulong,
               0 as libc::c_int,
               __CONST_SOCKADDR_ARG{__sockaddr__: &mut addr.sa,},
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong as
                   socklen_t);
        ts.tv_sec = 0 as libc::c_int as __time_t;
        ts.tv_nsec = 100000000 as libc::c_int as __syscall_slong_t;
        nanosleep(&mut ts, 0 as *mut timespec);
        i += 1
    }
    *maclenp = maclen as libc::c_uint;
    *mactypep = 1 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn complete_context6(mut local: *mut in6_addr,
                                       mut prefix: libc::c_int,
                                       mut scope: libc::c_int,
                                       mut if_index: libc::c_int,
                                       mut flags: libc::c_int,
                                       mut preferred: libc::c_uint,
                                       mut valid: libc::c_uint,
                                       mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut share: *mut shared_network = 0 as *mut shared_network;
    let mut relay: *mut dhcp_relay = 0 as *mut dhcp_relay;
    let mut param: *mut iface_param = vparam as *mut iface_param;
    let mut tmp: *mut iname = 0 as *mut iname;
    /* warning */
    if if_index != (*param).ind { return 1 as libc::c_int }
    if ({
            let mut __a: *const in6_addr = local as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                 __bswap_32(0xffc00000 as libc::c_uint) ==
                 __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
        }) != 0 {
        (*param).ll_addr = *local
    } else if *(local as *const uint32_t).offset(0 as libc::c_int as isize) &
                  __bswap_32(0xff000000 as libc::c_uint) ==
                  __bswap_32(0xfd000000 as libc::c_uint) {
        (*param).ula_addr = *local
    }
    if ({
            let mut __a: *const in6_addr = local as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                 0 as libc::c_int as libc::c_uint &&
                 (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize] ==
                     0 as libc::c_int as libc::c_uint &&
                 (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize] ==
                     0 as libc::c_int as libc::c_uint &&
                 (*__a).__in6_u.__u6_addr32[3 as libc::c_int as usize] ==
                     __bswap_32(1 as libc::c_int as __uint32_t)) as
                libc::c_int
        }) != 0 ||
           ({
                let mut __a: *const in6_addr = local as *const in6_addr;
                ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                     __bswap_32(0xffc00000 as libc::c_uint) ==
                     __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
            }) != 0 ||
           *(local as *const uint8_t).offset(0 as libc::c_int as isize) as
               libc::c_int == 0xff as libc::c_int {
        return 1 as libc::c_int
    }
    /* if we have --listen-address config, see if the 
     arrival interface has a matching address. */
    tmp = (*dnsmasq_daemon).if_addrs;
    while !tmp.is_null() {
        if (*tmp).addr.sa.sa_family as libc::c_int == 10 as libc::c_int &&
               ({
                    let mut __a: *const in6_addr =
                        &mut (*tmp).addr.in6.sin6_addr as *mut in6_addr as
                            *const in6_addr;
                    let mut __b: *const in6_addr = local as *const in6_addr;
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
            (*param).addr_match = 1 as libc::c_int
        }
        tmp = (*tmp).next
    }
    /* Determine a globally address on the arrival interface, even
     if we have no matching dhcp-context, because we're only
     allocating on remote subnets via relays. This
     is used as a default for the DNS server option. */
    (*param).fallback = *local;
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 8 as libc::c_int != 0 &&
               (*context).flags as libc::c_uint &
                   ((1 as libc::c_uint) << 10 as libc::c_int |
                        (1 as libc::c_uint) << 16 as libc::c_int) == 0 &&
               prefix <= (*context).prefix && (*context).current == context {
            if is_same_net6(local, &mut (*context).start6, (*context).prefix)
                   != 0 &&
                   is_same_net6(local, &mut (*context).end6,
                                (*context).prefix) != 0 {
                let mut tmp_0: *mut dhcp_context = 0 as *mut dhcp_context;
                let mut up: *mut *mut dhcp_context =
                    0 as *mut *mut dhcp_context;
                /* use interface values only for constructed contexts */
                if (*context).flags as libc::c_uint &
                       (1 as libc::c_uint) << 11 as libc::c_int == 0 {
                    valid = 0xffffffff as libc::c_uint;
                    preferred = valid
                } else if flags & 2 as libc::c_int != 0 {
                    preferred = 0 as libc::c_int as libc::c_uint
                }
                if (*context).flags as libc::c_uint &
                       (1 as libc::c_uint) << 9 as libc::c_int != 0 {
                    preferred = 0 as libc::c_int as libc::c_uint
                }
                /* order chain, longest preferred time first */
                up = &mut (*param).current;
                tmp_0 = (*param).current;
                while !tmp_0.is_null() {
                    if (*tmp_0).preferred <= preferred { break ; }
                    up = &mut (*tmp_0).current;
                    tmp_0 = (*tmp_0).current
                }
                (*context).current = *up;
                *up = context;
                (*context).local6 = *local;
                (*context).preferred = preferred;
                (*context).valid = valid
            } else {
                let mut current_block_37: u64;
                share = (*dnsmasq_daemon).shared_networks;
                while !share.is_null() {
                    /* IPv4 shared_address - ignore */
                    if !((*share).shared_addr.s_addr !=
                             0 as libc::c_int as libc::c_uint) {
                        if (*share).if_index != 0 as libc::c_int {
                            if (*share).if_index != if_index {
                                current_block_37 = 11385396242402735691;
                            } else {
                                current_block_37 = 18435049525520518667;
                            }
                        } else if ({
                                       let mut __a: *const in6_addr =
                                           &mut (*share).match_addr6 as
                                               *mut in6_addr as
                                               *const in6_addr;
                                       let mut __b: *const in6_addr =
                                           local as *const in6_addr;
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
                                   }) == 0 {
                            current_block_37 = 11385396242402735691;
                        } else { current_block_37 = 18435049525520518667; }
                        match current_block_37 {
                            11385396242402735691 => { }
                            _ => {
                                if is_same_net6(&mut (*share).shared_addr6,
                                                &mut (*context).start6,
                                                (*context).prefix) != 0 &&
                                       is_same_net6(&mut (*share).shared_addr6,
                                                    &mut (*context).end6,
                                                    (*context).prefix) != 0 {
                                    (*context).current = (*param).current;
                                    (*param).current = context;
                                    (*context).local6 = *local;
                                    (*context).preferred =
                                        if (*context).flags as libc::c_uint &
                                               (1 as libc::c_uint) <<
                                                   9 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_uint
                                        } else { 0xffffffff as libc::c_uint };
                                    (*context).valid =
                                        0xffffffff as libc::c_uint
                                }
                            }
                        }
                    }
                    share = (*share).next
                }
            }
        }
        context = (*context).next
    }
    relay = (*dnsmasq_daemon).relay6;
    while !relay.is_null() {
        if ({
                let mut __a: *const in6_addr = local as *const in6_addr;
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
            }) != 0 && (*relay).current == relay &&
               (({
                     let mut __a: *const in6_addr =
                         &mut (*param).relay_local as *mut in6_addr as
                             *const in6_addr;
                     ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                          0 as libc::c_int as libc::c_uint &&
                          (*__a).__in6_u.__u6_addr32[1 as libc::c_int as
                                                         usize] ==
                              0 as libc::c_int as libc::c_uint &&
                          (*__a).__in6_u.__u6_addr32[2 as libc::c_int as
                                                         usize] ==
                              0 as libc::c_int as libc::c_uint &&
                          (*__a).__in6_u.__u6_addr32[3 as libc::c_int as
                                                         usize] ==
                              0 as libc::c_int as libc::c_uint) as libc::c_int
                 }) != 0 ||
                    ({
                         let mut __a: *const in6_addr =
                             local as *const in6_addr;
                         let mut __b: *const in6_addr =
                             &mut (*param).relay_local as *mut in6_addr as
                                 *const in6_addr;
                         ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as
                                                         usize] ==
                              (*__b).__in6_u.__u6_addr32[0 as libc::c_int as
                                                             usize] &&
                              (*__a).__in6_u.__u6_addr32[1 as libc::c_int as
                                                             usize] ==
                                  (*__b).__in6_u.__u6_addr32[1 as libc::c_int
                                                                 as usize] &&
                              (*__a).__in6_u.__u6_addr32[2 as libc::c_int as
                                                             usize] ==
                                  (*__b).__in6_u.__u6_addr32[2 as libc::c_int
                                                                 as usize] &&
                              (*__a).__in6_u.__u6_addr32[3 as libc::c_int as
                                                             usize] ==
                                  (*__b).__in6_u.__u6_addr32[3 as libc::c_int
                                                                 as usize]) as
                             libc::c_int
                     }) != 0) {
            (*relay).current = (*param).relay;
            (*param).relay = relay;
            (*param).relay_local = *local
        }
        relay = (*relay).next
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn config_find_by_address6(mut configs:
                                                     *mut dhcp_config,
                                                 mut net: *mut in6_addr,
                                                 mut prefix: libc::c_int,
                                                 mut addr: *mut in6_addr)
 -> *mut dhcp_config {
    let mut config: *mut dhcp_config = 0 as *mut dhcp_config;
    config = configs;
    while !config.is_null() {
        if (*config).flags & 4096 as libc::c_int as libc::c_uint != 0 {
            let mut addr_list: *mut addrlist = 0 as *mut addrlist;
            addr_list = (*config).addr6;
            while !addr_list.is_null() {
                if (net.is_null() ||
                        is_same_net6(&mut (*addr_list).addr.addr6, net,
                                     prefix) != 0 ||
                        (*addr_list).flags & 16 as libc::c_int != 0 &&
                            prefix == 64 as libc::c_int) &&
                       is_same_net6(&mut (*addr_list).addr.addr6, addr,
                                    (if (*addr_list).flags & 8 as libc::c_int
                                            != 0 {
                                         (*addr_list).prefixlen
                                     } else { 128 as libc::c_int })) != 0 {
                    return config
                }
                addr_list = (*addr_list).next
            }
        }
        config = (*config).next
    }
    return 0 as *mut dhcp_config;
}
#[no_mangle]
pub unsafe extern "C" fn address6_allocate(mut context: *mut dhcp_context,
                                           mut clid: *mut libc::c_uchar,
                                           mut clid_len: libc::c_int,
                                           mut temp_addr: libc::c_int,
                                           mut iaid: libc::c_uint,
                                           mut serial: libc::c_int,
                                           mut netids: *mut dhcp_netid,
                                           mut plain_range: libc::c_int,
                                           mut ans: *mut in6_addr)
 -> *mut dhcp_context {
    /* Find a free address: exclude anything in use and anything allocated to
     a particular hwaddr/clientid/hostname in our configuration.
     Try to return from contexts which match netids first. 
     
     Note that we assume the address prefix lengths are 64 or greater, so we can
     get by with 64 bit arithmetic.
*/
    let mut start: u64_0 = 0;
    let mut addr: u64_0 = 0;
    let mut c: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut d: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut i: libc::c_int = 0;
    let mut pass: libc::c_int = 0;
    let mut j: u64_0 = 0;
    /* hash hwaddr: use the SDBM hashing algorithm.  This works
     for MAC addresses, let's see how it manages with client-ids! 
     For temporary addresses, we generate a new random one each time. */
    if temp_addr != 0 {
        j = rand64()
    } else {
        j = iaid as u64_0;
        i = 0 as libc::c_int;
        while i < clid_len {
            j =
                (*clid.offset(i as isize) as
                     libc::c_ulonglong).wrapping_add(j <<
                                                         6 as
                                                             libc::c_int).wrapping_add(j
                                                                                           <<
                                                                                           16
                                                                                               as
                                                                                               libc::c_int).wrapping_sub(j);
            i += 1
        }
    }
    pass = 0 as libc::c_int;
    while if pass <= plain_range {
              1 as libc::c_int
          } else { 0 as libc::c_int } != 0 {
        c = context;
        while !c.is_null() {
            if !((*c).flags as libc::c_uint &
                     ((1 as libc::c_uint) << 9 as libc::c_int |
                          (1 as libc::c_uint) << 0 as libc::c_int |
                          (1 as libc::c_uint) << 7 as libc::c_int |
                          (1 as libc::c_uint) << 15 as libc::c_int) != 0) {
                if !(match_netid((*c).filter, netids, pass) == 0) {
                    if temp_addr == 0 &&
                           (*dnsmasq_daemon).options[(34 as libc::c_int as
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
                        /* seed is largest extant lease addr in this context,
		 skip addresses equal to the number of addresses rejected
		 by clients. This should avoid the same client being offered the same
		 address after it has rjected it. */
                        start =
                            lease_find_max_addr6(c).wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulonglong).wrapping_add(serial
                                                                                                         as
                                                                                                         libc::c_ulonglong).wrapping_add((*c).addr_epoch
                                                                                                                                             as
                                                                                                                                             libc::c_ulonglong);
                        if (*c).addr_epoch != 0 {
                            (*c).addr_epoch = (*c).addr_epoch.wrapping_sub(1)
                        }
                    } else {
                        let mut range: u64_0 =
                            (1 as libc::c_int as
                                 libc::c_ulonglong).wrapping_add(addr6part(&mut (*c).end6)).wrapping_sub(addr6part(&mut (*c).start6));
                        let mut offset: u64_0 =
                            j.wrapping_add((*c).addr_epoch as
                                               libc::c_ulonglong);
                        /* don't divide by zero if range is whole 2^64 */
                        if range != 0 as libc::c_int as libc::c_ulonglong {
                            offset = offset.wrapping_rem(range)
                        }
                        start =
                            addr6part(&mut (*c).start6).wrapping_add(offset)
                    }
                    /* iterate until we find a free address. */
                    addr = start;
                    loop  {
                        /* eliminate addresses in use by the server. */
                        d = context;
                        while !d.is_null() {
                            if addr == addr6part(&mut (*d).local6) { break ; }
                            d = (*d).current
                        }
                        *ans = (*c).start6;
                        setaddr6part(ans, addr);
                        if d.is_null() &&
                               lease6_find_by_addr(&mut (*c).start6,
                                                   (*c).prefix,
                                                   addr).is_null() &&
                               config_find_by_address6((*dnsmasq_daemon).dhcp_conf,
                                                       &mut (*c).start6,
                                                       (*c).prefix,
                                                       ans).is_null() {
                            return c
                        }
                        addr = addr.wrapping_add(1);
                        if addr ==
                               addr6part(&mut (*c).end6).wrapping_add(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulonglong)
                           {
                            addr = addr6part(&mut (*c).start6)
                        }
                        if !(addr != start) { break ; }
                    }
                }
            }
            c = (*c).current
        }
        pass += 1
    }
    return 0 as *mut dhcp_context;
}
/* can dynamically allocate addr */
#[no_mangle]
pub unsafe extern "C" fn address6_available(mut context: *mut dhcp_context,
                                            mut taddr: *mut in6_addr,
                                            mut netids: *mut dhcp_netid,
                                            mut plain_range: libc::c_int)
 -> *mut dhcp_context {
    let mut start: u64_0 = 0;
    let mut end: u64_0 = 0;
    let mut addr: u64_0 = addr6part(taddr);
    let mut tmp: *mut dhcp_context = 0 as *mut dhcp_context;
    tmp = context;
    while !tmp.is_null() {
        start = addr6part(&mut (*tmp).start6);
        end = addr6part(&mut (*tmp).end6);
        if (*tmp).flags as libc::c_uint &
               ((1 as libc::c_uint) << 0 as libc::c_int |
                    (1 as libc::c_uint) << 7 as libc::c_int) == 0 &&
               is_same_net6(&mut (*tmp).start6, taddr, (*tmp).prefix) != 0 &&
               is_same_net6(&mut (*tmp).end6, taddr, (*tmp).prefix) != 0 &&
               addr >= start && addr <= end &&
               match_netid((*tmp).filter, netids, plain_range) != 0 {
            return tmp
        }
        tmp = (*tmp).current
    }
    return 0 as *mut dhcp_context;
}
/* address OK if configured */
#[no_mangle]
pub unsafe extern "C" fn address6_valid(mut context: *mut dhcp_context,
                                        mut taddr: *mut in6_addr,
                                        mut netids: *mut dhcp_netid,
                                        mut plain_range: libc::c_int)
 -> *mut dhcp_context {
    let mut tmp: *mut dhcp_context = 0 as *mut dhcp_context;
    tmp = context;
    while !tmp.is_null() {
        if is_same_net6(&mut (*tmp).start6, taddr, (*tmp).prefix) != 0 &&
               match_netid((*tmp).filter, netids, plain_range) != 0 {
            return tmp
        }
        tmp = (*tmp).current
    }
    return 0 as *mut dhcp_context;
}
#[no_mangle]
pub unsafe extern "C" fn make_duid(mut now: time_t) {
    if !(*dnsmasq_daemon).duid_config.is_null() {
        let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        p =
            safe_malloc((*dnsmasq_daemon).duid_config_len.wrapping_add(6 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)
                            as size_t) as *mut libc::c_uchar;
        (*dnsmasq_daemon).duid = p;
        (*dnsmasq_daemon).duid_len =
            (*dnsmasq_daemon).duid_config_len.wrapping_add(6 as libc::c_int as
                                                               libc::c_uint)
                as libc::c_int;
        let mut t_s: u16_0 = 2 as libc::c_int as u16_0;
        let mut t_cp: *mut libc::c_uchar = p;
        let fresh6 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh6 = (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp = t_s as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        /* DUID_EN */
        let mut t_l: u32_0 = (*dnsmasq_daemon).duid_enterprise;
        let mut t_cp_0: *mut libc::c_uchar = p;
        let fresh7 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh7 = (t_l >> 24 as libc::c_int) as libc::c_uchar;
        let fresh8 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh8 = (t_l >> 16 as libc::c_int) as libc::c_uchar;
        let fresh9 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh9 = (t_l >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_0 = t_l as libc::c_uchar;
        p = p.offset(4 as libc::c_int as isize);
        memcpy(p as *mut libc::c_void,
               (*dnsmasq_daemon).duid_config as *const libc::c_void,
               (*dnsmasq_daemon).duid_config_len as libc::c_ulong);
    } else {
        let mut newnow: time_t = 0 as libc::c_int as time_t;
        /* If we have no persistent lease database, or a non-stable RTC, use DUID_LL (newnow == 0) */
        /* rebase epoch to 1/1/2000 */
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
               == 0 || !(*dnsmasq_daemon).lease_change_command.is_null() {
            newnow = now - 946684800 as libc::c_int as libc::c_long
        }
        iface_enumerate(1 as libc::c_int,
                        &mut newnow as *mut time_t as *mut libc::c_void,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                libc::c_int,
                                                                            _:
                                                                                libc::c_uint,
                                                                            _:
                                                                                *mut libc::c_char,
                                                                            _:
                                                                                size_t,
                                                                            _:
                                                                                *mut libc::c_void)
                                                           -> libc::c_int>,
                                                Option<unsafe extern "C" fn()
                                                           ->
                                                               libc::c_int>>(Some(make_duid1
                                                                                      as
                                                                                      unsafe extern "C" fn(_:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               libc::c_uint,
                                                                                                           _:
                                                                                                               *mut libc::c_char,
                                                                                                           _:
                                                                                                               size_t,
                                                                                                           _:
                                                                                                               *mut libc::c_void)
                                                                                          ->
                                                                                              libc::c_int)));
        if (*dnsmasq_daemon).duid.is_null() {
            die(b"Cannot create DHCPv6 server DUID: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char, 5 as libc::c_int);
        }
    };
}
unsafe extern "C" fn make_duid1(mut index: libc::c_int,
                                mut type_0: libc::c_uint,
                                mut mac: *mut libc::c_char,
                                mut maclen: size_t,
                                mut parm: *mut libc::c_void) -> libc::c_int {
    /* create DUID as specified in RFC3315. We use the MAC of the
     first interface we find that isn't loopback or P-to-P and
     has address-type < 256. Address types above 256 are things like 
     tunnels which don't have usable MAC addresses. */
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut newnow: time_t = *(parm as *mut time_t);
    if type_0 >= 256 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    if newnow == 0 as libc::c_int as libc::c_long {
        p =
            safe_malloc(maclen.wrapping_add(4 as libc::c_int as
                                                libc::c_ulong)) as
                *mut libc::c_uchar;
        (*dnsmasq_daemon).duid = p;
        (*dnsmasq_daemon).duid_len =
            maclen.wrapping_add(4 as libc::c_int as libc::c_ulong) as
                libc::c_int;
        let mut t_s: u16_0 = 3 as libc::c_int as u16_0;
        let mut t_cp: *mut libc::c_uchar = p;
        let fresh10 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh10 = (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp = t_s as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        /* address type */
        let mut t_s_0: u16_0 = type_0 as u16_0;
        let mut t_cp_0: *mut libc::c_uchar = p;
        let fresh11 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh11 =
            (t_s_0 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_0 = t_s_0 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize)
    } else {
        p =
            safe_malloc(maclen.wrapping_add(8 as libc::c_int as
                                                libc::c_ulong)) as
                *mut libc::c_uchar;
        (*dnsmasq_daemon).duid = p;
        (*dnsmasq_daemon).duid_len =
            maclen.wrapping_add(8 as libc::c_int as libc::c_ulong) as
                libc::c_int;
        let mut t_s_1: u16_0 = 1 as libc::c_int as u16_0;
        let mut t_cp_1: *mut libc::c_uchar = p;
        let fresh12 = t_cp_1;
        t_cp_1 = t_cp_1.offset(1);
        *fresh12 =
            (t_s_1 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_1 = t_s_1 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        /* DUID_LL */
        /* time */
        let mut t_s_2: u16_0 = type_0 as u16_0;
        let mut t_cp_2: *mut libc::c_uchar = p;
        let fresh13 = t_cp_2;
        t_cp_2 = t_cp_2.offset(1);
        *fresh13 =
            (t_s_2 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_2 = t_s_2 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_l: u32_0 = *(parm as *mut time_t) as u32_0;
        let mut t_cp_3: *mut libc::c_uchar = p;
        let fresh14 = t_cp_3;
        t_cp_3 = t_cp_3.offset(1);
        *fresh14 = (t_l >> 24 as libc::c_int) as libc::c_uchar;
        let fresh15 = t_cp_3;
        t_cp_3 = t_cp_3.offset(1);
        *fresh15 = (t_l >> 16 as libc::c_int) as libc::c_uchar;
        let fresh16 = t_cp_3;
        t_cp_3 = t_cp_3.offset(1);
        *fresh16 = (t_l >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_3 = t_l as libc::c_uchar;
        p = p.offset(4 as libc::c_int as isize)
    }
    memcpy(p as *mut libc::c_void, mac as *const libc::c_void, maclen);
    return 0 as libc::c_int;
}
unsafe extern "C" fn construct_worker(mut local: *mut in6_addr,
                                      mut prefix: libc::c_int,
                                      mut scope: libc::c_int,
                                      mut if_index: libc::c_int,
                                      mut flags: libc::c_int,
                                      mut preferred: libc::c_int,
                                      mut valid: libc::c_int,
                                      mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut ifrn_name: [libc::c_char; 16] = [0; 16];
    let mut start6: in6_addr =
        in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    let mut end6: in6_addr =
        in6_addr{__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    let mut template: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut tmp: *mut iname = 0 as *mut iname;
    let mut param: *mut cparam = vparam as *mut cparam;
    if ({
            let mut __a: *const in6_addr = local as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                 0 as libc::c_int as libc::c_uint &&
                 (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize] ==
                     0 as libc::c_int as libc::c_uint &&
                 (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize] ==
                     0 as libc::c_int as libc::c_uint &&
                 (*__a).__in6_u.__u6_addr32[3 as libc::c_int as usize] ==
                     __bswap_32(1 as libc::c_int as __uint32_t)) as
                libc::c_int
        }) != 0 ||
           ({
                let mut __a: *const in6_addr = local as *const in6_addr;
                ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                     __bswap_32(0xffc00000 as libc::c_uint) ==
                     __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
            }) != 0 ||
           *(local as *const uint8_t).offset(0 as libc::c_int as isize) as
               libc::c_int == 0xff as libc::c_int {
        return 1 as libc::c_int
    }
    if flags & 4 as libc::c_int == 0 { return 1 as libc::c_int }
    if flags & 2 as libc::c_int != 0 { return 1 as libc::c_int }
    /* DUID_LLT */
    /* address type */
    /* Ignore interfaces where we're not doing RA/DHCP6 */
    if indextoname((*dnsmasq_daemon).icmp6fd, if_index,
                   ifrn_name.as_mut_ptr()) == 0 ||
           iface_check(1 as libc::c_int, 0 as *mut all_addr,
                       ifrn_name.as_mut_ptr(), 0 as *mut libc::c_int) == 0 {
        return 1 as libc::c_int
    }
    tmp = (*dnsmasq_daemon).dhcp_except;
    while !tmp.is_null() {
        if !(*tmp).name.is_null() &&
               wildcard_match((*tmp).name, ifrn_name.as_mut_ptr()) != 0 {
            return 1 as libc::c_int
        }
        tmp = (*tmp).next
    }
    template = (*dnsmasq_daemon).dhcp6;
    while !template.is_null() {
        if (*template).flags as libc::c_uint &
               ((1 as libc::c_uint) << 10 as libc::c_int |
                    (1 as libc::c_uint) << 11 as libc::c_int) == 0 {
            /* non-template entries, just fill in interface and local addresses */
            if prefix <= (*template).prefix &&
                   is_same_net6(local, &mut (*template).start6,
                                (*template).prefix) != 0 &&
                   is_same_net6(local, &mut (*template).end6,
                                (*template).prefix) != 0 {
                /* First time found, do fast RA. */
                if (*template).if_index == 0 as libc::c_int {
                    ra_start_unsolicited((*param).now, template);
                    (*param).newone = 1 as libc::c_int
                }
                (*template).if_index = if_index;
                (*template).local6 = *local
            }
        } else if wildcard_match((*template).template_interface,
                                 ifrn_name.as_mut_ptr()) != 0 &&
                      (*template).prefix >= prefix {
            start6 = *local;
            setaddr6part(&mut start6, addr6part(&mut (*template).start6));
            end6 = *local;
            setaddr6part(&mut end6, addr6part(&mut (*template).end6));
            context = (*dnsmasq_daemon).dhcp6;
            while !context.is_null() {
                if (*context).flags as libc::c_uint &
                       (1 as libc::c_uint) << 10 as libc::c_int == 0 &&
                       ({
                            let mut __a: *const in6_addr =
                                &mut start6 as *mut in6_addr as
                                    *const in6_addr;
                            let mut __b: *const in6_addr =
                                &mut (*context).start6 as *mut in6_addr as
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
                        }) != 0 &&
                       ({
                            let mut __a: *const in6_addr =
                                &mut end6 as *mut in6_addr as *const in6_addr;
                            let mut __b: *const in6_addr =
                                &mut (*context).end6 as *mut in6_addr as
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
                    /* If there's an absolute address context covering this address
		 then don't construct one as well. */
                    if (*context).flags as libc::c_uint &
                           (1 as libc::c_uint) << 11 as libc::c_int == 0 {
                        break ;
                    }
                    if (*context).if_index == if_index {
                        let mut cflags: libc::c_int = (*context).flags;
                        (*context).flags =
                            ((*context).flags as libc::c_uint &
                                 !((1 as libc::c_uint) << 12 as libc::c_int |
                                       (1 as libc::c_uint) <<
                                           16 as libc::c_int)) as libc::c_int;
                        if cflags as libc::c_uint &
                               (1 as libc::c_uint) << 16 as libc::c_int != 0 {
                            /* address went, now it's back, and on the same interface */
                            log_context(10 as libc::c_int, context);
                            /* fast RAs for a while */
                            ra_start_unsolicited((*param).now, context);
                            (*param).newone = 1 as libc::c_int;
                            /* Add address to name again */
                            if (*context).flags as libc::c_uint &
                                   (1 as libc::c_uint) << 6 as libc::c_int !=
                                   0 {
                                (*param).newname = 1 as libc::c_int
                            }
                        }
                        break ;
                    }
                }
                context = (*context).next
            }
            if context.is_null() &&
                   {
                       context =
                           whine_malloc(::std::mem::size_of::<dhcp_context>()
                                            as libc::c_ulong) as
                               *mut dhcp_context;
                       !context.is_null()
                   } {
                *context = *template;
                (*context).start6 = start6;
                (*context).end6 = end6;
                (*context).flags =
                    ((*context).flags as libc::c_uint &
                         !((1 as libc::c_uint) << 10 as libc::c_int)) as
                        libc::c_int;
                (*context).flags =
                    ((*context).flags as libc::c_uint |
                         (1 as libc::c_uint) << 11 as libc::c_int) as
                        libc::c_int;
                (*context).if_index = if_index;
                (*context).local6 = *local;
                (*context).saved_valid = 0 as libc::c_int as libc::c_uint;
                (*context).next = (*dnsmasq_daemon).dhcp6;
                (*dnsmasq_daemon).dhcp6 = context;
                ra_start_unsolicited((*param).now, context);
                /* we created a new one, need to call
	       lease_update_file to get periodic functions called */
                (*param).newone = 1 as libc::c_int;
                /* Will need to add new putative SLAAC addresses to existing leases */
                if (*context).flags as libc::c_uint &
                       (1 as libc::c_uint) << 6 as libc::c_int != 0 {
                    (*param).newname = 1 as libc::c_int
                }
                log_context(10 as libc::c_int, context);
            }
        }
        template = (*template).next
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dhcp_construct_contexts(mut now: time_t) {
    let mut context: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut tmp: *mut dhcp_context = 0 as *mut dhcp_context;
    let mut up: *mut *mut dhcp_context = 0 as *mut *mut dhcp_context;
    let mut param: cparam = cparam{now: 0, newone: 0, newname: 0,};
    param.newone = 0 as libc::c_int;
    param.newname = 0 as libc::c_int;
    param.now = now;
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 11 as libc::c_int != 0 {
            (*context).flags =
                ((*context).flags as libc::c_uint |
                     (1 as libc::c_uint) << 12 as libc::c_int) as libc::c_int
        }
        context = (*context).next
    }
    iface_enumerate(10 as libc::c_int,
                    &mut param as *mut cparam as *mut libc::c_void,
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
                                                           libc::c_int>>(Some(construct_worker
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
    up = &mut (*dnsmasq_daemon).dhcp6;
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        tmp = (*context).next;
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 12 as libc::c_int != 0 &&
               (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 16 as libc::c_int == 0 {
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
                       != 0 {
                /* previously constructed context has gone. advertise it's demise */
                (*context).flags =
                    ((*context).flags as libc::c_uint |
                         (1 as libc::c_uint) << 16 as libc::c_int) as
                        libc::c_int;
                (*context).address_lost_time = now;
                /* Apply same ceiling of configured lease time as in radv.c */
                if (*context).saved_valid > (*context).lease_time {
                    (*context).saved_valid = (*context).lease_time
                }
                /* maximum time is 2 hours, from RFC */
                if (*context).saved_valid >
                       7200 as libc::c_int as libc::c_uint {
                    /* 2 hours */
                    (*context).saved_valid =
                        7200 as libc::c_int as libc::c_uint
                } /* include deletion */
                ra_start_unsolicited(now, context);
                param.newone = 1 as libc::c_int;
                if (*context).flags as libc::c_uint &
                       (1 as libc::c_uint) << 6 as libc::c_int != 0 {
                    param.newname = 1 as libc::c_int
                }
                log_context(10 as libc::c_int, context);
                up = &mut (*context).next
            } else {
                /* we were never doing RA for this, so free now */
                *up = (*context).next;
                free(context as *mut libc::c_void);
            }
        } else { up = &mut (*context).next }
        context = tmp
    }
    if param.newone != 0 {
        if !(*dnsmasq_daemon).dhcp.is_null() ||
               (*dnsmasq_daemon).doing_dhcp6 != 0 {
            if param.newname != 0 { lease_update_slaac(now); }
            lease_update_file(now);
        } else {
            /* Not doing DHCP, so no lease system, manage alarms for ra only */
            send_alarm(periodic_ra(now), now);
        }
    };
}
