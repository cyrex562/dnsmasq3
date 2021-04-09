
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

use crate::defines::{NetAddress, In6Addr, C2RustUnnamed, SOCK_DGRAM, IPPROTO_UDP, IPPROTO_IPV6, socklen_t, DnsmasqDaemon, SaFamily, __bswap_16, ConstNetAddressArg, NetAddress, time::Instant, DhcpContext, DhcpRelay, IfaceParam, CmsgHdr, MsgHdr, iovec, C2RustUnnamed_13, IfReq, DigitalSignature, Iname, C2rustUnnamed12, DhcpBridge, DhcpLease, NetAddress, timespec, TimeT, SyscallSlongT, SharedNetwork, __bswap_32, DhcpConfig, AddressListEntry, DhcpNetId, Cparam, NetAddress, NetAddress, AddressType};
use crate::network::{fix_fd, set_ipv6pktinfo, indextoname, iface_check};
use crate::dnsmasq_log::{die, my_syslog};
use crate::dhcp_common::{recv_dhcp_packet, match_netid, dhcp_context_to_string};
use crate::rfc3315::{relay_reply6, relay_upstream6, dhcp6_reply};
use crate::util::{retry_send, wildcard_match, wildcard_matchn, is_same_net6, rand64, addr6part, setaddr6part, zero_array_1};
use crate::outpacket::save_counter;
use crate::netlink::iface_enumerate;
use crate::lease::{lease_prune, lease_update_file, lease_update_dns, lease_find_max_addr6, lease6_find_by_addr, lease_update_slaac};
use crate::slack::{neigh_packet, IPPROTO_ICMPV6};
use crate::arp::find_mac;
use crate::radv::{ra_start_unsolicited, periodic_ra};
use crate::send_alarm;
use num::zero;
use crate::in6_addr::In6Addr;

