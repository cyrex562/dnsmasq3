use crate::defines::{Listener, time::Instant, DnsmasqDaemon, NetAddress, NetAddress, MsgHdr, iovec, IfReq, DigitalSignature, Iname, TftpTransfer, TftpPrefix, NetAddress, NetAddress, C2rustUnnamed14, CmsgHdr, socklen_t, Server, NetAddressArg, SaFamily, IPPROTO_IP, C2RustUnnamed_13, IPPROTO_IPV6, C2rustUnnamed12, __bswap_16, SOCK_DGRAM, off_t, TftpFile, ConstNetAddressArg, stat, timespec, DhcpLease, uid_t, _ISPRINT};
use crate::network::{indextoname, iface_check, enumerate_interfaces, loopback_exception, label_exception, fix_fd};
use crate::util::{wildcard_match, safe_strncpy, NetAddress_isequal, whine_malloc, prettyprint_addr, sa_len, read_write};
use crate::dnsmasq_log::my_syslog;
use crate::lease::lease_find_by_addr;
use crate::arp::find_mac;
use crate::forward::send_from;
use crate::poll::poll_check;
use crate::slack::{ack, errmess, oackmess, datamess};
use crate::helper::queue_tftp;

#[no_mangle]
pub  fn tftp_request(mut listen: Listener,
                                      mut now: time::Instant) {
    let mut len: isize = 0;
    let mut packet: &mut String = daemon.packet;
    let mut filename: &mut String = 0 ;
    let mut mode: &mut String = 0 ;
    let mut p: &mut String = 0 ;
    let mut end: &mut String = 0 ;
    let mut opt: &mut String = 0 ;
    let mut addr: NetAddress =
        NetAddress {sa: NetAddress {sa_family: 0, sa_data: [0; 14],},};
    let mut peer: NetAddress =
        NetAddress {sa: NetAddress {sa_family: 0, sa_data: [0; 14],},};
    let mut msg: MsgHdr =
        MsgHdr {msg_name: 0,
               msg_namelen: 0,
               msg_iov: 0,
               msg_iovlen: 0,
               msg_control: 0,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut iov: iovec = iovec{iov_base: 0, iov_len: 0,};
    let mut ifr: IfReq =
        IfReq {ifr_ifrn: DigitalSignature {ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_1{ifru_addr:
                                      NetAddress {sa_family: 0,
                                               sa_data: [0; 14],},},};
    let mut is_err: i32 = 1;
    let mut if_index: i32 = 0;
    let mut mtu: i32 = 0;
    let mut tmp: Iname = 0;
    let mut transfer: TftpTransfer = 0 ;
    let mut up: TftpTransfer;
    let mut port: i32 = daemon.start_tftp_port;
    let mut mtuflag: i32 = 0;
    let mut namebuff: [libc::c_char; 16] = [0; 16];
    let mut name: &mut String = 0 ;
    let mut prefix: &mut String = daemon.tftp_prefix;
    let mut pref: TftpPrefix = 0 ;
    let mut addra: NetAddress = NetAddress {addr4: NetAddress {s_addr: 0,},};
    let mut family: i32 = listen.addr.sa.sa_family;
    /* Can always get recvd interface for IPv6 */
    let mut check_dest: i32 =
        (daemon.options[(13 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       ).wrapping_mul(8))
                                       ] &
             (1) <<
                 (13 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                   ).wrapping_mul(8))
             == 0 || family == 10);
    let mut control_u: C2rustUnnamed14 =
        C2rustUnnamed14 {align:
                             CmsgHdr {cmsg_len: 0,
                                     cmsg_level: 0,
                                     cmsg_type: 0,
                                     __cmsg_data: [],},};
    msg.msg_controllen =
        ::std::mem::size_of::<C2rustUnnamed14>();
    msg.msg_control = control_u.control.as_mut_ptr();
    msg.msg_flags = 0;
    msg.msg_name = &mut peer ;
    msg.msg_namelen =
        ::std::mem::size_of::<NetAddress>();
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1 ;
    iov.iov_base = packet;
    iov.iov_len = daemon.packet_buff_sz ;
    /* we overwrote the buffer... */
    daemon.srv_save = 0;
    len = recvmsg(listen.tftpfd, &mut msg, 0);
    if len < 2 { return }
    /* Can always get recvd interface for IPv6 */
    if check_dest == 0 {
        if !listen.iface.is_null() {
            addr = (*listen.iface).addr;
            name = (*listen.iface).name;
            mtu = (*listen.iface).mtu;
            if daemon.tftp_mtu != 0 &&
                   daemon.tftp_mtu < mtu {
                mtu = daemon.tftp_mtu
            }
        } else {
            /* we're listening on an address that doesn't appear on an interface,
	     ask the kernel what the socket is bound to */
            let mut tcp_len: socklen_t =
                ::std::mem::size_of::<NetAddress>()              socklen_t;
            if getsockname(listen.tftpfd,
                           NetAddressArg {__NetAddress__:
                                              &mut addr                                             NetAddress,},
                           &mut tcp_len) == -(1) {
                return
            }
        }
    } else {
        let mut cmptr: CmsgHdr = 0;
        if msg.msg_controllen <
               ::std::mem::size_of::<CmsgHdr>() {
            return
        }
        addr.sa.sa_family = family;
        if family == 2 {
            cmptr =
                if msg.msg_controllen >=
                       ::std::mem::size_of::<CmsgHdr>() {
                    msg.msg_control
                } else { 0 };
            while !cmptr.is_null() {
                if cmptr.cmsg_level == IPPROTO_IP &&
                       cmptr.cmsg_type == 8 {
                    let mut p_0: C2RustUnnamed_13 =
                        C2RustUnnamed_13{c: 0,};
                    p_0.c = cmptr.__cmsg_data.as_mut_ptr();
                    addr.in_0.sin_addr = (*p_0.p).ipi_spec_dst;
                    if_index = (*p_0.p).ipi_ifindex
                }
                cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            }
        }
        if family == 10 {
            cmptr =
                if msg.msg_controllen >=
                       ::std::mem::size_of::<CmsgHdr>() {
                    msg.msg_control
                } else { 0 };
            while !cmptr.is_null() {
                if cmptr.cmsg_level == IPPROTO_IPV6 &&
                       cmptr.cmsg_type == daemon.v6pktinfo {
                    let mut p_1: C2rustUnnamed12 =
                        C2rustUnnamed12 {c: 0,};
                    p_1.c = cmptr.__cmsg_data.as_mut_ptr();
                    addr.in6.sin6_addr = (*p_1.p).ipi6_addr;
                    if_index = (*p_1.p).ipi6_ifindex
                }
                cmptr = __cmsg_nxthdr(&mut msg, cmptr)
            }
        }
        if indextoname(listen.tftpfd, if_index, namebuff.as_mut_ptr()) == 0
           {
            return
        }
        name = namebuff.as_mut_ptr();
        addra.addr4 = addr.in_0.sin_addr;
        if family == 10 { addra.addr6 = addr.in6.sin6_addr }
        if !daemon.tftp_interfaces.is_null() {
            /* dedicated tftp interface list */
            tmp = daemon.tftp_interfaces;
            while !tmp.is_null() {
                if !tmp.name.is_null() &&
                       wildcard_match(tmp.name, name) != 0 {
                    break ;
                }
                tmp = tmp.next
            }
            if tmp.is_null() { return }
        } else {
            /* Do the same as DHCP */
            if iface_check(family, &mut addra, name, 0) ==
                   0 {
                if daemon.options[(39 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                           ).wrapping_mul(8                                                     libc::c_int                                              ))
                                                 ] &
                       (1) <<
                           (39                   ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                       ).wrapping_mul(8                 libc::c_int          ))
                       == 0 {
                    enumerate_interfaces(0);
                }
                if loopback_exception(listen.tftpfd, family, &mut addra,
                                      name) == 0 &&
                       label_exception(if_index, family, &mut addra) == 0 {
                    return
                }
            }
            /* allowed interfaces are the same as for DHCP */
            tmp = daemon.dhcp_except;
            while !tmp.is_null() {
                if !tmp.name.is_null() &&
                       wildcard_match(tmp.name, name) != 0 {
                    return
                }
                tmp = tmp.next
            }
        }
        safe_strncpy(ifr.ifr_ifrn.ifrn_name.as_mut_ptr(), name,
                     16 );
        if ioctl(listen.tftpfd, 0x8921,
                 &mut ifr) != -(1) {
            mtu = ifr.ifr_ifru.ifru_mtu;
            if daemon.tftp_mtu != 0 &&
                   daemon.tftp_mtu < mtu {
                mtu = daemon.tftp_mtu
            }
        }
    }
    /* Failed to get interface mtu - can use configured value. */
    if mtu == 0 { mtu = daemon.tftp_mtu }
    /* data transfer via server listening socket */
    if daemon.options[(60).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (60).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 {
        let mut tftp_cnt: i32 = 0;
        tftp_cnt = 0;
        transfer = daemon.tftp_trans;
        up = &mut daemon.tftp_trans;
        while !transfer.is_null() {
            tftp_cnt += 1;
            if NetAddress_isequal(&mut peer, &mut transfer.peer) != 0 {
                if __bswap_16(*(packet ))
                       == 1 {
                    /* Handle repeated RRQ or abandoned transfer from same host and port 
		     by unlinking and reusing the struct transfer. */
                    *up = transfer.next;
                    break ;
                } else { handle_tftp(now, transfer, len); return }
            } else { up = &mut transfer.next; transfer = transfer.next }
        }
        /* Enforce simultaneous transfer limit. In non-single-port mode
	 this is doene by not listening on the server socket when
	 too many transfers are in progress. */
        if transfer.is_null() && tftp_cnt >= daemon.tftp_max {
            return
        }
    }
    if !name.is_null() {
        /* check for per-interface prefix */
        pref = daemon.if_prefix;
        while !pref.is_null() {
            if strcmp(pref.interface, name) == 0 {
                prefix = pref.prefix
            }
            pref = pref.next
        }
    }
    if family == 2 {
        addr.in_0.sin_port = __bswap_16(port)
    } else {
        addr.in6.sin6_port = __bswap_16(port);
        addr.in6.sin6_flowinfo = 0;
        addr.in6.sin6_scope_id = 0
    }
    /* May reuse struct transfer from abandoned transfer in single port mode. */
    if transfer.is_null() &&
           {
               transfer =
                   whine_malloc(::std::mem::size_of::<TftpTransfer>()                       ) ;
               transfer.is_null()
           } {
        return
    }
    if daemon.options[(60).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (60).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 {
        transfer.sockfd = listen.tftpfd
    } else {
        transfer.sockfd =
            socket(family, SOCK_DGRAM, 0);
        if transfer.sockfd == -(1) {
            free(transfer);
            return
        }
    }
    transfer.peer = peer;
    transfer.source = addra;
    transfer.if_index = if_index;
    transfer.timeout = now + 2;
    transfer.backoff = 1;
    transfer.block = 1;
    transfer.blocksize = 512;
    transfer.offset = 0 as off_t;
    transfer.file = 0 ;
    transfer.opt_transize = 0;
    transfer.opt_blocksize = transfer.opt_transize;
    transfer.carrylf = 0;
    transfer.netascii = transfer.carrylf;
    prettyprint_addr(&mut peer, daemon.addrbuff);
    /* if we have a nailed-down range, iterate until we find a free one. */
    while daemon.options[(60).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                         ).wrapping_mul(8                                   libc::c_int                            ))
                                        ] &
              (1) <<
                  (60 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     ).wrapping_mul(8
                                                                                                                            libc::c_int
                                                                                                                     ))
              == 0 {
        if !(bind(transfer.sockfd,
                  ConstNetAddressArg {__NetAddress__: &mut addr.sa,},
                  sa_len(&mut addr)) == -(1) ||
                 setsockopt(transfer.sockfd, IPPROTO_IP,
                            10,
                            &mut mtuflag as
                            ::std::mem::size_of::<libc::c_int>()                   ) ==
                     -(1) || fix_fd(transfer.sockfd) == 0) {
            break ;
        }
        if *__errno_location() == 98 &&
               daemon.start_tftp_port != 0 {
            port += 1;
            if port <= daemon.end_tftp_port {
                if family == 2 {
                    addr.in_0.sin_port = __bswap_16(port)
                } else { addr.in6.sin6_port = __bswap_16(port) }
                continue ;
            } else {
                my_syslog((1) << 3 |
                              3,
                          "unable to get free port for TFTP");
            }
        }
        free_transfer(transfer);
        return
    }
    p = packet.offset(2);
    end = packet.offset(len);
    if __bswap_16(*(packet )) !=
           1 ||
           { filename = next(&mut p, end); filename.is_null() } ||
           { mode = next(&mut p, end); mode.is_null() } ||
           strcasecmp(mode, "octet" )
               != 0 &&
               strcasecmp(mode,
                          "netascii" )
                   != 0 {
        len =
            tftp_err(4, packet,
                     "unsupported request from %s"                    *const libc::c_char ,
                     daemon.addrbuff);
        is_err = 1
    } else {
        if strcasecmp(mode,
                      "netascii" ) ==
               0 {
            transfer.netascii = 1
        }
        loop  {
            opt = next(&mut p, end);
            if opt.is_null() { break ; }
            if strcasecmp(opt,
                          "blksize" )
                   == 0 {
                opt = next(&mut p, end);
                if !opt.is_null() &&
                       daemon.options[(27   libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                   ).wrapping_mul(8                                                             libc::c_int                                                      ))
                                                     ] &
                           (1) <<
                               (27                       ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                               ).wrapping_mul(8                         libc::c_int                  ))
                           == 0 {
                    /* 32 bytes for IP, UDP and TFTP headers, 52 bytes for IPv6 */
                    let mut overhead: i32 =
                        if family == 2 {
                            32
                        } else { 52 };
                    transfer.blocksize = atoi(opt);
                    if transfer.blocksize <
                           1 {
                        transfer.blocksize =
                            1
                    }
                    if transfer.blocksize >
                           (daemon.packet_buff_sz                          libc::c_uint).wrapping_sub(4            libc::c_uint) {
                        transfer.blocksize =
                            (daemon.packet_buff_sz                           libc::c_uint).wrapping_sub(4
                                                                            libc::c_uint)
                    }
                    if mtu != 0 &&
                           transfer.blocksize >
                               (mtu                              libc::c_uint).wrapping_sub(overhead                libc::c_uint)
                       {
                        transfer.blocksize =
                            (mtu                           libc::c_uint).wrapping_sub(overhead             libc::c_uint)
                    }
                    transfer.opt_blocksize =
                        1;
                    transfer.block = 0
                }
            } else if strcasecmp(opt,
                                 "tsize"                                *const libc::c_char) == 0
                          && !next(&mut p, end).is_null() &&
                          transfer.netascii == 0 {
                transfer.opt_transize = 1;
                transfer.block = 0
            }
        }
        /* cope with backslashes from windows boxen. */
        p = filename;
        while *p != 0 {
            if *p == '\\' as i32 {
                *p = '/'
            } else if daemon.options[(38  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                 ).wrapping_mul(8                                                           libc::c_int                                                    ))
                                                    ] &
                          (1) <<
                              (38).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                             ).wrapping_mul(8                       libc::c_int                ))
                          != 0 {
                *p =
                    ({
                         let mut __res: i32 = 0;
                         if ::std::mem::size_of::<libc::c_char>()                          libc::c_ulong >
                                1 {
                             if 0 != 0 {
                                 let mut __c: i32 = *p;
                                 __res =
                                     if __c < -(128) ||
                                            __c > 255 {
                                         __c
                                     } else {
                                         *(*__ctype_tolower_loc()).offset(__c
                                                                  )
                                     }
                             } else { __res = tolowerp }
                         } else {
                             __res =
                                 *(*__ctype_tolower_loc()).offset(*p                   libc::c_int
                                                  )
                         }
                         __res
                     })
            }
            p = p.offset(1)
        }
        strcpy(daemon.namebuff,
               "/" );
        if !prefix.is_null() {
            if *prefix.offset(0) ==
                   '/' as i32 {
                *daemon.namebuff.offset(0)
                    = 0
            }
            strncat(daemon.namebuff, prefix,
                    ((1025 - 1)).wrapping_sub(strlen(daemon.namebuff)));
            if *prefix.offset(strlen(prefix).wrapping_sub(1    )
                                 ) != '/' as i32 {
                strncat(daemon.namebuff,
                        "/" ,
                        ((1025 - 1)).wrapping_sub(strlen(daemon.namebuff)));
            }
            if daemon.options[(29                                 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                   ).wrapping_mul(8                                             libc::c_int                                      ))
                                             ] &
                   (1) <<
                       (29 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                               ).wrapping_mul(8         libc::c_int  ))
                   != 0 {
                let mut oldlen: usize = strlen(daemon.namebuff);
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
                strncat(daemon.namebuff,
                        daemon.addrbuff,
                        ((1025 - 1)).wrapping_sub(strlen(daemon.namebuff)));
                strncat(daemon.namebuff,
                        "/" ,
                        ((1025 - 1)).wrapping_sub(strlen(daemon.namebuff)));
                /* remove unique-directory if it doesn't exist */
                if stat(daemon.namebuff, &mut statbuf) ==
                       -(1) ||
                       !(statbuf.st_mode &
                             0o170000 ==
                             0o40000) {
                    *daemon.namebuff.offset(oldlen) =
                        0
                }
            }
            if daemon.options[(56                                 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                   ).wrapping_mul(8                                             libc::c_int                                      ))
                                             ] &
                   (1) <<
                       (56 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                               ).wrapping_mul(8         libc::c_int  ))
                   != 0 {
                let mut macaddr: mut Vec<u8> = 0;
                let mut macbuf: [libc::c_uchar; 16] = [0; 16];
                if !daemon.dhcp.is_null() &&
                       peer.sa.sa_family == 2 {
                    /* Check if the client IP is in our lease database */
                    let mut lease: DhcpLease =
                        lease_find_by_addr(peer.in_0.sin_addr);
                    if !lease.is_null() &&
                           lease.hwaddr_type == 1 &&
                           lease.hwaddr_len == 6 {
                        macaddr = lease.hwaddr.as_mut_ptr()
                    }
                }
                /* If no luck, try to find in ARP table. This only works if client is in same (V)LAN */
                if macaddr.is_null() &&
                       find_mac(&mut peer, macbuf.as_mut_ptr(),
                                1, now) > 0 {
                    macaddr = macbuf.as_mut_ptr()
                }
                if !macaddr.is_null() {
                    let mut oldlen_0: usize =
                        strlen(daemon.namebuff);
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
                    snprintf(daemon.namebuff.offset(oldlen_0                isize),
                             ((1025 - 1)                     ).wrapping_sub(oldlen_0),
                             "%.2x-%.2x-%.2x-%.2x-%.2x-%.2x/"                           *const u8,
                             *macaddr.offset(0)                           libc::c_int,
                             *macaddr.offset(1)                           libc::c_int,
                             *macaddr.offset(2)                           libc::c_int,
                             *macaddr.offset(3)                           libc::c_int,
                             *macaddr.offset(4)                           libc::c_int,
                             *macaddr.offset(5)                           libc::c_int);
                    /* remove unique-directory if it doesn't exist */
                    if stat(daemon.namebuff, &mut statbuf_0) ==
                           -(1) ||
                           !(statbuf_0.st_mode &
                                 0o170000 ==
                                 0o40000) {
                        *daemon.namebuff.offset(oldlen_0)
                            = 0
                    }
                }
            }
            /* Absolute pathnames OK if they match prefix */
            if *filename.offset(0) ==
                   '/' as i32 {
                if strstr(filename, daemon.namebuff) == filename {
                    *daemon.namebuff.offset(0        isize) =
                        0
                } else { filename = filename.offset(1) }
            }
        } else if *filename.offset(0)
                      == '/' as i32 {
            *daemon.namebuff.offset(0) =
                0
        }
        strncat(daemon.namebuff, filename,
                ((1025 - 1)        ).wrapping_sub(strlen(daemon.namebuff)));
        /* check permissions and open file */
        transfer.file = check_tftp_fileperm(&mut len, prefix);
        if !transfer.file.is_null() {
            len = get_block(packet, transfer);
            if len == -(1) {
                len = tftp_err_oops(packet, daemon.namebuff)
            } else { is_err = 0 }
        }
    }
    send_from(transfer.sockfd,
              (daemon.options[(60                                 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                   ).wrapping_mul(8                                             libc::c_int                                      ))
                                             ] &
                   (1) <<
                       (60 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                               ).wrapping_mul(8         libc::c_int  ))
                   == 0), packet, len , &mut peer,
              &mut addra, if_index);
    if is_err != 0 {
        free_transfer(transfer);
    } else {
        transfer.next = daemon.tftp_trans;
        daemon.tftp_trans = transfer
    };
}
 fn check_tftp_fileperm(mut len: &mut isize,
                                         mut prefix: &mut String)
 -> TftpFile {
    let mut current_block: u64;
    let mut packet: &mut String = daemon.packet;
    let mut namebuff: &mut String = daemon.namebuff;
    let mut file: TftpFile = 0 ;
    let mut t: TftpTransfer = 0 ;
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
    let mut fd: i32 = -(1);
    /* trick to ban moving out of the subtree */
    if !(!prefix.is_null() &&
             !strstr(namebuff,
                     "/../"                    *const libc::c_char).is_null()) {
        fd = open(namebuff, 0);
        if fd == -(1) {
            if *__errno_location() == 2 {
                *len =
                    tftp_err(1, packet,
                             "file %s not found"                            *const libc::c_char ,
                             namebuff);
                return 0
            } else if *__errno_location() == 13 {
                current_block = 15533630919196144218;
            } else { current_block = 9018216499526184084; }
        } else if fstat(fd, &mut statbuf) == -(1) {
            current_block = 9018216499526184084;
        } else {
            /* stat the file descriptor to avoid stat->open races */
            /* running as root, must be world-readable */
            if uid == 0 {
                if statbuf.st_mode &
                       (0o400 >> 3 >>
                            3) == 0 {
                    current_block = 15533630919196144218;
                } else { current_block = 1054647088692577877; }
            } else if daemon.options[(26  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                 ).wrapping_mul(8                                                           libc::c_int                                                    ))
                                                    ] &
                          (1) <<
                              (26).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                             ).wrapping_mul(8                       libc::c_int                ))
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
                    t = daemon.tftp_trans;
                    while !t.is_null() {
                        if (*t.file).dev == statbuf.st_dev &&
                               (*t.file).inode == statbuf.st_ino &&
                               strcmp((*t.file).filename.as_mut_ptr(),
                                      namebuff) == 0 {
                            close(fd);
                            (*t.file).refcount += 1;
                            return t.file
                        }
                        t = t.next
                    }
                    file =
                        whine_malloc((::std::mem::size_of::<TftpFile>() ).wrapping_add(strlen(namebuff)).wrapping_add(1))
                            ;
                    if file.is_null() {
                        *__errno_location() = 12
                    } else {
                        file.fd = fd;
                        file.size = statbuf.st_size;
                        file.dev = statbuf.st_dev;
                        file.inode = statbuf.st_ino;
                        file.refcount = 1;
                        strcpy(file.filename.as_mut_ptr(), namebuff);
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
                if fd != -(1) { close(fd); }
                return 0
            }
        }
    }
    *__errno_location() = 13;
    *len =
        tftp_err(2, packet,
                 "cannot access %s: %s"                *const libc::c_char , namebuff);
    if fd != -(1) { close(fd); }
    return 0 ;
}
#[no_mangle]
pub  fn check_tftp_listeners(mut now: time::Instant) {
    let mut transfer: TftpTransfer = 0 ;
    let mut tmp: TftpTransfer = 0 ;
    let mut up: TftpTransfer;
    /* In single port mode, all packets come via port 69 and tftp_request() */
    if daemon.options[(60).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (60).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           == 0 {
        transfer = daemon.tftp_trans;
        while !transfer.is_null() {
            if poll_check(transfer.sockfd,
                          0x1 ) != 0 {
                /* we overwrote the buffer... */
                daemon.srv_save = 0;
                handle_tftp(now, transfer,
                            recv(transfer.sockfd,
                                 daemon.packet                              Vec<u8>,
                                 daemon.packet_buff_sz ,
                                 0));
            }
            transfer = transfer.next
        }
    }
    let mut current_block_32: u64;
    transfer = daemon.tftp_trans;
    up = &mut daemon.tftp_trans;
    while !transfer.is_null() {
        tmp = transfer.next;
        if difftime(now, transfer.timeout) >= 0.0f64 {
            let mut endcon: i32 = 0;
            let mut len: isize = 0;
            /* timeout, retransmit */
            transfer.timeout +=
                (1 +
                     ((1) <<
                          transfer.backoff / 2))              i32;
            /* we overwrote the buffer... */
            daemon.srv_save = 0;
            len = get_block(daemon.packet, transfer);
            if len == -(1) {
                len =
                    tftp_err_oops(daemon.packet,
                                  (*transfer.file).filename.as_mut_ptr());
                endcon = 1
            } else {
                transfer.backoff += 1;
                if transfer.backoff > 7 {
                    /* don't complain about timeout when we're awaiting the last
		 ACK, some clients never send it */
                    if len ==
                           transfer.blocksize.wrapping_add(4
                                                                                libc::c_uint)
                       {
                        endcon = 1
                    }
                    len = 0
                }
            }
            if len != 0 {
                send_from(transfer.sockfd,
                          (daemon.options[(60).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                           ).wrapping_mul(8
                                                                                                                           ))
                                                         ] &
                               (1) <<
                                   (60 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                       ).wrapping_mul(8))
                               == 0), daemon.packet,
                          len , &mut transfer.peer,
                          &mut transfer.source,
                          transfer.if_index);
            }
            if endcon != 0 || len == 0 {
                strcpy(daemon.namebuff,
                       (*transfer.file).filename.as_mut_ptr());
                sanitise(daemon.namebuff);
                prettyprint_addr(&mut transfer.peer,
                                 daemon.addrbuff);
                my_syslog((1) << 3 |
                              6,
                          if endcon != 0 {
                              "failed sending %s to %s"
                          } else {
                              "sent %s to %s"
                          }, daemon.namebuff,
                          daemon.addrbuff);
                /* unlink */
                *up = tmp;
                if endcon != 0 {
                    free_transfer(transfer);
                } else {
                    /* put on queue to be sent to script and deleted */
                    transfer.next = daemon.tftp_done_trans;
                    daemon.tftp_done_trans = transfer
                }
                current_block_32 = 14523784380283086299;
            } else { current_block_32 = 11385396242402735691; }
        } else { current_block_32 = 11385396242402735691; }
        match current_block_32 {
            11385396242402735691 => { up = &mut transfer.next }
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
 fn handle_tftp(mut now: time::Instant,
                                 mut transfer: &mut TftpTransfer,
                                 mut len: isize) {
    let mut mess: ack = daemon.packet ;
    if len >= ::std::mem::size_of::<ack>() {
        if __bswap_16(mess.op) == 4 &&
               __bswap_16(mess.block) ==
                   transfer.block  {
            /* Got ack, ensure we take the (re)transmit path */
            transfer.timeout = now;
            transfer.backoff = 0;
            let fresh6 = transfer.block;
            transfer.block = transfer.block.wrapping_add(1);
            if fresh6 != 0 {
                transfer.offset +=
                    transfer.blocksize.wrapping_sub(transfer.expansion)

            }
        } else if __bswap_16(mess.op) == 5 {
            let mut p: &mut String =
                daemon.packet.offset(::std::mem::size_of::<ack>()
              );
            let mut end: &mut String =
                daemon.packet.offset(len);
            let mut err: &mut String = next(&mut p, end);
            prettyprint_addr(&mut transfer.peer,
                             daemon.addrbuff);
            /* Sanitise error message */
            if err.is_null() {
                err =
                    ""                   &mut String
            } else { sanitise(err); }
            my_syslog((1) << 3 |
                          3,
                      "error %d %s received from %s",
                      __bswap_16(mess.block), err,
                      daemon.addrbuff);
            /* Got err, ensure we take abort */
            transfer.timeout = now;
            transfer.backoff = 100
        }
    };
}
 fn free_transfer(mut transfer: &mut TftpTransfer) {
    if daemon.options[(60).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (60).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           == 0 {
        close(transfer.sockfd);
    }
    if !transfer.file.is_null() &&
           {
               (*transfer.file).refcount -= 1;
               ((*transfer.file).refcount) == 0
           } {
        close((*transfer.file).fd);
        free(transfer.file);
    }
    free(transfer);
}
 fn next(mut p: String,
                          mut end: &mut String) -> &mut String {
    let mut ret: &mut String = *p;
    let mut len: usize = 0;
    if *end.offset(-(1)) !=
           0 || *p == end ||
           { len = strlen(ret); (len) == 0 } {
        return 0
    }
    *p =
        p.offset(len.wrapping_add(1)                  isize);
    return ret;
}
 fn sanitise(mut buf: &mut String) {
    let mut q: mut Vec<u8> = 0;
    let mut r: mut Vec<u8> = 0;
    r = buf;
    q = r;
    while *r != 0 {
        if *(*__ctype_b_loc()).offsetr  &
               _ISPRINT  != 0 {
            let fresh7 = q;
            q = q.offset(1);
            *fresh7 = *r
        }
        r = r.offset(1)
    }
    *q = 0;
}
/* limit to make packet < 512 bytes and definitely smaller than buffer */
 fn tftp_err(mut err: i32, mut packet: &mut String, mut message: &mut String, mut file: &mut String) -> isize {
    let mut mess: errmess = packet ; /* include terminating zero */
    let mut len: isize = 0;
    let mut ret: isize = 4;
    let mut errstr: &mut String = strerror(*__errno_location());
    
    sanitise(file);
    mess.op = __bswap_16(5);
    mess.err = __bswap_16(err);
    len =
        snprintf(mess.message.as_mut_ptr(),
                 500, message, file, errstr)
           ;
    ret +=
        if len < 500 {
            (len) + 1
        } else { 500 };
    my_syslog((1) << 3 | 3,
              "%s" ,
              mess.message.as_mut_ptr());
    return ret;
}
 fn tftp_err_oops(mut packet: &mut String,
                                   mut file: &mut String) -> isize {
    /* May have >1 refs to file, so potentially mangle a copy of the name */
    strcpy(daemon.namebuff, file);
    return tftp_err(0, packet,
                    "cannot read %s: %s"                   *const libc::c_char ,
                    daemon.namebuff);
}
/* return -1 for error, zero for done. */
 fn get_block(mut packet: &mut String, mut transfer: &mut TftpTransfer) -> isize {
    if transfer.block == 0 {
        /* send OACK */
        let mut p: &mut String = 0 ;
        let mut mess: oackmess = packet ;
        p = mess.data.as_mut_ptr();
        mess.op = __bswap_16(6);
        if transfer.opt_blocksize != 0 {
            p =
                p.offset((sprintf(p,
                                  "blksize") + 1)
                            );
            p =
                p.offset((sprintf(p,
                                  "%u",
                                  transfer.blocksize) + 1)
                            )
        }
        if transfer.opt_transize != 0 {
            p =
                p.offset((sprintf(p,
                                  "tsize") + 1)
                            );
            p =
                p.offset((sprintf(p,
                                  "%u",
                                  (*transfer.file).size) +
                              1))
        }
        return p.wrapping_offset_from(packet)
    } else {
        /* send data packet */
        let mut mess_0: datamess =
            packet ; /* finished */
        let mut size: usize =
            ((*transfer.file).size - transfer.offset) ;
        if transfer.offset > (*transfer.file).size {
            return 0
        }
        if size > transfer.blocksize {
            size = transfer.blocksize
        }
        mess_0.op = __bswap_16(3);
        mess_0.block = __bswap_16(transfer.block );
        if lseek((*transfer.file).fd, transfer.offset, 0)
               == -(1) as off_t ||
               read_write((*transfer.file).fd, mess_0.data.as_mut_ptr(),
                          size, 1) == 0 {
            return -(1)
        }
        transfer.expansion = 0;
        /* Map '\n' to CR-LF in netascii mode */
        if transfer.netascii != 0 {
            let mut i: usize =
                0; /* don't expand LF again if it moves to the next block */
            let mut newcarrylf: i32 = 0; /* room in this block */
            i = 0 ;
            newcarrylf = 0;
            while i < size {
                if *mess_0.data.as_mut_ptr().offset(i) == '\n' as i32 &&
                       (i != 0 ||
                            transfer.carrylf == 0) {
                    transfer.expansion =
                        transfer.expansion.wrapping_add(1);
                    if size != transfer.blocksize {
                        size = size.wrapping_add(1)
                    } else if i ==
                                  size.wrapping_sub(1     libc::c_ulong) {
                        newcarrylf = 1
                    }
                    /* make space and insert CR */
                    memmove(&mut *mess_0.data.as_mut_ptr().offset(i.wrapping_add(1
                                                                                                                            libc::c_int
                                                                                                                     )
                                                        )
                               ,
                            &mut *mess_0.data.as_mut_ptr().offset(i                      isize)
                               ,
                            size.wrapping_sub(i.wrapping_add(1
                                                                       )));
                    *mess_0.data.as_mut_ptr().offset(i) =
                        '\r' as i32;
                    i = i.wrapping_add(1)
                }
                i = i.wrapping_add(1)
            }
            transfer.carrylf = newcarrylf
        }
        return size.wrapping_add(4)
    };
}
#[no_mangle]
pub  fn do_tftp_script_run() -> i32 {
    let mut transfer: TftpTransfer = 0 ;
    transfer = daemon.tftp_done_trans;
    if !transfer.is_null() {
        daemon.tftp_done_trans = transfer.next;
        queue_tftp((*transfer.file).size,
                   (*transfer.file).filename.as_mut_ptr(),
                   &mut transfer.peer);
        free_transfer(transfer);
        return 1
    }
    return 0;
}
