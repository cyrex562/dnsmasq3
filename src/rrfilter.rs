
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
use crate::defines::{DnsHeader};
use crate::rfc1035::skip_name;


unsafe extern "C" fn check_name(mut namep: ,
                                mut header: DnsHeader, mut plen: usize,
                                mut fixup: i32,
                                mut rrs: ,
                                mut rr_count: i32) -> i32 {
    let mut ansp: mut Vec<u8> = *namep;
    loop  {
        let mut label_type: u32 = 0;
        if !((ansp.wrapping_offset_from(header)  + 1)
                 <= plen) {
            return 0
        }
        label_type =
            (*ansp & 0xc0);
        if label_type == 0xc0 {
            /* pointer for compression. */
            let mut offset: u32 = 0;
            let mut i: i32 = 0;
            let mut p: mut Vec<u8> = 0;
            if !((ansp.wrapping_offset_from(header)                i32 + 2)               usize <= plen) {
                return 0
            }
            let fresh6 = ansp;
            ansp = ansp.offset(1);
            offset =
                ((*fresh6 & 0x3f) <<
                     8);
            let fresh7 = ansp;
            ansp = ansp.offset(1);
            offset |= *fresh7;
            p = (header).offset(offset);
            i = 0;
            while i < rr_count {
                if p < *rrs.offset(i) { break ; }
                if i & 1 != 0 {
                    offset =
                        (offset -
                             (*rrs.offset(i  )).wrapping_offset_from(*rrs.offset((i
                                                                                            -
                                                                                            1               )     ))
                                )
                }
                i += 1
            }
            /* does the pointer end up in an elided RR? */
            if i & 1 != 0 { return 0 }
            /* No, scale the pointer */
            if fixup != 0 {
                ansp =
                    ansp.offset(-(2)); /* reserved */
                let fresh8 = ansp;
                ansp = ansp.offset(1);
                *fresh8 =
                    (offset >> 8 |
                         0xc0) ;
                let fresh9 = ansp;
                ansp = ansp.offset(1);
                *fresh9 =
                    (offset & 0xff)
            }
            break ;
        } else if label_type == 0x80 {
            return 0
        } else if label_type == 0x40 {
            /* Extended label type */
            let mut count: u32 =
                0; /* we only understand bitstrings */
            if !((ansp.wrapping_offset_from(header)                i32 + 2)               usize <= plen) {
                return 0
            } /* Bits in bitstring */
            let fresh10 = ansp;
            ansp = ansp.offset(1);
            if *fresh10 & 0x3f !=
                   1 {
                return 0
            }
            let fresh11 = ansp;
            ansp = ansp.offset(1);
            count = *fresh11;
            if count == 0 {
                /* count == 0 means 256 bits */
                ansp = ansp.offset(32)
            } else {
                ansp =
                    ansp.offset((count.wrapping_sub(1     libc::c_uint) >>
                                     3                                   ).wrapping_add(1
                                                                                          libc::c_uint)
                                   )
            }
        } else {
            /* label type == 0 Bottom six bits is length */
            let fresh12 = ansp;
            ansp = ansp.offset(1);
            let mut len: u32 =
                (*fresh12 & 0x3f)              libc::c_uint;
            if if !((ansp.wrapping_offset_from(header)
                         + len)  <=
                        plen) {
                   0
               } else { ansp = ansp.offset(len); 1 }
                   == 0 {
                return 0
            }
            if len == 0 { break ; }
            /* zero length label marks the end. */
        }
    }
    *namep = ansp;
    return 1;
}
/* Go through RRs and check or fixup the domain names contained within */
unsafe extern "C" fn check_rrs(mut p: mut Vec<u8>,
                               mut header: DnsHeader, mut plen: usize,
                               mut fixup: i32,
                               mut rrs: ,
                               mut rr_count: i32) -> i32 {
    let mut i: i32 = 0; /* TTL */
    let mut j: i32 = 0;
    let mut type_0: i32 = 0;
    let mut class: i32 = 0;
    let mut rdlen: i32 = 0;
    let mut pp: mut Vec<u8> = 0;
    i = 0;
    while i <
              __bswap_16(header.ancount) +
                  __bswap_16(header.nscount) +
                  __bswap_16(header.arcount) {
        pp = p;
        p = skip_name(p, header, plen, 10);
        if p.is_null() { return 0 }
        let mut t_cp: mut Vec<u8> = p;
        type_0 =
            (*t_cp.offset(0))
                << 8 |
                *t_cp.offset(1) ;
        p = p.offset(2);
        let mut t_cp_0: mut Vec<u8> = p;
        class =
            (*t_cp_0.offset(0) ) << 8 |
                *t_cp_0.offset(1) ;
        p = p.offset(2);
        p = p.offset(4);
        let mut t_cp_1: mut Vec<u8> = p;
        rdlen =
            (*t_cp_1.offset(0) ) << 8 |
                *t_cp_1.offset(1) ;
        p = p.offset(2);
        /* If this RR is to be elided, don't fix up its contents */
        j = 0;
        while j < rr_count {
            if *rrs.offset(j) == pp { break ; }
            j += 2
        }
        if j >= rr_count {
            /* fixup name of RR */
            if check_name(&mut pp, header, plen, fixup, rrs, rr_count) == 0 {
                return 0
            }
            if class == 1 {
                let mut d: u16 = 0 ;
                pp = p;
                d = rrfilter_desc(type_0);
                while *d !=
                          -(1) {
                    if *d != 0 {
                        pp = pp.offsetd
                    } else if check_name(&mut pp, header, plen, fixup, rrs,
                                         rr_count) == 0 {
                        return 0
                    }
                    d = d.offset(1)
                }
            }
        }
        if if !((p.wrapping_offset_from(header)               i32 + rdlen)  <= plen)
              {
               0
           } else { p = p.offset(rdlen); 1 } == 0 {
            return 0
        }
        i += 1
    }
    return 1;
}
/* mode is 0 to remove EDNS0, 1 to filter DNSSEC RRs */
#[no_mangle]
pub unsafe extern "C" fn rrfilter(mut header: DnsHeader,
                                  mut plen: usize, mut mode: i32)
                                  -> usize {
    static mut rrs:  =
        0 ;
    static mut rr_sz: i32 = 0;
    let mut p: mut Vec<u8> =
        header.offset(1);
    let mut i: i32 = 0;
    let mut rdlen: i32 = 0;
    let mut qtype: i32 = 0;
    let mut qclass: i32 = 0;
    let mut rr_found: i32 = 0;
    let mut chop_an: i32 = 0;
    let mut chop_ns: i32 = 0;
    let mut chop_ar: i32 = 0;
    if __bswap_16(header.qdcount) != 1 ||
           { p = skip_name(p, header, plen, 4); p.is_null() } {
        return plen
    }
    let mut t_cp: mut Vec<u8> = p;
    qtype =
        (*t_cp.offset(0)) <<
            8 |
            *t_cp.offset(1);
    p = p.offset(2);
    let mut t_cp_0: mut Vec<u8> = p;
    qclass =
        (*t_cp_0.offset(0)) <<
            8 |
            *t_cp_0.offset(1);
    p = p.offset(2);
    let mut current_block_36: u64;
    /* First pass, find pointers to start and end of all the records we wish to elide:
     records added for DNSSEC, unless explicitly queried for */
    rr_found = 0; /* TTL */
    chop_ns = 0;
    chop_an = 0;
    chop_ar = 0;
    i = 0;
    while i <
              __bswap_16(header.ancount) +
                  __bswap_16(header.nscount) +
                  __bswap_16(header.arcount) {
        let mut pstart: mut Vec<u8> = p;
        let mut type_0: i32 = 0;
        let mut class: i32 = 0;
        p = skip_name(p, header, plen, 10);
        if p.is_null() { return plen }
        let mut t_cp_1: mut Vec<u8> = p;
        type_0 =
            (*t_cp_1.offset(0) ) << 8 |
                *t_cp_1.offset(1) ;
        p = p.offset(2);
        let mut t_cp_2: mut Vec<u8> = p;
        class =
            (*t_cp_2.offset(0) ) << 8 |
                *t_cp_2.offset(1) ;
        p = p.offset(2);
        p = p.offset(4);
        let mut t_cp_3: mut Vec<u8> = p;
        rdlen =
            (*t_cp_3.offset(0) ) << 8 |
                *t_cp_3.offset(1) ;
        p = p.offset(2);
        if if !((p.wrapping_offset_from(header)               i32 + rdlen)  <= plen)
              {
               0
           } else { p = p.offset(rdlen); 1 } == 0 {
            return plen
        }
        /* Don't remove the answer. */
        if !(i < __bswap_16(header.ancount) &&
                 type_0 == qtype && class == qclass) {
            if mode == 0 {
                /* EDNS */
                /* EDNS mode, remove T_OPT from additional section only */
                if i <
                       __bswap_16(header.nscount) +
                           __bswap_16(header.ancount) ||
                       type_0 != 41 {
                    current_block_36 = 2979737022853876585;
                } else { current_block_36 = 10692455896603418738; }
            } else if type_0 != 47 &&
                          type_0 != 50 &&
                          type_0 != 46 {
                current_block_36 = 2979737022853876585;
            } else { current_block_36 = 10692455896603418738; }
            match current_block_36 {
                2979737022853876585 => { }
                _ => {
                    if expand_workspace(&mut rrs, &mut rr_sz,
                                        rr_found + 1) == 0 {
                        return plen
                    }
                    let fresh13 = rr_found;
                    rr_found = rr_found + 1;
                    let ref mut fresh14 = *rrs.offset(fresh13);
                    *fresh14 = pstart;
                    let fresh15 = rr_found;
                    rr_found = rr_found + 1;
                    let ref mut fresh16 = *rrs.offset(fresh15);
                    *fresh16 = p;
                    if i < __bswap_16(header.ancount) {
                        chop_an += 1
                    } else if i <
                                  __bswap_16(header.nscount)
                                      +
                                      __bswap_16(header.ancount)                                     {
                        chop_ns += 1
                    } else { chop_ar += 1 }
                }
            }
        }
        /* DNSSEC mode, remove SIGs and NSECs from all three sections. */
        i += 1
    }
    /* Nothing to do. */
    if rr_found == 0 { return plen }
    /* Second pass, look for pointers in names in the records we're keeping and make sure they don't
     point to records we're going to elide. This is theoretically possible, but unlikely. If
     it happens, we give up and leave the answer unchanged. */
    p = header.offset(1);
    /* question first */
    if check_name(&mut p, header, plen, 0, rrs, rr_found) == 0
       {
        return plen
    } /* qclass, qtype */
    p = p.offset(4);
    /* Now answers and NS */
    if check_rrs(p, header, plen, 0, rrs, rr_found) == 0 {
        return plen
    }
    /* Third pass, actually fix up pointers in the records */
    p =
        header.offset(1)      mut Vec<u8>; /* qclass, qtype */
    check_name(&mut p, header, plen, 1, rrs, rr_found);
    p = p.offset(4);
    check_rrs(p, header, plen, 1, rrs, rr_found);
    /* Fourth pass, elide records */
    p = *rrs.offset(0);
    i = 1;
    while i < rr_found {
        let mut start: mut Vec<u8> = *rrs.offset(i);
        let mut end: mut Vec<u8> =
            if i != rr_found - 1 {
                *rrs.offset((i + 1))
            } else { (header).offset(plen) };
        // memmove(p, start, end.wrapping_offset_from(start));
        p = start[0..end.wrapping_offset_from(start)].to_vec();
        p =
            p.offset(end.wrapping_offset_from(start) ));
        i += 2
    }
    plen =
        p.wrapping_offset_from(header)
            ;
    header.ancount =
        __bswap_16((__bswap_16(header.ancount) - chop_an)                 u16);
    header.nscount =
        __bswap_16((__bswap_16(header.nscount) - chop_ns)                 u16);
    header.arcount =
        __bswap_16((__bswap_16(header.arcount) - chop_ar)                 u16);
    return plen;
}
/* This is used in the DNSSEC code too, hence it's exported */
#[no_mangle]
pub unsafe extern "C" fn rrfilter_desc(mut type_0: i32)
 -> u16 {
    /* List of RRtypes which include domains in the data.
     0 -> domain
     integer -> no. of plain bytes
     -1 -> end

     zero is not a valid RRtype, so the final entry is returned for
     anything which needs no mangling.
  */
    static mut rr_desc: [u16; 73] =
        [2, 0,
         -(1), 3,
         0, -(1),
         4, 0,
         -(1), 5,
         0, -(1),
         6, 0,
         0, -(1),
         7, 0,
         -(1), 8,
         0, -(1),
         9, 0,
         -(1), 12,
         0, -(1),
         14, 0,
         0, -(1),
         15, 2,
         0, -(1),
         17, 0,
         0, -(1),
         18, 2,
         0, -(1),
         21, 2,
         0, -(1),
         24, 18,
         0, -(1),
         26, 2,
         0, 0,
         -(1), 30,
         0, -(1),
         36, 2,
         0, -(1),
         33, 6,
         0, -(1),
         39, 0,
         -(1), 0,
         -(1)];
    let mut p: u16 = rr_desc.as_mut_ptr();
    while *p != type_0 && *p != 0
          {
        loop  {
            let fresh17 = p;
            p = p.offset(1);
            if !(*fresh17 !=
                     -(1)) {
                break ;
            }
        }
    }
    return p.offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn expand_workspace(mut wkspc:
                                              ,
                                          mut szp: ,
                                          mut new: i32)
 -> i32 {
    let mut p:  = 0 as ;
    let mut old: i32 = *szp;
    if old >= new + 1 { return 1 }
    if new >= 100 { return 0 }
    new += 5;
    p =
        whine_malloc((new).wrapping_mul(::std::mem::size_of::<mut Vec<u8>>()
                                                         ))
            as ;
    if p.is_null() { return 0 }
    if old != 0 && !wkspc.is_null() {
        memcpy(p, *wkspc,
               (old).wrapping_mul(::std::mem::size_of::<mut Vec<u8>>()
                                                   ));
        freewkspc;
    }
    *wkspc = p;
    *szp = new;
    return 1;
}
