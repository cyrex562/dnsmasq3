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





indextoname: i32(fd: i32, index: i32, name: &mut String)
{
  let mut ifr: ifreq;
  
  if (index == 0)
    return 0;

  ifr.ifr_ifindex = index;
  if (ioctl(fd, SIOCGIFNAME, &ifr) == -1)
    return 0;

  safe_strncpy(name, ifr.ifr_name, IF_NAMESIZE);

  return 1;
}


#elif defined(HAVE_SOLARIS_NETWORK)

#include <zone.h>
#include <alloca.h>
#ifndef LIFC_UNDER_IPMP
pub const LIFC_UNDER_IPMP: u32 = 0;


indextoname: i32(fd: i32, index: i32, name: &mut String)
{
  int64_t lifc_flags;
  struct lifnum lifn;
  numifs: i32, bufsize, i;
  struct lifconf lifc;
  let mut lifrp: lifreq;
  
  if (index == 0)
    return 0;
  
  if (getzoneid() == GLOBAL_ZONEID) 
    {
      if (!if_indextoname(index, name))
	return 0;
      return 1;
    }
  
  lifc_flags = LIFC_NOXMIT | LIFC_TEMPORARY | LIFC_ALLZONES | LIFC_UNDER_IPMP;
  lifn.lifn_family = AF_UNSPEC;
  lifn.lifn_flags = lifc_flags;
  if (ioctl(fd, SIOCGLIFNUM, &lifn) < 0) 
    return 0;
  
  numifs = lifn.lifn_count;
  bufsize = numifs * sizeof(struct lifreq);
  
  lifc.lifc_family = AF_UNSPEC;
  lifc.lifc_flags = lifc_flags;
  lifc.lifc_len = bufsize;
  lifc.lifc_buf = alloca(bufsize);
  
  if (ioctl(fd, SIOCGLIFCONF, &lifc) < 0)  
    return 0;
  
  lifrp = lifc.lifc_req;
  for (i = lifc.lifc_len / sizeof(struct lifreq); i; i--, lifrp++) 
    {
      struct lifreq lifr;
      safe_strncpy(lifr.lifr_name, lifrp.lifr_name, IF_NAMESIZE);
      if (ioctl(fd, SIOCGLIFINDEX, &lifr) < 0) 
	return 0;
      
      if (lifr.lifr_index == index) {
	safe_strncpy(name, lifr.lifr_name, IF_NAMESIZE);
	return 1;
      }
    }
  return 0;
}




indextoname: i32(fd: i32, index: i32, name: &mut String)
{ 
  ()fd;

  if (index == 0 || !if_indextoname(index, name))
    return 0;

  return 1;
}



iface_check: i32(family: i32, union all_addr *addr, name: &mut String, auth: &i32)
{
  let mut tmp: iname;
  ret: i32 = 1, match_addr = 0;

  /* Note: have to check all and not bail out early, so that we set the
     "used" flags.

     May be called with family == AF_LOCALto check interface by name only. */
  
  if (auth)
    *auth = 0;
  
  if (daemon.if_names || daemon.if_addrs)
    {
      ret = 0;

      for (tmp = daemon.if_names; tmp; tmp = tmp.next)
	if (tmp.name && wildcard_match(tmp.name, name))
	  ret = tmp.used = 1;
	        
      if (addr)
	for (tmp = daemon.if_addrs; tmp; tmp = tmp.next)
	  if (tmp.addr.sa.sa_family == family)
	    {
	      if (family == AF_INET &&
		  tmp.addr.in.sin_addr.s_addr == addr.addr4.s_addr)
		ret = match_addr = tmp.used = 1;
	      else if (family == AF_INET6 &&
		       IN6_ARE_ADDR_EQUAL(&tmp.addr.in6.sin6_addr, 
					  &addr.addr6))
		ret = match_addr = tmp.used = 1;
	    }          
    }
  
  if (!match_addr)
    for (tmp = daemon.if_except; tmp; tmp = tmp.next)
      if (tmp.name && wildcard_match(tmp.name, name))
	ret = 0;
    

  for (tmp = daemon.authinterface; tmp; tmp = tmp.next)
    if (tmp.name)
      {
	if (strcmp(tmp.name, name) == 0 &&
	    (tmp.addr.sa.sa_family == 0 || tmp.addr.sa.sa_family == family))
	  break;
      }
    else if (addr && tmp.addr.sa.sa_family == AF_INET && family == AF_INET &&
	     tmp.addr.in.sin_addr.s_addr == addr.addr4.s_addr)
      break;
    else if (addr && tmp.addr.sa.sa_family == AF_INET6 && family == AF_INET6 &&
	     IN6_ARE_ADDR_EQUAL(&tmp.addr.in6.sin6_addr, &addr.addr6))
      break;

  if (tmp && auth) 
    {
      *auth = 1;
      ret = 1;
    }

  return ret; 
}


/* Fix for problem that the kernel sometimes reports the loopback interface as the
   arrival interface when a packet originates locally, even when sent to address of 
   an interface other than the loopback. Accept packet if it arrived via a loopback 
   interface, even when we're not accepting packets that way, as long as the destination
   address is one we're believing. Interface list must be up-to-date before calling. */
loopback_exception: i32(fd: i32, family: i32, union all_addr *addr, name: &mut String)    
{
  let mut ifr: ifreq;
  let mut iface: irec;

  safe_strncpy(ifr.ifr_name, name, IF_NAMESIZE);
  if (ioctl(fd, SIOCGIFFLAGS, &ifr) != -1 &&
      ifr.ifr_flags & IFF_LOOPBACK)
    {
      for (iface = daemon.interfaces; iface; iface = iface.next)
	if (iface.addr.sa.sa_family == family)
	  {
	    if (family == AF_INET)
	      {
		if (iface.addr.in.sin_addr.s_addr == addr.addr4.s_addr)
		  return 1;
	      }
	    else if (IN6_ARE_ADDR_EQUAL(&iface.addr.in6.sin6_addr, &addr.addr6))
	      return 1;
	  }
    }
  return 0;
}

/* If we're configured with something like --interface=eth0:0 then we'll listen correctly
   on the relevant address, but the name of the arrival interface, derived from the
   index won't match the config. Check that we found an interface address for the arrival 
   interface: daemon.interfaces must be up-to-date. */
label_exception: i32(index: i32, family: i32, union all_addr *addr)
{
  let mut iface: irec;

  /* labels only supported on IPv4 addresses. */
  if (family != AF_INET)
    return 0;

  for (iface = daemon.interfaces; iface; iface = iface.next)
    if (iface.index == index && iface.addr.sa.sa_family == AF_INET &&
	iface.addr.in.sin_addr.s_addr == addr.addr4.s_addr)
      return 1;

  return 0;
}

