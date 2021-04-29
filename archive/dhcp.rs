
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
use std::{io::stdout, net::UdpSocket};

use crate::{cache::{cache_find_by_addr, cache_get_name}, defines::ArpRecord};
use crate::defines::{ConstNetAddressArg, DevT, ModeT, _ISSPACE, NetAddress, C2rustUnnamed14, DigitalSignature, CmsgHdr, Crec, DhcpBridge, DhcpConfig, DhcpContext, DhcpLease, DhcpNetId, DhcpNetIdList, DhcpRelay, DnsmasqDaemon, HwaddrConfig, IfaceParam, IfReq, NetAddress, InPktInfo, Iname, IPPROTO_IP, IPPROTO_UDP, MatchParam, MsgHdr, NetAddress, PingResult, SaFamily, SharedNetwork, SOCK_DGRAM, NetAddress, NetAddress};
use crate::dhcp_common::{match_netid, recv_dhcp_packet, strip_hostname};
use crate::domain::get_domain;
use crate::forward::send_from;
use crate::icmp_ping;
use crate::lease::{lease_find_by_addr, lease_find_max_addr, lease_prune, lease_update_dns, lease_update_file};
use crate::netlink::iface_enumerate;
use crate::network::{fix_fd, iface_check, indextoname};
use crate::rfc2131::dhcp_reply;
use crate::util::{canonicalise, hostname_isequal, is_same_net4, legal_hostname, parse_hex, retry_send};

fn make_fd(mut port: i32) -> i32 {
    let mut socket: Socket;
    let mut saddr = NetAddress::new();
    let mut oneopt: i32 = 1;
    let mut mtu: u16 = 0;
    let mut tos: u16 = 0xc0;
    if socket == -(1) {
        panic!("cannot create DHCP socket");
    }
    if fix_fd(socket) == 0 ||
           setsockopt(socket, IPPROTO_IP, 10,
                      &mut mtu,
                      ::std::mem::size_of::<>()) == -(1) ||
           setsockopt(socket, IPPROTO_IP, 1,
                      &mut tos,
                      ::std::mem::size_of::<>()) == -(1) ||
           setsockopt(socket, IPPROTO_IP, 8,
                      &mut oneopt,
                      ::std::mem::size_of::<>()) == -(1) ||
           setsockopt(socket, 1, 6,
                      &mut oneopt,
                      ::std::mem::size_of::<>()) == -(1) {
        die("failed to set options on DHCP socket: %s",
            0 , 2);
    }
    /* When bind-interfaces is set, there might be more than one dnsmasq
     instance binding port 67. That's OK if they serve different networks.
     Need to set REUSEADDR|REUSEPORT to make this possible.
     Handle the case that REUSEPORT is defined, but the kernel doesn't 
     support it. This handles the introduction of REUSEPORT on Linux. */
    if daemon.options[13] != 0 || daemon.options[39] != 0 {
        let mut rc: i32 = 0;
        rc = setsockopt(socket, 1, 15, &mut oneopt);
        if rc == -(1) &&
               *__errno_location() == 92 {
            rc = 0
        }
        if rc != -(1) {
            rc = setsockopt(socket, 1, 2, &mut oneopt)
        }
        if rc == -(1) {
            panic!("failed to set SO_REUSE(ADDR|PORT) on DHCP socket");
        }
    }
    saddr = Default::default();
    saddr.sin_family = 2;
    saddr.sin_port = __bswap_16(port);
    saddr.sin_addr.s_addr = 0;
    if bind(socket,
            ConstNetAddressArg {__NetAddress__:
                                     &mut saddr,},
            ::std::mem::size_of::<NetAddress>() ) != 0 {
        panic!("failed to bind DHCP server socket");
    }
    return socket;
}

pub fn dhcp_init() {
    daemon.dhcpfd = make_fd(daemon.dhcp_server_port);
    if daemon.enable_pxe != 0 {
        daemon.pxefd = make_fd(4011)
    } else { daemon.pxefd = -(1) };
}

