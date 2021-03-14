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

use crate::dnsmasq_sys::In6Addr;

const ALL_NODES: &str = "FF02::1";
const ALL_ROUTERS: &str = "FF02::2";

const ICMP6_OPT_SOURCE_MAC: u8 =  1;
const ICMP6_OPT_PREFIX: u8 =      3;
const ICMP6_OPT_MTU: u8 =         5;
const ICMP6_OPT_ADV_INTERVAL: u8 = 7;
const ICMP6_OPT_RT_INFO: u8 =     24;
const ICMP6_OPT_RDNSS: u8 =      25;
const ICMP6_OPT_DNSSL: u8 =      31;

pub struct ping_packet {
  pkt_type: u8,
  code: u8,
  checksum: u16,
  identifier: u16,
  sequence_no: u16,
}

struct ra_packet {
  pkt_type: u8,
  code: u8,
  checksum: u16,
  hop_limit: u8,
  flags: u8,
  lifetime: u16,
  reachable_time: u32,
  retrans_time: u32,
}

struct neigh_packet {
  pkt_type: u8,
  code: u8,
  checksum: u16,
  reserved: u16,
  target: In6Addr,
}

struct prefix_opt {
  opt_type: u8,
  len: u8,
  prefix_len: u8,
  flags: u8,
  valid_lifetime: u32,
  preferred_lifetime: u32,
  reserved: u32,
  prefix: In6Addr
}





