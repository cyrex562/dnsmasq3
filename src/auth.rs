
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
use crate::defines::{AddressListEntry, AuthZone, DnsHeader, DnsmasqDaemon, Crec, MxSrvRecord, TxtRecord, InterfaceName, NaPtr, Cname, Iname, NameListEntry, NetAddress};
use crate::util::{is_same_net, is_same_net6, hostname_isequal, hostname_issubdomain, NetAddress_isequal};
use crate::rfc1035::{skip_questions, extract_name, in_arpa_name_2_addr, add_resource_record};
use crate::cache::{log_query, cache_find_by_addr, cache_get_name, record_source, querystr, cache_find_by_name, cache_find_non_terminal, cache_enumerate};
use crate::dnsmasq_log::my_syslog;
use std::time;

pub fn find_addrlist(list: &Vec<AddressListEntry>, flag: u32, addr_u: &NetAddress) -> Option<AddressListEntry> {
    for entry in list {
        if list_addr.flags[2] == false {
            let mut netmask: NetAddress = Default::default();
            let mut address: NetAddress = addr_u.ip_address.clone();
            if is_same_net(address, entry.addr.ip_address, netmask) {
                return Some(entry.clone())
            }
        } else if is_ame_net6(&mut addr_u.ip_address, &mut entry.addr.ip_address, entry.prefixlen) {
            return Seom(entry.clone())
        }
    }
    return None
}


pub fn find_subnet(mut zone: &AuthZone,
                   mut flag: u32,
                   mut addr_u: &NetAddress) -> Option<AddressListEntry> {
    if zone.subnet.is_null() { return None }
    return find_addrlist(&zone.subnet, flag, addr_u);
}
pub fn find_exclude(mut zone: &AuthZone,
                    mut flag: u32,
                    mut addr_u: &NetAddress)
                    -> Option<AddressListEntry> {
    if zone.exclude.is_null() { return None }
    return find_addrlist(&zone.exclude, flag, addr_u);
}

pub fn filter_zone(mut zone: &AuthZone,
                   mut flag: u32,
                   mut addr_u: &NetAddress) -> bool {
    if find_exclude(zone, flag, addr_u).is_some() { return false }
    /* No subnets specified, no filter */
    if zone.subnet.is_empty() { return true }
    return find_subnet(zone, flag, addr_u).is_some();
}

pub fn in_zone(mut zone: &AuthZone, mut name: &String, mut cut: Option<&String>) -> bool
{
    let mut namelen: usize = name.len();
    let mut domainlen: usize = zone.domain.len();
    // if !cut.is_empty() { *cut = 0 as *mut libc::c_char }
    if namelen >= domainlen && hostname_isequal(&zone.domain, &name[domainlen..].to_string()) {
        if namelen == domainlen { return true }
        if name[domainlen] == '.' {
            if cut.is_some() {
                let mut cut_val = cut.unwrap();
                if cut_val.is_empty() == false {
                    cut_val = &name[domainlen..].to_string();
                }
            }
            true
        }
    }
    false
}

