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
use rand::Rng;
use std::fs::{File, read, write};
use std::io::{Seek, Read};
use crate::dnsmasq_log::{die, my_syslog};
use crate::network::fix_fd;
use crate::defines::{NetAddress, SyscallSlongT, };


// static mut seed: [u32; 32] = [0; 32];
// static mut in_0: [u32; 12] = [0; 12];
// static mut out: [u32; 8] = [0; 8];
// static mut outleft: libc::c_int = 0 as libc::c_int;

// pub fn rand_init() {
//     let mut fd: libc::c_int =
//         open(b"/dev/urandom\x00" as *const u8 as *const libc::c_char,
//              0 as libc::c_int);
//     if fd == -(1 as libc::c_int) ||
//            read_write(fd, &mut seed as *mut [u32; 32] as *mut libc::c_uchar,
//                       ::std::mem::size_of::<[u32; 32]>() as libc::c_ulong as
//                           libc::c_int, 1 as libc::c_int) == 0 ||
//            read_write(fd, &mut in_0 as *mut [u32; 12] as *mut libc::c_uchar,
//                       ::std::mem::size_of::<[u32; 12]>() as libc::c_ulong as
//                           libc::c_int, 1 as libc::c_int) == 0 {
//         die(b"failed to seed the random number generator: %s\x00" as *const u8
//                 as *const libc::c_char as *mut libc::c_char,
//             0 as *mut libc::c_char, 5 as libc::c_int);
//     }
//     close(fd);
// }


