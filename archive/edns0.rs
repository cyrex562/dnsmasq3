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
use crate::arp::find_mac;
use crate::defines::{time::Instant, DnsHeader, DnsmasqDaemon, NetAddress, NetAddress};
use crate::rfc1035::{skip_name, skip_questions, skip_section};
use crate::rrfilter::rrfilter;
use crate::util::print_mac;

pub fn find_pseudoheader(
    mut header: DnsHeader,
    mut plen: usize,
    mut len: &mut usize,
    mut p: &mut Vec<u8>,
    mut is_sign: bool,
    mut is_last: bool,
) -> Vec<u8> {
    /* See if packet has an RFC2671 pseudoheader, and if so return a pointer to it.
    also return length of pseudoheader in *len and pointer to the UDP size in *p
    Finally, check to see if a packet is signed. If it is we cannot change a single bit before
    forwarding. We look for TSIG in the addition section, and TKEY queries (for GSS-TSIG) */
    let mut i: i32 = 0; /* TTL */
    let mut arcount: i32 = __bswap_16(header.arcount);
    let mut ansp: Vec<u8> = header.offset(1);
    let mut rdlen: u16 = 0;
    let mut type_0: u16 = 0;
    let mut class: u16 = 0;
    let mut ret: Vec<u8> = Vec::new();
    if !is_sign.is_null() {
        *is_sign = 0;
        if (header.hb3 & 0x78) >> 3 == 0 {
            i = __bswap_16(header.qdcount);
            while i != 0 {
                ansp = skip_name(ansp, header, plen, 4);
                if ansp.is_null() {
                    return 0;
                }
                let mut t_cp: Vec<u8> = ansp;
                type_0 = ((*t_cp.offset(0)) << 8 | *t_cp.offset(1));
                ansp = ansp.offset(2);
                let mut t_cp_0: Vec<u8> = ansp;
                class = ((*t_cp_0.offset(0)) << 8 | *t_cp_0.offset(1));
                ansp = ansp.offset(2);
                if class == 1 && type_0 == 249 {
                    *is_sign = 1
                }
                i -= 1
            }
        }
    } else {
        ansp = skip_questions(header, plen);
        if ansp.is_null() {
            return 0;
        }
    }
    if arcount == 0 {
        return 0;
    }
    ansp = skip_section(
        ansp,
        __bswap_16(header.ancount) + __bswap_16(header.nscount),
        header,
        plen,
    );
    if ansp.is_null() {
        return 0;
    }
    i = 0;
    while i < arcount {
        let mut save: Vec<u8> = 0;
        let mut start: Vec<u8> = ansp;
        ansp = skip_name(ansp, header, plen, 10);
        if ansp.is_null() {
            return 0;
        }
        let mut t_cp_1: Vec<u8> = ansp;
        type_0 = ((*t_cp_1.offset(0)) << 8 | *t_cp_1.offset(1));
        ansp = ansp.offset(2);
        save = ansp;
        let mut t_cp_2: Vec<u8> = ansp;
        class = ((*t_cp_2.offset(0)) << 8 | *t_cp_2.offset(1));
        ansp = ansp.offset(2);
        ansp = ansp.offset(4);
        let mut t_cp_3: Vec<u8> = ansp;
        rdlen = ((*t_cp_3.offset(0)) << 8 | *t_cp_3.offset(1));
        ansp = ansp.offset(2);
        if if !((ansp.wrapping_offset_from(header) + rdlen) <= plen) {
            0
        } else {
            ansp = ansp.offset(rdlen);
            1
        } == 0
        {
            return 0;
        }
        if type_0 == 41 {
            if !len.is_null() {
                *len = ansp.wrapping_offset_from(start)
            }
            if !p.is_null() {
                *p = save
            }
            if !is_last.is_null() {
                *is_last = (i == arcount - 1)
            }
            ret = start
        } else if !is_sign.is_null() && i == arcount - 1 && class == 255 && type_0 == 250 {
            *is_sign = 1
        }
        i += 1
    }
    return ret;
}
/* replace == 2 ->delete existing option only. */