struct iface_param {
  let mut spare: addrlist;
  let mut fd: i32;
};

 iface_allowed: i32(struct iface_param *param, if_index: i32, label: &mut String,
			 addr: &mut net::IpAddr, netmask: net::IpAddr, prefixlen: i32, iface_flags: i32) 
{
  let mut iface: irec;
  mtu: i32 = 0, loopback;
  let mut ifr: ifreq;
  tftp_ok: i32 = !!daemon.opt_tftp;
  let mut dhcp_ok: i32 = 1;
  let mut auth_dns: i32 = 0;
  let mut is_label: i32 = 0;
#if defined() || defined()
  let mut tmp: iname;


  ()prefixlen;

  if (!indextoname(param.fd, if_index, ifr.ifr_name) ||
      ioctl(param.fd, SIOCGIFFLAGS, &ifr) == -1)
    return 0;
   
  loopback = ifr.ifr_flags & IFF_LOOPBACK;
  
  if (loopback)
    dhcp_ok = 0;
  
  if (ioctl(param.fd, SIOCGIFMTU, &ifr) != -1)
    mtu = ifr.ifr_mtu;
  
  if (!label)
    label = ifr.ifr_name;
  else
    is_label = strcmp(label, ifr.ifr_name);
 
  /* maintain a list of all addresses on all interfaces for --local-service option */
  if (option_bool(OPT_LOCAL_SERVICE))
    {
      let mut al: addrlist;

      if (param.spare)
	{
	  al = param.spare;
	  param.spare = al.next;
	}
      else
	al = whine_malloc(sizeof(struct addrlist));
      
      if (al)
	{
	  al.next = daemon.interface_addrs;
	  daemon.interface_addrs = al;
	  al.prefixlen = prefixlen;
	  
	  if (addr.sa.sa_family == AF_INET)
	    {
	      al.addr.addr4 = addr.in.sin_addr;
	      al.flags = 0;
	    }
	  else
	    {
	      al.addr.addr6 = addr.in6.sin6_addr;
	      al.flags = ADDRLIST_IPV6;
	    } 
	}
    }
  
  if (addr.sa.sa_family != AF_INET6 || !IN6_IS_ADDR_LINKLOCAL(&addr.in6.sin6_addr))
    {
      let mut int_name: interface_name;
      let mut al: addrlist;

      let mut zone: auth_zone;
      let mut name: auth_name_list;

      /* Find subnets in auth_zones */
      for (zone = daemon.auth_zones; zone; zone = zone.next)
	for (name = zone.interface_names; name; name = name.next)
	  if (wildcard_match(name.name, label))
	    {
	      if (addr.sa.sa_family == AF_INET && (name.flags & AUTH4))
		{
		  if (param.spare)
		    {
		      al = param.spare;
		      param.spare = al.next;
		    }
		  else
		    al = whine_malloc(sizeof(struct addrlist));
		  
		  if (al)
		    {
		      al.next = zone.subnet;
		      zone.subnet = al;
		      al.prefixlen = prefixlen;
		      al.addr.addr4 = addr.in.sin_addr;
		      al.flags = 0;
		    }
		}
	      
	      if (addr.sa.sa_family == AF_INET6 && (name.flags & AUTH6))
		{
		  if (param.spare)
		    {
		      al = param.spare;
		      param.spare = al.next;
		    }
		  else
		    al = whine_malloc(sizeof(struct addrlist));
		  
		  if (al)
		    {
		      al.next = zone.subnet;
		      zone.subnet = al;
		      al.prefixlen = prefixlen;
		      al.addr.addr6 = addr.in6.sin6_addr;
		      al.flags = ADDRLIST_IPV6;
		    }
		} 
	    }

       
      /* Update addresses from interface_names. These are a set independent
	 of the set we're listening on. */  
      for (int_name = daemon.int_names; int_name; int_name = int_name.next)
	if (strncmp(label, int_name.intr, IF_NAMESIZE) == 0 && 
	    (addr.sa.sa_family == int_name.family || int_name.family == 0))
	  {
	    if (param.spare)
	      {
		al = param.spare;
		param.spare = al.next;
	      }
	    else
	      al = whine_malloc(sizeof(struct addrlist));
	    
	    if (al)
	      {
		al.next = int_name.addr;
		int_name.addr = al;
		
		if (addr.sa.sa_family == AF_INET)
		  {
		    al.addr.addr4 = addr.in.sin_addr;
		    al.flags = 0;
		  }
		else
		 {
		    al.addr.addr6 = addr.in6.sin6_addr;
		    al.flags = ADDRLIST_IPV6;
		    /* Privacy addresses and addresses still undergoing DAD and deprecated addresses
		       don't appear in forward queries, but will in reverse ones. */
		    if (!(iface_flags & IFACE_PERMANENT) || (iface_flags & (IFACE_DEPRECATED | IFACE_TENTATIVE)))
		      al.flags |= ADDRLIST_REVONLY;
		 } 
	      }
	  }
    }
 
  /* check whether the interface IP has been added already 
     we call this routine multiple times. */
  for (iface = daemon.interfaces; iface; iface = iface.next) 
    if (sockaddr_isequal(&iface.addr, addr) && iface.index == if_index)
      {
	iface.dad = !!(iface_flags & IFACE_TENTATIVE);
	iface.found = 1; /* for garbage collection */
	iface.netmask = netmask;
	return 1;
      }

 /* If we are restricting the set of interfaces to use, make
     sure that loopback interfaces are in that set. */
  if (daemon.if_names && loopback)
    {
      let mut lo: iname;
      for (lo = daemon.if_names; lo; lo = lo.next)
	if (lo.name && strcmp(lo.name, ifr.ifr_name) == 0)
	  break;
      
      if (!lo && (lo = whine_malloc(sizeof(struct iname)))) 
	{
	  if ((lo.name = whine_malloc(strlen(ifr.ifr_name)+1)))
	    {
	      strcpy(lo.name, ifr.ifr_name);
	      lo.used = 1;
	      lo.next = daemon.if_names;
	      daemon.if_names = lo;
	    }
	  else
	    free(lo);
	}
    }
  
  if (addr.sa.sa_family == AF_INET &&
      !iface_check(AF_INET, (union all_addr *)&addr.in.sin_addr, label, &auth_dns))
    return 1;

  if (addr.sa.sa_family == AF_INET6 &&
      !iface_check(AF_INET6, (union all_addr *)&addr.in6.sin6_addr, label, &auth_dns))
    return 1;
    
 
  /* No DHCP where we're doing auth DNS. */
  if (auth_dns)
    {
      tftp_ok = 0;
      dhcp_ok = 0;
    }
  else
    for (tmp = daemon.dhcp_except; tmp; tmp = tmp.next)
      if (tmp.name && wildcard_match(tmp.name, ifr.ifr_name))
	{
	  tftp_ok = 0;
	  dhcp_ok = 0;
	}

 
  
 
  if (daemon.tftp_interfaces)
    {
      /* dedicated tftp interface list */
      tftp_ok = 0;
      for (tmp = daemon.tftp_interfaces; tmp; tmp = tmp.next)
	if (tmp.name && wildcard_match(tmp.name, ifr.ifr_name))
	  tftp_ok = 1;
    }

  
  /* add to list */
  if ((iface = whine_malloc(sizeof(struct irec))))
    {
      iface.addr = *addr;
      iface.netmask = netmask;
      iface.tftp_ok = tftp_ok;
      iface.dhcp_ok = dhcp_ok;
      iface.dns_auth = auth_dns;
      iface.mtu = mtu;
      iface.dad = !!(iface_flags & IFACE_TENTATIVE);
      iface.found = 1;
      iface.done = iface.multicast_done = iface.warned = 0;
      iface.index = if_index;
      iface.label = is_label;
      if ((iface.name = whine_malloc(strlen(ifr.ifr_name)+1)))
	{
	  strcpy(iface.name, ifr.ifr_name);
	  iface.next = daemon.interfaces;
	  daemon.interfaces = iface;
	  return 1;
	}
      free(iface);

    }
  
  errno = ENOMEM; 
  return 0;
}

 iface_allowed_v6: i32(local: &mut net::IpAddr, prefix: i32, 
			    scope: i32, if_index: i32, flags: i32, 
			    preferred: i32, valid: i32, vparam: Vec<u8>)
{
  union mysockaddr addr;
  netmask: net::IpAddr; /* dummy */
  netmask.s_addr = 0;

  ()scope; /* warning */
  ()preferred;
  ()valid;
  
  memset(&addr, 0, sizeof(addr));
 HAVE_SOCKADDR_SA_LEN
  addr.in6.sin6_len = sizeof(addr.in6);

  addr.in6.sin6_family = AF_INET6;
  addr.in6.sin6_addr = *local;
  addr.in6.sin6_port = htons(daemon.port);
  /* FreeBSD insists this is zero for non-linklocal addresses */
  if (IN6_IS_ADDR_LINKLOCAL(local))
    addr.in6.sin6_scope_id = if_index;
  else
    addr.in6.sin6_scope_id = 0;
  
  return iface_allowed((struct iface_param *)vparam, if_index, NULL, &addr, netmask, prefix, flags);
}

 iface_allowed_v4: i32(local: net::IpAddr, if_index: i32, label: &mut String,
			    netmask: net::IpAddr, broadcast: net::IpAddr, vparam: Vec<u8>)
{
  union mysockaddr addr;
  prefix: i32, bit;
 
  ()broadcast; /* warning */

  memset(&addr, 0, sizeof(addr));
 HAVE_SOCKADDR_SA_LEN
  addr.in.sin_len = sizeof(addr.in);

  addr.in.sin_family = AF_INET;
  addr.in.sin_addr = local;
  addr.in.sin_port = htons(daemon.port);

  /* determine prefix length from netmask */
  for (prefix = 32, bit = 1; (bit & ntohl(netmask.s_addr)) == 0 && prefix != 0; bit = bit << 1, prefix--);

  return iface_allowed((struct iface_param *)vparam, if_index, label, &addr, netmask, prefix, 0);
}

