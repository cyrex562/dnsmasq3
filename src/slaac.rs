
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
use crate::defines::{DhcpLease, time_t, SlaacAddress, DhcpContext, DnsmasqDaemon, In6Addr, SockAddrIn6, C2RustUnnamed, SaFamily, __bswap_16, ConstSockaddrArg, SockAddr, socklen_t};
use crate::util::{whine_malloc, rand16};
use crate::radv::ra_start_unsolicited;
use crate::lease::lease_update_dns;
use crate::slack::{ping_packet, IPPROTO_ICMPV6};
use crate::outpacket::{reset_counter, expand, save_counter};
use crate::dnsmasq_log::my_syslog;

static mut ping_id: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn slaac_add_addrs(mut lease: *mut DhcpLease,
                                         mut now: time_t,
                                         mut force: libc::c_int) {
    let mut slaac: *mut SlaacAddress = 0 as *mut SlaacAddress;
    let mut old: *mut SlaacAddress = 0 as *mut SlaacAddress;
    let mut up: *mut *mut SlaacAddress = 0 as *mut *mut SlaacAddress;
    let mut context: *mut DhcpContext = 0 as *mut DhcpContext;
    let mut dns_dirty: libc::c_int = 0 as libc::c_int;
    if (*lease).flags & 128 as libc::c_int == 0 ||
           (*lease).flags & (64 as libc::c_int | 32 as libc::c_int) != 0 ||
           (*lease).last_interface == 0 as libc::c_int ||
           (*lease).hostname.is_null() {
        return
    }
    old = (*lease).slaac_address;
    (*lease).slaac_address = 0 as *mut SlaacAddress;
    let mut current_block_31: u64;
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 6 as libc::c_int != 0 &&
               (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 16 as libc::c_int == 0 &&
               (*lease).last_interface == (*context).if_index {
            let mut addr: In6Addr = (*context).start6;
            if (*lease).hwaddr_len == 6 as libc::c_int &&
                   ((*lease).hwaddr_type == 1 as libc::c_int ||
                        (*lease).hwaddr_type == 6 as libc::c_int) {
                /* convert MAC address to EUI-64 */
                memcpy(&mut *addr.__in6_u.__u6_addr8.as_mut_ptr().offset(8 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                           as *mut u8 as *mut libc::c_void,
                       (*lease).hwaddr.as_mut_ptr() as *const libc::c_void,
                       3 as libc::c_int as libc::c_ulong);
                memcpy(&mut *addr.__in6_u.__u6_addr8.as_mut_ptr().offset(13 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                           as *mut u8 as *mut libc::c_void,
                       &mut *(*lease).hwaddr.as_mut_ptr().offset(3 as
                                                                     libc::c_int
                                                                     as isize)
                           as *mut libc::c_uchar as *const libc::c_void,
                       3 as libc::c_int as libc::c_ulong);
                addr.__in6_u.__u6_addr8[11 as libc::c_int as usize] =
                    0xff as libc::c_int as u8;
                addr.__in6_u.__u6_addr8[12 as libc::c_int as usize] =
                    0xfe as libc::c_int as u8;
                current_block_31 = 12039483399334584727;
            } else if (*lease).hwaddr_len == 8 as libc::c_int &&
                          (*lease).hwaddr_type == 27 as libc::c_int {
                memcpy(&mut *addr.__in6_u.__u6_addr8.as_mut_ptr().offset(8 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                           as *mut u8 as *mut libc::c_void,
                       (*lease).hwaddr.as_mut_ptr() as *const libc::c_void,
                       8 as libc::c_int as libc::c_ulong);
                current_block_31 = 12039483399334584727;
            } else if (*lease).clid_len == 9 as libc::c_int &&
                          *(*lease).clid.offset(0 as libc::c_int as isize) as
                              libc::c_int == 27 as libc::c_int &&
                          (*lease).hwaddr_type == 24 as libc::c_int {
                /* firewire has EUI-64 identifier as clid */
                memcpy(&mut *addr.__in6_u.__u6_addr8.as_mut_ptr().offset(8 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)
                           as *mut u8 as *mut libc::c_void,
                       &mut *(*lease).clid.offset(1 as libc::c_int as isize)
                           as *mut libc::c_uchar as *const libc::c_void,
                       8 as libc::c_int as libc::c_ulong);
                current_block_31 = 12039483399334584727;
            } else { current_block_31 = 6873731126896040597; }
            match current_block_31 {
                6873731126896040597 => { }
                _ => {
                    addr.__in6_u.__u6_addr8[8 as libc::c_int as usize] =
                        (addr.__in6_u.__u6_addr8[8 as libc::c_int as usize] as
                             libc::c_int ^ 0x2 as libc::c_int) as u8;
                    /* check if we already have this one */
                    up = &mut old;
                    slaac = old;
                    while !slaac.is_null() {
                        if ({
                                let mut __a: *const In6Addr =
                                    &mut addr as *mut In6Addr as
                                        *const In6Addr;
                                let mut __b: *const In6Addr =
                                    &mut (*slaac).addr as *mut In6Addr as
                                        *const In6Addr;
                                ((*__a).__in6_u.__u6_addr32[0 as libc::c_int
                                                                as usize] ==
                                     (*__b).__in6_u.__u6_addr32[0 as
                                                                    libc::c_int
                                                                    as usize]
                                     &&
                                     (*__a).__in6_u.__u6_addr32[1 as
                                                                    libc::c_int
                                                                    as usize]
                                         ==
                                         (*__b).__in6_u.__u6_addr32[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                     &&
                                     (*__a).__in6_u.__u6_addr32[2 as
                                                                    libc::c_int
                                                                    as usize]
                                         ==
                                         (*__b).__in6_u.__u6_addr32[2 as
                                                                        libc::c_int
                                                                        as
                                                                        usize]
                                     &&
                                     (*__a).__in6_u.__u6_addr32[3 as
                                                                    libc::c_int
                                                                    as usize]
                                         ==
                                         (*__b).__in6_u.__u6_addr32[3 as
                                                                        libc::c_int
                                                                        as
                                                                        usize])
                                    as libc::c_int
                            }) != 0 {
                            *up = (*slaac).next;
                            /* recheck when DHCPv4 goes through init-reboot */
                            if force != 0 {
                                (*slaac).ping_time = now;
                                (*slaac).backoff = 1 as libc::c_int;
                                dns_dirty = 1 as libc::c_int
                            }
                            break ;
                        } else {
                            up = &mut (*slaac).next;
                            slaac = (*slaac).next
                        }
                    }
                    /* No, make new one */
                    if slaac.is_null() &&
                           {
                               slaac =
                                   whine_malloc(::std::mem::size_of::<SlaacAddress>()
                                                    as libc::c_ulong) as
                                       *mut SlaacAddress;
                               !slaac.is_null()
                           } {
                        (*slaac).ping_time = now;
                        (*slaac).backoff = 1 as libc::c_int;
                        (*slaac).addr = addr;
                        /* Do RA's to prod it */
                        ra_start_unsolicited(now, context);
                    }
                    if !slaac.is_null() {
                        (*slaac).next = (*lease).slaac_address;
                        (*lease).slaac_address = slaac
                    }
                }
            }
        }
        context = (*context).next
    }
    if !old.is_null() || dns_dirty != 0 {
        lease_update_dns(1 as libc::c_int);
    }
    /* Free any no reused */
    while !old.is_null() {
        slaac = (*old).next;
        free(old as *mut libc::c_void);
        old = slaac
    };
}
#[no_mangle]
pub unsafe extern "C" fn periodic_slaac(mut now: time_t,
                                        mut leases: *mut DhcpLease)
 -> time_t {
    let mut context: *mut DhcpContext = 0 as *mut DhcpContext;
    let mut lease: *mut DhcpLease = 0 as *mut DhcpLease;
    let mut slaac: *mut SlaacAddress = 0 as *mut SlaacAddress;
    let mut next_event: time_t = 0 as libc::c_int as time_t;
    context = (*dnsmasq_daemon).dhcp6;
    while !context.is_null() {
        if (*context).flags as libc::c_uint &
               (1 as libc::c_uint) << 6 as libc::c_int != 0 &&
               (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 16 as libc::c_int == 0 {
            break ;
        }
        context = (*context).next
    }
    /* nothing configured */
    if context.is_null() { return 0 as libc::c_int as time_t }
    while ping_id == 0 as libc::c_int { ping_id = rand16() as libc::c_int }
    lease = leases;
    while !lease.is_null() {
        let mut current_block_26: u64;
        slaac = (*lease).slaac_address;
        while !slaac.is_null() {
            /* confirmed or given up? */
            if !((*slaac).backoff == 0 as libc::c_int ||
                     (*slaac).ping_time == 0 as libc::c_int as libc::c_long) {
                if difftime((*slaac).ping_time, now) <= 0.0f64 {
                    let mut ping: *mut ping_packet =
                        0 as *mut ping_packet; /* Give up */
                    let mut addr: SockAddrIn6 =
                        SockAddrIn6 {sin6_family: 0,
                                     sin6_port: 0,
                                     sin6_flowinfo: 0,
                                     sin6_addr:
                                         In6Addr {__in6_u:
                                                      C2RustUnnamed{__u6_addr8:
                                                                        [0;
                                                                            16],},},
                                     sin6_scope_id: 0,}; /* 0 - 3 */
                    reset_counter(); /* 0 - 15 */
                    ping =
                        expand(::std::mem::size_of::<ping_packet>() as
                                   libc::c_ulong) as *mut ping_packet;
                    if ping.is_null() {
                        current_block_26 = 12209867499936983673;
                    } else {
                        (*ping).type_0 = 128 as libc::c_int as u8;
                        (*ping).code = 0 as libc::c_int as u8;
                        (*ping).identifier = ping_id as u16;
                        (*ping).sequence_no = (*slaac).backoff as u16;
                        memset(&mut addr as *mut SockAddrIn6 as
                                   *mut libc::c_void, 0 as libc::c_int,
                               ::std::mem::size_of::<SockAddrIn6>() as
                                   libc::c_ulong);
                        addr.sin6_family = 10 as libc::c_int as SaFamily;
                        addr.sin6_port =
                            __bswap_16(IPPROTO_ICMPV6 as libc::c_int as
                                           u16);
                        addr.sin6_addr = (*slaac).addr;
                        if sendto((*dnsmasq_daemon).icmp6fd,
                                  (*dnsmasq_daemon).outpacket.iov_base,
                                  save_counter(-(1 as libc::c_int)) as usize,
                                  0 as libc::c_int,
                                  ConstSockaddrArg {__sockaddr__:
                                                           &mut addr as
                                                               *mut SockAddrIn6
                                                               as
                                                               *mut SockAddr,},
                                  ::std::mem::size_of::<SockAddrIn6>() as
                                      libc::c_ulong as socklen_t) ==
                               -(1 as libc::c_int) as libc::c_long &&
                               *__errno_location() == 113 as libc::c_int &&
                               (*slaac).backoff == 12 as libc::c_int {
                            (*slaac).ping_time = 0 as libc::c_int as time_t
                        } else {
                            (*slaac).ping_time +=
                                (((1 as libc::c_int) <<
                                      (*slaac).backoff - 1 as libc::c_int) +
                                     rand16() as libc::c_int /
                                         21785 as libc::c_int) as
                                    libc::c_long;
                            if (*slaac).backoff > 4 as libc::c_int {
                                (*slaac).ping_time +=
                                    (rand16() as libc::c_int /
                                         4000 as libc::c_int) as libc::c_long
                            }
                            if (*slaac).backoff < 12 as libc::c_int {
                                (*slaac).backoff += 1
                            }
                        }
                        current_block_26 = 3275366147856559585;
                    }
                } else { current_block_26 = 3275366147856559585; }
                match current_block_26 {
                    12209867499936983673 => { }
                    _ => {
                        if (*slaac).ping_time !=
                               0 as libc::c_int as libc::c_long &&
                               (next_event == 0 as libc::c_int as libc::c_long
                                    ||
                                    difftime(next_event, (*slaac).ping_time)
                                        >= 0.0f64) {
                            next_event = (*slaac).ping_time
                        }
                    }
                }
            }
            slaac = (*slaac).next
        }
        lease = (*lease).next
    }
    return next_event;
}
#[no_mangle]
pub unsafe extern "C" fn slaac_ping_reply(mut sender: *mut In6Addr,
                                          mut packet: *mut libc::c_uchar,
                                          mut interface: *mut libc::c_char,
                                          mut leases: *mut DhcpLease) {
    let mut lease: *mut DhcpLease = 0 as *mut DhcpLease;
    let mut slaac: *mut SlaacAddress = 0 as *mut SlaacAddress;
    let mut ping: *mut ping_packet = packet as *mut ping_packet;
    let mut gotone: libc::c_int = 0 as libc::c_int;
    if (*ping).identifier as libc::c_int == ping_id {
        lease = leases;
        while !lease.is_null() {
            slaac = (*lease).slaac_address;
            while !slaac.is_null() {
                if (*slaac).backoff != 0 as libc::c_int &&
                       ({
                            let mut __a: *const In6Addr =
                                sender as *const In6Addr;
                            let mut __b: *const In6Addr =
                                &mut (*slaac).addr as *mut In6Addr as
                                    *const In6Addr;
                            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as
                                                            usize] ==
                                 (*__b).__in6_u.__u6_addr32[0 as libc::c_int
                                                                as usize] &&
                                 (*__a).__in6_u.__u6_addr32[1 as libc::c_int
                                                                as usize] ==
                                     (*__b).__in6_u.__u6_addr32[1 as
                                                                    libc::c_int
                                                                    as usize]
                                 &&
                                 (*__a).__in6_u.__u6_addr32[2 as libc::c_int
                                                                as usize] ==
                                     (*__b).__in6_u.__u6_addr32[2 as
                                                                    libc::c_int
                                                                    as usize]
                                 &&
                                 (*__a).__in6_u.__u6_addr32[3 as libc::c_int
                                                                as usize] ==
                                     (*__b).__in6_u.__u6_addr32[3 as
                                                                    libc::c_int
                                                                    as usize])
                                as libc::c_int
                        }) != 0 {
                    (*slaac).backoff = 0 as libc::c_int;
                    gotone = 1 as libc::c_int;
                    inet_ntop(10 as libc::c_int,
                              sender as *const libc::c_void,
                              (*dnsmasq_daemon).addrbuff,
                              46 as libc::c_int as socklen_t);
                    if (*dnsmasq_daemon).options[(43 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (43 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           == 0 {
                        my_syslog((3 as libc::c_int) << 3 as libc::c_int |
                                      6 as libc::c_int,
                                  b"SLAAC-CONFIRM(%s) %s %s\x00" as *const u8
                                      as *const libc::c_char, interface,
                                  (*dnsmasq_daemon).addrbuff,
                                  (*lease).hostname);
                    }
                }
                slaac = (*slaac).next
            }
            lease = (*lease).next
        }
    }
    lease_update_dns(gotone);
}
