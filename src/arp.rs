use std::{convert::TryInto, ops::Range};

use chrono::{DateTime, Utc};
use libc::{c_void, time_t};



use crate::{defines::AF_INET, dhcp_protocol::DHCP_CHADDR_MAX, dnsmasq_h::{Action, AF_INET6, AF_UNSPEC, Daemon, Mysockaddr, OPT_SCRIPT_ARP, option_bool}, dnsmasq_sys::{In6Addr, InAddr, sa_family_t}, helper::queue_arp};


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
pub const INTERVAL: u32 = 90;

pub const ARP_MARK: u32 =  0;
pub const ARP_FOUND: u32 = 1;  /* Confirmed */
pub const ARP_NEW: u32  = 2;  /* Newly created */
pub const ARP_EMPTY: u32 = 3;  /* No MAC addr */


pub fn zero_hw_addr(hw_addr: [u8;DHCP_CHADDR_MAX]) {

}


#[derive(Clone, Copy, Debug)]
pub struct ArpRecord {
    pub hwlen: u16,
    pub status: u16,
    pub family: sa_family_t,
    pub hwaddr: [u8;DHCP_CHADDR_MAX],
    pub addr: AddressPointer,
    // next: arp_record*
}

static arps: Vec<ArpRecord> = Vec::new();
static old: Vec<ArpRecord> = Vec::new();
static freelist: Vec<ArpRecord> = Vec::new();
// static last: time_t;

// #[repr(C)]
// pub union AddressPointer {
//     addr4: in_addr,
//     addr6: in6_addr,
// }

#[derive(Debug, Clone, Copy, Default)]
pub struct AddressPointer {
    pub addr4: InAddr,
    pub addr6: In6Addr, 
}

pub fn hwaddr_mac_eq(hwaddr: &[u8;DHCP_CHADDR_MAX], mac: &[u8;6]) -> bool {
    if hwaddr[0] != mac[0] {
        return false;
    }
    if hwaddr[1] != mac[1] {
        return false;
    }
    if hwaddr[2] != mac[2] {
        return false;
    }
    if hwaddr[3] != mac[3] {
        return false;
    }
    if hwaddr[4] != mac[4] {
        return false;
    }
    if hwaddr[5] != mac[5] {
        return false;
    }

    for i in 6..DHCP_CHADDR_MAX {
        if hwaddr[i] != 0 {
            return false;
        }
    }
    true
}

pub fn copy_mac_into_hwaddr(hwaddr: &mut [u8;DHCP_CHADDR_MAX], mac: &[u8;6]) {
    hwaddr[0] = mac[0];
    hwaddr[1] = mac[1];
    hwaddr[2] = mac[2];
    hwaddr[3] = mac[3];
    hwaddr[4] = mac[4];
    hwaddr[5] = mac[5];
    for i in 6..DHCP_CHADDR_MAX {
        hwaddr[i] = 0;
    }
}

pub fn hwaddr_from_mac(mac: &[u8;6]) -> [u8;DHCP_CHADDR_MAX] {
    let mut out: [u8;DHCP_CHADDR_MAX] = [0; DHCP_CHADDR_MAX];
    out[0] = mac[0];
    out[1] = mac[1];
    out[2] = mac[2];
    out[3] = mac[3];
    out[4] = mac[4];
    out[5] = mac[5];
    out
}

pub fn filter_mac(
    family: sa_family_t, 
    addrp: AddressPointer, 
    mac: [u8;6], 
    maclen: usize, 
    parmv: *mut c_void,
    arp_records: &mut Vec<ArpRecord>) -> i32
{
    let mut matching_arp: Option<ArpRecord>; 
    
    if maclen > DHCP_CHADDR_MAX {
        return 1;
    }

    if family != AF_INET {
        return 1;
    }

    // TODO: get rid of use of global variable
    for arp in arp_records {
        if family != arp.family || arp.status == ARP_NEW.try_into().unwrap() {
            continue;
        }
        if family == AF_INET {
            if arp.addr.addr4.s_addr != addrp.addr4.s_addr {
                continue;
            }
        } else {
            // if addresses are not equal
            if arp.addr.addr6 != addrp.addr6 {
                continue;
            }
        }

        if arp.status == ARP_EMPTY.try_into().unwrap() {
            arp.status = ARP_NEW.try_into().unwrap();
            arp.hwlen = maclen.try_into().unwrap();
            arp.hwaddr[0] = mac[0];
            arp.hwaddr[1] = mac[1];
            arp.hwaddr[2] = mac[2];
            arp.hwaddr[3] = mac[3];
            arp.hwaddr[4] = mac[4];
            arp.hwaddr[5] = mac[5];

        } else if arp.hwlen == maclen.try_into().unwrap() && hwaddr_mac_eq(&arp.hwaddr, &mac) {
            arp.status == ARP_FOUND.try_into().unwrap();
        } else {
            continue;
        }
        matching_arp = Some(arp.clone());
        break;
    }

    // if matching_arp == None {
    if matching_arp.is_none() {    
        matching_arp = Some(ArpRecord { status: ARP_NEW.try_into().unwrap(), hwlen: maclen.try_into().unwrap(), family: family, hwaddr: hwaddr_from_mac(&mac), addr: addrp});
    }

    return 1;
}

