
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
/* NB. This code may be called during a DHCPv4 or transaction which is in ping-wait
   It therefore cannot use any DHCP buffer resources except outpacket, which is
   not used by DHCPv4 code. This code may also be called when DHCP 4 or 6 isn't
   active, so we ensure that outpacket is allocated here too */
use crate::defines::{time::Instant, DhcpNetId, In6Addr, DhcpContext, CmsgHdr, DhcpBridge, MsgHdr, size_t, ModeT, DevT, __uint64_t, __u32, __uint16_t, FILE, SsizeT, __compar_fn_t, intmax_t, uintmax_t, __gwchar_t, socklen_t, DnsmasqDaemon, DhcpPacket, SOCK_RAW, IPPROTO_IPV6, ssize_t, iovec, NetAddress, C2RustUnnamed, Iname, NetAddress, DhcpOpt, RaInterface, SaFamily, ConstNetAddressArg, NetAddress};
use crate::slack::{in6_pktinfo, icmp6_filter, IPPROTO_ICMPV6, ra_packet, prefix_opt};
use std::io::{stdout, stdin};
use crate::util::{expand_buf, rand16, wildcard_match, print_mac, wildcard_matchn, setaddr6part, addr6part, retry_send, is_same_net6, whine_malloc};
use crate::network::{set_ipv6pktinfo, fix_fd, indextoname, iface_check};
use crate::dnsmasq_log::{die, my_syslog};
use crate::dhcp_common::{recv_dhcp_packet, option_filter};
use crate::lease::lease_ping_reply;
use crate::outpacket::{reset_counter, expand, put_opt6_char, put_opt6_short, put_opt6_long, put_opt6, save_counter};
use crate::netlink::iface_enumerate;
use std::time;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ra_param {
    pub now: time::Instant,
    pub ind: i32,
    pub managed: i32,
    pub other: i32,
    pub first: i32,
    pub adv_router: i32,
    pub if_name: &mut String,
    pub tags: &mut DhcpNetId,
    pub link_local: In6Addr,
    pub link_global: In6Addr,
    pub ula: In6Addr,
    pub glob_pref_time: u32,
    pub link_pref_time: u32,
    pub ula_pref_time: u32,
    pub adv_interval: u32,
    pub prio: u32,
    pub found_context: DhcpContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub c: mut Vec<u8>,
    pub p: &mut in6_pktinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub align: CmsgHdr,
    pub control6: [libc::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AliasParam {
    pub iface: i32,
    pub bridge: &mut DhcpBridge,
    pub num_alias_ifs: i32,
    pub max_alias_ifs: i32,
    pub alias_ifs: ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SearchParam {
    pub now: time::Instant,
    pub iface: i32,
    pub name: [libc::c_char; 17],
}


pub fn ra_init(mut now: time::Instant) {
    let mut filter: icmp6_filter = icmp6_filter{icmp6_filt: [0; 8],};
    let mut fd: i32 = 0;
    let mut class: i32 = 0xc0;
    let mut val: i32 = 255;
    let mut len: socklen_t =
        ::std::mem::size_of::<libc::c_int>();
    let mut context: DhcpContext = 0;
    /* ensure this is around even if we're not doing DHCPv6 */
    expand_buf(&mut daemon.outpacket,
               ::std::mem::size_of::<DhcpPacket>());
    /* See if we're guessing SLAAC addresses, if so we need to receive ping replies */
    context = daemon.dhcp6;
    while !context.is_null() {
        if context.flags &
               (1) << 6 != 0 {
            break ;
        }
        context = context.next
    }
    /* Need ICMP6 socket for transmission for DHCPv6 even when not doing RA. */
    icmp6_filter = Default::default();
    if daemon.doing_ra != 0 {
        filter.icmp6_filt[(133 >> 5) ]
            &=
            !((1) << (133 & 31))
               ;
        if !context.is_null() {
            filter.icmp6_filt[(129 >> 5) ] &=
                !((1) <<
                      (129 & 31))              libc::c_uint
        }
    }
    fd =
        socket(10, SOCK_RAW,
               IPPROTO_ICMPV6);
    if fd == -(1) ||
           getsockopt(fd, IPPROTO_IPV6, 16,
                      &mut hop_limit,
                      &mut len) != 0 ||
           setsockopt(fd, IPPROTO_IPV6, 67,
                      &mut class,
                      ::std::mem::size_of::<libc::c_int>()) == -(1) || fix_fd(fd) == 0
           || set_ipv6pktinfo(fd) == 0 ||
           setsockopt(fd, IPPROTO_IPV6, 16,
                      &mut val,
                      ::std::mem::size_of::<libc::c_int>()) != 0 ||
           setsockopt(fd, IPPROTO_IPV6, 18,
                      &mut val,
                      ::std::mem::size_of::<libc::c_int>()) != 0 ||
           setsockopt(fd, IPPROTO_ICMPV6, 1,
                      &mut filter ,
                      ::std::mem::size_of::<icmp6_filter>()
                         ) == -(1) {
        die("cannot create ICMPv6 socket: %s",
            0 , 2);
    }
    daemon.icmp6fd = fd;
    if daemon.doing_ra != 0 {
        ra_start_unsolicited(now, 0);
    };
}

pub fn ra_start_unsolicited(mut now: time::Instant,
                                              mut context:
                                                  DhcpContext) {
    /* init timers so that we do ra's for some/all soon. some ra_times will end up zeroed
     if it's not appropriate to advertise those contexts.
     This gets re-called on a netlink route-change to re-do the advertisement
     and pick up new interfaces */
    if !context.is_null() {
        context.ra_time = now; /* range 0 - 5 */
        context.ra_short_period_start = context.ra_time
    } else {
        context = daemon.dhcp6;
        while !context.is_null() {
            if context.flags &
                   (1) << 10 == 0 {
                context.ra_time =
                    now +
                        (rand16() / 13000)                      i32;
                /* re-do frequently for a minute or so, in case the first gets lost. */
                context.ra_short_period_start = now
            }
            context = context.next
        }
    };
}

pub fn icmp6_packet(mut now: time::Instant) {
    let mut interface: [libc::c_char; 17] = [0; 17];
    let mut sz: ssize_t = 0;
    let mut if_index: i32 = 0;
    let mut cmptr: CmsgHdr = 0;
    let mut msg: MsgHdr =
        MsgHdr {msg_name: 0,
               msg_namelen: 0,
               msg_iov: 0,
               msg_iovlen: 0,
               msg_control: 0,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut control_u: C2RustUnnamed_11 =
        C2RustUnnamed_11{align:
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
    let mut packet: mut Vec<u8> = 0;
    let mut tmp: Iname = 0;
    /* Note: use outpacket for input buffer */
    msg.msg_control = control_u.control6.as_mut_ptr();
    msg.msg_controllen =
        ::std::mem::size_of::<C2RustUnnamed_11>();
    msg.msg_flags = 0;
    msg.msg_name = &mut from6;
    msg.msg_namelen =
        ::std::mem::size_of::<NetAddress>();
    msg.msg_iov = &mut daemon.outpacket;
    msg.msg_iovlen = 1 ;
    sz = recv_dhcp_packet(daemon.icmp6fd, &mut msg);
    if sz == -(1) ||
           sz < 8 {
        return
    }
    packet = daemon.outpacket.iov_base;
    cmptr =
        if msg.msg_controllen >=
               ::std::mem::size_of::<CmsgHdr>() {
            msg.msg_control
        } else { 0 };
    while !cmptr.is_null() {
        if cmptr.cmsg_level == IPPROTO_IPV6 &&
               cmptr.cmsg_type == daemon.v6pktinfo {
            let mut p: C2RustUnnamed_10 =
                C2RustUnnamed_10{c: 0,};
            p.c = cmptr.__cmsg_data.as_mut_ptr();
            if_index = (*p.p).ipi6_ifindex
        }
        cmptr = __cmsg_nxthdr(&mut msg, cmptr)
    }
    if indextoname(daemon.icmp6fd, if_index,
                   interface.as_mut_ptr()) == 0 {
        return
    }
    if iface_check(1, 0 ,
                   interface.as_mut_ptr(), 0) == 0 {
        return
    }
    tmp = daemon.dhcp_except;
    while !tmp.is_null() {
        if !tmp.name.is_null() &&
               wildcard_match(tmp.name, interface.as_mut_ptr()) != 0 {
            return
        }
        tmp = tmp.next
    }
    if *packet.offset(1) !=
           0 {
        return
    }
    if *packet.offset(0) ==
           129 {
        lease_ping_reply(&mut from.sin6_addr, packet, interface.as_mut_ptr());
    } else if *packet.offset(0) ==
                  133 {
        let mut mac: &mut String =
            ""  ;
        let mut bridge: DhcpBridge = 0;
        let mut alias: DhcpBridge = 0;
        /* look for link-layer address option for logging */
        if sz >= 16 &&
               *packet.offset(8) ==
                   1 &&
               (*packet.offset(9) *
                    8 + 8) <= sz
           {
            if (*packet.offset(9) *
                    8 - 2) * 3 -
                   1 >= 1025 {
                return
            }
            print_mac(daemon.namebuff,
                      &mut *packet.offset(10),
                      *packet.offset(9)
                          * 8 - 2);
            mac = daemon.namebuff
        }
        if daemon.options[(44 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (44 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               == 0 {
            my_syslog((3) << 3 |
                          6,
                      "RTR-SOLICIT(%s) %s", interface.as_mut_ptr(), mac);
        }
        /* If the incoming interface is an alias of some other one (as
         specified by the --bridge-interface option), send an RA using
         the context of the aliased interface. */
        bridge = daemon.bridges;
        while !bridge.is_null() {
            let mut bridge_index: i32 =
                if_nametoindex(bridge.iface.as_mut_ptr());
            if bridge_index != 0 {
                alias = bridge.alias;
                while !alias.is_null() {
                    if wildcard_matchn(alias.iface.as_mut_ptr(),
                                       interface.as_mut_ptr(),
                                       16) != 0 {
                        /* Send an RA on if_index with information from
		       bridge_index. */
                        send_ra_alias(now, bridge_index,
                                      bridge.iface.as_mut_ptr(),
                                      0, if_index);
                        break ;
                    } else { alias = alias.next }
                }
                if !alias.is_null() { break ; }
            }
            bridge = bridge.next
        }
        /* If the incoming interface wasn't an alias, send an RA using
	 the context of the incoming interface. */
        if bridge.is_null() {
            /* source address may not be valid in solicit request. */
            send_ra(now, if_index, interface.as_mut_ptr(),
                    if ({
                            let mut __a: *const In6Addr =
                                &mut from.sin6_addr;
                            (__a.__in6_u.__u6_addr32[0         usize] ==
                                 0 &&
                                 __a.__in6_u.__u6_addr32[1] ==
                                     0 &&
                                 __a.__in6_u.__u6_addr32[2] ==
                                     0 &&
                                 __a.__in6_u.__u6_addr32[3] ==
                                     0)                          libc::c_int
                        }) == 0 {
                        &mut from.sin6_addr
                    } else { 0 });
        }
    };
}
fn send_ra_alias(mut now: time::Instant, mut iface: i32,
                                   mut iface_name: &mut String,
                                   mut dest: &mut In6Addr,
                                   mut send_iface: i32) {
    let mut ra: ra_packet = 0 ;
    let mut parm: ra_param =
        ra_param{now: 0,
                 ind: 0,
                 managed: 0,
                 other: 0,
                 first: 0,
                 adv_router: 0,
                 if_name: 0 ,
                 tags: 0 ,
                 link_local:
                     In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},},
                 link_global:
                     In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},},
                 ula: In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},},
                 glob_pref_time: 0,
                 link_pref_time: 0,
                 ula_pref_time: 0,
                 adv_interval: 0,
                 prio: 0,
                 found_context: 0,};
    let mut addr: NetAddress =
        NetAddress {sin6_family: 0,
                     sin6_port: 0,
                     sin6_flowinfo: 0,
                     sin6_addr:
                         In6Addr {__in6_u:
                                      C2RustUnnamed{__u6_addr8: [0; 16],},},
                     sin6_scope_id: 0,};
    let mut context: DhcpContext = 0;
    let mut tmp: DhcpContext = 0;
    let mut up: DhcpContext = 0 ;
    let mut iface_id: DhcpNetId =
        DhcpNetId {net: 0 , next: 0 ,};
    let mut opt_cfg: DhcpOpt = 0 ;
    let mut ra_param: RaInterface = find_iface_param(iface_name);
    let mut done_dns: i32 = 0;
    let mut old_prefix: i32 = 0;
    let mut mtu: i32 = 0;
    let mut min_pref_time: u32 = 0;
    let mut f: FILE = 0 ;
    parm.ind = iface;
    parm.managed = 0;
    parm.other = 0;
    parm.found_context = 0;
    parm.adv_router = 0;
    parm.if_name = iface_name;
    parm.first = 1;
    parm.now = now;
    parm.ula_pref_time = 0;
    parm.link_pref_time = parm.ula_pref_time;
    parm.glob_pref_time = parm.link_pref_time;
    parm.adv_interval = calc_interval(ra_param);
    parm.prio = calc_prio(ra_param);
    reset_counter();
    ra =
        expand(::std::mem::size_of::<ra_packet>())      ra_packet;
    if ra.is_null() { return }
    ra.type_0 = 134 as u8;
    ra.code = 0 as u8;
    ra.hop_limit = hop_limit as u8;
    ra.flags = parm.prio as u8;
    ra.lifetime = __bswap_16(calc_lifetime(ra_param) );
    ra.reachable_time = 0;
    ra.retrans_time = 0;
    /* set tag with name == interface */
    iface_id.net = iface_name;
    iface_id.next = 0 ;
    parm.tags = &mut iface_id;
    context = daemon.dhcp6;
    while !context.is_null() {
        context.flags =
            (context.flags &
                 !((1) << 5));
        context.netid.next = &mut context.netid;
        context = context.next
    }
    /* If no link-local address then we can't advertise since source address of
     advertisement must be link local address: RFC 4861 para 6.1.2. */
    if iface_enumerate(10,
                       &mut parm ,
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
                                                                               libc::c_uint,
                                                                           _:
                                                                               libc::c_uint,
                                                                           _:
                                                                              Vec<u8>)
                                                                           -> i32>,
                                               Option<fn()
                                                          ->
                                                              libc::c_int>>(Some(add_prefixes
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
                                                                                                              libc::c_uint,
                                                                                                          _:
                                                                                                              libc::c_uint,
                                                                                                          _:
                                                                                                             Vec<u8>)
                                                                                                          ->
                                                                                             libc::c_int)))
           == 0 || parm.link_pref_time == 0 {
        return
    }
    /* Find smallest preferred time within address classes,
     to use as lifetime for options. This is a rather arbitrary choice. */
    min_pref_time = 0xffffffff;
    if parm.glob_pref_time != 0 &&
           parm.glob_pref_time < min_pref_time {
        min_pref_time = parm.glob_pref_time
    }
    if parm.ula_pref_time != 0 &&
           parm.ula_pref_time < min_pref_time {
        min_pref_time = parm.ula_pref_time
    }
    if parm.link_pref_time != 0 &&
           parm.link_pref_time < min_pref_time {
        min_pref_time = parm.link_pref_time
    }
    /* Look for constructed contexts associated with addresses which have gone, 
     and advertise them with preferred_time == 0  RFC 6204 4.3 L-13 */
    up = &mut daemon.dhcp6;
    context = daemon.dhcp6;
    while !context.is_null() {
        tmp = context.next;
        if context.if_index == iface &&
               context.flags &
                   (1) << 16 != 0 {
            let mut old: u32 =
                difftime(now, context.address_lost_time);
            if old > context.saved_valid {
                /* We've advertised this enough, time to go */
                /* If this context held the timeout, and there's another context in use
		 transfer the timeout there. */
                if context.ra_time != 0 &&
                       !parm.found_context.is_null() &&
                       (*parm.found_context).ra_time ==
                           0 {
                    new_timeout(parm.found_context, iface_name, now);
                }
                *up = context.next;
                free(context);
            } else {
                let mut opt: prefix_opt = 0 ;
                let mut local: In6Addr = context.start6;
                let mut do_slaac: i32 = 0;
                old_prefix = 1;
                /* zero net part of address */
                setaddr6part(&mut local,
                             addr6part(&mut local) &
                                 !(if context.prefix == 64 {
                                       -(1long) as u64
                                   } else {
                                       ((1long) <<
                                            128 -
                                                context.prefix).wrapping_sub(1
                                                                                                                    libc::c_ulonglong)
                                   }));
                if context.flags &
                       (1) << 13 != 0 {
                    do_slaac = 1;
                    if context.flags &
                           (1) << 8 != 0 {
                        parm.other = 1;
                        if context.flags &
                               (1) << 7 == 0 {
                            parm.managed = 1
                        }
                    }
                } else if daemon.options[(37      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                         ).wrapping_mul(8                                                                   libc::c_int                                                            ))
                                                        ] &
                              (1) <<
                                  (37                          ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                     ).wrapping_mul(8                               libc::c_int                        ))
                              != 0 {
                    parm.managed = 1;
                    parm.other = 1
                }
                opt =
                    expand(::std::mem::size_of::<prefix_opt>()) ;
                if !opt.is_null() {
                    opt.type_0 = 3 as u8;
                    opt.len = 4 as u8;
                    opt.prefix_len = context.prefix as u8;
                    /* don't do RA for non-ra-only unless --enable-ra is set */
                    /* autonomous only if we're not doing dhcp, set
                     "on-link" unless "off-link" was specified */
                    opt.flags =
                        ((if do_slaac != 0 {
                              0x40
                          } else { 0 }) |
                             (if context.flags &
                                     (1) << 18
                                     != 0 {
                                  0
                              } else { 0x80 })) as u8;
                    opt.valid_lifetime =
                        __bswap_32(context.saved_valid.wrapping_sub(old));
                    opt.preferred_lifetime =
                        __bswap_32(0 );
                    opt.reserved = 0;
                    opt.prefix = local;
                    inet_ntop(10,
                              &mut local
                              daemon.addrbuff,
                              46);
                    if daemon.options[(44   libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                   ).wrapping_mul(8                                                             libc::c_int                                                      ))
                                                     ] &
                           (1) <<
                               (44                       ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                               ).wrapping_mul(8                         libc::c_int                  ))
                           == 0 {
                        my_syslog((3) << 3 |
                                      6,
                                  "RTR-ADVERT(%s) %s old prefix"                                *const u8,
                                  iface_name, daemon.addrbuff);
                    }
                }
                up = &mut context.next
            }
        } else { up = &mut context.next }
        context = tmp
    }
    /* If we're advertising only old prefixes, set router lifetime to zero. */
    if old_prefix != 0 && parm.found_context.is_null() {
        ra.lifetime = __bswap_16(0 )
    }
    /* No prefixes to advertise. */
    if old_prefix == 0 && parm.found_context.is_null() { return }
    /* If we're sending router address instead of prefix in at least on prefix,
     include the advertisement interval option. */
    if parm.adv_router != 0 {
        put_opt6_char(7);
        put_opt6_char(1);
        put_opt6_short(0);
        /* interval value is in milliseconds */
        put_opt6_long((1000                     libc::c_uint).wrapping_mul(calc_interval(find_iface_param(iface_name))));
    }
    /* Set the MTU from ra_param if any, an MTU of 0 mean automatic for linux, */
  /* an MTU of -1 prevents the option from being sent. */
    if !ra_param.is_null() { mtu = ra_param.mtu }
    /* Note that IPv6 MTU is not necessarily the same as the IPv4 MTU
     available from SIOCGIFMTU */
    if mtu == 0 {
        let mut mtu_name: &mut String =
            if !ra_param.is_null() {
                ra_param.mtu_name
            } else { 0  };
        sprintf(daemon.namebuff,
                "/proc/sys/net/ipv6/conf/%s/mtu"               *const libc::c_char,
                if !mtu_name.is_null() { mtu_name } else { iface_name });
        f =
            fopen(daemon.namebuff,
                  "r" );
        if !f.is_null() {
            if !fgets(daemon.namebuff, 1025,
                      f).is_null() {
                mtu = atoi(daemon.namebuff)
            }
            fclose(f);
        }
    }
    if mtu > 0 {
        put_opt6_char(5);
        put_opt6_char(1);
        put_opt6_short(0);
        put_opt6_long(mtu);
    }
    iface_enumerate(1,
                    &mut send_iface,
                    ::std::mem::transmute::<Option<fn(_:
                                                                            libc::c_int,
                                                                        _:
                                                                            libc::c_uint,
                                                                        _:
                                                                            &mut String,
                                                                        _:
                                                                            size_t,
                                                                        _:
                                                                           Vec<u8>)
                                                       -> i32>,
                                            Option<fn()
                                                       ->
                                                           libc::c_int>>(Some(add_lla
                                                                                                                fn(_:
                                                                                                           libc::c_int,
                                                                                                       _:
                                                                                                           libc::c_uint,
                                                                                                       _:
                                                                                                           &mut String,
                                                                                                       _:
                                                                                                           size_t,
                                                                                                       _:
                                                                                                          Vec<u8>)
                                                                                      ->
                                                                                          libc::c_int)));
    /* RDNSS, RFC 6106, use relevant DHCP6 options */
    option_filter(parm.tags, 0 ,
                  daemon.dhcp_opts6);
    let mut current_block_145: u64;
    opt_cfg = daemon.dhcp_opts6;
    while !opt_cfg.is_null() {
        let mut i: i32 = 0;
        /* netids match and not encapsulated? */
        if !(opt_cfg.flags & 4096 == 0) {
            if opt_cfg.opt == 23 {
                let mut a: In6Addr = 0;
                let mut len: i32 = 0;
                done_dns = 1;
                if opt_cfg.len == 0 {
                    current_block_145 = 5265702136860997526;
                } else {
                    /* reduce len for any addresses we can't substitute */
                    a = opt_cfg.val;
                    len = opt_cfg.len;
                    i = 0;
                    while i < opt_cfg.len {
                        if ({
                                let mut __a: *const In6Addr =
                                    a ;
                                (__a.__in6_u.__u6_addr32[0] ==
                                     0 &&
                                     __a.__in6_u.__u6_addr32[1       ]
                                         == 0
                                     &&
                                     __a.__in6_u.__u6_addr32[2       ]
                                         == 0
                                     &&
                                     __a.__in6_u.__u6_addr32[3       ]
                                         == 0)

                            }) != 0 &&
                               parm.glob_pref_time ==
                                   0 ||
                               *(a                               *const u32).offset(0
                                                                ) ==
                                   __bswap_32(0xfd000000) &&
                                   *(a                                   *const u32).offset(1                  libc::c_int
                                                                    )
                                       == 0 &&
                                   *(a                                   *const u32).offset(2                  libc::c_int
                                                                    )
                                       == 0 &&
                                   *(a                                   *const u32).offset(3                  libc::c_int
                                                                    )
                                       == 0 &&
                                   parm.ula_pref_time ==
                                       0 ||
                               *(a                               *const u32).offset(0
                                                                ) ==
                                   __bswap_32(0xfe800000) &&
                                   *(a                                   *const u32).offset(1                  libc::c_int
                                                                    )
                                       == 0 &&
                                   *(a                                   *const u32).offset(2                  libc::c_int
                                                                    )
                                       == 0 &&
                                   *(a                                   *const u32).offset(3                  libc::c_int
                                                                    )
                                       == 0 &&
                                   parm.link_pref_time ==
                                       0 {
                            len -= 16
                        }
                        i += 16;
                        a = a.offset(1)
                    }
                    if len != 0 {
                        put_opt6_char(25);
                        put_opt6_char((len / 8 +
                                           1));
                        put_opt6_short(0);
                        put_opt6_long(min_pref_time);
                        a = opt_cfg.val;
                        i = 0;
                        while i < opt_cfg.len {
                            if ({
                                    let mut __a: *const In6Addr =
                                        a ;
                                    (__a.__in6_u.__u6_addr32[0       ]
                                         == 0
                                         &&
                                         __a.__in6_u.__u6_addr32[1                     libc::c_int
                                                                                            usize]
                                             ==
                                             0
                                         &&
                                         __a.__in6_u.__u6_addr32[2                     libc::c_int
                                                                                            usize]
                                             ==
                                             0
                                         &&
                                         __a.__in6_u.__u6_addr32[3                     libc::c_int
                                                                                            usize]
                                             ==
                                             0)

                                }) != 0 {
                                if parm.glob_pref_time !=
                                       0 {
                                    put_opt6(&mut parm.link_global                                           &mut In6Addr                                          Vec<u8>,
                                             16 );
                                }
                            } else if *(a                                      *const u32).offset(0                     libc::c_int
                                                      )
                                          ==
                                          __bswap_32(0xfd000000      libc::c_uint) &&
                                          *(a                                          *const u32).offset(1                         libc::c_int
                                                              )
                                              ==
                                              0
                                          &&
                                          *(a                                          *const u32).offset(2                         libc::c_int
                                                              )
                                              ==
                                              0
                                          &&
                                          *(a                                          *const u32).offset(3                         libc::c_int
                                                              )
                                              ==
                                              0
                             {
                                if parm.ula_pref_time !=
                                       0 {
                                    put_opt6(&mut parm.ula                                          Vec<u8>,
                                             16 );
                                }
                            } else if *(a                                      *const u32).offset(0                     libc::c_int
                                                      )
                                          ==
                                          __bswap_32(0xfe800000      libc::c_uint) &&
                                          *(a                                          *const u32).offset(1                         libc::c_int
                                                              )
                                              ==
                                              0
                                          &&
                                          *(a                                          *const u32).offset(2                         libc::c_int
                                                              )
                                              ==
                                              0
                                          &&
                                          *(a                                          *const u32).offset(3                         libc::c_int
                                                              )
                                              ==
                                              0
                             {
                                if parm.link_pref_time !=
                                       0 {
                                    put_opt6(&mut parm.link_local                                           &mut In6Addr                                          Vec<u8>,
                                             16 );
                                }
                            } else {
                                put_opt6(a,
                                         16 );
                            }
                            i += 16;
                            a = a.offset(1)
                        }
                    }
                    current_block_145 = 11235674318412060590;
                }
            } else { current_block_145 = 11235674318412060590; }
            match current_block_145 {
                5265702136860997526 => { }
                _ => {
                    if opt_cfg.opt == 24 &&
                           opt_cfg.len != 0 {
                        let mut len_0: i32 =
                            (opt_cfg.len + 7) /
                                8;
                        put_opt6_char(31);
                        put_opt6_char((len_0 + 1)                                    libc::c_uint);
                        put_opt6_short(0);
                        put_opt6_long(min_pref_time);
                        put_opt6(opt_cfg.val,
                                 opt_cfg.len );
                        /* pad */
                        i = opt_cfg.len;
                        while i < len_0 * 8 {
                            put_opt6_char(0);
                            i += 1
                        }
                    }
                }
            }
        }
        opt_cfg = opt_cfg.next
    }
    if daemon.port == 53 && done_dns == 0 &&
           parm.link_pref_time != 0 {
        /* default == us, as long as we are supplying DNS service. */
        put_opt6_char(25);
        put_opt6_char(3);
        put_opt6_short(0);
        put_opt6_long(min_pref_time);
        put_opt6(&mut parm.link_local,
                 16 );
    }
    /* set managed bits unless we're providing only RA on this link */
    if parm.managed != 0 {
        ra.flags =
            (ra.flags | 0x80) as u8
    } /* M flag, managed, */
    if parm.other != 0 {
        ra.flags =
            (ra.flags | 0x40) as u8
    } /* O flag, other */
    /* decide where we're sending */
    addr.sin6_family = 10;
    addr.sin6_port = __bswap_16(IPPROTO_ICMPV6 );
    if !dest.is_null() {
        addr.sin6_addr = *dest;
        if ({
                let mut __a: *const In6Addr = dest ;
                (__a.__in6_u.__u6_addr32[0 ] &
                     __bswap_32(0xffc00000) ==
                     __bswap_32(0xfe800000))
            }) != 0 ||
               *(dest ).offset(0)             libc::c_int == 0xff &&
                   *(dest ).offset(1)
                       & 0xf ==
                       0x2 {
            addr.sin6_scope_id = iface
        }
    } else {
        inet_pton(10,
                  "FF02::1" ,
                  &mut addr.sin6_addr);
        setsockopt(daemon.icmp6fd, IPPROTO_IPV6,
                   17,
                   &mut send_iface,
                   ::std::mem::size_of::<libc::c_int>()                 socklen_t);
    }
    while retry_send(sendto(daemon.icmp6fd,
                            daemon.outpacket.iov_base,
                            save_counter(-(1)) ,
                            0,
                            ConstNetAddressArg {__NetAddress__:
                                                     &mut addr      NetAddress      NetAddress,},
                            ::std::mem::size_of::<NetAddress>()                   )) != 0 {
    };
}
fn send_ra(mut now: time::Instant, mut iface: i32,
                             mut iface_name: &mut String,
                             mut dest: &mut In6Addr) {
    /* Send an RA on the same interface that the RA content is based
     on. */
    send_ra_alias(now, iface, iface_name, dest, iface);
}
fn add_prefixes(mut local: &mut In6Addr,
                                  mut prefix: i32,
                                  mut scope: i32,
                                  mut if_index: i32,
                                  mut flags: i32,
                                  mut preferred: u32,
                                  mut valid: u32,
                                  mut vparam:Vec<u8>)
                                  -> i32 {
    let mut param: ra_param = vparam ;
    /* warning */
    if if_index == param.ind {
        if ({
                let mut __a: *const In6Addr = local ;
                (__a.__in6_u.__u6_addr32[0 ] &
                     __bswap_32(0xffc00000) ==
                     __bswap_32(0xfe800000))
            }) != 0 {
            /* Can there be more than one LL address?
	     Select the one with the longest preferred time 
	     if there is. */
            if preferred > param.link_pref_time {
                param.link_pref_time = preferred;
                param.link_local = *local
            }
        } else if ({
                       let mut __a: *const In6Addr =
                           local ;
                       (__a.__in6_u.__u6_addr32[0 ]
                            == 0 &&
                            __a.__in6_u.__u6_addr32[1        usize] ==
                                0 &&
                            __a.__in6_u.__u6_addr32[2        usize] ==
                                0 &&
                            __a.__in6_u.__u6_addr32[3        usize] ==
                                __bswap_32(1 ))                     libc::c_int
                   }) == 0 &&
                      !(*(local                        *const u8).offset(0      isize)
                            == 0xff) {
            let mut real_prefix: i32 = 0;
            let mut do_slaac: i32 = 0;
            let mut deprecate: i32 = 0;
            let mut constructed: i32 = 0;
            let mut adv_router: i32 = 0;
            let mut off_link: i32 = 0;
            let mut time: u32 = 0xffffffff;
            let mut context: DhcpContext = 0;
            let mut current_block_43: u64;
            context = daemon.dhcp6;
            while !context.is_null() {
                if context.flags &
                       ((1) << 10 |
                            (1) << 16) == 0 &&
                       prefix <= context.prefix &&
                       is_same_net6(local, &mut context.start6,
                                    context.prefix) != 0 &&
                       is_same_net6(local, &mut context.end6,
                                    context.prefix) != 0 {
                    context.saved_valid = valid;
                    if context.flags &
                           (1) << 13 != 0 {
                        do_slaac = 1;
                        if context.flags &
                               (1) << 8 != 0 {
                            param.other = 1;
                            if context.flags &
                                   (1) << 7 ==
                                   0 {
                                param.managed = 1
                            }
                        }
                        current_block_43 = 7056779235015430508;
                    } else if daemon.options[(37   ).wrapping_div((::std::mem::size_of::<libc::c_uint>()    ).wrapping_mul(8                                                                           libc::c_int                                                                    ))
                                                            ] &
                                  (1) <<
                                      (37                              ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                             ).wrapping_mul(8                                       libc::c_int                                ))
                                  == 0 {
                        current_block_43 = 10599921512955367680;
                    } else {
                        param.managed = 1;
                        param.other = 1;
                        current_block_43 = 7056779235015430508;
                    }
                    match current_block_43 {
                        10599921512955367680 => { }
                        _ => {
                            /* don't do RA for non-ra-only unless --enable-ra is set */
                            /* Configured to advertise router address, not prefix. See RFC 3775 7.2 
		 In this case we do all addresses associated with a context, 
		 hence the real_prefix setting here. */
                            if context.flags &
                                   (1) << 4 !=
                                   0 {
                                adv_router = 1;
                                param.adv_router = 1;
                                real_prefix = context.prefix
                            }
                            /* find floor time, don't reduce below 3 * RA interval.
		   If the lease time has been left as default, don't
		   use that as a floor. */
                            if context.flags &
                                   (1) << 19 !=
                                   0 && time > context.lease_time {
                                time = context.lease_time;
                                if time <
                                       (3                                      libc::c_uint).wrapping_mul(param.adv_interval)
                                   {
                                    time =
                                        (3                                       libc::c_uint).wrapping_mul(param.adv_interval)
                                }
                            }
                            if context.flags &
                                   (1) << 9 !=
                                   0 {
                                deprecate = 1
                            }
                            if context.flags &
                                   (1) << 11 !=
                                   0 {
                                constructed = 1
                            }
                            /* collect dhcp-range tags */
                            if context.netid.next ==
                                   &mut context.netid  &&
                                   !context.netid.net.is_null() {
                                context.netid.next = param.tags;
                                param.tags = &mut context.netid
                            }
                            /* subsequent prefixes on the same interface 
		   and subsequent instances of this prefix don't need timers.
		   Be careful not to find the same prefix twice with different
		   addresses unless we're advertising the actual addresses. */
                            if context.flags &
                                   (1) << 5 ==
                                   0 {
                                if param.first == 0 {
                                    context.ra_time =
                                        0
                                }
                                context.flags =
                                    (context.flags |
                                         (1) <<
                                             5);
                                real_prefix = context.prefix;
                                off_link =
                                    (context.flags &
                                         (1) <<
                                             18)
                            }
                            param.first = 0;
                            /* found_context is the _last_ one we found, so if there's 
		   more than one, it's not the first. */
                            param.found_context = context
                        }
                    }
                }
                context = context.next
            }
            /* configured time is ceiling */
            if constructed == 0 || valid > time { valid = time }
            if flags & 2 != 0 {
                preferred = 0
            }
            if deprecate != 0 { time = 0 }
            /* configured time is ceiling */
            if constructed == 0 || preferred > time { preferred = time }
            if *(local ).offset(0) &
                   __bswap_32(0xff000000) ==
                   __bswap_32(0xfd000000) {
                if preferred > param.ula_pref_time {
                    param.ula_pref_time = preferred;
                    param.ula = *local
                }
            } else if preferred > param.glob_pref_time {
                param.glob_pref_time = preferred;
                param.link_global = *local
            }
            if real_prefix != 0 {
                let mut opt: prefix_opt = 0 ;
                opt =
                    expand(::std::mem::size_of::<prefix_opt>()) ;
                if !opt.is_null() {
                    /* zero net part of address */
                    if adv_router == 0 {
                        setaddr6part(local,
                                     addr6part(local) &
                                         !(if real_prefix == 64
                                              {
                                               -(1long)u64
                                           } else {
                                               ((1long) <<
                                                    128 -
                                                        real_prefix).wrapping_sub(1
                                                                                                                        libc::c_ulonglong)
                                           }));
                    }
                    opt.type_0 = 3 as u8;
                    opt.len = 4 as u8;
                    opt.prefix_len = real_prefix as u8;
                    /* autonomous only if we're not doing dhcp, set
                     "on-link" unless "off-link" was specified */
                    opt.flags =
                        if off_link != 0 {
                            0
                        } else { 0x80 } as u8;
                    if do_slaac != 0 {
                        opt.flags =
                            (opt.flags |
                                 0x40) as u8
                    }
                    if adv_router != 0 {
                        opt.flags =
                            (opt.flags |
                                 0x20) as u8
                    }
                    opt.valid_lifetime = __bswap_32(valid);
                    opt.preferred_lifetime = __bswap_32(preferred);
                    opt.reserved = 0;
                    opt.prefix = *local;
                    inet_ntop(10, local,
                              daemon.addrbuff,
                              46);
                    if daemon.options[(44   libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                   ).wrapping_mul(8                                                             libc::c_int                                                      ))
                                                     ] &
                           (1) <<
                               (44                       ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                               ).wrapping_mul(8                         libc::c_int                  ))
                           == 0 {
                        my_syslog((3) << 3 |
                                      6,
                                  "RTR-ADVERT(%s) %s", param.if_name,
                                  daemon.addrbuff);
                    }
                }
            }
        }
    }
    return 1;
}
fn add_lla(mut index: i32, mut type_0: u32,
                             mut mac: &mut String, mut maclen: usize,
                             mut parm: Vec<u8>) -> i32 {
    if index == *(parm) {
        /* size is in units of 8 octets and includes type and length (2 bytes)
	 add 7 to round up */
        let mut len: i32 =
            (maclen.wrapping_add(9) >>
                 3);
        let mut p: Vec<u8> = Vec::new();
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = 1;
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = len;
        memcpy(p, mac, maclen);
        return 0
    }
    return 1;
}