pub fn dhcp_packet(daemon: &mut DnsmasqDaemon, mut now: time::Instant, mut pxe_fd: i32) {
    let mut fd: UdpSocket =
        if pxe_fd != 0 {
            daemon.pxefd
        } else { daemon.dhcpfd };
    let mut mess = DhcpPacket::new();
    let mut context = DhcpContext::new();
    let mut relay = DhcpRelay::new();
    let mut is_relay_reply: i32 = 0;
    let mut tmp: Iname = 0;
    let mut ifr = IfReq::new();
    // let mut msg: MsgHdr =
    //     MsgHdr {msg_name: 0,
    //            msg_namelen: 0,
    //            msg_iov: 0,
    //            msg_iovlen: 0,
    //            msg_control: 0,
    //            msg_controllen: 0,
    //            msg_flags: 0,};
    let mut dest = NetAddress::new();
    // let mut cmptr: CmsgHdr = 0;
    // let mut iov: iovec = iovec{iov_base: 0, iov_len: 0,};
    let mut sz: isize = 0;
    let mut iface_index: i32 = 0;
    let mut unicast_dest: i32 = 0;
    let mut is_inform = false;
    let mut loopback: i32 = 0;
    let mut rcvd_iface_index: i32 = 0;
    let mut iface_addr = NetAddress::new();
    let mut parm = IfaceParam::new();
    let mut recvtime: time::Instant = now;
    let mut arp_req = ArpRecord::new();
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut control_u: C2RustUnnamed_13::new();
    let mut bridge: DhcpBridge = 0;
    let mut alias: DhcpBridge = 0;
    // msg.msg_controllen =
    //     ::std::mem::size_of::<C2RustUnnamed_13>();
    // msg.msg_control = control_u.control.as_mut_ptr();
    // msg.msg_name = &mut dest;
    // msg.msg_namelen =
    //     ::std::mem::size_of::<NetAddress>();
    // msg.msg_iov = &mut daemon.dhcp_packet;
    // msg.msg_iovlen = 1 ;
    let mut packet_buf: Vec<u8> = Vec::new();
    sz = recv_dhcp_packet(&mut daemon, fd, &mut packet_buf);
    if sz == -(1) ||
           sz <
               (::std::mem::size_of::<dhcp_packet>()).wrapping_sub(::std::mem::size_of::<[u8_0; 312]>()
                                                   ){
        return
    }
    if ioctl(fd, 0x8906,
             &mut tv) == 0 {
        recvtime = tv.tv_sec
    }
    if msg.msg_controllen >= ::std::mem::size_of::<CmsgHdr>()
       {
        cmptr =
            if msg.msg_controllen >=
                   ::std::mem::size_of::<CmsgHdr>() {
                msg.msg_control
            } else { 0 };
        while !cmptr.is_null() {
            if cmptr.cmsg_level == IPPROTO_IP &&
                   cmptr.cmsg_type == 8 {
                let mut p = C2rustUnnamed14::new();
                p.c = cmptr.__cmsg_data.as_mut_ptr();
                iface_index = (*p.p).ipi_ifindex;
                if (*p.p).ipi_addr.s_addr != 0xffffffff {
                    unicast_dest = 1
                }
            }
            cmptr = __cmsg_nxthdr(&mut msg, cmptr)
        }
    }
    if indextoname(daemon.dhcpfd, iface_index, ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) == 0 
        || ioctl(daemon.dhcpfd, 0x8913, &mut ifr) != 0 {
        return
    }
    mess = daemon.dhcp_packet.iov_base;
    loopback =
        (mess.giaddr.s_addr == 0 &&
             ifr.ifr_ifru.ifru_flags &
                 IFF_LOOPBACK != 0);
    /* ARP fiddling uses original interface even if we pretend to use a different one. */
    safe_strncpy(arp_req.arp_dev.as_mut_ptr(),
                 ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 16]>()        );
    /* If the interface on which the DHCP request was received is an
     alias of some other interface (as specified by the
     --bridge-interface option), change ifr.ifr_name so that we look
     for DHCP contexts associated with the aliased interface instead
     of with the aliasing one. */
    rcvd_iface_index = iface_index;
    bridge = daemon.bridges;
    while !bridge.is_null() {
        alias = bridge.alias;
        while !alias.is_null() {
            if wildcard_matchn(alias.iface.as_mut_ptr(),
                               ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                               16) != 0 {
                iface_index =
                    if_nametoindex(bridge.iface.as_mut_ptr()) ;
                if iface_index == 0 {
                    my_syslog((3) << 3 |
                                  4,
                              "unknown interface %s in bridge-interface"
                                  ,
                              bridge.iface.as_mut_ptr());
                    return
                } else {
                    safe_strncpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                                 bridge.iface.as_mut_ptr(),
                                 ::std::mem::size_of::<[libc::c_char; 16]>()
                                    );
                    break ;
                }
            } else { alias = alias.next }
        }
        if !alias.is_null() { break ; }
        bridge = bridge.next
    }
    relay =
        relay_reply4(daemon.dhcp_packet.iov_base,
                     ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
    if !relay.is_null() {
        /* Reply from server, using us as relay. */
        rcvd_iface_index = relay.iface_index;
        if indextoname(daemon.dhcpfd, rcvd_iface_index,
                       ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) == 0 {
            return
        }
        is_relay_reply = 1;
        iov.iov_len = sz ;
        safe_strncpy(arp_req.arp_dev.as_mut_ptr(),
                     ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 16]>());
    } else {
        ifr.ifr_ifru.ifru_addr.sa_family = 2;
        if ioctl(daemon.dhcpfd,
                 0x8915,
                 &mut ifr) != -(1) {
            iface_addr =
                (*(&mut ifr.ifr_ifru.ifru_addr)).sin_addr
        } else {
            if iface_check(2, 0 ,
                           ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                           0) != 0 {
                my_syslog((3) << 3 |
                              4,
                          "DHCP packet received on %s which has no address"
                              ,
                          ifr.ifr_ifrn.ifrn_name.as_mut_ptr());
            }
            return
        }
        tmp = daemon.dhcp_except;
        while !tmp.is_null() {
            if !tmp.name.is_null() &&
                   wildcard_match(tmp.name,
                                  ifr.ifr_ifrn.ifrn_name.as_mut_ptr()) != 0 {
                return
            }
            tmp = tmp.next
        }
        /* unlinked contexts/relays are marked by context->current == context */
        context = daemon.dhcp;
        while !context.is_null() {
            context.current = context;
            context = context.next
        }
        relay = daemon.relay4;
        while !relay.is_null() {
            relay.current = relay;
            relay = relay.next
        }
        parm.current = 0;
        parm.relay = 0;
        parm.relay_local.s_addr = 0;
        parm.ind = iface_index;
        if iface_check(2,
                       &mut iface_addr ,
                       ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                       0) == 0 {
            /* If we failed to match the primary address of the interface, see if we've got a --listen-address
	     for a secondary */
            let mut match_0 = MatchParam::new();
            match_0.matched = 0;
            match_0.ind = iface_index;
            if daemon.if_addrs.is_null() ||
                   iface_enumerate( 2, &mut match_0) == 0 || match_0.matched == 0 {
                return
            }
            iface_addr = match_0.addr;
            /* make sure secondary address gets priority in case
	     there is more than one address on the interface in the same subnet */
            complete_context(match_0.addr, 
                iface_index,
                             0 , 
                             match_0.netmask,
                             match_0.broadcast,
                             &mut parm );
        }
        if iface_enumerate(2, &mut parm )== 0 {
            return
        }
        /* We're relaying this request */
        if parm.relay_local.s_addr != 0 &&
               relay_upstream4(parm.relay, mess, sz , iface_index) !=
                   0 {
            return
        }
        /* May have configured relay, but not DHCP server */
        if daemon.dhcp.is_null() {
            return
        } /* lose any expired leases */
        lease_prune(0, now);
        iov.iov_len =
            dhcp_reply(parm.current, ifr.ifr_ifrn.ifrn_name.as_mut_ptr(),
                       iface_index, sz , now, unicast_dest, loopback,
                       &mut is_inform, pxe_fd, iface_addr, recvtime);
        lease_update_file(now);
        lease_update_dns(0);
        if iov.iov_len == 0 { return }
    }
    msg.msg_name = &mut dest;
    msg.msg_namelen =
        ::std::mem::size_of::<NetAddress>();
    msg.msg_control = 0;
    msg.msg_controllen = 0 ;
    msg.msg_iov = &mut iov;
    iov.iov_base = daemon.dhcp_packet.iov_base;
    /* packet buffer may have moved */
    mess = daemon.dhcp_packet.iov_base;
    if pxe_fd != 0 {
        if mess.ciaddr.s_addr != 0 {
            dest.sin_addr = mess.ciaddr
        }
    } else if mess.giaddr.s_addr != 0 && is_relay_reply == 0 {
        /* Send to BOOTP relay  */
        dest.sin_port =
            __bswap_16(daemon.dhcp_server_port);
        dest.sin_addr = mess.giaddr
    } else if mess.ciaddr.s_addr != 0 {
        /* If the client's idea of its own address tallys with
	 the source address in the request packet, we believe the
	 source port too, and send back to that.  If we're replying 
	 to a DHCPINFORM, trust the source address always. */
        if is_inform == 0 && dest.sin_addr.s_addr != mess.ciaddr.s_addr ||
               dest.sin_port == 0 ||
               dest.sin_addr.s_addr == 0 ||
               is_relay_reply != 0 {
            dest.sin_port =
                __bswap_16(daemon.dhcp_client_port);
            dest.sin_addr = mess.ciaddr
        }
    } else {
        /* fill cmsg for outbound interface (both broadcast & unicast) */
        let mut pkt: InPktInfo = 0 ;
        // msg.msg_control = control_u.control.as_mut_ptr();
        // msg.msg_controllen =
        //     ::std::mem::size_of::<C2RustUnnamed_13>();
        cmptr =
            if msg.msg_controllen >=
                   ::std::mem::size_of::<CmsgHdr>() {
                msg.msg_control
            } else { 0 };
        pkt = cmptr.__cmsg_data.as_mut_ptr() ;
        pkt.ipi_ifindex = rcvd_iface_index;
        pkt.ipi_spec_dst.s_addr = 0;
        // msg.msg_controllen = ::std::mem::size_of::<InPktInfo>() + ::std::mem::size_of::<usize> - 1 & !(::std::mem::size_of::<usize>) - 1 + ::std::mem::size_of::<CmsgHdr>() + ::std::mem::size_of::<usize> - 1
        //     (( ).wrapping_add( & !(::std::mem::size_of::<usize>()) - 1;
        // cmptr.cmsg_len = ((::std::mem::size_of::<CmsgHdr>() )
        //     .wrapping_add(::std::mem::size_of::<usize>())
        //     .wrapping_sub(1) & !(::std::mem::size_of::<usize>())
        //     .wrapping_sub(1))
        //     .wrapping_add(::std::mem::size_of::<InPktInfo>());
        // cmptr.cmsg_level = IPPROTO_IP;
        // cmptr.cmsg_type = 8;
        // todo: convert mess.flags to be bytes
        if mess.flags & 0x8000 != 0 || mess.hlen == 0 || mess.hlen > 14 || mess.htype == 0 {
            /* broadcast to 255.255.255.255 (or mac address invalid) */
            dest.sin_addr.s_addr = 0xffffffff;
            dest.sin_port = daemon.dhcp_client_port.to_be();
        } else {
            /* unicast to unconfigured client. Inject mac address direct into ARP cache.
          struct sockaddr limits size to 14 bytes. */
            dest.sin_addr = mess.yiaddr;
            dest.sin_port = daemon.dhcp_client_port.to_be();
            // memcpy(&mut arp_req.arp_pa,
            //        &mut dest,
            //        ::std::mem::size_of::<NetAddress>());
            arp_req.arp_pa = dest.clone();
            arp_req.arp_ha.sa_family = mess.htype;
            // memcpy(arp_req.arp_ha.sa_data.as_mut_ptr(),
            //        mess.chaddr.as_mut_ptr(),
            //        mess.hlen);
            arp_req.arp_ha.sa_data = mess.chaddr.clone();
            /* interface name already copied in */
            arp_req.arp_flags = 0x2;
            if ioctl(daemon.dhcpfd,
                     0x8955,
                     &mut arp_req) == -(1) {
                my_syslog((3) << 3 |
                              3,
                          "ARP-cache injection failed: %s",
                          strerror(*__errno_location()));
            }
        }
    }
    while retry_send(sendmsg(fd, &mut msg, 0)) != 0 { }
    /* This can fail when, eg, iptables DROPS destination 255.255.255.255 */
    if *__errno_location() != 0 {
        // my_syslog((3) << 3 | 4,
        //           "Error sending DHCP packet to %s: %s", inet_ntoa(dest.sin_addr),
        //           strerror(*__errno_location()));
    }
}
/* check against secondary interface addresses */
fn check_listen_addrs(mut local: NetAddress,
                                        mut if_index: i32,
                                        mut label: &mut String,
                                        mut netmask: NetAddress,
                                        mut broadcast: NetAddress,
                                        mut vparam:Vec<u8>)
                                        -> i32 {
    let mut param: MatchParam = vparam;
    let mut tmp: Iname = 0;
    if if_index == param.ind {
        tmp = daemon.if_addrs;
        while !tmp.is_null() {
            if tmp.addr.sa.sa_family == 2 &&
                   tmp.addr.in_0.sin_addr.s_addr == local.s_addr {
                param.matched = 1;
                param.addr = local;
                param.netmask = netmask;
                param.broadcast = broadcast;
                break ;
            } else { tmp = tmp.next }
        }
    }
    return 1;
}
/* This is a complex routine: it gets called with each (address,netmask,broadcast) triple 
   of each interface (and any relay address) and does the  following things:

   1) Discards stuff for interfaces other than the one on which a DHCP packet just arrived.
   2) Fills in any netmask and broadcast addresses which have not been explicitly configured.
   3) Fills in local (this host) and router (this host or relay) addresses.
   4) Links contexts which are valid for hosts directly connected to the arrival interface on ->current.

   Note that the current chain may be superseded later for configured hosts or those coming via gateways. */
