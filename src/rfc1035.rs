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
use std::env::args;
use std::time;

use crate::blockdata::{blockdata_alloc, blockdata_retrieve};
use crate::cache::{cache_end_insert, cache_find_by_addr, cache_find_by_name, cache_find_non_terminal, cache_get_cname_target, cache_get_name, cache_insert, cache_make_stat, cache_start_insert, log_query, next_uid, querystr, record_source};
use crate::defines::{AddressListEntry, BogusAddr, Crec, DnsHeader, DnsmasqDaemon, Doctor, InterfaceName, MxSrvRecord, NaPtr, NetAddress, PtrRecord, Server, TxtRecord};
use crate::dnsmasq_log::my_syslog;
use crate::edns0::add_pseudoheader;
use crate::ipset::add_to_ipset;
use crate::network::enumerate_interfaces;
use crate::util::{do_rfc1035_name, hostname_isequal, hostname_issubdomain, is_same_net4};

pub fn extract_name(mut header: &DnsHeader,
                    mut plen: usize,
                    mut pp: &mut String,
                    mut name: &mut String,
                    mut is_extract: bool,
                    mut extrabytes: i32)
                    -> i32 {
    let mut cp: String = name.clone();
    let mut p: String = pp.clone();
    let mut p1: String;
    let mut j: usize = 0;
    let mut l: usize = 0;
    let mut namelen: usize = 0;
    let mut hops: u32 = 0;
    let mut retvalue: i32 = 1;
    if is_extract != false { cp = String::new() }
    loop {
        let mut label_type: u32 = 0;
        if !((p.wrapping_offset_from(header) + 1) <= plen) {
            return 0;
        }
        let fresh6 = p;
        p = p.offset(1);
        l = fresh6;
        if l == 0 {
            /* label types 0x40 and 0x80 not supported */
            /* end marker */
            /* check that there are the correct no. of bytes after the name */
            if !(((if !p1.is_null() {
                p1
            } else {
                p
            }).wrapping_offset_from(header) + extrabytes) <= plen) {
                return 0;
            }
            if is_extract != false {
                if cp != name { cp = cp.offset(-1) }
                cp = 0
                /* terminate: lose final period */
            } else if cp != 0 {
                retvalue = 2
            }
            if !p1.is_null() {
                /* we jumped via compression */
                *pp = p1
            } else { *pp = p }
            return retvalue;
        }
        label_type = l as u32 & 0xc0;
        if label_type == 0xc0 {
            /* pointer */
            if !((p.wrapping_offset_from(header) + 1) <= plen) {
                return 0;
            }
            /* get offset */
            l = (l & 0x3f) << 8;
            let fresh7 = p;
            p = p.offset(1);
            l |= *fresh7;
            if p1.is_null() {
                /* first jump, save location to go back to */
                p1 = p
            } /* break malicious infinite loops */
            hops = hops.wrapping_add(1);
            if hops > 255 {
                return 0;
            }
            p = (header).offset(l)
        } else if label_type == 0 {
            /* label_type = 0 -> label. */
            namelen = namelen.wrapping_add(l.wrapping_add(1)); /* include period */
            if namelen >= 1025 {
                return 0;
            }
            if !((p.wrapping_offset_from(header) + l) <= plen) {
                return 0;
            }
            j = 0;
            while j < l {
                if is_extract != false {
                    let mut c: libc::c_uchar = *p;
                    if c != 0 && c != '.' as i32 {
                        let fresh8 = cp;
                        cp = cp.offset(1);
                        *fresh8 = c
                    } else { return 0; }
                } else {
                    let mut c1: libc::c_uchar = cp;
                    let mut c2: libc::c_uchar = *p;
                    if c1 == 0 {
                        retvalue = 2
                    } else {
                        cp = cp.offset(1);
                        if c1 >= 'A' as i32 && c1 <= 'Z' as i32 {
                            c1 = (c1 + ('a' as i32 - 'A' as i32))
                        }
                        if c2 >= 'A' as i32 && c2 <= 'Z' as i32 {
                            c2 = (c2 + ('a' as i32 - 'A' as i32))
                        }
                        if c1 != c2 {
                            retvalue = 2
                        }
                    }
                }
                j = j.wrapping_add(1);
                p = p.offset(1)
            }
            if is_extract != false {
                let fresh9 = cp;
                cp = cp.offset(1);
                *fresh9 = '.' as i32
            } else if cp != 0 && {
                let fresh10 = cp;
                cp = cp.offset(1);
                fresh10 != '.' as i32
            } {
                retvalue = 2
            }
        } else { return 0; }
    };
}

pub fn in_arpa_name_2_addr(mut namein: &mut String, mut addrp: &mut NetAddress)
                           -> i32 {
    let mut j: i32 = 0;
    let mut name: String;
    let mut cp1: String;
    let mut addr: NetAddress = addrp.clone();
    let mut lastchunk: String = String::new();
    let mut penchunk: String;

    /* turn name into a series of asciiz strings */
    /* j counts no. of labels */
    j = 1;
    for c in namein {
        if c == "." {
            penchunk = lastchunk;
            lastchunk = cp1[1..].to_string();
            cp1 = String::new();
        } else {
            cp1[0] = c;
        }
        cp1 = cp1[1..].to_string();
        *namein = namein[1..].to_string();
    }
    cp1.clear();
    if j < 3 { return 0; }
    if hostname_isequal(&lastchunk, &String::from("arpa")) && hostname_isequal(penchunk, &String::from("in-addr")) {
        /* IP v4 */
        /* address arrives as a name of the form
       www.xxx.yyy.zzz.in-addr.arpa
       some of the low order address octets might be missing
       and should be set to zero. */
        cp1 = name;
        while cp1 != penchunk {
            /* check for digits only (weeds out things like
	     50.0/24.67.28.64.in-addr.arpa which are used 
	     as CNAME targets according to RFC 2317 */
            let mut cp: &mut String = 0;
            cp = cp1;
            while cp != 0 {
                if *(*__ctype_b_loc()).offset(cp) & _ISDIGIT == 0 {
                    return 0;
                }
                cp = cp.offset(1)
            }
            *addr.offset(3) = *addr.offset(2);
            *addr.offset(2) = *addr.offset(1);
            *addr.offset(1) = *addr.offset(0);
            *addr.offset(0) = atoi(cp1);
            cp1 = cp1.offset(strlen(cp1).wrapping_add(1))
        }
        return (1) << 7;
    } else {
        if hostname_isequal(penchunk, &String::from("ip6")) != false && (hostname_isequal(lastchunk, &String::from("int")) != false || hostname_isequal(lastchunk, &String::from("arpa")) != false) {
            /* IP v6:
         Address arrives as 0.1.2.3.4.5.6.7.8.9.a.b.c.d.e.f.ip6.[int|arpa]
    	 or \[xfedcba9876543210fedcba9876543210/128].ip6.[int|arpa]
      
	 Note that most of these the various representations are obsolete and 
	 left-over from the many DNS-for-IPv6 wars. We support all the formats
	 that we can since there is no reason not to.
      */
            if *name.as_mut_ptr() == '\\' as i32 && *name.as_mut_ptr().offset(1) == '[' as i32 && (*name.as_mut_ptr().offset(2) == 'x' as i32 || *name.as_mut_ptr().offset(2) == 'X' as i32) {
                j = 0;
                cp1 = name.as_mut_ptr().offset(3);
                while cp1 != 0 && *(*__ctype_b_loc()).offset(cp1) & _ISXDIGIT != 0 && j < 32 {
                    let mut xdig: [libc::c_char; 2] = [0; 2];
                    xdig[0] = cp1;
                    xdig[1] = 0;
                    if j % 2 != 0 {
                        let ref mut fresh11 = *addr.offset((j / 2));
                        *fresh11 = (*fresh11 | strtol(xdig.as_mut_ptr(),
                                                      0,
                                                      16))
                    } else {
                        *addr.offset((j / 2)) = (strtol(xdig.as_mut_ptr(), 0, 16) << 4)
                    }
                    cp1 = cp1.offset(1);
                    j += 1
                }
                if cp1 == '/' as i32 && j == 32 {
                    return ((1) << 8);
                }
            } else {
                cp1 = name.as_mut_ptr();
                while cp1 != penchunk {
                    if cp1.offset(1) != 0 || *(*__ctype_b_loc()).offset(cp1

                    ) &_ISXDIGIT == 0
                    {
                        return 0;
                    }
                    j = (::std::mem::size_of::<In6Addr>()).wrapping_sub(1);
                    while j > 0 {
                        *addr.offset(j) = (*addr.offset(j) >> 4 | (*addr.offset((j - 1))) << 4);
                        j -= 1
                    }
                    *addr.offset(0) = ((*addr.offset(0) >> 4) | strtol(cp1, 0, 16) << 4);
                    libc::c_uchar;
                    cp1 = cp1.offset(strlen(cp1).wrapping_add(1))
                }
                return (1) << 8;
            }
        }
    }
    return 0;
}

pub fn skip_name(mut ansp: &mut String, mut header: &mut DnsHeader, mut plen: usize, mut extrabytes: u32) -> Option<String> {
    loop {
        let mut label_type = 0;
        if !((ansp.wrapping_offset_from(header) + 1) <= plen) {
            return None;
        }
        label_type = (ansp[0] & 0xc0);
        if label_type == 0xc0 {
            /* pointer for compression. */
            *ansp = ansp[2..].to_string(); /* reserved */
            break;
        } else if label_type == 0x80 {
            return None;
        } else if label_type == 0x40 {
            /* Extended label type */
            let mut count = 0; /* we only understand bitstrings */
            if !((ansp.wrapping_offset_from(header) + 2) <= plen) {
                return None;
            } /* Bits in bitstring */
            let fresh12 = ansp;
            *ansp = ansp[1..].to_string();
            if fresh12[0] & 0x3f != 1 {
                return None;
            }
            let fresh13 = ansp;
            *ansp = ansp[1..].to_string();
            count = fresh13[0];
            if count == 0 {
                /* count == 0 means 256 bits */
                *ansp = ansp[32..].to_string();
            } else {
                *ansp = ansp[1..].to_string();
            }
        } else {
            /* label type == 0 Bottom six bits is length */
            let fresh14 = ansp;
            *ansp = ansp[1..].to_string();
            let mut len: u32 = (fresh14[0] & 0x3f);
            if if !((ansp.wrapping_offset_from(header) + len) <= plen) {
                return None;
            } else {
                ansp = ansp.offset(len);
                1
            } == 0 {
                return None;
            }
            if len == 0 { break; }
            /* zero length label marks the end. */
        }
    }
    if !((ansp.wrapping_offset_from(header) + extrabytes) <= plen) {
        return None;
    }
    return Some(ansp.clone());
}

pub fn skip_questions(mut header: &DnsHeader, mut plen: usize) -> Option<String> {
    let mut q: u16 = 0;
    let mut ansp: String = header[1..].to_string();
    q = header.qdcount.to_be();
    while q != 0 {
        ansp = skip_name(ansp, header, plen, 4);
        if ansp.is_null() { return None; }
        ansp = ansp.offset(4);
        q -= 1
        /* class and type */
    } /* type, class, TTL */
    return Some(ansp);
}

pub fn skip_section(mut ansp: &mut String, mut count: usize mut header: &mut DnsHeader, mut plen: usize) -> Option<String> {
    let mut i = 0;
    let mut rdlen = 0;
    i = 0;
    while i < count {
        match skip_name(ansp, header, plen, 10) {
            Some(x) => ansp,
            None => return None,
        };
        *ansp = ansp[8..].to_string();

        let mut t_cp: String = ansp.clone();
        rdlen = t_cp[0] << 8 | t_cp[1];
        *ansp = ansp[2..].to_string();
        if if !((ansp.wrapping_offset_from(header) + rdlen) <= plen) {
            return None;
        } else {
            ansp = ansp.offset(rdlen);
            1
        } == 0 {
            return None;
        }
        i += 1
    }
    return Some(ansp.clone());
}

pub fn resize_packet(mut header: &mut DnsHeader, mut plen: usize,
                     mut pheader: &mut String,
                     mut hlen: usize) -> usize {
    let mut ansp_res = skip_questions(&header, plen);
    /* if packet is malformed, just return as-is. */
    if ansp_res.is_none() { return plen; }
    ansp_res = skip_section(&mut ansp.unwrap(),
                            header.ancount.to_be() + header.nscount.to_be() + header.arcount.to_be(),
                            header,
                            plen);
    if ansp_res.is_none() { return plen; }
    let mut ansp = ansp_res.unwrap();
    /* restore pseudoheader */
    if !pheader.is_empty() && header.arcount.to_be() == 0 {
        /* must use memmove, may overlap */
        pheader[0..hlen].clone_into(&mut ansp[0..hlen].as_string());
        header.arcount = 1;
        ansp = ansp[hlen..]
    }
    return ansp[header..];
}
/* is addr in the non-globally-routed IP space? */

