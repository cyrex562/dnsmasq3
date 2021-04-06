use crate::defines::{iovec, socklen_t, KernelSaFamily, __u32, DnsmasqDaemon, SOCK_RAW, ConstNetAddressArg, NetAddress, NetAddressArg, size_t, ssize_t, MsgHdr, MSG_PEEK, MSG_TRUNC, C2rustUnnamed10, NetAddress, __bswap_32, InAddrT, In6Addr};
use crate::slack::{NetAddress_nl, nlmsghdr, rtgenmsg, RTM_GETNEIGH, RTM_GETLINK, RTM_GETADDR, RTM_NEWADDR, ifaddrmsg, rtattr, IFA_LOCAL, IFA_BROADCAST, IFA_LABEL, IFA_ADDRESS, IFA_CACHEINFO, ifa_cacheinfo, RTM_NEWNEIGH, ndmsg, NDA_DST, NDA_LLADDR, RTM_NEWLINK, ifinfomsg, IFLA_ADDRESS, IFF_LOOPBACK, IFF_POINTOPOINT, nlmsgerr, RTM_NEWROUTE, rtmsg, RTN_UNICAST, RT_SCOPE_LINK, RT_TABLE_MAIN, RT_TABLE_LOCAL, RTM_DELADDR};
use crate::dnsmasq_log::{die, my_syslog};
use crate::util::{expand_buf, retry_send};
use std::thread::sleep;
use crate::queue_event;

static mut iov: iovec =
    iovec{iov_base: 0,
          iov_len: 0,};
static mut netlink_pid: u32 = 0;

