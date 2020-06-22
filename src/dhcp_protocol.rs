use libc::in_addr;

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
pub const DHCP_SERVER_PORT: u32= 67;
pub const DHCP_CLIENT_PORT: u32= 68;
pub const DHCP_SERVER_ALTPORT: u32= 1067;
pub const DHCP_CLIENT_ALTPORT: u32= 1068;
pub const PXE_PORT: u32= 4011;

/* These each hold a DHCP option max size 255
   and get a terminating zero added */
pub const DHCP_BUFF_SZ: u32= 256;

pub const BOOTREQUEST  :u32 =            1;
pub const BOOTREPLY: u32= 2;
pub const DHCP_COOKIE: u32= 0x63825363;

/* The Linux in-kernel DHCP client silently ignores any packet 
   smaller than this. Sigh...........   */
pub const MIN_PACKETSZ: u32= 300;

pub const OPTION_PAD: u32= 0;
pub const OPTION_NETMASK: u32= 1;
pub const OPTION_ROUTER: u32= 3;
pub const OPTION_DNSSERVER: u32= 6;
pub const OPTION_HOSTNAME: u32= 12;
pub const OPTION_DOMAINNAME: u32= 15;
pub const OPTION_BROADCAST: u32= 28;
pub const OPTION_VENDOR_CLASS_OPT: u32= 43;
pub const OPTION_REQUESTED_IP: u32= 50; 
pub const OPTION_LEASE_TIME: u32= 51;
pub const OPTION_OVERLOAD: u32= 52;
pub const OPTION_MESSAGE_TYPE: u32= 53;
pub const OPTION_SERVER_IDENTIFIER: u32= 54;
pub const OPTION_REQUESTED_OPTIONS: u32= 55;
pub const OPTION_MESSAGE: u32= 56;
pub const OPTION_MAXMESSAGE: u32= 57;
pub const OPTION_T1: u32= 58;
pub const OPTION_T2: u32= 59;
pub const OPTION_VENDOR_ID: u32= 60;
pub const OPTION_CLIENT_ID: u32= 61;
pub const OPTION_SNAME: u32= 66;
pub const OPTION_FILENAME: u32= 67;
pub const OPTION_USER_CLASS: u32= 77;
pub const OPTION_RAPID_COMMIT: u32= 80;
pub const OPTION_CLIENT_FQDN: u32= 81;
pub const OPTION_AGENT_ID: u32= 82;
pub const OPTION_ARCH: u32= 93;
pub const OPTION_PXE_UUID: u32= 97;
pub const OPTION_SUBNET_SELECT: u32= 118;
pub const OPTION_DOMAIN_SEARCH: u32= 119;
pub const OPTION_SIP_SERVER: u32= 120;
pub const OPTION_VENDOR_IDENT: u32= 124;
pub const OPTION_VENDOR_IDENT_OPT: u32= 125;
pub const OPTION_END: u32= 255;

pub const SUBOPT_CIRCUIT_ID: u32= 1;
pub const SUBOPT_REMOTE_ID: u32= 2;
pub const SUBOPT_SUBNET_SELECT: u32= 5;     /* RFC 3527 */
pub const SUBOPT_SUBSCR_ID: u32= 6;     /* RFC 3393 */
pub const SUBOPT_SERVER_OR: u32= 11;    /* RFC 5107 */

pub const SUBOPT_PXE_BOOT_ITEM: u32= 71;    /* PXE standard */
pub const SUBOPT_PXE_DISCOVERY: u32= 6;
pub const SUBOPT_PXE_SERVERS: u32= 8;
pub const SUBOPT_PXE_MENU: u32= 9;
pub const SUBOPT_PXE_MENU_PROMPT: u32= 10;

pub const DHCPDISCOVER: u32= 1;
pub const DHCPOFFER: u32= 2;
pub const DHCPREQUEST: u32= 3;
pub const DHCPDECLINE: u32= 4;
pub const DHCPACK: u32= 5;
pub const DHCPNAK: u32= 6;
pub const DHCPRELEASE: u32= 7;
pub const DHCPINFORM: u32= 8;

pub const BRDBAND_FORUM_IANA: u32= 3561; /* Broadband forum IANA enterprise */

pub const DHCP_CHADDR_MAX: usize = 16;

pub struct dhcp_packet {
   op: u8,
   htype: u8,
   hlen: u8,
   hops: u8,
   xid: u32,
   flags: u16,
   secs: u16,
   ciaddr: in_addr,
   yiaddr: in_addr,
   siaddr: in_addr,
   giaddr: in_addr,
   chaddr: [u8;DHCP_CHADDR_MAX],
   sname: [u8;64],
   file: [u8;128],
   options: [u8;312],
}