pub fn private_net(mut addr: NetAddress,
                   mut ban_localhost: i32)
                   -> bool {
    let mut ip_addr: InAddrT = __bswap_32(addr.s_addr);
    return ip_addr & 0xff000000 == 0x7f000000 && ban_localhost != 0 || ip_addr & 0xff000000 == 0 || ip_addr & 0xff000000 == 0xa000000 || ip_addr & 0xfff00000 == 0xac100000 || ip_addr & 0xffff0000 == 0xc0a80000 || ip_addr & 0xffff0000 == 0xa9fe0000 || ip_addr & 0xffffff00 == 0xc0000200 || ip_addr & 0xffffff00 == 0xc6336400 || ip_addr & 0xffffff00 == 0xcb007100 || ip_addr & 0xffffffff == 0xffffffff;
    /* 255.255.255.255/32 (broadcast)*/
}

fn private_net6(mut a: &mut In6Addr) -> i32 {
    return ({
        let mut __a: *const In6Addr = a;
        (__a.__in6_u.__u6_addr32[0] == 0 && __a.__in6_u.__u6_addr32[1] == 0 && __a.__in6_u.__u6_addr32[2] == 0 && __a.__in6_u.__u6_addr32[3] == 0)
    }) != 0 || ({
        let mut __a: *const In6Addr = a;
        (__a.__in6_u.__u6_addr32[0] == 0 && __a.__in6_u.__u6_addr32[1] == 0 && __a.__in6_u.__u6_addr32[2] == 0 && __a.__in6_u.__u6_addr32[3] == __bswap_32(1))
    }) != 0 || ({
        let mut __a: *const In6Addr = a;
        (__a.__in6_u.__u6_addr32[0] & __bswap_32(0xffc00000) == __bswap_32(0xfe800000))
    }) != 0 || *(a).offset(0) == 0xfd || *(a).offset(0) == __bswap_32(0x20010db8);
    /* RFC 6303 4.6 */
}

pub fn do_doctor(mut p: &mut Vec<u8>, mut count: i32, mut header: &DnsHeader, mut qlen: usize,
                 mut name: &String,
                 mut doctored: bool) -> Option<Vec<u8>> {
    let mut i: i32 = 0; /* bad packet */
    let mut qtype: i32 = 0;
    let mut qclass: i32 = 0;
    let mut rdlen: i32 = 0;
    i = count;
    while i != 0 {
        if !name.is_null() && dnsmasq_daemon.options[2] != 0 {
            if extract_name(header, qlen, &mut String::from_utf8((*p).clone()).unwrap(), name, 1, 10) == 0 {
                return 0;
            }
        } else {
            p = skip_name(p, header, qlen, 10);
            if p.is_null() { return 0; }
        }
        let mut t_cp: Vec<u8> = p;
        qtype = (*t_cp.offset(0)) << 8 | *t_cp.offset(1);
        p = p.offset(2);
        let mut t_cp_0: Vec<u8> = p;
        qclass = (*t_cp_0.offset(0)) << 8 | *t_cp_0.offset(1);
        p = p.offset(2);
        /* bad packet */
        p = p.offset(4); /* ttl */
        let mut t_cp_1: Vec<u8> = p;
        rdlen = (*t_cp_1.offset(0)) << 8 | *t_cp_1.offset(1);
        p = p.offset(2);
        if qclass == 1 && qtype == 1 {
            let mut doctor: Doctor = 0;
            let mut addr: NetAddress = NetAddress:new();
            if !((p.wrapping_offset_from(header) + 4) <= qlen) {
                return None;
            }
            /* alignment */
            memcpy(&mut addr, p, 4);
            let mut current_block_28: u64;
            doctor = dnsmasq_daemon.doctors;
            while !doctor.is_null() {
                if doctor.end.s_addr == 0 {
                    if is_same_net(doctor.in_0, addr, doctor.mask) == 0 {
                        current_block_28 = 6669252993407410313;
                    } else { current_block_28 = 11636175345244025579; }
                } else if __bswap_32(doctor.in_0.s_addr) > __bswap_32(addr.s_addr) || __bswap_32(doctor.end.s_addr) < __bswap_32(addr.s_addr) {
                    current_block_28 = 6669252993407410313;
                } else { current_block_28 = 11636175345244025579; }
                match current_block_28 {
                    6669252993407410313 => { doctor = doctor.next }
                    _ => {
                        addr.s_addr &= !doctor.mask.s_addr;
                        addr.s_addr |= doctor.out.s_addr & doctor.mask.s_addr;
                        /* Since we munged the data, the server it came from is no longer authoritative */
                        header.hb3 = (header.hb3 & !(0x4))
                        u8; /* bad packet */
                        *doctored = 1;
                        memcpy(p, &mut addr, 4);
                        break;
                    }
                }
            }
        } else if qtype == 16 && !name.is_null() && daemon.options[2] != 0 {
            let mut p1: Vec<u8> = p;
            if !((p1.wrapping_offset_from(header) + rdlen) <= qlen) {
                return 0;
            }
            while (p1.wrapping_offset_from(p)) < rdlen {
                let mut i_0: u32 = 0;
                let mut len: u32 = *p1;
                let mut p2: Vec<u8> = p1;
                if p1.offset(len).wrapping_offset_from(p)
                i32 >= rdlen {
                    return 0,
                }
                /* make counted string zero-term  and sanitise */
                i_0 = 0;
                while i_0 < len {
                    if *(*__ctype_b_loc()).offset(*p2.offset(1)) & _ISPRINT == 0 {
                        break;
                    }
                    *p2 = *p2.offset(1);
                    p2 = p2.offset(1);
                    i_0 = i_0.wrapping_add(1)
                }
                *p2 = 0;
                // my_syslog(6,
                //           "reply %s is %s", name, p1);
                /* restore */
                // memmove(p1.offset(1), p1, i_0);
                p1 = p1[1..1 + i_0].to_string();
                *p1 = len;
                p1 = p1.offset(len.wrapping_add(1))
            }
        }
        if if !((p.wrapping_offset_from(header) + rdlen) <= qlen) {
            0
        } else {
            p = p.offset(rdlen);
            1
        } == 0 {
            return 0;
        }
        i -= 1
    }
    return p;
}

fn find_soa(mut header: &DnsHeader, mut qlen: usize, mut name: &String, mut doctored: bool) -> u32 {
    let mut p: Vec<u8>;
    let mut qtype: i32 = 0;
    let mut qclass: i32 = 0;
    let mut rdlen: i32 = 0;
    let mut ttl: u32 = 0;
    let mut minttl: u32 = (9223372036854775807).wrapping_mul(2).wrapping_add(1);
    let mut i: i32 = 0;
    let mut found_soa: i32 = 0;
    /* first move to NS section and find TTL from any SOA section */
    p = skip_questions(header, qlen).unwrap(); /* bad packet */
    if p.is_null() || {
        p = do_doctor(p, header.ancount.to_be() as i32, &header, qlen, name, doctored); /* bad packet */
        p.is_null()
    } {
        return 0;
    }
    i = header.nscount.to_be();
    while i != 0 {
        p = skip_name(p, header, qlen, 10);
        if p.is_null() { return 0; }
        let mut t_cp: Vec<u8> = p;
        qtype = (*t_cp.offset(0)) << 8 | *t_cp.offset(1);
        p = p[2..];
        let mut t_cp_0: Vec<u8> = p;
        qclass = (*t_cp_0.offset(0)) << 8 | *t_cp_0.offset(1);
        p = p[2..];
        let mut t_cp_1: Vec<u8> = p;
        ttl = u32::from_le_bytes([t_cp_1[0], t_cp_1[1], t_cp_1[2], t_cp_1[3]]);
        p = p[4..];
        let mut t_cp_2: Vec<u8> = p;
        rdlen = (*t_cp_2.offset(0)) << 8 | *t_cp_2.offset(1);
        p = p[2..];
        if qclass == 1 && qtype == 6 {
            found_soa = 1;
            if ttl < minttl { minttl = ttl }
            /* bad packet */
            /* MNAME */
            p = skip_name(p, header, qlen, 0);
            if p.is_null() { return 0; }
            /* RNAME */
            p = skip_name(p, header, qlen, 20); /* SERIAL REFRESH RETRY EXPIRE */
            if p.is_null() { return 0; }
            p = p[16..];
            let mut t_cp_3: Vec<u8> = p;
            ttl = u32::from_le_bytes([t_cp_3[0], t_cp_3[1], t_cp_3[2], t_cp_3[2]]);
            p = p[4..];
            /* minTTL */
            if ttl < minttl { minttl = ttl }
        } else if if !((p.wrapping_offset_from(header) + rdlen) <= qlen) {
            0
        } else {
            p = p.offset(rdlen);
            1
        } == 0 {
            return 0;
        }
        i -= 1
    }
    /* rewrite addresses in additional section too */
    if do_doctor(p, __bswap_16(header.arcount), header, qlen, 0, doctored).is_null() {
        return 0;
    }
    if found_soa == 0 { minttl = dnsmasq_daemon.neg_ttl }
    return minttl;
}
/* Note that the following code can create CNAME chains that don't point to a real record,
   either because of lack of memory, or lack of SOA records.  These are treated by the cache code as 
   expired and cleaned out that way. 
   Return 1 if we reject an address because it look like part of dns-rebinding attack. */

