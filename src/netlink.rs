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

/* Blergh. Radv does this, so that's our excuse. */
pub const SOL_NETLINK: u32 = 270;

pub const NETLINK_NO_ENOBUFS: u32 = 5;


/* linux 2.6.19 buggers up the headers, patch it up here. */ 

//  let mut iov: iovec;
//  u32 netlink_pid;
// pub fn nl_async(struct nlmsghdr *h);

pub fn netlink_init() -> String
{
//   struct sockaddr_nl addr;
	let addr: sockaddr_nl;
// socklen_t slen = sizeof(addr);
	let t_slen: socklen_t = mem::sizeof::<addr>();
	let mut opt: i32 = 1;

  addr.nl_family = AF_NETLINK;
  addr.nl_pad = 0;
  addr.nl_pid = 0; /* autobind */
  addr.nl_groups = RTMGRP_IPV4_ROUTE;
  if (daemon.opt_cleverbind) {
    addr.nl_groups |= RTMGRP_IPV4_IFADDR;  
  }
  addr.nl_groups |= RTMGRP_IPV6_ROUTE;
  if (daemon.opt_cleverbind)
    addr.nl_groups |= RTMGRP_IPV6_IFADDR;

 
  if (daemon.doing_ra || daemon.doing_dhcp6)
    addr.nl_groups |= RTMGRP_IPV6_IFADDR;

  
  /* May not be able to have permission to set multicast groups don't die in that case */
  if ((daemon.netlinkfd = socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE)) != -1)
    {
      if (bind(daemon.netlinkfd, &addr, sizeof(addr)) == -1)
	{
	  addr.nl_groups = 0;
	  if (errno != EPERM || bind(daemon.netlinkfd, &addr, sizeof(addr)) == -1)
	    daemon.netlinkfd = -1;
	}
    }
  
  if (daemon.netlinkfd == -1 || 
      getsockname(daemon.netlinkfd, &addr, &slen) == -1)
    die(format!("cannot create netlink socket: {}"), NULL, EC_MISC);
  
  
  /* save pid assigned by bind() and retrieved by getsockname() */ 
  netlink_pid = addr.nl_pid;
  
  iov.iov_len = 100;
  iov.iov_base = safe_malloc(iov.iov_len);
  
  if (daemon.kernel_version >= KERNEL_VERSION(2,6,30) &&
      setsockopt(daemon.netlinkfd, SOL_NETLINK, NETLINK_NO_ENOBUFS, &opt, sizeof(opt)) == -1)
    return format!("warning: failed to set NETLINK_NO_ENOBUFS on netlink socket");
  
  return NULL;
}

 snetlink_recv: usize()
{
  let mut msg: msghdr;
  struct sockaddr_nl nladdr;
  src: usize;

  while (1)
    {
      msg.msg_control = NULL;
      msg.msg_controllen = 0;
      msg.msg_name = &nladdr;
      msg.msg_namelen = sizeof(nladdr);
      msg.msg_iov = &iov;
      msg.msg_iovlen = 1;
      msg.msg_flags = 0;
      
      while ((rc = recvmsg(daemon.netlinkfd, &msg, MSG_PEEK | MSG_TRUNC)) == -1 && errno == EINTR);
      
      /* make buffer big enough */
      if (rc != -1 && (msg.msg_flags & MSG_TRUNC))
	{
	  /* Very new Linux kernels return the actual size needed, older ones always return truncated size */
	  if ((size_t)rc == iov.iov_len)
	    {
	      if (expand_buf(&iov, rc + 100))
		continue;
	    }
	  else
	    expand_buf(&iov, rc);
	}

      /* read it for real */
      msg.msg_flags = 0;
      while ((rc = recvmsg(daemon.netlinkfd, &msg, 0)) == -1 && errno == EINTR);
      
      /* Make sure this is from the kernel */
      if (rc == -1 || nladdr.nl_pid == 0)
	break;
    }
      
  /* discard stuff which is truncated at this point (expand_buf() may fail) */
  if (msg.msg_flags & MSG_TRUNC)
    {
      rc = -1;
      errno = ENOMEM;
    }
  
  return rc;
}
  

/* family = AF_UNSPEC finds ARP table entries.
   family = AF_LOCAL finds MAC addresses. */
