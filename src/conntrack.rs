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



 HAVE_CONNTRACK

#include <libnetfilter_conntrack/libnetfilter_conntrack.h>

let mut gotit: i32 = 0; /* yuck */

 callback: i32(enum nf_conntrack_msg_type type, struct nf_conntrack *ct, data: Vec<u8>);

get_incoming_mark: i32(peer_addr: &mut net::IpAddr, union all_addr *local_addr, istcp: i32, markp: &mut u32)
{
  let mut ct: nf_conntrack;
  let mut h: nfct_handle;
  
  gotit = 0;
  
  if ((ct = nfct_new())) 
    {
      nfct_set_attr_u8(ct, ATTR_L4PROTO, istcp ? IPPROTO_TCP : IPPROTO_UDP);
      nfct_set_attr_u16(ct, ATTR_PORT_DST, htons(daemon.port));
      
      if (peer_addr.sa.sa_family == AF_INET6)
	{
	  nfct_set_attr_u8(ct, ATTR_L3PROTO, AF_INET6);
	  nfct_set_attr(ct, ATTR_IPV6_SRC, peer_addr.in6.sin6_addr.s6_addr);
	  nfct_set_attr_u16(ct, ATTR_PORT_SRC, peer_addr.in6.sin6_port);
	  nfct_set_attr(ct, ATTR_IPV6_DST, local_addr.addr6.s6_addr);
	}
      else
	{
	  nfct_set_attr_u8(ct, ATTR_L3PROTO, AF_INET);
	  nfct_set_attr_u32(ct, ATTR_IPV4_SRC, peer_addr.in.sin_addr.s_addr);
	  nfct_set_attr_u16(ct, ATTR_PORT_SRC, peer_addr.in.sin_port);
	  nfct_set_attr_u32(ct, ATTR_IPV4_DST, local_addr.addr4.s_addr);
	}
      
      
      if ((h = nfct_open(CONNTRACK, 0))) 
	{
	  nfct_callback_register(h, NFCT_T_ALL, callback, (void *)markp);  
	  if (nfct_query(h, NFCT_Q_GET, ct) == -1)
	    {
	      let mut warned: i32 = 0;
	      if (!warned)
		{
		  my_syslog(LOG_ERR, format!("Conntrack connection mark retrieval failed: {}"), strerror(errno));
		  warned = 1;
		}
	    }
	  nfct_close(h);  
	}
      nfct_destroy(ct);
    }

  return gotit;
}

 callback: i32(enum nf_conntrack_msg_type type, struct nf_conntrack *ct, data: Vec<u8>)
{
  ret: &mut u32 = (unsigned int *)data;
  *ret = nfct_get_attr_u32(ct, ATTR_MARK);
  ()type; /* eliminate warning */
  gotit = 1;

  return NFCT_CB_CONTINUE;
}


  


