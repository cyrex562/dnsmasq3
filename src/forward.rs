/* Send a UDP packet with its source address set as "source"
   unless nowild is true, when we just send it with the kernel default */
use crate::auth::{answer_auth, in_zone};
use crate::cache::{querystr};
use crate::defines::{ConstNetAddressArg, AddressListEntry, NetAddress, AuthZone, C2rustUnnamed14, C2rustUnnamed15, C2rustUnnamed16, C2RustUnnamed4, CmsgHdr, DnsHeader, DnsmasqDaemon, Frec, FrecSrc, IfReq, InPktInfo, IPPROTO_IP, IPPROTO_IPV6, IpSets, Irec, Listener, MSG_FASTOPEN, MSG_TRUNC, MsgHdr, RandFd, SaFamily, Server, SOCK_STREAM};
use crate::dnsmasq_loop::detect_loop;
use crate::dump::dump_packet;
use crate::edns0::{add_do_bit, add_edns0_config, add_pseudoheader, check_source, find_pseudoheader};
use crate::hash_questions::hash_questions;
use crate::network::{enumerate_interfaces, iface_check, indextoname, label_exception, local_bind, loopback_exception, random_sock};
use crate::rfc1035::{answer_request, check_for_bogus_wildcard, check_for_ignored_address, check_for_local_domain, extract_addresses, extract_request, resize_packet, setup_reply};
use crate::rrfilter::rrfilter;
use crate::util::{hostname_isequal, is_same_net4, is_same_net6, prettyprint_addr, retry_send, sa_len, NetAddress_isequal};


pub fn send_from(mut fd: i32,
                 mut nowild: i32,
                 mut packet: &mut String,
                 mut len: usize, mut to: NetAddress,
                 mut source: &mut NetAddress,
                 mut iface: u32) -> i32 {
    // let mut msg: MsgHdr =
    //     MsgHdr {
    //         msg_name: 0,
    //         msg_namelen: 0,
    //         msg_iov: 0,
    //         msg_iovlen: 0,
    //         msg_control: 0,
    //         msg_controllen: 0,
    //         msg_flags:
    //         0,
    //     }; /* Need iface for IPv6 to handle link-local addrs */
    // let mut iov: [iovec; 1] =
    //     [iovec { iov_base: 0, iov_len: 0 }; 1];
    let mut control_u: C2RustUnnamed_13 =
        C2RustUnnamed_13 {
            align:
            CmsgHdr {
                cmsg_len: 0,
                cmsg_level: 0,
                cmsg_type: 0,
                __cmsg_data: [],
            },
        };
    // iov[0].iov_base = packet;
    // iov[0].iov_len = len;
    // msg.msg_control = 0;
    // msg.msg_controllen = 0;
    // msg.msg_flags = 0;
    // msg.msg_name = to;
    // msg.msg_namelen = sa_len(to);
    // msg.msg_iov = iov;
    // msg.msg_iovlen = 1;
    if nowild == 0 {
        // let mut cmptr: CmsgHdr = 0;
        // msg.msg_control =
        //     &mut control_u;
        // msg.msg_controllen =
        //     ::std::mem::size_of::<C2RustUnnamed_13>();
        // cmptr =
        //     if msg.msg_controllen >=
        //         ::std::mem::size_of::<CmsgHdr>() {
        //         msg.msg_control
        //     } else { 0 };
        if to.sa.sa_family == 2 {
            let mut p = InPktInfo::new();
            p.ipi_ifindex = 0;
            p.ipi_spec_dst = source.addr4;
            msg.msg_controllen =
                ((::std::mem::size_of::<InPktInfo>()).wrapping_add(::std::mem::size_of::<usize>()
                ).wrapping_sub(1)
                    &
                    !(::std::mem::size_of::<usize>()).wrapping_sub(1)).wrapping_add((::std::mem::size_of::<CmsgHdr>()).wrapping_add(::std::mem::size_of::<usize>()).wrapping_sub(1
                                                                                                                                              )
            &
                !(::std::mem::size_of::<usize>()).wrapping_sub(1
                                                                                        ));
            memcpy(cmptr.__cmsg_data,
                   &mut p,
                   ::std::mem::size_of::<InPktInfo>());
            cmptr.cmsg_len =
                ((::std::mem::size_of::<CmsgHdr>()).wrapping_add(::std::mem::size_of::<usize>()
                ).wrapping_sub(1)
                    &
                    !(::std::mem::size_of::<usize>()).wrapping_sub(1)).wrapping_add(::std::mem::size_of::<InPktInfo>());
            cmptr.cmsg_level = IPPROTO_IP;
            cmptr.cmsg_type = 8
        } else {
            let mut p_0: in6_pktinfo =
                in6_pktinfo {
                    ipi6_addr:
                    In6Addr {
                        __in6_u:
                        C2RustUnnamed_0 {
                            __u6_addr8:
                            [0; 16],
                        },
                    },
                    ipi6_ifindex: 0,
                };
            p_0.ipi6_ifindex = iface;
            p_0.ipi6_addr = source.addr6;
            msg.msg_controllen =
                ((::std::mem::size_of::<in6_pktinfo>()).wrapping_add(::std::mem::size_of::<usize>()
                ).wrapping_sub(1)
                    &
                    !(::std::mem::size_of::<usize>()).wrapping_sub(1)).wrapping_add((::std::mem::size_of::<CmsgHdr>()).wrapping_add(::std::mem::size_of::<usize>()).wrapping_sub(1
                                                                                                                                              )
            &
                !(::std::mem::size_of::<usize>()).wrapping_sub(1
                                                                                        ));
            memcpy(cmptr.__cmsg_data,
                   &mut p_0,
                   ::std::mem::size_of::<in6_pktinfo>());
            cmptr.cmsg_len =
                ((::std::mem::size_of::<CmsgHdr>()).wrapping_add(::std::mem::size_of::<usize>()
                ).wrapping_sub(1)
                    &
                    !(::std::mem::size_of::<usize>()).wrapping_sub(1)).wrapping_add(::std::mem::size_of::<in6_pktinfo>());
            cmptr.cmsg_type = daemon.v6pktinfo;
            cmptr.cmsg_level = IPPROTO_IPV6
        }
    }
    while retry_send(sendmsg(fd, &mut msg, 0)) != 0 {}
    if *__errno_location() != 0 {
        /* If interface is still in DAD, EINVAL results - ignore that. */
        if *__errno_location() != 22 {
            my_syslog(3,
                      "failed to send packet: %s", strerror(*__errno_location()));
        }
        return 0;
    }
    return 1;
}

