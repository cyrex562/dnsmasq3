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



 

pub struct  iface_param {
  pub current: dhcp_context,
  pub relay: dhcp_relay,
  pub relay_local: net::IpAddr,
  pub ind: i32,
}

pub struct match_param {
  // ind: i32, matched;
  pub ind: i32,
  pub matched: i32,
  // netmask: net::IpAddr, broadcast, addr;
  pub netmask: net::IpAddr,
  pub broadcast: net::IpAddr,
  pub addr: net::IpAddr,
}



 pub struct control_u {
    pub align: cmsghdr, //struct cmsghdr align; /* this ensures alignment */
    //char control[CMSG_SPACE(sizeof(struct in_pktinfo))];
    pub control: Vec<u8>,
  } 

 pub fn make_fd(port: i32) -> i32
{
  let fd = socket(PF_INET, SOCK_DGRAM, IPPROTO_UDP);
  // struct sockaddr_in saddr;
  let mut oneopt: i32 = 1;
  let mtu = IP_PMTUDISC_DONT;
  let tos = IPTOS_CLASS_CS6;


  if (fd == -1) {
    panic!(String::from("cannot create DHCP socket"));
  }
  
  if (!fix_fd(fd) ||
      setsockopt(fd, IPPROTO_IP, IP_MTU_DISCOVER, &mtu, sizeof(mtu)) == -1 ||
      setsockopt(fd, IPPROTO_IP, IP_TOS, &tos, sizeof(tos)) == -1 ||
      setsockopt(fd, IPPROTO_IP, IP_PKTINFO, &oneopt, sizeof(oneopt)) == -1 ||
      setsockopt(fd, IPPROTO_IP, IP_RECVIF, &oneopt, sizeof(oneopt)) == -1 ||
      setsockopt(fd, SOL_SOCKET, SO_BROADCAST, &oneopt, sizeof(oneopt)) == -1)  {
    panic!(String::fromat("failed to set options on DHCP socket"));
      }
  
  /* When bind-interfaces is set, there might be more than one dnsmasq
     instance binding port 67. That's OK if they serve different networks.
     Need to set REUSEADDR|REUSEPORT to make this possible.
     Handle the case that REUSEPORT is defined, but the kernel doesn't 
     support it. This handles the introduction of REUSEPORT on Linux. */
  if (daemon.opt_nowild || daemon.opt_cleverbind)
    {
      let mut rc: i32 = 0;

 SO_REUSEPORT
      if ((rc = setsockopt(fd, SOL_SOCKET, SO_REUSEPORT, &oneopt, sizeof(oneopt))) == -1 && 
	  errno == ENOPROTOOPT)
	rc = 0;

      
      if (rc != -1)
	rc = setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &oneopt, sizeof(oneopt));
      
      if (rc == -1)
	die(format!("failed to set SO_REUSE{ADDR|PORT} on DHCP socket: {}"), NULL, EC_BADNET);
    }
  
  memset(&saddr, 0, sizeof(saddr));
  saddr.sin_family = AF_INET;
  saddr.sin_port = htons(port);
  saddr.sin_addr.s_addr = INADDR_ANY;
 HAVE_SOCKADDR_SA_LEN
  saddr.sin_len = sizeof(struct sockaddr_in);


  if (bind(fd, &saddr, sizeof(struct sockaddr_in)))
    die(format!("failed to bind DHCP server socket: {}"), NULL, EC_BADNET);

  return fd;
}

pub fn dhcp_init()
{

  let mut oneopt: i32 = 1;


  daemon.dhcpfd = make_fd(daemon.dhcp_server_port);
  if (daemon.enable_pxe) {
    daemon.pxefd = make_fd(PXE_PORT);
  }
  else {
    daemon.pxefd = -1;
  }

  /* When we're not using capabilities, we need to do this here before
     we drop root. Also, set buffer size small, to avoid wasting
     kernel buffers */
  
  if (option_bool(OPT_NO_PING)) {
    daemon.dhcp_icmp_fd = -1;
  }
  else if ((daemon.dhcp_icmp_fd = make_icmp_sock()) == -1 ||
	   setsockopt(daemon.dhcp_icmp_fd, SOL_SOCKET, SO_RCVBUF, &oneopt, sizeof(oneopt)) == -1 ) {
    panic!(format!("cannot create ICMP raw socket."));
     }
  
  /* Make BPF raw send socket */
  init_bpf();
  
}