pub fn difftime(now: &time_t, last: &time_t) -> time_t {
    now - last
}

pub fn in6_addr_eq(a: &In6Addr, b: &In6Addr) -> bool {
    return a == b
}

pub fn copy_hwaddr_into_mac(mac: &mut[u8;6], hwaddr: &[u8;DHCP_CHADDR_MAX]) {
    mac[0] = hwaddr[0];
    mac[1] = hwaddr[1];
    mac[2] = hwaddr[2];
    mac[3] = hwaddr[3];
    mac[4] = hwaddr[4];
    mac[5] = hwaddr[5];
}

/* If in lazy mode, we cache absence of ARP entries. */
pub fn find_mac(addr: &mut Option<Mysockaddr>, mac: [u8;6], lazy: bool, now: &time_t, last: &mut time_t, arp_records: &mut Vec<ArpRecord>) -> u32
{
    let mut arp: ArpRecord;
    let mut tmp: ArpRecord;
    let mut up: ArpRecord;
    let updated: bool = false;
    
    if addr.is_none() {
        return 0;
    }

    let _addr = addr.unwrap();

    loop {
        /* If the database is less then INTERVAL old, look in there */
        if difftime(&now, &last) < INTERVAL.try_into().unwrap() {
            /* addr == NULL -> just make cache up-to-date */
           
                
            for arp in arp_records {
                if _addr.sa.sa_family != arp.family {
                    continue;
                }
                if arp.family == AF_INET && arp.addr.addr4.s_addr != 
                _addr.in4.sin_addr.s_addr {
                    continue;
                }
                if arp.status == ARP_EMPTY.try_into().unwrap() || lazy || updated {
                    if arp.hwlen != 0 {

                    }
                }
            }

            // for (arp = arps; arp; arp = arp->next) {
            for arp in arp_records {    
                if _addr.sa.sa_family!=arp.family.try_into().unwrap() {
                    continue;
                }

                if arp.family==AF_INET &&
                        arp.addr.addr4.s_addr!= _addr.in4.sin_addr.s_addr {
                            continue;
                        }

                if arp.family == AF_INET6.try_into().unwrap() && 
                    !in6_addr_eq(&arp.addr.addr6, &_addr.in6.sin6_addr) {
                        continue;
                    }


                /* Only accept positive entries unless in lazy mode. */
                if arp.status!=ARP_EMPTY.try_into().unwrap() || lazy || updated {
                    //if (mac && arp.hwlen!=0) {
                    if arp.hwlen != 0 {
                        copy_hwaddr_into_mac(&mut mac, &arp.hwaddr);
                    }
                        
                    return arp.hwlen.try_into().unwrap();
                }
            }
        }

        /* Not found, try the kernel */
        if !updated {
            updated = true;
            *last = *now;

            /* Mark all non-negative entries */
            for arp in arp_records {
                if arp.status != ARP_EMPTY.try_into().unwrap() {
                    arp.status = ARP_MARK.try_into().unwrap();
                }
            }

            // TODO: deal with filter_mac callback signature mismatch
            // iface_enumerate(AF_UNSPEC, None, filter_mac);

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
    let arp = ArpRecord {
        hwlen: 0,
        status: 0,
        family: 0,
        hwaddr: [0;DHCP_CHADDR_MAX],
        addr: AddressPointer::default(),
    };

    arp.status = ARP_EMPTY.try_into().unwrap();
    arp.family = _addr.sa.sa_family.try_into().unwrap();
    arp.hwlen = 0;
    arps.push(arp);

    if _addr.sa.sa_family==AF_INET.try_into().unwrap() {
        arp.addr.addr4.s_addr = _addr.in4.sin_addr.s_addr;
    }
    else {
        arp.addr.addr6 = _addr.in6.sin6_addr;
    }

    return 0;
}

pub fn do_arp_script_run(daemon: &Daemon) -> i32
{
    // struct arp_record* arp;
    let mut arp: ArpRecord;

    /* Notify any which went, then move to free list */
    // TODO: iterate over old vec 
    if old.len() > 0 {
        // if option_bool(OPT_SCRIPT_ARP, daemon) > 0 {
        //     queue_arp(daemon, ACTION_ARP_DEL, old.hwaddr, old.hwlen, old.family, &old.addr);
        // }
        // todo: implement differently
        // arp = old;
        // old = arp->next;
        // arp->next = freelist;
        // freelist = arp;
        return 1;
    }

    for arp in arps {
        if arp.status == ARP_NEW.try_into().unwrap() {
            if option_bool(OPT_SCRIPT_ARP, daemon) > 0 {
                queue_arp(daemon, Action::ACTION_ARP, arp.hwaddr, arp.hwlen, arp.family, arp.addr);
            }
            arp.status = ARP_FOUND.try_into().unwrap();
            return 1;
        }
    }
    return 0;
}

// END OF FILE