pub fn extract_addresses(mut header: DnsHeader,
                         mut qlen: usize,
                         mut name: &String,
                         mut now: time::Instant,
                         mut ipsets: String,
                         mut is_sign: bool,
                         mut check_rebind: bool,
                         mut no_cache_dnssec: bool,
                         mut secure: bool,
                         mut doctored: bool) -> i32 {
    let mut p: Vec<u8>;
    let mut p1: Vec<u8>;
    let mut endrr: Vec<u8>;
    let mut namep: Vec<u8>;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut qtype: i32 = 0;
    let mut qclass: i32 = 0;
    let mut aqtype: i32 = 0;
    let mut aqclass: i32 = 0;
    let mut ardlen: i32 = 0;
    let mut res: i32 = 0;
    let mut searched_soa: i32 = 0;
    let mut ttl: u32 = 0;
    let mut addr: NetAddress;
    let mut ipsets_cur: String = 0;
    cache_start_insert();
    /* find_soa is needed for dns_doctor and logging side-effects, so don't call it lazily if there are any. */
    if !dnsmasq_daemon.doctors.is_null() || dnsmasq_daemon.options[2] != 0 || dnsmasq_daemon.options[45] != 0 {
        searched_soa = 1;
        ttl = find_soa(&header, qlen, name, doctored);
        if *doctored != 0 { if secure != false { return 0; } }
    }
    /* go through the questions. */
    p = header[1..]; /* bad packet */
    let mut current_block_206: u64;
    i = __bswap_16(header.qdcount);
    while i != 0 {
        let mut found: i32 = 0;
        let mut cname_count: i32 = 10;
        let mut cpp: Crec = 0;
        let mut flags: i32 = if header.hb4 & 0xf == 3 {
            ((1)) << 10
        } else { 0 };
        let mut cttl: u32 = (9223372036854775807).wrapping_mul(2).wrapping_add(1);
        let mut attl: u32 = 0;
        namep = p;
        if extract_name(&header, qlen, &mut p, name, 1, 4) == 0 {
            return 0;
        }
        let mut t_cp: Vec<u8> = p;
        qtype = (*t_cp.offset(0)) << 8 | *t_cp.offset(1);
        p = p.offset(2);
        let mut t_cp_0: Vec<u8> = p;
        qclass = (*t_cp_0.offset(0)) << 8 | *t_cp_0.offset(1);
        p = p.offset(2);
        if !(qclass != 1) {
            /* PTRs: we chase CNAMEs here, since we have no way to 
	 represent them in the cache. */
            if qtype == 12 {
                let mut name_encoding: i32 = in_arpa_name_2_addr(name, &mut addr);
                if !(name_encoding == 0) {
                    if flags & (1) << 10 == 0 {
                        'c_14031: loop {
                            p1 = skip_questions(&header, qlen);
                            if p1.is_null() { return 0; }
                            j = 0;
                            loop {
                                if !(j < __bswap_16(&header.ancount)
                                ) {
                                    break 'c_14031;
                                }
                                let mut secflag: i32 = 0;
                                let mut tmp: Vec<u8> = namep;
                                /* bad packet */
                                /* the loop body overwrites the original name, so get it back here. */
                                if extract_name(&header, qlen, &mut tmp,
                                                &name, 1,
                                                0) == 0 || {
                                    res = extract_name(header, qlen,
                                                       &mut p1, name,
                                                       false,
                                                       10,
                                    ); /* bad packet */
                                    (res) == 0
                                } {
                                    return 0;
                                }
                                let mut t_cp_1: Vec<u8> = p1;
                                aqtype = (*t_cp_1.offset(0)) << 8 | *t_cp_1.offset(1)
                                ;
                                p1 = p1[2..];
                                let mut t_cp_2: Vec<u8> = p1;
                                aqclass = (*t_cp_2.offset(0) ) << 8 | *t_cp_2.offset(1)
                                ;
                                p1 = p1[2..];
                                let mut t_cp_3: Vec<u8> = p1;
                                attl = u32::from_le_bytes([t_cp_3[0],t_cp_3[1],t_cp_3[2], t_cp_3[3]]);
                                p1 = p1.offset(4);
                                if dnsmasq_daemon.max_ttl != 0 && attl > dnsmasq_daemon.max_ttl && is_sign == 0 {
                                    p1 = p1.offset(-(4
                                    ));
                                    let mut t_l: u32_0 = dnsmasq_daemon.max_ttl
                                    u32_0;
                                    let mut t_cp_4: Vec<u8> = p1;
                                    let fresh15 = t_cp_4;
                                    t_cp_4 = t_cp_4.offset(1);
                                    *fresh15 = (t_l >> 24)
                                    libc::c_uchar;
                                    let fresh16 = t_cp_4;
                                    t_cp_4 = t_cp_4.offset(1);
                                    *fresh16 = (t_l >> 16)
                                    libc::c_uchar;
                                    let fresh17 = t_cp_4;
                                    t_cp_4 = t_cp_4.offset(1);
                                    *fresh17 = (t_l >> 8)
                                    libc::c_uchar;
                                    *t_cp_4 = t_l;
                                    p1 = p1.offset(4
                                    )
                                }
                                let mut t_cp_5: Vec<u8> = p1;
                                ardlen = (*t_cp_5.offset(0
                                )                                       ) << 8 | *t_cp_5.offset(1
                                )
                                ;
                                p1 = p1[2..];
                                endrr = p1.offset(ardlen);
                                /* TTL of record is minimum of CNAMES and PTR */
                                if attl < cttl {
                                    cttl = attl
                                } /* looped CNAMES, we can't cache. */
                                if aqclass == 1 && res != 2 && (aqtype == 5 || aqtype == 12) {
                                    if extract_name(header, qlen, &mut p1,
                                                    name,
                                                    1,
                                                    0) == 0 {
                                        return 0;
                                    }
                                    if aqtype == 5 {
                                        let fresh18 = cname_count;
                                        cname_count = cname_count - 1;
                                        if fresh18 == 0 {
                                            return 0;
                                        }
                                        break;
                                    } else {
                                        cache_insert(name, &mut addr,
                                                     1,
                                                     now, cttl,
                                                     (name_encoding | secflag)
                                        libc::c_uint | (1
                                        libc::c_uint) << 2
                                        );
                                        found = 1
                                    }
                                }
                                p1 = endrr;
                                if !((p1.wrapping_offset_from(header
                                Vec < u8 >) + 0
                                i32) < =
                                qlen) {
                                    return 0;
                                }
                                j += 1
                            }
                        }
                    }
                    if found == 0 && dnsmasq_daemon.options[(11).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                    ).wrapping_mul(8
                    ))
                        ] & (1) << (11).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                    ).wrapping_mul(8)) == 0 {
                        if searched_soa == 0 {
                            searched_soa = 1;
                            ttl = find_soa(header, qlen, 0,
                                           doctored)
                        }
                        if ttl != 0 {
                            cache_insert(0, &mut addr,
                                         1,
                                         now, ttl,
                                         name_encoding | (1) << 2 | (1) << 5 | flags | (if secure != 0 {
                                             ((1)) << 15
                                         } else {
                                             0
                                             libc::c_uint
                                         }));
                        }
                    }
                }
            } else {
                /* everything other than PTR */
                let mut newc: Crec = 0;
                let mut addrlen: i32 = 0;
                if qtype == 1 {
                    addrlen = 4;
                    flags = (flags | (1) << 7);
                    current_block_206 = 18356193971123529525;
                } else if qtype == 28 {
                    addrlen = 16;
                    flags = (flags | (1) << 8);
                    current_block_206 = 18356193971123529525;
                } else if qtype == 33 {
                    flags = (flags | (1) << 30);
                    current_block_206 = 18356193971123529525;
                } else { current_block_206 = 1856101646708284338; }
                match current_block_206 {
                    1856101646708284338 => {}
                    _ => {
                        'c_10467: loop {
                            p1 = skip_questions(header, qlen);
                            if p1.is_null() { return 0; }
                            j = 0;
                            loop {
                                if !(j < __bswap_16(header.ancount)
                                ) {
                                    break 'c_10467;
                                }
                                let mut secflag_0: i32 = 0;
                                /* bad packet */
                                res = extract_name(header, qlen, &mut p1,
                                                   name, 0,
                                                   10,
                                ); /* bad packet */
                                if res == 0 {
                                    return 0;
                                } /* looped CNAMES */
                                let mut t_cp_6: Vec<u8> = p1; /* bad packet */
                                aqtype = (*t_cp_6.offset(0
                                )                                       ) << 8 | *t_cp_6.offset(1
                                )
                                ; /* include terminating zero */
                                p1 = p1[2..];
                                let mut t_cp_7: Vec<u8> = p1;
                                aqclass = (*t_cp_7.offset(0
                                )                                       ) << 8 | *t_cp_7.offset(1
                                )
                                ;
                                p1 = p1[2..];
                                let mut t_cp_8: Vec<u8> = p1;
                                attl = ((*t_cp_8.offset(0
                                )_0) << 24 | (*t_cp_8.offset(1
                                )
                                u32_0) << 16 | (*t_cp_8.offset(2
                                )
                                u32_0) << 8 | *t_cp_8.offset(3
                                )
                                u32_0);
                                p1 = p1.offset(4);
                                if dnsmasq_daemon.max_ttl != 0 && attl > dnsmasq_daemon.max_ttl && is_sign == 0 {
                                    p1 = p1.offset(-(4
                                    ));
                                    let mut t_l_0: u32_0 = dnsmasq_daemon.max_ttl
                                    u32_0;
                                    let mut t_cp_9: Vec<u8> = p1;
                                    let fresh19 = t_cp_9;
                                    t_cp_9 = t_cp_9.offset(1);
                                    *fresh19 = (t_l_0 >> 24)
                                    libc::c_uchar;
                                    let fresh20 = t_cp_9;
                                    t_cp_9 = t_cp_9.offset(1);
                                    *fresh20 = (t_l_0 >> 16)
                                    libc::c_uchar;
                                    let fresh21 = t_cp_9;
                                    t_cp_9 = t_cp_9.offset(1);
                                    *fresh21 = (t_l_0 >> 8)
                                    libc::c_uchar;
                                    *t_cp_9 = t_l_0;
                                    p1 = p1.offset(4
                                    )
                                }
                                let mut t_cp_10: Vec<u8> = p1;
                                ardlen = (*t_cp_10.offset(0
                                )
                                ) << 8 | *t_cp_10.offset(1
                                )
                                u16;
                                p1 = p1[2..];
                                endrr = p1.offset(ardlen);
                                if aqclass == 1 && res != 2 && (aqtype == 5 || aqtype == qtype) {
                                    if aqtype == 5 {
                                        let fresh22 = cname_count;
                                        cname_count = cname_count - 1;
                                        if fresh22 == 0 {
                                            return 0;
                                        }
                                        newc = cache_insert(name,
                                                            0,
                                                            1,
                                                            now, attl,
                                                            (1
                                        libc::c_uint) << 11 | (1
                                        libc::c_uint) << 3 | secflag_0
                                        libc::c_uint);
                                        if !newc.is_null() {
                                            newc.addr.cname.target.cache = 0;
                                            newc.addr.cname.is_name_ptr = 0;
                                            if !cpp.is_null() {
                                                next_uid(newc);
                                                (cpp).addr.cname.target.cache = newc;
                                                (cpp).addr.cname.uid = newc.uid
                                            }
                                        }
                                        cpp = newc;
                                        if attl < cttl { cttl = attl }
                                        namep = p1;
                                        if extract_name(header, qlen,
                                                        &mut p1, name,
                                                        1,
                                                        0) == 0 {
                                            return 0;
                                        }
                                        break;
                                    } else if flags & (1) << 10 == 0 {
                                        found = 1;
                                        if flags & (1) << 30 != 0 {
                                            let mut tmp_0: Vec<u8> = namep;
                                            if !((p1.wrapping_offset_from(header
                                            Vec < u8 >) + 6
                                            i32)      size_t <= qlen) {
                                                return 0;
                                            }
                                            let mut t_cp_11: Vec<u8> = p1;
                                            addr.srv.priority = ((*t_cp_11.offset(0
                                            )
                                            ) << 8 | *t_cp_11.offset(1
                                            )
                                            )
                                            u16;
                                            p1 = p1.offset(2
                                            );
                                            let mut t_cp_12: Vec<u8> = p1;
                                            addr.srv.weight = ((*t_cp_12.offset(0
                                            )
                                            ) << 8 | *t_cp_12.offset(1
                                            )
                                            )
                                            u16;
                                            p1 = p1.offset(2
                                            );
                                            let mut t_cp_13: Vec<u8> = p1;
                                            addr.srv.srvport = ((*t_cp_13.offset(0
                                            )
                                            ) << 8 | *t_cp_13.offset(1
                                            )
                                            )
                                            u16;
                                            p1 = p1.offset(2
                                            );
                                            if extract_name(header, qlen,
                                                            &mut p1, name,
                                                            1,
                                                            0,
                                            ) == 0 {
                                                return 0;
                                            }
                                            addr.srv.targetlen = strlen(name).wrapping_add(1);
                                            addr.srv.target = blockdata_alloc(name, addr.srv.targetlen);
                                            if addr.srv.target.is_null() {
                                                return 0;
                                            }
                                            /* we overwrote the original name, so get it back here. */
                                            if extract_name(header, qlen,
                                                            &mut tmp_0,
                                                            name,
                                                            1,
                                                            0,
                                            ) == 0 {
                                                return 0;
                                            }
                                        } else {
                                            /* copy address into aligned storage */
                                            if !((p1.wrapping_offset_from(header
                                            Vec < u8 >) + addrlen
                                            i32)      size_t <= qlen) {
                                                return 0;
                                            } /* bad packet */
                                            memcpy(&mut addr
                                            Vec < u8 >,
                                            p1 *const libc
                                            ::c_void,
                                            addrlen );
                                            /* check for returned address in private space */
                                            if check_rebind != 0 {
                                                if flags & (1) << 7 != 0 && private_net(addr.addr4,
                                                                                        (dnsmasq_daemon.options
                                                [(25
                                                ).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                ))
                                                usize]
                                                &(1
                                                libc::c_uint) << (25
                                                ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                )) == 0)
                                                ) != 0
                                                {
                                                    return 1;
                                                }
                                                /* Block IPv4-mapped IPv6 addresses in private IPv4 address space */
                                                if flags & (1) << 8 != 0 {
                                                    if ({
                                                        let mut __a: *const In6Addr = &mut addr.addr6 & mut In6Addr *const In6Addr;
                                                        (__a.__in6_u.__u6_addr32
                                                        [0

                                                        usize] == 0

                                                        libc::c_uint && __a.__in6_u.__u6_addr32
                                                        [1

                                                        usize] == 0

                                                        libc::c_uint && __a.__in6_u.__u6_addr32
                                                        [2

                                                        usize] == __bswap_32(0xffff

                                                        u32))
                                                    }) != 0 {
                                                        let mut v4: NetAddress = NetAddress {
                                                            s_addr: 0,
                                                        };
                                                        v4.s_addr = *(&mut addr.addr6 & mut In6Addr *const u32).offset(3

                                                        );
                                                        if private_net(v4,
                                                                       (dnsmasq_daemon.options
                                                        [(25
                                                        ).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                        ))
                                                        usize]
                                                        &(1
                                                        libc::c_uint) << (25
                                                        ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                        )) == 0)
                                                        ) != 0
                                                        {
                                                            return 1;
                                                        }
                                                    }
                                                    /* Check for link-local (LL) and site-local (ULA) IPv6 addresses */
                                                    if ({
                                                        let mut __a: *const In6Addr = &mut addr.addr6 & mut In6Addr *const In6Addr;
                                                        (__a.__in6_u.__u6_addr32
                                                        [0

                                                        usize]
                                                        &__bswap_32(0xffc00000
                                                        libc::c_uint) == __bswap_32(0xfe800000
                                                        libc::c_uint))
                                                    }) != 0 || ({
                                                        let mut __a: *const In6Addr = &mut addr.addr6 & mut In6Addr *const In6Addr;
                                                        (__a.__in6_u.__u6_addr32
                                                        [0

                                                        usize]
                                                        &__bswap_32(0xffc00000
                                                        libc::c_uint) == __bswap_32(0xfec00000
                                                        libc::c_uint))
                                                    }) != 0 {
                                                        return 1;
                                                    }
                                                    /* Check for the IPv6 loopback address (::1) when
                 option rebind-localhost-ok is NOT set */
                                                    if dnsmasq_daemon.options
                                                    [(25
                                                    ).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                    ))
                                                    usize]
                                                    &(1
                                                    libc::c_uint) << (25
                                                    ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                    )) == 0 && ({
                                                        let mut __a: *const In6Addr = &mut addr.addr6 & mut In6Addr *const In6Addr;
                                                        (__a.__in6_u.__u6_addr32
                                                        [0

                                                        usize] == 0

                                                        libc::c_uint && __a.__in6_u.__u6_addr32
                                                        [1

                                                        usize] == 0

                                                        libc::c_uint && __a.__in6_u.__u6_addr32
                                                        [2

                                                        usize] == 0

                                                        libc::c_uint && __a.__in6_u.__u6_addr32
                                                        [3

                                                        usize] == __bswap_32(1

                                                        u32))
                                                    }) != 0
                                                    {
                                                        return 1;
                                                    }
                                                }
                                            }
                                            if !ipsets.is_null() && flags & ((1
                                            libc::c_uint) << 7 | (1
                                            libc::c_uint) << 8
                                            ) != 0
                                            {
                                                ipsets_cur = ipsets;
                                                while !ipsets_cur.is_null() {
                                                    // log_query(flags                   libc::c_uint
                                                    //               &
                                                    //               ((1                         libc::c_uint)
                                                    //                    <<
                                                    //                    7
                                                    //                                                   
                                                    //                    |
                                                    //                    (1
                                                    //                                                     libc::c_uint)
                                                    //                        <<
                                                    //                        8
                                                    //                                                           )
                                                    //               |
                                                    //               (1                        libc::c_uint)
                                                    //                   <<
                                                    //                   26
                                                    //                                                 ,
                                                    //           name,
                                                    //           &mut addr,
                                                    //           *ipsets_cur);
                                                    let fresh23 = ipsets_cur;
                                                    ipsets_cur = ipsets_cur.offset(1);
                                                    add_to_ipset(*fresh23,
                                                                 &mut addr,
                                                                 flags,
                                                                 0,
                                                    );
                                                }
                                            }
                                        }
                                        newc = cache_insert(name, &mut addr,
                                                            1,
                                                            now, attl,
                                                            flags
                                        libc::c_uint | (1
                                        libc::c_uint) << 3 | secflag_0
                                        libc::c_uint);
                                        if !newc.is_null() && !cpp.is_null() {
                                            next_uid(newc);
                                            (cpp).addr.cname.target.cache = newc;
                                            (cpp).addr.cname.uid = newc.uid
                                        }
                                        cpp = 0
                                    }
                                }
                                p1 = endrr;
                                if !((p1.wrapping_offset_from(header
                                Vec < u8 >) + 0
                                i32) < =
                                qlen) {
                                    return 0;
                                }
                                j += 1
                            }
                        }
                        if found == 0 && dnsmasq_daemon.options[(11).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                        ))
                            ] & (1) << (11).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                        ).wrapping_mul(8
                        )) == 0 {
                            if searched_soa == 0 {
                                searched_soa = 1;
                                ttl = find_soa(header, qlen,
                                               0, doctored)
                            }
                            /* If there's no SOA to get the TTL from, but there is a CNAME 
		 pointing at this, inherit its TTL */
                            if ttl != 0 || !cpp.is_null() {
                                newc = cache_insert(name, 0,
                                                    1
                                u16, now,
                                if ttl != 0 {
                                    ttl
                                } else { cttl },
                                (1) << 3 | (1) << 5 | flags | (if secure != 0 {
                                    ((1
                                    libc::c_uint)) << 15
                                } else {
                                    0
                                    libc::c_uint
                                }));
                                if !newc.is_null() && !cpp.is_null() {
                                    next_uid(newc);
                                    (cpp).addr.cname.target.cache = newc;
                                    (cpp).addr.cname.uid = newc.uid
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
    if header.hb3 & 0x2 == 0 && header.hb4 & 0x10 == 0 && header.hb4 & 0x80 != 0 && no_cache_dnssec == 0 {
        cache_end_insert();
    }
    return 0;
}
/* If the packet holds exactly one query
   return F_IPV4 or F_IPV6  and leave the name from the query in name */

pub fn extract_request(mut header: DnsHeader,
                       mut qlen: usize,
                       mut name: &mut String,
                       mut typep: &mut u16)
                       -> libc::c_uint {
    let mut p: Vec<u8> = header.offset(1)
    Vec < u8 >; /* must be exactly one query. */
    let mut qtype: i32 = 0; /* bad packet */
    let mut qclass: i32 = 0;
    if !typep.is_null() { *typep = 0 }
    if __bswap_16(header.qdcount) != 1 || (header.hb3 & 0x78) >> 3 != 0 {
        return 0;
    }
    if extract_name(header, qlen, &mut p, name, 1,
                    4) == 0 {
        return 0;
    }
    let mut t_cp: Vec<u8> = p;
    qtype = (*t_cp.offset(0)) << 8 | *t_cp.offset(1);
    p = p.offset(2);
    let mut t_cp_0: Vec<u8> = p;
    qclass = (*t_cp_0.offset(0)) << 8 | *t_cp_0.offset(1);
    p = p.offset(2);
    if !typep.is_null() { *typep = qtype }
    if qclass == 1 {
        if qtype == 1 {
            return (1) << 7;
        }
        if qtype == 28 {
            return (1) << 8;
        }
        if qtype == 255 {
            return (1) << 7 | (1) << 8;
        }
    }
    /* F_DNSSECOK as agument to search_servers() inhibits forwarding
     to servers for domains without a trust anchor. This make the
     behaviour for DS and DNSKEY queries we forward the same
     as for DS and DNSKEY queries we originate. */
    if qtype == 43 || qtype == 48 {
        return (1) << 15;
    }
    return (1) << 19;
}

pub fn setup_reply(mut header: DnsHeader,
                   mut qlen: usize,
                   mut addrp: &mut NetAddress,
                   mut flags: u32,
                   mut ttl: u32) -> size_t {
    let mut p: Vec<u8> = 0;
    p = skip_questions(header, qlen);
    if p.is_null() { return 0; }
    /* clear authoritative and truncated flags, set QR flag */
    header.hb3 = (header.hb3 & !(0x4 | 0x2) | 0x80) as u8;
    /* clear AD flag, set RA flag */
    header.hb4 = (header.hb4 & !(0x20) | 0x80)
    u8; /* no answers unless changed below */
    header.nscount = __bswap_16(0); /* empty domain */
    header.arcount = __bswap_16(0);
    header.ancount = __bswap_16(0);
    if flags == (1) << 20 {
        header.hb4 = (header.hb4 & !(0xf) | 0) as u8
    } else if flags == (1) << 10 {
        header.hb4 = (header.hb4 & !(0xf) | 3) as u8
    } else if flags == (1) << 28 {
        let mut a: NetAddress = NetAddress { addr4: NetAddress { s_addr: 0 } };
        a.log.rcode = 2;
        // log_query((1) << 13 |
        //               (1) << 29,
        //           "error"                 &mut String, &mut a, 0 );
        header.hb4 = (header.hb4 & !(0xf) | 2) as u8
    } else if flags & ((1) << 7 | (1) << 8) != 0 {
        if flags & (1) << 7 != 0 {
            /* we know the address */
            header.hb4 = (header.hb4 & !(0xf) | 0) as u8;
            header.ancount = __bswap_16(1);
            header.hb3 = (header.hb3 | 0x4) as u8;
            add_resource_record(header, 0,
                                0,
                                ::std::mem::size_of::<DnsHeader>(),
                                &mut p, ttl,
                                0,
                                1,
                                1,
                                "4", addrp);
        }
        if flags & (1) << 8 != 0 {
            header.hb4 = (header.hb4 & !(0xf) | 0) as u8;
            header.ancount = __bswap_16((__bswap_16(header.ancount) + 1));
            header.hb3 = (header.hb3 | 0x4) as u8;
            add_resource_record(header, 0,
                                0,
                                ::std::mem::size_of::<DnsHeader>(),
                                &mut p, ttl,
                                0,
                                28,
                                1,
                                "6", addrp);
        }
    } else {
        /* nowhere to forward to */
        let mut a_0: NetAddress = NetAddress { addr4: NetAddress { s_addr: 0 } };
        a_0.log.rcode = 5;
        // log_query((1) << 13 |
        //               (1) << 29,
        //           "error"                 &mut String, &mut a_0, 0 );
        header.hb4 = (header.hb4 & !(0xf) | 5) as u8
    }
    return p.wrapping_offset_from(header);
}
/* check if name matches local names ie from /etc/hosts or DHCP or local mx names. */

pub fn check_for_local_domain(mut name: &mut String,
                              mut now: time::Instant)
                              -> i32 {
    let mut mx: MxSrvRecord = 0;
    let mut txt: TxtRecord = 0;
    let mut intr: InterfaceName = ;
    let mut ptr: PtrRecord = 0;
    let mut naptr: NaPtr = 0;
    naptr = dnsmasq_daemon.naptr;
    while !naptr.is_null() {
        if hostname_issubdomain(name, naptr.name) != 0 {
            return 1;
        }
        naptr = naptr.next
    }
    mx = dnsmasq_daemon.mxnames;
    while !mx.is_null() {
        if hostname_issubdomain(name, mx.name) != 0 {
            return 1;
        }
        mx = mx.next
    }
    txt = dnsmasq_daemon.txt;
    while !txt.is_null() {
        if hostname_issubdomain(name, txt.name) != 0 {
            return 1;
        }
        txt = txt.next
    }
    intr = dnsmasq_daemon.int_names;
    while !intr.is_null() {
        if hostname_issubdomain(name, intr.name) != 0 {
            return 1;
        }
        intr = intr.next
    }
    ptr = dnsmasq_daemon.ptr;
    while !ptr.is_null() {
        if hostname_issubdomain(name, ptr.name) != 0 {
            return 1;
        }
        ptr = ptr.next
    }
    if cache_find_non_terminal(name, now) != 0 { return 1; }
    return 0;
}
/* Is the packet a reply with the answer address equal to addr?
   If so mung is into an NXDOMAIN reply and also put that information
   in the cache. */

pub fn check_for_bogus_wildcard(mut header: DnsHeader,
                                mut qlen: usize,
                                mut name: &mut String,
                                mut baddr: BogusAddr,
                                mut now: time::Instant)
                                -> i32 {
    let mut p: Vec<u8> = 0;
    let mut i: i32 = 0;
    let mut qtype: i32 = 0;
    let mut qclass: i32 = 0;
    let mut rdlen: i32 = 0;
    let mut ttl: u32 = 0;
    let mut baddrp: BogusAddr = 0;
    /* skip over questions */
    p = skip_questions(header, qlen); /* bad packet */
    if p.is_null() { return 0; } /* bad packet */
    i = __bswap_16(header.ancount);
    while i != 0 {
        if extract_name(header, qlen, &mut p, name, 1,
                        10) == 0 {
            return 0;
        }
        let mut t_cp: Vec<u8> = p;
        qtype = (*t_cp.offset(0)) << 8 | *t_cp.offset(1);
        p = p.offset(2);
        let mut t_cp_0: Vec<u8> = p;
        qclass = (*t_cp_0.offset(0)) << 8 | *t_cp_0.offset(1);
        p = p.offset(2);
        let mut t_cp_1: Vec<u8> = p;
        ttl = ((*t_cp_1.offset(0)
        _0) << 24 | (*t_cp_1.offset(1)
        _0) << 16 | (*t_cp_1.offset(2)
        _0) << 8 | *t_cp_1.offset(3)
        _0)          libc::c_ulong;
        p = p.offset(4);
        let mut t_cp_2: Vec<u8> = p;
        rdlen = (*t_cp_2.offset(0)) << 8 | *t_cp_2.offset(1);
        p = p.offset(2);
        if qclass == 1 && qtype == 1 {
            if !((p.wrapping_offset_from(header)
            i32 + 4)               size_t <= qlen) {
                return 0;
            }
            baddrp = baddr;
            while !baddrp.is_null() {
                if memcmp(&mut baddrp.addr
                p,
                4) == 0
                {
                    /* Found a bogus address. Insert that info here, since there no SOA record
		   to get the ttl from in the normal processing */
                    cache_start_insert();
                    cache_insert(name, 0,
                                 1, now, ttl,
                                 (1) << 7 | (1) << 3 | (1) << 5 | (1) << 10);
                    cache_end_insert();
                    return 1;
                }
                baddrp = baddrp.next
            }
        }
        if if !((p.wrapping_offset_from(header)
        i32 + rdlen) < = qlen)
        {
            0
        } else {
            p = p.offset(rdlen);
            1
        } == 0
        {
            return 0;
        }
        i -= 1
    }
    return 0;
}

pub fn check_for_ignored_address(mut header: DnsHeader,
                                 mut qlen: usize,
                                 mut baddr: BogusAddr)
                                 -> i32 {
    let mut p: Vec<u8> = 0;
    let mut i: i32 = 0;
    let mut qtype: i32 = 0;
    let mut qclass: i32 = 0;
    let mut rdlen: i32 = 0;
    let mut baddrp: BogusAddr = 0;
    /* skip over questions */
    p = skip_questions(header, qlen); /* bad packet */
    if p.is_null() { return 0; } /* bad packet */
    i = __bswap_16(header.ancount); /* TTL */
    while i != 0 {
        p = skip_name(p, header, qlen,
                      10); /* make ap point to 1st unamed argument */
        if p.is_null() { return 0; }
        let mut t_cp: Vec<u8> = p;
        qtype = (*t_cp.offset(0)) << 8 | *t_cp.offset(1);
        p = p.offset(2);
        let mut t_cp_0: Vec<u8> = p;
        qclass = (*t_cp_0.offset(0)) << 8 | *t_cp_0.offset(1);
        p = p.offset(2);
        p = p.offset(4);
        let mut t_cp_1: Vec<u8> = p;
        rdlen = (*t_cp_1.offset(0)) << 8 | *t_cp_1.offset(1);
        p = p.offset(2);
        if qclass == 1 && qtype == 1 {
            if !((p.wrapping_offset_from(header)
            i32 + 4)               size_t <= qlen) {
                return 0;
            }
            baddrp = baddr;
            while !baddrp.is_null() {
                if memcmp(&mut baddrp.addr
                p,
                4) == 0
                {
                    return 1;
                }
                baddrp = baddrp.next
            }
        }
        if if !((p.wrapping_offset_from(header)
        i32 + rdlen) < = qlen)
        {
            0
        } else {
            p = p.offset(rdlen);
            1
        } == 0
        {
            return 0;
        }
        i -= 1
    }
    return 0;
}

pub fn add_resource_record(mut header: &DnsHeader,
                           mut limit: &mut String,
                           mut truncp: i32,
                           mut nameoffset: isize,
                           mut pp: &mut String,
                           mut ttl: u32,
                           mut offset: isize,
                           mut type_0: u16,
                           mut class: u16,
                           mut format: &String) -> i32 {
    let mut current_block: u64;
    let mut sav: Vec<u8> = Vec::new();
    let mut p: Vec<u8> = Vec::new();
    let mut j: i32 = 0;
    let mut usval: u16 = 0;
    let mut lval: i32 = 0;
    let mut sval: &mut String = 0;
    ap = args.clone();
    if !(!truncp.is_null() && *truncp != 0) {
        if nameoffset > 0 {
            if !limit.is_null() && p.offset(2) > limit {
                current_block = 16696653877814833746;
            } else {
                let mut t_s: u16 = (nameoffset | 0xc000);
                let mut t_cp: Vec<u8> = p;
                let fresh24 = t_cp;
                t_cp = t_cp.offset(1);
                *fresh24 = (t_s >> 8);
                *t_cp = t_s;
                p = p.offset(2);
                current_block = 4488286894823169796;
            }
        } else {
            let mut name: &mut String = ap.arg::<&mut String>();
            if !name.is_null() && {
                p = do_rfc1035_name(p, name, limit);
                p.is_null()
            } {
                current_block = 16696653877814833746;
            } else if nameoffset < 0 {
                if !limit.is_null() && p.offset(2) > limit {
                    current_block = 16696653877814833746;
                } else {
                    let mut t_s_0: u16 = (-nameoffset | 0xc000);
                    let mut t_cp_0: Vec<u8> = p;
                    let fresh25 = t_cp_0;
                    t_cp_0 = t_cp_0.offset(1);
                    *fresh25 = (t_s_0 >> 8)
                    libc::c_uchar;
                    *t_cp_0 = t_s_0;
                    p = p.offset(2);
                    current_block = 4488286894823169796;
                }
            } else if !limit.is_null() && p.offset(1) > limit {
                current_block = 16696653877814833746;
            } else {
                let fresh26 = p;
                p = p.offset(1);
                *fresh26 = 0;
                current_block = 4488286894823169796;
            }
        }
        match current_block {
            16696653877814833746 => {}
            _ => /* type (2) + class (2) + ttl (4) + rdlen (2) */
                {
                    if !(!limit.is_null() && p.offset(10) > limit) {
                        let mut t_s_1: u16 = type_0;
                        let mut t_cp_1: Vec<u8> = p;
                        let fresh27 = t_cp_1;
                        t_cp_1 = t_cp_1.offset(1);
                        *fresh27 = (t_s_1 >> 8)
                        libc::c_uchar;
                        *t_cp_1 = t_s_1;
                        p = p.offset(2);
                        let mut t_s_2: u16 = class;
                        let mut t_cp_2: Vec<u8> = p;
                        let fresh28 = t_cp_2;
                        t_cp_2 = t_cp_2.offset(1);
                        *fresh28 = (t_s_2 >> 8)
                        libc::c_uchar;
                        *t_cp_2 = t_s_2;
                        p = p.offset(2);
                        let mut t_l: u32_0 = ttl_0;
                        let mut t_cp_3: Vec<u8> = p;
                        let fresh29 = t_cp_3;
                        t_cp_3 = t_cp_3.offset(1);
                        *fresh29 = (t_l >> 24);
                        let fresh30 = t_cp_3;
                        t_cp_3 = t_cp_3.offset(1);
                        *fresh30 = (t_l >> 16);
                        let fresh31 = t_cp_3;
                        t_cp_3 = t_cp_3.offset(1);
                        *fresh31 = (t_l >> 8);
                        *t_cp_3 = t_l;
                        p = p.offset(4);
                        /* TTL */
                        sav = p; /* Save pointer to RDLength field */
                        let mut t_s_3: u16 = 0;
                        let mut t_cp_4: Vec<u8> = p;
                        let fresh32 = t_cp_4;
                        t_cp_4 = t_cp_4.offset(1);
                        *fresh32 = (t_s_3 >> 8)
                        libc::c_uchar;
                        *t_cp_4 = t_s_3;
                        p = p.offset(2);
                        loop /* Placeholder RDLength */ {
                            if !(*format != 0) {
                                current_block = 6665878751423064961;
                                break;
                            }
                            match *format {
                                54 => {
                                    if !limit.is_null() && p.offset(16) > limit {
                                        current_block = 16696653877814833746;
                                        break;
                                    }
                                    sval = ap.arg::<&mut String>();
                                    memcpy(p,
                                           sval,
                                           16);
                                    p = p.offset(16)
                                }
                                52 => {
                                    if !limit.is_null() && p.offset(4) > limit {
                                        current_block = 16696653877814833746;
                                        break;
                                    }
                                    sval = ap.arg::<&mut String>();
                                    memcpy(p,
                                           sval,
                                           4);
                                    p = p.offset(4)
                                }
                                98 => {
                                    if !limit.is_null() && p.offset(1) > limit {
                                        current_block = 16696653877814833746;
                                        break;
                                    }
                                    usval = ap.arg::<>();
                                    let fresh33 = p;
                                    p = p.offset(1);
                                    *fresh33 = usval
                                }
                                115 => {
                                    if !limit.is_null() && p.offset(2) > limit {
                                        current_block = 16696653877814833746;
                                        break;
                                    }
                                    usval = ap.arg::<>();
                                    let mut t_s_4: u16 = usval;
                                    let mut t_cp_5: Vec<u8> = p;
                                    let fresh34 = t_cp_5;
                                    t_cp_5 = t_cp_5.offset(1);
                                    *fresh34 = (t_s_4 >> 8);
                                    *t_cp_5 = t_s_4;
                                    p = p.offset(2)
                                }
                                108 => {
                                    if !limit.is_null() && p.offset(4) > limit {
                                        current_block = 16696653877814833746;
                                        break;
                                    }
                                    lval = ap.arg::<i32>();
                                    let mut t_l_0: u32_0 = lval_0;
                                    let mut t_cp_6: Vec<u8> = p;
                                    let fresh35 = t_cp_6;
                                    t_cp_6 = t_cp_6.offset(1);
                                    *fresh35 = (t_l_0 >> 24)
                                    libc::c_uchar;
                                    let fresh36 = t_cp_6;
                                    t_cp_6 = t_cp_6.offset(1);
                                    *fresh36 = (t_l_0 >> 16)
                                    libc::c_uchar;
                                    let fresh37 = t_cp_6;
                                    t_cp_6 = t_cp_6.offset(1);
                                    *fresh37 = (t_l_0 >> 8)
                                    libc::c_uchar;
                                    *t_cp_6 = t_l_0;
                                    p = p.offset(4)
                                }
                                100 => {
                                    /* get domain-name answer arg and store it in RDATA field */
                                    if !offset.is_null() {
                                        *offset = p.wrapping_offset_from(header
                                        Vec < u8 >)
                                    }
                                    p = do_rfc1035_name(p,
                                                        ap.arg::<&mut String>(),
                                                        limit);
                                    if p.is_null() {
                                        current_block = 16696653877814833746;
                                        break;
                                    }
                                    if !limit.is_null() && p.offset(1) > limit {
                                        current_block = 16696653877814833746;
                                        break;
                                    }
                                    let fresh38 = p;
                                    p = p.offset(1);
                                    *fresh38 = 0
                                }
                                116 => {
                                    usval = ap.arg::<>();
                                    if !limit.is_null() && p.offset(usval) > limit {
                                        current_block = 16696653877814833746;
                                        break;
                                    }
                                    sval = ap.arg::<&mut String>();
                                    if usval != 0 {
                                        memcpy(p,
                                               sval,
                                               usval);
                                    }
                                    p = p.offset(usval)
                                }
                                122 => {
                                    sval = ap.arg::<&mut String>();
                                    usval = if !sval.is_null() {
                                        strlen(sval)
                                    } else {
                                        0
                                    };
                                    if usval > 255 {
                                        usval = 255
                                    }
                                    if !limit.is_null() && p.offset((usval + 1)
                                    ) > limit {
                                        current_block = 16696653877814833746; break; }
                                    let fresh39 = p;
                                    p = p.offset(1);
                                    *fresh39 = usval;
                                    memcpy(p,
                                           sval,
                                           usval);
                                    p = p.offset(usval)
                                }
                                _ => {}
                            }
                            format = format.offset(1)
                        }
                        match current_block {
                            16696653877814833746 => {}
                            _ => /* clean up variable argument pointer */
                            /* Now, store real RDLength. sav already checked against limit. */
                                {
                                    j = (p.wrapping_offset_from(sav) - 2);
                                    let mut t_s_5: u16 = j;
                                    let mut t_cp_7: Vec<u8> = sav;
                                    let fresh40 = t_cp_7;
                                    t_cp_7 = t_cp_7.offset(1);
                                    *fresh40 = (t_s_5 >> 8)
                                    libc::c_uchar;
                                    *t_cp_7 = t_s_5;
                                    sav = sav.offset(2);
                                    *pp = p;
                                    return 1;
                                }
                        }
                    }
                }
        }
    }
    if !truncp.is_null() { *truncp = 1 }
    return 0;
}

fn crec_ttl(mut crecp: Crec, mut now: time::Instant)
            -> libc::c_ulong {
    /* Return 0 ttl for DHCP entries, which might change
     before the lease expires, unless configured otherwise. */
    if crecp.flags & (1) << 4 != 0 {
        let mut conf_ttl: i32 = if dnsmasq_daemon.use_dhcp_ttl != 0 {
            dnsmasq_daemon.dhcp_ttl
        } else { dnsmasq_daemon.local_ttl };
        /* Apply ceiling of actual lease length to configured TTL. */
        if crecp.flags & (1) << 0 == 0 && crecp.ttd - now < conf_ttl {
            return (crecp.ttd - now);
        }
        return conf_ttl;
    }
    /* Immortal entries other than DHCP are local, and hold TTL in TTD field. */
    if crecp.flags & (1) << 0 != 0 {
        return crecp.ttd;
    }
    /* Return the Max TTL value if it is lower than the actual TTL */
    if dnsmasq_daemon.max_ttl == 0 || ((crecp.ttd - now)) < dnsmasq_daemon.max_ttl {
        return (crecp.ttd - now);
    } else { return dnsmasq_daemon.max_ttl; };
}

fn cache_validated(mut crecp: *const Crec) -> i32 {
    return (dnsmasq_daemon.options;
    [(45).wrapping_div((::std::mem::size_of::<libc::c_uint>()
    ).wrapping_mul(8
    ))
    ] & (1) << (45).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
    ).wrapping_mul(8
    )) != 0 && crecp.flags & (1) << 15 == 0);
}
/* return zero if we can't answer from cache, or packet size if we can */

