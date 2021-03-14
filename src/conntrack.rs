use libc::c_void;

use crate::{arp::AddressPointer, dnsmasq_h::{Daemon, Mysockaddr}};

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

//#include "dnsmasq.h"

//#ifdef HAVE_CONNTRACK

//#include <libnetfilter_conntrack/libnetfilter_conntrack.h>

// static int gotit = 0; /* yuck */


// static int callback(enum nf_conntrack_msg_type type, struct nf_conntrack *ct, void *data);

pub fn get_incoming_mark(
  daemon: &Daemon,
  peer_addr: &mut Mysockaddr,
  local_addr: &mut AddressPointer,
  istcp: bool,
  markp: *mut c_uint,
  gotit: i32,
  warned: bool) -> i32
{
  // struct nf_conntrack *ct;
  let ct: nf_conntrack;
  // struct nfct_handle *h;
  let h: nfct_handle;

  gotit = 0;
  
  if ((ct = nfct_new())) 
  {
    // nfct_set_attr_u8(ct, ATTR_L4PROTO, istcp ? IPPROTO_TCP : IPPROTO_UDP);
    let mut proto = IPPROTO_TCP;
    if istcp == false {
      proto = IPPROTO_UDP;
    }
    nfct_set_attr_u8(ct, ATTR_L4PROTO, proto);
    nfct_set_attr_u16(ct, ATTR_PORT_DST, htons(daemon.port));
      
//#ifdef HAVE_IPV6
      if (peer_addr.sa.sa_family == AF_INET6)
      {
        nfct_set_attr_u8(ct, ATTR_L3PROTO, AF_INET6);
        nfct_set_attr(ct, ATTR_IPV6_SRC, peer_addr.in6.sin6_addr.s6_addr);
        nfct_set_attr_u16(ct, ATTR_PORT_SRC, peer_addr.in6.sin6_port);
        nfct_set_attr(ct, ATTR_IPV6_DST, local_addr.addr.addr6.s6_addr);
	    }
      else
//#endif
	{
	  nfct_set_attr_u8(ct, ATTR_L3PROTO, AF_INET);
	  nfct_set_attr_u32(ct, ATTR_IPV4_SRC, peer_addr.in4.sin_addr.s_addr);
	  nfct_set_attr_u16(ct, ATTR_PORT_SRC, peer_addr.in4.sin_port);
	  nfct_set_attr_u32(ct, ATTR_IPV4_DST, local_addr.addr.addr4.s_addr);
	}
      
      
      if ((h = nfct_open(CONNTRACK, 0))) 
	{
	  nfct_callback_register(h, NFCT_T_ALL, callback, markp);  
	  if (nfct_query(h, NFCT_Q_GET, ct) == -1)
	    {
        if warned == false
		{
		  // my_syslog(LOG_ERR, _("Conntrack connection mark retrieval failed: %s"), strerror(errno));
		  warned = true;
		}
	    }
	  nfct_close(h);  
	}
      nfct_destroy(ct);
    }

  return gotit;
}

pub fn callback(
  msg_type: nf_conntrack_msg_type,
  ct: &mut nf_conntrack,
  data: &mut u32,
  gotit: &mut i32) -> i32
{

  // unsigned int *ret = (unsigned int *)data;
  ret = nfct_get_attr_u32(ct, ATTR_MARK);
  // (void)type; /* eliminate warning */
  gotit = 1;
  return NFCT_CB_CONTINUE;
}

//#endif
  


