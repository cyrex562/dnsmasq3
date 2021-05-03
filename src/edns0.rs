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



pub fn find_pseudoheader(struct dns_header *header, plen: usize, size_t  *len, unsigned char **p, int *is_sign, int *is_last) -> &mut Vec<u8>
{
  /* See if packet has an RFC2671 pseudoheader, and if so return a pointer to it. 
     also return length of pseudoheader in *len and pointer to the UDP size in *p
     Finally, check to see if a packet is signed. If it is we cannot change a single bit before
     forwarding. We look for TSIG in the addition section, and TKEY queries (for GSS-TSIG) */
  
  i: i32, arcount = ntohs(header.arcount);
  unsigned char *ansp = (header+1);
  u16 rdlen, type, class;
  unsigned char *ret = NULL;

  if (is_sign)
    {
      *is_sign = 0;

      if (OPCODE(header) == QUERY)
	{
	  for (i = ntohs(header.qdcount); i != 0; i--)
	    {
	      if (!(ansp = skip_name(ansp, header, plen, 4)))
		return NULL;
	      
	      GETSHORT(type, ansp); 
	      GETSHORT(class, ansp);
	      
	      if (class == C_IN && type == T_TKEY)
		*is_sign = 1;
	    }
	}
    }
  else
    {
      if (!(ansp = skip_questions(header, plen)))
	return NULL;
    }
    
  if (arcount == 0)
    return NULL;
  
  if (!(ansp = skip_section(ansp, ntohs(header.ancount) + ntohs(header.nscount), header, plen)))
    return NULL; 
  
  for i in 0..arcount
    {
      save: &mut Vec<u8>, *start = ansp;
      if (!(ansp = skip_name(ansp, header, plen, 10)))
	return NULL; 

      GETSHORT(type, ansp);
      save = ansp;
      GETSHORT(class, ansp);
      ansp += 4; /* TTL */
      GETSHORT(rdlen, ansp);
      if (!ADD_RDLEN(header, ansp, plen, rdlen))
	return NULL;
      if (type == T_OPT)
	{
	  if (len)
	    *len = ansp - start;

	  if (p)
	    *p = save;
	  
	  if (is_last)
	    *is_last = (i == arcount-1);

	  ret = start;
	}
      else if (is_sign && 
	       i == arcount - 1 && 
	       class == C_ANY && 
	       type == T_TSIG)
	*is_sign = 1;
    }
  
  return ret;
}
 

/* replace == 2 .delete existing option only. */
add_pseudoheader: usize(struct dns_header *header, plen: usize, limit: &mut Vec<u8>, 
			u16 udp_sz, optno: i32, opt: &mut Vec<u8>, optlen: usize, set_do: i32, replace: i32)
{ 
  lenp: &mut Vec<u8>, *datap, *p, *udp_len, *buff = NULL;
  int rdlen = 0, is_sign, is_last;
  u16 flags = set_do ? 0x8000 : 0, rcode = 0;

  p = find_pseudoheader(header, plen, NULL, &udp_len, &is_sign, &is_last);
  
  if (is_sign)
    return plen;

  if (p)
    {
      /* Existing header */
      let mut i: i32;
      u16 code, len;

      p = udp_len;
      GETSHORT(udp_sz, p);
      GETSHORT(rcode, p);
      GETSHORT(flags, p);

      if (set_do)
	{
	  p -= 2;
	  flags |= 0x8000;
	  PUTSHORT(flags, p);
	}

      lenp = p;
      GETSHORT(rdlen, p);
      if (!CHECK_LEN(header, p, plen, rdlen))
	return plen; /* bad packet */
      datap = p;

       /* no option to add */
      if (optno == 0)
	return plen;
      	  
      /* check if option already there */
      for (i = 0; i + 4 < rdlen;)
	{
	  GETSHORT(code, p);
	  GETSHORT(len, p);
	  
	  /* malformed option, delete the whole OPT RR and start again. */
	  if (i + 4 + len > rdlen)
	    {
	      rdlen = 0;
	      is_last = 0;
	      break;
	    }
	  
	  if (code == optno)
	    {
	      if (replace == 0)
		return plen;

	      /* delete option if we're to replace it. */
	      p -= 4;
	      rdlen -= len + 4;
	      memmove(p, p+len+4, rdlen - i);
	      PUTSHORT(rdlen, lenp);
	      lenp -= 2;
	    }
	  else
	    {
	      p += len;
	      i += len + 4;
	    }
	}

      /* If we're going to extend the RR, it has to be the last RR in the packet */
      if (!is_last)
	{
	  /* First, take a copy of the options. */
	  if (rdlen != 0 && (buff = whine_malloc(rdlen)))
	    memcpy(buff, datap, rdlen);	      
	  
	  /* now, delete OPT RR */
	  plen = rrfilter(header, plen, 0);
	  
	  /* Now, force addition of a new one */
	  p = NULL;	  
	}
    }
  
  if (!p)
    {
      /* We are (re)adding the pseudoheader */
      if (!(p = skip_questions(header, plen)) ||
	  !(p = skip_section(p, 
			     ntohs(header.ancount) + ntohs(header.nscount) + ntohs(header.arcount), 
			     header, plen)))
      {
	free(buff);
	return plen;
      }
      if (p + 11 > limit)
      {
        free(buff);
        return plen; /* Too big */
      }
      *p++ = 0; /* empty name */
      PUTSHORT(T_OPT, p);
      PUTSHORT(udp_sz, p); /* max packet length, 512 if not given in EDNS0 header */
      PUTSHORT(rcode, p);    /* extended RCODE and version */
      PUTSHORT(flags, p); /* DO flag */
      lenp = p;
      PUTSHORT(rdlen, p);    /* RDLEN */
      datap = p;
      /* Copy back any options */
      if (buff)
	{
          if (p + rdlen > limit)
          {
            free(buff);
            return plen; /* Too big */
          }
	  memcpy(p, buff, rdlen);
	  free(buff);
	  p += rdlen;
	}
      
      /* Only bump arcount if RR is going to fit */ 
      if (((ssize_t)optlen) <= (limit - (p + 4)))
	header.arcount = htons(ntohs(header.arcount) + 1);
    }
  
  if (((ssize_t)optlen) > (limit - (p + 4)))
    return plen; /* Too big */
  
  /* Add new option */
  if (optno != 0 && replace != 2)
    {
      if (p + 4 > limit)
       return plen; /* Too big */
      PUTSHORT(optno, p);
      PUTSHORT(optlen, p);
      if (p + optlen > limit)
       return plen; /* Too big */
      memcpy(p, opt, optlen);
      p += optlen;  
      PUTSHORT(p - datap, lenp);
    }
  return p - header;
}

