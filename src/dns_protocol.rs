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

pub const NAMESERVER_PORT: u32 = 53;
pub const TFTP_PORT: u32 = 69;
pub const MIN_PORT: u32 = 1024;           /* first non-reserved port */
pub const MAX_PORT: u32 = 65535;u

pub const IN6ADDRSZ: u32 = 16;
pub const INADDRSZ: u32 = 4;

pub const PACKETSZ: u32 = 512;		/* maximum packet size */
pub const MAXDNAME: usize = 1025;		/* maximum presentation domain name */
pub const RRFIXEDSZ: u32 = 10;		/* #/bytes of fixed data in r record */
pub const MAXLABEL: u32 = 63;              /* maximum length of domain label */

pub const NOERROR: u32 = 0;		/* no error */
pub const FORMERR: u32 = 1;		/* format error */
pub const SERVFAIL: u32 = 2;		/* server failure */
pub const NXDOMAIN: u32 = 3;		/* non existent domain */
pub const NOTIMP: u32 = 4;		/* not implemented */
pub const REFUSED: u32 = 5;		/* query refused */

pub const QUERY: u32 = 0;               /* opcode */

pub const C_IN: u32 = 1;               /* the arpa internet */
pub const C_CHAOS: u32 = 3;               /* for chaos net (MIT) */
pub const C_HESIOD: u32 = 4;               /* hesiod */
pub const C_ANY: u32 = 255;             /* wildcard match */

pub const T_A: u32 = 1;
pub const T_NS: u32 = 2;
pub const T_MD: u32 = 3;
pub const T_MF: u32 = 4;
pub const T_CNAME: u32 = 5;
pub const T_SOA: u32 = 6;
pub const T_MB: u32 = 7;
pub const T_MG: u32 = 8;
pub const T_MR: u32 = 9;
pub const T_PTR: u32 = 12;
pub const T_MINFO: u32 = 14;
pub const T_MX: u32 = 15;
pub const T_TXT: u32 = 16;
pub const T_RP: u32 = 17;
pub const T_AFSDB: u32 = 18;
pub const T_RT: u32 = 21;
pub const T_SIG: u32 = 24;
pub const T_PX: u32 = 26;
pub const T_AAAA: u32 = 28;
pub const T_NXT: u32 = 30;
pub const T_SRV: u32 = 33;
pub const T_NAPTR: u32 = 35;
pub const T_KX: u32 = 36;
pub const T_DNAME: u32 = 39;
pub const T_OPT: u32 = 41;
pub const T_DS: u32 = 43;
pub const T_RRSIG: u32 = 46;
pub const T_NSEC: u32 = 47;
pub const T_DNSKEY: u32 = 48;
pub const T_NSEC3: u32 = 50;
pub const T_TKEY: u32 = 249;
pub const T_TSIG: u32 = 250;
pub const T_AXFR: u32 = 252;
pub const T_MAILB: u32 = 253;
pub const T_ANY: u32 = 255;
pub const T_CAA: u32 = 257;

pub const EDNS0_OPTION_MAC: u32 = 65001; /* dyndns.org temporary assignment */
pub const EDNS0_OPTION_CLIENT_SUBNET: u32 = 8;     /* IANA */
pub const EDNS0_OPTION_EDE: u32 = 15;    /* IANA - RFC 8914 */
pub const EDNS0_OPTION_NOMDEVICEID: u32 = 65073; /* Nominum temporary assignment */
pub const EDNS0_OPTION_NOMCPEID: u32 = 65074; /* Nominum temporary assignment */
pub const EDNS0_OPTION_UMBRELLA: u32 = 20292; /* Cisco Umbrella temporary assignment */

