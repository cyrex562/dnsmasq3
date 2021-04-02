use crate::defines::{NetAddress, DhcpContext, time::Instant, DhcpLease, DhcpVendor, DhcpMac, DhcpNetIdList, DhcpPacket, DnsmasqDaemon, DhcpConfig, DhcpNetId, DhcpOpt, __bswap_32, InAddrT, SharedNetwork, DhcpMatchName, AddrList2, PxeService, C2rustUnnamed9, DhcpBoot, __bswap_16, Irec, _ISPRINT, DhcpPxeVendor, socklen_t, DelayConfig};
use crate::util::{expand_buf, memcmp_masked, is_same_net, legal_hostname, hostname_isequal, safe_strncpy, prettyprint_time, print_mac, rand16, whine_malloc, do_rfc1035_name};
use crate::lease::{lease_find_by_client, lease_find_by_addr, lease_prune, lease4_allocate, lease_set_hwaddr, lease_set_hostname, lease_set_expires, lease_set_interface, lease_add_extradata};
use crate::dnsmasq_log::my_syslog;
use crate::dhcp_common::{match_bytes, find_config, run_tag_if, match_netid, log_tags, strip_hostname, config_has_mac, option_string, option_filter};
use crate::dhcp::{address_available, address_allocate, narrow_context, config_find_by_address, do_icmp_ping, host_from_dns};
use crate::domain::get_domain;
use crate::slack::{METRIC_BOOTP, METRIC_PXE, METRIC_DHCPDECLINE, METRIC_DHCPRELEASE, METRIC_DHCPDISCOVER, METRIC_DHCPOFFER, METRIC_DHCPREQUEST, METRIC_DHCPINFORM, METRIC_DHCPACK, METRIC_NOANSWER, METRIC_DHCPNAK};
use crate::cache::a_record_from_hosts;
use crate::network::enumerate_interfaces;
use crate::delay_dhcp;

