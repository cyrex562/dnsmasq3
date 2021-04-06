
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
use crate::defines::{DhcpLease, SlaacAddress, DhcpContext, DnsmasqDaemon, In6Addr, NetAddress, C2RustUnnamed, SaFamily, __bswap_16, ConstNetAddressArg, socklen_t};
use crate::radv::ra_start_unsolicited;
use crate::lease::lease_update_dns;
use crate::slack::{ping_packet, IPPROTO_ICMPV6};
use crate::outpacket::{reset_counter, expand, save_counter};
use crate::dnsmasq_log::my_syslog;
use std::time;

static mut ping_id: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn slaac_add_addrs(mut lease: DhcpLease,
                                         mut now: time::Instant,
                                         mut force: i32) {
    let mut slaac: SlaacAddress = 0 ;
    let mut old: SlaacAddress = 0 ;
    let mut up: SlaacAddress;
    let mut context: DhcpContext = 0;
    let mut dns_dirty: i32 = 0;
    if lease.flags & 128 == 0 ||
           lease.flags & (64 | 32) != 0 ||
           lease.last_interface == 0 ||
           lease.hostname.is_null() {
        return
    }
    old = lease.slaac_address;
    lease.slaac_address = 0 ;
    let mut current_block_31: u64;
    context = daemon.dhcp6;
    while !context.is_null() {
        if context.flags &
               (1) << 6 != 0 &&
               context.flags &
                   (1) << 16 == 0 &&
               lease.last_interface == context.if_index {
            let mut addr: In6Addr = context.start6;
            if lease.hwaddr_len == 6 &&
                   (lease.hwaddr_type == 1 ||
                        lease.hwaddr_type == 6) {
                /* convert MAC address to EUI-64 */
                memcpy(&mut *addr.__in6_u.__u6_addr8.as_mut_ptr().offset(8                          libc::c_int
                                                                )
                           ,
                       lease.hwaddr.as_mut_ptr(),
                       3);
                memcpy(&mut *addr.__in6_u.__u6_addr8.as_mut_ptr().offset(13                          libc::c_int
                                                                )
                           ,
                       &mut *lease.hwaddr.as_mut_ptr().offset(3                  libc::c_int
                                                                    )
                          ,
                       3);
                addr.__in6_u.__u6_addr8[11 ] =
                    0xff as u8;
                addr.__in6_u.__u6_addr8[12 ] =
                    0xfe as u8;
                current_block_31 = 12039483399334584727;
            } else if lease.hwaddr_len == 8 &&
                          lease.hwaddr_type == 27 {
                memcpy(&mut *addr.__in6_u.__u6_addr8.as_mut_ptr().offset(8                          libc::c_int
                                                                )
                           ,
                       lease.hwaddr.as_mut_ptr(),
                       8);
                current_block_31 = 12039483399334584727;
            } else if lease.clid_len == 9 &&
                          *lease.clid.offset(0) == 27 &&
                          lease.hwaddr_type == 24 {
                /* firewire has EUI-64 identifier as clid */
                memcpy(&mut *addr.__in6_u.__u6_addr8.as_mut_ptr().offset(8                          libc::c_int
                                                                )
                           ,
                       &mut *lease.clid.offset(1)
                          ,
                       8);
                current_block_31 = 12039483399334584727;
            } else { current_block_31 = 6873731126896040597; }
            match current_block_31 {
                6873731126896040597 => { }
                _ => {
                    addr.__in6_u.__u6_addr8[8 ] =
                        (addr.__in6_u.__u6_addr8[8 ]                       libc::c_int ^ 0x2) as u8;
                    /* check if we already have this one */
                    up = &mut old;
                    slaac = old;
                    while !slaac.is_null() {
                        if ({
                                let mut __a: *const In6Addr =
                                    &mut addr                                  *const In6Addr;
                                let mut __b: *const In6Addr =
                                    &mut slaac.addr                                  *const In6Addr;
                                (__a.__in6_u.__u6_addr32[0] ==
                                     __b.__in6_u.__u6_addr32[0       ]
                                     &&
                                     __a.__in6_u.__u6_addr32[1       ]
                                         ==
                                         __b.__in6_u.__u6_addr32[1                     libc::c_int
                                                                                            usize]
                                     &&
                                     __a.__in6_u.__u6_addr32[2       ]
                                         ==
                                         __b.__in6_u.__u6_addr32[2                     libc::c_int
                                                                                            usize]
                                     &&
                                     __a.__in6_u.__u6_addr32[3       ]
                                         ==
                                         __b.__in6_u.__u6_addr32[3                     libc::c_int
                                                                                            usize])

                            }) != 0 {
                            *up = slaac.next;
                            /* recheck when DHCPv4 goes through init-reboot */
                            if force != 0 {
                                slaac.ping_time = now;
                                slaac.backoff = 1;
                                dns_dirty = 1
                            }
                            break ;
                        } else {
                            up = &mut slaac.next;
                            slaac = slaac.next
                        }
                    }
                    /* No, make new one */
                    if slaac.is_null() &&
                           {
                               slaac =
                                   whine_malloc(::std::mem::size_of::<SlaacAddress>()
                                                   )                                 SlaacAddress;
                               !slaac.is_null()
                           } {
                        slaac.ping_time = now;
                        slaac.backoff = 1;
                        slaac.addr = addr;
                        /* Do RA's to prod it */
                        ra_start_unsolicited(now, context);
                    }
                    if !slaac.is_null() {
                        slaac.next = lease.slaac_address;
                        lease.slaac_address = slaac
                    }
                }
            }
        }
        context = context.next
    }
    if !old.is_null() || dns_dirty != 0 {
        lease_update_dns(1);
    }
    /* Free any no reused */
    while !old.is_null() {
        slaac = old.next;
        // free(old);
        old = slaac
    };
}
#[no_mangle]
pub unsafe extern "C" fn periodic_slaac(mut now: time::Instant,
                                        mut leases: DhcpLease)
 -> time::Instant {
    let mut context: DhcpContext = 0;
    let mut lease: DhcpLease = 0;
    let mut slaac: SlaacAddress = 0 ;
    let mut next_event: time::Instant = 0;
    context = daemon.dhcp6;
    while !context.is_null() {
        if context.flags &
               (1) << 6 != 0 &&
               context.flags &
                   (1) << 16 == 0 {
            break ;
        }
        context = context.next
    }
    /* nothing configured */
    if context.is_null() { return 0 }
    while ping_id == 0 { ping_id = rand16() }
    lease = leases;
    while !lease.is_null() {
        let mut current_block_26: u64;
        slaac = lease.slaac_address;
        while !slaac.is_null() {
            /* confirmed or given up? */
            if !(slaac.backoff == 0 ||
                     slaac.ping_time == 0) {
                if difftime(slaac.ping_time, now) <= 0.0f64 {
                    let mut ping: ping_packet =
                        0 ; /* Give up */
                    let mut addr: NetAddress =
                        NetAddress {sin6_family: 0,
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
                        expand(::std::mem::size_of::<ping_packet>()) ;
                    if ping.is_null() {
                        current_block_26 = 12209867499936983673;
                    } else {
                        ping.type_0 = 128 as u8;
                        ping.code = 0 as u8;
                        ping.identifier = ping_id;
                        ping.sequence_no = slaac.backoff;
                        addr = Default::default();
                        addr.sin6_family = 10;
                        addr.sin6_port = IPPROTO_ICMPV6;
                        addr.sin6_addr = slaac.addr;
                        if sendto(daemon.icmp6fd,
                                  daemon.outpacket.iov_base,
                                  save_counter(-(1)) ,
                                  0,
                                  ConstNetAddressArg {__NetAddress__:
                                                           &mut addr            NetAddress
                                                                          NetAddress,},
                                  ::std::mem::size_of::<NetAddress>()) ==
                               -(1) &&
                               *__errno_location() == 113 &&
                               slaac.backoff == 12 {
                            slaac.ping_time = 0
                        } else {
                            slaac.ping_time +=
                                (((1) <<
                                      slaac.backoff - 1) +
                                     rand16() /
                                         21785)                              i32;
                            if slaac.backoff > 4 {
                                slaac.ping_time +=
                                    (rand16() /
                                         4000)
                            }
                            if slaac.backoff < 12 {
                                slaac.backoff += 1
                            }
                        }
                        current_block_26 = 3275366147856559585;
                    }
                } else { current_block_26 = 3275366147856559585; }
                match current_block_26 {
                    12209867499936983673 => { }
                    _ => {
                        if slaac.ping_time !=
                               0 &&
                               (next_event == 0
                                    ||
                                    difftime(next_event, slaac.ping_time)
                                        >= 0.0f64) {
                            next_event = slaac.ping_time
                        }
                    }
                }
            }
            slaac = slaac.next
        }
        lease = lease.next
    }
    return next_event;
}
#[no_mangle]
pub unsafe extern "C" fn slaac_ping_reply(mut sender: &mut In6Addr,
                                          mut packet: mut Vec<u8>,
                                          mut interface: &mut String,
                                          mut leases: DhcpLease) {
    let mut lease: DhcpLease = 0;
    let mut slaac: SlaacAddress = 0 ;
    let mut ping: ping_packet = packet ;
    let mut gotone: i32 = 0;
    if ping.identifier == ping_id {
        lease = leases;
        while !lease.is_null() {
            slaac = lease.slaac_address;
            while !slaac.is_null() {
                if slaac.backoff != 0 &&
                       ({
                            let mut __a: *const In6Addr =
                                sender ;
                            let mut __b: *const In6Addr =
                                &mut slaac.addr;
                            (__a.__in6_u.__u6_addr32[0         usize] ==
                                 __b.__in6_u.__u6_addr32[0] &&
                                 __a.__in6_u.__u6_addr32[1] ==
                                     __b.__in6_u.__u6_addr32[1       ]
                                 &&
                                 __a.__in6_u.__u6_addr32[2] ==
                                     __b.__in6_u.__u6_addr32[2       ]
                                 &&
                                 __a.__in6_u.__u6_addr32[3] ==
                                     __b.__in6_u.__u6_addr32[3       ])

                        }) != 0 {
                    slaac.backoff = 0;
                    gotone = 1;
                    inet_ntop(10,
                              sender,
                              daemon.addrbuff,
                              46);
                    if daemon.options[(43   libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                   ).wrapping_mul(8                                                             libc::c_int                                                      ))
                                                     ] &
                           (1) <<
                               (43                       ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                               ).wrapping_mul(8                         libc::c_int                  ))
                           == 0 {
                        my_syslog((3) << 3 |
                                      6,
                                  "SLAAC-CONFIRM(%s) %s %s"        , interface,
                                  daemon.addrbuff,
                                  lease.hostname);
                    }
                }
                slaac = slaac.next
            }
            lease = lease.next
        }
    }
    lease_update_dns(gotone);
}