fn search_servers(mut now: time::Instant,
                  mut addrpp: &mut NetAddress,
                  mut qtype: u32,
                  mut qdomain: &mut String,
                  mut type_0: u32 ,
                  mut domain: String,
                  mut norebind: bool)
                  -> libc::c_uint {
    /* If the query ends in the domain in one of our servers, set
     domain to point to that name. We find the largest match to allow both
     domain.org and sub.domain.org to exist. */
    let mut namelen: u32 = strlen(qdomain);
    let mut matchlen: u32 = 0;
    let mut serv: Server = 0;
    let mut flags: u32 = 0;
    // static mut zero: AllAddr = AllAddr {addr4: InAddr {s_addr: 0,},};
    let mut current_block_45: u64;
    serv = daemon.servers;
    while !serv.is_null() {
        if !(qtype == (1) << 15 &&
            serv.flags & 16384 == 0) {
            /* domain matches take priority over NODOTS matches */
            if serv.flags & 32 != 0 &&
                *type_0 != 8 &&
                strchr(qdomain, '.' as i32).is_null() &&
                namelen != 0 {
                let mut sflag: u32 =
                    if serv.addr.sa.sa_family ==
                        2 {
                        ((1)) << 7
                    } else { ((1)) << 8 };
                *type_0 = 32;
                if serv.flags & 2048 != 0 &&
                    !norebind.is_null() {
                    *norebind = 1
                } else if serv.flags & 2 != 0 {
                    flags = (1) << 10
                } else if serv.flags & 4 != 0 {
                    /* literal address = '#' -> return all-zero address for IPv4 and IPv6 */
                    if serv.flags & 1024 != 0 &&
                        qtype &
                            ((1) << 8 |
                                (1) << 7)
                            != 0 {
                        zero = Default::default();
                        flags = qtype;
                        *addrpp = &mut zero
                    } else if sflag & qtype != 0 {
                        flags = sflag;
                        if serv.addr.sa.sa_family == 2 {
                            *addrpp = &mut serv.addr.in_0.sin_addr;
                        } else {
                            *addrpp = &mut serv.addr.in6.sin6_addr;
                        }
                    } else if flags == 0 || flags & (1) << 10 != 0 { flags = (1) << 20 }
                }
            } else if serv.flags & 8 != 0 {
                let mut domainlen: u32 =
                    strlen(serv.domain);
                let mut matchstart: &mut String =
                    qdomain.offset(namelen
                ).offset(-(domainlen));
                if namelen >= domainlen &&
                    hostname_isequal(matchstart, serv.domain) != 0 &&
                    (domainlen == 0 ||
                        namelen == domainlen ||
                        *matchstart.offset(-(1))
                            == '.' as i32) {
                    if serv.flags & 2048 != 0 &&
                        !norebind.is_null() {
                        *norebind = 1
                    } else {
                        let mut sflag_0: u32 =
                            if serv.addr.sa.sa_family ==
                                2 {
                                ((1)) << 7
                            } else {
                                ((1)) << 8
                            };
                        /* implement priority rules for --address and --server for same domain.
		   --address wins if the address is for the correct AF
		   --server wins otherwise. */
                        if domainlen != 0 &&
                            domainlen == matchlen {
                            if serv.flags & 4 != 0 {
                                if sflag_0 & qtype == 0 &&
                                    flags ==
                                        0 {
                                    current_block_45 = 4644295000439058019;
                                } else {
                                    current_block_45 = 6450636197030046351;
                                }
                            } else if flags &
                                ((1) <<
                                    7 |
                                    (1) <<
                                        8) != 0 {
                                current_block_45 = 4644295000439058019;
                            } else { current_block_45 = 6450636197030046351; }
                        } else { current_block_45 = 6450636197030046351; }
                        match current_block_45 {
                            4644295000439058019 => {}
                            _ => {
                                if domainlen >= matchlen {
                                    *type_0 =
                                        serv.flags &
                                            (8 |
                                                1024 |
                                                2048 |
                                                16384);
                                    *domain = serv.domain;
                                    matchlen = domainlen;
                                    if serv.flags & 2 != 0 {
                                        flags =
                                            (1) <<
                                                10
                                    } else if serv.flags & 4
                                        != 0 {
                                        /* literal address = '#' -> return all-zero address for IPv4 and IPv6 */
                                        if serv.flags & 1024 != 0 && qtype & ((1) << 8 |  (1) << 7) != 0 {
                                            flags = qtype;
                                            *addrpp = &mut zero
                                        } else if sflag_0 & qtype != 0 {
                                            flags = sflag_0;
                                            if serv.addr.sa.sa_family == 2
                                            {
                                                *addrpp =
                                                    &mut serv.addr.in_0.sin_addr
                                            } else {
                                                *addrpp =
                                                    &mut serv.addr.in6.sin6_addr
                                            }
                                        } else if flags == 0 ||
                                            flags & (1) << 10 != 0
                                        {
                                            flags = (1) << 20
                                        }
                                    } else {
                                        flags = 0
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        serv = serv.next
    }
    if flags == 0 &&
        qtype &
            ((1) << 19 |
                (1) << 15) == 0 &&
        daemon.options
    [12] != 0 && strchr(qdomain, '.' as i32).is_null() &&
        namelen != 0
    {
        /* don't forward A or AAAA queries for simple names, except the empty name */
        flags = (1) << 20
    }
    if flags == (1) << 10 &&
        check_for_local_domain(qdomain, now) != 0 {
        flags = (1) << 20
    }
    if flags != 0 {
        if flags == (1) << 10 ||
            flags == (1) << 20 {
            // log_query(flags | qtype | (1) << 5
            //               | (1) << 13 |
            //               (1) << 3, qdomain,
            //           0, 0);
        } else {
            /* handle F_IPV4 and F_IPV6 set on ANY query to 0.0.0.0/:: domain. */
            if flags & (1) << 7 != 0 {
                // log_query((flags | (1) << 13 |
                //     (1) << 3) &
                //               !((1) << 8),
                //           qdomain, *addrpp,
                //           0 & mut String); /* use normal servers for this domain */
            }
            if flags & (1) << 8 != 0 {
                // log_query((flags | (1) << 13 |
                //     (1) << 3) &
                //               !((1) << 7),
                //           qdomain, *addrpp, 0);
            }
        }
    } else if *type_0 & 1024 != 0 {
        *type_0 = 0;
        *domain = 0
    }
    return flags;
}

fn forward_query(mut udpfd: i32,
                 mut udpaddr: NetAddress,
                 mut dst_addr: &mut NetAddress,
                 mut dst_iface: u32,
                 mut header: DnsHeader,
                 mut plen: usize, mut now: time::Instant,
                 mut forward: &mut Frec,
                 mut ad_reqd: i32,
                 mut do_bit: i32) -> i32 {
    let mut domain: &mut String = 0;
    let mut type_0: i32 = 16384;
    let mut norebind: i32 = 0;
    let mut addrp: &mut NetAddress = 0;
    let mut flags: u32 = 0;
    let mut fwd_flags: u32 = 0;
    let mut start: Server = 0;
    let mut hash: Vec<u8> = hash_questions(header, plen, daemon.namebuff);
    let mut gotname: u32 =
        extract_request(header, plen, daemon.namebuff,
                        0);
    let mut oph: Vec<u8> =
        find_pseudoheader(header, plen, 0,
                          0, 0,
                          0);
    if header.hb4 & 0x10 != 0 {
        fwd_flags |= 2
    }
    if ad_reqd != 0 { fwd_flags |= 32 }
    if !oph.is_null() { fwd_flags |= 1024 }
    /* may be no servers available. */
    if !forward.is_null() ||
        {
            forward =
                lookup_frec_by_sender(__bswap_16(header.id), udpaddr,
                                      hash);
            !forward.is_null()
        } {
        /* If we didn't get an answer advertising a maximal packet in EDNS,
	 fall back to 1280, which should work everywhere on IPv6.
	 If that generates an answer, it will become the new default
	 for this server */
        forward.flags |= 256;
        /* retry on existing query, send to all available servers  */
        domain = (*forward.sentto).domain; /* at end of list, recycle */
        (*forward.sentto).failed_queries =
            (*forward.sentto).failed_queries.wrapping_add(1);
        if daemon.options[7] == 0
        {
            forward.forwardall = 1;
            daemon.last_server = 0
        }
        type_0 = (*forward.sentto).flags & (8 | 32);
        start = (*forward.sentto).next;
        if start.is_null() { start = daemon.servers }
        header.id = __bswap_16(forward.new_id)
    } else {
        /* Query from new source, but the same query may be in progress
	 from another source. If so, just add this client to the
	 list that will get the reply.*/
        if daemon.options[32] == 0 && daemon.options[54] == 0 &&
            {
                forward = lookup_frec_by_query(hash, fwd_flags);
                !forward.is_null()
            }
        {
            /* Note whine_malloc() zeros memory. */
            if daemon.free_frec_src.is_null() &&
                daemon.frec_src_count <
                    daemon.ftabsize &&
                {
                    // daemon.free_frec_src =
                    //     whine_malloc(::std::mem::size_of::<FrecSrc>());
                    // !daemon.free_frec_src.is_null()
                    true
                } {
                daemon.frec_src_count += 1;
                (*daemon.free_frec_src).next = 0
            }
            /* If we've been spammed with many duplicates, just drop the query. */
            if !daemon.free_frec_src.is_null() {
                let mut new: FrecSrc = daemon.free_frec_src;
                daemon.free_frec_src = new.next;
                new.next = forward.frec_src.next;
                forward.frec_src.next = new;
                new.orig_id = __bswap_16(header.id);
                new.source = *udpaddr;
                new.dest = *dst_addr;
                new.log_id = daemon.log_id;
                new.iface = dst_iface;
                new.fd = udpfd
            }
            return 1;
        }
        if gotname != 0 {
            flags =
                search_servers(now, &mut addrp, gotname,
                               daemon.namebuff, &mut type_0,
                               &mut domain, &mut norebind)
        }
        type_0 &= !(16384);
        if !daemon.servers.is_null() && flags == 0 {
            forward = get_new_frec(now, 0, 0)
        }
        /* table full - flags == 0, return REFUSED */
        if !forward.is_null() {
            forward.frec_src.source = *udpaddr;
            forward.frec_src.orig_id = __bswap_16(header.id);
            forward.frec_src.dest = *dst_addr;
            forward.frec_src.iface = dst_iface;
            forward.frec_src.next = 0;
            forward.frec_src.fd = udpfd;
            forward.new_id = get_id();
            memcpy(forward.hash, hash,
                   32);
            forward.forwardall = 0;
            forward.flags = fwd_flags;
            if norebind != 0 { forward.flags |= 1 }
            if header.hb4 & 0x10 != 0 {
                forward.flags |= 2
            }
            if ad_reqd != 0 { forward.flags |= 32 }
            header.id = __bswap_16(forward.new_id);
            /* In strict_order mode, always try servers in the order 
	     specified in resolv.conf, if a domain is given 
	     always try all the available servers,
	     otherwise, use the one last known to work. */
            if type_0 == 0 {
                if daemon.options
                [(7).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                  ).wrapping_mul(8
                                                              ))
                ] &
                    (1) <<
                    (7).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                     ).wrapping_mul(8
                          ))
                != 0
                {
                    start = daemon.servers
                } else {
                    start = daemon.last_server;
                    if start.is_null() ||
                        {
                            let fresh6 = daemon.forwardcount;
                            daemon.forwardcount =
                                daemon.forwardcount + 1;
                            (fresh6) > 50
                        } ||
                        difftime(now, daemon.forwardtime) >
                            20 {
                        start = daemon.servers;
                        forward.forwardall = 1;
                        daemon.forwardcount = 0;
                        daemon.forwardtime = now
                    }
                }
            } else {
                start = daemon.servers;
                if daemon.options
                [(7).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                  ).wrapping_mul(8
                                                              ))
                ] &
                    (1) <<
                    (7).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                     ).wrapping_mul(8
                          ))
                == 0
                {
                    forward.forwardall = 1
                }
            }
        }
    }
    /* check for send errors here (no route to host) 
     if we fail to send to all nameservers, send back an error
     packet straight away (helps modem users when offline)  */
    if flags == 0 && !forward.is_null() {
        let mut firstsentto: Server = start;
        let mut subnet: i32 = 0;
        let mut cacheable: i32 = 0;
        let mut forwarded: i32 = 0;
        let mut edns0_len: usize = 0;
        let mut pheader: Vec<u8> = 0;
        /* cancel */
        forward.frec_src.log_id = daemon.log_id;
        plen =
            add_edns0_config(header, plen,
                             (header).offset(512

        ),
        &mut forward.frec_src.source, now,
        &mut subnet, &mut cacheable);
        if subnet != 0 { forward.flags |= 4 }
        if cacheable == 0 { forward.flags |= 2048 }
        if !find_pseudoheader(header, plen, &mut edns0_len, &mut pheader,
                              0,
                              0).is_null() {
            /* If a query is retried, use the log_id for the retry when logging the answer. */
            /* If there wasn't a PH before, and there is now, we added it. */
            if oph.is_null() { forward.flags |= 128 }
            /* If we're sending an EDNS0 with any options, we can't recreate the query from a reply. */
            if edns0_len > 11 {
                forward.flags |= 512
            }
            /* Reduce udp size on retransmits. */
            if forward.flags & 256 != 0 {
                let mut t_s: u16 = 1280;
                let mut t_cp: Vec<u8> = pheader;
                let fresh7 = t_cp;
                t_cp = t_cp.offset(1);
                *fresh7 =
                    (t_s >> 8);
                *t_cp = t_s;
                pheader = pheader.offset(2)
            }
        }
        loop
        /* only send to servers dealing with our domain.
    domain may be NULL, in which case server->domain
    must be NULL also. */
        {
            if type_0 ==
                start.flags & (8 | 32) &&
                (type_0 != 8 ||
                    hostname_isequal(domain, start.domain) != 0) &&
                start.flags & (4 | 8192)
                    == 0 {
                let mut fd: i32 = 0;
                /* find server socket to use, may need to get random one. */
                if !start.sfd.is_null() {
                    fd = (*start.sfd).fd
                } else if start.addr.sa.sa_family ==
                    10 {
                    if forward.rfd6.is_null() &&
                        {
                            forward.rfd6 =
                                allocate_rfd(10);
                            forward.rfd6.is_null()
                        } {
                        break;
                    }
                    daemon.rfd_save = forward.rfd6;
                    fd = (*forward.rfd6).fd
                } else {
                    if forward.rfd4.is_null() &&
                        {
                            forward.rfd4 =
                                allocate_rfd(2);
                            forward.rfd4.is_null()
                        } {
                        break;
                    }
                    daemon.rfd_save = forward.rfd4;
                    fd = (*forward.rfd4).fd
                }
                if retry_send(sendto(fd, header, plen,
                0,
                ConstNetAddressArg::new(),
                sa_len(&mut start.addr)))
                != 0
                {
                    continue;
                }
                if *__errno_location() == 0 {
                    dump_packet(0x4,
                                header, plen,
                                0, &mut start.addr);
                    /* Keep info in case we want to re-send this packet */
                    daemon.srv_save = start;
                    daemon.packet_len = plen;
                    if gotname == 0 {
                        strcpy(daemon.namebuff, "query");
                    }
                    if start.addr.sa.sa_family ==
                        2 {
                        // log_query((1) << 18 |
                        //               (1) << 7
                        //               |
                        //               (1) << 3,
                        //           daemon.namebuff,
                        //           &mut start.addr.in_0.sin_addr
                        // NetAddress,
                        // 0 );
                    } else {
                        // log_query((1) << 18 |
                        //               (1) << 8
                        //               |
                        //               (1) << 3,
                        //           daemon.namebuff,
                        //           &mut start.addr.in6.sin6_addr * mut In6Addr,
                        // 0 );
                    }
                    start.queries = start.queries.wrapping_add(1);
                    forwarded = 1;
                    forward.sentto = start;
                    if forward.forwardall == 0 { break; }
                    forward.forwardall += 1
                }
            }
            start = start.next;
            if start.is_null() { start = daemon.servers }
            if start == firstsentto { break; }
        }
        if forwarded != 0 { return 1; }
        header.id = __bswap_16(forward.frec_src.orig_id);
        free_frec(forward);
    }
    /* could not send on, prepare to return */
    /* could not send on, return empty answer or address if known for whole domain */
    if udpfd != -(1) {
        plen =
            setup_reply(header, plen, addrp, flags,
                        daemon.local_ttl);
        if !oph.is_null() {
            plen =
                add_pseudoheader(header, plen, (header).offset(512),
            daemon.edns_pktsz,
            0, 0,
            0, do_bit,
            0)
        }
        send_from(udpfd,
                  (daemon.options[13] != 0 || daemon.options[39] != 0), header,
        plen, udpaddr, dst_addr, dst_iface);
    }
    return 0;
}