/* RFC-8914 extended errors, negative values are our definitions */
#define EDE_UNSET          -1  /* No extended DNS error available */
pub const EDE_OTHER: u32 = 0;  /* Other */
pub const EDE_USUPDNSKEY: u32 = 1;  /* Unsupported DNSKEY algo */
pub const EDE_USUPDS: u32 = 2;  /* Unsupported DS Digest */
pub const EDE_STALE: u32 = 3;  /* Stale answer */
pub const EDE_FORGED: u32 = 4;  /* Forged answer */
pub const EDE_DNSSEC_IND: u32 = 5;  /* DNSSEC Indeterminate  */
pub const EDE_DNSSEC_BOGUS: u32 = 6;  /* DNSSEC Bogus */
pub const EDE_SIG_EXP: u32 = 7;  /* Signature Expired */
pub const EDE_SIG_NYV: u32 = 8;  /* Signature Not Yet Valid  */
pub const EDE_NO_DNSKEY: u32 = 9;  /* DNSKEY missing */
pub const EDE_NO_RRSIG: u32 = 10;  /* RRSIGs missing */
pub const EDE_NO_ZONEKEY: u32 = 11;  /* No Zone Key Bit Set */
pub const EDE_NO_NSEC: u32 = 12;  /* NSEC Missing  */
pub const EDE_CACHED_ERR: u32 = 13;  /* Cached Error */
pub const EDE_NOT_READY: u32 = 14;  /* Not Ready */
pub const EDE_BLOCKED: u32 = 15;  /* Blocked */
pub const EDE_CENSORED: u32 = 16;  /* Censored */
pub const EDE_FILTERED: u32 = 17;  /* Filtered */
pub const EDE_PROHIBITED: u32 = 18;  /* Prohibited */
pub const EDE_STALE_NXD: u32 = 19;  /* Stale NXDOMAIN */
pub const EDE_NOT_AUTH: u32 = 20;  /* Not Authoritative */
pub const EDE_NOT_SUP: u32 = 21;  /* Not Supported */
pub const EDE_NO_AUTH: u32 = 22;  /* No Reachable Authority */
pub const EDE_NETERR: u32 = 23;  /* Network error */
pub const EDE_INVALID_DATA: u32 = 24;  /* Invalid Data */





pub const HB3_QR: u32 = 0x80; /* Query */
pub const HB3_OPCODE: u32 = 0x78;
pub const HB3_AA: u32 = 0x04; /* Authoritative Answer */
pub const HB3_TC: u32 = 0x02; /* TrunCated */
pub const HB3_RD: u32 = 0x01; /* Recursion Desired */

pub const HB4_RA: u32 = 0x80; /* Recursion Available */
pub const HB4_AD: u32 = 0x20; /* Authenticated Data */
pub const HB4_CD: u32 = 0x10; /* Checking Disabled */
pub const HB4_RCODE: u32 = 0x0;f

#define OPCODE(x)          (((x).hb3 & HB3_OPCODE) >> 3)
#define SET_OPCODE(x, code) (x).hb3 = ((x).hb3 & ~HB3_OPCODE) | code

#define RCODE(x)           ((x).hb4 & HB4_RCODE)
#define SET_RCODE(x, code) (x).hb4 = ((x).hb4 & ~HB4_RCODE) | code
  
#define GETSHORT(s, cp) { \
	unsigned char *t_cp = (unsigned char *)(cp); \
	(s) = ((u16)t_cp[0] << 8) \
	    | ((u16)t_cp[1]) \
	    ; \
	(cp) += 2; \
}

#define GETLONG(l, cp) { \
	unsigned char *t_cp = (unsigned char *)(cp); \
	(l) = ((u32)t_cp[0] << 24) \
	    | ((u32)t_cp[1] << 16) \
	    | ((u32)t_cp[2] << 8) \
	    | ((u32)t_cp[3]) \
	    ; \
	(cp) += 4; \
}

#define PUTSHORT(s, cp) { \
	u16 t_s = (u16)(s); \
	unsigned char *t_cp = (unsigned char *)(cp); \
	*t_cp++ = t_s >> 8; \
	*t_cp   = t_s; \
	(cp) += 2; \
}

#define PUTLONG(l, cp) { \
	u32 t_l = (u32)(l); \
	unsigned char *t_cp = (unsigned char *)(cp); \
	*t_cp++ = t_l >> 24; \
	*t_cp++ = t_l >> 16; \
	*t_cp++ = t_l >> 8; \
	*t_cp   = t_l; \
	(cp) += 4; \
}

#define CHECK_LEN(header, pp, plen, len) \
    ((size_t)((pp) - (unsigned char *)(header) + (len)) <= (plen))

#define ADD_RDLEN(header, pp, plen, len) \
  (!CHECK_LEN(header, pp, plen, len) ? 0 : (((pp) += (len)), 1))

/* Escape character in our presentation format for names.
   Cannot be '.' or /000 and must be !isprint().
   Note that escaped chars are stored as
   <NAME_ESCAPE> <orig-char+1>
   to ensure that the escaped form of /000 doesn't include /000
*/
pub const NAME_ESCAPE: u32 = 1;
