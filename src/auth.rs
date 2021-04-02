
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
use crate::defines::{AddrList, AllAddr, InAddr, __bswap_32, InAddrT, AuthZone, DnsHeader, MySockAddr, DnsmasqDaemon, Crec, MxSrvRecord, TxtRecord, InterfaceName, NaPtr, Cname, __bswap_16, Iname, NameList};
use crate::util::{is_same_net, is_same_net6, hostname_isequal, hostname_issubdomain, sockaddr_isequal};
use crate::rfc1035::{skip_questions, extract_name, in_arpa_name_2_addr, add_resource_record};
use crate::cache::{log_query, cache_find_by_addr, cache_get_name, record_source, querystr, cache_find_by_name, cache_find_non_terminal, cache_enumerate};
use crate::dnsmasq_log::my_syslog;
use crate::in6_addr::In6Addr;

pub fn find_addrlist(list: &mut Vec<AddrList>, flag: libc::c_int, addr_u: &mut AllAddr) -> Option<AddrList> {
    for list_addr in list {
        if list_addr.flags & 2 == 0 {
            let mut netmask: InAddr = Default::default();
            let mut addr: InAddr = addr_u.addr4;
            if !(flag __b &
                (1 __b) << 7 == 0) { netmask.s_addr = __bswap_32((!(0 as InAddrT)) << 32 - (*list).prefixlen);
                if is_same_net(addr, (*list).addr.addr4, netmask) != 0 {
                    return Some(list_addr.clone())
                }
            }
        } else if is_same_net6(&mut addr_u.addr6, &mut (*list).addr.addr6,
                               (*list).prefixlen) != 0 {
            return Some(list_addr.clone())
        }
    }
    return None
}


pub fn find_subnet(mut zone: &mut AuthZone,
                   mut flag: libc::c_int,
                   mut addr_u: &mut AllAddr) -> Option<AddrList> {
    if zone.subnet.is_null() { return None }
    return find_addrlist(zone.subnet, flag, addr_u);
}
pub fn find_exclude(mut zone: &mut AuthZone,
                    mut flag: libc::c_int,
                    mut addr_u:&mut AllAddr)
                    -> Option<AddrList> {
    if zone.exclude.is_null() { return None }
    return find_addrlist(zone.exclude, flag, addr_u);
}

pub fn filter_zone(mut zone: &mut AuthZone,
                   mut flag: i32,
                   mut addr_u: &mut AllAddr) -> bool {
    if find_exclude(zone, flag, addr_u).is_some() { return false }
    /* No subnets specified, no filter */
    if zone.subnet.is_empty() { return true }
    return find_subnet(zone, flag, addr_u).is_some();
}

pub fn in_zone(mut zone: &mut AuthZone, mut name: &mut String, mut cut: &String) -> bool 
{
    let mut namelen: usize = name.len();
    let mut domainlen: usize = zone.domain.len();
    // if !cut.is_empty() { *cut = 0 as *mut libc::c_char }
    if namelen >= domainlen && hostname_isequal(&zone.domain, &name[domainlen..].to_string()) {
        if namelen == domainlen { return true }
        if name[domainlen] == '.' {
            if !cut.is_empty() {
                cut = &name[domainlen..].to_string();
            }
            return true
        }
    }
    return false;
}