fn process_reply(mut header: DnsHeader,
                 mut now: time::Instant, mut server: Server,
                 mut n: usize,
                 mut check_rebind: i32,
                 mut no_cache: i32,
                 mut cache_secure: i32,
                 mut bogusanswer: i32,
                 mut ad_reqd: i32,
                 mut do_bit: i32,
                 mut added_pheader: i32,
                 mut check_subnet: i32,
                 mut query_source: NetAddress)
                 -> usize {
    let mut pheader: Vec<u8> = 0;
    let mut sizep: Vec<u8> = 0;
    let mut sets: String = 0;
    let mut munged: i32 = 0;
    let mut is_sign: i32 = 0;
    let mut rcode: u32 =
        (header.hb4 & 0xf);
    let mut plen: usize = 0;
    if !daemon.ipsets.is_null() &&
        extract_request(header, n, daemon.namebuff,
                        0) != 0 {
        /* Similar algorithm to search_servers. */
        let mut ipset_pos: IpSets = 0;
        let mut namelen: u32 =
            strlen(daemon.namebuff);
        let mut matchlen: u32 = 0;
        ipset_pos = daemon.ipsets;
        while !ipset_pos.is_null() {
            let mut domainlen: u32 =
                strlen(ipset_pos.domain);
            let mut matchstart: &mut String =
                daemon.namebuff.offset(namelen
            ).offset(-(domainlen
            ));
            if namelen >= domainlen &&
                hostname_isequal(matchstart, ipset_pos.domain) != 0 &&
                (domainlen == 0 ||
                    namelen == domainlen ||
                    *matchstart.offset(-(1))
             == '.' as i32) &&
                domainlen >= matchlen {
                matchlen = domainlen;
                sets = (ipset_pos).sets;
            }
            ipset_pos = ipset_pos.next
        }
    }
    pheader =
        find_pseudoheader(header, n, &mut plen, &mut sizep, &mut is_sign,
                          0);
    if !pheader.is_null() {
        /* Get extended RCODE. */
        rcode |=
            ((*sizep.offset(2)) <<
                4);
        if check_subnet != 0 &&
            check_source(header, plen, pheader, query_source) == 0 {
            // my_syslog(4,
            //           "discarding DNS reply: subnet option mismatch" *const u8);
            return 0;
        }
        if is_sign == 0 {
            if added_pheader != 0 {
                /* client didn't send EDNS0, we added one, strip it off before returning answer. */
                n = rrfilter(header, n, 0);
                pheader = 0
            } else {
                let mut udpsz: u16 = 0;
                /* If upstream is advertising a larger UDP packet size
		 than we allow, trim it so that we don't get overlarge
		 requests for the client. We can't do this for signed packets. */
                let mut t_cp: Vec<u8> = sizep;
                udpsz =
                    ((*t_cp.offset(0)) << 8 |
                        *t_cp.offset(1)
                ) ;
                sizep = sizep.offset(2);
                if udpsz >
                    daemon.edns_pktsz {
                    sizep = sizep.offset(-(2));
                    let mut t_s: u16 = daemon.edns_pktsz;
                    let mut t_cp_0: Vec<u8> = sizep;
                    let fresh8 = t_cp_0;
                    t_cp_0 = t_cp_0.offset(1);
                    *fresh8 = (t_s >> 8);
                    *t_cp_0 = t_s;
                    sizep = sizep.offset(2)
                }
            }
        }
    }
    /* RFC 4035 sect 4.6 para 3 */
    if is_sign == 0 && daemon.options[33] == 0
    {
        header.hb4 =
            (header.hb4 & !(0x20)) as u8
    }
    if (header.hb3 & 0x78) >>
        3 != 0 {
        return resize_packet(header, n, pheader, plen);
    }
    if rcode != 0 &&
        rcode != 3 {
        let mut a = NetAddress::new();
        a.log.rcode = rcode;
        // log_query((1) << 16 |
        //               (1) << 29,
        //           "error" & mut String, &mut a, 0 );
        return resize_packet(header, n, pheader, plen);
    }
    /* Complain loudly if the upstream server is non-recursive. */
    if header.hb4 & 0x80 == 0 &&
        rcode == 0 && !server.is_null() &&
        server.flags & 64 == 0 {
        prettyprint_addr(&mut server.addr, daemon.namebuff);
        // my_syslog(4,
        //           "nameserver %s refused to do a recursive query" *const u8,
        // daemon.namebuff);
        if daemon.options
        [2] == 0
        {
            server.flags |= 64
        }
    }
    if !daemon.bogus_addr.is_null() &&
        rcode != 3 &&
        check_for_bogus_wildcard(header, n, daemon.namebuff,
                                 daemon.bogus_addr, now) != 0 {
        munged = 1;
        header.hb4 =
            (header.hb4 & !(0xf) |
                3) as u8;
        header.hb3 =
            (header.hb3 & !(0x4)) as u8;
        cache_secure = 0
    } else {
        let mut doctored: i32 = 0;
        if rcode == 3 &&
            extract_request(header, n, daemon.namebuff,
                            0) != 0 &&
            check_for_local_domain(daemon.namebuff, now) != 0 {
            /* if we forwarded a query for a locally known name (because it was for 
	     an unknown type) and the answer is NXDOMAIN, convert that to NODATA,
	     since we know that the domain exists, even if upstream doesn't */
            munged = 1;
            header.hb3 =
                (header.hb3 | 0x4) as u8;
            header.hb4 =
                (header.hb4 & !(0xf) |
                    0) as u8;
            cache_secure = 0
        }
        if extract_addresses(header, n, daemon.namebuff, now, sets,
                             is_sign, check_rebind, no_cache, cache_secure,
                             &mut doctored) != 0 {
            // my_syslog(4,
            //           "possible DNS-rebind attack detected: %s" *const u8,
            // daemon.namebuff);
            munged = 1;
            cache_secure = 0
        }
        if doctored != 0 { cache_secure = 0 }
    }
    /* do this after extract_addresses. Ensure NODATA reply and remove
     nameserver info. */
    if munged != 0 {
        header.ancount = __bswap_16(0);
        header.nscount = __bswap_16(0);
        header.arcount = __bswap_16(0);
        header.hb3 =
            (header.hb3 & !(0x2)) as u8
    }
    /* the bogus-nxdomain stuff, doctor and NXDOMAIN->NODATA munging can all elide
     sections of the packet. Find the new length here and put back pseudoheader
     if it was removed. */
    return resize_packet(header, n, pheader, plen);
}
/* sets new last_server */