/*
 * Clean old interfaces no longer found.
 */
pub fn clean_interfaces()
{
  let mut iface: irec;
  struct irec **up = &daemon.interfaces;

  for (iface = *up; iface; iface = *up)
  {
    if (!iface.found && !iface.done)
      {
        *up = iface.next;
        free(iface.name);
        free(iface);
      }
    else
      {
        up = &iface.next;
      }
  }
}

/** Release listener if no other interface needs it.
 *
 * @return 1 if released, 0 if still required
 */
 release_listener: i32(struct listener *l)
{
  if (l.used > 1)
    {
      let mut iface: irec;
      for (iface = daemon.interfaces; iface; iface = iface.next)
	if (iface.done && sockaddr_isequal(&l.addr, &iface.addr))
	  {
	    if (iface.found)
	      {
		/* update listener to poto: i32 active interface instead */
		if (!l.iface.found)
		  l.iface = iface;
	      }
	    else
	      {
		l.used--;
		iface.done = 0;
	      }
	  }

      /* Someone is still using this listener, skip its deletion */
      if (l.used > 0)
	return 0;
    }

  if (l.iface.done)
    {
      let mut port: i32;

      port = prettyprint_addr(&l.iface.addr, daemon.addrbuff);
      my_syslog(LOG_DEBUG, format!("stopped listening on {}(#{}): {} port {}"),
		l.iface.name, l.iface.index, daemon.addrbuff, port);
      /* In case it ever returns */
      l.iface.done = 0;
    }

  if (l.fd != -1)
    close(l.fd);
  if (l.tcpfd != -1)
    close(l.tcpfd);
  if (l.tftpfd != -1)
    close(l.tftpfd);

  free(l);
  return 1;
}

enumerate_interfaces: i32(reset: i32)
{
   struct addrlist *spare = NULL;
  let mut done: i32 = 0;
  let mut parm: iface_param;
  errsave: i32, ret = 1;
  struct addrlist *addr, *tmp;
  let mut intname: interface_name;
  let mut iface: irec;

  let mut zone: auth_zone;


  /* Do this max once per select cycle  - also inhibits netlink socket use
   in TCP child processes. */

  if (reset)
    {
      done = 0;
      return 1;
    }

  if (done)
    return 1;

  done = 1;

  if ((param.fd = socket(PF_INET, SOCK_DGRAM, 0)) == -1)
    return 0;
 
  /* Mark interfaces for garbage collection */
  for (iface = daemon.interfaces; iface; iface = iface.next) 
    iface.found = 0;

  /* remove addresses stored against interface_names */
  for (intname = daemon.int_names; intname; intname = intname.next)
    {
      for (addr = intname.addr; addr; addr = tmp)
	{
	  tmp = addr.next;
	  addr.next = spare;
	  spare = addr;
	}
      
      intname.addr = NULL;
    }

  /* Remove list of addresses of local interfaces */
  for (addr = daemon.interface_addrs; addr; addr = tmp)
    {
      tmp = addr.next;
      addr.next = spare;
      spare = addr;
    }
  daemon.interface_addrs = NULL;
  

  /* remove addresses stored against auth_zone subnets, but not 
   ones configured as address literals */
  for (zone = daemon.auth_zones; zone; zone = zone.next)
    if (zone.interface_names)
      {
	struct addrlist **up;
	for (up = &zone.subnet, addr = zone.subnet; addr; addr = tmp)
	  {
	    tmp = addr.next;
	    if (addr.flags & ADDRLIST_LITERAL)
	      up = &addr.next;
	    else
	      {
		*up = addr.next;
		addr.next = spare;
		spare = addr;
	      }
	  }
      }


  param.spare = spare;
  
  ret = iface_enumerate(AF_INET6, &param, iface_allowed_v6);

  if (ret)
    ret = iface_enumerate(AF_INET, &param, iface_allowed_v4); 
 
  errsave = errno;
  close(param.fd);
  
  if (daemon.opt_cleverbind)
    { 
      /* Garbage-collect listeners listening on addresses that no longer exist.
	 Does nothing when not binding interfaces or for listeners on localhost, 
	 since the .iface field is NULL. Note that this needs the protections
	 against reentrancy, hence it's here.  It also means there's a possibility,
	 in OPT_CLEVERBIND mode, that at listener will just disappear after
	 a call to enumerate_interfaces, this is checked OK on all calls. */
      struct listener *l, *tmp, **up;
      let mut freed: i32 = 0;
      
      for (up = &daemon.listeners, l = daemon.listeners; l; l = tmp)
	{
	  tmp = l.next;
	  
	  if (!l.iface || l.iface.found)
	    up = &l.next;
	  else if (release_listener(l))
	    {
	      *up = tmp;
	      freed = 1;
	    }
	}

      if (freed)
	clean_interfaces();
    }

  errno = errsave;
  spare = param.spare;
    
  return ret;
}