pub fn dhcp_packet(daemon: &mut DnsmasqDaemon, now: time::Instant, pxe_fd: i32)
{
  // fd: i32 = pxe_fd ? daemon.pxefd : daemon.dhcpfd;
  let mut fd: i32;
  if pxe_fd {
    fd = daemon.pxefd;
  } else {
    fd = daemon.dhcpfd;
  }
  let mut mess: dhcp_packet;
  let mut context: dhcp_context;
  let mut relay: dhcp_relay;
  let mut is_relay_reply: i32 = 0;
  let mut tmp: iname;
  let mut ifr: ifreq;
  let mut msg: msghdr;
  // struct sockaddr_in dest;
  let mut dest: net::IpAddr;
  let mut cmptr: cmsghdr;
  let mut iov: iovec;
  let mut ssz: usize; 
  // iface_index: i32 = 0, unicast_dest = 0, is_inform = 0, loopback = 0;
  let mut  iface_index: i32 = 0;
  let mut unicast_dest: i32 = 0;
  let mut is_inform: i32 = 0;
  let mut rcvd_iface_index: i32;
  iface_addr: net::IpAddr;
  let mut parm: iface_param;
  recvtime: time::Instant = now;

  // struct arpreq arp_req;
  let arp_reg: arpreq;
  // struct timeval tv;
  let tv: time::Instant;
  
 
  struct dhcp_bridge *bridge, *alias;

  msg.msg_controllen = sizeof(control_u);
  msg.msg_control = control_u.control;
  msg.msg_name = &dest;
  msg.msg_namelen = sizeof(dest);
  msg.msg_iov = &daemon.dhcp_packet;
  msg.msg_iovlen = 1;
  
  if ((sz = recv_dhcp_packet(fd, &msg)) == -1 || 
      (sz < (ssize_t)(sizeof(*mess) - sizeof(mess.options)))) 
    return;
    
  if (ioctl(fd, SIOCGSTAMP, &tv) == 0) {
    recvtime = tv.tv_sec;
  }

  if (msg.msg_controllen >= sizeof(struct cmsghdr)) {
    for (cmptr = CMSG_FIRSTHDR(&msg); cmptr; cmptr = CMSG_NXTHDR(&msg, cmptr)) {
      if (cmptr.cmsg_level == IPPROTO_IP && cmptr.cmsg_type == IP_PKTINFO)
	{
	  union {
	    let mut c: *mut u8;
	    let mut p: in_pktinfo;
	  } p;
	  p.c = CMSG_DATA(cmptr);
	  iface_index = p.p.ipi_ifindex;
	  if (p.p.ipi_addr.s_addr != INADDR_BROADCAST)
	    unicast_dest = 1;
	}
}
  }

 
  if (msg.msg_controllen >= sizeof(struct cmsghdr))
    for (cmptr = CMSG_FIRSTHDR(&msg); cmptr; cmptr = CMSG_NXTHDR(&msg, cmptr))
      if (cmptr.cmsg_level == IPPROTO_IP && cmptr.cmsg_type == IP_RECVIF)
        {
	  union {
            let mut c: *mut u8;
            let mut s: sockaddr_dl;
          } p;
	  p.c = CMSG_DATA(cmptr);
	  iface_index = p.s.sdl_index;
	}
  
  if (msg.msg_controllen >= sizeof(struct cmsghdr))
    for (cmptr = CMSG_FIRSTHDR(&msg); cmptr; cmptr = CMSG_NXTHDR(&msg, cmptr))
      if (cmptr.cmsg_level == IPPROTO_IP && cmptr.cmsg_type == IP_RECVIF)
	{
	  union {
	    let mut c: *mut u8;
	    i: &mut u32;
	  } p;
	  p.c = CMSG_DATA(cmptr);
	  iface_index = *(p.i);
	}

	
  if (!indextoname(daemon.dhcpfd, iface_index, ifr.ifr_name) ||
      ioctl(daemon.dhcpfd, SIOCGIFFLAGS, &ifr) != 0)
    return;
  
  mess = (struct dhcp_packet *)daemon.dhcp_packet.iov_base;
  loopback = !mess.giaddr.s_addr && (ifr.ifr_flags & IFF_LOOPBACK);
  

  /* ARP fiddling uses original interface even if we pretend to use a different one. */
  safe_strncpy(arp_req.arp_dev, ifr.ifr_name, sizeof(arp_req.arp_dev));
 

  /* If the interface on which the DHCP request was received is an
     alias of some other interface (as specified by the
     --bridge-interface option), change ifr.ifr_name so that we look
     for DHCP contexts associated with the aliased interface instead
     of with the aliasing one. */
  rcvd_iface_index = iface_index;
  // for (bridge = daemon.bridges; bridge; bridge = bridge.next)
  for bridge in daemon.bridges
    {
      // for (alias = bridge.alias; alias; alias = alias.next) 
      for alias in bridge.alias
      {
	if (wildcard_matchn(alias.iface, ifr.ifr_name, IF_NAMESIZE))
	  {
	    if (!(iface_index = if_nametoindex(bridge.iface)))
	      {
		my_syslog(MS_DHCP | LOG_WARNING,
			  format!("unknown interface {} in bridge-interface"),
			  bridge.iface);
		return;
	      }
	    else 
	      {
		safe_strncpy(ifr.ifr_name,  bridge.iface, sizeof(ifr.ifr_name));
		break;
	      }
	  }
  }
      
      if (alias) {
	break;
      }
    }

 MSG_BCAST
  /* OpenBSD tells us when a packet was broadcast */
  if (!(msg.msg_flags & MSG_BCAST))
    unicast_dest = 1;

  
  if ((relay = relay_reply4((struct dhcp_packet *)daemon.dhcp_packet.iov_base, ifr.ifr_name)))
    {
      /* Reply from server, using us as relay. */
      rcvd_iface_index = relay.iface_index;
      if (!indextoname(daemon.dhcpfd, rcvd_iface_index, ifr.ifr_name))
	return;
      is_relay_reply = 1; 
      iov.iov_len = sz;

      safe_strncpy(arp_req.arp_dev, ifr.ifr_name, sizeof(arp_req.arp_dev));
 
    }
  else
    {
      ifr.ifr_addr.sa_family = AF_INET;
      if (ioctl(daemon.dhcpfd, SIOCGIFADDR, &ifr) != -1 )
	iface_addr = ( &ifr.ifr_addr).sin_addr;
      else
	{
	  if (iface_check(AF_INET, NULL, ifr.ifr_name, NULL))
	    my_syslog(MS_DHCP | LOG_WARNING, format!("DHCP packet received on {} which has no address"), ifr.ifr_name);
	  return;
	}
      
      for (tmp = daemon.dhcp_except; tmp; tmp = tmp.next)
	if (tmp.name && wildcard_match(tmp.name, ifr.ifr_name))
	  return;
      
      /* unlinked contexts/relays are marked by context.current == context */
      for (context = daemon.dhcp; context; context = context.next)
	context.current = context;
      
      for (relay = daemon.relay4; relay; relay = relay.next)
	relay.current = relay;
      
      parm.current = NULL;
      parm.relay = NULL;
      parm.relay_local.s_addr = 0;
      parm.ind = iface_index;
      
      if (!iface_check(AF_INET, (union all_addr *)&iface_addr, ifr.ifr_name, NULL))
	{
	  /* If we failed to match the primary address of the interface, see if we've got a --listen-address
	     for a secondary */
	  struct match_param match;
	  
	  match.matched = 0;
	  match.ind = iface_index;
	  
	  if (!daemon.if_addrs ||
	      !iface_enumerate(AF_INET, &match, check_listen_addrs) ||
	      !match.matched)
	    return;
	  
	  iface_addr = match.addr;
	  /* make sure secondary address gets priority in case
	     there is more than one address on the interface in the same subnet */
	  complete_context(match.addr, iface_index, NULL, match.netmask, match.broadcast, &parm);
	}    
      
      if (!iface_enumerate(AF_INET, &parm, complete_context))
	return;

      /* We're relaying this request */
      if  (parm.relay_local.s_addr != 0 &&
	   relay_upstream4(parm.relay, mess, sz, iface_index))
	return;

      /* May have configured relay, but not DHCP server */
      if (!daemon.dhcp)
	return;

      lease_prune(NULL, now); /* lose any expired leases */
      iov.iov_len = dhcp_reply(parm.current, ifr.ifr_name, iface_index, sz, 
			       now, unicast_dest, loopback, &is_inform, pxe_fd, iface_addr, recvtime);
      lease_update_file(now);
      lease_update_dns(0);
      
      if (iov.iov_len == 0)
	return;
    }

  msg.msg_name = &dest;
  msg.msg_namelen = sizeof(dest);
  msg.msg_control = NULL;
  msg.msg_controllen = 0;
  msg.msg_iov = &iov;
  iov.iov_base = daemon.dhcp_packet.iov_base;
  
  /* packet buffer may have moved */
  mess = (struct dhcp_packet *)daemon.dhcp_packet.iov_base;
  
 HAVE_SOCKADDR_SA_LEN
  dest.sin_len = sizeof(struct sockaddr_in);

  
  if (pxe_fd)
    { 
      if (mess.ciaddr.s_addr != 0)
	dest.sin_addr = mess.ciaddr;
    }
  else if (mess.giaddr.s_addr && !is_relay_reply)
    {
      /* Send to BOOTP relay  */
      dest.sin_port = htons(daemon.dhcp_server_port);
      dest.sin_addr = mess.giaddr; 
    }
  else if (mess.ciaddr.s_addr)
    {
      /* If the client's idea of its own address tallys with
	 the source address in the request packet, we believe the
	 source port too, and send back to that.  If we're replying 
	 to a DHCPINFORM, trust the source address always. */
      if ((!is_inform && dest.sin_addr.s_addr != mess.ciaddr.s_addr) ||
	  dest.sin_port == 0 || dest.sin_addr.s_addr == 0 || is_relay_reply)
	{
	  dest.sin_port = htons(daemon.dhcp_client_port); 
	  dest.sin_addr = mess.ciaddr;
	}
    } 
#if defined(HAVE_LINUX_NETWORK)
  else
    {
      /* fill cmsg for outbound interface (both broadcast & unicast) */
      let mut pkt: in_pktinfo;
      msg.msg_control = control_u.control;
      msg.msg_controllen = sizeof(control_u);
      cmptr = CMSG_FIRSTHDR(&msg);
      pkt = (struct in_pktinfo *)CMSG_DATA(cmptr);
      pkt.ipi_ifindex = rcvd_iface_index;
      pkt.ipi_spec_dst.s_addr = 0;
      msg.msg_controllen = CMSG_SPACE(sizeof(struct in_pktinfo));
      cmptr.cmsg_len = CMSG_LEN(sizeof(struct in_pktinfo));
      cmptr.cmsg_level = IPPROTO_IP;
      cmptr.cmsg_type = IP_PKTINFO;

      if ((ntohs(mess.flags) & 0x8000) || mess.hlen == 0 ||
         mess.hlen > sizeof(ifr.ifr_addr.sa_data) || mess.htype == 0)
        {
          /* broadcast to 255.255.255.255 (or mac address invalid) */
          dest.sin_addr.s_addr = INADDR_BROADCAST;
          dest.sin_port = htons(daemon.dhcp_client_port);
        }
      else
        {
          /* unicast to unconfigured client. Inject mac address direct into ARP cache.
          struct sockaddr limits size to 14 bytes. */
          dest.sin_addr = mess.yiaddr;
          dest.sin_port = htons(daemon.dhcp_client_port);
          memcpy(&arp_req.arp_pa, &dest, sizeof(struct sockaddr_in));
          arp_req.arp_ha.sa_family = mess.htype;
          memcpy(arp_req.arp_ha.sa_data, mess.chaddr, mess.hlen);
          /* interface name already copied in */
          arp_req.arp_flags = ATF_COM;
          if (ioctl(daemon.dhcpfd, SIOCSARP, &arp_req) == -1)
            my_syslog(MS_DHCP | LOG_ERR, format!("ARP-cache injection failed: {}"), strerror(errno));
        }
    }
#elif defined(HAVE_SOLARIS_NETWORK)
  else if ((ntohs(mess.flags) & 0x8000) || mess.hlen != ETHER_ADDR_LEN || mess.htype != ARPHRD_ETHER)
    {
      /* broadcast to 255.255.255.255 (or mac address invalid) */
      dest.sin_addr.s_addr = INADDR_BROADCAST;
      dest.sin_port = htons(daemon.dhcp_client_port);
      /* note that we don't specify the interface here: that's done by the
	 IP_BOUND_IF sockopt lower down. */
    }
  else
    {
      /* unicast to unconfigured client. Inject mac address direct into ARP cache. 
	 Note that this only works for ethernet on solaris, because we use SIOCSARP
	 and not SIOCSXARP, which would be perfect, except that it returns ENXIO 
	 mysteriously. Bah. Fall back to broadcast for other net types. */
      struct arpreq req;
      dest.sin_addr = mess.yiaddr;
      dest.sin_port = htons(daemon.dhcp_client_port);
      *(&req.arp_pa) = dest;
      req.arp_ha.sa_family = AF_UNSPEC;
      memcpy(req.arp_ha.sa_data, mess.chaddr, mess.hlen);
      req.arp_flags = ATF_COM;
      ioctl(daemon.dhcpfd, SIOCSARP, &req);
    }

  else 
    {
      send_via_bpf(mess, iov.iov_len, iface_addr, &ifr);
      return;
    }

   
 HAVE_SOLARIS_NETWORK
  setsockopt(fd, IPPROTO_IP, IP_BOUND_IF, &iface_index, sizeof(iface_index));

  
  while(retry_send(sendmsg(fd, &msg, 0)));

  /* This can fail when, eg, iptables DROPS destination 255.255.255.255 */
  if (errno != 0)
    my_syslog(MS_DHCP | LOG_WARNING, format!("Error sending DHCP packet to {}: {}"),
	      inet_ntoa(dest.sin_addr), strerror(errno));
}