pub fn answer_auth(
    mut daemon: &mut DnsmasqDaemon,
    mut header: &mut DnsHeader,
                       mut limit: &mut String,
                       mut qlen: usize, mut now: time::Instant,
                       mut peer_addr: &mut NetAddress,
                       mut local_query: i32,
                       mut do_bit: i32,
                       mut have_pseudoheader: i32)
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
    let mut subnet: AddressListEntry;
    let mut cut: String;
    let mut rec: MxSrvRecord;
    let mut move_0: MxSrvRecord;
    let mut up: MxSrvRecord;
    let mut txt: TxtRecord;
    let mut intr: InterfaceName;
    let mut na: NaPtr;
    let mut addr: NetAddress;
    let mut a: Cname;
    let mut candidate: Cname;
    let mut wclen = 0;
    if __bswap_16(header.qdcount) == 0 || (header.hb3 & 0x78) >> 3 != 0 {
        return 0
    }
    /* determine end of question section (we put answers there) */
    match skip_questions(header, qlen) {
        Some(x) => ansp = x,
        None() => return 0,
    }; /* bad packet */

    /* now process each question, answers go in RRs after the question */
    p = header.offset(1);
    let mut current_block_247: u64;
    q = __bswap_16(header.qdcount);
    while q != 0 {
        let mut flag: u32 = 0;
        let mut found: u32 = 0;
        let mut cname_wildcard: u32 = 0;
        /* save pointer to name for copying into answers */
        nameoffset = p.wrapping_offset_from(header);
        /* now extract name as .-concatenated string into name */
        if extract_name(header, qlen, &mut p, &mut name, 1, 4) == 0 {
            return 0
        } /* bad packet */
        let mut t_cp: String = p; /* must be bare name */
        qtype = (t_cp.offset(0)) << 8 | t_cp.offset(1) ;
        p = p.offset(2);
        let mut t_cp_0: String = p;
        qclass = (t_cp_0.offset(0) ) << 8 | t_cp_0.offset(1) ;
        p = p.offset(2);
        if qclass != 1 {
            auth = false
        } else {
            if (qtype == 12 || qtype == 6 || qtype == 2) && in_arpa_name_2_addr(&mut name, &mut addr) != 0 && local_query == 0 {
                for zone in daemon.auth_zones {
                    match find_subnet(&zone, flag, &mut addr) {
                        Some(_) => continue,
                        None => break
                    }
                }

                if zone.is_null() {
                    auth = false;
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
                        // intr = None;
                        if flag == 1 << 7 {
                            for intr in daemon.int_names {
                                for addr in intr.addresses {
                                    if addr.flags & 2 == 0 && addr.addr4.s_addr == addr.addr.addr4.s_addr {
                                        break ;
                                    }
                                    // addrlist = (*addrlist).next
                                }
                                if !addrlist.is_null() { break ; }
                                // for intr in
                                // while !intr.next.is_null() &&
                                //           strcmp(intr.intr, (*intr.next).intr) == 0 {
                                //     // intr = intr.next
                                // }
                                // intr = intr.next
                            }
                        } else if flag == 1 << 8 {
                            for intr in daemon.int_names {
                                let mut addrlist_0: AddressListEntry;
                                for addrlist_0 in intr.addresses {
                                    if addrlist_0.flags & 2 != 0 && ({
                                                let mut __a: NetAddress = addr.addr6.clone();
                                                let mut __b: NetAddress = addrlist_0.addr.addr6.clone();
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
                                    // addrlist_0 = addrlist_0.next
                                }
                                if !addrlist_0.is_null() { break ; }
                                // while !intr.next.is_null() && strcmp(intr.intr, (*intr.next).intr) ==
                                //               0 {
                                //     intr = intr.next
                                // }
                                // intr = intr.next
                            }
                        }
                        if !intr.is_null() {
                            if local_query != 0 ||
                                   in_zone(&zone, &intr.name, None) != false {
                                found = 1;
                                log_query(&mut daemon,flag | (1 << 2) | (1 << 13), &intr.name, &mut addr, None );
                                if add_resource_record(header,
                                                       limit,
                                                       trunc,
                                                       nameoffset,
                                                       &mut ansp,
                                                       daemon.auth_ttl,
                                                       0,
                                                       12,
                                                       1,
                                                       &format!("{}", intr.name)) != 0 {
                                    anscount += 1
                                }
                            }
                        }
                        crecp = cache_find_by_addr(None, &mut addr, now, flag);
                        if !crecp.is_null() {
                            loop  {
                                name = cache_get_name(crecp);
                                if crecp.flags & (1 << 4) != 0 && daemon.options[20] == false {
                                    // let mut p_0: String = strchr(name, '.' as i32);
                                    let mut p_0: String = String::new();
                                    let idx = name.find(".");
                                    if idx.is_none() {
                                        p_0 = String::new();
                                    } else {
                                        p_0 = name[idx..].to_string();
                                    }
                                    /* add  external domain */
                                    if !zone.is_null() {
                                        name = name + "." + zone.domain.as_str();
                                    }
                                    log_query(daemon, flag | (1 << 4) | (1 << 2), &name,
                                              &mut addr, Some(record_source(crecp.uid)));
                                    found = 1;
                                    if add_resource_record(header, limit, trunc, nameoffset, &mut ansp, daemon.auth_ttl, 0 , 12 , 1 , &name) != 0 {
                                        anscount += 1
                                    }
                                } else if crecp.flags & (1 << 4) | (1 << 6) != 0
                                              && (local_query != 0 || in_zone(&zone, &name, None ) != false) {
                                    log_query(daemon, crecp.flags & !(1<< 3), &name, &mut addr,
                                              Some(record_source(crecp.uid)));
                                    found = 1;
                                    if add_resource_record(header, limit,
                                                           trunc ,
                                                           nameoffset,
                                                           &mut ansp ,
                                                           daemon.auth_ttl,
                                                           0 ,
                                                           12,
                                                           1,
                                                           &name) != 0 {
                                        anscount += 1
                                    }
                                }
                                crecp = cache_find_by_addr(crecp, &mut addr, now, flag);
                                if crecp.is_null() { break ; }
                            }
                        }
                        if found != 0 {
                            nxdomain = 0
                        } else {
                            // log_query(daemon, flag | ( 1 << 5) | ( 1 << 10) | (1 << 2) |
                            //               (if auth != false {
                            //                    1 << 21
                            //                } else {
                            //                    0
                            //                }), &format!("addr: {}", addr));
                        }
                    } else {
                        loop  {
                            if found != 0 {
                                /* NS and SOA .arpa requests have set found above. */
                                cut = String::new()
                            } else {
                                for zone in daemon.auth_zones {
                                    if in_zone(&zone, &name, Some(&cut)) != false {
                                        break;
                                    }
                                }

                                if zone.is_null() {
                                    auth = false;
                                    break ;
                                }
                            }
                            for rec in daemon.mxnames {
                                if rec.issrv == 0 &&
                                       {
                                           rc = hostname_issubdomain(&name, &rec.name);
                                           (rc) != 0
                                       } {
                                    nxdomain = 0;
                                    if rc == 2 && qtype == 15 {
                                        found = 1;
                                        // log_query((1 __b) <<
                                        //               13 |
                                        //               (1 __b) <<
                                        //                   17,
                                        //           name, 0 ,
                                        //           b"<MX>\x00" );
                                        if add_resource_record(header, limit,
                                                               trunc ,
                                                               nameoffset,
                                                               &mut ansp ,
                                                               daemon.auth_ttl,
                                                               0 ,
                                                               15 ,
                                                               1 ,
                                                               &format!("{}{}", rec.weight, rec.target)) != 0 {
                                            anscount += 1
                                        }
                                    }
                                }
                                // rec = rec.next
                            }
                            move_0 = Default::default();
                            // up = daemon.mxnames;
                            // rec = daemon.mxnames;
                            while !rec.is_null() {
                                if rec.issrv != 0 &&
                                       {
                                           rc = hostname_issubdomain(&name, &rec.name);
                                           (rc) != 0
                                       } {
                                    nxdomain = 0;
                                    if rc == 2 && qtype == 33 {
                                        found = 1;
                                        // log_query((1 __b) <<
                                        //               13 |
                                        //               (1 __b) <<
                                        //                   17,
                                        //           name, 0 ,
                                        //           b"<SRV>\x00" );
                                        if add_resource_record(header, limit,
                                                               trunc ,
                                                               nameoffset,
                                                               &mut ansp ,
                                                               daemon.auth_ttl,
                                                               0 ,
                                                               33 ,
                                                               1 ,
                                                               &format!("{}{}{}{}", rec.priority,
                                                                       rec.weight,
                                                                       rec.srvport,
                                                                       rec.target)                                                               )
                                               != 0 {
                                            anscount += 1
                                        }
                                    }
                                    /* unlink first SRV record found */
                                    if move_0.is_null() {
                                        // move_0 = rec;
                                        // *up = rec.next
                                    } else { 
                                        // up = &mut rec.next 
                                    }
                                } else { 
                                    // up = &mut rec.next 
                                }
                                // rec = rec.next
                            }
                            /* put first SRV record back at the end. */
                            if !move_0.is_null() {
                                *up = move_0; /* inhibits auth section */
                                (*move_0).next = 0
                            }
                            txt = daemon.rr;
                            while !txt.is_null() {
                                rc = hostname_issubdomain(&name, &txt.name);
                                if rc != 0 {
                                    nxdomain = 0;
                                    if rc == 2 && txt.class == qtype {
                                        found = 1;
                                        // log_query((1 __b) <<
                                        //               13 |
                                        //               (1 __b) <<
                                        //                   17,
                                        //           name, 0 ,
                                        //           querystr(0 ,
                                        //                    txt.class));
                                        if add_resource_record(header, limit,
                                                               trunc ,
                                                               nameoffset,
                                                               &mut ansp ,
                                                               daemon.auth_ttl,
                                                               0 ,
                                                               txt.class,
                                                               1 ,
                                                               &format!("{}{}", txt.len, txt.txt)) != 0 {
                                            anscount += 1
                                        }
                                    }
                                }
                                // txt = txt.next
                            }
                            txt = daemon.txt;
                            while !txt.is_null() {
                                if txt.class == 1 && {
                                           rc = hostname_issubdomain(&name, &txt.name);
                                           (rc) != 0
                                       } {
                                    nxdomain = 0;
                                    if rc == 2 && qtype == 16 {
                                        found = 1;
                                        // log_query((1 __b) <<
                                        //               13 |
                                        //               (1 __b) <<
                                        //                   17,
                                        //           name, 0 ,
                                        //           b"<TXT>\x00" );
                                        if add_resource_record(header, limit,
                                                               trunc ,
                                                               nameoffset,
                                                               &mut ansp ,
                                                               daemon.auth_ttl,
                                                               0 ,
                                                               16 ,
                                                               1 ,
                                                               &format!("{}{}",
                                                               txt.len ,
                                                               txt.txt)) !=
                                               0 {
                                            anscount += 1
                                        }
                                    }
                                }
                                txt = txt.next
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
                                                  name, 0 ,
                                                  b"<NAPTR>\x00" );
                                        if add_resource_record(header, limit,
                                                               &mut trunc ,
                                                               nameoffset,
                                                               &mut ansp ,
                                                               daemon.auth_ttl,
                                                               0 ,
                                                               35 ,
                                                               1 ,
                                                               b"sszzzd\x00"
                                                                   ,
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
                                    let mut addrlist_1: *mut AddressListEntry =
                                        0 ;
                                    nxdomain = 0;
                                    if rc == 2 && flag != 0 {
                                        addrlist_1 = intr.addresses;
                                        while !addrlist_1.is_null() {
                                            if (if (*addrlist_1).flags &
                                                       2 != 0 {
                                                    28
                                                } else { 1 })
                                                   == qtype &&
                                                   (local_query != 0 ||
                                                        filter_zone(zone,
                                                                    flag ,
                                                                    &mut (*addrlist_1).addr)
                                                            != 0) {
                                                if !((*addrlist_1).flags &
                                                         4 !=
                                                         0) {
                                                    found = 1;
                                                    log_query((1 )
                                                                  <<
                                                                  3
                                                                  |
                                                                  (1 )
                                                                      <<
                                                                      13
                                                                  | flag,
                                                              name,
                                                              &mut (*addrlist_1).addr,
                                                              0 );
                                                    if add_resource_record(header,
                                                                           limit,
                                                                           &mut trunc
                                                                               ,
                                                                           nameoffset,
                                                                           &mut ansp
                                                                               ,
                                                                           daemon.auth_ttl,
                                                                           0
                                                                               ,
                                                                           qtype
                                                                               ,
                                                                           1
                                                                               ,
                                                                           if qtype
                                                                                  ==
                                                                                  1

                                                                              {
                                                                               b"4\x00"

                                                                           } else {
                                                                               b"6\x00"

                                                                           }
                                                                               ,
                                                                           &mut (*addrlist_1).addr
                                                                               )
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
                                              0 ,
                                              b"<SOA>\x00" );
                                } else if qtype == 252 {
                                    let mut peers: Iname;
                                    if (*peer_addr).sa.sa_family  == 2 {
                                        (*peer_addr).in_0.sin_port =
                                            0 =
                                    } else {
                                        (*peer_addr).in6.sin6_port =
                                            0 ;
                                        (*peer_addr).in6.sin6_scope_id =
                                            0
                                    }
                                    peers = daemon.auth_peers;
                                    while !peers.is_null() {
                                        if NetAddress_isequal(peer_addr,
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
                                        if (*peer_addr).sa.sa_family                                         libc::c_int == 2
                                           {
                                            inet_ntop(2,
                                                      &mut (*peer_addr).in_0.sin_addr
                                                          ,
                                                      daemon.addrbuff,
                                                      46       socklen_t); /* inhibits auth section */
                                        } else {
                                            inet_ntop(10,
                                                      &mut (*peer_addr).in6.sin6_addr
                                                          ,
                                                      daemon.addrbuff,
                                                      46       socklen_t); /* ensure we include NS records! */
                                        } /* inhibits auth section */
                                        my_syslog(4,
                                                  b"ignoring zone transfer request from %s\x00"
                                                      ,
                                                  daemon.addrbuff); /* remove domain part */
                                        return 0
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
                                              0 ,
                                              b"<AXFR>\x00" );
                                } else if qtype == 2 {
                                    auth = 1;
                                    ns = 1;
                                    found = 1;
                                    log_query((1 __b) <<
                                                  17 |
                                                  (1 __b) <<
                                                      21,
                                              zone.domain,
                                              0 ,
                                              b"<NS>\x00" );
                                }
                            }
                            if daemon.options[(20    libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                            ).wrapping_mul(8                                                                      libc::c_int                                                               ))
                                                             ] &
                                   (1 __b) <<
                                       (20).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
                                   == 0 && !cut.is_null() {
                                *cut = 0;
                                if strchr(name, '.' as i32).is_null() &&
                                       {
                                           crecp =
                                               cache_find_by_name(0            Crec,
                                                                  name, now,
                                                                  (1             libc::c_uint)
                                                                      <<
                                                                      7                libc::c_int
                                                                      |
                                                                      (1                 libc::c_uint)
                                                                          <<
                                                                          8                    libc::c_int);
                                           !crecp.is_null()
                                       } {
                                    if crecp.flags &
                                           (1 __b) <<
                                               4 != 0 {
                                        loop  {
                                            nxdomain = 0;
                                            if crecp.flags & flag != 0 &&
                                                   (local_query != 0 ||
                                                        filter_zone(zone,
                                                                    flag              libc::c_int,
                                                                    &mut crecp.addr)
                                                            != 0) {
                                                /* restore domain part */
                                                *cut =
                                                    '.' as i32     libc::c_char; /* restore domain part */
                                                log_query(crecp.flags,
                                                          name,
                                                          &mut crecp.addr,
                                                          record_source(crecp.uid)); /* remove domain part */
                                                *cut =
                                                    0     libc::c_char;
                                                found = 1;
                                                if add_resource_record(header,
                                                                       limit,
                                                                       &mut trunc
                                                                                           ,
                                                                       nameoffset,
                                                                       &mut ansp
                                                                                           ,
                                                                       daemon.auth_ttl,
                                                                       0                 ,
                                                                       qtype
                                                                                        ,
                                                                       1                 libc::c_int
                                                                                        ,
                                                                       if qtype
                                                                              ==
                                                                              1
                                                                                                         libc::c_int
                                                                          {
                                                                           b"4\x00"
                                                                                                   *const u8
                                                                                                   *const libc::c_char
                                                                       } else {
                                                                           b"6\x00"
                                                                                                   *const u8
                                                                                                   *const libc::c_char
                                                                       }                 &mut String,
                                                                       &mut crecp.addr
                                                                                       )
                                                       != 0 {
                                                    anscount += 1
                                                }
                                            }
                                            crecp =
                                                cache_find_by_name(crecp,
                                                                   name, now,
                                                                   (1              libc::c_uint)
                                                                       <<
                                                                       7                 libc::c_int
                                                                       |
                                                                       (1                  libc::c_uint)
                                                                           <<
                                                                           8
                                                                                                   libc::c_int);
                                            if crecp.is_null() { break ; }
                                        }
                                    }
                                }
                                *cut = '.'
                            }
                            crecp =
                                cache_find_by_name(0 , name, now,
                                                   (1 __b) <<
                                                       7 |
                                                       (1 __b) <<
                                                           8);
                            if !crecp.is_null() {
                                if crecp.flags &
                                       (1 __b) << 6
                                       != 0 ||
                                       crecp.flags &
                                           (1 __b) <<
                                               4 != 0 &&
                                           daemon.options[(20                libc::c_int
                                                                                  ).wrapping_div((::std::mem::size_of::<libc::c_uint>()                       ).wrapping_mul(8                                                                                              libc::c_int                                                                                       ))
                                                                                       usize]
                                               &
                                               (1 __b) <<
                                                   (20     libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                                ).wrapping_mul(8                                                          libc::c_int                                                   ))
                                               != 0 {
                                    loop  {
                                        nxdomain = 0;
                                        if crecp.flags & flag != 0 &&
                                               (local_query != 0 ||
                                                    filter_zone(zone,
                                                                flag          libc::c_int,
                                                                &mut crecp.addr)
                                                        != 0) {
                                            log_query(crecp.flags, name,
                                                      &mut crecp.addr,
                                                      record_source(crecp.uid));
                                            found = 1;
                                            if add_resource_record(header,
                                                                   limit,
                                                                   &mut trunc
                                                                                   ,
                                                                   nameoffset,
                                                                   &mut ansp
                                                                                   ,
                                                                   daemon.auth_ttl,
                                                                   0             ,
                                                                   qtype          ,
                                                                   1             libc::c_int
                                                                                ,
                                                                   if qtype ==
                                                                          1                    libc::c_int
                                                                      {
                                                                       b"4\x00"
                                                                                           *const u8
                                                                                           *const libc::c_char
                                                                   } else {
                                                                       b"6\x00"
                                                                                           *const u8
                                                                                           *const libc::c_char
                                                                   }             &mut String,
                                                                   &mut crecp.addr
                                                                               )
                                                   != 0 {
                                                anscount += 1
                                            }
                                        }
                                        crecp =
                                            cache_find_by_name(crecp, name,
                                                               now,
                                                               (1          libc::c_uint)
                                                                   <<
                                                                   7             libc::c_int
                                                                   |
                                                                   (1              libc::c_uint)
                                                                       <<
                                                                       8                 libc::c_int);
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
                            candidate = 0 ;
                            a = daemon.cnames;
                            while !a.is_null() {
                                if *(*a).alias.offset(0       isize)                                 libc::c_int == '*' as i32 {
                                    let mut test: &mut String = name;
                                    loop  {
                                        test =
                                            strchr(test.offset(1         libc::c_int
                                                                  ),
                                                   '.' as i32);
                                        if test.is_null() { break ; }
                                        if !(hostname_isequal(test,
                                                              &mut *(*a).alias.offset(1
                                                                                                                         libc::c_int
                                                                                   ))
                                                 != 0) {
                                            continue ;
                                        }
                                        if strlen(test) >
                                               wclen &&
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
                                                  wclen {
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
                                          0 ,
                                          0 );
                                strcpy(name, (*candidate).target);
                                if strchr(name, '.' as i32).is_null() {
                                    strcat(name,
                                           b".\x00" );
                                    strcat(name, zone.domain);
                                }
                                found = 1;
                                if add_resource_record(header, limit,
                                                       &mut trunc ,
                                                       nameoffset,
                                                       &mut ansp ,
                                                       daemon.auth_ttl,
                                                       &mut nameoffset
                                                           ,
                                                       5 ,
                                                       1 ,
                                                       b"d\x00" ,
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
                                                   0
                                               }) |
                                              (1 __b) <<
                                                  3 |
                                              (1 __b) <<
                                                  21, name,
                                          0 ,
                                          0 );
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
        let mut authname: &mut String = 0 ;
        let mut newoffset: i32 = 0;
        let mut offset: i32 = 0;
        if subnet.is_null() {
            authname = zone.domain
        } else {
            /* handle NS and SOA for PTR records */
            authname = name;
            if (*subnet).flags & 2 == 0 {
                let mut a_0: InAddrT =
                    __bswap_32((*subnet).addr.addr4.s_addr) >>
                        8;
                let mut p_1: &mut String = name;
                if (*subnet).prefixlen >= 24 {
                    p_1 =
                        p_1.offset(sprintf(p_1,
                                           b"%u.\x00" ,
                                           a_0 &
                                               0xff ))
                }
                a_0 = a_0 >> 8;
                if (*subnet).prefixlen >= 16 {
                    p_1 =
                        p_1.offset(sprintf(p_1,
                                           b"%u.\x00" ,
                                           a_0 &
                                               0xff ))
                }
                a_0 = a_0 >> 8;
                p_1 =
                    p_1.offset(sprintf(p_1,
                                       b"%u.in-addr.arpa\x00" ,
                                       a_0 &
                                           0xff ))
            } else {
                let mut p_2: &mut String = name;
                let mut i: i32 = 0;
                i = (*subnet).prefixlen - 1;
                while i >= 0 {
                    let mut dig: i32 =
                        *(&mut (*subnet).addr.addr6).offset((i >>
                                                              3 )
                                                            )                      libc::c_int;
                    p_2 =
                        p_2.offset(sprintf(p_2,
                                           b"%.1x.\x00" ,
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
                                       b"ip6.arpa\x00" ))
            }
        }
        /* handle NS and SOA in auth section or for explicit queries */
        newoffset =
            ansp.wrapping_offset_from(header) ;
        if (anscount == 0 && ns == 0 || soa != 0) &&
               add_resource_record(header, limit,
                                   &mut trunc,
                                   0,
                                   &mut ansp,
                                   daemon.auth_ttl,
                                   0,
                                   6 ,
                                   1 ,
                                   b"ddlllll\x00", authname,
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
            let mut secondary: NameListEntry = 0 ;
            /* Only include the machine running dnsmasq if it's acting as an auth server */
            if !daemon.authinterface.is_null() {
                newoffset =
                    ansp.wrapping_offset_from(header);
                if add_resource_record(header, limit,
                                       &mut trunc,
                                       -offset,
                                       &mut ansp,
                                       daemon.auth_ttl,
                                       0,
                                       2,
                                       1,
                                       b"d\x00",
                                       if offset == 0 {
                                           authname
                                       } else { 0  },
                                       daemon.authserver) != 0 {
                    if offset == 0 { offset = newoffset }
                    if ns != 0 { anscount += 1 } else { authcount += 1 }
                }
            }
            if subnet.is_null() {
                secondary = daemon.secondary_forward_server;
                while !secondary.is_null() {
                    if add_resource_record(header, limit,
                                           &mut trunc,
                                           offset,
                                           &mut ansp,
                                           daemon.auth_ttl,
                                           0,
                                           2 ,
                                           1 ,
                                           b"d\x00",
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
                if in_zone(zone, rec.name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0
                    }
                    if rec.issrv != 0 {
                        if add_resource_record(header, limit,
                                               &mut trunc,
                                               -axfroffset,
                                               &mut ansp,
                                               daemon.auth_ttl,
                                               0,
                                               33libc::c_ushort,
                                               1libc::c_ushort,
                                               b"sssd\x00",
                                               if !cut.is_null() {
                                                   rec.name
                                               } else {
                                                   0
                                               }, rec.priority,
                                               rec.weight, rec.srvport,
                                               rec.target) != 0 {
                            anscount += 1
                        }
                    } else if add_resource_record(header, limit,
                                                  &mut trunc   ,
                                                  -axfroffset,
                                                  &mut ansp   ,
                                                  daemon.auth_ttl,
                                                  0,
                                                  15,
                                                  1,
                                                  b"sd\x00",
                                                  if !cut.is_null() {
                                                      rec.name
                                                  } else {
                                                      0
                                                  }, rec.weight,
                                                  rec.target) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.'  }
                }
                rec = rec.next
            }
            txt = daemon.rr;
            while !txt.is_null() {
                if in_zone(zone, txt.name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc,
                                           -axfroffset,
                                           &mut ansp,
                                           daemon.auth_ttl,
                                           0,
                                           txt.class,
                                           1 ,
                                           b"t\x00",
                                           if !cut.is_null() {
                                               txt.name
                                           } else { 0  },
                                           txt.len,
                                           txt.txt) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.'  }
                }
                txt = txt.next
            }
            txt = daemon.txt;
            while !txt.is_null() {
                if txt.class == 1 &&
                       in_zone(zone, txt.name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc,
                                           -axfroffset,
                                           &mut ansp,
                                           daemon.auth_ttl,
                                           0,
                                           16,
                                           1 ,
                                           b"t\x00" ,
                                           if !cut.is_null() {
                                               txt.name
                                           } else { 0  },
                                           txt.len,
                                           txt.txt) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.'  }
                }
                txt = txt.next
            }
            na = daemon.naptr;
            while !na.is_null() {
                if in_zone(zone, (*na).name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc,
                                           -axfroffset,
                                           &mut ansp,
                                           daemon.auth_ttl,
                                           0,
                                           35,
                                           1 ,
                                           b"sszzzd\x00",
                                           if !cut.is_null() {
                                               (*na).name
                                           } else { 0  },
                                           (*na).order, (*na).pref,
                                           (*na).flags, (*na).services,
                                           (*na).regexp, (*na).replace) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.'  }
                }
                na = (*na).next
            }
            intr = daemon.int_names;
            while !intr.is_null() {
                if in_zone(zone, intr.name, &mut cut) != 0 {
                    let mut addrlist_2: *mut AddressListEntry = 0 ;
                    if !cut.is_null() {
                        *cut = 0
                    }
                    addrlist_2 = intr.addresses;
                    while !addrlist_2.is_null() {
                        if (*addrlist_2).flags & 2 == 0 &&
                               (local_query != 0 ||
                                    filter_zone(zone,
                                                ((1 __b) <<
                                                     7) ,
                                                &mut (*addrlist_2).addr) != 0)
                               &&
                               add_resource_record(header, limit,
                                                   &mut trunc ,
                                                   -axfroffset,
                                                   &mut ansp ,
                                                   daemon.auth_ttl,
                                                   0 ,
                                                   1,
                                                   1,
                                                   b"4\x00",
                                                   (if !cut.is_null() {
                                                        intr.name
                                                    } else {
                                                        0
                                                    }),
                                                   &mut (*addrlist_2).addr) != 0 {
                            anscount += 1
                        }
                        addrlist_2 = (*addrlist_2).next
                    }
                    addrlist_2 = intr.addresses;
                    while !addrlist_2.is_null() {
                        if (*addrlist_2).flags & 2 != 0 &&
                               (local_query != 0 ||
                                    filter_zone(zone,
                                                ((1 __b) <<
                                                     8),
                                                &mut (*addrlist_2).addr) != 0)
                               &&
                               add_resource_record(header, limit,
                                                   &mut trunc,
                                                   -axfroffset,
                                                   &mut ansp,
                                                   daemon.auth_ttl,
                                                   0,
                                                   28,
                                                   1,
                                                   b"6\x00" ,
                                                   (if !cut.is_null() {
                                                        intr.name
                                                    } else {
                                                        0
                                                    }),
                                                   &mut (*addrlist_2).addr) != 0 {
                            anscount += 1
                        }
                        addrlist_2 = (*addrlist_2).next
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.'  }
                }
                intr = intr.next
            }
            a = daemon.cnames;
            while !a.is_null() {
                if in_zone(zone, (*a).alias, &mut cut) != 0 {
                    strcpy(name, (*a).target);
                    if strchr(name, '.').is_null() {
                        strcat(name,
                               b".\x00" );
                        strcat(name, zone.domain);
                    }
                    if !cut.is_null() {
                        *cut = 0
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc,
                                           -axfroffset,
                                           &mut ansp,
                                           daemon.auth_ttl,
                                           0,
                                           5 ,
                                           1 ,
                                           b"d\x00",
                                           if !cut.is_null() {
                                               (*a).ali                                    } else { 0  },
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
                if crecp.flags &
                       ((1 __b) << 7 |
                            (1 __b) << 8) != 0 &&
                       crecp.flags &
                           ((1 __b) << 5 |
                                (1 __b) << 10) == 0
                       &&
                       crecp.flags &
                           (1 __b) << 3 != 0 {
                    if crecp.flags &
                           (1 __b) << 4 != 0 &&
                           daemon.options[(20).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                    ).wrapping_mul(8
                                                                                                                           ))
                                                         ] &
                               (1 __b) <<
                                   (20 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                ).wrapping_mul(8                          libc::c_int                   ))
                               == 0 {
                        let mut cache_name: &mut String =
                            cache_get_name(crecp);
                        if strchr(cache_name, '.' as i32).is_null() &&
                               (local_query != 0 ||
                                    filter_zone(zone,
                                                (crecp.flags &
                                                     ((1 __b) <<
                                                          8 |
                                                          (1 __b)
                                                              <<
                                                              7        libc::c_int))
                                                   ,
                                                &mut crecp.addr) != 0) &&
                               add_resource_record(header, limit,
                                                   &mut trunc    ,
                                                   -axfroffset,
                                                   &mut ansp    ,
                                                   daemon.auth_ttl,
                                                   0,
                                                   (if crecp.flags &
                                                           (1 __b)
                                                               <<
                                                               8         libc::c_int
                                                           != 0 {
                                                        28
                                                    } else {
                                                        1
                                                    }) ,
                                                   1 ,
                                                   (if crecp.flags &
                                                           (1 __b)
                                                               <<
                                                               7         libc::c_int
                                                           != 0 {
                                                        b"4\x00"
                                                    } else {
                                                        b"6\x00"
                                                    }) ,
                                                   cache_name,
                                                   &mut crecp.addr) != 0 {
                            anscount += 1
                        }
                    }
                    if crecp.flags &
                           (1 __b) << 6 != 0 ||
                           crecp.flags &
                               (1 __b) << 4 != 0 &&
                               daemon.options[(20).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))] &
                                   (1 __b) <<
                                       (20).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
                                   != 0 {
                        strcpy(name, cache_get_name(crecp));
                        if in_zone(zone, name, &mut cut) != 0 &&
                               (local_query != 0 ||
                                    filter_zone(zone,
                                                (crecp.flags &
                                                     ((1 __b) <<
                                                          8 |
                                                          (1 __b)
                                                              <<
                                                              7        libc::c_int))
                                                   ,
                                                &mut crecp.addr) != 0) {
                            if !cut.is_null() {
                                *cut = 0
                            }
                            if add_resource_record(header, limit,
                                                   &mut trunc    ,
                                                   -axfroffset,
                                                   &mut ansp    ,
                                                   daemon.auth_ttl,
                                                   0,
                                                   if crecp.flags &
                                                          (1 __b)
                                                              <<
                                                              8
                                                          != 0 {
                                                       28
                                                   } else { 1 }
                                                       ,
                                                   1 ,
                                                   if crecp.flags &
                                                          (1 __b)
                                                              <<
                                                              7
                                                          != 0 {
                                                       b"4\x00"
                                                   } else {
                                                       b"6\x00"
                                                   } ,
                                                   if !cut.is_null() {
                                                       name
                                                   } else {
                                                       0
                                                   },
                                                   &mut crecp.addr) != 0 {
                                anscount += 1
                            }
                        }
                    }
                }
            }
            /* repeat SOA as last record */
            if add_resource_record(header, limit,
                                   &mut trunc, axfroffset,
                                   &mut ansp,
                                   daemon.auth_ttl,
                                   0,
                                   6 ,
                                   1 ,
                                   b"ddlllll\x00",
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
    header.ancount = __bswap_16(anscount);
    header.nscount = __bswap_16(authcount);
    header.arcount = __bswap_16(0);
    /* Advertise our packet size limit in our reply */
    if have_pseudoheader != 0 {
        return add_pseudoheader(header,
                                ansp.wrapping_offset_from(header    mut Vec<u8>)
                                    ,
                                limit,
                                daemon.edns_pktsz,
                                0, 0,
                                0 , do_bit,
                                0)
    }
    return ansp.wrapping_offset_from(header)  ;
}
