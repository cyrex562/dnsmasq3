
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
use crate::defines::{addrlist, all_addr, in_addr, __bswap_32, in_addr_t};
use crate::util::{is_same_net, is_same_net6};

fn find_addrlist(list: &mut addrlist, flag: libc::c_int, addr_u: &mut all_addr) -> addrlist {
    loop  {
        if (*list).flags & 2 as libc::c_int == 0 {
            let mut netmask: in_addr = in_addr{s_addr: 0,};
            let mut addr: in_addr = (*addr_u).addr4;
            if !(flag as libc::c_uint &
                     (1 as libc::c_uint) << 7 as libc::c_int == 0) { netmask.s_addr = __bswap_32((!(0 as libc::c_int as in_addr_t)) << 32 as libc::c_int - (*list).prefixlen);
                if is_same_net(addr, (*list).addr.addr4, netmask) != 0 {
                    return list
                }
            }
        } else if is_same_net6(&mut (*addr_u).addr6, &mut (*list).addr.addr6,
                               (*list).prefixlen) != 0 {
            return list
        }
        list = (*list).next;
        if list.is_null() { break ; }
    }
    return 0;
}


unsafe extern "C" fn find_subnet(mut zone: *mut auth_zone,
                                 mut flag: libc::c_int,
                                 mut addr_u: *mut all_addr) -> *mut addrlist {
    if (*zone).subnet.is_null() { return 0 as *mut addrlist }
    return find_addrlist((*zone).subnet, flag, addr_u);
}
unsafe extern "C" fn find_exclude(mut zone: *mut auth_zone,
                                  mut flag: libc::c_int,
                                  mut addr_u: *mut all_addr)
 -> *mut addrlist {
    if (*zone).exclude.is_null() { return 0 as *mut addrlist }
    return find_addrlist((*zone).exclude, flag, addr_u);
}
unsafe extern "C" fn filter_zone(mut zone: *mut auth_zone,
                                 mut flag: libc::c_int,
                                 mut addr_u: *mut all_addr) -> libc::c_int {
    if !find_exclude(zone, flag, addr_u).is_null() { return 0 as libc::c_int }
    /* No subnets specified, no filter */
    if (*zone).subnet.is_null() { return 1 as libc::c_int }
    return (find_subnet(zone, flag, addr_u) !=
                0 as *mut libc::c_void as *mut addrlist) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn in_zone(mut zone: *mut auth_zone,
                                 mut name: *mut libc::c_char,
                                 mut cut: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut namelen: size_t = strlen(name);
    let mut domainlen: size_t = strlen((*zone).domain);
    if !cut.is_null() { *cut = 0 as *mut libc::c_char }
    if namelen >= domainlen &&
           hostname_isequal((*zone).domain,
                            &mut *name.offset(namelen.wrapping_sub(domainlen)
                                                  as isize)) != 0 {
        if namelen == domainlen { return 1 as libc::c_int }
        if *name.offset(namelen.wrapping_sub(domainlen).wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                            as isize) as libc::c_int == '.' as i32 {
            if !cut.is_null() {
                *cut =
                    &mut *name.offset(namelen.wrapping_sub(domainlen).wrapping_sub(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong)
                                          as isize) as *mut libc::c_char
            }
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn answer_auth(mut header: *mut dns_header,
                                     mut limit: *mut libc::c_char,
                                     mut qlen: size_t, mut now: time_t,
                                     mut peer_addr: *mut mysockaddr,
                                     mut local_query: libc::c_int,
                                     mut do_bit: libc::c_int,
                                     mut have_pseudoheader: libc::c_int)
 -> size_t {
    let mut name: *mut libc::c_char = (*dnsmasq_daemon).namebuff;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ansp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut qtype: libc::c_int = 0;
    let mut qclass: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut nameoffset: libc::c_int = 0;
    let mut axfroffset: libc::c_int = 0 as libc::c_int;
    let mut q: libc::c_int = 0;
    let mut anscount: libc::c_int = 0 as libc::c_int;
    let mut authcount: libc::c_int = 0 as libc::c_int;
    let mut crecp: *mut crec = 0 as *mut crec;
    let mut auth: libc::c_int = (local_query == 0) as libc::c_int;
    let mut trunc: libc::c_int = 0 as libc::c_int;
    let mut nxdomain: libc::c_int = 1 as libc::c_int;
    let mut soa: libc::c_int = 0 as libc::c_int;
    let mut ns: libc::c_int = 0 as libc::c_int;
    let mut axfr: libc::c_int = 0 as libc::c_int;
    let mut zone: *mut auth_zone = 0 as *mut auth_zone;
    let mut subnet: *mut addrlist = 0 as *mut addrlist;
    let mut cut: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rec: *mut mx_srv_record = 0 as *mut mx_srv_record;
    let mut move_0: *mut mx_srv_record = 0 as *mut mx_srv_record;
    let mut up: *mut *mut mx_srv_record = 0 as *mut *mut mx_srv_record;
    let mut txt: *mut txt_record = 0 as *mut txt_record;
    let mut intr: *mut interface_name = 0 as *mut interface_name;
    let mut na: *mut naptr = 0 as *mut naptr;
    let mut addr: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut a: *mut cname = 0 as *mut cname;
    let mut candidate: *mut cname = 0 as *mut cname;
    let mut wclen: libc::c_uint = 0;
    if __bswap_16((*header).qdcount) as libc::c_int == 0 as libc::c_int ||
           ((*header).hb3 as libc::c_int & 0x78 as libc::c_int) >>
               3 as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int as size_t
    }
    /* determine end of question section (we put answers there) */
    ansp = skip_questions(header, qlen); /* bad packet */
    if ansp.is_null() { return 0 as libc::c_int as size_t }
    /* now process each question, answers go in RRs after the question */
    p = header.offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
    let mut current_block_247: u64;
    q = __bswap_16((*header).qdcount) as libc::c_int;
    while q != 0 as libc::c_int {
        let mut flag: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut found: libc::c_int = 0 as libc::c_int;
        let mut cname_wildcard: libc::c_int = 0 as libc::c_int;
        /* save pointer to name for copying into answers */
        nameoffset =
            p.wrapping_offset_from(header as *mut libc::c_uchar) as
                libc::c_long as libc::c_int;
        /* now extract name as .-concatenated string into name */
        if extract_name(header, qlen, &mut p, name, 1 as libc::c_int,
                        4 as libc::c_int) == 0 {
            return 0 as libc::c_int as size_t
        } /* bad packet */
        let mut t_cp: *mut libc::c_uchar = p; /* must be bare name */
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
        if qclass != 1 as libc::c_int {
            auth = 0 as libc::c_int
        } else {
            if (qtype == 12 as libc::c_int || qtype == 6 as libc::c_int ||
                    qtype == 2 as libc::c_int) &&
                   {
                       flag =
                           in_arpa_name_2_addr(name, &mut addr) as
                               libc::c_uint;
                       (flag) != 0
                   } && local_query == 0 {
                zone = (*dnsmasq_daemon).auth_zones;
                while !zone.is_null() {
                    subnet =
                        find_subnet(zone, flag as libc::c_int, &mut addr);
                    if !subnet.is_null() { break ; }
                    zone = (*zone).next
                }
                if zone.is_null() {
                    auth = 0 as libc::c_int;
                    current_block_247 = 17860125682698302841;
                } else {
                    if qtype == 6 as libc::c_int {
                        soa = 1 as libc::c_int;
                        found = 1 as libc::c_int
                    } else if qtype == 2 as libc::c_int {
                        ns = 1 as libc::c_int;
                        found = 1 as libc::c_int
                    }
                    current_block_247 = 4567019141635105728;
                }
            } else { current_block_247 = 4567019141635105728; }
            match current_block_247 {
                17860125682698302841 => { }
                _ => {
                    if qtype == 12 as libc::c_int && flag != 0 {
                        intr = 0 as *mut interface_name;
                        if flag == (1 as libc::c_uint) << 7 as libc::c_int {
                            intr = (*dnsmasq_daemon).int_names;
                            while !intr.is_null() {
                                let mut addrlist: *mut addrlist =
                                    0 as *mut addrlist;
                                addrlist = (*intr).addr;
                                while !addrlist.is_null() {
                                    if (*addrlist).flags & 2 as libc::c_int ==
                                           0 &&
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
                        } else if flag ==
                                      (1 as libc::c_uint) << 8 as libc::c_int
                         {
                            intr = (*dnsmasq_daemon).int_names;
                            while !intr.is_null() {
                                let mut addrlist_0: *mut addrlist =
                                    0 as *mut addrlist;
                                addrlist_0 = (*intr).addr;
                                while !addrlist_0.is_null() {
                                    if (*addrlist_0).flags & 2 as libc::c_int
                                           != 0 &&
                                           ({
                                                let mut __a: *const in6_addr =
                                                    &mut addr.addr6 as
                                                        *mut in6_addr as
                                                        *const in6_addr;
                                                let mut __b: *const in6_addr =
                                                    &mut (*addrlist_0).addr.addr6
                                                        as *mut in6_addr as
                                                        *const in6_addr;
                                                ((*__a).__in6_u.__u6_addr32[0
                                                                                as
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
                            if local_query != 0 ||
                                   in_zone(zone, (*intr).name,
                                           0 as *mut *mut libc::c_char) != 0 {
                                found = 1 as libc::c_int;
                                log_query(flag |
                                              (1 as libc::c_uint) <<
                                                  2 as libc::c_int |
                                              (1 as libc::c_uint) <<
                                                  13 as libc::c_int,
                                          (*intr).name, &mut addr,
                                          0 as *mut libc::c_char);
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       (*dnsmasq_daemon).auth_ttl,
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
                                                       (*intr).name) != 0 {
                                    anscount += 1
                                }
                            }
                        }
                        crecp =
                            cache_find_by_addr(0 as *mut crec, &mut addr, now,
                                               flag);
                        if !crecp.is_null() {
                            loop  {
                                strcpy(name, cache_get_name(crecp));
                                if (*crecp).flags &
                                       (1 as libc::c_uint) << 4 as libc::c_int
                                       != 0 &&
                                       (*dnsmasq_daemon).options[(20 as
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
                                               (20 as libc::c_int as
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
                                            0 as libc::c_int as libc::c_char
                                    }
                                    /* add  external domain */
                                    if !zone.is_null() {
                                        strcat(name,
                                               b".\x00" as *const u8 as
                                                   *const libc::c_char);
                                        strcat(name, (*zone).domain);
                                    }
                                    log_query(flag |
                                                  (1 as libc::c_uint) <<
                                                      4 as libc::c_int |
                                                  (1 as libc::c_uint) <<
                                                      2 as libc::c_int, name,
                                              &mut addr,
                                              record_source((*crecp).uid));
                                    found = 1 as libc::c_int;
                                    if add_resource_record(header, limit,
                                                           &mut trunc as
                                                               *mut libc::c_int,
                                                           nameoffset,
                                                           &mut ansp as
                                                               *mut *mut libc::c_uchar,
                                                           (*dnsmasq_daemon).auth_ttl,
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
                                                           name) != 0 {
                                        anscount += 1
                                    }
                                } else if (*crecp).flags &
                                              ((1 as libc::c_uint) <<
                                                   4 as libc::c_int |
                                                   (1 as libc::c_uint) <<
                                                       6 as libc::c_int) != 0
                                              &&
                                              (local_query != 0 ||
                                                   in_zone(zone, name,
                                                           0 as
                                                               *mut *mut libc::c_char)
                                                       != 0) {
                                    log_query((*crecp).flags &
                                                  !((1 as libc::c_uint) <<
                                                        3 as libc::c_int),
                                              name, &mut addr,
                                              record_source((*crecp).uid));
                                    found = 1 as libc::c_int;
                                    if add_resource_record(header, limit,
                                                           &mut trunc as
                                                               *mut libc::c_int,
                                                           nameoffset,
                                                           &mut ansp as
                                                               *mut *mut libc::c_uchar,
                                                           (*dnsmasq_daemon).auth_ttl,
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
                            nxdomain = 0 as libc::c_int
                        } else {
                            log_query(flag |
                                          (1 as libc::c_uint) <<
                                              5 as libc::c_int |
                                          (1 as libc::c_uint) <<
                                              10 as libc::c_int |
                                          (1 as libc::c_uint) <<
                                              2 as libc::c_int |
                                          (if auth != 0 {
                                               ((1 as libc::c_uint)) <<
                                                   21 as libc::c_int
                                           } else {
                                               0 as libc::c_int as
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
                                zone = (*dnsmasq_daemon).auth_zones;
                                while !zone.is_null() {
                                    if in_zone(zone, name, &mut cut) != 0 {
                                        break ;
                                    }
                                    zone = (*zone).next
                                }
                                if zone.is_null() {
                                    auth = 0 as libc::c_int;
                                    break ;
                                }
                            }
                            rec = (*dnsmasq_daemon).mxnames;
                            while !rec.is_null() {
                                if (*rec).issrv == 0 &&
                                       {
                                           rc =
                                               hostname_issubdomain(name,
                                                                    (*rec).name);
                                           (rc) != 0
                                       } {
                                    nxdomain = 0 as libc::c_int;
                                    if rc == 2 as libc::c_int &&
                                           qtype == 15 as libc::c_int {
                                        found = 1 as libc::c_int;
                                        log_query((1 as libc::c_uint) <<
                                                      13 as libc::c_int |
                                                      (1 as libc::c_uint) <<
                                                          17 as libc::c_int,
                                                  name, 0 as *mut all_addr,
                                                  b"<MX>\x00" as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char);
                                        if add_resource_record(header, limit,
                                                               &mut trunc as
                                                                   *mut libc::c_int,
                                                               nameoffset,
                                                               &mut ansp as
                                                                   *mut *mut libc::c_uchar,
                                                               (*dnsmasq_daemon).auth_ttl,
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
                            move_0 = 0 as *mut mx_srv_record;
                            up = &mut (*dnsmasq_daemon).mxnames;
                            rec = (*dnsmasq_daemon).mxnames;
                            while !rec.is_null() {
                                if (*rec).issrv != 0 &&
                                       {
                                           rc =
                                               hostname_issubdomain(name,
                                                                    (*rec).name);
                                           (rc) != 0
                                       } {
                                    nxdomain = 0 as libc::c_int;
                                    if rc == 2 as libc::c_int &&
                                           qtype == 33 as libc::c_int {
                                        found = 1 as libc::c_int;
                                        log_query((1 as libc::c_uint) <<
                                                      13 as libc::c_int |
                                                      (1 as libc::c_uint) <<
                                                          17 as libc::c_int,
                                                  name, 0 as *mut all_addr,
                                                  b"<SRV>\x00" as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char);
                                        if add_resource_record(header, limit,
                                                               &mut trunc as
                                                                   *mut libc::c_int,
                                                               nameoffset,
                                                               &mut ansp as
                                                                   *mut *mut libc::c_uchar,
                                                               (*dnsmasq_daemon).auth_ttl,
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
                                (*move_0).next = 0 as *mut mx_srv_record
                            }
                            txt = (*dnsmasq_daemon).rr;
                            while !txt.is_null() {
                                rc = hostname_issubdomain(name, (*txt).name);
                                if rc != 0 {
                                    nxdomain = 0 as libc::c_int;
                                    if rc == 2 as libc::c_int &&
                                           (*txt).class as libc::c_int ==
                                               qtype {
                                        found = 1 as libc::c_int;
                                        log_query((1 as libc::c_uint) <<
                                                      13 as libc::c_int |
                                                      (1 as libc::c_uint) <<
                                                          17 as libc::c_int,
                                                  name, 0 as *mut all_addr,
                                                  querystr(0 as
                                                               *mut libc::c_char,
                                                           (*txt).class));
                                        if add_resource_record(header, limit,
                                                               &mut trunc as
                                                                   *mut libc::c_int,
                                                               nameoffset,
                                                               &mut ansp as
                                                                   *mut *mut libc::c_uchar,
                                                               (*dnsmasq_daemon).auth_ttl,
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
                            txt = (*dnsmasq_daemon).txt;
                            while !txt.is_null() {
                                if (*txt).class as libc::c_int ==
                                       1 as libc::c_int &&
                                       {
                                           rc =
                                               hostname_issubdomain(name,
                                                                    (*txt).name);
                                           (rc) != 0
                                       } {
                                    nxdomain = 0 as libc::c_int;
                                    if rc == 2 as libc::c_int &&
                                           qtype == 16 as libc::c_int {
                                        found = 1 as libc::c_int;
                                        log_query((1 as libc::c_uint) <<
                                                      13 as libc::c_int |
                                                      (1 as libc::c_uint) <<
                                                          17 as libc::c_int,
                                                  name, 0 as *mut all_addr,
                                                  b"<TXT>\x00" as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char);
                                        if add_resource_record(header, limit,
                                                               &mut trunc as
                                                                   *mut libc::c_int,
                                                               nameoffset,
                                                               &mut ansp as
                                                                   *mut *mut libc::c_uchar,
                                                               (*dnsmasq_daemon).auth_ttl,
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
                            na = (*dnsmasq_daemon).naptr;
                            while !na.is_null() {
                                rc = hostname_issubdomain(name, (*na).name);
                                if rc != 0 {
                                    nxdomain = 0 as libc::c_int;
                                    if rc == 2 as libc::c_int &&
                                           qtype == 35 as libc::c_int {
                                        found = 1 as libc::c_int;
                                        log_query((1 as libc::c_uint) <<
                                                      13 as libc::c_int |
                                                      (1 as libc::c_uint) <<
                                                          17 as libc::c_int,
                                                  name, 0 as *mut all_addr,
                                                  b"<NAPTR>\x00" as *const u8
                                                      as *const libc::c_char
                                                      as *mut libc::c_char);
                                        if add_resource_record(header, limit,
                                                               &mut trunc as
                                                                   *mut libc::c_int,
                                                               nameoffset,
                                                               &mut ansp as
                                                                   *mut *mut libc::c_uchar,
                                                               (*dnsmasq_daemon).auth_ttl,
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
                            if qtype == 1 as libc::c_int {
                                flag = (1 as libc::c_uint) << 7 as libc::c_int
                            }
                            if qtype == 28 as libc::c_int {
                                flag = (1 as libc::c_uint) << 8 as libc::c_int
                            }
                            intr = (*dnsmasq_daemon).int_names;
                            while !intr.is_null() {
                                rc = hostname_issubdomain(name, (*intr).name);
                                if rc != 0 {
                                    let mut addrlist_1: *mut addrlist =
                                        0 as *mut addrlist;
                                    nxdomain = 0 as libc::c_int;
                                    if rc == 2 as libc::c_int && flag != 0 {
                                        addrlist_1 = (*intr).addr;
                                        while !addrlist_1.is_null() {
                                            if (if (*addrlist_1).flags &
                                                       2 as libc::c_int != 0 {
                                                    28 as libc::c_int
                                                } else { 1 as libc::c_int })
                                                   == qtype &&
                                                   (local_query != 0 ||
                                                        filter_zone(zone,
                                                                    flag as
                                                                        libc::c_int,
                                                                    &mut (*addrlist_1).addr)
                                                            != 0) {
                                                if !((*addrlist_1).flags &
                                                         4 as libc::c_int !=
                                                         0) {
                                                    found = 1 as libc::c_int;
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
                                                                           (*dnsmasq_daemon).auth_ttl,
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
                                                                               *mut all_addr)
                                                           != 0 {
                                                        anscount += 1
                                                    }
                                                }
                                            }
                                            addrlist_1 = (*addrlist_1).next
                                        }
                                    }
                                }
                                intr = (*intr).next
                            }
                            if cut.is_null() {
                                nxdomain = 0 as libc::c_int;
                                if qtype == 6 as libc::c_int {
                                    soa = 1 as libc::c_int;
                                    auth = soa;
                                    found = 1 as libc::c_int;
                                    log_query((1 as libc::c_uint) <<
                                                  17 as libc::c_int |
                                                  (1 as libc::c_uint) <<
                                                      21 as libc::c_int,
                                              (*zone).domain,
                                              0 as *mut all_addr,
                                              b"<SOA>\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                                } else if qtype == 252 as libc::c_int {
                                    let mut peers: *mut iname =
                                        0 as *mut iname;
                                    if (*peer_addr).sa.sa_family as
                                           libc::c_int == 2 as libc::c_int {
                                        (*peer_addr).in_0.sin_port =
                                            0 as libc::c_int as in_port_t
                                    } else {
                                        (*peer_addr).in6.sin6_port =
                                            0 as libc::c_int as in_port_t;
                                        (*peer_addr).in6.sin6_scope_id =
                                            0 as libc::c_int as uint32_t
                                    }
                                    peers = (*dnsmasq_daemon).auth_peers;
                                    while !peers.is_null() {
                                        if sockaddr_isequal(peer_addr,
                                                            &mut (*peers).addr)
                                               != 0 {
                                            break ;
                                        }
                                        peers = (*peers).next
                                    }
                                    /* Refuse all AXFR unless --auth-sec-servers or auth-peers is set */
                                    if (*dnsmasq_daemon).secondary_forward_server.is_null()
                                           &&
                                           (*dnsmasq_daemon).auth_peers.is_null()
                                           ||
                                           !(*dnsmasq_daemon).auth_peers.is_null()
                                               && peers.is_null() {
                                        if (*peer_addr).sa.sa_family as
                                               libc::c_int == 2 as libc::c_int
                                           {
                                            inet_ntop(2 as libc::c_int,
                                                      &mut (*peer_addr).in_0.sin_addr
                                                          as *mut in_addr as
                                                          *const libc::c_void,
                                                      (*dnsmasq_daemon).addrbuff,
                                                      46 as libc::c_int as
                                                          socklen_t); /* inhibits auth section */
                                        } else {
                                            inet_ntop(10 as libc::c_int,
                                                      &mut (*peer_addr).in6.sin6_addr
                                                          as *mut in6_addr as
                                                          *const libc::c_void,
                                                      (*dnsmasq_daemon).addrbuff,
                                                      46 as libc::c_int as
                                                          socklen_t); /* ensure we include NS records! */
                                        } /* inhibits auth section */
                                        my_syslog(4 as libc::c_int,
                                                  b"ignoring zone transfer request from %s\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  (*dnsmasq_daemon).addrbuff); /* remove domain part */
                                        return 0 as libc::c_int as size_t
                                    }
                                    auth = 1 as libc::c_int;
                                    soa = 1 as libc::c_int;
                                    ns = 1 as libc::c_int;
                                    axfr = 1 as libc::c_int;
                                    found = 1 as libc::c_int;
                                    axfroffset = nameoffset;
                                    log_query((1 as libc::c_uint) <<
                                                  17 as libc::c_int |
                                                  (1 as libc::c_uint) <<
                                                      21 as libc::c_int,
                                              (*zone).domain,
                                              0 as *mut all_addr,
                                              b"<AXFR>\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                                } else if qtype == 2 as libc::c_int {
                                    auth = 1 as libc::c_int;
                                    ns = 1 as libc::c_int;
                                    found = 1 as libc::c_int;
                                    log_query((1 as libc::c_uint) <<
                                                  17 as libc::c_int |
                                                  (1 as libc::c_uint) <<
                                                      21 as libc::c_int,
                                              (*zone).domain,
                                              0 as *mut all_addr,
                                              b"<NS>\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char);
                                }
                            }
                            if (*dnsmasq_daemon).options[(20 as libc::c_int as
                                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                               as
                                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_ulong))
                                                             as usize] &
                                   (1 as libc::c_uint) <<
                                       (20 as libc::c_int as
                                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong))
                                   == 0 && !cut.is_null() {
                                *cut = 0 as libc::c_int as libc::c_char;
                                if strchr(name, '.' as i32).is_null() &&
                                       {
                                           crecp =
                                               cache_find_by_name(0 as
                                                                      *mut crec,
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
                                           (1 as libc::c_uint) <<
                                               4 as libc::c_int != 0 {
                                        loop  {
                                            nxdomain = 0 as libc::c_int;
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
                                                    0 as libc::c_int as
                                                        libc::c_char;
                                                found = 1 as libc::c_int;
                                                if add_resource_record(header,
                                                                       limit,
                                                                       &mut trunc
                                                                           as
                                                                           *mut libc::c_int,
                                                                       nameoffset,
                                                                       &mut ansp
                                                                           as
                                                                           *mut *mut libc::c_uchar,
                                                                       (*dnsmasq_daemon).auth_ttl,
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
                                                                           *mut all_addr)
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
                                cache_find_by_name(0 as *mut crec, name, now,
                                                   (1 as libc::c_uint) <<
                                                       7 as libc::c_int |
                                                       (1 as libc::c_uint) <<
                                                           8 as libc::c_int);
                            if !crecp.is_null() {
                                if (*crecp).flags &
                                       (1 as libc::c_uint) << 6 as libc::c_int
                                       != 0 ||
                                       (*crecp).flags &
                                           (1 as libc::c_uint) <<
                                               4 as libc::c_int != 0 &&
                                           (*dnsmasq_daemon).options[(20 as
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
                                               (1 as libc::c_uint) <<
                                                   (20 as libc::c_int as
                                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                         as
                                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         libc::c_ulong))
                                               != 0 {
                                    loop  {
                                        nxdomain = 0 as libc::c_int;
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
                                            found = 1 as libc::c_int;
                                            if add_resource_record(header,
                                                                   limit,
                                                                   &mut trunc
                                                                       as
                                                                       *mut libc::c_int,
                                                                   nameoffset,
                                                                   &mut ansp
                                                                       as
                                                                       *mut *mut libc::c_uchar,
                                                                   (*dnsmasq_daemon).auth_ttl,
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
                                                                       *mut all_addr)
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
                            wclen = 0 as libc::c_int as libc::c_uint;
                            candidate = 0 as *mut cname;
                            a = (*dnsmasq_daemon).cnames;
                            while !a.is_null() {
                                if *(*a).alias.offset(0 as libc::c_int as
                                                          isize) as
                                       libc::c_int == '*' as i32 {
                                    let mut test: *mut libc::c_char = name;
                                    loop  {
                                        test =
                                            strchr(test.offset(1 as
                                                                   libc::c_int
                                                                   as isize),
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
                                                strlen(test) as libc::c_uint;
                                            candidate = a;
                                            cname_wildcard = 1 as libc::c_int
                                        }
                                        break ;
                                    }
                                } else if hostname_isequal((*a).alias, name)
                                              != 0 &&
                                              strlen((*a).alias) >
                                                  wclen as libc::c_ulong {
                                    /* Simple case, no wildcard */
                                    wclen =
                                        strlen((*a).alias) as libc::c_uint;
                                    candidate = a
                                }
                                a = (*a).next
                            }
                            if !candidate.is_null() {
                                log_query((1 as libc::c_uint) <<
                                              13 as libc::c_int |
                                              (1 as libc::c_uint) <<
                                                  11 as libc::c_int, name,
                                          0 as *mut all_addr,
                                          0 as *mut libc::c_char);
                                strcpy(name, (*candidate).target);
                                if strchr(name, '.' as i32).is_null() {
                                    strcat(name,
                                           b".\x00" as *const u8 as
                                               *const libc::c_char);
                                    strcat(name, (*zone).domain);
                                }
                                found = 1 as libc::c_int;
                                if add_resource_record(header, limit,
                                                       &mut trunc as
                                                           *mut libc::c_int,
                                                       nameoffset,
                                                       &mut ansp as
                                                           *mut *mut libc::c_uchar,
                                                       (*dnsmasq_daemon).auth_ttl,
                                                       &mut nameoffset as
                                                           *mut libc::c_int,
                                                       5 as libc::c_int as
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
                            } else {
                                if cache_find_non_terminal(name, now) != 0 {
                                    nxdomain = 0 as libc::c_int
                                }
                                log_query(flag |
                                              (1 as libc::c_uint) <<
                                                  5 as libc::c_int |
                                              (if nxdomain != 0 {
                                                   ((1 as libc::c_uint)) <<
                                                       10 as libc::c_int
                                               } else {
                                                   0 as libc::c_int as
                                                       libc::c_uint
                                               }) |
                                              (1 as libc::c_uint) <<
                                                  3 as libc::c_int |
                                              (1 as libc::c_uint) <<
                                                  21 as libc::c_int, name,
                                          0 as *mut all_addr,
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
        let mut offset: libc::c_int = 0 as libc::c_int;
        if subnet.is_null() {
            authname = (*zone).domain
        } else {
            /* handle NS and SOA for PTR records */
            authname = name;
            if (*subnet).flags & 2 as libc::c_int == 0 {
                let mut a_0: in_addr_t =
                    __bswap_32((*subnet).addr.addr4.s_addr) >>
                        8 as libc::c_int;
                let mut p_1: *mut libc::c_char = name;
                if (*subnet).prefixlen >= 24 as libc::c_int {
                    p_1 =
                        p_1.offset(sprintf(p_1,
                                           b"%u.\x00" as *const u8 as
                                               *const libc::c_char,
                                           a_0 &
                                               0xff as libc::c_int as
                                                   libc::c_uint) as isize)
                }
                a_0 = a_0 >> 8 as libc::c_int;
                if (*subnet).prefixlen >= 16 as libc::c_int {
                    p_1 =
                        p_1.offset(sprintf(p_1,
                                           b"%u.\x00" as *const u8 as
                                               *const libc::c_char,
                                           a_0 &
                                               0xff as libc::c_int as
                                                   libc::c_uint) as isize)
                }
                a_0 = a_0 >> 8 as libc::c_int;
                p_1 =
                    p_1.offset(sprintf(p_1,
                                       b"%u.in-addr.arpa\x00" as *const u8 as
                                           *const libc::c_char,
                                       a_0 &
                                           0xff as libc::c_int as
                                               libc::c_uint) as isize)
            } else {
                let mut p_2: *mut libc::c_char = name;
                let mut i: libc::c_int = 0;
                i = (*subnet).prefixlen - 1 as libc::c_int;
                while i >= 0 as libc::c_int {
                    let mut dig: libc::c_int =
                        *(&mut (*subnet).addr.addr6 as *mut in6_addr as
                              *mut libc::c_uchar).offset((i >>
                                                              3 as
                                                                  libc::c_int)
                                                             as isize) as
                            libc::c_int;
                    p_2 =
                        p_2.offset(sprintf(p_2,
                                           b"%.1x.\x00" as *const u8 as
                                               *const libc::c_char,
                                           if i >> 2 as libc::c_int &
                                                  1 as libc::c_int != 0 {
                                               (dig) & 15 as libc::c_int
                                           } else {
                                               (dig) >> 4 as libc::c_int
                                           }) as isize);
                    i -= 4 as libc::c_int
                }
                p_2 =
                    p_2.offset(sprintf(p_2,
                                       b"ip6.arpa\x00" as *const u8 as
                                           *const libc::c_char) as isize)
            }
        }
        /* handle NS and SOA in auth section or for explicit queries */
        newoffset =
            ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                libc::c_long as libc::c_int;
        if (anscount == 0 as libc::c_int && ns == 0 || soa != 0) &&
               add_resource_record(header, limit,
                                   &mut trunc as *mut libc::c_int,
                                   0 as libc::c_int,
                                   &mut ansp as *mut *mut libc::c_uchar,
                                   (*dnsmasq_daemon).auth_ttl,
                                   0 as *mut libc::c_int,
                                   6 as libc::c_int as libc::c_ushort,
                                   1 as libc::c_int as libc::c_ushort,
                                   b"ddlllll\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, authname,
                                   (*dnsmasq_daemon).authserver,
                                   (*dnsmasq_daemon).hostmaster,
                                   (*dnsmasq_daemon).soa_sn,
                                   (*dnsmasq_daemon).soa_refresh,
                                   (*dnsmasq_daemon).soa_retry,
                                   (*dnsmasq_daemon).soa_expiry,
                                   (*dnsmasq_daemon).auth_ttl) != 0 {
            offset = newoffset;
            if soa != 0 { anscount += 1 } else { authcount += 1 }
        }
        if anscount != 0 as libc::c_int || ns != 0 {
            let mut secondary: *mut name_list = 0 as *mut name_list;
            /* Only include the machine running dnsmasq if it's acting as an auth server */
            if !(*dnsmasq_daemon).authinterface.is_null() {
                newoffset =
                    ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
                        libc::c_long as libc::c_int;
                if add_resource_record(header, limit,
                                       &mut trunc as *mut libc::c_int,
                                       -offset,
                                       &mut ansp as *mut *mut libc::c_uchar,
                                       (*dnsmasq_daemon).auth_ttl,
                                       0 as *mut libc::c_int,
                                       2 as libc::c_int as libc::c_ushort,
                                       1 as libc::c_int as libc::c_ushort,
                                       b"d\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char,
                                       if offset == 0 as libc::c_int {
                                           authname
                                       } else { 0 as *mut libc::c_char },
                                       (*dnsmasq_daemon).authserver) != 0 {
                    if offset == 0 as libc::c_int { offset = newoffset }
                    if ns != 0 { anscount += 1 } else { authcount += 1 }
                }
            }
            if subnet.is_null() {
                secondary = (*dnsmasq_daemon).secondary_forward_server;
                while !secondary.is_null() {
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           offset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           (*dnsmasq_daemon).auth_ttl,
                                           0 as *mut libc::c_int,
                                           2 as libc::c_int as libc::c_ushort,
                                           1 as libc::c_int as libc::c_ushort,
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
            rec = (*dnsmasq_daemon).mxnames;
            while !rec.is_null() {
                if in_zone(zone, (*rec).name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0 as libc::c_int as libc::c_char
                    }
                    if (*rec).issrv != 0 {
                        if add_resource_record(header, limit,
                                               &mut trunc as *mut libc::c_int,
                                               -axfroffset,
                                               &mut ansp as
                                                   *mut *mut libc::c_uchar,
                                               (*dnsmasq_daemon).auth_ttl,
                                               0 as *mut libc::c_int,
                                               33 as libc::c_int as
                                                   libc::c_ushort,
                                               1 as libc::c_int as
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
                                                  (*dnsmasq_daemon).auth_ttl,
                                                  0 as *mut libc::c_int,
                                                  15 as libc::c_int as
                                                      libc::c_ushort,
                                                  1 as libc::c_int as
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
            txt = (*dnsmasq_daemon).rr;
            while !txt.is_null() {
                if in_zone(zone, (*txt).name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0 as libc::c_int as libc::c_char
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           -axfroffset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           (*dnsmasq_daemon).auth_ttl,
                                           0 as *mut libc::c_int,
                                           (*txt).class,
                                           1 as libc::c_int as libc::c_ushort,
                                           b"t\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char,
                                           if !cut.is_null() {
                                               (*txt).name
                                           } else { 0 as *mut libc::c_char },
                                           (*txt).len as libc::c_int,
                                           (*txt).txt) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.' as i32 as libc::c_char }
                }
                txt = (*txt).next
            }
            txt = (*dnsmasq_daemon).txt;
            while !txt.is_null() {
                if (*txt).class as libc::c_int == 1 as libc::c_int &&
                       in_zone(zone, (*txt).name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0 as libc::c_int as libc::c_char
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           -axfroffset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           (*dnsmasq_daemon).auth_ttl,
                                           0 as *mut libc::c_int,
                                           16 as libc::c_int as
                                               libc::c_ushort,
                                           1 as libc::c_int as libc::c_ushort,
                                           b"t\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char,
                                           if !cut.is_null() {
                                               (*txt).name
                                           } else { 0 as *mut libc::c_char },
                                           (*txt).len as libc::c_int,
                                           (*txt).txt) != 0 {
                        anscount += 1
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.' as i32 as libc::c_char }
                }
                txt = (*txt).next
            }
            na = (*dnsmasq_daemon).naptr;
            while !na.is_null() {
                if in_zone(zone, (*na).name, &mut cut) != 0 {
                    if !cut.is_null() {
                        *cut = 0 as libc::c_int as libc::c_char
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           -axfroffset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           (*dnsmasq_daemon).auth_ttl,
                                           0 as *mut libc::c_int,
                                           35 as libc::c_int as
                                               libc::c_ushort,
                                           1 as libc::c_int as libc::c_ushort,
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
            intr = (*dnsmasq_daemon).int_names;
            while !intr.is_null() {
                if in_zone(zone, (*intr).name, &mut cut) != 0 {
                    let mut addrlist_2: *mut addrlist = 0 as *mut addrlist;
                    if !cut.is_null() {
                        *cut = 0 as libc::c_int as libc::c_char
                    }
                    addrlist_2 = (*intr).addr;
                    while !addrlist_2.is_null() {
                        if (*addrlist_2).flags & 2 as libc::c_int == 0 &&
                               (local_query != 0 ||
                                    filter_zone(zone,
                                                ((1 as libc::c_uint) <<
                                                     7 as libc::c_int) as
                                                    libc::c_int,
                                                &mut (*addrlist_2).addr) != 0)
                               &&
                               add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   -axfroffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).auth_ttl,
                                                   0 as *mut libc::c_int,
                                                   1 as libc::c_int as
                                                       libc::c_ushort,
                                                   1 as libc::c_int as
                                                       libc::c_ushort,
                                                   b"4\x00" as *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char,
                                                   (if !cut.is_null() {
                                                        (*intr).name
                                                    } else {
                                                        0 as *mut libc::c_char
                                                    }),
                                                   &mut (*addrlist_2).addr as
                                                       *mut all_addr) != 0 {
                            anscount += 1
                        }
                        addrlist_2 = (*addrlist_2).next
                    }
                    addrlist_2 = (*intr).addr;
                    while !addrlist_2.is_null() {
                        if (*addrlist_2).flags & 2 as libc::c_int != 0 &&
                               (local_query != 0 ||
                                    filter_zone(zone,
                                                ((1 as libc::c_uint) <<
                                                     8 as libc::c_int) as
                                                    libc::c_int,
                                                &mut (*addrlist_2).addr) != 0)
                               &&
                               add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   -axfroffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).auth_ttl,
                                                   0 as *mut libc::c_int,
                                                   28 as libc::c_int as
                                                       libc::c_ushort,
                                                   1 as libc::c_int as
                                                       libc::c_ushort,
                                                   b"6\x00" as *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char,
                                                   (if !cut.is_null() {
                                                        (*intr).name
                                                    } else {
                                                        0 as *mut libc::c_char
                                                    }),
                                                   &mut (*addrlist_2).addr as
                                                       *mut all_addr) != 0 {
                            anscount += 1
                        }
                        addrlist_2 = (*addrlist_2).next
                    }
                    /* restore config data */
                    if !cut.is_null() { *cut = '.' as i32 as libc::c_char }
                }
                intr = (*intr).next
            }
            a = (*dnsmasq_daemon).cnames;
            while !a.is_null() {
                if in_zone(zone, (*a).alias, &mut cut) != 0 {
                    strcpy(name, (*a).target);
                    if strchr(name, '.' as i32).is_null() {
                        strcat(name,
                               b".\x00" as *const u8 as *const libc::c_char);
                        strcat(name, (*zone).domain);
                    }
                    if !cut.is_null() {
                        *cut = 0 as libc::c_int as libc::c_char
                    }
                    if add_resource_record(header, limit,
                                           &mut trunc as *mut libc::c_int,
                                           -axfroffset,
                                           &mut ansp as
                                               *mut *mut libc::c_uchar,
                                           (*dnsmasq_daemon).auth_ttl,
                                           0 as *mut libc::c_int,
                                           5 as libc::c_int as libc::c_ushort,
                                           1 as libc::c_int as libc::c_ushort,
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
            cache_enumerate(1 as libc::c_int);
            loop  {
                crecp = cache_enumerate(0 as libc::c_int);
                if crecp.is_null() { break ; }
                if (*crecp).flags &
                       ((1 as libc::c_uint) << 7 as libc::c_int |
                            (1 as libc::c_uint) << 8 as libc::c_int) != 0 &&
                       (*crecp).flags &
                           ((1 as libc::c_uint) << 5 as libc::c_int |
                                (1 as libc::c_uint) << 10 as libc::c_int) == 0
                       &&
                       (*crecp).flags &
                           (1 as libc::c_uint) << 3 as libc::c_int != 0 {
                    if (*crecp).flags &
                           (1 as libc::c_uint) << 4 as libc::c_int != 0 &&
                           (*dnsmasq_daemon).options[(20 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (20 as libc::c_int as
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
                                                     ((1 as libc::c_uint) <<
                                                          8 as libc::c_int |
                                                          (1 as libc::c_uint)
                                                              <<
                                                              7 as
                                                                  libc::c_int))
                                                    as libc::c_int,
                                                &mut (*crecp).addr) != 0) &&
                               add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   -axfroffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).auth_ttl,
                                                   0 as *mut libc::c_int,
                                                   (if (*crecp).flags &
                                                           (1 as libc::c_uint)
                                                               <<
                                                               8 as
                                                                   libc::c_int
                                                           != 0 {
                                                        28 as libc::c_int
                                                    } else {
                                                        1 as libc::c_int
                                                    }) as libc::c_ushort,
                                                   1 as libc::c_int as
                                                       libc::c_ushort,
                                                   (if (*crecp).flags &
                                                           (1 as libc::c_uint)
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
                                                       *mut all_addr) != 0 {
                            anscount += 1
                        }
                    }
                    if (*crecp).flags &
                           (1 as libc::c_uint) << 6 as libc::c_int != 0 ||
                           (*crecp).flags &
                               (1 as libc::c_uint) << 4 as libc::c_int != 0 &&
                               (*dnsmasq_daemon).options[(20 as libc::c_int as
                                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                               as
                                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_ulong))
                                                             as usize] &
                                   (1 as libc::c_uint) <<
                                       (20 as libc::c_int as
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
                                                     ((1 as libc::c_uint) <<
                                                          8 as libc::c_int |
                                                          (1 as libc::c_uint)
                                                              <<
                                                              7 as
                                                                  libc::c_int))
                                                    as libc::c_int,
                                                &mut (*crecp).addr) != 0) {
                            if !cut.is_null() {
                                *cut = 0 as libc::c_int as libc::c_char
                            }
                            if add_resource_record(header, limit,
                                                   &mut trunc as
                                                       *mut libc::c_int,
                                                   -axfroffset,
                                                   &mut ansp as
                                                       *mut *mut libc::c_uchar,
                                                   (*dnsmasq_daemon).auth_ttl,
                                                   0 as *mut libc::c_int,
                                                   if (*crecp).flags &
                                                          (1 as libc::c_uint)
                                                              <<
                                                              8 as libc::c_int
                                                          != 0 {
                                                       28 as libc::c_int
                                                   } else { 1 as libc::c_int }
                                                       as libc::c_ushort,
                                                   1 as libc::c_int as
                                                       libc::c_ushort,
                                                   if (*crecp).flags &
                                                          (1 as libc::c_uint)
                                                              <<
                                                              7 as libc::c_int
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
                                                       *mut all_addr) != 0 {
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
                                   (*dnsmasq_daemon).auth_ttl,
                                   0 as *mut libc::c_int,
                                   6 as libc::c_int as libc::c_ushort,
                                   1 as libc::c_int as libc::c_ushort,
                                   b"ddlllll\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,
                                   (*dnsmasq_daemon).authserver,
                                   (*dnsmasq_daemon).hostmaster,
                                   (*dnsmasq_daemon).soa_sn,
                                   (*dnsmasq_daemon).soa_refresh,
                                   (*dnsmasq_daemon).soa_retry,
                                   (*dnsmasq_daemon).soa_expiry,
                                   (*dnsmasq_daemon).auth_ttl) != 0 {
                anscount += 1
            }
        }
    }
    /* done all questions, set up header and return length of result */
  /* clear authoritative and truncated flags, set QR flag */
    (*header).hb3 =
        ((*header).hb3 as libc::c_int &
             !(0x4 as libc::c_int | 0x2 as libc::c_int) | 0x80 as libc::c_int)
            as u8_0;
    if local_query != 0 {
        /* set RA flag */
        (*header).hb4 =
            ((*header).hb4 as libc::c_int | 0x80 as libc::c_int) as u8_0
    } else {
        /* clear RA flag */
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !(0x80 as libc::c_int)) as u8_0
    }
    /* data is never DNSSEC signed. */
    (*header).hb4 =
        ((*header).hb4 as libc::c_int & !(0x20 as libc::c_int)) as u8_0;
    /* authoritative */
    if auth != 0 {
        (*header).hb3 =
            ((*header).hb3 as libc::c_int | 0x4 as libc::c_int) as u8_0
    }
    /* truncation */
    if trunc != 0 {
        (*header).hb3 =
            ((*header).hb3 as libc::c_int | 0x2 as libc::c_int) as u8_0
    } /* no error */
    if (auth != 0 || local_query != 0) && nxdomain != 0 {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !(0xf as libc::c_int) |
                 3 as libc::c_int) as u8_0
    } else {
        (*header).hb4 =
            ((*header).hb4 as libc::c_int & !(0xf as libc::c_int) |
                 0 as libc::c_int) as u8_0
    }
    (*header).ancount = __bswap_16(anscount as __uint16_t);
    (*header).nscount = __bswap_16(authcount as __uint16_t);
    (*header).arcount = __bswap_16(0 as libc::c_int as __uint16_t);
    /* Advertise our packet size limit in our reply */
    if have_pseudoheader != 0 {
        return add_pseudoheader(header,
                                ansp.wrapping_offset_from(header as
                                                              *mut libc::c_uchar)
                                    as libc::c_long as size_t,
                                limit as *mut libc::c_uchar,
                                (*dnsmasq_daemon).edns_pktsz,
                                0 as libc::c_int, 0 as *mut libc::c_uchar,
                                0 as libc::c_int as size_t, do_bit,
                                0 as libc::c_int)
    }
    return ansp.wrapping_offset_from(header as *mut libc::c_uchar) as
               libc::c_long as size_t;
}