/* set NONBLOCK bit on fd: See Stevens 16.6 */
fix_fd: i32(fd: i32)
{
  let mut flags: i32;

  if ((flags = fcntl(fd, F_GETFL)) == -1 ||
      fcntl(fd, F_SETFL, flags | O_NONBLOCK) == -1)
    return 0;
  
  return 1;
}

 make_sock: i32(addr: &mut net::IpAddr, type: i32, dienow: i32)
{
  family: i32 = addr.sa.sa_family;
  fd: i32, rc, opt = 1;
  
  if ((fd = socket(family, type, 0)) == -1)
    {
      port: i32, errsave;
      char *s;

      /* No error if the kernel just doesn't support this IP flavour */
      if (errno == EPROTONOSUPPORT ||
	  errno == EAFNOSUPPORT ||
	  errno == EINVAL)
	return -1;
      
    err:
      errsave = errno;
      port = prettyprint_addr(addr, daemon.addrbuff);
      if (!daemon.opt_nowild && !daemon.opt_cleverbind)
	sprintf(daemon.addrbuff, "port {}", port);
      s = format!("failed to create listening socket for {}: {}");
      
      if (fd != -1)
	close (fd);
	
      errno = errsave;

      if (dienow)
	{
	  /* failure to bind addresses given by --listen-address at this point
	     is OK if we're doing bind-dynamic */
	  if (!daemon.opt_cleverbind)
	    die(s, daemon.addrbuff, EC_BADNET);
	}
      else
	my_syslog(LOG_WARNING, s, daemon.addrbuff, strerror(errno));
      
      return -1;
    }	
  
  if (setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt)) == -1 || !fix_fd(fd))
    goto err;
  
  if (family == AF_INET6 && setsockopt(fd, IPPROTO_IPV6, IPV6_V6ONLY, &opt, sizeof(opt)) == -1)
    goto err;
  
  if ((rc = bind(fd, addr, sa_len(addr))) == -1)
    goto err;
  
  if (type == SOCK_STREAM)
    {
 TCP_FASTOPEN
      let mut qlen: i32 = 5;                           
      setsockopt(fd, IPPROTO_TCP, TCP_FASTOPEN, &qlen, sizeof(qlen));

      
      if (listen(fd, TCP_BACKLOG) == -1)
	goto err;
    }
  else if (family == AF_INET)
    {
      if (!daemon.opt_nowild)
	{
#if defined(HAVE_LINUX_NETWORK) 
	  if (setsockopt(fd, IPPROTO_IP, IP_PKTINFO, &opt, sizeof(opt)) == -1)
	    goto err;
#elif defined(IP_RECVDSTADDR) && defined(IP_RECVIF)
	  if (setsockopt(fd, IPPROTO_IP, IP_RECVDSTADDR, &opt, sizeof(opt)) == -1 ||
	      setsockopt(fd, IPPROTO_IP, IP_RECVIF, &opt, sizeof(opt)) == -1)
	    goto err;

	}
    }
  else if (!set_ipv6pktinfo(fd))
    goto err;
  
  return fd;
}

set_ipv6pktinfo: i32(fd: i32)
{
  let mut opt: i32 = 1;

  /* The API changed around Linux 2.6.14 but the old ABI is still supported:
     handle all combinations of headers and kernel.
     OpenWrt note that this fixes the problem addressed by your very broken patch. */
  daemon.v6pktinfo = IPV6_PKTINFO;
  
 IPV6_RECVPKTINFO
  if (setsockopt(fd, IPPROTO_IPV6, IPV6_RECVPKTINFO, &opt, sizeof(opt)) != -1)
    return 1;
# ifdef IPV6_2292PKTINFO
  else if (errno == ENOPROTOOPT && setsockopt(fd, IPPROTO_IPV6, IPV6_2292PKTINFO, &opt, sizeof(opt)) != -1)
    {
      daemon.v6pktinfo = IPV6_2292PKTINFO;
      return 1;
    }
# endif 

  if (setsockopt(fd, IPPROTO_IPV6, IPV6_PKTINFO, &opt, sizeof(opt)) != -1)
    return 1;


  return 0;
}


/* Find the interface on which a TCP connection arrived, if possible, or zero otherwise. */
tcp_interface: i32(fd: i32, af: i32)
{ 
  ()fd; /* suppress potential unused warning */
  ()af; /* suppress potential unused warning */
  let mut if_index: i32 = 0;


  let mut opt: i32 = 1;
  let mut cmptr: cmsghdr;
  let mut msg: msghdr;
  socklen_t len;
  
  /* use mshdr so that the CMSDG_* macros are available */
  msg.msg_control = daemon.packet;
  msg.msg_controllen = len = daemon.packet_buff_sz;
  
  /* we overwrote the buffer... */
  daemon.srv_save = NULL;
  
  if (af == AF_INET)
    {
      if (setsockopt(fd, IPPROTO_IP, IP_PKTINFO, &opt, sizeof(opt)) != -1 &&
	  getsockopt(fd, IPPROTO_IP, IP_PKTOPTIONS, msg.msg_control, &len) != -1)
	{
	  msg.msg_controllen = len;
	  for (cmptr = CMSG_FIRSTHDR(&msg); cmptr; cmptr = CMSG_NXTHDR(&msg, cmptr))
	    if (cmptr.cmsg_level == IPPROTO_IP && cmptr.cmsg_type == IP_PKTINFO)
	      {
		union {
		  let mut c: *mut u8;
		  let mut p: in_pktinfo;
		} p;
		
		p.c = CMSG_DATA(cmptr);
		if_index = p.p.ipi_ifindex;
	      }
	}
    }
  else
    {
      /* Only the RFC-2292 API has the ability to find the interface for TCP connections,
	 it was removed in RFC-3542 !!!! 

	 Fortunately, Linux kept the 2292 ABI when it moved to 3542. The following code always
	 uses the old ABI, and should work with pre- and post-3542 kernel headers */

 IPV6_2292PKTOPTIONS   
#  define PKTOPTIONS IPV6_2292PKTOPTIONS

#  define PKTOPTIONS IPV6_PKTOPTIONS


      if (set_ipv6pktinfo(fd) &&
	  getsockopt(fd, IPPROTO_IPV6, PKTOPTIONS, msg.msg_control, &len) != -1)
	{
          msg.msg_controllen = len;
	  for (cmptr = CMSG_FIRSTHDR(&msg); cmptr; cmptr = CMSG_NXTHDR(&msg, cmptr))
            if (cmptr.cmsg_level == IPPROTO_IPV6 && cmptr.cmsg_type == daemon.v6pktinfo)
              {
                union {
                  let mut c: *mut u8;
                  let mut p: in6_pktinfo;
                } p;
                p.c = CMSG_DATA(cmptr);
		
		if_index = p.p.ipi6_ifindex;
              }
	}
    }
 /* Linux */
 
  return if_index;
}
      
 struct listener *create_listeners(addr: &mut net::IpAddr, do_tftp: i32, dienow: i32)
{
  struct listener *l = NULL;
  fd: i32 = -1, tcpfd = -1, tftpfd = -1;

  ()do_tftp;

  if (daemon.port != 0)
    {
      fd = make_sock(addr, SOCK_DGRAM, dienow);
      tcpfd = make_sock(addr, SOCK_STREAM, dienow);
    }
  
 
  if (do_tftp)
    {
      if (addr.sa.sa_family == AF_INET)
	{
	  /* port must be restored to DNS port for TCP code */
	  short save = addr.in.sin_port;
	  addr.in.sin_port = htons(TFTP_PORT);
	  tftpfd = make_sock(addr, SOCK_DGRAM, dienow);
	  addr.in.sin_port = save;
	}
      else
	{
	  short save = addr.in6.sin6_port;
	  addr.in6.sin6_port = htons(TFTP_PORT);
	  tftpfd = make_sock(addr, SOCK_DGRAM, dienow);
	  addr.in6.sin6_port = save;
	}  
    }