pub fn reply_query(mut fd: i32,
                   mut family: i32,
                   mut now: time::Instant) {
    /* packet from peer server, extract data for cache, and send to
     original requester */
    let mut header: DnsHeader = 0;
    let mut serveraddr = NetAdress::new();
    let mut forward: Frec = 0;
    let mut addrlen: socklen_t =
        ::std::mem::size_of::<NetAddress>();
    let mut n: susize =
        recvfrom(fd, daemon.packet,
                 daemon.packet_buff_sz, 0,
                 NetAddressArg {
                     __NetAddress__:
                     &mut serveraddr.sa,
                 },
                 &mut addrlen);
    let mut nn: usize = 0;
    let mut server: Server = 0;
    let mut hash: Vec<u8> = 0;
    /* packet buffer overwritten */
    daemon.srv_save = 0;
    /* Determine the address of the server replying  so that we can mark that as good */
    serveraddr.sa.sa_family = family;
    if serveraddr.sa.sa_family == 10 {
        serveraddr.in6.sin6_flowinfo = 0
    }
    header = daemon.packet;
    if n <
        ::std::mem::size_of::<DnsHeader>()
        ||
        header.hb3 & 0x80 == 0 {
        return;
    }
    /* spoof check: answer must come from known server, */
    server = daemon.servers;
    while !server.is_null() {
        if server.flags & (4 | 2) == 0 &&
            NetAddress_isequal(&mut server.addr, &mut serveraddr) != 0 {
            break;
        }
        server = server.next
    }
    if server.is_null() { return; }
    /* If sufficient time has elapsed, try and expand UDP buffer size again. */
    if difftime(now, server.pktsz_reduced) >
        60 {
        server.edns_pktsz = daemon.edns_pktsz
    }
    hash = hash_questions(header, n, daemon.namebuff);
    forward = lookup_frec(__bswap_16(header.id), fd, family, hash);
    if forward.is_null() { return; }
    dump_packet(if forward.flags & (8 | 16)
        != 0 {
        0x20
    } else { 0x8 }, header,
                n, &mut serveraddr, 0);
    /* log_query gets called indirectly all over the place, so 
     pass these in global variables - sorry. */
    daemon.log_display_id =
        forward.frec_src.log_id;
    daemon.log_source_addr = &mut forward.frec_src.source;
    if !daemon.ignore_addr.is_null() &&
        header.hb4 & 0xf ==
            0 &&
        check_for_ignored_address(header, n,
                                  daemon.ignore_addr) != 0 {
        return;
    }
    /* Note: if we send extra options in the EDNS0 header, we can't recreate
     the query from the reply. */
    if (header.hb4 & 0xf == 5
        ||
        header.hb4 & 0xf ==
            2) && forward.forwardall == 0
        && forward.flags & 512 == 0 {
        /* for broken servers, attempt to send to another one. */
        let mut pheader: Vec<u8> = 0;
        let mut plen: usize = 0;
        let mut is_sign: i32 = 0;
        /* In strict order mode, there must be a server later in the chain
	 left to send to, otherwise without the forwardall mechanism,
	 code further on will cycle around the list forwever if they
	 all return REFUSED. Note that server is always non-NULL before 
	 this executes. */
        if daemon.options
        [7] != 0
        {
            server = (*forward.sentto).next;
            while !server.is_null() {
                if server.flags &
                    (4 | 8 |
                        32 | 2 |
                        8192) == 0 {
                    break;
                }
                server = server.next
            }
        }
        /* recreate query from reply */
        pheader =
            find_pseudoheader(header, n, &mut plen,
                              0, &mut is_sign,
                              0);
        if is_sign == 0 && !server.is_null() {
            header.ancount = __bswap_16(0);
            header.nscount = __bswap_16(0);
            header.arcount = __bswap_16(0);
            nn = resize_packet(header, n, pheader, plen);
            if nn != 0 {
                header.hb3 =
                    (header.hb3 &
                        !(0x80 | 0x4 |
                            0x2)) as u8;
                header.hb4 =
                    (header.hb4 &
                        !(0x80 | 0xf |
                            0x10 | 0x20));
                if forward.flags & 2 != 0 {
                    header.hb4 =
                        (header.hb4 | 0x10)
                            as u8
                }
                if forward.flags & 32 != 0 {
                    header.hb4 =
                        (header.hb4 | 0x20)
                            as u8
                }
                if forward.flags & 64 != 0 {
                    add_do_bit(header, nn, pheader.offset(plen));
                }
                forward_query(-(1), 0,
                              0,
                              0, header, nn,
                              now, forward,
                              forward.flags & 32,
                              forward.flags & 64);
                return;
            }
        }
    }
    server = forward.sentto;
    if (*forward.sentto).flags & (8 | 32) ==
        0 {
        if header.hb4 & 0xf ==
            5 {
            server = 0
        } else {
            let mut last_server: Server = 0;
            /* find good server by address if possible, otherwise assume the last one we sent to */
            last_server = daemon.servers;
            while !last_server.is_null() {
                if last_server.flags &
                    (4 | 8 |
                        32 | 2) == 0 &&
                    NetAddress_isequal(&mut last_server.addr,
                                       &mut serveraddr) != 0 {
                    server = last_server;
                    break;
                } else { last_server = last_server.next }
            }
        }
        if daemon.options
        [23] == 0
        {
            daemon.last_server = server
        }
    }
    /* We tried resending to this server with a smaller maximum size and got an answer.
     Make that permanent. To avoid reduxing the packet size for a single dropped packet,
     only do this when we get a truncated answer, or one larger than the safe size. */
    if (*forward.sentto).edns_pktsz > 1280 &&
        forward.flags & 256 != 0 &&
        (header.hb3 & 0x2 != 0 ||
            n >= 1280) {
        (*forward.sentto).edns_pktsz = 1280;
        (*forward.sentto).pktsz_reduced = now;
        prettyprint_addr(&mut (*forward.sentto).addr,
                         daemon.addrbuff);
        // my_syslog(4,
        //           "reducing DNS packet size for nameserver %s to %d" *const u8,
        // daemon.addrbuff, 1280);
    }
    /* If the answer is an error, keep the forward record in place in case
     we get a good reply from another server. Kill it when we've
     had replies from all to avoid filling the forwarding table when
     everything is broken */
    if forward.forwardall == 0 ||
        {
            forward.forwardall -= 1;
            (forward.forwardall) == 1
        } ||
        header.hb4 & 0xf !=
            5 {
        let mut check_rebind: i32 = 0;
        let mut no_cache_dnssec: i32 = 0;
        let mut cache_secure: i32 = 0;
        let mut bogusanswer: i32 = 0;
        if daemon.options
        [31] != 0
        {
            check_rebind =
                (forward.flags & 1 == 0)
        }
        /* cancel */
        if header.hb4 & 0x10 != 0 ||
            forward.flags & 2 != 0 {
            no_cache_dnssec = 1
        }
        if forward.flags & 2 != 0 {
            header.hb4 =
                (header.hb4 | 0x10) as u8
        } else {
            header.hb4 =
                (header.hb4 & !(0x10))
        }
        if forward.flags & 2048 != 0 {
            no_cache_dnssec = 1
        }
        nn =
            process_reply(header, now, forward.sentto, n,
                          check_rebind, no_cache_dnssec, cache_secure,
                          bogusanswer, forward.flags & 32,
                          forward.flags & 64,
                          forward.flags & 128,
                          forward.flags & 4,
                          &mut forward.frec_src.source);
        if nn != 0 {
            let mut src: FrecSrc = 0;
            header.id = __bswap_16(forward.frec_src.orig_id);
            /*   Don't cache replies where DNSSEC validation was turned off, either
	   the upstream server told us so, or the original query specified it.  */
            /* restore CD bit to the value in the query */
            /* Never cache answers which are contingent on the source or MAC address EDSN0 option,
	 since the cache is ignorant of such things. */
            header.hb4 = (header.hb4 | 0x80); /* recursion if available */
            src = &mut forward.frec_src; /* default if no EDNS0 */
            while !src.is_null() {
                header.id = __bswap_16(src.orig_id);
                dump_packet(0x2,
                            daemon.packet, nn,
                            0, &mut src.source);
                send_from(src.fd,
                          (daemon.options[(13).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                          ).wrapping_mul(8
                          ))
                              ] &
                              (1) <<
                                  (13).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                  ).wrapping_mul(8))
                              != 0 ||
                              daemon.options
                [(39).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                      ))
                ] &
                    (1) <<
                    (39).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                      ).wrapping_mul(8
                                                  ))
                != 0),
                daemon.packet, nn, &mut src.source,
                &mut src.dest, src.iface);
                if daemon.options
                [(51).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                   ).wrapping_mul(8
                                                              ))
                ] &
                    (1) <<
                    (51).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                      ).wrapping_mul(8
                          ))
                != 0 &&
                    src != &mut forward.frec_src
                {
                    daemon.log_display_id =
                        src.log_id;
                    daemon.log_source_addr = &mut src.source;
                    // log_query((1) << 16,
                    //           "query"
                    //           , 0,
                    //           "duplicate");
                }
                src = src.next
            }
        }
        free_frec(forward);
    };
}

