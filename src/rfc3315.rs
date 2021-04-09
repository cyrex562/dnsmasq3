
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
use crate::defines::{DhcpContext, In6Addr, DhcpNetId, time::Instant, DhcpVendor, SharedNetwork, __bswap_32, DnsmasqDaemon, socklen_t, C2RustUnnamed, DhcpOpt, DhcpConfig, DhcpMac, DhcpMatchName, DhcpNetIdList, AddressListEntry, DhcpLease, DhcpRelay, NetAddress, NetAddress, NetAddress, NetAddress, SaFamily, __bswap_16, IPPROTO_IPV6, NetAddress};
use crate::dhcp6::{get_client_mac, address6_valid, address6_available, address6_allocate};
use crate::util::{is_same_net6, print_mac, memcmp_masked, legal_hostname, hostname_isequal, prettyprint_time, do_rfc1035_name, rand16, setaddr6part, addr6part, wildcard_match};
use crate::dnsmasq_log::my_syslog;
use crate::outpacket::{put_opt6, new_opt6, end_opt6, save_counter, put_opt6_short, put_opt6_string, put_opt6_long, put_opt6_char, expand, reset_counter};
use crate::dhcp_common::{match_bytes, find_config, run_tag_if, strip_hostname, match_netid, log_tags, option_filter, option_string};
use crate::lease::{lease6_find, lease_set_expires, lease_set_hwaddr, lease_set_hostname, lease_prune, lease6_reset, lease6_find_by_client, lease6_find_by_addr, lease6_allocate, lease_set_iaid, lease_set_interface, lease_add_extradata};
use crate::domain::get_domain6;
use crate::forward::send_from;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub clid: mut Vec<u8>,
    pub clid_len: i32,
    pub ia_type: i32,
    pub interface: i32,
    pub hostname_auth: i32,
    pub lease_allocate: i32,
    pub client_hostname: &mut String,
    pub hostname: &mut String,
    pub domain: &mut String,
    pub send_domain: &mut String,
    pub context: DhcpContext,
    pub link_address: &mut In6Addr,
    pub fallback: &mut In6Addr,
    pub ll_addr: &mut In6Addr,
    pub ula_addr: &mut In6Addr,
    pub xid: u32,
    pub fqdn_flags: u32,
    pub iaid: u32,
    pub iface_name: &mut String,
    pub packet_options:Vec<u8>,
    pub end:Vec<u8>,
    pub tags: &mut DhcpNetId,
    pub context_tags: &mut DhcpNetId,
    pub mac: [libc::c_uchar; 16],
    pub mac_len: u32,
    pub mac_type: u32,
}