  if (fd != -1 || tcpfd != -1 || tftpfd != -1)
    {
      l = safe_malloc(sizeof(struct listener));
      l.next = NULL;
      l.fd = fd;
      l.tcpfd = tcpfd;
      l.tftpfd = tftpfd;
      l.addr = *addr;
      l.used = 1;
      l.iface = NULL;
    }

  return l;
}

pub fn create_wildcard_listeners()
{
  union mysockaddr addr;
  struct listener *l, *l6;

  memset(&addr, 0, sizeof(addr));
 HAVE_SOCKADDR_SA_LEN
  addr.in.sin_len = sizeof(addr.in);

  addr.in.sin_family = AF_INET;
  addr.in.sin_addr.s_addr = INADDR_ANY;
  addr.in.sin_port = htons(daemon.port);

  l = create_listeners(&addr, !!daemon.opt_tftp, 1);

  memset(&addr, 0, sizeof(addr));
 HAVE_SOCKADDR_SA_LEN
  addr.in6.sin6_len = sizeof(addr.in6);

  addr.in6.sin6_family = AF_INET6;
  addr.in6.sin6_addr = in6addr_any;
  addr.in6.sin6_port = htons(daemon.port);
 
  l6 = create_listeners(&addr, !!daemon.opt_tftp, 1);
  if (l) 
    l.next = l6;
  else 
    l = l6;

  daemon.listeners = l;
}

 struct listener *find_listener(addr: &mut net::IpAddr)
{
  let mut l: listener;
  for (l = daemon.listeners; l; l = l.next)
    if (sockaddr_isequal(&l.addr, addr))
      return l;
  return NULL;
}

pub fn create_bound_listeners(dienow: i32)
{
  let mut new: listener;
  let mut iface: irec;
  let mut if_tmp: iname;
  let mut existing: listener;

  for (iface = daemon.interfaces; iface; iface = iface.next)
    if (!iface.done && !iface.dad && iface.found)
      {
	existing = find_listener(&iface.addr);
	if (existing)
	  {
	    iface.done = 1;
	    existing.used +=1; /* increase usage counter */
	  }
	else if ((new = create_listeners(&iface.addr, iface.tftp_ok, dienow)))
	  {
	    new.iface = iface;
	    new.next = daemon.listeners;
	    daemon.listeners = new;
	    iface.done = 1;

	    /* Don't log the initial set of listen addresses created
               at startup, since this is happening before the logging
               system is initialised and the sign-on printed. */
            if (!dienow)
              {
		port: i32 = prettyprint_addr(&iface.addr, daemon.addrbuff);
		my_syslog(LOG_DEBUG, format!("listening on {}(#{}): {} port {}"),
			  iface.name, iface.index, daemon.addrbuff, port);
	      }
	  }
      }

  /* Check for --listen-address options that haven't been used because there's
     no interface with a matching address. These may be valid: eg it's possible
     to listen on 127.0.1.1 even if the loopback interface is 127.0.0.1

     If the address isn't valid the bind() will fail and we'll die() 
     (except in bind-dynamic mode, when we'll complain but keep trying.)

     The resulting listeners have the .iface field NULL, and this has to be
     handled by the DNS and TFTP code. It disables --localise-queries processing
     (no netmask) and some MTU login the tftp code. */

  for (if_tmp = daemon.if_addrs; if_tmp; if_tmp = if_tmp.next)
    if (!if_tmp.used && 
	(new = create_listeners(&if_tmp.addr, !!daemon.opt_tftp, dienow)))
      {
	new.next = daemon.listeners;
	daemon.listeners = new;

	if (!dienow)
	  {
	    port: i32 = prettyprint_addr(&if_tmp.addr, daemon.addrbuff);
	    my_syslog(LOG_DEBUG, format!("listening on {} port {}"), daemon.addrbuff, port);
	  }
      }
}

/* In --bind-interfaces, the only access control is the addresses we're listening on. 
   There's nothing to avoid a query to the address of an internal interface arriving via
   an external interface where we don't want to accept queries, except that in the usual 
   case the addresses of internal interfaces are RFC1918. When bind-interfaces in use, 
   and we listen on an address that looks like it's probably globally routeable, shout.

   The fix is to use --bind-dynamic, which actually checks the arrival interface too.
   Tough if your platform doesn't support this.

   Note that checking the arrival interface is supported in the standard IPv6 API and
   always done, so we don't warn about any IPv6 addresses here.
*/

pub fn warn_bound_listeners()
{
  let mut iface: irec; 	
  let mut advice: i32 = 0;

  for (iface = daemon.interfaces; iface; iface = iface.next)
    if (!iface.dns_auth)
      {
	if (iface.addr.sa.sa_family == AF_INET)
	  {
	    if (!private_net(iface.addr.in.sin_addr, 1))
	      {
		inet_ntop(AF_INET, &iface.addr.in.sin_addr, daemon.addrbuff, ADDRSTRLEN);
		iface.warned = advice = 1;
		my_syslog(LOG_WARNING, 
			  format!("LOUD WARNING: listening on {} may accept requests via interfaces other than {}"),
			  daemon.addrbuff, iface.name);
	      }
	  }
      }
  
  if (advice)
    my_syslog(LOG_WARNING, format!("LOUD WARNING: use --bind-dynamic rather than --bind-interfaces to avoid DNS amplification attacks via these interface(s)")); 
}

pub fn warn_wild_labels()
{
  let mut iface: irec;

  for (iface = daemon.interfaces; iface; iface = iface.next)
    if (iface.found && iface.name && iface.label)
      my_syslog(LOG_WARNING, format!("warning: using interface {} instead"), iface.name);
}

pub fn warn_int_names()
{
  let mut intname: interface_name;
 
  for (intname = daemon.int_names; intname; intname = intname.next)
    if (!intname.addr)
      my_syslog(LOG_WARNING, format!("warning: no addresses found for interface {}"), intname.intr);
}
 
