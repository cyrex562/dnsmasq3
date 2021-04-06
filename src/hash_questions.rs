
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
use crate::defines::{MsgHdr, CmsgHdr, size_t, ModeT, DevT, FILE, SsizeT, __compar_fn_t, intmax_t, uintmax_t, __gwchar_t, DnsHeader};
use std::io::{stdout, stdin};
use crate::rfc1035::extract_name;
use crate::util::zero_array_1;

// SHA256 outputs a 32 byte digest
pub type BYTE = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA256_CTX {
    pub data: [BYTE; 64],
    pub datalen: WORD,
    pub bitlen: u32long,
    pub state: [WORD; 8],
}
// 8-bit byte
pub type WORD = libc::c_uint;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: u16) -> u16 {
    return (__bsx >> 8 & 0xff |
                (__bsx & 0xff) <<
                    8);
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: u32) -> u32 {
    return (__bsx & 0xff000000) >> 24 |
               (__bsx & 0xff0000) >> 8 |
               (__bsx & 0xff00) << 8 |
               (__bsx & 0xff) << 24;
}
#[inline]
unsafe extern "C" fn __bswap_64(mut __bsx: u64) -> u64 {
    return ((__bsxlong &
                 0xff00000000000000long) >> 56
                |
                (__bsxlong &
                     0xff000000000000long) >>
                    40 |
                (__bsxlong &
                     0xff0000000000long) >> 24
                |
                (__bsxlong &
                     0xff00000000long) >> 8 |
                (__bsxlong & 0xff000000long)
                    << 8 |
                (__bsxlong & 0xff0000long)
                    << 24 |
                (__bsxlong & 0xff00long) <<
                    40 |
                (__bsxlong & 0xfflong) <<
                    56) as u64;
}
#[inline]
unsafe extern "C" fn __uint16_identity(mut __x: u16) -> u16 {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint32_identity(mut __x: u32) -> u32 {
    return __x;
}
#[inline]
unsafe extern "C" fn __uint64_identity(mut __x: u64) -> u64 {
    return __x;
}
#[inline]
unsafe extern "C" fn __cmsg_nxthdr(mut __mhdr: &mut MsgHdr,
                                   mut __cmsg: &mut CmsgHdr) -> {
    if __cmsg.cmsg_len < ::std::mem::size_of::<CmsgHdr>()
       {
        return 0
    }
    __cmsg =
        (__cmsg       mut Vec<u8>).offset((__cmsg.cmsg_len.wrapping_add(::std::mem::size_of::<size_t>()
                                                                                               ).wrapping_sub(1                                         libc::c_int                                  )
                                             &
                                             !(::std::mem::size_of::<size_t>()
                                           ).wrapping_sub(1
                                                                                                                  libc::c_int
                                                                                                           ))
                                           );
    if __cmsg.offset(1) >
           (__mhdr.msg_control          mut Vec<u8>).offset(__mhdr.msg_controllen)
           ||
           (__cmsg          mut Vec<u8>).offset((__cmsg.cmsg_len.wrapping_add(::std::mem::size_of::<size_t>()
                                                                                                     ).wrapping_sub(1                                               libc::c_int                                        )
                                                &
                                                !(::std::mem::size_of::<size_t>()
                                                 ).wrapping_sub(1))
                                              ) >
               (__mhdr.msg_control              mut Vec<u8>).offset(__mhdr.msg_controllenisize) {
        return 0
    }
    return __cmsg;
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const libc::c_char,
                          mut __statbuf: &mut stat) -> i32 {
    return __xstat(1, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: &mut stat)
 -> i32 {
    return __fxstat(1, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn stat64(mut __path: *const libc::c_char,
                            mut __statbuf: &mut stat64) -> i32 {
    return __xstat64(1, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat64(mut __fd: i32,
                             mut __statbuf: &mut stat64) -> i32 {
    return __fxstat64(1, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn fstatat(mut __fd: i32,
                             mut __filename: *const libc::c_char,
                             mut __statbuf: &mut stat,
                             mut __flag: i32) -> i32 {
    return __fxstatat(1, __fd, __filename, __statbuf, __flag);
}
#[inline]
unsafe extern "C" fn fstatat64(mut __fd: i32,
                               mut __filename: *const libc::c_char,
                               mut __statbuf: &mut stat64,
                               mut __flag: i32) -> i32 {
    return __fxstatat64(1, __fd, __filename, __statbuf,
                        __flag);
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const libc::c_char,
                           mut __statbuf: &mut stat) -> i32 {
    return __lxstat(1, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat64(mut __path: *const libc::c_char,
                             mut __statbuf: &mut stat64) -> i32 {
    return __lxstat64(1, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn mknod(mut __path: *const libc::c_char,
                           mut __mode: ModeT, mut __dev: DevT)
                           -> i32 {
    return __xmknod(0, __path, __mode, &mut __dev);
}
#[inline]
unsafe extern "C" fn mknodat(mut __fd: i32,
                             mut __path: *const libc::c_char,
                             mut __mode: ModeT, mut __dev: DevT)
                             -> i32 {
    return __xmknodat(0, __fd, __path, __mode, &mut __dev);
}
#[inline]
unsafe extern "C" fn vprintf(mut __fmt: *const libc::c_char,
                             mut __arg: ::std::ffi::VaList) -> i32 {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
#[inline]
unsafe extern "C" fn getchar() -> i32 { return getc(stdin); }
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: &mut FILE) -> i32 {
    return if (__fp._IO_read_ptr >= __fp._IO_read_end)  != 0 {
               __uflow(__fp)
           } else {
               let fresh0 = __fp._IO_read_ptr;
               __fp._IO_read_ptr = __fp._IO_read_ptr.offset(1);
               *(fresh0)
           };
}
#[inline]
unsafe extern "C" fn getchar_unlocked() -> i32 {
    return if (stdin._IO_read_ptr >= stdin._IO_read_end)
                  != 0 {
               __uflow(stdin)
           } else {
               let fresh1 = stdin._IO_read_ptr;
               stdin._IO_read_ptr = stdin._IO_read_ptr.offset(1);
               *(fresh1)
           };
}
#[inline]
unsafe extern "C" fn fgetc_unlocked(mut __fp: &mut FILE) -> i32 {
    return if (__fp._IO_read_ptr >= __fp._IO_read_end)  != 0 {
               __uflow(__fp)
           } else {
               let fresh2 = __fp._IO_read_ptr;
               __fp._IO_read_ptr = __fp._IO_read_ptr.offset(1);
               *(fresh2)
           };
}
#[inline]
unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
    return putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn fputc_unlocked(mut __c: i32,
                                    mut __stream: &mut FILE) -> i32 {
    return if (__stream._IO_write_ptr >= __stream._IO_write_end)  != 0 {
               __overflow(__stream, __c)
           } else {
               let fresh3 = __stream._IO_write_ptr;
               __stream._IO_write_ptr =
                   __stream._IO_write_ptr.offset(1);
               *fresh3 = __c;
               *fresh3
           };
}
#[inline]
unsafe extern "C" fn putc_unlocked(mut __c: i32,
                                   mut __stream: &mut FILE) -> i32 {
    return if (__stream._IO_write_ptr >= __stream._IO_write_end)  != 0 {
               __overflow(__stream, __c)
           } else {
               let fresh4 = __stream._IO_write_ptr;
               __stream._IO_write_ptr =
                   __stream._IO_write_ptr.offset(1);
               *fresh4 = __c;
               *fresh4
           };
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: i32) -> i32 {
    return if (stdout._IO_write_ptr >= stdout._IO_write_end)  != 0 {
               __overflow(stdout, __c)
           } else {
               let fresh5 = stdout._IO_write_ptr;
               stdout._IO_write_ptr = stdout._IO_write_ptr.offset(1);
               *fresh5 = __c;
               *fresh5
           };
}
#[inline]
unsafe extern "C" fn getline(mut __lineptr: String,
                             mut __n: &mut size_t, mut __stream: &mut FILE)
 -> SsizeT {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: &mut FILE) -> i32 {
    return (__stream._flags & 0x10 != 0) ;
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: &mut FILE) -> i32 {
    return (__stream._flags & 0x20 != 0) ;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 );
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> i32 {
    return strtol(__nptr, 0 ,
                  10);
}
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> i32 {
    return strtol(__nptr, 0 ,
                  10);
}
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char)
 -> libc::c_longlong {
    return strtoll(__nptr, 0 ,
                   10);
}
#[inline]
unsafe extern "C" fn bsearch(mut __key: *const libc::c_void,
                             mut __base: *const libc::c_void,
                             mut __nmemb: usize, mut __size: usize,
                             mut __compar: __compar_fn_t)
 ->Vec<u8> {
    let mut __l: usize = 0;
    let mut __u: usize = 0;
    let mut __idx: usize = 0;
    let mut __p: *const libc::c_void = 0;
    let mut __comparison: i32 = 0;
    __l = 0 ;
    __u = __nmemb;
    while __l < __u {
        __idx =
            __l.wrapping_add(__u).wrapping_div(2libc::c_ulong);
        __p =
            (__base           *const libc::c_char).offset(__idx.wrapping_mul(__size)     );
        __comparison =
            Some(__compar.expect("non-null function pointer")).expect("non-null function pointer")(__key,
                                                                                                   __p);
        if __comparison < 0 {
            __u = __idx
        } else if __comparison > 0 {
            __l = __idx.wrapping_add(1)
        } else { return __p }
    }
    return 0;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128) && __c < 256 {
               *(*__ctype_tolower_loc()).offset(__c)
           } else { __c };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128) && __c < 256 {
               *(*__ctype_toupper_loc()).offset(__c)
           } else { __c };
}
#[inline]
unsafe extern "C" fn strtoimax(mut nptr: *const libc::c_char,
                               mut endptr: String,
                               mut base: i32) -> intmax_t {
    return __strtol_internal(nptr, endptr, base, 0);
}
#[inline]
unsafe extern "C" fn strtoumax(mut nptr: *const libc::c_char,
                               mut endptr: String,
                               mut base: i32) -> uintmax_t {
    return __strtoul_internal(nptr, endptr, base, 0);
}
#[inline]
unsafe extern "C" fn wcstoimax(mut nptr: *const __gwchar_t,
                               mut endptr: &mut __gwchar_t.
                               mut base: i32) -> intmax_t {
    return __wcstol_internal(nptr, endptr, base, 0);
}
#[inline]
unsafe extern "C" fn wcstoumax(mut nptr: *const __gwchar_t,
                               mut endptr: &mut __gwchar_t.
                               mut base: i32) -> uintmax_t {
    return __wcstoul_internal(nptr, endptr, base, 0);
}
#[no_mangle]
pub unsafe extern "C" fn hash_questions_init() { }
#[no_mangle]
pub unsafe extern "C" fn hash_questions(mut header: DnsHeader,
                                        mut plen: usize,
                                        mut name: &mut String)
                                        -> mut Vec<u8> {
    let mut q: i32 = 0;
    let mut p: mut Vec<u8> =
        header.offset(1);
    let mut ctx: SHA256_CTX =
        SHA256_CTX{data: [0; 64], datalen: 0, bitlen: 0, state: [0; 8],};
    static mut digest: [BYTE; 32] = [0; 32];
    sha256_init(&mut ctx);
    q = __bswap_16(header.qdcount);
    while q != 0 {
        let mut cp: &mut String = 0 ;
        let mut c: libc::c_char = 0;
        /* bad packet */
        if extract_name(header, plen, &mut p, name, 1,
                        4) == 0 {
            break ; /* bad packet */
        }
        cp = name;
        loop  {
            c = cp;
            if !(c != 0) { break ; }
            if c >= 'A' as i32 &&
                   c <= 'Z' as i32 {
                cp =
                    (cp + ('a' as i32 - 'A' as i32))                  libc::c_char
            }
            cp = cp.offset(1)
        }
        sha256_update(&mut ctx, name ,
                      cp.wrapping_offset_from(name) );
        /* CRC the class and type as well */
        sha256_update(&mut ctx, p ,
                      4 );
        p = p.offset(4);
        if !((p.wrapping_offset_from(header)  + 0)
                 <= plen) {
            break ;
        }
        q -= 1
    }
    sha256_final(&mut ctx, digest.as_mut_ptr());
    return digest.as_mut_ptr();
}
/* *************************** VARIABLES *****************************/
static mut k: [WORD; 64] =
    [0x428a2f98), 0x71374491),
     0xb5c0fbcf, 0xe9b5dba5,
     0x3956c25b), 0x59f111f1),
     0x923f82a4, 0xab1c5ed5,
     0xd807aa98, 0x12835b01),
     0x243185be), 0x550c7dc3),
     0x72be5d74), 0x80deb1fe,
     0x9bdc06a7, 0xc19bf174,
     0xe49b69c1, 0xefbe4786,
     0xfc19dc6), 0x240ca1cc),
     0x2de92c6f), 0x4a7484aa),
     0x5cb0a9dc), 0x76f988da),
     0x983e5152, 0xa831c66d,
     0xb00327c8, 0xbf597fc7,
     0xc6e00bf3, 0xd5a79147,
     0x6ca6351), 0x14292967),
     0x27b70a85), 0x2e1b2138),
     0x4d2c6dfc), 0x53380d13),
     0x650a7354), 0x766a0abb),
     0x81c2c92e, 0x92722c85,
     0xa2bfe8a1, 0xa81a664b,
     0xc24b8b70, 0xc76c51a3,
     0xd192e819, 0xd6990624,
     0xf40e3585, 0x106aa070),
     0x19a4c116), 0x1e376c08),
     0x2748774c), 0x34b0bcb5),
     0x391c0cb3), 0x4ed8aa4a),
     0x5b9cca4f), 0x682e6ff3),
     0x748f82ee), 0x78a5636f),
     0x84c87814, 0x8cc70208,
     0x90befffa, 0xa4506ceb,
     0xbef9a3f7, 0xc67178f2];