fn guess_range_netmask(mut addr: NetAddress,
                                         mut netmask: NetAddress) {
    let mut context: DhcpContext = 0;
    context = daemon.dhcp;
    while !context.is_null() {
        if context.flags &
               (1) << 1 == 0 &&
               (is_same_net4(addr, context.start, netmask) != 0 ||
                    is_same_net4(addr, context.end, netmask) != 0) {
            if context.netmask.s_addr != netmask.s_addr &&
                   !(is_same_net4(addr, context.start, netmask) != 0 &&
                         is_same_net4(addr, context.end, netmask) != 0) {
                strcpy(daemon.dhcp_buff,
                       inet_ntoa(context.start));
                strcpy(daemon.dhcp_buff2,
                       inet_ntoa(context.end));
                my_syslog((3) << 3 |
                              4,
                          "DHCP range %s -- %s is not consistent with netmask %s"
                              ,
                          daemon.dhcp_buff,
                          daemon.dhcp_buff2, inet_ntoa(netmask));
            }
            context.netmask = netmask
        }
        context = context.next
    };
}
fn complete_context(mut local: NetAddress,
                                      mut if_index: i32,
                                      mut label: &mut String,
                                      mut netmask: NetAddress,
                                      mut broadcast: NetAddress,
                                      mut vparam:Vec<u8>)
                                      -> i32 {
    let mut context: DhcpContext = 0;
    let mut relay: DhcpRelay = 0;
    let mut param: IfaceParam = vparam;
    let mut share: SharedNetwork = 0 ;
    let mut current_block_14: u64;
    share = daemon.shared_networks;
    while !share.is_null() {
        if !(share.shared_addr.s_addr == 0)
           {
            if share.if_index != 0 {
                if share.if_index != if_index {
                    current_block_14 = 17778012151635330486;
                } else { current_block_14 = 13536709405535804910; }
            } else if share.match_addr.s_addr != local.s_addr {
                current_block_14 = 17778012151635330486;
            } else { current_block_14 = 13536709405535804910; }
            match current_block_14 {
                17778012151635330486 => { }
                _ => {
                    context = daemon.dhcp;
                    while !context.is_null() {
                        if context.netmask.s_addr !=
                               0 &&
                               is_same_net4(share.shared_addr, context.start, context.netmask) != 0 &&
                               is_same_net4(share.shared_addr, context.end, context.netmask) != 0 {
                            // link it onto the current chain if we've not seen it before
                            if context.current == context {
                                // For a shared network, we have no way to guess what the default route should be.
                                // Use configured address for Server Identifier
                                context.router.s_addr = 0; 
                                context.local = local;
                                context.current = param.current;
                                param.current = context
                            }
                            if context.flags & (1) << 2 == 0 {
                                context.broadcast.s_addr =
                                    context.start.s_addr |
                                        !context.netmask.s_addr
                            }
                        }
                        context = context.next
                    }
                }
            }
        }
        share = share.next
    }
    guess_range_netmask(local, netmask);
    context = daemon.dhcp;
    while !context.is_null() {
        if context.netmask.s_addr != 0 &&
               is_same_net4(local, context.start, context.netmask) != 0
               && is_same_net4(local, context.end, context.netmask) != 0
           {
            /* link it onto the current chain if we've not seen it before */
            if if_index == param.ind && context.current == context {
                context.router = local;
                context.local = local;
                context.current = param.current;
                param.current = context
            }
            if context.flags &
                   (1) << 2 == 0 {
                if is_same_net4(broadcast, context.start,
                                context.netmask) != 0 {
                    context.broadcast = broadcast
                } else {
                    context.broadcast.s_addr =
                        context.start.s_addr | !context.netmask.s_addr
                }
            }
        }
        context = context.next
    }
    relay = daemon.relay4;
    while !relay.is_null() {
        if if_index == param.ind &&
               relay.local.addr4.s_addr == local.s_addr &&
               relay.current == relay &&
               (param.relay_local.s_addr ==
                    0 ||
                    param.relay_local.s_addr == local.s_addr) {
            relay.current = param.relay;
            param.relay = relay;
            param.relay_local = local
        }
        relay = relay.next
    }
    return 1;
}