int iface_enumerate(family: i32, parm: Vec<u8>, int (*callback)())
{
  struct sockaddr_nl addr;
  let mut h: nlmsghdr;
  slen: usize;
   unsigned int seq = 0;
  let mut callback_ok: i32 = 1;

  struct {
    struct nlmsghdr nlh;
    struct rtgenmsg g; 
  } req;

  memset(&req, 0, sizeof(req));
  memset(&addr, 0, sizeof(addr));

  addr.nl_family = AF_NETLINK;
 
 again: 
  if (family == AF_UNSPEC)
    req.nlh.nlmsg_type = RTM_GETNEIGH;
  else if (family == AF_LOCAL)
    req.nlh.nlmsg_type = RTM_GETLINK;
  else
    req.nlh.nlmsg_type = RTM_GETADDR;

  req.nlh.nlmsg_len = sizeof(req);
  req.nlh.nlmsg_flags = NLM_F_ROOT | NLM_F_MATCH | NLM_F_REQUEST | NLM_F_ACK; 
  req.nlh.nlmsg_pid = 0;
  req.nlh.nlmsg_seq = ++seq;
  req.g.rtgen_family = family; 

  /* Don't block in recvfrom if send fails */
  while(retry_send(sendto(daemon.netlinkfd, (void *)&req, sizeof(req), 0, 
			  &addr, sizeof(addr))));

  if (errno != 0)
    return 0;
    
  while (1)
    {
      if ((len = netlink_recv()) == -1)
	{
	  if (errno == ENOBUFS)
	    {
	      sleep(1);
	      goto again;
	    }
	  return 0;
	}

      for (h = (struct nlmsghdr *)iov.iov_base; NLMSG_OK(h, (size_t)len); h = NLMSG_NEXT(h, len))
	if (h.nlmsg_pid != netlink_pid || h.nlmsg_type == NLMSG_ERROR)
	  {
	    /* May be multicast arriving async */
	    nl_async(h);
	  }
	else if (h.nlmsg_seq != seq)
	  {
	    /* May be part of incomplete response to previous request after
	       ENOBUFS. Drop it. */
	    continue;
	  }
	else if (h.nlmsg_type == NLMSG_DONE)
	  return callback_ok;
	else if (h.nlmsg_type == RTM_NEWADDR && family != AF_UNSPEC && family != AF_LOCAL)
	  {
	    struct ifaddrmsg *ifa = NLMSG_DATA(h);  
	    struct rtattr *rta = IFA_RTA(ifa);
	    unsigned int len1 = h.nlmsg_len - NLMSG_LENGTH(sizeof(*ifa));
	    
	    if (ifa.ifa_family == family)
	      {
		if (ifa.ifa_family == AF_INET)
		  {
		    netmask: net::IpAddr, addr, broadcast;
		    char *label = NULL;

		    netmask.s_addr = htonl(~(in_addr_t)0 << (32 - ifa.ifa_prefixlen));

		    addr.s_addr = 0;
		    broadcast.s_addr = 0;
		    
		    while (RTA_OK(rta, len1))
		      {
			if (rta.rta_type == IFA_LOCAL)
			  addr = *((rta+1));
			else if (rta.rta_type == IFA_BROADCAST)
			  broadcast = *((rta+1));
			else if (rta.rta_type == IFA_LABEL)
			  label = RTA_DATA(rta);
			
			rta = RTA_NEXT(rta, len1);
		      }
		    
		    if (addr.s_addr && callback_ok)
		      if (!((*callback)(addr, ifa.ifa_index, label,  netmask, broadcast, parm)))
			callback_ok = 0;
		  }
		else if (ifa.ifa_family == AF_INET6)
		  {
		    addrp: &mut net::IpAddr = NULL;
		    u32 valid = 0, preferred = 0;
		    let mut flags: i32 = 0;
		    
		    while (RTA_OK(rta, len1))
		      {
			if (rta.rta_type == IFA_ADDRESS)
			  addrp = ((struct in6_addr *)(rta+1)); 
			else if (rta.rta_type == IFA_CACHEINFO)
			  {
			    struct ifa_cacheinfo *ifc = (struct ifa_cacheinfo *)(rta+1);
			    preferred = ifc.ifa_prefered;
			    valid = ifc.ifa_valid;
			  }
			rta = RTA_NEXT(rta, len1);
		      }
		    
		    if (ifa.ifa_flags & IFA_F_TENTATIVE)
		      flags |= IFACE_TENTATIVE;
		    
		    if (ifa.ifa_flags & IFA_F_DEPRECATED)
		      flags |= IFACE_DEPRECATED;
		    
		    if (!(ifa.ifa_flags & IFA_F_TEMPORARY))
		      flags |= IFACE_PERMANENT;
    		    
		    if (addrp && callback_ok)
		      if (!((*callback)(addrp, (ifa.ifa_prefixlen), (ifa.ifa_scope), 
					(ifa.ifa_index), flags, 
					 preferred, valid, parm)))
			callback_ok = 0;
		  }
	      }
	  }
	else if (h.nlmsg_type == RTM_NEWNEIGH && family == AF_UNSPEC)
	  {
	    struct ndmsg *neigh = NLMSG_DATA(h);  
	    struct rtattr *rta = NDA_RTA(neigh);
	    unsigned int len1 = h.nlmsg_len - NLMSG_LENGTH(sizeof(*neigh));
	    maclen: usize = 0;
	    char *inaddr = NULL, *mac = NULL;
	    
	    while (RTA_OK(rta, len1))
	      {
		if (rta.rta_type == NDA_DST)
		  inaddr = (rta+1);
		else if (rta.rta_type == NDA_LLADDR)
		  {
		    maclen = rta.rta_len - sizeof(struct rtattr);
		    mac = (rta+1);
		  }
		
		rta = RTA_NEXT(rta, len1);
	      }

	    if (!(neigh.ndm_state & (NUD_NOARP | NUD_INCOMPLETE | NUD_FAILED)) &&
		inaddr && mac && callback_ok)
	      if (!((*callback)(neigh.ndm_family, inaddr, mac, maclen, parm)))
		callback_ok = 0;
	  }
 
	else if (h.nlmsg_type == RTM_NEWLINK && family == AF_LOCAL)
	  {
	    struct ifinfomsg *link =  NLMSG_DATA(h);
	    struct rtattr *rta = IFLA_RTA(link);
	    unsigned int len1 = h.nlmsg_len - NLMSG_LENGTH(sizeof(*link));
	    char *mac = NULL;
	    maclen: usize = 0;

	    while (RTA_OK(rta, len1))
	      {
		if (rta.rta_type == IFLA_ADDRESS)
		  {
		    maclen = rta.rta_len - sizeof(struct rtattr);
		    mac = (rta+1);
		  }
		
		rta = RTA_NEXT(rta, len1);
	      }

	    if (mac && callback_ok && !((link.ifi_flags & (IFF_LOOPBACK | IFF_POINTOPOINT))) && 
		!((*callback)(link.ifi_index, (unsigned int)link.ifi_type, mac, maclen, parm)))
	      callback_ok = 0;
	  }

    }
}

