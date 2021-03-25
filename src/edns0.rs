
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
pub unsafe extern "C" fn find_pseudoheader(mut header: *mut dns_header,
                                           mut plen: size_t,
                                           mut len: *mut size_t,
                                           mut p: *mut *mut libc::c_uchar,
                                           mut is_sign: *mut libc::c_int,
                                           mut is_last: *mut libc::c_int)
 -> *mut libc::c_uchar {
    /* See if packet has an RFC2671 pseudoheader, and if so return a pointer to it. 
     also return length of pseudoheader in *len and pointer to the UDP size in *p
     Finally, check to see if a packet is signed. If it is we cannot change a single bit before
     forwarding. We look for TSIG in the addition section, and TKEY queries (for GSS-TSIG) */
    let mut i: libc::c_int = 0; /* TTL */
    let mut arcount: libc::c_int =
        __bswap_16((*header).arcount) as libc::c_int;
    let mut ansp: *mut libc::c_uchar =
        header.offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
    let mut rdlen: libc::c_ushort = 0;
    let mut type_0: libc::c_ushort = 0;
    let mut class: libc::c_ushort = 0;
    let mut ret: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if !is_sign.is_null() {
        *is_sign = 0 as libc::c_int;
        if ((*header).hb3 as libc::c_int & 0x78 as libc::c_int) >>
               3 as libc::c_int == 0 as libc::c_int {
            i = __bswap_16((*header).qdcount) as libc::c_int;
            while i != 0 as libc::c_int {
                ansp = skip_name(ansp, header, plen, 4 as libc::c_int);
                if ansp.is_null() { return 0 as *mut libc::c_uchar }
                let mut t_cp: *mut libc::c_uchar = ansp;
                type_0 =
                    ((*t_cp.offset(0 as libc::c_int as isize) as u16_0 as
                          libc::c_int) << 8 as libc::c_int |
                         *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                             libc::c_int) as libc::c_ushort;
                ansp = ansp.offset(2 as libc::c_int as isize);
                let mut t_cp_0: *mut libc::c_uchar = ansp;
                class =
                    ((*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                          libc::c_int) << 8 as libc::c_int |
                         *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                             libc::c_int) as libc::c_ushort;
                ansp = ansp.offset(2 as libc::c_int as isize);
                if class as libc::c_int == 1 as libc::c_int &&
                       type_0 as libc::c_int == 249 as libc::c_int {
                    *is_sign = 1 as libc::c_int
                }
                i -= 1
            }
        }
    } else {
        ansp = skip_questions(header, plen);
        if ansp.is_null() { return 0 as *mut libc::c_uchar }
    }
    if arcount == 0 as libc::c_int { return 0 as *mut libc::c_uchar }
    ansp =
        skip_section(ansp,
                     __bswap_16((*header).ancount) as libc::c_int +
                         __bswap_16((*header).nscount) as libc::c_int, header,
                     plen);
    if ansp.is_null() { return 0 as *mut libc::c_uchar }
    i = 0 as libc::c_int;
    while i < arcount {
        let mut save: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut start: *mut libc::c_uchar = ansp;
        ansp = skip_name(ansp, header, plen, 10 as libc::c_int);
        if ansp.is_null() { return 0 as *mut libc::c_uchar }
        let mut t_cp_1: *mut libc::c_uchar = ansp;
        type_0 =
            ((*t_cp_1.offset(0 as libc::c_int as isize) as u16_0 as
                  libc::c_int) << 8 as libc::c_int |
                 *t_cp_1.offset(1 as libc::c_int as isize) as u16_0 as
                     libc::c_int) as libc::c_ushort;
        ansp = ansp.offset(2 as libc::c_int as isize);
        save = ansp;
        let mut t_cp_2: *mut libc::c_uchar = ansp;
        class =
            ((*t_cp_2.offset(0 as libc::c_int as isize) as u16_0 as
                  libc::c_int) << 8 as libc::c_int |
                 *t_cp_2.offset(1 as libc::c_int as isize) as u16_0 as
                     libc::c_int) as libc::c_ushort;
        ansp = ansp.offset(2 as libc::c_int as isize);
        ansp = ansp.offset(4 as libc::c_int as isize);
        let mut t_cp_3: *mut libc::c_uchar = ansp;
        rdlen =
            ((*t_cp_3.offset(0 as libc::c_int as isize) as u16_0 as
                  libc::c_int) << 8 as libc::c_int |
                 *t_cp_3.offset(1 as libc::c_int as isize) as u16_0 as
                     libc::c_int) as libc::c_ushort;
        ansp = ansp.offset(2 as libc::c_int as isize);
        if if !((ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                     libc::c_long + rdlen as libc::c_long) as size_t <= plen)
              {
               0 as libc::c_int
           } else {
               ansp = ansp.offset(rdlen as libc::c_int as isize);
               1 as libc::c_int
           } == 0 {
            return 0 as *mut libc::c_uchar
        }
        if type_0 as libc::c_int == 41 as libc::c_int {
            if !len.is_null() {
                *len =
                    ansp.wrapping_offset_from(start) as libc::c_long as size_t
            }
            if !p.is_null() { *p = save }
            if !is_last.is_null() {
                *is_last = (i == arcount - 1 as libc::c_int) as libc::c_int
            }
            ret = start
        } else if !is_sign.is_null() && i == arcount - 1 as libc::c_int &&
                      class as libc::c_int == 255 as libc::c_int &&
                      type_0 as libc::c_int == 250 as libc::c_int {
            *is_sign = 1 as libc::c_int
        }
        i += 1
    }
    return ret;
}
/* replace == 2 ->delete existing option only. */
#[no_mangle]
pub unsafe extern "C" fn add_pseudoheader(mut header: *mut dns_header,
                                          mut plen: size_t,
                                          mut limit: *mut libc::c_uchar,
                                          mut udp_sz: libc::c_ushort,
                                          mut optno: libc::c_int,
                                          mut opt: *mut libc::c_uchar,
                                          mut optlen: size_t,
                                          mut set_do: libc::c_int,
                                          mut replace: libc::c_int)
 -> size_t {
    let mut lenp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut datap: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut udp_len: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buff: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut rdlen: libc::c_int = 0 as libc::c_int;
    let mut is_sign: libc::c_int = 0;
    let mut is_last: libc::c_int = 0;
    let mut flags: libc::c_ushort =
        if set_do != 0 { 0x8000 as libc::c_int } else { 0 as libc::c_int } as
            libc::c_ushort;
    let mut rcode: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    p =
        find_pseudoheader(header, plen, 0 as *mut size_t, &mut udp_len,
                          &mut is_sign, &mut is_last);
    if is_sign != 0 { return plen }
    if !p.is_null() {
        /* Existing header */
        let mut i: libc::c_int = 0; /* bad packet */
        let mut code: libc::c_ushort = 0;
        let mut len: libc::c_ushort = 0;
        p = udp_len;
        let mut t_cp: *mut libc::c_uchar = p;
        udp_sz =
            ((*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int)
                 << 8 as libc::c_int |
                 *t_cp.offset(1 as libc::c_int as isize) as u16_0 as
                     libc::c_int) as libc::c_ushort;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_0: *mut libc::c_uchar = p;
        rcode =
            ((*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                  libc::c_int) << 8 as libc::c_int |
                 *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                     libc::c_int) as libc::c_ushort;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_1: *mut libc::c_uchar = p;
        flags =
            ((*t_cp_1.offset(0 as libc::c_int as isize) as u16_0 as
                  libc::c_int) << 8 as libc::c_int |
                 *t_cp_1.offset(1 as libc::c_int as isize) as u16_0 as
                     libc::c_int) as libc::c_ushort;
        p = p.offset(2 as libc::c_int as isize);
        if set_do != 0 {
            p = p.offset(-(2 as libc::c_int as isize));
            flags =
                (flags as libc::c_int | 0x8000 as libc::c_int) as
                    libc::c_ushort;
            let mut t_s: u16_0 = flags;
            let mut t_cp_2: *mut libc::c_uchar = p;
            let fresh6 = t_cp_2;
            t_cp_2 = t_cp_2.offset(1);
            *fresh6 =
                (t_s as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
            *t_cp_2 = t_s as libc::c_uchar;
            p = p.offset(2 as libc::c_int as isize)
        }
        lenp = p;
        let mut t_cp_3: *mut libc::c_uchar = p;
        rdlen =
            (*t_cp_3.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_3.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as
                  libc::c_long + rdlen as libc::c_long) as size_t <= plen) {
            return plen
        }
        datap = p;
        /* no option to add */
        if optno == 0 as libc::c_int { return plen }
        /* check if option already there */
        i = 0 as libc::c_int;
        while (i + 4 as libc::c_int) < rdlen {
            let mut t_cp_4: *mut libc::c_uchar = p;
            code =
                ((*t_cp_4.offset(0 as libc::c_int as isize) as u16_0 as
                      libc::c_int) << 8 as libc::c_int |
                     *t_cp_4.offset(1 as libc::c_int as isize) as u16_0 as
                         libc::c_int) as libc::c_ushort;
            p = p.offset(2 as libc::c_int as isize);
            let mut t_cp_5: *mut libc::c_uchar = p;
            len =
                ((*t_cp_5.offset(0 as libc::c_int as isize) as u16_0 as
                      libc::c_int) << 8 as libc::c_int |
                     *t_cp_5.offset(1 as libc::c_int as isize) as u16_0 as
                         libc::c_int) as libc::c_ushort;
            p = p.offset(2 as libc::c_int as isize);
            /* malformed option, delete the whole OPT RR and start again. */
            if i + 4 as libc::c_int + len as libc::c_int > rdlen {
                rdlen = 0 as libc::c_int;
                is_last = 0 as libc::c_int;
                break ;
            } else if code as libc::c_int == optno {
                if replace == 0 as libc::c_int { return plen }
                /* delete option if we're to replace it. */
                p = p.offset(-(4 as libc::c_int as isize));
                rdlen -= len as libc::c_int + 4 as libc::c_int;
                memmove(p as *mut libc::c_void,
                        p.offset(len as libc::c_int as
                                     isize).offset(4 as libc::c_int as isize)
                            as *const libc::c_void,
                        (rdlen - i) as libc::c_ulong);
                let mut t_s_0: u16_0 = rdlen as u16_0;
                let mut t_cp_6: *mut libc::c_uchar = lenp;
                let fresh7 = t_cp_6;
                t_cp_6 = t_cp_6.offset(1);
                *fresh7 =
                    (t_s_0 as libc::c_int >> 8 as libc::c_int) as
                        libc::c_uchar;
                *t_cp_6 = t_s_0 as libc::c_uchar;
                lenp = lenp.offset(2 as libc::c_int as isize);
                lenp = lenp.offset(-(2 as libc::c_int as isize))
            } else {
                p = p.offset(len as libc::c_int as isize);
                i += len as libc::c_int + 4 as libc::c_int
            }
        }
        /* If we're going to extend the RR, it has to be the last RR in the packet */
        if is_last == 0 {
            /* First, take a copy of the options. */
            if rdlen != 0 as libc::c_int &&
                   {
                       buff =
                           whine_malloc(rdlen as size_t) as
                               *mut libc::c_uchar;
                       !buff.is_null()
                   } {
                memcpy(buff as *mut libc::c_void,
                       datap as *const libc::c_void, rdlen as libc::c_ulong);
            }
            /* now, delete OPT RR */
            plen = rrfilter(header, plen, 0 as libc::c_int);
            /* Now, force addition of a new one */
            p = 0 as *mut libc::c_uchar
        }
    }
    if p.is_null() {
        /* We are (re)adding the pseudoheader */
        p = skip_questions(header, plen);
        if p.is_null() ||
               {
                   p =
                       skip_section(p,
                                    __bswap_16((*header).ancount) as
                                        libc::c_int +
                                        __bswap_16((*header).nscount) as
                                            libc::c_int +
                                        __bswap_16((*header).arcount) as
                                            libc::c_int, header, plen);
                   p.is_null()
               } {
            free(buff as *mut libc::c_void);
            return plen
        }
        if p.offset(11 as libc::c_int as isize) > limit {
            free(buff as *mut libc::c_void);
            return plen
            /* Too big */
        } /* empty name */
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = 0 as libc::c_int as libc::c_uchar;
        let mut t_s_1: u16_0 = 41 as libc::c_int as u16_0;
        let mut t_cp_7: *mut libc::c_uchar = p;
        let fresh9 = t_cp_7;
        t_cp_7 = t_cp_7.offset(1);
        *fresh9 = (t_s_1 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_7 = t_s_1 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_s_2: u16_0 = udp_sz;
        let mut t_cp_8: *mut libc::c_uchar = p;
        let fresh10 = t_cp_8;
        t_cp_8 = t_cp_8.offset(1);
        *fresh10 =
            (t_s_2 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_8 = t_s_2 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        /* max packet length, 512 if not given in EDNS0 header */
        let mut t_s_3: u16_0 = rcode;
        let mut t_cp_9: *mut libc::c_uchar = p;
        let fresh11 = t_cp_9;
        t_cp_9 = t_cp_9.offset(1);
        *fresh11 =
            (t_s_3 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_9 = t_s_3 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        /* extended RCODE and version */
        let mut t_s_4: u16_0 = flags;
        let mut t_cp_10: *mut libc::c_uchar = p;
        let fresh12 = t_cp_10;
        t_cp_10 = t_cp_10.offset(1);
        *fresh12 =
            (t_s_4 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_10 = t_s_4 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        /* DO flag */
        lenp = p;
        let mut t_s_5: u16_0 = rdlen as u16_0;
        let mut t_cp_11: *mut libc::c_uchar = p;
        let fresh13 = t_cp_11;
        t_cp_11 = t_cp_11.offset(1);
        *fresh13 =
            (t_s_5 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_11 = t_s_5 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        /* RDLEN */
        datap = p;
        /* Copy back any options */
        if !buff.is_null() {
            if p.offset(rdlen as isize) > limit {
                free(buff as *mut libc::c_void);
                return plen
                /* Too big */
            }
            memcpy(p as *mut libc::c_void, buff as *const libc::c_void,
                   rdlen as libc::c_ulong);
            free(buff as *mut libc::c_void);
            p = p.offset(rdlen as isize)
        }
        /* Only bump arcount if RR is going to fit */
        if optlen as ssize_t <=
               limit.wrapping_offset_from(p.offset(4 as libc::c_int as isize))
                   as libc::c_long {
            (*header).arcount =
                __bswap_16((__bswap_16((*header).arcount) as libc::c_int +
                                1 as libc::c_int) as __uint16_t)
        }
    } /* Too big */
    if optlen as ssize_t >
           limit.wrapping_offset_from(p.offset(4 as libc::c_int as isize)) as
               libc::c_long {
        return plen
    }
    /* Add new option */
    if optno != 0 as libc::c_int && replace != 2 as libc::c_int {
        if p.offset(4 as libc::c_int as isize) > limit {
            return plen
        } /* Too big */
        let mut t_s_6: u16_0 = optno as u16_0; /* Too big */
        let mut t_cp_12: *mut libc::c_uchar =
            p; /* can't get mac address, just delete any incoming. */
        let fresh14 = t_cp_12; /* handle 6 byte MACs */
        t_cp_12 = t_cp_12.offset(1);
        *fresh14 =
            (t_s_6 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_12 = t_s_6 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_s_7: u16_0 = optlen as u16_0;
        let mut t_cp_13: *mut libc::c_uchar = p;
        let fresh15 = t_cp_13;
        t_cp_13 = t_cp_13.offset(1);
        *fresh15 =
            (t_s_7 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_13 = t_s_7 as libc::c_uchar;
        p = p.offset(2 as libc::c_int as isize);
        if p.offset(optlen as isize) > limit { return plen }
        memcpy(p as *mut libc::c_void, opt as *const libc::c_void, optlen);
        p = p.offset(optlen as isize);
        let mut t_s_8: u16_0 =
            p.wrapping_offset_from(datap) as libc::c_long as u16_0;
        let mut t_cp_14: *mut libc::c_uchar = lenp;
        let fresh16 = t_cp_14;
        t_cp_14 = t_cp_14.offset(1);
        *fresh16 =
            (t_s_8 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
        *t_cp_14 = t_s_8 as libc::c_uchar;
        lenp = lenp.offset(2 as libc::c_int as isize)
    }
    return p.wrapping_offset_from(header as *mut libc::c_uchar) as
               libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn add_do_bit(mut header: *mut dns_header,
                                    mut plen: size_t,
                                    mut limit: *mut libc::c_uchar) -> size_t {
    return add_pseudoheader(header, plen, limit,
                            512 as libc::c_int as libc::c_ushort,
                            0 as libc::c_int, 0 as *mut libc::c_uchar,
                            0 as libc::c_int as size_t, 1 as libc::c_int,
                            0 as libc::c_int);
}
unsafe extern "C" fn char64(mut c: libc::c_uchar) -> libc::c_uchar {
    return (*::std::mem::transmute::<&[u8; 65],
                                     &[libc::c_char; 65]>(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\x00"))[(c
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         &
                                                                                                                                         0x3f
                                                                                                                                             as
                                                                                                                                             libc::c_int)
                                                                                                                                        as
                                                                                                                                        usize]
               as libc::c_uchar;
}
unsafe extern "C" fn encoder(mut in_0: *mut libc::c_uchar,
                             mut out: *mut libc::c_char) {
    *out.offset(0 as libc::c_int as isize) =
        char64((*in_0.offset(0 as libc::c_int as isize) as libc::c_int >>
                    2 as libc::c_int) as libc::c_uchar) as libc::c_char;
    *out.offset(1 as libc::c_int as isize) =
        char64(((*in_0.offset(0 as libc::c_int as isize) as libc::c_int) <<
                    4 as libc::c_int |
                    *in_0.offset(1 as libc::c_int as isize) as libc::c_int >>
                        4 as libc::c_int) as libc::c_uchar) as libc::c_char;
    *out.offset(2 as libc::c_int as isize) =
        char64(((*in_0.offset(1 as libc::c_int as isize) as libc::c_int) <<
                    2 as libc::c_int |
                    *in_0.offset(2 as libc::c_int as isize) as libc::c_int >>
                        6 as libc::c_int) as libc::c_uchar) as libc::c_char;
    *out.offset(3 as libc::c_int as isize) =
        char64(*in_0.offset(2 as libc::c_int as isize)) as libc::c_char;
}
unsafe extern "C" fn add_dns_client(mut header: *mut dns_header,
                                    mut plen: size_t,
                                    mut limit: *mut libc::c_uchar,
                                    mut l3: *mut mysockaddr, mut now: time_t,
                                    mut cacheablep: *mut libc::c_int)
 -> size_t {
    let mut maclen: libc::c_int = 0;
    let mut replace: libc::c_int = 2 as libc::c_int;
    let mut mac: [libc::c_uchar; 16] = [0; 16];
    let mut encode: [libc::c_char; 18] = [0; 18];
    maclen = find_mac(l3, mac.as_mut_ptr(), 1 as libc::c_int, now);
    if maclen == 6 as libc::c_int {
        replace = 1 as libc::c_int;
        *cacheablep = 0 as libc::c_int;
        if (*dnsmasq_daemon).options[(55 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (55 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
            print_mac(encode.as_mut_ptr(), mac.as_mut_ptr(), maclen);
        } else {
            encoder(mac.as_mut_ptr(), encode.as_mut_ptr());
            encoder(mac.as_mut_ptr().offset(3 as libc::c_int as isize),
                    encode.as_mut_ptr().offset(4 as libc::c_int as isize));
            encode[8 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_char
        }
    }
    return add_pseudoheader(header, plen, limit,
                            512 as libc::c_int as libc::c_ushort,
                            65073 as libc::c_int,
                            encode.as_mut_ptr() as *mut libc::c_uchar,
                            strlen(encode.as_mut_ptr()), 0 as libc::c_int,
                            replace);
}
unsafe extern "C" fn add_mac(mut header: *mut dns_header, mut plen: size_t,
                             mut limit: *mut libc::c_uchar,
                             mut l3: *mut mysockaddr, mut now: time_t,
                             mut cacheablep: *mut libc::c_int) -> size_t {
    let mut maclen: libc::c_int = 0;
    let mut mac: [libc::c_uchar; 16] = [0; 16];
    maclen = find_mac(l3, mac.as_mut_ptr(), 1 as libc::c_int, now);
    if maclen != 0 as libc::c_int {
        *cacheablep = 0 as libc::c_int;
        plen =
            add_pseudoheader(header, plen, limit,
                             512 as libc::c_int as libc::c_ushort,
                             65001 as libc::c_int, mac.as_mut_ptr(),
                             maclen as size_t, 0 as libc::c_int,
                             0 as libc::c_int)
    }
    return plen;
}
unsafe extern "C" fn get_addrp(mut addr: *mut mysockaddr,
                               family: libc::c_short) -> *mut libc::c_void {
    if family as libc::c_int == 10 as libc::c_int {
        return &mut (*addr).in6.sin6_addr as *mut in6_addr as
                   *mut libc::c_void
    }
    return &mut (*addr).in_0.sin_addr as *mut in_addr as *mut libc::c_void;
}
unsafe extern "C" fn calc_subnet_opt(mut opt: *mut subnet_opt,
                                     mut source: *mut mysockaddr,
                                     mut cacheablep: *mut libc::c_int)
 -> size_t {
    /* http://tools.ietf.org/html/draft-vandergaast-edns-client-subnet-02 */
    let mut len: libc::c_int = 0;
    let mut addrp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut sa_family: libc::c_int = (*source).sa.sa_family as libc::c_int;
    let mut cacheable: libc::c_int = 0 as libc::c_int;
    (*opt).source_netmask = 0 as libc::c_int as u8_0;
    (*opt).scope_netmask = 0 as libc::c_int as u8_0;
    if (*source).sa.sa_family as libc::c_int == 10 as libc::c_int &&
           !(*dnsmasq_daemon).add_subnet6.is_null() {
        (*opt).source_netmask = (*(*dnsmasq_daemon).add_subnet6).mask as u8_0;
        if (*(*dnsmasq_daemon).add_subnet6).addr_used != 0 {
            sa_family =
                (*(*dnsmasq_daemon).add_subnet6).addr.sa.sa_family as
                    libc::c_int;
            addrp =
                get_addrp(&mut (*(*dnsmasq_daemon).add_subnet6).addr,
                          sa_family as libc::c_short);
            cacheable = 1 as libc::c_int
        } else {
            addrp =
                &mut (*source).in6.sin6_addr as *mut in6_addr as
                    *mut libc::c_void
        }
    }
    if (*source).sa.sa_family as libc::c_int == 2 as libc::c_int &&
           !(*dnsmasq_daemon).add_subnet4.is_null() {
        (*opt).source_netmask = (*(*dnsmasq_daemon).add_subnet4).mask as u8_0;
        if (*(*dnsmasq_daemon).add_subnet4).addr_used != 0 {
            sa_family =
                (*(*dnsmasq_daemon).add_subnet4).addr.sa.sa_family as
                    libc::c_int;
            addrp =
                get_addrp(&mut (*(*dnsmasq_daemon).add_subnet4).addr,
                          sa_family as libc::c_short);
            cacheable = 1 as libc::c_int
            /* Address is constant */
        } else {
            addrp =
                &mut (*source).in_0.sin_addr as *mut in_addr as
                    *mut libc::c_void
        }
    } /* No address ever supplied. */
    (*opt).family =
        __bswap_16(if sa_family == 10 as libc::c_int {
                       2 as libc::c_int
                   } else { 1 as libc::c_int } as __uint16_t);
    if !addrp.is_null() &&
           (*opt).source_netmask as libc::c_int != 0 as libc::c_int {
        len =
            ((*opt).source_netmask as libc::c_int - 1 as libc::c_int >>
                 3 as libc::c_int) + 1 as libc::c_int;
        memcpy((*opt).addr.as_mut_ptr() as *mut libc::c_void, addrp,
               len as libc::c_ulong);
        if (*opt).source_netmask as libc::c_int & 7 as libc::c_int != 0 {
            (*opt).addr[(len - 1 as libc::c_int) as usize] =
                ((*opt).addr[(len - 1 as libc::c_int) as usize] as libc::c_int
                     &
                     (0xff as libc::c_int) <<
                         8 as libc::c_int -
                             ((*opt).source_netmask as libc::c_int &
                                  7 as libc::c_int)) as u8_0
        }
    } else { cacheable = 1 as libc::c_int; len = 0 as libc::c_int }
    if !cacheablep.is_null() { *cacheablep = cacheable }
    return (len + 4 as libc::c_int) as size_t;
}
unsafe extern "C" fn add_source_addr(mut header: *mut dns_header,
                                     mut plen: size_t,
                                     mut limit: *mut libc::c_uchar,
                                     mut source: *mut mysockaddr,
                                     mut cacheable: *mut libc::c_int)
 -> size_t {
    /* http://tools.ietf.org/html/draft-vandergaast-edns-client-subnet-02 */
    let mut len: libc::c_int = 0;
    let mut opt: subnet_opt =
        subnet_opt{family: 0,
                   source_netmask: 0,
                   scope_netmask: 0,
                   addr: [0; 16],};
    len = calc_subnet_opt(&mut opt, source, cacheable) as libc::c_int;
    return add_pseudoheader(header, plen, limit,
                            512 as libc::c_int as libc::c_ushort,
                            8 as libc::c_int,
                            &mut opt as *mut subnet_opt as *mut libc::c_uchar,
                            len as size_t, 0 as libc::c_int,
                            0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn check_source(mut header: *mut dns_header,
                                      mut plen: size_t,
                                      mut pseudoheader: *mut libc::c_uchar,
                                      mut peer: *mut mysockaddr)
 -> libc::c_int {
    /* Section 9.2, Check that subnet option in reply matches. */
    let mut len: libc::c_int = 0; /* skip UDP length and RCODE */
    let mut calc_len: libc::c_int = 0; /* bad packet */
    let mut opt: subnet_opt =
        subnet_opt{family: 0,
                   source_netmask: 0,
                   scope_netmask: 0,
                   addr: [0; 16],};
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut code: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    calc_len =
        calc_subnet_opt(&mut opt, peer, 0 as *mut libc::c_int) as libc::c_int;
    p = skip_name(pseudoheader, header, plen, 10 as libc::c_int);
    if p.is_null() { return 1 as libc::c_int }
    p = p.offset(8 as libc::c_int as isize);
    let mut t_cp: *mut libc::c_uchar = p;
    rdlen =
        (*t_cp.offset(0 as libc::c_int as isize) as u16_0 as libc::c_int) <<
            8 as libc::c_int |
            *t_cp.offset(1 as libc::c_int as isize) as u16_0 as libc::c_int;
    p = p.offset(2 as libc::c_int as isize);
    if !((p.wrapping_offset_from(header as *mut libc::c_uchar) as libc::c_long
              + rdlen as libc::c_long) as size_t <= plen) {
        return 1 as libc::c_int
    }
    /* check if option there */
    i = 0 as libc::c_int;
    while (i + 4 as libc::c_int) < rdlen {
        let mut t_cp_0: *mut libc::c_uchar = p;
        code =
            (*t_cp_0.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_0.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        let mut t_cp_1: *mut libc::c_uchar = p;
        len =
            (*t_cp_1.offset(0 as libc::c_int as isize) as u16_0 as
                 libc::c_int) << 8 as libc::c_int |
                *t_cp_1.offset(1 as libc::c_int as isize) as u16_0 as
                    libc::c_int;
        p = p.offset(2 as libc::c_int as isize);
        if code == 8 as libc::c_int {
            /* make sure this doesn't mismatch. */
            opt.scope_netmask = *p.offset(3 as libc::c_int as isize);
            if len != calc_len ||
                   memcmp(p as *const libc::c_void,
                          &mut opt as *mut subnet_opt as *const libc::c_void,
                          len as libc::c_ulong) != 0 as libc::c_int {
                return 0 as libc::c_int
            }
        }
        p = p.offset(len as isize);
        i += len + 4 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Set *check_subnet if we add a client subnet option, which needs to checked 
   in the reply. Set *cacheable to zero if we add an option which the answer
   may depend on. */
#[no_mangle]
pub unsafe extern "C" fn add_edns0_config(mut header: *mut dns_header,
                                          mut plen: size_t,
                                          mut limit: *mut libc::c_uchar,
                                          mut source: *mut mysockaddr,
                                          mut now: time_t,
                                          mut check_subnet: *mut libc::c_int,
                                          mut cacheable: *mut libc::c_int)
 -> size_t {
    *check_subnet = 0 as libc::c_int;
    *cacheable = 1 as libc::c_int;
    if (*dnsmasq_daemon).options[(32 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (32 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        plen = add_mac(header, plen, limit, source, now, cacheable)
    }
    if (*dnsmasq_daemon).options[(54 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (54 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 ||
           (*dnsmasq_daemon).options[(55 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (55 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        plen = add_dns_client(header, plen, limit, source, now, cacheable)
    }
    if !(*dnsmasq_daemon).dns_client_id.is_null() {
        plen =
            add_pseudoheader(header, plen, limit,
                             512 as libc::c_int as libc::c_ushort,
                             65074 as libc::c_int,
                             (*dnsmasq_daemon).dns_client_id as
                                 *mut libc::c_uchar,
                             strlen((*dnsmasq_daemon).dns_client_id),
                             0 as libc::c_int, 1 as libc::c_int)
    }
    if (*dnsmasq_daemon).options[(41 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (41 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        plen = add_source_addr(header, plen, limit, source, cacheable);
        *check_subnet = 1 as libc::c_int
    }
    return plen;
}