pub fn address_available(mut context: DhcpContext,
                                           mut taddr: NetAddress,
                                           mut netids: &mut DhcpNetId)
                                           -> DhcpContext {
    /* Check is an address is OK for this network, check all
     possible ranges. Make sure that the address isn't in use
     by the server itself. */
    let mut start: u32 = 0;
    let mut end: u32 = 0;
    let mut addr: u32 = __bswap_32(taddr.s_addr);
    let mut tmp: DhcpContext = 0;
    tmp = context;
    while !tmp.is_null() {
        if taddr.s_addr == context.router.s_addr {
            return 0
        }
        tmp = tmp.current
    }
    tmp = context;
    while !tmp.is_null() {
        start = __bswap_32(tmp.start.s_addr);
        end = __bswap_32(tmp.end.s_addr);
        if tmp.flags &
               ((1) << 0 |
                    (1) << 3) == 0 &&
               addr >= start && addr <= end &&
               match_netid(tmp.filter, netids, 1) != 0 {
            return tmp
        }
        tmp = tmp.current
    }
    return 0;
}

pub fn narrow_context(mut context: DhcpContext,
                                        mut taddr: NetAddress,
                                        mut netids: &mut DhcpNetId)
                                        -> DhcpContext {
    /* We start of with a set of possible contexts, all on the current physical interface.
     These are chained on ->current.
     Here we have an address, and return the actual context corresponding to that
     address. Note that none may fit, if the address came a dhcp-host and is outside
     any dhcp-range. In that case we return a static range if possible, or failing that,
     any context on the correct subnet. (If there's more than one, this is a dodgy 
     configuration: maybe there should be a warning.) */
    let mut tmp: DhcpContext = 0;
    tmp = address_available(context, taddr, netids);
    if tmp.is_null() {
        tmp = context;
        while !tmp.is_null() {
            if match_netid(tmp.filter, netids, 1) != 0 &&
                   is_same_net4(taddr, tmp.start, tmp.netmask) != 0 &&
                   tmp.flags &
                       (1) << 0 != 0 {
                break ;
            }
            tmp = tmp.current
        }
        if tmp.is_null() {
            tmp = context;
            while !tmp.is_null() {
                if match_netid(tmp.filter, netids, 1) != 0
                       &&
                       is_same_net4(taddr, tmp.start, tmp.netmask) != 0
                       &&
                       tmp.flags &
                           (1) << 3 == 0 {
                    break ;
                }
                tmp = tmp.current
            }
        }
    }
    /* Only one context allowed now */
    if !tmp.is_null() { tmp.current = 0 }
    return tmp;
}