/* ********************** FUNCTION DEFINITIONS ***********************/
unsafe extern "C" fn sha256_transform(mut ctx: &mut SHA256_CTX,
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
    i = 0);
    j = 0);
    while i < 16 {
        m[i ] =
            ((*data.offset(j)) << 24 |
                 (*data.offset(j.wrapping_add(1                                            libc::c_uint)) ) << 16 |
                 (*data.offset(j.wrapping_add(2                                            libc::c_uint)) ) << 8 |
                 *data.offset(j.wrapping_add(3)
                                 )));
        i = i.wrapping_add(1);
        j =
            (j).wrapping_add(4)
               ))
    }
    while i < 64 {
        m[i ] =
            ((m[i.wrapping_sub(2) ] >>
                  17 |
                  m[i.wrapping_sub(2) ]
                      << 32 - 17) ^
                 (m[i.wrapping_sub(2) ]
                      >> 19 |
                      m[i.wrapping_sub(2)                      usize] << 32 - 19) ^
                 m[i.wrapping_sub(2) ]
                     >>
                     10 ).wrapping_add(m[i.wrapping_sub(7                     libc::c_int
                                                                                            libc::c_uint)
                                                              usize]).wrapping_add((m[i.wrapping_sub(15                       libc::c_int                       libc::c_uint)
                                                                                                                      usize]
                                                                                   >>
                                                                                   7
                                                                                                                          libc::c_int
                                                                                   |
                                                                                   m[i.wrapping_sub(15                               libc::c_int                               libc::c_uint) usize]
                                                                                       <<
                                                                                       32     libc::c_int
                                                                                           -
                                                                                           7             libc::c_int)
                                                                                  ^
                                                                                  (m[i.wrapping_sub(15                               libc::c_int                               libc::c_uint) usize]
                                                                                       >>
                                                                                       18     libc::c_int
                                                                                       |
                                                                                       m[i.wrapping_sub(15                                       libc::c_int                                       libc::c_uint)         usize]
                                                                                           <<
                                                                                           32             libc::c_int
                                                                                               -
                                                                                               18                     libc::c_int)
                                                                                  ^
                                                                                  m[i.wrapping_sub(15                             libc::c_int                             libc::c_uint)
                                                                                                                            usize]
                                                                                      >>
                                                                                      3   libc::c_int).wrapping_add(m[i.wrapping_sub(16                                                                                                 libc::c_int                                                                                                 libc::c_uint)                                                                   usize]);
        i = i.wrapping_add(1)
    }
    a = ctx.state[0 ];
    b = ctx.state[1 ];
    c = ctx.state[2 ];
    d = ctx.state[3 ];
    e = ctx.state[4 ];
    f = ctx.state[5 ];
    g = ctx.state[6 ];
    h = ctx.state[7 ];
    i = 0);
    while i < 64 {
        t1 =
            h.wrapping_add((e >> 6 |
                                e << 32 - 6) ^
                               (e >> 11 |
                                    e <<
                                        32 - 11)
                               ^
                               (e >> 25 |
                                    e <<
                                        32 -
                                            25                                          libc::c_int)).wrapping_add(e &
                                                                               f
                                                                               ^
                                                                               !e
                                                                                   &
                                                                                   g).wrapping_add(k[i                                 usize]).wrapping_add(m[i                                                                                       usize]);
        t2 =
            ((a >> 2 |
                  a << 32 - 2) ^
                 (a >> 13 |
                      a << 32 - 13) ^
                 (a >> 22 |
                      a <<
                          32 -
                              22                            libc::c_int)).wrapping_add(a & b ^ a & c ^
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
    ctx.state[0 ] =
        (ctx.state[0 ]       libc::c_uint).wrapping_add(a)));
    ctx.state[1 ] =
        (ctx.state[1 ]       libc::c_uint).wrapping_add(b)));
    ctx.state[2 ] =
        (ctx.state[2 ]       libc::c_uint).wrapping_add(c)));
    ctx.state[3 ] =
        (ctx.state[3 ]       libc::c_uint).wrapping_add(d)));
    ctx.state[4 ] =
        (ctx.state[4 ]       libc::c_uint).wrapping_add(e)));
    ctx.state[5 ] =
        (ctx.state[5 ]       libc::c_uint).wrapping_add(f)));
    ctx.state[6 ] =
        (ctx.state[6 ]       libc::c_uint).wrapping_add(g)));
    ctx.state[7 ] =
        (ctx.state[7 ]       libc::c_uint).wrapping_add(h)));
}
unsafe extern "C" fn sha256_init(mut ctx: &mut SHA256_CTX) {
    ctx.datalen = 0);
    ctx.bitlen = 0long;
    ctx.state[0 ] =
        0x6a09e667);
    ctx.state[1 ] = 0xbb67ae85;
    ctx.state[2 ] =
        0x3c6ef372);
    ctx.state[3 ] = 0xa54ff53a;
    ctx.state[4 ] =
        0x510e527f);
    ctx.state[5 ] = 0x9b05688c;
    ctx.state[6 ] =
        0x1f83d9ab);
    ctx.state[7 ] =
        0x5be0cd19);
}
unsafe extern "C" fn sha256_update(mut ctx: &mut SHA256_CTX,
                                   mut data: *const BYTE, mut len: usize) {
    let mut i: WORD = 0;
    i = 0);
    while (i) < len {
        ctx.data[ctx.datalen ] = *data.offset(i);
        ctx.datalen = ctx.datalen.wrapping_add(1);
        if ctx.datalen == 64 {
            sha256_transform(ctx, ctx.data.as_mut_ptr() );
            ctx.bitlen =
                ctx.bitlen.wrapping_add(512long);
            ctx.datalen = 0)
        }
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn sha256_final(mut ctx: &mut SHA256_CTX,
                                  mut hash: &mut BYTE) {
    let mut i: WORD = 0;
    i = ctx.datalen;
    // Pad whatever data is left in the buffer.
    if ctx.datalen < 56 {
        let fresh6 = i;
        i = i.wrapping_add(1);
        ctx.data[fresh6 ] = 0x80 ;
        while i < 56 {
            let fresh7 = i;
            i = i.wrapping_add(1);
            ctx.data[fresh7 ] = 0
        }
    } else {
        let fresh8 = i;
        i = i.wrapping_add(1);
        ctx.data[fresh8 ] = 0x80 ;
        while i < 64 {
            let fresh9 = i;
            i = i.wrapping_add(1);
            ctx.data[fresh9 ] = 0
        }
        sha256_transform(ctx, ctx.data.as_mut_ptr() );
        zero_array_1(&mut ctx.data, 56);
    }
    // Append to the padding the total message's length in bits and transform.
    ctx.bitlen =
        ctx.bitlen.wrapping_add(ctx.datalen.wrapping_mul(8 ));
    ctx.data[63 ] = ctx.bitlen ;
    ctx.data[62 ] =
        (ctx.bitlen >> 8) ;
    ctx.data[61 ] =
        (ctx.bitlen >> 16) ;
    ctx.data[60 ] =
        (ctx.bitlen >> 24) ;
    ctx.data[59 ] =
        (ctx.bitlen >> 32) ;
    ctx.data[58 ] =
        (ctx.bitlen >> 40) ;
    ctx.data[57 ] =
        (ctx.bitlen >> 48) ;
    ctx.data[56 ] =
        (ctx.bitlen >> 56) ;
    sha256_transform(ctx, ctx.data.as_mut_ptr() );
    // Since this implementation uses little endian byte ordering and SHA uses big endian,
  // reverse all the bytes when copying the final state to the output hash.
    i = 0);
    while i < 4 {
        *hash.offset(i) =
            (ctx.state[0 ] >>
                 (24)).wrapping_sub(i.wrapping_mul(8)))
                 & 0xff) ;
        *hash.offset(i.wrapping_add(4) )) =
            (ctx.state[1 ] >>
                 (24)).wrapping_sub(i.wrapping_mul(8)))
                 & 0xff) ;
        *hash.offset(i.wrapping_add(8) )) =
            (ctx.state[2 ] >>
                 (24)).wrapping_sub(i.wrapping_mul(8)))
                 & 0xff) ;
        *hash.offset(i.wrapping_add(12) )) =
            (ctx.state[3 ] >>
                 (24)).wrapping_sub(i.wrapping_mul(8)))
                 & 0xff) ;
        *hash.offset(i.wrapping_add(16) )) =
            (ctx.state[4 ] >>
                 (24)).wrapping_sub(i.wrapping_mul(8)))
                 & 0xff) ;
        *hash.offset(i.wrapping_add(20) )) =
            (ctx.state[5 ] >>
                 (24)).wrapping_sub(i.wrapping_mul(8)))
                 & 0xff) ;
        *hash.offset(i.wrapping_add(24) )) =
            (ctx.state[6 ] >>
                 (24)).wrapping_sub(i.wrapping_mul(8)))
                 & 0xff) ;
        *hash.offset(i.wrapping_add(28) )) =
            (ctx.state[7 ] >>
                 (24)).wrapping_sub(i.wrapping_mul(8)))
                 & 0xff) ;
        i = i.wrapping_add(1)
    };
}