pub fn periodic_ra(daemon: &mut DnsmasqDaemon, mut now: time::Instant) -> time::Instant {
    let mut param: SearchParam = Default::default();
    let mut context: DhcpContext;
    let mut next_event: time::Instant;
    let mut aparam: AliasParam = Default::default();
    param.now = now;
    param.iface = 0;
    loop  {
        /* find overdue events, and time of first future event */
        next_event = 0; /* overdue */
        context = daemon.dhcp6;
        while !context.is_null() {
            if context.ra_time != 0 {
                if difftime(context.ra_time, now) <= 0.0f64 { break ; }
                if next_event == 0 ||
                       difftime(next_event, context.ra_time) > 0.0f64 {
                    next_event = context.ra_time
                }
            }
            context = context.next
        }
        /* none overdue */
        if context.is_null() { break ; }
        if context.flags &
               (1) << 16 != 0 &&
               context.if_index != 0 &&
               indextoname(daemon.icmp6fd, context.if_index,
                           param.name.as_mut_ptr()) != 0 {
            /* A context for an old address. We'll not find the interface by 
	     looking for addresses, but we know it anyway, since the context is
	     constructed */
            param.iface = context.if_index;
            new_timeout(context, param.name.as_mut_ptr(), now);
        } else if iface_enumerate(10,
                                  &mut param                                Vec<u8>,
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
                                                                                      ->
                                                                         libc::c_int>,
                                                          Option<fn()
                                                                     ->
                                                                         libc::c_int>>(Some(iface_search               fn(_:
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
                                                                                                        libc::c_int)))
                      != 0 {
            /* There's a context overdue, but we can't find an interface
	   associated with it, because it's for a subnet we dont 
	   have an interface on. Probably we're doing DHCP on
	   a remote subnet via a relay. Zero the timer, since we won't
	   ever be able to send ra's and satisfy it. */
            context.ra_time = 0
        }
        if param.iface != 0 &&
               iface_check(1, 0 ,
                           param.name.as_mut_ptr(), 0) !=
                   0 {
            let mut tmp: Iname = 0;
            tmp = daemon.dhcp_except;
            while !tmp.is_null() {
                if !tmp.name.is_null() &&
                       wildcard_match(tmp.name, param.name.as_mut_ptr()) !=
                           0 {
                    break ;
                }
                tmp = tmp.next
            }
            if tmp.is_null() {
                send_ra(now, param.iface, param.name.as_mut_ptr(),
                        0);
                /* Also send on all interfaces that are aliases of this
                 one. */
                aparam.bridge = daemon.bridges;
                while !aparam.bridge.is_null() {
                    if if_nametoindex((*aparam.bridge).iface.as_mut_ptr())                     libc::c_int == param.iface {
                        /* Count the number of alias interfaces for this
                       'bridge', by calling iface_enumerate with
                       send_ra_to_aliases and NULL alias_ifs. */
                        aparam.iface = param.iface;
                        aparam.alias_ifs = 0 as ;
                        aparam.num_alias_ifs = 0;
                        iface_enumerate(1,
                                        &mut aparam ,
                                        ::std::mem::transmute::<Option<fn(_:
                                                                                                libc::c_int,
                                                                                            _:
                                                                                                libc::c_uint,
                                                                                            _:
                                                                                                &mut String,
                                                                                            _:
                                                                                                size_t,
                                                                                            _:
                                                                                               Vec<u8>)
                                                                           ->
                                                                               libc::c_int>,
                                                                Option<fn()
                                                                           ->
                                                                               libc::c_int>>(Some(send_ra_to_aliases                           fn(_:  libc::c_int,
                                                                                                                           _:  libc::c_uint,
                                                                                                                           _:  &mut String,
                                                                                                                           _:  size_t,
                                                                                                                           _: Vec<u8>)
                                                                                                          ->
                                                                                                              libc::c_int)));
                        my_syslog((3) << 3 |
                                      6,
                                  "RTR-ADVERT(%s) %s => %d alias(es)"                                *const u8,
                                  param.name.as_mut_ptr(),
                                  daemon.addrbuff,
                                  aparam.num_alias_ifs);
                        /* Allocate memory to store the alias interface
                       indices. */
                        aparam.alias_ifs =
                            whine_malloc((aparam.num_alias_ifs                                 ).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                                                 ))
                                as ;
                        if !aparam.alias_ifs.is_null() {
                            /* Use iface_enumerate again to get the alias
                           interface indices, then send on each of
                           those. */
                            aparam.max_alias_ifs = aparam.num_alias_ifs;
                            aparam.num_alias_ifs = 0;
                            iface_enumerate(1,
                                            &mut aparam                                          Vec<u8>,
                                            ::std::mem::transmute::<Option<fn(_:
                                                                                                    libc::c_int,
                                                                                                _:
                                                                                                    libc::c_uint,
                                                                                                _:
                                                                                                    &mut String,
                                                                                                _:
                                                                                                    size_t,
                                                                                                _:
                                                                                                   Vec<u8>)
                                                                               ->
                                                                                   libc::c_int>,
                                                                    Option<fn()
                                                                               ->
                                                                                   libc::c_int>>(Some(send_ra_to_aliases                                   fn(_:      libc::c_int,  _:      libc::c_uint,  _:      &mut String,  _:      size_t,  _:     Vec<u8>)
                                                                                                              ->
                                                                                                                  libc::c_int)));
                            while aparam.num_alias_ifs != 0 {
                                my_syslog((3) <<
                                              3 |
                                              6,
                                          "RTR-ADVERT(%s) %s => i/f %d"
                                                                                      *const libc::c_char,
                                          param.name.as_mut_ptr(),
                                          daemon.addrbuff,
                                          *aparam.alias_ifs.offset((aparam.num_alias_ifs
                                                                        -
                                                                        1                         libc::c_int)
                                                    ));
                                send_ra_alias(now, param.iface,
                                              param.name.as_mut_ptr(),
                                              0,
                                              *aparam.alias_ifs.offset((aparam.num_alias_ifs
                                                                            -
                                                                            1
                                                                                                            libc::c_int)
                                                            ));
                                aparam.num_alias_ifs -= 1
                            }
                            free(aparam.alias_ifs);
                        }
                        break ;
                    } else { aparam.bridge = (*aparam.bridge).next }
                }
            }
        }
    }
    return next_event;
}
fn send_ra_to_aliases(mut index: i32,
                                        mut type_0: u32,
                                        mut mac: &mut String,
                                        mut maclen: usize,
                                        mut parm:Vec<u8>)
 -> i32 {
    let mut aparam: AliasParam = parm ;
    let mut ifrn_name: [libc::c_char; 16] = [0; 16];
    let mut alias: DhcpBridge = 0;
    if !if_indextoname(index,
                       ifrn_name.as_mut_ptr()).is_null() {
        alias = (*aparam.bridge).alias;
        while !alias.is_null() {
            if wildcard_matchn(alias.iface.as_mut_ptr(),
                               ifrn_name.as_mut_ptr(), 16) != 0
               {
                if !aparam.alias_ifs.is_null() &&
                       aparam.num_alias_ifs < aparam.max_alias_ifs {
                    *aparam.alias_ifs.offset(aparam.num_alias_ifs isize) = index
                }
                aparam.num_alias_ifs += 1
            }
            alias = alias.next
        }
    }
    return 1;
}
fn iface_search(mut local: &mut In6Addr,
                                  mut prefix: i32,
                                  mut scope: i32,
                                  mut if_index: i32,
                                  mut flags: i32,
                                  mut preferred: i32,
                                  mut valid: i32,
                                  mut vparam:Vec<u8>)
                                  -> i32 {
    let mut param: SearchParam = vparam ;
    let mut context: DhcpContext = 0;
    let mut tmp: Iname = 0;
    /* ignore interfaces we're not doing DHCP on. */
    if indextoname(daemon.icmp6fd, if_index,
                   param.name.as_mut_ptr()) == 0 ||
           iface_check(1, 0 ,
                       param.name.as_mut_ptr(), 0) == 0
       {
        return 1
    }
    tmp = daemon.dhcp_except;
    while !tmp.is_null() {
        if !tmp.name.is_null() &&
               wildcard_match(tmp.name, param.name.as_mut_ptr()) != 0 {
            return 1
        }
        tmp = tmp.next
    }
    context = daemon.dhcp6;
    while !context.is_null() {
        if context.flags &
               ((1) << 10 |
                    (1) << 16) == 0 &&
               prefix <= context.prefix &&
               is_same_net6(local, &mut context.start6, context.prefix)
                   != 0 &&
               is_same_net6(local, &mut context.end6, context.prefix) !=
                   0 && context.ra_time != 0
               && difftime(context.ra_time, param.now) <= 0.0f64 {
            /* found an interface that's overdue for RA determine new 
	   timeout value and arrange for RA to be sent unless interface is
	   still doing DAD.*/
            if flags & 1 == 0 { param.iface = if_index }
            new_timeout(context, param.name.as_mut_ptr(), param.now);
            /* found, abort */
            context = context.next;
            while !context.is_null() {
                if prefix <= context.prefix &&
                       is_same_net6(local, &mut context.start6,
                                    context.prefix) != 0 &&
                       is_same_net6(local, &mut context.end6,
                                    context.prefix) != 0 {
                    context.ra_time = 0
                }
                context = context.next
            }
            return 0
        }
        context = context.next
    }
    return 1;
    /* zero timers for other contexts on the same subnet, so they don't timeout 
	   independently */
    /* keep searching */
}
fn new_timeout(mut context: DhcpContext,
                                 mut iface_name: &mut String,
                                 mut now: time::Instant) {
    if difftime(now, context.ra_short_period_start) < 60.0f64 {
        /* range 5 - 20 */
        context.ra_time =
            now + 5 +
                (rand16() / 4400)              i32
    } else {
        /* range 3/4 - 1 times MaxRtrAdvInterval */
        let mut adv_interval: u32 =
            calc_interval(find_iface_param(iface_name));
        context.ra_time =
            now +
                (3               libc::c_uint).wrapping_mul(adv_interval).wrapping_div(4
                                                                                                          libc::c_int
                                                                                                          libc::c_uint)
                    +
                (adv_interval.wrapping_mul(rand16()) >>
                     18)
    };
}
fn find_iface_param(mut iface: &mut String)
 -> RaInterface {
    let mut ra: RaInterface = 0 ;
    ra = daemon.ra_interfaces;
    while !ra.is_null() {
        if wildcard_match(ra.name, iface) != 0 { return ra }
        ra = ra.next
    }
    return 0 ;
}
fn calc_interval(mut ra: &mut RaInterface)
 -> libc::c_uint {
    let mut interval: i32 = 600;
    if !ra.is_null() && ra.interval != 0 {
        interval = ra.interval;
        if interval > 1800 {
            interval = 1800
        } else if interval < 4 { interval = 4 }
    }
    return interval;
}
fn calc_lifetime(mut ra: &mut RaInterface)
 -> libc::c_uint {
    let mut lifetime: i32 = 0;
    let mut interval: i32 = calc_interval(ra);
    if ra.is_null() || ra.lifetime == -(1) {
        /* not specified */
        lifetime = 3 * interval
    } else {
        lifetime = ra.lifetime;
        if lifetime < interval && lifetime != 0 {
            lifetime = interval
        } else if lifetime > 9000 {
            lifetime = 9000
        }
    }
    return lifetime;
}
fn calc_prio(mut ra: &mut RaInterface) -> libc::c_uint {
    if !ra.is_null() { return ra.prio }
    return 0;
}