pub fn add_pseudoheader(
    mut header: DnsHeader,
    mut plen: usize,
    mut limit: &mut Vec<u8>,
    mut udp_sz: u16,
    mut optno: i32,
    mut opt: &mut Vec<u8>,
    mut optlen: usize,
    mut set_do: i32,
    mut replace: i32,
) -> size_t {
    let mut lenp: Vec<u8> = 0;
    let mut datap: Vec<u8> = 0;
    let mut p: Vec<u8> = 0;
    let mut udp_len: Vec<u8> = 0;
    let mut buff: Vec<u8> = 0;
    let mut rdlen: i32 = 0;
    let mut is_sign: i32 = 0;
    let mut is_last: i32 = 0;
    let mut flags: u16 = if set_do != 0 { 0x8000 } else { 0 };
    let mut rcode: u16 = 0;
    p = find_pseudoheader(header, plen, 0, &mut udp_len, &mut is_sign, &mut is_last);
    if is_sign != 0 {
        return plen;
    }
    if !p.is_null() {
        /* Existing header */
        let mut i: i32 = 0; /* bad packet */
        let mut code: u16 = 0;
        let mut len: u16 = 0;
        p = udp_len;
        let mut t_cp: Vec<u8> = p;
        udp_sz = ((*t_cp.offset(0)) << 8 | *t_cp.offset(1));
        p = p.offset(2);
        let mut t_cp_0: Vec<u8> = p;
        rcode = ((*t_cp_0.offset(0)) << 8 | *t_cp_0.offset(1));
        p = p.offset(2);
        let mut t_cp_1: Vec<u8> = p;
        flags = ((*t_cp_1.offset(0)) << 8 | *t_cp_1.offset(1));
        p = p.offset(2);
        if set_do != 0 {
            p = p.offset(-(2));
            flags = (flags | 0x8000);
            let mut t_s: u16 = flags;
            let mut t_cp_2: Vec<u8> = p;
            let fresh6 = t_cp_2;
            t_cp_2 = t_cp_2.offset(1);
            *fresh6 = (t_s >> 8);
            *t_cp_2 = t_s;
            p = p.offset(2)
        }
        lenp = p;
        let mut t_cp_3: Vec<u8> = p;
        rdlen = (*t_cp_3.offset(0)) << 8 | *t_cp_3.offset(1);
        p = p.offset(2);
        if !((p.wrapping_offset_from(header) + rdlen) <= plen) {
            return plen;
        }
        datap = p;
        /* no option to add */
        if optno == 0 {
            return plen;
        }
        /* check if option already there */
        i = 0;
        while (i + 4) < rdlen {
            let mut t_cp_4: Vec<u8> = p;
            code = ((*t_cp_4.offset(0)) << 8 | *t_cp_4.offset(1));
            p = p.offset(2);
            let mut t_cp_5: Vec<u8> = p;
            len = ((*t_cp_5.offset(0)) << 8 | *t_cp_5.offset(1));
            p = p.offset(2);
            /* malformed option, delete the whole OPT RR and start again. */
            if i + 4 + len > rdlen {
                rdlen = 0;
                is_last = 0;
                break;
            } else if code == optno {
                if replace == 0 {
                    return plen;
                }
                /* delete option if we're to replace it. */
                p = p.offset(-(4));
                rdlen -= len + 4;

                p_copy = p.clone[len + 4..len + 4 + rdlen - i].to_string();
                p = p_copy;
                let mut t_s_0: u16 = rdlen;
                let mut t_cp_6: Vec<u8> = lenp;
                let fresh7 = t_cp_6;
                t_cp_6 = t_cp_6.offset(1);
                *fresh7 = (t_s_0 >> 8);
                *t_cp_6 = t_s_0;
                lenp = lenp.offset(2);
                lenp = lenp.offset(-(2))
            } else {
                p = p.offset(len);
                i += len + 4
            }
        }
        /* If we're going to extend the RR, it has to be the last RR in the packet */
        if is_last == 0 {
            /* First, take a copy of the options. */
            if rdlen != 0 && {
                // buff =
                //     whine_malloc(rdlen );
                // !buff.is_null()
                true
            } {
                memcpy(buff, datap, rdlen);
            }
            /* now, delete OPT RR */
            plen = rrfilter(header, plen, 0);
            /* Now, force addition of a new one */
            p = 0
        }
    }
    if p.is_null() {
        /* We are (re)adding the pseudoheader */
        p = skip_questions(header, plen);
        if p.is_null() || {
            p = skip_section(
                p,
                __bswap_16(header.ancount)
                    + __bswap_16(header.nscount)
                    + __bswap_16(header.arcount),
                header,
                plen,
            );
            p.is_null()
        } {
            // free(buff);
            return plen;
        }
        if p.offset(11) > limit {
            // free(buff);
            return plen;
            /* Too big */
        } /* empty name */
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = 0;
        let mut t_s_1: u16 = 41;
        let mut t_cp_7: Vec<u8> = p;
        let fresh9 = t_cp_7;
        t_cp_7 = t_cp_7.offset(1);
        *fresh9 = (t_s_1 >> 8);
        *t_cp_7 = t_s_1;
        p = p.offset(2);
        let mut t_s_2: u16 = udp_sz;
        let mut t_cp_8: Vec<u8> = p;
        let fresh10 = t_cp_8;
        t_cp_8 = t_cp_8.offset(1);
        *fresh10 = (t_s_2 >> 8);
        *t_cp_8 = t_s_2;
        p = p.offset(2);
        /* max packet length, 512 if not given in EDNS0 header */
        let mut t_s_3: u16 = rcode;
        let mut t_cp_9: Vec<u8> = p;
        let fresh11 = t_cp_9;
        t_cp_9 = t_cp_9.offset(1);
        *fresh11 = (t_s_3 >> 8);
        *t_cp_9 = t_s_3;
        p = p.offset(2);
        /* extended RCODE and version */
        let mut t_s_4: u16 = flags;
        let mut t_cp_10: Vec<u8> = p;
        let fresh12 = t_cp_10;
        t_cp_10 = t_cp_10.offset(1);
        *fresh12 = (t_s_4 >> 8);
        *t_cp_10 = t_s_4;
        p = p.offset(2);
        /* DO flag */
        lenp = p;
        let mut t_s_5: u16 = rdlen;
        let mut t_cp_11: Vec<u8> = p;
        let fresh13 = t_cp_11;
        t_cp_11 = t_cp_11.offset(1);
        *fresh13 = (t_s_5 >> 8);
        *t_cp_11 = t_s_5;
        p = p.offset(2);
        /* RDLEN */
        datap = p;
        /* Copy back any options */
        if !buff.is_null() {
            if p.offset(rdlen) > limit {
                // free(buff);
                return plen;
                /* Too big */
            }
            memcpy(p, buff, rdlen);
            // free(buff);
            p = p.offset(rdlen)
        }
        /* Only bump arcount if RR is going to fit */
        if optlen <= limit.wrapping_offset_from(p.offset(4)) {
            header.arcount = __bswap_16((__bswap_16(header.arcount) + 1))
        }
    } /* Too big */
    if optlen > limit.wrapping_offset_from(p.offset(4)) {
        return plen;
    }
    /* Add new option */
    if optno != 0 && replace != 2 {
        if p.offset(4) > limit {
            return plen;
        } /* Too big */
        let mut t_s_6: u16 = optno; /* Too big */
        let mut t_cp_12: Vec<u8> = p; /* can't get mac address, just delete any incoming. */
        let fresh14 = t_cp_12; /* handle 6 byte MACs */
        t_cp_12 = t_cp_12.offset(1);
        *fresh14 = (t_s_6 >> 8);
        *t_cp_12 = t_s_6;
        p = p.offset(2);
        let mut t_s_7: u16 = optlen;
        let mut t_cp_13: Vec<u8> = p;
        let fresh15 = t_cp_13;
        t_cp_13 = t_cp_13.offset(1);
        *fresh15 = (t_s_7 >> 8);
        *t_cp_13 = t_s_7;
        p = p.offset(2);
        if p.offset(optlen) > limit {
            return plen;
        }
        memcpy(p, opt, optlen);
        p = p.offset(optlen);
        let mut t_s_8: u16 = p.wrapping_offset_from(datap);
        let mut t_cp_14: Vec<u8> = lenp;
        let fresh16 = t_cp_14;
        t_cp_14 = t_cp_14.offset(1);
        *fresh16 = (t_s_8 >> 8);
        *t_cp_14 = t_s_8;
        lenp = lenp.offset(2)
    }
    return p.wrapping_offset_from(header);
}