pub fn config_find_by_address(mut configs: &mut DhcpConfig,
                                                mut addr: NetAddress)
                                                -> DhcpConfig {
    let mut config: DhcpConfig = 0;
    config = configs;
    while !config.is_null() {
        if config.flags & 32 != 0 &&
               config.addr.s_addr == addr.s_addr {
            return config
        }
        config = config.next
    }
    return 0;
}
/* Check if and address is in use by sending ICMP ping.
   This wrapper handles a cache and load-limiting.
   Return is NULL is address in use, or a pointer to a cache entry
   recording that it isn't. */

pub fn do_icmp_ping(mut now: time::Instant, mut addr: NetAddress,
                                      mut hash: u32,
                                      mut loopback: i32)
                                      -> PingResult {
    let mut dummy = PingResult::new();
    let mut r: PingResult = 0 ;
    let mut victim: PingResult = 0 ;
    let mut count: i32 = 0;
    let mut max: i32 =
        (0.6f64 *
             (30  /
                  3 ) ) ;
    /* check if we failed to ping addr sometime in the last
     PING_CACHE_TIME seconds. If so, assume the same situation still exists.
     This avoids problems when a stupid client bangs
     on us repeatedly. As a final check, if we did more
     than 60% of the possible ping checks in the last 
     PING_CACHE_TIME, we are in high-load mode, so don't do any more. */
    count = 0; /* old record */
    r = daemon.ping_results;
    while !r.is_null() {
        if difftime(now, r.time) >
               30   {
            victim = r
        } else { count += 1; if r.addr.s_addr == addr.s_addr { return r } }
        r = r.next
    }
    /* didn't find cached entry */
    if count >= max ||
           daemon.options[21] != 0 || loopback != 0 {
        /* overloaded, or configured not to check, loopback interface, return "not in use" */
        dummy.hash = hash; /* address in use. */
        return dummy
    } else if icmp_ping(addr) != 0 {
        return 0
    } else {
        /* at this point victim may hold an expired record */
        if victim.is_null() {
            victim = PingResult::new();
            if !victim.is_null() {
                victim.next = daemon.ping_results;
                daemon.ping_results = victim
            }
        }
        /* record that this address is OK for 30s 
	 without more ping checks */
        if !victim.is_null() {
            victim.addr = addr;
            victim.time = now;
            victim.hash = hash
        }
        return victim
    };
}

