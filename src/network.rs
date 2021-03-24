
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
pub unsafe extern "C" fn indextoname(mut fd: libc::c_int,
                                     mut index: libc::c_int,
                                     mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut ifr: ifreq =
        ifreq{ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_2{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    if index == 0 as libc::c_int { return 0 as libc::c_int }
    ifr.ifr_ifru.ifru_ivalue = index;
    if ioctl(fd, 0x8910 as libc::c_int as libc::c_ulong,
             &mut ifr as *mut ifreq) == -(1 as libc::c_int) {
        return 0 as libc::c_int
    }
    safe_strncpy(name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                 16 as libc::c_int as size_t);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn iface_check(mut family: libc::c_int,
                                     mut addr: *mut all_addr,
                                     mut name: *mut libc::c_char,
                                     mut auth: *mut libc::c_int)
 -> libc::c_int {
    let mut tmp: *mut iname = 0 as *mut iname;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut match_addr: libc::c_int = 0 as libc::c_int;
    /* Note: have to check all and not bail out early, so that we set the
     "used" flags.

     May be called with family == AF_LOCALto check interface by name only. */
    if !auth.is_null() { *auth = 0 as libc::c_int }
    if !(*dnsmasq_daemon).if_names.is_null() ||
           !(*dnsmasq_daemon).if_addrs.is_null() {
        ret = 0 as libc::c_int;
        tmp = (*dnsmasq_daemon).if_names;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name, name) != 0 {
                (*tmp).used = 1 as libc::c_int;
                ret = (*tmp).used
            }
            tmp = (*tmp).next
        }
        if !addr.is_null() {
            tmp = (*dnsmasq_daemon).if_addrs;
            while !tmp.is_null() {
                if (*tmp).addr.sa.sa_family as libc::c_int == family {
                    if family == 2 as libc::c_int &&
                           (*tmp).addr.in_0.sin_addr.s_addr ==
                               (*addr).addr4.s_addr {
                        (*tmp).used = 1 as libc::c_int;
                        match_addr = (*tmp).used;
                        ret = match_addr
                    } else if family == 10 as libc::c_int &&
                                  ({
                                       let mut __a: *const in6_addr =
                                           &mut (*tmp).addr.in6.sin6_addr as
                                               *mut in6_addr as
                                               *const in6_addr;
                                       let mut __b: *const in6_addr =
                                           &mut (*addr).addr6 as *mut in6_addr
                                               as *const in6_addr;
                                       ((*__a).__in6_u.__u6_addr32[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                            ==
                                            (*__b).__in6_u.__u6_addr32[0 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                            &&
                                            (*__a).__in6_u.__u6_addr32[1 as
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
                                            (*__a).__in6_u.__u6_addr32[2 as
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
                                            (*__a).__in6_u.__u6_addr32[3 as
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
                        (*tmp).used = 1 as libc::c_int;
                        match_addr = (*tmp).used;
                        ret = match_addr
                    }
                }
                tmp = (*tmp).next
            }
        }
    }
    if match_addr == 0 {
        tmp = (*dnsmasq_daemon).if_except;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name, name) != 0 {
                ret = 0 as libc::c_int
            }
            tmp = (*tmp).next
        }
    }
    tmp = (*dnsmasq_daemon).authinterface;
    while !tmp.is_null() {
        if !(*tmp).name.is_null() {
            if strcmp((*tmp).name, name) == 0 as libc::c_int &&
                   ((*tmp).addr.sa.sa_family as libc::c_int ==
                        0 as libc::c_int ||
                        (*tmp).addr.sa.sa_family as libc::c_int == family) {
                break ;
            }
        } else {
            if !addr.is_null() &&
                   (*tmp).addr.sa.sa_family as libc::c_int == 2 as libc::c_int
                   && family == 2 as libc::c_int &&
                   (*tmp).addr.in_0.sin_addr.s_addr == (*addr).addr4.s_addr {
                break ;
            }
            if !addr.is_null() &&
                   (*tmp).addr.sa.sa_family as libc::c_int ==
                       10 as libc::c_int && family == 10 as libc::c_int &&
                   ({
                        let mut __a: *const in6_addr =
                            &mut (*tmp).addr.in6.sin6_addr as *mut in6_addr as
                                *const in6_addr;
                        let mut __b: *const in6_addr =
                            &mut (*addr).addr6 as *mut in6_addr as
                                *const in6_addr;
                        ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                             ==
                             (*__b).__in6_u.__u6_addr32[0 as libc::c_int as
                                                            usize] &&
                             (*__a).__in6_u.__u6_addr32[1 as libc::c_int as
                                                            usize] ==
                                 (*__b).__in6_u.__u6_addr32[1 as libc::c_int
                                                                as usize] &&
                             (*__a).__in6_u.__u6_addr32[2 as libc::c_int as
                                                            usize] ==
                                 (*__b).__in6_u.__u6_addr32[2 as libc::c_int
                                                                as usize] &&
                             (*__a).__in6_u.__u6_addr32[3 as libc::c_int as
                                                            usize] ==
                                 (*__b).__in6_u.__u6_addr32[3 as libc::c_int
                                                                as usize]) as
                            libc::c_int
                    }) != 0 {
                break ;
            }
        }
        tmp = (*tmp).next
    }
    if !tmp.is_null() && !auth.is_null() {
        *auth = 1 as libc::c_int;
        ret = 1 as libc::c_int
    }
    return ret;
}
/* Fix for problem that the kernel sometimes reports the loopback interface as the
   arrival interface when a packet originates locally, even when sent to address of 
   an interface other than the loopback. Accept packet if it arrived via a loopback 
   interface, even when we're not accepting packets that way, as long as the destination
   address is one we're believing. Interface list must be up-to-date before calling. */
#[no_mangle]
pub unsafe extern "C" fn loopback_exception(mut fd: libc::c_int,
                                            mut family: libc::c_int,
                                            mut addr: *mut all_addr,
                                            mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut ifr: ifreq =
        ifreq{ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_2{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut iface: *mut irec = 0 as *mut irec;
    safe_strncpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), name,
                 16 as libc::c_int as size_t);
    if ioctl(fd, 0x8913 as libc::c_int as libc::c_ulong,
             &mut ifr as *mut ifreq) != -(1 as libc::c_int) &&
           ifr.ifr_ifru.ifru_flags as libc::c_int &
               IFF_LOOPBACK as libc::c_int != 0 {
        iface = (*dnsmasq_daemon).interfaces;
        while !iface.is_null() {
            if (*iface).addr.sa.sa_family as libc::c_int == family {
                if family == 2 as libc::c_int {
                    if (*iface).addr.in_0.sin_addr.s_addr ==
                           (*addr).addr4.s_addr {
                        return 1 as libc::c_int
                    }
                } else if ({
                               let mut __a: *const in6_addr =
                                   &mut (*iface).addr.in6.sin6_addr as
                                       *mut in6_addr as *const in6_addr;
                               let mut __b: *const in6_addr =
                                   &mut (*addr).addr6 as *mut in6_addr as
                                       *const in6_addr;
                               ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as
                                                               usize] ==
                                    (*__b).__in6_u.__u6_addr32[0 as
                                                                   libc::c_int
                                                                   as usize]
                                    &&
                                    (*__a).__in6_u.__u6_addr32[1 as
                                                                   libc::c_int
                                                                   as usize]
                                        ==
                                        (*__b).__in6_u.__u6_addr32[1 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                    &&
                                    (*__a).__in6_u.__u6_addr32[2 as
                                                                   libc::c_int
                                                                   as usize]
                                        ==
                                        (*__b).__in6_u.__u6_addr32[2 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                    &&
                                    (*__a).__in6_u.__u6_addr32[3 as
                                                                   libc::c_int
                                                                   as usize]
                                        ==
                                        (*__b).__in6_u.__u6_addr32[3 as
                                                                       libc::c_int
                                                                       as
                                                                       usize])
                                   as libc::c_int
                           }) != 0 {
                    return 1 as libc::c_int
                }
            }
            iface = (*iface).next
        }
    }
    return 0 as libc::c_int;
}
/* If we're configured with something like --interface=eth0:0 then we'll listen correctly
   on the relevant address, but the name of the arrival interface, derived from the
   index won't match the config. Check that we found an interface address for the arrival 
   interface: daemon->interfaces must be up-to-date. */
#[no_mangle]
pub unsafe extern "C" fn label_exception(mut index: libc::c_int,
                                         mut family: libc::c_int,
                                         mut addr: *mut all_addr)
 -> libc::c_int {
    let mut iface: *mut irec = 0 as *mut irec;
    /* labels only supported on IPv4 addresses. */
    if family != 2 as libc::c_int { return 0 as libc::c_int }
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).index == index &&
               (*iface).addr.sa.sa_family as libc::c_int == 2 as libc::c_int
               && (*iface).addr.in_0.sin_addr.s_addr == (*addr).addr4.s_addr {
            return 1 as libc::c_int
        }
        iface = (*iface).next
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn iface_allowed(mut param: *mut iface_param,
                                   mut if_index: libc::c_int,
                                   mut label: *mut libc::c_char,
                                   mut addr: *mut mysockaddr,
                                   mut netmask: in_addr,
                                   mut prefixlen: libc::c_int,
                                   mut iface_flags: libc::c_int)
 -> libc::c_int {
    let mut iface: *mut irec = 0 as *mut irec;
    let mut mtu: libc::c_int = 0 as libc::c_int;
    let mut loopback: libc::c_int = 0;
    let mut ifr: ifreq =
        ifreq{ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_2{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut tftp_ok: libc::c_int =
        ((*dnsmasq_daemon).options[(40 as libc::c_int as
                                        libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                                       as usize] &
             (1 as libc::c_uint) <<
                 (40 as libc::c_int as
                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       as
                                                       libc::c_ulong).wrapping_mul(8
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong))
             != 0) as libc::c_int;
    let mut dhcp_ok: libc::c_int = 1 as libc::c_int;
    let mut auth_dns: libc::c_int = 0 as libc::c_int;
    let mut is_label: libc::c_int = 0 as libc::c_int;
    let mut tmp: *mut iname = 0 as *mut iname;
    if indextoname((*param).fd, if_index, ifr.ifr_ifrn.ifrn_name.as_mut_ptr())
           == 0 ||
           ioctl((*param).fd, 0x8913 as libc::c_int as libc::c_ulong,
                 &mut ifr as *mut ifreq) == -(1 as libc::c_int) {
        return 0 as libc::c_int
    }
    loopback =
        ifr.ifr_ifru.ifru_flags as libc::c_int & IFF_LOOPBACK as libc::c_int;
    if loopback != 0 { dhcp_ok = 0 as libc::c_int }
    if ioctl((*param).fd, 0x8921 as libc::c_int as libc::c_ulong,
             &mut ifr as *mut ifreq) != -(1 as libc::c_int) {
        mtu = ifr.ifr_ifru.ifru_mtu
    }
    if label.is_null() {
        label = ifr.ifr_ifrn.ifrn_name.as_mut_ptr()
    } else { is_label = strcmp(label, ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) }
    /* maintain a list of all addresses on all interfaces for --local-service option */
    if (*dnsmasq_daemon).options[(49 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (49 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        let mut al: *mut addrlist = 0 as *mut addrlist;
        if !(*param).spare.is_null() {
            al = (*param).spare;
            (*param).spare = (*al).next
        } else {
            al =
                whine_malloc(::std::mem::size_of::<addrlist>() as
                                 libc::c_ulong) as *mut addrlist
        }
        if !al.is_null() {
            (*al).next = (*dnsmasq_daemon).interface_addrs;
            (*dnsmasq_daemon).interface_addrs = al;
            (*al).prefixlen = prefixlen;
            if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int {
                (*al).addr.addr4 = (*addr).in_0.sin_addr;
                (*al).flags = 0 as libc::c_int
            } else {
                (*al).addr.addr6 = (*addr).in6.sin6_addr;
                (*al).flags = 2 as libc::c_int
            }
        }
    }
    if (*addr).sa.sa_family as libc::c_int != 10 as libc::c_int ||
           ({
                let mut __a: *const in6_addr =
                    &mut (*addr).in6.sin6_addr as *mut in6_addr as
                        *const in6_addr;
                ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                     __bswap_32(0xffc00000 as libc::c_uint) ==
                     __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
            }) == 0 {
        let mut int_name: *mut interface_name = 0 as *mut interface_name;
        let mut al_0: *mut addrlist = 0 as *mut addrlist;
        let mut zone: *mut auth_zone = 0 as *mut auth_zone;
        let mut name: *mut auth_name_list = 0 as *mut auth_name_list;
        /* Find subnets in auth_zones */
        zone = (*dnsmasq_daemon).auth_zones;
        while !zone.is_null() {
            name = (*zone).interface_names;
            while !name.is_null() {
                if wildcard_match((*name).name, label) != 0 {
                    if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int
                           && (*name).flags & 2 as libc::c_int != 0 {
                        if !(*param).spare.is_null() {
                            al_0 = (*param).spare;
                            (*param).spare = (*al_0).next
                        } else {
                            al_0 =
                                whine_malloc(::std::mem::size_of::<addrlist>()
                                                 as libc::c_ulong) as
                                    *mut addrlist
                        }
                        if !al_0.is_null() {
                            (*al_0).next = (*zone).subnet;
                            (*zone).subnet = al_0;
                            (*al_0).prefixlen = prefixlen;
                            (*al_0).addr.addr4 = (*addr).in_0.sin_addr;
                            (*al_0).flags = 0 as libc::c_int
                        }
                    }
                    if (*addr).sa.sa_family as libc::c_int ==
                           10 as libc::c_int &&
                           (*name).flags & 1 as libc::c_int != 0 {
                        if !(*param).spare.is_null() {
                            al_0 = (*param).spare;
                            (*param).spare = (*al_0).next
                        } else {
                            al_0 =
                                whine_malloc(::std::mem::size_of::<addrlist>()
                                                 as libc::c_ulong) as
                                    *mut addrlist
                        }
                        if !al_0.is_null() {
                            (*al_0).next = (*zone).subnet;
                            (*zone).subnet = al_0;
                            (*al_0).prefixlen = prefixlen;
                            (*al_0).addr.addr6 = (*addr).in6.sin6_addr;
                            (*al_0).flags = 2 as libc::c_int
                        }
                    }
                }
                name = (*name).next
            }
            zone = (*zone).next
        }
        /* Update addresses from interface_names. These are a set independent
	 of the set we're listening on. */
        int_name = (*dnsmasq_daemon).int_names;
        while !int_name.is_null() {
            if strncmp(label, (*int_name).intr,
                       16 as libc::c_int as libc::c_ulong) == 0 as libc::c_int
                   &&
                   ((*addr).sa.sa_family as libc::c_int == (*int_name).family
                        || (*int_name).family == 0 as libc::c_int) {
                if !(*param).spare.is_null() {
                    al_0 = (*param).spare;
                    (*param).spare = (*al_0).next
                } else {
                    al_0 =
                        whine_malloc(::std::mem::size_of::<addrlist>() as
                                         libc::c_ulong) as *mut addrlist
                }
                if !al_0.is_null() {
                    (*al_0).next = (*int_name).addr;
                    (*int_name).addr = al_0;
                    if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int
                       {
                        (*al_0).addr.addr4 = (*addr).in_0.sin_addr;
                        (*al_0).flags = 0 as libc::c_int
                    } else {
                        (*al_0).addr.addr6 = (*addr).in6.sin6_addr;
                        (*al_0).flags = 2 as libc::c_int;
                        /* Privacy addresses and addresses still undergoing DAD and deprecated addresses
		       don't appear in forward queries, but will in reverse ones. */
                        if iface_flags & 4 as libc::c_int == 0 ||
                               iface_flags &
                                   (2 as libc::c_int | 1 as libc::c_int) != 0
                           {
                            (*al_0).flags |= 4 as libc::c_int
                        }
                    }
                }
            }
            int_name = (*int_name).next
        }
    }
    /* check whether the interface IP has been added already 
     we call this routine multiple times. */
    iface = (*dnsmasq_daemon).interfaces; /* for garbage collection */
    while !iface.is_null() {
        if sockaddr_isequal(&mut (*iface).addr, addr) != 0 &&
               (*iface).index == if_index {
            (*iface).dad =
                (iface_flags & 1 as libc::c_int != 0) as libc::c_int;
            (*iface).found = 1 as libc::c_int;
            (*iface).netmask = netmask;
            return 1 as libc::c_int
        }
        iface = (*iface).next
    }
    /* If we are restricting the set of interfaces to use, make
     sure that loopback interfaces are in that set. */
    if !(*dnsmasq_daemon).if_names.is_null() && loopback != 0 {
        let mut lo: *mut iname = 0 as *mut iname;
        lo = (*dnsmasq_daemon).if_names;
        while !lo.is_null() {
            if !(*lo).name.is_null() &&
                   strcmp((*lo).name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) ==
                       0 as libc::c_int {
                break ;
            }
            lo = (*lo).next
        }
        if lo.is_null() &&
               {
                   lo =
                       whine_malloc(::std::mem::size_of::<iname>() as
                                        libc::c_ulong) as *mut iname;
                   !lo.is_null()
               } {
            (*lo).name =
                whine_malloc(strlen(ifr.ifr_ifrn.ifrn_name.as_mut_ptr()).wrapping_add(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong))
                    as *mut libc::c_char;
            if !(*lo).name.is_null() {
                strcpy((*lo).name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
                (*lo).used = 1 as libc::c_int;
                (*lo).next = (*dnsmasq_daemon).if_names;
                (*dnsmasq_daemon).if_names = lo
            } else { free(lo as *mut libc::c_void); }
        }
    }
    if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int &&
           iface_check(2 as libc::c_int,
                       &mut (*addr).in_0.sin_addr as *mut in_addr as
                           *mut all_addr, label, &mut auth_dns) == 0 {
        return 1 as libc::c_int
    }
    if (*addr).sa.sa_family as libc::c_int == 10 as libc::c_int &&
           iface_check(10 as libc::c_int,
                       &mut (*addr).in6.sin6_addr as *mut in6_addr as
                           *mut all_addr, label, &mut auth_dns) == 0 {
        return 1 as libc::c_int
    }
    /* No DHCP where we're doing auth DNS. */
    if auth_dns != 0 {
        tftp_ok = 0 as libc::c_int;
        dhcp_ok = 0 as libc::c_int
    } else {
        tmp = (*dnsmasq_daemon).dhcp_except;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                tftp_ok = 0 as libc::c_int;
                dhcp_ok = 0 as libc::c_int
            }
            tmp = (*tmp).next
        }
    }
    if !(*dnsmasq_daemon).tftp_interfaces.is_null() {
        /* dedicated tftp interface list */
        tftp_ok = 0 as libc::c_int;
        tmp = (*dnsmasq_daemon).tftp_interfaces;
        while !tmp.is_null() {
            if !(*tmp).name.is_null() &&
                   wildcard_match((*tmp).name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                tftp_ok = 1 as libc::c_int
            }
            tmp = (*tmp).next
        }
    }
    /* add to list */
    iface =
        whine_malloc(::std::mem::size_of::<irec>() as libc::c_ulong) as
            *mut irec; /* dummy */
    if !iface.is_null() {
        (*iface).addr = *addr;
        (*iface).netmask = netmask;
        (*iface).tftp_ok = tftp_ok;
        (*iface).dhcp_ok = dhcp_ok;
        (*iface).dns_auth = auth_dns;
        (*iface).mtu = mtu;
        (*iface).dad = (iface_flags & 1 as libc::c_int != 0) as libc::c_int;
        (*iface).found = 1 as libc::c_int;
        (*iface).warned = 0 as libc::c_int;
        (*iface).multicast_done = (*iface).warned;
        (*iface).done = (*iface).multicast_done;
        (*iface).index = if_index;
        (*iface).label = is_label;
        (*iface).name =
            whine_malloc(strlen(ifr.ifr_ifrn.ifrn_name.as_mut_ptr()).wrapping_add(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong))
                as *mut libc::c_char;
        if !(*iface).name.is_null() {
            strcpy((*iface).name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
            (*iface).next = (*dnsmasq_daemon).interfaces;
            (*dnsmasq_daemon).interfaces = iface;
            return 1 as libc::c_int
        }
        free(iface as *mut libc::c_void);
    }
    *__errno_location() = 12 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn iface_allowed_v6(mut local: *mut in6_addr,
                                      mut prefix: libc::c_int,
                                      mut scope: libc::c_int,
                                      mut if_index: libc::c_int,
                                      mut flags: libc::c_int,
                                      mut preferred: libc::c_int,
                                      mut valid: libc::c_int,
                                      mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut addr: mysockaddr =
        mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut netmask: in_addr = in_addr{s_addr: 0,};
    netmask.s_addr = 0 as libc::c_int as in_addr_t;
    /* warning */
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in6.sin6_family = 10 as libc::c_int as sa_family_t;
    addr.in6.sin6_addr = *local;
    addr.in6.sin6_port = __bswap_16((*dnsmasq_daemon).port as __uint16_t);
    /* FreeBSD insists this is zero for non-linklocal addresses */
    if ({
            let mut __a: *const in6_addr = local as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                 __bswap_32(0xffc00000 as libc::c_uint) ==
                 __bswap_32(0xfe800000 as libc::c_uint)) as libc::c_int
        }) != 0 {
        addr.in6.sin6_scope_id = if_index as uint32_t
    } else { addr.in6.sin6_scope_id = 0 as libc::c_int as uint32_t }
    return iface_allowed(vparam as *mut iface_param, if_index,
                         0 as *mut libc::c_char, &mut addr, netmask, prefix,
                         flags);
}
unsafe extern "C" fn iface_allowed_v4(mut local: in_addr,
                                      mut if_index: libc::c_int,
                                      mut label: *mut libc::c_char,
                                      mut netmask: in_addr,
                                      mut broadcast: in_addr,
                                      mut vparam: *mut libc::c_void)
 -> libc::c_int {
    let mut addr: mysockaddr =
        mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut prefix: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    /* warning */
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in_0.sin_family = 2 as libc::c_int as sa_family_t;
    addr.in_0.sin_addr = local;
    addr.in_0.sin_port = __bswap_16((*dnsmasq_daemon).port as __uint16_t);
    /* determine prefix length from netmask */
    prefix = 32 as libc::c_int;
    bit = 1 as libc::c_int;
    while bit as libc::c_uint & __bswap_32(netmask.s_addr) ==
              0 as libc::c_int as libc::c_uint && prefix != 0 as libc::c_int {
        bit = bit << 1 as libc::c_int;
        prefix -= 1
    }
    return iface_allowed(vparam as *mut iface_param, if_index, label,
                         &mut addr, netmask, prefix, 0 as libc::c_int);
}
/*
 * Clean old interfaces no longer found.
 */
unsafe extern "C" fn clean_interfaces() {
    let mut iface: *mut irec = 0 as *mut irec;
    let mut up: *mut *mut irec = &mut (*dnsmasq_daemon).interfaces;
    iface = *up;
    while !iface.is_null() {
        if (*iface).found == 0 && (*iface).done == 0 {
            *up = (*iface).next;
            free((*iface).name as *mut libc::c_void);
            free(iface as *mut libc::c_void);
        } else { up = &mut (*iface).next }
        iface = *up
    };
}
/* * Release listener if no other interface needs it.
 *
 * @return 1 if released, 0 if still required
 */
unsafe extern "C" fn release_listener(mut l: *mut listener) -> libc::c_int {
    if (*l).used > 1 as libc::c_int {
        let mut iface: *mut irec = 0 as *mut irec;
        iface = (*dnsmasq_daemon).interfaces;
        while !iface.is_null() {
            if (*iface).done != 0 &&
                   sockaddr_isequal(&mut (*l).addr, &mut (*iface).addr) != 0 {
                if (*iface).found != 0 {
                    /* update listener to point to active interface instead */
                    if (*(*l).iface).found == 0 { (*l).iface = iface }
                } else { (*l).used -= 1; (*iface).done = 0 as libc::c_int }
            }
            iface = (*iface).next
        }
        /* Someone is still using this listener, skip its deletion */
        if (*l).used > 0 as libc::c_int { return 0 as libc::c_int }
    }
    if (*(*l).iface).done != 0 {
        let mut port: libc::c_int = 0;
        port =
            prettyprint_addr(&mut (*(*l).iface).addr,
                             (*dnsmasq_daemon).addrbuff);
        my_syslog(7 as libc::c_int,
                  b"stopped listening on %s(#%d): %s port %d\x00" as *const u8
                      as *const libc::c_char, (*(*l).iface).name,
                  (*(*l).iface).index, (*dnsmasq_daemon).addrbuff, port);
        /* In case it ever returns */
        (*(*l).iface).done = 0 as libc::c_int
    }
    if (*l).fd != -(1 as libc::c_int) { close((*l).fd); }
    if (*l).tcpfd != -(1 as libc::c_int) { close((*l).tcpfd); }
    if (*l).tftpfd != -(1 as libc::c_int) { close((*l).tftpfd); }
    free(l as *mut libc::c_void);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn enumerate_interfaces(mut reset: libc::c_int)
 -> libc::c_int {
    static mut spare: *mut addrlist = 0 as *const addrlist as *mut addrlist;
    static mut done: libc::c_int = 0 as libc::c_int;
    let mut param: iface_param =
        iface_param{spare: 0 as *mut addrlist, fd: 0,};
    let mut errsave: libc::c_int = 0;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut addr: *mut addrlist = 0 as *mut addrlist;
    let mut tmp: *mut addrlist = 0 as *mut addrlist;
    let mut intname: *mut interface_name = 0 as *mut interface_name;
    let mut iface: *mut irec = 0 as *mut irec;
    let mut zone: *mut auth_zone = 0 as *mut auth_zone;
    /* Do this max once per select cycle  - also inhibits netlink socket use
   in TCP child processes. */
    if reset != 0 { done = 0 as libc::c_int; return 1 as libc::c_int }
    if done != 0 { return 1 as libc::c_int }
    done = 1 as libc::c_int;
    param.fd =
        socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
    if param.fd == -(1 as libc::c_int) { return 0 as libc::c_int }
    /* Mark interfaces for garbage collection */
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        (*iface).found = 0 as libc::c_int;
        iface = (*iface).next
    }
    /* remove addresses stored against interface_names */
    intname = (*dnsmasq_daemon).int_names;
    while !intname.is_null() {
        addr = (*intname).addr;
        while !addr.is_null() {
            tmp = (*addr).next;
            (*addr).next = spare;
            spare = addr;
            addr = tmp
        }
        (*intname).addr = 0 as *mut addrlist;
        intname = (*intname).next
    }
    /* Remove list of addresses of local interfaces */
    addr = (*dnsmasq_daemon).interface_addrs;
    while !addr.is_null() {
        tmp = (*addr).next;
        (*addr).next = spare;
        spare = addr;
        addr = tmp
    }
    (*dnsmasq_daemon).interface_addrs = 0 as *mut addrlist;
    /* remove addresses stored against auth_zone subnets, but not 
   ones configured as address literals */
    zone = (*dnsmasq_daemon).auth_zones;
    while !zone.is_null() {
        if !(*zone).interface_names.is_null() {
            let mut up: *mut *mut addrlist = 0 as *mut *mut addrlist;
            up = &mut (*zone).subnet;
            addr = (*zone).subnet;
            while !addr.is_null() {
                tmp = (*addr).next;
                if (*addr).flags & 1 as libc::c_int != 0 {
                    up = &mut (*addr).next
                } else {
                    *up = (*addr).next;
                    (*addr).next = spare;
                    spare = addr
                }
                addr = tmp
            }
        }
        zone = (*zone).next
    }
    param.spare = spare;
    ret =
        iface_enumerate(10 as libc::c_int,
                        &mut param as *mut iface_param as *mut libc::c_void,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut in6_addr,
                                                                            _:
                                                                                libc::c_int,
                                                                            _:
                                                                                libc::c_int,
                                                                            _:
                                                                                libc::c_int,
                                                                            _:
                                                                                libc::c_int,
                                                                            _:
                                                                                libc::c_int,
                                                                            _:
                                                                                libc::c_int,
                                                                            _:
                                                                                *mut libc::c_void)
                                                           -> libc::c_int>,
                                                Option<unsafe extern "C" fn()
                                                           ->
                                                               libc::c_int>>(Some(iface_allowed_v6
                                                                                      as
                                                                                      unsafe extern "C" fn(_:
                                                                                                               *mut in6_addr,
                                                                                                           _:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               *mut libc::c_void)
                                                                                          ->
                                                                                              libc::c_int)));
    if ret != 0 {
        ret =
            iface_enumerate(2 as libc::c_int,
                            &mut param as *mut iface_param as
                                *mut libc::c_void,
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                    in_addr,
                                                                                _:
                                                                                    libc::c_int,
                                                                                _:
                                                                                    *mut libc::c_char,
                                                                                _:
                                                                                    in_addr,
                                                                                _:
                                                                                    in_addr,
                                                                                _:
                                                                                    *mut libc::c_void)
                                                               ->
                                                                   libc::c_int>,
                                                    Option<unsafe extern "C" fn()
                                                               ->
                                                                   libc::c_int>>(Some(iface_allowed_v4
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   in_addr,
                                                                                                               _:
                                                                                                                   libc::c_int,
                                                                                                               _:
                                                                                                                   *mut libc::c_char,
                                                                                                               _:
                                                                                                                   in_addr,
                                                                                                               _:
                                                                                                                   in_addr,
                                                                                                               _:
                                                                                                                   *mut libc::c_void)
                                                                                              ->
                                                                                                  libc::c_int)))
    }
    errsave = *__errno_location();
    close(param.fd);
    if (*dnsmasq_daemon).options[(39 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (39 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        /* Garbage-collect listeners listening on addresses that no longer exist.
	 Does nothing when not binding interfaces or for listeners on localhost, 
	 since the ->iface field is NULL. Note that this needs the protections
	 against reentrancy, hence it's here.  It also means there's a possibility,
	 in OPT_CLEVERBIND mode, that at listener will just disappear after
	 a call to enumerate_interfaces, this is checked OK on all calls. */
        let mut l: *mut listener = 0 as *mut listener;
        let mut tmp_0: *mut listener = 0 as *mut listener;
        let mut up_0: *mut *mut listener = 0 as *mut *mut listener;
        let mut freed: libc::c_int = 0 as libc::c_int;
        up_0 = &mut (*dnsmasq_daemon).listeners;
        l = (*dnsmasq_daemon).listeners;
        while !l.is_null() {
            tmp_0 = (*l).next;
            if (*l).iface.is_null() || (*(*l).iface).found != 0 {
                up_0 = &mut (*l).next
            } else if release_listener(l) != 0 {
                *up_0 = tmp_0;
                freed = 1 as libc::c_int
            }
            l = tmp_0
        }
        if freed != 0 { clean_interfaces(); }
    }
    *__errno_location() = errsave;
    spare = param.spare;
    return ret;
}
/* set NONBLOCK bit on fd: See Stevens 16.6 */
#[no_mangle]
pub unsafe extern "C" fn fix_fd(mut fd: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    flags = fcntl(fd, 3 as libc::c_int);
    if flags == -(1 as libc::c_int) ||
           fcntl(fd, 4 as libc::c_int, flags | 0o4000 as libc::c_int) ==
               -(1 as libc::c_int) {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn make_sock(mut addr: *mut mysockaddr,
                               mut type_0: libc::c_int,
                               mut dienow: libc::c_int) -> libc::c_int {
    let mut port: libc::c_int = 0;
    let mut errsave: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_block: u64;
    let mut family: libc::c_int = (*addr).sa.sa_family as libc::c_int;
    let mut fd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut opt: libc::c_int = 1 as libc::c_int;
    fd = socket(family, type_0, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        port = 0;
        errsave = 0;
        s = 0 as *mut libc::c_char;
        /* No error if the kernel just doesn't support this IP flavour */
        if *__errno_location() == 93 as libc::c_int ||
               *__errno_location() == 97 as libc::c_int ||
               *__errno_location() == 22 as libc::c_int {
            return -(1 as libc::c_int)
        }
    } else if !(setsockopt(fd, 1 as libc::c_int, 2 as libc::c_int,
                           &mut opt as *mut libc::c_int as
                               *const libc::c_void,
                           ::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong as socklen_t) ==
                    -(1 as libc::c_int) || fix_fd(fd) == 0) {
        if !(family == 10 as libc::c_int &&
                 setsockopt(fd, IPPROTO_IPV6 as libc::c_int,
                            26 as libc::c_int,
                            &mut opt as *mut libc::c_int as
                                *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as
                                libc::c_ulong as socklen_t) ==
                     -(1 as libc::c_int)) {
            rc =
                bind(fd,
                     __CONST_SOCKADDR_ARG{__sockaddr__:
                                              addr as *mut sockaddr,},
                     sa_len(addr) as socklen_t);
            if !(rc == -(1 as libc::c_int)) {
                if type_0 == SOCK_STREAM as libc::c_int {
                    let mut qlen: libc::c_int = 5 as libc::c_int;
                    setsockopt(fd, IPPROTO_TCP as libc::c_int,
                               23 as libc::c_int,
                               &mut qlen as *mut libc::c_int as
                                   *const libc::c_void,
                               ::std::mem::size_of::<libc::c_int>() as
                                   libc::c_ulong as socklen_t);
                    if listen(fd, 32 as libc::c_int) == -(1 as libc::c_int) {
                        current_block = 4055993212646746884;
                    } else { current_block = 11459959175219260272; }
                } else if family == 2 as libc::c_int {
                    if (*dnsmasq_daemon).options[(13 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (13 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           == 0 {
                        if setsockopt(fd, IPPROTO_IP as libc::c_int,
                                      8 as libc::c_int,
                                      &mut opt as *mut libc::c_int as
                                          *const libc::c_void,
                                      ::std::mem::size_of::<libc::c_int>() as
                                          libc::c_ulong as socklen_t) ==
                               -(1 as libc::c_int) {
                            current_block = 4055993212646746884;
                        } else { current_block = 11459959175219260272; }
                    } else { current_block = 11459959175219260272; }
                } else if set_ipv6pktinfo(fd) == 0 {
                    current_block = 4055993212646746884;
                } else { current_block = 11459959175219260272; }
                match current_block {
                    4055993212646746884 => { }
                    _ => { return fd }
                }
            }
        }
    }
    errsave = *__errno_location();
    port = prettyprint_addr(addr, (*dnsmasq_daemon).addrbuff);
    if (*dnsmasq_daemon).options[(13 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (13 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 &&
           (*dnsmasq_daemon).options[(39 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (39 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
        sprintf((*dnsmasq_daemon).addrbuff,
                b"port %d\x00" as *const u8 as *const libc::c_char, port);
    }
    s =
        b"failed to create listening socket for %s: %s\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char;
    if fd != -(1 as libc::c_int) { close(fd); }
    *__errno_location() = errsave;
    if dienow != 0 {
        /* failure to bind addresses given by --listen-address at this point
	     is OK if we're doing bind-dynamic */
        if (*dnsmasq_daemon).options[(39 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (39 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
            die(s, (*dnsmasq_daemon).addrbuff, 2 as libc::c_int);
        }
    } else {
        my_syslog(4 as libc::c_int, s, (*dnsmasq_daemon).addrbuff,
                  strerror(*__errno_location()));
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn set_ipv6pktinfo(mut fd: libc::c_int) -> libc::c_int {
    let mut opt: libc::c_int = 1 as libc::c_int;
    /* The API changed around Linux 2.6.14 but the old ABI is still supported:
     handle all combinations of headers and kernel.
     OpenWrt note that this fixes the problem addressed by your very broken patch. */
    (*dnsmasq_daemon).v6pktinfo = 50 as libc::c_int;
    if setsockopt(fd, IPPROTO_IPV6 as libc::c_int, 49 as libc::c_int,
                  &mut opt as *mut libc::c_int as *const libc::c_void,
                  ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                      socklen_t) != -(1 as libc::c_int) {
        return 1 as libc::c_int
    } else {
        if *__errno_location() == 92 as libc::c_int &&
               setsockopt(fd, IPPROTO_IPV6 as libc::c_int, 2 as libc::c_int,
                          &mut opt as *mut libc::c_int as *const libc::c_void,
                          ::std::mem::size_of::<libc::c_int>() as
                              libc::c_ulong as socklen_t) !=
                   -(1 as libc::c_int) {
            (*dnsmasq_daemon).v6pktinfo = 2 as libc::c_int;
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/* Find the interface on which a TCP connection arrived, if possible, or zero otherwise. */
#[no_mangle]
pub unsafe extern "C" fn tcp_interface(mut fd: libc::c_int,
                                       mut af: libc::c_int) -> libc::c_int {
    /* suppress potential unused warning */
    let mut if_index: libc::c_int = 0 as libc::c_int;
    let mut opt: libc::c_int = 1 as libc::c_int;
    let mut cmptr: *mut cmsghdr = 0 as *mut cmsghdr;
    let mut msg: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut len: socklen_t = 0;
    /* use mshdr so that the CMSDG_* macros are available */
    msg.msg_control = (*dnsmasq_daemon).packet as *mut libc::c_void;
    len = (*dnsmasq_daemon).packet_buff_sz as socklen_t;
    msg.msg_controllen = len as size_t;
    /* we overwrote the buffer... */
    (*dnsmasq_daemon).srv_save = 0 as *mut server;
    if af == 2 as libc::c_int {
        if setsockopt(fd, IPPROTO_IP as libc::c_int, 8 as libc::c_int,
                      &mut opt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) != -(1 as libc::c_int) &&
               getsockopt(fd, IPPROTO_IP as libc::c_int, 9 as libc::c_int,
                          msg.msg_control, &mut len) != -(1 as libc::c_int) {
            msg.msg_controllen = len as size_t;
            cmptr =
                if msg.msg_controllen >=
                       ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                    msg.msg_control as *mut cmsghdr
                } else { 0 as *mut cmsghdr };
            while !cmptr.is_null() {
                if (*cmptr).cmsg_level == IPPROTO_IP as libc::c_int &&
                       (*cmptr).cmsg_type == 8 as libc::c_int {
                    let mut p: C2RustUnnamed_13 =
                        C2RustUnnamed_13{c: 0 as *mut libc::c_uchar,};
                    p.c = (*cmptr).__cmsg_data.as_mut_ptr();
                    if_index = (*p.p).ipi_ifindex
                }
                cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            }
        }
    } else if set_ipv6pktinfo(fd) != 0 &&
                  getsockopt(fd, IPPROTO_IPV6 as libc::c_int,
                             6 as libc::c_int, msg.msg_control, &mut len) !=
                      -(1 as libc::c_int) {
        msg.msg_controllen = len as size_t;
        cmptr =
            if msg.msg_controllen >=
                   ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                msg.msg_control as *mut cmsghdr
            } else { 0 as *mut cmsghdr };
        while !cmptr.is_null() {
            if (*cmptr).cmsg_level == IPPROTO_IPV6 as libc::c_int &&
                   (*cmptr).cmsg_type == (*dnsmasq_daemon).v6pktinfo {
                let mut p_0: C2RustUnnamed_12 =
                    C2RustUnnamed_12{c: 0 as *mut libc::c_uchar,};
                p_0.c = (*cmptr).__cmsg_data.as_mut_ptr();
                if_index = (*p_0.p).ipi6_ifindex as libc::c_int
            }
            cmptr = __cmsg_nxthdr(&mut msg, cmptr)
        }
    }
    /* Only the RFC-2292 API has the ability to find the interface for TCP connections,
	 it was removed in RFC-3542 !!!! 

	 Fortunately, Linux kept the 2292 ABI when it moved to 3542. The following code always
	 uses the old ABI, and should work with pre- and post-3542 kernel headers */
    /* Linux */
    return if_index;
}
unsafe extern "C" fn create_listeners(mut addr: *mut mysockaddr,
                                      mut do_tftp: libc::c_int,
                                      mut dienow: libc::c_int)
 -> *mut listener {
    let mut l: *mut listener = 0 as *mut listener;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut tcpfd: libc::c_int = -(1 as libc::c_int);
    let mut tftpfd: libc::c_int = -(1 as libc::c_int);
    if (*dnsmasq_daemon).port != 0 as libc::c_int {
        fd = make_sock(addr, SOCK_DGRAM as libc::c_int, dienow);
        tcpfd = make_sock(addr, SOCK_STREAM as libc::c_int, dienow)
    }
    if do_tftp != 0 {
        if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int {
            /* port must be restored to DNS port for TCP code */
            let mut save: libc::c_short =
                (*addr).in_0.sin_port as libc::c_short;
            (*addr).in_0.sin_port =
                __bswap_16(69 as libc::c_int as __uint16_t);
            tftpfd = make_sock(addr, SOCK_DGRAM as libc::c_int, dienow);
            (*addr).in_0.sin_port = save as in_port_t
        } else {
            let mut save_0: libc::c_short =
                (*addr).in6.sin6_port as libc::c_short;
            (*addr).in6.sin6_port =
                __bswap_16(69 as libc::c_int as __uint16_t);
            tftpfd = make_sock(addr, SOCK_DGRAM as libc::c_int, dienow);
            (*addr).in6.sin6_port = save_0 as in_port_t
        }
    }
    if fd != -(1 as libc::c_int) || tcpfd != -(1 as libc::c_int) ||
           tftpfd != -(1 as libc::c_int) {
        l =
            safe_malloc(::std::mem::size_of::<listener>() as libc::c_ulong) as
                *mut listener;
        (*l).next = 0 as *mut listener;
        (*l).fd = fd;
        (*l).tcpfd = tcpfd;
        (*l).tftpfd = tftpfd;
        (*l).addr = *addr;
        (*l).used = 1 as libc::c_int;
        (*l).iface = 0 as *mut irec
    }
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn create_wildcard_listeners() {
    let mut addr: mysockaddr =
        mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut l: *mut listener = 0 as *mut listener;
    let mut l6: *mut listener = 0 as *mut listener;
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in_0.sin_family = 2 as libc::c_int as sa_family_t;
    addr.in_0.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    addr.in_0.sin_port = __bswap_16((*dnsmasq_daemon).port as __uint16_t);
    l =
        create_listeners(&mut addr,
                         ((*dnsmasq_daemon).options[(40 as libc::c_int as
                                                         libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_ulong))
                                                        as usize] &
                              (1 as libc::c_uint) <<
                                  (40 as libc::c_int as
                                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong))
                              != 0) as libc::c_int, 1 as libc::c_int);
    memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
    addr.in6.sin6_family = 10 as libc::c_int as sa_family_t;
    addr.in6.sin6_addr = in6addr_any;
    addr.in6.sin6_port = __bswap_16((*dnsmasq_daemon).port as __uint16_t);
    l6 =
        create_listeners(&mut addr,
                         ((*dnsmasq_daemon).options[(40 as libc::c_int as
                                                         libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_ulong))
                                                        as usize] &
                              (1 as libc::c_uint) <<
                                  (40 as libc::c_int as
                                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                        as
                                                                        libc::c_ulong).wrapping_mul(8
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong))
                              != 0) as libc::c_int, 1 as libc::c_int);
    if !l.is_null() { (*l).next = l6 } else { l = l6 }
    (*dnsmasq_daemon).listeners = l;
}
unsafe extern "C" fn find_listener(mut addr: *mut mysockaddr)
 -> *mut listener {
    let mut l: *mut listener = 0 as *mut listener;
    l = (*dnsmasq_daemon).listeners;
    while !l.is_null() {
        if sockaddr_isequal(&mut (*l).addr, addr) != 0 { return l }
        l = (*l).next
    }
    return 0 as *mut listener;
}
#[no_mangle]
pub unsafe extern "C" fn create_bound_listeners(mut dienow: libc::c_int) {
    let mut new: *mut listener = 0 as *mut listener;
    let mut iface: *mut irec = 0 as *mut irec;
    let mut if_tmp: *mut iname = 0 as *mut iname;
    let mut existing: *mut listener = 0 as *mut listener;
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).done == 0 && (*iface).dad == 0 && (*iface).found != 0 {
            existing = find_listener(&mut (*iface).addr);
            if !existing.is_null() {
                (*iface).done = 1 as libc::c_int;
                (*existing).used += 1
                /* increase usage counter */
            } else {
                new =
                    create_listeners(&mut (*iface).addr, (*iface).tftp_ok,
                                     dienow);
                if !new.is_null() {
                    (*new).iface = iface;
                    (*new).next = (*dnsmasq_daemon).listeners;
                    (*dnsmasq_daemon).listeners = new;
                    (*iface).done = 1 as libc::c_int;
                    /* Don't log the initial set of listen addresses created
               at startup, since this is happening before the logging
               system is initialised and the sign-on printed. */
                    if dienow == 0 {
                        let mut port: libc::c_int =
                            prettyprint_addr(&mut (*iface).addr,
                                             (*dnsmasq_daemon).addrbuff);
                        my_syslog(7 as libc::c_int,
                                  b"listening on %s(#%d): %s port %d\x00" as
                                      *const u8 as *const libc::c_char,
                                  (*iface).name, (*iface).index,
                                  (*dnsmasq_daemon).addrbuff, port);
                    }
                }
            }
        }
        iface = (*iface).next
    }
    /* Check for --listen-address options that haven't been used because there's
     no interface with a matching address. These may be valid: eg it's possible
     to listen on 127.0.1.1 even if the loopback interface is 127.0.0.1

     If the address isn't valid the bind() will fail and we'll die() 
     (except in bind-dynamic mode, when we'll complain but keep trying.)

     The resulting listeners have the ->iface field NULL, and this has to be
     handled by the DNS and TFTP code. It disables --localise-queries processing
     (no netmask) and some MTU login the tftp code. */
    if_tmp = (*dnsmasq_daemon).if_addrs;
    while !if_tmp.is_null() {
        if (*if_tmp).used == 0 &&
               {
                   new =
                       create_listeners(&mut (*if_tmp).addr,
                                        ((*dnsmasq_daemon).options[(40 as
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
                                                 (40 as libc::c_int as
                                                      libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                             != 0) as libc::c_int, dienow);
                   !new.is_null()
               } {
            (*new).next = (*dnsmasq_daemon).listeners;
            (*dnsmasq_daemon).listeners = new;
            if dienow == 0 {
                let mut port_0: libc::c_int =
                    prettyprint_addr(&mut (*if_tmp).addr,
                                     (*dnsmasq_daemon).addrbuff);
                my_syslog(7 as libc::c_int,
                          b"listening on %s port %d\x00" as *const u8 as
                              *const libc::c_char, (*dnsmasq_daemon).addrbuff,
                          port_0);
            }
        }
        if_tmp = (*if_tmp).next
    };
}
/* In --bind-interfaces, the only access control is the addresses we're listening on. 
   There's nothing to avoid a query to the address of an internal interface arriving via
   an external interface where we don't want to accept queries, except that in the usual 
   case the addresses of internal interfaces are RFC1918. When bind-interfaces in use, 
   and we listen on an address that looks like it's probably globally routeable, shout.

   The fix is to use --bind-dynamic, which actually checks the arrival interface too.
   Tough if your platform doesn't support this.

   Note that checking the arrival interface is supported in the standard IPv6 API and
   always done, so we don't warn about any IPv6 addresses here.
*/
#[no_mangle]
pub unsafe extern "C" fn warn_bound_listeners() {
    let mut iface: *mut irec = 0 as *mut irec;
    let mut advice: libc::c_int = 0 as libc::c_int;
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).dns_auth == 0 {
            if (*iface).addr.sa.sa_family as libc::c_int == 2 as libc::c_int {
                if private_net((*iface).addr.in_0.sin_addr, 1 as libc::c_int)
                       == 0 {
                    inet_ntop(2 as libc::c_int,
                              &mut (*iface).addr.in_0.sin_addr as *mut in_addr
                                  as *const libc::c_void,
                              (*dnsmasq_daemon).addrbuff,
                              46 as libc::c_int as socklen_t);
                    advice = 1 as libc::c_int;
                    (*iface).warned = advice;
                    my_syslog(4 as libc::c_int,
                              b"LOUD WARNING: listening on %s may accept requests via interfaces other than %s\x00"
                                  as *const u8 as *const libc::c_char,
                              (*dnsmasq_daemon).addrbuff, (*iface).name);
                }
            }
        }
        iface = (*iface).next
    }
    if advice != 0 {
        my_syslog(4 as libc::c_int,
                  b"LOUD WARNING: use --bind-dynamic rather than --bind-interfaces to avoid DNS amplification attacks via these interface(s)\x00"
                      as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn warn_wild_labels() {
    let mut iface: *mut irec = 0 as *mut irec;
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).found != 0 && !(*iface).name.is_null() &&
               (*iface).label != 0 {
            my_syslog(4 as libc::c_int,
                      b"warning: using interface %s instead\x00" as *const u8
                          as *const libc::c_char, (*iface).name);
        }
        iface = (*iface).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn warn_int_names() {
    let mut intname: *mut interface_name = 0 as *mut interface_name;
    intname = (*dnsmasq_daemon).int_names;
    while !intname.is_null() {
        if (*intname).addr.is_null() {
            my_syslog(4 as libc::c_int,
                      b"warning: no addresses found for interface %s\x00" as
                          *const u8 as *const libc::c_char, (*intname).intr);
        }
        intname = (*intname).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn is_dad_listeners() -> libc::c_int {
    let mut iface: *mut irec = 0 as *mut irec;
    if (*dnsmasq_daemon).options[(13 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (13 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        iface = (*dnsmasq_daemon).interfaces;
        while !iface.is_null() {
            if (*iface).dad != 0 && (*iface).done == 0 {
                return 1 as libc::c_int
            }
            iface = (*iface).next
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn join_multicast(mut dienow: libc::c_int) {
    let mut iface: *mut irec = 0 as *mut irec;
    let mut tmp: *mut irec = 0 as *mut irec;
    iface = (*dnsmasq_daemon).interfaces;
    while !iface.is_null() {
        if (*iface).addr.sa.sa_family as libc::c_int == 10 as libc::c_int &&
               (*iface).dhcp_ok != 0 && (*iface).multicast_done == 0 {
            /* There's an irec per address but we only want to join for multicast 
	   once per interface. Weed out duplicates. */
            tmp = (*dnsmasq_daemon).interfaces;
            while !tmp.is_null() {
                if (*tmp).multicast_done != 0 &&
                       (*tmp).index == (*iface).index {
                    break ;
                }
                tmp = (*tmp).next
            }
            (*iface).multicast_done = 1 as libc::c_int;
            if tmp.is_null() {
                let mut mreq: ipv6_mreq =
                    ipv6_mreq{ipv6mr_multiaddr:
                                  in6_addr{__in6_u:
                                               C2RustUnnamed{__u6_addr8:
                                                                 [0; 16],},},
                              ipv6mr_interface: 0,};
                let mut err: libc::c_int = 0 as libc::c_int;
                mreq.ipv6mr_interface = (*iface).index as libc::c_uint;
                inet_pton(10 as libc::c_int,
                          b"FF02::1:2\x00" as *const u8 as
                              *const libc::c_char,
                          &mut mreq.ipv6mr_multiaddr as *mut in6_addr as
                              *mut libc::c_void);
                if ((*dnsmasq_daemon).doing_dhcp6 != 0 ||
                        !(*dnsmasq_daemon).relay6.is_null()) &&
                       setsockopt((*dnsmasq_daemon).dhcp6fd,
                                  IPPROTO_IPV6 as libc::c_int,
                                  20 as libc::c_int,
                                  &mut mreq as *mut ipv6_mreq as
                                      *const libc::c_void,
                                  ::std::mem::size_of::<ipv6_mreq>() as
                                      libc::c_ulong as socklen_t) ==
                           -(1 as libc::c_int) {
                    err = *__errno_location()
                }
                inet_pton(10 as libc::c_int,
                          b"FF05::1:3\x00" as *const u8 as
                              *const libc::c_char,
                          &mut mreq.ipv6mr_multiaddr as *mut in6_addr as
                              *mut libc::c_void);
                if (*dnsmasq_daemon).doing_dhcp6 != 0 &&
                       setsockopt((*dnsmasq_daemon).dhcp6fd,
                                  IPPROTO_IPV6 as libc::c_int,
                                  20 as libc::c_int,
                                  &mut mreq as *mut ipv6_mreq as
                                      *const libc::c_void,
                                  ::std::mem::size_of::<ipv6_mreq>() as
                                      libc::c_ulong as socklen_t) ==
                           -(1 as libc::c_int) {
                    err = *__errno_location()
                }
                inet_pton(10 as libc::c_int,
                          b"FF02::2\x00" as *const u8 as *const libc::c_char,
                          &mut mreq.ipv6mr_multiaddr as *mut in6_addr as
                              *mut libc::c_void);
                if (*dnsmasq_daemon).doing_ra != 0 &&
                       setsockopt((*dnsmasq_daemon).icmp6fd,
                                  IPPROTO_IPV6 as libc::c_int,
                                  20 as libc::c_int,
                                  &mut mreq as *mut ipv6_mreq as
                                      *const libc::c_void,
                                  ::std::mem::size_of::<ipv6_mreq>() as
                                      libc::c_ulong as socklen_t) ==
                           -(1 as libc::c_int) {
                    err = *__errno_location()
                }
                if err != 0 {
                    let mut s: *mut libc::c_char =
                        b"interface %s failed to join DHCPv6 multicast group: %s\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char;
                    *__errno_location() = err;
                    if *__errno_location() == 12 as libc::c_int {
                        my_syslog(3 as libc::c_int,
                                  b"try increasing /proc/sys/net/core/optmem_max\x00"
                                      as *const u8 as *const libc::c_char);
                    }
                    if dienow != 0 {
                        die(s, (*iface).name, 2 as libc::c_int);
                    } else {
                        my_syslog(3 as libc::c_int, s, (*iface).name,
                                  strerror(*__errno_location()));
                    }
                }
            }
        }
        iface = (*iface).next
    };
}
/* return a UDP socket bound to a random port, have to cope with straying into
   occupied port nos and reserved ones. */
#[no_mangle]
pub unsafe extern "C" fn random_sock(mut family: libc::c_int) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    fd = socket(family, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
    if fd != -(1 as libc::c_int) {
        let mut addr: mysockaddr =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        let mut ports_avail: libc::c_uint =
            ((*dnsmasq_daemon).max_port as libc::c_ushort as libc::c_int -
                 (*dnsmasq_daemon).min_port as libc::c_ushort as libc::c_int +
                 1 as libc::c_int) as libc::c_uint;
        let mut tries: libc::c_int =
            if ports_avail < 30 as libc::c_int as libc::c_uint {
                (3 as libc::c_int as libc::c_uint).wrapping_mul(ports_avail)
            } else { 100 as libc::c_int as libc::c_uint } as libc::c_int;
        memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        addr.sa.sa_family = family as sa_family_t;
        /* don't loop forever if all ports in use. */
        if fix_fd(fd) != 0 {
            loop  {
                let fresh6 = tries;
                tries = tries - 1;
                if !(fresh6 != 0) { break ; }
                let mut port: libc::c_ushort =
                    __bswap_16(((*dnsmasq_daemon).min_port +
                                    rand16() as libc::c_int %
                                        ports_avail as libc::c_ushort as
                                            libc::c_int) as __uint16_t);
                if family == 2 as libc::c_int {
                    addr.in_0.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
                    addr.in_0.sin_port = port
                } else {
                    addr.in6.sin6_addr = in6addr_any;
                    addr.in6.sin6_port = port
                }
                if bind(fd,
                        __CONST_SOCKADDR_ARG{__sockaddr__:
                                                 &mut addr as *mut mysockaddr
                                                     as *mut sockaddr,},
                        sa_len(&mut addr) as socklen_t) == 0 as libc::c_int {
                    return fd
                }
                if *__errno_location() != 98 as libc::c_int &&
                       *__errno_location() != 13 as libc::c_int {
                    break ;
                }
            }
        }
        close(fd);
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn local_bind(mut fd: libc::c_int,
                                    mut addr: *mut mysockaddr,
                                    mut intname: *mut libc::c_char,
                                    mut ifindex: libc::c_uint,
                                    mut is_tcp: libc::c_int) -> libc::c_int {
    let mut addr_copy: mysockaddr = *addr;
    let mut port: libc::c_ushort = 0;
    let mut tries: libc::c_int = 1 as libc::c_int;
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut ports_avail: libc::c_uint =
        ((*dnsmasq_daemon).max_port as libc::c_ushort as libc::c_int -
             (*dnsmasq_daemon).min_port as libc::c_ushort as libc::c_int +
             1 as libc::c_int) as libc::c_uint;
    if addr_copy.sa.sa_family as libc::c_int == 2 as libc::c_int {
        port = addr_copy.in_0.sin_port
    } else { port = addr_copy.in6.sin6_port }
    /* cannot set source _port_ for TCP connections. */
    if is_tcp != 0 { port = 0 as libc::c_int as libc::c_ushort }
    /* Bind a random port within the range given by min-port and max-port */
    if port as libc::c_int == 0 as libc::c_int {
        tries =
            if ports_avail < 30 as libc::c_int as libc::c_uint {
                (3 as libc::c_int as libc::c_uint).wrapping_mul(ports_avail)
            } else { 100 as libc::c_int as libc::c_uint } as libc::c_int;
        port =
            __bswap_16(((*dnsmasq_daemon).min_port +
                            rand16() as libc::c_int %
                                ports_avail as libc::c_ushort as libc::c_int)
                           as __uint16_t)
    }
    loop  {
        let fresh7 = tries;
        tries = tries - 1;
        if !(fresh7 != 0) { break ; }
        if addr_copy.sa.sa_family as libc::c_int == 2 as libc::c_int {
            addr_copy.in_0.sin_port = port
        } else { addr_copy.in6.sin6_port = port }
        if bind(fd,
                __CONST_SOCKADDR_ARG{__sockaddr__:
                                         &mut addr_copy as *mut mysockaddr as
                                             *mut sockaddr,},
                sa_len(&mut addr_copy) as socklen_t) != -(1 as libc::c_int) {
            done = 1 as libc::c_int;
            break ;
        } else {
            if *__errno_location() != 98 as libc::c_int &&
                   *__errno_location() != 13 as libc::c_int {
                return 0 as libc::c_int
            }
            port =
                __bswap_16(((*dnsmasq_daemon).min_port +
                                rand16() as libc::c_int %
                                    ports_avail as libc::c_ushort as
                                        libc::c_int) as __uint16_t)
        }
    }
    if done == 0 { return 0 as libc::c_int }
    if is_tcp == 0 && ifindex > 0 as libc::c_int as libc::c_uint {
        if addr_copy.sa.sa_family as libc::c_int == 2 as libc::c_int {
            let mut ifindex_opt: uint32_t = __bswap_32(ifindex);
            return (setsockopt(fd, IPPROTO_IP as libc::c_int,
                               50 as libc::c_int,
                               &mut ifindex_opt as *mut uint32_t as
                                   *const libc::c_void,
                               ::std::mem::size_of::<uint32_t>() as
                                   libc::c_ulong as socklen_t) ==
                        0 as libc::c_int) as libc::c_int
        }
        if addr_copy.sa.sa_family as libc::c_int == 10 as libc::c_int {
            let mut ifindex_opt_0: uint32_t = __bswap_32(ifindex);
            return (setsockopt(fd, IPPROTO_IPV6 as libc::c_int,
                               76 as libc::c_int,
                               &mut ifindex_opt_0 as *mut uint32_t as
                                   *const libc::c_void,
                               ::std::mem::size_of::<uint32_t>() as
                                   libc::c_ulong as socklen_t) ==
                        0 as libc::c_int) as libc::c_int
        }
    }
    /* suppress potential unused warning */
    if *intname.offset(0 as libc::c_int as isize) as libc::c_int !=
           0 as libc::c_int &&
           setsockopt(fd, 1 as libc::c_int, 25 as libc::c_int,
                      intname as *const libc::c_void,
                      16 as libc::c_int as socklen_t) == -(1 as libc::c_int) {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn allocate_sfd(mut addr: *mut mysockaddr,
                                  mut intname: *mut libc::c_char)
 -> *mut serverfd {
    let mut sfd: *mut serverfd = 0 as *mut serverfd;
    let mut ifindex: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut errsave: libc::c_int = 0;
    let mut opt: libc::c_int = 1 as libc::c_int;
    /* when using random ports, servers which would otherwise use
     the INADDR_ANY/port0 socket have sfd set to NULL */
    if (*dnsmasq_daemon).osport == 0 &&
           *intname.offset(0 as libc::c_int as isize) as libc::c_int ==
               0 as libc::c_int {
        *__errno_location() =
            0 as
                libc::c_int; /* index == 0 when not binding to an interface */
        if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int &&
               (*addr).in_0.sin_addr.s_addr == 0 as libc::c_int as in_addr_t
               &&
               (*addr).in_0.sin_port as libc::c_int ==
                   __bswap_16(0 as libc::c_int as __uint16_t) as libc::c_int {
            return 0 as *mut serverfd
        }
        if (*addr).sa.sa_family as libc::c_int == 10 as libc::c_int &&
               memcmp(&mut (*addr).in6.sin6_addr as *mut in6_addr as
                          *const libc::c_void,
                      &in6addr_any as *const in6_addr as *const libc::c_void,
                      ::std::mem::size_of::<in6_addr>() as libc::c_ulong) ==
                   0 as libc::c_int &&
               (*addr).in6.sin6_port as libc::c_int ==
                   __bswap_16(0 as libc::c_int as __uint16_t) as libc::c_int {
            return 0 as *mut serverfd
        }
    }
    if !intname.is_null() &&
           strlen(intname) != 0 as libc::c_int as libc::c_ulong {
        ifindex = if_nametoindex(intname)
    }
    /* may have a suitable one already */
    sfd = (*dnsmasq_daemon).sfds;
    while !sfd.is_null() {
        if sockaddr_isequal(&mut (*sfd).source_addr, addr) != 0 &&
               strcmp(intname, (*sfd).interface.as_mut_ptr()) ==
                   0 as libc::c_int && ifindex == (*sfd).ifindex {
            return sfd
        }
        sfd = (*sfd).next
    }
    /* need to make a new one. */
    *__errno_location() = 12 as libc::c_int; /* in case malloc fails. */
    sfd =
        whine_malloc(::std::mem::size_of::<serverfd>() as libc::c_ulong) as
            *mut serverfd; /* save error from bind/setsockopt. */
    if sfd.is_null() { return 0 as *mut serverfd }
    (*sfd).fd =
        socket((*addr).sa.sa_family as libc::c_int, SOCK_DGRAM as libc::c_int,
               0 as libc::c_int);
    if (*sfd).fd == -(1 as libc::c_int) {
        free(sfd as *mut libc::c_void);
        return 0 as *mut serverfd
    }
    if (*addr).sa.sa_family as libc::c_int == 10 as libc::c_int &&
           setsockopt((*sfd).fd, IPPROTO_IPV6 as libc::c_int,
                      26 as libc::c_int,
                      &mut opt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) ||
           local_bind((*sfd).fd, addr, intname, ifindex, 0 as libc::c_int) ==
               0 || fix_fd((*sfd).fd) == 0 {
        errsave = *__errno_location();
        close((*sfd).fd);
        free(sfd as *mut libc::c_void);
        *__errno_location() = errsave;
        return 0 as *mut serverfd
    }
    safe_strncpy((*sfd).interface.as_mut_ptr(), intname,
                 ::std::mem::size_of::<[libc::c_char; 17]>() as
                     libc::c_ulong);
    (*sfd).source_addr = *addr;
    (*sfd).next = (*dnsmasq_daemon).sfds;
    (*sfd).ifindex = ifindex;
    (*sfd).preallocated = 0 as libc::c_int as libc::c_uint;
    (*dnsmasq_daemon).sfds = sfd;
    return sfd;
}
/* create upstream sockets during startup, before root is dropped which may be needed
   this allows query_port to be a low port and interface binding */
#[no_mangle]
pub unsafe extern "C" fn pre_allocate_sfds() {
    let mut srv: *mut server = 0 as *mut server;
    let mut sfd: *mut serverfd = 0 as *mut serverfd;
    if (*dnsmasq_daemon).query_port != 0 as libc::c_int {
        let mut addr: mysockaddr =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        addr.in_0.sin_family = 2 as libc::c_int as sa_family_t;
        addr.in_0.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
        addr.in_0.sin_port =
            __bswap_16((*dnsmasq_daemon).query_port as __uint16_t);
        sfd =
            allocate_sfd(&mut addr,
                         b"\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char);
        if !sfd.is_null() {
            (*sfd).preallocated = 1 as libc::c_int as libc::c_uint
        }
        memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        addr.in6.sin6_family = 10 as libc::c_int as sa_family_t;
        addr.in6.sin6_addr = in6addr_any;
        addr.in6.sin6_port =
            __bswap_16((*dnsmasq_daemon).query_port as __uint16_t);
        sfd =
            allocate_sfd(&mut addr,
                         b"\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char);
        if !sfd.is_null() {
            (*sfd).preallocated = 1 as libc::c_int as libc::c_uint
        }
    }
    srv = (*dnsmasq_daemon).servers;
    while !srv.is_null() {
        if (*srv).flags &
               (4 as libc::c_int | 2 as libc::c_int | 1024 as libc::c_int |
                    2048 as libc::c_int) == 0 &&
               allocate_sfd(&mut (*srv).source_addr,
                            (*srv).interface.as_mut_ptr()).is_null() &&
               *__errno_location() != 0 as libc::c_int &&
               (*dnsmasq_daemon).options[(13 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (13 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
            prettyprint_addr(&mut (*srv).source_addr,
                             (*dnsmasq_daemon).namebuff);
            if (*srv).interface[0 as libc::c_int as usize] as libc::c_int !=
                   0 as libc::c_int {
                strcat((*dnsmasq_daemon).namebuff,
                       b" \x00" as *const u8 as *const libc::c_char);
                strcat((*dnsmasq_daemon).namebuff,
                       (*srv).interface.as_mut_ptr());
            }
            die(b"failed to bind server socket for %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*dnsmasq_daemon).namebuff, 2 as libc::c_int);
        }
        srv = (*srv).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn mark_servers(mut flag: libc::c_int) {
    let mut serv: *mut server = 0 as *mut server;
    /* mark everything with argument flag */
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if (*serv).flags & flag != 0 { (*serv).flags |= 256 as libc::c_int }
        /* Give looped servers another chance */
        (*serv).flags &= !(8192 as libc::c_int);
        serv = (*serv).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn cleanup_servers() {
    let mut serv: *mut server = 0 as *mut server;
    let mut tmp: *mut server = 0 as *mut server;
    let mut up: *mut *mut server = 0 as *mut *mut server;
    /* unlink and free anything still marked. */
    serv = (*dnsmasq_daemon).servers;
    up = &mut (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        tmp = (*serv).next;
        if (*serv).flags & 256 as libc::c_int != 0 {
            server_gone(serv);
            *up = (*serv).next;
            if !(*serv).domain.is_null() {
                free((*serv).domain as *mut libc::c_void);
            }
            free(serv as *mut libc::c_void);
        } else { up = &mut (*serv).next }
        serv = tmp
    }
    /* Now we have a new set of servers, test for loops. */
    loop_send_probes();
}
#[no_mangle]
pub unsafe extern "C" fn add_update_server(mut flags: libc::c_int,
                                           mut addr: *mut mysockaddr,
                                           mut source_addr: *mut mysockaddr,
                                           mut interface: *const libc::c_char,
                                           mut domain: *const libc::c_char) {
    let mut serv: *mut server = 0 as *mut server;
    let mut next: *mut server = 0 as *mut server;
    let mut domain_str: *mut libc::c_char = 0 as *mut libc::c_char;
    /* See if there is a suitable candidate, and unmark */
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if (*serv).flags & 256 as libc::c_int != 0 {
            if !domain.is_null() {
                if !((*serv).flags & 8 as libc::c_int == 0 ||
                         hostname_isequal(domain, (*serv).domain) == 0) {
                    break ;
                }
            } else if !((*serv).flags & 8 as libc::c_int != 0) { break ; }
        }
        serv = (*serv).next
    }
    if !serv.is_null() {
        domain_str = (*serv).domain;
        next = (*serv).next
    } else {
        serv =
            whine_malloc(::std::mem::size_of::<server>() as libc::c_ulong) as
                *mut server;
        if !serv.is_null() {
            /* Not found, create a new one. */
            if !domain.is_null() &&
                   {
                       domain_str =
                           whine_malloc(strlen(domain).wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong))
                               as *mut libc::c_char;
                       domain_str.is_null()
                   } {
                free(serv as *mut libc::c_void);
                serv = 0 as *mut server
            } else {
                let mut s: *mut server = 0 as *mut server;
                /* Add to the end of the chain, for order */
                if (*dnsmasq_daemon).servers.is_null() {
                    (*dnsmasq_daemon).servers = serv
                } else {
                    s = (*dnsmasq_daemon).servers;
                    while !(*s).next.is_null() { s = (*s).next }
                    (*s).next = serv
                }
                if !domain.is_null() { strcpy(domain_str, domain); }
            }
        }
    }
    if !serv.is_null() {
        memset(serv as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<server>() as libc::c_ulong);
        (*serv).flags = flags;
        (*serv).domain = domain_str;
        (*serv).next = next;
        (*serv).failed_queries = 0 as libc::c_int as libc::c_uint;
        (*serv).queries = (*serv).failed_queries;
        (*serv).uid = rand32();
        if !domain.is_null() { (*serv).flags |= 8 as libc::c_int }
        if !interface.is_null() {
            safe_strncpy((*serv).interface.as_mut_ptr(), interface,
                         ::std::mem::size_of::<[libc::c_char; 17]>() as
                             libc::c_ulong);
        }
        if !addr.is_null() { (*serv).addr = *addr }
        if !source_addr.is_null() { (*serv).source_addr = *source_addr }
    };
}
#[no_mangle]
pub unsafe extern "C" fn check_servers() {
    let mut iface: *mut irec = 0 as *mut irec;
    let mut serv: *mut server = 0 as *mut server;
    let mut sfd: *mut serverfd = 0 as *mut serverfd;
    let mut tmp: *mut serverfd = 0 as *mut serverfd;
    let mut up: *mut *mut serverfd = 0 as *mut *mut serverfd;
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0;
    let mut locals: libc::c_int = 0 as libc::c_int;
    /* interface may be new since startup */
    if (*dnsmasq_daemon).options[(13 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (13 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        enumerate_interfaces(0 as libc::c_int);
    }
    /* don't garbage collect pre-allocated sfds. */
    sfd = (*dnsmasq_daemon).sfds;
    while !sfd.is_null() {
        (*sfd).used = (*sfd).preallocated;
        sfd = (*sfd).next
    }
    let mut current_block_37: u64;
    count = 0 as libc::c_int;
    serv = (*dnsmasq_daemon).servers;
    while !serv.is_null() {
        if (*serv).flags &
               (4 as libc::c_int | 2 as libc::c_int | 1024 as libc::c_int |
                    2048 as libc::c_int) == 0 {
            /* Init edns_pktsz for newly created server records. */
            if (*serv).edns_pktsz == 0 as libc::c_int {
                (*serv).edns_pktsz =
                    (*dnsmasq_daemon).edns_pktsz as libc::c_int
            }
            port =
                prettyprint_addr(&mut (*serv).addr,
                                 (*dnsmasq_daemon).namebuff);
            /* 0.0.0.0 is nothing, the stack treats it like 127.0.0.1 */
            if (*serv).addr.sa.sa_family as libc::c_int == 2 as libc::c_int &&
                   (*serv).addr.in_0.sin_addr.s_addr ==
                       0 as libc::c_int as libc::c_uint {
                (*serv).flags |= 256 as libc::c_int;
                current_block_37 = 8515828400728868193;
            } else {
                iface = (*dnsmasq_daemon).interfaces;
                while !iface.is_null() {
                    if sockaddr_isequal(&mut (*serv).addr, &mut (*iface).addr)
                           != 0 {
                        break ;
                    }
                    iface = (*iface).next
                }
                if !iface.is_null() {
                    my_syslog(4 as libc::c_int,
                              b"ignoring nameserver %s - local interface\x00"
                                  as *const u8 as *const libc::c_char,
                              (*dnsmasq_daemon).namebuff);
                    (*serv).flags |= 256 as libc::c_int;
                    current_block_37 = 8515828400728868193;
                } else if (*serv).sfd.is_null() &&
                              {
                                  (*serv).sfd =
                                      allocate_sfd(&mut (*serv).source_addr,
                                                   (*serv).interface.as_mut_ptr());
                                  (*serv).sfd.is_null()
                              } && *__errno_location() != 0 as libc::c_int {
                    my_syslog(4 as libc::c_int,
                              b"ignoring nameserver %s - cannot make/bind socket: %s\x00"
                                  as *const u8 as *const libc::c_char,
                              (*dnsmasq_daemon).namebuff,
                              strerror(*__errno_location()));
                    (*serv).flags |= 256 as libc::c_int;
                    current_block_37 = 8515828400728868193;
                } else {
                    if !(*serv).sfd.is_null() {
                        (*(*serv).sfd).used = 1 as libc::c_int as libc::c_uint
                    }
                    current_block_37 = 3437258052017859086;
                }
            }
        } else { current_block_37 = 3437258052017859086; }
        match current_block_37 {
            3437258052017859086 => {
                if (*serv).flags & 2048 as libc::c_int == 0 &&
                       (*serv).flags & 4 as libc::c_int == 0 {
                    count += 1;
                    if !(count > 30 as libc::c_int) {
                        if (*serv).flags &
                               (8 as libc::c_int | 32 as libc::c_int |
                                    1024 as libc::c_int) != 0 {
                            let mut s1: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut s2: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut s3: *mut libc::c_char =
                                b"\x00" as *const u8 as *const libc::c_char as
                                    *mut libc::c_char;
                            if (*serv).flags & 8 as libc::c_int == 0 {
                                s1 =
                                    b"unqualified\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char;
                                s2 =
                                    b"names\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char
                            } else if strlen((*serv).domain) ==
                                          0 as libc::c_int as libc::c_ulong {
                                s1 =
                                    b"default\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char;
                                s2 =
                                    b"\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char
                            } else {
                                s1 =
                                    b"domain\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char;
                                s2 = (*serv).domain
                            }
                            if (*serv).flags & 2 as libc::c_int != 0 {
                                count -= 1;
                                locals += 1;
                                if locals <= 8 as libc::c_int {
                                    my_syslog(6 as libc::c_int,
                                              b"using only locally-known addresses for %s %s\x00"
                                                  as *const u8 as
                                                  *const libc::c_char, s1,
                                              s2);
                                }
                            } else if (*serv).flags & 1024 as libc::c_int != 0
                             {
                                my_syslog(6 as libc::c_int,
                                          b"using standard nameservers for %s %s\x00"
                                              as *const u8 as
                                              *const libc::c_char, s1, s2);
                            } else {
                                my_syslog(6 as libc::c_int,
                                          b"using nameserver %s#%d for %s %s %s\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          (*dnsmasq_daemon).namebuff, port,
                                          s1, s2, s3);
                            }
                        } else if (*serv).flags & 8192 as libc::c_int != 0 {
                            my_syslog(6 as libc::c_int,
                                      b"NOT using nameserver %s#%d - query loop detected\x00"
                                          as *const u8 as *const libc::c_char,
                                      (*dnsmasq_daemon).namebuff, port);
                        } else if (*serv).interface[0 as libc::c_int as usize]
                                      as libc::c_int != 0 as libc::c_int {
                            my_syslog(6 as libc::c_int,
                                      b"using nameserver %s#%d(via %s)\x00" as
                                          *const u8 as *const libc::c_char,
                                      (*dnsmasq_daemon).namebuff, port,
                                      (*serv).interface.as_mut_ptr());
                        } else {
                            my_syslog(6 as libc::c_int,
                                      b"using nameserver %s#%d\x00" as
                                          *const u8 as *const libc::c_char,
                                      (*dnsmasq_daemon).namebuff, port);
                        }
                    }
                }
            }
            _ => { }
        }
        serv = (*serv).next
    }
    if locals > 8 as libc::c_int {
        my_syslog(6 as libc::c_int,
                  b"using %d more local addresses\x00" as *const u8 as
                      *const libc::c_char, locals - 8 as libc::c_int);
    }
    if count - 1 as libc::c_int > 30 as libc::c_int {
        my_syslog(6 as libc::c_int,
                  b"using %d more nameservers\x00" as *const u8 as
                      *const libc::c_char,
                  count - 30 as libc::c_int - 1 as libc::c_int);
    }
    /* Do we need a socket set? */
    /* Remove unused sfds */
    sfd = (*dnsmasq_daemon).sfds;
    up = &mut (*dnsmasq_daemon).sfds;
    while !sfd.is_null() {
        tmp = (*sfd).next;
        if (*sfd).used == 0 {
            *up = (*sfd).next;
            close((*sfd).fd);
            free(sfd as *mut libc::c_void);
        } else { up = &mut (*sfd).next }
        sfd = tmp
    }
    cleanup_servers();
}
/* Return zero if no servers found, in that case we keep polling.
   This is a protection against an update-time/write race on resolv.conf */
#[no_mangle]
pub unsafe extern "C" fn reload_servers(mut fname: *mut libc::c_char)
 -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gotone: libc::c_int = 0 as libc::c_int;
    /* buff happens to be MAXDNAME long... */
    f = fopen(fname, b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        my_syslog(3 as libc::c_int,
                  b"failed to read %s: %s\x00" as *const u8 as
                      *const libc::c_char, fname,
                  strerror(*__errno_location()));
        return 0 as libc::c_int
    }
    mark_servers(1 as libc::c_int);
    loop  {
        line = fgets((*dnsmasq_daemon).namebuff, 1025 as libc::c_int, f);
        if line.is_null() { break ; }
        let mut addr: mysockaddr =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        let mut source_addr: mysockaddr =
            mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
        let mut token: *mut libc::c_char =
            strtok(line, b" \t\n\r\x00" as *const u8 as *const libc::c_char);
        if token.is_null() { continue ; }
        if strcmp(token,
                  b"nameserver\x00" as *const u8 as *const libc::c_char) !=
               0 as libc::c_int &&
               strcmp(token,
                      b"server\x00" as *const u8 as *const libc::c_char) !=
                   0 as libc::c_int {
            continue ;
        }
        token =
            strtok(0 as *mut libc::c_char,
                   b" \t\n\r\x00" as *const u8 as *const libc::c_char);
        if token.is_null() { continue ; }
        memset(&mut addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        memset(&mut source_addr as *mut mysockaddr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<mysockaddr>() as libc::c_ulong);
        addr.in_0.sin_addr.s_addr = inet_addr(token);
        if addr.in_0.sin_addr.s_addr != -(1 as libc::c_int) as in_addr_t {
            addr.in_0.sin_family = 2 as libc::c_int as sa_family_t;
            source_addr.in_0.sin_family = addr.in_0.sin_family;
            addr.in_0.sin_port = __bswap_16(53 as libc::c_int as __uint16_t);
            source_addr.in_0.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
            source_addr.in_0.sin_port =
                __bswap_16((*dnsmasq_daemon).query_port as __uint16_t)
        } else {
            let mut scope_index: libc::c_int = 0 as libc::c_int;
            let mut scope_id: *mut libc::c_char = strchr(token, '%' as i32);
            if !scope_id.is_null() {
                let fresh8 = scope_id;
                scope_id = scope_id.offset(1);
                *fresh8 = 0 as libc::c_int as libc::c_char;
                scope_index = if_nametoindex(scope_id) as libc::c_int
            }
            if !(inet_pton(10 as libc::c_int, token,
                           &mut addr.in6.sin6_addr as *mut in6_addr as
                               *mut libc::c_void) > 0 as libc::c_int) {
                continue ;
            }
            addr.in6.sin6_family = 10 as libc::c_int as sa_family_t;
            source_addr.in6.sin6_family = addr.in6.sin6_family;
            addr.in6.sin6_flowinfo = 0 as libc::c_int as uint32_t;
            source_addr.in6.sin6_flowinfo = addr.in6.sin6_flowinfo;
            addr.in6.sin6_port = __bswap_16(53 as libc::c_int as __uint16_t);
            addr.in6.sin6_scope_id = scope_index as uint32_t;
            source_addr.in6.sin6_addr = in6addr_any;
            source_addr.in6.sin6_port =
                __bswap_16((*dnsmasq_daemon).query_port as __uint16_t);
            source_addr.in6.sin6_scope_id = 0 as libc::c_int as uint32_t
        }
        add_update_server(1 as libc::c_int, &mut addr, &mut source_addr,
                          0 as *const libc::c_char, 0 as *const libc::c_char);
        gotone = 1 as libc::c_int
    }
    fclose(f);
    cleanup_servers();
    return gotone;
}
/* Called when addresses are added or deleted from an interface */
#[no_mangle]
pub unsafe extern "C" fn newaddress(mut now: time_t) {
    if (*dnsmasq_daemon).options[(39 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (39 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 ||
           (*dnsmasq_daemon).options[(49 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (49 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 || (*dnsmasq_daemon).doing_dhcp6 != 0 ||
           !(*dnsmasq_daemon).relay6.is_null() ||
           (*dnsmasq_daemon).doing_ra != 0 {
        enumerate_interfaces(0 as libc::c_int);
    }
    if (*dnsmasq_daemon).options[(39 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (39 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        create_bound_listeners(0 as libc::c_int);
    }
    if (*dnsmasq_daemon).doing_dhcp6 != 0 ||
           !(*dnsmasq_daemon).relay6.is_null() ||
           (*dnsmasq_daemon).doing_ra != 0 {
        join_multicast(0 as libc::c_int);
    }
    if (*dnsmasq_daemon).doing_dhcp6 != 0 || (*dnsmasq_daemon).doing_ra != 0 {
        dhcp_construct_contexts(now);
    }
    if (*dnsmasq_daemon).doing_dhcp6 != 0 { lease_find_interfaces(now); };
}
