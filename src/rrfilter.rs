
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