pub fn receive_query(mut listen: Listener,
                     mut now: time::Instant) {
    let mut header: DnsHeader =
        daemon.packet;
    let mut source_addr = NetAddress::new();
    let mut pheader: Vec<u8> = 0;
    let mut type_0: u16 = 0;
    let mut udp_size: u16 = 512;
    let mut dst_addr= NetAddress::new();
    let mut netmask= NetAddress::new();
    let mut dst_addr_4= NetAddress::new();
    let mut m: usize = 0;
    let mut n: susize = 0;
    let mut if_index: i32 = 0;
    let mut auth_dns: i32 = 0;
    let mut do_bit: i32 = 0;
    let mut have_pseudoheader: i32 = 0;
    let mut local_auth: i32 = 0;
    // let mut iov: [iovec; 1] =
    //     [iovec { iov_base: 0, iov_len: 0 }; 1];
    // let mut msg: MsgHdr =
    //     MsgHdr {
    //         msg_name: 0,
    //         msg_namelen: 0,
    //         msg_iov: 0,
    //         msg_iovlen: 0,
    //         msg_control: 0,
    //         msg_controllen: 0,
    //         msg_flags: 0,
    //     };
    // let mut cmptr: CmsgHdr = 0;
    let mut control_u = C2rustUnnamed16::new();
    let mut family: i32 = listen.addr.sa.sa_family;
    /* Can always get recvd interface for IPv6 */
    let mut check_dst: i32 =
        (daemon.options[(13).wrapping_div((::std::mem::size_of::<libc::c_uint>()
        ).wrapping_mul(8))
            ] &
            (1) <<
                (13).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                ).wrapping_mul(8))
            == 0 || family == 10);
    /* packet buffer overwritten */
    daemon.srv_save = 0;
    dst_addr.addr4.s_addr = 0;
    dst_addr_4.s_addr = dst_addr.addr4.s_addr;
    netmask.s_addr = 0;
    if daemon.options
    [(13).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                       ).wrapping_mul(8
                          ))
    ] &
        (1) <<
        (13).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8

    ))
    != 0 && !listen.iface.is_null()
    {
        auth_dns = (*listen.iface).dns_auth;
        if family == 2 {
            dst_addr.addr4 = (*listen.iface).addr.in_0.sin_addr;
            dst_addr_4 = dst_addr.addr4;
            netmask = (*listen.iface).netmask
        }
    }
    iov[0].iov_base =
        daemon.packet;
    iov[0].iov_len =
        daemon.edns_pktsz;
    msg.msg_control = control_u.control;
    msg.msg_controllen =
        ::std::mem::size_of::<C2rustUnnamed16>();
    msg.msg_flags = 0;
    msg.msg_name = &mut source_addr;
    msg.msg_namelen =
        ::std::mem::size_of::<NetAddress>();
    msg.msg_iov = iov;
    msg.msg_iovlen = 1;
    n = recvmsg(listen.fd, &mut msg, 0);
    if n == -(1) { return; }
    if n <
        ::std::mem::size_of::<DnsHeader>()
        ||
        msg.msg_flags & MSG_TRUNC != 0 ||
        header.hb3 & 0x80 != 0 {
        return;
    }
    /* Clear buffer beyond request to avoid risk of
     information disclosure. */
    memset(daemon.packet.offset(n),
           0,
           (daemon.edns_pktsz - n));
    source_addr.sa.sa_family = family;
    if family == 2 {
        /* Source-port == 0 is an error, we can't send back to that. 
	  http://www.ietf.org/mail-archive/web/dnsop/current/msg11441.html */
        if source_addr.in_0.sin_port == 0 {
            return;
        }
    } else {
        /* Source-port == 0 is an error, we can't send back to that. */
        if source_addr.in6.sin6_port == 0 {
            return;
        }
        source_addr.in6.sin6_flowinfo = 0
    }
    /* We can be configured to only accept queries from at-most-one-hop-away addresses. */
    if daemon.options
    [(49).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                       ).wrapping_mul(8
                          ))
    ] &
        (1) <<
        (49).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8

    ))
    != 0
    {
        let mut addr: AddressListEntry = 0;
        if family == 10 {
            addr = daemon.interface_addrs;
            while !addr.is_null() {
                if addr.flags & 2 != 0 &&
                    is_same_net6(&mut addr.addr.addr6,
                                 &mut source_addr.in6.sin6_addr,
                                 addr.prefixlen) != 0 {
                    break;
                }
                addr = addr.next
            }
        } else {
            let mut netmask_0 = NetAddress::new();
            addr = daemon.interface_addrs;
            while !addr.is_null() {
                netmask_0.s_addr =
                    __bswap_32((!(0)) <<
                        32 - addr.prefixlen);
                if addr.flags & 2 == 0 &&
                    is_same_net4(addr.addr.addr4,
                                 source_addr.in_0.sin_addr, netmask_0) != 0
                {
                    break;
                }
                addr = addr.next
            }
        }
        if addr.is_null() {
            let mut warned: bool = false;
            if warned == false {
                // my_syslog(4,
                //           "Ignoring query from non-local network" *const u8);
                warned = true;
            }
            return;
        }
    }
    if check_dst != 0 {
        let mut ifr = IfReq::new();
        // if msg.msg_controllen < ::std::mem::size_of::<CmsgHdr>() {
        //     return;
        // }
        if family == 2 {
            // cmptr =
            //     if msg.msg_controllen >=
            //         ::std::mem::size_of::<CmsgHdr>() {
            //         msg.msg_control
            //     } else { 0 };
            // while !cmptr.is_null() {
            //     if cmptr.cmsg_level == IPPROTO_IP &&
            //         cmptr.cmsg_type == 8 {
            //         let mut p: C2rustUnnamed15 =
            //             C2rustUnnamed15 { c: 0 };
            //         p.c = cmptr.__cmsg_data;
            //         dst_addr.addr4 = (*p.p).ipi_spec_dst;
            //         dst_addr_4 = dst_addr.addr4;
            //         if_index = (*p.p).ipi_ifindex
            //     }
            //     cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            // }
        }
        if family == 10 {
            // cmptr =
            //     if msg.msg_controllen >=
            //         ::std::mem::size_of::<CmsgHdr>() {
            //         msg.msg_control
            //     } else { 0 };
            // while !cmptr.is_null() {
            //     if cmptr.cmsg_level == IPPROTO_IPV6 &&
            //         cmptr.cmsg_type == daemon.v6pktinfo {
            //         let mut p_0: C2rustUnnamed14 =
            //             C2rustUnnamed14 { c: 0 };
            //         p_0.c = cmptr.__cmsg_data;
            //         dst_addr.addr6 = (*p_0.p).ipi6_addr;
            //         if_index = (*p_0.p).ipi6_ifindex
            //     }
            //     cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            // }
        }
        /* enforce available interface configuration */
        if indextoname(listen.fd, if_index, ifr.ifr_ifrn.ifrn_name) == 0 {
            return;
        }
        if iface_check(family, &mut dst_addr,
                       ifr.ifr_ifrn.ifrn_name, &mut auth_dns) ==
            0 {
            if daemon.options
            [(39).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                               ).wrapping_mul(8
                                                  ))
            ] &
                (1) <<
                (39).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                  ).wrapping_mul(8
              ))
            == 0
            {
                enumerate_interfaces(0);
            }
            if loopback_exception(listen.fd, family, &mut dst_addr,
                                  ifr.ifr_ifrn.ifrn_name) == 0 &&
                label_exception(if_index, family, &mut dst_addr) == 0 {
                return;
            }
        }
        if family == 2 &&
            daemon.options
        [(18).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                           ).wrapping_mul(8
                                              ))
        ] &
            (1) <<
            (18).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                              ).wrapping_mul(8
          ))
        != 0
        {
            let mut iface: Irec = 0;
            /* get the netmask of the interface which has the address we were sent to.
	     This is no necessarily the interface we arrived on. */
            iface = daemon.interfaces;
            while !iface.is_null() {
                if iface.addr.sa.sa_family ==
                    2 &&
                    iface.addr.in_0.sin_addr.s_addr == dst_addr_4.s_addr
                {
                    break;
                }
                iface = iface.next
            }
            /* interface may be new */
            if iface.is_null() &&
                daemon.options
            [(39).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                               ).wrapping_mul(8
                                                          ))
            ] &
            (1) <<
            (39).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                              ).wrapping_mul(8
                      ))
            == 0
            {
                enumerate_interfaces(0);
            }
            iface = daemon.interfaces;
            while !iface.is_null() {
                if iface.addr.sa.sa_family ==
                    2 &&
                    iface.addr.in_0.sin_addr.s_addr == dst_addr_4.s_addr
                {
                    break;
                }
                iface = iface.next
            }
            /* If we failed, abandon localisation */
            if !iface.is_null() {
                netmask = iface.netmask
            } else { dst_addr_4.s_addr = 0 }
        }
    }
    /* log_query gets called indirectly all over the place, so 
     pass these in global variables - sorry. */
    daemon.log_id += 1;
    daemon.log_display_id = daemon.log_id;
    daemon.log_source_addr = &mut source_addr;
    dump_packet(0x1,
                daemon.packet, n,
                &mut source_addr, 0);
    if extract_request(header, n, daemon.namebuff,
                       &mut type_0) != 0 {
        let mut zone: AuthZone = 0;
        let mut types: &mut String =
            querystr(if auth_dns != 0 {
                "auth"
            } else {
                "query"
            }, type_0);
        if family == 2 {
            // log_query((1) << 19 |
            //               (1) << 7 |
            //               (1) << 3,
            //           daemon.namebuff,
            //           &mut source_addr.in_0.sin_addr, types);
        } else {
            // log_query((1) << 19 |
            //               (1) << 8 |
            //               (1) << 3,
            //           daemon.namebuff,
            //           &mut source_addr.in6.sin6_addr, types);
        }
        /* find queries for zones we're authoritative for, and answer them directly */
        if auth_dns == 0 &&
            daemon.options
        [(18).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                           ).wrapping_mul(8
                                              ))
        ] &
            (1) <<
            (18).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                              ).wrapping_mul(8
          ))
        == 0
        {
            zone = daemon.auth_zones;
            while !zone.is_null() {
                if in_zone(zone, daemon.namebuff,
                           0) != 0 {
                    auth_dns = 1;
                    local_auth = 1;
                    break;
                } else { zone = zone.next }
            }
        }
        /* Check for forwarding loop */
        if detect_loop(daemon.namebuff, type_0) != 0
        {
            return;
        }
    }
    if !find_pseudoheader(header, n, 0, &mut pheader,
                          0,
                          0).is_null() {
        let mut flags: u16 = 0;
        have_pseudoheader = 1;
        let mut t_cp: Vec<u8> = pheader;
        udp_size =
            ((*t_cp.offset(0))
                << 8 |
                *t_cp.offset(1));
        pheader = pheader.offset(2);
        /* Sanity check - can't reduce below default. RFC 6891 6.2.3 */
        pheader = pheader.offset(2); /* ext_rcode */
        let mut t_cp_0: Vec<u8> = pheader; /* do bit */
        flags =
            ((*t_cp_0.offset(0)) << 8 |
                *t_cp_0.offset(1));
        pheader = pheader.offset(2);
        if flags & 0x8000 != 0 {
            do_bit = 1
        }
        if udp_size >
            daemon.edns_pktsz {
            udp_size = daemon.edns_pktsz
        } else if (udp_size) < 512 {
            udp_size = 512
        }
    }
    if auth_dns != 0 {
        m =
            answer_auth(header,
                        (header).offset(udp_size
        ),
        n, now, &mut source_addr, local_auth,
        do_bit, have_pseudoheader);
        if m >= 1 {
            send_from(listen.fd,
                      (daemon.options[13] != 0 || daemon.options[39] != 0),
            header, m, &mut source_addr,
            &mut dst_addr, if_index);
            daemon.metrics[METRIC_DNS_AUTH_ANSWERED
                ] =
                daemon.metrics[METRIC_DNS_AUTH_ANSWERED].wrapping_add(1)
        }
    } else {
        let mut ad_reqd: i32 = do_bit;
        /* If the client provides an EDNS0 UDP size, use that to limit our reply.
	 (bounded by the maximum configured). If no EDNS0, then it
	 defaults to 512 */
        /* RFC 6840 5.7 */
        if header.hb4 & 0x20 != 0 {
            ad_reqd = 1
        }
        m =
            answer_request(header,
                           (header).offset(udp_size

        ),
        n, dst_addr_4, netmask, now, ad_reqd,
        do_bit, have_pseudoheader);
        if m >= 1 {
            send_from(listen.fd,
                      (daemon.options
            [13] != 0 || daemon.options[39] != 0),
            header, m, &mut source_addr,
            &mut dst_addr, if_index);
            daemon.metrics[METRIC_DNS_LOCAL_ANSWERED] = daemon.metrics[METRIC_DNS_LOCAL_ANSWERED].wrapping_add(1)
        } else if forward_query(listen.fd, &mut source_addr, &mut dst_addr,
                                if_index, header, n,
                                now, 0, ad_reqd, do_bit) != 0 {
            daemon.metrics[METRIC_DNS_QUERIES_FORWARDED] = daemon.metricss           [METRIC_DNS_QUERIES_FORWARDED

            ].wrapping_add(1)
        } else {
            daemon.metrics[METRIC_DNS_LOCAL_ANSWERED
                ] =
                daemon.metrics
            [METRIC_DNS_LOCAL_ANSWERED

            ].wrapping_add(1)
        }
    };
}
/* The daemon forks before calling this: it should deal with one connection,
   blocking as necessary, and then return. Note, need to be a bit careful
   about resources for debug mode, when the fork is suppressed: that's
   done by the caller. */

