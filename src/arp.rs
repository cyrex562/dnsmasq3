use crate::defines::{arp_record, in_addr, in6_addr, time_t, mysockaddr, size_t, dnsmasq_daemon};
use crate::helper::queue_arp;

// #![register_tool(c2rust)]
// #![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]

// static mut arps: *mut arp_record = 0 as *const arp_record as *mut arp_record;
// static mut old: *mut arp_record = 0 as *const arp_record as *mut arp_record;
pub fn filter_mac<T>(mut family: libc::c_int,
                  mut addrp: &T,
                  mut mac: String,
                  arps: &mut Vec<arp_record>) -> libc::c_int {
    // let mut arp: *mut arp_record = 0 as *mut arp_record;
    // let mut arp: arp_record = Default::default();
    if maclen > 16 { return 1 }
    let mut current_block_8: u64;
    /* Look for existing entry */
    let mut modified: bool = false;
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
            }) == false {
                current_block_8 = 4644295000439058019;
            } else { current_block_8 = 11650488183268122163; }
            match current_block_8 {
                4644295000439058019 => { }
                _ => {
                    if arp.status == 3 {
                        /* existing address, was negative. */
                        arp.status = 2;
                        arp.hwlen = maclen;
                        arp.hwaddr.copy_from_slice(mac.as_bytes());
                        modified = true;
                        break ;
                    } else if arp.hwlen == maclen && arp.hwaddr == mac {
                        /* Existing entry matches - confirm. */
                        arp.status = 1 as libc::c_int as libc::c_ushort;
                        modified = true;
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
    if modified == false {
        let mut new_arp: arp_record = Default::default();
        new_arp.status = 2;
        new_arp.hwlen = maclen;
        new_arp.family = family;
        new_arp.hwaddr.copy_from_slice(mac.as_bytes());
        new_arp.addr.addr4.s_addr = addrp.s_addr;
        // TODO: copy as ipv6 instead when using ipv6
        arps.push(new_arp);
    }
    return 1;
}

/* If in lazy mode, we cache absence of ARP entries. */
pub fn find_mac(addr: Optiona<mysockaddr>,
                mac: Option<Vec<u8>>,
                lazy: libc::c_int,
                now: time_t,
                arps: &mut Vec<arp_record>)
 -> u16 {
    let mut arp: arp_record = Default::default();
    // let mut tmp: arp_record = Default::default();
    // let mut up: arp_record = Default::default();
    let mut updated: libc::c_int = 0;
    loop  {
        /* If the database is less then INTERVAL old, look in there */
        if difftime(now, last) < 90 {
            /* addr == NULL -> just make cache up-to-date */
            if addr.is_none() {
                return 0;
            }
            for arp in arps {
                if !(addr.sa.sa_family != arp.family) {
                    if !(arp.family == 2 && arp.addr.addr4.s_addr != addr.in_0.sin_addr.s_addr) {
                        if !(arp.family == 10 && ({
                            let mut __a: in6_addr = arp.addr.addr6;
                            let mut __b: in6_addr = addr.in6.sin6_addr;
                            (__a.__in6_u.__u6_addr32[0] == *__b.__in6_u.__u6_addr32[0] && __a.__in6_u.__u6_addr32[1] == __b.__in6_u.__u6_addr32[1] &&  __a.__in6_u.__u6_addr32[2] == __b.__in6_u.__u6_addr32[2] && __a.__in6_u.__u6_addr32[3] == __b.__in6_u.__u6_addr32[3])}) == false) {
                            /* Only accept positive entries unless in lazy mode. */
                            if arp.status != 3 || lazy != 0 || updated != 0 {
                                if mac.is_some() && arp.hwlen != 0 {
                                    let mut _mac = arp.hwaddr;
                                }
                                return arp.hwlen;
                            }
                        }
                    }
                }
            }
        }
        /* Not found, try the kernel */
        if !(updated == 0) { break ; }
        updated = 1;
        last = now;
        /* Mark all non-negative entries */
        for arp in arps {
            if arp.status != 3 {
                arp.status = 0;
            }
        }
        // iface_enumerate(0, 0,
        //                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char, _:  *mut libc::c_char, _: size_t, _:  *mut libc::c_void) -> libc::c_int>, Option<unsafe extern "C" fn() ->
        //                                                        libc::c_int>>(Some(filter_mac);
        /* Remove all unconfirmed entries to old list. */
        // arp = arps;
        // up = &mut arps;
        // while !arp.is_null() {
        //     tmp = arp.next;
        //     if arp.status as libc::c_int == 0 as libc::c_int {
        //         *up = arp.next;
        //         arp.next = old;
        //         old = arp
        //     } else { up = &mut arp.next }
        //     arp = tmp
        // }
        //
        // for arp in arps {
        //     if arp.status == 0 {
        //
        //     }
        // }
    }
    /* record failure, so we don't consult the kernel each time
     we're asked for this address */
    // if !freelist.is_null() {
    //     arp = freelist;
    //     freelist = (*freelist).next
    // } else {
    //     arp = Default::default();
    // }
    arp = Default::default();
    arp.status = 3;
    arp.family = addr.sa.sa_family;
    arp.hwlen = 0;
    if addr.sa.sa_family == 2 {
        arp.addr.addr4.s_addr = addr.in_0.sin_addr.s_addr
    } else {
        memcpy(&mut arp.addr.addr6 as *mut in6_addr as
                   *mut libc::c_void,
               &mut addr.in6.sin6_addr as *mut in6_addr as
                   *const libc::c_void,
               16 as libc::c_int as libc::c_ulong);
    }

    return 0;
}

pub fn do_arp_script_run(daemon: &mut dnsmasq_daemon) -> libc::c_int {
    let mut arp: *mut arp_record = 0 as *mut arp_record;

    /* Notify any which went, then move to free list */
    if !old.is_null() {
        if daemon.options[(53).wrapping_div((()).wrapping_mul(8))] & (1) << (53).wrapping_rem((()).wrapping_mul(8)) != 0 {
            queue_arp(7 as libc::c_int, (*old).hwaddr.as_mut_ptr(), (*old).hwlen, (*old).family, &mut (*old).addr);
        }
        arp = old;
        old = arp.next;
        arp.next = freelist;
        freelist = arp;
        return 1 as libc::c_int
    }
    arp = arps;
    while !arp.is_null() {
        if arp.status as libc::c_int == 2 as libc::c_int {
            if (daemon).options[(53).wrapping_div(().wrapping_mul(8))] & (1) << (53).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8))
                   != false {
                queue_arp(6, arp.hwaddr, arp.hwlen, arp.family, arp.addr);
            }
            arp.status = 1;
            return 1
        }
        // arp = (*arp).next
    }
    return 0;
}
