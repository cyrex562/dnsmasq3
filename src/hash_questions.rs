
/* Copyright (c) 2012-2020 Simon Kelley

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
/* Hash the question section. This is used to safely detect query 
   retransmission and to detect answers to questions we didn't ask, which 
   might be poisoning attacks. Note that we decode the name rather 
   than CRC the raw bytes, since replies might be compressed differently. 
   We ignore case in the names for the same reason. 

   The hash used is SHA-256. If we're building with DNSSEC support,
   we use the Nettle cypto library. If not, we prefer not to
   add a dependency on Nettle, and use a stand-alone implementaion. 
*/
/* HAVE_DNSSEC  || HAVE_CRYPTOHASH */
// SHA256 outputs a 32 byte digest
pub type BYTE = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA256_CTX {
    pub data: [BYTE; 64],
    pub datalen: WORD,
    pub bitlen: libc::c_ulonglong,
    pub state: [WORD; 8],
}
// 8-bit byte
pub type WORD = libc::c_uint;
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
pub unsafe extern "C" fn hash_questions_init() { }
#[no_mangle]
pub unsafe extern "C" fn hash_questions(mut header: *mut dns_header,
                                        mut plen: size_t,
                                        mut name: *mut libc::c_char)
 -> *mut libc::c_uchar {
    let mut q: libc::c_int = 0;
    let mut p: *mut libc::c_uchar =
        header.offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
    let mut ctx: SHA256_CTX =
        SHA256_CTX{data: [0; 64], datalen: 0, bitlen: 0, state: [0; 8],};
    static mut digest: [BYTE; 32] = [0; 32];
    sha256_init(&mut ctx);
    q = __bswap_16((*header).qdcount) as libc::c_int;
    while q != 0 as libc::c_int {
        let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut c: libc::c_char = 0;
        /* bad packet */
        if extract_name(header, plen, &mut p, name, 1 as libc::c_int,
                        4 as libc::c_int) == 0 {
            break ; /* bad packet */
        }
        cp = name;
        loop  {
            c = *cp;
            if !(c != 0) { break ; }
            if c as libc::c_int >= 'A' as i32 &&
                   c as libc::c_int <= 'Z' as i32 {
                *cp =
                    (*cp as libc::c_int + ('a' as i32 - 'A' as i32)) as
                        libc::c_char
            }
            cp = cp.offset(1)
        }
        sha256_update(&mut ctx, name as *mut BYTE as *const BYTE,
                      cp.wrapping_offset_from(name) as libc::c_long as
                          size_t);
        /* CRC the class and type as well */
        sha256_update(&mut ctx, p as *mut BYTE as *const BYTE,
                      4 as libc::c_int as size_t);
        p = p.offset(4 as libc::c_int as isize);
        if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                  libc::c_long + 0 as libc::c_int as libc::c_long) as size_t
                 <= plen) {
            break ;
        }
        q -= 1
    }
    sha256_final(&mut ctx, digest.as_mut_ptr());
    return digest.as_mut_ptr() as *mut libc::c_uchar;
}
/* *************************** VARIABLES *****************************/
static mut k: [WORD; 64] =
    [0x428a2f98 as libc::c_int as WORD, 0x71374491 as libc::c_int as WORD,
     0xb5c0fbcf as libc::c_uint, 0xe9b5dba5 as libc::c_uint,
     0x3956c25b as libc::c_int as WORD, 0x59f111f1 as libc::c_int as WORD,
     0x923f82a4 as libc::c_uint, 0xab1c5ed5 as libc::c_uint,
     0xd807aa98 as libc::c_uint, 0x12835b01 as libc::c_int as WORD,
     0x243185be as libc::c_int as WORD, 0x550c7dc3 as libc::c_int as WORD,
     0x72be5d74 as libc::c_int as WORD, 0x80deb1fe as libc::c_uint,
     0x9bdc06a7 as libc::c_uint, 0xc19bf174 as libc::c_uint,
     0xe49b69c1 as libc::c_uint, 0xefbe4786 as libc::c_uint,
     0xfc19dc6 as libc::c_int as WORD, 0x240ca1cc as libc::c_int as WORD,
     0x2de92c6f as libc::c_int as WORD, 0x4a7484aa as libc::c_int as WORD,
     0x5cb0a9dc as libc::c_int as WORD, 0x76f988da as libc::c_int as WORD,
     0x983e5152 as libc::c_uint, 0xa831c66d as libc::c_uint,
     0xb00327c8 as libc::c_uint, 0xbf597fc7 as libc::c_uint,
     0xc6e00bf3 as libc::c_uint, 0xd5a79147 as libc::c_uint,
     0x6ca6351 as libc::c_int as WORD, 0x14292967 as libc::c_int as WORD,
     0x27b70a85 as libc::c_int as WORD, 0x2e1b2138 as libc::c_int as WORD,
     0x4d2c6dfc as libc::c_int as WORD, 0x53380d13 as libc::c_int as WORD,
     0x650a7354 as libc::c_int as WORD, 0x766a0abb as libc::c_int as WORD,
     0x81c2c92e as libc::c_uint, 0x92722c85 as libc::c_uint,
     0xa2bfe8a1 as libc::c_uint, 0xa81a664b as libc::c_uint,
     0xc24b8b70 as libc::c_uint, 0xc76c51a3 as libc::c_uint,
     0xd192e819 as libc::c_uint, 0xd6990624 as libc::c_uint,
     0xf40e3585 as libc::c_uint, 0x106aa070 as libc::c_int as WORD,
     0x19a4c116 as libc::c_int as WORD, 0x1e376c08 as libc::c_int as WORD,
     0x2748774c as libc::c_int as WORD, 0x34b0bcb5 as libc::c_int as WORD,
     0x391c0cb3 as libc::c_int as WORD, 0x4ed8aa4a as libc::c_int as WORD,
     0x5b9cca4f as libc::c_int as WORD, 0x682e6ff3 as libc::c_int as WORD,
     0x748f82ee as libc::c_int as WORD, 0x78a5636f as libc::c_int as WORD,
     0x84c87814 as libc::c_uint, 0x8cc70208 as libc::c_uint,
     0x90befffa as libc::c_uint, 0xa4506ceb as libc::c_uint,
     0xbef9a3f7 as libc::c_uint, 0xc67178f2 as libc::c_uint];
