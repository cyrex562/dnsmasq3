
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
    [b"dns_cache_inserted\x00" ,
     b"dns_cache_live_freed\x00" ,
     b"dns_queries_forwarded\x00" ,
     b"dns_auth_answered\x00" ,
     b"dns_local_answered\x00" ,
     b"bootp\x00" ,
     b"pxe\x00" ,
     b"dhcp_ack\x00" ,
     b"dhcp_decline\x00" ,
     b"dhcp_discover\x00" ,
     b"dhcp_inform\x00" ,
     b"dhcp_nak\x00" ,
     b"dhcp_offer\x00" ,
     b"dhcp_release\x00" ,
     b"dhcp_request\x00" ,
     b"noanswer\x00" ,
     b"leases_allocated_4\x00" ,
     b"leases_pruned_4\x00" ,
     b"leases_allocated_6\x00" ,
     b"leases_pruned_6\x00" ];
#[no_mangle]
pub unsafe extern "C" fn get_metric_name(mut i: i32)
 -> *const libc::c_char {
    return metric_names[i ];
}