#[no_mangle]
pub unsafe extern "C" fn dhcp6_init() {
    let mut fd: i32 = 0;
    let mut saddr: NetAddress =
        NetAddress {sin6_family: 0,
                     sin6_port: 0,
                     sin6_flowinfo: 0,
                     sin6_addr:
                         In6Addr {__in6_u:
                                      C2RustUnnamed{__u6_addr8: [0; 16],},},
                     sin6_scope_id: 0,};
    let mut class: i32 = 0xc0;
    let mut oneopt: i32 = 1;
    fd =
        socket(10, SOCK_DGRAM,
               IPPROTO_UDP);
    if fd == -(1) ||
           setsockopt(fd, IPPROTO_IPV6, 67,
                      &mut class,
                      ::std::mem::size_of::<>()) == -(1) ||
           setsockopt(fd, IPPROTO_IPV6, 26,
                      &mut oneopt,
                      ::std::mem::size_of::<>()) == -(1) || fix_fd(fd) == 0
           || set_ipv6pktinfo(fd) == 0 {
        die("cannot create DHCPv6 socket: %s",
            0 , 2);
    }
    /* When bind-interfaces is set, there might be more than one dnsmasq
     instance binding port 547. That's OK if they serve different networks.
     Need to set REUSEADDR|REUSEPORT to make this possible.
     Handle the case that REUSEPORT is defined, but the kernel doesn't 
     support it. This handles the introduction of REUSEPORT on Linux. */
    if dnsmasq_daemon.options[(13).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
                                     ] &
           (1) <<
               (13).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
           != 0 ||
           dnsmasq_daemon.options[(39 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8                                                                   ))
                                         ] &
               (1) <<
                   (39 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8
                                                                                                                       ))
               != 0 {
        let mut rc: i32 = 0;
        rc =
            setsockopt(fd, 1, 15,
                       &mut oneopt,
                       ::std::mem::size_of::<>()
                          );
        if rc == -(1) &&
               *__errno_location() == 92 {
            rc = 0
        }
        if rc != -(1) {
            rc =
                setsockopt(fd, 1, 2,
                           &mut oneopt as
                           ::std::mem::size_of::<>())
        }
        if rc == -(1) {
            die("failed to set SO_REUSE{ADDR|PORT} on DHCPv6 socket: %s"
                     ,
                0 , 2);
        }
    }
    saddr6 = Default::default();
    saddr.sin6_family = 10;
    saddr.sin6_addr = in6addr_any;
    saddr.sin6_port = __bswap_16(547);
    if bind(fd,
            ConstNetAddressArg {__NetAddress__:
                                     &mut saddr6                                   NetAddress,},
            ::std::mem::size_of::<NetAddress>() ) != 0 {
        die("failed to bind DHCPv6 server socket: %s",
            0 , 2);
    }
    dnsmasq_daemon.dhcp6fd = fd;
}
#[no_mangle]
pub unsafe extern "C" fn dhcp6_packet(mut now: time::Instant) {
    let mut context: DhcpContext = 0;
    let mut relay: DhcpRelay = 0;
    let mut parm: IfaceParam =
        IfaceParam {current: 0,
                    relay: 0,
                    fallback:
                        In6Addr {__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},},
                    relay_local:
                        In6Addr {__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},},
                    ll_addr:
                        In6Addr {__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},},
                    ula_addr:
                        In6Addr {__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},},
                    ind: 0,
                    addr_match: 0,};
    let mut cmptr: CmsgHdr = 0;
    let mut msg: MsgHdr =
        MsgHdr {msg_name: 0,
               msg_namelen: 0,
               msg_iov: 0,
               msg_iovlen: 0,
               msg_control: 0,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut if_index: i32 = 0;
    let mut control_u: C2RustUnnamed_13 =
        C2RustUnnamed_13{align:
                             CmsgHdr {cmsg_len: 0,
                                     cmsg_level: 0,
                                     cmsg_type: 0,
                                     __cmsg_data: [],},};
    let mut from: NetAddress =
        NetAddress {sin6_family: 0,
                     sin6_port: 0,
                     sin6_flowinfo: 0,
                     sin6_addr:
                         In6Addr {__in6_u:
                                      C2RustUnnamed{__u6_addr8: [0; 16],},},
                     sin6_scope_id: 0,};
    let mut sz: isize = 0;
    let mut ifr: IfReq =
        IfReq {ifr_ifrn: C2RustUnnamed_3{ifrn_name: [0; 16],},
              ifr_ifru:
                  DigitalSignature {ifru_addr:
                                      NetAddress {sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut tmp: Iname = 0;
    let mut port: u16 = 0;
    let mut dst_addr: NetAddress = NetAddress{ _type: AddressType::Ipv6Address, value: [0;16]};
    msg.msg_control = control_u.control6.as_mut_ptr();
    msg.msg_controllen =
        ::std::mem::size_of::<C2RustUnnamed_13>();
    msg.msg_flags = 0;
    msg.msg_name = &mut from6;
    msg.msg_namelen =
        ::std::mem::size_of::<NetAddress>();
    msg.msg_iov = &mut dnsmasq_daemon.dhcp_packet;
    msg.msg_iovlen = 1 ;
    sz = recv_dhcp_packet(dnsmasq_daemon.dhcp6fd, &mut msg);
    if sz == -(1) { return }
    cmptr =
        if msg.msg_controllen >=
               ::std::mem::size_of::<CmsgHdr>() {
            msg.msg_control
        } else { 0 };
    while !cmptr.is_null() {
        if cmptr.cmsg_level == IPPROTO_IPV6 &&
               cmptr.cmsg_type == dnsmasq_daemon.v6pktinfo {
            let mut p: C2rustUnnamed12 =
                C2rustUnnamed12 {c: 0,};
            p.c = cmptr.__cmsg_data.as_mut_ptr();
            if_index = (*p.p).ipi6_ifindex;
            dst_addr = (*p.p).ipi6_addr
        }
        cmptr = __cmsg_nxthdr(&mut msg, cmptr)
    }
    if indextoname(dnsmasq_daemon.dhcp6fd, if_index,
                   ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) == 0 {
        return
    }
    port = relay_reply6(&mut from, sz, ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
    if port != 0 {
        from.sin6_port = __bswap_16(port);
        while retry_send(sendto(dnsmasq_daemon.dhcp6fd,
                                dnsmasq_daemon.outpacket.iov_base,
                                save_counter(-(1)) ,
                                0,
                                ConstNetAddressArg {__NetAddress__:
                                                         &mut from          NetAddress
                                                                      NetAddress,},
                                ::std::mem::size_of::<NetAddress>()                       )) != 0 {
        }
    } else {
        let mut bridge: DhcpBridge = 0;
        let mut alias: DhcpBridge = 0;
        tmp = dnsmasq_daemon.if_except;
        while !tmp.is_null() {
            if !tmp.name.is_null() &&
                   wildcard_match(tmp.name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                return
            }
            tmp = tmp.next
        }
        tmp = dnsmasq_daemon.dhcp_except;
        while !tmp.is_null() {
            if !tmp.name.is_null() &&
                   wildcard_match(tmp.name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                return
            }
            tmp = tmp.next
        }
        parm.current = Default::default();
        parm.relay = Default::default();
        parm.relay_local = Default::default();
        parm.ind = if_index;
        parm.addr_match = 0;
        zero_array_1(&mut parm.fallback, 16);
        zero_array_1(&mut parm.ll_addr, 16);
        zero_array_1(&mut parm.ula_addr, 16);
        /* If the interface on which the DHCPv6 request was received is
         an alias of some other interface (as specified by the
         --bridge-interface option), change parm.ind so that we look
         for DHCPv6 contexts associated with the aliased interface
         instead of with the aliasing one. */
        bridge = dnsmasq_daemon.bridges;
        while !bridge.is_null() {
            alias = bridge.alias;
            while !alias.is_null() {
                if wildcard_matchn(alias.iface.as_mut_ptr(),
                                   ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                                   16) != 0 {
                    parm.ind =
                        if_nametoindex(bridge.iface.as_mut_ptr())                      ;
                    if parm.ind == 0 {
                        my_syslog((3) << 3 |
                                      4,
                                  "unknown interface %s in bridge-interface"
                                      ,
                                  bridge.iface.as_mut_ptr());
                        return
                    }
                    break ;
                } else { alias = alias.next }
            }
            if !alias.is_null() { break ; }
            bridge = bridge.next
        }
        context = dnsmasq_daemon.dhcp6;
        while !context.is_null() {
            if ({
                    let mut __a: *const In6Addr =
                        &mut context.start6                      *const In6Addr;
                    (__a.__in6_u.__u6_addr32[0 ] ==
                         0 &&
                         __a.__in6_u.__u6_addr32[1 ]
                             == 0 &&
                         __a.__in6_u.__u6_addr32[2 ]
                             == 0 &&
                         __a.__in6_u.__u6_addr32[3 ]
                             == 0)
                }) != 0 && context.prefix == 0 {
                /* wildcard context for DHCP-stateless only */
                parm.current = context;
                context.current = 0
            } else {
                /* unlinked contexts are marked by context->current == context */
                context.current = context;

            }
            context = context.next
        }
        relay = dnsmasq_daemon.relay6;
        while !relay.is_null() {
            relay.current = relay;
            relay = relay.next
        }
        if iface_enumerate(10,
                           &mut parm,
                           ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                   &mut In6Addr,
                                                                               _:
                                                                                   ,
                                                                               _:
                                                                                   ,
                                                                               _:
                                                                                   ,
                                                                               _:
                                                                                   ,
                                                                               _:
                                                                                   libc::c_uint,
                                                                               _:
                                                                                   libc::c_uint,
                                                                               _:
                                                                                  Vec<u8>)
                                                                               -> i32>,
                                                   Option<unsafe extern "C" fn()
                                                              ->
                                                                  >>(Some(complete_context6 unsafe extern "C" fn(_:
                                                                                                                  &mut In6Addr,
                                                                                                              _:
                                                                                                                  ,
                                                                                                              _:
                                                                                                                  ,
                                                                                                              _:
                                                                                                                  ,
                                                                                                              _:
                                                                                                                  ,
                                                                                                              _:
                                                                                                                  libc::c_uint,
                                                                                                              _:
                                                                                                                  libc::c_uint,
                                                                                                              _:
                                                                                                                 Vec<u8>)
                                                                                                              ->
                                                                                                 )))
               == 0 {
            return
        }
        if !dnsmasq_daemon.if_names.is_null() ||
               !dnsmasq_daemon.if_addrs.is_null() {
            tmp = dnsmasq_daemon.if_names;
            while !tmp.is_null() {
                if !tmp.name.is_null() &&
                       wildcard_match(tmp.name,
                                      ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) !=
                           0 {
                    break ;
                }
                tmp = tmp.next
            }
            if tmp.is_null() && parm.addr_match == 0 { return }
        }
        if !parm.relay.is_null() {
            /* Ignore requests sent to the ALL_SERVERS multicast address for relay when
	     we're listening there for DHCPv6 server reasons. */
            let mut all_servers: In6Addr =
                In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
            inet_pton(10,
                      "FF05::1:3" ,
                      &mut all_servers);
            if ({
                    let mut __a: *const In6Addr =
                        &mut dst_addr ;
                    let mut __b: *const In6Addr =
                        &mut all_servers ;
                    (__a.__in6_u.__u6_addr32[0 ] ==
                         __b.__in6_u.__u6_addr32[0 ]
                         &&
                         __a.__in6_u.__u6_addr32[1 ]
                             ==
                             __b.__in6_u.__u6_addr32[1         usize] &&
                         __a.__in6_u.__u6_addr32[2 ]
                             ==
                             __b.__in6_u.__u6_addr32[2         usize] &&
                         __a.__in6_u.__u6_addr32[3 ]
                             ==
                             __b.__in6_u.__u6_addr32[3         usize])
                }) == 0 {
                relay_upstream6(parm.relay, sz, &mut from.sin6_addr,
                                from.sin6_scope_id, now);
            }
            return
        }
        /* May have configured relay, but not DHCP server */
        if dnsmasq_daemon.doing_dhcp6 == 0 {
            return
        } /* lose any expired leases */
        lease_prune(0, now);
        port =
            dhcp6_reply(parm.current, if_index,
                        ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                        &mut parm.fallback, &mut parm.ll_addr,
                        &mut parm.ula_addr, sz , &mut from.sin6_addr,
                        now);
        /* The port in the source address of the original request should
	 be correct, but at least once client sends from the server port,
	 so we explicitly send to the client port to a client, and the
	 server port to a relay. */
        if port != 0 {
            from.sin6_port = __bswap_16(port);
            while retry_send(sendto(dnsmasq_daemon.dhcp6fd,
                                    dnsmasq_daemon.outpacket.iov_base,
                                    save_counter(-(1))                                  usize, 0,
                                    ConstNetAddressArg {__NetAddress__:
                                                             &mut from              NetAddress
                                                                              NetAddress,},
                                    ::std::mem::size_of::<NetAddress>() )) != 0 {
            }
        }
        /* These need to be called _after_ we send DHCPv6 packet, since lease_update_file()
	 may trigger sending an RA packet, which overwrites our buffer. */
        lease_update_file(now);
        lease_update_dns(0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_client_mac(mut client: &mut In6Addr,
                                        mut iface: i32,
                                        mut mac: mut Vec<u8>,
                                        mut maclenp: &mut libc::c_uint,
                                        mut mactypep: &mut libc::c_uint,
                                        mut now: time::Instant) {
    /* Receiving a packet from a host does not populate the neighbour
     cache, so we send a neighbour discovery request if we can't 
     find the sender. Repeat a few times in case of packet loss. */
    let mut neigh: neigh_packet =
        neigh_packet{type_0: 0,
                     code: 0,
                     checksum: 0,
                     reserved: 0,
                     target:
                         In6Addr {__in6_u:
                                      C2RustUnnamed{__u6_addr8: [0; 16],},},};
    let mut addr: NetAddress = NetAddress:new();
    let mut i: i32 = 0;
    let mut maclen: i32 = 0;
    neigh.type_0 = 135 as u8;
    neigh.code = 0 as u8;
    neigh.reserved = 0;
    neigh.target = *client;
    /* RFC4443 section-2.3: checksum has to be zero to be calculated */
    neigh.checksum = 0; /* 100ms */

    addr.in6.sin6_family = 10;
    addr.in6.sin6_port =
        __bswap_16(IPPROTO_ICMPV6);
    addr.in6.sin6_addr = *client;
    addr.in6.sin6_scope_id = iface;
    i = 0;
    while i < 5 {
        let mut ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
        maclen = find_mac(&mut addr, mac, 0, now);
        if maclen != 0 { break ; }
        sendto(dnsmasq_daemon.icmp6fd,
               &mut neigh ,
               ::std::mem::size_of::<neigh_packet>(),
               0,
               ConstNetAddressArg {__NetAddress__: &mut addr.sa,},
               ::std::mem::size_of::<NetAddress>() );
        ts.tv_sec = 0 ;
        ts.tv_nsec = 100000000;
        nanosleep(&mut ts, 0);
        i += 1
    }
    *maclenp = maclen;
    *mactypep = 1;
}
unsafe extern "C" fn complete_context6(mut local: &mut In6Addr,
                                       mut prefix: i32,
                                       mut scope: i32,
                                       mut if_index: i32,
                                       mut flags: i32,
                                       mut preferred: u32,
                                       mut valid: u32,
                                       mut vparam:Vec<u8>)
                                       -> i32 {
    let mut context: DhcpContext = 0;
    let mut share: SharedNetwork = 0 ;
    let mut relay: DhcpRelay = 0;
    let mut param: IfaceParam = vparam;
    let mut tmp: Iname = 0;
    /* warning */
    if if_index != param.ind { return 1 }
    if ({
            let mut __a: *const In6Addr = local ;
            (__a.__in6_u.__u6_addr32[0 ] &
                 __bswap_32(0xffc00000) ==
                 __bswap_32(0xfe800000))
        }) != 0 {
        param.ll_addr = *local
    } else if *(local ).offset(0) &
                  __bswap_32(0xff000000) ==
                  __bswap_32(0xfd000000) {
        param.ula_addr = *local
    }
    if ({
            let mut __a: *const In6Addr = local ;
            (__a.__in6_u.__u6_addr32[0 ] ==
                 0 &&
                 __a.__in6_u.__u6_addr32[1 ] ==
                     0 &&
                 __a.__in6_u.__u6_addr32[2 ] ==
                     0 &&
                 __a.__in6_u.__u6_addr32[3 ] ==
                     __bswap_32(1))
        }) != 0 ||
           ({
                let mut __a: *const In6Addr = local ;
                (__a.__in6_u.__u6_addr32[0 ] &
                     __bswap_32(0xffc00000) ==
                     __bswap_32(0xfe800000))
            }) != 0 ||
           *(local ).offset(0)  == 0xff {
        return 1
    }
    /* if we have --listen-address config, see if the 
     arrival interface has a matching address. */
    tmp = dnsmasq_daemon.if_addrs;
    while !tmp.is_null() {
        if tmp.addr.sa.sa_family == 10 &&
               ({
                    let mut __a: *const In6Addr =
                        &mut tmp.addr.in6.sin6_addr                      *const In6Addr;
                    let mut __b: *const In6Addr = local ;
                    (__a.__in6_u.__u6_addr32[0 ] ==
                         __b.__in6_u.__u6_addr32[0 ]
                         &&
                         __a.__in6_u.__u6_addr32[1 ]
                             ==
                             __b.__in6_u.__u6_addr32[1         usize] &&
                         __a.__in6_u.__u6_addr32[2 ]
                             ==
                             __b.__in6_u.__u6_addr32[2         usize] &&
                         __a.__in6_u.__u6_addr32[3 ]
                             ==
                             __b.__in6_u.__u6_addr32[3         usize])
                }) != 0 {
            param.addr_match = 1
        }
        tmp = tmp.next
    }
    /* Determine a globally address on the arrival interface, even
     if we have no matching dhcp-context, because we're only
     allocating on remote subnets via relays. This
     is used as a default for the DNS server option. */
    param.fallback = *local;
    context = dnsmasq_daemon.dhcp6;
    while !context.is_null() {
        if context.flags &
               (1) << 8 != 0 &&
               context.flags &
                   ((1) << 10 |
                        (1) << 16) == 0 &&
               prefix <= context.prefix && context.current == context {
            if is_same_net6(local, &mut context.start6, context.prefix)
                   != 0 &&
                   is_same_net6(local, &mut context.end6,
                                context.prefix) != 0 {
                let mut tmp_0: DhcpContext = 0;
                let mut up: DhcpContext =
                    0 ;
                /* use interface values only for constructed contexts */
                if context.flags &
                       (1) << 11 == 0 {
                    valid = 0xffffffff;
                    preferred = valid
                } else if flags & 2 != 0 {
                    preferred = 0
                }
                if context.flags &
                       (1) << 9 != 0 {
                    preferred = 0
                }
                /* order chain, longest preferred time first */
                up = &mut param.current;
                tmp_0 = param.current;
                while !tmp_0.is_null() {
                    if tmp_0.preferred <= preferred { break ; }
                    up = &mut tmp_0.current;
                    tmp_0 = tmp_0.current
                }
                context.current = *up;
                *up = context;
                context.local6 = *local;
                context.preferred = preferred;
                context.valid = valid
            } else {
                let mut current_block_37: u64;
                share = dnsmasq_daemon.shared_networks;
                while !share.is_null() {
                    /* IPv4 shared_address - ignore */
                    if !(share.shared_addr.s_addr !=
                             0) {
                        if share.if_index != 0 {
                            if share.if_index != if_index {
                                current_block_37 = 11385396242402735691;
                            } else {
                                current_block_37 = 18435049525520518667;
                            }
                        } else if ({
                                       let mut __a: *const In6Addr =
                                           &mut share.match_addr6                                         &mut In6Addr                                         *const In6Addr;
                                       let mut __b: *const In6Addr =
                                           local ;
                                       (__a.__in6_u.__u6_addr32[0
                                                                                          usize]
                                            ==
                                            __b.__in6_u.__u6_addr32[0
                                                                                                  usize]
                                            &&
                                            __a.__in6_u.__u6_addr32[1
                                                                                                  usize]
                                                ==
                                                __b.__in6_u.__u6_addr32[1

                                                                                                          usize]
                                            &&
                                            __a.__in6_u.__u6_addr32[2
                                                                                                  usize]
                                                ==
                                                __b.__in6_u.__u6_addr32[2

                                                                                                          usize]
                                            &&
                                            __a.__in6_u.__u6_addr32[3
                                                                                                  usize]
                                                ==
                                                __b.__in6_u.__u6_addr32[3

                                                                                                          usize])

                                   }) == 0 {
                            current_block_37 = 11385396242402735691;
                        } else { current_block_37 = 18435049525520518667; }
                        match current_block_37 {
                            11385396242402735691 => { }
                            _ => {
                                if is_same_net6(&mut share.shared_addr6,
                                                &mut context.start6,
                                                context.prefix) != 0 &&
                                       is_same_net6(&mut share.shared_addr6,
                                                    &mut context.end6,
                                                    context.prefix) != 0 {
                                    context.current = param.current;
                                    param.current = context;
                                    context.local6 = *local;
                                    context.preferred =
                                        if context.flags &
                                               (1) <<
                                                   9 != 0 {
                                            0
                                        } else { 0xffffffff };
                                    context.valid =
                                        0xffffffff
                                }
                            }
                        }
                    }
                    share = share.next
                }
            }
        }
        context = context.next
    }
    relay = dnsmasq_daemon.relay6;
    while !relay.is_null() {
        if ({
                let mut __a: *const In6Addr = local ;
                let mut __b: *const In6Addr =
                    &mut relay.local.addr6 ;
                (__a.__in6_u.__u6_addr32[0 ] ==
                     __b.__in6_u.__u6_addr32[0 ] &&
                     __a.__in6_u.__u6_addr32[1 ] ==
                         __b.__in6_u.__u6_addr32[1 ]
                     &&
                     __a.__in6_u.__u6_addr32[2 ] ==
                         __b.__in6_u.__u6_addr32[2 ]
                     &&
                     __a.__in6_u.__u6_addr32[3 ] ==
                         __b.__in6_u.__u6_addr32[3     usize])
            }) != 0 && relay.current == relay &&
               (({
                     let mut __a: *const In6Addr =
                         &mut param.relay_local                       *const In6Addr;
                     (__a.__in6_u.__u6_addr32[0 ] ==
                          0 &&
                          __a.__in6_u.__u6_addr32[1      usize] ==
                              0 &&
                          __a.__in6_u.__u6_addr32[2      usize] ==
                              0 &&
                          __a.__in6_u.__u6_addr32[3      usize] ==
                              0)
                 }) != 0 ||
                    ({
                         let mut __a: *const In6Addr =
                             local ;
                         let mut __b: *const In6Addr =
                             &mut param.relay_local                           *const In6Addr;
                         (__a.__in6_u.__u6_addr32[0      usize] ==
                              __b.__in6_u.__u6_addr32[0          usize] &&
                              __a.__in6_u.__u6_addr32[1          usize] ==
                                  __b.__in6_u.__u6_addr32[1
                                                                 ] &&
                              __a.__in6_u.__u6_addr32[2          usize] ==
                                  __b.__in6_u.__u6_addr32[2
                                                                 ] &&
                              __a.__in6_u.__u6_addr32[3          usize] ==
                                  __b.__in6_u.__u6_addr32[3
                                                                 ])
                     }) != 0) {
            relay.current = param.relay;
            param.relay = relay;
            param.relay_local = *local
        }
        relay = relay.next
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn config_find_by_address6(mut configs:
                                                     &mut DhcpConfig,
                                                 mut net: &mut In6Addr,
                                                 mut prefix: i32,
                                                 mut addr: &mut In6Addr)
                                                 -> DhcpConfig {
    let mut config: DhcpConfig = 0;
    config = configs;
    while !config.is_null() {
        if config.flags & 4096 != 0 {
            let mut addr_list: AddressListEntry = 0 ;
            addr_list = config.addr6;
            while !addr_list.is_null() {
                if (net.is_null() ||
                        is_same_net6(&mut addr_list.addr.addr6, net,
                                     prefix) != 0 ||
                        addr_list.flags & 16 != 0 &&
                            prefix == 64) &&
                       is_same_net6(&mut addr_list.addr.addr6, addr,
                                    (if addr_list.flags & 8
                                            != 0 {
                                         addr_list.prefixlen
                                     } else { 128 })) != 0 {
                    return config
                }
                addr_list = addr_list.next
            }
        }
        config = config.next
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn address6_allocate(mut context: DhcpContext,
                                           mut clid: mut Vec<u8>,
                                           mut clid_len: i32,
                                           mut temp_addr: i32,
                                           mut iaid: u32,
                                           mut serial: i32,
                                           mut netids: &mut DhcpNetId,
                                           mut plain_range: i32,
                                           mut ans: &mut In6Addr)
                                           -> DhcpContext {
    /* Find a free address: exclude anything in use and anything allocated to
     a particular hwaddr/clientid/hostname in our configuration.
     Try to return from contexts which match netids first. 
     
     Note that we assume the address prefix lengths are 64 or greater, so we can
     get by with 64 bit arithmetic.
*/
    let mut start: u64 = 0;
    let mut addr: u64 = 0;
    let mut c: DhcpContext = 0;
    let mut d: DhcpContext = 0;
    let mut i: i32 = 0;
    let mut pass: i32 = 0;
    let mut j: u64 = 0;
    /* hash hwaddr: use the SDBM hashing algorithm.  This works
     for MAC addresses, let's see how it manages with client-ids! 
     For temporary addresses, we generate a new random one each time. */
    if temp_addr != 0 {
        j = rand64()
    } else {
        j = iaid as u64;
        i = 0;
        while i < clid_len {
            j =
                (*clid.offset(i)               libc::c_ulonglong).wrapping_add(j <<
                                                         6          ).wrapping_add(j
                                                                                           <<
                                                                                           16             ).wrapping_sub(j);
            i += 1
        }
    }
    pass = 0;
    while if pass <= plain_range {
              1
          } else { 0 } != 0 {
        c = context;
        while !c.is_null() {
            if !(c.flags &
                     ((1) << 9 |
                          (1) << 0 |
                          (1) << 7 |
                          (1) << 15) != 0) {
                if !(match_netid(c.filter, netids, pass) == 0) {
                    if temp_addr == 0 &&
                           dnsmasq_daemon.options[(34).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                           ).wrapping_mul(8
                                                                                                                           ))
                                                         ] &
                               (1) <<
                                   (34 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                       ).wrapping_mul(8))
                               != 0 {
                        /* seed is largest extant lease addr in this context,
		 skip addresses equal to the number of addresses rejected
		 by clients. This should avoid the same client being offered the same
		 address after it has rjected it. */
                        start =
                            lease_find_max_addr6(c).wrapping_add(1
                                                                                      libc::c_ulonglong).wrapping_add(serial                                 libc::c_ulonglong).wrapping_add(c.addr_epoch                                                                                                         libc::c_ulonglong);
                        if c.addr_epoch != 0 {
                            c.addr_epoch = c.addr_epoch.wrapping_sub(1)
                        }
                    } else {
                        let mut range: u64 =
                            (1)long).wrapping_add(addr6part(&mut c.end6)).wrapping_sub(addr6part(&mut c.start6));
                        let mut offset: u64 =
                            j.wrapping_add(c.addr_epochlong);
                        /* don't divide by zero if range is whole 2^64 */
                        if range != 0long {
                            offset = offset.wrapping_rem(range)
                        }
                        start =
                            addr6part(&mut c.start6).wrapping_add(offset)
                    }
                    /* iterate until we find a free address. */
                    addr = start;
                    loop  {
                        /* eliminate addresses in use by the server. */
                        d = context;
                        while !d.is_null() {
                            if addr == addr6part(&mut d.local6) { break ; }
                            d = d.current
                        }
                        *ans = c.start6;
                        setaddr6part(ans, addr);
                        if d.is_null() &&
                               lease6_find_by_addr(&mut c.start6,
                                                   c.prefix,
                                                   addr).is_null() &&
                               config_find_by_address6(dnsmasq_daemon.dhcp_conf,
                                                       &mut c.start6,
                                                       c.prefix,
                                                       ans).is_null() {
                            return c
                        }
                        addr = addr.wrapping_add(1);
                        if addr ==
                               addr6part(&mut c.end6).wrapping_add(1
                                                                                                libc::c_ulonglong)
                           {
                            addr = addr6part(&mut c.start6)
                        }
                        if !(addr != start) { break ; }
                    }
                }
            }
            c = c.current
        }
        pass += 1
    }
    return 0;
}
/* can dynamically allocate addr */
#[no_mangle]
pub unsafe extern "C" fn address6_available(mut context: DhcpContext,
                                            mut taddr: &mut In6Addr,
                                            mut netids: &mut DhcpNetId,
                                            mut plain_range: i32)
                                            -> DhcpContext {
    let mut start: u64 = 0;
    let mut end: u64 = 0;
    let mut addr: u64 = addr6part(taddr);
    let mut tmp: DhcpContext = 0;
    tmp = context;
    while !tmp.is_null() {
        start = addr6part(&mut tmp.start6);
        end = addr6part(&mut tmp.end6);
        if tmp.flags &
               ((1) << 0 |
                    (1) << 7) == 0 &&
               is_same_net6(&mut tmp.start6, taddr, tmp.prefix) != 0 &&
               is_same_net6(&mut tmp.end6, taddr, tmp.prefix) != 0 &&
               addr >= start && addr <= end &&
               match_netid(tmp.filter, netids, plain_range) != 0 {
            return tmp
        }
        tmp = tmp.current
    }
    return 0;
}
/* address OK if configured */
#[no_mangle]
pub unsafe extern "C" fn address6_valid(mut context: DhcpContext,
                                        mut taddr: &mut In6Addr,
                                        mut netids: &mut DhcpNetId,
                                        mut plain_range: i32)
                                        -> DhcpContext {
    let mut tmp: DhcpContext = 0;
    tmp = context;
    while !tmp.is_null() {
        if is_same_net6(&mut tmp.start6, taddr, tmp.prefix) != 0 &&
               match_netid(tmp.filter, netids, plain_range) != 0 {
            return tmp
        }
        tmp = tmp.current
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn make_duid(mut now: time::Instant) {
    if !dnsmasq_daemon.duid_config.is_null() {
        let mut p: mut Vec<u8> = 0;
        p =
            safe_malloc(dnsmasq_daemon.duid_config_len.wrapping_add(6
                                                                                                  libc::c_uint)
                            );
        dnsmasq_daemon.duid = p;
        dnsmasq_daemon.duid_len =
            dnsmasq_daemon.duid_config_len.wrapping_add(6            libc::c_uint)
               ;
        let mut t_s: u16 = 2;
        let mut t_cp: mut Vec<u8> = p;
        let fresh6 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh6 = (t_s >> 8);
        *t_cp = t_s;
        p = p.offset(2);
        /* DUID_EN */
        let mut t_l: u32 = dnsmasq_daemon.duid_enterprise;
        let mut t_cp_0: mut Vec<u8> = p;
        let fresh7 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh7 = (t_l >> 24);
        let fresh8 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh8 = (t_l >> 16);
        let fresh9 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh9 = (t_l >> 8);
        *t_cp_0 = t_l;
        p = p.offset(4);
        memcpy(p,
               dnsmasq_daemon.duid_config,
               dnsmasq_daemon.duid_config_len);
    } else {
        let mut newnow: time::Instant = 0;
        /* If we have no persistent lease database, or a non-stable RTC, use DUID_LL (newnow == 0) */
        /* rebase epoch to 1/1/2000 */
        if dnsmasq_daemon.options[(22 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                                                   ))
                                         ] &
               (1) <<
                   (22 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8
                                                                                                                       ))
               == 0 || !dnsmasq_daemon.lease_change_command.is_null() {
            newnow = now - 946684800
        }
        iface_enumerate(1,
                        &mut newnow,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                ,
                                                                            _:
                                                                                libc::c_uint,
                                                                            _:
                                                                                &mut String,
                                                                            _:
                                                                                usize,
                                                                            _:
                                                                               Vec<u8>)
                                                           -> i32>,
                                                Option<unsafe extern "C" fn()
                                                           ->
                                                               >>(Some(make_duid1
                                                                                                                        unsafe extern "C" fn(_:
                                                                                                               ,
                                                                                                           _:
                                                                                                               libc::c_uint,
                                                                                                           _:
                                                                                                               &mut String,
                                                                                                           _:
                                                                                                               usize,
                                                                                                           _:
                                                                                                              Vec<u8>)
                                                                                          ->
                                                                                              )));
        if dnsmasq_daemon.duid.is_null() {
            die("Cannot create DHCPv6 server DUID: %s"               *const libc::c_char ,
                0 , 5);
        }
    };
}
unsafe extern "C" fn make_duid1(mut index: i32,
                                mut type_0: u32,
                                mut mac: &mut String,
                                mut maclen: usize,
                                mut parm:Vec<u8>) -> i32 {
    /* create DUID as specified in RFC3315. We use the MAC of the
     first interface we find that isn't loopback or P-to-P and
     has address-type < 256. Address types above 256 are things like 
     tunnels which don't have usable MAC addresses. */
    let mut p: mut Vec<u8> = 0;
    let mut newnow: time::Instant = *(parm);
    if type_0 >= 256 {
        return 1
    }
    if newnow == 0 {
        // p =
        //     safe_malloc(maclen.wrapping_add(4                                   ))          mut Vec<u8>;
        dnsmasq_daemon.duid = p;
        dnsmasq_daemon.duid_len =
            maclen.wrapping_add(4) ;
        let mut t_s: u16 = 3;
        let mut t_cp: mut Vec<u8> = p;
        let fresh10 = t_cp;
        t_cp = t_cp.offset(1);
        *fresh10 = (t_s >> 8);
        *t_cp = t_s;
        p = p.offset(2);
        /* address type */
        let mut t_s_0: u16 = type_0;
        let mut t_cp_0: mut Vec<u8> = p;
        let fresh11 = t_cp_0;
        t_cp_0 = t_cp_0.offset(1);
        *fresh11 =
            (t_s_0 >> 8);
        *t_cp_0 = t_s_0;
        p = p.offset(2)
    } else {
        // p =
        //     safe_malloc(maclen.wrapping_add(8                                   ))          mut Vec<u8>;
        dnsmasq_daemon.duid = p;
        dnsmasq_daemon.duid_len =
            maclen.wrapping_add(8) ;
        let mut t_s_1: u16 = 1;
        let mut t_cp_1: mut Vec<u8> = p;
        let fresh12 = t_cp_1;
        t_cp_1 = t_cp_1.offset(1);
        *fresh12 =
            (t_s_1 >> 8);
        *t_cp_1 = t_s_1;
        p = p.offset(2);
        /* DUID_LL */
        /* time */
        let mut t_s_2: u16 = type_0;
        let mut t_cp_2: mut Vec<u8> = p;
        let fresh13 = t_cp_2;
        t_cp_2 = t_cp_2.offset(1);
        *fresh13 =
            (t_s_2 >> 8);
        *t_cp_2 = t_s_2;
        p = p.offset(2);
        let mut t_l: u32 = *(parm);
        let mut t_cp_3: mut Vec<u8> = p;
        let fresh14 = t_cp_3;
        t_cp_3 = t_cp_3.offset(1);
        *fresh14 = (t_l >> 24);
        let fresh15 = t_cp_3;
        t_cp_3 = t_cp_3.offset(1);
        *fresh15 = (t_l >> 16);
        let fresh16 = t_cp_3;
        t_cp_3 = t_cp_3.offset(1);
        *fresh16 = (t_l >> 8);
        *t_cp_3 = t_l;
        p = p.offset(4)
    }
    memcpy(p, mac, maclen);
    return 0;
}
unsafe extern "C" fn construct_worker(mut local: &mut In6Addr,
                                      mut prefix: i32,
                                      mut scope: i32,
                                      mut if_index: i32,
                                      mut flags: i32,
                                      mut preferred: i32,
                                      mut valid: i32,
                                      mut vparam:Vec<u8>)
                                      -> i32 {
    let mut ifrn_name: [libc::c_char; 16] = [0; 16];
    let mut start6: In6Addr =
        In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    let mut end6: In6Addr =
        In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    let mut template: DhcpContext = 0;
    let mut context: DhcpContext = 0;
    let mut tmp: Iname = 0;
    let mut param: Cparam = vparam ;
    if ({
            let mut __a: *const In6Addr = local ;
            (__a.__in6_u.__u6_addr32[0 ] ==
                 0 &&
                 __a.__in6_u.__u6_addr32[1 ] ==
                     0 &&
                 __a.__in6_u.__u6_addr32[2 ] ==
                     0 &&
                 __a.__in6_u.__u6_addr32[3 ] ==
                     __bswap_32(1))
        }) != 0 ||
           ({
                let mut __a: *const In6Addr = local ;
                (__a.__in6_u.__u6_addr32[0 ] &
                     __bswap_32(0xffc00000) ==
                     __bswap_32(0xfe800000))
            }) != 0 ||
           *(local ).offset(0)  == 0xff {
        return 1
    }
    if flags & 4 == 0 { return 1 }
    if flags & 2 != 0 { return 1 }
    /* DUID_LLT */
    /* address type */
    /* Ignore interfaces where we're not doing RA/DHCP6 */
    if indextoname(dnsmasq_daemon.icmp6fd, if_index,
                   ifrn_name.as_mut_ptr()) == 0 ||
           iface_check(1, 0 ,
                       ifrn_name.as_mut_ptr(), 0) == 0 {
        return 1
    }
    tmp = dnsmasq_daemon.dhcp_except;
    while !tmp.is_null() {
        if !tmp.name.is_null() &&
               wildcard_match(tmp.name, ifrn_name.as_mut_ptr()) != 0 {
            return 1
        }
        tmp = tmp.next
    }
    template = dnsmasq_daemon.dhcp6;
    while !template.is_null() {
        if template.flags &
               ((1) << 10 |
                    (1) << 11) == 0 {
            /* non-template entries, just fill in interface and local addresses */
            if prefix <= template.prefix &&
                   is_same_net6(local, &mut template.start6,
                                template.prefix) != 0 &&
                   is_same_net6(local, &mut template.end6,
                                template.prefix) != 0 {
                /* First time found, do fast RA. */
                if template.if_index == 0 {
                    ra_start_unsolicited(param.now, template);
                    param.newone = 1
                }
                template.if_index = if_index;
                template.local6 = *local
            }
        } else if wildcard_match(template.template_interface,
                                 ifrn_name.as_mut_ptr()) != 0 &&
                      template.prefix >= prefix {
            start6 = *local;
            setaddr6part(&mut start6, addr6part(&mut template.start6));
            end6 = *local;
            setaddr6part(&mut end6, addr6part(&mut template.end6));
            context = dnsmasq_daemon.dhcp6;
            while !context.is_null() {
                if context.flags &
                       (1) << 10 == 0 &&
                       ({
                            let mut __a: *const In6Addr =
                                &mut start6;
                            let mut __b: *const In6Addr =
                                &mut context.start6;
                            (__a.__in6_u.__u6_addr32[0         usize] ==
                                 __b.__in6_u.__u6_addr32[0] &&
                                 __a.__in6_u.__u6_addr32[1] ==
                                     __b.__in6_u.__u6_addr32[1       ]
                                 &&
                                 __a.__in6_u.__u6_addr32[2] ==
                                     __b.__in6_u.__u6_addr32[2       ]
                                 &&
                                 __a.__in6_u.__u6_addr32[3] ==
                                     __b.__in6_u.__u6_addr32[3       ])

                        }) != 0 &&
                       ({
                            let mut __a: *const In6Addr =
                                &mut end6 ;
                            let mut __b: *const In6Addr =
                                &mut context.end6;
                            (__a.__in6_u.__u6_addr32[0         usize] ==
                                 __b.__in6_u.__u6_addr32[0] &&
                                 __a.__in6_u.__u6_addr32[1] ==
                                     __b.__in6_u.__u6_addr32[1       ]
                                 &&
                                 __a.__in6_u.__u6_addr32[2] ==
                                     __b.__in6_u.__u6_addr32[2       ]
                                 &&
                                 __a.__in6_u.__u6_addr32[3] ==
                                     __b.__in6_u.__u6_addr32[3       ])

                        }) != 0 {
                    /* If there's an absolute address context covering this address
		 then don't construct one as well. */
                    if context.flags &
                           (1) << 11 == 0 {
                        break ;
                    }
                    if context.if_index == if_index {
                        let mut cflags: i32 = context.flags;
                        context.flags =
                            (context.flags &
                                 !((1) << 12 |
                                       (1) <<
                                           16));
                        if cflags &
                               (1) << 16 != 0 {
                            /* address went, now it's back, and on the same interface */
                            dhcp_context_to_string(10, context);
                            /* fast RAs for a while */
                            ra_start_unsolicited(param.now, context);
                            param.newone = 1;
                            /* Add address to name again */
                            if context.flags &
                                   (1) << 6 !=
                                   0 {
                                param.newname = 1
                            }
                        }
                        break ;
                    }
                }
                context = context.next
            }
            if context.is_null() &&
                   {
                       // context =
                       //     whine_malloc(::std::mem::size_of::<DhcpContext>()
                       //                     )                         DhcpContext;
                       // !context.is_null()
                       true
                   } {
                *context = *template;
                context.start6 = start6;
                context.end6 = end6;
                context.flags =
                    (context.flags &
                         !((1) << 10)) ;
                context.flags =
                    (context.flags |
                         (1) << 11) ;
                context.if_index = if_index;
                context.local6 = *local;
                context.saved_valid = 0;
                context.next = dnsmasq_daemon.dhcp6;
                dnsmasq_daemon.dhcp6 = context;
                ra_start_unsolicited(param.now, context);
                /* we created a new one, need to call
	       lease_update_file to get periodic functions called */
                param.newone = 1;
                /* Will need to add new putative SLAAC addresses to existing leases */
                if context.flags &
                       (1) << 6 != 0 {
                    param.newname = 1
                }
                dhcp_context_to_string(10, context);
            }
        }
        template = template.next
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn dhcp_construct_contexts(mut now: time::Instant) {
    let mut context: DhcpContext = 0;
    let mut tmp: DhcpContext = 0;
    let mut up: DhcpContext = 0 ;
    let mut param: Cparam = Cparam {now: 0, newone: 0, newname: 0,};
    param.newone = 0;
    param.newname = 0;
    param.now = now;
    context = dnsmasq_daemon.dhcp6;
    while !context.is_null() {
        if context.flags &
               (1) << 11 != 0 {
            context.flags =
                (context.flags |
                     (1) << 12)
        }
        context = context.next
    }
    iface_enumerate(10,
                    &mut param ,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            &mut In6Addr,
                                                                        _:
                                                                            ,
                                                                        _:
                                                                            ,
                                                                        _:
                                                                            ,
                                                                        _:
                                                                            ,
                                                                        _:
                                                                            ,
                                                                        _:
                                                                            ,
                                                                        _:
                                                                           Vec<u8>)
                                                                        -> i32>,
                                            Option<unsafe extern "C" fn()
                                                       ->
                                                           >>(Some(construct_worker
                                                                                                                unsafe extern "C" fn(_:
                                                                                                           &mut In6Addr,
                                                                                                       _:
                                                                                                           ,
                                                                                                       _:
                                                                                                           ,
                                                                                                       _:
                                                                                                           ,
                                                                                                       _:
                                                                                                           ,
                                                                                                       _:
                                                                                                           ,
                                                                                                       _:
                                                                                                           ,
                                                                                                       _:
                                                                                                          Vec<u8>)
                                                                                                       ->
                                                                                          )));
    up = &mut dnsmasq_daemon.dhcp6;
    context = dnsmasq_daemon.dhcp6;
    while !context.is_null() {
        tmp = context.next;
        if context.flags &
               (1) << 12 != 0 &&
               context.flags &
                   (1) << 16 == 0 {
            if context.flags &
                   (1) << 13 != 0 ||
                   dnsmasq_daemon.options[(37 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                           ).wrapping_mul(8                                                                                                   ))
                                                 ] &
                       (1) <<
                           (37                   ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                       ).wrapping_mul(8                           ))
                       != 0 {
                /* previously constructed context has gone. advertise it's demise */
                context.flags =
                    (context.flags |
                         (1) << 16) ;
                context.address_lost_time = now;
                /* Apply same ceiling of configured lease time as in radv.c */
                if context.saved_valid > context.lease_time {
                    context.saved_valid = context.lease_time
                }
                /* maximum time is 2 hours, from RFC */
                if context.saved_valid >
                       7200 {
                    /* 2 hours */
                    context.saved_valid =
                        7200
                } /* include deletion */
                ra_start_unsolicited(now, context);
                param.newone = 1;
                if context.flags &
                       (1) << 6 != 0 {
                    param.newname = 1
                }
                dhcp_context_to_string(10, context);
                up = &mut context.next
            } else {
                /* we were never doing RA for this, so free now */
                *up = context.next;
                // free(context);
            }
        } else { up = &mut context.next }
        context = tmp
    }
    if param.newone != 0 {
        if !dnsmasq_daemon.dhcp.is_null() ||
               dnsmasq_daemon.doing_dhcp6 != 0 {
            if param.newname != 0 { lease_update_slaac(now); }
            lease_update_file(now);
        } else {
            /* Not doing DHCP, so no lease system, manage alarms for ra only */
            send_alarm(periodic_ra(now), now);
        }
    };
}