pub fn answer_auth(
    daemon: &mut DnsmasqDaemon,
    mut header: &mut DnsHeader,
                       mut limit: &mut String,
                       mut qlen: size_t, mut now: time_t,
                       mut peer_addr: &mut MySockAddr,
                       mut local_query: libc::c_int,
                       mut do_bit: libc::c_int,
                       mut have_pseudoheader: libc::c_int)
                       -> usize {
    let mut name= daemon.namebuff.clone();
    let mut p: String;
    let mut ansp: String;
    let mut qtype = 0;
    let mut qclass = 0;
    let mut rc = 0;
    let mut nameoffset = 0;
    let mut axfroffset = 0;
    let mut q = 0;
    let mut anscount = 0;
    let mut authcount = 0;
    let mut crecp: Crec;
    let mut auth = local_query == 0;
    let mut trunc = 0;
    let mut nxdomain = 1;
    let mut soa = 0;
    let mut ns = 0;
    let mut axfr = 0;
    let mut zone: AuthZone;
    let mut subnet: AddrList;
    let mut cut: String;
    let mut rec: MxSrvRecord;
    let mut move_0: MxSrvRecord;
    let mut up: MxSrvRecord = 0;
    let mut txt: TxtRecord;
    let mut intr: InterfaceName;
    let mut na: NaPtr;
    let mut addr: AllAddr;
    let mut a: Cname;
    let mut candidate: Cname;
    let mut wclen = 0;
    if __bswap_16(header.qdcount) == 0 || (header.hb3 & 0x78) >> 3 != 0 {
        return 0
    }
    /* determine end of question section (we put answers there) */
    ansp = skip_questions(header, qlen); /* bad packet */
    if ansp.is_null() { return 0 as size_t }
    /* now process each question, answers go in RRs after the question */
    p = header.offset(1);
    let mut current_block_247: u64;
    q = __bswap_16(header.qdcount);
    while q != 0 {
        let mut flag: libc::c_uint = 0 __b;
        let mut found: libc::c_int = 0;
        let mut cname_wildcard: libc::c_int = 0;
        /* save pointer to name for copying into answers */
        nameoffset =
            p.wrapping_offset_from(header) as
                libc::c_long;
        /* now extract name as .-concatenated string into name */
        if extract_name(header, qlen, &mut p, name, 1, 4) == 0 {
            return 0 as size_t
        } /* bad packet */
        let mut t_cp: *mut libc::c_uchar = p; /* must be bare name */
        qtype =
            (t_cp.offset(0) as u16)
                << 8 |
                t_cp.offset(1) as u16 as
                    libc::c_int;
        p = p.offset(2);
        let mut t_cp_0: String = p;
        qclass =
            (t_cp_0.offset(0) as u16 as
                 libc::c_int) << 8 |
                t_cp_0.offset(1) as u16 as
                    libc::c_int;
        p = p.offset(2);
        if qclass != 1 {
            auth = 0
        } else {
            if (qtype == 12 || qtype == 6 ||
                    qtype == 2) &&
                   {
                       flag =
                           in_arpa_name_2_addr(name, &mut addr) as
                               libc::c_uint;
                       (flag) != 0
                   } && local_query == 0 {
                zone = daemon.auth_zones;
                while !zone.is_null() {
                    subnet =
                        find_subnet(zone, flag, &mut addr);
                    if !subnet.is_null() { break ; }
                    zone = zone.next
                }
                if zone.is_null() {
                    auth = 0;
                    current_block_247 = 17860125682698302841;
                } else {
                    if qtype == 6 {
                        soa = 1;
                        found = 1
                    } else if qtype == 2 {
                        ns = 1;
                        found = 1
                    }
                    current_block_247 = 4567019141635105728;
                }
            } else { current_block_247 = 4567019141635105728; }
            match current_block_247 {
                17860125682698302841 => { }
                _ => {
                    if qtype == 12 && flag != 0 {
                        intr = None;
                        if flag == (1 __b) << 7 {
                            intr = daemon.int_names;
                            while !intr.is_null() {
                                let mut addrlist: AddrList;
                                addrlist = intr.addr;
                                while !addrlist.is_null() {
                                    if (*addrlist).flags & 2 ==
                                           0 &&
                                           addr.addr4.s_addr ==
                                               (*addrlist).addr.addr4.s_addr {
                                        break ;
                                    }
                                    addrlist = (*addrlist).next
                                }
                                if !addrlist.is_null() { break ; }
                                while !intr.next.is_null() &&
                                          strcmp(intr.intr,
                                                 (*intr.next).intr) ==
                                              0 {
                                    intr = intr.next
                                }
                                intr = intr.next
                            }
                        } else if flag ==
                                      (1 __b) << 8
                         {
                            intr = daemon.int_names;
                            while !intr.is_null() {
                                let mut addrlist_0: AddrList;
                                addrlist_0 = intr.addr;
                                while !addrlist_0.is_null() {
                                    if addrlist_0.flags & 2
                                           != 0 &&
                                           ({
                                                let mut __a: In6Addr = addr.addr6.clone();
                                                let mut __b: In6Addr = addrlist_0.addr.addr6.clone();
                                                (__a.__in6_u.__u6_addr32[0]
                                                     ==
                                                     __b.__in6_u.__u6_addr32[0]
                                                     &&
                                                     __a.__in6_u.__u6_addr32[1]
                                                         ==
                                                         __b.__in6_u.__u6_addr32[1]
                                                     &&
                                                     __a.__in6_u.__u6_addr32[2]
                                                         ==
                                                         __b.__in6_u.__u6_addr32[2]
                                                     &&
                                                     __a.__in6_u.__u6_addr32[3]
                                                         ==
                                                         __b.__in6_u.__u6_addr32[3])
                                                   
                                            }) != false {
                                        break ;
                                    }
                                    addrlist_0 = addrlist_0.next
                                }
                                if !addrlist_0.is_null() { break ; }
                                while !intr.next.is_null() &&
                                          strcmp(intr.intr, (*intr.next).intr) ==
                                              0 {
                                    intr = intr.next
                                }
                                intr = intr.next
                            }
                        }
                        if !intr.is_null() {
                            if local_query != 0 ||
                                   in_zone(zone, intr.name, 0) != false {
                                found = 1;
                                log_query(flag | (1 __b) << 2 | (1 __b) << 13,
                                          intr.name,
                                &mut addr,
                                          0 as *mut libc::c_char);
                                if add_resource_record(header,
                                                       limit,
                                                       &mut trunc,
                                                       nameoffset,
                                                       &mut ansp,
                                                       daemon.auth_ttl,
                                                       0,
                                                       12,
                                                       1,
                                                       b"d\x00",
                                                       intr.name) != 0 {
                                    anscount += 1
                                }
                            }
                        }
                        crecp =
                            cache_find_by_addr(0 as *mut Crec, &mut addr, now,
                                               flag);
                        if !crecp.is_null() {
                            loop  {
                                strcpy(name, cache_get_name(crecp));
                                if (*crecp).flags &
                                       (1 __b) << 4
                                       != 0 &&
                                       daemon.options[(20 as
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
                                           (1 __b) <<
                                               (20 as
                                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong))
                                           == 0 {
                                    let mut p_0: *mut libc::c_char =
                                        strchr(name, '.' as i32);
                                    if !p_0.is_null() {
                                        *p_0 =
                                            0 as libc::c_char
                                    }
                                    /* add  external domain */
                                    if !zone.is_null() {
                                        strcat(name,
                                               b".\x00" as *const u8 as
                                                   *const libc::c_char);
                                        strcat(name, zone.domain);
                                    }
                                    log_query(flag |
                                                  (1 __b) <<
                                                      4 |
                                                  (1 __b) <<
                                                      2, name,
                                              &mut addr,
                                              record_source((*crecp).uid));
                                    found = 1;
                                    if add_resource_record(header, limit,
                                                           &mut trunc as
                                                               *mut libc::c_int,
                                                           nameoffset,
                                                           &mut ansp as
                                                               *mut *mut libc::c_uchar,
                                                           daemon.auth_ttl,
                                                           0 as
                                                               *mut libc::c_int,
                                                           12
                                                               as
                                                               libc::c_ushort,
                                                           1 as
                                                               libc::c_ushort,
                                                           b"d\x00" as
                                                               *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           name) != 0 {
                                        anscount += 1
                                    }
                                } else if (*crecp).flags &
                                              ((1 __b) <<
                                                   4 |
                                                   (1 __b) <<
                                                       6) != 0
                                              &&
                                              (local_query != 0 ||
                                                   in_zone(zone, name,
                                                           0 as
                                                               *mut *mut libc::c_char)
                                                       != 0) {
                                    log_query((*crecp).flags &
                                                  !((1 __b) <<
                                                        3),
                                              name, &mut addr,
                                              record_source((*crecp).uid));
                                    found = 1;
                                    if add_resource_record(header, limit,
                                                           &mut trunc as
                                                               *mut libc::c_int,
                                                           nameoffset,
                                                           &mut ansp as
                                                               *mut *mut libc::c_uchar,
                                                           daemon.auth_ttl,
                                                           0 as
                                                               *mut libc::c_int,
                                                           12
                                                               as
                                                               libc::c_ushort,
                                                           1 as
                                                               libc::c_ushort,
                                                           b"d\x00" as
                                                               *const u8 as
                                                               *const libc::c_char
                                                               as
                                                               *mut libc::c_char,
                                                           name) != 0 {
                                        anscount += 1
                                    }
                                }
                                crecp =
                                    cache_find_by_addr(crecp, &mut addr, now,
                                                       flag);
                                if crecp.is_null() { break ; }
                            }
                        }
                        if found != 0 {
                            nxdomain = 0
                        } else {
                            log_query(flag |
                                          (1 __b) <<
                                              5 |
                                          (1 __b) <<
                                              10 |
                                          (1 __b) <<
                                              2 |
                                          (if auth != 0 {
                                               ((1 __b)) <<
                                                   21
                                           } else {
                                               0 as
                                                   libc::c_uint
                                           }), 0 as *mut libc::c_char,
                                      &mut addr, 0 as *mut libc::c_char);
                        }
                    } else {
                        loop  {
                            if found != 0 {
                                /* NS and SOA .arpa requests have set found above. */
                                cut = 0 as *mut libc::c_char
                            } else {
                                zone = daemon.auth_zones;
                                while !zone.is_null() {
                                    if in_zone(zone, name, &mut cut) != 0 {
                                        break ;
                                    }
                                    zone = zone.next
                                }
                                if zone.is_null() {
                                    auth = 0;
                                    break ;
                                }
                            }
                            rec = daemon.mxnames;
                            while !rec.is_null() {
                                if (*rec).issrv == 0 &&
                                       {
                                           rc =
                                               hostname_issubdomain(name,
                                                                    (*rec).name);
                                           (rc) != 0
                                       } {
                                    nxdomain = 0;
                                    if rc == 2 &&
                                           qtype == 15 {
                                        found = 1;
                                        log_query((1 __b) <<
                                                      13 |
                                                      (1 __b) <<
                                                          17,
                                                  name, 0 as *mut AllAddr,
                                                  b"<MX>\x00" as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char);
                                        if add_resource_record(header, limit,
                                                               &mut trunc as
                                                                   *mut libc::c_int,
                                                               nameoffset,
                                                               &mut ansp as
                                                                   *mut *mut libc::c_uchar,
                                                               daemon.auth_ttl,
                                                               0 as
                                                                   *mut libc::c_int,
                                                               15 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ushort,
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ushort,
                                                               b"sd\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char,
                                                               (*rec).weight,
                                                               (*rec).target)
                                               != 0 {
                                            anscount += 1
                                        }
                                    }
                                }
                                rec = (*rec).next
                            }
                            move_0 = 0 as *mut MxSrvRecord;
                            up = &mut daemon.mxnames;
                            rec = daemon.mxnames;
                            while !rec.is_null() {
                                if (*rec).issrv != 0 &&
                                       {
                                           rc =
                                               hostname_issubdomain(name,
                                                                    (*rec).name);
                                           (rc) != 0
                                       } {
                                    nxdomain = 0;
                                    if rc == 2 &&
                                           qtype == 33 {
                                        found = 1;
                                        log_query((1 __b) <<
                                                      13 |
                                                      (1 __b) <<
                                                          17,
                                                  name, 0 as *mut AllAddr,
                                                  b"<SRV>\x00" as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char);
                                        if add_resource_record(header, limit,
                                                               &mut trunc as
                                                                   *mut libc::c_int,
                                                               nameoffset,
                                                               &mut ansp as
                                                                   *mut *mut libc::c_uchar,
                                                               daemon.auth_ttl,
                                                               0 as
                                                                   *mut libc::c_int,
                                                               33 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ushort,
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ushort,
                                                               b"sssd\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char,
                                                               (*rec).priority,
                                                               (*rec).weight,
                                                               (*rec).srvport,
                                                               (*rec).target)
                                               != 0 {
                                            anscount += 1
                                        }
                                    }
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
                                *up = move_0; /* inhibits auth section */
                                (*move_0).next = 0 as *mut MxSrvRecord
                            }
                            txt = daemon.rr;
                            while !txt.is_null() {
                                rc = hostname_issubdomain(name, (*txt).name);
                                if rc != 0 {
                                    nxdomain = 0;
                                    if rc == 2 &&
                                           (*txt).class ==
                                               qtype {
                                        found = 1;
                                        log_query((1 __b) <<
                                                      13 |
                                                      (1 __b) <<
                                                          17,
                                                  name, 0 as *mut AllAddr,
                                                  querystr(0 as
                                                               *mut libc::c_char,
                                                           (*txt).class));
                                        if add_resource_record(header, limit,
                                                               &mut trunc as
                                                                   *mut libc::c_int,
                                                               nameoffset,
                                                               &mut ansp as
                                                                   *mut *mut libc::c_uchar,
                                                               daemon.auth_ttl,
                                                               0 as
                                                                   *mut libc::c_int,
                                                               (*txt).class,
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ushort,
                                                               b"t\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char,
                                                               (*txt).len as
                                                                   libc::c_int,
                                                               (*txt).txt) !=
                                               0 {
                                            anscount += 1
                                        }
                                    }
                                }
                                txt = (*txt).next
                            }
                            txt = daemon.txt;
                            while !txt.is_null() {
                                if (*txt).class ==
                                       1 &&
                                       {
                                           rc =
                                               hostname_issubdomain(name,
                                                                    (*txt).name);
                                           (rc) != 0
                                       } {
                                    nxdomain = 0;
                                    if rc == 2 &&
                                           qtype == 16 {
                                        found = 1;
                                        log_query((1 __b) <<
                                                      13 |
                                                      (1 __b) <<
                                                          17,
                                                  name, 0 as *mut AllAddr,
                                                  b"<TXT>\x00" as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char);
                                        if add_resource_record(header, limit,
                                                               &mut trunc as
                                                                   *mut libc::c_int,
                                                               nameoffset,
                                                               &mut ansp as
                                                                   *mut *mut libc::c_uchar,
                                                               daemon.auth_ttl,
                                                               0 as
                                                                   *mut libc::c_int,
                                                               16 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ushort,
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ushort,
                                                               b"t\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char,
                                                               (*txt).len as
                                                                   libc::c_int,
                                                               (*txt).txt) !=
                                               0 {
                                            anscount += 1
                                        }
                                    }
                                }
                                txt = (*txt).next
                            }
                            na = daemon.naptr;
                            while !na.is_null() {
                                rc = hostname_issubdomain(name, (*na).name);
                                if rc != 0 {
                                    nxdomain = 0;
                                    if rc == 2 &&
                                           qtype == 35 {
                                        found = 1;
                                        log_query((1 __b) <<
                                                      13 |
                                                      (1 __b) <<
                                                          17,
                                                  name, 0 as *mut AllAddr,
                                                  b"<NAPTR>\x00" as *const u8
                                                      as *const libc::c_char
                                                      as *mut libc::c_char);
                                        if add_resource_record(header, limit,
                                                               &mut trunc as
                                                                   *mut libc::c_int,
                                                               nameoffset,
                                                               &mut ansp as
                                                                   *mut *mut libc::c_uchar,
                                                               daemon.auth_ttl,
                                                               0 as
                                                                   *mut libc::c_int,
                                                               35 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ushort,
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ushort,
                                                               b"sszzzd\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char,
                                                               (*na).order,
                                                               (*na).pref,
                                                               (*na).flags,
                                                               (*na).services,
                                                               (*na).regexp,
                                                               (*na).replace)
                                               != 0 {
                                            anscount += 1
                                        }
                                    }
                                }
                                na = (*na).next
                            }
                            if qtype == 1 {
                                flag = (1 __b) << 7
                            }
                            if qtype == 28 {
                                flag = (1 __b) << 8
                            }
                            intr = daemon.int_names;
                            while !intr.is_null() {
                                rc = hostname_issubdomain(name, intr.name);
                                if rc != 0 {
                                    let mut addrlist_1: *mut AddrList =
                                        0 as *mut AddrList;
                                    nxdomain = 0;
                                    if rc == 2 && flag != 0 {
                                        addrlist_1 = intr.addr;
                                        while !addrlist_1.is_null() {
                                            if (if (*addrlist_1).flags &
                                                       2 != 0 {
                                                    28
                                                } else { 1 })
                                                   == qtype &&
                                                   (local_query != 0 ||
                                                        filter_zone(zone,
                                                                    flag as
                                                                        libc::c_int,
                                                                    &mut (*addrlist_1).addr)
                                                            != 0) {
                                                if !((*addrlist_1).flags &
                                                         4 !=
                                                         0) {
                                                    found = 1;
                                                    log_query((1 as
                                                                   libc::c_uint)
                                                                  <<
                                                                  3 as
                                                                      libc::c_int
                                                                  |
                                                                  (1 as
                                                                       libc::c_uint)
                                                                      <<
                                                                      13 as
                                                                          libc::c_int
                                                                  | flag,
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
                                                                           daemon.auth_ttl,
                                                                           0
                                                                               as
                                                                               *mut libc::c_int,
                                                                           qtype
                                                                               as
                                                                               libc::c_ushort,
                                                                           1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ushort,
                                                                           if qtype
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
                                                                               *mut AllAddr)
                                                           != 0 {
                                                        anscount += 1
                                                    }
                                                }
                                            }
                                            addrlist_1 = (*addrlist_1).next
                                        }
                                    }
                                }
                                intr = intr.next
                            }
                            if cut.is_null() {
                                nxdomain = 0;
                                if qtype == 6 {
                                    soa = 1;
                                    auth = soa;
                                    found = 1;
                                    log_query((1 __b) <<
                                                  17 |
                                                  (1 __b) <<
                                                      21,
                                              zone.domain,
                                              0 as *mut AllAddr,
                                              b"<SOA>\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                                } else if qtype == 252 {
                                    let mut peers: *mut Iname =
                                        0 as *mut Iname;
                                    if (*peer_addr).sa.sa_family as
                                           libc::c_int == 2 {
                                        (*peer_addr).in_0.sin_port =
                                            0 as in_port_t
                                    } else {
                                        (*peer_addr).in6.sin6_port =
                                            0 as in_port_t;
                                        (*peer_addr).in6.sin6_scope_id =
                                            0 as u32
                                    }
                                    peers = daemon.auth_peers;
                                    while !peers.is_null() {
                                        if sockaddr_isequal(peer_addr,
                                                            &mut (*peers).addr)
                                               != 0 {
                                            break ;
                                        }
                                        peers = (*peers).next
                                    }
                                    /* Refuse all AXFR unless --auth-sec-servers or auth-peers is set */
                                    if daemon.secondary_forward_server.is_null()
                                           &&
                                           daemon.auth_peers.is_null()
                                           ||
                                           !daemon.auth_peers.is_null()
                                               && peers.is_null() {
                                        if (*peer_addr).sa.sa_family as
                                               libc::c_int == 2
                                           {
                                            inet_ntop(2,
                                                      &mut (*peer_addr).in_0.sin_addr
                                                          as *mut InAddr as
                                                          *const libc::c_void,
                                                      daemon.addrbuff,
                                                      46 as
                                                          socklen_t); /* inhibits auth section */
                                        } else {
                                            inet_ntop(10,
                                                      &mut (*peer_addr).in6.sin6_addr
                                                          as *mut In6Addr as
                                                          *const libc::c_void,
                                                      daemon.addrbuff,
                                                      46 as
                                                          socklen_t); /* ensure we include NS records! */
                                        } /* inhibits auth section */
                                        my_syslog(4,
                                                  b"ignoring zone transfer request from %s\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  daemon.addrbuff); /* remove domain part */
                                        return 0 as size_t
                                    }
                                    auth = 1;
                                    soa = 1;
                                    ns = 1;
                                    axfr = 1;
                                    found = 1;
                                    axfroffset = nameoffset;
                                    log_query((1 __b) <<
                                                  17 |
                                                  (1 __b) <<
                                                      21,
                                              zone.domain,
                                              0 as *mut AllAddr,
                                              b"<AXFR>\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                                } else if qtype == 2 {
                                    auth = 1;
                                    ns = 1;
                                    found = 1;
                                    log_query((1 __b) <<
                                                  17 |
                                                  (1 __b) <<
                                                      21,
                                              zone.domain,
                                              0 as *mut AllAddr,
                                              b"<NS>\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                                }
                            }
                            if daemon.options[(20 as
                                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                               as
                                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_ulong))
                                                             as usize] &
                                   (1 __b) <<
                                       (20 as
                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong))
                                   == 0 && !cut.is_null() {
                                *cut = 0 as libc::c_char;
                                if strchr(name, '.' as i32).is_null() &&
                                       {
                                           crecp =
                                               cache_find_by_name(0 as
                                                                      *mut Crec,
                                                                  name, now,
                                                                  (1 as
                                                                       libc::c_uint)
                                                                      <<
                                                                      7 as
                                                                          libc::c_int
                                                                      |
                                                                      (1 as
                                                                           libc::c_uint)
                                                                          <<
                                                                          8 as
                                                                              libc::c_int);
                                           !crecp.is_null()
                                       } {
                                    if (*crecp).flags &
                                           (1 __b) <<
                                               4 != 0 {
                                        loop  {
                                            nxdomain = 0;
                                            if (*crecp).flags & flag != 0 &&
                                                   (local_query != 0 ||
                                                        filter_zone(zone,
                                                                    flag as
                                                                        libc::c_int,
                                                                    &mut (*crecp).addr)
                                                            != 0) {
                                                /* restore domain part */
                                                *cut =
                                                    '.' as i32 as
                                                        libc::c_char; /* restore domain part */
                                                log_query((*crecp).flags,
                                                          name,
                                                          &mut (*crecp).addr,
                                                          record_source((*crecp).uid)); /* remove domain part */
                                                *cut =
                                                    0 as
                                                        libc::c_char;
                                                found = 1;
                                                if add_resource_record(header,
                                                                       limit,
                                                                       &mut trunc
                                                                           as
                                                                           *mut libc::c_int,
                                                                       nameoffset,
                                                                       &mut ansp
                                                                           as
                                                                           *mut *mut libc::c_uchar,
                                                                       daemon.auth_ttl,
                                                                       0 as
                                                                           *mut libc::c_int,
                                                                       qtype
                                                                           as
                                                                           libc::c_ushort,
                                                                       1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ushort,
                                                                       if qtype
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
                                                                           *mut AllAddr)
                                                       != 0 {
                                                    anscount += 1
                                                }
                                            }
                                            crecp =
                                                cache_find_by_name(crecp,
                                                                   name, now,
                                                                   (1 as
                                                                        libc::c_uint)
                                                                       <<
                                                                       7 as
                                                                           libc::c_int
                                                                       |
                                                                       (1 as
                                                                            libc::c_uint)
                                                                           <<
                                                                           8
                                                                               as
                                                                               libc::c_int);
                                            if crecp.is_null() { break ; }
                                        }
                                    }
                                }
                                *cut = '.' as i32 as libc::c_char
                            }
                            crecp =
                                cache_find_by_name(0 as *mut Crec, name, now,
                                                   (1 __b) <<
                                                       7 |
                                                       (1 __b) <<
                                                           8);
                            if !crecp.is_null() {
                                if (*crecp).flags &
                                       (1 __b) << 6
                                       != 0 ||
                                       (*crecp).flags &
                                           (1 __b) <<
                                               4 != 0 &&
                                           daemon.options[(20 as
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
                                               (1 __b) <<
                                                   (20 as
                                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                         as
                                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         libc::c_ulong))
                                               != 0 {
                                    loop  {
                                        nxdomain = 0;
                                        if (*crecp).flags & flag != 0 &&
                                               (local_query != 0 ||
                                                    filter_zone(zone,
                                                                flag as
                                                                    libc::c_int,
                                                                &mut (*crecp).addr)
                                                        != 0) {
                                            log_query((*crecp).flags, name,
                                                      &mut (*crecp).addr,
                                                      record_source((*crecp).uid));
                                            found = 1;
                                            if add_resource_record(header,
                                                                   limit,
                                                                   &mut trunc
                                                                       as
                                                                       *mut libc::c_int,
                                                                   nameoffset,
                                                                   &mut ansp
                                                                       as
                                                                       *mut *mut libc::c_uchar,
                                                                   daemon.auth_ttl,
                                                                   0 as
                                                                       *mut libc::c_int,
                                                                   qtype as
                                                                       libc::c_ushort,
                                                                   1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ushort,
                                                                   if qtype ==
                                                                          1 as
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
                                                                       *mut AllAddr)
                                                   != 0 {
                                                anscount += 1
                                            }
                                        }
                                        crecp =
                                            cache_find_by_name(crecp, name,
                                                               now,
                                                               (1 as
                                                                    libc::c_uint)
                                                                   <<
                                                                   7 as
                                                                       libc::c_int
                                                                   |
                                                                   (1 as
                                                                        libc::c_uint)
                                                                       <<
                                                                       8 as
                                                                           libc::c_int);
                                        if crecp.is_null() { break ; }
                                    }
                                }
                            }
                            /* Only supply CNAME if no record for any type is known. */
                            if !(nxdomain != 0) { break ; }
                            /* Check for possible wildcard match against *.domain 
	     return length of match, to get longest.
	     Note that if return length of wildcard section, so
	     we match b.simon to _both_ *.simon and b.simon
	     but return a longer (better) match to b.simon.
	  */
                            wclen = 0 __b;
                            candidate = 0 as *mut Cname;
                            a = daemon.cnames;
                            while !a.is_null() {
                                if *(*a).alias.offset(0 as
                                                          isize) as
                                       libc::c_int == '*' as i32 {
                                    let mut test: *mut libc::c_char = name;
                                    loop  {
                                        test =
                                            strchr(test.offset(1 as
                                                                   libc::c_int
                                                                  ),
                                                   '.' as i32);
                                        if test.is_null() { break ; }
                                        if !(hostname_isequal(test,
                                                              &mut *(*a).alias.offset(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          isize))
                                                 != 0) {
                                            continue ;
                                        }
                                        if strlen(test) >
                                               wclen as libc::c_ulong &&
                                               cname_wildcard == 0 {
                                            wclen =
                                                strlen(test) __b;
                                            candidate = a;
                                            cname_wildcard = 1
                                        }
                                        break ;
                                    }
                                } else if hostname_isequal((*a).alias, name)
                                              != 0 &&
                                              strlen((*a).alias) >
                                                  wclen as libc::c_ulong {
                                    /* Simple case, no wildcard */
                                    wclen =
                                        strlen((*a).alias) __b;
                                    candidate = a
                                }
                                a = (*a).next
                            }
                            if !candidate.is_null() {
                                log_query((1 __b) <<
                                              13 |
                                              (1 __b) <<
                                                  11, name,
                                          0 as *mut AllAddr,
                                          0 as *mut libc::c_char);
                                strcpy(name, (*candidate).target);
                                if strchr(name, '.' as i32).is_null() {
                                    strcat(name,
                                           b".\x00" as *const u8 as
                                               *const libc::c_char);
                                    strcat(name, zone.domain);
                                }
                                found = 1;
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       daemon.auth_ttl,
                                                       &mut nameoffset as
                                                           *mut libc::c_int,
                                                       5 as
                                                           libc::c_ushort,
                                                       1 as
                                                           libc::c_ushort,
                                                       b"d\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char,
                                                       name) != 0 {
                                    anscount += 1
                                }
                            } else {
                                if cache_find_non_terminal(name, now) != 0 {
                                    nxdomain = 0
                                }
                                log_query(flag |
                                              (1 __b) <<
                                                  5 |
                                              (if nxdomain != 0 {
                                                   ((1 __b)) <<
                                                       10
                                               } else {
                                                   0 as
                                                       libc::c_uint
                                               }) |
                                              (1 __b) <<
                                                  3 |
                                              (1 __b) <<
                                                  21, name,
                                          0 as *mut AllAddr,
                                          0 as *mut libc::c_char);
                                break ;
                            }
                        }
                    }
                }
            }
        }
        q -= 1
    }
    /* Add auth section */
    if auth != 0 && !zone.is_null() {
        let mut authname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut newoffset: libc::c_int = 0;
        let mut offset: libc::c_int = 0;
        if subnet.is_null() {
            authname = zone.domain
        } else {
            /* handle NS and SOA for PTR records */
            authname = name;
            if (*subnet).flags & 2 == 0 {
                let mut a_0: InAddrT =
                    __bswap_32((*subnet).addr.addr4.s_addr) >>
                        8;
                let mut p_1: *mut libc::c_char = name;
                if (*subnet).prefixlen >= 24 {
                    p_1 =
                        p_1.offset(sprintf(p_1,
                                           b"%u.\x00" as *const u8 as
                                               *const libc::c_char,
                                           a_0 &
                                               0xff as
                                                   libc::c_uint))
                }
                a_0 = a_0 >> 8;
                if (*subnet).prefixlen >= 16 {
                    p_1 =
                        p_1.offset(sprintf(p_1,
                                           b"%u.\x00" as *const u8 as
                                               *const libc::c_char,
                                           a_0 &
                                               0xff as
                                                   libc::c_uint))
                }
                a_0 = a_0 >> 8;
                p_1 =
                    p_1.offset(sprintf(p_1,
                                       b"%u.in-addr.arpa\x00" as *const u8 as
                                           *const libc::c_char,
                                       a_0 &
                                           0xff as
                                               libc::c_uint))
            } else {
                let mut p_2: *mut libc::c_char = name;
                let mut i: libc::c_int = 0;
                i = (*subnet).prefixlen - 1;
                while i >= 0 {
                    let mut dig: libc::c_int =
                        *(&mut (*subnet).addr.addr6 as *mut In6Addr as
                              *mut libc::c_uchar).offset((i >>
                                                              3 as
                                                                  libc::c_int)
                                                            ) as
                            libc::c_int;
                    p_2 =
                        p_2.offset(sprintf(p_2,
                                           b"%.1x.\x00" as *const u8 as
                                               *const libc::c_char,
                                           if i >> 2 &
                                                  1 != 0 {
                                               (dig) & 15
                                           } else {
                                               (dig) >> 4
                                           }));
                    i -= 4
                }
                p_2 =
                    p_2.offset(sprintf(p_2,
                                       b"ip6.arpa\x00" as *const u8 as
                                           *const libc::c_char))
            }
        }
        /* handle NS and SOA in auth section or for explicit queries */
        newoffset =
            ansp.wrapping_offset_from(header) as
                libc::c_long;
        if (anscount == 0 && ns == 0 || soa != 0) &&
               add_resource_record(header, limit,
                                   &mut trunc as *mut libc::c_int,
                                   0,
                                   &mut ansp as *mut *mut libc::c_uchar,
                                   daemon.auth_ttl,
                                   0 as *mut libc::c_int,
                                   6 as libc::c_ushort,
                                   1 as libc::c_ushort,
                                   b"ddlllll\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, authname,
                                   daemon.authserver,
                                   daemon.hostmaster,
                                   daemon.soa_sn,
                                   daemon.soa_refresh,
                                   daemon.soa_retry,
                                   daemon.soa_expiry,
                                   daemon.auth_ttl) != 0 {
            offset = newoffset;
            if soa != 0 { anscount += 1 } else { authcount += 1 }
        }
        if anscount != 0 || ns != 0 {
            let mut secondary: *mut NameList = 0 as *mut NameList;
            /* Only include the machine running dnsmasq if it's acting as an auth server */
            if !daemon.authinterface.is_null() {
                newoffset =
                    ansp.wrapping_offset_from(header) as
                        libc::c_long;
                if add_resource_record(header, limit,
                                       &mut trunc as *mut libc::c_int,
                                       -offset,
                                       &mut ansp as *mut *mut libc::c_uchar,
                                       daemon.auth_ttl,
                                       0 as *mut libc::c_int,
                                       2 as libc::c_ushort,
                                       1 as libc::c_ushort,
                                       b"d\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char,
                                       if offset == 0 {
                                           authname
                                       } else { 0 as *mut libc::c_char },
                                       daemon.authserver) != 0 {
                    if offset == 0 { offset = newoffset }
                    if ns != 0 { anscount += 1 } else { authcount += 1 }
                }
            }
            if subnet.is_null() {
                secondary = daemon.secondary_forward_server;
                while !secondary.is_null() {
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           offset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           daemon.auth_ttl,
                                           0 as *mut libc::c_int,
                                           2 as libc::c_ushort,
                                           1 as libc::c_ushort,
                                           b"d\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char,
                                           (*secondary).name) != 0 {
                        if ns != 0 { anscount += 1 } else { authcount += 1 }
                    }
                    secondary = (*secondary).next
                }
            }
        }
        if axfr != 0 {
            rec = daemon.mxnames;
            while !rec.is_null() {
                if in_zone(zone, (*rec).name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0 as libc::c_char
                    }
                    if (*rec).issrv != 0 {
                        if add_resource_record(header, limit,
                                               &mut trunc as *mut libc::c_int,
                                               -axfroffset,
                                               &mut ansp as
                                                   *mut *mut libc::c_uchar,
                                               daemon.auth_ttl,
                                               0 as *mut libc::c_int,
                                               33 as
                                                   libc::c_ushort,
                                               1 as
                                                   libc::c_ushort,
                                               b"sssd\x00" as *const u8 as
                                                   *const libc::c_char as
                                                   *mut libc::c_char,
                                               if !cut.is_null() {
                                                   (*rec).name
                                               } else {
                                                   0 as *mut libc::c_char
                                               }, (*rec).priority,
                                               (*rec).weight, (*rec).srvport,
                                               (*rec).target) != 0 {
                            anscount += 1
                        }
                    } else if add_resource_record(header, limit,
                                                  &mut trunc as
                                                      *mut libc::c_int,
                                                  -axfroffset,
                                                  &mut ansp as
                                                      *mut *mut libc::c_uchar,
                                                  daemon.auth_ttl,
                                                  0 as *mut libc::c_int,
                                                  15 as
                                                      libc::c_ushort,
                                                  1 as
                                                      libc::c_ushort,
                                                  b"sd\x00" as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char,
                                                  if !cut.is_null() {
                                                      (*rec).name
                                                  } else {
                                                      0 as *mut libc::c_char
                                                  }, (*rec).weight,
                                                  (*rec).target) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.' as i32 as libc::c_char }
                }
                rec = (*rec).next
            }
            txt = daemon.rr;
            while !txt.is_null() {
                if in_zone(zone, (*txt).name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0 as libc::c_char
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           -axfroffset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           daemon.auth_ttl,
                                           0 as *mut libc::c_int,
                                           (*txt).class,
                                           1 as libc::c_ushort,
                                           b"t\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char,
                                           if !cut.is_null() {
                                               (*txt).name
                                           } else { 0 as *mut libc::c_char },
                                           (*txt).len,
                                           (*txt).txt) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.' as i32 as libc::c_char }
                }
                txt = (*txt).next
            }
            txt = daemon.txt;
            while !txt.is_null() {
                if (*txt).class == 1 &&
                       in_zone(zone, (*txt).name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0 as libc::c_char
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           -axfroffset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           daemon.auth_ttl,
                                           0 as *mut libc::c_int,
                                           16 as
                                               libc::c_ushort,
                                           1 as libc::c_ushort,
                                           b"t\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char,
                                           if !cut.is_null() {
                                               (*txt).name
                                           } else { 0 as *mut libc::c_char },
                                           (*txt).len,
                                           (*txt).txt) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.' as i32 as libc::c_char }
                }
                txt = (*txt).next
            }
            na = daemon.naptr;
            while !na.is_null() {
                if in_zone(zone, (*na).name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0 as libc::c_char
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           -axfroffset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           daemon.auth_ttl,
                                           0 as *mut libc::c_int,
                                           35 as
                                               libc::c_ushort,
                                           1 as libc::c_ushort,
                                           b"sszzzd\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char,
                                           if !cut.is_null() {
                                               (*na).name
                                           } else { 0 as *mut libc::c_char },
                                           (*na).order, (*na).pref,
                                           (*na).flags, (*na).services,
                                           (*na).regexp, (*na).replace) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.' as i32 as libc::c_char }
                }
                na = (*na).next
            }
            intr = daemon.int_names;
            while !intr.is_null() {
                if in_zone(zone, intr.name, &mut cut) != 0 {
                    let mut addrlist_2: *mut AddrList = 0 as *mut AddrList;
                    if !cut.is_null() {
                        *cut = 0 as libc::c_char
                    }
                    addrlist_2 = intr.addr;
                    while !addrlist_2.is_null() {
                        if (*addrlist_2).flags & 2 == 0 &&
                               (local_query != 0 ||
                                    filter_zone(zone,
                                                ((1 __b) <<
                                                     7) as
                                                    libc::c_int,
                                                &mut (*addrlist_2).addr) != 0)
                               &&
                               add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   -axfroffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   daemon.auth_ttl,
                                                   0 as *mut libc::c_int,
                                                   1 as
                                                       libc::c_ushort,
                                                   1 as
                                                       libc::c_ushort,
                                                   b"4\x00" as *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char,
                                                   (if !cut.is_null() {
                                                        intr.name
                                                    } else {
                                                        0 as *mut libc::c_char
                                                    }),
                                                   &mut (*addrlist_2).addr as
                                                       *mut AllAddr) != 0 {
                            anscount += 1
                        }
                        addrlist_2 = (*addrlist_2).next
                    }
                    addrlist_2 = intr.addr;
                    while !addrlist_2.is_null() {
                        if (*addrlist_2).flags & 2 != 0 &&
                               (local_query != 0 ||
                                    filter_zone(zone,
                                                ((1 __b) <<
                                                     8) as
                                                    libc::c_int,
                                                &mut (*addrlist_2).addr) != 0)
                               &&
                               add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   -axfroffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   daemon.auth_ttl,
                                                   0 as *mut libc::c_int,
                                                   28 as
                                                       libc::c_ushort,
                                                   1 as
                                                       libc::c_ushort,
                                                   b"6\x00" as *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char,
                                                   (if !cut.is_null() {
                                                        intr.name
                                                    } else {
                                                        0 as *mut libc::c_char
                                                    }),
                                                   &mut (*addrlist_2).addr as
                                                       *mut AllAddr) != 0 {
                            anscount += 1
                        }
                        addrlist_2 = (*addrlist_2).next
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.' as i32 as libc::c_char }
                }
                intr = intr.next
            }
            a = daemon.cnames;
            while !a.is_null() {
                if in_zone(zone, (*a).alias, &mut cut) != 0 {
                    strcpy(name, (*a).target);
                    if strchr(name, '.' as i32).is_null() {
                        strcat(name,
                               b".\x00" as *const u8 as *const libc::c_char);
                        strcat(name, zone.domain);
                    }
                    if !cut.is_null() {
                        *cut = 0 as libc::c_char
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           -axfroffset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           daemon.auth_ttl,
                                           0 as *mut libc::c_int,
                                           5 as libc::c_ushort,
                                           1 as libc::c_ushort,
                                           b"d\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char,
                                           if !cut.is_null() {
                                               (*a).alias
                                           } else { 0 as *mut libc::c_char },
                                           name) != 0 {
                        anscount += 1
                    }
                }
                a = (*a).next
            }
            cache_enumerate(1);
            loop  {
                crecp = cache_enumerate(0);
                if crecp.is_null() { break ; }
                if (*crecp).flags &
                       ((1 __b) << 7 |
                            (1 __b) << 8) != 0 &&
                       (*crecp).flags &
                           ((1 __b) << 5 |
                                (1 __b) << 10) == 0
                       &&
                       (*crecp).flags &
                           (1 __b) << 3 != 0 {
                    if (*crecp).flags &
                           (1 __b) << 4 != 0 &&
                           daemon.options[(20 as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 __b) <<
                                   (20 as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               == 0 {
                        let mut cache_name: *mut libc::c_char =
                            cache_get_name(crecp);
                        if strchr(cache_name, '.' as i32).is_null() &&
                               (local_query != 0 ||
                                    filter_zone(zone,
                                                ((*crecp).flags &
                                                     ((1 __b) <<
                                                          8 |
                                                          (1 __b)
                                                              <<
                                                              7 as
                                                                  libc::c_int))
                                                   ,
                                                &mut (*crecp).addr) != 0) &&
                               add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   -axfroffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   daemon.auth_ttl,
                                                   0 as *mut libc::c_int,
                                                   (if (*crecp).flags &
                                                           (1 __b)
                                                               <<
                                                               8 as
                                                                   libc::c_int
                                                           != 0 {
                                                        28
                                                    } else {
                                                        1
                                                    }) as libc::c_ushort,
                                                   1 as
                                                       libc::c_ushort,
                                                   (if (*crecp).flags &
                                                           (1 __b)
                                                               <<
                                                               7 as
                                                                   libc::c_int
                                                           != 0 {
                                                        b"4\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                    } else {
                                                        b"6\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                    }) as *mut libc::c_char,
                                                   cache_name,
                                                   &mut (*crecp).addr as
                                                       *mut AllAddr) != 0 {
                            anscount += 1
                        }
                    }
                    if (*crecp).flags &
                           (1 __b) << 6 != 0 ||
                           (*crecp).flags &
                               (1 __b) << 4 != 0 &&
                               daemon.options[(20 as
                                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                               as
                                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_ulong))
                                                             as usize] &
                                   (1 __b) <<
                                       (20 as
                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong))
                                   != 0 {
                        strcpy(name, cache_get_name(crecp));
                        if in_zone(zone, name, &mut cut) != 0 &&
                               (local_query != 0 ||
                                    filter_zone(zone,
                                                ((*crecp).flags &
                                                     ((1 __b) <<
                                                          8 |
                                                          (1 __b)
                                                              <<
                                                              7 as
                                                                  libc::c_int))
                                                   ,
                                                &mut (*crecp).addr) != 0) {
                            if !cut.is_null() {
                                *cut = 0 as libc::c_char
                            }
                            if add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   -axfroffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   daemon.auth_ttl,
                                                   0 as *mut libc::c_int,
                                                   if (*crecp).flags &
                                                          (1 __b)
                                                              <<
                                                              8
                                                          != 0 {
                                                       28
                                                   } else { 1 }
                                                       as libc::c_ushort,
                                                   1 as
                                                       libc::c_ushort,
                                                   if (*crecp).flags &
                                                          (1 __b)
                                                              <<
                                                              7
                                                          != 0 {
                                                       b"4\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                   } else {
                                                       b"6\x00" as *const u8
                                                           as
                                                           *const libc::c_char
                                                   } as *mut libc::c_char,
                                                   if !cut.is_null() {
                                                       name
                                                   } else {
                                                       0 as *mut libc::c_char
                                                   },
                                                   &mut (*crecp).addr as
                                                       *mut AllAddr) != 0 {
                                anscount += 1
                            }
                        }
                    }
                }
            }
            /* repeat SOA as last record */
            if add_resource_record(header, limit,
                                   &mut trunc as *mut libc::c_int, axfroffset,
                                   &mut ansp as *mut *mut libc::c_uchar,
                                   daemon.auth_ttl,
                                   0 as *mut libc::c_int,
                                   6 as libc::c_ushort,
                                   1 as libc::c_ushort,
                                   b"ddlllll\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,
                                   daemon.authserver,
                                   daemon.hostmaster,
                                   daemon.soa_sn,
                                   daemon.soa_refresh,
                                   daemon.soa_retry,
                                   daemon.soa_expiry,
                                   daemon.auth_ttl) != 0 {
                anscount += 1
            }
        }
    }
    /* done all questions, set up header and return length of result */
  /* clear authoritative and truncated flags, set QR flag */
    header.hb3 =
        (header.hb3 &
             !(0x4 | 0x2) | 0x80)
            as u8;
    if local_query != 0 {
        /* set RA flag */
        header.hb4 =
            (header.hb4 | 0x80) as u8
    } else {
        /* clear RA flag */
        header.hb4 =
            (header.hb4 & !(0x80)) as u8
    }
    /* data is never DNSSEC signed. */
    header.hb4 =
        (header.hb4 & !(0x20)) as u8;
    /* authoritative */
    if auth != 0 {
        header.hb3 =
            (header.hb3 | 0x4) as u8
    }
    /* truncation */
    if trunc != 0 {
        header.hb3 =
            (header.hb3 | 0x2) as u8
    } /* no error */
    if (auth != 0 || local_query != 0) && nxdomain != 0 {
        header.hb4 =
            (header.hb4 & !(0xf) |
                 3) as u8
    } else {
        header.hb4 =
            (header.hb4 & !(0xf) |
                 0) as u8
    }
    header.ancount = __bswap_16(anscount as u16);
    header.nscount = __bswap_16(authcount as u16);
    header.arcount = __bswap_16(0 as u16);
    /* Advertise our packet size limit in our reply */
    if have_pseudoheader != 0 {
        return add_pseudoheader(header,
                                ansp.wrapping_offset_from(header as
                                                              *mut libc::c_uchar)
                                    as libc::c_long as size_t,
                                limit,
                                daemon.edns_pktsz,
                                0, 0,
                                0 as size_t, do_bit,
                                0)
    }
    return ansp.wrapping_offset_from(header) as
               libc::c_long as size_t;
}