pub fn tcp_request(mut confd: i32, mut now: time::Instant,
                   mut local_addr: NetAddress,
                   mut netmask: NetAddress,
                   mut auth_dns: i32)
                   ->  Vec<u8> {
let mut size: usize = 0;
let mut norebind: i32 = 0;
let mut local_auth: i32 = 0;
let mut checking_disabled: i32 = 0;
let mut do_bit: i32 = 0;
let mut added_pheader: i32 = 0;
let mut have_pseudoheader: i32 = 0;
let mut check_subnet: i32 = 0;
let mut cacheable: i32 = 0;
let mut no_cache_dnssec: i32 = 0;
let mut cache_secure: i32 = 0;
let mut bogusanswer: i32 = 0;
let mut m: usize = 0;
let mut qtype: u16 = 0;
let mut gotname: u32 = 0;
let mut c1: u8 = 0;
let mut c2: u8 = 0;
/* Max TCP packet + slop + size */
let mut packet: Vec<u8> = Vec::new();
// whine_malloc(((65536 + 1025 +
// 10)).wrapping_add(::std::mem::size_of::< u16 > ()
// ))
// ;
let mut payload: Vec<u8> = packet.offset(2);
/* largest field in header is 16-bits, so this is still sufficiently aligned */
let mut header: DnsHeader = payload;
let mut lengthf: u16 = packet;
let mut last_server: Server = 0;
let mut dst_addr_4 = NetAddress::new();
let mut peer_addr = NetAddress::new();
let mut peer_len = ::std::mem::size_of::< NetAddress > ();
let mut query_count: i32 = 0;
let mut pheader: Vec<u8> = 0;
let mut mark: u32 = 0;
let mut have_mark: i32 = 0;
if getpeername(confd, NetAddressArg::new(), & mut peer_len) == - (1) {
return packet
}
/* We can be configured to only accept queries from at-most-one-hop-away addresses. */
if daemon.options[(49).wrapping_div((::std::mem::size_of::<libc::c_uint > ()
).wrapping_mul(8                                                   ))
] &
(1) <<
(49).wrapping_rem((::std::mem::size_of::< libc::c_uint > ()).wrapping_mul(8

))
!= 0 {
let mut addr: * mut AddressListEntry = 0;
if peer_addr.sa.sa_family == 10 {
addr = daemon.interface_addrs;
while ! addr.is_null() {
if addr.flags & 2 != 0 && is_same_net6( &mut addr.addr.addr6, &mut peer_addr.in6.sin6_addr, addr.prefixlen) != 0 {
break;
}
addr = addr.next
}
} else {
let mut netmask_0 = NetAddress::new();
addr = daemon.interface_addrs;
while ! addr.is_null() {
netmask_0.s_addr =
__bswap_32(( ! (0)) <<
32 - addr.prefixlen);
if addr.flags & 2 == 0 &&
is_same_net(addr.addr.addr4,
peer_addr.in_0.sin_addr, netmask_0) != 0 {
break;
}
addr = addr.next
}
}
if addr.is_null() {
// my_syslog(4,
// "Ignoring query from non-local network" * const u8);
return packet
}
}
loop  {
if query_count == 100 || packet.is_null() ||
read_write(confd, & mut c1, 1, 1) == 0 ||
read_write(confd, & mut c2, 1, 1) == 0 ||
{
size =
((c1) << 8 |
c2);
(size) == 0
} ||
read_write(confd, payload, size,
1) == 0 {
return packet
}
if size <
::std::mem::size_of::< DnsHeader > ()              {
continue;
}
/* Clear buffer beyond request to avoid risk of
information disclosure. */
query_count += 1;
/* log_query gets called indirectly all over the place, so
pass these in global variables - sorry. */
daemon.log_id += 1;
daemon.log_display_id = daemon.log_id;
daemon.log_source_addr = & mut peer_addr;
/* save state of "cd" flag in query */
checking_disabled =
( *header).hb4 & 0x10;
if checking_disabled != 0 { no_cache_dnssec = 1 }
gotname =
extract_request(header, size,
daemon.namebuff, & mut qtype);
if gotname != 0 {
let mut zone: * mut AuthZone = 0;
let mut types: & mut String =
querystr(if auth_dns != 0 {
"auth"
} else {
"query"
} , qtype);
if peer_addr.sa.sa_family == 2 {
// log_query((1) << 19 |
// (1) << 7 |
// (1) << 3,
// daemon.namebuff,
// & mut peer_addr.in_0.sin_addr, types);
// } else {
// log_query((1) << 19 |
// (1) << 8 |
// (1) << 3,
// daemon.namebuff,
// & mut peer_addr.in6.sin6_addr, types);
}
/* find queries for zones we're authoritative for, and answer them directly */
if auth_dns == 0 &&
daemon.options[18] == 0 {
zone = daemon.auth_zones;
while ! zone.is_null() {
if in_zone(zone, daemon.namebuff,
0 ) != 0 {
auth_dns = 1;
local_auth = 1;
break;
} else { zone = zone.next }
}
}
}
if ( * local_addr).sa.sa_family == 2 {
dst_addr_4 = ( * local_addr).in_0.sin_addr
} else { dst_addr_4.s_addr = 0 }
do_bit = 0;
if ! find_pseudoheader(header, size, 0, & mut pheader,
0,
0).is_null() {
let mut flags: u16 = 0;
have_pseudoheader = 1;
/* do bit */
pheader =
pheader.offset(4                             ); /* udp_size, ext_rcode */
let mut t_cp: Vec<u8> = pheader;
flags =
(( * t_cp.offset(0) ) << 8 |
* t_cp.offset(1) );
pheader = pheader.offset(2);
if flags & 0x8000 != 0 {
do_bit = 1
}
}
if auth_dns != 0 {
m =
answer_auth(header,
(header).offset(65536), size,
now, & mut peer_addr, local_auth, do_bit,
have_pseudoheader)
} else {
let mut ad_reqd: i32 = do_bit;
/* RFC 6840 5.7 */
if (* header).hb4 & 0x20 != 0 {
ad_reqd = 1
}
/* m > 0 if answered from cache */
m =
answer_request(header,
(header).offset(65536),
size, dst_addr_4, netmask, now, ad_reqd,
do_bit, have_pseudoheader);
/* Do this by steam now we're not in the select() loop */
check_log_writer(1);
if m == 0 {
let mut flags_0: u32 =
0;
let mut addrp: & mut NetAddress = 0;
let mut type_0: i32 = 16384;
let mut domain: & mut String = 0;
let mut oph: Vec<u8> =
find_pseudoheader(header, size, 0,
0,
0,
0);
size =
add_edns0_config(header, size,
(header).offset(65536),
peer_addr, now, & mut check_subnet,
& mut cacheable);
if gotname != 0 {
flags_0 =
search_servers(now, & mut addrp, gotname,
daemon.namebuff,
& mut type_0, & mut domain,
& mut norebind)
}
/* Check if we added a pheader on forwarding - may need to
strip it from the reply. */
if oph.is_null() & &
! find_pseudoheader(header, size, 0,
0,
0,
0).is_null() {
added_pheader = 1
}
type_0 &= ! (16384);
if type_0 != 0 ||
daemon.options[7] != 0 || daemon.last_server.is_null() {
last_server = daemon.servers
} else { last_server = daemon.last_server }
if flags_0 == 0 & & ! last_server.is_null() {
let mut firstsendto: Server = 0;
let mut hash: [u8; 32] = [0; 32];
hash = hash_questions(header, size, daemon.namebuff);
let mut current_block_93: u64;
's_446:
loop
/* Loop round available servers until we succeed in connecting to one.
Note that this code subtly ensures that consecutive queries on this connection
which can go to the same server, do so. */
{
let mut data_sent: i32 = 0;
if firstsendto.is_null() {
firstsendto = last_server
} else {
last_server = last_server.next;
if last_server.is_null() {
last_server = daemon.servers
}
if last_server == firstsendto { break; }
}
/* server for wrong domain */
if type_0 !=
last_server.flags &
(8 | 32)
||
type_0 == 8 && hostname_isequal(domain, last_server.domain) == 0 || last_server.flags & (4 | 8192) != 0 {
    continue;
}
loop  {
* length = __bswap_16(size);
if (* last_server).tcpfd == - (1)
{
last_server.tcpfd =
socket((* last_server).addr.sa.sa_family
,
SOCK_STREAM,
0);
if last_server.tcpfd ==
- (1) {
continue 's_446;
}
if local_bind(last_server.tcpfd,
& mut last_server.source_addr, last_server.interface, 0, 1) == 0 {
close(last_server.tcpfd);
last_server.tcpfd =
- (1);
continue 's_446;
} else {
while retry_send(sendto(last_server.tcpfd, packet, size.wrapping_add(2), MSG_FASTOPEN, ConstNetAddressArg::new(), sa_len( &mut last_server.addr)) != 0 {}
if * __errno_location() == 0 {
    data_sent = 1
}
if (data_sent == 0) && connect(last_server.tcpfd, ConstNetAddressArg::new(), sa_len(last_server.addr)) == 1 {
close(last_server.tcpfd);
last_server.tcpfd =
- (1);
continue 's_446;
} else {
last_server.flags &=
! (32768)
}
}
}
/* get query name again for logging - may have been overwritten */
gotname =
extract_request(header,
size     usize,
daemon.namebuff,
& mut qtype);
if gotname == 0 {
strcpy(daemon.namebuff,
"query" );
}
if data_sent == 0 & &
read_write(( *last_server).tcpfd,
packet,
size.wrapping_add(::std::mem::size_of::< u16> ()
)
,
0) == 0 ||
read_write(last_server.tcpfd,
& mut c1, 1,
1) == 0 | |
read_write(last_server.tcpfd,
& mut c2, 1,
1) == 0 | |
read_write(last_server.tcpfd,
payload,
(c1) <<
8 |
c2,
1) == 0 {
close(last_server.tcpfd);
last_server.tcpfd =
- (1);
/* We get data then EOF, reopen connection to same server,
else try next. This avoids DoS from a server which accepts
connections and then closes them. */
if ! (last_server.flags &
32768 != 0) {
continue 's_446;
}
} else {
last_server.flags |=
32768;
m =
((c1) <<
8 |
c2);
if last_server.addr.sa.sa_family                                      == 2 {
// log_query((1) <<
// 18 |
// (1) <<
// 7 |
// (1) <<
// 3,
// daemon.namebuff,
// & mut (* last_server).addr.in_0.sin_addr
// NetAddress,
// 0 );
} else {
// log_query((1) <<
// 18 |
// (1) <<
// 8 |
// (1) <<
// 3,
// daemon.namebuff,
// & mut (* last_server).addr.in6.sin6_addr
// NetAddress,
// 0 );
}
/* restore CD bit to the value in the query */
if checking_disabled != 0 {
( * header).hb4 =
(( *header).hb4 |
0x10) as u8
} else {
( * header).hb4 =
(( *header).hb4 &
! (0x10))                                          u8
}
/* There's no point in updating the cache, since this process will exit and
lose the information after a few queries. We make this call for the alias and
bogus-nxdomain side-effects. */
/* If the crc of the question section doesn't match the crc we sent, then
someone might be attempting to insert bogus values into the cache by
sending replies containing questions and bogus answers. */
if memcmp(hash
hash_questions(header,
m
,
daemon.namebuff)
,
32 ) !=
0 {
current_block_93 =
9430418855388998878;
break;
} else {
current_block_93 =
7079180960716815705;
break;
}
}
}
match current_block_93 {
9430418855388998878 => {
m = 0;
break;
}
_ => {
/* Never cache answers which are contingent on the source or MAC address EDSN0 option,
since the cache is ignorant of such things. */
if cacheable == 0 {
no_cache_dnssec = 1
}
m =
process_reply(header, now,
last_server,
m       usize,
(daemon.options[(31).wrapping_div((::std::mem::size_of::< libc::c_uint > ()                                                      ).wrapping_mul(8                                                                                                                                                                                                                                                   ))
usize]
&
(1)
<<
(31
).wrapping_rem((::std::mem::size_of::< libc::c_uint > ()                  ).wrapping_mul(8                                                                                                                                                                           ))
!= 0 & &
norebind == 0)       ,
no_cache_dnssec,
cache_secure,
bogusanswer, ad_reqd,
do_bit, added_pheader,
check_subnet,
& mut peer_addr);
break;
}
}
}
}
/* In case of local answer or no connections made. */
if m == 0 {
m =
setup_reply(header, size,
addrp, flags_0,
daemon.local_ttl);
if have_pseudoheader != 0 {
m =
add_pseudoheader(header, m,
(header                                            Vec<u8> ).offset(65536

),
daemon.edns_pktsz,
0,
0,
0,
do_bit, 0)
}
}
}
}
check_log_writer(1);
* length = __bswap_16(m);
if m == 0 | |
read_write(confd, packet,
m.wrapping_add(::std::mem::size_of::< u16 > () ),
0) == 0 {
return packet
}
};
}
fn allocate_frec(mut now: time::Instant) -> Frec {
    let mut f: Frec = 0;
    // f =
    //     whine_malloc(::std::mem::size_of::<Frec>()) * mut Frec;
    if !f.is_null() {
        f.next = daemon.frec_list;
        f.time = now;
        f.sentto = 0;
        f.rfd4 = 0;
        f.flags = 0;
        f.rfd6 = 0;
        daemon.frec_list = f
    }
    return f;
}

pub fn allocate_rfd(mut family: i32)
                    -> RandFd {
    static mut finger: i32 = 0;
    let mut i: i32 = 0;
    /* limit the number of sockets we have open to avoid starvation of 
     (eg) TFTP. Once we have a reasonable number, randomness should be OK */
    i = 0;
    while i < 64 {
        if daemon.randomsocks[i].refcount
            == 0 {
            daemon.randomsocks[i].fd =
                random_sock(family);
            if daemon.randomsocks[i].fd ==
                -(1) {
                break;
            }
            daemon.randomsocks[i].refcount =
                1;
            daemon.randomsocks[i].family =
                family;
            return &mut *daemon.randomsocks.offset(i
            );

        } else { i += 1 }
    }
    /* No free ones or cannot get new socket, grab an existing one */
    i = 0;
    while i < 64 {
        let mut j: i32 = (i + finger) % 64;
        if daemon.randomsocks[j].refcount
            != 0 &&
            daemon.randomsocks[j].family
                == family &&
            daemon.randomsocks[j].refcount
         != 0xffff
        {
            finger = j;
            daemon.randomsocks[j].refcount =
                daemon.randomsocks
            [j
            usize].refcount.wrapping_add(1);
            return &mut *daemon.randomsocks.offset(j
            );

        }
        i += 1
    }
    return 0;
    /* doom */
}

pub fn free_rfd(mut rfd: &mut RandFd) {
    if !rfd.is_null() &&
        {
            rfd.refcount = rfd.refcount.wrapping_sub(1);
            (rfd.refcount) == 0
        } {
        close(rfd.fd);
    };
}

fn free_frec(mut f: &mut Frec) {
    let mut last: FrecSrc = 0;
    /* add back to freelist if not the record builtin to every frec. */
    last = f.frec_src.next;
    while !last.is_null() && !last.next.is_null() { last = last.next }
    if !last.is_null() {
        last.next = daemon.free_frec_src;
        daemon.free_frec_src = f.frec_src.next
    }
    f.frec_src.next = 0;
    free_rfd(f.rfd4);
    f.rfd4 = 0;
    f.sentto = 0;
    f.flags = 0;
    free_rfd(f.rfd6);
    f.rfd6 = 0;
}
/* if wait==NULL return a free or older than TIMEOUT record.
   else return *wait zero if one available, or *wait is delay to
   when the oldest in-use record will expire. Impose an absolute
   limit of 4*TIMEOUT before we wipe things (for random sockets).
   If force is non-NULL, always return a result, even if we have
   to allocate above the limit, and never free the record pointed
   to by the force argument. */

pub fn get_new_frec(mut now: time::Instant,
                    mut wait: ,
                    mut force: &mut Frec) -> Frec {
    let mut f: Frec = 0;
    let mut oldest: Frec = 0;
    let mut target: Frec = 0;
    let mut count: i32 = 0;
    if !wait.is_null() { *wait = 0 }
    f = daemon.frec_list;
    oldest = 0;
    target = 0;
    count = 0;
    while !f.is_null() {
        if f.sentto.is_null() {
            target = f
        } else {
            if difftime(now, f.time) >=
                (4 * 10) {
                free_frec(f);
                target = f
            }
            if oldest.is_null() ||
                difftime(f.time, oldest.time) <=
                    0 {
                oldest = f
            }
        }
        f = f.next;
        count += 1
    }
    if !target.is_null() {
        target.time = now;
        return target; }
    /* can't find empty one, use oldest if there is one
     and it's older than timeout */
    if force.is_null() && !oldest.is_null() &&
        difftime(now, oldest.time) >= 10 {
        /* keep stuff for twice timeout if we can by allocating a new
	 record instead */
        if difftime(now, oldest.time) <
            (2 * 10) &&
            count <= daemon.ftabsize &&
            {
                f = allocate_frec(now);
                !f.is_null()
            } {
            return f;
        }
        if wait.is_null() {
            free_frec(oldest);
            oldest.time = now
        }
        return oldest;
    }
    /* none available, calculate time 'till oldest record expires */
    if force.is_null() && count > daemon.ftabsize {
        static mut last_log: time::Instant = 0;
        if !oldest.is_null() && !wait.is_null() {
            *wait =
                (oldest.time + 10 - now)
        }
        if difftime(now, last_log) > 5 {
            last_log = now;
            my_syslog(4,
                      "Maximum number of concurrent DNS queries reached (max: %d)"
                      ,
                      daemon.ftabsize);
        }
        return 0;
    }
    f = allocate_frec(now);
    if f.is_null() && !wait.is_null() {
        /* wait one second on malloc failure */
        *wait = 1
    }
    return f;
    /* OK if malloc fails and this is NULL */
}
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
fn lookup_frec(mut id: u16, mut fd: i32,
               mut family: i32,
               mut hash: Vec<u8>) -> Frec {
    let mut f: Frec = 0;
    f = daemon.frec_list;
    while !f.is_null() {
        if !f.sentto.is_null() &&
            f.new_id == id &&
            memcmp(hash, f.hash,
                   32) == 0
        {
            /* sent from random port */
            if family == 2 && !f.rfd4.is_null() &&
                (*f.rfd4).fd == fd {
                return f;
            }
            if family == 10 && !f.rfd6.is_null() &&
                (*f.rfd6).fd == fd {
                return f;
            }
            /* sent to upstream from bound socket. */
            if !(*f.sentto).sfd.is_null() && (*(*f.sentto).sfd).fd == fd
            {
                return f;
            }
        }
        f = f.next
    }
    return 0;
}

fn lookup_frec_by_sender(mut id: u16,
                         mut addr: NetAddress,
                         mut hash: Vec<u8>)
                         -> Frec {
    let mut f: Frec = 0;
    let mut src: FrecSrc = 0;
    f = daemon.frec_list;
    while !f.is_null() {
        if !f.sentto.is_null() &&
            f.flags & (8 | 16) == 0 &&
            memcmp(hash, f.hash,
                   32) == 0
        {
            src = &mut f.frec_src;
            while !src.is_null() {
                if src.orig_id == id &&
                    NetAddress_isequal(&mut src.source, addr) != 0 {
                    return f;
                }
                src = src.next
            }
        }
        f = f.next
    }
    return 0;
}

fn lookup_frec_by_query(mut hash: Vec<u8>,
                        mut flags: u32)
                        -> Frec {
    let mut f: Frec = 0;
    /* FREC_DNSKEY and FREC_DS_QUERY are never set in flags, so the test below 
     ensures that no frec created for internal DNSSEC query can be returned here.
     
     Similarly FREC_NO_CACHE is never set in flags, so a query which is
     contigent on a particular source address EDNS0 option will never be matched. */
    f = daemon.frec_list;
    while !f.is_null() {
        if !f.sentto.is_null() &&
            (f.flags &
                (2 | 32 | 64
                    | 1024 | 8 |
                    16 | 2048))
        libc::c_uint == flags &&
            memcmp(hash, f.hash,
                   32) == 0
        {
            return f;
        }
        f = f.next
    }
    return 0;
}
/* Send query packet again, if we can. */

pub fn resend_query() {
    if !daemon.srv_save.is_null() {
        let mut fd: i32 = 0;
        if !(*daemon.srv_save).sfd.is_null() {
            fd = (*(*daemon.srv_save).sfd).fd
        } else if !daemon.rfd_save.is_null() &&
            (*daemon.rfd_save).refcount !=
                0 {
            fd = (*daemon.rfd_save).fd
        } else { return; }
        while retry_send(sendto(fd,
                                daemon.packet
        daemon.packet_len,
        0,
        ConstNetAddressArg {
            __NetAddress__:
            &mut (*daemon.srv_save).addr.sa,
        },
        sa_len(&mut (*daemon.srv_save).addr)
        )) != 0
        {}
    };
}
/* A server record is going away, remove references to it */

pub fn server_gone(mut server: Server) {
    let mut f: Frec = 0;
    f = daemon.frec_list;
    while !f.is_null() {
        if !f.sentto.is_null() && f.sentto == server { free_frec(f); }
        f = f.next
    }
    if daemon.last_server == server {
        daemon.last_server = 0
    }
    if daemon.srv_save == server {
        daemon.srv_save = 0
    };
}
/* return unique random ids. */
fn get_id() -> u16 {
    let mut ret: u16 = 0;
    let mut f: Frec = 0;
    loop {
        ret = rand16();
        /* ensure id is unique. */
        f = daemon.frec_list;
        while !f.is_null() {
            if !f.sentto.is_null() &&
                f.new_id == ret {
                break;
            }
            f = f.next
        }
        if f.is_null() { return ret; }
    };
}
