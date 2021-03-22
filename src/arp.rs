use crate::defines::{arp_record, in_addr, in6_addr};

// #![register_tool(c2rust)]
// #![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]

// static mut arps: *mut arp_record = 0 as *const arp_record as *mut arp_record;
// static mut old: *mut arp_record = 0 as *const arp_record as *mut arp_record;
pub fn filter_mac<T>(mut family: libc::c_int,
                  mut addrp: &T,
                  mut mac: String,
                  mut parmv: *mut libc::c_void,
                  arps: &mut Vec<arp_record>) -> libc::c_int {
    // let mut arp: *mut arp_record = 0 as *mut arp_record;
    let mut arp: arp_record = Default::default();
    if maclen > 16 { return 1 }
    let mut current_block_8: u64;
    /* Look for existing entry */
    for arp in arps {
        if !(family != arp.family || arp.status == 2) {
            if family == 2 {
                if arp.addr.addr4.s_addr != addrp.s_addr {
                    current_block_8 = 4644295000439058019;
                } else { current_block_8 = 11650488183268122163; }
            } else if ({
                let mut __a: in6_addr = arp.addr.addr6;
                let mut __b: in6_addr = addrp as in6_addr;
                (__a.__in6_u.__u6_addr32[0] == __b.__in6_u.__u6_addr32[0] &&
                    __a.__in6_u.__u6_addr32[1] == __b.__in6_u.__u6_addr32[1]
                    && __a.__in6_u.__u6_addr32[2] ==
                        __b.__in6_u.__u6_addr32[2]
                    &&
                    __a.__in6_u.__u6_addr32[3] ==
                        __b.__in6_u.__u6_addr32[3])
            }) == 0 {
                current_block_8 = 4644295000439058019;
            } else { current_block_8 = 11650488183268122163; }
            match current_block_8 {
                4644295000439058019 => { }
                _ => {
                    if arp.status as libc::c_int == 3 as libc::c_int {
                        /* existing address, was negative. */
                        arp.status = 2 as libc::c_int as libc::c_ushort;
                        arp.hwlen = maclen as libc::c_ushort;
                        memcpy(arp.hwaddr.as_mut_ptr() as
                                   *mut libc::c_void,
                               mac as *const libc::c_void, maclen);
                        break ;
                    } else if arp.hwlen as libc::c_ulong == maclen &&
                        memcmp(arp.hwaddr.as_mut_ptr() as
                                   *const libc::c_void,
                               mac as *const libc::c_void, maclen)
                            == 0 as libc::c_int {
                        /* Existing entry matches - confirm. */
                        arp.status = 1 as libc::c_int as libc::c_ushort;
                        break ;
                    }
                }
            }
        }
    }
    // arp = arps;
    // while !arp.is_null() {
    //
    //     arp = (*arp).next
    // }
    if arp.is_null() {
        /* New entry */
        if !freelist.is_null() {
            arp = freelist;
            freelist = (*freelist).next
        } else {
            arp =
                whine_malloc(::std::mem::size_of::<arp_record>() as
                                 libc::c_ulong) as *mut arp_record;
            if arp.is_null() { return 1 as libc::c_int }
        }
        (*arp).next = arps;
        arps = arp;
        (*arp).status = 2 as libc::c_int as libc::c_ushort;
        (*arp).hwlen = maclen as libc::c_ushort;
        (*arp).family = family;
        memcpy((*arp).hwaddr.as_mut_ptr() as *mut libc::c_void,
               mac as *const libc::c_void, maclen);
        if family == 2 as libc::c_int {
            (*arp).addr.addr4.s_addr = (*(addrp as *mut in_addr)).s_addr
        } else {
            memcpy(&mut (*arp).addr.addr6 as *mut in6_addr as
                       *mut libc::c_void, addrp as *const libc::c_void,
                   16 as libc::c_int as libc::c_ulong);
        }
    }
    return 1 as libc::c_int;
}
/* If in lazy mode, we cache absence of ARP entries. */
#[no_mangle]
pub unsafe extern "C" fn find_mac(mut addr: *mut mysockaddr,
                                  mut mac: *mut libc::c_uchar,
                                  mut lazy: libc::c_int, mut now: time_t)
 -> libc::c_int {
    let mut arp: *mut arp_record = 0 as *mut arp_record;
    let mut tmp: *mut arp_record = 0 as *mut arp_record;
    let mut up: *mut *mut arp_record = 0 as *mut *mut arp_record;
    let mut updated: libc::c_int = 0 as libc::c_int;
    loop  {
        /* If the database is less then INTERVAL old, look in there */
        if difftime(now, last) < 90 as libc::c_int as libc::c_double {
            /* addr == NULL -> just make cache up-to-date */
            if addr.is_null() { return 0 as libc::c_int }
            arp = arps;
            while !arp.is_null() {
                if !((*addr).sa.sa_family as libc::c_int != (*arp).family) {
                    if !((*arp).family == 2 as libc::c_int &&
                             (*arp).addr.addr4.s_addr !=
                                 (*addr).in_0.sin_addr.s_addr) {
                        if !((*arp).family == 10 as libc::c_int &&
                                 ({
                                      let mut __a: *const in6_addr =
                                          &mut (*arp).addr.addr6 as
                                              *mut in6_addr as
                                              *const in6_addr;
                                      let mut __b: *const in6_addr =
                                          &mut (*addr).in6.sin6_addr as
                                              *mut in6_addr as
                                              *const in6_addr;
                                      ((*__a).__in6_u.__u6_addr32[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                                           ==
                                           (*__b).__in6_u.__u6_addr32[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                           &&
                                           (*__a).__in6_u.__u6_addr32[1 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                               ==
                                               (*__b).__in6_u.__u6_addr32[1 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                                           &&
                                           (*__a).__in6_u.__u6_addr32[2 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                               ==
                                               (*__b).__in6_u.__u6_addr32[2 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                                           &&
                                           (*__a).__in6_u.__u6_addr32[3 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                               ==
                                               (*__b).__in6_u.__u6_addr32[3 as
                                                                              libc::c_int
                                                                              as
                                                                              usize])
                                          as libc::c_int
                                  }) == 0) {
                            /* Only accept positive entries unless in lazy mode. */
                            if (*arp).status as libc::c_int !=
                                   3 as libc::c_int || lazy != 0 ||
                                   updated != 0 {
                                if !mac.is_null() &&
                                       (*arp).hwlen as libc::c_int !=
                                           0 as libc::c_int {
                                    memcpy(mac as *mut libc::c_void,
                                           (*arp).hwaddr.as_mut_ptr() as
                                               *const libc::c_void,
                                           (*arp).hwlen as libc::c_ulong);
                                }
                                return (*arp).hwlen as libc::c_int
                            }
                        }
                    }
                }
                arp = (*arp).next
            }
        }
        /* Not found, try the kernel */
        if !(updated == 0) { break ; }
        updated = 1 as libc::c_int;
        last = now;
        /* Mark all non-negative entries */
        arp = arps;
        while !arp.is_null() {
            if (*arp).status as libc::c_int != 3 as libc::c_int {
                (*arp).status = 0 as libc::c_int as libc::c_ushort
            }
            arp = (*arp).next
        }
        iface_enumerate(0 as libc::c_int, 0 as *mut libc::c_void,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                libc::c_int,
                                                                            _:
                                                                                *mut libc::c_char,
                                                                            _:
                                                                                *mut libc::c_char,
                                                                            _:
                                                                                size_t,
                                                                            _:
                                                                                *mut libc::c_void)
                                                           -> libc::c_int>,
                                                Option<unsafe extern "C" fn()
                                                           ->
                                                               libc::c_int>>(Some(filter_mac
                                                                                      as
                                                                                      unsafe extern "C" fn(_:
                                                                                                               libc::c_int,
                                                                                                           _:
                                                                                                               *mut libc::c_char,
                                                                                                           _:
                                                                                                               *mut libc::c_char,
                                                                                                           _:
                                                                                                               size_t,
                                                                                                           _:
                                                                                                               *mut libc::c_void)
                                                                                          ->
                                                                                              libc::c_int)));
        /* Remove all unconfirmed entries to old list. */
        arp = arps;
        up = &mut arps;
        while !arp.is_null() {
            tmp = (*arp).next;
            if (*arp).status as libc::c_int == 0 as libc::c_int {
                *up = (*arp).next;
                (*arp).next = old;
                old = arp
            } else { up = &mut (*arp).next }
            arp = tmp
        }
    }
    /* record failure, so we don't consult the kernel each time
     we're asked for this address */
    if !freelist.is_null() {
        arp = freelist;
        freelist = (*freelist).next
    } else {
        arp =
            whine_malloc(::std::mem::size_of::<arp_record>() as libc::c_ulong)
                as *mut arp_record
    }
    if !arp.is_null() {
        (*arp).next = arps;
        arps = arp;
        (*arp).status = 3 as libc::c_int as libc::c_ushort;
        (*arp).family = (*addr).sa.sa_family as libc::c_int;
        (*arp).hwlen = 0 as libc::c_int as libc::c_ushort;
        if (*addr).sa.sa_family as libc::c_int == 2 as libc::c_int {
            (*arp).addr.addr4.s_addr = (*addr).in_0.sin_addr.s_addr
        } else {
            memcpy(&mut (*arp).addr.addr6 as *mut in6_addr as
                       *mut libc::c_void,
                   &mut (*addr).in6.sin6_addr as *mut in6_addr as
                       *const libc::c_void,
                   16 as libc::c_int as libc::c_ulong);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn do_arp_script_run() -> libc::c_int {
    let mut arp: *mut arp_record = 0 as *mut arp_record;
    /* Notify any which went, then move to free list */
    if !old.is_null() {
        if (*dnsmasq_daemon).options[(53 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (53 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
            queue_arp(7 as libc::c_int, (*old).hwaddr.as_mut_ptr(),
                      (*old).hwlen as libc::c_int, (*old).family,
                      &mut (*old).addr);
        }
        arp = old;
        old = (*arp).next;
        (*arp).next = freelist;
        freelist = arp;
        return 1 as libc::c_int
    }
    arp = arps;
    while !arp.is_null() {
        if (*arp).status as libc::c_int == 2 as libc::c_int {
            if (*dnsmasq_daemon).options[(53 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (53 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
                queue_arp(6 as libc::c_int, (*arp).hwaddr.as_mut_ptr(),
                          (*arp).hwlen as libc::c_int, (*arp).family,
                          &mut (*arp).addr);
            }
            (*arp).status = 1 as libc::c_int as libc::c_ushort;
            return 1 as libc::c_int
        }
        arp = (*arp).next
    }
    return 0 as libc::c_int;
}
