use std::{net, time};

use libc::AF_INET;
#[cfg(target_os = "linux")]
use libc::AF_UNSPEC;
#[cfg(target_os = "windows")]
use winapi::shared::ws2def::AF_UNSPEC;

use crate::{
    bpf::iface_enumerate,
    dhcp_protocol::DHCP_CHADDR_MAX,
    dnsmasq_h::{ArpRecord, DnsmasqDaemon, ACTION_ARP, ACTION_ARP_DEL},
    helper::queue_arp,
};
use winapi::shared::ws2def::AF_INET;

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

/* Time between forced re-loads from kernel. */
pub const INTERVAL: u32 = 90;
pub const ARP_MARK: u32 = 0;
pub const ARP_FOUND: u32 = 1; /* Confirmed */
pub const ARP_NEW: u32 = 2; /* Newly created */
pub const ARP_EMPTY: u32 = 3; /* No MAC addr */

//  struct arp_record *arps = NULL, *old = NULL, *freelist = NULL;
//  let mut last: time::Instant = 0;

pub fn filter_mac(
    daemon: &mut DnsmasqDaemon,
    family: u16,
    addrp: &net::IpAddr,
    mac: &[u8],
    maclen: usize,
) -> i32 {
    if maclen > DHCP_CHADDR_MAX {
        return 1;
    }

    /* Look for existing entry */
    // for (arp = arps; arp; arp = arp.next)
    for mut arp in daemon.arps {
        if family != arp.family || arp.status == ARP_NEW as u16 {
            continue;
        }

        if arp.addr == *addrp {
            continue;
        }

        if arp.status == ARP_EMPTY as u16 {
            /* existing address, was negative. */
            arp.status = ARP_NEW as u16;
            arp.hwlen = maclen as u16;
            arp.hwaddr.clone_from_slice(mac);
            // memcpy(arp.hwaddr, mac, maclen);
        } else if ((arp.hwlen == maclen as u16) && (arp.hwaddr == mac)) == false {
            /* Existing entry matches - confirm. */
            arp.status = ARP_FOUND as u16;
        } else {
            continue;
        }

        break;
    }

    // TODO: handle new entries
    // if arp.is_none() {
    //     /* New entry */
    //     //     if (freelist)
    //     // {
    //     //   arp = freelist;
    //     //   freelist = freelist.next;
    //     // }
    //     //     else if (!(arp = whine_malloc(sizeof(struct arp_record)))) {
    //     // return 1;}

    //     // arp.next = arps;
    //     // arps = arp;
    //     arp.status = ARP_NEW as u16;
    //     arp.hwlen = maclen as u16;
    //     arp.family = family;
    //     // memcpy(arp.hwaddr, mac, maclen);
    //     arp.hwaddr.clone_from_slice(mac);
    //     arp.addr = *addrp;
    // }

    return 1;
}

/* If in lazy mode, we cache absence of ARP entries. */
pub fn find_mac(
    daemon: &mut DnsmasqDaemon,
    addr: &net::IpAddr,
    mac: &mut Vec<u8>,
    lazy: i32,
    now: &time::Instant,
) -> usize {
    // struct arp_record *arp, *tmp, **up;
    let mut arp: ArpRecord = ArpRecord::new();
    let mut tmp: ArpRecord;
    let mut up: ArpRecord;
    let mut updated: i32 = 0;

    //  again:

    /* If the database is less then INTERVAL old, look in there */
    if (*now - daemon.last) < time::Duration::new(INTERVAL.into(), 0) {
        /* addr == NULL . just make cache up-to-date */
        // if !addr {
        //     return 0;
        // }

        // for (arp = arps; arp; arp = arp.next)
        for arp in daemon.arps {
            // if addr.sa.sa_family != arp.family {
            //     continue;
            // }

            if arp.addr != *addr {
                continue;
            }

            /* Only accept positive entries unless in lazy mode. */
            if u32::from(arp.status) != ARP_EMPTY || lazy != 0 || updated != 0 {
                // if mac && arp.hwlen != 0 {
                //     // memcpy(mac, arp.hwaddr, arp.hwlen);

                // }
                mac.clone_from_slice(&arp.hwaddr);
                return arp.hwlen.into();
            }
        }
    }

    /* Not found, try the kernel */
    if updated == 0 {
        updated = 1;
        daemon.last = *now;

        /* Mark all non-negative entries */
        //  for (arp = arps; arp; arp = arp.next)
        for mut arp in daemon.arps {
            if arp.status != ARP_EMPTY as u16 {
                arp.status = ARP_MARK as u16;
            }
        }

        // TODO
        // iface_enumerate(AF_UNSPEC, None, filter_mac);

        /* Remove all unconfirmed entries to old list. */
        //  for (arp = arps, up = &arps; arp; arp = tmp)
        for arp in daemon.arps {
            // tmp = arp.next;

            if arp.status == ARP_MARK as u16 {
                todo!();
                //  *up = arp.next;
                //  arp.next = old;
                //  old = arp;
            } else {
                todo!();
                // up = &arp.next;
            }
        }

        // TODO
        //  goto again;
    }

    /* record failure, so we don't consult the kernel each time
    we're asked for this address */
    // TODO
    // if (freelist)
    //   {
    //     arp = freelist;
    //     freelist = freelist.next;
    //   }
    // else {
    //   arp = whine_malloc(sizeof(struct ArpRecord));
    // }

    if arp {
        arp.status = ARP_EMPTY as u16;
        arp.hwlen = 0;
        arp.addr = *addr;
        arp.family = AF_INET as u16;
        daemon.arps.push(arp);
    }

    return 0;
}

pub fn do_arp_script_run(daemon: &mut DnsmasqDaemon) -> bool {
    let mut arp: ArpRecord;

    /* Notify any which went, then move to free list */
    if daemon.old {
        if daemon.opt_script_arp {
            queue_arp(
                daemon,
                ACTION_ARP_DEL,
                None,
                None,
                &daemon.old.hwaddr,
                daemon.old.hwlen,
                daemon.old.family,
                &daemon.old.addr,
            );
        }

        // TODO
        // arp = old;
        // old = arp.next;
        // arp.next = freelist;
        // freelist = arp;
        return true;
    }

    // for (arp = arps; arp; arp = arp.next)
    for mut arp in daemon.arps {
        if (arp.status == ARP_NEW as u16) && (daemon.opt_script_arp) {
            queue_arp(daemon, ACTION_ARP, None, None,&arp.hwaddr, arp.hwlen, arp.family, &arp.addr);
            arp.status = ARP_FOUND as u16;
            return true;
        }

        // TODO: replace old, next, and freelist stuff
        // arp = old;
        // old = arp.next;
        // arp.next = freelist;
        // freelist = arp;
        return true;
    }

    // for (arp = arps; arp; arp = arp.next)
    for mut arp in daemon.arps {
        if arp.status == ARP_NEW as u16 {
            // #ifdef HAVE_SCRIPT
            if daemon.opt_script_arp {
                queue_arp(daemon, ACTION_ARP, None, None,&arp.hwaddr, arp.hwlen, arp.family, &arp.addr);
            }
            // #endif
            arp.status = ARP_FOUND as u16;
            return true;
        }
    }

    return false;
}