/* check against secondary interface addresses */
 pub fn check_listen_addrs(local: net::IpAddr, if_index: i32, label: &mut String,
			      netmask: net::IpAddr, broadcast: net::IpAddr, vparam: Vec<u8>) -> i32
{
  struct match_param *param = vparam;
  let mut tmp: iname;

  () label;

  if (if_index == param.ind)
    {
      for (tmp = daemon.if_addrs; tmp; tmp = tmp.next)
	if ( tmp.addr.sa.sa_family == AF_INET &&
	     tmp.addr.in.sin_addr.s_addr == local.s_addr)
	  {
	    param.matched = 1;
	    param.addr = local;
	    param.netmask = netmask;
	    param.broadcast = broadcast;
	    break;
	  }
    }
  
  return 1;
}

/* This is a complex routine: it gets called with each (address,netmask,broadcast) triple 
   of each interface (and any relay address) and does the  following things:

   1) Discards stuff for interfaces other than the one on which a DHCP packet just arrived.
   2) Fills in any netmask and broadcast addresses which have not been explicitly configured.
   3) Fills in local (this host) and router (this host or relay) addresses.
   4) Links contexts which are valid for hosts directly connected to the arrival interface on .current.

   Note that the current chain may be superseded later for configured hosts or those coming via gateways. */