pub fn answer_request(mut header: DnsHeader,
                      mut limit: &mut String,
                      mut qlen: usize,
                      mut local_addr: NetAddress,
                      mut local_netmask: NetAddress,
                      mut now: time::Instant,
                      mut ad_reqd: i32,
                      mut do_bit: i32,
                      mut have_pseudoheader: i32)
                      -> size_t {
    let mut name: &mut String = dnsmasq_daemon.namebuff;
    let mut p: Vec<u8> = 0;
    let mut ansp: Vec<u8> = 0;
    let mut qtype: u32 = 0;
    let mut qclass: u32 = 0;
    let mut addr: NetAddress = NetAddress { addr4: NetAddress { s_addr: 0 } };
    let mut nameoffset: i32 = 0;
    let mut flag: u16 = 0;
    let mut q: i32 = 0;
    let mut ans: i32 = 0;
    let mut anscount: i32 = 0;
    let mut addncount: i32 = 0;
    let mut dryrun: i32 = 0;
    let mut crecp: Crec = 0;
    let mut nxdomain: i32 = 0;
    let mut notimp: i32 = 0;
    let mut auth: i32 = 1;
    let mut trunc: i32 = 0;
    let mut sec_data: i32 = 1;
    let mut rec: MxSrvRecord = 0;
    let mut len: usize = 0;
    let mut rd_bit: i32 = header.hb3 & 0x1;
    /* never answer queries with RD unset, to avoid cache snooping. */
    if __bswap_16(header.ancount) != 0 || __bswap_16(header.nscount) != 0 || __bswap_16(header.qdcount) == 0 || (header.hb3 & 0x78) >> 3 != 0 {
        return 0;
    }
    /* Don't return AD set if checking disabled. */
    if header.hb4 & 0x10 != 0 {
        sec_data = 0
    }
    /* If there is an  additional data section then it will be overwritten by
     partial replies, so we have to do a dry run to see if we can answer
     the query. */
    if __bswap_16(header.arcount) != 0 {
        dryrun = 1
    } /* bad packet */
    rec = dnsmasq_daemon.mxnames;
    while !rec.is_null() {
        rec.offset = 0;
        rec = rec.next
    }
    loop /* determine end of question section (we put answers there) */ {
        ansp = skip_questions(header, qlen);
        if ansp.is_null() { return 0; }
        /* now process each question, answers go in RRs after the question */
        p = header.offset(1)
        Vec < u8 >; /* catch loops */
        q = __bswap_16(header.qdcount);
        while q != 0 {
            let mut count: i32 = 255;
            /* save pointer to name for copying into answers */
            nameoffset = p.wrapping_offset_from(header)
            i32;
            /* now extract name as .-concatenated string into name */
            if extract_name(header, qlen, &mut p, name, 1,
                            4) == 0 {
                return 0;
            } /* bad packet */
            let mut t_cp: Vec<u8> = p; /* have we answered this question */
            qtype = ((*t_cp.offset(0)) << 8 | *t_cp.offset(1));
            p = p.offset(2);
            let mut t_cp_0: Vec<u8> = p;
            qclass = ((*t_cp_0.offset(0)) << 8 | *t_cp_0.offset(1));
            p = p.offset(2);
            ans = 0;
            loop {
                count -= 1;
                if !(count != 0 && {
                    crecp = cache_find_by_name(0, name, now,
                                               (1) << 11);
                    !crecp.is_null()
                }) {
                    break;
                }
                let mut cname_target: &mut String = cache_get_cname_target(crecp);
                /* If the client asked for DNSSEC  don't use cached data. */
                if crecp.flags & ((1) << 6 | (1) << 4 | (1) << 13) != 0 || rd_bit != 0 && (do_bit == 0 || cache_validated(crecp) != 0) {
                    if crecp.flags & (1) << 13 != 0 || qtype == 5 {
                        ans = 1
                    } /* give up if any cached CNAME in chain can't be used for DNSSEC reasons. */
                    if crecp.flags & (1) << 15 == 0 {
                        sec_data = 0
                    }
                    if dryrun == 0 {
                        // log_query(crecp.flags, name, 0 ,
                        //           record_source(crecp.uid));
                        if add_resource_record(header, limit,
                                               &mut trunc,
                                               nameoffset,
                                               &mut ansp,
                                               crec_ttl(crecp, now),
                                               &mut nameoffset,
                                               5libc
                        ::c_ushort,
                        1libc
                        ::c_ushort,
                        "d",
                        cname_target) != 0
                        {
                            anscount += 1
                        }
                    }
                } else { return 0; }
                strcpy(name, cname_target);
            }
            if qtype == 16 || qtype == 255 {
                let mut t: TxtRecord = 0;
                t = dnsmasq_daemon.txt;
                while !t.is_null() {
                    if t.class == qclass && hostname_isequal(name, t.name) != 0 {
                        ans = 1;
                        sec_data = 0;
                        if dryrun == 0 {
                            let mut ttl: u32 = dnsmasq_daemon.local_ttl;
                            let mut ok: i32 = 1;
                            /* Dynamically generate stat record */
                            if t.stat != 0 {
                                ttl = 0;
                                if cache_make_stat(t) == 0 {
                                    ok = 0
                                }
                            }
                            if ok != 0 {
                                // log_query((1) <<
                                //               13 |
                                //               (1) <<
                                //                   17, name,
                                //           0 ,
                                //           "<TXT>"                                         *const libc::c_char                                        &mut String);
                                if add_resource_record(header, limit,
                                                       &mut trunc,
                                                       nameoffset,
                                                       &mut ansp,
                                                       ttl,
                                                       0,
                                                       16,
                                                       t.class,
                                                       "t" *const libc
                                ::c_char & mut String,
                                t.len,
                                t.txt) != 0
                                {
                                    anscount += 1
                                }
                            }
                        }
                    }
                    t = t.next
                }
            }
            if qclass == 3 {
                /* don't forward *.bind and *.server chaos queries - always reply with NOTIMP */
                if hostname_issubdomain("bind" *const libc
                ::c_char & mut String, name) != 0 || hostname_issubdomain("server" *const libc
                ::c_char & mut String, name) != 0
                {
                    if ans == 0 {
                        notimp = 1;
                        auth = 0;
                        if dryrun == 0 {
                            addr.log.rcode = 4;
                            // log_query((1) << 13
                            //               |
                            //               (1) <<
                            //                   29, name,
                            //           &mut addr, 0 );
                        }
                        ans = 1;
                        sec_data = 0
                    }
                }
            }
            if qclass == 1 {
                let mut t_0: TxtRecord = 0;
                t_0 = dnsmasq_daemon.rr;
                while !t_0.is_null() {
                    if (t_0.class == qtype || qtype == 255) && hostname_isequal(name, t_0.name) != 0 {
                        ans = 1;
                        sec_data = 0;
                        if dryrun == 0 {
                            // log_query((1) << 13
                            //               |
                            //               (1) <<
                            //                   17, name,
                            //           0 ,
                            //           querystr(0 ,
                            //                    t_0.class));
                            if add_resource_record(header, limit,
                                                   &mut trunc,
                                                   nameoffset,
                                                   &mut ansp,
                                                   dnsmasq_daemon.local_ttl,
                                                   0,
                                                   t_0.class,
                                                   1,
                                                   "t" *const libc
                            ::c_char & mut String,
                            t_0.len,
                            t_0.txt) != 0
                            {
                                anscount += 1
                            }
                        }
                    }
                    t_0 = t_0.next
                }
                if qtype == 12 || qtype == 255 {
                    /* see if it's w.z.y.z.in-addr.arpa format */
                    let mut is_arpa: i32 = in_arpa_name_2_addr(name, &mut addr);
                    let mut ptr: PtrRecord = 0;
                    let mut intr: InterfaceName = ;
                    ptr = dnsmasq_daemon.ptr;
                    while !ptr.is_null() {
                        if hostname_isequal(name, ptr.name) != 0 {
                            break;
                        }
                        ptr = ptr.next
                    }
                    if is_arpa == (1) << 7 {
                        intr = dnsmasq_daemon.int_names;
                        while !intr.is_null() {
                            let mut addrlist: AddressListEntry = 0;
                            addrlist = intr.addresses;
                            while !addrlist.is_null() {
                                if addrlist.flags & 2 == 0 && addr.addr4.s_addr == addrlist.addr.addr4.s_addr {
                                    break;
                                }
                                addrlist = addrlist.next
                            }
                            if !addrlist.is_null() { break; }
                            while !intr.next.is_null() && strcmp(intr.intr,
                                                                 (*intr.next).intr) == 0 {
                                intr = intr.next
                            }
                            intr = intr.next
                        }
                    } else if is_arpa == (1) << 8 {
                        intr = dnsmasq_daemon.int_names;
                        while !intr.is_null() {
                            let mut addrlist_0: AddressListEntry = 0;
                            addrlist_0 = intr.addresses;
                            while !addrlist_0.is_null() {
                                if addrlist_0.flags & 2 != 0 && ({
                                    let mut __a: *const In6Addr = &mut addr.addr6 & mut In6Addr *const In6Addr;
                                    let mut __b: *const In6Addr = &mut addrlist_0.addr.addr6 *const In6Addr;
                                    (__a.__in6_u.__u6_addr32
                                    [0

                                    usize] == __b.__in6_u.__u6_addr32
                                    [0

                                    usize]
                                    &&__a.__in6_u.__u6_addr32
                                    [1

                                    usize] == __b.__in6_u.__u6_addr32
                                    [1

                                    usize]
                                    &&__a.__in6_u.__u6_addr32
                                    [2

                                    usize] == __b.__in6_u.__u6_addr32
                                    [2

                                    usize]
                                    &&__a.__in6_u.__u6_addr32
                                    [3

                                    usize] == __b.__in6_u.__u6_addr32
                                    [3

                                    usize])
                                }) != 0 {
                                    break;
                                }
                                addrlist_0 = addrlist_0.next
                            }
                            if !addrlist_0.is_null() { break; }
                            while !intr.next.is_null() && strcmp(intr.intr,
                                                                 (*intr.next).intr) == 0 {
                                intr = intr.next
                            }
                            intr = intr.next
                        }
                    }
                    if !intr.is_null() {
                        sec_data = 0;
                        ans = 1;
                        if dryrun == 0 {
                            // log_query(is_arpa |
                            //               (1) <<
                            //                   2 |
                            //               (1) <<
                            //                   13, intr.name,
                            //           &mut addr, 0 );
                            if add_resource_record(header, limit,
                                                   &mut trunc,
                                                   nameoffset,
                                                   &mut ansp,
                                                   dnsmasq_daemon.local_ttl,
                                                   0,
                                                   12,
                                                   1,
                                                   "d" *const libc
                            ::c_char & mut String,
                            intr.name) != 0
                            {
                                anscount += 1
                            }
                        }
                    } else if !ptr.is_null() {
                        ans = 1;
                        sec_data = 0;
                        if dryrun == 0 {
                            // log_query((1) << 13
                            //               |
                            //               (1) <<
                            //                   17, name,
                            //           0 ,
                            //           "<PTR>"                                     *const libc::c_char                                    &mut String);
                            ptr = dnsmasq_daemon.ptr;
                            while !ptr.is_null() {
                                if hostname_isequal(name, ptr.name) != 0 && add_resource_record(header, limit,
                                                                                                &mut trunc,
                                                                                                nameoffset,
                                                                                                &mut ansp,
                                                                                                dnsmasq_daemon.local_ttl,
                                                                                                0,
                                                                                                12,
                                                                                                1,
                                                                                                "d" *const u8
                                *const libc
                                ::c_char & mut String,
                                ptr.ptr) != 0
                                {
                                    anscount += 1
                                }
                                ptr = ptr.next
                            }
                        }
                    } else {
                        crecp = cache_find_by_addr(0, &mut addr, now,
                                                   is_arpa);
                        if !crecp.is_null() {
                            /* Don't use cache when DNSSEC data required, unless we know that
		     the zone is unsigned, which implies that we're doing
		     validation. */
                            if crecp.flags & ((1) << 6 | (1) << 4 | (1) << 13) != 0 || rd_bit != 0 && (do_bit == 0 || cache_validated(crecp) != 0) {
                                loop /* don't answer wildcard queries with data not from /etc/hosts or dhcp leases */ {
                                    if !(qtype == 255
                                    libc::c_uint && crecp.flags & ((1) << 6 | (1) << 4) == 0) {
                                        if crecp.flags & (1) << 15 == 0 {
                                            sec_data = 0
                                        }
                                        ans = 1;
                                        if crecp.flags & (1) << 5 != 0 {
                                            auth = 0;
                                            if crecp.flags & (1) << 10 != 0 {
                                                nxdomain = 1
                                            }
                                            if dryrun == 0 {
                                                // log_query(crecp.flags &
                                                //               !((1                  libc::c_uint)
                                                //                     <<
                                                //                     3                     ),
                                                //           name, &mut addr,
                                                //           0           &mut String);
                                            }
                                        } else {
                                            if crecp.flags & ((1) << 6 | (1) << 4) == 0 {
                                                auth = 0
                                            }
                                            if dryrun == 0 {
                                                // log_query(crecp.flags &
                                                //               !((1                  libc::c_uint)
                                                //                     <<
                                                //                     3                     ),
                                                //           cache_get_name(crecp),
                                                //           &mut addr,
                                                //           record_source(crecp.uid));
                                                if add_resource_record(header,
                                                                       limit,
                                                                       &mut trunc,
                                                                       nameoffset,
                                                                       &mut ansp,
                                                                       crec_ttl(crecp,
                                                                                now),
                                                                       0,
                                                                       12,
                                                                       1,
                                                                       "d" *const u8
                                                *const libc
                                                ::c_char & mut String,
                                                cache_get_name(crecp)) != 0
                                                {
                                                    anscount += 1
                                                }
                                            }
                                        }
                                    }
                                    crecp = cache_find_by_addr(crecp, &mut addr,
                                                               now,
                                                               is_arpa
                                    libc::c_uint);
                                    if crecp.is_null() { break; }
                                }
                            }
                        } else if is_rev_synth(is_arpa, &mut addr, name) != 0 {
                            ans = 1;
                            sec_data = 0;
                            if dryrun == 0 {
                                // log_query((1) <<
                                //               13 |
                                //               (1) <<
                                //                   2 |
                                //               is_arpa, name,
                                //           &mut addr, 0 );
                                if add_resource_record(header, limit,
                                                       &mut trunc,
                                                       nameoffset,
                                                       &mut ansp,
                                                       dnsmasq_daemon.local_ttl,
                                                       0,
                                                       12,
                                                       1,
                                                       "d" *const libc
                                ::c_char & mut String,
                                name) != 0
                                {
                                    anscount += 1
                                }
                            }
                        } else if dnsmasq_daemon.options[(0
                        ).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                        ))] & (1) << (0).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                        ).wrapping_mul(8
                        )) != 0 && (is_arpa == (1) << 8 && private_net6(&mut addr.addr6) != 0 || is_arpa == (1) << 7 && private_net(addr.addr4,
                                                                                                                                    1) != 0) {
                            let mut serv: Server = 0;
                            let mut namelen: u32 = strlen(name);
                            let mut nameend: &mut String = name.offset(namelen);
                            /* see if have rev-server set */
                            serv = dnsmasq_daemon.servers;
                            while !serv.is_null() {
                                let mut domainlen: u32 = 0;
                                let mut matchstart: &mut String = 0;
                                if !(serv.flags & (8 | 2) != 8) {
                                    domainlen = strlen(serv.domain)
                                    libc::c_uint;
                                    if !(domainlen == 0 || domainlen > namelen) {
                                        matchstart = nameend.offset(-(domainlen
                                        ));
                                        if hostname_isequal(matchstart,
                                                            serv.domain) != 0 && (namelen == domainlen || *matchstart.offset(-(1
                                        )) == '.' as i32) {
                                            break;
                                        }
                                    }
                                }
                                serv = serv.next
                            }
                            /* if no configured server, not in cache, enabled and private IPV4 address, return NXDOMAIN */
                            if serv.is_null() {
                                ans = 1;
                                sec_data = 0;
                                nxdomain = 1;
                                if dryrun == 0 {
                                    // log_query((1) <<
                                    //               13 |
                                    //               (1) <<
                                    //                   2 |
                                    //               is_arpa |
                                    //               (1) <<
                                    //                   5 |
                                    //               (1) <<
                                    //                   10, name,
                                    //           &mut addr,
                                    //           0 );
                                }
                            }
                        }
                    }
                }
                flag = ((1) << 7)
                u16;
                while flag != 0 {
                    let mut type_0: u16 = if flag == (1) << 8 {
                        28
                    } else { 1 };
                    let mut intr_0: InterfaceName = ;
                    if !(qtype != type_0 && qtype != 255) {
                        /* interface name stuff */
                        intr_0 = dnsmasq_daemon.int_names;
                        while !intr_0.is_null() {
                            if hostname_isequal(name, intr_0.name) != 0 {
                                break;
                            }
                            intr_0 = intr_0.next
                        }
                        if !intr_0.is_null() {
                            let mut addrlist_1: AddressListEntry = 0;
                            let mut gotit: i32 = 0;
                            let mut localise: i32 = 0;
                            enumerate_interfaces(0);
                            /* See if a putative address is on the network from which we received
		     the query, is so we'll filter other answers. */
                            if local_addr.s_addr != 0 && dnsmasq_daemon.options[(18
                            ).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                            ))
                                ] & (1) << (18).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                            ).wrapping_mul(8
                            )) != 0 && type_0 == 1 {
                                intr_0 = dnsmasq_daemon.int_names;
                                while !intr_0.is_null() {
                                    if hostname_isequal(name, intr_0.name) != 0 {
                                        addrlist_1 = intr_0.addresses;
                                        while !addrlist_1.is_null() {
                                            if addrlist_1.flags & 2 == 0 && is_same_net4(addrlist_1.addr.addr4,
                                                                                         local_addr,
                                                                                         local_netmask) != 0 {
                                                localise = 1;
                                                break;
                                            } else {
                                                addrlist_1 = addrlist_1.next
                                            }
                                        }
                                    }
                                    intr_0 = intr_0.next
                                }
                            }
                            intr_0 = dnsmasq_daemon.int_names;
                            while !intr_0.is_null() {
                                if hostname_isequal(name, intr_0.name) != 0 {
                                    addrlist_1 = intr_0.addresses;
                                    while !addrlist_1.is_null() {
                                        if (if addrlist_1.flags & 2 != 0 {
                                            28
                                        } else { 1 }) == type_0 {
                                            if !(localise != 0 && is_same_net4(addrlist_1.addr.addr4,
                                                                               local_addr,
                                                                               local_netmask) == 0) {
                                                if !(addrlist_1.flags & 4 != 0) {
                                                    ans = 1;
                                                    sec_data = 0;
                                                    if dryrun == 0 {
                                                        gotit = 1;
                                                        // log_query((1                    libc::c_uint)
                                                        //               <<
                                                        //               3                       
                                                        //               |
                                                        //               (1                        libc::c_uint)
                                                        //                   <<
                                                        //                   13
                                                        //                                                 
                                                        //               |
                                                        //               flag                       libc::c_uint,
                                                        //           name,
                                                        //           &mut addrlist_1.addr,
                                                        //           0                   &mut String);
                                                        if add_resource_record(header,
                                                                               limit,
                                                                               &mut trunc,
                                                                               nameoffset,
                                                                               &mut ansp,
                                                                               dnsmasq_daemon.local_ttl,
                                                                               0,
                                                                               type_0,
                                                                               1,
                                                                               if type_0 == 1 {
                                                                                   "4" * const u8
                                                                                   * const libc
                                                                                   ::c_char,
                                                                               } else {
                                                                                   "6" *const u8
                                                                                   *const libc
                                                                                   ::c_char
                                                                               } & mut String,
                                                        &mut addrlist_1.addr
                                                        ) != 0
                                                        {
                                                            anscount += 1
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        addrlist_1 = addrlist_1.next
                                    }
                                }
                                intr_0 = intr_0.next
                            }
                            if dryrun == 0 && gotit == 0 {
                                // log_query((1) <<
                                //               3 |
                                //               (1) <<
                                //                   13 |
                                //               flag |
                                //               (1) <<
                                //                   5, name,
                                //           0 ,
                                //           0 );
                            }
                        } else {
                            crecp = cache_find_by_name(0, name, now,
                                                       flag | (if dryrun != 0 {
                                                           ((1
                                                           libc::c_uint)) << 25
                                                       } else {
                                                           0
                                                           libc::c_uint
                                                       }));
                            if !crecp.is_null() {
                                let mut localise_0: i32 = 0;
                                /* See if a putative address is on the network from which we received
		     the query, is so we'll filter other answers. */
                                if local_addr.s_addr != 0 && dnsmasq_daemon.options[(18
                                ).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                ))
                                    ] & (1) << (18
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                ).wrapping_mul(8
                                )) != 0 && flag == (1) << 7
                                {
                                    let mut save: Crec = crecp;
                                    loop {
                                        if crecp.flags & (1) << 6 != 0 && is_same_net4(crecp.addr.addr4,
                                                                                       local_addr,
                                                                                       local_netmask) != 0 {
                                            localise_0 = 1;
                                            break;
                                        } else {
                                            crecp = cache_find_by_name(crecp,
                                                                       name, now,
                                                                       flag
                                            libc::c_uint);
                                            if crecp.is_null() { break; }
                                        }
                                    }
                                    crecp = save
                                }
                                /* If the client asked for DNSSEC  don't use cached data. */
                                if crecp.flags & ((1) << 6 | (1) << 4 | (1) << 13) != 0 || rd_bit != 0 && (do_bit == 0 || cache_validated(crecp) != 0) {
                                    /* don't answer wildcard queries with data not from /etc/hosts
			   or DHCP leases */
                                    while !(qtype == 255
                                    libc::c_uint && crecp.flags & ((1) << 6 | (1) << 4 | (1) << 13
                                    ) == 0) {
                                        if crecp.flags & (1) << 15 == 0 {
                                            sec_data = 0
                                        }
                                        if crecp.flags & (1) << 5 != 0 {
                                            ans = 1;
                                            auth = 0;
                                            if crecp.flags & (1) << 10 != 0 {
                                                nxdomain = 1
                                            }
                                            if dryrun == 0 {
                                                // log_query(crecp.flags,
                                                //           name,
                                                //           0 ,
                                                //           0           &mut String);
                                            }
                                        } else if !(localise_0 != 0 && crecp.flags & (1
                                        libc::c_uint) << 6 != 0 && is_same_net4(crecp.addr.addr4,
                                                                                 local_addr,
                                                                                 local_netmask) == 0) {
                                            if crecp.flags & ((1) << 6 | (1) << 4) == 0 {
                                                auth = 0
                                            }
                                            ans = 1;
                                            if dryrun == 0 {
                                                // log_query(crecp.flags &
                                                //               !((1                  libc::c_uint)
                                                //                     <<
                                                //                     2                     ),
                                                //           name,
                                                //           &mut crecp.addr,
                                                //           record_source(crecp.uid));
                                                if add_resource_record(header,
                                                                       limit,
                                                                       &mut trunc,
                                                                       nameoffset,
                                                                       &mut ansp,
                                                                       crec_ttl(crecp,
                                                                                now),
                                                                       0,
                                                                       type_0,
                                                                       1,
                                                                       if type_0 == 1 {
                                                                           "4" * const u8
                                                                           * const libc
                                                                           ::c_char,
                                                                       } else {
                                                                           "6" *const u8
                                                                           *const libc
                                                                           ::c_char
                                                                       } & mut String,
                                                &mut crecp.addr
                                                ) != 0
                                                {
                                                    anscount += 1
                                                }
                                            }
                                        }
                                        crecp = cache_find_by_name(crecp, name,
                                                                   now,
                                                                   flag
                                        libc::c_uint);
                                        if crecp.is_null() { break; }
                                    }
                                }
                            } else if is_name_synthetic(flag,
                                                        name, &mut addr) != 0 {
                                ans = 1;
                                sec_data = 0;
                                if dryrun == 0 {
                                    // log_query((1) <<
                                    //               3 |
                                    //               (1) <<
                                    //                   13 |
                                    //               flag, name,
                                    //           &mut addr,
                                    //           0 );
                                    if add_resource_record(header, limit,
                                                           &mut trunc,
                                                           nameoffset,
                                                           &mut ansp,
                                                           dnsmasq_daemon.local_ttl,
                                                           0,
                                                           type_0,
                                                           1,
                                                           if type_0 == 1 {
                                                               "4" * const u8
                                                               * const libc
                                                               ::c_char,
                                                           } else {
                                                               "6" *const u8
                                                               *const libc
                                                               ::c_char
                                                           } & mut String,
                                    &mut addr        ) != 0
                                    {
                                        anscount += 1
                                    }
                                }
                            }
                        }
                    }
                    flag = if flag == (1) << 7 {
                        ((1)) << 8
                    } else { 0 }
                    u16
                }
                if qtype == 15 || qtype == 255 {
                    let mut found: i32 = 0;
                    rec = dnsmasq_daemon.mxnames;
                    while !rec.is_null() {
                        if rec.issrv == 0 && hostname_isequal(name, rec.name) != 0 {
                            found = 1;
                            ans = found;
                            sec_data = 0;
                            if dryrun == 0 {
                                let mut offset: i32 = 0;
                                // log_query((1) <<
                                //               13 |
                                //               (1) <<
                                //                   17, name,
                                //           0 ,
                                //           "<MX>"                                         *const libc::c_char                                        &mut String);
                                if add_resource_record(header, limit,
                                                       &mut trunc,
                                                       nameoffset,
                                                       &mut ansp,
                                                       dnsmasq_daemon.local_ttl,
                                                       &mut offset,
                                                       15,
                                                       1,
                                                       "sd" *const libc
                                ::c_char & mut String,
                                rec.weight,
                                rec.target) != 0
                                {
                                    anscount += 1;
                                    if !rec.target.is_null() {
                                        rec.offset = offset
                                    }
                                }
                            }
                        }
                        rec = rec.next
                    }
                    if found == 0 && (dnsmasq_daemon.options[(3).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                    ))
                        ] & (1) << (3).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                    ).wrapping_mul(8
                    )) != 0 || dnsmasq_daemon.options[(10
                    ).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                    ))
                        ] & (1) << (10).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                    ).wrapping_mul(8)) != 0) && !cache_find_by_name(0, name, now,
                                                                    (1) << 6 | (1) << 4 | (1) << 25,
                    ).is_null() {
                        ans = 1;
                        sec_data = 0;
                        if dryrun == 0 {
                            // log_query((1) << 13
                            //               |
                            //               (1) <<
                            //                   17, name,
                            //           0 ,
                            //           "<MX>"                                     *const libc::c_char                                    &mut String);
                            if add_resource_record(header, limit,
                                                   &mut trunc,
                                                   nameoffset,
                                                   &mut ansp,
                                                   dnsmasq_daemon.local_ttl,
                                                   0,
                                                   15,
                                                   1,
                                                   "sd" *const libc
                            ::c_char & mut String,
                            1,
                            if dnsmasq_daemon.options
                            [(3
                            ).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                            ))
                            usize]
                            &(1) << (3
                            ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                            )) != 0
                            {
                                name
                            } else {
                                dnsmasq_daemon.mxtarget
                            }) != 0
                            {
                                anscount += 1
                            }
                        }
                    }
                }
                if qtype == 33 || qtype == 255 {
                    let mut found_0: i32 = 0;
                    let mut move_0: MxSrvRecord = 0;
                    let mut up: &mut MxSrvRecord = &mut dnsmasq_daemon.mxnames;
                    rec = dnsmasq_daemon.mxnames;
                    while !rec.is_null() {
                        if rec.issrv != 0 && hostname_isequal(name, rec.name) != 0 {
                            ans = 1;
                            found_0 = ans;
                            sec_data = 0;
                            if dryrun == 0 {
                                let mut offset_0: i32 = 0;
                                // log_query((1) <<
                                //               13 |
                                //               (1) <<
                                //                   17, name,
                                //           0 ,
                                //           "<SRV>"                                         *const libc::c_char                                        &mut String);
                                if add_resource_record(header, limit,
                                                       &mut trunc,
                                                       nameoffset,
                                                       &mut ansp,
                                                       dnsmasq_daemon.local_ttl,
                                                       &mut offset_0,
                                                       33,
                                                       1,
                                                       "sssd" *const u8
                                *const libc
                                ::c_char & mut String,
                                rec.priority,
                                rec.weight,
                                rec.srvport,
                                rec.target) != 0
                                {
                                    anscount += 1;
                                    if !rec.target.is_null() {
                                        rec.offset = offset_0
                                    }
                                }
                            }
                            /* If we are returning local answers depending on network,
			       filter here. */
                            /* unlink first SRV record found */
                            if move_0.is_null() {
                                move_0 = rec;
                                *up = rec.next
                            } else { up = &mut rec.next }
                        } else { up = &mut rec.next }
                        rec = rec.next
                    }
                    /* put first SRV record back at the end. */
                    if !move_0.is_null() {
                        *up = move_0;
                        move_0.next = 0
                    }
                    if found_0 == 0 {
                        crecp = cache_find_by_name(0, name, now,
                                                   (1) << 30 | (if dryrun != 0 {
                                                       ((1)) << 25
                                                   } else {
                                                       0
                                                       libc::c_uint
                                                   }));
                        if !crecp.is_null() && rd_bit != 0 && (do_bit == 0 || dnsmasq_daemon.options[(45
                        ).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                        ))
                            ] & (1) << (45).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                        ).wrapping_mul(8
                        )) != 0 && crecp.flags & (1) << 15 == 0) {
                            if crecp.flags & (1) << 15 == 0 {
                                sec_data = 0
                            }
                            auth = 0;
                            ans = 1;
                            found_0 = ans;
                            loop {
                                if crecp.flags & (1) << 5 != 0 {
                                    if crecp.flags & (1) << 10 != 0 {
                                        nxdomain = 1
                                    }
                                    if dryrun == 0 {
                                        // log_query(crecp.flags, name,
                                        //           0 ,
                                        //           0 );
                                    }
                                } else if dryrun == 0 {
                                    let mut target: &mut String = blockdata_retrieve(crecp.addr.srv.target,
                                                                                     crecp.addr.srv.targetlen,
                                                                                     0
                                    Vec < u8 >)
                                    ;
                                    // log_query(crecp.flags, name,
                                    //           0 ,
                                    //           0 );
                                    if add_resource_record(header, limit,
                                                           &mut trunc,
                                                           nameoffset,
                                                           &mut ansp,
                                                           crec_ttl(crecp,
                                                                    now),
                                                           0,
                                                           33,
                                                           1,
                                                           "sssd" *const u8
                                    *const libc
                                    ::c_char & mut String,
                                    crecp.addr.srv.priority,
                                    crecp.addr.srv.weight,
                                    crecp.addr.srv.srvport,
                                    target) != 0
                                    {
                                        anscount += 1
                                    }
                                }
                                crecp = cache_find_by_name(crecp, name, now,
                                                           (1) << 30);
                                if crecp.is_null() { break; }
                            }
                        }
                    }
                    if found_0 == 0 && dnsmasq_daemon.options[(1).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                    ).wrapping_mul(8
                    ))
                        ] & (1) << (1).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                    ).wrapping_mul(8)) != 0 && (qtype == 33 || qtype == 255 && !strchr(name, '_' as i32).is_null()) {
                        ans = 1;
                        sec_data = 0;
                        if dryrun == 0 {
                            // log_query((1) << 13
                            //               |
                            //               (1) <<
                            //                   5, name,
                            //           0 ,
                            //           0 );
                        }
                    }
                }
                if qtype == 35 || qtype == 255 {
                    let mut na: NaPtr = 0;
                    na = dnsmasq_daemon.naptr;
                    while !na.is_null() {
                        if hostname_isequal(name, na.name) != 0 {
                            ans = 1;
                            sec_data = 0;
                            if dryrun == 0 {
                                // log_query((1) <<
                                //               13 |
                                //               (1) <<
                                //                   17, name,
                                //           0 ,
                                //           "<NAPTR>"                                         *const libc::c_char                                        &mut String);
                                if add_resource_record(header, limit,
                                                       &mut trunc,
                                                       nameoffset,
                                                       &mut ansp,
                                                       dnsmasq_daemon.local_ttl,
                                                       0,
                                                       35,
                                                       1,
                                                       "sszzzd" *const u8
                                *const libc
                                ::c_char & mut String,
                                na.order,
                                na.pref,
                                na.flags,
                                na.services,
                                na.regexp,
                                na.replace) != 0
                                {
                                    anscount += 1
                                }
                            }
                        }
                        na = na.next
                    }
                }
                if qtype == 253 {
                    ans = 1;
                    nxdomain = 1;
                    sec_data = 0
                }
                if qtype == 6 && dnsmasq_daemon.options
                [(1
                libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                ).wrapping_mul(8
                ))
                ] &(1) << (1).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                ).wrapping_mul(8
                )) != 0
                {
                    ans = 1;
                    sec_data = 0;
                    if dryrun == 0 {
                        // log_query((1) << 13 |
                        //               (1) << 5,
                        //           name, &mut addr, 0 );
                    }
                }
            }
            if ans == 0 { return 0; }
            q -= 1
        }
        if !(dryrun != 0) { break; }
        dryrun = 0
    }
    /* create an additional data section, for stuff in SRV and MX record replies. */
    rec = dnsmasq_daemon.mxnames;
    while !rec.is_null() {
        if rec.offset != 0 {
            /* squash dupes */
            let mut tmp: MxSrvRecord = 0;
            tmp = rec.next;
            while !tmp.is_null() {
                if tmp.offset != 0 && hostname_isequal(rec.target, tmp.target) != 0 {
                    tmp.offset = 0
                }
                tmp = tmp.next
            }
            crecp = 0;
            loop {
                crecp = cache_find_by_name(crecp, rec.target, now,
                                           (1) << 7 | (1) << 8);
                if crecp.is_null() { break; }
                let mut type_1: i32 = if crecp.flags & (1) << 7 != 0 {
                    1
                } else { 28 };
                if crecp.flags & (1) << 5 != 0 {
                    continue;
                }
                if add_resource_record(header, limit, 0,
                                       rec.offset,
                                       &mut ansp,
                                       crec_ttl(crecp, now),
                                       0,
                                       type_1,
                                       1,
                                       if crecp.flags & (1) << 7 != 0 {
                                           "4"
                                       } else {
                                           "6"
                                       },
                                       &mut crecp.addr) != 0 {
                    addncount += 1
                }
            }
        }
        rec = rec.next
    }
    /* done all questions, set up header and return length of result */
    /* clear authoritative and truncated flags, set QR flag */
    header.hb3 = (header.hb3 & !(0x4 | 0x2) | 0x80) as u8;
    /* set RA flag */
    header.hb4 = (header.hb4 | 0x80) as u8;
    /* authoritative - only hosts and DHCP derived names. */
    if auth != 0 {
        header.hb3 = (header.hb3 | 0x4) as u8
    }
    /* truncation */
    if trunc != 0 {
        header.hb3 = (header.hb3 | 0x2) as u8
    } /* no error */
    if nxdomain != 0 {
        header.hb4 = (header.hb4 & !(0xf) | 3) as u8
    } else if notimp != 0 {
        header.hb4 = (header.hb4 & !(0xf) | 4) as u8
    } else {
        header.hb4 = (header.hb4 & !(0xf) | 0) as u8
    }
    header.ancount = __bswap_16(anscount);
    header.nscount = __bswap_16(0);
    header.arcount = __bswap_16(addncount);
    len = ansp.wrapping_offset_from(header)
    i32;
    /* Advertise our packet size limit in our reply */
    if have_pseudoheader != 0 {
        len = add_pseudoheader(header, len, limit,
                               dnsmasq_daemon.edns_pktsz, 0,
                               0,
                               0, do_bit,
                               0)
    }
    if ad_reqd != 0 && sec_data != 0 {
        header.hb4 = (header.hb4 | 0x20) as u8
    } else {
        header.hb4 = (header.hb4 & !(0x20)) as u8
    }
    return len;
}
