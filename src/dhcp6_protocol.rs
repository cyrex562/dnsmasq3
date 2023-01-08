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

pub const DHCPV6_SERVER_PORT: u32 = 547;
pub const DHCPV6_CLIENT_PORT: u32 = 546;

#define ALL_SERVERS                  "FF05::1:3"
#define ALL_RELAY_AGENTS_AND_SERVERS "FF02::1:2"

pub const DHCP6SOLICIT: u32 = 1;
pub const DHCP6ADVERTISE: u32 = 2;
pub const DHCP6REQUEST: u32 = 3;
pub const DHCP6CONFIRM: u32 = 4;
pub const DHCP6RENEW: u32 = 5;
pub const DHCP6REBIND: u32 = 6;
pub const DHCP6REPLY: u32 = 7;
pub const DHCP6RELEASE: u32 = 8;
pub const DHCP6DECLINE: u32 = 9;
pub const DHCP6RECONFIGURE: u32 = 10;
pub const DHCP6IREQ: u32 = 11;
pub const DHCP6RELAYFORW: u32 = 12;
pub const DHCP6RELAYREPL: u32 = 13;

pub const OPTION6_CLIENT_ID: u32 = 1;
pub const OPTION6_SERVER_ID: u32 = 2;
pub const OPTION6_IA_NA: u32 = 3;
pub const OPTION6_IA_TA: u32 = 4;
pub const OPTION6_IAADDR: u32 = 5;
pub const OPTION6_ORO: u32 = 6;
pub const OPTION6_PREFERENCE: u32 = 7;
pub const OPTION6_ELAPSED_TIME: u32 = 8;
pub const OPTION6_RELAY_MSG: u32 = 9;
pub const OPTION6_AUTH: u32 = 11;
pub const OPTION6_UNICAST: u32 = 12;
pub const OPTION6_STATUS_CODE: u32 = 13;
pub const OPTION6_RAPID_COMMIT: u32 = 14;
pub const OPTION6_USER_CLASS: u32 = 15;
pub const OPTION6_VENDOR_CLASS: u32 = 16;
pub const OPTION6_VENDOR_OPTS: u32 = 17;
pub const OPTION6_INTERFACE_ID: u32 = 18;
pub const OPTION6_RECONFIGURE_MSG: u32 = 19;
pub const OPTION6_RECONF_ACCEPT: u32 = 20;
pub const OPTION6_DNS_SERVER: u32 = 23;
pub const OPTION6_DOMAIN_SEARCH: u32 = 24;
pub const OPTION6_IA_PD: u32 = 25;
pub const OPTION6_IAPREFIX: u32 = 26;
pub const OPTION6_REFRESH_TIME: u32 = 32;
pub const OPTION6_REMOTE_ID: u32 = 37;
pub const OPTION6_SUBSCRIBER_ID: u32 = 38;
pub const OPTION6_FQDN: u32 = 39;
pub const OPTION6_NTP_SERVER: u32 = 56;
pub const OPTION6_CLIENT_MAC: u32 = 79;
pub const OPTION6_MUD_URL: u32 = 112;

pub const NTP_SUBOPTION_SRV_ADDR: u32 = 1;
pub const NTP_SUBOPTION_MC_ADDR: u32 = 2;
pub const NTP_SUBOPTION_SRV_FQDN: u32 = 3;

pub const DHCP6SUCCESS: u32 = 0;
pub const DHCP6UNSPEC: u32 = 1;
pub const DHCP6NOADDRS: u32 = 2;
pub const DHCP6NOBINDING: u32 = 3;
pub const DHCP6NOTONLINK: u32 = 4;
pub const DHCP6USEMULTI: u32 = 5;