pub fn address_allocate(mut context: DhcpContext,
                                          mut addrp: NetAddress,
                                          mut hwaddr: &mut Vec<u8>,
                                          mut hw_len: i32,
                                          mut netids: &mut DhcpNetId,
                                          mut now: time::Instant,
                                          mut loopback: i32)
                                          -> i32 {
    /* Find a free address: exclude anything in use and anything allocated to
     a particular hwaddr/clientid/hostname in our configuration.
     Try to return from contexts which match netids first. */
    let mut start = NetAddress::new();
    let mut addr = NetAddress::new();
    let mut c: DhcpContext = 0;
    let mut d: DhcpContext = 0;
    let mut i: i32 = 0;
    let mut pass: i32 = 0;
    let mut j: u32 = 0;
    /* hash hwaddr: use the SDBM hashing algorithm.  Seems to give good
     dispersal even with similarly-valued "strings". */
    j = 0;
    i = 0;
    while i < hw_len {
        j = (*hwaddr.offset(i))
            .wrapping_add(j << 6)
            .wrapping_add(j << 16)
            .wrapping_sub(j);
        i += 1
    }
    /* j == 0 is marker */
    if j == 0 {
        j = 1
    }
    pass = 0;
    while pass <= 1 {
        c = context;
        while !c.is_null() {
            if !(c.flags &
                     ((1) << 0 |
                          (1) << 3) != 0) {
                if !(match_netid(c.filter, netids, pass) == 0) {
                    if daemon.options[34] != 0 {
                        /* seed is largest extant lease addr in this context */
                        start = lease_find_max_addr(c)
                    } else {
                        /* pick a seed based on hwaddr */
                        start.s_addr = __bswap_32(__bswap_32(c.start.s_addr)
                            .wrapping_add(j.wrapping_add(c.addr_epoch)
                            .wrapping_rem((1)
                            .wrapping_add(__bswap_32(c.end.s_addr))
                            .wrapping_sub(__bswap_32(c.start.s_addr)))))
                    }
                    /* iterate until we find a free address. */
                    addr = start;
                    loop  {
                        /* eliminate addresses in use by the server. */
                        d = context;
                        while !d.is_null() {
                            if addr.s_addr == d.router.s_addr { break ; }
                            d = d.current
                        }
                        /* Addresses which end in .255 and .0 are broken in Windows even when using 
	       supernetting. ie dhcp-range=192.168.0.1,192.168.1.254,255,255,254.0
	       then 192.168.0.255 is a valid IP address, but not for Windows as it's
	       in the class C range. See  KB281579. We therefore don't allocate these 
	       addresses to avoid hard-to-diagnose problems. Thanks Bill. */
                        if d.is_null() && lease_find_by_addr(addr).is_null()
                               &&
                               config_find_by_address(daemon.dhcp_conf,
                                                      addr).is_null() &&
                               (!(__bswap_32(addr.s_addr) &
                                      0xe0000000 ==
                                      0xc0000000) ||
                                    __bswap_32(addr.s_addr) &
                                        0xff !=
                                        0xff &&
                                        __bswap_32(addr.s_addr) & 0xff  != 0)
                           {
                            /* in consec-ip mode, skip addresses equal to
		   the number of addresses rejected by clients. This
		   should avoid the same client being offered the same
		   address after it has rjected it. */
                            if daemon.options[34] != 0 && c.addr_epoch != 0 {
                                c.addr_epoch = c.addr_epoch.wrapping_sub(1)
                            } else {
                                let mut r: PingResult =
                                    0 ;
                                r = do_icmp_ping(now, addr, j, loopback);
                                if !r.is_null() {
                                    /* consec-ip mode: we offered this address for another client
			   (different hash) recently, don't offer it to this one. */
                                    if daemon.options[34] == 0 || r.hash == j {
                                        *addrp = addr;
                                        return 1
                                    }
                                } else if daemon.options[34] == 0 {
                                    c.addr_epoch =
                                        c.addr_epoch.wrapping_add(1)
                                }
                            }
                        }
                        addr.s_addr =
                            __bswap_32(__bswap_32(addr.s_addr).wrapping_add(1));
                        if addr.s_addr ==
                               __bswap_32(__bswap_32(c.end.s_addr).wrapping_add(1))
                           {
                            addr = c.start
                        }
                        if !(addr.s_addr != start.s_addr) { break ; }
                    }
                }
            }
            c = c.current
        }
        pass += 1
    }
    return 0;
}