pub fn guess_range_netmask(addr: net::IpAddr, netmask: net::IpAddr)
{
  let mut context: dhcp_context;

  for (context = daemon.dhcp; context; context = context.next)
    if (!(context.flags & CONTEXT_NETMASK) &&
	(is_same_net(addr, context.start, netmask) ||
	 is_same_net(addr, context.end, netmask)))
      { 
	if (context.netmask.s_addr != netmask.s_addr &&
	    !(is_same_net(addr, context.start, netmask) &&
	      is_same_net(addr, context.end, netmask)))
	  {
	    strcpy(daemon.dhcp_buff, inet_ntoa(context.start));
	    strcpy(daemon.dhcp_buff2, inet_ntoa(context.end));
	    my_syslog(MS_DHCP | LOG_WARNING, format!("DHCP range {} -- {} is not consistent with netmask {}"),
		      daemon.dhcp_buff, daemon.dhcp_buff2, inet_ntoa(netmask));
	  }	
	context.netmask = netmask;
      }
}

 complete_context: i32(local: net::IpAddr, if_index: i32, label: &mut String,
			    netmask: net::IpAddr, broadcast: net::IpAddr, vparam: Vec<u8>)
{
  let mut context: dhcp_context;
  let mut relay: dhcp_relay;
  struct iface_param *param = vparam;
  let mut share: shared_network;
  
  ()label;

  for (share = daemon.shared_networks; share; share = share.next)
    {
      
 
      if (share.shared_addr.s_addr == 0)
	continue;

      
      if (share.if_index != 0)
	{
	  if (share.if_index != if_index)
	    continue;
	}
      else
	{
	  if (share.match_addr.s_addr != local.s_addr)
	    continue;
	}

      for (context = daemon.dhcp; context; context = context.next)
	{
	  if (context.netmask.s_addr != 0 &&
	      is_same_net(share.shared_addr, context.start, context.netmask) &&
	      is_same_net(share.shared_addr, context.end, context.netmask))
	    {
	      /* link it onto the current chain if we've not seen it before */
	      if (context.current == context)
		{
		  /* For a shared network, we have no way to guess what the default route should be. */
		  context.router.s_addr = 0;
		  context.local = local; /* Use configured address for Server Identifier */
		  context.current = param.current;
		  param.current = context;
		}
	      
	      if (!(context.flags & CONTEXT_BRDCAST))
		context.broadcast.s_addr  = context.start.s_addr | ~context.netmask.s_addr;
	    }		
	}
    }

  guess_range_netmask(local, netmask);
  
  for (context = daemon.dhcp; context; context = context.next)
    {
      if (context.netmask.s_addr != 0 &&
	  is_same_net(local, context.start, context.netmask) &&
	  is_same_net(local, context.end, context.netmask))
	{
	  /* link it onto the current chain if we've not seen it before */
	  if (if_index == param.ind && context.current == context)
	    {
	      context.router = local;
	      context.local = local;
	      context.current = param.current;
	      param.current = context;
	    }
	  
	  if (!(context.flags & CONTEXT_BRDCAST))
	    {
	      if (is_same_net(broadcast, context.start, context.netmask))
		context.broadcast = broadcast;
	      else 
		context.broadcast.s_addr  = context.start.s_addr | ~context.netmask.s_addr;
	    }
	}		
    }

  for (relay = daemon.relay4; relay; relay = relay.next)
    if (if_index == param.ind && relay.local.addr4.s_addr == local.s_addr && relay.current == relay &&
	(param.relay_local.s_addr == 0 || param.relay_local.s_addr == local.s_addr))
      {
	relay.current = param.relay;
	param.relay = relay;
	param.relay_local = local;	
      }

  return 1;
}
	  
