use libc::c_void;

use crate::{arp::AddressPointer, dnsmasq_sys::{Sockaddr, pid_t}};

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

// //#include "dnsmasq.h"

// #if defined(HAVE_BSD_NETWORK) || defined(HAVE_SOLARIS_NETWORK)
// //#include <ifaddrs.h>

// //#include <sys/param.h>
// #if defined(HAVE_BSD_NETWORK) && !defined(__APPLE__)
// //#include <sys/sysctl.h>
// //#endif
// //#include <net/if.h>
// //#include <net/route.h>
// //#include <net/if_dl.h>
// //#include <netinet/if_ether.h>
// #if defined(__FreeBSD__)
// #  include <net/if_var.h> 
// //#endif
// //#include <netinet/in_var.h>
// //#ifdef HAVE_IPV6
// #  include <netinet6/in6_var.h>
// //#endif

// #ifndef SA_SIZE
// #define SA_SIZE(sa)                                             \
//     (  (!(sa) || ((struct sockaddr *)(sa))->sa_len == 0) ?      \
//         sizeof(long)            :                               \
//         1 + ( (((struct sockaddr *)(sa))->sa_len - 1) | (sizeof(long) - 1) ) )
// //#endif

// https://www.freebsd.org/cgi/man.cgi?query=route&sektion=4&manpath=netbsd
pub struct rt_metrics {
  pub rmx_locks: u32, // kernel must leave these values alone
  pub rmx_mut: u32, // MTU for this path
  pub rmx_hopcount: u32, // max hops expected
  pub rmx_expire: u32, // lifetime for route
  pub rmx_recvpipe: u32, // inbound delay bandwidth product
  pub rmx_sendpipe: u32, // outbound delay bandwidth product
  pub rmx_ssthresh: u32, // outbound gateway buffer limit
  pub rmx_rtt: u32, // estimated rtt
  pub rmx_rttvar: u32, // estimated rtt variance
  pub rmx_pksent: u32, // packets sent using this route
}

pub struct rt_msghdr {
  pub rtm_msglen: u16, // to skip over non-understood messages
  pub rtm_version: u8, // future binary compatibility
  pub rtm_type: u8, // message type
  pub rtm_index: u16, // index for associated ifp
  pub rtm_flags: i32, // flags, incl kern & message
  pub rtm_addrs: u32, // bitmask identifying sockaddrs in msg
  pub rtm_pid: pid_t, // identify sender
  pub rtm_seq: i32, // for sender to identify action
  pub rtm_errno: i32, // why failed
  pub rtm_use: i32, // from rtentry
  pub rtm_inits: u32, // which metrics we are initializing
  pub rtm_max: rt_metrics // metrics
}

pub fn sa_size(sa: &Sockaddr) -> usize {
  if sa.sa_len == 0 {
    return 4;
  } else {
    return (1 + sa.sa_len - 1) | (4 - 1);
  }
}

// //#ifdef HAVE_BSD_NETWORK
static del_family: i32 = 0;
static del_addr: Vec<AddressPointer> = Vec::new();
// //#endif

// #if defined(HAVE_BSD_NETWORK) && !defined(__APPLE__)

fn arp_enumerate(parm: *mut c_void, callback: fn()->i32) -> i32
{
  let mut mib: [i32;6];
  let mut needed: usize;
  let mut rtm: rt_msghdr;
  // char *next;
  let mut sin2: sockaddr_inarp;
  let mut sdl: sockaddr_dl;
  let mut buf: iovec;
  let mut rc: i32;

  // buff.iov_base = nullptr;
  buff.iov_len = 0;

  mib[0] = CTL_NET;
  mib[1] = PF_ROUTE;
  mib[2] = 0;
  mib[3] = AF_INET;
  mib[4] = NET_RT_FLAGS;
  mib[5] = RTF_LLINFO;
  if (sysctl(mib, 6, nullptr, &needed, nullptr, 0) == -1 || needed == 0) {
    return 0;
  }

  while (1) 
    {
      if (!expand_buf(&buff, needed)) {
        return 0;
      }
      
      if ((rc = sysctl(mib, 6, buff.iov_base, &needed, nullptr, 0)) == 0 ||
	  errno != ENOMEM) {
      break;
    }
      needed += needed / 8;
    }
  if (rc == -1) {
    return 0;
  }

  // TODO: re-implement
  // for (next = buff.iov_base ; next < (char *)buff.iov_base + needed; next += rtm->rtm_msglen)
  //   {
  //     rtm = (struct rt_msghdr *)next;
  //     sin2 = (struct sockaddr_inarp *)(rtm + 1);
  //     sdl = (struct sockaddr_dl *)((char *)sin2 + SA_SIZE(sin2));
  //     if (!(*callback)(AF_INET, &sin2->sin_addr, LLADDR(sdl), sdl->sdl_alen, parm))
	// return 0;
  //   }

  return 1;
}
// //#endif /* defined(HAVE_BSD_NETWORK) && !defined(__APPLE__) */