pub fn add_do_bit(mut header: DnsHeader, mut plen: usize, mut limit: &mut Vec<u8>) -> usize {
    return add_pseudoheader(header, plen, limit, 512, 0, 0, 0, 1, 0);
}
fn char64(mut c: u8) -> u8 {
    return ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")[c & 0x3f];
}
fn encoder(mut in_0: &mut Vec<u8>, mut out: &mut String) {
    *out.offset(0) = char64((*in_0.offset(0) >> 2));
    *out.offset(1) = char64(((*in_0.offset(0)) << 4 | *in_0.offset(1) >> 4));
    *out.offset(2) = char64(((*in_0.offset(1)) << 2 | *in_0.offset(2) >> 6));
    *out.offset(3) = char64(*in_0.offset(2));
}
fn add_dns_client(
    mut header: DnsHeader,
    mut plen: usize,
    mut limit: &mut Vec<u8>,
    mut l3: NetAddress,
    mut now: time::Instant,
    mut cacheablep: i32,
) -> size_t {
    let mut maclen: i32 = 0;
    let mut replace: i32 = 2;
    let mut mac: [u8; 16] = [0; 16];
    let mut encode: [libc::c_char; 18] = [0; 18];
    maclen = find_mac(l3, mac, 1, now);
    if maclen == 6 {
        replace = 1;
        *cacheablep = 0;
        if dnsmasq_daemon.options[55] != 0 {
            print_mac(encode, mac, maclen);
        } else {
            encoder(mac, encode);
            encoder(mac.offset(3), encode.offset(4));
            encode[8] = 0
        }
    }
    return add_pseudoheader(
        header,
        plen,
        limit,
        512,
        65073,
        encode,
        strlen(encode),
        0,
        replace,
    );
}
fn add_mac(
    mut header: DnsHeader,
    mut plen: usize,
    mut limit: &mut Vec<u8>,
    mut l3: NetAddress,
    mut now: time::Instant,
    mut cacheablep: i32,
) -> size_t {
    let mut maclen: i32 = 0;
    let mut mac: [u8; 16] = [0; 16];
    maclen = find_mac(l3, mac, 1, now);
    if maclen != 0 {
        *cacheablep = 0;
        plen = add_pseudoheader(header, plen, limit, 512, 65001, mac, maclen, 0, 0)
    }
    return plen;
}
fn get_addrp(mut addr: NetAddress, family: libc::c_short) -> Vec<u8> {
    if family == 10 {
        let mut out = vec![0 as u8; 16];
        out.clone_from_slice(addr.value[0..16]);
    }
    let mut out = vec![0 as u8; 4];
    out.clone_from_slice(addr.value[0..4]);
    out
}
fn calc_subnet_opt(
    mut opt: &mut subnet_opt,
    mut source: NetAddress,
    mut cacheablep: i32,
) -> size_t {
    /* http://tools.ietf.org/html/draft-vandergaast-edns-client-subnet-02 */
    let mut len: i32 = 0;
    let mut addrp: Vec<u8> = 0;
    let mut sa_family: i32 = source.sa.sa_family;
    let mut cacheable: i32 = 0;
    opt.source_netmask = 0 as u8;
    opt.scope_netmask = 0 as u8;
    if source.sa.sa_family == 10 && !dnsmasq_daemon.add_subnet6.is_null() {
        opt.source_netmask = (*dnsmasq_daemon.add_subnet6).mask as u8;
        if (*dnsmasq_daemon.add_subnet6).addr_used != 0 {
            sa_family = (*dnsmasq_daemon.add_subnet6).addr.sa.sa_family;
            addrp = get_addrp(&mut (*dnsmasq_daemon.add_subnet6).addr, sa_family);
            cacheable = 1
        } else {
            addrp = &mut source.in6.sin6_addr;
        }
    }
    if source.sa.sa_family == 2 && !dnsmasq_daemon.add_subnet4.is_null() {
        opt.source_netmask = (*dnsmasq_daemon.add_subnet4).mask as u8;
        if (*dnsmasq_daemon.add_subnet4).addr_used != 0 {
            sa_family = (*dnsmasq_daemon.add_subnet4).addr.sa.sa_family;
            addrp = get_addrp(&mut (*dnsmasq_daemon.add_subnet4).addr, sa_family);
            cacheable = 1
            /* Address is constant */
        } else {
            addrp = &mut source.in_0.sin_addr;
        }
    } /* No address ever supplied. */
    opt.family = __bswap_16(if sa_family == 10 { 2 } else { 1 });
    if !addrp.is_null() && opt.source_netmask != 0 {
        len = (opt.source_netmask - 1 >> 3) + 1;
        memcpy(opt.addr, addrp, len);
        if opt.source_netmask & 7 != 0 {
            opt.addr[(len - 1)] =
                (opt.addr[(len - 1)] & (0xff) << 8 - (opt.source_netmask & 7)) as u8
        }
    } else {
        cacheable = 1;
        len = 0
    }
    if !cacheablep.is_null() {
        *cacheablep = cacheable
    }
    return (len + 4);
}
fn add_source_addr(
    mut header: DnsHeader,
    mut plen: usize,
    mut limit: &mut Vec<u8>,
    mut source: NetAddress,
    mut cacheable: i32,
) -> size_t {
    /* http://tools.ietf.org/html/draft-vandergaast-edns-client-subnet-02 */
    let mut len: i32 = 0;
    let mut opt: subnet_opt = subnet_opt {
        family: 0,
        source_netmask: 0,
        scope_netmask: 0,
        addr: [0; 16],
    };
    len = calc_subnet_opt(&mut opt, source, cacheable);
    return add_pseudoheader(header, plen, limit, 512, 8, &mut opt, len, 0, 0);
}