struct dhcp_context *address_available(struct dhcp_context *context, 
				       taddr: net::IpAddr,
				       struct dhcp_netid *netids)
{
  /* Check is an address is OK for this network, check all
     possible ranges. Make sure that the address isn't in use
     by the server itself. */
  
  unsigned start: i32, end, addr = ntohl(taddr.s_addr);
  let mut tmp: dhcp_context;

  for (tmp = context; tmp; tmp = tmp.current)
    if (taddr.s_addr == context.router.s_addr)
      return NULL;
  
  for (tmp = context; tmp; tmp = tmp.current)
    {
      start = ntohl(tmp.start.s_addr);
      end = ntohl(tmp.end.s_addr);

      if (!(tmp.flags & (CONTEXT_ | CONTEXT_PROXY)) &&
	  addr >= start &&
	  addr <= end &&
	  match_netid(tmp.filter, netids, 1))
	return tmp;
    }

  return NULL;
}

struct dhcp_context *narrow_context(struct dhcp_context *context, 
				    taddr: net::IpAddr,
				    struct dhcp_netid *netids)
{
  /* We start of with a set of possible contexts, all on the current physical interface.
     These are chained on .current.
     Here we have an address, and return the actual context corresponding to that
     address. Note that none may fit, if the address came a dhcp-host and is outside
     any dhcp-range. In that case we return a  range if possible, or failing that,
     any context on the correct subnet. (If there's more than one, this is a dodgy 
     configuration: maybe there should be a warning.) */
  
  let mut tmp: dhcp_context;

  if (!(tmp = address_available(context, taddr, netids)))
    {
      for (tmp = context; tmp; tmp = tmp.current)
	if (match_netid(tmp.filter, netids, 1) &&
	    is_same_net(taddr, tmp.start, tmp.netmask) && 
	    (tmp.flags & CONTEXT_))
	  break;
      
      if (!tmp)
	for (tmp = context; tmp; tmp = tmp.current)
	  if (match_netid(tmp.filter, netids, 1) &&
	      is_same_net(taddr, tmp.start, tmp.netmask) &&
	      !(tmp.flags & CONTEXT_PROXY))
	    break;
    }
  
