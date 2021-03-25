
#[no_mangle]
pub unsafe extern "C" fn tftp_request(mut listen: *mut listener,
                                      mut now: time_t) {
    let mut len: ssize_t = 0;
    let mut packet: *mut libc::c_char = (*dnsmasq_daemon).packet;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mode: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addr: mysockaddr =
        mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut peer: mysockaddr =
        mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
    let mut msg: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut iov: iovec = iovec{iov_base: 0 as *mut libc::c_void, iov_len: 0,};
    let mut ifr: ifreq =
        ifreq{ifr_ifrn: C2RustUnnamed_2{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_1{ifru_addr:
                                      sockaddr{sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut is_err: libc::c_int = 1 as libc::c_int;
    let mut if_index: libc::c_int = 0 as libc::c_int;
    let mut mtu: libc::c_int = 0 as libc::c_int;
    let mut tmp: *mut iname = 0 as *mut iname;
    let mut transfer: *mut tftp_transfer = 0 as *mut tftp_transfer;
    let mut up: *mut *mut tftp_transfer = 0 as *mut *mut tftp_transfer;
    let mut port: libc::c_int = (*dnsmasq_daemon).start_tftp_port;
    let mut mtuflag: libc::c_int = 0 as libc::c_int;
    let mut namebuff: [libc::c_char; 16] = [0; 16];
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prefix: *mut libc::c_char = (*dnsmasq_daemon).tftp_prefix;
    let mut pref: *mut tftp_prefix = 0 as *mut tftp_prefix;
    let mut addra: all_addr = all_addr{addr4: in_addr{s_addr: 0,},};
    let mut family: libc::c_int = (*listen).addr.sa.sa_family as libc::c_int;
    /* Can always get recvd interface for IPv6 */
    let mut check_dest: libc::c_int =
        ((*dnsmasq_daemon).options[(13 as libc::c_int as
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
             == 0 || family == 10 as libc::c_int) as libc::c_int;
    let mut control_u: C2RustUnnamed_14 =
        C2RustUnnamed_14{align:
                             cmsghdr{cmsg_len: 0,
                                     cmsg_level: 0,
                                     cmsg_type: 0,
                                     __cmsg_data: [],},};
    msg.msg_controllen =
        ::std::mem::size_of::<C2RustUnnamed_14>() as libc::c_ulong;
    msg.msg_control = control_u.control.as_mut_ptr() as *mut libc::c_void;
    msg.msg_flags = 0 as libc::c_int;
    msg.msg_name = &mut peer as *mut mysockaddr as *mut libc::c_void;
    msg.msg_namelen =
        ::std::mem::size_of::<mysockaddr>() as libc::c_ulong as socklen_t;
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1 as libc::c_int as size_t;
    iov.iov_base = packet as *mut libc::c_void;
    iov.iov_len = (*dnsmasq_daemon).packet_buff_sz as size_t;
    /* we overwrote the buffer... */
    (*dnsmasq_daemon).srv_save = 0 as *mut server;
    len = recvmsg((*listen).tftpfd, &mut msg, 0 as libc::c_int);
    if len < 2 as libc::c_int as libc::c_long { return }
    /* Can always get recvd interface for IPv6 */
    if check_dest == 0 {
        if !(*listen).iface.is_null() {
            addr = (*(*listen).iface).addr;
            name = (*(*listen).iface).name;
            mtu = (*(*listen).iface).mtu;
            if (*dnsmasq_daemon).tftp_mtu != 0 as libc::c_int &&
                   (*dnsmasq_daemon).tftp_mtu < mtu {
                mtu = (*dnsmasq_daemon).tftp_mtu
            }
        } else {
            /* we're listening on an address that doesn't appear on an interface,
	     ask the kernel what the socket is bound to */
            let mut tcp_len: socklen_t =
                ::std::mem::size_of::<mysockaddr>() as libc::c_ulong as
                    socklen_t;
            if getsockname((*listen).tftpfd,
                           __SOCKADDR_ARG{__sockaddr__:
                                              &mut addr as *mut mysockaddr as
                                                  *mut sockaddr,},
                           &mut tcp_len) == -(1 as libc::c_int) {
                return
            }
        }
    } else {
        let mut cmptr: *mut cmsghdr = 0 as *mut cmsghdr;
        if msg.msg_controllen <
               ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
            return
        }
        addr.sa.sa_family = family as sa_family_t;
        if family == 2 as libc::c_int {
            cmptr =
                if msg.msg_controllen >=
                       ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                    msg.msg_control as *mut cmsghdr
                } else { 0 as *mut cmsghdr };
            while !cmptr.is_null() {
                if (*cmptr).cmsg_level == IPPROTO_IP as libc::c_int &&
                       (*cmptr).cmsg_type == 8 as libc::c_int {
                    let mut p_0: C2RustUnnamed_13 =
                        C2RustUnnamed_13{c: 0 as *mut libc::c_uchar,};
                    p_0.c = (*cmptr).__cmsg_data.as_mut_ptr();
                    addr.in_0.sin_addr = (*p_0.p).ipi_spec_dst;
                    if_index = (*p_0.p).ipi_ifindex
                }
                cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            }
        }
        if family == 10 as libc::c_int {
            cmptr =
                if msg.msg_controllen >=
                       ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                    msg.msg_control as *mut cmsghdr
                } else { 0 as *mut cmsghdr };
            while !cmptr.is_null() {
                if (*cmptr).cmsg_level == IPPROTO_IPV6 as libc::c_int &&
                       (*cmptr).cmsg_type == (*dnsmasq_daemon).v6pktinfo {
                    let mut p_1: C2RustUnnamed_12 =
                        C2RustUnnamed_12{c: 0 as *mut libc::c_uchar,};
                    p_1.c = (*cmptr).__cmsg_data.as_mut_ptr();
                    addr.in6.sin6_addr = (*p_1.p).ipi6_addr;
                    if_index = (*p_1.p).ipi6_ifindex as libc::c_int
                }
                cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            }
        }
        if indextoname((*listen).tftpfd, if_index, namebuff.as_mut_ptr()) == 0
           {
            return
        }
        name = namebuff.as_mut_ptr();
        addra.addr4 = addr.in_0.sin_addr;
        if family == 10 as libc::c_int { addra.addr6 = addr.in6.sin6_addr }
        if !(*dnsmasq_daemon).tftp_interfaces.is_null() {
            /* dedicated tftp interface list */
            tmp = (*dnsmasq_daemon).tftp_interfaces;
            while !tmp.is_null() {
                if !(*tmp).name.is_null() &&
                       wildcard_match((*tmp).name, name) != 0 {
                    break ;
                }
                tmp = (*tmp).next
            }
            if tmp.is_null() { return }
        } else {
            /* Do the same as DHCP */
            if iface_check(family, &mut addra, name, 0 as *mut libc::c_int) ==
                   0 {
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
                    enumerate_interfaces(0 as libc::c_int);
                }
                if loopback_exception((*listen).tftpfd, family, &mut addra,
                                      name) == 0 &&
                       label_exception(if_index, family, &mut addra) == 0 {
                    return
                }
            }
            /* allowed interfaces are the same as for DHCP */
            tmp = (*dnsmasq_daemon).dhcp_except;
            while !tmp.is_null() {
                if !(*tmp).name.is_null() &&
                       wildcard_match((*tmp).name, name) != 0 {
                    return
                }
                tmp = (*tmp).next
            }
        }
        safe_strncpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), name,
                     16 as libc::c_int as size_t);
        if ioctl((*listen).tftpfd, 0x8921 as libc::c_int as libc::c_ulong,
                 &mut ifr as *mut ifreq) != -(1 as libc::c_int) {
            mtu = ifr.ifr_ifru.ifru_mtu;
            if (*dnsmasq_daemon).tftp_mtu != 0 as libc::c_int &&
                   (*dnsmasq_daemon).tftp_mtu < mtu {
                mtu = (*dnsmasq_daemon).tftp_mtu
            }
        }
    }
    /* Failed to get interface mtu - can use configured value. */
    if mtu == 0 as libc::c_int { mtu = (*dnsmasq_daemon).tftp_mtu }
    /* data transfer via server listening socket */
    if (*dnsmasq_daemon).options[(60 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (60 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        let mut tftp_cnt: libc::c_int = 0;
        tftp_cnt = 0 as libc::c_int;
        transfer = (*dnsmasq_daemon).tftp_trans;
        up = &mut (*dnsmasq_daemon).tftp_trans;
        while !transfer.is_null() {
            tftp_cnt += 1;
            if sockaddr_isequal(&mut peer, &mut (*transfer).peer) != 0 {
                if __bswap_16(*(packet as *mut libc::c_ushort)) as libc::c_int
                       == 1 as libc::c_int {
                    /* Handle repeated RRQ or abandoned transfer from same host and port 
		     by unlinking and reusing the struct transfer. */
                    *up = (*transfer).next;
                    break ;
                } else { handle_tftp(now, transfer, len); return }
            } else { up = &mut (*transfer).next; transfer = (*transfer).next }
        }
        /* Enforce simultaneous transfer limit. In non-single-port mode
	 this is doene by not listening on the server socket when
	 too many transfers are in progress. */
        if transfer.is_null() && tftp_cnt >= (*dnsmasq_daemon).tftp_max {
            return
        }
    }
    if !name.is_null() {
        /* check for per-interface prefix */
        pref = (*dnsmasq_daemon).if_prefix;
        while !pref.is_null() {
            if strcmp((*pref).interface, name) == 0 as libc::c_int {
                prefix = (*pref).prefix
            }
            pref = (*pref).next
        }
    }
    if family == 2 as libc::c_int {
        addr.in_0.sin_port = __bswap_16(port as __uint16_t)
    } else {
        addr.in6.sin6_port = __bswap_16(port as __uint16_t);
        addr.in6.sin6_flowinfo = 0 as libc::c_int as uint32_t;
        addr.in6.sin6_scope_id = 0 as libc::c_int as uint32_t
    }
    /* May reuse struct transfer from abandoned transfer in single port mode. */
    if transfer.is_null() &&
           {
               transfer =
                   whine_malloc(::std::mem::size_of::<tftp_transfer>() as
                                    libc::c_ulong) as *mut tftp_transfer;
               transfer.is_null()
           } {
        return
    }
    if (*dnsmasq_daemon).options[(60 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (60 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        (*transfer).sockfd = (*listen).tftpfd
    } else {
        (*transfer).sockfd =
            socket(family, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
        if (*transfer).sockfd == -(1 as libc::c_int) {
            free(transfer as *mut libc::c_void);
            return
        }
    }
    (*transfer).peer = peer;
    (*transfer).source = addra;
    (*transfer).if_index = if_index;
    (*transfer).timeout = now + 2 as libc::c_int as libc::c_long;
    (*transfer).backoff = 1 as libc::c_int;
    (*transfer).block = 1 as libc::c_int as libc::c_uint;
    (*transfer).blocksize = 512 as libc::c_int as libc::c_uint;
    (*transfer).offset = 0 as libc::c_int as off_t;
    (*transfer).file = 0 as *mut tftp_file;
    (*transfer).opt_transize = 0 as libc::c_int as libc::c_char;
    (*transfer).opt_blocksize = (*transfer).opt_transize;
    (*transfer).carrylf = 0 as libc::c_int as libc::c_char;
    (*transfer).netascii = (*transfer).carrylf;
    prettyprint_addr(&mut peer, (*dnsmasq_daemon).addrbuff);
    /* if we have a nailed-down range, iterate until we find a free one. */
    while (*dnsmasq_daemon).options[(60 as libc::c_int as
                                         libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                          as
                                                                          libc::c_ulong).wrapping_mul(8
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_ulong))
                                        as usize] &
              (1 as libc::c_uint) <<
                  (60 as libc::c_int as
                       libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                        as
                                                        libc::c_ulong).wrapping_mul(8
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong))
              == 0 {
        if !(bind((*transfer).sockfd,
                  __CONST_SOCKADDR_ARG{__sockaddr__: &mut addr.sa,},
                  sa_len(&mut addr) as socklen_t) == -(1 as libc::c_int) ||
                 setsockopt((*transfer).sockfd, IPPROTO_IP as libc::c_int,
                            10 as libc::c_int,
                            &mut mtuflag as *mut libc::c_int as
                                *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as
                                libc::c_ulong as socklen_t) ==
                     -(1 as libc::c_int) || fix_fd((*transfer).sockfd) == 0) {
            break ;
        }
        if *__errno_location() == 98 as libc::c_int &&
               (*dnsmasq_daemon).start_tftp_port != 0 as libc::c_int {
            port += 1;
            if port <= (*dnsmasq_daemon).end_tftp_port {
                if family == 2 as libc::c_int {
                    addr.in_0.sin_port = __bswap_16(port as __uint16_t)
                } else { addr.in6.sin6_port = __bswap_16(port as __uint16_t) }
                continue ;
            } else {
                my_syslog((1 as libc::c_int) << 3 as libc::c_int |
                              3 as libc::c_int,
                          b"unable to get free port for TFTP\x00" as *const u8
                              as *const libc::c_char);
            }
        }
        free_transfer(transfer);
        return
    }
    p = packet.offset(2 as libc::c_int as isize);
    end = packet.offset(len as isize);
    if __bswap_16(*(packet as *mut libc::c_ushort)) as libc::c_int !=
           1 as libc::c_int ||
           { filename = next(&mut p, end); filename.is_null() } ||
           { mode = next(&mut p, end); mode.is_null() } ||
           strcasecmp(mode, b"octet\x00" as *const u8 as *const libc::c_char)
               != 0 as libc::c_int &&
               strcasecmp(mode,
                          b"netascii\x00" as *const u8 as *const libc::c_char)
                   != 0 as libc::c_int {
        len =
            tftp_err(4 as libc::c_int, packet,
                     b"unsupported request from %s\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char,
                     (*dnsmasq_daemon).addrbuff);
        is_err = 1 as libc::c_int
    } else {
        if strcasecmp(mode,
                      b"netascii\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            (*transfer).netascii = 1 as libc::c_int as libc::c_char
        }
        loop  {
            opt = next(&mut p, end);
            if opt.is_null() { break ; }
            if strcasecmp(opt,
                          b"blksize\x00" as *const u8 as *const libc::c_char)
                   == 0 as libc::c_int {
                opt = next(&mut p, end);
                if !opt.is_null() &&
                       (*dnsmasq_daemon).options[(27 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (27 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           == 0 {
                    /* 32 bytes for IP, UDP and TFTP headers, 52 bytes for IPv6 */
                    let mut overhead: libc::c_int =
                        if family == 2 as libc::c_int {
                            32 as libc::c_int
                        } else { 52 as libc::c_int };
                    (*transfer).blocksize = atoi(opt) as libc::c_uint;
                    if (*transfer).blocksize <
                           1 as libc::c_int as libc::c_uint {
                        (*transfer).blocksize =
                            1 as libc::c_int as libc::c_uint
                    }
                    if (*transfer).blocksize >
                           ((*dnsmasq_daemon).packet_buff_sz as
                                libc::c_uint).wrapping_sub(4 as libc::c_int as
                                                               libc::c_uint) {
                        (*transfer).blocksize =
                            ((*dnsmasq_daemon).packet_buff_sz as
                                 libc::c_uint).wrapping_sub(4 as libc::c_int
                                                                as
                                                                libc::c_uint)
                    }
                    if mtu != 0 as libc::c_int &&
                           (*transfer).blocksize >
                               (mtu as
                                    libc::c_uint).wrapping_sub(overhead as
                                                                   libc::c_uint)
                       {
                        (*transfer).blocksize =
                            (mtu as
                                 libc::c_uint).wrapping_sub(overhead as
                                                                libc::c_uint)
                    }
                    (*transfer).opt_blocksize =
                        1 as libc::c_int as libc::c_char;
                    (*transfer).block = 0 as libc::c_int as libc::c_uint
                }
            } else if strcasecmp(opt,
                                 b"tsize\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                          && !next(&mut p, end).is_null() &&
                          (*transfer).netascii == 0 {
                (*transfer).opt_transize = 1 as libc::c_int as libc::c_char;
                (*transfer).block = 0 as libc::c_int as libc::c_uint
            }
        }
        /* cope with backslashes from windows boxen. */
        p = filename;
        while *p != 0 {
            if *p as libc::c_int == '\\' as i32 {
                *p = '/' as i32 as libc::c_char
            } else if (*dnsmasq_daemon).options[(38 as libc::c_int as
                                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_mul(8
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulong))
                                                    as usize] &
                          (1 as libc::c_uint) <<
                              (38 as libc::c_int as
                                   libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong))
                          != 0 {
                *p =
                    ({
                         let mut __res: libc::c_int = 0;
                         if ::std::mem::size_of::<libc::c_char>() as
                                libc::c_ulong >
                                1 as libc::c_int as libc::c_ulong {
                             if 0 != 0 {
                                 let mut __c: libc::c_int = *p as libc::c_int;
                                 __res =
                                     if __c < -(128 as libc::c_int) ||
                                            __c > 255 as libc::c_int {
                                         __c
                                     } else {
                                         *(*__ctype_tolower_loc()).offset(__c
                                                                              as
                                                                              isize)
                                     }
                             } else { __res = tolower(*p as libc::c_int) }
                         } else {
                             __res =
                                 *(*__ctype_tolower_loc()).offset(*p as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                         }
                         __res
                     }) as libc::c_char
            }
            p = p.offset(1)
        }
        strcpy((*dnsmasq_daemon).namebuff,
               b"/\x00" as *const u8 as *const libc::c_char);
        if !prefix.is_null() {
            if *prefix.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '/' as i32 {
                *(*dnsmasq_daemon).namebuff.offset(0 as libc::c_int as isize)
                    = 0 as libc::c_int as libc::c_char
            }
            strncat((*dnsmasq_daemon).namebuff, prefix,
                    ((1025 as libc::c_int - 1 as libc::c_int) as
                         libc::c_ulong).wrapping_sub(strlen((*dnsmasq_daemon).namebuff)));
            if *prefix.offset(strlen(prefix).wrapping_sub(1 as libc::c_int as
                                                              libc::c_ulong)
                                  as isize) as libc::c_int != '/' as i32 {
                strncat((*dnsmasq_daemon).namebuff,
                        b"/\x00" as *const u8 as *const libc::c_char,
                        ((1025 as libc::c_int - 1 as libc::c_int) as
                             libc::c_ulong).wrapping_sub(strlen((*dnsmasq_daemon).namebuff)));
            }
            if (*dnsmasq_daemon).options[(29 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (29 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
                let mut oldlen: size_t = strlen((*dnsmasq_daemon).namebuff);
                let mut statbuf: stat =
                    stat{st_dev: 0,
                         st_ino: 0,
                         st_nlink: 0,
                         st_mode: 0,
                         st_uid: 0,
                         st_gid: 0,
                         __pad0: 0,
                         st_rdev: 0,
                         st_size: 0,
                         st_blksize: 0,
                         st_blocks: 0,
                         st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
                         st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
                         st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
                         __glibc_reserved: [0; 3],};
                strncat((*dnsmasq_daemon).namebuff,
                        (*dnsmasq_daemon).addrbuff,
                        ((1025 as libc::c_int - 1 as libc::c_int) as
                             libc::c_ulong).wrapping_sub(strlen((*dnsmasq_daemon).namebuff)));
                strncat((*dnsmasq_daemon).namebuff,
                        b"/\x00" as *const u8 as *const libc::c_char,
                        ((1025 as libc::c_int - 1 as libc::c_int) as
                             libc::c_ulong).wrapping_sub(strlen((*dnsmasq_daemon).namebuff)));
                /* remove unique-directory if it doesn't exist */
                if stat((*dnsmasq_daemon).namebuff, &mut statbuf) ==
                       -(1 as libc::c_int) ||
                       !(statbuf.st_mode &
                             0o170000 as libc::c_int as libc::c_uint ==
                             0o40000 as libc::c_int as libc::c_uint) {
                    *(*dnsmasq_daemon).namebuff.offset(oldlen as isize) =
                        0 as libc::c_int as libc::c_char
                }
            }
            if (*dnsmasq_daemon).options[(56 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (56 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
                let mut macaddr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut macbuf: [libc::c_uchar; 16] = [0; 16];
                if !(*dnsmasq_daemon).dhcp.is_null() &&
                       peer.sa.sa_family as libc::c_int == 2 as libc::c_int {
                    /* Check if the client IP is in our lease database */
                    let mut lease: *mut dhcp_lease =
                        lease_find_by_addr(peer.in_0.sin_addr);
                    if !lease.is_null() &&
                           (*lease).hwaddr_type == 1 as libc::c_int &&
                           (*lease).hwaddr_len == 6 as libc::c_int {
                        macaddr = (*lease).hwaddr.as_mut_ptr()
                    }
                }
                /* If no luck, try to find in ARP table. This only works if client is in same (V)LAN */
                if macaddr.is_null() &&
                       find_mac(&mut peer, macbuf.as_mut_ptr(),
                                1 as libc::c_int, now) > 0 as libc::c_int {
                    macaddr = macbuf.as_mut_ptr()
                }
                if !macaddr.is_null() {
                    let mut oldlen_0: size_t =
                        strlen((*dnsmasq_daemon).namebuff);
                    let mut statbuf_0: stat =
                        stat{st_dev: 0,
                             st_ino: 0,
                             st_nlink: 0,
                             st_mode: 0,
                             st_uid: 0,
                             st_gid: 0,
                             __pad0: 0,
                             st_rdev: 0,
                             st_size: 0,
                             st_blksize: 0,
                             st_blocks: 0,
                             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
                             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
                             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
                             __glibc_reserved: [0; 3],};
                    snprintf((*dnsmasq_daemon).namebuff.offset(oldlen_0 as
                                                                   isize),
                             ((1025 as libc::c_int - 1 as libc::c_int) as
                                  libc::c_ulong).wrapping_sub(oldlen_0),
                             b"%.2x-%.2x-%.2x-%.2x-%.2x-%.2x/\x00" as
                                 *const u8 as *const libc::c_char,
                             *macaddr.offset(0 as libc::c_int as isize) as
                                 libc::c_int,
                             *macaddr.offset(1 as libc::c_int as isize) as
                                 libc::c_int,
                             *macaddr.offset(2 as libc::c_int as isize) as
                                 libc::c_int,
                             *macaddr.offset(3 as libc::c_int as isize) as
                                 libc::c_int,
                             *macaddr.offset(4 as libc::c_int as isize) as
                                 libc::c_int,
                             *macaddr.offset(5 as libc::c_int as isize) as
                                 libc::c_int);
                    /* remove unique-directory if it doesn't exist */
                    if stat((*dnsmasq_daemon).namebuff, &mut statbuf_0) ==
                           -(1 as libc::c_int) ||
                           !(statbuf_0.st_mode &
                                 0o170000 as libc::c_int as libc::c_uint ==
                                 0o40000 as libc::c_int as libc::c_uint) {
                        *(*dnsmasq_daemon).namebuff.offset(oldlen_0 as isize)
                            = 0 as libc::c_int as libc::c_char
                    }
                }
            }
            /* Absolute pathnames OK if they match prefix */
            if *filename.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '/' as i32 {
                if strstr(filename, (*dnsmasq_daemon).namebuff) == filename {
                    *(*dnsmasq_daemon).namebuff.offset(0 as libc::c_int as
                                                           isize) =
                        0 as libc::c_int as libc::c_char
                } else { filename = filename.offset(1) }
            }
        } else if *filename.offset(0 as libc::c_int as isize) as libc::c_int
                      == '/' as i32 {
            *(*dnsmasq_daemon).namebuff.offset(0 as libc::c_int as isize) =
                0 as libc::c_int as libc::c_char
        }
        strncat((*dnsmasq_daemon).namebuff, filename,
                ((1025 as libc::c_int - 1 as libc::c_int) as
                     libc::c_ulong).wrapping_sub(strlen((*dnsmasq_daemon).namebuff)));
        /* check permissions and open file */
        (*transfer).file = check_tftp_fileperm(&mut len, prefix);
        if !(*transfer).file.is_null() {
            len = get_block(packet, transfer);
            if len == -(1 as libc::c_int) as libc::c_long {
                len = tftp_err_oops(packet, (*dnsmasq_daemon).namebuff)
            } else { is_err = 0 as libc::c_int }
        }
    }
    send_from((*transfer).sockfd,
              ((*dnsmasq_daemon).options[(60 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (60 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   == 0) as libc::c_int, packet, len as size_t, &mut peer,
              &mut addra, if_index as libc::c_uint);
    if is_err != 0 {
        free_transfer(transfer);
    } else {
        (*transfer).next = (*dnsmasq_daemon).tftp_trans;
        (*dnsmasq_daemon).tftp_trans = transfer
    };
}
unsafe extern "C" fn check_tftp_fileperm(mut len: *mut ssize_t,
                                         mut prefix: *mut libc::c_char)
 -> *mut tftp_file {
    let mut current_block: u64;
    let mut packet: *mut libc::c_char = (*dnsmasq_daemon).packet;
    let mut namebuff: *mut libc::c_char = (*dnsmasq_daemon).namebuff;
    let mut file: *mut tftp_file = 0 as *mut tftp_file;
    let mut t: *mut tftp_transfer = 0 as *mut tftp_transfer;
    let mut uid: uid_t = geteuid();
    let mut statbuf: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut fd: libc::c_int = -(1 as libc::c_int);
    /* trick to ban moving out of the subtree */
    if !(!prefix.is_null() &&
             !strstr(namebuff,
                     b"/../\x00" as *const u8 as
                         *const libc::c_char).is_null()) {
        fd = open(namebuff, 0 as libc::c_int);
        if fd == -(1 as libc::c_int) {
            if *__errno_location() == 2 as libc::c_int {
                *len =
                    tftp_err(1 as libc::c_int, packet,
                             b"file %s not found\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             namebuff);
                return 0 as *mut tftp_file
            } else if *__errno_location() == 13 as libc::c_int {
                current_block = 15533630919196144218;
            } else { current_block = 9018216499526184084; }
        } else if fstat(fd, &mut statbuf) == -(1 as libc::c_int) {
            current_block = 9018216499526184084;
        } else {
            /* stat the file descriptor to avoid stat->open races */
            /* running as root, must be world-readable */
            if uid == 0 as libc::c_int as libc::c_uint {
                if statbuf.st_mode &
                       (0o400 as libc::c_int >> 3 as libc::c_int >>
                            3 as libc::c_int) as libc::c_uint == 0 {
                    current_block = 15533630919196144218;
                } else { current_block = 1054647088692577877; }
            } else if (*dnsmasq_daemon).options[(26 as libc::c_int as
                                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_mul(8
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulong))
                                                    as usize] &
                          (1 as libc::c_uint) <<
                              (26 as libc::c_int as
                                   libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong))
                          != 0 && uid != statbuf.st_uid {
                current_block = 15533630919196144218;
            } else { current_block = 1054647088692577877; }
            match current_block {
                15533630919196144218 => { }
                _ => {
                    /* in secure mode, must be owned by user running dnsmasq */
                    /* If we're doing many transfers from the same file, only 
     open it once this saves lots of file descriptors 
     when mass-booting a big cluster, for instance. 
     Be conservative and only share when inode and name match
     this keeps error messages sane. */
                    t = (*dnsmasq_daemon).tftp_trans;
                    while !t.is_null() {
                        if (*(*t).file).dev == statbuf.st_dev &&
                               (*(*t).file).inode == statbuf.st_ino &&
                               strcmp((*(*t).file).filename.as_mut_ptr(),
                                      namebuff) == 0 as libc::c_int {
                            close(fd);
                            (*(*t).file).refcount += 1;
                            return (*t).file
                        }
                        t = (*t).next
                    }
                    file =
                        whine_malloc((::std::mem::size_of::<tftp_file>() as
                                          libc::c_ulong).wrapping_add(strlen(namebuff)).wrapping_add(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                            as *mut tftp_file;
                    if file.is_null() {
                        *__errno_location() = 12 as libc::c_int
                    } else {
                        (*file).fd = fd;
                        (*file).size = statbuf.st_size;
                        (*file).dev = statbuf.st_dev;
                        (*file).inode = statbuf.st_ino;
                        (*file).refcount = 1 as libc::c_int;
                        strcpy((*file).filename.as_mut_ptr(), namebuff);
                        return file
                    }
                    current_block = 9018216499526184084;
                }
            }
        }
        match current_block {
            15533630919196144218 => { }
            _ => {
                *len = tftp_err_oops(packet, namebuff);
                if fd != -(1 as libc::c_int) { close(fd); }
                return 0 as *mut tftp_file
            }
        }
    }
    *__errno_location() = 13 as libc::c_int;
    *len =
        tftp_err(2 as libc::c_int, packet,
                 b"cannot access %s: %s\x00" as *const u8 as
                     *const libc::c_char as *mut libc::c_char, namebuff);
    if fd != -(1 as libc::c_int) { close(fd); }
    return 0 as *mut tftp_file;
}
#[no_mangle]
pub unsafe extern "C" fn check_tftp_listeners(mut now: time_t) {
    let mut transfer: *mut tftp_transfer = 0 as *mut tftp_transfer;
    let mut tmp: *mut tftp_transfer = 0 as *mut tftp_transfer;
    let mut up: *mut *mut tftp_transfer = 0 as *mut *mut tftp_transfer;
    /* In single port mode, all packets come via port 69 and tftp_request() */
    if (*dnsmasq_daemon).options[(60 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (60 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        transfer = (*dnsmasq_daemon).tftp_trans;
        while !transfer.is_null() {
            if poll_check((*transfer).sockfd,
                          0x1 as libc::c_int as libc::c_short) != 0 {
                /* we overwrote the buffer... */
                (*dnsmasq_daemon).srv_save = 0 as *mut server;
                handle_tftp(now, transfer,
                            recv((*transfer).sockfd,
                                 (*dnsmasq_daemon).packet as
                                     *mut libc::c_void,
                                 (*dnsmasq_daemon).packet_buff_sz as size_t,
                                 0 as libc::c_int));
            }
            transfer = (*transfer).next
        }
    }
    let mut current_block_32: u64;
    transfer = (*dnsmasq_daemon).tftp_trans;
    up = &mut (*dnsmasq_daemon).tftp_trans;
    while !transfer.is_null() {
        tmp = (*transfer).next;
        if difftime(now, (*transfer).timeout) >= 0.0f64 {
            let mut endcon: libc::c_int = 0 as libc::c_int;
            let mut len: ssize_t = 0;
            /* timeout, retransmit */
            (*transfer).timeout +=
                (1 as libc::c_int +
                     ((1 as libc::c_int) <<
                          (*transfer).backoff / 2 as libc::c_int)) as
                    libc::c_long;
            /* we overwrote the buffer... */
            (*dnsmasq_daemon).srv_save = 0 as *mut server;
            len = get_block((*dnsmasq_daemon).packet, transfer);
            if len == -(1 as libc::c_int) as libc::c_long {
                len =
                    tftp_err_oops((*dnsmasq_daemon).packet,
                                  (*(*transfer).file).filename.as_mut_ptr());
                endcon = 1 as libc::c_int
            } else {
                (*transfer).backoff += 1;
                if (*transfer).backoff > 7 as libc::c_int {
                    /* don't complain about timeout when we're awaiting the last
		 ACK, some clients never send it */
                    if len as libc::c_uint ==
                           (*transfer).blocksize.wrapping_add(4 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                       {
                        endcon = 1 as libc::c_int
                    }
                    len = 0 as libc::c_int as ssize_t
                }
            }
            if len != 0 as libc::c_int as libc::c_long {
                send_from((*transfer).sockfd,
                          ((*dnsmasq_daemon).options[(60 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (60 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               == 0) as libc::c_int, (*dnsmasq_daemon).packet,
                          len as size_t, &mut (*transfer).peer,
                          &mut (*transfer).source,
                          (*transfer).if_index as libc::c_uint);
            }
            if endcon != 0 || len == 0 as libc::c_int as libc::c_long {
                strcpy((*dnsmasq_daemon).namebuff,
                       (*(*transfer).file).filename.as_mut_ptr());
                sanitise((*dnsmasq_daemon).namebuff);
                prettyprint_addr(&mut (*transfer).peer,
                                 (*dnsmasq_daemon).addrbuff);
                my_syslog((1 as libc::c_int) << 3 as libc::c_int |
                              6 as libc::c_int,
                          if endcon != 0 {
                              b"failed sending %s to %s\x00" as *const u8 as
                                  *const libc::c_char
                          } else {
                              b"sent %s to %s\x00" as *const u8 as
                                  *const libc::c_char
                          }, (*dnsmasq_daemon).namebuff,
                          (*dnsmasq_daemon).addrbuff);
                /* unlink */
                *up = tmp;
                if endcon != 0 {
                    free_transfer(transfer);
                } else {
                    /* put on queue to be sent to script and deleted */
                    (*transfer).next = (*dnsmasq_daemon).tftp_done_trans;
                    (*dnsmasq_daemon).tftp_done_trans = transfer
                }
                current_block_32 = 14523784380283086299;
            } else { current_block_32 = 11385396242402735691; }
        } else { current_block_32 = 11385396242402735691; }
        match current_block_32 {
            11385396242402735691 => { up = &mut (*transfer).next }
            _ => { }
        }
        transfer = tmp
    };
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
/* packet in daemon->packet as this is called. */
unsafe extern "C" fn handle_tftp(mut now: time_t,
                                 mut transfer: *mut tftp_transfer,
                                 mut len: ssize_t) {
    let mut mess: *mut ack = (*dnsmasq_daemon).packet as *mut ack;
    if len >= ::std::mem::size_of::<ack>() as libc::c_ulong as ssize_t {
        if __bswap_16((*mess).op) as libc::c_int == 4 as libc::c_int &&
               __bswap_16((*mess).block) as libc::c_int ==
                   (*transfer).block as libc::c_ushort as libc::c_int {
            /* Got ack, ensure we take the (re)transmit path */
            (*transfer).timeout = now;
            (*transfer).backoff = 0 as libc::c_int;
            let fresh6 = (*transfer).block;
            (*transfer).block = (*transfer).block.wrapping_add(1);
            if fresh6 != 0 as libc::c_int as libc::c_uint {
                (*transfer).offset +=
                    (*transfer).blocksize.wrapping_sub((*transfer).expansion)
                        as libc::c_long
            }
        } else if __bswap_16((*mess).op) as libc::c_int == 5 as libc::c_int {
            let mut p: *mut libc::c_char =
                (*dnsmasq_daemon).packet.offset(::std::mem::size_of::<ack>()
                                                    as libc::c_ulong as
                                                    isize);
            let mut end: *mut libc::c_char =
                (*dnsmasq_daemon).packet.offset(len as isize);
            let mut err: *mut libc::c_char = next(&mut p, end);
            prettyprint_addr(&mut (*transfer).peer,
                             (*dnsmasq_daemon).addrbuff);
            /* Sanitise error message */
            if err.is_null() {
                err =
                    b"\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            } else { sanitise(err); }
            my_syslog((1 as libc::c_int) << 3 as libc::c_int |
                          3 as libc::c_int,
                      b"error %d %s received from %s\x00" as *const u8 as
                          *const libc::c_char,
                      __bswap_16((*mess).block) as libc::c_int, err,
                      (*dnsmasq_daemon).addrbuff);
            /* Got err, ensure we take abort */
            (*transfer).timeout = now;
            (*transfer).backoff = 100 as libc::c_int
        }
    };
}
unsafe extern "C" fn free_transfer(mut transfer: *mut tftp_transfer) {
    if (*dnsmasq_daemon).options[(60 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (60 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        close((*transfer).sockfd);
    }
    if !(*transfer).file.is_null() &&
           {
               (*(*transfer).file).refcount -= 1;
               ((*(*transfer).file).refcount) == 0 as libc::c_int
           } {
        close((*(*transfer).file).fd);
        free((*transfer).file as *mut libc::c_void);
    }
    free(transfer as *mut libc::c_void);
}
unsafe extern "C" fn next(mut p: *mut *mut libc::c_char,
                          mut end: *mut libc::c_char) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = *p;
    let mut len: size_t = 0;
    if *end.offset(-(1 as libc::c_int as isize)) as libc::c_int !=
           0 as libc::c_int || *p == end ||
           { len = strlen(ret); (len) == 0 as libc::c_int as libc::c_ulong } {
        return 0 as *mut libc::c_char
    }
    *p =
        (*p).offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong) as
                        isize);
    return ret;
}
unsafe extern "C" fn sanitise(mut buf: *mut libc::c_char) {
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut r: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    r = buf as *mut libc::c_uchar;
    q = r;
    while *r != 0 {
        if *(*__ctype_b_loc()).offset(*r as libc::c_int as isize) as
               libc::c_int &
               _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            let fresh7 = q;
            q = q.offset(1);
            *fresh7 = *r
        }
        r = r.offset(1)
    }
    *q = 0 as libc::c_int as libc::c_uchar;
}
/* limit to make packet < 512 bytes and definitely smaller than buffer */
unsafe extern "C" fn tftp_err(mut err: libc::c_int,
                              mut packet: *mut libc::c_char,
                              mut message: *mut libc::c_char,
                              mut file: *mut libc::c_char) -> ssize_t {
    let mut mess: *mut errmess =
        packet as *mut errmess; /* include terminating zero */
    let mut len: ssize_t = 0;
    let mut ret: ssize_t = 4 as libc::c_int as ssize_t;
    let mut errstr: *mut libc::c_char = strerror(*__errno_location());
    memset(packet as *mut libc::c_void, 0 as libc::c_int,
           (*dnsmasq_daemon).packet_buff_sz as libc::c_ulong);
    sanitise(file);
    (*mess).op = __bswap_16(5 as libc::c_int as __uint16_t);
    (*mess).err = __bswap_16(err as __uint16_t);
    len =
        snprintf((*mess).message.as_mut_ptr(),
                 500 as libc::c_int as libc::c_ulong, message, file, errstr)
            as ssize_t;
    ret +=
        if len < 500 as libc::c_int as libc::c_long {
            (len) + 1 as libc::c_int as libc::c_long
        } else { 500 as libc::c_int as libc::c_long };
    my_syslog((1 as libc::c_int) << 3 as libc::c_int | 3 as libc::c_int,
              b"%s\x00" as *const u8 as *const libc::c_char,
              (*mess).message.as_mut_ptr());
    return ret;
}
unsafe extern "C" fn tftp_err_oops(mut packet: *mut libc::c_char,
                                   mut file: *mut libc::c_char) -> ssize_t {
    /* May have >1 refs to file, so potentially mangle a copy of the name */
    strcpy((*dnsmasq_daemon).namebuff, file);
    return tftp_err(0 as libc::c_int, packet,
                    b"cannot read %s: %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    (*dnsmasq_daemon).namebuff);
}
/* return -1 for error, zero for done. */
unsafe extern "C" fn get_block(mut packet: *mut libc::c_char,
                               mut transfer: *mut tftp_transfer) -> ssize_t {
    memset(packet as *mut libc::c_void, 0 as libc::c_int,
           (*dnsmasq_daemon).packet_buff_sz as libc::c_ulong);
    if (*transfer).block == 0 as libc::c_int as libc::c_uint {
        /* send OACK */
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut mess: *mut oackmess = packet as *mut oackmess;
        p = (*mess).data.as_mut_ptr();
        (*mess).op = __bswap_16(6 as libc::c_int as __uint16_t);
        if (*transfer).opt_blocksize != 0 {
            p =
                p.offset((sprintf(p,
                                  b"blksize\x00" as *const u8 as
                                      *const libc::c_char) + 1 as libc::c_int)
                             as isize);
            p =
                p.offset((sprintf(p,
                                  b"%u\x00" as *const u8 as
                                      *const libc::c_char,
                                  (*transfer).blocksize) + 1 as libc::c_int)
                             as isize)
        }
        if (*transfer).opt_transize != 0 {
            p =
                p.offset((sprintf(p,
                                  b"tsize\x00" as *const u8 as
                                      *const libc::c_char) + 1 as libc::c_int)
                             as isize);
            p =
                p.offset((sprintf(p,
                                  b"%u\x00" as *const u8 as
                                      *const libc::c_char,
                                  (*(*transfer).file).size as libc::c_uint) +
                              1 as libc::c_int) as isize)
        }
        return p.wrapping_offset_from(packet) as libc::c_long
    } else {
        /* send data packet */
        let mut mess_0: *mut datamess =
            packet as *mut datamess; /* finished */
        let mut size: size_t =
            ((*(*transfer).file).size - (*transfer).offset) as size_t;
        if (*transfer).offset > (*(*transfer).file).size {
            return 0 as libc::c_int as ssize_t
        }
        if size > (*transfer).blocksize as libc::c_ulong {
            size = (*transfer).blocksize as size_t
        }
        (*mess_0).op = __bswap_16(3 as libc::c_int as __uint16_t);
        (*mess_0).block = __bswap_16((*transfer).block as libc::c_ushort);
        if lseek((*(*transfer).file).fd, (*transfer).offset, 0 as libc::c_int)
               == -(1 as libc::c_int) as off_t ||
               read_write((*(*transfer).file).fd, (*mess_0).data.as_mut_ptr(),
                          size as libc::c_int, 1 as libc::c_int) == 0 {
            return -(1 as libc::c_int) as ssize_t
        }
        (*transfer).expansion = 0 as libc::c_int as libc::c_uint;
        /* Map '\n' to CR-LF in netascii mode */
        if (*transfer).netascii != 0 {
            let mut i: size_t =
                0; /* don't expand LF again if it moves to the next block */
            let mut newcarrylf: libc::c_int = 0; /* room in this block */
            i = 0 as libc::c_int as size_t;
            newcarrylf = 0 as libc::c_int;
            while i < size {
                if *(*mess_0).data.as_mut_ptr().offset(i as isize) as
                       libc::c_int == '\n' as i32 &&
                       (i != 0 as libc::c_int as libc::c_ulong ||
                            (*transfer).carrylf == 0) {
                    (*transfer).expansion =
                        (*transfer).expansion.wrapping_add(1);
                    if size != (*transfer).blocksize as libc::c_ulong {
                        size = size.wrapping_add(1)
                    } else if i ==
                                  size.wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong) {
                        newcarrylf = 1 as libc::c_int
                    }
                    /* make space and insert CR */
                    memmove(&mut *(*mess_0).data.as_mut_ptr().offset(i.wrapping_add(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong)
                                                                         as
                                                                         isize)
                                as *mut libc::c_uchar as *mut libc::c_void,
                            &mut *(*mess_0).data.as_mut_ptr().offset(i as
                                                                         isize)
                                as *mut libc::c_uchar as *const libc::c_void,
                            size.wrapping_sub(i.wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)));
                    *(*mess_0).data.as_mut_ptr().offset(i as isize) =
                        '\r' as i32 as libc::c_uchar;
                    i = i.wrapping_add(1)
                }
                i = i.wrapping_add(1)
            }
            (*transfer).carrylf = newcarrylf as libc::c_char
        }
        return size.wrapping_add(4 as libc::c_int as libc::c_ulong) as ssize_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_tftp_script_run() -> libc::c_int {
    let mut transfer: *mut tftp_transfer = 0 as *mut tftp_transfer;
    transfer = (*dnsmasq_daemon).tftp_done_trans;
    if !transfer.is_null() {
        (*dnsmasq_daemon).tftp_done_trans = (*transfer).next;
        queue_tftp((*(*transfer).file).size,
                   (*(*transfer).file).filename.as_mut_ptr(),
                   &mut (*transfer).peer);
        free_transfer(transfer);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