#[no_mangle]
pub unsafe extern "C" fn dhcp6_reply(mut context: DhcpContext,
                                     mut interface: i32,
                                     mut iface_name: &mut String,
                                     mut fallback: &mut In6Addr,
                                     mut ll_addr: &mut In6Addr,
                                     mut ula_addr: &mut In6Addr,
                                     mut sz: usize,
                                     mut client_addr: &mut In6Addr,
                                     mut now: time::Instant) -> u16 {
    let mut vendor: DhcpVendor = 0 ;
    let mut msg_type: i32 = 0;
    let mut state: state =
        state{clid: 0,
              clid_len: 0,
              ia_type: 0,
              interface: 0,
              hostname_auth: 0,
              lease_allocate: 0,
              client_hostname: 0 ,
              hostname: 0 ,
              domain: 0 ,
              send_domain: 0 ,
              context: 0,
              link_address: 0,
              fallback: 0,
              ll_addr: 0,
              ula_addr: 0,
              xid: 0,
              fqdn_flags: 0,
              iaid: 0,
              iface_name: 0 ,
              packet_options: 0,
              end: 0,
              tags: 0 ,
              context_tags: 0 ,
              mac: [0; 16],
              mac_len: 0,
              mac_type: 0,};
    if sz <= 4 {
        return 0
    }
    msg_type =
        *(dnsmasq_daemon.dhcp_packet.iov_base) ;
    /* Mark these so we only match each at most once, to avoid tangled linked lists */
    vendor = dnsmasq_daemon.dhcp_vendors;
    while !vendor.is_null() {
        vendor.netid.next = &mut vendor.netid;
        vendor = vendor.next
    }
    reset_counter();
    state.context = context;
    state.interface = interface;
    state.iface_name = iface_name;
    state.fallback = fallback;
    state.ll_addr = ll_addr;
    state.ula_addr = ula_addr;
    state.mac_len = 0;
    state.tags = 0 ;
    state.link_address = 0;
    if dhcp6_maybe_relay(&mut state, dnsmasq_daemon.dhcp_packet.iov_base,
                         sz, client_addr,
                         (*(client_addr                          *const u8).offset(0        ) == 0xff)                       , now) != 0 {
        return if msg_type == 12 {
                   547
               } else { 546 }
    }
    return 0 ;
}
/* This cost me blood to write, it will probably cost you blood to understand - srk. */
unsafe extern "C" fn dhcp6_maybe_relay(mut state: &mut state,
                                       mut inbuff:Vec<u8>,
                                       mut sz: usize,
                                       mut client_addr: &mut In6Addr,
                                       mut is_unicast: i32,
                                       mut now: time::Instant) -> i32 {
    let mut end:Vec<u8> = inbuff.offset(sz);
    let mut opts:Vec<u8> =
        inbuff.offset(34);
    let mut msg_type: i32 =
        *(inbuff);
    let mut outmsgtypep: mut Vec<u8> = 0;
    let mut opt:Vec<u8> = 0;
    let mut vendor: DhcpVendor = 0 ;
    /* if not an encapsulated relayed message, just do the stuff */
    if msg_type != 12 {
        /* if link_address != NULL if points to the link address field of the 
	 innermost nested RELAYFORW message, which is where we find the
	 address of the network on which we can allocate an address.
	 Recalculate the available contexts using that information. 

      link_address == NULL means there's no relay in use, so we try and find the client's 
      MAC address from the local ND cache. */
        if state.link_address.is_null() {
            get_client_mac(client_addr, state.interface,
                           state.mac.as_mut_ptr(), &mut state.mac_len,
                           &mut state.mac_type, now);
        } else {
            let mut c: DhcpContext = 0;
            let mut share: SharedNetwork = 0 ;
            state.context = 0;
            if ({
                    let mut __a: *const In6Addr =
                        state.link_address ;
                    (__a.__in6_u.__u6_addr32[0 ] ==
                         0 &&
                         __a.__in6_u.__u6_addr32[1 ]
                             == 0 &&
                         __a.__in6_u.__u6_addr32[2 ]
                             == 0 &&
                         __a.__in6_u.__u6_addr32[3 ]
                             == __bswap_32(1))
                }) == 0 &&
                   ({
                        let mut __a: *const In6Addr =
                            state.link_address ;
                        (__a.__in6_u.__u6_addr32[0 ]
                             & __bswap_32(0xffc00000) ==
                             __bswap_32(0xfe800000))
                    }) == 0 &&
                   !(*(state.link_address                     *const u8).offset(0)
                         == 0xff) {
                c = dnsmasq_daemon.dhcp6;
                while !c.is_null() {
                    share = dnsmasq_daemon.shared_networks;
                    while !share.is_null() {
                        if !(share.shared_addr.s_addr !=
                                 0) {
                            if !(share.if_index != 0 ||
                                     ({
                                          let mut __a: *const In6Addr =
                                              state.link_address                                            *const In6Addr;
                                          let mut __b: *const In6Addr =
                                              &mut share.match_addr6                                            &mut In6Addr                                            *const In6Addr;
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

                                      }) == 0) {
                                if c.flags &
                                       (1) << 8
                                       != 0 &&
                                       c.flags &
                                           ((1) <<
                                                10 |
                                                (1) <<
                                                    16) == 0 &&
                                       is_same_net6(&mut share.shared_addr6,
                                                    &mut c.start6,
                                                    c.prefix) != 0 &&
                                       is_same_net6(&mut share.shared_addr6,
                                                    &mut c.end6,
                                                    c.prefix) != 0 {
                                    break ;
                                }
                            }
                        }
                        share = share.next
                    }
                    if !share.is_null() ||
                           c.flags &
                               (1) << 8 != 0 &&
                               c.flags &
                                   ((1) << 10 |
                                        (1) <<
                                            16) == 0 &&
                               is_same_net6(state.link_address,
                                            &mut c.start6, c.prefix) !=
                                   0 &&
                               is_same_net6(state.link_address,
                                            &mut c.end6, c.prefix) != 0
                       {
                        c.valid = 0xffffffff;
                        c.preferred = c.valid;
                        c.current = state.context;
                        state.context = c
                    }
                    c = c.next
                }
            }
            if state.context.is_null() {
                inet_ntop(10,
                          state.link_address,
                          dnsmasq_daemon.addrbuff,
                          46);
                my_syslog((3) << 3 |
                              4,
                          "no address range available for DHCPv6 request from relay at %s"
                              ,
                          dnsmasq_daemon.addrbuff);
                return 0
            }
        }
        if state.context.is_null() {
            my_syslog((3) << 3 |
                          4,
                      "no address range available for DHCPv6 request via %s"
                          ,
                      state.iface_name);
            return 0
        }
        return dhcp6_no_relay(state, msg_type, inbuff, sz, is_unicast, now)
    }
    /* must have at least msg_type+hopcount+link_address+peer_address+minimal size option
     which is               1   +    1   +    16      +     16     + 2 + 2 = 38 */
    if sz < 38 { return 0 }
    /* copy header stuff into reply message and set type to reply */
    outmsgtypep =
        put_opt6(inbuff, 34 );
    if outmsgtypep.is_null() { return 0 }
    *outmsgtypep = 13;
    let mut current_block_36: u64;
    /* look for relay options and set tags if found. */
    vendor = dnsmasq_daemon.dhcp_vendors;
    while !vendor.is_null() {
        let mut mopt: i32 = 0;
        if vendor.match_type == 5 {
            mopt = 38;
            current_block_36 = 2543120759711851213;
        } else if vendor.match_type == 4 {
            mopt = 37;
            current_block_36 = 2543120759711851213;
        } else { current_block_36 = 4090602189656566074; }
        match current_block_36 {
            2543120759711851213 => {
                opt =
                    opt6_find(opts, end, mopt,
                              1);
                if !opt.is_null() &&
                       vendor.len ==
                           opt6_uint(opt,
                                     -(2), 2) &&
                       memcmp(vendor.data,
                              &mut *(opt                                   mut Vec<u8>).offset((4
                                                                         +
                                                                         0                          )
                                                      )
                                 ,
                              vendor.len) ==
                           0 &&
                       vendor.netid.next !=
                           &mut vendor.netid  {
                    vendor.netid.next = state.tags;
                    state.tags = &mut vendor.netid;
                    break ;
                }
            }
            _ => { }
        }
        vendor = vendor.next
    }
    /* RFC-6939 */
    opt =
        opt6_find(opts, end, 79,
                  3);
    if !opt.is_null() {
        if opt6_uint(opt, -(2),
                     2) - 2 >
               16 {
            return 0
        }
        state.mac_type =
            opt6_uint(opt, 0,
                      2);
        state.mac_len =
            (opt6_uint(opt, -(2),
                       2) - 2)          libc::c_uint;
        memcpy(&mut *state.mac.as_mut_ptr().offset(0       )             mut Vec<u8>,
               &mut *(opt).offset((4 +
                                                          2)      )             mut Vec<u8>,
               state.mac_len);
    }
    opt = opts;
    while !opt.is_null() {
        if (&mut *(opt                 mut Vec<u8>).offset((4 +
                                                       0)   )          mut Vec<u8>         Vec<u8>).offset(opt6_uint(opt,
                                                    -(2),
                                                    2)                                        ) > end {
            return 0
        }
        /* Don't copy MAC address into reply. */
        if opt6_uint(opt, -(4),
                     2) != 79 {
            let mut o: i32 =
                new_opt6(opt6_uint(opt,
                                   -(4), 2)                       );
            if opt6_uint(opt, -(4),
                         2) == 9
               {
                let mut align: In6Addr =
                    In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
                /* the packet data is unaligned, copy to aligned storage */
                memcpy(&mut align,
                       inbuff.offset(2),
                       16);
                state.link_address = &mut align;
                /* zero is_unicast since that is now known to refer to the 
		 relayed packet, not the original sent by the client */
                if dhcp6_maybe_relay(state,
                                     &mut *(opt                                          mut Vec<u8>).offset((4

                                                                                +
                                                                                0
                                                                                                                    )
                                                                    )
                                                                          Vec<u8>,
                                     opt6_uint(opt,
                                               -(2),
                                               2)                                    , client_addr,
                                     0, now) == 0 {
                    return 0
                }
            } else {
                put_opt6(&mut *(opt                              mut Vec<u8>).offset((4
                                                                    +
                                                                    0                     )
                                                                  )
                            ,
                         opt6_uint(opt,
                                   -(2), 2)                                              usize); /* default to send if we receive no FQDN option */
            }
            end_opt6(o);
        }
        opt = opt6_next(opt, end)
    }
    return 1;
}
unsafe extern "C" fn dhcp6_no_relay(mut state: &mut state,
                                    mut msg_type: i32,
                                    mut inbuff:Vec<u8>,
                                    mut sz: usize,
                                    mut is_unicast: i32,
                                    mut now: time::Instant) -> i32 {
    let mut opt:Vec<u8> = 0;
    let mut i: i32 = 0;
    let mut o: i32 = 0;
    let mut o1: i32 = 0;
    let mut start_opts: i32 = 0;
    let mut opt_cfg: DhcpOpt = 0 ;
    let mut tagif: DhcpNetId = 0 ;
    let mut config: DhcpConfig = 0;
    let mut known_id: DhcpNetId =
        DhcpNetId {net: 0 , next: 0 ,};
    let mut iface_id: DhcpNetId =
        DhcpNetId {net: 0 , next: 0 ,};
    let mut v6_id: DhcpNetId =
        DhcpNetId {net: 0 , next: 0 ,};
    let mut outmsgtypep: mut Vec<u8> = 0;
    let mut vendor: DhcpVendor = 0 ;
    let mut context_tmp: DhcpContext = 0;
    let mut mac_opt: DhcpMac = 0 ;
    let mut ignore: u32 = 0;
    state.packet_options = inbuff.offset(4);
    state.end = inbuff.offset(sz);
    state.clid = 0;
    state.clid_len = 0;
    state.lease_allocate = 0;
    state.context_tags = 0 ;
    state.domain = 0 ;
    state.send_domain = 0 ;
    state.hostname_auth = 0;
    state.hostname = 0 ;
    state.client_hostname = 0 ;
    state.fqdn_flags = 0x1;
    /* set tag with name == interface */
    iface_id.net = state.iface_name;
    iface_id.next = state.tags;
    state.tags = &mut iface_id;
    /* set tag "dhcpv6" */
    v6_id.net =
        "dhcpv6"       &mut String;
    v6_id.next = state.tags;
    state.tags = &mut v6_id;
    /* copy over transaction-id, and save pointer to message type */
    outmsgtypep =
        put_opt6(inbuff, 4 );
    if outmsgtypep.is_null() { return 0 }
    start_opts = save_counter(-(1));
    state.xid =
        (*outmsgtypep.offset(3) |
             (*outmsgtypep.offset(2))
                 << 8 |
             (*outmsgtypep.offset(1))
                 << 16);
    /* We're going to be linking tags from all context we use. 
     mark them as unused so we don't link one twice and break the list */
    context_tmp = state.context;
    while !context_tmp.is_null() {
        context_tmp.netid.next = &mut context_tmp.netid;
        if dnsmasq_daemon.options[(28 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                                                   ))
                                         ] &
               (1) <<
                   (28 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8
                                                                                                                       ))
               != 0 {
            inet_ntop(10,
                      &mut context_tmp.start6 , dnsmasq_daemon.dhcp_buff,
                      46);
            inet_ntop(10,
                      &mut context_tmp.end6 , dnsmasq_daemon.dhcp_buff2,
                      46);
            if context_tmp.flags &
                   (1) << 0 != 0 {
                my_syslog((3) << 3 |
                              6,
                          "%u available DHCPv6 subnet: %s/%d"                        *const u8, state.xid,
                          dnsmasq_daemon.dhcp_buff, context_tmp.prefix);
            } else {
                my_syslog((3) << 3 |
                              6,
                          "%u available DHCP range: %s -- %s"                        *const u8, state.xid,
                          dnsmasq_daemon.dhcp_buff,
                          dnsmasq_daemon.dhcp_buff2);
            }
        }
        context_tmp = context_tmp.current
    }
    opt =
        opt6_find(state.packet_options, state.end,
                  1,
                  1);
    if !opt.is_null() {
        state.clid =
            &mut *(opt                 mut Vec<u8>).offset((4 +
                                                       0)   )          mut Vec<u8>;
        state.clid_len =
            opt6_uint(opt, -(2),
                      2);
        o = new_opt6(1);
        put_opt6(state.clid,
                 state.clid_len );
        end_opt6(o);
    } else if msg_type != 11 { return 0 }
    /* server-id must match except for SOLICIT, CONFIRM and REBIND messages */
    if msg_type != 1 && msg_type != 4 &&
           msg_type != 11 && msg_type != 6 &&
           {
               opt =
                   opt6_find(state.packet_options, state.end,
                             2,
                             1);
               (opt.is_null() ||
                    opt6_uint(opt, -(2),
                              2) !=
                        dnsmasq_daemon.duid_len) ||
                   memcmp(&mut *(opt                               mut Vec<u8>).offset((4
                                                                     +
                                                                     0                      )
                                                                   )
                             ,
                          dnsmasq_daemon.duid,
                          dnsmasq_daemon.duid_len) !=
                       0
           } {
        return 0
    }
    o = new_opt6(2);
    put_opt6(dnsmasq_daemon.duid,
             dnsmasq_daemon.duid_len );
    end_opt6(o);
    if is_unicast != 0 &&
           (msg_type == 3 || msg_type == 5 ||
                msg_type == 8 || msg_type == 9)
       {
        *outmsgtypep = 7;
        o1 = new_opt6(13);
        put_opt6_short(5);
        put_opt6_string("Use multicast"  );
        end_opt6(o1);
        return 1
    }
    let mut current_block_64: u64;
    /* match vendor and user class options */
    vendor = dnsmasq_daemon.dhcp_vendors;
    while !vendor.is_null() {
        let mut mopt: i32 = 0;
        if vendor.match_type == 1 {
            mopt = 16;
            current_block_64 = 6560072651652764009;
        } else if vendor.match_type == 2 {
            mopt = 15;
            current_block_64 = 6560072651652764009;
        } else { current_block_64 = 17747245473264231573; }
        match current_block_64 {
            6560072651652764009 => {
                opt =
                    opt6_find(state.packet_options, state.end,
                              mopt,
                              2);
                if !opt.is_null() {
                    let mut enc_opt:Vec<u8> =
                        0;
                    let mut enc_end:Vec<u8> =
                        &mut *(opt                             mut Vec<u8>).offset((4
                                                                   +
                                                                   (opt6_uint
                                                                                            unsafe extern "C" fn(_:
                                                                                                 mut Vec<u8>,
                                                                                             _:
                                                                                                 ,
                                                                                             _:
                                                                                                 )
                                                                            ->
                                                                                libc::c_uint)(opt                   mut Vec<u8>,
                                                                                              -(2                       ),
                                                                                              2                   )
                                                                                          )
                                                                 )                      mut Vec<u8>;
                    let mut offset: i32 = 0;
                    if mopt == 16 {
                        if (opt6_uint(opt,
                                      -(2), 2)
                               ) < 4 {
                            current_block_64 = 17747245473264231573;
                        } else if vendor.enterprise !=
                                      opt6_uint(opt,
                                                0,
                                                4) {
                            current_block_64 = 17747245473264231573;
                        } else {
                            offset = 4;
                            current_block_64 = 307447392441238883;
                        }
                    } else { current_block_64 = 307447392441238883; }
                    match current_block_64 {
                        17747245473264231573 => { }
                        _ => {
                            /* Note that format if user/vendor classes is different to DHCP options - no option types. */
                            enc_opt =
                                &mut *(opt                                     mut Vec<u8>).offset((4
                                                                           +
                                                                           offset)
                                                          )
                                                                Vec<u8>;
                            while !enc_opt.is_null() {
                                i = 0;
                                while i <=
                                          opt6_uint(enc_opt     mut Vec<u8>,
                                                    -(4),
                                                    2)                                         - vendor.len {
                                    if memcmp(vendor.data
                                              &mut *(enc_opt      mut Vec<u8>).offset((2
                                                                                         +
                                                                                         i)
                                                                                      )
                                                                                            Vec<u8>,
                                              vendor.len)
                                           == 0 {
                                        vendor.netid.next = state.tags;
                                        state.tags = &mut vendor.netid;
                                        break ;
                                    } else { i += 1 }
                                }
                                enc_opt =
                                    opt6_next(enc_opt.offset(-(2
                                                                  )),
                                              enc_end)
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        vendor = vendor.next
    }
    if dnsmasq_daemon.options[(28).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                                                   ))
                                     ] &
           (1) <<
               (28).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8

                                                                                                               ))
           != 0 &&
           {
               opt =
                   opt6_find(state.packet_options, state.end,
                             16,
                             4);
               !opt.is_null()
           } {
        my_syslog((3) << 3 | 6,
                  "%u vendor class: %u", state.xid,
                  opt6_uint(opt, 0,
                            4));
    }
    let mut current_block_78: u64;
    /* dhcp-match. If we have hex-and-wildcards, look for a left-anchored match.
     Otherwise assume the option is an array, and look for a matching element. 
     If no data given, existence of the option is enough. This code handles 
     V-I opts too. */
    opt_cfg = dnsmasq_daemon.dhcp_match6;
    while !opt_cfg.is_null() {
        let mut match_0: i32 = 0;
        if opt_cfg.flags & 2048 != 0 {
            opt =
                opt6_find(state.packet_options, state.end,
                          17,
                          4);
            while !opt.is_null() {
                let mut vopt:Vec<u8> = 0;
                let mut vend:Vec<u8> =
                    &mut *(opt).offset((4 +
                                                               (opt6_uint                 unsafe extern "C" fn(_:
                                                                                             mut Vec<u8>,
                                                                                         _:
                                                                                             ,
                                                                                         _:
                                                                                             )
                                                                        ->
                                                                            libc::c_uint)(opt           mut Vec<u8>,
                                                                                          -(2               ),
                                                                                          2           )
                                                                                  )
                                                             )                  mut Vec<u8>;
                vopt =
                    opt6_find(&mut *(opt                                   mut Vec<u8>).offset((4
                                                                         +
                                                                         4                          )
                                                      )
                                 ,
                              vend, opt_cfg.opt,
                              0);
                while !vopt.is_null() {
                    match_0 =
                        match_bytes(opt_cfg,
                                    &mut *(vopt                                         mut Vec<u8>).offset((4

                                                                               +
                                                                               0
                                                                                                                  )
                                                                  )
                                                                        Vec<u8>                                  mut Vec<u8>,
                                    opt6_uint(vopt,
                                              -(2),
                                              2) );
                    if match_0 != 0 { break ; }
                    vopt =
                        opt6_find(opt6_next(vopt, vend), vend,
                                  opt_cfg.opt,
                                  0)
                }
                opt =
                    opt6_find(opt6_next(opt, state.end), state.end,
                              17,
                              4)
            }
            if match_0 != 0 { break ; }
            current_block_78 = 2616667235040759262;
        } else {
            opt =
                opt6_find(state.packet_options, state.end,
                          opt_cfg.opt,
                          1);
            if opt.is_null() {
                current_block_78 = 5793491756164225964;
            } else {
                match_0 =
                    match_bytes(opt_cfg,
                                &mut *(opt                                     mut Vec<u8>).offset((4
                                                                           +
                                                                           0
                                                                                                          )
                                                          )

                                   ,
                                opt6_uint(opt,
                                          -(2),
                                          2));
                current_block_78 = 2616667235040759262;
            }
        }
        match current_block_78 {
            2616667235040759262 => {
                if match_0 != 0 {
                    (*opt_cfg.netid).next = state.tags;
                    state.tags = opt_cfg.netid
                }
            }
            _ => { }
        }
        opt_cfg = opt_cfg.next
    }
    if state.mac_len != 0 {
        if dnsmasq_daemon.options[(28 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                                                   ))
                                         ] &
               (1) <<
                   (28 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8
                                                                                                                       ))
               != 0 {
            print_mac(dnsmasq_daemon.dhcp_buff, state.mac.as_mut_ptr(),
                      state.mac_len);
            my_syslog((3) << 3 |
                          6,
                      "%u client MAC address: %s", state.xid,
                      dnsmasq_daemon.dhcp_buff);
        }
        mac_opt = dnsmasq_daemon.dhcp_macs;
        while !mac_opt.is_null() {
            if mac_opt.hwaddr_len == state.mac_len &&
                   (mac_opt.hwaddr_type ==
                        state.mac_type ||
                        mac_opt.hwaddr_type == 0) &&
                   memcmp_masked(mac_opt.hwaddr.as_mut_ptr(),
                                 state.mac.as_mut_ptr(),
                                 state.mac_len,
                                 mac_opt.mask) != 0 {
                mac_opt.netid.next = state.tags;
                state.tags = &mut mac_opt.netid
            }
            mac_opt = mac_opt.next
        }
    }
    opt =
        opt6_find(state.packet_options, state.end,
                  39,
                  1);
    if !opt.is_null() {
        /* RFC4704 refers */
        let mut len: i32 =
            opt6_uint(opt, -(2),
                      2) - 1;
        state.fqdn_flags =
            opt6_uint(opt, 0,
                      1);
        /* Always force update, since the client has no way to do it itself. */
        if dnsmasq_daemon.options[(36 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                                                   ))
                                         ] &
               (1) <<
                   (36 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8
                                                                                                                       ))
               == 0 &&
               state.fqdn_flags & 0x1 == 0 {
            state.fqdn_flags |= 0x3
        }
        state.fqdn_flags &= !(0x4);
        if len != 0 && len < 255 {
            let mut pp: mut Vec<u8> = 0;
            let mut op: mut Vec<u8> =
                &mut *(opt                     mut Vec<u8>).offset((4 +
                                                           1)
                                                         )              mut Vec<u8>              mut Vec<u8>;
            let mut pq: &mut String = dnsmasq_daemon.dhcp_buff;
            pp = op;
            while *op != 0 &&
                      (op.offset(*op ).wrapping_offset_from(pp)                     i32) < len {
                memcpy(pq,
                       op.offset(1)    *op);
                pq = pq.offsetop;
                op =
                    op.offset((*op + 1)                            );
                let fresh6 = pq;
                pq = pq.offset(1);
                *fresh6 = '.'
            }
            if pq != dnsmasq_daemon.dhcp_buff { pq = pq.offset(-1) }
            *pq = 0;
            if legal_hostname(dnsmasq_daemon.dhcp_buff) != 0 {
                let mut m: DhcpMatchName = 0 ;
                let mut nl: usize = strlen(dnsmasq_daemon.dhcp_buff);
                state.client_hostname = dnsmasq_daemon.dhcp_buff;
                if dnsmasq_daemon.options[(28 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                           ).wrapping_mul(8                                                                                                   ))
                                                 ] &
                       (1) <<
                           (28                   ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                       ).wrapping_mul(8                           ))
                       != 0 {
                    my_syslog((3) << 3 |
                                  6,
                              "%u client provides name: %s"    , state.xid,
                              state.client_hostname);
                }
                m = dnsmasq_daemon.dhcp_name_match;
                while !m.is_null() {
                    let mut ml: usize = strlen(m.name);
                    let mut save: libc::c_char =
                        0;
                    if !(nl < ml) {
                        if nl > ml {
                            save =
                                *state.client_hostname.offset(ml);
                            *state.client_hostname.offset(ml) =
                                0
                        }
                        if hostname_isequal(state.client_hostname,
                                            m.name) != 0 &&
                               (save == 0 ||
                                    m.wildcard != 0) {
                            (*m.netid).next = state.tags;
                            state.tags = m.netid
                        }
                        if save != 0 {
                            *state.client_hostname.offset(ml) =
                                save
                        }
                    }
                    m = m.next
                }
            }
        }
    }
    if !state.clid.is_null() &&
           {
               config =
                   find_config(dnsmasq_daemon.dhcp_conf, state.context,
                               state.clid, state.clid_len,
                               state.mac.as_mut_ptr(),
                               state.mac_len,
                               state.mac_type,
                               0 ,
                               run_tag_if(state.tags));
               !config.is_null()
           } &&
           (!config.is_null() &&
                config.flags & 16 != 0) {
        state.hostname = config.hostname;
        state.domain = config.domain;
        state.hostname_auth = 1
    } else if !state.client_hostname.is_null() {
        state.domain = strip_hostname(state.client_hostname);
        if strlen(state.client_hostname) !=
               0 {
            state.hostname = state.client_hostname;
            if config.is_null() {
                /* Search again now we have a hostname. 
		 Only accept configs without CLID here, (it won't match)
		 to avoid impersonation by name. */
                let mut new: DhcpConfig =
                    find_config(dnsmasq_daemon.dhcp_conf, state.context,
                                0, 0,
                                0, 0,
                                0, state.hostname,
                                run_tag_if(state.tags));
                if !new.is_null() &&
                       !(!new.is_null() &&
                             new.flags & 2
                                 != 0) && new.hwaddr.is_null() {
                    config = new
                }
            }
        }
    }
    if !config.is_null() {
        let mut list: DhcpNetIdList = 0;
        list = config.netid;
        while !list.is_null() {
            (*list.list).next = state.tags;
            state.tags = list.list;
            list = list.next
        }
        /* set "known" tag for known hosts */
        known_id.net =
            "known"           &mut String;
        known_id.next = state.tags;
        state.tags = &mut known_id;
        if !config.is_null() &&
               config.flags & 1 != 0 {
            ignore = 1
        }
    } else if !state.clid.is_null() &&
                  !find_config(dnsmasq_daemon.dhcp_conf,
                               0, state.clid,
                               state.clid_len, state.mac.as_mut_ptr(),
                               state.mac_len,
                               state.mac_type,
                               0 ,
                               run_tag_if(state.tags)).is_null() {
        known_id.net =
            "known-othernet"           &mut String;
        known_id.next = state.tags;
        state.tags = &mut known_id
    }
    tagif = run_tag_if(state.tags);
    /* if all the netids in the ignore list are present, ignore this client */
    if !dnsmasq_daemon.dhcp_ignore.is_null() {
        let mut id_list: DhcpNetIdList = 0;
        id_list = dnsmasq_daemon.dhcp_ignore;
        while !id_list.is_null() {
            if match_netid(id_list.list, tagif, 0) != 0 {
                ignore = 1
            }
            id_list = id_list.next
        }
    }
    /* if all the netids in the ignore_name list are present, ignore client-supplied name */
    if state.hostname_auth == 0 {
        let mut id_list_0: DhcpNetIdList = 0;
        id_list_0 = dnsmasq_daemon.dhcp_ignore_names;
        while !id_list_0.is_null() {
            if id_list_0.list.is_null() ||
                   match_netid(id_list_0.list, tagif, 0) !=
                       0 {
                break ;
            }
            id_list_0 = id_list_0.next
        }
        if !id_list_0.is_null() { state.hostname = 0  }
    }
    let mut address_assigned: i32 = 0;
    let mut solicit_tags: DhcpNetId = 0 ;
    let mut c: DhcpContext = 0;
    let mut current_block_486: u64;
    match msg_type {
        1 => {
            address_assigned = 0;
            /* tags without all prefix-class tags */
            solicit_tags = 0 ;
            c = 0;
            *outmsgtypep = 2;
            if !opt6_find(state.packet_options, state.end,
                          14,
                          0).is_null() {
                *outmsgtypep = 7;
                state.lease_allocate = 1;
                o = new_opt6(14);
                end_opt6(o);
            }
            log6_quiet(state,
                       "DHCPSOLICIT"
                           , 0,
                       if ignore != 0 {
                           "ignored"
                       } else { 0 }                     &mut String);
            current_block_486 = 15319502457978536222;
        }
        3 => {
            let mut address_assigned_0: i32 = 0;
            let mut start: i32 = save_counter(-(1));
            /* set reply message type */
            *outmsgtypep = 7;
            state.lease_allocate = 1;
            log6_quiet(state,
                       "DHCPREQUEST"
                           , 0,
                       if ignore != 0 {
                           "ignored"
                       } else { 0 }                     &mut String);
            if ignore != 0 { return 0 }
            opt = state.packet_options;
            loop  {
                if opt.is_null() {
                    current_block_486 = 309319537768397308;
                    break ;
                }
                let mut ia_option_0:Vec<u8> =
                    0;
                let mut ia_end_0:Vec<u8> = 0;
                let mut min_time_0: u32 = 0xffffffff;
                let mut t1cntr_0: i32 = 0;
                if !(check_ia(state, opt, &mut ia_end_0, &mut ia_option_0) ==
                         0) {
                    if ia_option_0.is_null() {
                        /* If we get a request with an IA_*A without addresses, treat it exactly like
		    a SOLICT with rapid commit set. */
                        save_counter(start);
                        current_block_486 = 15319502457978536222;
                        break ;
                    } else {
                        o = build_ia(state, &mut t1cntr_0);
                        while !ia_option_0.is_null() {
                            let mut req_addr_0: In6Addr =
                                In6Addr {__in6_u:
                                             C2RustUnnamed{__u6_addr8:
                                                               [0; 16],},};
                            let mut dynamic: DhcpContext =
                                0;
                            let mut c_0: DhcpContext =
                                0;
                            let mut lease_time_0: u32 = 0;
                            let mut config_ok: i32 = 0;
                            /* align. */
                            memcpy(&mut req_addr_0                                Vec<u8>,
                                   &mut *(ia_option_0                                        mut Vec<u8>).offset((4
                                                                              +
                                                                              0
                                                                                                                )
                                                                )
                                                                      Vec<u8>,
                                   16);
                            c_0 =
                                address6_valid(state.context,
                                               &mut req_addr_0, tagif,
                                               1);
                            if !c_0.is_null() {
                                config_ok =
                                    (config_implies(config, c_0,
                                                    &mut req_addr_0) !=
                                         0                                        &mut AddrList)
                            }
                            dynamic =
                                address6_available(state.context,
                                                   &mut req_addr_0, tagif,
                                                   1);
                            if !dynamic.is_null() || !c_0.is_null() {
                                if dynamic.is_null() && config_ok == 0 {
                                    /* Static range, not configured. */
                                    o1 = new_opt6(13);
                                    put_opt6_short(2);
                                    put_opt6_string("address unavailable"
                                                             *const libc::c_char     &mut String);
                                    end_opt6(o1);
                                } else if check_address(state,
                                                        &mut req_addr_0) == 0
                                 {
                                    /* Address leased to another DUID/IAID */
                                    o1 = new_opt6(13);
                                    put_opt6_short(1);
                                    put_opt6_string("address in use"     *const u8     *const libc::c_char     &mut String);
                                    end_opt6(o1);
                                } else {
                                    if dynamic.is_null() { dynamic = c_0 }
                                    lease_time_0 = dynamic.lease_time;
                                    if config_ok != 0 &&
                                           (!config.is_null() &&
                                                config.flags &
                                                    8     libc::c_uint != 0) {
                                        lease_time_0 = config.lease_time
                                    }
                                    add_address(state, dynamic, lease_time_0,
                                                ia_option_0, &mut min_time_0,
                                                &mut req_addr_0, now);
                                    get_context_tag(state, dynamic);
                                    address_assigned_0 = 1
                                }
                            } else {
                                /* requested address not on the correct link */
                                o1 = new_opt6(13);
                                put_opt6_short(4libc::c_uint);
                                put_opt6_string("not on link" *const u8 *const libc::c_char &mut String);
                                end_opt6(o1);
                            }
                            ia_option_0 =
                                opt6_find(opt6_next(ia_option_0, ia_end_0),
                                          ia_end_0,
                                          5,
                                          24)
                        }
                        end_ia(t1cntr_0, min_time_0, 0);
                        end_opt6(o);
                    }
                }
                opt = opt6_next(opt, state.end)
            }
            match current_block_486 {
                15319502457978536222 => { }
                _ => {
                    if address_assigned_0 != 0 {
                        o1 = new_opt6(13);
                        put_opt6_short(0);
                        put_opt6_string("success"                                       *const libc::c_char                                      &mut String);
                        end_opt6(o1);
                    } else {
                        /* no address, return error */
                        o1 = new_opt6(13);
                        put_opt6_short(2);
                        put_opt6_string("no addresses available"                                      *const u8
                                            );
                        end_opt6(o1);
                        log6_packet(state,
                                    "DHCPREPLY"                                   *const libc::c_char                                  &mut String, 0,
                                    "no addresses available"                                            &mut String);
                    }
                    tagif = add_options(state, 0);
                    current_block_486 = 14838758841813985983;
                }
            }
        }
        5 => {
            /* set reply message type */
            *outmsgtypep = 7;
            log6_quiet(state,
                       "DHCPRENEW"                      &mut String, 0,
                       0 );
            opt = state.packet_options;
            while !opt.is_null() {
                let mut ia_option_1:Vec<u8> =
                    0;
                let mut ia_end_1:Vec<u8> = 0;
                let mut min_time_1: u32 = 0xffffffff;
                let mut t1cntr_1: i32 = 0;
                let mut iacntr: i32 = 0;
                if !(check_ia(state, opt, &mut ia_end_1, &mut ia_option_1) ==
                         0) {
                    o = build_ia(state, &mut t1cntr_1);
                    iacntr = save_counter(-(1));
                    while !ia_option_1.is_null() {
                        let mut lease: DhcpLease = 0;
                        let mut req_addr_1: In6Addr =
                            In6Addr {__in6_u:
                                         C2RustUnnamed{__u6_addr8:
                                                           [0; 16],},};
                        let mut preferred_time: u32 =
                            opt6_uint(ia_option_1,
                                      16, 4);
                        let mut valid_time: u32 =
                            opt6_uint(ia_option_1,
                                      20, 4);
                        let mut message: &mut String =
                            0 ;
                        let mut this_context: DhcpContext =
                            0;
                        memcpy(&mut req_addr_1                            Vec<u8>,
                               &mut *(ia_option_1                                    mut Vec<u8>).offset((4
                                                                          +
                                                                          0                           )
                                                        )
                                  ,
                               16);
                        lease =
                            lease6_find(state.clid, state.clid_len,
                                        if state.ia_type ==
                                               3 {
                                            32
                                        } else { 64 },
                                        state.iaid, &mut req_addr_1);
                        if lease.is_null() {
                            /* If the server cannot find a client entry for the IA the server
		       returns the IA containing no addresses with a Status Code option set
		       to NoBinding in the Reply message. */
                            save_counter(iacntr);
                            t1cntr_1 = 0;
                            log6_packet(state,
                                        "DHCPREPLY"                                       *const libc::c_char                                      &mut String,
                                        &mut req_addr_1,
                                        "lease not found"                                       *const libc::c_char                                      &mut String);
                            o1 = new_opt6(13);
                            put_opt6_short(3);
                            put_opt6_string("no binding found"                                          *const u8                                          *const libc::c_char                                          &mut String);
                            end_opt6(o1);
                            valid_time = 0;
                            preferred_time = valid_time;
                            break ;
                        } else {
                            this_context =
                                address6_available(state.context,
                                                   &mut req_addr_1, tagif,
                                                   1);
                            if !this_context.is_null() ||
                                   {
                                       this_context =
                                           address6_valid(state.context,
                                                          &mut req_addr_1,
                                                          tagif,
                                                          1);
                                       !this_context.is_null()
                                   } {
                                let mut lease_time_1: u32 = 0;
                                get_context_tag(state, this_context);
                                if !config_implies(config, this_context,
                                                   &mut req_addr_1).is_null()
                                       &&
                                       (!config.is_null() &&
                                            config.flags &
                                                8 libc::c_uint != 0) {
                                    lease_time_1 = config.lease_time
                                } else {
                                    lease_time_1 = this_context.lease_time
                                }
                                calculate_times(this_context, &mut min_time_1,
                                                &mut valid_time,
                                                &mut preferred_time,
                                                lease_time_1);
                                lease_set_expires(lease, valid_time, now);
                                /* Update MAC record in case it's new information. */
                                if state.mac_len !=
                                       0 {
                                    lease_set_hwaddr(lease,
                                                     state.mac.as_mut_ptr(),
                                                     state.clid,
                                                     state.mac_len      ,
                                                     state.mac_type      ,
                                                     state.clid_len, now,
                                                     0);
                                }
                                if state.ia_type == 3 &&
                                       !state.hostname.is_null() {
                                    let mut addr_domain: &mut String =
                                        get_domain6(&mut req_addr_1);
                                    if state.send_domain.is_null() {
                                        state.send_domain = addr_domain
                                    }
                                    lease_set_hostname(lease,
                                                       state.hostname,
                                                       state.hostname_auth,
                                                       addr_domain,
                                                       state.domain);
                                    message = state.hostname
                                }
                                if preferred_time ==
                                       0 {
                                    message =
                                        "deprecated"                                       *const libc::c_char                                      &mut String
                                }
                            } else {
                                valid_time = 0;
                                preferred_time = valid_time;
                                message =
                                    "address invalid"                                   *const libc::c_char                                  &mut String
                            }
                            if !message.is_null() &&
                                   message != state.hostname {
                                log6_packet(state,
                                            "DHCPREPLY"                                           *const libc::c_char                                          &mut String,
                                            &mut req_addr_1, message);
                            } else {
                                log6_quiet(state,
                                           "DHCPREPLY",
                                           &mut req_addr_1, message);
                            }
                            o1 = new_opt6(5);
                            put_opt6(&mut req_addr_1                                  Vec<u8>,
                                     ::std::mem::size_of::<In6Addr>());
                            put_opt6_long(preferred_time);
                            put_opt6_long(valid_time);
                            end_opt6(o1);
                            ia_option_1 =
                                opt6_find(opt6_next(ia_option_1, ia_end_1),
                                          ia_end_1,
                                          5,
                                          24)
                        }
                    }
                    end_ia(t1cntr_1, min_time_1, 1);
                    end_opt6(o);
                }
                opt = opt6_next(opt, state.end)
            }
            tagif = add_options(state, 0);
            current_block_486 = 14838758841813985983;
        }
        4 => {
            let mut good_addr: i32 = 0;
            /* set reply message type */
            *outmsgtypep = 7;
            log6_quiet(state,
                       "DHCPCONFIRM"
                           , 0,
                       0 );
            opt = state.packet_options;
            while !opt.is_null() {
                let mut ia_option_2:Vec<u8> =
                    0;
                let mut ia_end_2:Vec<u8> = 0;
                check_ia(state, opt, &mut ia_end_2, &mut ia_option_2);
                while !ia_option_2.is_null() {
                    let mut req_addr_2: In6Addr =
                        In6Addr {__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},};
                    /* alignment */
                    memcpy(&mut req_addr_2,
                           &mut *(ia_option_2                                mut Vec<u8>).offset((4
                                                                      +
                                                                      0                       )
                                                                    )
                              ,
                           16);
                    if address6_valid(state.context, &mut req_addr_2,
                                      tagif, 1).is_null() {
                        o1 = new_opt6(13);
                        put_opt6_short(4);
                        put_opt6_string("confirm failed"                                       *const libc::c_char                                      &mut String);
                        end_opt6(o1);
                        log6_quiet(state,
                                   "DHCPREPLY", &mut req_addr_2,
                                   "confirm failed");
                        return 1
                    }
                    good_addr = 1;
                    log6_quiet(state,
                               "DHCPREPLY",
                               &mut req_addr_2, state.hostname);
                    ia_option_2 =
                        opt6_find(opt6_next(ia_option_2, ia_end_2), ia_end_2,
                                  5,
                                  24)
                }
                opt = opt6_next(opt, state.end)
            }
            /* No addresses, no reply: RFC 3315 18.2.2 */
            if good_addr == 0 { return 0 }
            o1 = new_opt6(13);
            put_opt6_short(0);
            put_opt6_string("all addresses still on link"                           *const libc::c_char );
            end_opt6(o1);
            current_block_486 = 14838758841813985983;
        }
        11 => {
            /* We can't discriminate contexts based on address, as we don't know it.
	   If there is only one possible context, we can use its tags */
            if !state.context.is_null() &&
                   !(*state.context).netid.net.is_null() &&
                   (*state.context).current.is_null() {
                (*state.context).netid.next = 0 ;
                state.context_tags = &mut (*state.context).netid
            }
            /* Similarly, we can't determine domain from address, but if the FQDN is
	   given in --dhcp-host, we can use that, and failing that we can use the 
	   unqualified configured domain, if any. */
            if state.hostname_auth != 0 {
                state.send_domain = state.domain
            } else { state.send_domain = get_domain6(0) }
            log6_quiet(state,
                       "DHCPINFORMATION-REQUEST"  ,
                       0,
                       if ignore != 0 {
                           "ignored"
                       } else { state.hostname }                     &mut String);
            if ignore != 0 { return 0 }
            *outmsgtypep = 7;
            tagif = add_options(state, 1);
            current_block_486 = 14838758841813985983;
        }
        8 => {
            /* set reply message type */
            *outmsgtypep = 7;
            log6_quiet(state,
                       "DHCPRELEASE"
                           , 0,
                       0 );
            opt = state.packet_options;
            while !opt.is_null() {
                let mut ia_option_3:Vec<u8> =
                    0;
                let mut ia_end_3:Vec<u8> = 0;
                let mut made_ia: i32 = 0;
                check_ia(state, opt, &mut ia_end_3, &mut ia_option_3);
                while !ia_option_3.is_null() {
                    let mut lease_0: DhcpLease = 0;
                    let mut addr_0: In6Addr =
                        In6Addr {__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},};
                    /* align */
                    memcpy(&mut addr_0,
                           &mut *(ia_option_3                                mut Vec<u8>).offset((4
                                                                      +
                                                                      0                       )
                                                                    )
                              ,
                           16);
                    lease_0 =
                        lease6_find(state.clid, state.clid_len,
                                    if state.ia_type == 3 {
                                        32
                                    } else { 64 },
                                    state.iaid, &mut addr_0);
                    if !lease_0.is_null() {
                        lease_prune(lease_0, now);
                    } else {
                        if made_ia == 0 {
                            o = new_opt6(state.ia_type);
                            put_opt6_long(state.iaid);
                            if state.ia_type == 3 {
                                put_opt6_long(0                                            libc::c_uint);
                                put_opt6_long(0                                            libc::c_uint);
                            }
                            made_ia = 1
                        }
                        o1 = new_opt6(5);
                        put_opt6(&mut addr_0                              Vec<u8>,
                                 16 );
                        put_opt6_long(0);
                        put_opt6_long(0);
                        end_opt6(o1);
                    }
                    ia_option_3 =
                        opt6_find(opt6_next(ia_option_3, ia_end_3), ia_end_3,
                                  5,
                                  24)
                }
                if made_ia != 0 {
                    o1 = new_opt6(13);
                    put_opt6_short(3);
                    put_opt6_string("no binding found"                                   *const libc::c_char                                  &mut String);
                    end_opt6(o1);
                    end_opt6(o);
                }
                opt = opt6_next(opt, state.end)
            }
            o1 = new_opt6(13);
            put_opt6_short(0);
            put_opt6_string("release received"                           *const libc::c_char );
            end_opt6(o1);
            current_block_486 = 14838758841813985983;
        }
        9 => {
            /* set reply message type */
            *outmsgtypep = 7;
            log6_quiet(state,
                       "DHCPDECLINE"
                           , 0,
                       0 );
            opt = state.packet_options;
            while !opt.is_null() {
                let mut ia_option_4:Vec<u8> =
                    0;
                let mut ia_end_4:Vec<u8> = 0;
                let mut made_ia_0: i32 = 0;
                check_ia(state, opt, &mut ia_end_4, &mut ia_option_4);
                while !ia_option_4.is_null() {
                    let mut lease_1: DhcpLease = 0;
                    let mut addr_1: In6Addr =
                        In6Addr {__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},};
                    let mut addr_list: AddressListEntry = 0 ;
                    /* align */
                    memcpy(&mut addr_1,
                           &mut *(ia_option_4                                mut Vec<u8>).offset((4
                                                                      +
                                                                      0                       )
                                                                    )
                              ,
                           16);
                    addr_list =
                        config_implies(config, state.context, &mut addr_1);
                    if !addr_list.is_null() {
                        prettyprint_time(dnsmasq_daemon.dhcp_buff3,
                                         600);
                        inet_ntop(10,
                                  &mut addr_1
                                  dnsmasq_daemon.addrbuff,
                                  46);
                        my_syslog((3) << 3 |
                                      4,
                                  "disabling DHCP static address %s for %s"
                                      ,
                                  dnsmasq_daemon.addrbuff,
                                  dnsmasq_daemon.dhcp_buff3);
                        addr_list.flags |= 32;
                        addr_list.decline_time = now
                    } else {
                        /* make sure this host gets a different address next time. */
                        context_tmp = state.context;
                        while !context_tmp.is_null() {
                            context_tmp.addr_epoch =
                                context_tmp.addr_epoch.wrapping_add(1);
                            context_tmp = context_tmp.current
                        }
                    }
                    lease_1 =
                        lease6_find(state.clid, state.clid_len,
                                    if state.ia_type == 3 {
                                        32
                                    } else { 64 },
                                    state.iaid, &mut addr_1);
                    if !lease_1.is_null() {
                        lease_prune(lease_1, now);
                    } else {
                        if made_ia_0 == 0 {
                            o = new_opt6(state.ia_type);
                            put_opt6_long(state.iaid);
                            if state.ia_type == 3 {
                                put_opt6_long(0                                            libc::c_uint);
                                put_opt6_long(0                                            libc::c_uint);
                            }
                            made_ia_0 = 1
                        }
                        o1 = new_opt6(5);
                        put_opt6(&mut addr_1                              Vec<u8>,
                                 16 );
                        put_opt6_long(0);
                        put_opt6_long(0);
                        end_opt6(o1);
                    }
                    ia_option_4 =
                        opt6_find(opt6_next(ia_option_4, ia_end_4), ia_end_4,
                                  5,
                                  24)
                }
                if made_ia_0 != 0 {
                    o1 = new_opt6(13);
                    put_opt6_short(3);
                    put_opt6_string("no binding found"                                   *const libc::c_char                                  &mut String);
                    end_opt6(o1);
                    end_opt6(o);
                }
                opt = opt6_next(opt, state.end)
            }
            /* We must answer with 'success' in global section anyway */
            o1 = new_opt6(13);
            put_opt6_short(0);
            put_opt6_string("success"
                                );
            end_opt6(o1);
            current_block_486 = 14838758841813985983;
        }
        _ => { return 0 }
    }
    match current_block_486 {
        15319502457978536222 => {
            solicit_tags = tagif;
            if ignore != 0 { return 0 }
            /* reset USED bits in leases */
            lease6_reset();
            /* Can use configured address max once per prefix */
            c = state.context;
            while !c.is_null() {
                c.flags =
                    (c.flags &
                         !((1) << 14)) ;
                c = c.current
            }
            let mut current_block_242: u64;
            opt = state.packet_options;
            while !opt.is_null() {
                let mut ia_option:Vec<u8> = 0;
                let mut ia_end:Vec<u8> = 0;
                let mut min_time: u32 = 0xffffffff;
                let mut t1cntr: i32 = 0;
                let mut ia_counter: i32 = 0;
                /* set unless we're sending a particular prefix-class, when we
	       want only dhcp-ranges with the correct tags set and not those without any tags. */
                let mut plain_range: i32 = 1;
                let mut lease_time: u32 = 0;
                let mut ltmp: DhcpLease = 0;
                let mut req_addr: In6Addr =
                    In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
                let mut addr: In6Addr =
                    In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
                if !(check_ia(state, opt, &mut ia_end, &mut ia_option) == 0) {
                    /* reset USED bits in contexts - one address per prefix per IAID */
                    c = state.context;
                    while !c.is_null() {
                        c.flags =
                            (c.flags &
                                 !((1) << 15))
                               ;
                        c = c.current
                    }
                    o = build_ia(state, &mut t1cntr);
                    if address_assigned != 0 {
                        address_assigned = 2
                    }
                    let mut current_block_206: u64;
                    ia_counter = 0;
                    while !ia_option.is_null() {
                        /* worry about alignment here. */
                        memcpy(&mut req_addr                            Vec<u8>,
                               &mut *(ia_option                                    mut Vec<u8>).offset((4
                                                                          +
                                                                          0                           )
                                                        )
                                  ,
                               16);
                        c =
                            address6_valid(state.context, &mut req_addr,
                                           solicit_tags, plain_range);
                        if !c.is_null() {
                            lease_time = c.lease_time;
                            /* If the client asks for an address on the same network as a configured address, 
		       offer the configured address instead, to make moving to newly-configured
		       addresses automatic. */
                            if c.flags &
                                   (1) << 14 ==
                                   0 &&
                                   config_valid(config, c, &mut addr, state,
                                                now) != 0 {
                                req_addr =
                                    addr; /* address leased elsewhere */
                                mark_config_used(c, &mut addr);
                                if !config.is_null() &&
                                       config.flags &
                                           8 !=
                                           0 {
                                    lease_time = config.lease_time
                                }
                                current_block_206 = 1851490986684842406;
                            } else {
                                c =
                                    address6_available(state.context,
                                                       &mut req_addr,
                                                       solicit_tags,
                                                       plain_range);
                                if c.is_null() {
                                    current_block_206 = 9350489878244555550;
                                } else if check_address(state, &mut req_addr)
                                              == 0 {
                                    current_block_206 = 9350489878244555550;
                                } else {
                                    current_block_206 = 1851490986684842406;
                                }
                            }
                            match current_block_206 {
                                9350489878244555550 => { }
                                _ => {
                                    /* add address to output packet */
                                    add_address(state, c, lease_time,
                                                ia_option, &mut min_time,
                                                &mut req_addr,
                                                now); /* not an address we're allowed */
                                    mark_context_used(state, &mut req_addr);
                                    get_context_tag(state, c);
                                    address_assigned = 1
                                }
                            }
                        }
                        ia_counter += 1;
                        ia_option =
                            opt6_find(opt6_next(ia_option, ia_end), ia_end,
                                      5,
                                      24)
                    }
                    /* Suggest configured address(es) */
                    c = state.context;
                    while !c.is_null() {
                        if c.flags &
                               (1) << 14 == 0
                               &&
                               match_netid(c.filter, solicit_tags,
                                           plain_range) != 0 &&
                               config_valid(config, c, &mut addr, state, now)
                                   != 0 {
                            mark_config_used(state.context, &mut addr);
                            if !config.is_null() &&
                                   config.flags &
                                       8 != 0 {
                                lease_time = config.lease_time
                            } else { lease_time = c.lease_time }
                            /* add address to output packet */
                            add_address(state, c, lease_time,
                                        0, &mut min_time,
                                        &mut addr, now);
                            mark_context_used(state, &mut addr);
                            get_context_tag(state, c);
                            address_assigned = 1
                        }
                        c = c.current
                    }
                    /* return addresses for existing leases */
                    ltmp = 0;
                    loop  {
                        ltmp =
                            lease6_find_by_client(ltmp,
                                                  if state.ia_type ==
                                                         3 {
                                                      32
                                                  } else {
                                                      64
                                                  }, state.clid,
                                                  state.clid_len,
                                                  state.iaid);
                        if ltmp.is_null() { break ; }
                        req_addr = ltmp.addr6;
                        c =
                            address6_available(state.context,
                                               &mut req_addr, solicit_tags,
                                               plain_range);
                        if !c.is_null() {
                            add_address(state, c, c.lease_time,
                                        0, &mut min_time,
                                        &mut req_addr, now);
                            mark_context_used(state, &mut req_addr);
                            get_context_tag(state, c);
                            address_assigned = 1
                        }
                    }
                    loop 
                         /* Return addresses for all valid contexts which don't yet have one */
                         {
                        c =
                            address6_allocate(state.context, state.clid,
                                              state.clid_len,
                                              (state.ia_type ==
                                                   4)                                            , state.iaid,
                                              ia_counter, solicit_tags,
                                              plain_range, &mut addr);
                        if c.is_null() { break ; }
                        add_address(state, c, c.lease_time,
                                    0, &mut min_time,
                                    &mut addr, now);
                        mark_context_used(state, &mut addr);
                        get_context_tag(state, c);
                        address_assigned = 1
                    }
                    if address_assigned != 1 {
                        /* If the server will not assign any addresses to any IAs in a
		   subsequent Request from the client, the server MUST send an Advertise
		   message to the client that doesn't include any IA options. */
                        if state.lease_allocate == 0 {
                            save_counter(o);
                            current_block_242 = 13164310931121142693;
                        } else {
                            /* If the server cannot assign any addresses to an IA in the message
		   from the client, the server MUST include the IA in the Reply message
		   with no addresses in the IA and a Status Code option in the IA
		   containing status code NoAddrsAvail. */
                            o1 = new_opt6(13);
                            put_opt6_short(2);
                            put_opt6_string("address unavailable"                                          *const u8                                          *const libc::c_char                                          &mut String);
                            end_opt6(o1);
                            current_block_242 = 15605369199999130895;
                        }
                    } else { current_block_242 = 15605369199999130895; }
                    match current_block_242 {
                        13164310931121142693 => { }
                        _ => {
                            end_ia(t1cntr, min_time, 0);
                            end_opt6(o);
                        }
                    }
                }
                opt = opt6_next(opt, state.end)
            }
            if address_assigned != 0 {
                o1 = new_opt6(13);
                put_opt6_short(0);
                put_opt6_string("success"                               *const libc::c_char );
                end_opt6(o1);
                /* If --dhcp-authoritative is set, we can tell client not to wait for
	       other possible servers */
                o = new_opt6(7);
                put_opt6_char(if dnsmasq_daemon.options[(17
                                                                     ).wrapping_div((::std::mem::size_of::<libc::c_uint>()          ).wrapping_mul(8                                                                                                                                                           ))
                                                               ] &
                                     (1) <<
                                         (17                                 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                   ).wrapping_mul(8                                                                                   ))
                                     != 0 {
                                  255
                              } else { 0 });
                end_opt6(o);
                tagif = add_options(state, 0)
            } else {
                /* no address, return error */
                o1 = new_opt6(13);
                put_opt6_short(2);
                put_opt6_string("no addresses available"                               *const libc::c_char );
                end_opt6(o1);
                /* Some clients will ask repeatedly when we're not giving
	       out addresses because we're in stateless mode. Avoid spamming
	       the log in that case. */
                c = state.context;
                while !c.is_null() {
                    if c.flags &
                           (1) << 7 == 0 {
                        log6_packet(state,
                                    if state.lease_allocate != 0 {
                                        "DHCPREPLY"                                       *const libc::c_char
                                    } else {
                                        "DHCPADVERTISE"                                       *const libc::c_char
                                    } ,
                                    0,
                                    "no addresses available"                                            &mut String);
                        break ;
                    } else { c = c.current }
                }
            }
        }
        _ => { }
    }
    log_tags(tagif, state.xid);
    log6_opts(0, state.xid,
              dnsmasq_daemon.outpacket.iov_base.offset(start_opts           ),
              dnsmasq_daemon.outpacket.iov_base.offset(save_counter(-(1                          ))
                                                             ));
    return 1;
}
unsafe extern "C" fn add_options(mut state: &mut state,
                                 mut do_refresh: i32)
 -> DhcpNetId {
    let mut oro:Vec<u8> = 0;
    /* filter options based on tags, those we want get DHOPT_TAGOK bit set */
    let mut tagif: DhcpNetId =
        option_filter(state.tags, state.context_tags,
                      dnsmasq_daemon.dhcp_opts6);
    let mut opt_cfg: DhcpOpt = 0 ;
    let mut done_dns: i32 = 0;
    let mut done_refresh: i32 = (do_refresh == 0);
    let mut do_encap: i32 = 0;
    let mut i: i32 = 0;
    let mut o: i32 = 0;
    let mut o1: i32 = 0;
    oro =
        opt6_find(state.packet_options, state.end,
                  6,
                  0);
    let mut current_block_45: u64;
    opt_cfg = dnsmasq_daemon.dhcp_opts6;
    while !opt_cfg.is_null() {
        /* netids match and not encapsulated? */
        if !(opt_cfg.flags & 4096 == 0) {
            if opt_cfg.flags & 16 == 0 && !oro.is_null() {
                i = 0;
                while i <
                          opt6_uint(oro,
                                    -(2), 2) - 1 {
                    if opt6_uint(oro, i,
                                 2) ==
                           opt_cfg.opt {
                        break ;
                    }
                    i += 2
                }
                /* option not requested */
                if i >=
                       opt6_uint(oro,
                                 -(2), 2)                      - 1 {
                    current_block_45 = 735147466149431745;
                } else { current_block_45 = 11050875288958768710; }
            } else { current_block_45 = 11050875288958768710; }
            match current_block_45 {
                735147466149431745 => { }
                _ => {
                    if opt_cfg.opt == 32 {
                        done_refresh = 1
                    }
                    if opt_cfg.opt == 23 {
                        done_dns = 1
                    }
                    if opt_cfg.flags & 8192 != 0 {
                        let mut len: i32 = 0;
                        let mut j: i32 = 0;
                        let mut a: In6Addr = 0;
                        a = opt_cfg.val;
                        len = opt_cfg.len;
                        j = 0;
                        while j < opt_cfg.len {
                            if *(a                               *const u32).offset(0
                                                                ) ==
                                   __bswap_32(0xfd000000) &&
                                   *(a                                   *const u32).offset(1
                                                                    )
                                       == 0 &&
                                   *(a                                   *const u32).offset(2
                                                                    )
                                       == 0 &&
                                   *(a                                   *const u32).offset(3
                                                                    )
                                       == 0 &&
                                   ({
                                        let mut __a: *const In6Addr =
                                            state.ula_addr                                          *const In6Addr;
                                        (__a.__in6_u.__u6_addr32[0
                                                                                            usize]
                                             ==
                                             0
                                             &&
                                             __a.__in6_u.__u6_addr32[1
                                                                                                    usize]
                                                 ==
                                                 0  libc::c_uint &&
                                             __a.__in6_u.__u6_addr32[2
                                                                                                    usize]
                                                 ==
                                                 0  libc::c_uint &&
                                             __a.__in6_u.__u6_addr32[3
                                                                                                    usize]
                                                 ==
                                                 0  libc::c_uint)
                                    }) != 0 ||
                                   *(a                                   *const u32).offset(0
                                                                    )
                                       ==
                                       __bswap_32(0xfe800000)
                                       &&
                                       *(a                                       *const u32).offset(1
                                                        )
                                           == 0
                                       &&
                                       *(a                                       *const u32).offset(2
                                                        )
                                           == 0
                                       &&
                                       *(a                                       *const u32).offset(3
                                                        )
                                           == 0
                                       &&
                                       ({
                                            let mut __a: *const In6Addr =
                                                state.ll_addr *const In6Addr;
                                            (__a.__in6_u.__u6_addr32[0
                                                                                                    usize]
                                                 ==
                                                 0  libc::c_uint &&
                                                 __a.__in6_u.__u6_addr32[1

                                                                                                            usize]
                                                     ==
                                                     0      libc::c_uint &&
                                                 __a.__in6_u.__u6_addr32[2

                                                                                                            usize]
                                                     ==
                                                     0      libc::c_uint &&
                                                 __a.__in6_u.__u6_addr32[3

                                                                                                            usize]
                                                     ==
                                                     0      libc::c_uint)
                                        }) != 0 {
                                len -= 16
                            }
                            j += 16;
                            a = a.offset(1)
                        }
                        if len != 0 {
                            o = new_opt6(opt_cfg.opt);
                            a = opt_cfg.val;
                            j = 0;
                            while j < opt_cfg.len {
                                let mut p: In6Addr = 0;
                                if ({
                                        let mut __a: *const In6Addr =
                                            a ;
                                        (__a.__in6_u.__u6_addr32[0
                                                                                            usize]
                                             ==
                                             0
                                             &&
                                             __a.__in6_u.__u6_addr32[1
                                                                                                    usize]
                                                 ==
                                                 0  libc::c_uint &&
                                             __a.__in6_u.__u6_addr32[2
                                                                                                    usize]
                                                 ==
                                                 0  libc::c_uint &&
                                             __a.__in6_u.__u6_addr32[3
                                                                                                    usize]
                                                 ==
                                                 0  libc::c_uint)
                                    }) != 0 {
                                    if add_local_addrs(state.context) == 0
                                       {
                                        p = state.fallback
                                    }
                                } else if *(a                                          *const u32).offset(0
                                                              )
                                              ==
                                              __bswap_32(0xfd000000          libc::c_uint) &&
                                              *(a *const u32).offset(1

                                                                      )
                                                  ==
                                                  0   libc::c_uint &&
                                              *(a *const u32).offset(2

                                                                      )
                                                  ==
                                                  0   libc::c_uint &&
                                              *(a *const u32).offset(3

                                                                      )
                                                  ==
                                                  0   libc::c_uint {
                                    if ({
                                            let mut __a: *const In6Addr =
                                                state.ula_addr *const In6Addr;
                                            (__a.__in6_u.__u6_addr32[0
                                                                                                    usize]
                                                 ==
                                                 0  libc::c_uint &&
                                                 __a.__in6_u.__u6_addr32[1

                                                                                                            usize]
                                                     ==
                                                     0      libc::c_uint &&
                                                 __a.__in6_u.__u6_addr32[2

                                                                                                            usize]
                                                     ==
                                                     0      libc::c_uint &&
                                                 __a.__in6_u.__u6_addr32[3

                                                                                                            usize]
                                                     ==
                                                     0      libc::c_uint)
                                        }) == 0 {
                                        p = state.ula_addr
                                    }
                                } else if *(a                                          *const u32).offset(0
                                                              )
                                              ==
                                              __bswap_32(0xfe800000          libc::c_uint) &&
                                              *(a *const u32).offset(1

                                                                      )
                                                  ==
                                                  0   libc::c_uint &&
                                              *(a *const u32).offset(2

                                                                      )
                                                  ==
                                                  0   libc::c_uint &&
                                              *(a *const u32).offset(3

                                                                      )
                                                  ==
                                                  0   libc::c_uint {
                                    if ({
                                            let mut __a: *const In6Addr =
                                                state.ll_addr *const In6Addr;
                                            (__a.__in6_u.__u6_addr32[0
                                                                                                    usize]
                                                 ==
                                                 0  libc::c_uint &&
                                                 __a.__in6_u.__u6_addr32[1

                                                                                                            usize]
                                                     ==
                                                     0      libc::c_uint &&
                                                 __a.__in6_u.__u6_addr32[2

                                                                                                            usize]
                                                     ==
                                                     0      libc::c_uint &&
                                                 __a.__in6_u.__u6_addr32[3

                                                                                                            usize]
                                                     ==
                                                     0      libc::c_uint)
                                        }) == 0 {
                                        p = state.ll_addr
                                    }
                                } else { p = a }
                                if !p.is_null() {
                                    if opt_cfg.opt == 56 {
                                        if *(p                                           *const u8).offset(0
                                                              )
                                               ==
                                               0xff {
                                            o1 = new_opt6(2)
                                        } else {
                                            o1 = new_opt6(1)
                                        }
                                        put_opt6(p,
                                                 16 );
                                        end_opt6(o1);
                                    } else {
                                        put_opt6(p,
                                                 16 );
                                    }
                                }
                                j += 16;
                                a = a.offset(1)
                            }
                            end_opt6(o);
                        }
                    } else {
                        o = new_opt6(opt_cfg.opt);
                        if !opt_cfg.val.is_null() {
                            put_opt6(opt_cfg.val,
                                     opt_cfg.len );
                        }
                        end_opt6(o);
                    }
                }
            }
        }
        opt_cfg = opt_cfg.next
    }
    if dnsmasq_daemon.port == 53 && done_dns == 0 {
        o = new_opt6(23);
        if add_local_addrs(state.context) == 0 {
            put_opt6(state.fallback,
                     16 );
        }
        end_opt6(o);
    }
    if !state.context.is_null() && done_refresh == 0 {
        let mut c: DhcpContext = 0;
        let mut lease_time: u32 = 0xffffffff;
        /* Find the smallest lease tie of all contexts,
	 subject to the RFC-4242 stipulation that this must not 
	 be less than 600. */
        c = state.context;
        while !c.is_null() {
            if c.lease_time < lease_time {
                if c.lease_time < 600 {
                    lease_time = 600
                } else { lease_time = c.lease_time }
            }
            c = c.next
        }
        o = new_opt6(32);
        put_opt6_long(lease_time);
        end_opt6(o);
    }
    /* handle vendor-identifying vendor-encapsulated options,
       dhcp-option = vi-encap:13,17,....... */
    opt_cfg = dnsmasq_daemon.dhcp_opts6;
    while !opt_cfg.is_null() {
        opt_cfg.flags &= !(64);
        opt_cfg = opt_cfg.next
    }
    if !oro.is_null() {
        i = 0;
        while i <
                  opt6_uint(oro, -(2),
                            2) -
                      1 {
            if opt6_uint(oro, i, 2) ==
                   17 {
                do_encap = 1
            }
            i += 2
        }
    }
    opt_cfg = dnsmasq_daemon.dhcp_opts6;
    while !opt_cfg.is_null() {
        if opt_cfg.flags & 2048 != 0 {
            let mut found: i32 = 0;
            let mut oc: DhcpOpt = 0 ;
            if !(opt_cfg.flags & 64 != 0) {
                oc = dnsmasq_daemon.dhcp_opts6;
                while !oc.is_null() {
                    oc.flags &= !(8);
                    if !(oc.flags & 2048 == 0 ||
                             opt_cfg.u.encap != oc.u.encap) {
                        oc.flags |= 64;
                        if match_netid(oc.netid, tagif, 1)
                               != 0 {
                            /* option requested/forced? */
                            if oro.is_null() || do_encap != 0 ||
                                   oc.flags & 16 != 0 {
                                oc.flags |= 8;
                                found = 1
                            }
                        }
                    }
                    oc = oc.next
                }
                if found != 0 {
                    o = new_opt6(17);
                    put_opt6_long(opt_cfg.u.encap);
                    oc = dnsmasq_daemon.dhcp_opts6;
                    while !oc.is_null() {
                        if oc.flags & 8 != 0 {
                            o1 = new_opt6(oc.opt);
                            put_opt6(oc.val,
                                     oc.len );
                            end_opt6(o1);
                        }
                        oc = oc.next
                    }
                    end_opt6(o);
                }
            }
        }
        opt_cfg = opt_cfg.next
    }
    if !state.hostname.is_null() {
        let mut p_0: mut Vec<u8> = 0;
        let mut len_0: usize = strlen(state.hostname);
        if !state.send_domain.is_null() {
            len_0 =
                (len_0        ).wrapping_add(strlen(state.send_domain).wrapping_add(2                   ))

        }
        o = new_opt6(39);
        p_0 =
            expand(len_0.wrapping_add(2))          mut Vec<u8>;
        if !p_0.is_null() {
            let fresh7 = p_0;
            p_0 = p_0.offset(1);
            *fresh7 = state.fqdn_flags;
            p_0 =
                do_rfc1035_name(p_0, state.hostname,
                                0 );
            if !state.send_domain.is_null() {
                p_0 =
                    do_rfc1035_name(p_0, state.send_domain,
                                    0 );
                *p_0 = 0
            }
        }
        end_opt6(o);
    }
    /* logging */
    if dnsmasq_daemon.options[(28).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                                                   ))
                                     ] &
           (1) <<
               (28).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8

                                                                                                               ))
           != 0 && !oro.is_null() {
        let mut q: &mut String = dnsmasq_daemon.namebuff;
        i = 0;
        while i <
                  opt6_uint(oro, -(2),
                            2) -
                      1 {
            let mut s: &mut String =
                option_string(10,
                              opt6_uint(oro, i,
                                        2),
                              0, 0,
                              0 , 0);
            q =
                q.offset(snprintf(q,
                                  (1025 -
                                       q.wrapping_offset_from(dnsmasq_daemon.namebuff)
                                          ),
                                  "%d%s%s%s",
                                  opt6_uint(oro, i,
                                            2),
                                  if strlen(s) !=
                                         0 {
                                      ":"                                     *const libc::c_char
                                  } else {
                                      ""                                     *const libc::c_char
                                  }, s,
                                  if i >
                                         opt6_uint(oro,
                                                   -(2),
                                                   2)                                        - 3 {
                                      ""                                     *const libc::c_char
                                  } else {
                                      ", "                                     *const libc::c_char
                                  }));
            if i >
                   opt6_uint(oro, -(2),
                             2) -
                       3 ||
                   q.wrapping_offset_from(dnsmasq_daemon.namebuff)                 i32 > 40 {
                q = dnsmasq_daemon.namebuff;
                my_syslog((3) << 3 |
                              6,
                          "%u requested options: %s" , state.xid,
                          dnsmasq_daemon.namebuff);
            }
            i += 2
        }
    }
    return tagif;
}
unsafe extern "C" fn add_local_addrs(mut context: DhcpContext)
 -> i32 {
    let mut done: i32 = 0;
    while !context.is_null() {
        if context.flags &
               (1) << 15 != 0 &&
               ({
                    let mut __a: *const In6Addr =
                        &mut context.local6                      *const In6Addr;
                    (__a.__in6_u.__u6_addr32[0 ] ==
                         0 &&
                         __a.__in6_u.__u6_addr32[1 ]
                             == 0 &&
                         __a.__in6_u.__u6_addr32[2 ]
                             == 0 &&
                         __a.__in6_u.__u6_addr32[3 ]
                             == 0)
                }) == 0 {
            /* squash duplicates */
            let mut c: DhcpContext = 0;
            c = context.current;
            while !c.is_null() {
                if c.flags &
                       (1) << 15 != 0 &&
                       ({
                            let mut __a: *const In6Addr =
                                &mut context.local6;
                            let mut __b: *const In6Addr =
                                &mut c.local6;
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
                    break ;
                }
                c = c.current
            }
            if c.is_null() {
                done = 1;
                put_opt6(&mut context.local6                      Vec<u8>, 16 );
            }
        }
        context = context.current
    }
    return done;
}
unsafe extern "C" fn get_context_tag(mut state: &mut state,
                                     mut context: DhcpContext) {
    /* get tags from context if we've not used it before */
    if context.netid.next == &mut context.netid  &&
           !context.netid.net.is_null() {
        context.netid.next = state.context_tags;
        state.context_tags = &mut context.netid;
        if state.hostname_auth == 0 {
            let mut id_list: DhcpNetIdList = 0;
            id_list = dnsmasq_daemon.dhcp_ignore_names;
            while !id_list.is_null() {
                if id_list.list.is_null() ||
                       match_netid(id_list.list, &mut context.netid,
                                   0) != 0 {
                    break ;
                }
                id_list = id_list.next
            }
            if !id_list.is_null() {
                state.hostname = 0
            }
        }
    };
}
unsafe extern "C" fn check_ia(mut state: &mut state,
                              mut opt:Vec<u8>,
                              mut endp: Vec<u8>,
                              mut ia_option: Vec<u8>)
 -> i32 {
    state.ia_type =
        opt6_uint(opt, -(4),
                  2);
    *ia_option = 0;
    if state.ia_type != 3 &&
           state.ia_type != 4 {
        return 0
    }
    if state.ia_type == 3 &&
           (opt6_uint(opt, -(2),
                      2)) < 12 {
        return 0
    }
    if state.ia_type == 4 &&
           (opt6_uint(opt, -(2),
                      2)) < 4 {
        return 0
    }
    *endp =
        &mut *(opt             mut Vec<u8>).offset((4 +
                                                   (opt6_uint     unsafe extern "C" fn(_:
                                                                                 mut Vec<u8>,
                                                                             _:
                                                                                 ,
                                                                             _:
                                                                                 )
                                                            ->
                                                                libc::c_uint)(opt
                                                                                                                mut Vec<u8>,
                                                                              -(2
                                                                                                                    ),
                                                                              2
                                                                                                                )
                                                      )      )
           ;
    state.iaid =
        opt6_uint(opt, 0,
                  4);
    *ia_option =
        opt6_find(&mut *(opt                       mut Vec<u8>).offset((4 +
                                                             (if state.ia_type
                                                                     ==
                                                                     3
                                                                 {
                                                                  12
                                                              } else {
                                                                  4
                                                              })))                mut Vec<u8>, *endp,
                  5,
                  24);
    return 1;
}
unsafe extern "C" fn build_ia(mut state: &mut state,
                              mut t1cntr: ) -> i32 {
    let mut o: i32 = new_opt6(state.ia_type);
    put_opt6_long(state.iaid);
    *t1cntr = 0;
    if state.ia_type == 3 {
        /* save pointer */
        *t1cntr = save_counter(-(1));
        /* so we can fill these in later */
        put_opt6_long(0);
        put_opt6_long(0);
    }
    return o;
}
unsafe extern "C" fn end_ia(mut t1cntr: i32,
                            mut min_time: u32,
                            mut do_fuzz: i32) {
    if t1cntr != 0 {
        /* go back and fill in fields in IA_NA option */
        let mut sav: i32 = save_counter(t1cntr);
        let mut t1: u32 = 0;
        let mut t2: u32 = 0;
        let mut fuzz: u32 = 0;
        if do_fuzz != 0 {
            fuzz = rand16();
            while fuzz >
                      min_time.wrapping_div(16)
                  {
                fuzz = fuzz.wrapping_div(2)
            }
        }
        t1 =
            if min_time == 0xffffffff {
                0xffffffff
            } else {
                min_time.wrapping_div(2                                    libc::c_uint).wrapping_sub(fuzz)
            };
        t2 =
            if min_time == 0xffffffff {
                0xffffffff
            } else {
                min_time.wrapping_div(8                                    libc::c_uint).wrapping_mul(7
                                                                                              libc::c_uint).wrapping_sub(fuzz)
            };
        put_opt6_long(t1);
        put_opt6_long(t2);
        save_counter(sav);
    };
}
unsafe extern "C" fn add_address(mut state: &mut state,
                                 mut context: DhcpContext,
                                 mut lease_time: u32,
                                 mut ia_option:Vec<u8>,
                                 mut min_time: &mut libc::c_uint,
                                 mut addr: &mut In6Addr, mut now: time::Instant) {
    let mut valid_time: u32 = 0;
    let mut preferred_time: u32 = 0;
    let mut o: i32 = new_opt6(5);
    let mut lease: DhcpLease = 0;
    /* get client requested times */
    if !ia_option.is_null() {
        preferred_time =
            opt6_uint(ia_option, 16,
                      4);
        valid_time =
            opt6_uint(ia_option, 20,
                      4)
    }
    calculate_times(context, min_time, &mut valid_time, &mut preferred_time,
                    lease_time);
    put_opt6(addr,
             ::std::mem::size_of::<In6Addr>());
    put_opt6_long(preferred_time);
    put_opt6_long(valid_time);
    end_opt6(o);
    if state.lease_allocate != 0 {
        update_leases(state, context, addr, valid_time, now);
    }
    lease =
        lease6_find_by_addr(addr, 128,
                            0 as u64);
    if !lease.is_null() { lease.flags |= 16 }
    /* get tags from context if we've not used it before */
    if context.netid.next == &mut context.netid  &&
           !context.netid.net.is_null() {
        context.netid.next = state.context_tags;
        state.context_tags = &mut context.netid;
        if state.hostname_auth == 0 {
            let mut id_list: DhcpNetIdList = 0;
            id_list = dnsmasq_daemon.dhcp_ignore_names;
            while !id_list.is_null() {
                if id_list.list.is_null() ||
                       match_netid(id_list.list, &mut context.netid,
                                   0) != 0 {
                    break ;
                }
                id_list = id_list.next
            }
            if !id_list.is_null() {
                state.hostname = 0
            }
        }
    }
    log6_quiet(state,
               if state.lease_allocate != 0 {
                   "DHCPREPLY"
               } else {
                   "DHCPADVERTISE"
               } , addr, state.hostname);
}
unsafe extern "C" fn mark_context_used(mut state: &mut state,
                                       mut addr: &mut In6Addr) {
    let mut context: DhcpContext = 0;
    /* Mark that we have an address for this prefix. */
    context = state.context;
    while !context.is_null() {
        if is_same_net6(addr, &mut context.start6, context.prefix) != 0
           {
            context.flags =
                (context.flags |
                     (1) << 15)
        }
        context = context.current
    };
}
unsafe extern "C" fn mark_config_used(mut context: DhcpContext,
                                      mut addr: &mut In6Addr) {
    while !context.is_null() {
        if is_same_net6(addr, &mut context.start6, context.prefix) != 0
           {
            context.flags =
                (context.flags |
                     (1) << 14)
        }
        context = context.current
    };
}
/* make sure address not leased to another CLID/IAID */
unsafe extern "C" fn check_address(mut state: &mut state,
                                   mut addr: &mut In6Addr) -> i32 {
    let mut lease: DhcpLease = 0;
    lease =
        lease6_find_by_addr(addr, 128,
                            0 as u64);
    if lease.is_null() { return 1 }
    if lease.clid_len != state.clid_len ||
           memcmp(lease.clid,
                  state.clid,
                  state.clid_len) != 0 ||
           lease.iaid != state.iaid {
        return 0
    }
    return 1;
}
/* return true of *addr could have been generated from config. */
unsafe extern "C" fn config_implies(mut config: &mut DhcpConfig,
                                    mut context: DhcpContext,
                                    mut addr: &mut In6Addr)
                                    -> AddressListEntry {
    let mut prefix: i32 = 0;
    let mut wild_addr: In6Addr =
        In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    let mut addr_list: AddressListEntry = 0 ;
    if config.is_null() ||
           config.flags & 4096 == 0 {
        return 0
    }
    let mut current_block_9: u64;
    addr_list = config.addr6;
    while !addr_list.is_null() {
        prefix =
            if addr_list.flags & 8 != 0 {
                addr_list.prefixlen
            } else { 128 };
        wild_addr = addr_list.addr.addr6;
        if addr_list.flags & 16 != 0 &&
               context.prefix == 64 {
            wild_addr = context.start6;
            setaddr6part(&mut wild_addr,
                         addr6part(&mut addr_list.addr.addr6));
            current_block_9 = 7746791466490516765;
        } else if is_same_net6(&mut context.start6, addr,
                               context.prefix) == 0 {
            current_block_9 = 17179679302217393232;
        } else { current_block_9 = 7746791466490516765; }
        match current_block_9 {
            7746791466490516765 => {
                if is_same_net6(&mut wild_addr, addr, prefix) != 0 {
                    return addr_list
                }
            }
            _ => { }
        }
        addr_list = addr_list.next
    }
    return 0 ;
}
unsafe extern "C" fn config_valid(mut config: &mut DhcpConfig,
                                  mut context: DhcpContext,
                                  mut addr: &mut In6Addr,
                                  mut state: &mut state, mut now: time::Instant)
                                  -> i32 {
    let mut addrpart: u64 = 0;
    let mut i: u64 = 0;
    let mut addresses: u64 = 0;
    let mut addr_list: AddressListEntry = 0 ;
    if config.is_null() ||
           config.flags & 4096 == 0 {
        return 0
    }
    let mut current_block_14: u64;
    addr_list = config.addr6;
    while !addr_list.is_null() {
        if addr_list.flags & 32 == 0 ||
               difftime(now, addr_list.decline_time) >=
                   600   {
            addrpart = addr6part(&mut addr_list.addr.addr6);
            addresses = 1 as u64;
            if addr_list.flags & 8 != 0 {
                addresses =
                    (1 as u64) <<
                        128 - addr_list.prefixlen
            }
            if addr_list.flags & 16 != 0 {
                if context.prefix != 64 {
                    current_block_14 = 10680521327981672866;
                } else {
                    *addr = context.start6;
                    current_block_14 = 3512920355445576850;
                }
            } else if is_same_net6(&mut context.start6,
                                   &mut addr_list.addr.addr6,
                                   context.prefix) != 0 {
                *addr = addr_list.addr.addr6;
                current_block_14 = 3512920355445576850;
            } else { current_block_14 = 10680521327981672866; }
            match current_block_14 {
                10680521327981672866 => { }
                _ => {
                    i = 0 as u64;
                    while i < addresses {
                        setaddr6part(addr, addrpart.wrapping_add(i));
                        if check_address(state, addr) != 0 {
                            return 1
                        }
                        i = i.wrapping_add(1)
                    }
                }
            }
        }
        addr_list = addr_list.next
    }
    return 0;
}
/* Calculate valid and preferred times to send in leases/renewals. 

   Inputs are:

   *valid_timep, *preferred_timep - requested times from IAADDR options.
   context->valid, context->preferred - times associated with subnet address on local interface.
   context->flags | CONTEXT_DEPRECATE - "deprecated" flag in dhcp-range.
   lease_time - configured time for context for individual client.
   *min_time - smallest valid time sent so far.

   Outputs are :
   
   *valid_timep, *preferred_timep - times to be send in IAADDR option.
   *min_time - smallest valid time sent so far, to calculate T1 and T2.
   
   */
unsafe extern "C" fn calculate_times(mut context: DhcpContext,
                                     mut min_time: &mut libc::c_uint,
                                     mut valid_timep: &mut libc::c_uint,
                                     mut preferred_timep: &mut libc::c_uint,
                                     mut lease_time: u32) {
    let mut req_preferred: u32 = *preferred_timep;
    let mut req_valid: u32 = *valid_timep;
    let mut valid_time: u32 = lease_time;
    let mut preferred_time: u32 = lease_time;
    /* RFC 3315: "A server ignores the lifetimes set
     by the client if the preferred lifetime is greater than the valid
     lifetime. */
    if req_preferred <= req_valid {
        if req_preferred != 0 {
            /* 0 == "no preference from client" */
            if req_preferred < 120 {
                req_preferred = 120
            } /* sanity */
            if req_preferred < preferred_time {
                preferred_time = req_preferred
            }
        }
        if req_valid != 0 {
            /* 0 == "no preference from client" */
            if req_valid < 120 {
                req_valid = 120
            } /* sanity */
            if req_valid < valid_time { valid_time = req_valid }
        }
    }
    /* deprecate (preferred == 0) which configured, or when local address 
     is deprecated */
    if context.flags &
           (1) << 9 != 0 ||
           context.preferred == 0 {
        preferred_time = 0
    }
    if preferred_time != 0 &&
           preferred_time < *min_time {
        *min_time = preferred_time
    }
    if valid_time != 0 &&
           valid_time < *min_time {
        *min_time = valid_time
    }
    *valid_timep = valid_time;
    *preferred_timep = preferred_time;
}
unsafe extern "C" fn update_leases(mut state: &mut state,
                                   mut context: DhcpContext,
                                   mut addr: &mut In6Addr,
                                   mut lease_time: u32,
                                   mut now: time::Instant) {
    let mut lease: DhcpLease =
        lease6_find_by_addr(addr, 128,
                            0 as u64);
    let mut tagif: DhcpNetId = run_tag_if(state.tags);
    if lease.is_null() {
        lease =
            lease6_allocate(addr,
                            if state.ia_type == 3 {
                                32
                            } else { 64 })
    }
    if !lease.is_null() {
        lease_set_expires(lease, lease_time, now);
        lease_set_iaid(lease, state.iaid);
        lease_set_hwaddr(lease, state.mac.as_mut_ptr(), state.clid,
                         state.mac_len,
                         state.mac_type, state.clid_len,
                         now, 0);
        lease_set_interface(lease, state.interface, now);
        if !state.hostname.is_null() &&
               state.ia_type == 3 {
            let mut addr_domain: &mut String = get_domain6(addr);
            if state.send_domain.is_null() {
                state.send_domain = addr_domain
            }
            lease_set_hostname(lease, state.hostname,
                               state.hostname_auth, addr_domain,
                               state.domain);
        }
        if !dnsmasq_daemon.lease_change_command.is_null() {
            let mut class_opt:Vec<u8> = 0;
            lease.flags |= 2;
            // free(lease.extradata);
            lease.extradata = 0;
            lease.extradata_len = 0;
            lease.extradata_size = lease.extradata_len;
            lease.vendorclass_count = 0;
            class_opt =
                opt6_find(state.packet_options, state.end,
                          16,
                          4);
            if !class_opt.is_null() {
                let mut enc_opt:Vec<u8> = 0;
                let mut enc_end:Vec<u8> =
                    &mut *(class_opt).offset((4 +
                                                               (opt6_uint                 unsafe extern "C" fn(_:
                                                                                             mut Vec<u8>,
                                                                                         _:
                                                                                             ,
                                                                                         _:
                                                                                             )
                                                                        ->
                                                                            libc::c_uint)(class_opt           mut Vec<u8>,
                                                                                          -(2               ),
                                                                                          2           )
                                                                                  )
                                                             )                  mut Vec<u8>;
                lease.vendorclass_count += 1;
                /* send enterprise number first  */
                sprintf(dnsmasq_daemon.dhcp_buff2,
                        "%u" ,
                        opt6_uint(class_opt,
                                  0, 4));
                lease_add_extradata(lease,
                                    dnsmasq_daemon.dhcp_buff2                                  mut Vec<u8>,
                                    strlen(dnsmasq_daemon.dhcp_buff2)                                  libc::c_uint, 0);
                if opt6_uint(class_opt,
                             -(2), 2) >= 6 {
                    enc_opt =
                        &mut *(class_opt                             mut Vec<u8>).offset((4
                                                                   +
                                                                   4                    )
                                                                 )                      mut Vec<u8>;
                    while !enc_opt.is_null() {
                        lease.vendorclass_count += 1;
                        lease_add_extradata(lease,
                                            &mut *(enc_opt    mut Vec<u8>).offset((4

                                                                                       +
                                                                                       0     )
                                                                                  )
                                                                                        Vec<u8>                                          mut Vec<u8>,
                                            opt6_uint(enc_opt       mut Vec<u8>,
                                                      -(2),
                                                      2)                                          ,
                                            0);
                        enc_opt = opt6_next(enc_opt, enc_end)
                    }
                }
            }
            lease_add_extradata(lease,
                                state.client_hostname                              mut Vec<u8>,
                                if !state.client_hostname.is_null() {
                                    strlen(state.client_hostname)
                                } else { 0 }
                                   , 0);
            /* space-concat tag set */
            if tagif.is_null() && context.netid.net.is_null() {
                lease_add_extradata(lease, 0,
                                    0,
                                    0);
            } else {
                if !context.netid.net.is_null() {
                    lease_add_extradata(lease,
                                        context.netid.net                                      mut Vec<u8>,
                                        strlen(context.netid.net)                                      libc::c_uint,
                                        if !tagif.is_null() {
                                            ' ' as i32
                                        } else { 0 });
                }
                if !tagif.is_null() {
                    let mut n: DhcpNetId = 0 ;
                    n = tagif;
                    while !n.is_null() {
                        let mut n1: DhcpNetId = 0 ;
                        /* kill dupes */
                        n1 = n.next;
                        while !n1.is_null() {
                            if strcmp(n.net, n1.net) == 0
                               {
                                break ;
                            }
                            n1 = n1.next
                        }
                        if n1.is_null() {
                            lease_add_extradata(lease,
                                                n.net mut Vec<u8>,
                                                strlen(n.net) libc::c_uint,
                                                if !n.next.is_null() {
                                                    ' ' as i32
                                                } else { 0 });
                        }
                        n = n.next
                    }
                }
            }
            if !state.link_address.is_null() {
                inet_ntop(10,
                          state.link_address,
                          dnsmasq_daemon.addrbuff,
                          46);
            }
            lease_add_extradata(lease,
                                dnsmasq_daemon.addrbuff                              mut Vec<u8>,
                                if !state.link_address.is_null() {
                                    strlen(dnsmasq_daemon.addrbuff)
                                } else { 0 }
                                   , 0);
            class_opt =
                opt6_find(state.packet_options, state.end,
                          15,
                          2);
            if !class_opt.is_null() {
                let mut enc_opt_0:Vec<u8> = 0;
                let mut enc_end_0:Vec<u8> =
                    &mut *(class_opt).offset((4 +
                                                               (opt6_uint                 unsafe extern "C" fn(_:
                                                                                             mut Vec<u8>,
                                                                                         _:
                                                                                             ,
                                                                                         _:
                                                                                             )
                                                                        ->
                                                                            libc::c_uint)(class_opt           mut Vec<u8>,
                                                                                          -(2               ),
                                                                                          2           )
                                                                                  )
                                                             )                  mut Vec<u8>;
                enc_opt_0 =
                    &mut *(class_opt).offset((4 +
                                                               0                )
                                                             )                  mut Vec<u8>;
                while !enc_opt_0.is_null() {
                    lease_add_extradata(lease,
                                        &mut *(enc_opt_0mut Vec<u8>).offset((4

                                                                                   +
                                                                                   0
                                                                                                                          )
                                                                          )
                                                                                Vec<u8>                                      mut Vec<u8>,
                                        opt6_uint(enc_opt_0   mut Vec<u8>,
                                                  -(2),
                                                  2) ,
                                        0);
                    enc_opt_0 = opt6_next(enc_opt_0, enc_end_0)
                }
            }
        }
    };
}
unsafe extern "C" fn log6_opts(mut nest: i32, mut xid: u32,
                               mut start_opts:Vec<u8>,
                               mut end_opts:Vec<u8>) {
    let mut opt:Vec<u8> = 0;
    let mut desc: &mut String =
        if nest != 0 {
            "nest"
        } else { "sent"  }      &mut String;
    if dnsmasq_daemon.options[(28).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                                                   ))
                                     ] &
           (1) <<
               (28).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8

                                                                                                               ))
           == 0 || start_opts == end_opts {
        return
    }
    opt = start_opts;
    while !opt.is_null() {
        let mut type_0: i32 =
            opt6_uint(opt, -(4),
                      2);
        let mut ia_options:Vec<u8> = 0;
        let mut optname: &mut String = 0 ;
        if type_0 == 3 {
            sprintf(dnsmasq_daemon.namebuff,
                    "IAID=%u T1=%u T2=%u"                   *const libc::c_char,
                    opt6_uint(opt, 0,
                              4),
                    opt6_uint(opt, 4,
                              4),
                    opt6_uint(opt, 8,
                              4));
            optname =
                "ia-na"  );
            ia_options =
                &mut *(opt                     mut Vec<u8>).offset((4 +
                                                           12)
                                                         )              mut Vec<u8>
        } else if type_0 == 4 {
            sprintf(dnsmasq_daemon.namebuff,
                    "IAID=%u" ,
                    opt6_uint(opt, 0,
                              4));
            optname =
                "ia-ta"  );
            ia_options =
                &mut *(opt                     mut Vec<u8>).offset((4 +
                                                           4)
                                                         )              mut Vec<u8>
        } else if type_0 == 5 {
            let mut addr: In6Addr =
                In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
            /* align */
            memcpy(&mut addr,
                   &mut *(opt).offset((4 +
                                                              0               )
                                                            )                 mut Vec<u8>,
                   16);
            inet_ntop(10,
                      &mut addr,
                      dnsmasq_daemon.addrbuff,
                      46);
            sprintf(dnsmasq_daemon.namebuff,
                    "%s PL=%u VL=%u" ,
                    dnsmasq_daemon.addrbuff,
                    opt6_uint(opt, 16,
                              4),
                    opt6_uint(opt, 20,
                              4));
            optname =
                "iaaddr"  );
            ia_options =
                &mut *(opt                     mut Vec<u8>).offset((4 +
                                                           24)
                                                         )              mut Vec<u8>
        } else if type_0 == 13 {
            let mut len: i32 =
                sprintf(dnsmasq_daemon.namebuff,
                        "%u " ,
                        opt6_uint(opt, 0,
                                  2));
            memcpy(dnsmasq_daemon.namebuff.offset(len)                Vec<u8>,
                   &mut *(opt).offset((4 +
                                                              2               )
                                                            )                 mut Vec<u8>,
                   (opt6_uint(opt, -(2),
                              2) -
                        2));
            *dnsmasq_daemon.namebuff.offset((len +
                                                    opt6_uint(opt               mut Vec<u8>,
                                                              -(2                 ),
                                                              2               )
                                                        -
                                                    2)) =
                0;
            optname =
                "status"  )
        } else {
            /* account for flag byte on FQDN */
            let mut offset: i32 =
                if type_0 == 39 {
                    1
                } else { 0 };
            optname =
                option_string(10, type_0,
                              &mut *(opt                                   mut Vec<u8>).offset((4
                                                                         +
                                                                         offset)
                                                      )

                                 ,
                              opt6_uint(opt,
                                        -(2), 2)
                                  - offset,
                              dnsmasq_daemon.namebuff, 1025)
        }
        my_syslog((3) << 3 | 6,
                  "%u %s size:%3d option:%3d %s  %s", xid, desc,
                  opt6_uint(opt, -(2),
                            2), type_0, optname,
                  dnsmasq_daemon.namebuff);
        if !ia_options.is_null() {
            log6_opts(1, xid, ia_options,
                      &mut *(opt                           mut Vec<u8>).offset((4
                                                                 +
                                                                 (opt6_uint                   unsafe extern "C" fn(_:
                                                                                               mut Vec<u8>,
                                                                                           _:
                                                                                               ,
                                                                                           _:
                                                                                               )
                                                                          ->
                                                                              libc::c_uint)(opt               mut Vec<u8>,
                                                                                            -(2                   ),
                                                                                            2               )
                                                                                      )
                                                               ));
        }
        opt = opt6_next(opt, end_opts)
    };
}
unsafe extern "C" fn log6_quiet(mut state: &mut state,
                                mut type_0: &mut String,
                                mut addr: &mut In6Addr,
                                mut string: &mut String) {
    if dnsmasq_daemon.options[(28).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                                                   ))
                                     ] &
           (1) <<
               (28).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8

                                                                                                               ))
           != 0 ||
           dnsmasq_daemon.options[(43 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                                                   ))
                                         ] &
               (1) <<
                   (43 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8
                                                                                                                       ))
               == 0 {
        log6_packet(state, type_0, addr, string);
    };
}
unsafe extern "C" fn log6_packet(mut state: &mut state,
                                 mut type_0: &mut String,
                                 mut addr: &mut In6Addr,
                                 mut string: &mut String) {
    let mut clid_len: i32 = state.clid_len;
    /* avoid buffer overflow */
    if clid_len > 100 { clid_len = 100 }
    print_mac(dnsmasq_daemon.namebuff, state.clid, clid_len);
    if !addr.is_null() {
        inet_ntop(10, addr,
                  dnsmasq_daemon.dhcp_buff2,
                  (256 - 1));
        strcat(dnsmasq_daemon.dhcp_buff2,
               " " );
    } else {
        *dnsmasq_daemon.dhcp_buff2.offset(0) =
            0
    }
    if dnsmasq_daemon.options[(28).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                                                   ))
                                     ] &
           (1) <<
               (28).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8

                                                                                                               ))
           != 0 {
        my_syslog((3) << 3 | 6,
                  "%u %s(%s) %s%s %s", state.xid, type_0,
                  state.iface_name, dnsmasq_daemon.dhcp_buff2,
                  dnsmasq_daemon.namebuff,
                  if !string.is_null() {
                      string
                  } else { ""  });
    } else {
        my_syslog((3) << 3 | 6,
                  "%s(%s) %s%s %s" ,
                  type_0, state.iface_name, dnsmasq_daemon.dhcp_buff2,
                  dnsmasq_daemon.namebuff,
                  if !string.is_null() {
                      string
                  } else { ""  });
    };
}
unsafe extern "C" fn opt6_find(mut opts:Vec<u8>,
                               mut end:Vec<u8>,
                               mut search: u32,
                               mut minsize: u32)
 ->Vec<u8> {
    let mut opt: u16 = 0;
    let mut opt_len: u16 = 0;
    let mut start:Vec<u8> = 0;
    if opts.is_null() { return 0 }
    loop  {
        if (end.wrapping_offset_from(opts)) <
               4 {
            return 0
        }
        start = opts;
        let mut t_cp: mut Vec<u8> = opts;
        opt =
            ((*t_cp.offset(0))
                 << 8 |
                 *t_cp.offset(1) );
        opts = opts.offset(2);
        let mut t_cp_0: mut Vec<u8> = opts;
        opt_len =
            ((*t_cp_0.offset(0) ) << 8 |
                 *t_cp_0.offset(1) );
        opts = opts.offset(2);
        if opt_len >
               end.wrapping_offset_from(opts) {
            return 0
        }
        if opt == search && opt_len >= minsize
           {
            return start
        }
        opts = opts.offset(opt_len)
    };
}
unsafe extern "C" fn opt6_next(mut opts:Vec<u8>,
                               mut end:Vec<u8>)
 ->Vec<u8> {
    let mut opt_len: u16 = 0;
    if (end.wrapping_offset_from(opts)) <
           4 {
        return 0
    }
    opts = opts.offset(2);
    let mut t_cp: mut Vec<u8> = opts;
    opt_len =
        ((*t_cp.offset(0)) <<
             8 |
             *t_cp.offset(1))
           ;
    opts = opts.offset(2);
    if opt_len >=
           end.wrapping_offset_from(opts) {
        return 0
    }
    return opts.offset(opt_len);
}
unsafe extern "C" fn opt6_uint(mut opt: mut Vec<u8>,
                               mut offset: i32, mut size: i32)
 -> libc::c_uint {
    /* this worries about unaligned data and byte order */
    let mut ret: u32 = 0;
    let mut i: i32 = 0;
    let mut p: mut Vec<u8> =
        &mut *opt.offset((4 + offset))      mut Vec<u8>;
    i = 0;
    while i < size {
        let fresh8 = p;
        p = p.offset(1);
        ret = ret << 8 | *fresh8;
        i += 1
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn relay_upstream6(mut relay: &mut DhcpRelay,
                                         mut sz: susize,
                                         mut peer_address: &mut In6Addr,
                                         mut scope_id: u32,
                                         mut now: time::Instant) {
    /* ->local is same value for all relays on ->current chain */
    let mut from: NetAddress = NetAddress {addr4: NetAddress {s_addr: 0,},};
    let mut header: mut Vec<u8> = 0;
    let mut inbuff: mut Vec<u8> =
        dnsmasq_daemon.dhcp_packet.iov_base;
    let mut msg_type: i32 = *inbuff;
    let mut hopcount: i32 = 0;
    let mut multicast: In6Addr =
        In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    let mut maclen: u32 = 0;
    let mut mactype: u32 = 0;
    let mut mac: [libc::c_uchar; 16] = [0; 16];
    inet_pton(10,
              "FF05::1:3" ,
              &mut multicast);
    get_client_mac(peer_address, scope_id, mac.as_mut_ptr(),
                   &mut maclen, &mut mactype, now);
    /* source address == relay address */
    from.addr6 = relay.local.addr6;
    /* Get hop count from nested relayed message */
    if msg_type == 12 {
        hopcount =
            *inbuff.offset(1) +
                1
    } else { hopcount = 0 }
    /* RFC 3315 HOP_COUNT_LIMIT */
    if hopcount > 32 { return }
    reset_counter();
    header =
        put_opt6(0, 34 )      mut Vec<u8>;
    if !header.is_null() {
        let mut o: i32 = 0;
        *header.offset(0) =
            12;
        *header.offset(1) = hopcount;
        memcpy(&mut *header.offset(2)             mut Vec<u8>,
               &mut relay.local.addr6             *const libc::c_void, 16);
        memcpy(&mut *header.offset(18)             mut Vec<u8>,
               peer_address,
               16);
        /* RFC-6939 */
        if maclen != 0 {
            o = new_opt6(79);
            put_opt6_short(mactype);
            put_opt6(mac.as_mut_ptr(), maclen );
            end_opt6(o);
        }
        o = new_opt6(9);
        put_opt6(inbuff, sz );
        end_opt6(o);
        while !relay.is_null() {
            let mut to: NetAddress =
                NetAddress {sa: NetAddress {sa_family: 0, sa_data: [0; 14],},};
            to.sa.sa_family = 10;
            to.in6.sin6_addr = relay.server.addr6;
            to.in6.sin6_port = __bswap_16(547);
            to.in6.sin6_flowinfo = 0;
            to.in6.sin6_scope_id = 0;
            if ({
                    let mut __a: *const In6Addr =
                        &mut relay.server.addr6                      *const In6Addr;
                    let mut __b: *const In6Addr =
                        &mut multicast ;
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
                let mut multicast_iface: i32 = 0;
                if relay.interface.is_null() ||
                       !strchr(relay.interface, '*' as i32).is_null() ||
                       {
                           multicast_iface =
                               if_nametoindex(relay.interface)                             ;
                           (multicast_iface) == 0
                       } ||
                       setsockopt(dnsmasq_daemon.dhcp6fd,
                                  IPPROTO_IPV6,
                                  17,
                                  &mut multicast_iface as
                                  ::std::mem::size_of::<>()) ==
                           -(1) {
                    my_syslog((3) << 3 |
                                  3,
                              "Cannot multicast to DHCPv6 server without correct interface"
                                  );
                }
            }
            send_from(dnsmasq_daemon.dhcp6fd, 0,
                      dnsmasq_daemon.outpacket.iov_base                    &mut String,
                      save_counter(-(1)) , &mut to,
                      &mut from, 0);
            if dnsmasq_daemon.options[(28                                 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                   ).wrapping_mul(8                                                                                   ))
                                             ] &
                   (1) <<
                       (28 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                               ).wrapping_mul(8           ))
                   != 0 {
                inet_ntop(10,
                          &mut relay.local        dnsmasq_daemon.addrbuff,
                          46);
                inet_ntop(10,
                          &mut relay.server        dnsmasq_daemon.namebuff,
                          46);
                my_syslog((3) << 3 |
                              6,
                          "DHCP relay %s -> %s" , dnsmasq_daemon.addrbuff,
                          dnsmasq_daemon.namebuff);
            }
            /* Save this for replies */
            relay.iface_index = scope_id;
            relay = relay.current
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn relay_reply6(mut peer: NetAddress,
                                      mut sz: susize,
                                      mut arrival_interface:
                                          &mut String)
                                      -> u16 {
    let mut relay: DhcpRelay = 0;
    let mut link: In6Addr =
        In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
    let mut inbuff: mut Vec<u8> =
        dnsmasq_daemon.dhcp_packet.iov_base;
    /* must have at least msg_type+hopcount+link_address+peer_address+minimal size option
     which is               1   +    1   +    16      +     16     + 2 + 2 = 38 */
    if sz < 38 ||
           *inbuff != 13 {
        return 0
    }
    memcpy(&mut link,
           &mut *inbuff.offset(2)         mut Vec<u8>,
           16);
    relay = dnsmasq_daemon.relay6;
    while !relay.is_null() {
        if ({
                let mut __a: *const In6Addr =
                    &mut link ;
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
            }) != 0 &&
               (relay.interface.is_null() ||
                    wildcard_match(relay.interface, arrival_interface) !=
                        0) {
            break ;
        }
        relay = relay.next
    }
    reset_counter();
    if !relay.is_null() {
        let mut opt:Vec<u8> = 0;
        let mut opts:Vec<u8> =
            inbuff.offset(34);
        let mut end:Vec<u8> =
            inbuff.offset(sz);
        opt = opts;
        while !opt.is_null() {
            if opt6_uint(opt, -(4),
                         2) == 9
                   &&
                   opt6_uint(opt, -(2),
                             2) >
                       0 {
                let mut encap_type: i32 =
                    *(&mut *(opt                           mut Vec<u8>).offset((4
                                                                 +
                                                                 0                  )
                                                               ));
                put_opt6(&mut *(opt                              mut Vec<u8>).offset((4
                                                                    +
                                                                    0                     )
                                                                  )
                            ,
                         opt6_uint(opt,
                                   -(2), 2)                        );
                memcpy(&mut peer.sin6_addr                    Vec<u8>,
                       &mut *inbuff.offset(18)                     mut Vec<u8>,
                       16);
                peer.sin6_scope_id = relay.iface_index;
                return if encap_type == 13 {
                           547
                       } else { 546 }
            }
            opt = opt6_next(opt, end)
        }
    }
    return 0 ;
}