  /* Only one context allowed now */
  if (tmp)
    tmp.current = NULL;
  
  return tmp;
}

struct dhcp_config *config_find_by_address(struct dhcp_config *configs, addr: net::IpAddr)
{
  let mut config: dhcp_config;
  
  for (config = configs; config; config = config.next)
    if ((config.flags & CONFIG_ADDR) && config.addr.s_addr == addr.s_addr)
      return config;

  return NULL;
}

/* Check if and address is in use by sending ICMP ping.
   This wrapper handles a cache and load-limiting.
   Return is NULL is address in use, or a pointer to a cache entry
   recording that it isn't. */
struct ping_result *do_icmp_ping(now: time::Instant, addr: net::IpAddr, unsigned hash: i32, loopback: i32)
{
   struct ping_result dummy;
  struct ping_result *r, *victim = NULL;
  count: i32, max = (0.6 * (((float)PING_CACHE_TIME)/
				((float)PING_WAIT)));

  /* check if we failed to ping addr sometime in the last
     PING_CACHE_TIME seconds. If so, assume the same situation still exists.
     This avoids problems when a stupid client bangs
     on us repeatedly. As a final check, if we did more
     than 60% of the possible ping checks in the last 
     PING_CACHE_TIME, we are in high-load mode, so don't do any more. */
  for (count = 0, r = daemon.ping_results; r; r = r.next)
    if (difftime(now, r.time) >  (float)PING_CACHE_TIME)
      victim = r; /* old record */
    else 
      {
	count +=1;
	if (r.addr.s_addr == addr.s_addr)
	  return r;
      }
  
  /* didn't find cached entry */
  if ((count >= max) || option_bool(OPT_NO_PING) || loopback)
    {
      /* overloaded, or configured not to check, loopback interface, return "not in use" */
      dummy.hash = hash;
      return &dummy;
    }
  else if (icmp_ping(addr))
    return NULL; /* address in use. */
  else
    {
      /* at this povictim: i32 may hold an expired record */
      if (!victim)
	{
	  if ((victim = whine_malloc(sizeof(struct ping_result))))
	    {
	      victim.next = daemon.ping_results;
	      daemon.ping_results = victim;
	    }
	}
      
      /* record that this address is OK for 30s 
	 without more ping checks */
      if (victim)
	{
	  victim.addr = addr;
	  victim.time = now;
	  victim.hash = hash;
	}
      return victim;
    }
}

address_allocate: i32(struct dhcp_context *context,
		     struct in_addr *addrp, hwaddr: &mut Vec<u8>, hw_len: i32, 
		     struct dhcp_netid *netids, now: &time::Instant, loopback: i32)   
{
  /* Find a free address: exclude anything in use and anything allocated to
     a particular hwaddr/clientid/hostname in our configuration.
     Try to return from contexts which match netids first. */

  start: net::IpAddr, addr;
  struct dhcp_context *c, *d;
  i: i32, pass;
  let mut j: u32; 

  /* hash hwaddr: use the SDBM hashing algorithm.  Seems to give good
     dispersal even with similarly-valued "strings". */ 
  for (j = 0, i = 0; i < hw_len; i++)
    j = hwaddr[i] + (j << 6) + (j << 16) - j;

  /* j == 0 is marker */
  if (j == 0)
    j = 1;
  
  for (pass = 0; pass <= 1; pass++)
    for (c = context; c; c = c.current)
      if (c.flags & (CONTEXT_ | CONTEXT_PROXY))
	continue;
      else if (!match_netid(c.filter, netids, pass))
	continue;
      else
	{
	  if (option_bool(OPT_CONSEC_ADDR))
	    /* seed is largest extant lease addr in this context */
	    start = lease_find_max_addr(c);
	  else
	    /* pick a seed based on hwaddr */
	    start.s_addr = htonl(ntohl(c.start.s_addr) + 
				 ((j + c.addr_epoch) % (1 + ntohl(c.end.s_addr) - ntohl(c.start.s_addr))));

	  /* iterate until we find a free address. */
	  addr = start;
	  
	  do {
	    /* eliminate addresses in use by the server. */
	    for (d = context; d; d = d.current)
	      if (addr.s_addr == d.router.s_addr)
		break;

	    /* Addresses which end in .255 and .0 are broken in Windows even when using 
	       supernetting. ie dhcp-range=192.168.0.1,192.168.1.254,255,255,254.0
	       then 192.168.0.255 is a valid IP address, but not for Windows as it's
	       in the class C range. See  KB281579. We therefore don't allocate these 
	       addresses to avoid hard-to-diagnose problems. Thanks Bill. */	    
	    if (!d &&
		!lease_find_by_addr(addr) && 
		!config_find_by_address(daemon.dhcp_conf, addr) &&
		(!IN_CLASSC(ntohl(addr.s_addr)) || 
		 ((ntohl(addr.s_addr) & 0xff) != 0xff && ((ntohl(addr.s_addr) & 0xff) != 0x0))))
	      {
		/* in consec-ip mode, skip addresses equal to
		   the number of addresses rejected by clients. This
		   should avoid the same client being offered the same
		   address after it has rjected it. */
		if (option_bool(OPT_CONSEC_ADDR) && c.addr_epoch)
		  c.addr_epoch--;
		else
		  {
		    let mut r: ping_result;
		    
		    if ((r = do_icmp_ping(now, addr, j, loopback)))
		      {
			/* consec-ip mode: we offered this address for another client
			   (different hash) recently, don't offer it to this one. */
			if (!option_bool(OPT_CONSEC_ADDR) || r.hash == j)
			  {
			    *addrp = addr;
			    return 1;
			  }
		      }
		    else
		      {
			/* address in use: perturb address selection so that we are
			   less likely to try this address again. */
			if (!option_bool(OPT_CONSEC_ADDR))
			  c.addr_epoch +=1;
		      }
		  }
	      }
	    
	    addr.s_addr = htonl(ntohl(addr.s_addr) + 1);
	    
	    if (addr.s_addr == htonl(ntohl(c.end.s_addr) + 1))
	      addr = c.start;
	    
	  } while (addr.s_addr != start.s_addr);
	}

