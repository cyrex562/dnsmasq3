
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
use crate::defines::{DhcpLease, time::Instant, FILE, NetAddress, NetAddress, DnsmasqDaemon, In6Addr, DhcpConfig, DhcpContext, DhcpNetId, Off64T, socklen_t, SlaacAddress, __bswap_32, DhcpLeaseContext};
use crate::util::{parse_hex, netmask_length, is_same_net4, is_same_net6, addr6part, whine_malloc, hostname_isequal};
use crate::dnsmasq_log::{my_syslog, die};
use crate::domain::{get_domain, get_domain6};
use crate::dhcp_common::find_config;
use crate::dhcp::host_from_dns;
use std::env::args;
use crate::slaac::{periodic_slaac, slaac_ping_reply, slaac_add_addrs};
use crate::radv::periodic_ra;
use crate::send_alarm;
use crate::netlink::iface_enumerate;
use crate::dhcp6::make_duid;
use crate::cache::{cache_unhash_dhcp, cache_add_dhcp_entry};
use crate::slack::{METRIC_LEASES_PRUNED_4, METRIC_LEASES_PRUNED_6, METRIC_LEASES_ALLOCATED_4, METRIC_LEASES_ALLOCATED_6};
use crate::helper::queue_script;
use std::time;

// static mut leases: DhcpLease =
//     0 ;
// static mut old_leases: DhcpLease =
//     0 ;
// static mut dns_dirty: i32 = 0;
// static mut file_dirty: i32 = 0;
// static mut leases_left: i32 = 0;
unsafe extern "C" fn read_leases(daemon: &mut DnsmasqDaemon, lease_ctx: &mut DhcpLeaseContext, mut now: time::Instant, mut leasestream: &mut std::fs::File)
 -> i32 {
    let mut ei: u32 = 0;
    let mut addr: NetAddress = NetAddress:new();
    let mut lease: DhcpLease = Default::default();
    let mut clid_len: i32 = 0;
    let mut hw_len: i32 = 0;
    let mut hw_type: i32 = 0;
    let mut items: i32 = 0;
    let mut domain: String;
    daemon.dhcp_buff2.clear();
    daemon.dhcp_buff3.clear();
    loop 
         /* client-id max length is 255 which is 255*2 digits + 254 colons
     borrow DNS packet buffer which is always larger than 1000 bytes

     Check various buffers are big enough for the code below */
         {
        items =
            fscanf(leasestream,
                   "%255s %255s" ,
                   daemon.dhcp_buff3,
                   daemon.dhcp_buff2);
        if !(items == 2) { break ; }
        daemon.packet.clear();
        daemon.dhcp_buff.clear();
        daemon.namebuff.clear();
        clid_len = 0;
        hw_type = clid_len;
        hw_len = hw_type;
        if strcmp(daemon.dhcp_buff3,
                  "duid" ) ==
               0 {
            daemon.duid_len =
                parse_hex(daemon.dhcp_buff2, daemon.dhcp_buff2, 130, 0, 0);
            if daemon.duid_len < 0 {
                return 0
            }
            daemon.duid = Vec::new();
            daemon.duid.copy_from_slice(daemon.dhcp_buff2[0..daemon.duid_len]);
        } else if fscanf(leasestream, " %64s %255s %764s", daemon.namebuff,
                         daemon.dhcp_buff,
                         daemon.packet) != 3 {
            my_syslog((3) << 3 |
                          4,
                      "ignoring invalid line in lease database: %s %s %s %s ..."
                          ,
                      daemon.dhcp_buff3,
                      daemon.dhcp_buff2,
                      daemon.namebuff,
                      daemon.dhcp_buff);
        } else {
            if inet_pton(2, daemon.namebuff,
                         &mut addr.addr4)
                   != 0 {
                lease = lease4_allocate(addr.addr4);
                if !lease.is_null() { domain = get_domain(lease.addr) }
                hw_len =
                    parse_hex(daemon.dhcp_buff2,
                              daemon.dhcp_buff2                            mut Vec<u8>, 16,
                              0, &mut hw_type);
                /* For backwards compatibility, no explicit MAC address type means ether. */
                if hw_type == 0 && hw_len != 0 {
                    hw_type = 1
                }
            } else if inet_pton(10, daemon.namebuff,
                                &mut addr.addr6                             Vec<u8>) != 0 {
                let mut s: &mut String = daemon.dhcp_buff2;
                let mut lease_type: i32 = 32;
                if *s.offset(0) ==
                       'T' as i32 {
                    lease_type = 64;
                    s = s.offset(1)
                }
                lease = lease6_allocate(&mut addr.addr6, lease_type);
                if !lease.is_null() {
                    lease_set_iaid(lease,
                                   strtoul(s, 0 ,
                                           10)                                 libc::c_uint);
                    domain = get_domain6(&mut lease.addr6)
                }
            } else {
                my_syslog((3) << 3 |
                              4,
                          "ignoring invalid line in lease database, bad address: %s"
                              ,
                          daemon.namebuff);
                continue ;
            }
            if lease.is_null() {
                die("too many stored leases"                   *const libc::c_char ,
                    0 , 5);
            }
            if strcmp(daemon.packet,
                      "*" ) !=
                   0 {
                clid_len =
                    parse_hex(daemon.packet,
                              daemon.packet,
                              255, 0,
                              0)
            }
            lease_set_hwaddr(lease,
                             daemon.dhcp_buff2                           mut Vec<u8>,
                             daemon.packet,
                             hw_len, hw_type, clid_len, now,
                             0);
            if strcmp(daemon.dhcp_buff,
                      "*" ) !=
                   0 {
                lease_set_hostname(lease, daemon.dhcp_buff,
                                   0, domain,
                                   0 );
            }
            ei = atol(daemon.dhcp_buff3);
            /* strictly time_t is opaque, but this hack should work on all sane systems,
	   even when sizeof(time_t) == 8 */
            lease.expires = ei;
            /* set these correctly: the "old" events are generated later from
	   the startup synthesised SIGHUP. */
            lease.flags &= !(1 | 2);
            daemon.dhcp_buff2 = '\u{0}' ;
            daemon.dhcp_buff3 = daemon.dhcp_buff2
        }
    }
    return (items == 0 || items == -(1)) ;
}
#[no_mangle]
pub unsafe extern "C" fn lease_init(mut now: time::Instant) {
    let mut leasestream: FILE = 0 ;
    leases_left = daemon.dhcp_max;
    if daemon.options[(22).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                                                   ))
                                     ] &
           (1) <<
               (22).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8

                                                                                                               ))
           != 0 {
        /* run "<lease_change_script> init" once to get the
	 initial state of the database. If leasefile-ro is
	 set without a script, we just do without any
	 lease database. */
        if !daemon.lease_change_command.is_null() {
            strcpy(daemon.dhcp_buff,
                   daemon.lease_change_command);
            strcat(daemon.dhcp_buff,
                   " init" );
            leasestream =
                popen(daemon.dhcp_buff,
                      "r" )
        } else {
            dns_dirty = 0;
            file_dirty = dns_dirty;
            return
        }
    } else {
        /* NOTE: need a+ mode to create file if it doesn't exist */
        daemon.lease_stream =
            fopen(daemon.lease_file,
                  "a+" );
        leasestream = daemon.lease_stream;
        if leasestream.is_null() {
            die("cannot open or create lease file %s: %s"               *const libc::c_char ,
                daemon.lease_file, 3);
        }
        /* a+ mode leaves pointer at end. */
        rewind(leasestream);
    }
    if !leasestream.is_null() {
        if read_leases(now, leasestream) == 0 {
            my_syslog((3) << 3 |
                          3,
                      "failed to parse lease database cleanly"                    *const u8);
        }
        if ferror(leasestream) != 0 {
            die("failed to read lease file %s: %s"               *const libc::c_char ,
                daemon.lease_file, 3);
        }
    }
    if daemon.lease_stream.is_null() {
        let mut rc: i32 = 0;
        /* shell returns 127 for "command not found", 126 for bad permissions. */
        if leasestream.is_null() ||
               { rc = pclose(leasestream); (rc) == -(1) } ||
               (rc & 0xff00) >> 8 ==
                   127 ||
               (rc & 0xff00) >> 8 ==
                   126 {
            if (rc & 0xff00) >> 8 ==
                   127 {
                *__errno_location() = 2
            } else if (rc & 0xff00) >> 8 ==
                          126 {
                *__errno_location() = 13
            }
            die("cannot run lease-init script %s: %s"               *const libc::c_char ,
                daemon.lease_change_command, 3);
        }
        if (rc & 0xff00) >> 8 !=
               0 {
            sprintf(daemon.dhcp_buff,
                    "%d" ,
                    (rc & 0xff00) >> 8);
            die("lease-init script returned exit code %s"               *const libc::c_char ,
                daemon.dhcp_buff,
                ((rc & 0xff00) >> 8) +
                    10);
        }
    }
    /* Some leases may have expired */
    file_dirty = 0;
    lease_prune(0, now);
    dns_dirty = 1;
}
#[no_mangle]
pub unsafe extern "C" fn lease_update_from_configs() {
    /* changes to the config may change current leases. */
    let mut lease: DhcpLease = 0;
    let mut config: DhcpConfig = 0;
    let mut name: &mut String = 0 ;
    lease = leases;
    while !lease.is_null() {
        if !(lease.flags & (64 | 32) != 0) {
            config =
                find_config(daemon.dhcp_conf,
                            0, lease.clid,
                            lease.clid_len, lease.hwaddr.as_mut_ptr(),
                            lease.hwaddr_len, lease.hwaddr_type,
                            0 , 0 );
            if !config.is_null() &&
                   config.flags & 16 != 0 &&
                   (config.flags & 32 == 0
                        || config.addr.s_addr == lease.addr.s_addr) {
                lease_set_hostname(lease, config.hostname,
                                   1,
                                   get_domain(lease.addr),
                                   0 );
            } else {
                name = host_from_dns(lease.addr);
                if !name.is_null() {
                    lease_set_hostname(lease, name, 1,
                                       get_domain(lease.addr),
                                       0 );
                }
            }
        }
        lease = lease.next
    };
    /* updates auth flag only */
}
unsafe extern "C" fn ourprintf(mut errp: ,
                               mut format: &mut String, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if *errp == 0 &&
           vfprintf(daemon.lease_stream, format, ap.as_va_list()) <
               0 {
        *errp = *__errno_location()
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_update_file(mut now: time::Instant) {
    let mut lease: DhcpLease = 0;
    let mut next_event: time::Instant = 0;
    let mut i: i32 = 0;
    let mut err: i32 = 0;
    if file_dirty != 0 &&
           !daemon.lease_stream.is_null() {
        *__errno_location() = 0;
        rewind(daemon.lease_stream);
        if *__errno_location() != 0 ||
               ftruncate(fileno(daemon.lease_stream),
                         0 as Off64T) != 0 {
            err = *__errno_location()
        }
        lease = leases;
        while !lease.is_null() {
            if !(lease.flags & (64 | 32) !=
                     0) {
                ourprintf(&mut err,
                          "%lu "                         &mut String,
                          lease.expires);
                if lease.hwaddr_type != 1 ||
                       lease.hwaddr_len == 0 {
                    ourprintf(&mut err,
                              "%.2x-"
                                  , lease.hwaddr_type);
                }
                i = 0;
                while i < lease.hwaddr_len {
                    ourprintf(&mut err,
                              "%.2x"
                                  ,
                              lease.hwaddr[i ]);
                    if i != lease.hwaddr_len - 1 {
                        ourprintf(&mut err,
                                  ":"
                                      );
                    }
                    i += 1
                }
                inet_ntop(2,
                          &mut lease.addr       daemon.addrbuff,
                          46);
                ourprintf(&mut err,
                          " %s "                         &mut String, daemon.addrbuff);
                ourprintf(&mut err,
                          "%s "                         &mut String,
                          if !lease.hostname.is_null() {
                              lease.hostname
                          } else {
                              "*"
                          });
                if !lease.clid.is_null() &&
                       lease.clid_len != 0 {
                    i = 0;
                    while i < lease.clid_len - 1 {
                        ourprintf(&mut err,
                                  "%.2x:"                                &mut String,
                                  *lease.clid.offset(i)                                );
                        i += 1
                    }
                    ourprintf(&mut err,
                              "%.2x\n" ,
                              *lease.clid.offset(i)                            );
                } else {
                    ourprintf(&mut err,
                              "*\n"
                                  );
                }
            }
            lease = lease.next
        }
        if !daemon.duid.is_null() {
            ourprintf(&mut err,
                      "duid "                     &mut String);
            i = 0;
            while i < daemon.duid_len - 1 {
                ourprintf(&mut err,
                          "%.2x:"                         &mut String,
                          daemon.duid.offset(i));
                i += 1
            }
            ourprintf(&mut err,
                      "%.2x\n"                     &mut String,
                      daemon.duid.offset(i));
            lease = leases;
            while !lease.is_null() {
                if !(lease.flags & (64 | 32)
                         == 0) {
                    ourprintf(&mut err,
                              "%lu "
                                  ,
                              lease.expires);
                    inet_ntop(10,
                              &mut lease.addr6
                              daemon.addrbuff,
                              46);
                    ourprintf(&mut err,
                              "%s%u %s " ,
                              if lease.flags & 64 != 0 {
                                  "T"
                              } else {
                                  ""
                              }, lease.iaid, daemon.addrbuff);
                    ourprintf(&mut err,
                              "%s "
                                  ,
                              if !lease.hostname.is_null() {
                                  lease.hostname
                              } else {
                                  "*"
                              });
                    if !lease.clid.is_null() &&
                           lease.clid_len != 0 {
                        i = 0;
                        while i < lease.clid_len - 1 {
                            ourprintf(&mut err,
                                      "%.2x:"                                     *const libc::c_char                                    &mut String,
                                      *lease.clid.offset(i)                                    );
                            i += 1
                        }
                        ourprintf(&mut err,
                                  "%.2x\n"                                &mut String,
                                  *lease.clid.offset(i)                                );
                    } else {
                        ourprintf(&mut err,
                                  "*\n"                                &mut String);
                    }
                }
                lease = lease.next
            }
        }
        if fflush(daemon.lease_stream) != 0 ||
               fsync(fileno(daemon.lease_stream)) <
                   0 {
            err = *__errno_location()
        }
        if err == 0 { file_dirty = 0 }
    }
    /* Set alarm for when the first lease expires. */
    next_event = 0;
    /* do timed RAs and determine when the next is, also pings to potential SLAAC addresses */
    if daemon.doing_ra != 0 {
        let mut event: time::Instant = 0;
        event = periodic_slaac(now, leases);
        if event != 0 {
            if next_event == 0 ||
                   difftime(next_event, event) > 0.0f64 {
                next_event = event
            }
        }
        event = periodic_ra(now);
        if event != 0 {
            if next_event == 0 ||
                   difftime(next_event, event) > 0.0f64 {
                next_event = event
            }
        }
    }
    lease = leases;
    while !lease.is_null() {
        if lease.expires != 0 &&
               (next_event == 0 ||
                    difftime(next_event, lease.expires) > 0.0f64) {
            next_event = lease.expires
        }
        lease = lease.next
    }
    if err != 0 {
        if next_event == 0 ||
               difftime(next_event, 60 + now) >
                   0.0f64 {
            next_event = 60 + now
        }
        my_syslog((3) << 3 | 3,
                  "failed to write %s: %s (retry in %us)", daemon.lease_file,
                  strerror(err), difftime(next_event, now));
    }
    send_alarm(next_event, now);
}
unsafe extern "C" fn find_interface_v4(mut local: NetAddress,
                                       mut if_index: i32,
                                       mut label: &mut String,
                                       mut netmask: NetAddress,
                                       mut broadcast: NetAddress,
                                       mut vparam:Vec<u8>)
                                       -> i32 {
    let mut lease: DhcpLease = 0;
    let mut prefix: i32 = netmask_length(netmask);
    lease = leases;
    while !lease.is_null() {
        if lease.flags & (64 | 32) == 0 &&
               is_same_net4(local, lease.addr, netmask) != 0 &&
               prefix > lease.new_prefixlen {
            lease.new_interface = if_index;
            lease.new_prefixlen = prefix
        }
        lease = lease.next
    }
    return 1;
}
unsafe extern "C" fn find_interface_v6(mut local: &mut In6Addr,
                                       mut prefix: i32,
                                       mut scope: i32,
                                       mut if_index: i32,
                                       mut flags: i32,
                                       mut preferred: i32,
                                       mut valid: i32,
                                       mut vparam:Vec<u8>)
                                       -> i32 {
    let mut lease: DhcpLease = 0;
    lease = leases;
    while !lease.is_null() {
        if lease.flags & (64 | 32) != 0 {
            if is_same_net6(local, &mut lease.addr6, prefix) != 0 &&
                   prefix > lease.new_prefixlen {
                /* save prefix length for comparison, as we might get shorter matching
         * prefix in upcoming netlink GETADDR responses
         * */
                lease.new_interface = if_index;
                lease.new_prefixlen = prefix
            }
        }
        lease = lease.next
    }
    return 1;
}
#[no_mangle]
pub unsafe extern "C" fn lease_ping_reply(mut sender: &mut In6Addr,
                                          mut packet: mut Vec<u8>,
                                          mut interface: &mut String) {
    /* We may be doing RA but not DHCPv4, in which case the lease
     database may not exist and we have nothing to do anyway */
    if !daemon.dhcp.is_null() {
        slaac_ping_reply(sender, packet, interface, leases);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_update_slaac(mut now: time::Instant) {
    /* Called when we construct a new RA-names context, to add putative
     new SLAAC addresses to existing leases. */
    let mut lease: DhcpLease = 0;
    if !daemon.dhcp.is_null() {
        lease = leases;
        while !lease.is_null() {
            slaac_add_addrs(lease, now, 0);
            lease = lease.next
        }
    };
}
/* Find interfaces associated with leases at start-up. This gets updated as
   we do DHCP transactions, but information about directly-connected subnets
   is useful from scrips and necessary for determining SLAAC addresses from
   start-time. */
#[no_mangle]
pub unsafe extern "C" fn lease_find_interfaces(mut now: time::Instant) {
    let mut lease: DhcpLease = 0;
    lease = leases;
    while !lease.is_null() {
        lease.new_interface = 0;
        lease.new_prefixlen = lease.new_interface;
        lease = lease.next
    }
    iface_enumerate(2,
                    &mut now,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        NetAddress,
                                                                        _:
                                                                            ,
                                                                        _:
                                                                            &mut String,
                                                                        _:
                                                                        NetAddress,
                                                                        _:
                                                                        NetAddress,
                                                                        _:
                                                                           Vec<u8>)
                                                                        -> i32>,
                                            Option<unsafe extern "C" fn()
                                                       ->
                                                           >>(Some(find_interface_v4
                                                                                                                unsafe extern "C" fn(_:
                                                                                                       NetAddress,
                                                                                                       _:
                                                                                                           ,
                                                                                                       _:
                                                                                                           &mut String,
                                                                                                       _:
                                                                                                       NetAddress,
                                                                                                       _:
                                                                                                       NetAddress,
                                                                                                       _:
                                                                                                          Vec<u8>)
                                                                                                       ->
                                                                                          )));
    iface_enumerate(10,
                    &mut now,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            &mut In6Addr,
                                                                        _:
                                                                            ,
                                                                        _:
                                                                            ,
                                                                        _:
                                                                            ,
                                                                        _:
                                                                            ,
                                                                        _:
                                                                            ,
                                                                        _:
                                                                            ,
                                                                        _:
                                                                           Vec<u8>)
                                                                        -> i32>,
                                            Option<unsafe extern "C" fn()
                                                       ->
                                                           >>(Some(find_interface_v6
                                                                                                                unsafe extern "C" fn(_:
                                                                                                           &mut In6Addr,
                                                                                                       _:
                                                                                                           ,
                                                                                                       _:
                                                                                                           ,
                                                                                                       _:
                                                                                                           ,
                                                                                                       _:
                                                                                                           ,
                                                                                                       _:
                                                                                                           ,
                                                                                                       _:
                                                                                                           ,
                                                                                                       _:
                                                                                                          Vec<u8>)
                                                                                                       ->
                                                                                          )));
    lease = leases;
    while !lease.is_null() {
        if lease.new_interface != 0 {
            lease_set_interface(lease, lease.new_interface, now);
        }
        lease = lease.next
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_make_duid(mut now: time::Instant) {
    /* If we're not doing DHCPv6, and there are not v6 leases, don't add the DUID to the database */
    if daemon.duid.is_null() && daemon.doing_dhcp6 != 0
       {
        file_dirty = 1;
        make_duid(now);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_update_dns(mut force: i32) {
    let mut lease: DhcpLease = 0;
    if daemon.port != 0 &&
           (dns_dirty != 0 || force != 0) {
        /* force transfer to authoritative secondaries */
        daemon.soa_sn =
            daemon.soa_sn.wrapping_add(1); /* unlink */
        cache_unhash_dhcp();
        lease = leases;
        while !lease.is_null() {
            let mut prot: i32 = 2;
            if lease.flags & (64 | 32) != 0 {
                prot = 10
            } else if !lease.hostname.is_null() || !lease.fqdn.is_null()
             {
                let mut slaac: SlaacAddress = 0 ;
                slaac = lease.slaac_address;
                while !slaac.is_null() {
                    if slaac.backoff == 0 {
                        if !lease.fqdn.is_null() {
                            cache_add_dhcp_entry(lease.fqdn,
                                                 10,
                                                 &mut slaac.addr  &mut In6Addr  NetAddress,
                                                 lease.expires);
                        }
                        if daemon.options[(20).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                                           ).wrapping_mul(8
                                                                                                                           ))
                                                         ] &
                               (1) <<
                                   (20 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                       ).wrapping_mul(8))
                               == 0 && !lease.hostname.is_null() {
                            cache_add_dhcp_entry(lease.hostname,
                                                 10,
                                                 &mut slaac.addr  &mut In6Addr  NetAddress,
                                                 lease.expires);
                        }
                    }
                    slaac = slaac.next
                }
            }
            if !lease.fqdn.is_null() {
                cache_add_dhcp_entry(lease.fqdn, prot,
                                     if prot == 2 {
                                         &mut lease.addr as

                                     } else {
                                         &mut lease.addr6

                                     }, lease.expires);
            }
            if daemon.options[(20                                 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                   ).wrapping_mul(8                                                                                   ))
                                             ] &
                   (1) <<
                       (20 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                               ).wrapping_mul(8           ))
                   == 0 && !lease.hostname.is_null() {
                cache_add_dhcp_entry(lease.hostname, prot,
                                     if prot == 2 {
                                         &mut lease.addr as

                                     } else {
                                         &mut lease.addr6

                                     }, lease.expires);
            }
            lease = lease.next
        }
        dns_dirty = 0
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_prune(mut target: DhcpLease,
                                     mut now: time::Instant) {
    let mut lease: DhcpLease = 0;
    let mut tmp: DhcpLease = 0;
    let mut up: DhcpLease;
    lease = leases;
    up = &mut leases;
    while !lease.is_null() {
        tmp = lease.next;
        if lease.expires != 0 &&
               difftime(now, lease.expires) >=
                   0  || lease == target {
            file_dirty = 1;
            if !lease.hostname.is_null() { dns_dirty = 1 }
            daemon.metrics[if lease.addr.s_addr != 0 {
                                          METRIC_LEASES_PRUNED_4
                                      } else {
                                          METRIC_LEASES_PRUNED_6
                                      } ] =
                daemon.metrics[if lease.addr.s_addr != 0 {
                                              METRIC_LEASES_PRUNED_4
                                          } else {
                                              METRIC_LEASES_PRUNED_6
                                          } ].wrapping_add(1);
            *up = lease.next;
            /* Put on old_leases list 'till we
	     can run the script */
            lease.next = old_leases;
            old_leases = lease;
            leases_left += 1
        } else { up = &mut lease.next }
        lease = tmp
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_find_by_client(mut hwaddr: mut Vec<u8>,
                                              mut hw_len: i32,
                                              mut hw_type: i32,
                                              mut clid: mut Vec<u8>,
                                              mut clid_len: i32)
 -> DhcpLease {
    let mut lease: DhcpLease = 0;
    if !clid.is_null() {
        lease = leases;
        while !lease.is_null() {
            if !(lease.flags & (64 | 32) !=
                     0) {
                if !lease.clid.is_null() && clid_len == lease.clid_len
                       &&
                       memcmp(clid,
                              lease.clid,
                              clid_len) == 0 {
                    return lease
                }
            }
            lease = lease.next
        }
    }
    lease = leases;
    while !lease.is_null() {
        if !(lease.flags & (64 | 32) != 0) {
            if (lease.clid.is_null() || clid.is_null()) &&
                   hw_len != 0 && lease.hwaddr_len == hw_len
                   && lease.hwaddr_type == hw_type &&
                   memcmp(hwaddr,
                          lease.hwaddr.as_mut_ptr(),
                          hw_len) == 0 {
                return lease
            }
        }
        lease = lease.next
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn lease_find_by_addr(mut addr: NetAddress)
 -> DhcpLease {
    let mut lease: DhcpLease = 0;
    lease = leases;
    while !lease.is_null() {
        if !(lease.flags & (64 | 32) != 0) {
            if lease.addr.s_addr == addr.s_addr { return lease }
        }
        lease = lease.next
    }
    return 0;
}
/* find address for {CLID, IAID, address} */
#[no_mangle]
pub unsafe extern "C" fn lease6_find(mut clid: mut Vec<u8>,
                                     mut clid_len: i32,
                                     mut lease_type: i32,
                                     mut iaid: u32,
                                     mut addr: &mut In6Addr)
 -> DhcpLease {
    let mut lease: DhcpLease = 0;
    lease = leases;
    while !lease.is_null() {
        if !(lease.flags & lease_type == 0 || lease.iaid != iaid) {
            if !(({
                      let mut __a: *const In6Addr =
                          &mut lease.addr6                        *const In6Addr;
                      let mut __b: *const In6Addr = addr ;
                      (__a.__in6_u.__u6_addr32[0 ]
                           ==
                           __b.__in6_u.__u6_addr32[0       usize] &&
                           __a.__in6_u.__u6_addr32[1       usize] ==
                               __b.__in6_u.__u6_addr32[1           usize] &&
                           __a.__in6_u.__u6_addr32[2       usize] ==
                               __b.__in6_u.__u6_addr32[2           usize] &&
                           __a.__in6_u.__u6_addr32[3       usize] ==
                               __b.__in6_u.__u6_addr32[3           usize])
                  }) == 0) {
                if !(clid_len != lease.clid_len ||
                         memcmp(clid,
                                lease.clid,
                                clid_len) !=
                             0) {
                    return lease
                }
            }
        }
        lease = lease.next
    }
    return 0;
}
/* reset "USED flags */
#[no_mangle]
pub unsafe extern "C" fn lease6_reset() {
    let mut lease: DhcpLease = 0;
    lease = leases;
    while !lease.is_null() {
        lease.flags &= !(16);
        lease = lease.next
    };
}
/* enumerate all leases belonging to {CLID, IAID} */
#[no_mangle]
pub unsafe extern "C" fn lease6_find_by_client(mut first: DhcpLease,
                                               mut lease_type: i32,
                                               mut clid: mut Vec<u8>,
                                               mut clid_len: i32,
                                               mut iaid: u32)
                                               -> DhcpLease {
    let mut lease: DhcpLease = 0;
    if first.is_null() { first = leases } else { first = first.next }
    lease = first;
    while !lease.is_null() {
        if !(lease.flags & 16 != 0) {
            if !(lease.flags & lease_type == 0 || lease.iaid != iaid) {
                if !(clid_len != lease.clid_len ||
                         memcmp(clid,
                                lease.clid,
                                clid_len) !=
                             0) {
                    return lease
                }
            }
        }
        lease = lease.next
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn lease6_find_by_addr(mut net: &mut In6Addr,
                                             mut prefix: i32,
                                             mut addr: u64)
                                             -> DhcpLease {
    let mut lease: DhcpLease = 0;
    lease = leases;
    while !lease.is_null() {
        if !(lease.flags & (64 | 32) == 0) {
            if is_same_net6(&mut lease.addr6, net, prefix) != 0 &&
                   (prefix == 128 ||
                        addr6part(&mut lease.addr6) == addr) {
                return lease
            }
        }
        lease = lease.next
    }
    return 0;
}
/* Find largest assigned address in context */
#[no_mangle]
pub unsafe extern "C" fn lease_find_max_addr6(mut context: DhcpContext)
 -> u64 {
    let mut lease: DhcpLease = 0;
    let mut addr: u64 = addr6part(&mut context.start6);
    if context.flags &
           ((1) << 0 |
                (1) << 3) == 0 {
        lease = leases;
        while !lease.is_null() {
            if !(lease.flags & (64 | 32) ==
                     0) {
                if is_same_net6(&mut lease.addr6, &mut context.start6,
                                64) != 0 &&
                       addr6part(&mut lease.addr6) >
                           addr6part(&mut context.start6) &&
                       addr6part(&mut lease.addr6) <=
                           addr6part(&mut context.end6) &&
                       addr6part(&mut lease.addr6) > addr {
                    addr = addr6part(&mut lease.addr6)
                }
            }
            lease = lease.next
        }
    }
    return addr;
}
/* Find largest assigned address in context */
#[no_mangle]
pub unsafe extern "C" fn lease_find_max_addr(mut context: DhcpContext)
 -> NetAddress {
    let mut lease: DhcpLease = 0; /* illegal value */
    let mut addr: NetAddress = context.start;
    if context.flags &
           ((1) << 0 |
                (1) << 3) == 0 {
        lease = leases;
        while !lease.is_null() {
            if !(lease.flags & (64 | 32) !=
                     0) {
                if __bswap_32(lease.addr.s_addr) >
                       __bswap_32(context.start.s_addr) &&
                       __bswap_32(lease.addr.s_addr) <=
                           __bswap_32(context.end.s_addr) &&
                       __bswap_32(lease.addr.s_addr) >
                           __bswap_32(addr.s_addr) {
                    addr = lease.addr
                }
            }
            lease = lease.next
        }
    }
    return addr;
}
unsafe extern "C" fn lease_allocate(leases_left: u32, leases: &mut Vec<DhcpLease>) -> Option<DhcpLease> {
    let mut lease: DhcpLease = Default::default();
    if leases_left == 0 {
        return None
    }
    lease.flags = 1;
    lease.expires = 1;
    lease.hwaddr_len = 256;
    // lease.next = leases;
    // leases = lease;
    file_dirty = 1;
    leases_left -= 1;
    
    return lease;
}
#[no_mangle]
pub unsafe extern "C" fn lease4_allocate(mut addr: NetAddress)
 -> DhcpLease {
    let mut lease: DhcpLease = lease_allocate();
    if !lease.is_null() {
        lease.addr = addr;
        daemon.metrics[METRIC_LEASES_ALLOCATED_4                                usize] =
            daemon.metrics[METRIC_LEASES_ALLOCATED_4
                                          ].wrapping_add(1)
    }
    return lease;
}
#[no_mangle]
pub unsafe extern "C" fn lease6_allocate(mut addrp: &mut In6Addr,
                                         mut lease_type: i32)
                                         -> DhcpLease {
    let mut lease: DhcpLease = lease_allocate();
    if !lease.is_null() {
        lease.addr6 = *addrp;
        lease.flags |= lease_type;
        lease.iaid = 0;
        daemon.metrics[METRIC_LEASES_ALLOCATED_6                                usize] =
            daemon.metrics[METRIC_LEASES_ALLOCATED_6
                                          ].wrapping_add(1)
    }
    return lease;
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_expires(mut lease: DhcpLease,
                                           mut len: u32,
                                           mut now: time::Instant) {
    let mut exp: time::Instant = 0;
    if len == 0xffffffff {
        exp = 0;
        len = 0
    } else {
        exp = now + len;
        /* Check for 2038 overflow. Make the lease
	 infinite in that case, as the least disruptive
	 thing we can do. */
        if difftime(exp, now) <= 0.0f64 { exp = 0 }
    }
    if exp != lease.expires {
        dns_dirty = 1;
        lease.expires = exp;
        lease.flags |= 4 | 256;
        file_dirty = 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_iaid(mut lease: DhcpLease,
                                        mut iaid: u32) {
    if lease.iaid != iaid {
        lease.iaid = iaid;
        lease.flags |= 2
    };
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_hwaddr(mut lease: DhcpLease,
                                          mut hwaddr: *const libc::c_uchar,
                                          mut clid: *const libc::c_uchar,
                                          mut hw_len: i32,
                                          mut hw_type: i32,
                                          mut clid_len: i32,
                                          mut now: time::Instant,
                                          mut force: i32) {
    let mut change: i32 = force;
    lease.flags |= 128;
    if hw_len != lease.hwaddr_len || hw_type != lease.hwaddr_type ||
           hw_len != 0 &&
               memcmp(lease.hwaddr.as_mut_ptr(),
                      hwaddr, hw_len)
                   != 0 {
        if hw_len != 0 {
            memcpy(lease.hwaddr.as_mut_ptr(),
                   hwaddr, hw_len);
        }
        lease.hwaddr_len = hw_len;
        lease.hwaddr_type = hw_type;
        lease.flags |= 2;
        file_dirty = 1
        /* run script on change */
    }
    /* only update clid when one is available, stops packets
     without a clid removing the record. Lease init uses
     clid_len == 0 for no clid. */
    if clid_len != 0 && !clid.is_null() {
        if lease.clid.is_null() { lease.clid_len = 0 }
        if lease.clid_len != clid_len {
            lease.flags |= 4;
            file_dirty = 1;
            // free(lease.clid);
            // lease.clid =
            //     whine_malloc(clid_len );
            if lease.clid.is_null() { return }
            change = 1
        } else if memcmp(lease.clid,
                         clid,
                         clid_len) != 0 {
            lease.flags |= 4;
            file_dirty = 1;
            change = 1
        }
        lease.clid_len = clid_len;
        memcpy(lease.clid,
               clid, clid_len);
    }
    if change != 0 { slaac_add_addrs(lease, now, force); };
}
unsafe extern "C" fn kill_name(mut lease: DhcpLease) {
    /* run script to say we lost our old name */
    /* this shouldn't happen unless updates are very quick and the
     script very slow, we just avoid a memory leak if it does. */
    // free(lease.old_hostname);
    /* If we know the fqdn, pass that. The helper will derive the
     unqualified name from it, free the unqualified name here. */
    if !lease.fqdn.is_null() {
        lease.old_hostname = lease.fqdn;
        // free(lease.hostname);
    } else { lease.old_hostname = lease.hostname }
    lease.fqdn = 0 ;
    lease.hostname = lease.fqdn;
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_hostname(mut lease: DhcpLease,
                                            mut name: *const libc::c_char,
                                            mut auth: i32,
                                            mut domain: &mut String,
                                            mut config_domain:
                                                &mut String) {
    let mut lease_tmp: DhcpLease = 0;
    let mut new_name: &mut String = 0 ;
    let mut new_fqdn: &mut String = 0 ;
    if !config_domain.is_null() &&
           (domain.is_null() || hostname_isequal(domain, config_domain) == 0)
       {
        my_syslog((3) << 3 | 4,
                  "Ignoring domain %s for DHCP host name %s"
                     , config_domain, name);
    }
    if !lease.hostname.is_null() && !name.is_null() &&
           hostname_isequal(lease.hostname, name) != 0 {
        if auth != 0 { lease.flags |= 8 }
        return
    }
    if name.is_null() && lease.hostname.is_null() { return }
    /* If a machine turns up on a new net without dropping the old lease,
     or two machines claim the same name, then we end up with two interfaces with
     the same name. Check for that here and remove the name from the old lease.
     Note that IPv6 leases are different. All the leases to the same DUID are 
     allowed the same name.

     Don't allow a name from the client to override a name from dnsmasq config. */
    if !name.is_null() {
        // new_name =
        //     whine_malloc(strlen(name).wrapping_add(1    libc::c_ulong))          &mut String;
        if !new_name.is_null() {
            strcpy(new_name, name);
            if !domain.is_null() &&
                   {
                       // new_fqdn =
                       //     whine_malloc(strlen(new_name).wrapping_add(strlen(domain)).wrapping_add(2                                                   ))
                       //         ;
                       // !new_fqdn.is_null()
                       true
                   } {
                strcpy(new_fqdn, name);
                strcat(new_fqdn,
                       "." );
                strcat(new_fqdn, domain);
            }
        }
        let mut current_block_23: u64;
        /* Depending on mode, we check either unqualified name or FQDN. */
        lease_tmp = leases;
        while !lease_tmp.is_null() {
            if daemon.options[(20                                 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                   ).wrapping_mul(8                                                                                   ))
                                             ] &
                   (1) <<
                       (20 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                               ).wrapping_mul(8           ))
                   != 0 {
                if new_fqdn.is_null() || lease_tmp.fqdn.is_null() ||
                       hostname_isequal(lease_tmp.fqdn, new_fqdn) == 0 {
                    current_block_23 = 17833034027772472439;
                } else { current_block_23 = 11307063007268554308; }
            } else if new_name.is_null() || lease_tmp.hostname.is_null() ||
                          hostname_isequal(lease_tmp.hostname, new_name) ==
                              0 {
                current_block_23 = 17833034027772472439;
            } else { current_block_23 = 11307063007268554308; }
            match current_block_23 {
                11307063007268554308 => {
                    if lease.flags &
                           (64 | 32) != 0 {
                        if lease_tmp.flags &
                               (64 | 32) == 0 {
                            current_block_23 = 17833034027772472439;
                        } else if lease.clid_len == lease_tmp.clid_len
                                      && !lease.clid.is_null() &&
                                      !lease_tmp.clid.is_null() &&
                                      memcmp(lease.clid
                                             lease_tmp.clid
                                             lease.clid_len                                    ) ==
                                          0 {
                            current_block_23 = 17833034027772472439;
                        } else { current_block_23 = 1608152415753874203; }
                    } else if lease_tmp.flags &
                                  (64 | 32) != 0
                     {
                        current_block_23 = 17833034027772472439;
                    } else { current_block_23 = 1608152415753874203; }
                    match current_block_23 {
                        17833034027772472439 => { }
                        _ => {
                            if lease_tmp.flags & 8 != 0 &&
                                   auth == 0 {
                                // free(new_name);
                                // free(new_fqdn);
                                return
                            }
                            kill_name(lease_tmp);
                            break ;
                        }
                    }
                }
                _ => { }
            }
            lease_tmp = lease_tmp.next
        }
    }
    if !lease.hostname.is_null() { kill_name(lease); }
    lease.hostname = new_name;
    lease.fqdn = new_fqdn;
    if auth != 0 { lease.flags |= 8 }
    file_dirty = 1;
    dns_dirty = 1;
    lease.flags |= 2;
    /* another lease for the same DUID is OK for IPv6 */
    /* run script on change */
}
#[no_mangle]
pub unsafe extern "C" fn lease_set_interface(mut lease: DhcpLease,
                                             mut interface: i32,
                                             mut now: time::Instant) {
    if lease.last_interface == interface { return }
    lease.last_interface = interface;
    lease.flags |= 2;
    slaac_add_addrs(lease, now, 0);
}
#[no_mangle]
pub unsafe extern "C" fn rerun_scripts() {
    let mut lease: DhcpLease = 0;
    lease = leases;
    while !lease.is_null() {
        lease.flags |= 2;
        lease = lease.next
    };
}
/* deleted leases get transferred to the old_leases list.
   remove them here, after calling the lease change
   script. Also run the lease change script on new/modified leases.

   Return zero if nothing to do. */
#[no_mangle]
pub unsafe extern "C" fn do_script_run(mut now: time::Instant) -> i32 {
    let mut lease: DhcpLease = 0;
    if !old_leases.is_null() {
        lease = old_leases;
        /* If the lease still has an old_hostname, do the "old" action on that first */
        if !lease.old_hostname.is_null() {
            queue_script(2, lease, lease.old_hostname, now);
            // free(lease.old_hostname);
            lease.old_hostname = 0 ;
            return 1
        } else {
            let mut slaac: SlaacAddress = 0 ;
            let mut tmp: SlaacAddress = 0 ;
            slaac = lease.slaac_address;
            while !slaac.is_null() {
                tmp = slaac.next;
                // free(slaac);
                slaac = tmp
            }
            kill_name(lease);
            queue_script(1, lease, lease.old_hostname, now);
            old_leases = lease.next;
            // free(lease.old_hostname);
            // free(lease.clid);
            // free(lease.extradata);
            // free(lease);
            return 1
        }
    }
    /* make sure we announce the loss of a hostname before its new location. */
    lease = leases;
    while !lease.is_null() {
        if !lease.old_hostname.is_null() {
            queue_script(2, lease, lease.old_hostname, now);
            // free(lease.old_hostname);
            lease.old_hostname = 0 ;
            return 1
        }
        lease = lease.next
    }
    lease = leases;
    while !lease.is_null() {
        if lease.flags & (1 | 2) != 0 ||
               lease.flags & 4 != 0 &&
                   daemon.options[(22 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                           ).wrapping_mul(8                                                                                                   ))
                                                 ] &
                       (1) <<
                           (22                   ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                       ).wrapping_mul(8                           ))
                       != 0 ||
               lease.flags & 256 != 0 &&
                   daemon.options[(61 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                           ).wrapping_mul(8                                                                                                   ))
                                                 ] &
                       (1) <<
                           (61                   ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                       ).wrapping_mul(8                           ))
                       != 0 {
            queue_script(if lease.flags & 1 != 0 {
                             4
                         } else { 3 }, lease,
                         if !lease.fqdn.is_null() {
                             lease.fqdn
                         } else { lease.hostname }, now);
            lease.flags &=
                !(1 | 2 | 4 |
                      256);
            /* this is used for the "add" call, then junked, since they're not in the database */
            // free(lease.extradata);
            lease.extradata = 0;
            return 1
        }
        lease = lease.next
    }
    return 0;
    /* nothing to do */
}
/* delim == -1 -> delim = 0, but embedded 0s, creating extra records, are OK. */
#[no_mangle]
pub unsafe extern "C" fn lease_add_extradata(mut lease: DhcpLease,
                                             mut data: mut Vec<u8>,
                                             mut len: u32,
                                             mut delim: i32) {
    let mut i: u32 = 0;
    if delim == -(1) {
        delim = 0
    } else {
        /* check for embedded NULLs */
        i = 0;
        while i < len {
            if *data.offset(i) == 0 {
                len = i;
                break ;
            } else { i = i.wrapping_add(1) }
        }
    }
    if lease.extradata_size.wrapping_sub(lease.extradata_len) <
           len.wrapping_add(1) {
        let mut newsz: usize =
            lease.extradata_len.wrapping_add(len).wrapping_add(100
                                                                                        libc::c_uint)
                ;
        // let mut new: mut Vec<u8> =
        //     whine_malloc(newsz);
        if new.is_null() { return }
        if !lease.extradata.is_null() {
            memcpy(new,
                   lease.extradata,
                   lease.extradata_len);
            // free(lease.extradata);
        }
        lease.extradata = new;
        lease.extradata_size = newsz
    }
    if len != 0 {
        memcpy(lease.extradata.offset(lease.extradata_len)            Vec<u8>, data,
               len);
    }
    *lease.extradata.offset(lease.extradata_len.wrapping_add(len)                             ) = delim;
    lease.extradata_len =
        lease.extradata_len.wrapping_add(len.wrapping_add(1
                                                                              libc::c_uint));
}