pub fn iface_enumerate(family: i32, parm: Option<*mut c_void>, callback: fn()) -> i32
{
  // struct ifaddrs *head, *addrs;
  let mut addrs: if_addrs;
  let mut head: if_addrs;
  let mut errsave: i32 = -1;
  let mut fd: i32 = -1;
  let mut ret: i32 = 0;

  if (family == AF_UNSPEC) {
    return  arp_enumerate(parm, callback);
  }
    
  /* AF_LINK doesn't exist in Linux, so we can't use it in our API */
  if (family == AF_LOCAL) {
    family = AF_LINK;
  }
    

  if (getifaddrs(&head) == -1) {
    return 0;
  }

  if (family == AF_INET6) {
    fd = socket(PF_INET6, SOCK_DGRAM, 0);
  }

  // TODO
  // for (addrs = head; addrs; addrs = addrs->ifa_next)
  //   {
  //     if (addrs->ifa_addr->sa_family == family)
	// {
	//   int iface_index = if_nametoindex(addrs->ifa_name);

	//   if (iface_index == 0 || !addrs->ifa_addr || 
	//       (!addrs->ifa_netmask && family != AF_LINK))
	//     continue;

	//   if (family == AF_INET)
	//     {
	//       struct in_addr addr, netmask, broadcast;
	//       addr = ((struct sockaddr_in *) addrs->ifa_addr)->sin_addr;

	//       if (del_family == AF_INET && del_addr.addr.addr4.s_addr == addr.s_addr)
	// 	continue;
	//       netmask = ((struct sockaddr_in *) addrs->ifa_netmask)->sin_addr;
	//       if (addrs->ifa_broadaddr)
	// 	broadcast = ((struct sockaddr_in *) addrs->ifa_broadaddr)->sin_addr; 
	//       else 
	// 	broadcast.s_addr = 0;	      
	//       if (!((*callback)(addr, iface_index, nullptr, netmask, broadcast, parm)))
	// 	goto err;
	//     }
	//   else if (family == AF_INET6)
	//     {
	//       struct in6_addr *addr = &((struct sockaddr_in6 *) addrs->ifa_addr)->sin6_addr;
	//       unsigned char *netmask = (unsigned char *) &((struct sockaddr_in6 *) addrs->ifa_netmask)->sin6_addr;
	//       int scope_id = ((struct sockaddr_in6 *) addrs->ifa_addr)->sin6_scope_id;
	//       int i, j, prefix = 0;
	//       uint32_t valid = 0xffffffff, preferred = 0xffffffff;
	//       int flags = 0;
	//       if (del_family == AF_INET6 && IN6_ARE_ADDR_EQUAL(&del_addr.addr.addr6, addr))
	// 	continue;
	//       struct in6_ifreq ifr6;

	//       memset(&ifr6, 0, sizeof(ifr6));
	//       safe_strncpy(ifr6.ifr_name, addrs->ifa_name, sizeof(ifr6.ifr_name));
	      
	//       ifr6.ifr_addr = *((struct sockaddr_in6 *) addrs->ifa_addr);
	//       if (fd != -1 && ioctl(fd, SIOCGIFAFLAG_IN6, &ifr6) != -1)
	// 	{
	// 	  if (ifr6.ifr_ifru.ifru_flags6 & IN6_IFF_TENTATIVE)
	// 	    flags |= IFACE_TENTATIVE;
		  
	// 	  if (ifr6.ifr_ifru.ifru_flags6 & IN6_IFF_DEPRECATED)
	// 	    flags |= IFACE_DEPRECATED;

	// 	  if (!(ifr6.ifr_ifru.ifru_flags6 & (IN6_IFF_AUTOCONF | IN6_IFF_TEMPORARY)))
	// 	    flags |= IFACE_PERMANENT;

	// 	  if (!(ifr6.ifr_ifru.ifru_flags6 & (IN6_IFF_AUTOCONF | IN6_IFF_PRIVACY)))
	// 	    flags |= IFACE_PERMANENT;
	// 	}
	      
	//       ifr6.ifr_addr = *((struct sockaddr_in6 *) addrs->ifa_addr);
	//       if (fd != -1 && ioctl(fd, SIOCGIFALIFETIME_IN6, &ifr6) != -1)
	// 	{
	// 	  valid = ifr6.ifr_ifru.ifru_lifetime.ia6t_vltime;
	// 	  preferred = ifr6.ifr_ifru.ifru_lifetime.ia6t_pltime;
	// 	}	      	      
	//       for (i = 0; i < IN6ADDRSZ; i++, prefix += 8) 
  //               if (netmask[i] != 0xff)
	// 	  break;
	      
	//       if (i != IN6ADDRSZ && netmask[i]) 
  //               for (j = 7; j > 0; j--, prefix++) 
	// 	  if ((netmask[i] & (1 << j)) == 0)
	// 	    break;
	      
	//       /* voodoo to clear interface field in address */
	//       if (!option_bool(OPT_NOWILD) && IN6_IS_ADDR_LINKLOCAL(addr))
	// 	{
	// 	  addr->s6_addr[2] = 0;
	// 	  addr->s6_addr[3] = 0;
	// 	} 
	     
	//       if (!((*callback)(addr, prefix, scope_id, iface_index, flags,
	// 			(int) preferred, (int)valid, parm)))
	// 	goto err;	      
	//     }
	//   else if (family == AF_LINK)
	//     { 
	//       /* Assume ethernet again here */
	//       struct sockaddr_dl *sdl = (struct sockaddr_dl *) addrs->ifa_addr;
	//       if (sdl->sdl_alen != 0 && 
	// 	  !((*callback)(iface_index, ARPHRD_ETHER, LLADDR(sdl), sdl->sdl_alen, parm)))
	// 	goto err;
	//     }

	// }
  //   }
  
  ret = 1;

  errsave = errno;
  freeifaddrs(head); 
  // if (fd != -1)
  //   close(fd);
  errno = errsave;

  return ret;
}