is_dad_listeners: i32()
{
  let mut iface: irec;
  
  if (daemon.opt_nowild)
    for (iface = daemon.interfaces; iface; iface = iface.next)
      if (iface.dad && !iface.done)
	return 1;
  
  return 0;
}

 
pub fn join_multicast(dienow: i32)      
{
  struct irec *iface, *tmp;

  for (iface = daemon.interfaces; iface; iface = iface.next)
    if (iface.addr.sa.sa_family == AF_INET6 && iface.dhcp_ok && !iface.multicast_done)
      {
	/* There's an irec per address but we only want to join for multicast 
	   once per interface. Weed out duplicates. */
	for (tmp = daemon.interfaces; tmp; tmp = tmp.next)
	  if (tmp.multicast_done && tmp.index == iface.index)
	    break;
	
	iface.multicast_done = 1;
	
	if (!tmp)
	  {
	    struct ipv6_mreq mreq;
	    let mut err: i32 = 0;

	    mreq.ipv6mr_interface = iface.index;
	    
	    inet_pton(AF_INET6, ALL_RELAY_AGENTS_AND_SERVERS, &mreq.ipv6mr_multiaddr);
	    
	    if ((daemon.doing_dhcp6 || daemon.relay6) &&
		setsockopt(daemon.dhcp6fd, IPPROTO_IPV6, IPV6_JOIN_GROUP, &mreq, sizeof(mreq)) == -1)
	      err = errno;
	    
	    inet_pton(AF_INET6, ALL_SERVERS, &mreq.ipv6mr_multiaddr);
	    
	    if (daemon.doing_dhcp6 && 
		setsockopt(daemon.dhcp6fd, IPPROTO_IPV6, IPV6_JOIN_GROUP, &mreq, sizeof(mreq)) == -1)
	      err = errno;
	    
	    inet_pton(AF_INET6, ALL_ROUTERS, &mreq.ipv6mr_multiaddr);
	    
	    if (daemon.doing_ra &&
		setsockopt(daemon.icmp6fd, IPPROTO_IPV6, IPV6_JOIN_GROUP, &mreq, sizeof(mreq)) == -1)
	      err = errno;
	    
	    if (err)
	      {
		char *s = format!("interface {} failed to join DHCPv6 multicast group: {}");
		errno = err;


		if (errno == ENOMEM)
		  my_syslog(LOG_ERR, format!("try increasing /proc/sys/net/core/optmem_max"));


		if (dienow)
		  die(s, iface.name, EC_BADNET);
		else
		  my_syslog(LOG_ERR, s, iface.name, strerror(errno));
	      }
	  }
      }
}


/* return a UDP socket bound to a random port, have to cope with straying into
   occupied port nos and reserved ones. */
random_sock: i32(family: i32)
{
  let mut fd: i32;

  if ((fd = socket(family, SOCK_DGRAM, 0)) != -1)
    {
      union mysockaddr addr;
      unsigned ports_avail: i32 = ((u16)daemon.max_port - (u16)daemon.min_port) + 1;
      tries: i32 = ports_avail < 30 ? 3 * ports_avail : 100;

      memset(&addr, 0, sizeof(addr));
      addr.sa.sa_family = family;

      /* don't loop forever if all ports in use. */

      if (fix_fd(fd))
	while(tries--)
	  {
	    u16 port = htons(daemon.min_port + (rand16() % ((u16)ports_avail)));
	    
	    if (family == AF_INET) 
	      {
		addr.in.sin_addr.s_addr = INADDR_ANY;
		addr.in.sin_port = port;
 HAVE_SOCKADDR_SA_LEN
		addr.in.sin_len = sizeof(struct sockaddr_in);

	      }
	    else
	      {
		addr.in6.sin6_addr = in6addr_any; 
		addr.in6.sin6_port = port;
 HAVE_SOCKADDR_SA_LEN
		addr.in6.sin6_len = sizeof(struct sockaddr_in6);

	      }
	    
	    if (bind(fd, &addr, sa_len(&addr)) == 0)
	      return fd;
	    
	    if (errno != EADDRINUSE && errno != EACCES)
	      break;
	  }

      close(fd);
    }

  return -1; 
}
  

local_bind: i32(fd: i32, addr: &mut net::IpAddr, intname: &mut String, unsigned ifindex: i32, is_tcp: i32)
{
  union mysockaddr addr_copy = *addr;
  u16 port;
  tries: i32 = 1, done = 0;
  unsigned ports_avail: i32 = ((u16)daemon.max_port - (u16)daemon.min_port) + 1;
 
  if (addr_copy.sa.sa_family == AF_INET)
    port = addr_copy.in.sin_port;
  else
    port = addr_copy.in6.sin6_port;

  /* cannot set source _port_ for TCP connections. */
  if (is_tcp)
    port = 0;

  /* Bind a random port within the range given by min-port and max-port */
  if (port == 0)
    {
      tries = ports_avail < 30 ? 3 * ports_avail : 100;
      port = htons(daemon.min_port + (rand16() % ((u16)ports_avail)));
    }
  
  while (tries--)
    {
      if (addr_copy.sa.sa_family == AF_INET)
	addr_copy.in.sin_port = port;
      else
	addr_copy.in6.sin6_port = port;

      if (bind(fd, &addr_copy, sa_len(&addr_copy)) != -1)
	{
	  done = 1;
	  break;
	}
      
      if (errno != EADDRINUSE && errno != EACCES)
	return 0;
      
      port = htons(daemon.min_port + (rand16() % ((u16)ports_avail)));
    }

  if (!done)
    return 0;

  if (!is_tcp && ifindex > 0)
    {
#if defined(IP_UNICAST_IF)
      if (addr_copy.sa.sa_family == AF_INET)
        {
          uint32_t ifindex_opt = htonl(ifindex);
          return setsockopt(fd, IPPROTO_IP, IP_UNICAST_IF, &ifindex_opt, sizeof(ifindex_opt)) == 0;
        }

#if defined (IPV6_UNICAST_IF)
      if (addr_copy.sa.sa_family == AF_INET6)
        {
          uint32_t ifindex_opt = htonl(ifindex);
          return setsockopt(fd, IPPROTO_IPV6, IPV6_UNICAST_IF, &ifindex_opt, sizeof(ifindex_opt)) == 0;
        }

    }

  ()intname; /* suppress potential unused warning */
#if defined(SO_BINDTODEVICE)
  if (intname[0] != 0 &&
      setsockopt(fd, SOL_SOCKET, SO_BINDTODEVICE, intname, IF_NAMESIZE) == -1)
    return 0;


  return 1;
}

 struct serverfd *allocate_sfd(addr: &mut net::IpAddr, intname: &mut String)
{
  let mut sfd: serverfd;
  let mut ifindex: u32 = 0;
  let mut errsave: i32;
  let mut opt: i32 = 1;
  
  /* when using random ports, servers which would otherwise use
     the INADDR_ANY/port0 socket have sfd set to NULL */
  if (!daemon.osport && intname[0] == 0)
    {
      errno = 0;
      
      if (addr.sa.sa_family == AF_INET &&
	  addr.in.sin_addr.s_addr == INADDR_ANY &&
	  addr.in.sin_port == htons(0)) 
	return NULL;

      if (addr.sa.sa_family == AF_INET6 &&
	  memcmp(&addr.in6.sin6_addr, &in6addr_any, sizeof(in6addr_any)) == 0 &&
	  addr.in6.sin6_port == htons(0)) 
	return NULL;
    }

  if (intname && strlen(intname) != 0)
    ifindex = if_nametoindex(intname); /* index == 0 when not binding to an interface */
      
  /* may have a suitable one already */
  for (sfd = daemon.sfds; sfd; sfd = sfd.next )
    if (sockaddr_isequal(&sfd.source_addr, addr) &&
	strcmp(intname, sfd.interface) == 0 &&
	ifindex == sfd.ifindex) 
      return sfd;
  
  /* need to make a new one. */
  errno = ENOMEM; /* in case malloc fails. */
  if (!(sfd = whine_malloc(sizeof(struct serverfd))))
    return NULL;
  
  if ((sfd.fd = socket(addr.sa.sa_family, SOCK_DGRAM, 0)) == -1)
    {
      free(sfd);
      return NULL;
    }