add_do_bit: usize(struct dns_header *header, plen: usize, unsigned char *limit)
{
  return add_pseudoheader(header, plen, limit, PACKETSZ, 0, NULL, 0, 1, 0);
}

 unsigned char char64(unsigned char c)
{
  return "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"[c & 0x3f];
}

pub fn encoder(in: &mut Vec<u8>, out: &mut String)
{
  out[0] = char64(in[0]>>2);
  out[1] = char64((in[0]<<4) | (in[1]>>4));
  out[2] = char64((in[1]<<2) | (in[2]>>6));
  out[3] = char64(in[2]);
}

 add_dns_client: usize(struct dns_header *header, plen: usize, limit: &mut Vec<u8>,
			     union mysockaddr *l3, now: &time::Instant, int *cacheablep)
{
  maclen: i32, replace = 2; /* can't get mac address, just delete any incoming. */
  unsigned char mac[DHCP_CHADDR_MAX];
  char encode[18]; /* handle 6 byte MACs */

  if ((maclen = find_mac(l3, mac, 1, now)) == 6)
    {
      replace = 1;
      *cacheablep = 0;

      if (option_bool(OPT_MAC_HEX))
	print_mac(encode, mac, maclen);
      else
	{
	  encoder(mac, encode);
	  encoder(mac+3, encode+4);
	  encode[8] = 0;
	}
    }

  return add_pseudoheader(header, plen, limit, PACKETSZ, EDNS0_OPTION_NOMDEVICEID, encode, strlen(encode), 0, replace); 
}


 add_mac: usize(struct dns_header *header, plen: usize, limit: &mut Vec<u8>,
		      union mysockaddr *l3, now: &time::Instant, int *cacheablep)
{
  let mut maclen: i32;
  unsigned char mac[DHCP_CHADDR_MAX];

  if ((maclen = find_mac(l3, mac, 1, now)) != 0)
    {
      *cacheablep = 0;
      plen = add_pseudoheader(header, plen, limit, PACKETSZ, EDNS0_OPTION_MAC, mac, maclen, 0, 0); 
    }
  
  return plen; 
}

struct subnet_opt {
  u16 family;
  u8 source_netmask, scope_netmask; 
  u8 addr[IN6ADDRSZ];
};

