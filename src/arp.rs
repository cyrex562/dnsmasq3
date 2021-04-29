use crate::dnsmasq_h::{all_addr, ArpRecord, DnsmasqDaemon};

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

// #include "dnsmasq.h"

/* Time between forced re-loads from kernel. */
pub const INTERVAL: u32 = 90;

pub const ARP_MARK: u32 = 0;
pub const ARP_FOUND: u32 = 1; /* Confirmed */
pub const ARP_NEW: u32 = 2; /* Newly created */
pub const ARP_EMPTY: u32 = 3; /* No MAC addr */

// static struct arp_record *arps = NULL, *old = NULL, *freelist = NULL;
// static time_t last = 0;

pub fn filter_mac(
    daemon: &mut DnsmasqDaemon,
    family: i32,
    addrp: &net::IpAddr,
    mac: &[u8],
    maclen: usize,
) -> i32 {
    // struct arp_record *arp;
    let mut arp: ArpRecord = Default::default();

    if (maclen > DHCP_CHADDR_MAX) {
        return 1;
    }

    /* Look for existing entry */
    for arp in daemon.arps {
        if (family != arp.family || arp.status == ARP_NEW) {
            continue;
        }
        if arp.addr == addrp {
            continue;
        }

        if (arp.status == ARP_EMPTY) {
            /* existing address, was negative. */
            arp.status = ARP_NEW;
            arp.hwlen = maclen;
            memcpy(arp.hwaddr, mac, maclen);
        } else if (arp.hwlen == maclen && memcmp(arp.hwaddr, mac, maclen) == 0) {
            /* Existing entry matches - confirm. */
            arp.status = ARP_FOUND;
        } else {
            continue;
        }

        break;
    }

    if (!arp) {
        arp = Default::default();
        arp.status = ARP_NEW;
        arp.hwlen = maclen;
        arp.family = family;
        arp.hwaddr.clone_from_slice(mac);
        arp.addr = addrp;
    }

    return 1;
}

/* If in lazy mode, we cache absence of ARP entries. */
pub fn find_mac(
    daemon: &mut DnsmasqDaemon,
    addr: &net::IpAddr,
    mac: &[u8],
    lazy: i32,
    now: &time::Instsant,
) -> i32 {
    //struct arp_record *arp, *tmp, **up;
    let mut arp: ArpRecord;
    let mut tmp: ArpRecord;
    let mut up: ArpRecord;
    //int updated = 0;
    let mut updated: i32 = 0;
    //  again:

    /* If the database is less then INTERVAL old, look in there */
    if (difftime(now, last) < INTERVAL) {
        /* addr == NULL . just make cache up-to-date */
        if (!addr) {
            return 0;
        }

        for arp in daemon.arps {
            if (addr.sa.sa_family != arp.family) {
                continue;
            }

            if arp.addr != addr {
                continue;
            }

            /* Only accept positive entries unless in lazy mode. */
            if (arp.status != ARP_EMPTY || lazy || updated) {
                if (mac && arp.hwlen != 0) {
                    memcpy(mac, arp.hwaddr, arp.hwlen);
                }
                return arp.hwlen;
            }
        }
    }

    /* Not found, try the kernel */
    if (!updated) {
        updated = 1;
        last = now;

        /* Mark all non-negative entries */
        // for (arp = arps; arp; arp = arp.next)
        for arp in daemon.arps {
            if (arp.status != ARP_EMPTY) {
                arp.status = ARP_MARK;
            }
        }

        iface_enumerate(AF_UNSPEC, NULL, filter_mac);

        /* Remove all unconfirmed entries to old list. */
        // for (arp = arps, up = &arps; arp; arp = tmp)
        let up = &daemon.arps;
        for arp in daemon.arps {
            let tmp = arp.next;
            if (arp.status == ARP_MARK) {
                *up = arp.next;
                arp.next = old;
                old = arp;
            } else {
                up = &arp.next;
            }
        }

        // goto again;

        /* record failure, so we don't consult the kernel each time
        we're asked for this address */
        if (freelist) {
            // arp = freelist;
            // freelist = freelist.next;
        } else {
            // arp = whine_malloc(sizeof(struct arp_record));
            arp = Default::default();
        }

        if (arp) {
            // arp.next = arps;
            // arps = arp;
            arp.status = ARP_EMPTY;
            arp.family = addr.sa.sa_family;
            arp.hwlen = 0;
            arp.addr = addr;
        }
    }

    return 0;
}

pub fn do_arp_script_run(daemon: &mut DnsmasqDaemon) -> i32 {
    // struct arp_record *arp;
    let arp: ArpRecord;

    /* Notify any which went, then move to free list */
    if (old) {
        if (daemon.opt_script_arp) {
            queue_arp(ACTION_ARP_DEL, old.hwaddr, old.hwlen, old.family, &old.addr);
        }

        arp = old;
        old = arp.next;
        arp.next = freelist;
        freelist = arp;
        return 1;
    }

    // for (arp = arps; arp; arp = arp.next)

    if (arp.status == ARP_NEW) {
        // #ifdef HAVE_SCRIPT
        if (daemon.opt_script_arp) {
            queue_arp(ACTION_ARP, arp.hwaddr, arp.hwlen, arp.family, &arp.addr);
        }
        // #endif
        arp.status = ARP_FOUND;
        return 1;
    }

    return 0;
}
