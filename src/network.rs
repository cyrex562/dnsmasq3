
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
use crate::slack::{ifreq, IFF_LOOPBACK, uint32_t, ipv6_mreq};
use crate::defines::{DigitalSignature, NetAddress, C2rustUnnamed1a, IfReq, NetAddress, Iname, DnsmasqDaemon, In6Addr, Irec, IfaceParam, NetAddress, NetAddress, AddressListEntry, __bswap_32, InterfaceName, AuthZone, AuthNameList, InAddrT, SaFamily, __bswap_16, __uint16_t, Listener, SOCK_DGRAM, socklen_t, IPPROTO_IPV6, ConstNetAddressArg, SOCK_STREAM, IPPROTO_TCP, IPPROTO_IP, CmsgHdr, MsgHdr, iovec, Server, C2RustUnnamed_13, C2rustUnnamed12, in_port_t, C2RustUnnamed, ServerFd, time::Instant, NetAddress, AddressType};
use crate::util::{safe_strncpy, wildcard_match, NetAddress_isequal, prettyprint_addr, sa_len, rand16, hostname_isequal, rand32, netaddr_isequal};
use crate::dnsmasq_log::{my_syslog, die};
use crate::netlink::iface_enumerate;
use crate::rfc1035::private_net;
use crate::forward::server_gone;
use crate::dnsmasq_loop::loop_send_probes;
use crate::dhcp6::dhcp_construct_contexts;
use crate::lease::lease_find_interfaces;
use winapi::um::winbase::{AddAtomA, DefineDosDeviceA};


pub fn indextoname(mut fd: i32,
                                     mut index: i32,
                                     mut name: &mut String)
 -> i32 {
    let mut ifr: IfReq =
        IfReq {ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2rustUnnamed1a {ifru_addr:
                                      NetAddress {sa_family: 0,
                                               sa_data: [0; 14],},},};
    if index == 0 { return 0 }
    ifr.ifr_ifru.ifru_ivalue = index;
    if ioctl(fd, 0x8910,
             &mut ifr) == -(1) {
        return 0
    }
    safe_strncpy(name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                 16 );
    return 1;
}

pub fn iface_check(mut family: i32,
                                     mut addr: &mut NetAddress,
                                     mut name: &mut String,
                                     mut auth: )
                                     -> i32 {
    let mut tmp: Iname = 0;
    let mut ret: i32 = 1;
    let mut match_addr: i32 = 0;
    /* Note: have to check all and not bail out early, so that we set the
     "used" flags.

     May be called with family == AF_LOCALto check interface by name only. */
    if !auth.is_null() { *auth = 0 }
    if !daemon.if_names.is_null() ||
           !daemon.if_addrs.is_null() {
        ret = 0;
        tmp = daemon.if_names;
        while !tmp.is_null() {
            if !tmp.name.is_null() &&
                   wildcard_match(tmp.name, name) != 0 {
                tmp.used = 1;
                ret = tmp.used
            }
            tmp = tmp.next
        }
        if !addr.is_null() {
            tmp = daemon.if_addrs;
            while !tmp.is_null() {
                if tmp.addr.sa.sa_family == family {
                    if family == 2 &&
                           tmp.addr.in_0.sin_addr.s_addr ==
                               addr.addr4.s_addr {
                        tmp.used = 1;
                        match_addr = tmp.used;
                        ret = match_addr
                    } else if family == 10 &&
                                  ({
                                       let mut __a: *const In6Addr =
                                           &mut tmp.addr.in6.sin6_addr                                         &mut In6Addr                                         *const In6Addr;
                                       let mut __b: *const In6Addr =
                                           &mut addr.addr6
                                               ;
                                       (__a.__in6_u.__u6_addr32[0                    libc::c_int
                                                                                          usize]
                                            ==
                                            __b.__in6_u.__u6_addr32[0                        libc::c_int
                                                                                                  usize]
                                            &&
                                            __a.__in6_u.__u6_addr32[1                        libc::c_int
                                                                                                  usize]
                                                ==
                                                __b.__in6_u.__u6_addr32[1
                                                                                                          libc::c_int
                                                                                                          usize]
                                            &&
                                            __a.__in6_u.__u6_addr32[2                        libc::c_int
                                                                                                  usize]
                                                ==
                                                __b.__in6_u.__u6_addr32[2
                                                                                                          libc::c_int
                                                                                                          usize]
                                            &&
                                            __a.__in6_u.__u6_addr32[3                        libc::c_int
                                                                                                  usize]
                                                ==
                                                __b.__in6_u.__u6_addr32[3
                                                                                                          libc::c_int
                                                                                                          usize])

                                   }) != 0 {
                        tmp.used = 1;
                        match_addr = tmp.used;
                        ret = match_addr
                    }
                }
                tmp = tmp.next
            }
        }
    }
    if match_addr == 0 {
        tmp = daemon.if_except;
        while !tmp.is_null() {
            if !tmp.name.is_null() &&
                   wildcard_match(tmp.name, name) != 0 {
                ret = 0
            }
            tmp = tmp.next
        }
    }
    tmp = daemon.authinterface;
    while !tmp.is_null() {
        if !tmp.name.is_null() {
            if strcmp(tmp.name, name) == 0 &&
                   (tmp.addr.sa.sa_family ==
                        0 ||
                        tmp.addr.sa.sa_family == family) {
                break ;
            }
        } else {
            if !addr.is_null() &&
                   tmp.addr.sa.sa_family == 2
                   && family == 2 &&
                   tmp.addr.in_0.sin_addr.s_addr == addr.addr4.s_addr {
                break ;
            }
            if !addr.is_null() &&
                   tmp.addr.sa.sa_family ==
                       10 && family == 10 &&
                   ({
                        let mut __a: *const In6Addr =
                            &mut tmp.addr.in6.sin6_addr                          *const In6Addr;
                        let mut __b: *const In6Addr =
                            &mut addr.addr6                          *const In6Addr;
                        (__a.__in6_u.__u6_addr32[0 ]
                             ==
                             __b.__in6_u.__u6_addr32[0         usize] &&
                             __a.__in6_u.__u6_addr32[1         usize] ==
                                 __b.__in6_u.__u6_addr32[1] &&
                             __a.__in6_u.__u6_addr32[2         usize] ==
                                 __b.__in6_u.__u6_addr32[2] &&
                             __a.__in6_u.__u6_addr32[3         usize] ==
                                 __b.__in6_u.__u6_addr32[3])                      libc::c_int
                    }) != 0 {
                break ;
            }
        }
        tmp = tmp.next
    }
    if !tmp.is_null() && !auth.is_null() {
        *auth = 1;
        ret = 1
    }
    return ret;
}
/* Fix for problem that the kernel sometimes reports the loopback interface as the
   arrival interface when a packet originates locally, even when sent to address of 
   an interface other than the loopback. Accept packet if it arrived via a loopback 
   interface, even when we're not accepting packets that way, as long as the destination
   address is one we're believing. Interface list must be up-to-date before calling. */

pub fn loopback_exception(mut fd: i32,
                                            mut family: i32,
                                            mut addr: &mut NetAddress,
                                            mut name: &mut String)
                                            -> i32 {
    let mut ifr: IfReq =
        IfReq {ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  DigitalSignature {
                      keydata: blockdata {},
                      keylen: 0,
                      keytag: 0,
                      algo: 0,
                      ifru_addr:
                                      NetAddress {sa_family: 0,
                                               sa_data: [0; 14],},
                      digest: 0
                  },};
    let mut iface: Irec = 0 ;
    safe_strncpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), name,
                 16 );
    if ioctl(fd, 0x8913,
             &mut ifr) != -(1) &&
           ifr.ifr_ifru.ifru_flags &
               IFF_LOOPBACK != 0 {
        iface = daemon.interfaces;
        while !iface.is_null() {
            if iface.addr.sa.sa_family == family {
                if family == 2 {
                    if iface.addr.in_0.sin_addr.s_addr ==
                           addr.addr4.s_addr {
                        return 1
                    }
                } else if ({
                               let mut __a: *const In6Addr =
                                   &mut iface.addr.in6.sin6_addr                                 &mut In6Addr ;
                               let mut __b: *const In6Addr =
                                   &mut addr.addr6                                 *const In6Addr;
                               (__a.__in6_u.__u6_addr32[0            usize] ==
                                    __b.__in6_u.__u6_addr32[0                libc::c_int
                                                                   ]
                                    &&
                                    __a.__in6_u.__u6_addr32[1                libc::c_int
                                                                   ]
                                        ==
                                        __b.__in6_u.__u6_addr32[1                    libc::c_int
                                                                                          usize]
                                    &&
                                    __a.__in6_u.__u6_addr32[2                libc::c_int
                                                                   ]
                                        ==
                                        __b.__in6_u.__u6_addr32[2                    libc::c_int
                                                                                          usize]
                                    &&
                                    __a.__in6_u.__u6_addr32[3                libc::c_int
                                                                   ]
                                        ==
                                        __b.__in6_u.__u6_addr32[3                    libc::c_int
                                                                                          usize])

                           }) != 0 {
                    return 1
                }
            }
            iface = iface.next
        }
    }
    return 0;
}
/* If we're configured with something like --interface=eth0:0 then we'll listen correctly
   on the relevant address, but the name of the arrival interface, derived from the
   index won't match the config. Check that we found an interface address for the arrival 
   interface: daemon->interfaces must be up-to-date. */