fn surf() {
    let mut t: [u32; 12] = [0; 12];
    let mut x: u32 = 0;
    let mut sum: u32 = 0;
    let mut r: i32 = 0;
    let mut i: i32 = 0;
    let mut loop_0: i32 = 0;
    i = 0;
    while i < 12 {
        t[i ] =
            in_0[i ] ^ seed[(12 + i) ];
        i += 1
    }
    i = 0;
    while i < 8 {
        out[i ] = seed[(24 + i) ];
        i += 1
    }
    x = t[11 ];
    loop_0 = 0;
    while loop_0 < 2 {
        r = 0;
        while r < 16 {
            sum =
                (sum).wrapping_add(0x9e3779b9)
                   ;
            t[0 ] =
                (t[0 ]               libc::c_uint).wrapping_add((x ^
                                                     seed[0           usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 5 |
                                                         x >>
                                                             32
                                                                 -
                                                                 5                  libc::c_int))
                   ;
            x = t[0 ];
            t[1 ] =
                (t[1 ]               libc::c_uint).wrapping_add((x ^
                                                     seed[1           usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 7 |
                                                         x >>
                                                             32
                                                                 -
                                                                 7                  libc::c_int))
                   ;
            x = t[1 ];
            t[2 ] =
                (t[2 ]               libc::c_uint).wrapping_add((x ^
                                                     seed[2           usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 9 |
                                                         x >>
                                                             32
                                                                 -
                                                                 9                  libc::c_int))
                   ;
            x = t[2 ];
            t[3 ] =
                (t[3 ]               libc::c_uint).wrapping_add((x ^
                                                     seed[3           usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 13 |
                                                         x >>
                                                             32
                                                                 -
                                                                 13                  libc::c_int))
                   ;
            x = t[3 ];
            t[4 ] =
                (t[4 ]               libc::c_uint).wrapping_add((x ^
                                                     seed[4           usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 5 |
                                                         x >>
                                                             32
                                                                 -
                                                                 5                  libc::c_int))
                   ;
            x = t[4 ];
            t[5 ] =
                (t[5 ]               libc::c_uint).wrapping_add((x ^
                                                     seed[5           usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 7 |
                                                         x >>
                                                             32
                                                                 -
                                                                 7                  libc::c_int))
                   ;
            x = t[5 ];
            t[6 ] =
                (t[6 ]               libc::c_uint).wrapping_add((x ^
                                                     seed[6           usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 9 |
                                                         x >>
                                                             32
                                                                 -
                                                                 9                  libc::c_int))
                   ;
            x = t[6 ];
            t[7 ] =
                (t[7 ]               libc::c_uint).wrapping_add((x ^
                                                     seed[7           usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 13 |
                                                         x >>
                                                             32
                                                                 -
                                                                 13                  libc::c_int))
                   ;
            x = t[7 ];
            t[8 ] =
                (t[8 ]               libc::c_uint).wrapping_add((x ^
                                                     seed[8           usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 5 |
                                                         x >>
                                                             32
                                                                 -
                                                                 5                  libc::c_int))
                   ;
            x = t[8 ];
            t[9 ] =
                (t[9 ]               libc::c_uint).wrapping_add((x ^
                                                     seed[9           usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 7 |
                                                         x >>
                                                             32
                                                                 -
                                                                 7                  libc::c_int))
                   ;
            x = t[9 ];
            t[10 ] =
                (t[10 ]               libc::c_uint).wrapping_add((x ^
                                                     seed[10           usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 9 |
                                                         x >>
                                                             32
                                                                 -
                                                                 9                  libc::c_int))
                   ;
            x = t[10 ];
            t[11 ] =
                (t[11 ]               libc::c_uint).wrapping_add((x ^
                                                     seed[11           usize]).wrapping_add(sum)
                                                    ^
                                                    (x << 13 |
                                                         x >>
                                                             32
                                                                 -
                                                                 13                  libc::c_int))
                   ;
            x = t[11 ];
            r += 1
        }
        i = 0;
        while i < 8 {
            out[i ] ^= t[(i + 4) ];
            i += 1
        }
        loop_0 += 1
    };
}

pub fn rand16() -> u16 {
    if outleft == 0 {
        in_0[0 ] =
            in_0[0 ].wrapping_add(1);
        if in_0[0 ] == 0 {
            in_0[1 ] =
                in_0[1 ].wrapping_add(1);
            if in_0[1 ] == 0 {
                in_0[2 ] =
                    in_0[2 ].wrapping_add(1);
                if in_0[2 ] == 0 {
                    in_0[3 ] =
                        in_0[3 ].wrapping_add(1)
                }
            }
        }
        surf();
        outleft = 8
    }
    outleft -= 1;
    return out[outleft ] ;
}
pub fn rand32() -> u32 {
    if outleft == 0 {
        in_0[0 ] =
            in_0[0 ].wrapping_add(1);
        if in_0[0 ] == 0 {
            in_0[1 ] =
                in_0[1 ].wrapping_add(1);
            if in_0[1 ] == 0 {
                in_0[2 ] =
                    in_0[2 ].wrapping_add(1);
                if in_0[2 ] == 0 {
                    in_0[3 ] =
                        in_0[3 ].wrapping_add(1)
                }
            }
        }
        surf();
        outleft = 8
    }
    outleft -= 1;
    return out[outleft ];
}
pub fn rand64() -> u64 {
    let x: u64 = rng.gen();
    x
}

/* returns 2 if names is OK but contains one or more underscores */
pub fn check_name(in_1: &mut String) -> i32 {
    /* remove trailing . 
     also fail empty string and label > 63 chars */
    let mut dotgap: usize = 0;
    let mut l: usize = in_1.len();
    let mut c: libc::c_char = 0;
    let mut nowhite: i32 = 0;
    let mut hasuscore: i32 = 0;
    if l == 0 || l > 1025 {
        return 0
    }
    if in_1.offset(l.wrapping_sub(1)) == '.' as i32 {
        in_1.offset(l.wrapping_sub(1)) = 0;
        nowhite = 1
    }
    for c in in_1  {
        if c == 0 { break ; }
        if c == '.' as i32 {
            dotgap = 0
        } else {
            dotgap = dotgap.wrapping_add(1);
            if dotgap > 63 {
                return 0
            } else {
                if c & !(0x7f) == 0 && __ctype_b_loc().offset(c) & _ISCNTRL != 0 {
                    /* iscntrl only gives expected results for ascii */
                    return 0
                } else {
                    if !(c &
                             !(0x7f) == 0) {
                        return 0
                    } else {
                        if c != ' ' as i32 {
                            nowhite = 1;
                            if c == '_' as i32 {
                                hasuscore = 1
                            }
                        }
                    }
                }
            }
        }
    }
    if nowhite == 0 { return 0 }
    return if hasuscore != 0 { 2 } else { 1 };
}
/* Hostnames have a more limited valid charset than domain names
   so check for legal char a-z A-Z 0-9 - _ 
   Note that this may receive a FQDN, so only check the first label 
   for the tighter criteria. */

pub fn legal_hostname(mut name: &String)
 -> bool {
    let mut c: libc::c_char = 0;
    let mut first: i32 = 0;
    if check_name(name) == 0 { return 0 }
    
    let mut first: bool = true;
    for c in name {
        if (c as u8 >= 'A' as u8 && c as u8 <= 'Z' as u8) || (c as u8 >= 'a' as u8 && c as u8 <= 'z' as u8) || (c as u8 >= '0' as u8 && c as u8 <= '9') {
            if !(first && (c as u8  == '-' as u8 || c as u8 == '_' as u8)) {
                if (c as u8 == '.' as u8) {
                    return true;
                }
                return false;
            }

        }
    }
    return true;
}

pub fn string_from_offset(in_str: &String, offset: usize) -> String {
    String::from(in_str.as_bytes()[offset..])
}


pub fn canonicalise(in_1: &mut String, nomem: i32)
 -> Option<String> {
    let mut ret= String::new();
    let mut rc: i32 = 0;
    rc = check_name(in_1);
    if rc == 0 { return None }

    ret = in_1.clone();
    return Some(ret);
}
pub fn do_rfc1035_name(mut p:&mut String,
                                         mut sval: &mut String,
                                         mut limit: &mut String)
 -> String {
    let mut j = 0;
    while !sval.is_null() && *sval != 0 {
        let fresh6 = p;
        p = p.offset(1);
        let mut cp: String = fresh6;
        if !limit.is_null() && p > limit {
            return 0
        }
        j = 0;
        while *sval != 0 && *sval != '.' as i32
              {
            if !limit.is_null() &&
                   p.offset(1) >
                       limit {
                return 0
            }
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = *sval;
            sval = sval.offset(1);
            j += 1
        }
        cp = j;
        if *sval != 0 { sval = sval.offset(1) }
    }
    return p;
}

pub fn netaddr_isequal(a: &NetAddress, b: &NetAddress) -> bool {
    if a._type != b._type {
        false
    }
    if a.port != b.port {
        false
    }
    if a.name != b.name {
        false
    }
    if a.value != b.value {
        false
    }
    true
}

pub fn NetAddress_isequal(mut s1: &NetAddress, mut s2: &NetAddress) -> bool
{
    if s1



    if (*s1).sa.sa_family == (*s2).sa.sa_family
       {
        if (*s1).sa.sa_family == 2 &&
               (*s1).in_0.sin_port ==
                   (*s2).in_0.sin_port &&
               (*s1).in_0.sin_addr.s_addr == (*s2).in_0.sin_addr.s_addr {
            return 1
        }
        if (*s1).sa.sa_family == 10 &&
               (*s1).in6.sin6_port ==
                   (*s2).in6.sin6_port &&
               (*s1).in6.sin6_scope_id == (*s2).in6.sin6_scope_id &&
               ({
                    let mut __a: *const In6Addr =
                        &mut (*s1).in6.sin6_addr                      *const In6Addr;
                    let mut __b: *const In6Addr =
                        &mut (*s2).in6.sin6_addr                      *const In6Addr;
                    ((*__a).__in6_u.__u6_addr32[0 ] ==
                         (*__b).__in6_u.__u6_addr32[0 ]
                         &&
                         (*__a).__in6_u.__u6_addr32[1 ]
                             ==
                             (*__b).__in6_u.__u6_addr32[1         usize] &&
                         (*__a).__in6_u.__u6_addr32[2 ]
                             ==
                             (*__b).__in6_u.__u6_addr32[2         usize] &&
                         (*__a).__in6_u.__u6_addr32[3 ]
                             ==
                             (*__b).__in6_u.__u6_addr32[3         usize])
                }) != false {
            return true
        }
    }
    return false;
}

pub fn sa_len(mut addr: NetAddress) -> usize {
    if addr.sa.sa_family == 10 {
        ::std::mem::size_of::<NetAddress>()
    } else {
        ::std::mem::size_of::<NetAddress>()
    }
}

/* don't use strcasecmp and friends here - they may be messed up by LOCALE */
pub fn hostname_isequal(mut a: &String, mut b: &String)
 -> bool {
    a == b
}

/* is b equal to or a subdomain of a return 2 for equal, 1 for subdomain */
pub fn hostname_issubdomain(mut a: &String, mut b: &String)
 -> i32 {
    let mut ap: String;
    let mut bp: String;
    let mut c1: u32 = 0;
    let mut c2: u32 = 0;
    /* move to the end */
    ap = a.clone();
    while *ap != 0 { ap = ap.offset(1) }
    bp = b.clone();
    while *bp != 0 { bp = bp.offset(1) }
    /* a shorter than b or a empty. */
    if (bp.wrapping_offset_from(b)) <
           ap.wrapping_offset_from(a) || ap == a {
        return 0
    }
    loop  {
        ap = ap.offset(-1);
        c1 = *ap;
        bp = bp.offset(-1);
        c2 = *bp;
        if c1 >= 'A' as i32 &&
               c1 <= 'Z' as i32 {
            c1 = c1.wrapping_add(('a' as i32 - 'A' as i32))
        }
        if c2 >= 'A' as i32 &&
               c2 <= 'Z' as i32 {
            c2 = c2.wrapping_add(('a' as i32 - 'A' as i32))
        }
        if c1 != c2 { return 0 }
        if !(ap != a) { break ; }
    }
    if bp == b { return 2 }
    bp = bp.offset(-1);
    if *bp == '.' as i32 { return 1 }
    return 0;
}
pub fn dnsmasq_time() -> time::Instant {
    return time(0);
}
pub fn netmask_length(mut mask: NetAddress) -> i32 {
    let mut zero_count: i32 = 0;
    while 0 ==
              mask.s_addr & 0x1 &&
              zero_count < 32 {
        mask.s_addr >>= 1;
        zero_count += 1
    }
    return 32 - zero_count;
}
pub fn is_same_net(mut a: NetAddress, mut b: NetAddress, mut mask: NetAddress) -> i32 {
    return (a.s_addr & mask.s_addr == b.s_addr & mask.s_addr);
}

pub fn is_same_net6(mut a: *mut In6Addr,
                    mut b: *mut In6Addr,
                    mut prefixlen: i32)
                    -> i32 {
    let mut pfbytes: i32 = prefixlen >> 3;
    let mut pfbits: i32 = prefixlen & 7;
    if memcmp(&mut a.__in6_u.__u6_addr8             *const libc::c_void,
              &mut b.__in6_u.__u6_addr8             *const libc::c_void, pfbytes) !=
           0 {
        return 0
    }
    if pfbits == 0 ||
           a.__in6_u.__u6_addr8[pfbytes ] >>
               8 - pfbits ==
               b.__in6_u.__u6_addr8[pfbytes ] >>
                   8 - pfbits {
        return 1
    }
    return 0;
}

/* return least significant 64 bits if IPv6 address */
pub fn addr6part(mut addr: *mut In6Addr) -> u64 {
    let mut i: i32 = 0;
    let mut ret: u64 = 0 as u64;
    i = 8;
    while i < 16 {
        ret =
            (ret <<
                 8 ).wrapping_add(addr.__in6_u.__u6_addr8[i                           usize]
                                                  long);
        i += 1
    }
    return ret;
}
pub fn setaddr6part(mut addr: *mut In6Addr,
                                      mut host: u64) {
    let mut i: i32 = 0;
    i = 15;
    while i >= 8 {
        addr.__in6_u.__u6_addr8[i ] = host as u8;
        host = host >> 8;
        i -= 1
    };
}
/* returns port number from address */
pub fn prettyprint_addr(mut addr: NetAddress,
                                          mut buf: &mut String)
                                          -> i32 {
    let mut port: i32 = 0;
    if addr.sa.sa_family == 2 {
        inet_ntop(2,
                  &mut addr.in_0.sin_addr                *const libc::c_void, buf,
                  46);
        port = __bswap_16(addr.in_0.sin_port)
    } else if addr.sa.sa_family == 10 {
        let mut name: [libc::c_char; 16] = [0; 16];
        inet_ntop(10,
                  &mut addr.in6.sin6_addr                *const libc::c_void, buf,
                  46);
        if addr.in6.sin6_scope_id != 0 &&
               !if_indextoname(addr.in6.sin6_scope_id,
                               name.as_mut_ptr()).is_null() &&
               strlen(buf).wrapping_add(strlen(name.as_mut_ptr())).wrapping_add(2
                                                                                                                    libc::c_int
                                                                                                             )
                   <= 46 {
            strcat(buf, b"%\x00" );
            strcat(buf, name.as_mut_ptr());
        }
        port = __bswap_16(addr.in6.sin6_port)
    }
    return port;
}
pub fn prettyprint_time(mut buf: &mut String,
                                          mut t: u32) {
    if t == 0xffffffff {
        sprintf(buf, b"infinite\x00" );
    } else {
        let mut x: u32 = 0;
        let mut p: u32 = 0;
        x = t.wrapping_div(86400);
        if x != 0 {
            p =
                p.wrapping_add(sprintf(&mut *buf.offset(p)                                     &mut String,
                                       b"%ud\x00"                                      *const libc::c_char, x)                             libc::c_uint)
        }
        x =
            t.wrapping_div(3600                         libc::c_uint).wrapping_rem(24           libc::c_uint);
        if x != 0 {
            p =
                p.wrapping_add(sprintf(&mut *buf.offset(p)                                     &mut String,
                                       b"%uh\x00"                                      *const libc::c_char, x)                             libc::c_uint)
        }
        x =
            t.wrapping_div(60                         libc::c_uint).wrapping_rem(60           libc::c_uint);
        if x != 0 {
            p =
                p.wrapping_add(sprintf(&mut *buf.offset(p)                                     &mut String,
                                       b"%um\x00"                                      *const libc::c_char, x)                             libc::c_uint)
        }
        x = t.wrapping_rem(60);
        if x != 0 {
            p =
                p.wrapping_add(sprintf(&mut *buf.offset(p)                                     &mut String,
                                       b"%us\x00"                                      *const libc::c_char, x)                             libc::c_uint)
        }
    };
}
/* in may equal out, when maxlen may be -1 (No max len). 
   Return -1 for extraneous no-hex chars found. */
pub fn parse_hex(mut in_1: &mut String,
                                   mut out_0: mut Vec<u8>,
                                   mut maxlen: i32,
                                   mut wildcard_mask: *mut libc::c_uint,
                                   mut mac_type: )
 -> i32 {
    let mut done: i32 = 0;
    let mut mask: i32 = 0;
    let mut i: i32 = 0;
    let mut r: &mut String = 0 ;
    if !mac_type.is_null() { *mac_type = 0 }
    while done == 0 && (maxlen == -(1) || i < maxlen) {
        r = in_1;
        while *r != 0 &&
                  *r != ':' as i32 &&
                  *r != '-' as i32 &&
                  *r != ' ' as i32 {
            if *r != '*' as i32 &&
                   *(*__ctype_b_loc()).offset(*r                                            libc::c_int) &
                       _ISXDIGIT                      libc::c_int == 0 {
                return -(1)
            }
            r = r.offset(1)
        }
        if *r == 0 { done = 1 }
        if r != in_1 {
            if *r == '-' as i32 && i == 0 &&
                   !mac_type.is_null() {
                *r = 0;
                *mac_type =
                    strtol(in_1, 0 ,
                           16);
                mac_type = 0      } else {
                *r = 0;
                if strcmp(in_1, b"*\x00" )
                       == 0 {
                    mask = mask << 1 | 1;
                    i += 1
                } else {
                    let mut j: i32 = 0;
                    let mut bytes: i32 =
                        ((1 +
                              r.wrapping_offset_from(in_1)) /
                             2);
                    j = 0;
                    while j < bytes {
                        let mut sav: libc::c_char = 0;
                        sav = sav;
                        if j < bytes - 1 {
                            sav =
                                *in_1.offset(((j + 1) *
                                                  2));
                            *in_1.offset(((j + 1) *
                                              2)) =
                                0
                        }
                        /* checks above allow mix of hexdigit and *, which
			 is illegal. */
                        if !strchr(&mut *in_1.offset((j * 2)      isize),
                                   '*' as i32).is_null() {
                            return -(1)
                        }
                        *out_0.offset(i) =
                            strtol(&mut *in_1.offset((j * 2)      isize),
                                   0 ,
                                   16);
                        mask = mask << 1;
                        i += 1;
                        if i == maxlen { break ; }
                        if j < bytes - 1 {
                            *in_1.offset(((j + 1) *
                                              2)) =
                                sav
                        }
                        j += 1
                    }
                }
            }
        }
        in_1 = r.offset(1)
    }
    if !wildcard_mask.is_null() { *wildcard_mask = mask }
    return i;
}
/* return 0 for no match, or (no matched octets) + 1 */
pub fn memcmp_masked(mut a: mut Vec<u8>,
                                       mut b: mut Vec<u8>,
                                       mut len: i32,
                                       mut mask: u32)
 -> i32 {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    count = 1;
    i = len - 1;
    while i >= 0 {
        if mask & 1 == 0 {
            if *a.offset(i) ==
                   *b.offset(i) {
                count += 1
            } else { return 0 }
        }
        i -= 1;
        mask = mask >> 1
    }
    return count;
}

/* _note_ may copy buffer */
pub unsafe fn expand_buf(mut iov: &mut iovec, mut size: usize)
 -> u8 {
    let mut new:Vec<u8> = 0;
    if size <= iov.iov_len { return 1; }
    new = whine_malloc(size);
    if new.is_null() {
        *__errno_location() = 12;
        return 0;
    }
    if !iov.iov_base.is_null() {
        memcpy(new, iov.iov_base, iov.iov_len);
        free(iov.iov_base);
    }
    iov.iov_base = new;
    iov.iov_len = size;
    return 1;
}

pub fn print_mac(mut buff: &mut String,
                                   mut mac: mut Vec<u8>,
                                   mut len: i32)
 -> &mut String {
    let mut p: &mut String = buff;
    let mut i: i32 = 0;
    if len == 0 {
        sprintf(p, b"<null>\x00" );
    } else {
        i = 0;
        while i < len {
            p =
                p.offset(sprintf(p,
                                 b"%.2x%s\x00"                                *const libc::c_char,
                                 *mac.offset(i),
                                 if i == len - 1 {
                                     b"\x00"
                                 } else {
                                     b":\x00"
                                 }));
            i += 1
        }
    }
    return buff;
}
/* rc is return from sendto and friends.
   Return 1 if we should retry.
   Set errno to zero if we succeeded. */
pub fn retry_send(mut rc: susize) -> i32 {
    static mut retries: i32 = 0;
    let mut waiter: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
    if rc != -(1) {
        retries = 0;
        *__errno_location() = 0;
        return 0
    }
    /* Linux kernels can return EAGAIN in perpetuity when calling
     sendmsg() and the relevant interface has gone. Here we loop
     retrying in EAGAIN for 1 second max, to avoid this hanging 
     dnsmasq. */
    if *__errno_location() == 11 ||
           *__errno_location() == 11 {
        waiter.tv_sec = 0 ;
        waiter.tv_nsec = 10000;
        nanosleep(&mut waiter, 0);
        let fresh10 = retries;
        retries = retries + 1;
        if fresh10 < 1000 { return 1 }
    }
    retries = 0;
    if *__errno_location() == 4 { return 1 }
    return 0;
}

enum ReadWriteMode {
    READ,
    WRITE
}

// pub fn read_write(fd: &mut File,
//                   packet: &mut Vec<u8>,
//                   size: &mut usize,
//                   rw: &ReadWriteMode) -> Result<(), &'static str> {
//     let mut n: usize = 0;
//     let mut done: usize = 0;
//     while done < size as libc::c_long {
//         loop  {
//             if rw == ReadWriteMode::READ {
//                 fd.seek(done)?;
//                 n = fd.read(packet, size)?;
//                 n =
//                     read(fd,
//                          &mut *packet.offset(done as isize) as
//                              *mut libc::c_uchar as *mut libc::c_void,
//                          (size as libc::c_long - done) as usize)
//             } else {
//                 n =
//                     write(fd,
//                           &mut *packet.offset(done as isize) as
//                               *mut libc::c_uchar as *const libc::c_void,
//                           (size as libc::c_long - done) as usize)
//             }
//             if n == 0 as libc::c_int as libc::c_long {
//                 return 0 as libc::c_int
//             }
//             if !(retry_send(n) != 0 ||
//                      *__errno_location() == 12 as libc::c_int ||
//                      *__errno_location() == 105 as libc::c_int) {
//                 break ;
//             }
//         }
//         if *__errno_location() != 0 as libc::c_int { return 0 as libc::c_int }
//         done += n
//     }
//     return 1 as libc::c_int;
// }

/* close all fds except STDIN, STDOUT and STDERR, spare1, spare2 and spare3 */
// pub fn close_fds(mut max_fd: libc::c_long,
//                                    mut spare1: libc::c_int,
//                                    mut spare2: libc::c_int,
//                                    mut spare3: libc::c_int) {
//     /* On Linux, use the /proc/ filesystem to find which files
//      are actually open, rather than iterate over the whole space,
//      for efficiency reasons. If this fails we drop back to the dumb code. */
//     let mut d: *mut DIR = 0 as *mut DIR;
//     d = opendir(b"/proc/self/fd\x00" as *const u8 as *const libc::c_char);
//     if !d.is_null() {
//         let mut de: *mut dirent = 0 as *mut dirent;
//         loop  {
//             de = readdir(d);
//             if de.is_null() { break ; }
//             let mut fd: libc::c_long = 0;
//             let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
//             *__errno_location() = 0 as libc::c_int;
//             fd = strtol((*de).d_name.as_mut_ptr(), &mut e, 10 as libc::c_int);
//             if *__errno_location() != 0 as libc::c_int || e.is_null() ||
//                    *e as libc::c_int != 0 || fd == dirfd(d) as libc::c_long ||
//                    fd == 1 as libc::c_int as libc::c_long ||
//                    fd == 2 as libc::c_int as libc::c_long ||
//                    fd == 0 as libc::c_int as libc::c_long ||
//                    fd == spare1 as libc::c_long ||
//                    fd == spare2 as libc::c_long ||
//                    fd == spare3 as libc::c_long {
//                 continue ;
//             }
//             close(fd as libc::c_int);
//         }
//         closedir(d);
//         return
//     }
//     /* fallback, dumb code. */
//     max_fd -= 1;
//     while max_fd >= 0 as libc::c_int as libc::c_long {
//         if max_fd != 1 as libc::c_int as libc::c_long &&
//                max_fd != 2 as libc::c_int as libc::c_long &&
//                max_fd != 0 as libc::c_int as libc::c_long &&
//                max_fd != spare1 as libc::c_long &&
//                max_fd != spare2 as libc::c_long &&
//                max_fd != spare3 as libc::c_long {
//             close(max_fd as libc::c_int);
//         }
//         max_fd -= 1
//     };
// }

/* Basically match a string value against a wildcard pattern.  */
// pub fn wildcard_match(mut wildcard: *const libc::c_char,
//                                         mut match_0: *const libc::c_char)
//  -> libc::c_int {
//     while *wildcard as libc::c_int != 0 && *match_0 as libc::c_int != 0 {
//         if *wildcard as libc::c_int == '*' as i32 { return 1 as libc::c_int }
//         if *wildcard as libc::c_int != *match_0 as libc::c_int {
//             return 0 as libc::c_int
//         }
//         wildcard = wildcard.offset(1);
//         match_0 = match_0.offset(1)
//     }
//     return (*wildcard as libc::c_int == *match_0 as libc::c_int) as
//                libc::c_int;
// }

/* The same but comparing a maximum of NUM characters, like strncmp.  */
// pub fn wildcard_matchn(mut wildcard: *const libc::c_char,
//                                          mut match_0: *const libc::c_char,
//                                          mut num: libc::c_int)
//  -> libc::c_int {
//     while *wildcard as libc::c_int != 0 && *match_0 as libc::c_int != 0 &&
//               num != 0 {
//         if *wildcard as libc::c_int == '*' as i32 { return 1 as libc::c_int }
//         if *wildcard as libc::c_int != *match_0 as libc::c_int {
//             return 0 as libc::c_int
//         }
//         wildcard = wildcard.offset(1);
//         match_0 = match_0.offset(1);
//         num -= 1
//     }
//     return (num == 0 || *wildcard as libc::c_int == *match_0 as libc::c_int)
//                as libc::c_int;
// }

#[cfg(target_os = "linux")]
pub unsafe fn get_linux_kernel_version() -> i32 {
    let mut utsname: utsname =
        utsname{sysname: [0; 65],
                nodename: [0; 65],
                release: [0; 65],
                version: [0; 65],
                machine: [0; 65],
                domainname: [0; 65],};
    let mut version: i32 = 0;
    let mut split: &mut String = 0 ;
    if libc::uname(&mut utsname) < 0 {
        die(b"failed to find kernel version: %s\x00",
            0 , 5);
    }
    split =
        strtok(utsname.release.as_mut_ptr(),
               b".\x00" );
    version = if !split.is_null() { atoi(split) } else { 0 };
    split =
        strtok(0 ,
               b".\x00" );
    version =
        version * 256 +
            (if !split.is_null() { atoi(split) } else { 0 });
    split =
        strtok(0 ,
               b".\x00" );
    return version * 256 +
               (if !split.is_null() {
                    atoi(split)
                } else { 0 });
}

pub fn zero_array_1<T>(array: &mut T, len: usize) {
    for i in len {
        array[i] = 0
    }
}
