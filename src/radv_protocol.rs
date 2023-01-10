/* dnsmasq is Copyright (c) 2000-2022 Simon Kelley

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

pub const ALL_NODES: String = String::from("FF02::1");
pub const ALL_ROUTERS: String = String::from("FF02::2");


pub const ICMP6_OPT_SOURCE_MAC: u32 = 1;
pub const ICMP6_OPT_PREFIX: u32 = 3;
pub const ICMP6_OPT_MTU: u32 = 5;
pub const ICMP6_OPT_ADV_INTERVAL: u32 = 7;
pub const ICMP6_OPT_RT_INFO: u32 = 24;
pub const ICMP6_OPT_RDNSS: u32 = 25;
pub const ICMP6_OPT_DNSSL: u32 = 31;
