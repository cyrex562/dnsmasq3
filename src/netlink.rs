
static mut iov: iovec =
    iovec{iov_base: 0 as *const libc::c_void as *mut libc::c_void,
          iov_len: 0,};
static mut netlink_pid: u32_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn netlink_init() -> *mut libc::c_char {
    let mut addr: sockaddr_nl =
        sockaddr_nl{nl_family: 0, nl_pad: 0, nl_pid: 0, nl_groups: 0,};
    let mut slen: socklen_t =
        ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as socklen_t;
    let mut opt: libc::c_int = 1 as libc::c_int;
    addr.nl_family = 16 as libc::c_int as __kernel_sa_family_t;
    addr.nl_pad = 0 as libc::c_int as libc::c_ushort;
    addr.nl_pid = 0 as libc::c_int as __u32;
    addr.nl_groups = 0x40 as libc::c_int as __u32;
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
           != 0 {
        addr.nl_groups |= 0x10 as libc::c_int as libc::c_uint
    }
    addr.nl_groups |= 0x400 as libc::c_int as libc::c_uint;
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
           != 0 {
        addr.nl_groups |= 0x100 as libc::c_int as libc::c_uint
    }
    if (*dnsmasq_daemon).doing_ra != 0 || (*dnsmasq_daemon).doing_dhcp6 != 0 {
        addr.nl_groups |= 0x100 as libc::c_int as libc::c_uint
    }
    /* May not be able to have permission to set multicast groups don't die in that case */
    (*dnsmasq_daemon).netlinkfd =
        socket(16 as libc::c_int, SOCK_RAW as libc::c_int, 0 as libc::c_int);
    if (*dnsmasq_daemon).netlinkfd != -(1 as libc::c_int) {
        if bind((*dnsmasq_daemon).netlinkfd,
                __CONST_SOCKADDR_ARG{__sockaddr__:
                                         &mut addr as *mut sockaddr_nl as
                                             *mut sockaddr,},
                ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as
                    socklen_t) == -(1 as libc::c_int) {
            addr.nl_groups = 0 as libc::c_int as __u32;
            if *__errno_location() != 1 as libc::c_int ||
                   bind((*dnsmasq_daemon).netlinkfd,
                        __CONST_SOCKADDR_ARG{__sockaddr__:
                                                 &mut addr as *mut sockaddr_nl
                                                     as *mut sockaddr,},
                        ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong
                            as socklen_t) == -(1 as libc::c_int) {
                (*dnsmasq_daemon).netlinkfd = -(1 as libc::c_int)
            }
        }
    }
    if (*dnsmasq_daemon).netlinkfd == -(1 as libc::c_int) ||
           getsockname((*dnsmasq_daemon).netlinkfd,
                       __SOCKADDR_ARG{__sockaddr__:
                                          &mut addr as *mut sockaddr_nl as
                                              *mut sockaddr,}, &mut slen) ==
               -(1 as libc::c_int) {
        die(b"cannot create netlink socket: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 5 as libc::c_int);
    }
    /* save pid assigned by bind() and retrieved by getsockname() */
    netlink_pid = addr.nl_pid;
    iov.iov_len = 100 as libc::c_int as size_t;
    iov.iov_base = safe_malloc(iov.iov_len);
    if (*dnsmasq_daemon).kernel_version >=
           ((2 as libc::c_int) << 16 as libc::c_int) +
               ((6 as libc::c_int) << 8 as libc::c_int) + 30 as libc::c_int &&
           setsockopt((*dnsmasq_daemon).netlinkfd, 270 as libc::c_int,
                      5 as libc::c_int,
                      &mut opt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) == -(1 as libc::c_int) {
        return b"warning: failed to set NETLINK_NO_ENOBUFS on netlink socket\x00"
                   as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn netlink_recv() -> ssize_t {
    let mut msg: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut nladdr: sockaddr_nl =
        sockaddr_nl{nl_family: 0, nl_pad: 0, nl_pid: 0, nl_groups: 0,};
    let mut rc: ssize_t = 0;
    loop  {
        msg.msg_control = 0 as *mut libc::c_void;
        msg.msg_controllen = 0 as libc::c_int as size_t;
        msg.msg_name = &mut nladdr as *mut sockaddr_nl as *mut libc::c_void;
        msg.msg_namelen =
            ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as
                socklen_t;
        msg.msg_iov = &mut iov;
        msg.msg_iovlen = 1 as libc::c_int as size_t;
        msg.msg_flags = 0 as libc::c_int;
        loop  {
            rc =
                recvmsg((*dnsmasq_daemon).netlinkfd, &mut msg,
                        MSG_PEEK as libc::c_int | MSG_TRUNC as libc::c_int);
            if !(rc == -(1 as libc::c_int) as libc::c_long &&
                     *__errno_location() == 4 as libc::c_int) {
                break ;
            }
        }
        /* make buffer big enough */
        if rc != -(1 as libc::c_int) as libc::c_long &&
               msg.msg_flags & MSG_TRUNC as libc::c_int != 0 {
            /* Very new Linux kernels return the actual size needed, older ones always return truncated size */
            if rc as size_t == iov.iov_len {
                if expand_buf(&mut iov,
                              (rc + 100 as libc::c_int as libc::c_long) as
                                  size_t) != 0 {
                    continue ;
                }
            } else { expand_buf(&mut iov, rc as size_t); }
        }
        /* read it for real */
        msg.msg_flags = 0 as libc::c_int;
        loop  {
            rc =
                recvmsg((*dnsmasq_daemon).netlinkfd, &mut msg,
                        0 as libc::c_int);
            if !(rc == -(1 as libc::c_int) as libc::c_long &&
                     *__errno_location() == 4 as libc::c_int) {
                break ;
            }
        }
        /* Make sure this is from the kernel */
        if rc == -(1 as libc::c_int) as libc::c_long ||
               nladdr.nl_pid == 0 as libc::c_int as libc::c_uint {
            break ;
        }
    }
    /* discard stuff which is truncated at this point (expand_buf() may fail) */
    if msg.msg_flags & MSG_TRUNC as libc::c_int != 0 {
        rc = -(1 as libc::c_int) as ssize_t;
        *__errno_location() = 12 as libc::c_int
    }
    return rc;
}
/* family = AF_UNSPEC finds ARP table entries.
   family = AF_LOCAL finds MAC addresses. */
#[no_mangle]
pub unsafe extern "C" fn iface_enumerate(mut family: libc::c_int,
                                         mut parm: *mut libc::c_void,
                                         mut callback:
                                             Option<unsafe extern "C" fn()
                                                        -> libc::c_int>)
 -> libc::c_int {
    let mut addr: sockaddr_nl =
        sockaddr_nl{nl_family: 0, nl_pad: 0, nl_pid: 0, nl_groups: 0,};
    let mut h: *mut nlmsghdr = 0 as *mut nlmsghdr;
    let mut len: ssize_t = 0;
    static mut seq: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut callback_ok: libc::c_int = 1 as libc::c_int;
    let mut req: C2RustUnnamed_10 =
        C2RustUnnamed_10{nlh:
                             nlmsghdr{nlmsg_len: 0,
                                      nlmsg_type: 0,
                                      nlmsg_flags: 0,
                                      nlmsg_seq: 0,
                                      nlmsg_pid: 0,},
                         g: rtgenmsg{rtgen_family: 0,},};
    memset(&mut req as *mut C2RustUnnamed_10 as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong);
    memset(&mut addr as *mut sockaddr_nl as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong);
    addr.nl_family = 16 as libc::c_int as __kernel_sa_family_t;
    loop  {
        if family == 0 as libc::c_int {
            req.nlh.nlmsg_type = RTM_GETNEIGH as libc::c_int as __u16
        } else if family == 1 as libc::c_int {
            req.nlh.nlmsg_type = RTM_GETLINK as libc::c_int as __u16
        } else { req.nlh.nlmsg_type = RTM_GETADDR as libc::c_int as __u16 }
        req.nlh.nlmsg_len =
            ::std::mem::size_of::<C2RustUnnamed_10>() as libc::c_ulong as
                __u32;
        req.nlh.nlmsg_flags =
            (0x100 as libc::c_int | 0x200 as libc::c_int | 0x1 as libc::c_int
                 | 0x4 as libc::c_int) as __u16;
        req.nlh.nlmsg_pid = 0 as libc::c_int as __u32;
        seq = seq.wrapping_add(1);
        req.nlh.nlmsg_seq = seq;
        req.g.rtgen_family = family as libc::c_uchar;
        /* Don't block in recvfrom if send fails */
        while retry_send(sendto((*dnsmasq_daemon).netlinkfd,
                                &mut req as *mut C2RustUnnamed_10 as
                                    *mut libc::c_void,
                                ::std::mem::size_of::<C2RustUnnamed_10>() as
                                    libc::c_ulong, 0 as libc::c_int,
                                __CONST_SOCKADDR_ARG{__sockaddr__:
                                                         &mut addr as
                                                             *mut sockaddr_nl
                                                             as
                                                             *mut sockaddr,},
                                ::std::mem::size_of::<sockaddr_nl>() as
                                    libc::c_ulong as socklen_t)) != 0 {
        }
        if *__errno_location() != 0 as libc::c_int { return 0 as libc::c_int }
        loop  {
            len = netlink_recv();
            if len == -(1 as libc::c_int) as libc::c_long { break ; }
            h = iov.iov_base as *mut nlmsghdr;
            while len as size_t >=
                      ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong as
                          libc::c_int as libc::c_ulong &&
                      (*h).nlmsg_len as libc::c_ulong >=
                          ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong
                      && (*h).nlmsg_len as libc::c_ulong <= len as size_t {
                if (*h).nlmsg_pid != netlink_pid ||
                       (*h).nlmsg_type as libc::c_int == 0x2 as libc::c_int {
                    /* May be multicast arriving async */
                    nl_async(h);
                } else if !((*h).nlmsg_seq != seq) {
                    if (*h).nlmsg_type as libc::c_int == 0x3 as libc::c_int {
                        return callback_ok
                    } else {
                        if (*h).nlmsg_type as libc::c_int ==
                               RTM_NEWADDR as libc::c_int &&
                               family != 0 as libc::c_int &&
                               family != 1 as libc::c_int {
                            let mut ifa: *mut ifaddrmsg =
                                (h as
                                     *mut libc::c_char).offset((0 as
                                                                    libc::c_int
                                                                    +
                                                                    ((::std::mem::size_of::<nlmsghdr>()
                                                                          as
                                                                          libc::c_ulong).wrapping_add(4
                                                                                                          as
                                                                                                          libc::c_uint
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_sub(1
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_ulong)
                                                                         &
                                                                         !(4
                                                                               as
                                                                               libc::c_uint).wrapping_sub(1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)
                                                                             as
                                                                             libc::c_ulong)
                                                                        as
                                                                        libc::c_int)
                                                                   as isize)
                                    as *mut libc::c_void as *mut ifaddrmsg;
                            let mut rta: *mut rtattr =
                                (ifa as
                                     *mut libc::c_char).offset(((::std::mem::size_of::<ifaddrmsg>()
                                                                     as
                                                                     libc::c_ulong).wrapping_add(4
                                                                                                     as
                                                                                                     libc::c_uint
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_sub(1
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_ulong)
                                                                    &
                                                                    !(4 as
                                                                          libc::c_uint).wrapping_sub(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint)
                                                                        as
                                                                        libc::c_ulong)
                                                                   as isize)
                                    as *mut rtattr;
                            let mut len1: libc::c_uint =
                                ((*h).nlmsg_len as
                                     libc::c_ulong).wrapping_sub((::std::mem::size_of::<ifaddrmsg>()
                                                                      as
                                                                      libc::c_ulong).wrapping_add(((::std::mem::size_of::<nlmsghdr>()
                                                                                                        as
                                                                                                        libc::c_ulong).wrapping_add(4
                                                                                                                                        as
                                                                                                                                        libc::c_uint
                                                                                                                                        as
                                                                                                                                        libc::c_ulong).wrapping_sub(1
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_int
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_ulong)
                                                                                                       &
                                                                                                       !(4
                                                                                                             as
                                                                                                             libc::c_uint).wrapping_sub(1
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)
                                                                                                           as
                                                                                                           libc::c_ulong)
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong))
                                    as libc::c_uint;
                            if (*ifa).ifa_family as libc::c_int == family {
                                if (*ifa).ifa_family as libc::c_int ==
                                       2 as libc::c_int {
                                    let mut netmask: in_addr =
                                        in_addr{s_addr: 0,};
                                    let mut addr_0: in_addr =
                                        in_addr{s_addr: 0,};
                                    let mut broadcast: in_addr =
                                        in_addr{s_addr: 0,};
                                    let mut label: *mut libc::c_char =
                                        0 as *mut libc::c_char;
                                    netmask.s_addr =
                                        __bswap_32((!(0 as libc::c_int as
                                                          in_addr_t)) <<
                                                       32 as libc::c_int -
                                                           (*ifa).ifa_prefixlen
                                                               as
                                                               libc::c_int);
                                    addr_0.s_addr =
                                        0 as libc::c_int as in_addr_t;
                                    broadcast.s_addr =
                                        0 as libc::c_int as in_addr_t;
                                    while len1 >=
                                              ::std::mem::size_of::<rtattr>()
                                                  as libc::c_ulong as
                                                  libc::c_int as libc::c_uint
                                              &&
                                              (*rta).rta_len as libc::c_ulong
                                                  >=
                                                  ::std::mem::size_of::<rtattr>()
                                                      as libc::c_ulong &&
                                              (*rta).rta_len as libc::c_uint
                                                  <= len1 {
                                        if (*rta).rta_type as libc::c_int ==
                                               IFA_LOCAL as libc::c_int {
                                            addr_0 =
                                                *(rta.offset(1 as libc::c_int
                                                                 as isize) as
                                                      *mut in_addr)
                                        } else if (*rta).rta_type as
                                                      libc::c_int ==
                                                      IFA_BROADCAST as
                                                          libc::c_int {
                                            broadcast =
                                                *(rta.offset(1 as libc::c_int
                                                                 as isize) as
                                                      *mut in_addr)
                                        } else if (*rta).rta_type as
                                                      libc::c_int ==
                                                      IFA_LABEL as libc::c_int
                                         {
                                            label =
                                                (rta as
                                                     *mut libc::c_char).offset(((::std::mem::size_of::<rtattr>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_add(4
                                                                                                                     as
                                                                                                                     libc::c_uint
                                                                                                                     as
                                                                                                                     libc::c_ulong).wrapping_sub(1
                                                                                                                                                     as
                                                                                                                                                     libc::c_int
                                                                                                                                                     as
                                                                                                                                                     libc::c_ulong)
                                                                                    &
                                                                                    !(4
                                                                                          as
                                                                                          libc::c_uint).wrapping_sub(1
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         libc::c_uint)
                                                                                        as
                                                                                        libc::c_ulong).wrapping_add(0
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_ulong)
                                                                                   as
                                                                                   isize)
                                                    as *mut libc::c_void as
                                                    *mut libc::c_char
                                        }
                                        len1 =
                                            len1.wrapping_sub(((*rta).rta_len
                                                                   as
                                                                   libc::c_uint).wrapping_add(4
                                                                                                  as
                                                                                                  libc::c_uint).wrapping_sub(1
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint)
                                                                  &
                                                                  !(4 as
                                                                        libc::c_uint).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint));
                                        rta =
                                            (rta as
                                                 *mut libc::c_char).offset((((*rta).rta_len
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(4
                                                                                                                as
                                                                                                                libc::c_uint).wrapping_sub(1
                                                                                                                                               as
                                                                                                                                               libc::c_int
                                                                                                                                               as
                                                                                                                                               libc::c_uint)
                                                                                &
                                                                                !(4
                                                                                      as
                                                                                      libc::c_uint).wrapping_sub(1
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_uint))
                                                                               as
                                                                               isize)
                                                as *mut rtattr
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
                                                                           libc::c_int>(Some(callback.expect("non-null function pointer")).expect("non-null function pointer"))(addr_0,
                                                                                                                                                                                (*ifa).ifa_index,
                                                                                                                                                                                label,
                                                                                                                                                                                netmask,
                                                                                                                                                                                broadcast,
                                                                                                                                                                                parm)
                                               == 0 {
                                            callback_ok = 0 as libc::c_int
                                        }
                                    }
                                } else if (*ifa).ifa_family as libc::c_int ==
                                              10 as libc::c_int {
                                    let mut addrp: *mut in6_addr =
                                        0 as *mut in6_addr;
                                    let mut valid: u32_0 =
                                        0 as libc::c_int as u32_0;
                                    let mut preferred: u32_0 =
                                        0 as libc::c_int as u32_0;
                                    let mut flags: libc::c_int =
                                        0 as libc::c_int;
                                    while len1 >=
                                              ::std::mem::size_of::<rtattr>()
                                                  as libc::c_ulong as
                                                  libc::c_int as libc::c_uint
                                              &&
                                              (*rta).rta_len as libc::c_ulong
                                                  >=
                                                  ::std::mem::size_of::<rtattr>()
                                                      as libc::c_ulong &&
                                              (*rta).rta_len as libc::c_uint
                                                  <= len1 {
                                        if (*rta).rta_type as libc::c_int ==
                                               IFA_ADDRESS as libc::c_int {
                                            addrp =
                                                rta.offset(1 as libc::c_int as
                                                               isize) as
                                                    *mut in6_addr
                                        } else if (*rta).rta_type as
                                                      libc::c_int ==
                                                      IFA_CACHEINFO as
                                                          libc::c_int {
                                            let mut ifc: *mut ifa_cacheinfo =
                                                rta.offset(1 as libc::c_int as
                                                               isize) as
                                                    *mut ifa_cacheinfo;
                                            preferred = (*ifc).ifa_prefered;
                                            valid = (*ifc).ifa_valid
                                        }
                                        len1 =
                                            len1.wrapping_sub(((*rta).rta_len
                                                                   as
                                                                   libc::c_uint).wrapping_add(4
                                                                                                  as
                                                                                                  libc::c_uint).wrapping_sub(1
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint)
                                                                  &
                                                                  !(4 as
                                                                        libc::c_uint).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint));
                                        rta =
                                            (rta as
                                                 *mut libc::c_char).offset((((*rta).rta_len
                                                                                 as
                                                                                 libc::c_uint).wrapping_add(4
                                                                                                                as
                                                                                                                libc::c_uint).wrapping_sub(1
                                                                                                                                               as
                                                                                                                                               libc::c_int
                                                                                                                                               as
                                                                                                                                               libc::c_uint)
                                                                                &
                                                                                !(4
                                                                                      as
                                                                                      libc::c_uint).wrapping_sub(1
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_uint))
                                                                               as
                                                                               isize)
                                                as *mut rtattr
                                    }
                                    if (*ifa).ifa_flags as libc::c_int &
                                           0x40 as libc::c_int != 0 {
                                        flags |= 1 as libc::c_int
                                    }
                                    if (*ifa).ifa_flags as libc::c_int &
                                           0x20 as libc::c_int != 0 {
                                        flags |= 2 as libc::c_int
                                    }
                                    if (*ifa).ifa_flags as libc::c_int &
                                           0x1 as libc::c_int == 0 {
                                        flags |= 4 as libc::c_int
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
                                                                           libc::c_int>(Some(callback.expect("non-null function pointer")).expect("non-null function pointer"))(addrp,
                                                                                                                                                                                (*ifa).ifa_prefixlen
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int,
                                                                                                                                                                                (*ifa).ifa_scope
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int,
                                                                                                                                                                                (*ifa).ifa_index
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int,
                                                                                                                                                                                flags,
                                                                                                                                                                                preferred
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int,
                                                                                                                                                                                valid
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int,
                                                                                                                                                                                parm)
                                               == 0 {
                                            callback_ok = 0 as libc::c_int
                                        }
                                    }
                                }
                            }
                        } else if (*h).nlmsg_type as libc::c_int ==
                                      RTM_NEWNEIGH as libc::c_int &&
                                      family == 0 as libc::c_int {
                            let mut neigh: *mut ndmsg =
                                (h as
                                     *mut libc::c_char).offset((0 as
                                                                    libc::c_int
                                                                    +
                                                                    ((::std::mem::size_of::<nlmsghdr>()
                                                                          as
                                                                          libc::c_ulong).wrapping_add(4
                                                                                                          as
                                                                                                          libc::c_uint
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_sub(1
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_ulong)
                                                                         &
                                                                         !(4
                                                                               as
                                                                               libc::c_uint).wrapping_sub(1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)
                                                                             as
                                                                             libc::c_ulong)
                                                                        as
                                                                        libc::c_int)
                                                                   as isize)
                                    as *mut libc::c_void as *mut ndmsg;
                            let mut rta_0: *mut rtattr =
                                (neigh as
                                     *mut libc::c_char).offset(((::std::mem::size_of::<ndmsg>()
                                                                     as
                                                                     libc::c_ulong).wrapping_add(4
                                                                                                     as
                                                                                                     libc::c_uint
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_sub(1
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_ulong)
                                                                    &
                                                                    !(4 as
                                                                          libc::c_uint).wrapping_sub(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint)
                                                                        as
                                                                        libc::c_ulong)
                                                                   as isize)
                                    as *mut rtattr;
                            let mut len1_0: libc::c_uint =
                                ((*h).nlmsg_len as
                                     libc::c_ulong).wrapping_sub((::std::mem::size_of::<ndmsg>()
                                                                      as
                                                                      libc::c_ulong).wrapping_add(((::std::mem::size_of::<nlmsghdr>()
                                                                                                        as
                                                                                                        libc::c_ulong).wrapping_add(4
                                                                                                                                        as
                                                                                                                                        libc::c_uint
                                                                                                                                        as
                                                                                                                                        libc::c_ulong).wrapping_sub(1
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_int
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_ulong)
                                                                                                       &
                                                                                                       !(4
                                                                                                             as
                                                                                                             libc::c_uint).wrapping_sub(1
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)
                                                                                                           as
                                                                                                           libc::c_ulong)
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong))
                                    as libc::c_uint;
                            let mut maclen: size_t =
                                0 as libc::c_int as size_t;
                            let mut inaddr: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut mac: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            while len1_0 >=
                                      ::std::mem::size_of::<rtattr>() as
                                          libc::c_ulong as libc::c_int as
                                          libc::c_uint &&
                                      (*rta_0).rta_len as libc::c_ulong >=
                                          ::std::mem::size_of::<rtattr>() as
                                              libc::c_ulong &&
                                      (*rta_0).rta_len as libc::c_uint <=
                                          len1_0 {
                                if (*rta_0).rta_type as libc::c_int ==
                                       NDA_DST as libc::c_int {
                                    inaddr =
                                        rta_0.offset(1 as libc::c_int as
                                                         isize) as
                                            *mut libc::c_char
                                } else if (*rta_0).rta_type as libc::c_int ==
                                              NDA_LLADDR as libc::c_int {
                                    maclen =
                                        ((*rta_0).rta_len as
                                             libc::c_ulong).wrapping_sub(::std::mem::size_of::<rtattr>()
                                                                             as
                                                                             libc::c_ulong);
                                    mac =
                                        rta_0.offset(1 as libc::c_int as
                                                         isize) as
                                            *mut libc::c_char
                                }
                                len1_0 =
                                    len1_0.wrapping_sub(((*rta_0).rta_len as
                                                             libc::c_uint).wrapping_add(4
                                                                                            as
                                                                                            libc::c_uint).wrapping_sub(1
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint)
                                                            &
                                                            !(4 as
                                                                  libc::c_uint).wrapping_sub(1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint));
                                rta_0 =
                                    (rta_0 as
                                         *mut libc::c_char).offset((((*rta_0).rta_len
                                                                         as
                                                                         libc::c_uint).wrapping_add(4
                                                                                                        as
                                                                                                        libc::c_uint).wrapping_sub(1
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                       as
                                                                                                                                       libc::c_uint)
                                                                        &
                                                                        !(4 as
                                                                              libc::c_uint).wrapping_sub(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint))
                                                                       as
                                                                       isize)
                                        as *mut rtattr
                            }
                            if (*neigh).ndm_state as libc::c_int &
                                   (0x40 as libc::c_int | 0x1 as libc::c_int |
                                        0x20 as libc::c_int) == 0 &&
                                   !inaddr.is_null() && !mac.is_null() &&
                                   callback_ok != 0 {
                                if ::std::mem::transmute::<_,
                                                           fn(_: _, _: _,
                                                              _: _, _: _,
                                                              _: _)
                                                               ->
                                                                   libc::c_int>(Some(callback.expect("non-null function pointer")).expect("non-null function pointer"))((*neigh).ndm_family
                                                                                                                                                                            as
                                                                                                                                                                            libc::c_int,
                                                                                                                                                                        inaddr,
                                                                                                                                                                        mac,
                                                                                                                                                                        maclen,
                                                                                                                                                                        parm)
                                       == 0 {
                                    callback_ok = 0 as libc::c_int
                                }
                            }
                        } else if (*h).nlmsg_type as libc::c_int ==
                                      RTM_NEWLINK as libc::c_int &&
                                      family == 1 as libc::c_int {
                            let mut link: *mut ifinfomsg =
                                (h as
                                     *mut libc::c_char).offset((0 as
                                                                    libc::c_int
                                                                    +
                                                                    ((::std::mem::size_of::<nlmsghdr>()
                                                                          as
                                                                          libc::c_ulong).wrapping_add(4
                                                                                                          as
                                                                                                          libc::c_uint
                                                                                                          as
                                                                                                          libc::c_ulong).wrapping_sub(1
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_ulong)
                                                                         &
                                                                         !(4
                                                                               as
                                                                               libc::c_uint).wrapping_sub(1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)
                                                                             as
                                                                             libc::c_ulong)
                                                                        as
                                                                        libc::c_int)
                                                                   as isize)
                                    as *mut libc::c_void as *mut ifinfomsg;
                            let mut rta_1: *mut rtattr =
                                (link as
                                     *mut libc::c_char).offset(((::std::mem::size_of::<ifinfomsg>()
                                                                     as
                                                                     libc::c_ulong).wrapping_add(4
                                                                                                     as
                                                                                                     libc::c_uint
                                                                                                     as
                                                                                                     libc::c_ulong).wrapping_sub(1
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_ulong)
                                                                    &
                                                                    !(4 as
                                                                          libc::c_uint).wrapping_sub(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint)
                                                                        as
                                                                        libc::c_ulong)
                                                                   as isize)
                                    as *mut rtattr;
                            let mut len1_1: libc::c_uint =
                                ((*h).nlmsg_len as
                                     libc::c_ulong).wrapping_sub((::std::mem::size_of::<ifinfomsg>()
                                                                      as
                                                                      libc::c_ulong).wrapping_add(((::std::mem::size_of::<nlmsghdr>()
                                                                                                        as
                                                                                                        libc::c_ulong).wrapping_add(4
                                                                                                                                        as
                                                                                                                                        libc::c_uint
                                                                                                                                        as
                                                                                                                                        libc::c_ulong).wrapping_sub(1
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_int
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_ulong)
                                                                                                       &
                                                                                                       !(4
                                                                                                             as
                                                                                                             libc::c_uint).wrapping_sub(1
                                                                                                                                            as
                                                                                                                                            libc::c_int
                                                                                                                                            as
                                                                                                                                            libc::c_uint)
                                                                                                           as
                                                                                                           libc::c_ulong)
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong))
                                    as libc::c_uint;
                            let mut mac_0: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            let mut maclen_0: size_t =
                                0 as libc::c_int as size_t;
                            while len1_1 >=
                                      ::std::mem::size_of::<rtattr>() as
                                          libc::c_ulong as libc::c_int as
                                          libc::c_uint &&
                                      (*rta_1).rta_len as libc::c_ulong >=
                                          ::std::mem::size_of::<rtattr>() as
                                              libc::c_ulong &&
                                      (*rta_1).rta_len as libc::c_uint <=
                                          len1_1 {
                                if (*rta_1).rta_type as libc::c_int ==
                                       IFLA_ADDRESS as libc::c_int {
                                    maclen_0 =
                                        ((*rta_1).rta_len as
                                             libc::c_ulong).wrapping_sub(::std::mem::size_of::<rtattr>()
                                                                             as
                                                                             libc::c_ulong);
                                    mac_0 =
                                        rta_1.offset(1 as libc::c_int as
                                                         isize) as
                                            *mut libc::c_char
                                }
                                len1_1 =
                                    len1_1.wrapping_sub(((*rta_1).rta_len as
                                                             libc::c_uint).wrapping_add(4
                                                                                            as
                                                                                            libc::c_uint).wrapping_sub(1
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint)
                                                            &
                                                            !(4 as
                                                                  libc::c_uint).wrapping_sub(1
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint));
                                rta_1 =
                                    (rta_1 as
                                         *mut libc::c_char).offset((((*rta_1).rta_len
                                                                         as
                                                                         libc::c_uint).wrapping_add(4
                                                                                                        as
                                                                                                        libc::c_uint).wrapping_sub(1
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                       as
                                                                                                                                       libc::c_uint)
                                                                        &
                                                                        !(4 as
                                                                              libc::c_uint).wrapping_sub(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint))
                                                                       as
                                                                       isize)
                                        as *mut rtattr
                            }
                            if !mac_0.is_null() && callback_ok != 0 &&
                                   (*link).ifi_flags &
                                       (IFF_LOOPBACK as libc::c_int |
                                            IFF_POINTOPOINT as libc::c_int) as
                                           libc::c_uint == 0 &&
                                   ::std::mem::transmute::<_,
                                                           fn(_: _, _: _,
                                                              _: _, _: _,
                                                              _: _)
                                                               ->
                                                                   libc::c_int>(Some(callback.expect("non-null function pointer")).expect("non-null function pointer"))((*link).ifi_index,
                                                                                                                                                                        (*link).ifi_type
                                                                                                                                                                            as
                                                                                                                                                                            libc::c_uint,
                                                                                                                                                                        mac_0,
                                                                                                                                                                        maclen_0,
                                                                                                                                                                        parm)
                                       == 0 {
                                callback_ok = 0 as libc::c_int
                            }
                        }
                    }
                }
                /* May be part of incomplete response to previous request after
	       ENOBUFS. Drop it. */
                len -=
                    ((*h).nlmsg_len.wrapping_add(4 as
                                                     libc::c_uint).wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                         &
                         !(4 as
                               libc::c_uint).wrapping_sub(1 as libc::c_int as
                                                              libc::c_uint))
                        as libc::c_long;
                h =
                    (h as
                         *mut libc::c_char).offset(((*h).nlmsg_len.wrapping_add(4
                                                                                    as
                                                                                    libc::c_uint).wrapping_sub(1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint)
                                                        &
                                                        !(4 as
                                                              libc::c_uint).wrapping_sub(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint))
                                                       as isize) as
                        *mut nlmsghdr
            }
        }
        if !(*__errno_location() == 105 as libc::c_int) { break ; }
        sleep(1 as libc::c_int as libc::c_uint);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn netlink_multicast() {
    let mut len: ssize_t = 0;
    let mut h: *mut nlmsghdr = 0 as *mut nlmsghdr;
    let mut flags: libc::c_int = 0;
    /* don't risk blocking reading netlink messages here. */
    flags = fcntl((*dnsmasq_daemon).netlinkfd, 3 as libc::c_int);
    if flags == -(1 as libc::c_int) ||
           fcntl((*dnsmasq_daemon).netlinkfd, 4 as libc::c_int,
                 flags | 0o4000 as libc::c_int) == -(1 as libc::c_int) {
        return
    }
    len = netlink_recv();
    if len != -(1 as libc::c_int) as libc::c_long {
        h = iov.iov_base as *mut nlmsghdr;
        while len as size_t >=
                  ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong as
                      libc::c_int as libc::c_ulong &&
                  (*h).nlmsg_len as libc::c_ulong >=
                      ::std::mem::size_of::<nlmsghdr>() as libc::c_ulong &&
                  (*h).nlmsg_len as libc::c_ulong <= len as size_t {
            nl_async(h);
            len -=
                ((*h).nlmsg_len.wrapping_add(4 as
                                                 libc::c_uint).wrapping_sub(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                     &
                     !(4 as
                           libc::c_uint).wrapping_sub(1 as libc::c_int as
                                                          libc::c_uint)) as
                    libc::c_long;
            h =
                (h as
                     *mut libc::c_char).offset(((*h).nlmsg_len.wrapping_add(4
                                                                                as
                                                                                libc::c_uint).wrapping_sub(1
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_uint)
                                                    &
                                                    !(4 as
                                                          libc::c_uint).wrapping_sub(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint))
                                                   as isize) as *mut nlmsghdr
        }
    }
    /* restore non-blocking status */
    fcntl((*dnsmasq_daemon).netlinkfd, 4 as libc::c_int, flags);
}
unsafe extern "C" fn nl_async(mut h: *mut nlmsghdr) {
    if (*h).nlmsg_type as libc::c_int == 0x2 as libc::c_int {
        let mut err: *mut nlmsgerr =
            (h as
                 *mut libc::c_char).offset((0 as libc::c_int +
                                                ((::std::mem::size_of::<nlmsghdr>()
                                                      as
                                                      libc::c_ulong).wrapping_add(4
                                                                                      as
                                                                                      libc::c_uint
                                                                                      as
                                                                                      libc::c_ulong).wrapping_sub(1
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulong)
                                                     &
                                                     !(4 as
                                                           libc::c_uint).wrapping_sub(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint)
                                                         as libc::c_ulong) as
                                                    libc::c_int) as isize) as
                *mut libc::c_void as *mut nlmsgerr;
        if (*err).error != 0 as libc::c_int {
            my_syslog(3 as libc::c_int,
                      b"netlink returns error: %s\x00" as *const u8 as
                          *const libc::c_char, strerror(-(*err).error));
        }
    } else if (*h).nlmsg_pid == 0 as libc::c_int as libc::c_uint &&
                  (*h).nlmsg_type as libc::c_int ==
                      RTM_NEWROUTE as libc::c_int {
        /* We arrange to receive netlink multicast messages whenever the network route is added.
	 If this happens and we still have a DNS packet in the buffer, we re-send it.
	 This helps on DoD links, where frequently the packet which triggers dialling is
	 a DNS query, which then gets lost. By re-sending, we can avoid the lookup
	 failing. */
        let mut rtm: *mut rtmsg =
            (h as
                 *mut libc::c_char).offset((0 as libc::c_int +
                                                ((::std::mem::size_of::<nlmsghdr>()
                                                      as
                                                      libc::c_ulong).wrapping_add(4
                                                                                      as
                                                                                      libc::c_uint
                                                                                      as
                                                                                      libc::c_ulong).wrapping_sub(1
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulong)
                                                     &
                                                     !(4 as
                                                           libc::c_uint).wrapping_sub(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint)
                                                         as libc::c_ulong) as
                                                    libc::c_int) as isize) as
                *mut libc::c_void as *mut rtmsg;
        if (*rtm).rtm_type as libc::c_int == RTN_UNICAST as libc::c_int &&
               (*rtm).rtm_scope as libc::c_int == RT_SCOPE_LINK as libc::c_int
               &&
               ((*rtm).rtm_table as libc::c_int ==
                    RT_TABLE_MAIN as libc::c_int ||
                    (*rtm).rtm_table as libc::c_int ==
                        RT_TABLE_LOCAL as libc::c_int) {
            queue_event(23 as libc::c_int);
        }
    } else if (*h).nlmsg_type as libc::c_int == RTM_NEWADDR as libc::c_int ||
                  (*h).nlmsg_type as libc::c_int == RTM_DELADDR as libc::c_int
     {
        queue_event(22 as libc::c_int);
    };
}