pub fn init_bpf(daemon: daemon)
{
  let mut i = 0;
  // TODO:
  // while (1) 
  //   {

  // //     sprintf(daemon.dhcp_buff, "/dev/bpf%d", i++);
  // //     if ((daemon->dhcp_raw_fd = open(daemon->dhcp_buff, O_RDWR, 0)) != -1)
	// // return;

  // //     if (errno != EBUSY)
	// // die(_("cannot create DHCP BPF socket: %s"), nullptr, EC_BADNET);
  // //   }	     
}

pub struct udphdr {
  pub uh_sport: u16,               /* source port */
  pub uh_dport: u16,               /* destination port */
  pub uh_ulen: u16,                /* udp length */
  pub uh_sum: u16,                 /* udp checksum */
}

pub fn send_via_bpf(mess: &mut dhcp_packet, len: usize,
		  iface_addr: in_addr, ifr: &mut ifreq)
{
   /* Hairy stuff, packet either has to go to the
      net broadcast or the destination can't reply to ARP yet,
      but we do know the physical address. 
      Build the packet by steam, and send directly, bypassing
      the kernel IP stack */
  let mut ether: ether_header;
  let mut ip: ip;
  let mut sum: u32;
  let mut i: u32;
  let mut iov: [iovec;4];
  /* Only know how to do ethernet on *BSD */
  if (mess.htype != ARPHRD_ETHER || mess.hlen != ETHER_ADDR_LEN)
    {
    //   my_syslog(MS_DHCP | LOG_WARNING, _("DHCP request for unsupported hardware type (%d) received on %s"), 
		// mess.htype, ifr.ifr_name);
      return;
    }
   
  ifr.ifr_addr.sa_family = AF_LINK;
  if (ioctl(daemon.dhcpfd, SIOCGIFADDR, ifr) < 0) {
    return;
  }
  
  memcpy(ether.ether_shost, LLADDR(&ifr.ifr_addr), ETHER_ADDR_LEN);
  ether.ether_type = htons(ETHERTYPE_IP);
  
  if (ntohs(mess.flags) & 0x8000)
    {
      memset(ether.ether_dhost, 255,  ETHER_ADDR_LEN);
      ip.ip_dst.s_addr = INADDR_BROADCAST;
    }
  else
    {
      memcpy(ether.ether_dhost, mess.chaddr, ETHER_ADDR_LEN); 
      ip.ip_dst.s_addr = mess.yiaddr.s_addr;
    }
  
  ip.ip_p = IPPROTO_UDP;
  ip.ip_src.s_addr = iface_addr.s_addr;
  ip.ip_len = IP_HDR_LEN + UDP_HDR_LEN + len;
  ip.ip_hl = IP_HDR_LEN / 4;
  ip.ip_v = IPVERSION;
  ip.ip_tos = 0;
  ip.ip_id = htons(0);
  ip.ip_off = htons(0x4000); /* don't fragment */
  ip.ip_ttl = IPDEFTTL;
  ip.ip_sum = 0;
  // TODO: calculate IP CHECKSUM
  // let mut sum = 0;
  // for i in 0..(IP_HDR_LEN / 2) {
  //   sum += ip[i];
  // }
  // while (sum>>16) {
  //   sum = (sum & 0xffff) + (sum >> 16);  
  // }

  // if sum == 0xffff {
  //   ip.ip_sum = sum;
  // } else {
  //   ip.ip_sum = ~sum;
  // }
  
  udp.uh_sport = htons(daemon.dhcp_server_port);
  udp.uh_dport = htons(daemon.dhcp_client_port);
  // if (len & 1)
  //   (mess)[len] = 0; /* for checksum, in case length is odd. */
  udp.uh_sum = 0;
  udp.uh_ulen = sum = htons(UDP_HDR_LEN + len);
  // TODO: calc UDP checksum
  // sum += htons(IPPROTO_UDP);
  // sum += ip.ip_src.s_addr & 0xffff;
  // sum += (ip.ip_src.s_addr >> 16) & 0xffff;
  // sum += ip.ip_dst.s_addr & 0xffff;
  // sum += (ip.ip_dst.s_addr >> 16) & 0xffff;
  // for (i = 0; i < sizeof(struct udphdr)/2; i++)
  //   sum += ((uint16_t *)&udp)[i];
  // for (i = 0; i < (len + 1) / 2; i++)
  //   sum += ((uint16_t *)mess)[i];
  // while (sum>>16)
  //   sum = (sum & 0xffff) + (sum >> 16);
  // udp.uh_sum = (sum == 0xffff) ? sum : ~sum;
  
  // TODO:
  // ioctl(daemon->dhcp_raw_fd, BIOCSETIF, ifr);
  
  iov[0].iov_base = &ether;
  iov[0].iov_len = sizeof(ether);
  iov[1].iov_base = &ip;
  iov[1].iov_len = sizeof(ip);
  iov[2].iov_base = &udp;
  iov[2].iov_len = sizeof(udp);
  iov[3].iov_base = mess;
  iov[3].iov_len = len;

  // while (retry_send(writev(daemon.dhcp_raw_fd, iov, 4)));
  loop {
    let result = retry_send(writev(daemon.dhcp_raw_fd, iov, 4));
    if !result {
      break;
    }
  }
}