pub fn netlink_init(daemon: &mut DnsmasqDaemon) -> String {
    let mut addr: NetAddress_nl =
        NetAddress_nl{nl_family: 16, nl_pad: 0, nl_pid: 0, nl_groups: 0x40,};
    let mut slen: socklen_t = ::std::mem::size_of::<NetAddress_nl>();
    let mut opt: i32 = 1;
    if daemon.options[(39).wrapping_div((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8)) ] &
           (1) <<
               (39).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 {
        addr.nl_groups |= 0x10
    }
    addr.nl_groups |= 0x400;
    if daemon.options[(39).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (39).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 {
        addr.nl_groups |= 0x100
    }
    if daemon.ra_enabled != 0 || daemon.dhcp6_enabled != 0 {
        addr.nl_groups |= 0x100
    }
    /* May not be able to have permission to set multicast groups don't die in that case */
    daemon.netlinkfd =
        socket(16, SOCK_RAW, 0);
    if daemon.netlinkfd != -(1) {
        if bind(daemon.netlinkfd,
                ConstNetAddressArg {__NetAddress__:
                                         &mut addr_nl                                       NetAddress,},
                ::std::mem::size_of::<NetAddress_nl>()              socklen_t) == -(1) {
            addr.nl_groups = 0 ;
            if *__errno_location() != 1 ||
                   bind(daemon.netlinkfd,
                        ConstNetAddressArg {__NetAddress__:
                                                 &mut addr_nl
                                                    ,},
                        ::std::mem::size_of::<NetAddress_nl>()
                           ) == -(1) {
                daemon.netlinkfd = -(1)
            }
        }
    }
    if daemon.netlinkfd == -(1) ||
           getsockname(daemon.netlinkfd,
                       NetAddressArg {__NetAddress__:
                                          &mut addr_nl                                        NetAddress,}, &mut slen) ==
               -(1) {
        die("cannot create netlink socket: %s",
            0 , 5);
    }
    /* save pid assigned by bind() and retrieved by getsockname() */
    netlink_pid = addr.nl_pid;
    iov.iov_len = 100 ;
    // iov.iov_base = safe_malloc(iov.iov_len);
    if daemon.kernel_version >=
           ((2) << 16) +
               ((6) << 8) + 30 &&
           setsockopt(daemon.netlinkfd, 270,
                      5,
                      &mut opt,
                      ::std::mem::size_of::<libc::c_int>()) == -(1) {
        return "warning: failed to set NETLINK_NO_ENOBUFS on netlink socket"

    }
    return 0 ;
}
unsafe extern "C" fn netlink_recv() -> ssize_t {
    let mut msg: MsgHdr =
        MsgHdr {msg_name: 0,
               msg_namelen: 0,
               msg_iov: 0,
               msg_iovlen: 0,
               msg_control: 0,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut nladdr: NetAddress_nl =
        NetAddress_nl{nl_family: 0, nl_pad: 0, nl_pid: 0, nl_groups: 0,};
    let mut rc: ssize_t = 0;
    loop  {
        msg.msg_control = 0;
        msg.msg_controllen = 0 ;
        msg.msg_name = &mut nladdr_nl;
        msg.msg_namelen =
            ::std::mem::size_of::<NetAddress_nl>() ;
        msg.msg_iov = &mut iov;
        msg.msg_iovlen = 1 ;
        msg.msg_flags = 0;
        loop  {
            rc =
                recvmsg(daemon.netlinkfd, &mut msg,
                        MSG_PEEK | MSG_TRUNC);
            if !(rc == -(1) &&
                     *__errno_location() == 4) {
                break ;
            }
        }
        /* make buffer big enough */
        if rc != -(1) &&
               msg.msg_flags & MSG_TRUNC != 0 {
            /* Very new Linux kernels return the actual size needed, older ones always return truncated size */
            if rc  == iov.iov_len {
                if expand_buf(&mut iov,
                              (rc + 100)                            size_t) != 0 {
                    continue ;
                }
            } else { expand_buf(&mut iov, rc ); }
        }
        /* read it for real */
        msg.msg_flags = 0;
        loop  {
            rc =
                recvmsg(daemon.netlinkfd, &mut msg,
                        0);
            if !(rc == -(1) &&
                     *__errno_location() == 4) {
                break ;
            }
        }
        /* Make sure this is from the kernel */
        if rc == -(1) ||
               nladdr.nl_pid == 0 {
            break ;
        }
    }
    /* discard stuff which is truncated at this point (expand_buf() may fail) */
    if msg.msg_flags & MSG_TRUNC != 0 {
        rc = -(1) as ssize_t;
        *__errno_location() = 12
    }
    return rc;
}
/* family = AF_UNSPEC finds ARP table entries.
   family = AF_LOCAL finds MAC addresses. */

pub fn iface_enumerate(mut family: i32, mut parm:Vec<u8>, mut callback: Option<fn()->i32>)
 -> i32 {
    let mut addr: NetAddress_nl =
        NetAddress_nl{nl_family: 0, nl_pad: 0, nl_pid: 0, nl_groups: 0,};
    let mut h: nlmsghdr = Deafult::default();
    let mut len: ssize_t = 0;
    let mut seq: u32 = 0;
    let mut callback_ok: i32 = 1;
    let mut req: C2rustUnnamed10 = Default::default();
    addr.nl_family = 16 ;
    loop  {
        if family == 0 {
            req.nlh.nlmsg_type = RTM_GETNEIGH
        } else if family == 1 {
            req.nlh.nlmsg_type = RTM_GETLINK
        } else { req.nlh.nlmsg_type = RTM_GETADDR }
        req.nlh.nlmsg_len =
            ::std::mem::size_of::<C2rustUnnamed10>();
        req.nlh.nlmsg_flags =
            (0x100 | 0x200 | 0x1 | 0x4);
        req.nlh.nlmsg_pid = 0 ;
        seq = seq.wrapping_add(1);
        req.nlh.nlmsg_seq = seq;
        req.g.rtgen_family = family;
        /* Don't block in recvfrom if send fails */
        while retry_send(sendto(daemon.netlinkfd,
                                &mut req ,
                                ::std::mem::size_of::<C2rustUnnamed10>() , 0,
                                ConstNetAddressArg {__NetAddress__:
                                                         &mut addr          NetAddress_nl
                                                                      NetAddress,},
                                ::std::mem::size_of::<NetAddress_nl>()                       )) != 0 {
        }
        if *__errno_location() != 0 { return 0 }
        loop  {
            len = netlink_recv();
            if len == -(1) { break ; }
            h = iov.iov_base );
            while len  >=
                      ::std::mem::size_of::<nlmsghdr>() &&
                      h.nlmsg_len >=
                          ::std::mem::size_of::<nlmsghdr>()
                      && h.nlmsg_len <= len  {
                if h.nlmsg_pid != netlink_pid ||
                       h.nlmsg_type == 0x2 {
                    /* May be multicast arriving async */
                    nl_async(h);
                } else if !(h.nlmsg_seq != seq) {
                    if h.nlmsg_type == 0x3 {
                        return callback_ok
                    } else {
                        if h.nlmsg_type ==
                               RTM_NEWADDR &&
                               family != 0 &&
                               family != 1 {
                            let mut ifa: mut ifaddrmsg = h.offset(0 + ::std::mem::size_of::<nlmsghdr>());
                            let mut rta: mut rtattr = ifa.offset(::std::mem::size_of::<ifaddrmsg>());
                            let mut len1: u32 = h.nlmsg_len;
                            if ifa.ifa_family == family {
                                if ifa.ifa_family == 2 {
                                    let mut netmask: NetAddress = NetAddress {s_addr: 0,};
                                    let mut addr_0: NetAddress = NetAddress {s_addr: 0,};
                                    let mut broadcast: NetAddress = NetAddress {s_addr: 0,};
                                    let mut label: &mut String = 0 ;
                                    netmask.s_addr = 1 << 32 - ifa.ifa_prefixlen;
                                    addr_0.s_addr =
                                        0;
                                    broadcast.s_addr =
                                        0;
                                    while len1 >=
                                              ::std::mem::size_of::<rtattr>()
                                                                                             libc::c_int
                                              &&
                                              rta.rta_len
                                                  >=
                                                  ::std::mem::size_of::<rtattr>()
                                                      &&
                                              rta.rta_len
                                                  <= len1 {
                                        if rta.rta_type ==
                                               IFA_LOCAL {
                                            addr_0 =
                                                *(rta.offset(1
                                                                )   NetAddress)
                                        } else if rta.rta_type   libc::c_int ==
                                                      IFA_BROADCAST       libc::c_int {
                                            broadcast =
                                                *(rta.offset(1
                                                                )   NetAddress)
                                        } else if rta.rta_type   libc::c_int ==
                                                      IFA_LABEL
                                         {
                                            label =
                                                (rta  &mut String).offset(((::std::mem::size_of::<rtattr>()
                                                                                                               ).wrapping_add(4                                                         libc::c_uint                                                  ).wrapping_sub(1                                                                                                                         libc::c_int                                                                                                                  )
                                                                                    &
                                                                                    !(4   libc::c_uint).wrapping_sub(1                                                                 libc::c_int                                                                 libc::c_uint)
                                                                                                                     ).wrapping_add(0                                                               libc::c_int                                                        )
                                                                            )
                                                    &mut String
                                        }
                                        len1 =
                                            len1.wrapping_sub((rta.rta_len
                                                                                  libc::c_uint).wrapping_add(4                   libc::c_uint).wrapping_sub(1                                                                                 libc::c_int                                                                                 libc::c_uint)
                                                                  &
                                                                  !(4                     libc::c_uint).wrapping_sub(1                             libc::c_int                             libc::c_uint));
                                        rta =
                                            (rta                                           &mut String).offset(((rta.rta_len
                                                                                                              libc::c_uint).wrapping_add(4                                               libc::c_uint).wrapping_sub(1                                                                                                             libc::c_int                                                                                                             libc::c_uint)
                                                                                &
                                                                                !(4
                                                                                                                        libc::c_uint).wrapping_sub(1                                                         libc::c_int                                                         libc::c_uint))
                                                                    )

                                    }
                                    if addr_0.s_addr != 0 && callback_ok != 0
                                       {
                                        if ::std::mem::transmute::<_,
                                                                   fn(_: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _)
                                                                       ->
                                                                           libc::c_int>(Some(callback.expect("non-null function pointer")).expect("non-null function pointer"))(addr_0,                                                   ifa.ifa_index,                                                   label,                                                   netmask,                                                   broadcast,                                                   parm)
                                               == 0 {
                                            callback_ok = 0
                                        }
                                    }
                                } else if ifa.ifa_family ==
                                              10 {
                                    let mut addrp: In6Addr =
                                        0;
                                    let mut valid: u32 =
                                        0;
                                    let mut preferred: u32 =
                                        0;
                                    let mut flags: i32 =
                                        0;
                                    while len1 >=
                                              ::std::mem::size_of::<rtattr>()
                                                                                             libc::c_int
                                              &&
                                              rta.rta_len
                                                  >=
                                                  ::std::mem::size_of::<rtattr>()
                                                      &&
                                              rta.rta_len
                                                  <= len1 {
                                        if rta.rta_type ==
                                               IFA_ADDRESS {
                                            addrp =
                                                rta.offset(1            isize) &mut In6Addr
                                        } else if rta.rta_type   libc::c_int ==
                                                      IFA_CACHEINFO       libc::c_int {
                                            let mut ifc: ifa_cacheinfo =
                                                rta.offset(1            isize) ifa_cacheinfo;
                                            preferred = ifc.ifa_prefered;
                                            valid = ifc.ifa_valid
                                        }
                                        len1 =
                                            len1.wrapping_sub((rta.rta_len
                                                                                  libc::c_uint).wrapping_add(4                   libc::c_uint).wrapping_sub(1                                                                                 libc::c_int                                                                                 libc::c_uint)
                                                                  &
                                                                  !(4                     libc::c_uint).wrapping_sub(1                             libc::c_int                             libc::c_uint));
                                        rta =
                                            (rta                                           &mut String).offset(((rta.rta_len
                                                                                                              libc::c_uint).wrapping_add(4                                               libc::c_uint).wrapping_sub(1                                                                                                             libc::c_int                                                                                                             libc::c_uint)
                                                                                &
                                                                                !(4
                                                                                                                        libc::c_uint).wrapping_sub(1                                                         libc::c_int                                                         libc::c_uint))
                                                                    )

                                    }
                                    if ifa.ifa_flags &
                                           0x40 != 0 {
                                        flags |= 1
                                    }
                                    if ifa.ifa_flags &
                                           0x20 != 0 {
                                        flags |= 2
                                    }
                                    if ifa.ifa_flags &
                                           0x1 == 0 {
                                        flags |= 4
                                    }
                                    if !addrp.is_null() && callback_ok != 0 {
                                        if ::std::mem::transmute::<_,
                                                                   fn(_: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _,
                                                                      _: _)
                                                                       ->
                                                                           libc::c_int>(Some(callback.expect("non-null function pointer")).expect("non-null function pointer"))(addrp,                                                   ifa.ifa_prefixlen                                                                                                                                                                                       libc::c_int,                                                   ifa.ifa_scope                                                                                                                                                                                       libc::c_int,                                                   ifa.ifa_index                                                                                                                                                                                       libc::c_int,                                                   flags,                                                   preferred                                                                                                                                                                                       libc::c_int,                                                   valid                                                                                                                                                                                       libc::c_int,                                                   parm)
                                               == 0 {
                                            callback_ok = 0
                                        }
                                    }
                                }
                            }
                        } else if h.nlmsg_type ==
                                      RTM_NEWNEIGH &&
                                      family == 0 {
                            let mut neigh: ndmsg =
                                (h                               &mut String).offset((0                 libc::c_int
                                                                    +
                                                                    ((::std::mem::size_of::<nlmsghdr>()
                                                                                         ).wrapping_add(4                                   libc::c_uint                            ).wrapping_sub(1                                                                                                   libc::c_int                                                                                            )
                                                                         &
                                                                         !(4
                                                                                                          libc::c_uint).wrapping_sub(1                                           libc::c_int                                           libc::c_uint)
                                                                                               )
                                                                                            libc::c_int)
                                                                  )
                                    ;
                            let mut rta_0: rtattr =
                                (neigh                               &mut String).offset(((::std::mem::size_of::<ndmsg>()
                                                                               ).wrapping_add(4                         libc::c_uint                  ).wrapping_sub(1                                                                                         libc::c_int                                                                                  )
                                                                    &
                                                                    !(4                       libc::c_uint).wrapping_sub(1                                 libc::c_int                                 libc::c_uint)
                                                                                     )
                                                                  )
                                    ;
                            let mut len1_0: u32 =
                                (h.nlmsg_len                        ).wrapping_sub((::std::mem::size_of::<ndmsg>()
                                                                                 ).wrapping_add(((::std::mem::size_of::<nlmsghdr>()                        ).wrapping_add(4                                                                                               libc::c_uint                                                                                        ).wrapping_sub(1                                                                                                                                                               libc::c_int                                                                                                                                                        )
                                                                                                       &
                                                                                                       !(4                                         libc::c_uint).wrapping_sub(1                                                                                                       libc::c_int                                                                                                       libc::c_uint)                              )                           libc::c_int                    ))
                                   ;
                            let mut maclen: usize =
                                0 ;
                            let mut inaddr: &mut String =
                                0 ;
                            let mut mac: &mut String =
                                0 ;
                            while len1_0 >=
                                      ::std::mem::size_of::<rtattr>()                                     libc::c_uint &&
                                      rta_0.rta_len >=
                                          ::std::mem::size_of::<rtattr>()                                        libc::c_ulong &&
                                      rta_0.rta_len <=
                                          len1_0 {
                                if rta_0.rta_type ==
                                       NDA_DST {
                                    inaddr =
                                        rta_0.offset(1      isize)                                      &mut String
                                } else if rta_0.rta_type ==
                                              NDA_LLADDR {
                                    maclen =
                                        (rta_0.rta_len ).wrapping_sub(::std::mem::size_of::<rtattr>()
                                                                                               );
                                    mac =
                                        rta_0.offset(1      isize)                                      &mut String
                                }
                                len1_0 =
                                    len1_0.wrapping_sub((rta_0.rta_len          libc::c_uint).wrapping_add(4       libc::c_uint).wrapping_sub(1                                                                     libc::c_int                                                                     libc::c_uint)
                                                            &
                                                            !(4               libc::c_uint).wrapping_sub(1                 libc::c_int                 libc::c_uint));
                                rta_0 =
                                    (rta_0                                   &mut String).offset(((rta_0.rta_len
                                                                                              libc::c_uint).wrapping_add(4                               libc::c_uint).wrapping_sub(1                                                                                             libc::c_int                                                                                             libc::c_uint)
                                                                        &
                                                                        !(4                           libc::c_uint).wrapping_sub(1                                         libc::c_int                                         libc::c_uint))
                                                    )

                            }
                            if neigh.ndm_state &
                                   (0x40 | 0x1 |
                                        0x20) == 0 &&
                                   !inaddr.is_null() && !mac.is_null() &&
                                   callback_ok != 0 {
                                if ::std::mem::transmute::<_,
                                                           fn(_: _, _: _,
                                                              _: _, _: _,
                                                              _: _)
                                                               ->
                                                                   libc::c_int>(Some(callback.expect("non-null function pointer")).expect("non-null function pointer"))(neigh.ndm_family                                                                                                                                                                       libc::c_int,                                           inaddr,                                           mac,                                           maclen,                                           parm)
                                       == 0 {
                                    callback_ok = 0
                                }
                            }
                        } else if h.nlmsg_type ==
                                      RTM_NEWLINK &&
                                      family == 1 {
                            let mut link: ifinfomsg =
                                (h                               &mut String).offset((0                 libc::c_int
                                                                    +
                                                                    ((::std::mem::size_of::<nlmsghdr>()
                                                                                         ).wrapping_add(4                                   libc::c_uint                            ).wrapping_sub(1                                                                                                   libc::c_int                                                                                            )
                                                                         &
                                                                         !(4
                                                                                                          libc::c_uint).wrapping_sub(1                                           libc::c_int                                           libc::c_uint)
                                                                                               )
                                                                                            libc::c_int)
                                                                  )
                                    ;
                            let mut rta_1: rtattr =
                                (link                               &mut String).offset(((::std::mem::size_of::<ifinfomsg>()
                                                                               ).wrapping_add(4                         libc::c_uint                  ).wrapping_sub(1                                                                                         libc::c_int                                                                                  )
                                                                    &
                                                                    !(4                       libc::c_uint).wrapping_sub(1                                 libc::c_int                                 libc::c_uint)
                                                                                     )
                                                                  )
                                    ;
                            let mut len1_1: u32 =
                                (h.nlmsg_len                        ).wrapping_sub((::std::mem::size_of::<ifinfomsg>()
                                                                                 ).wrapping_add(((::std::mem::size_of::<nlmsghdr>()                        ).wrapping_add(4                                                                                               libc::c_uint                                                                                        ).wrapping_sub(1                                                                                                                                                               libc::c_int                                                                                                                                                        )
                                                                                                       &
                                                                                                       !(4                                         libc::c_uint).wrapping_sub(1                                                                                                       libc::c_int                                                                                                       libc::c_uint)                              )                           libc::c_int                    ))
                                   ;
                            let mut mac_0: &mut String =
                                0 ;
                            let mut maclen_0: usize =
                                0 ;
                            while len1_1 >=
                                      ::std::mem::size_of::<rtattr>()                                     libc::c_uint &&
                                      rta_1.rta_len >=
                                          ::std::mem::size_of::<rtattr>()                                        libc::c_ulong &&
                                      rta_1.rta_len <=
                                          len1_1 {
                                if rta_1.rta_type ==
                                       IFLA_ADDRESS {
                                    maclen_0 =
                                        (rta_1.rta_len ).wrapping_sub(::std::mem::size_of::<rtattr>()
                                                                                               );
                                    mac_0 =
                                        rta_1.offset(1      isize)                                      &mut String
                                }
                                len1_1 =
                                    len1_1.wrapping_sub((rta_1.rta_len          libc::c_uint).wrapping_add(4       libc::c_uint).wrapping_sub(1                                                                     libc::c_int                                                                     libc::c_uint)
                                                            &
                                                            !(4               libc::c_uint).wrapping_sub(1                 libc::c_int                 libc::c_uint));
                                rta_1 =
                                    (rta_1                                   &mut String).offset(((rta_1.rta_len
                                                                                              libc::c_uint).wrapping_add(4                               libc::c_uint).wrapping_sub(1                                                                                             libc::c_int                                                                                             libc::c_uint)
                                                                        &
                                                                        !(4                           libc::c_uint).wrapping_sub(1                                         libc::c_int                                         libc::c_uint))
                                                    )

                            }
                            if !mac_0.is_null() && callback_ok != 0 &&
                                   link.ifi_flags &
                                       (IFF_LOOPBACK |
                                            IFF_POINTOPOINT)                                     libc::c_uint == 0 &&
                                   ::std::mem::transmute::<_,
                                                           fn(_: _, _: _,
                                                              _: _, _: _,
                                                              _: _)
                                                               ->
                                                                   libc::c_int>(Some(callback.expect("non-null function pointer")).expect("non-null function pointer"))(link.ifi_index,                                           link.ifi_type                                                                                                                                                                       libc::c_uint,                                           mac_0,                                           maclen_0,                                           parm)
                                       == 0 {
                                callback_ok = 0
                            }
                        }
                    }
                }
                /* May be part of incomplete response to previous request after
	       ENOBUFS. Drop it. */
                len -=
                    (h.nlmsg_len.wrapping_add(4  libc::c_uint).wrapping_sub(1
                                                                                                                    libc::c_int
                                                                                                                    libc::c_uint)
                         &
                         !(4                         libc::c_uint).wrapping_sub(1           libc::c_uint))
                       ;
                h =
                    (h                   &mut String).offset((h.nlmsg_len.wrapping_add(4
                                                                                                                    libc::c_uint).wrapping_sub(1                                                     libc::c_int                                                     libc::c_uint)
                                                        &
                                                        !(4           libc::c_uint).wrapping_sub(1         libc::c_int         libc::c_uint))
                                                      )                  &mut nlmsghdr
            }
        }
        if !(*__errno_location() == 105) { break ; }
        sleep(1);
    }
    return 0;
}

pub fn netlink_multicast() {
    let mut len: ssize_t = 0;
    let mut h: nlmsghdr = 0 );
    let mut flags: i32 = 0;
    /* don't risk blocking reading netlink messages here. */
    flags = fcntl(daemon.netlinkfd, 3);
    if flags == -(1) ||
           fcntl(daemon.netlinkfd, 4,
                 flags | 0o4000) == -(1) {
        return
    }
    len = netlink_recv();
    if len != -(1) {
        h = iov.iov_base );
        while len  >=
                  ::std::mem::size_of::<nlmsghdr>()  &&
                  h.nlmsg_len >=
                      ::std::mem::size_of::<nlmsghdr>() &&
                  h.nlmsg_len <= len  {
            nl_async(h);
            len -=
                (h.nlmsg_len.wrapping_add(4                                           libc::c_uint).wrapping_sub(1
                                                                                                            libc::c_int
                                                                                                            libc::c_uint)
                     &
                     !(4                     libc::c_uint).wrapping_sub(1       libc::c_uint))              i32;
            h =
                (h               &mut String).offset((h.nlmsg_len.wrapping_add(4
                                                                                                            libc::c_uint).wrapping_sub(1                                             libc::c_int                                             libc::c_uint)
                                                    &
                                                    !(4       libc::c_uint).wrapping_sub(1 libc::c_int libc::c_uint))
                                                  ) )
        }
    }
    /* restore non-blocking status */
    fcntl(daemon.netlinkfd, 4, flags);
}
unsafe extern "C" fn nl_async(mut h: &mut nlmsghdr) {
    if h.nlmsg_type == 0x2 {
        let mut err: nlmsgerr =
            (h           &mut String).offset((0 +
                                                ((::std::mem::size_of::<nlmsghdr>()
                                                 ).wrapping_add(4
                                                                                                                        libc::c_uint
                                                                                                                 ).wrapping_sub(1                                                           libc::c_int                                                    )
                                                     &
                                                     !(4        libc::c_uint).wrapping_sub(1   libc::c_int   libc::c_uint)
                                                        ) libc::c_int))         Vec<u8> ;
        if err.error != 0 {
            my_syslog(3,
                      "netlink returns error: %s", strerror(-err.error));
        }
    } else if h.nlmsg_pid == 0 &&
                  h.nlmsg_type ==
                      RTM_NEWROUTE {
        /* We arrange to receive netlink multicast messages whenever the network route is added.
	 If this happens and we still have a DNS packet in the buffer, we re-send it.
	 This helps on DoD links, where frequently the packet which triggers dialling is
	 a DNS query, which then gets lost. By re-sending, we can avoid the lookup
	 failing. */
        let mut rtm: rtmsg =
            (h           &mut String).offset((0 +
                                                ((::std::mem::size_of::<nlmsghdr>()
                                                 ).wrapping_add(4
                                                                                                                        libc::c_uint
                                                                                                                 ).wrapping_sub(1                                                           libc::c_int                                                    )
                                                     &
                                                     !(4        libc::c_uint).wrapping_sub(1   libc::c_int   libc::c_uint)
                                                        ) libc::c_int)) ;
        if rtm.rtm_type == RTN_UNICAST &&
               rtm.rtm_scope == RT_SCOPE_LINK
               &&
               (rtm.rtm_table ==
                    RT_TABLE_MAIN ||
                    rtm.rtm_table ==
                        RT_TABLE_LOCAL) {
            queue_event(23);
        }
    } else if h.nlmsg_type == RTM_NEWADDR ||
                  h.nlmsg_type == RTM_DELADDR
     {
        queue_event(22);
    };
}