/* ********************** FUNCTION DEFINITIONS ***********************/
unsafe extern "C" fn sha256_transform(mut ctx: *mut SHA256_CTX,
                                      mut data: *const BYTE) {
    let mut a: WORD = 0;
    let mut b: WORD = 0;
    let mut c: WORD = 0;
    let mut d: WORD = 0;
    let mut e: WORD = 0;
    let mut f: WORD = 0;
    let mut g: WORD = 0;
    let mut h: WORD = 0;
    let mut i: WORD = 0;
    let mut j: WORD = 0;
    let mut t1: WORD = 0;
    let mut t2: WORD = 0;
    let mut m: [WORD; 64] = [0; 64];
    i = 0 as libc::c_int as WORD;
    j = 0 as libc::c_int as WORD;
    while i < 16 as libc::c_int as libc::c_uint {
        m[i as usize] =
            ((*data.offset(j as isize) as libc::c_int) << 24 as libc::c_int |
                 (*data.offset(j.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint) as isize) as
                      libc::c_int) << 16 as libc::c_int |
                 (*data.offset(j.wrapping_add(2 as libc::c_int as
                                                  libc::c_uint) as isize) as
                      libc::c_int) << 8 as libc::c_int |
                 *data.offset(j.wrapping_add(3 as libc::c_int as libc::c_uint)
                                  as isize) as libc::c_int) as WORD;
        i = i.wrapping_add(1);
        j =
            (j as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint)
                as WORD as WORD
    }
    while i < 64 as libc::c_int as libc::c_uint {
        m[i as usize] =
            ((m[i.wrapping_sub(2 as libc::c_int as libc::c_uint) as usize] >>
                  17 as libc::c_int |
                  m[i.wrapping_sub(2 as libc::c_int as libc::c_uint) as usize]
                      << 32 as libc::c_int - 17 as libc::c_int) ^
                 (m[i.wrapping_sub(2 as libc::c_int as libc::c_uint) as usize]
                      >> 19 as libc::c_int |
                      m[i.wrapping_sub(2 as libc::c_int as libc::c_uint) as
                            usize] << 32 as libc::c_int - 19 as libc::c_int) ^
                 m[i.wrapping_sub(2 as libc::c_int as libc::c_uint) as usize]
                     >>
                     10 as
                         libc::c_int).wrapping_add(m[i.wrapping_sub(7 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                                         as
                                                         usize]).wrapping_add((m[i.wrapping_sub(15
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                                                                                     as
                                                                                     usize]
                                                                                   >>
                                                                                   7
                                                                                       as
                                                                                       libc::c_int
                                                                                   |
                                                                                   m[i.wrapping_sub(15
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint)
                                                                                         as
                                                                                         usize]
                                                                                       <<
                                                                                       32
                                                                                           as
                                                                                           libc::c_int
                                                                                           -
                                                                                           7
                                                                                               as
                                                                                               libc::c_int)
                                                                                  ^
                                                                                  (m[i.wrapping_sub(15
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint)
                                                                                         as
                                                                                         usize]
                                                                                       >>
                                                                                       18
                                                                                           as
                                                                                           libc::c_int
                                                                                       |
                                                                                       m[i.wrapping_sub(15
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint)
                                                                                             as
                                                                                             usize]
                                                                                           <<
                                                                                           32
                                                                                               as
                                                                                               libc::c_int
                                                                                               -
                                                                                               18
                                                                                                   as
                                                                                                   libc::c_int)
                                                                                  ^
                                                                                  m[i.wrapping_sub(15
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)
                                                                                        as
                                                                                        usize]
                                                                                      >>
                                                                                      3
                                                                                          as
                                                                                          libc::c_int).wrapping_add(m[i.wrapping_sub(16
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         as
                                                                                                                                         libc::c_uint)
                                                                                                                          as
                                                                                                                          usize]);
        i = i.wrapping_add(1)
    }
    a = (*ctx).state[0 as libc::c_int as usize];
    b = (*ctx).state[1 as libc::c_int as usize];
    c = (*ctx).state[2 as libc::c_int as usize];
    d = (*ctx).state[3 as libc::c_int as usize];
    e = (*ctx).state[4 as libc::c_int as usize];
    f = (*ctx).state[5 as libc::c_int as usize];
    g = (*ctx).state[6 as libc::c_int as usize];
    h = (*ctx).state[7 as libc::c_int as usize];
    i = 0 as libc::c_int as WORD;
    while i < 64 as libc::c_int as libc::c_uint {
        t1 =
            h.wrapping_add((e >> 6 as libc::c_int |
                                e << 32 as libc::c_int - 6 as libc::c_int) ^
                               (e >> 11 as libc::c_int |
                                    e <<
                                        32 as libc::c_int - 11 as libc::c_int)
                               ^
                               (e >> 25 as libc::c_int |
                                    e <<
                                        32 as libc::c_int -
                                            25 as
                                                libc::c_int)).wrapping_add(e &
                                                                               f
                                                                               ^
                                                                               !e
                                                                                   &
                                                                                   g).wrapping_add(k[i
                                                                                                         as
                                                                                                         usize]).wrapping_add(m[i
                                                                                                                                    as
                                                                                                                                    usize]);
        t2 =
            ((a >> 2 as libc::c_int |
                  a << 32 as libc::c_int - 2 as libc::c_int) ^
                 (a >> 13 as libc::c_int |
                      a << 32 as libc::c_int - 13 as libc::c_int) ^
                 (a >> 22 as libc::c_int |
                      a <<
                          32 as libc::c_int -
                              22 as
                                  libc::c_int)).wrapping_add(a & b ^ a & c ^
                                                                 b & c);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
        i = i.wrapping_add(1)
    }
    (*ctx).state[0 as libc::c_int as usize] =
        ((*ctx).state[0 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(a) as WORD as WORD;
    (*ctx).state[1 as libc::c_int as usize] =
        ((*ctx).state[1 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(b) as WORD as WORD;
    (*ctx).state[2 as libc::c_int as usize] =
        ((*ctx).state[2 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(c) as WORD as WORD;
    (*ctx).state[3 as libc::c_int as usize] =
        ((*ctx).state[3 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(d) as WORD as WORD;
    (*ctx).state[4 as libc::c_int as usize] =
        ((*ctx).state[4 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(e) as WORD as WORD;
    (*ctx).state[5 as libc::c_int as usize] =
        ((*ctx).state[5 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(f) as WORD as WORD;
    (*ctx).state[6 as libc::c_int as usize] =
        ((*ctx).state[6 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(g) as WORD as WORD;
    (*ctx).state[7 as libc::c_int as usize] =
        ((*ctx).state[7 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(h) as WORD as WORD;
}
unsafe extern "C" fn sha256_init(mut ctx: *mut SHA256_CTX) {
    (*ctx).datalen = 0 as libc::c_int as WORD;
    (*ctx).bitlen = 0 as libc::c_int as libc::c_ulonglong;
    (*ctx).state[0 as libc::c_int as usize] =
        0x6a09e667 as libc::c_int as WORD;
    (*ctx).state[1 as libc::c_int as usize] = 0xbb67ae85 as libc::c_uint;
    (*ctx).state[2 as libc::c_int as usize] =
        0x3c6ef372 as libc::c_int as WORD;
    (*ctx).state[3 as libc::c_int as usize] = 0xa54ff53a as libc::c_uint;
    (*ctx).state[4 as libc::c_int as usize] =
        0x510e527f as libc::c_int as WORD;
    (*ctx).state[5 as libc::c_int as usize] = 0x9b05688c as libc::c_uint;
    (*ctx).state[6 as libc::c_int as usize] =
        0x1f83d9ab as libc::c_int as WORD;
    (*ctx).state[7 as libc::c_int as usize] =
        0x5be0cd19 as libc::c_int as WORD;
}
unsafe extern "C" fn sha256_update(mut ctx: *mut SHA256_CTX,
                                   mut data: *const BYTE, mut len: size_t) {
    let mut i: WORD = 0;
    i = 0 as libc::c_int as WORD;
    while (i as libc::c_ulong) < len {
        (*ctx).data[(*ctx).datalen as usize] = *data.offset(i as isize);
        (*ctx).datalen = (*ctx).datalen.wrapping_add(1);
        if (*ctx).datalen == 64 as libc::c_int as libc::c_uint {
            sha256_transform(ctx, (*ctx).data.as_mut_ptr() as *const BYTE);
            (*ctx).bitlen =
                (*ctx).bitlen.wrapping_add(512 as libc::c_int as
                                               libc::c_ulonglong);
            (*ctx).datalen = 0 as libc::c_int as WORD
        }
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn sha256_final(mut ctx: *mut SHA256_CTX,
                                  mut hash: *mut BYTE) {
    let mut i: WORD = 0;
    i = (*ctx).datalen;
    // Pad whatever data is left in the buffer.
    if (*ctx).datalen < 56 as libc::c_int as libc::c_uint {
        let fresh6 = i;
        i = i.wrapping_add(1);
        (*ctx).data[fresh6 as usize] = 0x80 as libc::c_int as BYTE;
        while i < 56 as libc::c_int as libc::c_uint {
            let fresh7 = i;
            i = i.wrapping_add(1);
            (*ctx).data[fresh7 as usize] = 0 as libc::c_int as BYTE
        }
    } else {
        let fresh8 = i;
        i = i.wrapping_add(1);
        (*ctx).data[fresh8 as usize] = 0x80 as libc::c_int as BYTE;
        while i < 64 as libc::c_int as libc::c_uint {
            let fresh9 = i;
            i = i.wrapping_add(1);
            (*ctx).data[fresh9 as usize] = 0 as libc::c_int as BYTE
        }
        sha256_transform(ctx, (*ctx).data.as_mut_ptr() as *const BYTE);
        memset((*ctx).data.as_mut_ptr() as *mut libc::c_void,
               0 as libc::c_int, 56 as libc::c_int as libc::c_ulong);
    }
    // Append to the padding the total message's length in bits and transform.
    (*ctx).bitlen =
        (*ctx).bitlen.wrapping_add((*ctx).datalen.wrapping_mul(8 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                                       as libc::c_ulonglong);
    (*ctx).data[63 as libc::c_int as usize] = (*ctx).bitlen as BYTE;
    (*ctx).data[62 as libc::c_int as usize] =
        ((*ctx).bitlen >> 8 as libc::c_int) as BYTE;
    (*ctx).data[61 as libc::c_int as usize] =
        ((*ctx).bitlen >> 16 as libc::c_int) as BYTE;
    (*ctx).data[60 as libc::c_int as usize] =
        ((*ctx).bitlen >> 24 as libc::c_int) as BYTE;
    (*ctx).data[59 as libc::c_int as usize] =
        ((*ctx).bitlen >> 32 as libc::c_int) as BYTE;
    (*ctx).data[58 as libc::c_int as usize] =
        ((*ctx).bitlen >> 40 as libc::c_int) as BYTE;
    (*ctx).data[57 as libc::c_int as usize] =
        ((*ctx).bitlen >> 48 as libc::c_int) as BYTE;
    (*ctx).data[56 as libc::c_int as usize] =
        ((*ctx).bitlen >> 56 as libc::c_int) as BYTE;
    sha256_transform(ctx, (*ctx).data.as_mut_ptr() as *const BYTE);
    // Since this implementation uses little endian byte ordering and SHA uses big endian,
  // reverse all the bytes when copying the final state to the output hash.
    i = 0 as libc::c_int as WORD;
    while i < 4 as libc::c_int as libc::c_uint {
        *hash.offset(i as isize) =
            ((*ctx).state[0 as libc::c_int as usize] >>
                 (24 as libc::c_int as
                      libc::c_uint).wrapping_sub(i.wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint))
                 & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash.offset(i.wrapping_add(4 as libc::c_int as libc::c_uint) as
                         isize) =
            ((*ctx).state[1 as libc::c_int as usize] >>
                 (24 as libc::c_int as
                      libc::c_uint).wrapping_sub(i.wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint))
                 & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash.offset(i.wrapping_add(8 as libc::c_int as libc::c_uint) as
                         isize) =
            ((*ctx).state[2 as libc::c_int as usize] >>
                 (24 as libc::c_int as
                      libc::c_uint).wrapping_sub(i.wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint))
                 & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash.offset(i.wrapping_add(12 as libc::c_int as libc::c_uint) as
                         isize) =
            ((*ctx).state[3 as libc::c_int as usize] >>
                 (24 as libc::c_int as
                      libc::c_uint).wrapping_sub(i.wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint))
                 & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash.offset(i.wrapping_add(16 as libc::c_int as libc::c_uint) as
                         isize) =
            ((*ctx).state[4 as libc::c_int as usize] >>
                 (24 as libc::c_int as
                      libc::c_uint).wrapping_sub(i.wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint))
                 & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash.offset(i.wrapping_add(20 as libc::c_int as libc::c_uint) as
                         isize) =
            ((*ctx).state[5 as libc::c_int as usize] >>
                 (24 as libc::c_int as
                      libc::c_uint).wrapping_sub(i.wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint))
                 & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash.offset(i.wrapping_add(24 as libc::c_int as libc::c_uint) as
                         isize) =
            ((*ctx).state[6 as libc::c_int as usize] >>
                 (24 as libc::c_int as
                      libc::c_uint).wrapping_sub(i.wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint))
                 & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash.offset(i.wrapping_add(28 as libc::c_int as libc::c_uint) as
                         isize) =
            ((*ctx).state[7 as libc::c_int as usize] >>
                 (24 as libc::c_int as
                      libc::c_uint).wrapping_sub(i.wrapping_mul(8 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint))
                 & 0xff as libc::c_int as libc::c_uint) as BYTE;
        i = i.wrapping_add(1)
    };
}
