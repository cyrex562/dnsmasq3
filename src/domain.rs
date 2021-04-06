
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
use crate::defines::{NetAddress, CondDomain, __bswap_32, DnsmasqDaemon, In6Addr};
use crate::util::{addr6part, is_same_net6};

unsafe extern "C" fn search_domain(mut addr: NetAddress, mut c: &mut CondDomain)
                                   -> {
    while !c.is_null() {
        if c.is6 == 0 &&
               __bswap_32(addr.s_addr) >= __bswap_32(c.start.s_addr) &&
               __bswap_32(addr.s_addr) <= __bswap_32(c.end.s_addr) {
            return c
        }
        c = c.next
    }
    return 0 ;
}
#[no_mangle]
pub unsafe extern "C" fn get_domain(mut addr: NetAddress) -> &mut String {
    let mut c: CondDomain = 0 ;
    c = search_domain(addr, dnsmasq_daemon.cond_domain);
    if !c.is_null() { return c.domain }
    return dnsmasq_daemon.domain_suffix;
}
unsafe extern "C" fn search_domain6(mut addr: &mut In6Addr,
                                    mut c: &mut CondDomain)
                                    -> {
    let mut addrpart: u64 = addr6part(addr);
    while !c.is_null() {
        if c.is6 != 0 &&
               is_same_net6(addr, &mut c.start6, 64) != 0 &&
               addrpart >= addr6part(&mut c.start6) &&
               addrpart <= addr6part(&mut c.end6) {
            return c
        }
        c = c.next
    }
    return 0 ;
}
#[no_mangle]
pub unsafe extern "C" fn get_domain6(mut addr: &mut In6Addr)
 -> &mut String {
    let mut c: CondDomain = 0 ;
    if !addr.is_null() &&
           {
               c = search_domain6(addr, dnsmasq_daemon.cond_domain);
               !c.is_null()
           } {
        return c.domain
    }
    return dnsmasq_daemon.domain_suffix;
}