pub fn dhcp_read_ethers() {
    let mut f: FILE =
        fopen("/etc/ethers" ,
              "r" );
    let mut flags: u32 = 0;
    let mut buff: &mut String = daemon.namebuff;
    let mut ip: &mut String = 0 ;
    let mut cp: &mut String = 0 ;
    let mut addr = NetAddress::new();
    let mut hwaddr: [u8; 6] = [0; 6];
    let mut up: DhcpConfig;
    let mut tmp: DhcpConfig = 0;
    let mut config: DhcpConfig = 0;
    let mut count: i32 = 0;
    let mut lineno: i32 = 0;
    /* address in use: perturb address selection so that we are
			   less likely to try this address again. */
    addr.s_addr = 0; /* eliminate warning */
    if f.is_null() {
        // my_syslog((3) << 3 | 3,
        //           "failed to read %s: %s",
        //           "/etc/ethers" ,
        //           strerror(*__errno_location()));
        return
    }
    /* This can be called again on SIGHUP, so remove entries created last time round. */
    up = &mut daemon.dhcp_conf;
    config = daemon.dhcp_conf;
    while !config.is_null() {
        tmp = config.next;
        if config.flags & 256 != 0 {
            *up = tmp;
            /* cannot have a clid */
            if config.flags & 16 != 0 {
                // free(config.hostname);
            }
            // free(config.hwaddr);
            // free(config);
        } else { up = &mut config.next }
        config = tmp
    }
    while !fgets(buff, 1025, f).is_null() {
        let mut host: &mut String = 0 ;
        lineno += 1;
        while strlen(buff) > 0 &&
                  *(*__ctype_b_loc()).offset(*buff.offset(strlen(buff).wrapping_sub(1

                                                                                                              )
                                                             )                                           )  &
                      _ISSPACE
                      != 0 {
            *buff.offset(strlen(buff).wrapping_sub(1)) = 0
        }
        if *buff == '#' as i32 ||
               *buff == '+' as i32 ||
               *buff == 0 {
            continue ;
        }
        ip = buff;
        while *ip != 0 &&
                  *(*__ctype_b_loc()).offsetip  &
                      _ISSPACE
                      == 0 {
            ip = ip.offset(1)
        }
        while *ip != 0 &&
                  *(*__ctype_b_loc()).offsetip  &
                      _ISSPACE
                      != 0 {
            *ip = 0;
            ip = ip.offset(1)
        }
        if *ip == 0 ||
               parse_hex(buff, hwaddr.as_mut_ptr(), 6,
                         0, 0) !=
                   6 {
            my_syslog((3) << 3 |
                          3,
                      "bad line at %s line %d",
                      "/etc/ethers" ,
                      lineno);
        } else {
            /* check for name or dotted-quad */
            cp = ip;
            while cp != 0 {
                if !(cp == '.' as i32 ||
                         cp >= '0' as i32 &&
                             cp <= '9' as i32) {
                    break ;
                }
                cp = cp.offset(1)
            }
            if cp == 0 {
                addr.s_addr = inet_addr(ip);
                if addr.s_addr == -(1) {
                    my_syslog((3) << 3 |
                                  3,
                              "bad address at %s line %d",
                              "/etc/ethers", lineno);
                    continue ;
                } else {
                    flags = 32;
                    config = daemon.dhcp_conf;
                    while !config.is_null() {
                        if config.flags & 32
                               != 0 && config.addr.s_addr == addr.s_addr {
                            break ;
                        }
                        config = config.next
                    }
                }
            } else {
                let mut nomem: i32 = 0;
                host = canonicalise(ip, &mut nomem);
                if host.is_null() || legal_hostname(host) == 0 {
                    if nomem == 0 {
                        my_syslog((3) << 3 |
                                      3,
                                  "bad name at %s line %d"        ,
                                  "/etc/ethers", lineno);
                    }
                    // free(host);
                    continue ;
                } else {
                    flags = 16;
                    config = daemon.dhcp_conf;
                    while !config.is_null() {
                        if config.flags & 16
                               != 0 &&
                               hostname_isequal(config.hostname, host) != 0
                           {
                            break ;
                        }
                        config = config.next
                    }
                }
            }
            if !config.is_null() &&
                   config.flags & 256 != 0 {
                my_syslog((3) << 3 |
                              3,
                          "ignoring %s line %d, duplicate name or IP address"
                              ,
                          "/etc/ethers" , lineno);
            } else {
                if config.is_null() {
                    config = daemon.dhcp_conf;
                    while !config.is_null() {
                        let mut conf_addr: HwaddrConfig =
                            config.hwaddr;
                        if !conf_addr.is_null() && conf_addr.next.is_null()
                               &&
                               conf_addr.wildcard_mask ==
                                   0 &&
                               conf_addr.hwaddr_len == 6 &&
                               (conf_addr.hwaddr_type == 1
                                    ||
                                    conf_addr.hwaddr_type ==
                                        0) &&
                            //    memcmp(conf_addr.hwaddr,
                            //           hwaddr,
                            //           6) == 0 
                               conf_addr.hwaddr == hwaddr {
                            break ;
                        }
                        config = config.next
                    }
                    if config.is_null() {
                        // config =
                        //     whine_malloc(::std::mem::size_of::<DhcpConfig>()
                        //                     );
                        if config.is_null() { continue ; }
                        config.flags = 256;
                        config.hwaddr = 0;
                        config.domain = 0 ;
                        config.netid = 0;
                        config.next = daemon.dhcp_conf;
                        daemon.dhcp_conf = config
                    }
                    config.flags |= flags;
                    if flags & 16 != 0 {
                        config.hostname = host;
                        host = 0
                    }
                    if flags & 32 != 0 {
                        config.addr = addr
                    }
                }
                config.flags |= 128;
                if config.hwaddr.is_null() {
                    // config.hwaddr =
                    //     whine_malloc(::std::mem::size_of::<HwaddrConfig>())
                }
                if !config.hwaddr.is_null() {
                    memcpy((*config.hwaddr).hwaddr.as_mut_ptr(),
                           hwaddr.as_mut_ptr(),
                           6);
                    (*config.hwaddr).hwaddr_len = 6;
                    (*config.hwaddr).hwaddr_type = 1;
                    (*config.hwaddr).wildcard_mask =
                        0;
                    (*config.hwaddr).next = 0
                }
                count += 1;
                // free(host);
            }
        }
    }
    fclose(f);
    my_syslog((3) << 3 | 6,
              "read %s - %d addresses" ,
              "/etc/ethers" , count);
}
/* If we've not found a hostname any other way, try and see if there's one in /etc/hosts
   for this address. If it has a domain part, that must match the set domain and
   it gets stripped. The set of legal domain names is bigger than the set of legal hostnames
   so check here that the domain name is legal as a hostname. 
   NOTE: we're only allowed to overwrite daemon->dhcp_buff if we succeed. */

