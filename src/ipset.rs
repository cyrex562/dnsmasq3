
static mut snl: sockaddr_nl =
    {
        let mut init =
            sockaddr_nl{nl_family: 16 as libc::c_int as __kernel_sa_family_t,
                        nl_pad: 0,
                        nl_pid: 0,
                        nl_groups: 0,};
        init
    };
static mut ipset_sock: libc::c_int = 0;
static mut old_kernel: libc::c_int = 0;
static mut buffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[inline]
unsafe extern "C" fn add_attr(mut nlh: *mut nlmsghdr, mut type_0: uint16_t,
                              mut len: size_t,
                              mut data: *const libc::c_void) {
    let mut attr: *mut my_nlattr =
        (nlh as
             *mut libc::c_void).offset(((*nlh).nlmsg_len.wrapping_add(3 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                            &
                                            !(3 as libc::c_int) as
                                                libc::c_uint) as isize) as
            *mut my_nlattr;
    let mut payload_len: uint16_t =
        ((::std::mem::size_of::<my_nlattr>() as
              libc::c_ulong).wrapping_add(3 as libc::c_int as libc::c_ulong) &
             !(3 as libc::c_int) as libc::c_ulong).wrapping_add(len) as
            uint16_t;
    (*attr).nla_type = type_0;
    (*attr).nla_len = payload_len;
    memcpy((attr as
                *mut libc::c_void).offset(((::std::mem::size_of::<my_nlattr>()
                                                as
                                                libc::c_ulong).wrapping_add(3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                               &
                                               !(3 as libc::c_int) as
                                                   libc::c_ulong) as isize),
           data, len);
    (*nlh).nlmsg_len =
        ((*nlh).nlmsg_len as
             libc::c_uint).wrapping_add((payload_len as libc::c_int +
                                             3 as libc::c_int &
                                             !(3 as libc::c_int)) as
                                            libc::c_uint) as __u32 as __u32;
}
#[no_mangle]
pub unsafe extern "C" fn ipset_init() {
    old_kernel =
        ((*dnsmasq_daemon).kernel_version <
             ((2 as libc::c_int) << 16 as libc::c_int) +
                 ((6 as libc::c_int) << 8 as libc::c_int) + 32 as libc::c_int)
            as libc::c_int;
    if old_kernel != 0 &&
           {
               ipset_sock =
                   socket(2 as libc::c_int, SOCK_RAW as libc::c_int,
                          IPPROTO_RAW as libc::c_int);
               (ipset_sock) != -(1 as libc::c_int)
           } {
        return
    }
    if old_kernel == 0 &&
           {
               buffer =
                   safe_malloc(256 as libc::c_int as size_t) as
                       *mut libc::c_char;
               !buffer.is_null()
           } &&
           {
               ipset_sock =
                   socket(16 as libc::c_int, SOCK_RAW as libc::c_int,
                          12 as libc::c_int);
               (ipset_sock) != -(1 as libc::c_int)
           } &&
           bind(ipset_sock,
                __CONST_SOCKADDR_ARG{__sockaddr__:
                                         &snl as *const sockaddr_nl as
                                             *mut sockaddr,},
                ::std::mem::size_of::<sockaddr_nl>() as libc::c_ulong as
                    socklen_t) != -(1 as libc::c_int) {
        return
    }
    die(b"failed to create IPset control socket: %s\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char, 0 as *mut libc::c_char,
        5 as libc::c_int);
}
unsafe extern "C" fn new_add_to_ipset(mut setname: *const libc::c_char,
                                      mut ipaddr: *const all_addr,
                                      mut af: libc::c_int,
                                      mut remove: libc::c_int)
 -> libc::c_int {
    let mut nlh: *mut nlmsghdr = 0 as *mut nlmsghdr;
    let mut nfg: *mut my_nfgenmsg = 0 as *mut my_nfgenmsg;
    let mut nested: [*mut my_nlattr; 2] = [0 as *mut my_nlattr; 2];
    let mut proto: uint8_t = 0;
    let mut addrsz: libc::c_int =
        if af == 10 as libc::c_int {
            16 as libc::c_int
        } else { 4 as libc::c_int };
    if strlen(setname) >= 32 as libc::c_int as libc::c_ulong {
        *__errno_location() = 36 as libc::c_int;
        return -(1 as libc::c_int)
    }
    memset(buffer as *mut libc::c_void, 0 as libc::c_int,
           256 as libc::c_int as libc::c_ulong);
    nlh = buffer as *mut nlmsghdr;
    (*nlh).nlmsg_len =
        ((::std::mem::size_of::<nlmsghdr>() as
              libc::c_ulong).wrapping_add(3 as libc::c_int as libc::c_ulong) &
             !(3 as libc::c_int) as libc::c_ulong) as __u32;
    (*nlh).nlmsg_type =
        ((if remove != 0 { 10 as libc::c_int } else { 9 as libc::c_int }) |
             (6 as libc::c_int) << 8 as libc::c_int) as __u16;
    (*nlh).nlmsg_flags = 0x1 as libc::c_int as __u16;
    nfg = buffer.offset((*nlh).nlmsg_len as isize) as *mut my_nfgenmsg;
    (*nlh).nlmsg_len =
        ((*nlh).nlmsg_len as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<my_nfgenmsg>()
                                              as
                                              libc::c_ulong).wrapping_add(3 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
                                             &
                                             !(3 as libc::c_int) as
                                                 libc::c_ulong) as __u32 as
            __u32;
    (*nfg).nfgen_family = af as __u8;
    (*nfg).version = 0 as libc::c_int as __u8;
    (*nfg).res_id = __bswap_16(0 as libc::c_int as __uint16_t);
    proto = 6 as libc::c_int as uint8_t;
    add_attr(nlh, 1 as libc::c_int as uint16_t,
             ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
             &mut proto as *mut uint8_t as *const libc::c_void);
    add_attr(nlh, 2 as libc::c_int as uint16_t,
             strlen(setname).wrapping_add(1 as libc::c_int as libc::c_ulong),
             setname as *const libc::c_void);
    nested[0 as libc::c_int as usize] =
        buffer.offset(((*nlh).nlmsg_len.wrapping_add(3 as libc::c_int as
                                                         libc::c_uint) &
                           !(3 as libc::c_int) as libc::c_uint) as isize) as
            *mut my_nlattr;
    (*nlh).nlmsg_len =
        ((*nlh).nlmsg_len as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<my_nlattr>()
                                              as
                                              libc::c_ulong).wrapping_add(3 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
                                             &
                                             !(3 as libc::c_int) as
                                                 libc::c_ulong) as __u32 as
            __u32;
    (*nested[0 as libc::c_int as usize]).nla_type =
        ((1 as libc::c_int) << 15 as libc::c_int | 7 as libc::c_int) as __u16;
    nested[1 as libc::c_int as usize] =
        buffer.offset(((*nlh).nlmsg_len.wrapping_add(3 as libc::c_int as
                                                         libc::c_uint) &
                           !(3 as libc::c_int) as libc::c_uint) as isize) as
            *mut my_nlattr;
    (*nlh).nlmsg_len =
        ((*nlh).nlmsg_len as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<my_nlattr>()
                                              as
                                              libc::c_ulong).wrapping_add(3 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
                                             &
                                             !(3 as libc::c_int) as
                                                 libc::c_ulong) as __u32 as
            __u32;
    (*nested[1 as libc::c_int as usize]).nla_type =
        ((1 as libc::c_int) << 15 as libc::c_int | 1 as libc::c_int) as __u16;
    add_attr(nlh,
             ((if af == 2 as libc::c_int {
                   1 as libc::c_int
               } else { 2 as libc::c_int }) |
                  (1 as libc::c_int) << 14 as libc::c_int) as uint16_t,
             addrsz as size_t, ipaddr as *const libc::c_void);
    (*nested[1 as libc::c_int as usize]).nla_len =
        (buffer as
             *mut libc::c_void).offset(((*nlh).nlmsg_len.wrapping_add(3 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                            &
                                            !(3 as libc::c_int) as
                                                libc::c_uint) as
                                           isize).wrapping_offset_from(nested[1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                                                           as
                                                                           *mut libc::c_void)
            as libc::c_long as __u16;
    (*nested[0 as libc::c_int as usize]).nla_len =
        (buffer as
             *mut libc::c_void).offset(((*nlh).nlmsg_len.wrapping_add(3 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                            &
                                            !(3 as libc::c_int) as
                                                libc::c_uint) as
                                           isize).wrapping_offset_from(nested[0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                                                           as
                                                                           *mut libc::c_void)
            as libc::c_long as __u16;
    while retry_send(sendto(ipset_sock, buffer as *const libc::c_void,
                            (*nlh).nlmsg_len as size_t, 0 as libc::c_int,
                            __CONST_SOCKADDR_ARG{__sockaddr__:
                                                     &snl as
                                                         *const sockaddr_nl as
                                                         *mut sockaddr,},
                            ::std::mem::size_of::<sockaddr_nl>() as
                                libc::c_ulong as socklen_t)) != 0 {
    }
    return if *__errno_location() == 0 as libc::c_int {
               0 as libc::c_int
           } else { -(1 as libc::c_int) };
}
unsafe extern "C" fn old_add_to_ipset(mut setname: *const libc::c_char,
                                      mut ipaddr: *const all_addr,
                                      mut remove: libc::c_int)
 -> libc::c_int {
    let mut size: socklen_t = 0;
    let mut req_adt_get: ip_set_req_adt_get =
        ip_set_req_adt_get{op: 0,
                           version: 0,
                           set: C2RustUnnamed_9{name: [0; 32],},
                           typename: [0; 32],};
    let mut req_adt: ip_set_req_adt = ip_set_req_adt{op: 0, index: 0, ip: 0,};
    if strlen(setname) >=
           ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong {
        *__errno_location() = 36 as libc::c_int;
        return -(1 as libc::c_int)
    }
    req_adt_get.op = 0x10 as libc::c_int as libc::c_uint;
    req_adt_get.version = 3 as libc::c_int as libc::c_uint;
    strcpy(req_adt_get.set.name.as_mut_ptr(), setname);
    size =
        ::std::mem::size_of::<ip_set_req_adt_get>() as libc::c_ulong as
            socklen_t;
    if getsockopt(ipset_sock, 0 as libc::c_int, 83 as libc::c_int,
                  &mut req_adt_get as *mut ip_set_req_adt_get as
                      *mut libc::c_void, &mut size) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    req_adt.op =
        if remove != 0 { 0x102 as libc::c_int } else { 0x101 as libc::c_int }
            as libc::c_uint;
    req_adt.index = req_adt_get.set.index;
    req_adt.ip = __bswap_32((*ipaddr).addr4.s_addr);
    if setsockopt(ipset_sock, 0 as libc::c_int, 83 as libc::c_int,
                  &mut req_adt as *mut ip_set_req_adt as *const libc::c_void,
                  ::std::mem::size_of::<ip_set_req_adt>() as libc::c_ulong as
                      socklen_t) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn add_to_ipset(mut setname: *const libc::c_char,
                                      mut ipaddr: *const all_addr,
                                      mut flags: libc::c_int,
                                      mut remove: libc::c_int)
 -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut af: libc::c_int = 2 as libc::c_int;
    if flags as libc::c_uint & (1 as libc::c_uint) << 8 as libc::c_int != 0 {
        af = 10 as libc::c_int;
        /* old method only supports IPv4 */
        if old_kernel != 0 {
            *__errno_location() = 97 as libc::c_int;
            ret = -(1 as libc::c_int)
        }
    }
    if ret != -(1 as libc::c_int) {
        ret =
            if old_kernel != 0 {
                old_add_to_ipset(setname, ipaddr, remove)
            } else { new_add_to_ipset(setname, ipaddr, af, remove) }
    }
    if ret == -(1 as libc::c_int) {
        my_syslog(3 as libc::c_int,
                  b"failed to update ipset %s: %s\x00" as *const u8 as
                      *const libc::c_char, setname,
                  strerror(*__errno_location()));
    }
    return ret;
}
