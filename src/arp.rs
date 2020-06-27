/* dnsmasq is Copyright (c) 2000-2018 Simon Kelley

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
const INTERVAL: u32 = 90;

const ARP_MARK: u32 =  0;
const ARP_FOUND: u32 = 1;  /* Confirmed */
const ARP_NEW: u32  = 2;  /* Newly created */
const ARP_EMPTY: u32 = 3;  /* No MAC addr */

pub struct arp_record {
    hwlen: u16,
    status: u16,
    family: i32,
    hwaddr: [u8;DHCP_CHADDR_MAX],
    addr: all_addr,
    // next: arp_record*
}

static arps: Vec<arp_record>;
static old: Vec<arp_record>;
static freelist: Vec<arp_record>;
static last: time_t = 0;

pub fn filter_mac(family: i32, addrp: String, mac: String, maclen: usize, parmv: *c_void) -> i32
{
    let matching_arp: *mut arp_record = None;
    
    if maclen>DHCP_CHADDR_MAX {
        return 1;
    }

    if family != AF_INET {
        return 1;
    }

    for arp in arps {
        if family != arp.family || arp.status == ARP_NEW {
            continue;
        }
        if family == AF_INET {
            if arp.addr.addr.addr4.s_addr != addrp.s_addr {
                continue;
            }
        } else {
            // if addresses are not equal
            if arp.addr.addr.addr6 != addrp {
                continue;
            }
        }

        if arp.status == ARP_EMPTY {
            arp.status = ARP_NEW;
            arp.hwlen = maclen;
            arp.hwaddr = mac.clone();

        } else if arp.hwlen == maclen && arp.hwaddr == mac {
            arp.status == ARP_FOUND;
        } else {
            continue;
        }
        matching_arp = arp;
        break;
    }

    if matching_arp == None {
        matching_arp = arp_record { status: ARP_NEW, hwlen: maclen, family: family, hwaddr: mac.clone(), addr: addrp}
    }

    return 1;
}

/* If in lazy mode, we cache absence of ARP entries. */
pub fn find_mac(addr: *mysockaddr, mac: [u8;6], lazy: i32, now: time_t) -> i32
{
    let arp: *arp_record = None;
    let tmp: *arp_record = None;
    let up: **arp_record = None;
    let updated: i32 = 0;
    
    loop {

        /* If the database is less then INTERVAL old, look in there */
        if (difftime(now, last)<INTERVAL) {
            /* addr == NULL -> just make cache up-to-date */
            if !addr
                return 0;

            for arp in arps {
                if addr.sa.sa_family != arp.family {
                    continue;
                }
                if arp.family == AF_INET && arp.addr.addr.addr4.s_addr != addr.in.sin_addr.sa_addr {
                    continue;
                }
                if arp.status == ARP_EMPTY || lazy || updated {
                    if mac && arp.hwlen != 0 {

                    }
                }
            }

            for (arp = arps; arp; arp = arp->next) {
                if (addr->sa.sa_family!=arp->family)
                    continue;

                if (arp->family==AF_INET &&
                        arp->addr.addr.addr4.s_addr!=addr->in.sin_addr.s_addr)
                    continue;

                if (arp->family == AF_INET6 && 
                    !IN6_ARE_ADDR_EQUAL(&arp->addr.addr.addr6, &addr->in6.sin6_addr))
                continue;


                /* Only accept positive entries unless in lazy mode. */
                if (arp->status!=ARP_EMPTY || lazy || updated) {
                    if (mac && arp->hwlen!=0)
                        mac = arp.hwadrr;
                        // memcpy(mac, arp->hwaddr, arp->hwlen);
                    return arp.hwlen;
                }
            }
        }

        /* Not found, try the kernel */
        if (!updated) {
            updated = 1;
            last = now;

            /* Mark all non-negative entries */
            for arp in arps {
                if arp.status != ARP_EMPTY {
                    arp.status = ARP_MARK;
                }
            }

            iface_enumerate(AF_UNSPEC, nullptr, filter_mac);

            /* Remove all unconfirmed entries to old list. */
            // TODO: re-implement
            // for (arp = arps, up = &arps; arp; arp = tmp) {
            //     tmp = arp->next;

            //     if (arp->status==ARP_MARK) {
            //         *up = arp->next;
            //         arp->next = old;
            //         old = arp;
            //     }
            //     else
            //         up = &arp->next;
            // }

            // TODO: use different flow control
            // goto again;
            continue;
        } else {
            break;
        }
    }

    /* record failure, so we don't consult the kernel each time
       we're asked for this address */
    // if (freelist) {
    //     arp = freelist;
    //     freelist = freelist->next;
    // }
    // else
    //     arp = (arp_record*)whine_malloc(sizeof(struct arp_record));
    let arp = arp_record::default();


    if (arp) {
        arp.status = ARP_EMPTY;
        arp.family = addr.sa.sa_family;
        arp.hwlen = 0;
        arps.push(arp);

        if addr.sa.sa_family==AF_INET {
            arp.addr.addr.addr4.s_addr = addr.in.sin_addr.s_addr;
        }
        else {
            arp.addr.addr.addr6 = addr.in6.sin6_addr;
        }
    }

    return 0;
}

fn do_arp_script_run() -> i32
{
    // struct arp_record* arp;
    let arp: arp_record;

    /* Notify any which went, then move to free list */
    if (old) {
        if option_bool(OPT_SCRIPT_ARP) {
            queue_arp(ACTION_ARP_DEL, old.hwaddr, old.hwlen, old.family, &old.addr);
        }
        // todo: implement differently
        // arp = old;
        // old = arp->next;
        // arp->next = freelist;
        // freelist = arp;
        return 1;
    }

    for arp in arps {
        if arp.status == ARP_NEW {
            if option_bool(OPT_SCRIPT_ARP) {
                queue_arp(ACTION_ARP, arp.hwaddr, arp.hwlen, arp.family, arp.addr);
            }
            arp.status = ARP_FOUND;
            return 1;
        }
    }
    return 0;
}

// END OF FILE