pub fn check_source(
    mut header: DnsHeader,
    mut plen: usize,
    mut pseudoheader: &mut Vec<u8>,
    mut peer: NetAddress,
) -> i32 {
    /* Section 9.2, Check that subnet option in reply matches. */
    let mut len: i32 = 0; /* skip UDP length and RCODE */
    let mut calc_len: i32 = 0; /* bad packet */
    let mut opt: subnet_opt = subnet_opt {
        family: 0,
        source_netmask: 0,
        scope_netmask: 0,
        addr: [0; 16],
    };
    let mut p: Vec<u8> = Vec::new();
    let mut code: i32 = 0;
    let mut i: i32 = 0;
    let mut rdlen: i32 = 0;
    calc_len = calc_subnet_opt(&mut opt, peer, 0);
    p = skip_name(pseudoheader, header, plen, 10);
    if p.is_null() {
        return 1;
    }
    p = p.offset(8);
    let mut t_cp: Vec<u8> = p;
    rdlen = (*t_cp.offset(0)) << 8 | *t_cp.offset(1);
    p = p.offset(2);
    if !((p.wrapping_offset_from(header) + rdlen) <= plen) {
        return 1;
    }
    /* check if option there */
    i = 0;
    while (i + 4) < rdlen {
        let mut t_cp_0: Vec<u8> = p;
        code = (*t_cp_0.offset(0)) << 8 | *t_cp_0.offset(1);
        p = p.offset(2);
        let mut t_cp_1: Vec<u8> = p;
        len = (*t_cp_1.offset(0)) << 8 | *t_cp_1.offset(1);
        p = p.offset(2);
        if code == 8 {
            /* make sure this doesn't mismatch. */
            opt.scope_netmask = *p.offset(3);
            if len != calc_len || memcmp(p, &mut opt, len) != 0 {
                return 0;
            }
        }
        p = p.offset(len);
        i += len + 4
    }
    return 1;
}
/* Set *check_subnet if we add a client subnet option, which needs to checked
in the reply. Set *cacheable to zero if we add an option which the answer
may depend on. */

pub fn add_edns0_config(
    mut header: DnsHeader,
    mut plen: usize,
    mut limit: &mut Vec<u8>,
    mut source: NetAddress,
    mut now: time::Instant,
    mut check_subnet: bool,
    mut cacheable: bool,
) -> usize {
    *check_subnet = 0;
    *cacheable = 1;
    if dnsmasq_daemon.options[32] != 0 {
        plen = add_mac(header, plen, limit, source, now, cacheable)
    }
    if dnsmasq_daemon.options[54] != 0 || dnsmasq_daemon.options[55] != 0 {
        plen = add_dns_client(header, plen, limit, source, now, cacheable)
    }
    if !dnsmasq_daemon.dns_client_id.is_null() {
        plen = add_pseudoheader(
            header,
            plen,
            limit,
            512,
            65074,
            dnsmasq_daemon.dns_client_id,
            strlen(dnsmasq_daemon.dns_client_id),
            0,
            1,
        )
    }
    if dnsmasq_daemon.options
        [(41).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))]
        & (1) << (41).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
        != 0
    {
        plen = add_source_addr(header, plen, limit, source, cacheable);
        *check_subnet = 1
    }
    return plen;
}