  return 0;
}

pub fn dhcp_read_ethers()
{
  FILE *f = fopen(ETHERSFILE, "r");
  let mut flags: u32;
  char *buff = daemon.namebuff;
  ip: &mut String, *cp;
  addr: net::IpAddr;
  unsigned char hwaddr[ETHER_ADDR_LEN];
  struct dhcp_config **up, *tmp;
  let mut config: dhcp_config;
  count: i32 = 0, lineno = 0;

  addr.s_addr = 0; /* eliminate warning */
  
  if (!f)
    {
      my_syslog(MS_DHCP | LOG_ERR, format!("failed to read {}: {}"), ETHERSFILE, strerror(errno));
      return;
    }

  /* This can be called again on SIGHUP, so remove entries created last time round. */
  for (up = &daemon.dhcp_conf, config = daemon.dhcp_conf; config; config = tmp)
    {
      tmp = config.next;
      if (config.flags & CONFIG_FROM_ETHERS)
	{
	  *up = tmp;
	  /* cannot have a clid */
	  if (config.flags & CONFIG_NAME)
	    free(config.hostname);
	  free(config.hwaddr);
	  free(config);
	}
      else
	up = &config.next;
    }

  while (fgets(buff, MAXDNAME, f))
    {
      char *host = NULL;
      
      lineno +=1;
      
      while (strlen(buff) > 0 && isspace(buff[strlen(buff)-1]))
	buff[strlen(buff)-1] = 0;
      
      if ((*buff == '#') || (*buff == '+') || (*buff == 0))
	continue;
      
      for (ip = buff; *ip && !isspace(*ip); ip++);
      for(; *ip && isspace(*ip); ip++)
	*ip = 0;
      if (!*ip || parse_hex(buff, hwaddr, ETHER_ADDR_LEN, NULL, NULL) != ETHER_ADDR_LEN)
	{
	  my_syslog(MS_DHCP | LOG_ERR, format!("bad line at {} line {}"), ETHERSFILE, lineno); 
	  continue;
	}
      
      /* check for name or dotted-quad */
      for (cp = ip; *cp; cp++)
	if (!(*cp == '.' || (*cp >='0' && *cp <= '9')))
	  break;
      
      if (!*cp)
	{
	  if ((addr.s_addr = inet_addr(ip)) == (in_addr_t)-1)
	    {
	      my_syslog(MS_DHCP | LOG_ERR, format!("bad address at {} line {}"), ETHERSFILE, lineno); 
	      continue;
	    }

	  flags = CONFIG_ADDR;
	  
	  for (config = daemon.dhcp_conf; config; config = config.next)
	    if ((config.flags & CONFIG_ADDR) && config.addr.s_addr == addr.s_addr)
	      break;
	}
      else 
	{
	  let mut nomem: i32;
	  if (!(host = canonicalise(ip, &nomem)) || !legal_hostname(host))
	    {
	      if (!nomem)
		my_syslog(MS_DHCP | LOG_ERR, format!("bad name at {} line {}"), ETHERSFILE, lineno); 
	      free(host);
	      continue;
	    }
	      
	  flags = CONFIG_NAME;

	  for (config = daemon.dhcp_conf; config; config = config.next)
	    if ((config.flags & CONFIG_NAME) && hostname_isequal(config.hostname, host))
	      break;
	}

      if (config && (config.flags & CONFIG_FROM_ETHERS))
	{
	  my_syslog(MS_DHCP | LOG_ERR, format!("ignoring {} line {}, duplicate name or IP address"), ETHERSFILE, lineno); 
	  continue;
	}
	
      if (!config)
	{ 
	  for (config = daemon.dhcp_conf; config; config = config.next)
	    {
	      struct hwaddr_config *conf_addr = config.hwaddr;
	      if (conf_addr && 
		  conf_addr.next == NULL && 
		  conf_addr.wildcard_mask == 0 &&
		  conf_addr.hwaddr_len == ETHER_ADDR_LEN &&
		  (conf_addr.hwaddr_type == ARPHRD_ETHER || conf_addr.hwaddr_type == 0) &&
		  memcmp(conf_addr.hwaddr, hwaddr, ETHER_ADDR_LEN) == 0)
		break;
	    }
	  
	  if (!config)
	    {
	      if (!(config = whine_malloc(sizeof(struct dhcp_config))))
		continue;
	      config.flags = CONFIG_FROM_ETHERS;
	      config.hwaddr = NULL;
	      config.domain = NULL;
	      config.netid = NULL;
	      config.next = daemon.dhcp_conf;
	      daemon.dhcp_conf = config;
	    }
	  
	  config.flags |= flags;
	  
	  if (flags & CONFIG_NAME)
	    {
	      config.hostname = host;
	      host = NULL;
	    }
	  
	  if (flags & CONFIG_ADDR)
	    config.addr = addr;
	}
      
      config.flags |= CONFIG_NOCLID;
      if (!config.hwaddr)
	config.hwaddr = whine_malloc(sizeof(struct hwaddr_config));
      if (config.hwaddr)
	{
	  memcpy(config.hwaddr.hwaddr, hwaddr, ETHER_ADDR_LEN);
	  config.hwaddr.hwaddr_len = ETHER_ADDR_LEN;
	  config.hwaddr.hwaddr_type = ARPHRD_ETHER;
	  config.hwaddr.wildcard_mask = 0;
	  config.hwaddr.next = NULL;
	}
      count +=1;
      
      free(host);

    }
  
  fclose(f);

  my_syslog(MS_DHCP | LOG_INFO, format!("read {} - {} addresses"), ETHERSFILE, count);
}


