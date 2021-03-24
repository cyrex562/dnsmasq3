
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
#[no_mangle]
pub static mut metric_names: [*const libc::c_char; 20] =
    [b"dns_cache_inserted\x00" as *const u8 as *const libc::c_char,
     b"dns_cache_live_freed\x00" as *const u8 as *const libc::c_char,
     b"dns_queries_forwarded\x00" as *const u8 as *const libc::c_char,
     b"dns_auth_answered\x00" as *const u8 as *const libc::c_char,
     b"dns_local_answered\x00" as *const u8 as *const libc::c_char,
     b"bootp\x00" as *const u8 as *const libc::c_char,
     b"pxe\x00" as *const u8 as *const libc::c_char,
     b"dhcp_ack\x00" as *const u8 as *const libc::c_char,
     b"dhcp_decline\x00" as *const u8 as *const libc::c_char,
     b"dhcp_discover\x00" as *const u8 as *const libc::c_char,
     b"dhcp_inform\x00" as *const u8 as *const libc::c_char,
     b"dhcp_nak\x00" as *const u8 as *const libc::c_char,
     b"dhcp_offer\x00" as *const u8 as *const libc::c_char,
     b"dhcp_release\x00" as *const u8 as *const libc::c_char,
     b"dhcp_request\x00" as *const u8 as *const libc::c_char,
     b"noanswer\x00" as *const u8 as *const libc::c_char,
     b"leases_allocated_4\x00" as *const u8 as *const libc::c_char,
     b"leases_pruned_4\x00" as *const u8 as *const libc::c_char,
     b"leases_allocated_6\x00" as *const u8 as *const libc::c_char,
     b"leases_pruned_6\x00" as *const u8 as *const libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn get_metric_name(mut i: libc::c_int)
 -> *const libc::c_char {
    return metric_names[i as usize];
}