pub fn route_init()
{
  /* AF_UNSPEC: all addr families */
  daemon.routefd = socket(PF_ROUTE, SOCK_RAW, AF_UNSPEC);
  
  if (daemon.routefd == -1 || !fix_fd(daemon.routefd)) {
    return;
  }
    // die(_("cannot create PF_ROUTE socket: %s"), nullptr, EC_BADNET);
}

pub fn route_sock()
{
  // struct if_msghdr *msg;
  let mut msg: if_msghdr;

  let rc = recv(daemon.routefd, daemon.packet, daemon.packet_buff_sz, 0);

  if (rc < 4) {
    return;
  }

  msg = daemon.packet;
  
  if (rc < msg.ifm_msglen) {
    return;
  }

   if (msg.ifm_version != RTM_VERSION)
     {
       let mut warned = 0;
       if (!warned)
	 {
	  //  my_syslog(LOG_WARNING, _("Unknown protocol version from route socket"));
	   warned = 1;
	 }
     }
   else if (msg.ifm_type == RTM_NEWADDR)
     {
       del_family = 0;
       queue_event(EVENT_NEWADDR);
     }
   else if (msg.ifm_type == RTM_DELADDR)
     {
       /* There's a race in the kernel, such that if we run iface_enumerate() immediately
	  we get a DELADDR event, the deleted address still appears. Here we store the deleted address
	  in a static variable, and omit it from the set returned by iface_enumerate() */
       let mask = msg.ifam_addrs;
       let maskvec: [] = [RTA_DST, RTA_GATEWAY, RTA_NETMASK, RTA_GENMASK,
			 RTA_IFP, RTA_IFA, RTA_AUTHOR, RTA_BRD ];
       let mut of: i32;
       let mut i: u32;
       
        for i in 

       for (i = 0,  of = sizeof(struct ifa_msghdr); of < rc && i < sizeof(maskvec)/sizeof(maskvec[0]); i++) 
	 if (mask & maskvec[i]) 
	   {
	     struct sockaddr *sa = (struct sockaddr *)((char *)msg + of);
	     size_t diff = (sa->sa_len != 0) ? sa->sa_len : sizeof(long);
	     
	     if (maskvec[i] == RTA_IFA)
	       {
		 del_family = sa->sa_family;
		 if (del_family == AF_INET)
		   del_addr.addr.addr4 = ((struct sockaddr_in *)sa)->sin_addr;
		 else if (del_family == AF_INET6)
		   del_addr.addr.addr6 = ((struct sockaddr_in6 *)sa)->sin6_addr;

		 else
		   del_family = 0;
	       }
	     
	     of += diff;
	     /* round up as needed */
	     if (diff & (sizeof(long) - 1)) 
	       of += sizeof(long) - (diff & (sizeof(long) - 1));
	   }
       
       queue_event(EVENT_NEWADDR);
     }
}