pub fn dhcp_reply(mut context: DhcpContext,
                                    mut iface_name: &mut String,
                                    mut int_index: i32,
                                    mut sz: usize, mut now: time::Instant,
                                    mut unicast_dest: i32,
                                    mut loopback: i32,
                                    mut is_inform: ,
                                    mut pxe: i32,
                                    mut fallback: NetAddress,
                                    mut recvtime: time::Instant) -> usize {
    let mut opt: mut Vec<u8> = 0;
    let mut clid: mut Vec<u8> = 0;
    let mut ltmp: DhcpLease = 0;
    let mut lease: DhcpLease = 0;
    let mut vendor: *mut DhcpVendor = 0 ;
    let mut mac: *mut DhcpMac = 0 ;
    let mut id_list: *mut DhcpNetIdList = 0;
    let mut clid_len: i32 = 0;
    let mut ignore: i32 = 0;
    let mut do_classes: i32 = 0;
    let mut rapid_commit: i32 = 0;
    let mut selecting: i32 = 0;
    let mut pxearch: i32 = -(1);
    let mut pxevendor: *const libc::c_char = 0;
    let mut mess: DhcpPacket =
        daemon.dhcp_packet.iov_base ;
    let mut end: mut Vec<u8> =
        mess.offset(1);
    let mut real_end: mut Vec<u8> =
        mess.offset(1);
    let mut hostname: &mut String = 0 ;
    let mut offer_hostname: &mut String = 0 ;
    let mut client_hostname: &mut String = 0 ;
    let mut domain: &mut String = 0 ;
    let mut hostname_auth: i32 = 0;
    let mut borken_opt: i32 = 0;
    let mut req_options: mut Vec<u8> = 0;
    let mut message: &mut String = 0 ;
    let mut time: u32 = 0;
    let mut config: *mut DhcpConfig = 0;
    let mut netid: *mut DhcpNetId = 0 ;
    let mut tagif_netid: *mut DhcpNetId = 0 ;
    let mut subnet_addr: NetAddress = NetAddress {s_addr: 0,};
    let mut override_0: NetAddress = NetAddress {s_addr: 0,};
    let mut fuzz: u16 = 0 ;
    let mut mess_type: u32 = 0;
    let mut fqdn_flags: libc::c_uchar = 0;
    let mut agent_id: mut Vec<u8> = 0;
    let mut uuid: mut Vec<u8> = 0;
    let mut emac: mut Vec<u8> = 0;
    let mut vendor_class_len: i32 = 0;
    let mut emac_len: i32 = 0;
    let mut known_id: DhcpNetId =
        DhcpNetId {net: 0 , next: 0 ,};
    let mut iface_id: DhcpNetId =
        DhcpNetId {net: 0 , next: 0 ,};
    let mut cpewan_id: DhcpNetId =
        DhcpNetId {net: 0 , next: 0 ,};
    let mut o: *mut DhcpOpt = 0 ;
    let mut pxe_uuid: [libc::c_uchar; 17] = [0; 17];
    let mut oui: mut Vec<u8> = 0;
    let mut serial: mut Vec<u8> = 0;
    let mut class: mut Vec<u8> = 0;
    override_0.s_addr = 0;
    subnet_addr.s_addr = override_0.s_addr;
    /* set tag with name == interface */
    iface_id.net = iface_name;
    iface_id.next = 0 ;
    netid = &mut iface_id;
    if mess.op != 1 ||
           mess.hlen > 16 {
        return 0
    }
    if mess.htype == 0 &&
           mess.hlen != 0 {
        return 0
    }
    /* check for DHCP rather than BOOTP */
    opt = option_find(mess, sz, 53, 1);
    if !opt.is_null() {
        let mut cookie: u32 =
            __bswap_32(0x63825363);
        /* only insist on a cookie for DHCP. */
        if memcmp(mess.options.as_mut_ptr(),
                  &mut cookie ,
                  ::std::mem::size_of::<u32>()) !=
               0 {
            return 0
        }
        mess_type = option_uint(opt, 0, 1);
        /* two things to note here: expand_buf may move the packet,
	 so reassign mess from daemon->packet. Also, the size
	 sent includes the IP and UDP headers, hence the magic "-28" */
        opt = option_find(mess, sz, 57, 2);
        if !opt.is_null() {
            let mut size: usize =
                (option_uint(opt, 0, 2)               usize).wrapping_sub(28);
            if size > 16384 {
                size = 16384
            } else if size <
                          ::std::mem::size_of::<DhcpPacket>() {
                size = ::std::mem::size_of::<DhcpPacket>()
            }
            if expand_buf(&mut daemon.dhcp_packet, size) != 0 {
                mess =
                    daemon.dhcp_packet.iov_base                  DhcpPacket;
                end = (mess).offset(size);
                real_end = end
            }
        }
        /* Some buggy clients set ciaddr when they shouldn't, so clear that here since
	 it can affect the context-determination code. */
        if !option_find(mess, sz, 50,
                        4).is_null() ||
               mess_type == 1 {
            mess.ciaddr.s_addr = 0
        }
        /* search for device identity from CPEWAN devices, we pass this through to the script */
        opt = option_find(mess, sz, 125, 5);
        if !opt.is_null() {
            let mut elen: u32 = 0;
            let mut offset: u32 = 0;
            let mut len: u32 =
                *opt.offset(1)              libc::c_uint;
            offset = 0;
            while offset < len.wrapping_sub(5)
                  {
                elen =
                    option_uint(opt,
                                offset.wrapping_add(4     libc::c_uint)                              libc::c_int, 1);
                if option_uint(opt, offset, 4)
                       == 3561 &&
                       offset.wrapping_add(elen).wrapping_add(5
                                                                                libc::c_uint)
                           <= len {
                    let mut x: mut Vec<u8> =
                        &mut *opt.offset((2                                        libc::c_uint).wrapping_add(offset.wrapping_add(5                 libc::c_int                 libc::c_uint))
                                            )
                           ;
                    let mut y: mut Vec<u8> =
                        &mut *opt.offset((2                                        libc::c_uint).wrapping_add(offset.wrapping_add(elen).wrapping_add(5                                                       libc::c_int                                                       libc::c_uint))
                                            )
                           ;
                    oui =
                        option_find1(x, y, 1,
                                     1);
                    serial =
                        option_find1(x, y, 2,
                                     1);
                    class =
                        option_find1(x, y, 3,
                                     1);
                    /* If TR069-id is present set the tag "cpewan-id" to facilitate echoing 
		     the gateway id back. Note that the device class is optional */
                    if !oui.is_null() && !serial.is_null() {
                        cpewan_id.net =
                            b"cpewan-id\x00"                           *const libc::c_char ;
                        cpewan_id.next = netid;
                        netid = &mut cpewan_id
                    }
                    break ;
                } else {
                    offset =
                        offset.wrapping_add(elen.wrapping_add(5
                                                                                libc::c_uint))
                }
            }
        }
        opt = option_find(mess, sz, 82, 1);
        if !opt.is_null() {
            /* Any agent-id needs to be copied back out, verbatim, as the last option
	     in the packet. Here, we shift it to the very end of the buffer, if it doesn't
	     get overwritten, then it will be shuffled back at the end of processing.
	     Note that the incoming options must not be overwritten here, so there has to 
	     be enough free space at the end of the packet to copy the option. */
            let mut sopt: mut Vec<u8> = 0;
            let mut total: u32 =
                (*opt.offset(1) +
                     2);
            let mut last_opt: mut Vec<u8> =
                option_find1((&mut *mess.options.as_mut_ptr().offset(0                         libc::c_int
                                                              )
                                                             *mut u8).offset(::std::mem::size_of::<u32>()
                      ),
                             (mess).offset(sz),
                             255, 0);
            if !last_opt.is_null() && last_opt < end.offset(-(total))
               {
                end = end.offset(-(total));
                agent_id = end;
                memcpy(agent_id,
                       opt, total);
            }
            /* look for RFC3527 Link selection sub-option */
            sopt =
                option_find1(&mut *opt.offset((2libc::c_uint).wrapping_add(0
                                                                                                                libc::c_int
                                                                                                                libc::c_uint)
                                                 )                           mut Vec<u8>                           mut Vec<u8>,
                             &mut *opt.offset((2libc::c_uint).wrapping_add(*opt.offset(1           libc::c_int           isize)
                                                                                                                libc::c_int
                                                                                                                libc::c_uint)
                                                 )                           mut Vec<u8>                           mut Vec<u8>, 5,
                             4);
            if !sopt.is_null() { subnet_addr = option_addr(sopt) }
            /* look for RFC5107 server-identifier-override */
            sopt =
                option_find1(&mut *opt.offset((2libc::c_uint).wrapping_add(0
                                                                                                                libc::c_int
                                                                                                                libc::c_uint)
                                                 )                           mut Vec<u8>                           mut Vec<u8>,
                             &mut *opt.offset((2libc::c_uint).wrapping_add(*opt.offset(1           libc::c_int           isize)
                                                                                                                libc::c_int
                                                                                                                libc::c_uint)
                                                 )                           mut Vec<u8>                           mut Vec<u8>, 11,
                             4);
            if !sopt.is_null() { override_0 = option_addr(sopt) }
            let mut current_block_52: u64;
            /* if a circuit-id or remote-is option is provided, exact-match to options. */
            vendor = daemon.dhcp_vendors;
            while !vendor.is_null() {
                let mut search: i32 = 0;
                if (*vendor).match_type == 3 {
                    search = 1;
                    current_block_52 = 7385833325316299293;
                } else if (*vendor).match_type == 4 {
                    search = 2;
                    current_block_52 = 7385833325316299293;
                } else if (*vendor).match_type == 5 {
                    search = 6;
                    current_block_52 = 7385833325316299293;
                } else { current_block_52 = 15970011996474399071; }
                match current_block_52 {
                    7385833325316299293 => {
                        sopt =
                            option_find1(&mut *opt.offset((2            libc::c_uint).wrapping_add(0           libc::c_int           libc::c_uint)
                                                             )                                       mut Vec<u8>                                      Vec<u8>                                       mut Vec<u8>,
                                         &mut *opt.offset((2            libc::c_uint).wrapping_add(*opt.offset(1                                   libc::c_int                                   isize)           libc::c_int           libc::c_uint)
                                                             )                                       mut Vec<u8>                                      Vec<u8>                                       mut Vec<u8>, search,
                                         1);
                        if !sopt.is_null() &&
                               (*vendor).len ==
                                   *sopt.offset(1)                                 libc::c_int &&
                               memcmp(&mut *sopt.offset((2          libc::c_uint).wrapping_add(0       libc::c_int       libc::c_uint)
                                                           )                                    mut Vec<u8>                                   Vec<u8>,
                                      (*vendor).data,
                                      (*vendor).len) ==
                                   0 {
                            (*vendor).netid.next = netid;
                            netid = &mut (*vendor).netid
                        }
                    }
                    _ => { }
                }
                vendor = (*vendor).next
            }
        }
        /* Check for RFC3011 subnet selector - only if RFC3527 one not present */
        if subnet_addr.s_addr == 0 &&
               {
                   opt =
                       option_find(mess, sz, 118,
                                   4);
                   !opt.is_null()
               } {
            subnet_addr = option_addr(opt)
        }
        /* If there is no client identifier option, use the hardware address */
        if daemon.options[(59 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (59 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               == 0 &&
               {
                   opt =
                       option_find(mess, sz, 61,
                                   1);
                   !opt.is_null()
               } {
            clid_len = *opt.offset(1);
            clid =
                &mut *opt.offset((2                                libc::c_uint).wrapping_add(0                  libc::c_int
                                                                                      libc::c_uint)
                                    )             Vec<u8>
        }
        /* do we have a lease in store? */
        lease =
            lease_find_by_client(mess.chaddr.as_mut_ptr(),
                                 mess.hlen,
                                 mess.htype, clid,
                                 clid_len);
        /* If this request is missing a clid, but we've seen one before, 
	 use it again for option matching etc. */
        if !lease.is_null() && clid.is_null() && !(*lease).clid.is_null() {
            clid_len = (*lease).clid_len;
            clid = (*lease).clid
        }
        /* find mac to use for logging and hashing */
        emac =
            extended_hwaddr(mess.htype,
                            mess.hlen,
                            mess.chaddr.as_mut_ptr(), clid_len, clid,
                            &mut emac_len)
    }
    mac = daemon.dhcp_macs;
    while !mac.is_null() {
        if (*mac).hwaddr_len == mess.hlen &&
               ((*mac).hwaddr_type == mess.htype ||
                    (*mac).hwaddr_type == 0) &&
               memcmp_masked((*mac).hwaddr.as_mut_ptr(),
                             mess.chaddr.as_mut_ptr(),
                             mess.hlen, (*mac).mask) != 0 {
            (*mac).netid.next = netid;
            netid = &mut (*mac).netid
        }
        mac = (*mac).next
    }
    /* Determine network for this packet. Our caller will have already linked all the 
     contexts which match the addresses of the receiving interface but if the 
     machine has an address already, or came via a relay, or we have a subnet selector, 
     we search again. If we don't have have a giaddr or explicit subnet selector, 
     use the ciaddr. This is necessary because a  machine which got a lease via a 
     relay won't use the relay to renew. If matching a ciaddr fails but we have a context 
     from the physical network, continue using that to allow correct DHCPNAK generation later. */
    if mess.giaddr.s_addr != 0 || subnet_addr.s_addr != 0 ||
           mess.ciaddr.s_addr != 0 {
        let mut context_tmp: DhcpContext = 0;
        let mut context_new: DhcpContext = 0;
        let mut share: *mut SharedNetwork = 0 ;
        let mut addr: NetAddress = NetAddress {s_addr: 0,};
        let mut force: i32 = 0;
        let mut via_relay: i32 = 0;
        if subnet_addr.s_addr != 0 {
            addr = subnet_addr;
            force = 1
        } else if mess.giaddr.s_addr != 0 {
            addr = mess.giaddr;
            force = 1;
            via_relay = 1
        } else {
            /* If ciaddr is in the hardware derived set of contexts, leave that unchanged */
            addr = mess.ciaddr;
            context_tmp = context;
            while !context_tmp.is_null() {
                if (*context_tmp).netmask.s_addr != 0 &&
                       is_same_net(addr, (*context_tmp).start,
                                   (*context_tmp).netmask) != 0 &&
                       is_same_net(addr, (*context_tmp).end,
                                   (*context_tmp).netmask) != 0 {
                    context_new = context;
                    break ;
                } else { context_tmp = (*context_tmp).current }
            }
        }
        if context_new.is_null() {
            context_tmp = daemon.dhcp;
            while !context_tmp.is_null() {
                let mut netmask: NetAddress = (*context_tmp).netmask;
                /* guess the netmask for relayed networks */
                if (*context_tmp).flags &
                       (1) << 1 == 0 &&
                       (*context_tmp).netmask.s_addr ==
                           0 {
                    if __bswap_32((*context_tmp).start.s_addr) &
                           0x80000000 ==
                           0 &&
                           __bswap_32((*context_tmp).end.s_addr) &
                               0x80000000 ==
                               0 {
                        netmask.s_addr =
                            __bswap_32(0xff000000)
                    } else if __bswap_32((*context_tmp).start.s_addr) &
                                  0xc0000000 ==
                                  0x80000000 &&
                                  __bswap_32((*context_tmp).end.s_addr) &
                                      0xc0000000 ==
                                      0x80000000 {
                        netmask.s_addr =
                            __bswap_32(0xffff0000)
                    } else if __bswap_32((*context_tmp).start.s_addr) &
                                  0xe0000000 ==
                                  0xc0000000 &&
                                  __bswap_32((*context_tmp).end.s_addr) &
                                      0xe0000000 ==
                                      0xc0000000 {
                        netmask.s_addr =
                            __bswap_32(0xffffff00)
                    }
                }
                /* check to see is a context is OK because of a shared address on
		 the relayed subnet. */
                if via_relay != 0 {
                    share = daemon.shared_networks;
                    while !share.is_null() {
                        if !((*share).shared_addr.s_addr ==
                                 0) {
                            if !((*share).if_index != 0 ||
                                     (*share).match_addr.s_addr !=
                                         mess.giaddr.s_addr) {
                                if netmask.s_addr !=
                                       0 &&
                                       is_same_net((*share).shared_addr,
                                                   (*context_tmp).start,
                                                   netmask) != 0 &&
                                       is_same_net((*share).shared_addr,
                                                   (*context_tmp).end,
                                                   netmask) != 0 {
                                    break ;
                                }
                            }
                        }
                        share = (*share).next
                    }
                }
                /* This section fills in context mainly when a client which is on a remote (relayed)
		 network renews a lease without using the relay, after dnsmasq has restarted. */
                if !share.is_null() ||
                       netmask.s_addr != 0 &&
                           is_same_net(addr, (*context_tmp).start, netmask) !=
                               0 &&
                           is_same_net(addr, (*context_tmp).end, netmask) != 0
                   {
                    (*context_tmp).netmask = netmask;
                    if (*context_tmp).local.s_addr ==
                           0 {
                        (*context_tmp).local = fallback
                    }
                    if (*context_tmp).router.s_addr ==
                           0 && share.is_null()
                       {
                        (*context_tmp).router = mess.giaddr
                    }
                    /* fill in missing broadcast addresses for relayed ranges */
                    if (*context_tmp).flags &
                           (1) << 2 == 0 &&
                           (*context_tmp).broadcast.s_addr ==
                               0 {
                        (*context_tmp).broadcast.s_addr =
                            (*context_tmp).start.s_addr |
                                !(*context_tmp).netmask.s_addr
                    }
                    (*context_tmp).current = context_new;
                    context_new = context_tmp
                }
                context_tmp = (*context_tmp).next
            }
        }
        if !context_new.is_null() || force != 0 { context = context_new }
    }
    if context.is_null() {
        my_syslog((3) << 3 | 4,
                  b"no address range available for DHCP request %s %s\x00"                *const u8,
                  if subnet_addr.s_addr != 0 {
                      b"with subnet selector\x00"
                  } else { b"via\x00"  },
                  if subnet_addr.s_addr != 0 {
                      inet_ntoa(subnet_addr)
                  } else if mess.giaddr.s_addr != 0 {
                      inet_ntoa(mess.giaddr)
                  } else { iface_name });
        return 0
    }
    if daemon.options[(28).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (28).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 {
        let mut context_tmp_0: DhcpContext = 0;
        context_tmp_0 = context;
        while !context_tmp_0.is_null() {
            strcpy(daemon.namebuff,
                   inet_ntoa((*context_tmp_0).start));
            if (*context_tmp_0).flags &
                   ((1) << 0 |
                        (1) << 3) != 0 {
                my_syslog((3) << 3 |
                              6,
                          b"%u available DHCP subnet: %s/%s\x00", __bswap_32(mess.xid),
                          daemon.namebuff,
                          inet_ntoa((*context_tmp_0).netmask));
            } else {
                my_syslog((3) << 3 |
                              6,
                          b"%u available DHCP range: %s -- %s\x00"                        *const u8,
                          __bswap_32(mess.xid), daemon.namebuff,
                          inet_ntoa((*context_tmp_0).end));
            }
            context_tmp_0 = (*context_tmp_0).current
        }
    }
    let mut current_block_146: u64;
    /* dhcp-match. If we have hex-and-wildcards, look for a left-anchored match.
     Otherwise assume the option is an array, and look for a matching element. 
     If no data given, existence of the option is enough. This code handles 
     rfc3925 V-I classes too. */
    o = daemon.dhcp_match;
    while !o.is_null() {
        let mut len_0: u32 = 0;
        let mut elen_0: u32 = 0;
        let mut match_0: u32 = 0;
        let mut offset_0: usize = 0;
        let mut o2: usize = 0;
        if (*o).flags & 2048 != 0 {
            opt = option_find(mess, sz, 124, 5);
            if opt.is_null() {
                current_block_146 = 18316056106135622027;
            } else {
                offset_0 = 0 ;
                while offset_0 <
                          (*opt.offset(1)                         libc::c_uint).wrapping_sub(5)
                              {
                    len_0 =
                        option_uint(opt,
                                    offset_0.wrapping_add(4    )
                                       , 1);
                    /* Need to take care that bad data can't run us off the end of the packet */
                    if offset_0.wrapping_add(len_0                                    ).wrapping_add(5
                                                                                                              libc::c_int
                                                                                                       )
                           <=
                           *opt.offset(1) &&
                           option_uint(opt, offset_0,
                                       4) ==
                               (*o).u.encap {
                        o2 =
                            offset_0.wrapping_add(5   libc::c_ulong);
                        while o2 <
                                  offset_0.wrapping_add(len_0  ).wrapping_add(5)
                              {
                            elen_0 =
                                option_uint(opt, o2,
                                            1);
                            if o2.wrapping_add(elen_0libc::c_ulong).wrapping_add(1
                                                                                                                  libc::c_int
                                                                                                           )
                                   <=
                                   *opt.offset(1)                                 libc::c_int                                 libc::c_ulong &&
                                   {
                                       match_0 =
                                           match_bytes(o,
                                                       &mut *opt.offset((2                          libc::c_uint).wrapping_add(o2.wrapping_add(1                                                                       libc::c_int                                                                )                                       libc::c_uint)
                                                              )
                                                                  mut Vec<u8>
                                                                 Vec<u8>
                                                                  mut Vec<u8>,
                                                       elen_0)
                                              ;
                                       (match_0) != 0
                                   } {
                                break ;
                            }
                            o2 =
                                (o2                        ).wrapping_add(elen_0.wrapping_add(1 libc::c_int libc::c_uint)
                                                                               )

                        }
                    }
                    if match_0 != 0 { break ; }
                    offset_0 =
                        (offset_0).wrapping_add(len_0.wrapping_add(5
                                                                                                            libc::c_int
                                                                                                            libc::c_uint)
                                                            )

                }
                current_block_146 = 10720305954121010852;
            }
        } else {
            opt = option_find(mess, sz, (*o).opt, 1);
            if opt.is_null() {
                current_block_146 = 18316056106135622027;
            } else {
                match_0 =
                    match_bytes(o,
                                &mut *opt.offset((2   libc::c_uint).wrapping_add(0
                                                                                                                      libc::c_int
                                                                                                                      libc::c_uint)
                                                    )                              mut Vec<u8>                              mut Vec<u8>,
                                *opt.offset(1)                              libc::c_int);
                current_block_146 = 10720305954121010852;
            }
        }
        match current_block_146 {
            10720305954121010852 => {
                if match_0 != 0 {
                    (*(*o).netid).next = netid;
                    netid = (*o).netid
                }
            }
            _ => { }
        }
        o = (*o).next
    }
    /* user-class options are, according to RFC3004, supposed to contain
     a set of counted strings. Here we check that this is so (by seeing
     if the counts are consistent with the overall option length) and if
     so zero the counts so that we don't get spurious matches between 
     the vendor string and the counts. If the lengths don't add up, we
     assume that the option is a single string and non RFC3004 compliant 
     and just do the substring match. dhclient provides these broken options.
     The code, later, which sends user-class data to the lease-change script
     relies on the transformation done here.
  */
    opt = option_find(mess, sz, 77, 1);
    if !opt.is_null() {
        let mut ucp: mut Vec<u8> =
            &mut *opt.offset((2                            libc::c_uint).wrapping_add(0
                                                                              libc::c_uint)
                                )         Vec<u8>;
        let mut tmp: i32 = 0;
        let mut j: i32 = 0;
        j = 0;
        while j < *opt.offset(1) {
            j += *ucp.offset(j) + 1
        }
        if j == *opt.offset(1) {
            j = 0;
            while j < *opt.offset(1) {
                tmp =
                    j + *ucp.offset(j) +
                        1;
                *ucp.offset(j) = 0;
                j = tmp
            }
        }
    }
    let mut current_block_167: u64;
    vendor = daemon.dhcp_vendors;
    while !vendor.is_null() {
        let mut mopt: i32 = 0;
        if (*vendor).match_type == 1 {
            mopt = 60;
            current_block_167 = 13526015532137226550;
        } else if (*vendor).match_type == 2 {
            mopt = 77;
            current_block_167 = 13526015532137226550;
        } else { current_block_167 = 12227374774078719326; }
        match current_block_167 {
            13526015532137226550 => {
                opt = option_find(mess, sz, mopt, 1);
                if !opt.is_null() {
                    let mut i: i32 = 0;
                    i = 0;
                    while i <=
                              *opt.offset(1)                            libc::c_int - (*vendor).len {
                        if memcmp((*vendor).data,
                                  &mut *opt.offset((2     libc::c_uint).wrapping_add(i
                                                                                                                          libc::c_uint)
                                                      )                                mut Vec<u8>,
                                  (*vendor).len) ==
                               0 {
                            (*vendor).netid.next = netid;
                            netid = &mut (*vendor).netid;
                            break ;
                        } else { i += 1 }
                    }
                }
            }
            _ => { }
        }
        vendor = (*vendor).next
    }
    /* mark vendor-encapsulated options which match the client-supplied vendor class,
     save client-supplied vendor class */
    opt = option_find(mess, sz, 60, 1);
    if !opt.is_null() {
        memcpy(daemon.dhcp_buff3,
               &mut *opt.offset((2                               libc::c_uint).wrapping_add(0))
                                   )            Vec<u8>,
               *opt.offset(1));
        vendor_class_len =
            *opt.offset(1)
    }
    match_vendor_opts(opt, daemon.dhcp_opts);
    if daemon.options[(28).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (28).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 {
        if sanitise(opt, daemon.namebuff) != 0 {
            my_syslog((3) << 3 |
                          6,
                      b"%u vendor class: %s\x00", __bswap_32(mess.xid),
                      daemon.namebuff);
        }
        if sanitise(option_find(mess, sz, 77,
                                1), daemon.namebuff)
               != 0 {
            my_syslog((3) << 3 |
                          6,
                      b"%u user class: %s\x00", __bswap_32(mess.xid),
                      daemon.namebuff);
        }
    }
    mess.op = 2 as u8;
    config =
        find_config(daemon.dhcp_conf, context, clid, clid_len,
                    mess.chaddr.as_mut_ptr(), mess.hlen,
                    mess.htype, 0 ,
                    run_tag_if(netid));
    /* set "known" tag for known hosts */
    if !config.is_null() {
        known_id.net =
            b"known\x00"           &mut String;
        known_id.next = netid;
        netid = &mut known_id
    } else if !find_config(daemon.dhcp_conf,
                           0, clid, clid_len,
                           mess.chaddr.as_mut_ptr(),
                           mess.hlen,
                           mess.htype,
                           0 ,
                           run_tag_if(netid)).is_null() {
        known_id.net =
            b"known-othernet\x00"           &mut String;
        known_id.next = netid;
        netid = &mut known_id
    }
    if mess_type == 0 && pxe == 0 {
        /* BOOTP request */
        let mut id: DhcpNetId =
            DhcpNetId {net: 0 ,
                       next: 0 ,};
        let mut bootp_id: DhcpNetId =
            DhcpNetId {net: 0 ,
                       next: 0 ,};
        let mut logaddr: NetAddress = 0;
        /* must have a MAC addr for bootp */
        if mess.htype == 0 ||
               mess.hlen == 0 ||
               (*context).flags &
                   (1) << 3 != 0 {
            return 0
        } /* BOOTP vend area is only 64 bytes */
        if !config.is_null() &&
               (*config).flags & 1 != 0 {
            message =
                b"disabled\x00"  )
        }
        end = mess.options.as_mut_ptr().offset(64);
        if !config.is_null() &&
               (*config).flags & 16 != 0 {
            hostname = (*config).hostname;
            domain = (*config).domain
        }
        if !config.is_null() {
            let mut list: *mut DhcpNetIdList = 0;
            list = (*config).netid;
            while !list.is_null() {
                (*(*list).list).next = netid;
                netid = (*list).list;
                list = (*list).next
            }
        }
        /* Match incoming filename field as a netid. */
        if mess.file[0 ] != 0 {
            memcpy(daemon.dhcp_buff2,
                   mess.file.as_mut_ptr(),
                   ::std::mem::size_of::<[u8; 128]>() ); /* ensure zero term. */
            *daemon.dhcp_buff2.offset((::std::mem::size_of::<[u8; 128]>()
                                                 ).wrapping_add(1)
                                                    ) =
                0;
            id.net = daemon.dhcp_buff2;
            id.next = netid;
            netid = &mut id
        }
        /* Add "bootp" as a tag to allow different options, address ranges etc
	 for BOOTP clients */
        bootp_id.net =
            b"bootp\x00"           &mut String;
        bootp_id.next = netid;
        netid = &mut bootp_id;
        tagif_netid = run_tag_if(netid);
        id_list = daemon.dhcp_ignore;
        while !id_list.is_null() {
            if match_netid((*id_list).list, tagif_netid, 0) !=
                   0 {
                message =
                    b"ignored\x00"                   &mut String
            }
            id_list = (*id_list).next
        }
        if message.is_null() {
            let mut nailed: i32 = 0;
            if !config.is_null() &&
                   (*config).flags & 32 != 0 {
                nailed = 1;
                logaddr = &mut (*config).addr;
                mess.yiaddr = (*config).addr;
                lease = lease_find_by_addr((*config).addr);
                if !lease.is_null() &&
                       ((*lease).hwaddr_len != mess.hlen ||
                            (*lease).hwaddr_type !=
                                mess.htype ||
                            memcmp((*lease).hwaddr.as_mut_ptr()
                                   mess.chaddr.as_mut_ptr()
                                   (*lease).hwaddr_len) !=
                                0) {
                    message =
                        b"address in use\x00"
                }
            } else {
                lease =
                    lease_find_by_client(mess.chaddr.as_mut_ptr(),
                                         mess.hlen,
                                         mess.htype,
                                         0,
                                         0);
                if lease.is_null() ||
                       address_available(context, (*lease).addr,
                                         tagif_netid).is_null() {
                    if !lease.is_null() {
                        /* lease exists, wrong network. */
                        lease_prune(lease, now);
                        lease = 0
                    }
                    if address_allocate(context, &mut mess.yiaddr,
                                        mess.chaddr.as_mut_ptr(),
                                        mess.hlen,
                                        tagif_netid, now, loopback) == 0 {
                        message =
                            b"no address available\x00"                           *const libc::c_char
                    }
                } else { mess.yiaddr = (*lease).addr }
            }
            if message.is_null() &&
                   {
                       context =
                           narrow_context(context, mess.yiaddr, netid);
                       context.is_null()
                   } {
                message =
                    b"wrong network\x00"

            } else if !(*context).netid.net.is_null() {
                (*context).netid.next = netid;
                tagif_netid = run_tag_if(&mut (*context).netid)
            }
            log_tags(tagif_netid, __bswap_32(mess.xid));
            if message.is_null() && nailed == 0 {
                id_list = daemon.bootp_dynamic;
                while !id_list.is_null() {
                    if (*id_list).list.is_null() ||
                           match_netid((*id_list).list, tagif_netid,
                                       0) != 0 {
                        break ;
                    }
                    id_list = (*id_list).next
                }
                if id_list.is_null() {
                    message =
                        b"no address configured\x00"
                }
            }
            if message.is_null() && lease.is_null() &&
                   {
                       lease = lease4_allocate(mess.yiaddr);
                       lease.is_null()
                   } {
                message =
                    b"no leases left\x00"

            }
            if message.is_null() {
                logaddr = &mut mess.yiaddr;
                lease_set_hwaddr(lease, mess.chaddr.as_mut_ptr(),
                                 0 ,
                                 mess.hlen,
                                 mess.htype,
                                 0, now, 1);
                if !hostname.is_null() {
                    lease_set_hostname(lease, hostname, 1,
                                       get_domain((*lease).addr), domain);
                }
                /* infinite lease unless nailed in dhcp-host line. */
                lease_set_expires(lease,
                                  if !config.is_null() &&
                                         (*config).flags &
                                             8
                                             != 0 {
                                      (*config).lease_time
                                  } else { 0xffffffff }, now);
                lease_set_interface(lease, int_index, now);
                clear_packet(mess, end);
                do_options(context, mess, end, 0,
                           hostname, get_domain(mess.yiaddr), netid,
                           subnet_addr, 0,
                           0, -(1),
                           0, vendor_class_len, now,
                           0xffffffff,
                           0 ,
                           0);
            }
        }
        daemon.metrics[METRIC_BOOTP ] =
            daemon.metrics[METRIC_BOOTP                                    usize].wrapping_add(1);
        log_packet(b"BOOTP\x00" , logaddr,
                   mess.chaddr.as_mut_ptr(), mess.hlen,
                   iface_name, 0 , message, mess.xid);
        return if !message.is_null() {
                   0
               } else { dhcp_packet_size(mess, agent_id, real_end) }
    }
    opt = option_find(mess, sz, 81, 3);
    if !opt.is_null() {
        /* http://tools.ietf.org/wg/dhc/draft-ietf-dhc-fqdn-option/draft-ietf-dhc-fqdn-option-10.txt */
        let mut len_1: i32 =
            *opt.offset(1);
        let mut pq: &mut String = daemon.dhcp_buff;
        let mut pp: mut Vec<u8> = 0;
        let mut op: mut Vec<u8> =
            &mut *opt.offset((2                            libc::c_uint).wrapping_add(0
                                                                              libc::c_uint)
                                )         Vec<u8>;
        fqdn_flags = *op;
        len_1 -= 3;
        op = op.offset(3);
        pp = op;
        /* NB, the following always sets at least one bit */
        if daemon.options[(36 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (36 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               != 0 {
            if fqdn_flags & 0x1 != 0 {
                fqdn_flags =
                    (fqdn_flags | 0x2) ;
                fqdn_flags =
                    (fqdn_flags & !(0x1))  /* set O */
                /* clear S */
            }
            fqdn_flags =
                (fqdn_flags | 0x8)              libc::c_uchar
            /* set N */
        } else {
            if fqdn_flags & 0x1 == 0 {
                fqdn_flags =
                    (fqdn_flags | 0x3)
            }
            fqdn_flags =
                (fqdn_flags & !(0x8))              libc::c_uchar /* set S and O */
            /* clear N */
        }
        if fqdn_flags & 0x4 != 0 {
            while *op != 0 &&
                      (op.offset(*op ).wrapping_offset_from(pp)                     i32) < len_1 {
                memcpy(pq,
                       op.offset(1)    *op);
                pq = pq.offset(*op);
                op =
                    op.offset((*op + 1)                            isize);
                let fresh6 = pq;
                pq = pq.offset(1);
                *fresh6 = '.'
            }
        } else {
            memcpy(pq, op,
                   len_1);
            if len_1 > 0 &&
                   *op.offset((len_1 - 1)) == 0 {
                borken_opt = 1
            }
            pq = pq.offset((len_1 + 1))
        }
        if pq != daemon.dhcp_buff { pq = pq.offset(-1) }
        *pq = 0;
        if legal_hostname(daemon.dhcp_buff) != 0 {
            client_hostname = daemon.dhcp_buff;
            offer_hostname = client_hostname
        }
    } else {
        opt = option_find(mess, sz, 12, 1);
        if !opt.is_null() {
            let mut len_2: i32 =
                *opt.offset(1);
            memcpy(daemon.dhcp_buff,
                   &mut *opt.offset((2                                   libc::c_uint).wrapping_add(0                     libc::c_int
                                                                                            libc::c_uint)
                                       )                Vec<u8>, len_2);
            /* Microsoft clients are broken, and need zero-terminated strings
	 in options. We detect this state here, and do the same in
	 any options we send */
            if len_2 > 0 &&
                   *daemon.dhcp_buff.offset((len_2 -
                                                            1)
                                                          ) == 0 {
                borken_opt = 1
            } else {
                *daemon.dhcp_buff.offset(len_2) =
                    0
            }
            if legal_hostname(daemon.dhcp_buff) != 0 {
                client_hostname = daemon.dhcp_buff
            }
        }
    }
    if !client_hostname.is_null() {
        let mut m: *mut DhcpMatchName = 0 ;
        let mut nl: usize = strlen(client_hostname);
        if daemon.options[(28 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (28 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               != 0 {
            my_syslog((3) << 3 |
                          6,
                      b"%u client provides name: %s\x00", __bswap_32(mess.xid),
                      client_hostname);
        }
        m = daemon.dhcp_name_match;
        while !m.is_null() {
            let mut ml: usize = strlen((*m).name);
            let mut save: libc::c_char = 0;
            if !(nl < ml) {
                if nl > ml {
                    save = *client_hostname.offset(ml);
                    *client_hostname.offset(ml) =
                        0
                }
                if hostname_isequal(client_hostname, (*m).name) != 0 &&
                       (save == 0 ||
                            (*m).wildcard != 0) {
                    (*(*m).netid).next = netid;
                    netid = (*m).netid
                }
                if save != 0 {
                    *client_hostname.offset(ml) = save
                }
            }
            m = (*m).next
        }
    }
    if !config.is_null() &&
           (*config).flags & 16 != 0 {
        hostname = (*config).hostname;
        domain = (*config).domain;
        hostname_auth = 1;
        /* be careful not to send an OFFER with a hostname not matching the DISCOVER. */
        if fqdn_flags != 0 ||
               client_hostname.is_null() ||
               hostname_isequal(hostname, client_hostname) != 0 {
            offer_hostname = hostname
        }
    } else if !client_hostname.is_null() {
        domain = strip_hostname(client_hostname);
        if strlen(client_hostname) != 0 {
            hostname = client_hostname;
            if config.is_null() {
                /* Search again now we have a hostname. 
		 Only accept configs without CLID and HWADDR here, (they won't match)
		 to avoid impersonation by name. */
                let mut new: *mut DhcpConfig =
                    find_config(daemon.dhcp_conf, context,
                                0, 0,
                                mess.chaddr.as_mut_ptr(),
                                mess.hlen,
                                mess.htype, hostname,
                                run_tag_if(netid));
                if !new.is_null() &&
                       !(!new.is_null() &&
                             (*new).flags & 2
                                 != 0) && (*new).hwaddr.is_null() {
                    config = new;
                    /* set "known" tag for known hosts */
                    known_id.net =
                        b"known\x00";
                    known_id.next = netid;
                    netid = &mut known_id
                }
            }
        }
    }
    if !config.is_null() {
        let mut list_0: *mut DhcpNetIdList = 0;
        list_0 = (*config).netid;
        while !list_0.is_null() {
            (*(*list_0).list).next = netid;
            netid = (*list_0).list;
            list_0 = (*list_0).next
        }
    }
    tagif_netid = run_tag_if(netid);
    /* if all the netids in the ignore list are present, ignore this client */
    id_list = daemon.dhcp_ignore;
    while !id_list.is_null() {
        if match_netid((*id_list).list, tagif_netid, 0) != 0 {
            ignore = 1
        }
        id_list = (*id_list).next
    }
    /* If configured, we can override the server-id to be the address of the relay, 
     so that all traffic goes via the relay and can pick up agent-id info. This can be
     configured for all relays, or by address. */
    if daemon.override_0 != 0 &&
           mess.giaddr.s_addr != 0 &&
           override_0.s_addr == 0 {
        if daemon.override_relays.is_null() {
            override_0 = mess.giaddr
        } else {
            let mut l: *mut AddrList2 = 0 2;
            l = daemon.override_relays;
            while !l.is_null() {
                if (*l).addr.s_addr == mess.giaddr.s_addr { break ; }
                l = (*l).next
            }
            if !l.is_null() { override_0 = mess.giaddr }
        }
    }
    /* Can have setting to ignore the client ID for a particular MAC address or hostname */
    if !config.is_null() &&
           (*config).flags & 128 != 0 {
        clid = 0
    }
    /* Check if client is PXE client. */
    if daemon.enable_pxe != 0 &&
           is_pxe_client(mess, sz, &mut pxevendor) != 0 {
        opt = option_find(mess, sz, 97, 17);
        if !opt.is_null() {
            memcpy(pxe_uuid.as_mut_ptr(),
                   &mut *opt.offset((2                                   libc::c_uint).wrapping_add(0                     libc::c_int
                                                                                            libc::c_uint)
                                       )                Vec<u8>, 17);
            uuid = pxe_uuid.as_mut_ptr()
        }
        /* Check if this is really a PXE bootserver request, and handle specially if so. */
        if (mess_type == 3 ||
                mess_type == 8) &&
               {
                   opt =
                       option_find(mess, sz, 43,
                                   1);
                   !opt.is_null()
               } &&
               {
                   opt =
                       option_find1(&mut *opt.offset((2       libc::c_uint).wrapping_add(0 libc::c_int libc::c_uint)
                                                        )                                  mut Vec<u8>                                 Vec<u8>                                  mut Vec<u8>,
                                    &mut *opt.offset((2       libc::c_uint).wrapping_add(*opt.offset(1                         libc::c_int                         isize) libc::c_int libc::c_uint)
                                                        )                                  mut Vec<u8>                                 Vec<u8>                                  mut Vec<u8>, 71,
                                    4);
                   !opt.is_null()
               } {
            let mut service: *mut PxeService = 0 ;
            let mut type_0: i32 =
                option_uint(opt, 0, 2) ;
            let mut layer: i32 =
                option_uint(opt, 2, 2) ;
            let mut save71: [libc::c_uchar; 4] = [0; 4];
            let mut opt71: DhcpOpt =
                DhcpOpt {opt: 0,
                         len: 0,
                         flags: 0,
                         u: C2rustUnnamed9 {encap: 0,},
                         val: 0,
                         netid: 0 ,
                         next: 0 ,};
            if ignore != 0 { return 0  }
            if layer & 0x8000 != 0 {
                my_syslog((3) << 3 |
                              3,
                          b"PXE BIS not supported\x00" );
                return 0
            }
            memcpy(save71.as_mut_ptr(),
                   &mut *opt.offset((2                                   libc::c_uint).wrapping_add(0                     libc::c_int
                                                                                            libc::c_uint)
                                       )                Vec<u8>, 4);
            service = daemon.pxe_services;
            while !service.is_null() {
                if (*service).type_0 == type_0 { break ; }
                service = (*service).next
            }
            while !context.is_null() {
                if match_netid((*context).filter, tagif_netid,
                               1) != 0 &&
                       is_same_net(mess.ciaddr, (*context).start,
                                   (*context).netmask) != 0 {
                    break ;
                }
                context = (*context).current
            }
            if service.is_null() || (*service).basename.is_null() ||
                   context.is_null() {
                return 0
            }
            clear_packet(mess, end);
            mess.yiaddr = mess.ciaddr;
            mess.ciaddr.s_addr = 0;
            if !(*service).sname.is_null() {
                mess.siaddr = a_record_from_hosts((*service).sname, now)
            } else if (*service).server.s_addr !=
                          0 {
                mess.siaddr = (*service).server
            } else { mess.siaddr = (*context).local }
            if !strchr((*service).basename, '.' as i32).is_null() {
                snprintf(mess.file.as_mut_ptr() ,
                         ::std::mem::size_of::<[u8; 128]>(),
                         b"%s\x00" ,
                         (*service).basename);
            } else {
                snprintf(mess.file.as_mut_ptr() ,
                         ::std::mem::size_of::<[u8; 128]>(),
                         b"%s.%d\x00" ,
                         (*service).basename, layer);
            }
            option_put(mess, end, 53, 1,
                       5);
            option_put(mess, end, 54, 4,
                       __bswap_32((*context).local.s_addr));
            pxe_misc(mess, end, uuid, pxevendor);
            prune_vendor_opts(tagif_netid);
            opt71.val = save71.as_mut_ptr();
            opt71.opt = 71;
            opt71.len = 4;
            opt71.flags = 1024;
            opt71.netid = 0 ;
            opt71.next = daemon.dhcp_opts;
            do_encap_opts(&mut opt71, 43, 1024,
                          mess, end, 0);
            log_packet(b"PXE\x00"                      &mut String,
                       &mut mess.yiaddr                    Vec<u8>, emac, emac_len, iface_name,
                       mess.file.as_mut_ptr() ,
                       0 , mess.xid);
            log_tags(tagif_netid, __bswap_32(mess.xid));
            return dhcp_packet_size(mess, agent_id, real_end)
        }
        opt = option_find(mess, sz, 93, 2);
        if !opt.is_null() {
            pxearch =
                option_uint(opt, 0, 2) ;
            /* proxy DHCP here. */
            if mess_type == 1 ||
                   pxe != 0 && mess_type == 3 {
                let mut tmp_0: DhcpContext = 0;
                let mut workaround: i32 = 0;
                tmp_0 = context;
                while !tmp_0.is_null() {
                    if (*tmp_0).flags &
                           (1) << 3 != 0 &&
                           match_netid((*tmp_0).filter, tagif_netid,
                                       1) != 0 {
                        break ;
                    }
                    tmp_0 = (*tmp_0).current
                }
                if !tmp_0.is_null() {
                    let mut boot: *mut DhcpBoot = 0 ;
                    let mut redirect4011: i32 = 0;
                    if !(*tmp_0).netid.net.is_null() {
                        (*tmp_0).netid.next = netid;
                        tagif_netid = run_tag_if(&mut (*tmp_0).netid)
                    }
                    boot = find_boot(tagif_netid);
                    mess.yiaddr.s_addr = 0;
                    if mess_type == 1 ||
                           mess.ciaddr.s_addr ==
                               0 {
                        mess.ciaddr.s_addr = 0;
                        mess.flags =
                            (mess.flags |
                                 __bswap_16(0x8000                                          u16))                          u16
                        /* broadcast */
                    }
                    clear_packet(mess, end);
                    /* Redirect EFI clients to port 4011 */
                    if pxearch >= 6 {
                        redirect4011 = 1;
                        mess.siaddr = (*tmp_0).local
                    }
                    /* Returns true if only one matching service is available. On port 4011, 
		     it also inserts the boot file and server name. */
                    workaround =
                        pxe_uefi_workaround(pxearch, tagif_netid, mess,
                                            (*tmp_0).local, now, pxe);
                    if workaround == 0 && !boot.is_null() {
                        /* Provide the bootfile here, for iPXE, and in case we have no menu items
			 and set discovery_control = 8 */
                        if boot.next_server.s_addr != 0 {
                            mess.siaddr = boot.next_server
                        } else if !boot.tftp_sname.is_null() {
                            mess.siaddr =
                                a_record_from_hosts(boot.tftp_sname, now)
                        }
                        if !boot.file.is_null() {
                            safe_strncpy(mess.file.as_mut_ptr()                                       &mut String, boot.file,
                                         ::std::mem::size_of::<[u8; 128]>()
                                            );
                        }
                    }
                    option_put(mess, end, 53, 1,
                               if mess_type ==
                                      1 {
                                   2
                               } else { 5 });
                    option_put(mess, end, 54, 4,
                               __bswap_32((*tmp_0).local.s_addr));
                    pxe_misc(mess, end, uuid, pxevendor);
                    prune_vendor_opts(tagif_netid);
                    if pxe != 0 && workaround == 0 || redirect4011 == 0 {
                        do_encap_opts(pxe_opts(pxearch, tagif_netid,
                                               (*tmp_0).local, now),
                                      43, 1024,
                                      mess, end, 0);
                    }
                    daemon.metrics[METRIC_PXE                                            usize] =
                        daemon.metrics[METRIC_PXE   usize].wrapping_add(1);
                    log_packet(b"PXE\x00"
                                   ,
                               0, emac, emac_len,
                               iface_name,
                               if ignore != 0 {
                                   b"proxy-ignored\x00"                                  *const libc::c_char
                               } else {
                                   b"proxy\x00"                                  *const libc::c_char
                               } , 0 ,
                               mess.xid);
                    log_tags(tagif_netid, __bswap_32(mess.xid));
                    if ignore == 0 {
                        apply_delay(mess.xid, recvtime, tagif_netid);
                    }
                    return if ignore != 0 {
                               0
                           } else {
                               dhcp_packet_size(mess, agent_id, real_end)
                           }
                }
            }
        }
    }
    /* if we're just a proxy server, go no further */
    if (*context).flags &
           (1) << 3 != 0 || pxe != 0 {
        return 0
    }
    opt = option_find(mess, sz, 55, 0);
    if !opt.is_null() {
        req_options = daemon.dhcp_buff2;
        memcpy(req_options,
               &mut *opt.offset((2                               libc::c_uint).wrapping_add(0))
                                   )            Vec<u8>,
               *opt.offset(1));
        *req_options.offset(*opt.offset(1)                          libc::c_int) =
            255
    }
    's_3991:
        {
            match mess_type {
                4 => {
                    opt =
                        option_find(mess, sz, 54,
                                    4);
                    if opt.is_null() ||
                           option_addr(opt).s_addr !=
                               server_id(context, override_0, fallback).s_addr
                       {
                        return 0
                    }
                    /* sanitise any message. Paranoid? Moi? */
                    sanitise(option_find(mess, sz, 56,
                                         1),
                             daemon.dhcp_buff);
                    opt =
                        option_find(mess, sz, 50,
                                    4);
                    if opt.is_null() { return 0  }
                    daemon.metrics[METRIC_DHCPDECLINE                                            libc::c_int ] =
                        daemon.metrics[METRIC_DHCPDECLINE   libc::c_int   usize].wrapping_add(1);
                    log_packet(b"DHCPDECLINE\x00",
                               &mut *opt.offset((2  libc::c_uint).wrapping_add(0
                                                                                                                    libc::c_int
                                                                                                                    libc::c_uint)
                                                   )                             mut Vec<u8>,
                               emac, emac_len, iface_name,
                               0 ,
                               daemon.dhcp_buff, mess.xid);
                    if !lease.is_null() &&
                           (*lease).addr.s_addr == option_addr(opt).s_addr {
                        lease_prune(lease, now);
                    }
                    if !config.is_null() &&
                           (*config).flags & 32
                               != 0 &&
                           (*config).addr.s_addr == option_addr(opt).s_addr {
                        prettyprint_time(daemon.dhcp_buff,
                                         600);
                        my_syslog((3) << 3 |
                                      4,
                                  b"disabling DHCP static address %s for %s\x00"
                                      ,
                                  inet_ntoa((*config).addr),
                                  daemon.dhcp_buff);
                        (*config).flags |=
                            1024;
                        (*config).decline_time = now
                    } else {
                        /* make sure this host gets a different address next time. */
                        while !context.is_null() {
                            (*context).addr_epoch =
                                (*context).addr_epoch.wrapping_add(1);
                            context = (*context).current
                        }
                    }
                    return 0
                }
                7 => {
                    context =
                        narrow_context(context, mess.ciaddr, tagif_netid);
                    if context.is_null() ||
                           {
                               opt =
                                   option_find(mess, sz, 54,
                                               4);
                               opt.is_null()
                           } ||
                           option_addr(opt).s_addr !=
                               server_id(context, override_0, fallback).s_addr
                       {
                        return 0
                    }
                    if !lease.is_null() &&
                           (*lease).addr.s_addr == mess.ciaddr.s_addr {
                        lease_prune(lease, now);
                    } else {
                        message =
                            b"unknown lease\x00"                           *const libc::c_char
                    }
                    daemon.metrics[METRIC_DHCPRELEASE                                            libc::c_int ] =
                        daemon.metrics[METRIC_DHCPRELEASE   libc::c_int   usize].wrapping_add(1);
                    log_packet(b"DHCPRELEASE\x00",
                               &mut mess.ciaddr                            Vec<u8>, emac, emac_len,
                               iface_name, 0 , message,
                               mess.xid);
                    return 0
                }
                1 => {
                    if ignore != 0 ||
                           !config.is_null() &&
                               (*config).flags &
                                   1 != 0 {
                        if daemon.options[(42).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                           ).wrapping_mul(8
                                                                                                                           ))
                                                         ] &
                               (1) <<
                                   (42 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                       ).wrapping_mul(8))
                               != 0 {
                            return 0
                        }
                        message =
                            b"ignored\x00"
                                ;
                        opt = 0
                    } else {
                        let mut addr_0: NetAddress = NetAddress {s_addr: 0,};
                        let mut conf: NetAddress = NetAddress {s_addr: 0,};
                        conf.s_addr = 0;
                        addr_0.s_addr = conf.s_addr;
                        opt =
                            option_find(mess, sz, 50,
                                        4);
                        if !opt.is_null() { addr_0 = option_addr(opt) }
                        if !config.is_null() &&
                               (*config).flags &
                                   32 != 0 {
                            let mut addrs: &mut String =
                                inet_ntoa((*config).addr);
                            ltmp = lease_find_by_addr((*config).addr);
                            if !ltmp.is_null() && ltmp != lease &&
                                   config_has_mac(config,
                                                  (*ltmp).hwaddr.as_mut_ptr(),
                                                  (*ltmp).hwaddr_len,
                                                  (*ltmp).hwaddr_type) == 0 {
                                let mut len_3: i32 = 0;
                                let mut mac_0: mut Vec<u8> =
                                    extended_hwaddr((*ltmp).hwaddr_type,
                                                    (*ltmp).hwaddr_len,
                                                    (*ltmp).hwaddr.as_mut_ptr(),
                                                    (*ltmp).clid_len,
                                                    (*ltmp).clid, &mut len_3);
                                my_syslog((3) <<
                                              3 |
                                              4,
                                          b"not using configured address %s because it is leased to %s\x00"
                                                                                      *const libc::c_char, addrs,
                                          print_mac(daemon.namebuff,
                                                    mac_0, len_3));
                            } else {
                                let mut tmp_1: DhcpContext =
                                    0;
                                tmp_1 = context;
                                while !tmp_1.is_null() {
                                    if (*context).router.s_addr ==
                                           (*config).addr.s_addr {
                                        break ;
                                    }
                                    tmp_1 = (*tmp_1).current
                                }
                                if !tmp_1.is_null() {
                                    my_syslog((3) <<
                                                  3 |
                                                  4,
                                              b"not using configured address %s because it is in use by the server or relay\x00"
                                                                                              *const libc::c_char, addrs);
                                } else if !config.is_null() &&
                                              (*config).flags &
                                                  1024   libc::c_uint != 0 &&
                                              difftime(now,
                                                       (*config).decline_time)
                                                  <
                                                  600   libc::c_float   libc::c_double {
                                    my_syslog((3) <<
                                                  3 |
                                                  4,
                                              b"not using configured address %s because it was previously declined\x00"
                                                                                              *const libc::c_char, addrs);
                                } else { conf = (*config).addr }
                            }
                        }
                        if conf.s_addr != 0 {
                            mess.yiaddr = conf
                        } else if !lease.is_null() &&
                                      !address_available(context,
                                                         (*lease).addr,
                                                         tagif_netid).is_null()
                                      &&
                                      config_find_by_address(daemon.dhcp_conf,
                                                             (*lease).addr).is_null()
                         {
                            mess.yiaddr = (*lease).addr
                        } else if !opt.is_null() &&
                                      !address_available(context, addr_0,
                                                         tagif_netid).is_null()
                                      && lease_find_by_addr(addr_0).is_null()
                                      &&
                                      config_find_by_address(daemon.dhcp_conf,
                                                             addr_0).is_null()
                                      &&
                                      !do_icmp_ping(now, addr_0,
                                                    0     libc::c_uint,
                                                    loopback).is_null() {
                            mess.yiaddr = addr_0
                        } else if emac_len == 0 {
                            message =
                                b"no unique-id\x00"                               *const libc::c_char
                        } else if address_allocate(context,
                                                   &mut mess.yiaddr, emac,
                                                   emac_len, tagif_netid, now,
                                                   loopback) == 0 {
                            message =
                                b"no address available\x00"                               *const libc::c_char
                        }
                    }
                    daemon.metrics[METRIC_DHCPDISCOVER                                            libc::c_int ] =
                        daemon.metrics[METRIC_DHCPDISCOVER   libc::c_int   usize].wrapping_add(1);
                    log_packet(b"DHCPDISCOVER\x00",
                               if !opt.is_null() {
                                   &mut *opt.offset((2      libc::c_uint).wrapping_add(0
                                                                                                                            libc::c_int
                                                                                                                            libc::c_uint)
                                                       )                                 mut Vec<u8>
                               } else { 0 }, emac,
                               emac_len, iface_name, 0 ,
                               message, mess.xid);
                    if !message.is_null() ||
                           {
                               context =
                                   narrow_context(context, mess.yiaddr,
                                                  tagif_netid);
                               context.is_null()
                           } {
                        return 0
                    }
                    if !(*context).netid.net.is_null() {
                        (*context).netid.next = netid;
                        tagif_netid = run_tag_if(&mut (*context).netid)
                    }
                    log_tags(tagif_netid, __bswap_32(mess.xid));
                    apply_delay(mess.xid, recvtime, tagif_netid);
                    if daemon.options[(57   libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                   ).wrapping_mul(8                                                             libc::c_int                                                      ))
                                                     ] &
                           (1) <<
                               (57                       ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                               ).wrapping_mul(8                         libc::c_int                  ))
                           != 0 &&
                           !option_find(mess, sz, 80,
                                        0).is_null() {
                        rapid_commit = 1
                    } else {
                        daemon.metrics[METRIC_DHCPOFFER   libc::c_int ] =
                            daemon.metrics[METRIC_DHCPOFFER       libc::c_int       usize].wrapping_add(1);
                        log_packet(b"DHCPOFFER\x00",
                                   &mut mess.yiaddr                                Vec<u8>, emac, emac_len,
                                   iface_name, 0 ,
                                   0 , mess.xid);
                        time =
                            calc_time(context, config,
                                      option_find(mess, sz, 51,
                                                  4));
                        clear_packet(mess, end);
                        option_put(mess, end, 53,
                                   1,
                                   2);
                        option_put(mess, end, 54,
                                   4,
                                   __bswap_32(server_id(context, override_0,
                                                        fallback).s_addr));
                        option_put(mess, end, 51,
                                   4, time);
                        /* T1 and T2 are required in DHCPOFFER by HP's wacky Jetdirect client. */
                        do_options(context, mess, end, req_options,
                                   offer_hostname, get_domain(mess.yiaddr),
                                   netid, subnet_addr, fqdn_flags, borken_opt,
                                   pxearch, uuid, vendor_class_len, now, time,
                                   fuzz, pxevendor);
                        return dhcp_packet_size(mess, agent_id, real_end)
                    }
                }
                3 => {
                    if ignore != 0 ||
                           !config.is_null() &&
                               (*config).flags &
                                   1 != 0 {
                        return 0
                    }
                    opt =
                        option_find(mess, sz, 50,
                                    4);
                    if !opt.is_null() {
                        /* SELECTING  or INIT_REBOOT */
                        mess.yiaddr = option_addr(opt);
                        /* send vendor and user class info for new or recreated lease */
                        do_classes = 1;
                        opt =
                            option_find(mess, sz, 54,
                                        4);
                        if !opt.is_null() {
                            /* SELECTING */
                            selecting = 1;
                            if override_0.s_addr !=
                                   0 {
                                if option_addr(opt).s_addr !=
                                       override_0.s_addr {
                                    return 0
                                }
                            } else {
                                while !context.is_null() {
                                    if (*context).local.s_addr ==
                                           option_addr(opt).s_addr {
                                        break ;
                                    }
                                    context = (*context).current
                                }
                                if context.is_null() {
                                    /* Handle very strange configs where clients have more than one route to the server.
			 If a clients idea of its server-id matches any of our DHCP interfaces, we let it pass.
			 Have to set override to make sure we echo back the correct server-id */
                                    let mut intr: *mut Irec = 0 ;
                                    enumerate_interfaces(0);
                                    intr = daemon.interfaces;
                                    while !intr.is_null() {
                                        if (*intr).addr.sa.sa_family                                         libc::c_int == 2
                                               &&
                                               (*intr).addr.in_0.sin_addr.s_addr
                                                   == option_addr(opt).s_addr
                                               && (*intr).tftp_ok != 0 {
                                            break ;
                                        }
                                        intr = (*intr).next
                                    }
                                    if !intr.is_null() {
                                        override_0 =
                                            (*intr).addr.in_0.sin_addr
                                    } else {
                                        /* In auth mode, a REQUEST sent to the wrong server
			     should be faulted, so that the client establishes 
			     communication with us, otherwise, silently ignore. */
                                        if daemon.options[(17                       libc::c_int
                                                                                         ).wrapping_div((::std::mem::size_of::<libc::c_uint>()                              ).wrapping_mul(8                                                                                                     libc::c_int                                                                                              ))
                                                                                              usize]
                                               &
                                               (1) <<
                                                   (17     libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                                       ).wrapping_mul(8                                                                 libc::c_int                                                          ))
                                               == 0 {
                                            return 0
                                        }
                                        message =
                                            b"wrong server-ID\x00"                                          *const u8                                          *const libc::c_char                                          &mut String
                                    }
                                }
                            }
                            /* If a lease exists for this host and another address, squash it. */
                            if !lease.is_null() &&
                                   (*lease).addr.s_addr !=
                                       mess.yiaddr.s_addr {
                                lease_prune(lease, now);
                                lease = 0
                            }
                        } else {
                            /* INIT-REBOOT */
                            if lease.is_null() &&
                                   daemon.options[(17               libc::c_int
                                                                         ).wrapping_div((::std::mem::size_of::<libc::c_uint>()              ).wrapping_mul(8                                                                                     libc::c_int                                                                              ))
                                                                 ] &
                                       (1) <<
                                           (17                                   ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                       ).wrapping_mul(8                                                 libc::c_int                                          ))
                                       == 0 {
                                return 0
                            }
                            if !lease.is_null() &&
                                   (*lease).addr.s_addr !=
                                       mess.yiaddr.s_addr {
                                message =
                                    b"wrong address\x00"                                   *const libc::c_char                                  &mut String
                            }
                        }
                    } else {
                        /* RENEWING or REBINDING */ 
	  /* Check existing lease for this address.
	     We allow it to be missing if dhcp-authoritative mode
	     as long as we can allocate the lease now - checked below.
	     This makes for a smooth recovery from a lost lease DB */
                        if !lease.is_null() &&
                               mess.ciaddr.s_addr != (*lease).addr.s_addr
                               ||
                               lease.is_null() &&
                                   daemon.options[(17               libc::c_int
                                                                         ).wrapping_div((::std::mem::size_of::<libc::c_uint>()              ).wrapping_mul(8                                                                                     libc::c_int                                                                              ))
                                                                 ] &
                                       (1) <<
                                           (17                                   ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                                       ).wrapping_mul(8                                                 libc::c_int                                          ))
                                       == 0 {
                            /* A client rebinding will broadcast the request, so we may see it even 
		 if the lease is held by another server. Just ignore it in that case. 
		 If the request is unicast to us, then somethings wrong, NAK */
                            if unicast_dest == 0 {
                                return 0
                            }
                            message =
                                b"lease not found\x00"                               *const libc::c_char ;
                            /* ensure we broadcast NAK */
                            unicast_dest = 0
                        }
                        /* desynchronise renewals */
                        fuzz = rand16();
                        mess.yiaddr = mess.ciaddr
                    }
                    daemon.metrics[METRIC_DHCPREQUEST                                            libc::c_int ] =
                        daemon.metrics[METRIC_DHCPREQUEST   libc::c_int   usize].wrapping_add(1);
                    log_packet(b"DHCPREQUEST\x00",
                               &mut mess.yiaddr                            Vec<u8>, emac, emac_len,
                               iface_name, 0 ,
                               0 , mess.xid);
                }
                8 => {
                    if ignore != 0 ||
                           !config.is_null() &&
                               (*config).flags &
                                   1 != 0 {
                        message =
                            b"ignored\x00"

                    }
                    daemon.metrics[METRIC_DHCPINFORM
                                                  ] =
                        daemon.metrics[METRIC_DHCPINFORM   libc::c_int   usize].wrapping_add(1);
                    log_packet(b"DHCPINFORM\x00",
                               &mut mess.ciaddr                            Vec<u8>, emac, emac_len,
                               iface_name, message, 0 ,
                               mess.xid);
                    if !message.is_null() ||
                           mess.ciaddr.s_addr ==
                               0 {
                        return 0
                    }
                    /* For DHCPINFORM only, cope without a valid context */
                    context =
                        narrow_context(context, mess.ciaddr, tagif_netid);
                    /* Find a least based on IP address if we didn't
	 get one from MAC address/client-d */
                    if lease.is_null() &&
                           {
                               lease = lease_find_by_addr(mess.ciaddr);
                               !lease.is_null()
                           } && !(*lease).hostname.is_null() {
                        hostname = (*lease).hostname
                    }
                    if hostname.is_null() {
                        hostname = host_from_dns(mess.ciaddr)
                    }
                    if !context.is_null() && !(*context).netid.net.is_null() {
                        (*context).netid.next = netid;
                        tagif_netid = run_tag_if(&mut (*context).netid)
                    }
                    log_tags(tagif_netid, __bswap_32(mess.xid));
                    daemon.metrics[METRIC_DHCPACK                                            usize] =
                        daemon.metrics[METRIC_DHCPACK   libc::c_int   usize].wrapping_add(1);
                    log_packet(b"DHCPACK\x00",
                               &mut mess.ciaddr                            Vec<u8>, emac, emac_len,
                               iface_name, hostname, 0 ,
                               mess.xid);
                    if !lease.is_null() {
                        lease_set_interface(lease, int_index, now);
                        if override_0.s_addr !=
                               0 {
                            (*lease).override_0 = override_0
                        } else { override_0 = (*lease).override_0 }
                    }
                    clear_packet(mess, end);
                    option_put(mess, end, 53, 1,
                               5);
                    option_put(mess, end, 54, 4,
                               __bswap_32(server_id(context, override_0,
                                                    fallback).s_addr));
                    /* RFC 2131 says that DHCPINFORM shouldn't include lease-time parameters, but 
	 we supply a utility which makes DHCPINFORM requests to get this information.
	 Only include lease time if OPTION_LEASE_TIME is in the parameter request list,
	 which won't be true for ordinary clients, but will be true for the 
	 dhcp_lease_time utility. */
                    if !lease.is_null() &&
                           in_list(req_options, 51) != 0 {
                        if (*lease).expires ==
                               0 {
                            time = 0xffffffff
                        } else {
                            time =
                                difftime((*lease).expires, now)                              libc::c_uint
                        } /* handle reply differently */
                        option_put(mess, end, 51,
                                   4, time);
                    }
                    do_options(context, mess, end, req_options, hostname,
                               get_domain(mess.ciaddr), netid, subnet_addr,
                               fqdn_flags, borken_opt, pxearch, uuid,
                               vendor_class_len, now,
                               0xffffffff,
                               0 , pxevendor);
                    *is_inform = 1;
                    return dhcp_packet_size(mess, agent_id, real_end)
                }
                _ => { break 's_3991 ; }
            }
            if message.is_null() {
                let mut addr_config: *mut DhcpConfig = 0;
                let mut tmp_2: DhcpContext = 0;
                if !config.is_null() &&
                       (*config).flags & 32 !=
                           0 {
                    tmp_2 = context;
                    while !tmp_2.is_null() {
                        if (*context).router.s_addr == (*config).addr.s_addr {
                            break ;
                        }
                        tmp_2 = (*tmp_2).current
                    }
                }
                context =
                    narrow_context(context, mess.yiaddr, tagif_netid);
                if context.is_null() {
                    /* If a machine moves networks whilst it has a lease, we catch that here. */
                    message =
                        b"wrong network\x00"  ;
                    /* ensure we broadcast NAK */
                    unicast_dest = 0
                } else if address_available(context, mess.yiaddr,
                                            tagif_netid).is_null() &&
                              (!(!config.is_null() &&
                                     (*config).flags &
                                         32 !=
                                         0) ||
                                   (*config).addr.s_addr !=
                                       mess.yiaddr.s_addr) {
                    message =
                        b"address not available\x00"
                } else if tmp_2.is_null() && selecting == 0 &&
                              (!config.is_null() &&
                                   (*config).flags &
                                       32 != 0)
                              &&
                              (!(!config.is_null() &&
                                     (*config).flags &
                                         1024
                                         != 0) ||
                                   difftime(now, (*config).decline_time) >
                                       600                                      libc::c_double) &&
                              (*config).addr.s_addr != mess.yiaddr.s_addr
                              &&
                              {
                                  ltmp = lease_find_by_addr((*config).addr);
                                  (ltmp.is_null()) || ltmp == lease
                              } {
                    message =
                        b"static lease available\x00"
                } else {
                    /* Check for renewal of a lease which is outside the allowed range. */
                    /* Check if a new static address has been configured. Be very sure that
	     when the client does DISCOVER, it will get the static address, otherwise
	     an endless protocol loop will ensue. */
                    /* Check to see if the address is reserved as a static address for another host */
                    addr_config =
                        config_find_by_address(daemon.dhcp_conf,
                                               mess.yiaddr);
                    if !addr_config.is_null() && addr_config != config {
                        message =
                            b"address reserved\x00"                           *const libc::c_char
                    } else if lease.is_null() &&
                                  {
                                      ltmp =
                                          lease_find_by_addr(mess.yiaddr);
                                      !ltmp.is_null()
                                  } {
                        /* If a host is configured with more than one MAC address, it's OK to 'nix 
		 a lease from one of it's MACs to give the address to another. */
                        if !config.is_null() &&
                               config_has_mac(config,
                                              (*ltmp).hwaddr.as_mut_ptr(),
                                              (*ltmp).hwaddr_len,
                                              (*ltmp).hwaddr_type) != 0 {
                            my_syslog((3) << 3 |
                                          6,
                                      b"abandoning lease to %s of %s\x00"                                    *const u8,
                                      print_mac(daemon.namebuff,
                                                (*ltmp).hwaddr.as_mut_ptr(),
                                                (*ltmp).hwaddr_len),
                                      inet_ntoa((*ltmp).addr));
                            lease = ltmp
                        } else {
                            message =
                                b"address in use\x00"                               *const libc::c_char
                        }
                    }
                }
                if message.is_null() {
                    if emac_len == 0 {
                        message =
                            b"no unique-id\x00"                           *const libc::c_char
                    } else if lease.is_null() {
                        lease = lease4_allocate(mess.yiaddr);
                        if !lease.is_null() {
                            do_classes = 1
                        } else {
                            message =
                                b"no leases left\x00"                               *const libc::c_char
                        }
                    }
                }
            }
            if !message.is_null() {
                daemon.metrics[if rapid_commit != 0 {
                                              METRIC_NOANSWER
                                          } else {
                                              METRIC_DHCPNAK
                                          } ] =
                    daemon.metrics[if rapid_commit != 0 {
                                                  METRIC_NOANSWER   libc::c_int
                                              } else {
                                                  METRIC_DHCPNAK   libc::c_int
                                              } ].wrapping_add(1);
                log_packet(if rapid_commit != 0 {
                               b"NOANSWER\x00"                              *const libc::c_char
                           } else {
                               b"DHCPNAK\x00"                              *const libc::c_char
                           } ,
                           &mut mess.yiaddr, emac, emac_len, iface_name,
                           0 , message, mess.xid);
                /* rapid commit case: lease allocate failed but don't send DHCPNAK */
                if rapid_commit != 0 { return 0  }
                mess.yiaddr.s_addr = 0;
                clear_packet(mess, end);
                option_put(mess, end, 53, 1,
                           6);
                option_put(mess, end, 54, 4,
                           __bswap_32(server_id(context, override_0,
                                                fallback).s_addr));
                option_put_string(mess, end, 56, message,
                                  borken_opt);
                /* This fixes a problem with the DHCP spec, broadcasting a NAK to a host on 
	     a distant subnet which unicast a REQ to us won't work. */
                if unicast_dest == 0 ||
                       mess.giaddr.s_addr !=
                           0 ||
                       mess.ciaddr.s_addr ==
                           0 ||
                       is_same_net((*context).local, mess.ciaddr,
                                   (*context).netmask) != 0 {
                    mess.flags =
                        (mess.flags |
                             __bswap_16(0x8000)
                                ); /* broadcast */
                    mess.ciaddr.s_addr = 0
                }
            } else {
                if !(*context).netid.net.is_null() {
                    (*context).netid.next = netid;
                    tagif_netid = run_tag_if(&mut (*context).netid)
                }
                log_tags(tagif_netid, __bswap_32(mess.xid));
                if do_classes != 0 {
                    /* pick up INIT-REBOOT events. */
                    (*lease).flags |= 2;
                    if !daemon.lease_change_command.is_null() {
                        let mut n: *mut DhcpNetId = 0 ;
                        if mess.giaddr.s_addr != 0 {
                            (*lease).giaddr = mess.giaddr
                        }
                        free((*lease).extradata);
                        (*lease).extradata = 0;
                        (*lease).extradata_len =
                            0;
                        (*lease).extradata_size = (*lease).extradata_len;
                        add_extradata_opt(lease,
                                          option_find(mess, sz,
                                                      60,
                                                      1));
                        add_extradata_opt(lease,
                                          option_find(mess, sz,
                                                      12,
                                                      1));
                        add_extradata_opt(lease, oui);
                        add_extradata_opt(lease, serial);
                        add_extradata_opt(lease, class);
                        opt =
                            option_find(mess, sz, 82,
                                        1);
                        if !opt.is_null() {
                            add_extradata_opt(lease,
                                              option_find1(&mut *opt.offset((2
                                                                                                              libc::c_uint).wrapping_add(0                                               libc::c_int                                               libc::c_uint)
                                                                      )
                                                                          mut Vec<u8>
                                                                         Vec<u8>
                                                                          mut Vec<u8>,
                                                           &mut *opt.offset((2
                                                                                                              libc::c_uint).wrapping_add(*opt.offset(1                                                                       libc::c_int                                 )                                               libc::c_int                                               libc::c_uint)
                                                                      )
                                                                          mut Vec<u8>
                                                                         Vec<u8>
                                                                          mut Vec<u8>,
                                                           1,
                                                           1));
                            add_extradata_opt(lease,
                                              option_find1(&mut *opt.offset((2
                                                                                                              libc::c_uint).wrapping_add(0                                               libc::c_int                                               libc::c_uint)
                                                                      )
                                                                          mut Vec<u8>
                                                                         Vec<u8>
                                                                          mut Vec<u8>,
                                                           &mut *opt.offset((2
                                                                                                              libc::c_uint).wrapping_add(*opt.offset(1                                                                       libc::c_int                                 )                                               libc::c_int                                               libc::c_uint)
                                                                      )
                                                                          mut Vec<u8>
                                                                         Vec<u8>
                                                                          mut Vec<u8>,
                                                           6,
                                                           1));
                            add_extradata_opt(lease,
                                              option_find1(&mut *opt.offset((2
                                                                                                              libc::c_uint).wrapping_add(0                                               libc::c_int                                               libc::c_uint)
                                                                      )
                                                                          mut Vec<u8>
                                                                         Vec<u8>
                                                                          mut Vec<u8>,
                                                           &mut *opt.offset((2
                                                                                                              libc::c_uint).wrapping_add(*opt.offset(1                                                                       libc::c_int                                 )                                               libc::c_int                                               libc::c_uint)
                                                                      )
                                                                          mut Vec<u8>
                                                                         Vec<u8>
                                                                          mut Vec<u8>,
                                                           2,
                                                           1));
                        } else {
                            add_extradata_opt(lease, 0);
                            add_extradata_opt(lease, 0);
                            add_extradata_opt(lease, 0);
                        }
                        /* DNSMASQ_REQUESTED_OPTIONS */
                        opt =
                            option_find(mess, sz, 55,
                                        1);
                        if !opt.is_null() {
                            let mut len_4: i32 =
                                *opt.offset(1)                              libc::c_int;
                            let mut rop: mut Vec<u8> =
                                &mut *opt.offset((2   libc::c_uint).wrapping_add(0
                                                                                                                      libc::c_int
                                                                                                                      libc::c_uint)
                                                    )                              mut Vec<u8>                              mut Vec<u8>;
                            let mut q: &mut String =
                                daemon.namebuff;
                            let mut i_0: i32 = 0;
                            i_0 = 0;
                            while i_0 < len_4 {
                                q =
                                    q.offset(snprintf(q,
                                                      (1025        i32 -
                                                           q.wrapping_offset_from(daemon.namebuff)
                                                                          i32)
                                                         ,
                                                      b"%d%s\x00"                                   *const libc::c_char,
                                                      *rop.offset(i_0                   isize)
                                                         ,
                                                      if i_0 +
                                                             1
                                                             == len_4 {
                                                          b"\x00"                                           *const libc::c_char
                                                      } else {
                                                          b",\x00"           *const u8           *const libc::c_char
                                                      }));
                                i_0 += 1
                            }
                            lease_add_extradata(lease,
                                                daemon.namebuff mut Vec<u8>,
                                                q.wrapping_offset_from(daemon.namebuff)
                                                    libc::c_uint,
                                                0);
                        } else {
                            add_extradata_opt(lease, 0);
                        }
                        /* space-concat tag set */
                        if tagif_netid.is_null() {
                            add_extradata_opt(lease, 0);
                        } else {
                            n = tagif_netid;
                            while !n.is_null() {
                                let mut n1: *mut DhcpNetId =
                                    0 ;
                                /* kill dupes */
                                n1 = (*n).next;
                                while !n1.is_null() {
                                    if strcmp((*n).net, (*n1).net) ==
                                           0 {
                                        break ;
                                    }
                                    n1 = (*n1).next
                                }
                                if n1.is_null() {
                                    lease_add_extradata(lease,
                                                        (*n).net         mut Vec<u8>,
                                                        strlen((*n).net)         libc::c_uint,
                                                        if !(*n).next.is_null()
                                                           {
                                                            ' ' as i32
                                                        } else {
                                                            0
                                                        });
                                }
                                n = (*n).next
                            }
                        }
                        opt =
                            option_find(mess, sz, 77,
                                        1);
                        if !opt.is_null() {
                            let mut len_5: i32 =
                                *opt.offset(1)                              libc::c_int;
                            let mut ucp_0: mut Vec<u8> =
                                &mut *opt.offset((2   libc::c_uint).wrapping_add(0
                                                                                                                      libc::c_int
                                                                                                                      libc::c_uint)
                                                    )                              mut Vec<u8>                              mut Vec<u8>;
                            /* If the user-class option started as counted strings, the first byte will be zero. */
                            if len_5 != 0 &&
                                   *ucp_0.offset(0)                                 libc::c_int == 0 {
                                ucp_0 = ucp_0.offset(1);
                                len_5 -= 1
                            }
                            lease_add_extradata(lease, ucp_0,
                                                len_5,
                                                -(1));
                        }
                    }
                }
                if hostname_auth == 0 &&
                       {
                           client_hostname = host_from_dns(mess.yiaddr);
                           !client_hostname.is_null()
                       } {
                    domain = get_domain(mess.yiaddr);
                    hostname = client_hostname;
                    hostname_auth = 1
                }
                time =
                    calc_time(context, config,
                              option_find(mess, sz, 51,
                                          4));
                lease_set_hwaddr(lease, mess.chaddr.as_mut_ptr(), clid,
                                 mess.hlen,
                                 mess.htype, clid_len, now,
                                 do_classes);
                /* if all the netids in the ignore_name list are present, ignore client-supplied name */
                if hostname_auth == 0 {
                    id_list = daemon.dhcp_ignore_names;
                    while !id_list.is_null() {
                        if (*id_list).list.is_null() ||
                               match_netid((*id_list).list, tagif_netid,
                                           0) != 0 {
                            break ;
                        }
                        id_list = (*id_list).next
                    }
                    if !id_list.is_null() {
                        hostname = 0
                    }
                }
                /* Last ditch, if configured, generate hostname from mac address */
                if hostname.is_null() && emac_len != 0 {
                    id_list = daemon.dhcp_gen_names;
                    while !id_list.is_null() {
                        if (*id_list).list.is_null() ||
                               match_netid((*id_list).list, tagif_netid,
                                           0) != 0 {
                            break ;
                        }
                        id_list = (*id_list).next
                    }
                    if !id_list.is_null() {
                        let mut i_1: i32 = 0;
                        hostname = daemon.dhcp_buff;
                        /* buffer is 256 bytes, 3 bytes per octet */
                        i_1 = 0;
                        while i_1 < emac_len && i_1 < 80 {
                            hostname =
                                hostname.offset(sprintf(hostname,
                                                        b"%.2x%s\x00"         *const u8         *const libc::c_char,
                                                        *emac.offset(i_1                      isize)
                                                           ,
                                                        if i_1 ==
                                                               emac_len -
                                                                   1                    libc::c_int
                                                           {
                                                            b"\x00"             *const u8             *const libc::c_char
                                                        } else {
                                                            b"-\x00"             *const u8             *const libc::c_char
                                                        }));
                            i_1 += 1
                        }
                        hostname = daemon.dhcp_buff
                    }
                }
                if !hostname.is_null() {
                    lease_set_hostname(lease, hostname, hostname_auth,
                                       get_domain((*lease).addr), domain);
                }
                lease_set_expires(lease, time, now);
                lease_set_interface(lease, int_index, now);
                if override_0.s_addr != 0 {
                    (*lease).override_0 = override_0
                } else { override_0 = (*lease).override_0 }
                daemon.metrics[METRIC_DHCPACK                                        usize] =
                    daemon.metrics[METRIC_DHCPACK                                            usize].wrapping_add(1);
                log_packet(b"DHCPACK\x00"
                               ,
                           &mut mess.yiaddr, emac, emac_len, iface_name,
                           hostname, 0 , mess.xid);
                clear_packet(mess, end);
                option_put(mess, end, 53, 1,
                           5);
                option_put(mess, end, 54, 4,
                           __bswap_32(server_id(context, override_0,
                                                fallback).s_addr));
                option_put(mess, end, 51, 4,
                           time);
                if rapid_commit != 0 {
                    option_put(mess, end, 80, 0,
                               0);
                }
                do_options(context, mess, end, req_options, hostname,
                           get_domain(mess.yiaddr), netid, subnet_addr,
                           fqdn_flags, borken_opt, pxearch, uuid,
                           vendor_class_len, now, time, fuzz, pxevendor);
            }
            return dhcp_packet_size(mess, agent_id, real_end)
        }
    return 0 ;
}
/* find a good value to use as MAC address for logging and address-allocation hashing.
   This is normally just the chaddr field from the DHCP packet,
   but eg Firewire will have hlen == 0 and use the client-id instead. 
   This could be anything, but will normally be EUI64 for Firewire.
   We assume that if the first byte of the client-id equals the htype byte
   then the client-id is using the usual encoding and use the rest of the 
   client-id: if not we can use the whole client-id. This should give
   sane MAC address logs. */

pub fn extended_hwaddr(mut hwtype: i32,
                                         mut hwlen: i32,
                                         mut hwaddr: mut Vec<u8>,
                                         mut clid_len: i32,
                                         mut clid: mut Vec<u8>,
                                         mut len_out: )
 -> mut Vec<u8> {
    if hwlen == 0 && !clid.is_null() &&
           clid_len > 3 {
        if *clid.offset(0) == hwtype {
            *len_out = clid_len - 1; /* sanity */
            return clid.offset(1)
        } /* add terminator */
        if *clid.offset(0) ==
               27 && hwtype == 24 {
            *len_out = clid_len - 1;
            return clid.offset(1)
        }
        *len_out = clid_len;
        return clid
    }
    *len_out = hwlen;
    return hwaddr;
}
fn calc_time(mut context: DhcpContext,
                               mut config: *mut DhcpConfig,
                               mut opt: mut Vec<u8>) -> libc::c_uint {
    let mut time: u32 =
        if !config.is_null() &&
               (*config).flags & 8 != 0 {
            (*config).lease_time
        } else { (*context).lease_time };
    if !opt.is_null() {
        let mut req_time: u32 =
            option_uint(opt, 0, 4);
        if req_time < 120 {
            req_time = 120
        }
        if time == 0xffffffff ||
               req_time != 0xffffffff && req_time < time {
            time = req_time
        }
    }
    return time;
}
fn server_id(mut context: DhcpContext,
                               mut override_0: NetAddress, mut fallback: NetAddress)
                               -> NetAddress {
    if override_0.s_addr != 0 {
        return override_0
    } else if !context.is_null() &&
                  (*context).local.s_addr != 0
     {
        return (*context).local
    } else { return fallback };
}
fn sanitise(mut opt: mut Vec<u8>,
                              mut buf: &mut String) -> i32 {
    let mut p: &mut String = 0 ;
    let mut i: i32 = 0;
    *buf = 0;
    if opt.is_null() { return 0 }
    p =
        &mut *opt.offset((2                        libc::c_uint).wrapping_add(0          libc::c_uint)                       isize)
            ;
    i = *opt.offset(1);
    while i > 0 {
        let fresh7 = p;
        p = p.offset(1);
        let mut c: libc::c_char = *fresh7;
        if *(*__ctype_b_loc()).offset(c)  &
               _ISPRINT  != 0 {
            let fresh8 = buf;
            buf = buf.offset(1);
            *fresh8 = c
        }
        i -= 1
    }
    *buf = 0;
    return 1;
}
fn add_extradata_opt(mut lease: DhcpLease,
                                       mut opt: mut Vec<u8>) {
    if opt.is_null() {
        lease_add_extradata(lease, 0,
                            0,
                            0);
    } else {
        lease_add_extradata(lease,
                            &mut *opt.offset((2                                            libc::c_uint).wrapping_add(0
                                                                                                              libc::c_int
                                                                                                              libc::c_uint)
                                                )                          mut Vec<u8>                          mut Vec<u8>,
                            *opt.offset(1)                          libc::c_int,
                            0);
    };
}
fn log_packet(mut type_0: &mut String,
                                mut addr:Vec<u8>,
                                mut ext_mac: mut Vec<u8>,
                                mut mac_len: i32,
                                mut interface: &mut String,
                                mut string: &mut String,
                                mut err: &mut String, mut xid: u32) {
    let mut a: NetAddress = NetAddress {s_addr: 0,};
    if err.is_null() &&
           daemon.options[(28 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (28 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               == 0 &&
           daemon.options[(42 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (42 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               != 0 {
        return
    }
    /* addr may be misaligned */
    if !addr.is_null() {
        memcpy(&mut a, addr,
               ::std::mem::size_of::<NetAddress>()); /* malformed packet */
    } /* malformed packet */
    print_mac(daemon.namebuff, ext_mac, mac_len);
    if daemon.options[(28).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (28).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 {
        my_syslog((3) << 3 | 6,
                  b"%u %s(%s) %s%s%s %s%s\x00", __bswap_32(xid), type_0, interface,
                  if !addr.is_null() {
                      inet_ntoa(a)
                  } else { b"\x00"  },
                  if !addr.is_null() {
                      b" \x00"
                  } else { b"\x00"  },
                  daemon.namebuff,
                  if !string.is_null() {
                      string
                  } else { b"\x00"  },
                  if !err.is_null() {
                      err
                  } else { b"\x00"  });
    } else {
        my_syslog((3) << 3 | 6,
                  b"%s(%s) %s%s%s %s%s\x00", type_0, interface,
                  if !addr.is_null() {
                      inet_ntoa(a)
                  } else { b"\x00"  },
                  if !addr.is_null() {
                      b" \x00"
                  } else { b"\x00"  },
                  daemon.namebuff,
                  if !string.is_null() {
                      string
                  } else { b"\x00"  },
                  if !err.is_null() {
                      err
                  } else { b"\x00"  });
    };
}
fn log_options(mut start: mut Vec<u8>,
                                 mut xid: u32) {
    while *start != 255 {
        let mut optname: &mut String =
            option_string(2,
                          *start.offset(0)                        libc::c_uint,
                          &mut *start.offset((2                                            libc::c_uint).wrapping_add(0
                                                                                                              libc::c_int
                                                                                                              libc::c_uint)
                                                ),
                          *start.offset(1), daemon.namebuff,
                          1025);
        my_syslog((3) << 3 | 6,
                  b"%u sent size:%3d option:%3d %s  %s\x00", __bswap_32(xid),
                  *start.offset(1),
                  *start.offset(0),
                  optname, daemon.namebuff);
        start =
            start.offset((*start.offset(1) + 2))
    };
}
fn option_find1(mut p: mut Vec<u8>,
                                  mut end: mut Vec<u8>,
                                  mut opt: i32,
                                  mut minsize: i32)
 -> mut Vec<u8> {
    loop  {
        if p >= end {
            return 0
        } else {
            if *p == 255 {
                return if opt == 255 {
                           p
                       } else { 0 }
            } else {
                if *p == 0 {
                    p = p.offset(1)
                } else {
                    let mut opt_len: i32 = 0;
                    if p > end.offset(-(2)) {
                        return 0
                    }
                    opt_len =
                        *p.offset(1);
                    if p >
                           end.offset(-((2 + opt_len))) {
                        return 0
                    }
                    if *p == opt && opt_len >= minsize {
                        return p
                    }
                    p = p.offset((opt_len + 2))
                }
            }
        }
    };
}
fn option_find(mut mess: DhcpPacket, mut size: usize,
                                 mut opt_type: i32,
                                 mut minsize: i32)
                                 -> mut Vec<u8> {
    let mut ret: mut Vec<u8> = 0;
    let mut overload: mut Vec<u8> = 0;
    /* skip over DHCP cookie; */
    ret =
        option_find1((&mut *mess.options.as_mut_ptr().offset(0                 libc::c_int
                                                                   )
                                             *mut u8).offset(::std::mem::size_of::<u32>()                                   ),
                     (mess).offset(size),
                     opt_type, minsize);
    if !ret.is_null() { return ret }
    /* look for overload option. */
    overload =
        option_find1((&mut *mess.options.as_mut_ptr().offset(0                 libc::c_int
                                                                   )
                                             *mut u8).offset(::std::mem::size_of::<u32>()                                   ),
                     (mess).offset(size),
                     52, 1);
    if overload.is_null() { return 0 }
    /* Can we look in filename area ? */
    if *overload.offset(2) &
           1 != 0 &&
           {
               ret =
                   option_find1(&mut *mess.file.as_mut_ptr().offset(0                        libc::c_int
                                                            ),
                                &mut *mess.file.as_mut_ptr().offset(128                        libc::c_int
                                                            ),
                                opt_type, minsize);
               !ret.is_null()
           } {
        return ret
    }
    /* finally try sname area */
    if *overload.offset(2) &
           2 != 0 &&
           {
               ret =
                   option_find1(&mut *mess.sname.as_mut_ptr().offset(0                         libc::c_int
                                                              ),
                                &mut *mess.sname.as_mut_ptr().offset(64                         libc::c_int
                                                              ),
                                opt_type, minsize);
               !ret.is_null()
           } {
        return ret
    }
    return 0;
}
fn option_addr(mut opt: mut Vec<u8>) -> NetAddress {
    /* this worries about unaligned data in the option. */
  /* struct in_addr is network byte order */
    let mut ret: NetAddress = NetAddress {s_addr: 0,};
    memcpy(&mut ret,
           &mut *opt.offset((2                           libc::c_uint).wrapping_add(0
                                                                            libc::c_uint)
                               )        Vec<u8>, 4);
    return ret;
}
fn option_uint(mut opt: mut Vec<u8>,
                                 mut offset: i32,
                                 mut size: i32) -> libc::c_uint {
    /* this worries about unaligned data and byte order */
    let mut ret: u32 = 0;
    let mut i: i32 = 0;
    let mut p: mut Vec<u8> =
        &mut *opt.offset((2                        libc::c_uint).wrapping_add(offset          libc::c_uint)                       isize)
           ;
    i = 0;
    while i < size {
        let fresh9 = p;
        p = p.offset(1);
        ret = ret << 8 | *fresh9;
        i += 1
    }
    return ret;
}
fn dhcp_skip_opts(mut start: mut Vec<u8>)
 -> mut Vec<u8> {
    while *start != 0 {
        start =
            start.offset((*start.offset(1) + 2))
    }
    return start;
}
/* only for use when building packet: doesn't check for bad data. */
fn find_overload(mut mess: DhcpPacket)
 -> mut Vec<u8> {
    let mut p: mut Vec<u8> =
        (&mut *mess.options.as_mut_ptr().offset(0)
                   *mut u8).offset(::std::mem::size_of::<u32>()
                                  );
    while *p != 0 {
        if *p == 52 { return p }
        p =
            p.offset((*p.offset(1) +
                          2))
    }
    return 0;
}
fn dhcp_packet_size(
    daemon: &mut DnsmasqDaemon,
    mut mess: DhcpPacket,
                                      mut agent_id: &mut Vec<u8>,
                                      mut real_end: &mut Vec<u8>)
                                      -> usize {
    let mut p: Vec<u8> = dhcp_skip_opts((&mut *mess.options.as_mut_ptr().offset(0)).offset(::std::mem::size_of::<u32>()));
    let mut overload: Vec<u8> = Vec::new();
    let mut ret: usize = 0;
    /* move agent_id back down to the end of the packet */
    if !agent_id.is_null() {
        memmove(p, agent_id, real_end.wrapping_offset_from(agent_id));
        p = p.offset(real_end.wrapping_offset_from(agent_id));
        /* in case of overlap */
    }
    /* add END options to the regions. */
    overload = find_overload(mess);
    if !overload.is_null() && option_uint(overload, 0, 1) & 1 != 0 {
        *dhcp_skip_opts(mess.file.as_mut_ptr()) = 255;
        if daemon.options[28] != 0 {
            log_options(mess.file.as_mut_ptr(), mess.xid);
        }
    } else if daemon.options[28] != 0 &&
                  strlen(mess.file.as_mut_ptr() ) !=
                      0 {
        my_syslog((3) << 3 | 6,
                  b"%u bootfile name: %s\x00", __bswap_32(mess.xid),
                  mess.file.as_mut_ptr() );
    }
    if !overload.is_null() &&
           option_uint(overload, 0, 1) &
               2 != 0 {
        *dhcp_skip_opts(mess.sname.as_mut_ptr()) =
            255;
        if daemon.options[(28 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (28 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               != 0 {
            log_options(mess.sname.as_mut_ptr(), mess.xid);
        }
    } else if daemon.options[(28 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                 ).wrapping_mul(8))
                                            ] &
                  (1) <<
                      (28).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             ).wrapping_mul(8))
                  != 0 &&
                  strlen(mess.sname.as_mut_ptr() ) !=
                      0 {
        my_syslog((3) << 3 | 6,
                  b"%u server name: %s\x00", __bswap_32(mess.xid),
                  mess.sname.as_mut_ptr() );
    }
    let fresh10 = p;
    p = p.offset(1);
    *fresh10 = 255;
    if daemon.options[(28).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (28).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 {
        if mess.siaddr.s_addr != 0 {
            my_syslog((3) << 3 |
                          6,
                      b"%u next server: %s\x00", __bswap_32(mess.xid),
                      inet_ntoa(mess.siaddr));
        }
        if mess.flags &
               __bswap_16(0x8000)
               != 0 &&
               mess.ciaddr.s_addr == 0 {
            my_syslog((3) << 3 |
                          6,
                      b"%u broadcast response\x00", __bswap_32(mess.xid));
        }
        log_options((&mut *mess.options.as_mut_ptr().offset(0                libc::c_int
                                                                  )
                                           *mut u8).offset(::std::mem::size_of::<u32>()),
                    mess.xid);
    }
    ret =
        p.wrapping_offset_from(mess)      usize;
    if ret < 300 {
        ret = 300
    }
    return ret;
}
fn free_space(mut mess: DhcpPacket,
                                mut end: mut Vec<u8>,
                                mut opt: i32, mut len: i32)
                                -> mut Vec<u8> {
    let mut p: mut Vec<u8> =
        dhcp_skip_opts((&mut *mess.options.as_mut_ptr().offset(0                   libc::c_int
                                                  )
                                                 *mut u8).offset(::std::mem::size_of::<u32>()
                                                 ));
    if p.offset(len).offset(3) >= end {
        /* not enough space in options area, try and use overload, if poss */
        let mut overload: mut Vec<u8> = 0;
        overload = find_overload(mess);
        if overload.is_null() &&
               (mess.file[0 ] ==
                    0 ||
                    mess.sname[0 ] ==
                        0) {
            /* attempt to overload fname and sname areas, we've reserved space for the
	     overflow option previuously. */
            overload = p;
            let fresh11 = p;
            p = p.offset(1);
            *fresh11 = 52;
            let fresh12 = p;
            p = p.offset(1);
            *fresh12 = 1
        }
        p = 0;
        /* using filename field ? */
        if !overload.is_null() {
            if mess.file[0 ] ==
                   0 {
                let ref mut fresh13 =
                    *overload.offset(2);
                *fresh13 =
                    (*fresh13 | 1)
            }
            if *overload.offset(2) &
                   1 != 0 {
                p = dhcp_skip_opts(mess.file.as_mut_ptr());
                if p.offset(len).offset(3) >=
                       mess.file.as_mut_ptr().offset(::std::mem::size_of::<[u8; 128]>()

                                                           ) {
                    p = 0
                }
            }
            if p.is_null() {
                /* try to bring sname into play (it may be already) */
                if mess.sname[0 ] ==
                       0 {
                    let ref mut fresh14 =
                        *overload.offset(2);
                    *fresh14 =
                        (*fresh14 | 2)                      libc::c_uchar
                }
                if *overload.offset(2)
                       & 2 != 0 {
                    p = dhcp_skip_opts(mess.sname.as_mut_ptr());
                    if p.offset(len                              isize).offset(3)
                           >=
                           mess.sname.as_mut_ptr().offset(::std::mem::size_of::<[u8; 64]>()
                                                                              libc::c_ulong
                                                                ) {
                        p = 0
                    }
                }
            }
        }
        if p.is_null() {
            my_syslog((3) << 3 |
                          4,
                      b"cannot send DHCP/BOOTP option %d: no space left in packet\x00"
                          , opt);
        }
    }
    if !p.is_null() {
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = opt;
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = len
    }
    return p;
}
fn option_put(mut mess: DhcpPacket,
                                mut end: mut Vec<u8>,
                                mut opt: i32, mut len: i32,
                                mut val: u32) {
    let mut i: i32 = 0;
    let mut p: mut Vec<u8> = free_space(mess, end, opt, len);
    if !p.is_null() {
        i = 0;
        while i < len {
            let fresh17 = p;
            p = p.offset(1);
            *fresh17 =
                (val >> 8 * (len - (i + 1)))              libc::c_uchar;
            i += 1
        }
    };
}
fn option_put_string(mut mess: DhcpPacket,
                                       mut end: mut Vec<u8>,
                                       mut opt: i32,
                                       mut string: *const libc::c_char,
                                       mut null_term: i32) {
    let mut p: mut Vec<u8> = 0;
    let mut len: usize = strlen(string);
    if null_term != 0 && len != 255 {
        len = len.wrapping_add(1)
    }
    p = free_space(mess, end, opt, len);
    if !p.is_null() {
        memcpy(p, string, len);
    };
}
/* return length, note this only does the data part */
fn do_opt(mut opt: *mut DhcpOpt, mut p: mut Vec<u8>,
                            mut context: DhcpContext,
                            mut null_term: i32) -> i32 {
    let mut len: i32 = (*opt).len;
    if (*opt).flags & 2 != 0 && null_term != 0 &&
           len != 255 {
        len += 1
    }
    if !p.is_null() && len != 0 {
        if !context.is_null() && (*opt).flags & 1 != 0 {
            let mut j: i32 = 0;
            let mut a: NetAddress = (*opt).val;
            j = 0;
            while j < (*opt).len {
                /* zero means "self" (but not in vendorclass options.) */
                if (*a).s_addr == 0 {
                    memcpy(p,
                           &mut (*context).local
                           4);
                } else {
                    memcpy(p, a,
                           4);
                }
                p = p.offset(4);
                j += 4;
                a = a.offset(1)
            }
        } else {
            /* empty string may be extended to "\0" by null_term */
            memcpy(p,
                   if !(*opt).val.is_null() {
                       (*opt).val
                   } else {
                       b"\x00"                      mut Vec<u8>
                   }, len);
        }
    }
    return len;
}
fn in_list(mut list: mut Vec<u8>,
                             mut opt: i32) -> i32 {
    let mut i: i32 = 0;
    /* If no requested options, send everything, not nothing. */
    if list.is_null() { return 1 }
    i = 0;
    while *list.offset(i) != 255 {
        if opt == *list.offset(i) {
            return 1
        }
        i += 1
    }
    return 0;
}
fn option_find2(mut opt: i32) -> *mut DhcpOpt {
    let mut opts: *mut DhcpOpt = 0 ;
    opts = daemon.dhcp_opts;
    while !opts.is_null() {
        if (*opts).opt == opt && (*opts).flags & 4096 != 0 {
            return opts
        }
        opts = (*opts).next
    }
    return 0 ;
}
/* mark vendor-encapsulated options which match the client-supplied  or
   config-supplied vendor class */
fn match_vendor_opts(mut opt: mut Vec<u8>,
                                       mut dopt: *mut DhcpOpt) {
    while !dopt.is_null() {
        (*dopt).flags &= !(1024);
        if !opt.is_null() && (*dopt).flags & 256 != 0 {
            let mut pv: *const DhcpPxeVendor = 0 ;
            let mut dummy_vendor: DhcpPxeVendor =
                {
                    let mut init =
                        DhcpPxeVendor {data:
                                            (*dopt).u.vendor_class                                          &mut String,
                                        next: 0 ,};
                    init
                };
            if (*dopt).flags & 16384 != 0 {
                pv = daemon.dhcp_pxe_vendors
            } else { pv = &mut dummy_vendor }
            while !pv.is_null() {
                let mut i: i32 = 0;
                let mut len: i32 = 0;
                let mut matched: i32 = 0;
                if !(*pv).data.is_null() {
                    len = strlen((*pv).data)
                }
                i = 0;
                while i <=
                          *opt.offset(1) - len {
                    if len == 0 ||
                           memcmp((*pv).data,
                                  &mut *opt.offset((2     libc::c_uint).wrapping_add(i
                                                                                                                          libc::c_uint)
                                                      )                                mut Vec<u8>,
                                  len) == 0 {
                        matched = 1;
                        break ;
                    } else { i += 1 }
                }
                if matched != 0 {
                    (*dopt).flags |= 1024;
                    break ;
                } else { pv = (*pv).next }
            }
        }
        dopt = (*dopt).next
    };
}
fn do_encap_opts(mut opt: *mut DhcpOpt,
                                   mut encap: i32,
                                   mut flag: i32,
                                   mut mess: DhcpPacket,
                                   mut end: mut Vec<u8>,
                                   mut null_term: i32)
                                   -> i32 {
    let mut len: i32 = 0;
    let mut enc_len: i32 = 0;
    let mut ret: i32 = 0;
    let mut start: *mut DhcpOpt = 0 ;
    let mut p: mut Vec<u8> = 0;
    /* find size in advance */
    enc_len = 0;
    start = opt;
    while !opt.is_null() {
        if (*opt).flags & flag != 0 {
            let mut new: i32 =
                do_opt(opt, 0, 0,
                       null_term) + 2;
            ret = 1;
            if enc_len + new <= 255 {
                enc_len += new
            } else {
                p = free_space(mess, end, encap, enc_len);
                while !start.is_null() && start != opt {
                    if !p.is_null() && (*start).flags & flag != 0 {
                        len =
                            do_opt(start, p.offset(2),
                                   0, null_term);
                        let fresh18 = p;
                        p = p.offset(1);
                        *fresh18 = (*start).opt;
                        let fresh19 = p;
                        p = p.offset(1);
                        *fresh19 = len;
                        p = p.offset(len)
                    }
                    start = (*start).next
                }
                enc_len = new;
                start = opt
            }
        }
        opt = (*opt).next
    }
    if enc_len != 0 &&
           {
               p = free_space(mess, end, encap, enc_len + 1);
               !p.is_null()
           } {
        while !start.is_null() {
            if (*start).flags & flag != 0 {
                len =
                    do_opt(start, p.offset(2),
                           0, null_term);
                let fresh20 = p;
                p = p.offset(1);
                *fresh20 = (*start).opt;
                let fresh21 = p;
                p = p.offset(1);
                *fresh21 = len;
                p = p.offset(len)
            }
            start = (*start).next
        }
        *p = 255
    }
    return ret;
}
fn pxe_misc(mut mess: DhcpPacket,
                              mut end: mut Vec<u8>,
                              mut uuid: mut Vec<u8>,
                              mut pxevendor: *const libc::c_char) {
    let mut p: mut Vec<u8> = 0;
    if pxevendor.is_null() {
        pxevendor = b"PXEClient\x00"
    }
    option_put_string(mess, end, 60, pxevendor,
                      0);
    if !uuid.is_null() &&
           {
               p =
                   free_space(mess, end, 97,
                              17);
               !p.is_null()
           } {
        memcpy(p, uuid,
               17);
    };
}
fn prune_vendor_opts(mut netid: *mut DhcpNetId)
 -> i32 {
    let mut force: i32 = 0;
    let mut opt: *mut DhcpOpt = 0 ;
    /* prune vendor-encapsulated options based on netid, and look if we're forcing them to be sent */
    opt = daemon.dhcp_opts;
    while !opt.is_null() {
        if (*opt).flags & 1024 != 0 {
            if match_netid((*opt).netid, netid, 1) == 0 {
                (*opt).flags &= !(1024)
            } else if (*opt).flags & 16 != 0 {
                force = 1
            }
        }
        opt = (*opt).next
    }
    return force;
}
/* Many UEFI PXE implementations have badly broken menu code.
   If there's exactly one relevant menu item, we abandon the menu system,
   and jamb the data direct into the DHCP file, siaddr and sname fields.
   Note that in this case, we have to assume that layer zero would be requested
   by the client PXE stack. */
fn pxe_uefi_workaround(mut pxe_arch: i32,
                                         mut netid: *mut DhcpNetId,
                                         mut mess: DhcpPacket,
                                         mut local: NetAddress, mut now: time::Instant,
                                         mut pxe: i32)
                                         -> i32 {
    let mut service: *mut PxeService = 0 ;
    let mut found: *mut PxeService = 0 ;
    /* Only workaround UEFI archs. */
    if pxe_arch < 6 {
        return 0
    } /* More than one relevant menu item */
    found = 0 ; /* No relevant menu items. */
    service = daemon.pxe_services;
    while !service.is_null() {
        if pxe_arch == (*service).csa &&
               !(*service).basename.is_null() &&
               match_netid((*service).netid, netid, 1) != 0 {
            if !found.is_null() { return 0 }
            found = service
        }
        service = (*service).next
    }
    if found.is_null() { return 0 }
    if pxe == 0 { return 1 }
    if !(*found).sname.is_null() {
        mess.siaddr = a_record_from_hosts((*found).sname, now);
        snprintf(mess.sname.as_mut_ptr() ,
                 ::std::mem::size_of::<[u8; 64]>(),
                 b"%s\x00" ,
                 (*found).sname);
    } else {
        if (*found).server.s_addr != 0 {
            mess.siaddr = (*found).server
        } else { mess.siaddr = local }
        inet_ntop(2,
                  &mut mess.siaddr,
                  mess.sname.as_mut_ptr() ,
                  16);
    }
    snprintf(mess.file.as_mut_ptr() ,
             ::std::mem::size_of::<[u8; 128]>(),
             if !strchr((*found).basename, '.' as i32).is_null() {
                 b"%s\x00"
             } else { b"%s.0\x00"  },
             (*found).basename);
    return 1;
}
fn pxe_opts(mut pxe_arch: i32,
                              mut netid: *mut DhcpNetId, mut local: NetAddress,
                              mut now: time::Instant) -> *mut DhcpOpt {
    let mut p: mut Vec<u8> = 0;
    let mut q: mut Vec<u8> = 0;
    let mut service: *mut PxeService = 0 ;
    static mut o: *mut DhcpOpt = 0  ;
    static mut ret: *mut DhcpOpt = 0  ;
    let mut i: i32 = 0;
    let mut j: i32 = 4 - 1;
    let mut boot_server: NetAddress = NetAddress {s_addr: 0,};
    /* We pass back references to these, hence they are declared static */
    static mut discovery_control: libc::c_uchar = 0;
    static mut fake_prompt: [libc::c_uchar; 4] =
        [0, 'P' as i32,
         'X' as i32, 'E' as i32];
    static mut fake_opts: *mut DhcpOpt =
        0  ;
    /* Disable multicast, since we don't support it, and broadcast
     unless we need it */
    discovery_control = 3;
    ret = daemon.dhcp_opts;
    if fake_opts.is_null() &&
           {
               fake_opts =
                   whine_malloc((4                        ).wrapping_mul(::std::mem::size_of::<DhcpOpt>()
                                                                               ))
                       ;
               fake_opts.is_null()
           } {
        return ret
    }
    i = 0;
    while i < 4 {
        (*fake_opts.offset(i)).flags = 1024;
        let ref mut fresh22 = (*fake_opts.offset(i)).netid;
        *fresh22 = 0 ;
        let ref mut fresh23 = (*fake_opts.offset(i)).next;
        *fresh23 =
            if i == 4 - 1 {
                ret
            } else {
                &mut *fake_opts.offset((i + 1))              *mut DhcpOpt
            };
        i += 1
    }
    /* create the data for the PXE_MENU and PXE_SERVERS options. */
    p = daemon.dhcp_buff;
    q = daemon.dhcp_buff3;
    i = 0;
    service = daemon.pxe_services;
    while !service.is_null() {
        if pxe_arch == (*service).csa &&
               match_netid((*service).netid, netid, 1) != 0 {
            's_185:
                {
                    let mut current_block_29: u64;
                    let mut len: usize = strlen((*service).menu);
                    /* opt 43 max size is 255. encapsulated option has type and length
	   bytes, so its max size is 253. */
                    if (p.wrapping_offset_from(daemon.dhcp_buffmut Vec<u8>)                      i32 ).wrapping_add(len).wrapping_add(3                           libc::c_int
                                                                                                 )
                           < 253 {
                        let fresh24 = p;
                        p = p.offset(1);
                        *fresh24 =
                            ((*service).type_0 >>
                                 8);
                        let fresh25 = p;
                        p = p.offset(1);
                        *fresh25 = (*service).type_0;
                        let fresh26 = p;
                        p = p.offset(1);
                        *fresh26 = len;
                        memcpy(p,
                               (*service).menu, len);
                        p = p.offset(len);
                        i += 1;
                        boot_server =
                            if !(*service).basename.is_null() {
                                local
                            } else if !(*service).sname.is_null() {
                                a_record_from_hosts((*service).sname, now)
                            } else { (*service).server };
                        if boot_server.s_addr !=
                               0 {
                            if q.wrapping_offset_from(daemon.dhcp_buff3
                                                                mut Vec<u8>)
                                   +
                                   3 +
                                   4 >=
                                   253 {
                                current_block_29 = 14257305712396241914;
                            } else {
                                /* Boot service with known address - give it */
                                let fresh27 = q;
                                q = q.offset(1);
                                *fresh27 =
                                    ((*service).type_0 >>
                                         8);
                                let fresh28 = q;
                                q = q.offset(1);
                                *fresh28 = (*service).type_0;
                                let fresh29 = q;
                                q = q.offset(1);
                                *fresh29 = 1;
                                /* dest misaligned */
                                memcpy(q,
                                       &mut boot_server.s_addr                                     *mut InAddrT
                                       4);
                                q = q.offset(4);
                                current_block_29 = 6450636197030046351;
                            }
                        } else {
                            if (*service).type_0 !=
                                   0 {
                                /* We don't know the server for a service type, so we'll
	     allow the client to broadcast for it */
                                discovery_control =
                                    2
                            }
                            current_block_29 = 6450636197030046351;
                        }
                        match current_block_29 {
                            14257305712396241914 => { }
                            _ => { break 's_185 ; }
                        }
                    }
                    my_syslog((3) << 3 |
                                  3,
                              b"PXE menu too large\x00");
                    return daemon.dhcp_opts
                }
        }
        service = (*service).next
    }
    /* if no prompt, wait forever if there's a choice */
    fake_prompt[0 ] =
        if i > 1 {
            255
        } else { 0 }      libc::c_uchar; /* no menu - just use use mess->filename */
    if i == 0 {
        discovery_control = 8
    } else {
        let fresh30 = j;
        j = j - 1;
        ret = &mut *fake_opts.offset(fresh30) ;
        ret.len =
            p.wrapping_offset_from(daemon.dhcp_buff                                 mut Vec<u8>) ;
        ret.val = daemon.dhcp_buff;
        ret.opt = 9;
        if q.wrapping_offset_from(daemon.dhcp_buff3                                mut Vec<u8>) !=
               0 {
            let fresh31 = j;
            j = j - 1;
            ret = &mut *fake_opts.offset(fresh31) ;
            ret.len =
                q.wrapping_offset_from(daemon.dhcp_buff3                                     mut Vec<u8>)
                   ;
            ret.val = daemon.dhcp_buff3;
            ret.opt = 8
        }
    }
    o = daemon.dhcp_opts;
    while !o.is_null() {
        if (*o).flags & 1024 != 0 &&
               (*o).opt == 10 {
            break ;
        }
        o = (*o).next
    }
    if o.is_null() {
        let fresh32 = j;
        j = j - 1;
        ret = &mut *fake_opts.offset(fresh32) ;
        ret.len =
            ::std::mem::size_of::<[libc::c_uchar; 4]>() ;
        ret.val = fake_prompt.as_mut_ptr();
        ret.opt = 10
    }
    let fresh33 = j;
    j = j - 1;
    ret = &mut *fake_opts.offset(fresh33) ;
    ret.len = 1;
    ret.opt = 6;
    ret.val = discovery_control;
    return ret;
}
fn clear_packet(mut mess: DhcpPacket, mut end: &mut Vec<u8>) {
    mess.siaddr.s_addr = 0;
}

pub fn find_boot(mut netid: *mut DhcpNetId)
 -> *mut DhcpBoot {
    let mut boot: *mut DhcpBoot = 0 ;
    /* decide which dhcp-boot option we're using */
    boot = daemon.boot_config;
    while !boot.is_null() {
        if match_netid(boot.netid, netid, 0) != 0 {
            break ;
        }
        boot = boot.next
    }
    if boot.is_null() {
        /* No match, look for one without a netid */
        boot = daemon.boot_config;
        while !boot.is_null() {
            if match_netid(boot.netid, netid, 1) != 0 {
                break ;
            }
            boot = boot.next
        }
    }
    return boot;
}
fn is_pxe_client(mut mess: DhcpPacket, mut sz: usize,
                                   mut pxe_vendor: *mut *const libc::c_char)
                                   -> i32 {
    let mut opt: *const libc::c_uchar = 0 ;
    let mut conf_len: susize = 0 as susize;
    let mut conf: *const DhcpPxeVendor = daemon.dhcp_pxe_vendors;
    opt = option_find(mess, sz, 60, 0);
    if opt.is_null() { return 0 }
    while !conf.is_null() {
        conf_len = strlen((*conf).data) as susize;
        if !((*(opt).offset(1)
                 ) < conf_len) {
            if strncmp(&mut *(opt                            mut Vec<u8>).offset((2               libc::c_uint).wrapping_add(0                 libc::c_int                 libc::c_uint)
                                                                )                     mut Vec<u8>                     *const libc::c_char, (*conf).data,
                       conf_len) == 0 {
                if !pxe_vendor.is_null() { *pxe_vendor = (*conf).data }
                return 1
            }
        }
        conf = (*conf).next
    }
    return 0;
}
fn do_options(mut context: DhcpContext,
                                mut mess: DhcpPacket,
                                mut end: mut Vec<u8>,
                                mut req_options: mut Vec<u8>,
                                mut hostname: &mut String,
                                mut domain: &mut String,
                                mut netid: *mut DhcpNetId,
                                mut subnet_addr: NetAddress,
                                mut fqdn_flags: libc::c_uchar,
                                mut null_term: i32,
                                mut pxe_arch: i32,
                                mut uuid: mut Vec<u8>,
                                mut vendor_class_len: i32,
                                mut now: time::Instant, mut lease_time: u32,
                                mut fuzz: u16,
                                mut pxevendor: *const libc::c_char) {
    let mut opt: *mut DhcpOpt = 0 ;
    let mut config_opts: *mut DhcpOpt = daemon.dhcp_opts;
    let mut boot: *mut DhcpBoot = 0 ;
    let mut p: mut Vec<u8> = 0;
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    let mut force_encap: i32 = 0;
    let mut f0: libc::c_uchar = 0;
    let mut s0: libc::c_uchar = 0;
    let mut done_file: i32 = 0;
    let mut done_server: i32 = 0;
    let mut done_vendor_class: i32 = 0;
    let mut tagif: *mut DhcpNetId = 0 ;
    let mut id_list: *mut DhcpNetIdList = 0;
    /* filter options based on tags, those we want get DHOPT_TAGOK bit set */
    if !context.is_null() { (*context).netid.next = 0  }
    tagif =
        option_filter(netid,
                      if !context.is_null() && !(*context).netid.net.is_null()
                         {
                          &mut (*context).netid
                      } else { 0  }, config_opts);
    /* logging */
    if daemon.options[(28).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (28).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 && !req_options.is_null() {
        let mut q: &mut String =
            daemon.namebuff; /* force broadcast */
        i = 0;
        while *req_options.offset(i) !=
                  255 {
            let mut s: &mut String =
                option_string(2,
                              *req_options.offset(i),
                              0, 0,
                              0 , 0);
            q =
                q.offset(snprintf(q,
                                  (1025 -
                                       q.wrapping_offset_from(daemon.namebuff)
                                          ),
                                  b"%d%s%s%s\x00",
                                  *req_options.offset(i)                                libc::c_int,
                                  if strlen(s) !=
                                         0 {
                                      b":\x00"                                     *const libc::c_char
                                  } else {
                                      b"\x00"                                     *const libc::c_char
                                  }, s,
                                  if *req_options.offset((i +
                                                              1               libc::c_int)
                                                            )                                   libc::c_int == 255 {
                                      b"\x00"                                     *const libc::c_char
                                  } else {
                                      b", \x00"                                     *const libc::c_char
                                  }));
            if *req_options.offset((i + 1))             libc::c_int == 255 ||
                   q.wrapping_offset_from(daemon.namebuff)                 i32 > 40 {
                q = daemon.namebuff;
                my_syslog((3) << 3 |
                              6,
                          b"%u requested options: %s\x00" , __bswap_32(mess.xid),
                          daemon.namebuff);
            }
            i += 1
        }
    }
    id_list = daemon.force_broadcast;
    while !id_list.is_null() {
        if (*id_list).list.is_null() ||
               match_netid((*id_list).list, netid, 0) != 0 {
            break ;
        }
        id_list = (*id_list).next
    }
    if !id_list.is_null() {
        mess.flags =
            (mess.flags |
                 __bswap_16(0x8000) )
    }
    if !context.is_null() { mess.siaddr = (*context).local }
    /* See if we can send the boot stuff as options.
     To do this we need a requested option list, BOOTP
     and very old DHCP clients won't have this, we also 
     provide a manual option to disable it.
     Some PXE ROMs have bugs (surprise!) and need zero-terminated 
     names, so we always send those.  */
    boot = find_boot(tagif);
    if !boot.is_null() {
        if !boot.sname.is_null() {
            if daemon.options[(30                                 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                   ).wrapping_mul(8                                             libc::c_int                                      ))
                                             ] &
                   (1) <<
                       (30 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                               ).wrapping_mul(8         libc::c_int  ))
                   == 0 && !req_options.is_null() &&
                   in_list(req_options, 66) != 0 {
                option_put_string(mess, end, 66, boot.sname,
                                  1);
            } else {
                safe_strncpy(mess.sname.as_mut_ptr() ,
                             boot.sname,
                             ::std::mem::size_of::<[u8; 64]>()));
            }
        }
        if !boot.file.is_null() {
            if daemon.options[(30                                 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                   ).wrapping_mul(8                                             libc::c_int                                      ))
                                             ] &
                   (1) <<
                       (30 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                               ).wrapping_mul(8         libc::c_int  ))
                   == 0 && !req_options.is_null() &&
                   in_list(req_options, 67) != 0 {
                option_put_string(mess, end, 67, boot.file,
                                  1);
            } else {
                safe_strncpy(mess.file.as_mut_ptr() ,
                             boot.file,
                             ::std::mem::size_of::<[u8; 128]>()));
            }
        }
        if boot.next_server.s_addr != 0 {
            mess.siaddr = boot.next_server
        } else if !boot.tftp_sname.is_null() {
            mess.siaddr = a_record_from_hosts(boot.tftp_sname, now)
        }
    } else {
        /* Use the values of the relevant options if no dhcp-boot given and
       they're not explicitly asked for as options. OPTION_END is used
       as an internal way to specify siaddr without using dhcp-boot, for use in
       dhcp-optsfile. */
        if (req_options.is_null() ||
                in_list(req_options, 67) == 0) &&
               { opt = option_find2(67); !opt.is_null() } &&
               (*opt).flags & 16 == 0 {
            safe_strncpy(mess.file.as_mut_ptr() ,
                         (*opt).val ,
                         ::std::mem::size_of::<[u8; 128]>());
            done_file = 1
        }
        if (req_options.is_null() ||
                in_list(req_options, 66) == 0) &&
               { opt = option_find2(66); !opt.is_null() } &&
               (*opt).flags & 16 == 0 {
            safe_strncpy(mess.sname.as_mut_ptr() ,
                         (*opt).val ,
                         ::std::mem::size_of::<[u8; 64]>());
            done_server = 1
        }
        opt = option_find2(255);
        if !opt.is_null() {
            mess.siaddr.s_addr = (*((*opt).val)).s_addr
        }
    }
    /* We don't want to do option-overload for BOOTP, so make the file and sname
     fields look like they are in use, even when they aren't. This gets restored
     at the end of this function. */
    if req_options.is_null() ||
           daemon.options[(30 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (30 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               != 0 {
        f0 = mess.file[0 ];
        mess.file[0 ] = 1 as u8;
        s0 = mess.sname[0 ];
        mess.sname[0 ] = 1 as u8
    }
    /* At this point, if mess->sname or mess->file are zeroed, they are available
     for option overload, reserve space for the overload option. */
    if mess.file[0 ] ==
           0 ||
           mess.sname[0 ] ==
               0 {
        end = end.offset(-(3))
    }
    /* rfc3011 says this doesn't need to be in the requested options list. */
    if subnet_addr.s_addr != 0 {
        option_put(mess, end, 118, 4,
                   __bswap_32(subnet_addr.s_addr));
    }
    if lease_time != 0xffffffff {
        let mut t1val: u32 =
            lease_time.wrapping_div(2);
        let mut t2val: u32 =
            lease_time.wrapping_mul(7                                  libc::c_uint).wrapping_div(8                    libc::c_int
                                                                                          libc::c_uint);
        let mut hval: u32 = 0;
        /* If set by user, sanity check, so not longer than lease. */
        opt = option_find2(58);
        if !opt.is_null() {
            hval = __bswap_32(*((*opt).val));
            if hval < lease_time && hval > 2 {
                t1val = hval
            }
        }
        opt = option_find2(59);
        if !opt.is_null() {
            hval = __bswap_32(*((*opt).val));
            if hval < lease_time && hval > 2 {
                t2val = hval
            }
        }
        /* ensure T1 is still < T2 */
        if t2val <= t1val {
            t1val = t2val.wrapping_sub(1)
        }
        while fuzz >
                  t1val.wrapping_div(8) {
            fuzz = (fuzz / 2)
        }
        t1val = t1val.wrapping_sub(fuzz);
        t2val = t2val.wrapping_sub(fuzz);
        option_put(mess, end, 58, 4, t1val);
        option_put(mess, end, 59, 4, t2val);
    }
    /* replies to DHCPINFORM may not have a valid context */
    if !context.is_null() {
        if option_find2(1).is_null() {
            option_put(mess, end, 1, 4,
                       __bswap_32((*context).netmask.s_addr));
        }
        /* May not have a "guessed" broadcast address if we got no packets via a relay
	 from this net yet (ie just unicast renewals after a restart */
        if (*context).broadcast.s_addr != 0 &&
               option_find2(28).is_null() {
            option_put(mess, end, 28, 4,
                       __bswap_32((*context).broadcast.s_addr));
        }
        /* Same comments as broadcast apply, and also may not be able to get a sensible
	 default when using subnet select.  User must configure by steam in that case. */
        if (*context).router.s_addr != 0 &&
               in_list(req_options, 3) != 0 &&
               option_find2(3).is_null() {
            option_put(mess, end, 3, 4,
                       __bswap_32((*context).router.s_addr));
        }
        if daemon.port == 53 &&
               in_list(req_options, 6) != 0 &&
               option_find2(6).is_null() {
            option_put(mess, end, 6, 4,
                       __bswap_32((*context).local.s_addr));
        }
    }
    if !domain.is_null() && in_list(req_options, 15) != 0 &&
           option_find2(15).is_null() {
        option_put_string(mess, end, 15, domain, null_term);
    }
    /* Note that we ignore attempts to set the fqdn using --dhc-option=81,<name> */
    if !hostname.is_null() {
        if in_list(req_options, 12) != 0 &&
               option_find2(12).is_null() {
            option_put_string(mess, end, 12, hostname,
                              null_term); /* MBZ bits to zero */
        }
        if fqdn_flags != 0 {
            len =
                strlen(hostname).wrapping_add(3 ) ;
            if fqdn_flags & 0x4 != 0 {
                len += 2
            } else if null_term != 0 { len += 1 }
            if !domain.is_null() {
                len =
                    (len).wrapping_add(strlen(domain).wrapping_add(1
                                                                                                                      libc::c_int
                                                                                                               ))

            } else if fqdn_flags & 0x4 != 0 {
                len -= 1
            }
            p = free_space(mess, end, 81, len);
            if !p.is_null() {
                let fresh34 = p;
                p = p.offset(1);
                *fresh34 =
                    (fqdn_flags & 0xf) ;
                let fresh35 = p;
                p = p.offset(1);
                *fresh35 = 255;
                let fresh36 = p;
                p = p.offset(1);
                *fresh36 = 255;
                if fqdn_flags & 0x4 != 0 {
                    p = do_rfc1035_name(p, hostname, 0 );
                    if !domain.is_null() {
                        p =
                            do_rfc1035_name(p, domain,
                                            0 );
                        let fresh37 = p;
                        p = p.offset(1);
                        *fresh37 = 0
                    }
                } else {
                    memcpy(p,
                           hostname, strlen(hostname));
                    p = p.offset(strlen(hostname));
                    if !domain.is_null() {
                        let fresh38 = p;
                        p = p.offset(1);
                        *fresh38 = '.' as i32;
                        memcpy(p,
                               domain, strlen(domain));
                        p = p.offset(strlen(domain))
                    }
                    if null_term != 0 {
                        let fresh39 = p;
                        p = p.offset(1);
                        *fresh39 = 0
                    }
                }
            }
        }
    }
    opt = config_opts;
    while !opt.is_null() {
        let mut optno: i32 = (*opt).opt;
        /* netids match and not encapsulated? */
        if !((*opt).flags & 4096 == 0) {
            /* was it asked for, or are we sending it anyway? */
            if !((*opt).flags & 16 == 0 &&
                     in_list(req_options, optno) == 0) {
                /* prohibit some used-internally options. T1 and T2 already handled. */
                if !(optno == 81 || optno == 57
                         || optno == 52 ||
                         optno == 0 ||
                         optno == 255 ||
                         optno == 58 ||
                         optno == 59) {
                    if !(optno == 66 && done_server != 0) {
                        if !(optno == 67 && done_file != 0) {
                            /* For the options we have default values on
	 dhc-option=<optionno> means "don't include this option"
	 not "include a zero-length option" */
                            if !((*opt).len == 0 &&
                                     (optno == 1 ||
                                          optno == 28 ||
                                          optno == 3 ||
                                          optno == 6 ||
                                          optno == 15 ||
                                          optno == 12)) {
                                /* vendor-class comes from elsewhere for PXE */
                                if !(pxe_arch != -(1) &&
                                         optno == 60) {
                                    /* always force null-term for filename and servername - buggy PXE again. */
                                    len =
                                        do_opt(opt, 0,
                                               context,
                                               if optno == 66
                                                      ||
                                                      optno ==
                                                          67 {
                                                   1
                                               } else { null_term });
                                    p = free_space(mess, end, optno, len);
                                    if !p.is_null() {
                                        do_opt(opt, p, context,
                                               if optno == 66
                                                      ||
                                                      optno ==
                                                          67 {
                                                   1
                                               } else { null_term });
                                        /* If we send a vendor-id, revisit which vendor-ops we consider 
	     it appropriate to send. */
                                        if optno == 60 {
                                            match_vendor_opts(p.offset(-(2                          libc::c_int
                                                                )),
                                                              config_opts);
                                            done_vendor_class =
                                                1
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        opt = (*opt).next
    }
    /* Now send options to be encapsulated in arbitrary options, 
     eg dhcp-option=encap:172,17,.......
     Also handle vendor-identifying vendor-encapsulated options,
     dhcp-option = vi-encap:13,17,.......
     The may be more that one "outer" to do, so group
     all the options which match each outer in turn. */
    opt = config_opts;
    while !opt.is_null() {
        (*opt).flags &= !(64);
        opt = (*opt).next
    }
    opt = config_opts;
    while !opt.is_null() {
        let mut flags: i32 = 0;
        flags = (*opt).flags & (4 | 2048);
        if flags != 0 {
            let mut found: i32 = 0;
            let mut o: *mut DhcpOpt = 0 ;
            if !((*opt).flags & 64 != 0) {
                len = 0;
                o = config_opts;
                while !o.is_null() {
                    let mut outer: i32 =
                        if flags & 4 != 0 {
                            (*o).u.encap
                        } else { 125 };
                    (*o).flags &= !(8);
                    if !((*o).flags & flags == 0 ||
                             (*opt).u.encap != (*o).u.encap) {
                        (*o).flags |= 64;
                        if match_netid((*o).netid, tagif, 1) !=
                               0 &&
                               ((*o).flags & 16 != 0 ||
                                    in_list(req_options, outer) != 0) {
                            (*o).flags |= 8;
                            found = 1;
                            len +=
                                do_opt(o, 0,
                                       0,
                                       0) + 2
                        }
                    }
                    o = (*o).next
                }
                if found != 0 {
                    if flags & 4 != 0 {
                        do_encap_opts(config_opts, (*opt).u.encap,
                                      8, mess, end, null_term);
                    } else if len > 250 {
                        my_syslog((3) << 3 |
                                      4,
                                  b"cannot send RFC3925 option: too many options for enterprise number %d\x00"
                                      ,
                                  (*opt).u.encap);
                    } else {
                        p =
                            free_space(mess, end, 125,
                                       len + 5);
                        if !p.is_null() {
                            let mut swap_ent: i32 =
                                __bswap_32((*opt).u.encap)                              libc::c_int;
                            memcpy(p,
                                   &mut swap_ent as
                                   4);
                            p = p.offset(4);
                            let fresh40 = p;
                            p = p.offset(1);
                            *fresh40 = len;
                            o = config_opts;
                            while !o.is_null() {
                                if (*o).flags & 8 != 0 {
                                    len =
                                        do_opt(o,
                                               p.offset(2         isize),
                                               0,
                                               0);
                                    let fresh41 = p;
                                    p = p.offset(1);
                                    *fresh41 = (*o).opt;
                                    let fresh42 = p;
                                    p = p.offset(1);
                                    *fresh42 = len;
                                    p = p.offset(len)
                                }
                                o = (*o).next
                            }
                        }
                    }
                }
            }
        }
        opt = (*opt).next
    }
    force_encap = prune_vendor_opts(tagif);
    if !context.is_null() && pxe_arch != -(1) {
        pxe_misc(mess, end, uuid, pxevendor);
        if pxe_uefi_workaround(pxe_arch, tagif, mess, (*context).local, now,
                               0) == 0 {
            config_opts = pxe_opts(pxe_arch, tagif, (*context).local, now)
        }
    }
    if (force_encap != 0 || in_list(req_options, 43) != 0) &&
           do_encap_opts(config_opts, 43, 1024,
                         mess, end, null_term) != 0 &&
           pxe_arch == -(1) && done_vendor_class == 0 &&
           vendor_class_len != 0 &&
           {
               p = free_space(mess, end, 60, vendor_class_len);
               !p.is_null()
           } {
        /* If we send vendor encapsulated options, and haven't already sent option 60,
       echo back the value we got from the client. */
        memcpy(p,
               daemon.dhcp_buff3,
               vendor_class_len);
    }
    /* restore BOOTP anti-overload hack */
    if req_options.is_null() ||
           daemon.options[(30 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (30 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               != 0 {
        mess.file[0 ] = f0;
        mess.sname[0 ] = s0
    };
}
fn apply_delay(mut xid: u32, mut recvtime: time::Instant,
                                 mut netid: *mut DhcpNetId) {
    let mut delay_conf: *mut DelayConfig = 0 ;
    /* Decide which delay_config option we're using */
    delay_conf = daemon.delay_conf;
    while !delay_conf.is_null() {
        if match_netid((*delay_conf).netid, netid, 0) != 0 {
            break ;
        }
        delay_conf = (*delay_conf).next
    }
    if delay_conf.is_null() {
        /* No match, look for one without a netid */
        delay_conf = daemon.delay_conf;
        while !delay_conf.is_null() {
            if match_netid((*delay_conf).netid, netid, 1) != 0
               {
                break ;
            }
            delay_conf = (*delay_conf).next
        }
    }
    if !delay_conf.is_null() {
        if daemon.options[(42 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (42 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               == 0 {
            my_syslog((3) << 3 |
                          6,
                      b"%u reply delay: %d\x00", __bswap_32(xid),
                      (*delay_conf).delay);
        }
        delay_dhcp(recvtime, (*delay_conf).delay, -(1),
                   0,
                   0 );
    };
}
