use crate::slack::{NetAddress_nl, nlmsghdr, my_nlattr, my_nfgenmsg, ip_set_req_adt_get, ip_set_req_adt};
use crate::defines::{KernelSaFamily, DnsmasqDaemon, SOCK_RAW, IPPROTO_RAW, ConstNetAddressArg, NetAddress, socklen_t, NetAddress, __bswap_16, C2rustUnnamed9, __bswap_32};
use crate::util::{safe_malloc, retry_send};
use crate::dnsmasq_log::{die, my_syslog};

static mut snl: NetAddress_nl =
    {
        let mut init =
            NetAddress_nl{nl_family: 16 ,
                        nl_pad: 0,
                        nl_pid: 0,
                        nl_groups: 0,};
        init
    };
// static mut ipset_sock: i32 = 0;
// static mut old_kernel: i32 = 0;
// static mut buffer: &mut String =
//     0 ;

fn add_attr(mut nlh: *mut nlmsghdr, mut type_0: u16,
                              mut len: usize,
                              mut data: *const libc::c_void) {
    let mut attr: *mut my_nlattr =
        (nlh      Vec<u8>).offset(((*nlh).nlmsg_len.wrapping_add(3                       libc::c_int
                                                                                                libc::c_uint)
                                            &
                                            !(3)                                          libc::c_uint))      *mut my_nlattr;
    let mut payload_len: u16 =
        ((::std::mem::size_of::<my_nlattr>() ).wrapping_add(3) &
             !(3)).wrapping_add(len)      u16;
    (*attr).nla_type = type_0;
    (*attr).nla_len = payload_len;
    memcpy((attr         Vec<u8>).offset(((::std::mem::size_of::<my_nlattr>()
                                                                                  ).wrapping_add(3
                                                                                                            libc::c_int
                                                                                                     )
                                               &
                                               !(3)libc::c_ulong)),
           data, len);
    (*nlh).nlmsg_len =
        ((*nlh).nlmsg_len       libc::c_uint).wrapping_add((payload_len +
                                             3 &
                                             !(3))                                      libc::c_uint);
}
#[no_mangle]
pub fn ipset_init() {
    old_kernel =
        ((*dnsmasq_daemon).kernel_version <
             ((2) << 16) +
                 ((6) << 8) + 32)
           ;
    if old_kernel != 0 &&
           {
               ipset_sock =
                   socket(2, SOCK_RAW,
                          IPPROTO_RAW);
               (ipset_sock) != -(1)
           } {
        return
    }
    if old_kernel == 0 &&
           {
               buffer =
                   safe_malloc(256 );
               !buffer.is_null()
           } &&
           {
               ipset_sock =
                   socket(16, SOCK_RAW,
                          12);
               (ipset_sock) != -(1)
           } &&
           bind(ipset_sock,
                ConstNetAddressArg {__NetAddress__:
                                         &snl ),},
                ::std::mem::size_of::<NetAddress_nl>()              socklen_t) != -(1) {
        return
    }
    die(b"failed to create IPset control socket: %s\x00"       *const libc::c_char , 0 ,
        5);
}
fn new_add_to_ipset(mut setname: *const libc::c_char,
                                      mut ipaddr: *const NetAddress,
                                      mut af: i32,
                                      mut remove: i32, buffer: &mut Vec<u8>)
                                      -> i32 {
    let mut nlh: *mut nlmsghdr = 0 );
    let mut nfg: *mut my_nfgenmsg = 0 );
    let mut nested: [*mut my_nlattr; 2] = [0 ; 2];
    let mut proto: u8 = 0;
    let mut addrsz: i32 =
        if af == 10 {
            16
        } else { 4 };
    if strlen(setname) >= 32 {
        *__errno_location() = 36;
        return -(1)
    }
    buffer.clear();
    nlh = buffer;
    (*nlh).nlmsg_len =
        ((::std::mem::size_of::<nlmsghdr>() ).wrapping_add(3) &
             !(3));
    (*nlh).nlmsg_type =
        ((if remove != 0 { 10 } else { 9 }) |
             (6) << 8);
    (*nlh).nlmsg_flags = 0x1;
    nfg = buffer.offset((*nlh).nlmsg_len) );
    (*nlh).nlmsg_len =
        ((*nlh).nlmsg_len as
      ).wrapping_add((::std::mem::size_of::<my_nfgenmsg>()
                                                                              ).wrapping_add(3                           libc::c_int
                                                                                                 )
                                             &
                                             !(3)                                    )      u32;
    (*nfg).nfgen_family = af as u8;
    (*nfg).version = 0 as u8;
    (*nfg).res_id = __bswap_16(0);
    proto = 6 as u8;
    add_attr(nlh, 1,
             ::std::mem::size_of::<u8>(),
             &mut proto );
    add_attr(nlh, 2,
             strlen(setname).wrapping_add(1),
             setname);
    nested[0 ] =
        buffer.offset(((*nlh).nlmsg_len.wrapping_add(3      libc::c_uint) &
                           !(3)))      *mut my_nlattr;
    (*nlh).nlmsg_len =
        ((*nlh).nlmsg_len as
      ).wrapping_add((::std::mem::size_of::<my_nlattr>()
                                                                              ).wrapping_add(3                           libc::c_int
                                                                                                 )
                                             &
                                             !(3)                                    )      u32;
    (*nested[0 ]).nla_type =
        ((1) << 15 | 7);
    nested[1 ] =
        buffer.offset(((*nlh).nlmsg_len.wrapping_add(3      libc::c_uint) &
                           !(3)))      *mut my_nlattr;
    (*nlh).nlmsg_len =
        ((*nlh).nlmsg_len as
      ).wrapping_add((::std::mem::size_of::<my_nlattr>()
                                                                              ).wrapping_add(3                           libc::c_int
                                                                                                 )
                                             &
                                             !(3)                                    )      u32;
    (*nested[1 ]).nla_type =
        ((1) << 15 | 1);
    add_attr(nlh,
             ((if af == 2 {
                   1
               } else { 2 }) |
                  (1) << 14),
             addrsz , ipaddr);
    (*nested[1 ]).nla_len =
        (buffer      Vec<u8>).offset(((*nlh).nlmsg_len.wrapping_add(3                       libc::c_int
                                                                                                libc::c_uint)
                                            &
                                            !(3)                                          libc::c_uint)                                     isize).wrapping_offset_from(nested[1
                                                                                                                libc::c_int
                                                                                                                usize]
                                                                                                 Vec<u8>)
           ;
    (*nested[0 ]).nla_len =
        (buffer      Vec<u8>).offset(((*nlh).nlmsg_len.wrapping_add(3                       libc::c_int
                                                                                                libc::c_uint)
                                            &
                                            !(3)                                          libc::c_uint)                                     isize).wrapping_offset_from(nested[0
                                                                                                                libc::c_int
                                                                                                                usize]
                                                                                                 Vec<u8>)
           ;
    while retry_send(sendto(ipset_sock, buffer,
                            (*nlh).nlmsg_len , 0,
                            ConstNetAddressArg {__NetAddress__:
                                                     &snl      *const NetAddress_nl      NetAddress,},
                            ::std::mem::size_of::<NetAddress_nl>()                   )) != 0 {
    }
    return if *__errno_location() == 0 {
               0
           } else { -(1) };
}
fn old_add_to_ipset(mut setname: *const libc::c_char,
                                      mut ipaddr: *const NetAddress,
                                      mut remove: i32)
                                      -> i32 {
    let mut size: socklen_t = 0;
    let mut req_adt_get: ip_set_req_adt_get =
        ip_set_req_adt_get{op: 0,
                           version: 0,
                           set: C2rustUnnamed9 {name: [0; 32],},
                           typename: [0; 32],};
    let mut req_adt: ip_set_req_adt = ip_set_req_adt{op: 0, index: 0, ip: 0,};
    if strlen(setname) >=
           ::std::mem::size_of::<[libc::c_char; 32]>() {
        *__errno_location() = 36;
        return -(1)
    }
    req_adt_get.op = 0x10;
    req_adt_get.version = 3;
    strcpy(req_adt_get.set.name.as_mut_ptr(), setname);
    size =
        ::std::mem::size_of::<ip_set_req_adt_get>() a);
    if getsockopt(ipset_sock, 0, 83,
                  &mut req_adt_get ), &mut size) < 0 {
        return -(1)
    }
    req_adt.op =
        if remove != 0 { 0x102 } else { 0x101 }
           ;
    req_adt.index = req_adt_get.set.index;
    req_adt.ip = __bswap_32((*ipaddr).addr4.s_addr);
    if setsockopt(ipset_sock, 0, 83,
                  &mut req_adt ),
                  ::std::mem::size_of::<ip_set_req_adt>() )) < 0 {
        return -(1)
    }
    return 0;
}
#[no_mangle]
pub fn add_to_ipset(mut setname: *const libc::c_char,
                                      mut ipaddr: *const NetAddress,
                                      mut flags: i32,
                                      mut remove: i32)
                                      -> i32 {
    let mut ret: i32 = 0;
    let mut af: i32 = 2;
    if flags & (1) << 8 != 0 {
        af = 10;
        /* old method only supports IPv4 */
        if old_kernel != 0 {
            *__errno_location() = 97;
            ret = -(1)
        }
    }
    if ret != -(1) {
        ret =
            if old_kernel != 0 {
                old_add_to_ipset(setname, ipaddr, remove)
            } else { new_add_to_ipset(setname, ipaddr, af, remove) }
    }
    if ret == -(1) {
        my_syslog(3,
                  b"failed to update ipset %s: %s\x00", setname,
                  strerror(*__errno_location()));
    }
    return ret;
}