/* If we've not found a hostname any other way, try and see if there's one in /etc/hosts
   for this address. If it has a domain part, that must match the set domain and
   it gets stripped. The set of legal domain names is bigger than the set of legal hostnames
   so check here that the domain name is legal as a hostname. 
   NOTE: we're only allowed to overwrite daemon.dhcp_buff if we succeed. */
char *host_from_dns(addr: net::IpAddr)
{
  let mut lookup: crec;

  if (daemon.port == 0)
    return NULL; /* DNS disabled. */
  
  lookup = cache_find_by_addr(NULL, (union all_addr *)&addr, 0, F_IPV4);

  if (lookup && (lookup.flags & F_HOSTS))
    {
      dot: &mut String, *hostname = cache_get_name(lookup);
      dot = strchr(hostname, '.');
      
      if (dot && strlen(dot+1) != 0)
	{
	  char *d2 = get_domain(addr);
	  if (!d2 || !hostname_isequal(dot+1, d2))
	    return NULL; /* wrong domain */
	}

      if (!legal_hostname(hostname))
	return NULL;
      
      safe_strncpy(daemon.dhcp_buff, hostname, 256);
      strip_hostname(daemon.dhcp_buff);

      return daemon.dhcp_buff;
    }
  
  return NULL;
}

 int  relay_upstream4(struct dhcp_relay *relay, struct dhcp_packet *mess, sz: usize, iface_index: i32)
{
  /* .local is same value for all relays on .current chain */
  union all_addr from;
  
  if (mess.op != BOOTREQUEST)
    return 0;

  /* source address == relay address */
  from.addr4 = relay.local.addr4;
  
  /* already gatewayed ? */
  if (mess.giaddr.s_addr)
    {
      /* if so check if by us, to stomp on loops. */
      if (mess.giaddr.s_addr == relay.local.addr4.s_addr)
	return 1;
    }
  else
    {
      /* plug in our address */
      mess.giaddr.s_addr = relay.local.addr4.s_addr;
    }

  if ((mess.hops++) > 20)
    return 1;

  for (; relay; relay = relay.current)
    {
      union mysockaddr to;
      
      to.sa.sa_family = AF_INET;
      to.in.sin_addr = relay.server.addr4;
      to.in.sin_port = htons(daemon.dhcp_server_port);
      
      send_from(daemon.dhcpfd, 0, mess, sz, &to, &from, 0);
      
      if (daemon.opt_log_opts)
	{
	  inet_ntop(AF_INET, &relay.local, daemon.addrbuff, ADDRSTRLEN);
	  my_syslog(MS_DHCP | LOG_INFO, format!("DHCP relay {} . {}"), daemon.addrbuff, inet_ntoa(relay.server.addr4));
	}
      
      /* Save this for replies */
      relay.iface_index = iface_index;
    }
  
  return 1;
}


 struct dhcp_relay *relay_reply4(struct dhcp_packet *mess, arrival_interface: &mut String)
{
  let mut relay: dhcp_relay;

  if (mess.giaddr.s_addr == 0 || mess.op != BOOTREPLY)
    return NULL;

  for (relay = daemon.relay4; relay; relay = relay.next)
    {
      if (mess.giaddr.s_addr == relay.local.addr4.s_addr)
	{
	  if (!relay.interface || wildcard_match(relay.interface, arrival_interface))
	    return relay.iface_index != 0 ? relay : NULL;
	}
    }
  
  return NULL;	 
}     