pub fn host_from_dns(mut addr: NetAddress)
 -> &mut String {
    let mut lookup: Crec = 0 ; /* DNS disabled. */
    if daemon.port == 0 {
        return 0
    }
    lookup =
        cache_find_by_addr(0 ,
                           &mut addr ,
                           0,
                           (1) << 7);
    if !lookup.is_null() &&
           lookup.flags & (1) << 6 != 0 {
        let mut dot: &mut String = 0 ;
        let mut hostname: &mut String = cache_get_name(lookup);
        dot = strchr(hostname, '.' as i32);
        if !dot.is_null() &&
               strlen(dot.offset(1)) !=
                   0 {
            let mut d2: &mut String = get_domain(addr);
            if d2.is_null() ||
                   hostname_isequal(dot.offset(1), d2)
                       == 0 {
                return 0
            }
            /* wrong domain */
        }
        if legal_hostname(hostname) == 0 { return 0  }
        safe_strncpy(daemon.dhcp_buff, hostname,
                     256 );
        strip_hostname(daemon.dhcp_buff);
        return daemon.dhcp_buff
    }
    return 0 ;
}
fn relay_upstream4(mut relay: &mut DhcpRelay,
                                     mut mess: &mut dhcp_packet,
                                     mut sz: usize,
                                     mut iface_index: i32)
                                     -> i32 {
    /* ->local is same value for all relays on ->current chain */
    let mut from = NetAddress::new();
    if mess.op != 1 {
        return 0
    }
    /* source address == relay address */
    from.addr4 = relay.local.addr4;
    /* already gatewayed ? */
    if mess.giaddr.s_addr != 0 {
        /* if so check if by us, to stomp on loops. */
        if mess.giaddr.s_addr == relay.local.addr4.s_addr {
            return 1
        }
    } else {
        /* plug in our address */
        mess.giaddr.s_addr = relay.local.addr4.s_addr
    }
    let fresh6 = mess.hops;
    mess.hops = mess.hops.wrapping_add(1);
    if fresh6 > 20 { return 1 }
    while !relay.is_null() {
        let mut to = NetAddress::new();
        to.sa.sa_family = 2;
        to.in_0.sin_addr = relay.server.addr4;
        to.in_0.sin_port =
            __bswap_16(daemon.dhcp_server_port);
        send_from(daemon.dhcpfd, 0,
                  mess , sz, &mut to, &mut from,
                  0);
        if daemon.options[28] != 0 {
            inet_ntop(2, &mut relay.local, daemon.addrbuff, 46);
            // my_syslog((3) << 3 |
            //               6,
            //           "DHCP relay %s -> %s", daemon.addrbuff,
            //           inet_ntoa(relay.server.addr4));
        }
        /* Save this for replies */
        relay.iface_index = iface_index;
        relay = relay.current
    }
    return 1;
}
fn relay_reply4(mut mess: &mut dhcp_packet,
                                  mut arrival_interface: &mut String)
 -> DhcpRelay {
    let mut relay: DhcpRelay = 0;
    if mess.giaddr.s_addr == 0 || mess.op != 2 {
        return 0
    }
    relay = daemon.relay4;
    while !relay.is_null() {
        if mess.giaddr.s_addr == relay.local.addr4.s_addr {
            if relay.interface.is_null() ||
                   wildcard_match(relay.interface, arrival_interface) != 0
               {
                return if relay.iface_index != 0 {
                           relay
                       } else { 0 }
            }
        }
        relay = relay.next
    }
    return 0;
}