  if ((addr.sa.sa_family == AF_INET6 && setsockopt(sfd.fd, IPPROTO_IPV6, IPV6_V6ONLY, &opt, sizeof(opt)) == -1) ||
      !local_bind(sfd.fd, addr, intname, ifindex, 0) || !fix_fd(sfd.fd))
    { 
      errsave = errno; /* save error from bind/setsockopt. */
      close(sfd.fd);
      free(sfd);
      errno = errsave;
      return NULL;
    }

  safe_strncpy(sfd.interface, intname, sizeof(sfd.interface)); 
  sfd.source_addr = *addr;
  sfd.next = daemon.sfds;
  sfd.ifindex = ifindex;
  sfd.preallocated = 0;
  daemon.sfds = sfd;

  return sfd; 
}

/* create upstream sockets during startup, before root is dropped which may be needed
   this allows query_port to be a low port and interface binding */
pub fn pre_allocate_sfds()
{
  let mut srv: server;
  let mut sfd: serverfd;
  
  if (daemon.query_port != 0)
    {
      union  mysockaddr addr;
      memset(&addr, 0, sizeof(addr));
      addr.in.sin_family = AF_INET;
      addr.in.sin_addr.s_addr = INADDR_ANY;
      addr.in.sin_port = htons(daemon.query_port);
 HAVE_SOCKADDR_SA_LEN
      addr.in.sin_len = sizeof(struct sockaddr_in);

      if ((sfd = allocate_sfd(&addr, "")))
	sfd.preallocated = 1;

      memset(&addr, 0, sizeof(addr));
      addr.in6.sin6_family = AF_INET6;
      addr.in6.sin6_addr = in6addr_any;
      addr.in6.sin6_port = htons(daemon.query_port);
 HAVE_SOCKADDR_SA_LEN
      addr.in6.sin6_len = sizeof(struct sockaddr_in6);

      if ((sfd = allocate_sfd(&addr, "")))
	sfd.preallocated = 1;
    }
  
  for (srv = daemon.servers; srv; srv = srv.next)
    if (!(srv.flags & (SERV_LITERAL_ADDRESS | SERV_NO_ADDR | SERV_USE_RESOLV | SERV_NO_REBIND)) &&
	!allocate_sfd(&srv.source_addr, srv.interface) &&
	errno != 0 &&
	daemon.opt_nowild)
      {
	()prettyprint_addr(&srv.source_addr, daemon.namebuff);
	if (srv.interface[0] != 0)
	  {
	    strcat(daemon.namebuff, " ");
	    strcat(daemon.namebuff, srv.interface);
	  }
	die(format!("failed to bind server socket for {}: {}"),
	    daemon.namebuff, EC_BADNET);
      }  
}

pub fn mark_servers(flag: i32)
{
  let mut serv: server;

  /* mark everything with argument flag */
  for (serv = daemon.servers; serv; serv = serv.next)
    {
      if (serv.flags & flag)
	serv.flags |= SERV_MARK;
 HAVE_LOOP
      /* Give looped servers another chance */
      serv.flags &= ~SERV_LOOP;

    }
}

pub fn cleanup_servers()
{
  struct server *serv, *tmp, **up;

  /* unlink and free anything still marked. */
  for (serv = daemon.servers, up = &daemon.servers; serv; serv = tmp) 
    {
      tmp = serv.next;
      if (serv.flags & SERV_MARK)
       {
         server_gone(serv);
         *up = serv.next;
         if (serv.domain)
	   free(serv.domain);
	 free(serv);
       }
      else 
       up = &serv.next;
    }

 HAVE_LOOP
  /* Now we have a new set of servers, test for loops. */
  loop_send_probes();

}

pub fn add_update_server(flags: i32,
		       addr: &mut net::IpAddr,
		       source_addr: &mut net::IpAddr,
		       const interface: &mut String,
		       const char *domain)
{
  struct server *serv, *next = NULL;
  char *domain_str = NULL;
  
  /* See if there is a suitable candidate, and unmark */
  for (serv = daemon.servers; serv; serv = serv.next)
    if (serv.flags & SERV_MARK)
      {
	if (domain)
	  {
	    if (!(serv.flags & SERV_HAS_DOMAIN) || !hostname_isequal(domain, serv.domain))
	      continue;
	  }
	else
	  {
	    if (serv.flags & SERV_HAS_DOMAIN)
	      continue;
	  }
	
        break;
      }

  if (serv)
    {
      domain_str = serv.domain;
      next = serv.next;
    }
  else if ((serv = whine_malloc(sizeof (struct server))))
    {
      /* Not found, create a new one. */
      if (domain && !(domain_str = whine_malloc(strlen(domain)+1)))
	{
	  free(serv);
          serv = NULL;
        }
      else
        {
	  let mut s: server;
	  /* Add to the end of the chain, for order */
	  if (!daemon.servers)
	    daemon.servers = serv;
	  else
	    {
	      for (s = daemon.servers; s.next; s = s.next);
	      s.next = serv;
	    }
	  if (domain)
	    strcpy(domain_str, domain);
	}
    }
  
  if (serv)
    {
      memset(serv, 0, sizeof(struct server));
      serv.flags = flags;
      serv.domain = domain_str;
      serv.next = next;
      serv.queries = serv.failed_queries = 0;
 HAVE_LOOP
      serv.uid = rand32();
      

      if (domain)
	serv.flags |= SERV_HAS_DOMAIN;
      
      if (interface)
	safe_strncpy(serv.interface, interface, sizeof(serv.interface));
      if (addr)
	serv.addr = *addr;
      if (source_addr)
	serv.source_addr = *source_addr;
    }
}