void netlink_multicast()
{
  slen: usize;
  let mut h: nlmsghdr;
  let mut flags: i32;
  
  /* don't risk blocking reading netlink messages here. */
  if ((flags = fcntl(daemon.netlinkfd, F_GETFL)) == -1 ||
      fcntl(daemon.netlinkfd, F_SETFL, flags | O_NONBLOCK) == -1) 
    return;
  
  if ((len = netlink_recv()) != -1)
    for (h = (struct nlmsghdr *)iov.iov_base; NLMSG_OK(h, (size_t)len); h = NLMSG_NEXT(h, len))
      nl_async(h);
  
  /* restore non-blocking status */
  fcntl(daemon.netlinkfd, F_SETFL, flags);
}

pub fn nl_async(struct nlmsghdr *h)
{
  if (h.nlmsg_type == NLMSG_ERROR)
    {
      struct nlmsgerr *err = NLMSG_DATA(h);
      if (err.error != 0)
	my_syslog(LOG_ERR, format!("netlink returns error: {}"), strerror(-(err.error)));
    }
  else if (h.nlmsg_pid == 0 && h.nlmsg_type == RTM_NEWROUTE) 
    {
      /* We arrange to receive netlink multicast messages whenever the network route is added.
	 If this happens and we still have a DNS packet in the buffer, we re-send it.
	 This helps on DoD links, where frequently the packet which triggers dialling is
	 a DNS query, which then gets lost. By re-sending, we can avoid the lookup
	 failing. */ 
      struct rtmsg *rtm = NLMSG_DATA(h);
      
      if (rtm.rtm_type == RTN_UNICAST && rtm.rtm_scope == RT_SCOPE_LINK &&
	  (rtm.rtm_table == RT_TABLE_MAIN ||
	   rtm.rtm_table == RT_TABLE_LOCAL))
	queue_event(EVENT_NEWROUTE);
    }
  else if (h.nlmsg_type == RTM_NEWADDR || h.nlmsg_type == RTM_DELADDR) 
    queue_event(EVENT_NEWADDR);
}


      