pub fn *get_addrp(union mysockaddr *addr, const short family) 
{
  if (family == AF_INET6)
    return &addr.in6.sin6_addr;

  return &addr.in.sin_addr;
}

 calc_subnet_opt: usize(struct subnet_opt *opt, union mysockaddr *source, int *cacheablep)
{
  /* http://tools.ietf.org/html/draft-vandergaast-edns-client-subnet-02 */
  
  let mut len: i32;
  addrp: Vec<u8> = NULL;
  int sa_family = source.sa.sa_family;
  let mut cacheable: i32 = 0;
  
  opt.source_netmask = 0;
  opt.scope_netmask = 0;
    
  if (source.sa.sa_family == AF_INET6 && daemon.add_subnet6)
    {
      opt.source_netmask = daemon.add_subnet6.mask;
      if (daemon.add_subnet6.addr_used) 
	{
	  sa_family = daemon.add_subnet6.addr.sa.sa_family;
	  addrp = get_addrp(&daemon.add_subnet6.addr, sa_family);
	  cacheable = 1;
	} 
      else 
	addrp = &source.in6.sin6_addr;
    }

  if (source.sa.sa_family == AF_INET && daemon.add_subnet4)
    {
      opt.source_netmask = daemon.add_subnet4.mask;
      if (daemon.add_subnet4.addr_used)
	{
	  sa_family = daemon.add_subnet4.addr.sa.sa_family;
	  addrp = get_addrp(&daemon.add_subnet4.addr, sa_family);
	  cacheable = 1; /* Address is constant */
	} 
	else 
	  addrp = &source.in.sin_addr;
    }
  
  opt.family = htons(sa_family == AF_INET6 ? 2 : 1);
  
  if (addrp && opt.source_netmask != 0)
    {
      len = ((opt.source_netmask - 1) >> 3) + 1;
      memcpy(opt.addr, addrp, len);
      if (opt.source_netmask & 7)
	opt.addr[len-1] &= 0xff << (8 - (opt.source_netmask & 7));
    }
  else
    {
      cacheable = 1; /* No address ever supplied. */
      len = 0;
    }

  if (cacheablep)
    *cacheablep = cacheable;
  
  return len + 4;
}
 
 add_source_addr: usize(struct dns_header *header, plen: usize, limit: &mut Vec<u8>, union mysockaddr *source, int *cacheable)
{
  /* http://tools.ietf.org/html/draft-vandergaast-edns-client-subnet-02 */
  
  let mut len: i32;
  struct subnet_opt opt;
  
  len = calc_subnet_opt(&opt, source, cacheable);
  return add_pseudoheader(header, plen, limit, PACKETSZ, EDNS0_OPTION_CLIENT_SUBNET, &opt, len, 0, 0);
}

int check_source(struct dns_header *header, plen: usize, pseudoheader: &mut Vec<u8>, union mysockaddr *peer)
{
  /* Section 9.2, Check that subnet option in reply matches. */
  
  len: i32, calc_len;
  struct subnet_opt opt;
  unsigned char *p;
  code: i32, i, rdlen;
  
  calc_len = calc_subnet_opt(&opt, peer, NULL);
   
  if (!(p = skip_name(pseudoheader, header, plen, 10)))
    return 1;
  
  p += 8; /* skip UDP length and RCODE */
  
  GETSHORT(rdlen, p);
  if (!CHECK_LEN(header, p, plen, rdlen))
    return 1; /* bad packet */
  
  /* check if option there */
   for (i = 0; i + 4 < rdlen; i += len + 4)
     {
       GETSHORT(code, p);
       GETSHORT(len, p);
       if (code == EDNS0_OPTION_CLIENT_SUBNET)
	 {
	   /* make sure this doesn't mismatch. */
	   opt.scope_netmask = p[3];
	   if (len != calc_len || memcmp(p, &opt, len) != 0)
	     return 0;
	 }
       p += len;
     }
   
   return 1;
}

/* Set *check_subnet if we add a client subnet option, which needs to checked 
   in the reply. Set *cacheable to zero if we add an option which the answer
   may depend on. */
add_edns0_config: usize(struct dns_header *header, plen: usize, limit: &mut Vec<u8>, 
			union mysockaddr *source, now: &time::Instant, int *check_subnet, int *cacheable)    
{
  *check_subnet = 0;
  *cacheable = 1;
  
  if (option_bool(OPT_ADD_MAC))
    plen  = add_mac(header, plen, limit, source, now, cacheable);
  
  if (option_bool(OPT_MAC_B64) || option_bool(OPT_MAC_HEX))
    plen = add_dns_client(header, plen, limit, source, now, cacheable);
  
  if (daemon.dns_client_id)
    plen = add_pseudoheader(header, plen, limit, PACKETSZ, EDNS0_OPTION_NOMCPEID, 
			    daemon.dns_client_id, strlen(daemon.dns_client_id), 0, 1);
  
  if (option_bool(OPT_CLIENT_SUBNET))
    {
      plen = add_source_addr(header, plen, limit, source, cacheable); 
      *check_subnet = 1;
    }
	  
  return plen;
}