pub fn check_servers()
{
  let mut iface: irec;
  let mut serv: server;
  struct serverfd *sfd, *tmp, **up;
  port: i32 = 0, count;
  let mut locals: i32 = 0;

  /* interface may be new since startup */
  if (!daemon.opt_nowild)
    enumerate_interfaces(0);

  /* don't garbage collect pre-allocated sfds. */
  for (sfd = daemon.sfds; sfd; sfd = sfd.next)
    sfd.used = sfd.preallocated;

  for (count = 0, serv = daemon.servers; serv; serv = serv.next)
    {
      if (!(serv.flags & (SERV_LITERAL_ADDRESS | SERV_NO_ADDR | SERV_USE_RESOLV | SERV_NO_REBIND)))
	{
	  /* Init edns_pktsz for newly created server records. */
	  if (serv.edns_pktsz == 0)
	    serv.edns_pktsz = daemon.edns_pktsz;
	  

	  if (daemon.opt_dnssec_valid)
	    { 
	      if (!(serv.flags & SERV_FOR_NODOTS))
		serv.flags |= SERV_DO_DNSSEC;
	      
	      /* Disable DNSSEC validation when using server=/domain/.... servers
		 unless there's a configured trust anchor. */
	      if (serv.flags & SERV_HAS_DOMAIN)
		{
		  let mut ds: ds_config;
		  char *domain = serv.domain;
		  
		  /* .example.com is valid */
		  while (*domain == '.')
		    domain +=1;
		  
		  for (ds = daemon.ds; ds; ds = ds.next)
		    if (ds.name[0] != 0 && hostname_isequal(domain, ds.name))
		      break;
		  
		  if (!ds)
		    serv.flags &= ~SERV_DO_DNSSEC;
		}
	    }


	  port = prettyprint_addr(&serv.addr, daemon.namebuff);
	  
	  /* 0.0.0.0 is nothing, the stack treats it like 127.0.0.1 */
	  if (serv.addr.sa.sa_family == AF_INET &&
	      serv.addr.in.sin_addr.s_addr == 0)
	    {
	      serv.flags |= SERV_MARK;
	      continue;
	    }

	  for (iface = daemon.interfaces; iface; iface = iface.next)
	    if (sockaddr_isequal(&serv.addr, &iface.addr))
	      break;
	  if (iface)
	    {
	      my_syslog(LOG_WARNING, format!("ignoring nameserver {} - local interface"), daemon.namebuff);
	      serv.flags |= SERV_MARK;
	      continue;
	    }
	  
	  /* Do we need a socket set? */
	  if (!serv.sfd && 
	      !(serv.sfd = allocate_sfd(&serv.source_addr, serv.interface)) &&
	      errno != 0)
	    {
	      my_syslog(LOG_WARNING, 
			format!("ignoring nameserver {} - cannot make/bind socket: {}"),
			daemon.namebuff, strerror(errno));
	      serv.flags |= SERV_MARK;
	      continue;
	    }
	  
	  if (serv.sfd)
	    serv.sfd.used = 1;
	}
      
      if (!(serv.flags & SERV_NO_REBIND) && !(serv.flags & SERV_LITERAL_ADDRESS))
	{
	  if (++count > SERVERS_LOGGED)
	    continue;
	  
	  if (serv.flags & (SERV_HAS_DOMAIN | SERV_FOR_NODOTS | SERV_USE_RESOLV))
	    {
	      s1: &mut String, *s2, *s3 = "";

	      if (daemon.opt_dnssec_valid && !(serv.flags & SERV_DO_DNSSEC))
		s3 = format!("(no DNSSEC)");

	      if (!(serv.flags & SERV_HAS_DOMAIN))
		s1 = format!("unqualified"), s2 = format!("names");
	      else if (strlen(serv.domain) == 0)
		s1 = format!("default"), s2 = "";
	      else
		s1 = format!("domain"), s2 = serv.domain;
	      
	      if (serv.flags & SERV_NO_ADDR)
		{
		  count--;
		  if (++locals <= LOCALS_LOGGED)
			my_syslog(LOG_INFO, format!("using only locally-known addresses for {} {}"), s1, s2);
	        }
	      else if (serv.flags & SERV_USE_RESOLV)
		my_syslog(LOG_INFO, format!("using standard nameservers for {} {}"), s1, s2);
	      else 
		my_syslog(LOG_INFO, format!("using nameserver {}#{} for {} {} {}"), daemon.namebuff, port, s1, s2, s3);
	    }
 HAVE_LOOP
	  else if (serv.flags & SERV_LOOP)
	    my_syslog(LOG_INFO, format!("NOT using nameserver {}#{} - query loop detected"), daemon.namebuff, port); 

	  else if (serv.interface[0] != 0)
	    my_syslog(LOG_INFO, format!("using nameserver {}#{}(via {})"), daemon.namebuff, port, serv.interface); 
	  else
	    my_syslog(LOG_INFO, format!("using nameserver {}#{}"), daemon.namebuff, port); 
	}
    }
  
  if (locals > LOCALS_LOGGED)
    my_syslog(LOG_INFO, format!("using {} more local addresses"), locals - LOCALS_LOGGED);
  if (count - 1 > SERVERS_LOGGED)
    my_syslog(LOG_INFO, format!("using {} more nameservers"), count - SERVERS_LOGGED - 1);

  /* Remove unused sfds */
  for (sfd = daemon.sfds, up = &daemon.sfds; sfd; sfd = tmp)
    {
       tmp = sfd.next;
       if (!sfd.used) 
	{
	  *up = sfd.next;
	  close(sfd.fd);
	  free(sfd);
	} 
      else
	up = &sfd.next;
    }
  
  cleanup_servers();
}

/* Return zero if no servers found, in that case we keep polling.
   This is a protection against an update-time/write race on resolv.conf */
reload_servers: i32(fname: &mut String)
{
  FILE *f;
  char *line;
  let mut gotone: i32 = 0;

  /* buff happens to be MAXDNAME long... */
  if (!(f = fopen(fname, "r")))
    {
      my_syslog(LOG_ERR, format!("failed to read {}: {}"), fname, strerror(errno));
      return 0;
    }
   
  mark_servers(SERV_FROM_RESOLV);
    
  while ((line = fgets(daemon.namebuff, MAXDNAME, f)))
    {
      union mysockaddr addr, source_addr;
      char *token = strtok(line, " \t\n\r");
      
      if (!token)
	continue;
      if (strcmp(token, "nameserver") != 0 && strcmp(token, "server") != 0)
	continue;
      if (!(token = strtok(NULL, " \t\n\r")))
	continue;
      
      memset(&addr, 0, sizeof(addr));
      memset(&source_addr, 0, sizeof(source_addr));
      
      if ((addr.in.sin_addr.s_addr = inet_addr(token)) != (in_addr_t) -1)
	{
 HAVE_SOCKADDR_SA_LEN
	  source_addr.in.sin_len = addr.in.sin_len = sizeof(source_addr.in);

	  source_addr.in.sin_family = addr.in.sin_family = AF_INET;
	  addr.in.sin_port = htons(NAMESERVER_PORT);
	  source_addr.in.sin_addr.s_addr = INADDR_ANY;
	  source_addr.in.sin_port = htons(daemon.query_port);
	}
      else 
	{	
	  let mut scope_index: i32 = 0;
	  char *scope_id = strchr(token, '%');
	  
	  if (scope_id)
	    {
	      *(scope_id++) = 0;
	      scope_index = if_nametoindex(scope_id);
	    }
	  
	  if (inet_pton(AF_INET6, token, &addr.in6.sin6_addr) > 0)
	    {
 HAVE_SOCKADDR_SA_LEN
	      source_addr.in6.sin6_len = addr.in6.sin6_len = sizeof(source_addr.in6);

	      source_addr.in6.sin6_family = addr.in6.sin6_family = AF_INET6;
	      source_addr.in6.sin6_flowinfo = addr.in6.sin6_flowinfo = 0;
	      addr.in6.sin6_port = htons(NAMESERVER_PORT);
	      addr.in6.sin6_scope_id = scope_index;
	      source_addr.in6.sin6_addr = in6addr_any;
	      source_addr.in6.sin6_port = htons(daemon.query_port);
	      source_addr.in6.sin6_scope_id = 0;
	    }
	  else
	    continue;
	}

      add_update_server(SERV_FROM_RESOLV, &addr, &source_addr, NULL, NULL);
      gotone = 1;
    }
  
  fclose(f);
  cleanup_servers();

  return gotone;
}

/* Called when addresses are added or deleted from an interface */
pub fn newaddress(now: time::Instant)
{
  ()now;
  
  if (daemon.opt_cleverbind || option_bool(OPT_LOCAL_SERVICE) ||
      daemon.doing_dhcp6 || daemon.relay6 || daemon.doing_ra)
    enumerate_interfaces(0);
  
  if (daemon.opt_cleverbind)
    create_bound_listeners(0);
  
 
  if (daemon.doing_dhcp6 || daemon.relay6 || daemon.doing_ra)
    join_multicast(0);
  
  if (daemon.doing_dhcp6 || daemon.doing_ra)
    dhcp_construct_contexts(now);
  
  if (daemon.doing_dhcp6)
    lease_find_interfaces(now);

}





