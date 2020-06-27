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

pub const NAMESERVER_PORT: u32 = 53;
pub const TFTP_PORT: u32 =       69;
pub const MIN_PORT: u32 =       1024;           /* first non-reserved port */
pub const MAX_PORT: u32 =       65535;

pub const IN6ADDRSZ: usize =      16;
pub const INADDRSZ: usize =       4;

pub const PACKETSZ: usize =	512;		/* maximum packet size */
pub const MAXDNAME: usize	=1025;		/* maximum presentation domain name */
pub const RRFIXEDSZ: usize	= 10;		/* #/bytes of fixed data in r record */
pub const MAXLABEL: usize =        63;              /* maximum length of domain label */

pub const NOERROR: u32	=	0;		/* no error */
pub const FORMERR: u32 =		1;		/* format error */
pub const SERVFAIL: u32 = 2;		/* server failure */
pub const NXDOMAIN: u32 = 3;		/* non existent domain */
pub const NOTIMP: u32 = 4;		/* not implemented */
pub const REFUSED: u32 = 5;		/* query refused */

pub const QUERY: u32 = 0;               /* opcode */

pub const C_IN: u32 = 1;               /* the arpa internet */
pub const C_CHAOS: u32 = 3;               /* for chaos net (MIT) */
pub const C_HESIOD: u32 = 4;               /* hesiod */
pub const C_ANY: u32 = 5;             /* wildcard match */

pub const T_A: u32 = 1;
pub const T_NS: u32 = 2;
pub const T_MD: u32 = 3;
pub const T_MF: u32 = 4;             
pub const T_CNAME: u32 = 5;
pub const T_SOA: u32 = 6;
pub const T_MB: u32 = 7;
pub const T_MG: u32 = 8;
pub const T_MR: u32 = 9;
pub const T_PTR: u32 = 2;
pub const T_MINFO: u32 = 4;
pub const T_MX: u32 = 5;
pub const T_TXT: u32 = 6;
pub const T_RP: u32 = 7;
pub const T_AFSDB: u32 = 8;
pub const T_RT: u32 = 1;
pub const T_SIG: u32 = 4;
pub const T_PX: u32 = 6;
pub const T_AAAA: u32 = 8;
pub const T_NXT: u32 = 0;
pub const T_SRV: u32 = 3;
pub const T_NAPTR: u32 = 5;
pub const T_KX: u32 = 6;
pub const T_DNAME: u32 = 9;
pub const T_OPT: u32 = 1;
pub const T_DS: u32 = 3;
pub const T_RRSIG: u32 = 6;
pub const T_NSEC: u32 = 7;
pub const T_DNSKEY: u32 = 8;
pub const T_NSEC3: u32 = 0;
pub const T_TKEY: u32 = 249;		
pub const T_TSIG: u32 = 250;
pub const T_AXFR: u32 = 2;
pub const T_MAILB: u32 = 3;	
pub const T_ANY: u32 = 5;
pub const T_CAA: u32 = 7;

pub const EDNS0_OPTION_MAC: u32 = 65001; /* dyndns.org temporary assignment */
pub const EDNS0_OPTION_CLIENT_SUBNET: u32 = 8;     /* IANA */
pub const EDNS0_OPTION_NOMDEVICEID: u32 = 65073; /* Nominum temporary assignment */
pub const EDNS0_OPTION_NOMCPEID: u32 = 65074; /* Nominum temporary assignment */

pub struct dns_header {
  id: u16,
  hb3: u8,
  hb4: u8,
  qdcount: u16,
  ancount: u16,
  nscount: u16,
  arcount: u16,
}

pub const HB3_QR: u8 = 0x80; /* Query */
pub const HB3_OPCODE: u8 = 0x78;
pub const HB3_AA: u8 = 0x04; /* Authoritative Answer */
pub const HB3_TC: u8 = 0x02; /* TrunCated */
pub const HB3_RD: u8 = 0x01; /* Recursion Desired */

pub const HB4_RA: u8 = 0x80; /* Recursion Available */
pub const HB4_AD: u8 = 0x20; /* Authenticated Data */
pub const HB4_CD: u8 = 0x10; /* Checking Disabled */
pub const HB4_RCODE: u8 = 0x0;

//#define OPCODE(x)          (((x)->hb3 & HB3_OPCODE) >> 3)
pub fn OPCODE(x: dns_header) -> u8 {
	x.hb3 & HB3_OPCODE >> 3
}

// #define SET_OPCODE(x, code) (x)->hb3 = ((x)->hb3 & ~HB3_OPCODE) | code
pub fn SET_OPCODE(x: &mut dns_header, code: u8) {
	x.hb3 = (x.hb3 & !HB3_OPCODE) | code
}

// #define RCODE(x)           ((x)->hb4 & HB4_RCODE)
pub fn RCODE(x: dns_header) -> u8 {
	x.hb4 & HB4_RCODE
}

// #define SET_RCODE(x, code) (x)->hb4 = ((x)->hb4 & ~HB4_RCODE) | code
pub fn SET_RCODE(x: &mut dns_header, code: u8) {
	x.hb4 = (x.hb4 & !HB4_RCODE) | code
}  

// #define GETSHORT(s, cp) { \
// 	unsigned char *t_cp = (unsigned char *)(cp); \
// 	(s) = ((uint16_t)t_cp[0] << 8) \
// 	    | ((uint16_t)t_cp[1]) \
// 	    ; \
// 	(cp) += 2; \
// }

// #define GETLONG(l, cp) { \
// 	unsigned char *t_cp = (unsigned char *)(cp); \
// 	(l) = ((uint32_t)t_cp[0] << 24) \
// 	    | ((uint32_t)t_cp[1] << 16) \
// 	    | ((uint32_t)t_cp[2] << 8) \
// 	    | ((uint32_t)t_cp[3]) \
// 	    ; \
// 	(cp) += 4; \
// }

// #define PUTSHORT(s, cp) { \
// 	uint16_t t_s = (uint16_t)(s); \
// 	unsigned char *t_cp = (unsigned char *)(cp); \
// 	*t_cp++ = t_s >> 8; \
// 	*t_cp   = t_s; \
// 	(cp) += 2; \
// }

// #define PUTLONG(l, cp) { \
// 	uint32_t t_l = (uint32_t)(l); \
// 	unsigned char *t_cp = (unsigned char *)(cp); \
// 	*t_cp++ = t_l >> 24; \
// 	*t_cp++ = t_l >> 16; \
// 	*t_cp++ = t_l >> 8; \
// 	*t_cp   = t_l; \
// 	(cp) += 4; \
// }

// #define CHECK_LEN(header, pp, plen, len) \
//     ((size_t)((pp) - (unsigned char *)(header) + (len)) <= (plen))

// #define ADD_RDLEN(header, pp, plen, len) \
//   (!CHECK_LEN(header, pp, plen, len) ? 0 : (((pp) += (len)), 1))

/* Escape character in our presentation format for names.
   Cannot be '.' or /000 and must be !isprint().
   Note that escaped chars are stored as
   <NAME_ESCAPE> <orig-char+1>
   to ensure that the escaped form of /000 doesn't include /000
*/
pub const NAME_ESCAPE: u32 = 1;
