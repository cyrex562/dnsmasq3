
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
pub unsafe extern "C" fn extract_name(mut header: *mut dns_header,
                                      mut plen: size_t,
                                      mut pp: *mut *mut libc::c_uchar,
                                      mut name: *mut libc::c_char,
                                      mut isExtract: libc::c_int,
                                      mut extrabytes: libc::c_int)
 -> libc::c_int {
    let mut cp: *mut libc::c_uchar = name as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = *pp;
    let mut p1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut j: libc::c_uint = 0;
    let mut l: libc::c_uint = 0;
    let mut namelen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut hops: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut retvalue: libc::c_int = 1 as libc::c_int;
    if isExtract != 0 { *cp = 0 as libc::c_int as libc::c_uchar }
    loop  {
        let mut label_type: libc::c_uint = 0;
        if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                  libc::c_long + 1 as libc::c_int as libc::c_long) as size_t
                 <= plen) {
            return 0 as libc::c_int
        }
        let fresh6 = p;
        p = p.offset(1);
        l = *fresh6 as libc::c_uint;
        if l == 0 as libc::c_int as libc::c_uint {
            /* label types 0x40 and 0x80 not supported */
            /* end marker */
            /* check that there are the correct no. of bytes after the name */
            if !(((if !p1.is_null() {
                       p1
                   } else {
                       p
                   }).wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + extrabytes as libc::c_long) as size_t <=
                     plen) {
                return 0 as libc::c_int
            }
            if isExtract != 0 {
                if cp != name as *mut libc::c_uchar { cp = cp.offset(-1) }
                *cp = 0 as libc::c_int as libc::c_uchar
                /* terminate: lose final period */
            } else if *cp as libc::c_int != 0 as libc::c_int {
                retvalue = 2 as libc::c_int
            }
            if !p1.is_null() {
                /* we jumped via compression */
                *pp = p1
            } else { *pp = p }
            return retvalue
        }
        label_type = l & 0xc0 as libc::c_int as libc::c_uint;
        if label_type == 0xc0 as libc::c_int as libc::c_uint {
            /* pointer */
            if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + 1 as libc::c_int as libc::c_long) as
                     size_t <= plen) {
                return 0 as libc::c_int
            }
            /* get offset */
            l = (l & 0x3f as libc::c_int as libc::c_uint) << 8 as libc::c_int;
            let fresh7 = p;
            p = p.offset(1);
            l |= *fresh7 as libc::c_uint;
            if p1.is_null() {
                /* first jump, save location to go back to */
                p1 = p
            } /* break malicious infinite loops */
            hops = hops.wrapping_add(1);
            if hops > 255 as libc::c_int as libc::c_uint {
                return 0 as libc::c_int
            }
            p = (header as *mut libc::c_uchar).offset(l as isize)
        } else if label_type == 0 as libc::c_int as libc::c_uint {
            /* label_type = 0 -> label. */
            namelen =
                namelen.wrapping_add(l.wrapping_add(1 as libc::c_int as
                                                        libc::c_uint)); /* include period */
            if namelen >= 1025 as libc::c_int as libc::c_uint {
                return 0 as libc::c_int
            }
            if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + l as libc::c_long) as size_t <= plen) {
                return 0 as libc::c_int
            }
            j = 0 as libc::c_int as libc::c_uint;
            while j < l {
                if isExtract != 0 {
                    let mut c: libc::c_uchar = *p;
                    if c as libc::c_int != 0 as libc::c_int &&
                           c as libc::c_int != '.' as i32 {
                        let fresh8 = cp;
                        cp = cp.offset(1);
                        *fresh8 = c
                    } else { return 0 as libc::c_int }
                } else {
                    let mut c1: libc::c_uchar = *cp;
                    let mut c2: libc::c_uchar = *p;
                    if c1 as libc::c_int == 0 as libc::c_int {
                        retvalue = 2 as libc::c_int
                    } else {
                        cp = cp.offset(1);
                        if c1 as libc::c_int >= 'A' as i32 &&
                               c1 as libc::c_int <= 'Z' as i32 {
                            c1 =
                                (c1 as libc::c_int +
                                     ('a' as i32 - 'A' as i32)) as
                                    libc::c_uchar
                        }
                        if c2 as libc::c_int >= 'A' as i32 &&
                               c2 as libc::c_int <= 'Z' as i32 {
                            c2 =
                                (c2 as libc::c_int +
                                     ('a' as i32 - 'A' as i32)) as
                                    libc::c_uchar
                        }
                        if c1 as libc::c_int != c2 as libc::c_int {
                            retvalue = 2 as libc::c_int
                        }
                    }
                }
                j = j.wrapping_add(1);
                p = p.offset(1)
            }
            if isExtract != 0 {
                let fresh9 = cp;
                cp = cp.offset(1);
                *fresh9 = '.' as i32 as libc::c_uchar
            } else if *cp as libc::c_int != 0 as libc::c_int &&
                          {
                              let fresh10 = cp;
                              cp = cp.offset(1);
                              (*fresh10 as libc::c_int) != '.' as i32
                          } {
                retvalue = 2 as libc::c_int
            }
        } else { return 0 as libc::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn in_arpa_name_2_addr(mut namein: *mut libc::c_char,
                                             mut addrp: *mut all_addr)
 -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut name: [libc::c_char; 76] = [0; 76];
    let mut cp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addr: *mut libc::c_uchar = addrp as *mut libc::c_uchar;
    let mut lastchunk: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut penchunk: *mut libc::c_char = 0 as *mut libc::c_char;
    if strlen(namein) > 75 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    }
    memset(addrp as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<all_addr>() as libc::c_ulong);
    /* turn name into a series of asciiz strings */
  /* j counts no. of labels */
    j = 1 as libc::c_int;
    cp1 = name.as_mut_ptr();
    while *namein != 0 {
        if *namein as libc::c_int == '.' as i32 {
            penchunk = lastchunk;
            lastchunk = cp1.offset(1 as libc::c_int as isize);
            *cp1 = 0 as libc::c_int as libc::c_char;
            j += 1
        } else { *cp1 = *namein }
        cp1 = cp1.offset(1);
        namein = namein.offset(1)
    }
    *cp1 = 0 as libc::c_int as libc::c_char;
    if j < 3 as libc::c_int { return 0 as libc::c_int }
    if hostname_isequal(lastchunk,
                        b"arpa\x00" as *const u8 as *const libc::c_char) != 0
           &&
           hostname_isequal(penchunk,
                            b"in-addr\x00" as *const u8 as
                                *const libc::c_char) != 0 {
        /* IP v4 */
      /* address arrives as a name of the form
	 www.xxx.yyy.zzz.in-addr.arpa
	 some of the low order address octets might be missing
	 and should be set to zero. */
        cp1 = name.as_mut_ptr();
        while cp1 != penchunk {
            /* check for digits only (weeds out things like
	     50.0/24.67.28.64.in-addr.arpa which are used 
	     as CNAME targets according to RFC 2317 */
            let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
            cp = cp1;
            while *cp != 0 {
                if *(*__ctype_b_loc()).offset(*cp as libc::c_uchar as
                                                  libc::c_int as isize) as
                       libc::c_int &
                       _ISdigit as libc::c_int as libc::c_ushort as
                           libc::c_int == 0 {
                    return 0 as libc::c_int
                }
                cp = cp.offset(1)
            }
            *addr.offset(3 as libc::c_int as isize) =
                *addr.offset(2 as libc::c_int as isize);
            *addr.offset(2 as libc::c_int as isize) =
                *addr.offset(1 as libc::c_int as isize);
            *addr.offset(1 as libc::c_int as isize) =
                *addr.offset(0 as libc::c_int as isize);
            *addr.offset(0 as libc::c_int as isize) =
                atoi(cp1) as libc::c_uchar;
            cp1 =
                cp1.offset(strlen(cp1).wrapping_add(1 as libc::c_int as
                                                        libc::c_ulong) as
                               isize)
        }
        return ((1 as libc::c_uint) << 7 as libc::c_int) as libc::c_int
    } else {
        if hostname_isequal(penchunk,
                            b"ip6\x00" as *const u8 as *const libc::c_char) !=
               0 &&
               (hostname_isequal(lastchunk,
                                 b"int\x00" as *const u8 as
                                     *const libc::c_char) != 0 ||
                    hostname_isequal(lastchunk,
                                     b"arpa\x00" as *const u8 as
                                         *const libc::c_char) != 0) {
            /* IP v6:
         Address arrives as 0.1.2.3.4.5.6.7.8.9.a.b.c.d.e.f.ip6.[int|arpa]
    	 or \[xfedcba9876543210fedcba9876543210/128].ip6.[int|arpa]
      
	 Note that most of these the various representations are obsolete and 
	 left-over from the many DNS-for-IPv6 wars. We support all the formats
	 that we can since there is no reason not to.
      */
            if *name.as_mut_ptr() as libc::c_int == '\\' as i32 &&
                   *name.as_mut_ptr().offset(1 as libc::c_int as isize) as
                       libc::c_int == '[' as i32 &&
                   (*name.as_mut_ptr().offset(2 as libc::c_int as isize) as
                        libc::c_int == 'x' as i32 ||
                        *name.as_mut_ptr().offset(2 as libc::c_int as isize)
                            as libc::c_int == 'X' as i32) {
                j = 0 as libc::c_int;
                cp1 = name.as_mut_ptr().offset(3 as libc::c_int as isize);
                while *cp1 as libc::c_int != 0 &&
                          *(*__ctype_b_loc()).offset(*cp1 as libc::c_uchar as
                                                         libc::c_int as isize)
                              as libc::c_int &
                              _ISxdigit as libc::c_int as libc::c_ushort as
                                  libc::c_int != 0 && j < 32 as libc::c_int {
                    let mut xdig: [libc::c_char; 2] = [0; 2];
                    xdig[0 as libc::c_int as usize] = *cp1;
                    xdig[1 as libc::c_int as usize] =
                        0 as libc::c_int as libc::c_char;
                    if j % 2 as libc::c_int != 0 {
                        let ref mut fresh11 =
                            *addr.offset((j / 2 as libc::c_int) as isize);
                        *fresh11 =
                            (*fresh11 as libc::c_long |
                                 strtol(xdig.as_mut_ptr(),
                                        0 as *mut *mut libc::c_char,
                                        16 as libc::c_int)) as libc::c_uchar
                    } else {
                        *addr.offset((j / 2 as libc::c_int) as isize) =
                            (strtol(xdig.as_mut_ptr(),
                                    0 as *mut *mut libc::c_char,
                                    16 as libc::c_int) << 4 as libc::c_int) as
                                libc::c_uchar
                    }
                    cp1 = cp1.offset(1);
                    j += 1
                }
                if *cp1 as libc::c_int == '/' as i32 && j == 32 as libc::c_int
                   {
                    return ((1 as libc::c_uint) << 8 as libc::c_int) as
                               libc::c_int
                }
            } else {
                cp1 = name.as_mut_ptr();
                while cp1 != penchunk {
                    if *cp1.offset(1 as libc::c_int as isize) as libc::c_int
                           != 0 ||
                           *(*__ctype_b_loc()).offset(*cp1 as libc::c_uchar as
                                                          libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISxdigit as libc::c_int as libc::c_ushort as
                                   libc::c_int == 0 {
                        return 0 as libc::c_int
                    }
                    j =
                        (::std::mem::size_of::<in6_addr>() as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong) as
                            libc::c_int;
                    while j > 0 as libc::c_int {
                        *addr.offset(j as isize) =
                            (*addr.offset(j as isize) as libc::c_int >>
                                 4 as libc::c_int |
                                 (*addr.offset((j - 1 as libc::c_int) as
                                                   isize) as libc::c_int) <<
                                     4 as libc::c_int) as libc::c_uchar;
                        j -= 1
                    }
                    *addr.offset(0 as libc::c_int as isize) =
                        ((*addr.offset(0 as libc::c_int as isize) as
                              libc::c_int >> 4 as libc::c_int) as libc::c_long
                             |
                             strtol(cp1, 0 as *mut *mut libc::c_char,
                                    16 as libc::c_int) << 4 as libc::c_int) as
                            libc::c_uchar;
                    cp1 =
                        cp1.offset(strlen(cp1).wrapping_add(1 as libc::c_int
                                                                as
                                                                libc::c_ulong)
                                       as isize)
                }
                return ((1 as libc::c_uint) << 8 as libc::c_int) as
                           libc::c_int
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn skip_name(mut ansp: *mut libc::c_uchar,
                                   mut header: *mut dns_header,
                                   mut plen: size_t,
                                   mut extrabytes: libc::c_int)
 -> *mut libc::c_uchar {
    loop  {
        let mut label_type: libc::c_uint = 0;
        if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                  libc::c_long + 1 as libc::c_int as libc::c_long) as size_t
                 <= plen) {
            return 0 as *mut libc::c_uchar
        }
        label_type =
            (*ansp as libc::c_int & 0xc0 as libc::c_int) as libc::c_uint;
        if label_type == 0xc0 as libc::c_int as libc::c_uint {
            /* pointer for compression. */
            ansp = ansp.offset(2 as libc::c_int as isize); /* reserved */
            break ;
        } else if label_type == 0x80 as libc::c_int as libc::c_uint {
            return 0 as *mut libc::c_uchar
        } else if label_type == 0x40 as libc::c_int as libc::c_uint {
            /* Extended label type */
            let mut count: libc::c_uint =
                0; /* we only understand bitstrings */
            if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + 2 as libc::c_int as libc::c_long) as
                     size_t <= plen) {
                return 0 as *mut libc::c_uchar
            } /* Bits in bitstring */
            let fresh12 = ansp;
            ansp = ansp.offset(1);
            if *fresh12 as libc::c_int & 0x3f as libc::c_int !=
                   1 as libc::c_int {
                return 0 as *mut libc::c_uchar
            }
            let fresh13 = ansp;
            ansp = ansp.offset(1);
            count = *fresh13 as libc::c_uint;
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
            let fresh14 = ansp;
            ansp = ansp.offset(1);
            let mut len: libc::c_uint =
                (*fresh14 as libc::c_int & 0x3f as libc::c_int) as
                    libc::c_uint;
            if if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar)
                         as libc::c_long + len as libc::c_long) as size_t <=
                        plen) {
                   0 as libc::c_int
               } else { ansp = ansp.offset(len as isize); 1 as libc::c_int }
                   == 0 {
                return 0 as *mut libc::c_uchar
            }
            if len == 0 as libc::c_int as libc::c_uint { break ; }
            /* zero length label marks the end. */
        }
    }
    if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
              libc::c_long + extrabytes as libc::c_long) as size_t <= plen) {
        return 0 as *mut libc::c_uchar
    }
    return ansp;
}
#[no_mangle]
pub unsafe extern "C" fn skip_questions(mut header: *mut dns_header,
                                        mut plen: size_t)
 -> *mut libc::c_uchar {
    let mut q: libc::c_int = 0;
    let mut ansp: *mut libc::c_uchar =
        header.offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
    q = __bswap_16((*header).qdcount) as libc::c_int;
    while q != 0 as libc::c_int {
        ansp = skip_name(ansp, header, plen, 4 as libc::c_int);
        if ansp.is_null() { return 0 as *mut libc::c_uchar }
        ansp = ansp.offset(4 as libc::c_int as isize);
        q -= 1
        /* class and type */
    } /* type, class, TTL */
    return ansp;
}
#[no_mangle]
pub unsafe extern "C" fn skip_section(mut ansp: *mut libc::c_uchar,
                                      mut count: libc::c_int,
                                      mut header: *mut dns_header,
                                      mut plen: size_t)
 -> *mut libc::c_uchar {
    let mut i: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < count {
        ansp = skip_name(ansp, header, plen, 10 as libc::c_int);
        if ansp.is_null() { return 0 as *mut libc::c_uchar }
        ansp = ansp.offset(8 as libc::c_int as isize);
        let mut t_cp: *mut libc::c_uchar = ansp;
        rdlen =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        ansp = ansp.offset(2 as libc::c_int as isize);
        if if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                     libc::c_long + rdlen as libc::c_long) as size_t <= plen)
              {
               0 as libc::c_int
           } else { ansp = ansp.offset(rdlen as isize); 1 as libc::c_int } ==
               0 {
            return 0 as *mut libc::c_uchar
        }
        i += 1
    }
    return ansp;
}
#[no_mangle]
pub unsafe extern "C" fn resize_packet(mut header: *mut dns_header,
                                       mut plen: size_t,
                                       mut pheader: *mut libc::c_uchar,
                                       mut hlen: size_t) -> size_t {
    let mut ansp: *mut libc::c_uchar = skip_questions(header, plen);
    /* if packet is malformed, just return as-is. */
    if ansp.is_null() { return plen }
    ansp =
        skip_section(ansp,
                     __bswap_16((*header).ancount) as libc::c_int +
                         __bswap_16((*header).nscount) as libc::c_int +
                         __bswap_16((*header).arcount) as libc::c_int, header,
                     plen);
    if ansp.is_null() { return plen }
    /* restore pseudoheader */
    if !pheader.is_null() &&
           __bswap_16((*header).arcount) as libc::c_int == 0 as libc::c_int {
        /* must use memmove, may overlap */
        memmove(ansp as *mut libc::c_void, pheader as *const libc::c_void,
                hlen);
        (*header).arcount = __bswap_16(1 as libc::c_int as __uint16_t);
        ansp = ansp.offset(hlen as isize)
    }
    return ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
               libc::c_long as size_t;
}
/* is addr in the non-globally-routed IP space? */
#[no_mangle]
pub unsafe extern "C" fn private_net(mut addr: in_addr,
                                     mut ban_localhost: libc::c_int)
 -> libc::c_int {
    let mut ip_addr: in_addr_t = __bswap_32(addr.s_addr);
    return (ip_addr & 0xff000000 as libc::c_uint ==
                0x7f000000 as libc::c_int as libc::c_uint &&
                ban_localhost != 0 ||
                ip_addr & 0xff000000 as libc::c_uint ==
                    0 as libc::c_int as libc::c_uint ||
                ip_addr & 0xff000000 as libc::c_uint ==
                    0xa000000 as libc::c_int as libc::c_uint ||
                ip_addr & 0xfff00000 as libc::c_uint ==
                    0xac100000 as libc::c_uint ||
                ip_addr & 0xffff0000 as libc::c_uint ==
                    0xc0a80000 as libc::c_uint ||
                ip_addr & 0xffff0000 as libc::c_uint ==
                    0xa9fe0000 as libc::c_uint ||
                ip_addr & 0xffffff00 as libc::c_uint ==
                    0xc0000200 as libc::c_uint ||
                ip_addr & 0xffffff00 as libc::c_uint ==
                    0xc6336400 as libc::c_uint ||
                ip_addr & 0xffffff00 as libc::c_uint ==
                    0xcb007100 as libc::c_uint ||
                ip_addr & 0xffffffff as libc::c_uint ==
                    0xffffffff as libc::c_uint) as libc::c_int;
    /* 255.255.255.255/32 (broadcast)*/
}
unsafe extern "C" fn private_net6(mut a: *mut in6_addr) -> libc::c_int {
    return (({
                 let mut __a: *const in6_addr = a as *const in6_addr;
                 ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                      0 as libc::c_int as libc::c_uint &&
                      (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize] ==
                          0 as libc::c_int as libc::c_uint &&
                      (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize] ==
                          0 as libc::c_int as libc::c_uint &&
                      (*__a).__in6_u.__u6_addr32[3 as libc::c_int as usize] ==
                          0 as libc::c_int as libc::c_uint) as libc::c_int
             }) != 0 ||
                ({
                     let mut __a: *const in6_addr = a as *const in6_addr;
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
                              __bswap_32(1 as libc::c_int as __uint32_t)) as
                         libc::c_int
                 }) != 0 ||
                ({
                     let mut __a: *const in6_addr = a as *const in6_addr;
                     ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                          __bswap_32(0xffc00000 as libc::c_uint) ==
                          __bswap_32(0xfe800000 as libc::c_uint)) as
                         libc::c_int
                 }) != 0 ||
                *(a as *mut libc::c_uchar).offset(0 as libc::c_int as isize)
                    as libc::c_int == 0xfd as libc::c_int ||
                *(a as *mut u32_0).offset(0 as libc::c_int as isize) ==
                    __bswap_32(0x20010db8 as libc::c_int as __uint32_t)) as
               libc::c_int;
    /* RFC 6303 4.6 */
}
unsafe extern "C" fn do_doctor(mut p: *mut libc::c_uchar,
                               mut count: libc::c_int,
                               mut header: *mut dns_header, mut qlen: size_t,
                               mut name: *mut libc::c_char,
                               mut doctored: *mut libc::c_int)
 -> *mut libc::c_uchar {
    let mut i: libc::c_int = 0; /* bad packet */
    let mut qtype: libc::c_int = 0;
    let mut qclass: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    i = count;
    while i != 0 as libc::c_int {
        if !name.is_null() &&
               (*dnsmasq_daemon).options[(2 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (2 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
            if extract_name(header, qlen, &mut p, name, 1 as libc::c_int,
                            10 as libc::c_int) == 0 {
                return 0 as *mut libc::c_uchar
            }
        } else {
            p = skip_name(p, header, qlen, 10 as libc::c_int);
            if p.is_null() { return 0 as *mut libc::c_uchar }
        }
        let mut t_cp: *mut libc::c_uchar = p;
        qtype =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_0: *mut libc::c_uchar = p;
        qclass =
            (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        /* bad packet */
        p = p.offset(4 as libc::c_int as isize); /* ttl */
        let mut t_cp_1: *mut libc::c_uchar = p;
        rdlen =
            (*t_cp_1.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_1.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        if qclass == 1 as libc::c_int && qtype == 1 as libc::c_int {
            let mut doctor: *mut doctor = 0 as *mut doctor;
            let mut addr: in_addr = in_addr{s_addr: 0,};
            if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + 4 as libc::c_int as libc::c_long) as
                     size_t <= qlen) {
                return 0 as *mut libc::c_uchar
            }
            /* alignment */
            memcpy(&mut addr as *mut in_addr as *mut libc::c_void,
                   p as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
            let mut current_block_28: u64;
            doctor = (*dnsmasq_daemon).doctors;
            while !doctor.is_null() {
                if (*doctor).end.s_addr == 0 as libc::c_int as libc::c_uint {
                    if is_same_net((*doctor).in_0, addr, (*doctor).mask) == 0
                       {
                        current_block_28 = 6669252993407410313;
                    } else { current_block_28 = 11636175345244025579; }
                } else if __bswap_32((*doctor).in_0.s_addr) >
                              __bswap_32(addr.s_addr) ||
                              __bswap_32((*doctor).end.s_addr) <
                                  __bswap_32(addr.s_addr) {
                    current_block_28 = 6669252993407410313;
                } else { current_block_28 = 11636175345244025579; }
                match current_block_28 {
                    6669252993407410313 => { doctor = (*doctor).next }
                    _ => {
                        addr.s_addr &= !(*doctor).mask.s_addr;
                        addr.s_addr |=
                            (*doctor).out.s_addr & (*doctor).mask.s_addr;
                        /* Since we munged the data, the server it came from is no longer authoritative */
                        (*header).hb3 =
                            ((*header).hb3 as libc::c_int &
                                 !(0x4 as libc::c_int)) as
                                u8_0; /* bad packet */
                        *doctored = 1 as libc::c_int;
                        memcpy(p as *mut libc::c_void,
                               &mut addr as *mut in_addr as
                                   *const libc::c_void,
                               4 as libc::c_int as libc::c_ulong);
                        break ;
                    }
                }
            }
        } else if qtype == 16 as libc::c_int && !name.is_null() &&
                      (*dnsmasq_daemon).options[(2 as libc::c_int as
                                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_mul(8
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulong))
                                                    as usize] &
                          (1 as libc::c_uint) <<
                              (2 as libc::c_int as
                                   libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong))
                          != 0 {
            let mut p1: *mut libc::c_uchar = p;
            if !((p1.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + rdlen as libc::c_long) as size_t <= qlen)
               {
                return 0 as *mut libc::c_uchar
            }
            while (p1.wrapping_offset_from(p) as libc::c_long) <
                      rdlen as libc::c_long {
                let mut i_0: libc::c_uint = 0;
                let mut len: libc::c_uint = *p1 as libc::c_uint;
                let mut p2: *mut libc::c_uchar = p1;
                if p1.offset(len as isize).wrapping_offset_from(p) as
                       libc::c_long >= rdlen as libc::c_long {
                    return 0 as *mut libc::c_uchar
                }
                /* make counted string zero-term  and sanitise */
                i_0 = 0 as libc::c_int as libc::c_uint;
                while i_0 < len {
                    if *(*__ctype_b_loc()).offset(*p2.offset(1 as libc::c_int
                                                                 as isize) as
                                                      libc::c_int as isize) as
                           libc::c_int &
                           _ISprint as libc::c_int as libc::c_ushort as
                               libc::c_int == 0 {
                        break ;
                    }
                    *p2 = *p2.offset(1 as libc::c_int as isize);
                    p2 = p2.offset(1);
                    i_0 = i_0.wrapping_add(1)
                }
                *p2 = 0 as libc::c_int as libc::c_uchar;
                my_syslog(6 as libc::c_int,
                          b"reply %s is %s\x00" as *const u8 as
                              *const libc::c_char, name, p1);
                /* restore */
                memmove(p1.offset(1 as libc::c_int as isize) as
                            *mut libc::c_void, p1 as *const libc::c_void,
                        i_0 as libc::c_ulong);
                *p1 = len as libc::c_uchar;
                p1 =
                    p1.offset(len.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint) as isize)
            }
        }
        if if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                     libc::c_long + rdlen as libc::c_long) as size_t <= qlen)
              {
               0 as libc::c_int
           } else { p = p.offset(rdlen as isize); 1 as libc::c_int } == 0 {
            return 0 as *mut libc::c_uchar
        }
        i -= 1
    }
    return p;
}
unsafe extern "C" fn find_soa(mut header: *mut dns_header, mut qlen: size_t,
                              mut name: *mut libc::c_char,
                              mut doctored: *mut libc::c_int) -> libc::c_int {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut qtype: libc::c_int = 0;
    let mut qclass: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    let mut ttl: libc::c_ulong = 0;
    let mut minttl: libc::c_ulong =
        (9223372036854775807 as libc::c_long as
             libc::c_ulong).wrapping_mul(2 as
                                             libc::c_ulong).wrapping_add(1 as
                                                                             libc::c_ulong);
    let mut i: libc::c_int = 0;
    let mut found_soa: libc::c_int = 0 as libc::c_int;
    /* first move to NS section and find TTL from any SOA section */
    p = skip_questions(header, qlen); /* bad packet */
    if p.is_null() ||
           {
               p =
                   do_doctor(p, __bswap_16((*header).ancount) as libc::c_int,
                             header, qlen, name, doctored); /* bad packet */
               p.is_null()
           } {
        return 0 as libc::c_int
    }
    i = __bswap_16((*header).nscount) as libc::c_int;
    while i != 0 as libc::c_int {
        p = skip_name(p, header, qlen, 10 as libc::c_int);
        if p.is_null() { return 0 as libc::c_int }
        let mut t_cp: *mut libc::c_uchar = p;
        qtype =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_0: *mut libc::c_uchar = p;
        qclass =
            (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_1: *mut libc::c_uchar = p;
        ttl =
            ((*t_cp_1.offset(0 as libc::c_int as isize) as u32_0) <<
                 24 as libc::c_int |
                 (*t_cp_1.offset(1 as libc::c_int as isize) as u32_0) <<
                     16 as libc::c_int |
                 (*t_cp_1.offset(2 as libc::c_int as isize) as u32_0) <<
                     8 as libc::c_int |
                 *t_cp_1.offset(3 as libc::c_int as isize) as u32_0) as
                libc::c_ulong;
        p = p.offset(4 as libc::c_int as isize);
        let mut t_cp_2: *mut libc::c_uchar = p;
        rdlen =
            (*t_cp_2.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_2.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        if qclass == 1 as libc::c_int && qtype == 6 as libc::c_int {
            found_soa = 1 as libc::c_int;
            if ttl < minttl { minttl = ttl }
            /* bad packet */
            /* MNAME */
            p = skip_name(p, header, qlen, 0 as libc::c_int);
            if p.is_null() { return 0 as libc::c_int }
            /* RNAME */
            p =
                skip_name(p, header, qlen,
                          20 as
                              libc::c_int); /* SERIAL REFRESH RETRY EXPIRE */
            if p.is_null() { return 0 as libc::c_int }
            p = p.offset(16 as libc::c_int as isize);
            let mut t_cp_3: *mut libc::c_uchar = p;
            ttl =
                ((*t_cp_3.offset(0 as libc::c_int as isize) as u32_0) <<
                     24 as libc::c_int |
                     (*t_cp_3.offset(1 as libc::c_int as isize) as u32_0) <<
                         16 as libc::c_int |
                     (*t_cp_3.offset(2 as libc::c_int as isize) as u32_0) <<
                         8 as libc::c_int |
                     *t_cp_3.offset(3 as libc::c_int as isize) as u32_0) as
                    libc::c_ulong;
            p = p.offset(4 as libc::c_int as isize);
            /* minTTL */
            if ttl < minttl { minttl = ttl }
        } else if if !((p.wrapping_offset_from(header as *mut libc::c_uchar)
                            as libc::c_long + rdlen as libc::c_long) as size_t
                           <= qlen) {
                      0 as libc::c_int
                  } else { p = p.offset(rdlen as isize); 1 as libc::c_int } ==
                      0 {
            return 0 as libc::c_int
        }
        i -= 1
    }
    /* rewrite addresses in additional section too */
    if do_doctor(p, __bswap_16((*header).arcount) as libc::c_int, header,
                 qlen, 0 as *mut libc::c_char, doctored).is_null() {
        return 0 as libc::c_int
    }
    if found_soa == 0 { minttl = (*dnsmasq_daemon).neg_ttl }
    return minttl as libc::c_int;
}
/* Note that the following code can create CNAME chains that don't point to a real record,
   either because of lack of memory, or lack of SOA records.  These are treated by the cache code as 
   expired and cleaned out that way. 
   Return 1 if we reject an address because it look like part of dns-rebinding attack. */
#[no_mangle]
pub unsafe extern "C" fn extract_addresses(mut header: *mut dns_header,
                                           mut qlen: size_t,
                                           mut name: *mut libc::c_char,
                                           mut now: time_t,
                                           mut ipsets: *mut *mut libc::c_char,
                                           mut is_sign: libc::c_int,
                                           mut check_rebind: libc::c_int,
                                           mut no_cache_dnssec: libc::c_int,
                                           mut secure: libc::c_int,
                                           mut doctored: *mut libc::c_int)
 -> libc::c_int {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut endrr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut namep: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut qtype: libc::c_int = 0;
    let mut qclass: libc::c_int = 0;
    let mut aqtype: libc::c_int = 0;
    let mut aqclass: libc::c_int = 0;
    let mut ardlen: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut searched_soa: libc::c_int = 0 as libc::c_int;
    let mut ttl: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut addr: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut ipsets_cur: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    cache_start_insert();
    /* find_soa is needed for dns_doctor and logging side-effects, so don't call it lazily if there are any. */
    if !(*dnsmasq_daemon).doctors.is_null() ||
           (*dnsmasq_daemon).options[(2 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (2 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 ||
           (*dnsmasq_daemon).options[(45 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (45 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        searched_soa = 1 as libc::c_int;
        ttl = find_soa(header, qlen, name, doctored) as libc::c_ulong;
        if *doctored != 0 { if secure != 0 { return 0 as libc::c_int } }
    }
    /* go through the questions. */
    p =
        header.offset(1 as libc::c_int as isize) as
            *mut libc::c_uchar; /* bad packet */
    let mut current_block_206: u64;
    i = __bswap_16((*header).qdcount) as libc::c_int;
    while i != 0 as libc::c_int {
        let mut found: libc::c_int = 0 as libc::c_int;
        let mut cname_count: libc::c_int = 10 as libc::c_int;
        let mut cpp: *mut crec = 0 as *mut crec;
        let mut flags: libc::c_int =
            if (*header).hb4 as libc::c_int & 0xf as libc::c_int ==
                   3 as libc::c_int {
                ((1 as libc::c_uint)) << 10 as libc::c_int
            } else { 0 as libc::c_int as libc::c_uint } as libc::c_int;
        let mut cttl: libc::c_ulong =
            (9223372036854775807 as libc::c_long as
                 libc::c_ulong).wrapping_mul(2 as
                                                 libc::c_ulong).wrapping_add(1
                                                                                 as
                                                                                 libc::c_ulong);
        let mut attl: libc::c_ulong = 0;
        namep = p;
        if extract_name(header, qlen, &mut p, name, 1 as libc::c_int,
                        4 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
        let mut t_cp: *mut libc::c_uchar = p;
        qtype =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_0: *mut libc::c_uchar = p;
        qclass =
            (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        if !(qclass != 1 as libc::c_int) {
            /* PTRs: we chase CNAMEs here, since we have no way to 
	 represent them in the cache. */
            if qtype == 12 as libc::c_int {
                let mut name_encoding: libc::c_int =
                    in_arpa_name_2_addr(name, &mut addr);
                if !(name_encoding == 0) {
                    if flags as libc::c_uint &
                           (1 as libc::c_uint) << 10 as libc::c_int == 0 {
                        'c_14031:
                            loop  {
                                p1 = skip_questions(header, qlen);
                                if p1.is_null() { return 0 as libc::c_int }
                                j = 0 as libc::c_int;
                                loop  {
                                    if !(j <
                                             __bswap_16((*header).ancount) as
                                                 libc::c_int) {
                                        break 'c_14031 ;
                                    }
                                    let mut secflag: libc::c_int =
                                        0 as libc::c_int;
                                    let mut tmp: *mut libc::c_uchar = namep;
                                    /* bad packet */
                                    /* the loop body overwrites the original name, so get it back here. */
                                    if extract_name(header, qlen, &mut tmp,
                                                    name, 1 as libc::c_int,
                                                    0 as libc::c_int) == 0 ||
                                           {
                                               res =
                                                   extract_name(header, qlen,
                                                                &mut p1, name,
                                                                0 as
                                                                    libc::c_int,
                                                                10 as
                                                                    libc::c_int); /* bad packet */
                                               (res) == 0
                                           } {
                                        return 0 as libc::c_int
                                    }
                                    let mut t_cp_1: *mut libc::c_uchar = p1;
                                    aqtype =
                                        (*t_cp_1.offset(0 as libc::c_int as
                                                            isize) as u16_0 as
                                             libc::c_int) << 8 as libc::c_int
                                            |
                                            *t_cp_1.offset(1 as libc::c_int as
                                                               isize) as u16_0
                                                as libc::c_int;
                                    p1 = p1.offset(2 as libc::c_int as isize);
                                    let mut t_cp_2: *mut libc::c_uchar = p1;
                                    aqclass =
                                        (*t_cp_2.offset(0 as libc::c_int as
                                                            isize) as u16_0 as
                                             libc::c_int) << 8 as libc::c_int
                                            |
                                            *t_cp_2.offset(1 as libc::c_int as
                                                               isize) as u16_0
                                                as libc::c_int;
                                    p1 = p1.offset(2 as libc::c_int as isize);
                                    let mut t_cp_3: *mut libc::c_uchar = p1;
                                    attl =
                                        ((*t_cp_3.offset(0 as libc::c_int as
                                                             isize) as u32_0)
                                             << 24 as libc::c_int |
                                             (*t_cp_3.offset(1 as libc::c_int
                                                                 as isize) as
                                                  u32_0) << 16 as libc::c_int
                                             |
                                             (*t_cp_3.offset(2 as libc::c_int
                                                                 as isize) as
                                                  u32_0) << 8 as libc::c_int |
                                             *t_cp_3.offset(3 as libc::c_int
                                                                as isize) as
                                                 u32_0) as libc::c_ulong;
                                    p1 = p1.offset(4 as libc::c_int as isize);
                                    if (*dnsmasq_daemon).max_ttl !=
                                           0 as libc::c_int as libc::c_ulong
                                           && attl > (*dnsmasq_daemon).max_ttl
                                           && is_sign == 0 {
                                        p1 =
                                            p1.offset(-(4 as libc::c_int as
                                                            isize));
                                        let mut t_l: u32_0 =
                                            (*dnsmasq_daemon).max_ttl as
                                                u32_0;
                                        let mut t_cp_4: *mut libc::c_uchar =
                                            p1;
                                        let fresh15 = t_cp_4;
                                        t_cp_4 = t_cp_4.offset(1);
                                        *fresh15 =
                                            (t_l >> 24 as libc::c_int) as
                                                libc::c_uchar;
                                        let fresh16 = t_cp_4;
                                        t_cp_4 = t_cp_4.offset(1);
                                        *fresh16 =
                                            (t_l >> 16 as libc::c_int) as
                                                libc::c_uchar;
                                        let fresh17 = t_cp_4;
                                        t_cp_4 = t_cp_4.offset(1);
                                        *fresh17 =
                                            (t_l >> 8 as libc::c_int) as
                                                libc::c_uchar;
                                        *t_cp_4 = t_l as libc::c_uchar;
                                        p1 =
                                            p1.offset(4 as libc::c_int as
                                                          isize)
                                    }
                                    let mut t_cp_5: *mut libc::c_uchar = p1;
                                    ardlen =
                                        (*t_cp_5.offset(0 as libc::c_int as
                                                            isize) as u16_0 as
                                             libc::c_int) << 8 as libc::c_int
                                            |
                                            *t_cp_5.offset(1 as libc::c_int as
                                                               isize) as u16_0
                                                as libc::c_int;
                                    p1 = p1.offset(2 as libc::c_int as isize);
                                    endrr = p1.offset(ardlen as isize);
                                    /* TTL of record is minimum of CNAMES and PTR */
                                    if attl < cttl {
                                        cttl = attl
                                    } /* looped CNAMES, we can't cache. */
                                    if aqclass == 1 as libc::c_int &&
                                           res != 2 as libc::c_int &&
                                           (aqtype == 5 as libc::c_int ||
                                                aqtype == 12 as libc::c_int) {
                                        if extract_name(header, qlen, &mut p1,
                                                        name,
                                                        1 as libc::c_int,
                                                        0 as libc::c_int) == 0
                                           {
                                            return 0 as libc::c_int
                                        }
                                        if aqtype == 5 as libc::c_int {
                                            let fresh18 = cname_count;
                                            cname_count = cname_count - 1;
                                            if fresh18 == 0 {
                                                return 0 as libc::c_int
                                            }
                                            break ;
                                        } else {
                                            cache_insert(name, &mut addr,
                                                         1 as libc::c_int as
                                                             libc::c_ushort,
                                                         now, cttl,
                                                         (name_encoding |
                                                              secflag) as
                                                             libc::c_uint |
                                                             (1 as
                                                                  libc::c_uint)
                                                                 <<
                                                                 2 as
                                                                     libc::c_int);
                                            found = 1 as libc::c_int
                                        }
                                    }
                                    p1 = endrr;
                                    if !((p1.wrapping_offset_from(header as
                                                                      *mut libc::c_uchar)
                                              as libc::c_long +
                                              0 as libc::c_int as
                                                  libc::c_long) as size_t <=
                                             qlen) {
                                        return 0 as libc::c_int
                                    }
                                    j += 1
                                }
                            }
                    }
                    if found == 0 &&
                           (*dnsmasq_daemon).options[(11 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (11 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               == 0 {
                        if searched_soa == 0 {
                            searched_soa = 1 as libc::c_int;
                            ttl =
                                find_soa(header, qlen, 0 as *mut libc::c_char,
                                         doctored) as libc::c_ulong
                        }
                        if ttl != 0 {
                            cache_insert(0 as *mut libc::c_char, &mut addr,
                                         1 as libc::c_int as libc::c_ushort,
                                         now, ttl,
                                         name_encoding as libc::c_uint |
                                             (1 as libc::c_uint) <<
                                                 2 as libc::c_int |
                                             (1 as libc::c_uint) <<
                                                 5 as libc::c_int |
                                             flags as libc::c_uint |
                                             (if secure != 0 {
                                                  ((1 as libc::c_uint)) <<
                                                      15 as libc::c_int
                                              } else {
                                                  0 as libc::c_int as
                                                      libc::c_uint
                                              }));
                        }
                    }
                }
            } else {
                /* everything other than PTR */
                let mut newc: *mut crec = 0 as *mut crec;
                let mut addrlen: libc::c_int = 0 as libc::c_int;
                if qtype == 1 as libc::c_int {
                    addrlen = 4 as libc::c_int;
                    flags =
                        (flags as libc::c_uint |
                             (1 as libc::c_uint) << 7 as libc::c_int) as
                            libc::c_int;
                    current_block_206 = 18356193971123529525;
                } else if qtype == 28 as libc::c_int {
                    addrlen = 16 as libc::c_int;
                    flags =
                        (flags as libc::c_uint |
                             (1 as libc::c_uint) << 8 as libc::c_int) as
                            libc::c_int;
                    current_block_206 = 18356193971123529525;
                } else if qtype == 33 as libc::c_int {
                    flags =
                        (flags as libc::c_uint |
                             (1 as libc::c_uint) << 30 as libc::c_int) as
                            libc::c_int;
                    current_block_206 = 18356193971123529525;
                } else { current_block_206 = 1856101646708284338; }
                match current_block_206 {
                    1856101646708284338 => { }
                    _ => {
                        'c_10467:
                            loop  {
                                p1 = skip_questions(header, qlen);
                                if p1.is_null() { return 0 as libc::c_int }
                                j = 0 as libc::c_int;
                                loop  {
                                    if !(j <
                                             __bswap_16((*header).ancount) as
                                                 libc::c_int) {
                                        break 'c_10467 ;
                                    }
                                    let mut secflag_0: libc::c_int =
                                        0 as libc::c_int;
                                    /* bad packet */
                                    res =
                                        extract_name(header, qlen, &mut p1,
                                                     name, 0 as libc::c_int,
                                                     10 as
                                                         libc::c_int); /* bad packet */
                                    if res == 0 {
                                        return 0 as libc::c_int
                                    } /* looped CNAMES */
                                    let mut t_cp_6: *mut libc::c_uchar =
                                        p1; /* bad packet */
                                    aqtype =
                                        (*t_cp_6.offset(0 as libc::c_int as
                                                            isize) as u16_0 as
                                             libc::c_int) << 8 as libc::c_int
                                            |
                                            *t_cp_6.offset(1 as libc::c_int as
                                                               isize) as u16_0
                                                as
                                                libc::c_int; /* include terminating zero */
                                    p1 = p1.offset(2 as libc::c_int as isize);
                                    let mut t_cp_7: *mut libc::c_uchar = p1;
                                    aqclass =
                                        (*t_cp_7.offset(0 as libc::c_int as
                                                            isize) as u16_0 as
                                             libc::c_int) << 8 as libc::c_int
                                            |
                                            *t_cp_7.offset(1 as libc::c_int as
                                                               isize) as u16_0
                                                as libc::c_int;
                                    p1 = p1.offset(2 as libc::c_int as isize);
                                    let mut t_cp_8: *mut libc::c_uchar = p1;
                                    attl =
                                        ((*t_cp_8.offset(0 as libc::c_int as
                                                             isize) as u32_0)
                                             << 24 as libc::c_int |
                                             (*t_cp_8.offset(1 as libc::c_int
                                                                 as isize) as
                                                  u32_0) << 16 as libc::c_int
                                             |
                                             (*t_cp_8.offset(2 as libc::c_int
                                                                 as isize) as
                                                  u32_0) << 8 as libc::c_int |
                                             *t_cp_8.offset(3 as libc::c_int
                                                                as isize) as
                                                 u32_0) as libc::c_ulong;
                                    p1 = p1.offset(4 as libc::c_int as isize);
                                    if (*dnsmasq_daemon).max_ttl !=
                                           0 as libc::c_int as libc::c_ulong
                                           && attl > (*dnsmasq_daemon).max_ttl
                                           && is_sign == 0 {
                                        p1 =
                                            p1.offset(-(4 as libc::c_int as
                                                            isize));
                                        let mut t_l_0: u32_0 =
                                            (*dnsmasq_daemon).max_ttl as
                                                u32_0;
                                        let mut t_cp_9: *mut libc::c_uchar =
                                            p1;
                                        let fresh19 = t_cp_9;
                                        t_cp_9 = t_cp_9.offset(1);
                                        *fresh19 =
                                            (t_l_0 >> 24 as libc::c_int) as
                                                libc::c_uchar;
                                        let fresh20 = t_cp_9;
                                        t_cp_9 = t_cp_9.offset(1);
                                        *fresh20 =
                                            (t_l_0 >> 16 as libc::c_int) as
                                                libc::c_uchar;
                                        let fresh21 = t_cp_9;
                                        t_cp_9 = t_cp_9.offset(1);
                                        *fresh21 =
                                            (t_l_0 >> 8 as libc::c_int) as
                                                libc::c_uchar;
                                        *t_cp_9 = t_l_0 as libc::c_uchar;
                                        p1 =
                                            p1.offset(4 as libc::c_int as
                                                          isize)
                                    }
                                    let mut t_cp_10: *mut libc::c_uchar = p1;
                                    ardlen =
                                        (*t_cp_10.offset(0 as libc::c_int as
                                                             isize) as u16_0
                                             as libc::c_int) <<
                                            8 as libc::c_int |
                                            *t_cp_10.offset(1 as libc::c_int
                                                                as isize) as
                                                u16_0 as libc::c_int;
                                    p1 = p1.offset(2 as libc::c_int as isize);
                                    endrr = p1.offset(ardlen as isize);
                                    if aqclass == 1 as libc::c_int &&
                                           res != 2 as libc::c_int &&
                                           (aqtype == 5 as libc::c_int ||
                                                aqtype == qtype) {
                                        if aqtype == 5 as libc::c_int {
                                            let fresh22 = cname_count;
                                            cname_count = cname_count - 1;
                                            if fresh22 == 0 {
                                                return 0 as libc::c_int
                                            }
                                            newc =
                                                cache_insert(name,
                                                             0 as
                                                                 *mut all_addr,
                                                             1 as libc::c_int
                                                                 as
                                                                 libc::c_ushort,
                                                             now, attl,
                                                             (1 as
                                                                  libc::c_uint)
                                                                 <<
                                                                 11 as
                                                                     libc::c_int
                                                                 |
                                                                 (1 as
                                                                      libc::c_uint)
                                                                     <<
                                                                     3 as
                                                                         libc::c_int
                                                                 |
                                                                 secflag_0 as
                                                                     libc::c_uint);
                                            if !newc.is_null() {
                                                (*newc).addr.cname.target.cache
                                                    = 0 as *mut crec;
                                                (*newc).addr.cname.is_name_ptr
                                                    = 0 as libc::c_int;
                                                if !cpp.is_null() {
                                                    next_uid(newc);
                                                    (*cpp).addr.cname.target.cache
                                                        = newc;
                                                    (*cpp).addr.cname.uid =
                                                        (*newc).uid
                                                }
                                            }
                                            cpp = newc;
                                            if attl < cttl { cttl = attl }
                                            namep = p1;
                                            if extract_name(header, qlen,
                                                            &mut p1, name,
                                                            1 as libc::c_int,
                                                            0 as libc::c_int)
                                                   == 0 {
                                                return 0 as libc::c_int
                                            }
                                            break ;
                                        } else if flags as libc::c_uint &
                                                      (1 as libc::c_uint) <<
                                                          10 as libc::c_int ==
                                                      0 {
                                            found = 1 as libc::c_int;
                                            if flags as libc::c_uint &
                                                   (1 as libc::c_uint) <<
                                                       30 as libc::c_int != 0
                                               {
                                                let mut tmp_0:
                                                        *mut libc::c_uchar =
                                                    namep;
                                                if !((p1.wrapping_offset_from(header
                                                                                  as
                                                                                  *mut libc::c_uchar)
                                                          as libc::c_long +
                                                          6 as libc::c_int as
                                                              libc::c_long) as
                                                         size_t <= qlen) {
                                                    return 0 as libc::c_int
                                                }
                                                let mut t_cp_11:
                                                        *mut libc::c_uchar =
                                                    p1;
                                                addr.srv.priority =
                                                    ((*t_cp_11.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                                                          as u16_0 as
                                                          libc::c_int) <<
                                                         8 as libc::c_int |
                                                         *t_cp_11.offset(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                                             as u16_0 as
                                                             libc::c_int) as
                                                        libc::c_ushort;
                                                p1 =
                                                    p1.offset(2 as libc::c_int
                                                                  as isize);
                                                let mut t_cp_12:
                                                        *mut libc::c_uchar =
                                                    p1;
                                                addr.srv.weight =
                                                    ((*t_cp_12.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                                                          as u16_0 as
                                                          libc::c_int) <<
                                                         8 as libc::c_int |
                                                         *t_cp_12.offset(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                                             as u16_0 as
                                                             libc::c_int) as
                                                        libc::c_ushort;
                                                p1 =
                                                    p1.offset(2 as libc::c_int
                                                                  as isize);
                                                let mut t_cp_13:
                                                        *mut libc::c_uchar =
                                                    p1;
                                                addr.srv.srvport =
                                                    ((*t_cp_13.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                                                          as u16_0 as
                                                          libc::c_int) <<
                                                         8 as libc::c_int |
                                                         *t_cp_13.offset(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                                                             as u16_0 as
                                                             libc::c_int) as
                                                        libc::c_ushort;
                                                p1 =
                                                    p1.offset(2 as libc::c_int
                                                                  as isize);
                                                if extract_name(header, qlen,
                                                                &mut p1, name,
                                                                1 as
                                                                    libc::c_int,
                                                                0 as
                                                                    libc::c_int)
                                                       == 0 {
                                                    return 0 as libc::c_int
                                                }
                                                addr.srv.targetlen =
                                                    strlen(name).wrapping_add(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong)
                                                        as libc::c_ushort;
                                                addr.srv.target =
                                                    blockdata_alloc(name,
                                                                    addr.srv.targetlen
                                                                        as
                                                                        size_t);
                                                if addr.srv.target.is_null() {
                                                    return 0 as libc::c_int
                                                }
                                                /* we overwrote the original name, so get it back here. */
                                                if extract_name(header, qlen,
                                                                &mut tmp_0,
                                                                name,
                                                                1 as
                                                                    libc::c_int,
                                                                0 as
                                                                    libc::c_int)
                                                       == 0 {
                                                    return 0 as libc::c_int
                                                }
                                            } else {
                                                /* copy address into aligned storage */
                                                if !((p1.wrapping_offset_from(header
                                                                                  as
                                                                                  *mut libc::c_uchar)
                                                          as libc::c_long +
                                                          addrlen as
                                                              libc::c_long) as
                                                         size_t <= qlen) {
                                                    return 0 as libc::c_int
                                                } /* bad packet */
                                                memcpy(&mut addr as
                                                           *mut all_addr as
                                                           *mut libc::c_void,
                                                       p1 as
                                                           *const libc::c_void,
                                                       addrlen as
                                                           libc::c_ulong);
                                                /* check for returned address in private space */
                                                if check_rebind != 0 {
                                                    if flags as libc::c_uint &
                                                           (1 as libc::c_uint)
                                                               <<
                                                               7 as
                                                                   libc::c_int
                                                           != 0 &&
                                                           private_net(addr.addr4,
                                                                       ((*dnsmasq_daemon).options[(25
                                                                                                       as
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
                                                                            (1
                                                                                 as
                                                                                 libc::c_uint)
                                                                                <<
                                                                                (25
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                                      as
                                                                                                                      libc::c_ulong).wrapping_mul(8
                                                                                                                                                      as
                                                                                                                                                      libc::c_int
                                                                                                                                                      as
                                                                                                                                                      libc::c_ulong))
                                                                            ==
                                                                            0)
                                                                           as
                                                                           libc::c_int)
                                                               != 0 {
                                                        return 1 as
                                                                   libc::c_int
                                                    }
                                                    /* Block IPv4-mapped IPv6 addresses in private IPv4 address space */
                                                    if flags as libc::c_uint &
                                                           (1 as libc::c_uint)
                                                               <<
                                                               8 as
                                                                   libc::c_int
                                                           != 0 {
                                                        if ({
                                                                let mut __a:
                                                                        *const in6_addr =
                                                                    &mut addr.addr6
                                                                        as
                                                                        *mut in6_addr
                                                                        as
                                                                        *const in6_addr;
                                                                ((*__a).__in6_u.__u6_addr32[0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                usize]
                                                                     ==
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint
                                                                     &&
                                                                     (*__a).__in6_u.__u6_addr32[1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    usize]
                                                                         ==
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint
                                                                     &&
                                                                     (*__a).__in6_u.__u6_addr32[2
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    usize]
                                                                         ==
                                                                         __bswap_32(0xffff
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        __uint32_t))
                                                                    as
                                                                    libc::c_int
                                                            }) != 0 {
                                                            let mut v4:
                                                                    in_addr =
                                                                in_addr{s_addr:
                                                                            0,};
                                                            v4.s_addr =
                                                                *(&mut addr.addr6
                                                                      as
                                                                      *mut in6_addr
                                                                      as
                                                                      *const uint32_t).offset(3
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  isize);
                                                            if private_net(v4,
                                                                           ((*dnsmasq_daemon).options[(25
                                                                                                           as
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
                                                                                (1
                                                                                     as
                                                                                     libc::c_uint)
                                                                                    <<
                                                                                    (25
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                                          as
                                                                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                                                                          as
                                                                                                                                                          libc::c_int
                                                                                                                                                          as
                                                                                                                                                          libc::c_ulong))
                                                                                ==
                                                                                0)
                                                                               as
                                                                               libc::c_int)
                                                                   != 0 {
                                                                return 1 as
                                                                           libc::c_int
                                                            }
                                                        }
                                                        /* Check for link-local (LL) and site-local (ULA) IPv6 addresses */
                                                        if ({
                                                                let mut __a:
                                                                        *const in6_addr =
                                                                    &mut addr.addr6
                                                                        as
                                                                        *mut in6_addr
                                                                        as
                                                                        *const in6_addr;
                                                                ((*__a).__in6_u.__u6_addr32[0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                usize]
                                                                     &
                                                                     __bswap_32(0xffc00000
                                                                                    as
                                                                                    libc::c_uint)
                                                                     ==
                                                                     __bswap_32(0xfe800000
                                                                                    as
                                                                                    libc::c_uint))
                                                                    as
                                                                    libc::c_int
                                                            }) != 0 ||
                                                               ({
                                                                    let mut __a:
                                                                            *const in6_addr =
                                                                        &mut addr.addr6
                                                                            as
                                                                            *mut in6_addr
                                                                            as
                                                                            *const in6_addr;
                                                                    ((*__a).__in6_u.__u6_addr32[0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    usize]
                                                                         &
                                                                         __bswap_32(0xffc00000
                                                                                        as
                                                                                        libc::c_uint)
                                                                         ==
                                                                         __bswap_32(0xfec00000
                                                                                        as
                                                                                        libc::c_uint))
                                                                        as
                                                                        libc::c_int
                                                                }) != 0 {
                                                            return 1 as
                                                                       libc::c_int
                                                        }
                                                        /* Check for the IPv6 loopback address (::1) when
				     option rebind-localhost-ok is NOT set */
                                                        if (*dnsmasq_daemon).options[(25
                                                                                          as
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
                                                               (1 as
                                                                    libc::c_uint)
                                                                   <<
                                                                   (25 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                         as
                                                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         as
                                                                                                                                         libc::c_ulong))
                                                               == 0 &&
                                                               ({
                                                                    let mut __a:
                                                                            *const in6_addr =
                                                                        &mut addr.addr6
                                                                            as
                                                                            *mut in6_addr
                                                                            as
                                                                            *const in6_addr;
                                                                    ((*__a).__in6_u.__u6_addr32[0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    usize]
                                                                         ==
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint
                                                                         &&
                                                                         (*__a).__in6_u.__u6_addr32[1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        usize]
                                                                             ==
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint
                                                                         &&
                                                                         (*__a).__in6_u.__u6_addr32[2
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        usize]
                                                                             ==
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint
                                                                         &&
                                                                         (*__a).__in6_u.__u6_addr32[3
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        usize]
                                                                             ==
                                                                             __bswap_32(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            __uint32_t))
                                                                        as
                                                                        libc::c_int
                                                                }) != 0 {
                                                            return 1 as
                                                                       libc::c_int
                                                        }
                                                    }
                                                }
                                                if !ipsets.is_null() &&
                                                       flags as libc::c_uint &
                                                           ((1 as
                                                                 libc::c_uint)
                                                                <<
                                                                7 as
                                                                    libc::c_int
                                                                |
                                                                (1 as
                                                                     libc::c_uint)
                                                                    <<
                                                                    8 as
                                                                        libc::c_int)
                                                           != 0 {
                                                    ipsets_cur = ipsets;
                                                    while !(*ipsets_cur).is_null()
                                                          {
                                                        log_query(flags as
                                                                      libc::c_uint
                                                                      &
                                                                      ((1 as
                                                                            libc::c_uint)
                                                                           <<
                                                                           7
                                                                               as
                                                                               libc::c_int
                                                                           |
                                                                           (1
                                                                                as
                                                                                libc::c_uint)
                                                                               <<
                                                                               8
                                                                                   as
                                                                                   libc::c_int)
                                                                      |
                                                                      (1 as
                                                                           libc::c_uint)
                                                                          <<
                                                                          26
                                                                              as
                                                                              libc::c_int,
                                                                  name,
                                                                  &mut addr,
                                                                  *ipsets_cur);
                                                        let fresh23 =
                                                            ipsets_cur;
                                                        ipsets_cur =
                                                            ipsets_cur.offset(1);
                                                        add_to_ipset(*fresh23,
                                                                     &mut addr,
                                                                     flags,
                                                                     0 as
                                                                         libc::c_int);
                                                    }
                                                }
                                            }
                                            newc =
                                                cache_insert(name, &mut addr,
                                                             1 as libc::c_int
                                                                 as
                                                                 libc::c_ushort,
                                                             now, attl,
                                                             flags as
                                                                 libc::c_uint
                                                                 |
                                                                 (1 as
                                                                      libc::c_uint)
                                                                     <<
                                                                     3 as
                                                                         libc::c_int
                                                                 |
                                                                 secflag_0 as
                                                                     libc::c_uint);
                                            if !newc.is_null() &&
                                                   !cpp.is_null() {
                                                next_uid(newc);
                                                (*cpp).addr.cname.target.cache
                                                    = newc;
                                                (*cpp).addr.cname.uid =
                                                    (*newc).uid
                                            }
                                            cpp = 0 as *mut crec
                                        }
                                    }
                                    p1 = endrr;
                                    if !((p1.wrapping_offset_from(header as
                                                                      *mut libc::c_uchar)
                                              as libc::c_long +
                                              0 as libc::c_int as
                                                  libc::c_long) as size_t <=
                                             qlen) {
                                        return 0 as libc::c_int
                                    }
                                    j += 1
                                }
                            }
                        if found == 0 &&
                               (*dnsmasq_daemon).options[(11 as libc::c_int as
                                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                               as
                                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_ulong))
                                                             as usize] &
                                   (1 as libc::c_uint) <<
                                       (11 as libc::c_int as
                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong))
                                   == 0 {
                            if searched_soa == 0 {
                                searched_soa = 1 as libc::c_int;
                                ttl =
                                    find_soa(header, qlen,
                                             0 as *mut libc::c_char, doctored)
                                        as libc::c_ulong
                            }
                            /* If there's no SOA to get the TTL from, but there is a CNAME 
		 pointing at this, inherit its TTL */
                            if ttl != 0 || !cpp.is_null() {
                                newc =
                                    cache_insert(name, 0 as *mut all_addr,
                                                 1 as libc::c_int as
                                                     libc::c_ushort, now,
                                                 if ttl != 0 {
                                                     ttl
                                                 } else { cttl },
                                                 (1 as libc::c_uint) <<
                                                     3 as libc::c_int |
                                                     (1 as libc::c_uint) <<
                                                         5 as libc::c_int |
                                                     flags as libc::c_uint |
                                                     (if secure != 0 {
                                                          ((1 as
                                                                libc::c_uint))
                                                              <<
                                                              15 as
                                                                  libc::c_int
                                                      } else {
                                                          0 as libc::c_int as
                                                              libc::c_uint
                                                      }));
                                if !newc.is_null() && !cpp.is_null() {
                                    next_uid(newc);
                                    (*cpp).addr.cname.target.cache = newc;
                                    (*cpp).addr.cname.uid = (*newc).uid
                                }
                            }
                        }
                    }
                }
            }
        }
        i -= 1
    }
    /* Don't put stuff from a truncated packet into the cache.
     Don't cache replies from non-recursive nameservers, since we may get a 
     reply containing a CNAME but not its target, even though the target 
     does exist. */
    if (*header).hb3 as libc::c_int & 0x2 as libc::c_int == 0 &&
           (*header).hb4 as libc::c_int & 0x10 as libc::c_int == 0 &&
           (*header).hb4 as libc::c_int & 0x80 as libc::c_int != 0 &&
           no_cache_dnssec == 0 {
        cache_end_insert();
    }
    return 0 as libc::c_int;
}
/* If the packet holds exactly one query
   return F_IPV4 or F_IPV6  and leave the name from the query in name */
#[no_mangle]
pub unsafe extern "C" fn extract_request(mut header: *mut dns_header,
                                         mut qlen: size_t,
                                         mut name: *mut libc::c_char,
                                         mut typep: *mut libc::c_ushort)
 -> libc::c_uint {
    let mut p: *mut libc::c_uchar =
        header.offset(1 as libc::c_int as isize) as
            *mut libc::c_uchar; /* must be exactly one query. */
    let mut qtype: libc::c_int = 0; /* bad packet */
    let mut qclass: libc::c_int = 0;
    if !typep.is_null() { *typep = 0 as libc::c_int as libc::c_ushort }
    if __bswap_16((*header).qdcount) as libc::c_int != 1 as libc::c_int ||
           ((*header).hb3 as libc::c_int & 0x78 as libc::c_int) >>
               3 as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint
    }
    if extract_name(header, qlen, &mut p, name, 1 as libc::c_int,
                    4 as libc::c_int) == 0 {
        return 0 as libc::c_int as libc::c_uint
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
    if !typep.is_null() { *typep = qtype as libc::c_ushort }
    if qclass == 1 as libc::c_int {
        if qtype == 1 as libc::c_int {
            return (1 as libc::c_uint) << 7 as libc::c_int
        }
        if qtype == 28 as libc::c_int {
            return (1 as libc::c_uint) << 8 as libc::c_int
        }
        if qtype == 255 as libc::c_int {
            return (1 as libc::c_uint) << 7 as libc::c_int |
                       (1 as libc::c_uint) << 8 as libc::c_int
        }
    }
    /* F_DNSSECOK as agument to search_servers() inhibits forwarding
     to servers for domains without a trust anchor. This make the
     behaviour for DS and DNSKEY queries we forward the same
     as for DS and DNSKEY queries we originate. */
    if qtype == 43 as libc::c_int || qtype == 48 as libc::c_int {
        return (1 as libc::c_uint) << 15 as libc::c_int
    }
    return (1 as libc::c_uint) << 19 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setup_reply(mut header: *mut dns_header,
                                     mut qlen: size_t,
                                     mut addrp: *mut all_addr,
                                     mut flags: libc::c_uint,
                                     mut ttl: libc::c_ulong) -> size_t {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    p = skip_questions(header, qlen);
    if p.is_null() { return 0 as libc::c_int as size_t }
    /* clear authoritative and truncated flags, set QR flag */
    (*header).hb3 =
        ((*header).hb3 as libc::c_int &
             !(0x4 as libc::c_int | 0x2 as libc::c_int) | 0x80 as libc::c_int)
            as u8_0;
    /* clear AD flag, set RA flag */
    (*header).hb4 =
        ((*header).hb4 as libc::c_int & !(0x20 as libc::c_int) |
             0x80 as libc::c_int) as
            u8_0; /* no answers unless changed below */
    (*header).nscount =
        __bswap_16(0 as libc::c_int as __uint16_t); /* empty domain */
    (*header).arcount = __bswap_16(0 as libc::c_int as __uint16_t);
    (*header).ancount = __bswap_16(0 as libc::c_int as __uint16_t);
    if flags == (1 as libc::c_uint) << 20 as libc::c_int {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !(0xf as libc::c_int) |
                 0 as libc::c_int) as u8_0
    } else if flags == (1 as libc::c_uint) << 10 as libc::c_int {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !(0xf as libc::c_int) |
                 3 as libc::c_int) as u8_0
    } else if flags == (1 as libc::c_uint) << 28 as libc::c_int {
        let mut a: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
        a.log.rcode = 2 as libc::c_int as libc::c_ushort;
        log_query((1 as libc::c_uint) << 13 as libc::c_int |
                      (1 as libc::c_uint) << 29 as libc::c_int,
                  b"error\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut a, 0 as *mut libc::c_char);
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !(0xf as libc::c_int) |
                 2 as libc::c_int) as u8_0
    } else if flags &
                  ((1 as libc::c_uint) << 7 as libc::c_int |
                       (1 as libc::c_uint) << 8 as libc::c_int) != 0 {
        if flags & (1 as libc::c_uint) << 7 as libc::c_int != 0 {
            /* we know the address */
            (*header).hb4 =
                ((*header).hb4 as libc::c_int & !(0xf as libc::c_int) |
                     0 as libc::c_int) as u8_0;
            (*header).ancount = __bswap_16(1 as libc::c_int as __uint16_t);
            (*header).hb3 =
                ((*header).hb3 as libc::c_int | 0x4 as libc::c_int) as u8_0;
            add_resource_record(header, 0 as *mut libc::c_char,
                                0 as *mut libc::c_int,
                                ::std::mem::size_of::<dns_header>() as
                                    libc::c_ulong as libc::c_int,
                                &mut p as *mut *mut libc::c_uchar, ttl,
                                0 as *mut libc::c_int,
                                1 as libc::c_int as libc::c_ushort,
                                1 as libc::c_int as libc::c_ushort,
                                b"4\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char, addrp);
        }
        if flags & (1 as libc::c_uint) << 8 as libc::c_int != 0 {
            (*header).hb4 =
                ((*header).hb4 as libc::c_int & !(0xf as libc::c_int) |
                     0 as libc::c_int) as u8_0;
            (*header).ancount =
                __bswap_16((__bswap_16((*header).ancount) as libc::c_int +
                                1 as libc::c_int) as __uint16_t);
            (*header).hb3 =
                ((*header).hb3 as libc::c_int | 0x4 as libc::c_int) as u8_0;
            add_resource_record(header, 0 as *mut libc::c_char,
                                0 as *mut libc::c_int,
                                ::std::mem::size_of::<dns_header>() as
                                    libc::c_ulong as libc::c_int,
                                &mut p as *mut *mut libc::c_uchar, ttl,
                                0 as *mut libc::c_int,
                                28 as libc::c_int as libc::c_ushort,
                                1 as libc::c_int as libc::c_ushort,
                                b"6\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char, addrp);
        }
    } else {
        /* nowhere to forward to */
        let mut a_0: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
        a_0.log.rcode = 5 as libc::c_int as libc::c_ushort;
        log_query((1 as libc::c_uint) << 13 as libc::c_int |
                      (1 as libc::c_uint) << 29 as libc::c_int,
                  b"error\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, &mut a_0, 0 as *mut libc::c_char);
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !(0xf as libc::c_int) |
                 5 as libc::c_int) as u8_0
    }
    return p.wrapping_offset_from(header as *mut libc::c_uchar) as
               libc::c_long as size_t;
}
/* check if name matches local names ie from /etc/hosts or DHCP or local mx names. */
#[no_mangle]
pub unsafe extern "C" fn check_for_local_domain(mut name: *mut libc::c_char,
                                                mut now: time_t)
 -> libc::c_int {
    let mut mx: *mut mx_srv_record = 0 as *mut mx_srv_record;
    let mut txt: *mut txt_record = 0 as *mut txt_record;
    let mut intr: *mut interface_name = 0 as *mut interface_name;
    let mut ptr: *mut ptr_record = 0 as *mut ptr_record;
    let mut naptr: *mut naptr = 0 as *mut naptr;
    naptr = (*dnsmasq_daemon).naptr;
    while !naptr.is_null() {
        if hostname_issubdomain(name, (*naptr).name) != 0 {
            return 1 as libc::c_int
        }
        naptr = (*naptr).next
    }
    mx = (*dnsmasq_daemon).mxnames;
    while !mx.is_null() {
        if hostname_issubdomain(name, (*mx).name) != 0 {
            return 1 as libc::c_int
        }
        mx = (*mx).next
    }
    txt = (*dnsmasq_daemon).txt;
    while !txt.is_null() {
        if hostname_issubdomain(name, (*txt).name) != 0 {
            return 1 as libc::c_int
        }
        txt = (*txt).next
    }
    intr = (*dnsmasq_daemon).int_names;
    while !intr.is_null() {
        if hostname_issubdomain(name, (*intr).name) != 0 {
            return 1 as libc::c_int
        }
        intr = (*intr).next
    }
    ptr = (*dnsmasq_daemon).ptr;
    while !ptr.is_null() {
        if hostname_issubdomain(name, (*ptr).name) != 0 {
            return 1 as libc::c_int
        }
        ptr = (*ptr).next
    }
    if cache_find_non_terminal(name, now) != 0 { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
/* Is the packet a reply with the answer address equal to addr?
   If so mung is into an NXDOMAIN reply and also put that information
   in the cache. */
#[no_mangle]
pub unsafe extern "C" fn check_for_bogus_wildcard(mut header: *mut dns_header,
                                                  mut qlen: size_t,
                                                  mut name: *mut libc::c_char,
                                                  mut baddr: *mut bogus_addr,
                                                  mut now: time_t)
 -> libc::c_int {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut qtype: libc::c_int = 0;
    let mut qclass: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    let mut ttl: libc::c_ulong = 0;
    let mut baddrp: *mut bogus_addr = 0 as *mut bogus_addr;
    /* skip over questions */
    p = skip_questions(header, qlen); /* bad packet */
    if p.is_null() { return 0 as libc::c_int } /* bad packet */
    i = __bswap_16((*header).ancount) as libc::c_int;
    while i != 0 as libc::c_int {
        if extract_name(header, qlen, &mut p, name, 1 as libc::c_int,
                        10 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
        let mut t_cp: *mut libc::c_uchar = p;
        qtype =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_0: *mut libc::c_uchar = p;
        qclass =
            (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_1: *mut libc::c_uchar = p;
        ttl =
            ((*t_cp_1.offset(0 as libc::c_int as isize) as u32_0) <<
                 24 as libc::c_int |
                 (*t_cp_1.offset(1 as libc::c_int as isize) as u32_0) <<
                     16 as libc::c_int |
                 (*t_cp_1.offset(2 as libc::c_int as isize) as u32_0) <<
                     8 as libc::c_int |
                 *t_cp_1.offset(3 as libc::c_int as isize) as u32_0) as
                libc::c_ulong;
        p = p.offset(4 as libc::c_int as isize);
        let mut t_cp_2: *mut libc::c_uchar = p;
        rdlen =
            (*t_cp_2.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_2.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        if qclass == 1 as libc::c_int && qtype == 1 as libc::c_int {
            if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + 4 as libc::c_int as libc::c_long) as
                     size_t <= qlen) {
                return 0 as libc::c_int
            }
            baddrp = baddr;
            while !baddrp.is_null() {
                if memcmp(&mut (*baddrp).addr as *mut in_addr as
                              *const libc::c_void, p as *const libc::c_void,
                          4 as libc::c_int as libc::c_ulong) ==
                       0 as libc::c_int {
                    /* Found a bogus address. Insert that info here, since there no SOA record
		   to get the ttl from in the normal processing */
                    cache_start_insert();
                    cache_insert(name, 0 as *mut all_addr,
                                 1 as libc::c_int as libc::c_ushort, now, ttl,
                                 (1 as libc::c_uint) << 7 as libc::c_int |
                                     (1 as libc::c_uint) << 3 as libc::c_int |
                                     (1 as libc::c_uint) << 5 as libc::c_int |
                                     (1 as libc::c_uint) <<
                                         10 as libc::c_int);
                    cache_end_insert();
                    return 1 as libc::c_int
                }
                baddrp = (*baddrp).next
            }
        }
        if if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                     libc::c_long + rdlen as libc::c_long) as size_t <= qlen)
              {
               0 as libc::c_int
           } else { p = p.offset(rdlen as isize); 1 as libc::c_int } == 0 {
            return 0 as libc::c_int
        }
        i -= 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn check_for_ignored_address(mut header:
                                                       *mut dns_header,
                                                   mut qlen: size_t,
                                                   mut baddr: *mut bogus_addr)
 -> libc::c_int {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut qtype: libc::c_int = 0;
    let mut qclass: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    let mut baddrp: *mut bogus_addr = 0 as *mut bogus_addr;
    /* skip over questions */
    p = skip_questions(header, qlen); /* bad packet */
    if p.is_null() { return 0 as libc::c_int } /* bad packet */
    i = __bswap_16((*header).ancount) as libc::c_int; /* TTL */
    while i != 0 as libc::c_int {
        p =
            skip_name(p, header, qlen,
                      10 as
                          libc::c_int); /* make ap point to 1st unamed argument */
        if p.is_null() { return 0 as libc::c_int }
        let mut t_cp: *mut libc::c_uchar = p;
        qtype =
            (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                << 8 as libc::c_int |
                *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_0: *mut libc::c_uchar = p;
        qclass =
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
        if qclass == 1 as libc::c_int && qtype == 1 as libc::c_int {
            if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                      libc::c_long + 4 as libc::c_int as libc::c_long) as
                     size_t <= qlen) {
                return 0 as libc::c_int
            }
            baddrp = baddr;
            while !baddrp.is_null() {
                if memcmp(&mut (*baddrp).addr as *mut in_addr as
                              *const libc::c_void, p as *const libc::c_void,
                          4 as libc::c_int as libc::c_ulong) ==
                       0 as libc::c_int {
                    return 1 as libc::c_int
                }
                baddrp = (*baddrp).next
            }
        }
        if if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                     libc::c_long + rdlen as libc::c_long) as size_t <= qlen)
              {
               0 as libc::c_int
           } else { p = p.offset(rdlen as isize); 1 as libc::c_int } == 0 {
            return 0 as libc::c_int
        }
        i -= 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn add_resource_record(mut header: *mut dns_header,
                                             mut limit: *mut libc::c_char,
                                             mut truncp: *mut libc::c_int,
                                             mut nameoffset: libc::c_int,
                                             mut pp: *mut *mut libc::c_uchar,
                                             mut ttl: libc::c_ulong,
                                             mut offset: *mut libc::c_int,
                                             mut type_0: libc::c_ushort,
                                             mut class: libc::c_ushort,
                                             mut format: *mut libc::c_char,
                                             mut args: ...) -> libc::c_int {
    let mut current_block: u64;
    let mut ap: ::std::ffi::VaListImpl;
    let mut sav: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = *pp;
    let mut j: libc::c_int = 0;
    let mut usval: libc::c_ushort = 0;
    let mut lval: libc::c_long = 0;
    let mut sval: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    if !(!truncp.is_null() && *truncp != 0) {
        if nameoffset > 0 as libc::c_int {
            if !limit.is_null() &&
                   p.offset(2 as libc::c_int as isize) >
                       limit as *mut libc::c_uchar {
                current_block = 16696653877814833746;
            } else {
                let mut t_s: u16_0 =
                    (nameoffset | 0xc000 as libc::c_int) as u16_0;
                let mut t_cp: *mut libc::c_uchar = p;
                let fresh24 = t_cp;
                t_cp = t_cp.offset(1);
                *fresh24 =
                    (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
                *t_cp = t_s as libc::c_uchar;
                p = p.offset(2 as libc::c_int as isize);
                current_block = 4488286894823169796;
            }
        } else {
            let mut name: *mut libc::c_char = ap.arg::<*mut libc::c_char>();
            if !name.is_null() &&
                   { p = do_rfc1035_name(p, name, limit); p.is_null() } {
                current_block = 16696653877814833746;
            } else if nameoffset < 0 as libc::c_int {
                if !limit.is_null() &&
                       p.offset(2 as libc::c_int as isize) >
                           limit as *mut libc::c_uchar {
                    current_block = 16696653877814833746;
                } else {
                    let mut t_s_0: u16_0 =
                        (-nameoffset | 0xc000 as libc::c_int) as u16_0;
                    let mut t_cp_0: *mut libc::c_uchar = p;
                    let fresh25 = t_cp_0;
                    t_cp_0 = t_cp_0.offset(1);
                    *fresh25 =
                        (t_s_0 as libc::c_int >> 8 as libc::c_int) as
                            libc::c_uchar;
                    *t_cp_0 = t_s_0 as libc::c_uchar;
                    p = p.offset(2 as libc::c_int as isize);
                    current_block = 4488286894823169796;
                }
            } else if !limit.is_null() &&
                          p.offset(1 as libc::c_int as isize) >
                              limit as *mut libc::c_uchar {
                current_block = 16696653877814833746;
            } else {
                let fresh26 = p;
                p = p.offset(1);
                *fresh26 = 0 as libc::c_int as libc::c_uchar;
                current_block = 4488286894823169796;
            }
        }
        match current_block {
            16696653877814833746 => { }
            _ =>
            /* type (2) + class (2) + ttl (4) + rdlen (2) */
            {
                if !(!limit.is_null() &&
                         p.offset(10 as libc::c_int as isize) >
                             limit as *mut libc::c_uchar) {
                    let mut t_s_1: u16_0 = type_0;
                    let mut t_cp_1: *mut libc::c_uchar = p;
                    let fresh27 = t_cp_1;
                    t_cp_1 = t_cp_1.offset(1);
                    *fresh27 =
                        (t_s_1 as libc::c_int >> 8 as libc::c_int) as
                            libc::c_uchar;
                    *t_cp_1 = t_s_1 as libc::c_uchar;
                    p = p.offset(2 as libc::c_int as isize);
                    let mut t_s_2: u16_0 = class;
                    let mut t_cp_2: *mut libc::c_uchar = p;
                    let fresh28 = t_cp_2;
                    t_cp_2 = t_cp_2.offset(1);
                    *fresh28 =
                        (t_s_2 as libc::c_int >> 8 as libc::c_int) as
                            libc::c_uchar;
                    *t_cp_2 = t_s_2 as libc::c_uchar;
                    p = p.offset(2 as libc::c_int as isize);
                    let mut t_l: u32_0 = ttl as u32_0;
                    let mut t_cp_3: *mut libc::c_uchar = p;
                    let fresh29 = t_cp_3;
                    t_cp_3 = t_cp_3.offset(1);
                    *fresh29 = (t_l >> 24 as libc::c_int) as libc::c_uchar;
                    let fresh30 = t_cp_3;
                    t_cp_3 = t_cp_3.offset(1);
                    *fresh30 = (t_l >> 16 as libc::c_int) as libc::c_uchar;
                    let fresh31 = t_cp_3;
                    t_cp_3 = t_cp_3.offset(1);
                    *fresh31 = (t_l >> 8 as libc::c_int) as libc::c_uchar;
                    *t_cp_3 = t_l as libc::c_uchar;
                    p = p.offset(4 as libc::c_int as isize);
                    /* TTL */
                    sav = p; /* Save pointer to RDLength field */
                    let mut t_s_3: u16_0 = 0 as libc::c_int as u16_0;
                    let mut t_cp_4: *mut libc::c_uchar = p;
                    let fresh32 = t_cp_4;
                    t_cp_4 = t_cp_4.offset(1);
                    *fresh32 =
                        (t_s_3 as libc::c_int >> 8 as libc::c_int) as
                            libc::c_uchar;
                    *t_cp_4 = t_s_3 as libc::c_uchar;
                    p = p.offset(2 as libc::c_int as isize);
                    loop 
                         /* Placeholder RDLength */
                         {
                        if !(*format != 0) {
                            current_block = 6665878751423064961;
                            break ;
                        }
                        match *format as libc::c_int {
                            54 => {
                                if !limit.is_null() &&
                                       p.offset(16 as libc::c_int as isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                sval = ap.arg::<*mut libc::c_char>();
                                memcpy(p as *mut libc::c_void,
                                       sval as *const libc::c_void,
                                       16 as libc::c_int as libc::c_ulong);
                                p = p.offset(16 as libc::c_int as isize)
                            }
                            52 => {
                                if !limit.is_null() &&
                                       p.offset(4 as libc::c_int as isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                sval = ap.arg::<*mut libc::c_char>();
                                memcpy(p as *mut libc::c_void,
                                       sval as *const libc::c_void,
                                       4 as libc::c_int as libc::c_ulong);
                                p = p.offset(4 as libc::c_int as isize)
                            }
                            98 => {
                                if !limit.is_null() &&
                                       p.offset(1 as libc::c_int as isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                usval =
                                    ap.arg::<libc::c_int>() as libc::c_ushort;
                                let fresh33 = p;
                                p = p.offset(1);
                                *fresh33 = usval as libc::c_uchar
                            }
                            115 => {
                                if !limit.is_null() &&
                                       p.offset(2 as libc::c_int as isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                usval =
                                    ap.arg::<libc::c_int>() as libc::c_ushort;
                                let mut t_s_4: u16_0 = usval;
                                let mut t_cp_5: *mut libc::c_uchar = p;
                                let fresh34 = t_cp_5;
                                t_cp_5 = t_cp_5.offset(1);
                                *fresh34 =
                                    (t_s_4 as libc::c_int >> 8 as libc::c_int)
                                        as libc::c_uchar;
                                *t_cp_5 = t_s_4 as libc::c_uchar;
                                p = p.offset(2 as libc::c_int as isize)
                            }
                            108 => {
                                if !limit.is_null() &&
                                       p.offset(4 as libc::c_int as isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                lval = ap.arg::<libc::c_long>();
                                let mut t_l_0: u32_0 = lval as u32_0;
                                let mut t_cp_6: *mut libc::c_uchar = p;
                                let fresh35 = t_cp_6;
                                t_cp_6 = t_cp_6.offset(1);
                                *fresh35 =
                                    (t_l_0 >> 24 as libc::c_int) as
                                        libc::c_uchar;
                                let fresh36 = t_cp_6;
                                t_cp_6 = t_cp_6.offset(1);
                                *fresh36 =
                                    (t_l_0 >> 16 as libc::c_int) as
                                        libc::c_uchar;
                                let fresh37 = t_cp_6;
                                t_cp_6 = t_cp_6.offset(1);
                                *fresh37 =
                                    (t_l_0 >> 8 as libc::c_int) as
                                        libc::c_uchar;
                                *t_cp_6 = t_l_0 as libc::c_uchar;
                                p = p.offset(4 as libc::c_int as isize)
                            }
                            100 => {
                                /* get domain-name answer arg and store it in RDATA field */
                                if !offset.is_null() {
                                    *offset =
                                        p.wrapping_offset_from(header as
                                                                   *mut libc::c_uchar)
                                            as libc::c_long as libc::c_int
                                }
                                p =
                                    do_rfc1035_name(p,
                                                    ap.arg::<*mut libc::c_char>(),
                                                    limit);
                                if p.is_null() {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                if !limit.is_null() &&
                                       p.offset(1 as libc::c_int as isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                let fresh38 = p;
                                p = p.offset(1);
                                *fresh38 = 0 as libc::c_int as libc::c_uchar
                            }
                            116 => {
                                usval =
                                    ap.arg::<libc::c_int>() as libc::c_ushort;
                                if !limit.is_null() &&
                                       p.offset(usval as libc::c_int as isize)
                                           > limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                sval = ap.arg::<*mut libc::c_char>();
                                if usval as libc::c_int != 0 as libc::c_int {
                                    memcpy(p as *mut libc::c_void,
                                           sval as *const libc::c_void,
                                           usval as libc::c_ulong);
                                }
                                p = p.offset(usval as libc::c_int as isize)
                            }
                            122 => {
                                sval = ap.arg::<*mut libc::c_char>();
                                usval =
                                    if !sval.is_null() {
                                        strlen(sval)
                                    } else {
                                        0 as libc::c_int as libc::c_ulong
                                    } as libc::c_ushort;
                                if usval as libc::c_int > 255 as libc::c_int {
                                    usval =
                                        255 as libc::c_int as libc::c_ushort
                                }
                                if !limit.is_null() &&
                                       p.offset((usval as libc::c_int +
                                                     1 as libc::c_int) as
                                                    isize) >
                                           limit as *mut libc::c_uchar {
                                    current_block = 16696653877814833746;
                                    break ;
                                }
                                let fresh39 = p;
                                p = p.offset(1);
                                *fresh39 = usval as libc::c_uchar;
                                memcpy(p as *mut libc::c_void,
                                       sval as *const libc::c_void,
                                       usval as libc::c_ulong);
                                p = p.offset(usval as libc::c_int as isize)
                            }
                            _ => { }
                        }
                        format = format.offset(1)
                    }
                    match current_block {
                        16696653877814833746 => { }
                        _ =>
                        /* clean up variable argument pointer */
                        /* Now, store real RDLength. sav already checked against limit. */
                        {
                            j =
                                (p.wrapping_offset_from(sav) as libc::c_long -
                                     2 as libc::c_int as libc::c_long) as
                                    libc::c_int;
                            let mut t_s_5: u16_0 = j as u16_0;
                            let mut t_cp_7: *mut libc::c_uchar = sav;
                            let fresh40 = t_cp_7;
                            t_cp_7 = t_cp_7.offset(1);
                            *fresh40 =
                                (t_s_5 as libc::c_int >> 8 as libc::c_int) as
                                    libc::c_uchar;
                            *t_cp_7 = t_s_5 as libc::c_uchar;
                            sav = sav.offset(2 as libc::c_int as isize);
                            *pp = p;
                            return 1 as libc::c_int
                        }
                    }
                }
            }
        }
    }
    if !truncp.is_null() { *truncp = 1 as libc::c_int }
    return 0 as libc::c_int;
}
unsafe extern "C" fn crec_ttl(mut crecp: *mut crec, mut now: time_t)
 -> libc::c_ulong {
    /* Return 0 ttl for DHCP entries, which might change
     before the lease expires, unless configured otherwise. */
    if (*crecp).flags & (1 as libc::c_uint) << 4 as libc::c_int != 0 {
        let mut conf_ttl: libc::c_int =
            if (*dnsmasq_daemon).use_dhcp_ttl != 0 {
                (*dnsmasq_daemon).dhcp_ttl
            } else { (*dnsmasq_daemon).local_ttl } as libc::c_int;
        /* Apply ceiling of actual lease length to configured TTL. */
        if (*crecp).flags & (1 as libc::c_uint) << 0 as libc::c_int == 0 &&
               (*crecp).ttd - now < conf_ttl as libc::c_long {
            return ((*crecp).ttd - now) as libc::c_ulong
        }
        return conf_ttl as libc::c_ulong
    }
    /* Immortal entries other than DHCP are local, and hold TTL in TTD field. */
    if (*crecp).flags & (1 as libc::c_uint) << 0 as libc::c_int != 0 {
        return (*crecp).ttd as libc::c_ulong
    }
    /* Return the Max TTL value if it is lower than the actual TTL */
    if (*dnsmasq_daemon).max_ttl == 0 as libc::c_int as libc::c_ulong ||
           (((*crecp).ttd - now) as libc::c_uint as libc::c_ulong) <
               (*dnsmasq_daemon).max_ttl {
        return ((*crecp).ttd - now) as libc::c_ulong
    } else { return (*dnsmasq_daemon).max_ttl };
}
unsafe extern "C" fn cache_validated(mut crecp: *const crec) -> libc::c_int {
    return ((*dnsmasq_daemon).options[(45 as libc::c_int as
                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                            as
                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong))
                                          as usize] &
                (1 as libc::c_uint) <<
                    (45 as libc::c_int as
                         libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                          as
                                                          libc::c_ulong).wrapping_mul(8
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong))
                != 0 &&
                (*crecp).flags & (1 as libc::c_uint) << 15 as libc::c_int ==
                    0) as libc::c_int;
}
/* return zero if we can't answer from cache, or packet size if we can */
#[no_mangle]
pub unsafe extern "C" fn answer_request(mut header: *mut dns_header,
                                        mut limit: *mut libc::c_char,
                                        mut qlen: size_t,
                                        mut local_addr: in_addr,
                                        mut local_netmask: in_addr,
                                        mut now: time_t,
                                        mut ad_reqd: libc::c_int,
                                        mut do_bit: libc::c_int,
                                        mut have_pseudoheader: libc::c_int)
 -> size_t {
    let mut name: *mut libc::c_char = (*dnsmasq_daemon).namebuff;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ansp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut qtype: libc::c_uint = 0;
    let mut qclass: libc::c_uint = 0;
    let mut addr: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut nameoffset: libc::c_int = 0;
    let mut flag: libc::c_ushort = 0;
    let mut q: libc::c_int = 0;
    let mut ans: libc::c_int = 0;
    let mut anscount: libc::c_int = 0 as libc::c_int;
    let mut addncount: libc::c_int = 0 as libc::c_int;
    let mut dryrun: libc::c_int = 0 as libc::c_int;
    let mut crecp: *mut crec = 0 as *mut crec;
    let mut nxdomain: libc::c_int = 0 as libc::c_int;
    let mut notimp: libc::c_int = 0 as libc::c_int;
    let mut auth: libc::c_int = 1 as libc::c_int;
    let mut trunc: libc::c_int = 0 as libc::c_int;
    let mut sec_data: libc::c_int = 1 as libc::c_int;
    let mut rec: *mut mx_srv_record = 0 as *mut mx_srv_record;
    let mut len: size_t = 0;
    let mut rd_bit: libc::c_int =
        (*header).hb3 as libc::c_int & 0x1 as libc::c_int;
    /* never answer queries with RD unset, to avoid cache snooping. */
    if __bswap_16((*header).ancount) as libc::c_int != 0 as libc::c_int ||
           __bswap_16((*header).nscount) as libc::c_int != 0 as libc::c_int ||
           __bswap_16((*header).qdcount) as libc::c_int == 0 as libc::c_int ||
           ((*header).hb3 as libc::c_int & 0x78 as libc::c_int) >>
               3 as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int as size_t
    }
    /* Don't return AD set if checking disabled. */
    if (*header).hb4 as libc::c_int & 0x10 as libc::c_int != 0 {
        sec_data = 0 as libc::c_int
    }
    /* If there is an  additional data section then it will be overwritten by
     partial replies, so we have to do a dry run to see if we can answer
     the query. */
    if __bswap_16((*header).arcount) as libc::c_int != 0 as libc::c_int {
        dryrun = 1 as libc::c_int
    } /* bad packet */
    rec = (*dnsmasq_daemon).mxnames;
    while !rec.is_null() {
        (*rec).offset = 0 as libc::c_int as libc::c_uint;
        rec = (*rec).next
    }
    loop 
         /* determine end of question section (we put answers there) */
         {
        ansp = skip_questions(header, qlen);
        if ansp.is_null() { return 0 as libc::c_int as size_t }
        /* now process each question, answers go in RRs after the question */
        p =
            header.offset(1 as libc::c_int as isize) as
                *mut libc::c_uchar; /* catch loops */
        q = __bswap_16((*header).qdcount) as libc::c_int;
        while q != 0 as libc::c_int {
            let mut count: libc::c_int = 255 as libc::c_int;
            /* save pointer to name for copying into answers */
            nameoffset =
                p.wrapping_offset_from(header as *mut libc::c_uchar) as
                    libc::c_long as libc::c_int;
            /* now extract name as .-concatenated string into name */
            if extract_name(header, qlen, &mut p, name, 1 as libc::c_int,
                            4 as libc::c_int) == 0 {
                return 0 as libc::c_int as size_t
            } /* bad packet */
            let mut t_cp: *mut libc::c_uchar =
                p; /* have we answered this question */
            qtype =
                ((*t_cp.offset(0 as libc::c_int as isize) as u16_0 as
                      libc::c_int) << 8 as libc::c_int |
                     *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                         libc::c_int) as libc::c_uint;
            p = p.offset(2 as libc::c_int as isize);
            let mut t_cp_0: *mut libc::c_uchar = p;
            qclass =
                ((*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                      libc::c_int) << 8 as libc::c_int |
                     *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                         libc::c_int) as libc::c_uint;
            p = p.offset(2 as libc::c_int as isize);
            ans = 0 as libc::c_int;
            loop  {
                count -= 1;
                if !(count != 0 as libc::c_int &&
                         {
                             crecp =
                                 cache_find_by_name(0 as *mut crec, name, now,
                                                    (1 as libc::c_uint) <<
                                                        11 as libc::c_int);
                             !crecp.is_null()
                         }) {
                    break ;
                }
                let mut cname_target: *mut libc::c_char =
                    cache_get_cname_target(crecp);
                /* If the client asked for DNSSEC  don't use cached data. */
                if (*crecp).flags &
                       ((1 as libc::c_uint) << 6 as libc::c_int |
                            (1 as libc::c_uint) << 4 as libc::c_int |
                            (1 as libc::c_uint) << 13 as libc::c_int) != 0 ||
                       rd_bit != 0 &&
                           (do_bit == 0 || cache_validated(crecp) != 0) {
                    if (*crecp).flags &
                           (1 as libc::c_uint) << 13 as libc::c_int != 0 ||
                           qtype == 5 as libc::c_int as libc::c_uint {
                        ans = 1 as libc::c_int
                    } /* give up if any cached CNAME in chain can't be used for DNSSEC reasons. */
                    if (*crecp).flags &
                           (1 as libc::c_uint) << 15 as libc::c_int == 0 {
                        sec_data = 0 as libc::c_int
                    }
                    if dryrun == 0 {
                        log_query((*crecp).flags, name, 0 as *mut all_addr,
                                  record_source((*crecp).uid));
                        if add_resource_record(header, limit,
                                               &mut trunc as *mut libc::c_int,
                                               nameoffset,
                                               &mut ansp as
                                                   *mut *mut libc::c_uchar,
                                               crec_ttl(crecp, now),
                                               &mut nameoffset as
                                                   *mut libc::c_int,
                                               5 as libc::c_int as
                                                   libc::c_ushort,
                                               1 as libc::c_int as
                                                   libc::c_ushort,
                                               b"d\x00" as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char,
                                               cname_target) != 0 {
                            anscount += 1
                        }
                    }
                } else { return 0 as libc::c_int as size_t }
                strcpy(name, cname_target);
            }
            if qtype == 16 as libc::c_int as libc::c_uint ||
                   qtype == 255 as libc::c_int as libc::c_uint {
                let mut t: *mut txt_record = 0 as *mut txt_record;
                t = (*dnsmasq_daemon).txt;
                while !t.is_null() {
                    if (*t).class as libc::c_uint == qclass &&
                           hostname_isequal(name, (*t).name) != 0 {
                        ans = 1 as libc::c_int;
                        sec_data = 0 as libc::c_int;
                        if dryrun == 0 {
                            let mut ttl: libc::c_ulong =
                                (*dnsmasq_daemon).local_ttl;
                            let mut ok: libc::c_int = 1 as libc::c_int;
                            /* Dynamically generate stat record */
                            if (*t).stat != 0 as libc::c_int {
                                ttl = 0 as libc::c_int as libc::c_ulong;
                                if cache_make_stat(t) == 0 {
                                    ok = 0 as libc::c_int
                                }
                            }
                            if ok != 0 {
                                log_query((1 as libc::c_uint) <<
                                              13 as libc::c_int |
                                              (1 as libc::c_uint) <<
                                                  17 as libc::c_int, name,
                                          0 as *mut all_addr,
                                          b"<TXT>\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char);
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       ttl,
                                                       0 as *mut libc::c_int,
                                                       16 as libc::c_int as
                                                           libc::c_ushort,
                                                       (*t).class,
                                                       b"t\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*t).len as
                                                           libc::c_int,
                                                       (*t).txt) != 0 {
                                    anscount += 1
                                }
                            }
                        }
                    }
                    t = (*t).next
                }
            }
            if qclass == 3 as libc::c_int as libc::c_uint {
                /* don't forward *.bind and *.server chaos queries - always reply with NOTIMP */
                if hostname_issubdomain(b"bind\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char, name) != 0 ||
                       hostname_issubdomain(b"server\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char, name) != 0
                   {
                    if ans == 0 {
                        notimp = 1 as libc::c_int;
                        auth = 0 as libc::c_int;
                        if dryrun == 0 {
                            addr.log.rcode =
                                4 as libc::c_int as libc::c_ushort;
                            log_query((1 as libc::c_uint) << 13 as libc::c_int
                                          |
                                          (1 as libc::c_uint) <<
                                              29 as libc::c_int, name,
                                      &mut addr, 0 as *mut libc::c_char);
                        }
                        ans = 1 as libc::c_int;
                        sec_data = 0 as libc::c_int
                    }
                }
            }
            if qclass == 1 as libc::c_int as libc::c_uint {
                let mut t_0: *mut txt_record = 0 as *mut txt_record;
                t_0 = (*dnsmasq_daemon).rr;
                while !t_0.is_null() {
                    if ((*t_0).class as libc::c_uint == qtype ||
                            qtype == 255 as libc::c_int as libc::c_uint) &&
                           hostname_isequal(name, (*t_0).name) != 0 {
                        ans = 1 as libc::c_int;
                        sec_data = 0 as libc::c_int;
                        if dryrun == 0 {
                            log_query((1 as libc::c_uint) << 13 as libc::c_int
                                          |
                                          (1 as libc::c_uint) <<
                                              17 as libc::c_int, name,
                                      0 as *mut all_addr,
                                      querystr(0 as *mut libc::c_char,
                                               (*t_0).class));
                            if add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   nameoffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).local_ttl,
                                                   0 as *mut libc::c_int,
                                                   (*t_0).class,
                                                   1 as libc::c_int as
                                                       libc::c_ushort,
                                                   b"t\x00" as *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char,
                                                   (*t_0).len as libc::c_int,
                                                   (*t_0).txt) != 0 {
                                anscount += 1
                            }
                        }
                    }
                    t_0 = (*t_0).next
                }
                if qtype == 12 as libc::c_int as libc::c_uint ||
                       qtype == 255 as libc::c_int as libc::c_uint {
                    /* see if it's w.z.y.z.in-addr.arpa format */
                    let mut is_arpa: libc::c_int =
                        in_arpa_name_2_addr(name, &mut addr);
                    let mut ptr: *mut ptr_record = 0 as *mut ptr_record;
                    let mut intr: *mut interface_name =
                        0 as *mut interface_name;
                    ptr = (*dnsmasq_daemon).ptr;
                    while !ptr.is_null() {
                        if hostname_isequal(name, (*ptr).name) != 0 {
                            break ;
                        }
                        ptr = (*ptr).next
                    }
                    if is_arpa as libc::c_uint ==
                           (1 as libc::c_uint) << 7 as libc::c_int {
                        intr = (*dnsmasq_daemon).int_names;
                        while !intr.is_null() {
                            let mut addrlist: *mut addrlist =
                                0 as *mut addrlist;
                            addrlist = (*intr).addr;
                            while !addrlist.is_null() {
                                if (*addrlist).flags & 2 as libc::c_int == 0
                                       &&
                                       addr.addr4.s_addr ==
                                           (*addrlist).addr.addr4.s_addr {
                                    break ;
                                }
                                addrlist = (*addrlist).next
                            }
                            if !addrlist.is_null() { break ; }
                            while !(*intr).next.is_null() &&
                                      strcmp((*intr).intr,
                                             (*(*intr).next).intr) ==
                                          0 as libc::c_int {
                                intr = (*intr).next
                            }
                            intr = (*intr).next
                        }
                    } else if is_arpa as libc::c_uint ==
                                  (1 as libc::c_uint) << 8 as libc::c_int {
                        intr = (*dnsmasq_daemon).int_names;
                        while !intr.is_null() {
                            let mut addrlist_0: *mut addrlist =
                                0 as *mut addrlist;
                            addrlist_0 = (*intr).addr;
                            while !addrlist_0.is_null() {
                                if (*addrlist_0).flags & 2 as libc::c_int != 0
                                       &&
                                       ({
                                            let mut __a: *const in6_addr =
                                                &mut addr.addr6 as
                                                    *mut in6_addr as
                                                    *const in6_addr;
                                            let mut __b: *const in6_addr =
                                                &mut (*addrlist_0).addr.addr6
                                                    as *mut in6_addr as
                                                    *const in6_addr;
                                            ((*__a).__in6_u.__u6_addr32[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                 ==
                                                 (*__b).__in6_u.__u6_addr32[0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                                                 &&
                                                 (*__a).__in6_u.__u6_addr32[1
                                                                                as
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
                                                 (*__a).__in6_u.__u6_addr32[2
                                                                                as
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
                                                 (*__a).__in6_u.__u6_addr32[3
                                                                                as
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
                                        }) != 0 {
                                    break ;
                                }
                                addrlist_0 = (*addrlist_0).next
                            }
                            if !addrlist_0.is_null() { break ; }
                            while !(*intr).next.is_null() &&
                                      strcmp((*intr).intr,
                                             (*(*intr).next).intr) ==
                                          0 as libc::c_int {
                                intr = (*intr).next
                            }
                            intr = (*intr).next
                        }
                    }
                    if !intr.is_null() {
                        sec_data = 0 as libc::c_int;
                        ans = 1 as libc::c_int;
                        if dryrun == 0 {
                            log_query(is_arpa as libc::c_uint |
                                          (1 as libc::c_uint) <<
                                              2 as libc::c_int |
                                          (1 as libc::c_uint) <<
                                              13 as libc::c_int, (*intr).name,
                                      &mut addr, 0 as *mut libc::c_char);
                            if add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   nameoffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).local_ttl,
                                                   0 as *mut libc::c_int,
                                                   12 as libc::c_int as
                                                       libc::c_ushort,
                                                   1 as libc::c_int as
                                                       libc::c_ushort,
                                                   b"d\x00" as *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char,
                                                   (*intr).name) != 0 {
                                anscount += 1
                            }
                        }
                    } else if !ptr.is_null() {
                        ans = 1 as libc::c_int;
                        sec_data = 0 as libc::c_int;
                        if dryrun == 0 {
                            log_query((1 as libc::c_uint) << 13 as libc::c_int
                                          |
                                          (1 as libc::c_uint) <<
                                              17 as libc::c_int, name,
                                      0 as *mut all_addr,
                                      b"<PTR>\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char);
                            ptr = (*dnsmasq_daemon).ptr;
                            while !ptr.is_null() {
                                if hostname_isequal(name, (*ptr).name) != 0 &&
                                       add_resource_record(header, limit,
                                                           &mut trunc as
                                                               *mut libc::c_int,
                                                           nameoffset,
                                                           &mut ansp as
                                                               *mut *mut libc::c_uchar,
                                                           (*dnsmasq_daemon).local_ttl,
                                                           0 as
                                                               *mut libc::c_int,
                                                           12 as libc::c_int
                                                               as
                                                               libc::c_ushort,
                                                           1 as libc::c_int as
                                                               libc::c_ushort,
                                                           b"d\x00" as
                                                               *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*ptr).ptr) != 0 {
                                    anscount += 1
                                }
                                ptr = (*ptr).next
                            }
                        }
                    } else {
                        crecp =
                            cache_find_by_addr(0 as *mut crec, &mut addr, now,
                                               is_arpa as libc::c_uint);
                        if !crecp.is_null() {
                            /* Don't use cache when DNSSEC data required, unless we know that
		     the zone is unsigned, which implies that we're doing
		     validation. */
                            if (*crecp).flags &
                                   ((1 as libc::c_uint) << 6 as libc::c_int |
                                        (1 as libc::c_uint) <<
                                            4 as libc::c_int |
                                        (1 as libc::c_uint) <<
                                            13 as libc::c_int) != 0 ||
                                   rd_bit != 0 &&
                                       (do_bit == 0 ||
                                            cache_validated(crecp) != 0) {
                                loop 
                                     /* don't answer wildcard queries with data not from /etc/hosts or dhcp leases */
                                     {
                                    if !(qtype ==
                                             255 as libc::c_int as
                                                 libc::c_uint &&
                                             (*crecp).flags &
                                                 ((1 as libc::c_uint) <<
                                                      6 as libc::c_int |
                                                      (1 as libc::c_uint) <<
                                                          4 as libc::c_int) ==
                                                 0) {
                                        if (*crecp).flags &
                                               (1 as libc::c_uint) <<
                                                   15 as libc::c_int == 0 {
                                            sec_data = 0 as libc::c_int
                                        }
                                        ans = 1 as libc::c_int;
                                        if (*crecp).flags &
                                               (1 as libc::c_uint) <<
                                                   5 as libc::c_int != 0 {
                                            auth = 0 as libc::c_int;
                                            if (*crecp).flags &
                                                   (1 as libc::c_uint) <<
                                                       10 as libc::c_int != 0
                                               {
                                                nxdomain = 1 as libc::c_int
                                            }
                                            if dryrun == 0 {
                                                log_query((*crecp).flags &
                                                              !((1 as
                                                                     libc::c_uint)
                                                                    <<
                                                                    3 as
                                                                        libc::c_int),
                                                          name, &mut addr,
                                                          0 as
                                                              *mut libc::c_char);
                                            }
                                        } else {
                                            if (*crecp).flags &
                                                   ((1 as libc::c_uint) <<
                                                        6 as libc::c_int |
                                                        (1 as libc::c_uint) <<
                                                            4 as libc::c_int)
                                                   == 0 {
                                                auth = 0 as libc::c_int
                                            }
                                            if dryrun == 0 {
                                                log_query((*crecp).flags &
                                                              !((1 as
                                                                     libc::c_uint)
                                                                    <<
                                                                    3 as
                                                                        libc::c_int),
                                                          cache_get_name(crecp),
                                                          &mut addr,
                                                          record_source((*crecp).uid));
                                                if add_resource_record(header,
                                                                       limit,
                                                                       &mut trunc
                                                                           as
                                                                           *mut libc::c_int,
                                                                       nameoffset,
                                                                       &mut ansp
                                                                           as
                                                                           *mut *mut libc::c_uchar,
                                                                       crec_ttl(crecp,
                                                                                now),
                                                                       0 as
                                                                           *mut libc::c_int,
                                                                       12 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ushort,
                                                                       1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ushort,
                                                                       b"d\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char
                                                                           as
                                                                           *mut libc::c_char,
                                                                       cache_get_name(crecp))
                                                       != 0 {
                                                    anscount += 1
                                                }
                                            }
                                        }
                                    }
                                    crecp =
                                        cache_find_by_addr(crecp, &mut addr,
                                                           now,
                                                           is_arpa as
                                                               libc::c_uint);
                                    if crecp.is_null() { break ; }
                                }
                            }
                        } else if is_rev_synth(is_arpa, &mut addr, name) != 0
                         {
                            ans = 1 as libc::c_int;
                            sec_data = 0 as libc::c_int;
                            if dryrun == 0 {
                                log_query((1 as libc::c_uint) <<
                                              13 as libc::c_int |
                                              (1 as libc::c_uint) <<
                                                  2 as libc::c_int |
                                              is_arpa as libc::c_uint, name,
                                          &mut addr, 0 as *mut libc::c_char);
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       (*dnsmasq_daemon).local_ttl,
                                                       0 as *mut libc::c_int,
                                                       12 as libc::c_int as
                                                           libc::c_ushort,
                                                       1 as libc::c_int as
                                                           libc::c_ushort,
                                                       b"d\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       name) != 0 {
                                    anscount += 1
                                }
                            }
                        } else if (*dnsmasq_daemon).options[(0 as libc::c_int
                                                                 as
                                                                 libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                  as
                                                                                                  libc::c_ulong).wrapping_mul(8
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_ulong))
                                                                as usize] &
                                      (1 as libc::c_uint) <<
                                          (0 as libc::c_int as
                                               libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                as
                                                                                libc::c_ulong).wrapping_mul(8
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_ulong))
                                      != 0 &&
                                      (is_arpa as libc::c_uint ==
                                           (1 as libc::c_uint) <<
                                               8 as libc::c_int &&
                                           private_net6(&mut addr.addr6) != 0
                                           ||
                                           is_arpa as libc::c_uint ==
                                               (1 as libc::c_uint) <<
                                                   7 as libc::c_int &&
                                               private_net(addr.addr4,
                                                           1 as libc::c_int)
                                                   != 0) {
                            let mut serv: *mut server = 0 as *mut server;
                            let mut namelen: libc::c_uint =
                                strlen(name) as libc::c_uint;
                            let mut nameend: *mut libc::c_char =
                                name.offset(namelen as isize);
                            /* see if have rev-server set */
                            serv = (*dnsmasq_daemon).servers;
                            while !serv.is_null() {
                                let mut domainlen: libc::c_uint = 0;
                                let mut matchstart: *mut libc::c_char =
                                    0 as *mut libc::c_char;
                                if !((*serv).flags &
                                         (8 as libc::c_int | 2 as libc::c_int)
                                         != 8 as libc::c_int) {
                                    domainlen =
                                        strlen((*serv).domain) as
                                            libc::c_uint;
                                    if !(domainlen ==
                                             0 as libc::c_int as libc::c_uint
                                             || domainlen > namelen) {
                                        matchstart =
                                            nameend.offset(-(domainlen as
                                                                 isize));
                                        if hostname_isequal(matchstart,
                                                            (*serv).domain) !=
                                               0 &&
                                               (namelen == domainlen ||
                                                    *matchstart.offset(-(1 as
                                                                             libc::c_int
                                                                             as
                                                                             isize))
                                                        as libc::c_int ==
                                                        '.' as i32) {
                                            break ;
                                        }
                                    }
                                }
                                serv = (*serv).next
                            }
                            /* if no configured server, not in cache, enabled and private IPV4 address, return NXDOMAIN */
                            if serv.is_null() {
                                ans = 1 as libc::c_int;
                                sec_data = 0 as libc::c_int;
                                nxdomain = 1 as libc::c_int;
                                if dryrun == 0 {
                                    log_query((1 as libc::c_uint) <<
                                                  13 as libc::c_int |
                                                  (1 as libc::c_uint) <<
                                                      2 as libc::c_int |
                                                  is_arpa as libc::c_uint |
                                                  (1 as libc::c_uint) <<
                                                      5 as libc::c_int |
                                                  (1 as libc::c_uint) <<
                                                      10 as libc::c_int, name,
                                              &mut addr,
                                              0 as *mut libc::c_char);
                                }
                            }
                        }
                    }
                }
                flag =
                    ((1 as libc::c_uint) << 7 as libc::c_int) as
                        libc::c_ushort;
                while flag != 0 {
                    let mut type_0: libc::c_ushort =
                        if flag as libc::c_uint ==
                               (1 as libc::c_uint) << 8 as libc::c_int {
                            28 as libc::c_int
                        } else { 1 as libc::c_int } as libc::c_ushort;
                    let mut intr_0: *mut interface_name =
                        0 as *mut interface_name;
                    if !(qtype != type_0 as libc::c_uint &&
                             qtype != 255 as libc::c_int as libc::c_uint) {
                        /* interface name stuff */
                        intr_0 = (*dnsmasq_daemon).int_names;
                        while !intr_0.is_null() {
                            if hostname_isequal(name, (*intr_0).name) != 0 {
                                break ;
                            }
                            intr_0 = (*intr_0).next
                        }
                        if !intr_0.is_null() {
                            let mut addrlist_1: *mut addrlist =
                                0 as *mut addrlist;
                            let mut gotit: libc::c_int = 0 as libc::c_int;
                            let mut localise: libc::c_int = 0 as libc::c_int;
                            enumerate_interfaces(0 as libc::c_int);
                            /* See if a putative address is on the network from which we received
		     the query, is so we'll filter other answers. */
                            if local_addr.s_addr !=
                                   0 as libc::c_int as libc::c_uint &&
                                   (*dnsmasq_daemon).options[(18 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                   as
                                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   libc::c_ulong))
                                                                 as usize] &
                                       (1 as libc::c_uint) <<
                                           (18 as libc::c_int as
                                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_ulong))
                                       != 0 &&
                                   type_0 as libc::c_int == 1 as libc::c_int {
                                intr_0 = (*dnsmasq_daemon).int_names;
                                while !intr_0.is_null() {
                                    if hostname_isequal(name, (*intr_0).name)
                                           != 0 {
                                        addrlist_1 = (*intr_0).addr;
                                        while !addrlist_1.is_null() {
                                            if (*addrlist_1).flags &
                                                   2 as libc::c_int == 0 &&
                                                   is_same_net((*addrlist_1).addr.addr4,
                                                               local_addr,
                                                               local_netmask)
                                                       != 0 {
                                                localise = 1 as libc::c_int;
                                                break ;
                                            } else {
                                                addrlist_1 =
                                                    (*addrlist_1).next
                                            }
                                        }
                                    }
                                    intr_0 = (*intr_0).next
                                }
                            }
                            intr_0 = (*dnsmasq_daemon).int_names;
                            while !intr_0.is_null() {
                                if hostname_isequal(name, (*intr_0).name) != 0
                                   {
                                    addrlist_1 = (*intr_0).addr;
                                    while !addrlist_1.is_null() {
                                        if (if (*addrlist_1).flags &
                                                   2 as libc::c_int != 0 {
                                                28 as libc::c_int
                                            } else { 1 as libc::c_int }) ==
                                               type_0 as libc::c_int {
                                            if !(localise != 0 &&
                                                     is_same_net((*addrlist_1).addr.addr4,
                                                                 local_addr,
                                                                 local_netmask)
                                                         == 0) {
                                                if !((*addrlist_1).flags &
                                                         4 as libc::c_int !=
                                                         0) {
                                                    ans = 1 as libc::c_int;
                                                    sec_data =
                                                        0 as libc::c_int;
                                                    if dryrun == 0 {
                                                        gotit =
                                                            1 as libc::c_int;
                                                        log_query((1 as
                                                                       libc::c_uint)
                                                                      <<
                                                                      3 as
                                                                          libc::c_int
                                                                      |
                                                                      (1 as
                                                                           libc::c_uint)
                                                                          <<
                                                                          13
                                                                              as
                                                                              libc::c_int
                                                                      |
                                                                      flag as
                                                                          libc::c_uint,
                                                                  name,
                                                                  &mut (*addrlist_1).addr,
                                                                  0 as
                                                                      *mut libc::c_char);
                                                        if add_resource_record(header,
                                                                               limit,
                                                                               &mut trunc
                                                                                   as
                                                                                   *mut libc::c_int,
                                                                               nameoffset,
                                                                               &mut ansp
                                                                                   as
                                                                                   *mut *mut libc::c_uchar,
                                                                               (*dnsmasq_daemon).local_ttl,
                                                                               0
                                                                                   as
                                                                                   *mut libc::c_int,
                                                                               type_0,
                                                                               1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ushort,
                                                                               if type_0
                                                                                      as
                                                                                      libc::c_int
                                                                                      ==
                                                                                      1
                                                                                          as
                                                                                          libc::c_int
                                                                                  {
                                                                                   b"4\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const libc::c_char
                                                                               } else {
                                                                                   b"6\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const libc::c_char
                                                                               }
                                                                                   as
                                                                                   *mut libc::c_char,
                                                                               &mut (*addrlist_1).addr
                                                                                   as
                                                                                   *mut all_addr)
                                                               != 0 {
                                                            anscount += 1
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        addrlist_1 = (*addrlist_1).next
                                    }
                                }
                                intr_0 = (*intr_0).next
                            }
                            if dryrun == 0 && gotit == 0 {
                                log_query((1 as libc::c_uint) <<
                                              3 as libc::c_int |
                                              (1 as libc::c_uint) <<
                                                  13 as libc::c_int |
                                              flag as libc::c_uint |
                                              (1 as libc::c_uint) <<
                                                  5 as libc::c_int, name,
                                          0 as *mut all_addr,
                                          0 as *mut libc::c_char);
                            }
                        } else {
                            crecp =
                                cache_find_by_name(0 as *mut crec, name, now,
                                                   flag as libc::c_uint |
                                                       (if dryrun != 0 {
                                                            ((1 as
                                                                  libc::c_uint))
                                                                <<
                                                                25 as
                                                                    libc::c_int
                                                        } else {
                                                            0 as libc::c_int
                                                                as
                                                                libc::c_uint
                                                        }));
                            if !crecp.is_null() {
                                let mut localise_0: libc::c_int =
                                    0 as libc::c_int;
                                /* See if a putative address is on the network from which we received
		     the query, is so we'll filter other answers. */
                                if local_addr.s_addr !=
                                       0 as libc::c_int as libc::c_uint &&
                                       (*dnsmasq_daemon).options[(18 as
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
                                               (18 as libc::c_int as
                                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong))
                                           != 0 &&
                                       flag as libc::c_uint ==
                                           (1 as libc::c_uint) <<
                                               7 as libc::c_int {
                                    let mut save: *mut crec = crecp;
                                    loop  {
                                        if (*crecp).flags &
                                               (1 as libc::c_uint) <<
                                                   6 as libc::c_int != 0 &&
                                               is_same_net((*crecp).addr.addr4,
                                                           local_addr,
                                                           local_netmask) != 0
                                           {
                                            localise_0 = 1 as libc::c_int;
                                            break ;
                                        } else {
                                            crecp =
                                                cache_find_by_name(crecp,
                                                                   name, now,
                                                                   flag as
                                                                       libc::c_uint);
                                            if crecp.is_null() { break ; }
                                        }
                                    }
                                    crecp = save
                                }
                                /* If the client asked for DNSSEC  don't use cached data. */
                                if (*crecp).flags &
                                       ((1 as libc::c_uint) <<
                                            6 as libc::c_int |
                                            (1 as libc::c_uint) <<
                                                4 as libc::c_int |
                                            (1 as libc::c_uint) <<
                                                13 as libc::c_int) != 0 ||
                                       rd_bit != 0 &&
                                           (do_bit == 0 ||
                                                cache_validated(crecp) != 0) {
                                    /* don't answer wildcard queries with data not from /etc/hosts
			   or DHCP leases */
                                    while !(qtype ==
                                                255 as libc::c_int as
                                                    libc::c_uint &&
                                                (*crecp).flags &
                                                    ((1 as libc::c_uint) <<
                                                         6 as libc::c_int |
                                                         (1 as libc::c_uint)
                                                             <<
                                                             4 as libc::c_int
                                                         |
                                                         (1 as libc::c_uint)
                                                             <<
                                                             13 as
                                                                 libc::c_int)
                                                    == 0) {
                                        if (*crecp).flags &
                                               (1 as libc::c_uint) <<
                                                   15 as libc::c_int == 0 {
                                            sec_data = 0 as libc::c_int
                                        }
                                        if (*crecp).flags &
                                               (1 as libc::c_uint) <<
                                                   5 as libc::c_int != 0 {
                                            ans = 1 as libc::c_int;
                                            auth = 0 as libc::c_int;
                                            if (*crecp).flags &
                                                   (1 as libc::c_uint) <<
                                                       10 as libc::c_int != 0
                                               {
                                                nxdomain = 1 as libc::c_int
                                            }
                                            if dryrun == 0 {
                                                log_query((*crecp).flags,
                                                          name,
                                                          0 as *mut all_addr,
                                                          0 as
                                                              *mut libc::c_char);
                                            }
                                        } else if !(localise_0 != 0 &&
                                                        (*crecp).flags &
                                                            (1 as
                                                                 libc::c_uint)
                                                                <<
                                                                6 as
                                                                    libc::c_int
                                                            != 0 &&
                                                        is_same_net((*crecp).addr.addr4,
                                                                    local_addr,
                                                                    local_netmask)
                                                            == 0) {
                                            if (*crecp).flags &
                                                   ((1 as libc::c_uint) <<
                                                        6 as libc::c_int |
                                                        (1 as libc::c_uint) <<
                                                            4 as libc::c_int)
                                                   == 0 {
                                                auth = 0 as libc::c_int
                                            }
                                            ans = 1 as libc::c_int;
                                            if dryrun == 0 {
                                                log_query((*crecp).flags &
                                                              !((1 as
                                                                     libc::c_uint)
                                                                    <<
                                                                    2 as
                                                                        libc::c_int),
                                                          name,
                                                          &mut (*crecp).addr,
                                                          record_source((*crecp).uid));
                                                if add_resource_record(header,
                                                                       limit,
                                                                       &mut trunc
                                                                           as
                                                                           *mut libc::c_int,
                                                                       nameoffset,
                                                                       &mut ansp
                                                                           as
                                                                           *mut *mut libc::c_uchar,
                                                                       crec_ttl(crecp,
                                                                                now),
                                                                       0 as
                                                                           *mut libc::c_int,
                                                                       type_0,
                                                                       1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ushort,
                                                                       if type_0
                                                                              as
                                                                              libc::c_int
                                                                              ==
                                                                              1
                                                                                  as
                                                                                  libc::c_int
                                                                          {
                                                                           b"4\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char
                                                                       } else {
                                                                           b"6\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char
                                                                       } as
                                                                           *mut libc::c_char,
                                                                       &mut (*crecp).addr
                                                                           as
                                                                           *mut all_addr)
                                                       != 0 {
                                                    anscount += 1
                                                }
                                            }
                                        }
                                        crecp =
                                            cache_find_by_name(crecp, name,
                                                               now,
                                                               flag as
                                                                   libc::c_uint);
                                        if crecp.is_null() { break ; }
                                    }
                                }
                            } else if is_name_synthetic(flag as libc::c_int,
                                                        name, &mut addr) != 0
                             {
                                ans = 1 as libc::c_int;
                                sec_data = 0 as libc::c_int;
                                if dryrun == 0 {
                                    log_query((1 as libc::c_uint) <<
                                                  3 as libc::c_int |
                                                  (1 as libc::c_uint) <<
                                                      13 as libc::c_int |
                                                  flag as libc::c_uint, name,
                                              &mut addr,
                                              0 as *mut libc::c_char);
                                    if add_resource_record(header, limit,
                                                           &mut trunc as
                                                               *mut libc::c_int,
                                                           nameoffset,
                                                           &mut ansp as
                                                               *mut *mut libc::c_uchar,
                                                           (*dnsmasq_daemon).local_ttl,
                                                           0 as
                                                               *mut libc::c_int,
                                                           type_0,
                                                           1 as libc::c_int as
                                                               libc::c_ushort,
                                                           if type_0 as
                                                                  libc::c_int
                                                                  ==
                                                                  1 as
                                                                      libc::c_int
                                                              {
                                                               b"4\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                           } else {
                                                               b"6\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                           } as
                                                               *mut libc::c_char,
                                                           &mut addr as
                                                               *mut all_addr)
                                           != 0 {
                                        anscount += 1
                                    }
                                }
                            }
                        }
                    }
                    flag =
                        if flag as libc::c_uint ==
                               (1 as libc::c_uint) << 7 as libc::c_int {
                            ((1 as libc::c_uint)) << 8 as libc::c_int
                        } else { 0 as libc::c_int as libc::c_uint } as
                            libc::c_ushort
                }
                if qtype == 15 as libc::c_int as libc::c_uint ||
                       qtype == 255 as libc::c_int as libc::c_uint {
                    let mut found: libc::c_int = 0 as libc::c_int;
                    rec = (*dnsmasq_daemon).mxnames;
                    while !rec.is_null() {
                        if (*rec).issrv == 0 &&
                               hostname_isequal(name, (*rec).name) != 0 {
                            found = 1 as libc::c_int;
                            ans = found;
                            sec_data = 0 as libc::c_int;
                            if dryrun == 0 {
                                let mut offset: libc::c_int = 0;
                                log_query((1 as libc::c_uint) <<
                                              13 as libc::c_int |
                                              (1 as libc::c_uint) <<
                                                  17 as libc::c_int, name,
                                          0 as *mut all_addr,
                                          b"<MX>\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char);
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       (*dnsmasq_daemon).local_ttl,
                                                       &mut offset as
                                                           *mut libc::c_int,
                                                       15 as libc::c_int as
                                                           libc::c_ushort,
                                                       1 as libc::c_int as
                                                           libc::c_ushort,
                                                       b"sd\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*rec).weight,
                                                       (*rec).target) != 0 {
                                    anscount += 1;
                                    if !(*rec).target.is_null() {
                                        (*rec).offset = offset as libc::c_uint
                                    }
                                }
                            }
                        }
                        rec = (*rec).next
                    }
                    if found == 0 &&
                           ((*dnsmasq_daemon).options[(3 as libc::c_int as
                                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                            as
                                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong))
                                                          as usize] &
                                (1 as libc::c_uint) <<
                                    (3 as libc::c_int as
                                         libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                          as
                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_ulong))
                                != 0 ||
                                (*dnsmasq_daemon).options[(10 as libc::c_int
                                                               as
                                                               libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                as
                                                                                                libc::c_ulong).wrapping_mul(8
                                                                                                                                as
                                                                                                                                libc::c_int
                                                                                                                                as
                                                                                                                                libc::c_ulong))
                                                              as usize] &
                                    (1 as libc::c_uint) <<
                                        (10 as libc::c_int as
                                             libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                              as
                                                                              libc::c_ulong).wrapping_mul(8
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_ulong))
                                    != 0) &&
                           !cache_find_by_name(0 as *mut crec, name, now,
                                               (1 as libc::c_uint) <<
                                                   6 as libc::c_int |
                                                   (1 as libc::c_uint) <<
                                                       4 as libc::c_int |
                                                   (1 as libc::c_uint) <<
                                                       25 as
                                                           libc::c_int).is_null()
                       {
                        ans = 1 as libc::c_int;
                        sec_data = 0 as libc::c_int;
                        if dryrun == 0 {
                            log_query((1 as libc::c_uint) << 13 as libc::c_int
                                          |
                                          (1 as libc::c_uint) <<
                                              17 as libc::c_int, name,
                                      0 as *mut all_addr,
                                      b"<MX>\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char);
                            if add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   nameoffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).local_ttl,
                                                   0 as *mut libc::c_int,
                                                   15 as libc::c_int as
                                                       libc::c_ushort,
                                                   1 as libc::c_int as
                                                       libc::c_ushort,
                                                   b"sd\x00" as *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char,
                                                   1 as libc::c_int,
                                                   if (*dnsmasq_daemon).options[(3
                                                                                     as
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
                                                          (1 as libc::c_uint)
                                                              <<
                                                              (3 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                    as
                                                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_ulong))
                                                          != 0 {
                                                       name
                                                   } else {
                                                       (*dnsmasq_daemon).mxtarget
                                                   }) != 0 {
                                anscount += 1
                            }
                        }
                    }
                }
                if qtype == 33 as libc::c_int as libc::c_uint ||
                       qtype == 255 as libc::c_int as libc::c_uint {
                    let mut found_0: libc::c_int = 0 as libc::c_int;
                    let mut move_0: *mut mx_srv_record =
                        0 as *mut mx_srv_record;
                    let mut up: *mut *mut mx_srv_record =
                        &mut (*dnsmasq_daemon).mxnames;
                    rec = (*dnsmasq_daemon).mxnames;
                    while !rec.is_null() {
                        if (*rec).issrv != 0 &&
                               hostname_isequal(name, (*rec).name) != 0 {
                            ans = 1 as libc::c_int;
                            found_0 = ans;
                            sec_data = 0 as libc::c_int;
                            if dryrun == 0 {
                                let mut offset_0: libc::c_int = 0;
                                log_query((1 as libc::c_uint) <<
                                              13 as libc::c_int |
                                              (1 as libc::c_uint) <<
                                                  17 as libc::c_int, name,
                                          0 as *mut all_addr,
                                          b"<SRV>\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char);
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       (*dnsmasq_daemon).local_ttl,
                                                       &mut offset_0 as
                                                           *mut libc::c_int,
                                                       33 as libc::c_int as
                                                           libc::c_ushort,
                                                       1 as libc::c_int as
                                                           libc::c_ushort,
                                                       b"sssd\x00" as
                                                           *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*rec).priority,
                                                       (*rec).weight,
                                                       (*rec).srvport,
                                                       (*rec).target) != 0 {
                                    anscount += 1;
                                    if !(*rec).target.is_null() {
                                        (*rec).offset =
                                            offset_0 as libc::c_uint
                                    }
                                }
                            }
                            /* If we are returning local answers depending on network,
			       filter here. */
                            /* unlink first SRV record found */
                            if move_0.is_null() {
                                move_0 = rec;
                                *up = (*rec).next
                            } else { up = &mut (*rec).next }
                        } else { up = &mut (*rec).next }
                        rec = (*rec).next
                    }
                    /* put first SRV record back at the end. */
                    if !move_0.is_null() {
                        *up = move_0;
                        (*move_0).next = 0 as *mut mx_srv_record
                    }
                    if found_0 == 0 {
                        crecp =
                            cache_find_by_name(0 as *mut crec, name, now,
                                               (1 as libc::c_uint) <<
                                                   30 as libc::c_int |
                                                   (if dryrun != 0 {
                                                        ((1 as libc::c_uint))
                                                            <<
                                                            25 as libc::c_int
                                                    } else {
                                                        0 as libc::c_int as
                                                            libc::c_uint
                                                    }));
                        if !crecp.is_null() && rd_bit != 0 &&
                               (do_bit == 0 ||
                                    (*dnsmasq_daemon).options[(45 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                    as
                                                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_ulong))
                                                                  as usize] &
                                        (1 as libc::c_uint) <<
                                            (45 as libc::c_int as
                                                 libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                  as
                                                                                  libc::c_ulong).wrapping_mul(8
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_ulong))
                                        != 0 &&
                                        (*crecp).flags &
                                            (1 as libc::c_uint) <<
                                                15 as libc::c_int == 0) {
                            if (*crecp).flags &
                                   (1 as libc::c_uint) << 15 as libc::c_int ==
                                   0 {
                                sec_data = 0 as libc::c_int
                            }
                            auth = 0 as libc::c_int;
                            ans = 1 as libc::c_int;
                            found_0 = ans;
                            loop  {
                                if (*crecp).flags &
                                       (1 as libc::c_uint) << 5 as libc::c_int
                                       != 0 {
                                    if (*crecp).flags &
                                           (1 as libc::c_uint) <<
                                               10 as libc::c_int != 0 {
                                        nxdomain = 1 as libc::c_int
                                    }
                                    if dryrun == 0 {
                                        log_query((*crecp).flags, name,
                                                  0 as *mut all_addr,
                                                  0 as *mut libc::c_char);
                                    }
                                } else if dryrun == 0 {
                                    let mut target: *mut libc::c_char =
                                        blockdata_retrieve((*crecp).addr.srv.target,
                                                           (*crecp).addr.srv.targetlen
                                                               as size_t,
                                                           0 as
                                                               *mut libc::c_void)
                                            as *mut libc::c_char;
                                    log_query((*crecp).flags, name,
                                              0 as *mut all_addr,
                                              0 as *mut libc::c_char);
                                    if add_resource_record(header, limit,
                                                           &mut trunc as
                                                               *mut libc::c_int,
                                                           nameoffset,
                                                           &mut ansp as
                                                               *mut *mut libc::c_uchar,
                                                           crec_ttl(crecp,
                                                                    now),
                                                           0 as
                                                               *mut libc::c_int,
                                                           33 as libc::c_int
                                                               as
                                                               libc::c_ushort,
                                                           1 as libc::c_int as
                                                               libc::c_ushort,
                                                           b"sssd\x00" as
                                                               *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           (*crecp).addr.srv.priority
                                                               as libc::c_int,
                                                           (*crecp).addr.srv.weight
                                                               as libc::c_int,
                                                           (*crecp).addr.srv.srvport
                                                               as libc::c_int,
                                                           target) != 0 {
                                        anscount += 1
                                    }
                                }
                                crecp =
                                    cache_find_by_name(crecp, name, now,
                                                       (1 as libc::c_uint) <<
                                                           30 as libc::c_int);
                                if crecp.is_null() { break ; }
                            }
                        }
                    }
                    if found_0 == 0 &&
                           (*dnsmasq_daemon).options[(1 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (1 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               != 0 &&
                           (qtype == 33 as libc::c_int as libc::c_uint ||
                                qtype == 255 as libc::c_int as libc::c_uint &&
                                    !strchr(name, '_' as i32).is_null()) {
                        ans = 1 as libc::c_int;
                        sec_data = 0 as libc::c_int;
                        if dryrun == 0 {
                            log_query((1 as libc::c_uint) << 13 as libc::c_int
                                          |
                                          (1 as libc::c_uint) <<
                                              5 as libc::c_int, name,
                                      0 as *mut all_addr,
                                      0 as *mut libc::c_char);
                        }
                    }
                }
                if qtype == 35 as libc::c_int as libc::c_uint ||
                       qtype == 255 as libc::c_int as libc::c_uint {
                    let mut na: *mut naptr = 0 as *mut naptr;
                    na = (*dnsmasq_daemon).naptr;
                    while !na.is_null() {
                        if hostname_isequal(name, (*na).name) != 0 {
                            ans = 1 as libc::c_int;
                            sec_data = 0 as libc::c_int;
                            if dryrun == 0 {
                                log_query((1 as libc::c_uint) <<
                                              13 as libc::c_int |
                                              (1 as libc::c_uint) <<
                                                  17 as libc::c_int, name,
                                          0 as *mut all_addr,
                                          b"<NAPTR>\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char);
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       (*dnsmasq_daemon).local_ttl,
                                                       0 as *mut libc::c_int,
                                                       35 as libc::c_int as
                                                           libc::c_ushort,
                                                       1 as libc::c_int as
                                                           libc::c_ushort,
                                                       b"sszzzd\x00" as
                                                           *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       (*na).order,
                                                       (*na).pref,
                                                       (*na).flags,
                                                       (*na).services,
                                                       (*na).regexp,
                                                       (*na).replace) != 0 {
                                    anscount += 1
                                }
                            }
                        }
                        na = (*na).next
                    }
                }
                if qtype == 253 as libc::c_int as libc::c_uint {
                    ans = 1 as libc::c_int;
                    nxdomain = 1 as libc::c_int;
                    sec_data = 0 as libc::c_int
                }
                if qtype == 6 as libc::c_int as libc::c_uint &&
                       (*dnsmasq_daemon).options[(1 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (1 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           != 0 {
                    ans = 1 as libc::c_int;
                    sec_data = 0 as libc::c_int;
                    if dryrun == 0 {
                        log_query((1 as libc::c_uint) << 13 as libc::c_int |
                                      (1 as libc::c_uint) << 5 as libc::c_int,
                                  name, &mut addr, 0 as *mut libc::c_char);
                    }
                }
            }
            if ans == 0 { return 0 as libc::c_int as size_t }
            q -= 1
        }
        if !(dryrun != 0) { break ; }
        dryrun = 0 as libc::c_int
    }
    /* create an additional data section, for stuff in SRV and MX record replies. */
    rec = (*dnsmasq_daemon).mxnames;
    while !rec.is_null() {
        if (*rec).offset != 0 as libc::c_int as libc::c_uint {
            /* squash dupes */
            let mut tmp: *mut mx_srv_record = 0 as *mut mx_srv_record;
            tmp = (*rec).next;
            while !tmp.is_null() {
                if (*tmp).offset != 0 as libc::c_int as libc::c_uint &&
                       hostname_isequal((*rec).target, (*tmp).target) != 0 {
                    (*tmp).offset = 0 as libc::c_int as libc::c_uint
                }
                tmp = (*tmp).next
            }
            crecp = 0 as *mut crec;
            loop  {
                crecp =
                    cache_find_by_name(crecp, (*rec).target, now,
                                       (1 as libc::c_uint) << 7 as libc::c_int
                                           |
                                           (1 as libc::c_uint) <<
                                               8 as libc::c_int);
                if crecp.is_null() { break ; }
                let mut type_1: libc::c_int =
                    if (*crecp).flags &
                           (1 as libc::c_uint) << 7 as libc::c_int != 0 {
                        1 as libc::c_int
                    } else { 28 as libc::c_int };
                if (*crecp).flags & (1 as libc::c_uint) << 5 as libc::c_int !=
                       0 {
                    continue ;
                }
                if add_resource_record(header, limit, 0 as *mut libc::c_int,
                                       (*rec).offset as libc::c_int,
                                       &mut ansp as *mut *mut libc::c_uchar,
                                       crec_ttl(crecp, now),
                                       0 as *mut libc::c_int,
                                       type_1 as libc::c_ushort,
                                       1 as libc::c_int as libc::c_ushort,
                                       if (*crecp).flags &
                                              (1 as libc::c_uint) <<
                                                  7 as libc::c_int != 0 {
                                           b"4\x00" as *const u8 as
                                               *const libc::c_char
                                       } else {
                                           b"6\x00" as *const u8 as
                                               *const libc::c_char
                                       } as *mut libc::c_char,
                                       &mut (*crecp).addr as *mut all_addr) !=
                       0 {
                    addncount += 1
                }
            }
        }
        rec = (*rec).next
    }
    /* done all questions, set up header and return length of result */
  /* clear authoritative and truncated flags, set QR flag */
    (*header).hb3 =
        ((*header).hb3 as libc::c_int &
             !(0x4 as libc::c_int | 0x2 as libc::c_int) | 0x80 as libc::c_int)
            as u8_0;
    /* set RA flag */
    (*header).hb4 =
        ((*header).hb4 as libc::c_int | 0x80 as libc::c_int) as u8_0;
    /* authoritative - only hosts and DHCP derived names. */
    if auth != 0 {
        (*header).hb3 =
            ((*header).hb3 as libc::c_int | 0x4 as libc::c_int) as u8_0
    }
    /* truncation */
    if trunc != 0 {
        (*header).hb3 =
            ((*header).hb3 as libc::c_int | 0x2 as libc::c_int) as u8_0
    } /* no error */
    if nxdomain != 0 {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !(0xf as libc::c_int) |
                 3 as libc::c_int) as u8_0
    } else if notimp != 0 {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !(0xf as libc::c_int) |
                 4 as libc::c_int) as u8_0
    } else {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !(0xf as libc::c_int) |
                 0 as libc::c_int) as u8_0
    }
    (*header).ancount = __bswap_16(anscount as __uint16_t);
    (*header).nscount = __bswap_16(0 as libc::c_int as __uint16_t);
    (*header).arcount = __bswap_16(addncount as __uint16_t);
    len =
        ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
            libc::c_long as size_t;
    /* Advertise our packet size limit in our reply */
    if have_pseudoheader != 0 {
        len =
            add_pseudoheader(header, len, limit as *mut libc::c_uchar,
                             (*dnsmasq_daemon).edns_pktsz, 0 as libc::c_int,
                             0 as *mut libc::c_uchar,
                             0 as libc::c_int as size_t, do_bit,
                             0 as libc::c_int)
    }
    if ad_reqd != 0 && sec_data != 0 {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int | 0x20 as libc::c_int) as u8_0
    } else {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !(0x20 as libc::c_int)) as u8_0
    }
    return len;
}