pub fn label_exception(mut index: i32,
                                         mut family: i32,
                                         mut addr: &mut NetAddress)
 -> i32 {
    let mut iface: Irec = 0 ;
    /* labels only supported on IPv4 addresses. */
    if family != 2 { return 0 }
    iface = daemon.interfaces;
    while !iface.is_null() {
        if iface.index == index &&
               iface.addr.sa.sa_family == 2
               && iface.addr.in_0.sin_addr.s_addr == addr.addr4.s_addr {
            return 1
        }
        iface = iface.next
    }
    return 0;
}
fn iface_allowed(mut param: &mut IfaceParam,
                                   mut if_index: i32,
                                   mut label: &mut String,
                                   mut addr: NetAddress,
                                   mut netmask: NetAddress,
                                   mut prefixlen: usize,
                                   mut iface_flags: i32)
                                   -> i32 {
    let mut iface: Irec = Default::default ;
    let mut mtu: u16 = 0;
    let mut loopback: i32 = 0;
    let mut ifr: IfReq = Default::default();
    let mut tftp_ok: bool = (daemon.options[40] != 0);
    let mut dhcp_ok: bool = true;
    let mut auth_dns: bool = false;
    let mut is_label: bool = false;
    let mut tmp: Iname = Default::default();
    if indextoname(param.fd, if_index, ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) == 0 ||
           ioctl(param.fd, 0x8913, &mut ifr) == -(1) {
        return 0
    }
    loopback = ifr.ifr_ifru.ifru_flags & IFF_LOOPBACK;
    if loopback != 0 { dhcp_ok = 0 }
    if ioctl(param.fd, 0x8921,
             &mut ifr) != -(1) {
        mtu = ifr.ifr_ifru.ifru_mtu
    }
    if label.is_null() {
        label = ifr.ifr_ifrn.ifrn_name.as_mut_ptr()
    } else { is_label = strcmp(label, ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) }
    /* maintain a list of all addresses on all interfaces for --local-service option */
    if daemon.options[(49).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (49).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 {
        let mut al: AddressListEntry = Default::default() ;
        if !param.spare.is_null() {
            al = param.spare;
            param.spare = al.next
        } else {
            // al =
            //     whine_malloc(::std::mem::size_of::<AddressListEntry>()))
        }
        if !al.is_null() {
            al.next = daemon.interface_addrs;
            daemon.interface_addrs = al;
            al.prefixlen = prefixlen;
            if addr.sa.sa_family == 2 {
                al.addr.addr4 = addr.in_0.sin_addr;
                al.flags = 0
            } else {
                al.addr.addr6 = addr.in6.sin6_addr;
                al.flags = 2
            }
        }
    }
    if addr.sa.sa_family != 10 ||
           ({
                let mut __a: *const In6Addr =
                    &mut addr.in6.sin6_addr ;
                (__a.__in6_u.__u6_addr32[0 ] &
                     __bswap_32(0xffc00000) ==
                     __bswap_32(0xfe800000))
            }) == 0 {
        let mut int_name: InterfaceName = ;
        let mut al_0: AddressListEntry = 0 ;
        let mut zone: AuthZone = 0 ;
        let mut name: AuthNameList = 0 ;
        /* Find subnets in auth_zones */
        zone = daemon.auth_zones;
        while !zone.is_null() {
            name = zone.interface_names;
            while !name.is_null() {
                if wildcard_match(name.name, label) != 0 {
                    if addr.sa.sa_family == 2
                           && name.flags & 2 != 0 {
                        if !param.spare.is_null() {
                            al_0 = param.spare;
                            param.spare = al_0.next
                        } else {
                            // al_0 =
                            //     whine_malloc(::std::mem::size_of::<AddressListEntry>()
                            //                     )
                        }
                        if !al_0.is_null() {
                            al_0.next = zone.subnet;
                            zone.subnet = al_0;
                            al_0.prefixlen = prefixlen;
                            al_0.addr.addr4 = addr.in_0.sin_addr;
                            al_0.flags = 0
                        }
                    }
                    if addr.sa.sa_family ==
                           10 &&
                           name.flags & 1 != 0 {
                        if !param.spare.is_null() {
                            al_0 = param.spare;
                            param.spare = al_0.next
                        } else {
                            // al_0 =
                            //     whine_malloc(::std::mem::size_of::<AddressListEntry>()
                            //                     )
                        }
                        if !al_0.is_null() {
                            al_0.next = zone.subnet;
                            zone.subnet = al_0;
                            al_0.prefixlen = prefixlen;
                            al_0.addr.addr6 = addr.in6.sin6_addr;
                            al_0.flags = 2
                        }
                    }
                }
                name = name.next
            }
            zone = zone.next
        }
        /* Update addresses from interface_names. These are a set independent
	 of the set we're listening on. */
        int_name = daemon.int_names;
        while !int_name.is_null() {
            if strncmp(label, int_name.intr,
                       16) == 0
                   &&
                   (addr.sa.sa_family == int_name.family
                        || int_name.family == 0) {
                if !param.spare.is_null() {
                    al_0 = param.spare;
                    param.spare = al_0.next
                } else {
                    // al_0 =
                    //     whine_malloc(::std::mem::size_of::<AddressListEntry>())
                }
                if !al_0.is_null() {
                    al_0.next = int_name.addresses;
                    int_name.addresses = al_0;
                    if addr.sa.sa_family == 2
                       {
                        al_0.addr.addr4 = addr.in_0.sin_addr;
                        al_0.flags = 0
                    } else {
                        al_0.addr.addr6 = addr.in6.sin6_addr;
                        al_0.flags = 2;
                        /* Privacy addresses and addresses still undergoing DAD and deprecated addresses
		       don't appear in forward queries, but will in reverse ones. */
                        if iface_flags & 4 == 0 ||
                               iface_flags &
                                   (2 | 1) != 0
                           {
                            al_0.flags |= 4
                        }
                    }
                }
            }
            int_name = int_name.next
        }
    }
    /* check whether the interface IP has been added already 
     we call this routine multiple times. */
    iface = daemon.interfaces; /* for garbage collection */
    while !iface.is_null() {
        if NetAddress_isequal(&mut iface.addr, addr) != 0 &&
               iface.index == if_index {
            iface.dad =
                (iface_flags & 1 != 0);
            iface.found = 1;
            iface.netmask = netmask;
            return 1
        }
        iface = iface.next
    }
    /* If we are restricting the set of interfaces to use, make
     sure that loopback interfaces are in that set. */
    if !daemon.if_names.is_null() && loopback != 0 {
        let mut lo: Iname = 0;
        lo = daemon.if_names;
        while !lo.is_null() {
            if !lo.name.is_null() &&
                   strcmp(lo.name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) ==
                       0 {
                break ;
            }
            lo = lo.next
        }
        if lo.is_null() &&
               {
                   // lo =
                   //     whine_malloc(::std::mem::size_of::<Iname>() );
                   // !lo.is_null()
                   true
               } {
            // lo.name =
            //     whine_malloc(strlen(ifr.ifr_ifrn.ifrn_name.as_mut_ptr()).wrapping_add(1   libc::c_int
            //                                                                                                              ))
            //         ;
            if !lo.name.is_null() {
                strcpy(lo.name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
                lo.used = 1;
                lo.next = daemon.if_names;
                daemon.if_names = lo
            } else {
                // free(lo);
            }
        }
    }
    if addr.sa.sa_family == 2 &&
           iface_check(2,
                       &mut addr.in_0.sin_addr                 , label, &mut auth_dns) == 0 {
        return 1
    }
    if addr.sa.sa_family == 10 &&
           iface_check(10,
                       &mut addr.in6.sin6_addr                 , label, &mut auth_dns) == 0 {
        return 1
    }
    /* No DHCP where we're doing auth DNS. */
    if auth_dns != 0 {
        tftp_ok = 0;
        dhcp_ok = 0
    } else {
        tmp = daemon.dhcp_except;
        while !tmp.is_null() {
            if !tmp.name.is_null() &&
                   wildcard_match(tmp.name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                tftp_ok = 0;
                dhcp_ok = 0
            }
            tmp = tmp.next
        }
    }
    if !daemon.tftp_interfaces.is_null() {
        /* dedicated tftp interface list */
        tftp_ok = 0;
        tmp = daemon.tftp_interfaces;
        while !tmp.is_null() {
            if !tmp.name.is_null() &&
                   wildcard_match(tmp.name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                tftp_ok = 1
            }
            tmp = tmp.next
        }
    }
    /* add to list */
    // iface =
    //     whine_malloc(::std::mem::size_of::<Irec>())      Irec; /* dummy */
    if !iface.is_null() {
        iface.addr = *addr;
        iface.netmask = netmask;
        iface.tftp_ok = tftp_ok;
        iface.dhcp_ok = dhcp_ok;
        iface.dns_auth = auth_dns;
        iface.mtu = mtu;
        iface.dad = (iface_flags & 1 != 0);
        iface.found = 1;
        iface.warned = 0;
        iface.multicast_done = iface.warned;
        iface.done = iface.multicast_done;
        iface.index = if_index;
        iface.label = is_label;
        // iface.name =
        //     whine_malloc(strlen(ifr.ifr_ifrn.ifrn_name.as_mut_ptr()).wrapping_add(1))
        //         ;
        if !iface.name.is_null() {
            strcpy(iface.name, ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
            iface.next = daemon.interfaces;
            daemon.interfaces = iface;
            return 1
        }
        // free(iface);
    }
    *__errno_location() = 12;
    return 0;
}
fn iface_allowed_v6(
    daemon: &mut DnsmasqDaemon,
    mut local: NetAddress,
                                      mut prefix: i32,
                                      mut scope: i32,
                                      mut if_index: i32,
                                      mut flags: i32,
                                      mut preferred: i32,
                                      mut valid: i32,
                                      mut vparam:Vec<u8>)
                                      -> i32 {
    let mut addr: NetAddress = Default::default();
    addr._type = AddressType::Ipv6Address;
    addr.value.copy_from_slice(&local.value);
    
    let mut netmask: NetAddress = Default::default();
    /* warning */
    addr.in6.sin6_family = 10;
    addr.in6.sin6_addr = *local;
    addr.in6.sin6_port = daemon.port.to_be();
    /* FreeBSD insists this is zero for non-linklocal addresses */
    if {
        let mut __a: NetAddress = NetAddress{_type: AddressType::Ipv6Address, value: local.value};
        let mut a_u32: u32 = u32::from_le_bytes(__a.value[0..4]);
        a_u32 = a_u32 & 0xffc00000;
        a_u32 & 0xfe800000} != 0 {
        addr.in6.sin6_scope_id = if_index:
    } else { addr.in6.sin6_scope_id = 0: }
    return iface_allowed(vparam, if_index,
                         0 , &mut addr, netmask, prefix,
                         flags);
}
fn iface_allowed_v4(mut local: NetAddress,
                                      mut if_index: i32,
                                      mut label: &mut String,
                                      mut netmask: NetAddress,
                                      mut broadcast: NetAddress,
                                      mut vparam:Vec<u8>)
                                      -> i32 {
    let mut addr: NetAddress =
        NetAddress {sa: NetAddress {sa_family: 0, sa_data: [0; 14],},};
    let mut prefix: i32 = 0;
    let mut bit: i32 = 0;
    /* warning */
    addr.in_0.sin_family = 2;
    addr.in_0.sin_addr = local;
    addr.in_0.sin_port = __bswap_16(daemon.port );
    /* determine prefix length from netmask */
    prefix = 32;
    bit = 1;
    while bit & __bswap_32(netmask.s_addr) ==
              0 && prefix != 0 {
        bit = bit << 1;
        prefix -= 1
    }
    return iface_allowed(vparam, if_index, label,
                         &mut addr, netmask, prefix, 0);
}
/*
 * Clean old interfaces no longer found.
 */
fn clean_interfaces() {
    let mut iface: Irec = 0 ;
    let mut up: &mut Irec = &mut daemon.interfaces;
    iface = *up;
    while !iface.is_null() {
        if iface.found == 0 && iface.done == 0 {
            *up = iface.next;
            // free(iface.name);
            // free(iface);
        } else { up = &mut iface.next }
        iface = *up
    };
}
/* * Release listener if no other interface needs it.
 *
 * @return 1 if released, 0 if still required
 */
fn release_listener(mut l: Listener) -> i32 {
    if l.used > 1 {
        let mut iface: Irec = 0 ;
        iface = daemon.interfaces;
        while !iface.is_null() {
            if iface.done != 0 &&
                   NetAddress_isequal(&mut l.addr, &mut iface.addr) != 0 {
                if iface.found != 0 {
                    /* update listener to point to active interface instead */
                    if (*l.iface).found == 0 { l.iface = iface }
                } else { l.used -= 1; iface.done = 0 }
            }
            iface = iface.next
        }
        /* Someone is still using this listener, skip its deletion */
        if l.used > 0 { return 0 }
    }
    if (*l.iface).done != 0 {
        let mut port: i32 = 0;
        port =
            prettyprint_addr(&mut (*l.iface).addr,
                             daemon.addrbuff);
        my_syslog(7,
                  "stopped listening on %s(#%d): %s port %d"
                     , (*l.iface).name,
                  (*l.iface).index, daemon.addrbuff, port);
        /* In case it ever returns */
        (*l.iface).done = 0
    }
    if l.fd != -(1) { close(l.fd); }
    if l.tcpfd != -(1) { close(l.tcpfd); }
    if l.tftpfd != -(1) { close(l.tftpfd); }
    // free(l);
    return 1;
}

pub fn enumerate_interfaces(mut reset: i32)
 -> i32 {
    static mut spare: AddressListEntry = 0  ;
    static mut done: i32 = 0;
    let mut param: IfaceParam =
        IfaceParam {spare: 0 , fd: 0,};
    let mut errsave: i32 = 0;
    let mut ret: i32 = 1;
    let mut addr: AddressListEntry = 0 ;
    let mut tmp: AddressListEntry = 0 ;
    let mut intname: InterfaceName = ;
    let mut iface: Irec = 0 ;
    let mut zone: AuthZone = 0 ;
    /* Do this max once per select cycle  - also inhibits netlink socket use
   in TCP child processes. */
    if reset != 0 { done = 0; return 1 }
    if done != 0 { return 1 }
    done = 1;
    param.fd =
        socket(2, SOCK_DGRAM, 0);
    if param.fd == -(1) { return 0 }
    /* Mark interfaces for garbage collection */
    iface = daemon.interfaces;
    while !iface.is_null() {
        iface.found = 0;
        iface = iface.next
    }
    /* remove addresses stored against interface_names */
    intname = daemon.int_names;
    while !intname.is_null() {
        addr = intname.addresses;
        while !addr.is_null() {
            tmp = addr.next;
            addr.next = spare;
            spare = addr;
            addr = tmp
        }
        intname.addresses = 0 ;
        intname = intname.next
    }
    /* Remove list of addresses of local interfaces */
    addr = daemon.interface_addrs;
    while !addr.is_null() {
        tmp = addr.next;
        addr.next = spare;
        spare = addr;
        addr = tmp
    }
    daemon.interface_addrs = 0 ;
    /* remove addresses stored against auth_zone subnets, but not 
   ones configured as address literals */
    zone = daemon.auth_zones;
    while !zone.is_null() {
        if !zone.interface_names.is_null() {
            let mut up: AddressListEntry;
            up = &mut zone.subnet;
            addr = zone.subnet;
            while !addr.is_null() {
                tmp = addr.next;
                if addr.flags & 1 != 0 {
                    up = &mut addr.next
                } else {
                    *up = addr.next;
                    addr.next = spare;
                    spare = addr
                }
                addr = tmp
            }
        }
        zone = zone.next
    }
    param.spare = spare;
    ret =
        iface_enumerate(10,
                        &mut param,
                        ::std::mem::transmute::<Option<fn(_:
                                                                                &mut In6Addr,
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
                                                                               Vec<u8>)
                                                                            -> i32>,
                                                Option<fn()
                                                           ->
                                                               libc::c_int>>(Some(iface_allowed_v6
                                                                                                                        fn(_:
                                                                                                               &mut In6Addr,
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
                                                                                                              Vec<u8>)
                                                                                                           ->
                                                                                              libc::c_int)));
    if ret != 0 {
        ret =
            iface_enumerate(2,
                            &mut param                         Vec<u8>,
                            ::std::mem::transmute::<Option<fn(_:
                                                                                NetAddress,
                                                                                _:
                                                                                    libc::c_int,
                                                                                _:
                                                                                    &mut String,
                                                                                _:
                                                                                NetAddress,
                                                                                _:
                                                                                NetAddress,
                                                                                _:
                                                                                   Vec<u8>)
                                                                                ->
                                                                   libc::c_int>,
                                                    Option<fn()
                                                               ->
                                                                   libc::c_int>>(Some(iface_allowed_v4   fn(_:
                                                                                                               NetAddress,
                                                                                                               _:
                                                                                                                   libc::c_int,
                                                                                                               _:
                                                                                                                   &mut String,
                                                                                                               _:
                                                                                                               NetAddress,
                                                                                                               _:
                                                                                                               NetAddress,
                                                                                                               _:
                                                                                                                  Vec<u8>)
                                                                                                               ->
                                                                                                  libc::c_int)))
    }
    errsave = *__errno_location();
    close(param.fd);
    if daemon.options[(39).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (39).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 {
        /* Garbage-collect listeners listening on addresses that no longer exist.
	 Does nothing when not binding interfaces or for listeners on localhost, 
	 since the ->iface field is NULL. Note that this needs the protections
	 against reentrancy, hence it's here.  It also means there's a possibility,
	 in OPT_CLEVERBIND mode, that at listener will just disappear after
	 a call to enumerate_interfaces, this is checked OK on all calls. */
        let mut l: Listener = 0 ;
        let mut tmp_0: Listener = 0 ;
        let mut up_0: Listener = 0 ;
        let mut freed: i32 = 0;
        up_0 = &mut daemon.listeners;
        l = daemon.listeners;
        while !l.is_null() {
            tmp_0 = l.next;
            if l.iface.is_null() || (*l.iface).found != 0 {
                up_0 = &mut l.next
            } else if release_listener(l) != 0 {
                *up_0 = tmp_0;
                freed = 1
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
pub fn fix_fd(mut fd: &mut ) -> i32 {
    let mut flags: i32 = 0;
    flags = fcntl(fd, 3);
    if flags == -(1) ||
           fcntl(fd, 4, flags | 0o4000) ==
               -(1) {
        return 0
    }
    return 1;
}
fn make_sock(mut addr: NetAddress,
                               mut type_0: i32,
                               mut dienow: i32) -> i32 {
    let mut port: i32 = 0;
    let mut errsave: i32 = 0;
    let mut s: &mut String = 0 ;
    let mut current_block: u64;
    let mut family: i32 = addr.sa.sa_family;
    let mut fd: i32 = 0;
    let mut rc: i32 = 0;
    let mut opt: i32 = 1;
    fd = socket(family, type_0, 0);
    if fd == -(1) {
        port = 0;
        errsave = 0;
        s = 0 ;
        /* No error if the kernel just doesn't support this IP flavour */
        if *__errno_location() == 93 ||
               *__errno_location() == 97 ||
               *__errno_location() == 22 {
            return -(1)
        }
    } else if !(setsockopt(fd, 1, 2,
                           &mut opt as
                           ::std::mem::size_of::<libc::c_int>()) ==
                    -(1) || fix_fd(fd) == 0) {
        if !(family == 10 &&
                 setsockopt(fd, IPPROTO_IPV6,
                            26,
                            &mut opt as
                            ::std::mem::size_of::<libc::c_int>()                   ) ==
                     -(1)) {
            rc =
                bind(fd,
                     ConstNetAddressArg {__NetAddress__:
                                              addr,},
                     sa_len(addr));
            if !(rc == -(1)) {
                if type_0 == SOCK_STREAM {
                    let mut qlen: i32 = 5;
                    setsockopt(fd, IPPROTO_TCP,
                               23,
                               &mut qlen as
                               ::std::mem::size_of::<libc::c_int>());
                    if listen(fd, 32) == -(1) {
                        current_block = 4055993212646746884;
                    } else { current_block = 11459959175219260272; }
                } else if family == 2 {
                    if daemon.options[(13   libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                   ).wrapping_mul(8                                                             libc::c_int                                                      ))
                                                     ] &
                           (1) <<
                               (13                       ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                               ).wrapping_mul(8                         libc::c_int                  ))
                           == 0 {
                        if setsockopt(fd, IPPROTO_IP,
                                      8,
                                      &mut opt as
                                      ::std::mem::size_of::<libc::c_int>() ) ==
                               -(1) {
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
    port = prettyprint_addr(addr, daemon.addrbuff);
    if daemon.options[(13).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (13).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           == 0 &&
           daemon.options[(39 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (39 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               == 0 {
        sprintf(daemon.addrbuff,
                "port %d" , port);
    }
    s =
        "failed to create listening socket for %s: %s"       *const libc::c_char ;
    if fd != -(1) { close(fd); }
    *__errno_location() = errsave;
    if dienow != 0 {
        /* failure to bind addresses given by --listen-address at this point
	     is OK if we're doing bind-dynamic */
        if daemon.options[(39 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (39 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               == 0 {
            die(s, daemon.addrbuff, 2);
        }
    } else {
        my_syslog(4, s, daemon.addrbuff,
                  strerror(*__errno_location()));
    }
    return -(1);
}

pub fn set_ipv6pktinfo(mut fd: i32) -> i32 {
    let mut opt: i32 = 1;
    /* The API changed around Linux 2.6.14 but the old ABI is still supported:
     handle all combinations of headers and kernel.
     OpenWrt note that this fixes the problem addressed by your very broken patch. */
    daemon.v6pktinfo = 50;
    if setsockopt(fd, IPPROTO_IPV6, 49,
                  &mut opt,
                  ::std::mem::size_of::<libc::c_int>() )) != -(1) {
        return 1
    } else {
        if *__errno_location() == 92 &&
               setsockopt(fd, IPPROTO_IPV6, 2,
                          &mut opt,
                          ::std::mem::size_of::<libc::c_int>()) !=
                   -(1) {
            daemon.v6pktinfo = 2;
            return 1
        }
    }
    return 0;
}
/* Find the interface on which a TCP connection arrived, if possible, or zero otherwise. */

pub fn tcp_interface(mut fd: i32,
                                       mut af: i32) -> i32 {
    /* suppress potential unused warning */
    let mut if_index: i32 = 0;
    let mut opt: i32 = 1;
    let mut cmptr: CmsgHdr = 0;
    let mut msg: MsgHdr =
        MsgHdr {msg_name: 0,
               msg_namelen: 0,
               msg_iov: 0,
               msg_iovlen: 0,
               msg_control: 0,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut len: socklen_t = 0;
    /* use mshdr so that the CMSDG_* macros are available */
    msg.msg_control = daemon.packet;
    len = daemon.packet_buff_sz;
    msg.msg_controllen = len ;
    /* we overwrote the buffer... */
    daemon.srv_save = 0;
    if af == 2 {
        if setsockopt(fd, IPPROTO_IP, 8,
                      &mut opt,
                      ::std::mem::size_of::<libc::c_int>()) != -(1) &&
               getsockopt(fd, IPPROTO_IP, 9,
                          msg.msg_control, &mut len) != -(1) {
            msg.msg_controllen = len ;
            cmptr =
                if msg.msg_controllen >=
                       ::std::mem::size_of::<CmsgHdr>() {
                    msg.msg_control
                } else { 0 };
            while !cmptr.is_null() {
                if cmptr.cmsg_level == IPPROTO_IP &&
                       cmptr.cmsg_type == 8 {
                    let mut p: C2RustUnnamed_13 =
                        C2RustUnnamed_13{c: 0,};
                    p.c = cmptr.__cmsg_data.as_mut_ptr();
                    if_index = (*p.p).ipi_ifindex
                }
                cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            }
        }
    } else if set_ipv6pktinfo(fd) != 0 &&
                  getsockopt(fd, IPPROTO_IPV6,
                             6, msg.msg_control, &mut len) !=
                      -(1) {
        msg.msg_controllen = len ;
        cmptr =
            if msg.msg_controllen >=
                   ::std::mem::size_of::<CmsgHdr>() {
                msg.msg_control
            } else { 0 };
        while !cmptr.is_null() {
            if cmptr.cmsg_level == IPPROTO_IPV6 &&
                   cmptr.cmsg_type == daemon.v6pktinfo {
                let mut p_0: C2rustUnnamed12 =
                    C2rustUnnamed12 {c: 0,};
                p_0.c = cmptr.__cmsg_data.as_mut_ptr();
                if_index = (*p_0.p).ipi6_ifindex
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
fn create_listeners(mut addr: NetAddress,
                                      mut do_tftp: i32,
                                      mut dienow: i32)
                                      -> Listener {
    let mut l: Listener = 0 ;
    let mut fd: i32 = -(1);
    let mut tcpfd: i32 = -(1);
    let mut tftpfd: i32 = -(1);
    if daemon.port != 0 {
        fd = make_sock(addr, SOCK_DGRAM, dienow);
        tcpfd = make_sock(addr, SOCK_STREAM, dienow)
    }
    if do_tftp != 0 {
        if addr.sa.sa_family == 2 {
            /* port must be restored to DNS port for TCP code */
            let mut save: libc::c_short =
                addr.in_0.sin_port ;
            addr.in_0.sin_port =
                __bswap_16(69 );
            tftpfd = make_sock(addr, SOCK_DGRAM, dienow);
            addr.in_0.sin_port = save as in_port_t
        } else {
            let mut save_0: libc::c_short =
                addr.in6.sin6_port ;
            addr.in6.sin6_port =
                __bswap_16(69 );
            tftpfd = make_sock(addr, SOCK_DGRAM, dienow);
            addr.in6.sin6_port = save_0 as in_port_t
        }
    }
    if fd != -(1) || tcpfd != -(1) ||
           tftpfd != -(1) {
        // l =
        //     safe_malloc(::std::mem::size_of::<Listener>())          Listener;
        l.next = 0 ;
        l.fd = fd;
        l.tcpfd = tcpfd;
        l.tftpfd = tftpfd;
        l.addr = *addr;
        l.used = 1;
        l.iface = 0
    }
    return l;
}

pub fn create_wildcard_listeners(daemon: &mut DnsmasqDaemon) {
    let mut addr: NetAddress = Default::default();
    let mut l: Listener = 0 ;
    let mut l6: Listener = 0 ;
    addr._type = AddressType::Ipv4Address;
    addr.port = daemon.port;
    l = create_listeners(&mut addr, daemon.options[40], 1);
    addr = Default::default();
    addr._type = AddressType::Ipv6Address;
    addr.value = in6addr_any;
    addr.port = daemon.port;
    l6 = create_listeners(&mut addr, daemon.options[40], 1);
    if !l.is_null() { l.next = l6 } else { l = l6 }
    daemon.listeners = l;
}

fn find_listener(daemon: &mut DnsmasqDaemon, mut addr: &mut NetAddress) -> Option<Listener> {
    for listener in daemon.listeners {
        if netaddr_isequal(&listener.addr, addr) {
            Some(listener.clone)
        }
    }
    None
}

pub fn create_bound_listeners(mut dienow: i32) {
    let mut new: Listener = 0 ;
    let mut iface: Irec = 0 ;
    let mut if_tmp: Iname = 0;
    let mut existing: Listener = 0 ;
    iface = daemon.interfaces;
    while !iface.is_null() {
        if iface.done == 0 && iface.dad == 0 && iface.found != 0 {
            existing = find_listener(&mut iface.addr);
            if !existing.is_null() {
                iface.done = 1;
                existing.used += 1
                /* increase usage counter */
            } else {
                new =
                    create_listeners(&mut iface.addr, iface.tftp_ok,
                                     dienow);
                if !new.is_null() {
                    new.iface = iface;
                    new.next = daemon.listeners;
                    daemon.listeners = new;
                    iface.done = 1;
                    /* Don't log the initial set of listen addresses created
               at startup, since this is happening before the logging
               system is initialised and the sign-on printed. */
                    if dienow == 0 {
                        let mut port: i32 =
                            prettyprint_addr(&mut iface.addr,
                                             daemon.addrbuff);
                        my_syslog(7,
                                  "listening on %s(#%d): %s port %d"                                *const u8,
                                  iface.name, iface.index,
                                  daemon.addrbuff, port);
                    }
                }
            }
        }
        iface = iface.next
    }
    /* Check for --listen-address options that haven't been used because there's
     no interface with a matching address. These may be valid: eg it's possible
     to listen on 127.0.1.1 even if the loopback interface is 127.0.0.1

     If the address isn't valid the bind() will fail and we'll die() 
     (except in bind-dynamic mode, when we'll complain but keep trying.)

     The resulting listeners have the ->iface field NULL, and this has to be
     handled by the DNS and TFTP code. It disables --localise-queries processing
     (no netmask) and some MTU login the tftp code. */
    if_tmp = daemon.if_addrs;
    while !if_tmp.is_null() {
        if if_tmp.used == 0 &&
               {
                   new =
                       create_listeners(&mut if_tmp.addr,
                                        (daemon.options[(40                     libc::c_int
                                                                                     ).wrapping_div((::std::mem::size_of::<libc::c_uint>()                          ).wrapping_mul(8                                                                                                 libc::c_int                                                                                          ))
                                                                                          usize]
                                             &
                                             (1) <<
                                                 (40   libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                                   ).wrapping_mul(8                                                             libc::c_int                                                      ))
                                             != 0), dienow);
                   !new.is_null()
               } {
            new.next = daemon.listeners;
            daemon.listeners = new;
            if dienow == 0 {
                let mut port_0: i32 =
                    prettyprint_addr(&mut if_tmp.addr,
                                     daemon.addrbuff);
                my_syslog(7,
                          "listening on %s port %d" , daemon.addrbuff,
                          port_0);
            }
        }
        if_tmp = if_tmp.next
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

pub fn warn_bound_listeners() {
    let mut iface: Irec = 0 ;
    let mut advice: i32 = 0;
    iface = daemon.interfaces;
    while !iface.is_null() {
        if iface.dns_auth == 0 {
            if iface.addr.sa.sa_family == 2 {
                if private_net(iface.addr.in_0.sin_addr, 1)
                       == 0 {
                    inet_ntop(2,
                              &mut iface.addr.in_0.sin_addr
                                 ,
                              daemon.addrbuff,
                              46);
                    advice = 1;
                    iface.warned = advice;
                    my_syslog(4,
                              "LOUD WARNING: listening on %s may accept requests via interfaces other than %s"
                                  ,
                              daemon.addrbuff, iface.name);
                }
            }
        }
        iface = iface.next
    }
    if advice != 0 {
        my_syslog(4,
                  "LOUD WARNING: use --bind-dynamic rather than --bind-interfaces to avoid DNS amplification attacks via these interface(s)"
                      );
    };
}

pub fn warn_wild_labels() {
    let mut iface: Irec = 0 ;
    iface = daemon.interfaces;
    while !iface.is_null() {
        if iface.found != 0 && !iface.name.is_null() &&
               iface.label != 0 {
            my_syslog(4,
                      "warning: using interface %s instead"
                         , iface.name);
        }
        iface = iface.next
    };
}

pub fn warn_int_names() {
    let mut intname: InterfaceName = ;
    intname = daemon.int_names;
    while !intname.is_null() {
        if intname.addresses.is_null() {
            my_syslog(4,
                      "warning: no addresses found for interface %s"                    *const u8, intname.intr);
        }
        intname = intname.next
    };
}

pub fn is_dad_listeners() -> i32 {
    let mut iface: Irec = 0 ;
    if daemon.options[(13).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (13).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 {
        iface = daemon.interfaces;
        while !iface.is_null() {
            if iface.dad != 0 && iface.done == 0 {
                return 1
            }
            iface = iface.next
        }
    }
    return 0;
}

pub fn join_multicast(mut dienow: i32) {
    let mut iface: Irec = 0 ;
    let mut tmp: Irec = 0 ;
    iface = daemon.interfaces;
    while !iface.is_null() {
        if iface.addr.sa.sa_family == 10 &&
               iface.dhcp_ok != 0 && iface.multicast_done == 0 {
            /* There's an irec per address but we only want to join for multicast 
	   once per interface. Weed out duplicates. */
            tmp = daemon.interfaces;
            while !tmp.is_null() {
                if tmp.multicast_done != 0 &&
                       tmp.index == iface.index {
                    break ;
                }
                tmp = tmp.next
            }
            iface.multicast_done = 1;
            if tmp.is_null() {
                let mut mreq: ipv6_mreq =
                    ipv6_mreq{ipv6mr_multiaddr:
                                  In6Addr {__in6_u:
                                               C2RustUnnamed{__u6_addr8:
                                                                 [0; 16],},},
                              ipv6mr_interface: 0,};
                let mut err: i32 = 0;
                mreq.ipv6mr_interface = iface.index;
                inet_pton(10,
                          "FF02::1:2" ,
                          &mut mreq.ipv6mr_multiaddr );
                if (daemon.doing_dhcp6 != 0 ||
                        !daemon.relay6.is_null()) &&
                       setsockopt(daemon.dhcp6fd,
                                  IPPROTO_IPV6,
                                  20,
                                  &mut mreq ,
                                  ::std::mem::size_of::<ipv6_mreq>()) ==
                           -(1) {
                    err = *__errno_location()
                }
                inet_pton(10,
                          "FF05::1:3" ,
                          &mut mreq.ipv6mr_multiaddr );
                if daemon.doing_dhcp6 != 0 &&
                       setsockopt(daemon.dhcp6fd,
                                  IPPROTO_IPV6,
                                  20,
                                  &mut mreq ,
                                  ::std::mem::size_of::<ipv6_mreq>()) ==
                           -(1) {
                    err = *__errno_location()
                }
                inet_pton(10,
                          "FF02::2" ,
                          &mut mreq.ipv6mr_multiaddr );
                if daemon.doing_ra != 0 &&
                       setsockopt(daemon.icmp6fd,
                                  IPPROTO_IPV6,
                                  20,
                                  &mut mreq ,
                                  ::std::mem::size_of::<ipv6_mreq>()) ==
                           -(1) {
                    err = *__errno_location()
                }
                if err != 0 {
                    let mut s: &mut String =
                        "interface %s failed to join DHCPv6 multicast group: %s"
                           ;
                    *__errno_location() = err;
                    if *__errno_location() == 12 {
                        my_syslog(3,
                                  "try increasing /proc/sys/net/core/optmem_max"
                                      );
                    }
                    if dienow != 0 {
                        die(s, iface.name, 2);
                    } else {
                        my_syslog(3, s, iface.name,
                                  strerror(*__errno_location()));
                    }
                }
            }
        }
        iface = iface.next
    };
}
/* return a UDP socket bound to a random port, have to cope with straying into
   occupied port nos and reserved ones. */

pub fn random_sock(mut family: i32) -> i32 {
    let mut fd: i32 = 0;
    fd = socket(family, SOCK_DGRAM, 0);
    if fd != -(1) {
        let mut addr: NetAddress =
            NetAddress {sa: NetAddress {sa_family: 0, sa_data: [0; 14],},};
        let mut ports_avail: u32 =
            (daemon.max_port  -
                 daemon.min_port  +
                 1);
        let mut tries: i32 =
            if ports_avail < 30 {
                (3).wrapping_mul(ports_avail)
            } else { 100 };
        memset(&mut addr ,
               0,
               ::std::mem::size_of::<NetAddress>());
        addr.sa.sa_family = family;
        /* don't loop forever if all ports in use. */
        if fix_fd(fd) != 0 {
            loop  {
                let fresh6 = tries;
                tries = tries - 1;
                if !(fresh6 != 0) { break ; }
                let mut port: u16 =
                    __bswap_16((daemon.min_port +
                                    rand16() %
                                        ports_avail  ) );
                if family == 2 {
                    addr.in_0.sin_addr.s_addr = 0;
                    addr.in_0.sin_port = port
                } else {
                    addr.in6.sin6_addr = in6addr_any;
                    addr.in6.sin6_port = port
                }
                if bind(fd,
                        ConstNetAddressArg {__NetAddress__:
                                                 &mut addr
                                                    ,},
                        sa_len(&mut addr)) == 0 {
                    return fd
                }
                if *__errno_location() != 98 &&
                       *__errno_location() != 13 {
                    break ;
                }
            }
        }
        close(fd);
    }
    return -(1);
}

pub fn local_bind(mut fd: i32,
                                    mut addr: NetAddress,
                                    mut intname: &mut String,
                                    mut ifindex: u32,
                                    mut is_tcp: i32) -> i32 {
    let mut addr_copy: NetAddress = *addr;
    let mut port: u16 = 0;
    let mut tries: i32 = 1;
    let mut done: i32 = 0;
    let mut ports_avail: u32 =
        (daemon.max_port  -
             daemon.min_port  +
             1);
    if addr_copy.sa.sa_family == 2 {
        port = addr_copy.in_0.sin_port
    } else { port = addr_copy.in6.sin6_port }
    /* cannot set source _port_ for TCP connections. */
    if is_tcp != 0 { port = 0  }
    /* Bind a random port within the range given by min-port and max-port */
    if port == 0 {
        tries =
            if ports_avail < 30 {
                (3).wrapping_mul(ports_avail)
            } else { 100 };
        port =
            __bswap_16((daemon.min_port +
                            rand16() %
                                ports_avail )
                           )
    }
    loop  {
        let fresh7 = tries;
        tries = tries - 1;
        if !(fresh7 != 0) { break ; }
        if addr_copy.sa.sa_family == 2 {
            addr_copy.in_0.sin_port = port
        } else { addr_copy.in6.sin6_port = port }
        if bind(fd,
                ConstNetAddressArg {__NetAddress__:
                                         &mut addr_copy                                        NetAddress,},
                sa_len(&mut addr_copy)) != -(1) {
            done = 1;
            break ;
        } else {
            if *__errno_location() != 98 &&
                   *__errno_location() != 13 {
                return 0
            }
            port =
                __bswap_16((daemon.min_port +
                                rand16() %
                                    ports_avail  ) )
        }
    }
    if done == 0 { return 0 }
    if is_tcp == 0 && ifindex > 0 {
        if addr_copy.sa.sa_family == 2 {
            let mut ifindex_opt: uint32_t = __bswap_32(ifindex);
            return (setsockopt(fd, IPPROTO_IP,
                               50,
                               &mut ifindex_opt ,
                               ::std::mem::size_of::<uint32_t>()) ==
                        0)
        }
        if addr_copy.sa.sa_family == 10 {
            let mut ifindex_opt_0: uint32_t = __bswap_32(ifindex);
            return (setsockopt(fd, IPPROTO_IPV6,
                               76,
                               &mut ifindex_opt_0 ,
                               ::std::mem::size_of::<uint32_t>()) ==
                        0)
        }
    }
    /* suppress potential unused warning */
    if *intname.offset(0) !=
           0 &&
           setsockopt(fd, 1, 25,
                      intname,
                      16) == -(1) {
        return 0
    }
    return 1;
}
fn allocate_sfd(mut addr: NetAddress,
                                  mut intname: &mut String)
                                  ->ServerFd {
    let mut sfd:ServerFd = 0Fd;
    let mut ifindex: u32 = 0;
    let mut errsave: i32 = 0;
    let mut opt: i32 = 1;
    /* when using random ports, servers which would otherwise use
     the INADDR_ANY/port0 socket have sfd set to NULL */
    if daemon.osport == 0 &&
           *intname.offset(0) ==
               0 {
        *__errno_location() =
            0 ; /* index == 0 when not binding to an interface */
        if addr.sa.sa_family == 2 &&
               addr.in_0.sin_addr.s_addr == 0
               &&
               addr.in_0.sin_port ==
                   __bswap_16(0 ) {
            return 0Fd
        }
        if addr.sa.sa_family == 10 &&
               memcmp(&mut addr.in6.sin6_addr ,
                      &in6addr_any ,
                      ::std::mem::size_of::<In6Addr>()) ==
                   0 &&
               addr.in6.sin6_port ==
                   __bswap_16(0 ) {
            return 0Fd
        }
    }
    if !intname.is_null() &&
           strlen(intname) != 0 {
        ifindex = if_nametoindex(intname)
    }
    /* may have a suitable one already */
    sfd = daemon.sfds;
    while !sfd.is_null() {
        if NetAddress_isequal(&mut sfd.source_addr, addr) != 0 &&
               strcmp(intname, sfd.interface.as_mut_ptr()) ==
                   0 && ifindex == sfd.ifindex {
            return sfd
        }
        sfd = sfd.next
    }
    /* need to make a new one. */
    *__errno_location() = 12; /* in case malloc fails. */
    // sfd =
    //     whine_malloc(::std::mem::size_of::<ServerFd>()) ; /* save error from bind/setsockopt. */
    if sfd.is_null() { return 0Fd }
    sfd.fd =
        socket(addr.sa.sa_family, SOCK_DGRAM,
               0);
    if sfd.fd == -(1) {
        // free(sfd);
        return 0Fd
    }
    if addr.sa.sa_family == 10 &&
           setsockopt(sfd.fd, IPPROTO_IPV6,
                      26,
                      &mut opt,
                      ::std::mem::size_of::<libc::c_int>()) == -(1) ||
           local_bind(sfd.fd, addr, intname, ifindex, 0) ==
               0 || fix_fd(sfd.fd) == 0 {
        errsave = *__errno_location();
        close(sfd.fd);
        // free(sfd);
        *__errno_location() = errsave;
        return 0Fd
    }
    safe_strncpy(sfd.interface.as_mut_ptr(), intname,
                 ::std::mem::size_of::<[libc::c_char; 17]>()        );
    sfd.source_addr = *addr;
    sfd.next = daemon.sfds;
    sfd.ifindex = ifindex;
    sfd.preallocated = 0;
    daemon.sfds = sfd;
    return sfd;
}
/* create upstream sockets during startup, before root is dropped which may be needed
   this allows query_port to be a low port and interface binding */

pub fn pre_allocate_sfds(daemon: &mut DnsmasqDaemon) {
    let mut srv: Server = 0;
    let mut sfd:ServerFd = 0;
    if daemon.query_port != 0 {
        let mut addr: NetAddress = Default::default();
        addr._type = AddressType::Ipv4Address;
        addr.port = daemon.query_port;
        sfd = allocate_sfd(&mut addr, "");
        if !sfd.is_null() {
            sfd.preallocated = 1
        }
        addr = Default::default();
        addr._type = AddressType::Ipv6Address;
        addr.value = in6addr_any;
        addr.port = daemon.query_port;
        sfd = allocate_sfd(&mut addr, "" );
        if !sfd.is_null() {
            sfd.preallocated = 1
        }
    }
    
    for srv in daemon.servers {
        if srv.flags & (4 | 2 | 1024 | 2048) == 0 & allocate_sfd(srv.source_addr, srv.interface).is_none() && __errno_location() != 0 && daemon.options[13] {
            prettyprint_addr(&mut srv.source_addr, daemon.namebuff);
            if srv.interface[0] != 0 {
                strcat(daemon.namebuff, " " );
                strcat(daemon.namebuff, srv.interface.as_mut_ptr());
            }
            die("failed to bind server socket for %s: %s", daemon.namebuff,  2);
        }

    };
}

pub fn mark_servers(mut flag: i32) {
    let mut serv: Server;
    /* mark everything with argument flag */
    serv = daemon.servers;
    while !serv.is_null() {
        if serv.flags & flag != 0 { serv.flags |= 256 }
        /* Give looped servers another chance */
        serv.flags &= !(8192);
        serv = serv.next
    };
}

pub fn cleanup_servers() {
    let mut serv: Server = 0;
    let mut tmp: Server = 0;
    let mut up: Server = 0 ;
    /* unlink and free anything still marked. */
    serv = daemon.servers;
    up = &mut daemon.servers;
    while !serv.is_null() {
        tmp = serv.next;
        if serv.flags & 256 != 0 {
            server_gone(serv);
            *up = serv.next;
            if !serv.domain.is_null() {
                // free(serv.domain);
            }
            // free(serv);
        } else { up = &mut serv.next }
        serv = tmp
    }
    /* Now we have a new set of servers, test for loops. */
    loop_send_probes();
}

pub fn add_update_server(mut flags: i32,
                                           mut addr: NetAddress,
                                           mut source_addr: NetAddress,
                                           mut interface: *const libc::c_char,
                                           mut domain: *const libc::c_char) {
    let mut serv: Server = 0;
    let mut next: Server = 0;
    let mut domain_str: &mut String = 0 ;
    /* See if there is a suitable candidate, and unmark */
    serv = daemon.servers;
    while !serv.is_null() {
        if serv.flags & 256 != 0 {
            if !domain.is_null() {
                if !(serv.flags & 8 == 0 ||
                         hostname_isequal(domain, serv.domain) == 0) {
                    break ;
                }
            } else if !(serv.flags & 8 != 0) { break ; }
        }
        serv = serv.next
    }
    if !serv.is_null() {
        domain_str = serv.domain;
        next = serv.next
    } else {
        // serv =
        //     whine_malloc(::std::mem::size_of::<Server>())          Server;
        if !serv.is_null() {
            /* Not found, create a new one. */
            if !domain.is_null() &&
                   {
                       // domain_str =
                       //     whine_malloc(strlen(domain).wrapping_add(1                     libc::c_int
                       //                                                               ))
                       //         ;
                       // domain_str.is_null()
                       true
                   } {
                // free(serv);
                serv = 0
            } else {
                let mut s: Server = 0;
                /* Add to the end of the chain, for order */
                if daemon.servers.is_null() {
                    daemon.servers = serv
                } else {
                    s = daemon.servers;
                    while !s.next.is_null() { s = s.next }
                    s.next = serv
                }
                if !domain.is_null() { strcpy(domain_str, domain); }
            }
        }
    }
    if !serv.is_null() {
        serv.flags = flags;
        serv.domain = domain_str;
        serv.next = next;
        serv.failed_queries = 0;
        serv.queries = serv.failed_queries;
        serv.uid = rand32();
        if !domain.is_null() { serv.flags |= 8 }
        if !interface.is_null() {
            safe_strncpy(serv.interface.as_mut_ptr(), interface,
                         ::std::mem::size_of::<[libc::c_char; 17]>());
        }
        if !addr.is_null() { serv.addr = *addr }
        if !source_addr.is_null() { serv.source_addr = *source_addr }
    };
}

pub fn check_servers() {
    let mut iface: Irec = 0 ;
    let mut serv: Server = 0;
    let mut sfd:ServerFd = 0Fd;
    let mut tmp:ServerFd = 0Fd;
    let mut up: ServerFd = 0 ;
    let mut port: i32 = 0;
    let mut count: i32 = 0;
    let mut locals: i32 = 0;
    /* interface may be new since startup */
    if daemon.options[(13).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (13).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           == 0 {
        enumerate_interfaces(0);
    }
    /* don't garbage collect pre-allocated sfds. */
    sfd = daemon.sfds;
    while !sfd.is_null() {
        sfd.used = sfd.preallocated;
        sfd = sfd.next
    }
    let mut current_block_37: u64;
    count = 0;
    serv = daemon.servers;
    while !serv.is_null() {
        if serv.flags &
               (4 | 2 | 1024 |
                    2048) == 0 {
            /* Init edns_pktsz for newly created server records. */
            if serv.edns_pktsz == 0 {
                serv.edns_pktsz =
                    daemon.edns_pktsz
            }
            port =
                prettyprint_addr(&mut serv.addr,
                                 daemon.namebuff);
            /* 0.0.0.0 is nothing, the stack treats it like 127.0.0.1 */
            if serv.addr.sa.sa_family == 2 &&
                   serv.addr.in_0.sin_addr.s_addr ==
                       0 {
                serv.flags |= 256;
                current_block_37 = 8515828400728868193;
            } else {
                iface = daemon.interfaces;
                while !iface.is_null() {
                    if NetAddress_isequal(&mut serv.addr, &mut iface.addr)
                           != 0 {
                        break ;
                    }
                    iface = iface.next
                }
                if !iface.is_null() {
                    my_syslog(4,
                              "ignoring nameserver %s - local interface"
                                  ,
                              daemon.namebuff);
                    serv.flags |= 256;
                    current_block_37 = 8515828400728868193;
                } else if serv.sfd.is_null() &&
                              {
                                  serv.sfd =
                                      allocate_sfd(&mut serv.source_addr,
                                                   serv.interface.as_mut_ptr());
                                  serv.sfd.is_null()
                              } && *__errno_location() != 0 {
                    my_syslog(4,
                              "ignoring nameserver %s - cannot make/bind socket: %s"
                                  ,
                              daemon.namebuff,
                              strerror(*__errno_location()));
                    serv.flags |= 256;
                    current_block_37 = 8515828400728868193;
                } else {
                    if !serv.sfd.is_null() {
                        (*serv.sfd).used = 1
                    }
                    current_block_37 = 3437258052017859086;
                }
            }
        } else { current_block_37 = 3437258052017859086; }
        match current_block_37 {
            3437258052017859086 => {
                if serv.flags & 2048 == 0 &&
                       serv.flags & 4 == 0 {
                    count += 1;
                    if !(count > 30) {
                        if serv.flags &
                               (8 | 32 |
                                    1024) != 0 {
                            let mut s1: &mut String =
                                0 ;
                            let mut s2: &mut String =
                                0 ;
                            let mut s3: &mut String =
                                ""                               &mut String;
                            if serv.flags & 8 == 0 {
                                s1 =
                                    "unqualified"                                   *const libc::c_char                                  &mut String;
                                s2 =
                                    "names"                                   *const libc::c_char                                  &mut String
                            } else if strlen(serv.domain) ==
                                          0 {
                                s1 =
                                    "default"                                   *const libc::c_char                                  &mut String;
                                s2 =
                                    ""                                   *const libc::c_char                                  &mut String
                            } else {
                                s1 =
                                    "domain"                                   *const libc::c_char                                  &mut String;
                                s2 = serv.domain
                            }
                            if serv.flags & 2 != 0 {
                                count -= 1;
                                locals += 1;
                                if locals <= 8 {
                                    my_syslog(6,
                                              "using only locally-known addresses for %s %s"
                                                                                              *const libc::c_char, s1,
                                              s2);
                                }
                            } else if serv.flags & 1024 != 0
                             {
                                my_syslog(6,
                                          "using standard nameservers for %s %s"
                                                                                      *const libc::c_char, s1, s2);
                            } else {
                                my_syslog(6,
                                          "using nameserver %s#%d for %s %s %s"
                                                                                      *const libc::c_char,
                                          daemon.namebuff, port,
                                          s1, s2, s3);
                            }
                        } else if serv.flags & 8192 != 0 {
                            my_syslog(6,
                                      "NOT using nameserver %s#%d - query loop detected"
                                          ,
                                      daemon.namebuff, port);
                        } else if serv.interface[0 ]
                                      != 0 {
                            my_syslog(6,
                                      "using nameserver %s#%d(via %s)"                                    *const u8,
                                      daemon.namebuff, port,
                                      serv.interface.as_mut_ptr());
                        } else {
                            my_syslog(6,
                                      "using nameserver %s#%d"                                    *const u8,
                                      daemon.namebuff, port);
                        }
                    }
                }
            }
            _ => { }
        }
        serv = serv.next
    }
    if locals > 8 {
        my_syslog(6,
                  "using %d more local addresses", locals - 8);
    }
    if count - 1 > 30 {
        my_syslog(6,
                  "using %d more nameservers",
                  count - 30 - 1);
    }
    /* Do we need a socket set? */
    /* Remove unused sfds */
    sfd = daemon.sfds;
    up = &mut daemon.sfds;
    while !sfd.is_null() {
        tmp = sfd.next;
        if sfd.used == 0 {
            *up = sfd.next;
            close(sfd.fd);
            // free(sfd);
        } else { up = &mut sfd.next }
        sfd = tmp
    }
    cleanup_servers();
}
/* Return zero if no servers found, in that case we keep polling.
   This is a protection against an update-time/write race on resolv.conf */

pub fn reload_servers(mut fname: &mut String)
 -> i32 {
    let mut f: FILE = 0 ;
    let mut line: &mut String = 0 ;
    let mut gotone: i32 = 0;
    /* buff happens to be MAXDNAME long... */
    f = fopen(fname, "r" );
    if f.is_null() {
        my_syslog(3,
                  "failed to read %s: %s", fname,
                  strerror(*__errno_location()));
        return 0
    }
    mark_servers(1);
    loop  {
        line = fgets(daemon.namebuff, 1025, f);
        if line.is_null() { break ; }
        let mut addr: NetAddress = Default::default();
        let mut source_addr: NetAddress = Default::default();
        let mut token: &mut String = strtok(line, " \t\n\r" );
        if token.is_null() { continue ; }
        if strcmp(token, "nameserver\x00" ) != 0 && strcmp(token, b"server" ) != 0 {
            continue ;
        }
        token = strtok(0 , " \t\n\r" );
        if token.is_null() { continue ; }
        addr = Default::default();
        source_addr = Default::default();
        addr.value = inet_addr(token);
        if addr.in_0.sin_addr.s_addr != -(1) {
            addr.in_0.sin_family = 2;
            source_addr.in_0.sin_family = addr.in_0.sin_family;
            addr.in_0.sin_port = __bswap_16(53 );
            source_addr.in_0.sin_addr.s_addr = 0;
            source_addr.in_0.sin_port =
                __bswap_16(daemon.query_port )
        } else {
            let mut scope_index: i32 = 0;
            let mut scope_id: &mut String = strchr(token, '%' as i32);
            if !scope_id.is_null() {
                let fresh8 = scope_id;
                scope_id = scope_id.offset(1);
                *fresh8 = 0;
                scope_index = if_nametoindex(scope_id)
            }
            if !(inet_pton(10, token,
                           &mut addr.in6.sin6_addr) > 0) {
                continue ;
            }
            addr.in6.sin6_family = 10;
            source_addr.in6.sin6_family = addr.in6.sin6_family;
            addr.in6.sin6_flowinfo = 0:;
            source_addr.in6.sin6_flowinfo = addr.in6.sin6_flowinfo;
            addr.in6.sin6_port = __bswap_16(53 );
            addr.in6.sin6_scope_id = scope_index:;
            source_addr.in6.sin6_addr = in6addr_any;
            source_addr.in6.sin6_port =
                __bswap_16(daemon.query_port );
            source_addr.in6.sin6_scope_id = 0:
        }
        add_update_server(1, &mut addr, &mut source_addr,
                          0, 0);
        gotone = 1
    }
    fclose(f);
    cleanup_servers();
    return gotone;
}
/* Called when addresses are added or deleted from an interface */

pub fn newaddress(mut now: time::Instant) {
    if daemon.options[(39).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (39).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 ||
           daemon.options[(49 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (49 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               != 0 || daemon.doing_dhcp6 != 0 ||
           !daemon.relay6.is_null() ||
           daemon.doing_ra != 0 {
        enumerate_interfaces(0);
    }
    if daemon.options[(39).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (39).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 {
        create_bound_listeners(0);
    }
    if daemon.doing_dhcp6 != 0 ||
           !daemon.relay6.is_null() ||
           daemon.doing_ra != 0 {
        join_multicast(0);
    }
    if daemon.doing_dhcp6 != 0 || daemon.doing_ra != 0 {
        dhcp_construct_contexts(now);
    }
    if daemon.doing_dhcp6 != 0 { lease_find_interfaces(now); };
}
