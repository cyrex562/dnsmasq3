use std::{mem, net};

use winapi::{shared::ws2def::{AF_INET, AF_INET6, AF_LINK, AF_UNSPEC, SOCK_DGRAM}, um::winsock2::{PF_INET6, htons}};

use crate::{dns_protocol::IN6ADDRSZ, dnsmasq_h::{AF_LOCAL, Ip4Header, rt_msghdr}};

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



// #if defined() || defined(HAVE_SOLARIS_NETWORK)
// #include <ifaddrs.h>

// #include <sys/param.h>
// #if defined() && !defined(__APPLE__)
// #include <sys/sysctl.h>

// #include <net/if.h>
// #include <net/route.h>
// #include <net/if_dl.h>
// #include <netinet/if_ether.h>
// #if defined(__FreeBSD__)
// #  include <net/if_var.h> 

// #include <netinet/in_var.h>
// #include <netinet6/in6_var.h>


// #define SA_SIZE(sa)                                             \
//     (  (!(sa) || ((sa)).sa_len == 0) ?      \
//         sizeof(long)            :                               \
//         1 + ( (((sa)).sa_len - 1) | (sizeof(long) - 1) ) )


 
// let mut del_family: i32 = 0;
//  union all_addr del_addr;




pub fn arp_enumerate(parm: &mut Vec<u8>, callback: fn()->i32) -> i32
{
  // int mib[6];
  let mut mib: [i32;6];
  let mut needed: usize;
  //char *next;
  let next: String;
  let mut rtm: rt_msghdr;
  // let mut sin2: sockaddr_inarp;
  // let mut sdl: sockaddr_dl;
  // let mut buff: iovec;
  let mut rc: i32;

  // buff.iov_base = NULL;
  // buff.iov_len = 0;

  // mib[0] = CTL_NET;
  // mib[1] = PF_ROUTE;
  // mib[2] = 0;
  // mib[3] = AF_INET;
  // mib[4] = NET_RT_FLAGS;
  // mib[5] = 0;
	
  // if (sysctl(mib, 6, NULL, &needed, NULL, 0) == -1 || needed == 0) {
  //   return 0;
  // }

// loop
//     {
  //     if (!expand_buf(&buff, needed)) {
	// return 0;
  //     }
  //     if ((rc = sysctl(mib, 6, buff.iov_base, &needed, NULL, 0)) == 0 ||
	//   errno != ENOMEM) {
	// break;
  //   }
    //   needed += needed / 8;
    // }
  if rc == -1 {
    return 0;
  }
  
  // for (next = buff.iov_base ; next < buff.iov_base + needed; next += rtm.rtm_msglen)
  // next = buff.iov_base;
  // while next < buff.iov_base   
  // {
  //     rtm = next;
  //     sin2 = (rtm + 1);
  //     sdl = (sin2 + SA_SIZE(sin2));
  //     if (!(*callback)(AF_INET, &sin2.sin_addr, LLADDR(sdl), sdl.sdl_alen, parm)) {
	// return 0;}
  //       next += rtm.rtm_msglen;
  //   }

  return 1;
}
 /* defined() && !defined(__APPLE__) */


pub fn iface_enumerate(family: i32, par: Option<Vec<u8>>, callback: fn()->i32) -> i32
{
  // struct ifaddrs *head, *addrs;
//  let mut head: ifaddrs;
//  let mut addrs: ifaddrs;
  // errsave: i32, fd = -1, ret = 0;
  let mut errsave: i32;
  let mut fd: i32 = -1;
  let mut ret: i32 = 0;

  if family == AF_UNSPEC {
    return  arp_enumerate(parm, callback);}

  //return 0;


  /* AF_LINK doesn't exist in Linux, so we can't use it in our API */
  if family == AF_LOCAL {
    family = AF_LINK;
  }

  if getifaddrs(&head) == -1 {
    return 0;
  }

  if family == AF_INET6 {
    fd = socket(PF_INET6, SOCK_DGRAM, 0);
  }

  
  // for (addrs = head; addrs; addrs = addrs.ifa_next)
  for addrs in head
    {
      if addrs.ifa_addr.sa_family == family
	{
	  let iface_index = if_nametoindex(addrs.ifa_name);
	  if iface_index == 0 || !addrs.ifa_addr || 
	      (!addrs.ifa_netmask && family != AF_LINK) {
	    continue;
        }

	  if family == AF_INET
	    {
	      // addr: net::IpAddr, netmask, broadcast;
        let mut netmask: net::IpAddr;
        let mut broadcast: net::IpAddr;
	      addr = ( addrs.ifa_addr).sin_addr;
	      if del_family == AF_INET && del_addr.addr4.s_addr == addr.s_addr {
		continue;}

	      netmask = ( addrs.ifa_netmask).sin_addr;
	      if (addrs.ifa_broadaddr) {
		broadcast = ( addrs.ifa_broadaddr).sin_addr; 
        }
	      else 
        {
		broadcast.s_addr = 0;	
        }      
	      if (!((*callback)(addr, iface_index, NULL, netmask, broadcast, parm)))
        {
		//goto err;
        }
	    }
	  else if (family == AF_INET6)
	    {
	      let mut addr: net::IpAddr = addrs.ifa_addr;
	      let netmask =  addrs.ifa_netmask;
	      let scope_id = ( addrs.ifa_addr).sin6_scope_id;
	      // i: i32, j, prefix = 0;
	      let mut i: i32;
        let mut j: i32;
        let mut prefix: i32;
        let mut valid: u32 = 0xffffffff;
        let mut preferred: u32 = 0xffffffff;
	      let mut flags: i32 = 0;
 
	      if (del_family == AF_INET6 && IN6_ARE_ADDR_EQUAL(&del_addr.addr6, addr)) {
		continue;}

	      // struct in6_ifreq ifr6;
          let mut ifr6: in6_ifreq;

	      // memset(&ifr6, 0, sizeof(ifr6));
	      // safe_strncpy(ifr6.ifr_name, addrs.ifa_name, sizeof(ifr6.ifr_name));
	      ifr6.ifr_name = addrs.ifa_name;
	      ifr6.ifr_addr = addrs.ifa_addr;
	      if (fd != -1 && ioctl(fd, SIOCGIFAFLAG_IN6, &ifr6) != -1)
		{
		  if (ifr6.ifr_ifru.ifru_flags6 & IN6_IFF_TENTATIVE) {
		    flags |= IFACE_TENTATIVE;
      }
		  
		  if (ifr6.ifr_ifru.ifru_flags6 & IN6_IFF_DEPRECATED) {
		    flags |= IFACE_DEPRECATED;
      }


		  if (!(ifr6.ifr_ifru.ifru_flags6 & (IN6_IFF_AUTOCONF | IN6_IFF_TEMPORARY))) {
		    flags |= IFACE_PERMANENT;
      }



		  if (!(ifr6.ifr_ifru.ifru_flags6 & (IN6_IFF_AUTOCONF | IN6_IFF_PRIVACY))) {
		    flags |= IFACE_PERMANENT;
      }

		}
	      
	      ifr6.ifr_addr = *( addrs.ifa_addr); 
	      if (fd != -1 && ioctl(fd, SIOCGIFALIFETIME_IN6, &ifr6) != -1)
		{
		  valid = ifr6.ifr_ifru.ifru_lifetime.ia6t_vltime;
		  preferred = ifr6.ifr_ifru.ifru_lifetime.ia6t_pltime;
		}

	      	      
	      // for (i = 0; i < IN6ADDRSZ; i++, prefix += 8)  
        let mut i = 0;
        while i < IN6ADDRSZ
        {
                if (netmask[i] != 0xff) {
                  break;
                }
                prefix += 8;
                i += 1;
        }

		  
	      
	      if (i != IN6ADDRSZ && netmask[i])  {
            let mut j = 7;
                // for (j = 7; j > 0; j--, prefix++) 
                while j > 0
                {
		  if ((netmask[i] & (1 << j)) == 0) {
		    break;
      }
      j -=1 ;
      prefix += 1;
    }
  }
	      
	      /* voodoo to clear interface field in address */
	      if (!daemon.opt_nowild && IN6_IS_ADDR_LINKLOCAL(addr))
		{
		  addr.s6_addr[2] = 0;
		  addr.s6_addr[3] = 0;
		} 
	     
	      if (!((*callback)(addr, prefix, scope_id, iface_index, flags,
				 preferred, valid, parm))) {
		// goto err;	      
         }
	    }

       
	  else if (family == AF_LINK)
	    { 
	      /* Assume ethernet again here */
	      let sdl: sockaddr_dl =  addrs.ifa_addr;
	      if (sdl.sdl_alen != 0 && 
		  !((*callback)(iface_index, ARPHRD_ETHER, LLADDR(sdl), sdl.sdl_alen, parm))) {
		// goto err;
      }
	    }
 
	}
    }
  
  ret = 1;

//  err:
  errsave = errno;
  freeifaddrs(head); 
  if (fd != -1) {
    close(fd);
  }
  errno = errsave;

  return ret;
}

pub fn init_bpf()
{
  let mut i: i32 = 0;

  while (1) 
    {
      // sprintf(daemon.dhcp_buff, "/dev/bpf{}", i++);
      daemon.dhcp_buff = format!("/dev/bpf/{}", i);
      i += 1;
      if ((daemon.dhcp_raw_fd = open(daemon.dhcp_buff, O_RDWR, 0)) != -1) {
      	return;
      }

      if (errno != EBUSY) {
      	panic!(format!("cannot create DHCP BPF socket"));
      }
    }	     
}

pub fn send_via_bpf(
  daemon: &mut DnsmasqDaemon, 
  mess: &mut dhcp_packet, 
  len: usize, 
  iface_addr: net::IpAddr, 
  ifr: ifreq)
{
   /* Hairy stuff, packet either has to go to the
      net broadcast or the destination can't reply to ARP yet,
      but we do know the physical address. 
      Build the packet by steam, and send directly, bypassing
      the kernel IP stack */
  
  // struct ether_header ether; 
  let mut ether: ether_header;
  // struct ip ip;
  let mut ip: ip;
  
 
  
  // u32 i, sum;
  let mut i: u32;
  let mut sum: u32;
  // struct iovec iov[4];
  let mut iov: [iovec;4];

  /* Only know how to do ethernet on *BSD */
  if (mess.htype != ARPHRD_ETHER || mess.hlen != ETHER_ADDR_LEN)
    {
      my_syslog(MS_DHCP | LOG_WARNING, format!("DHCP request for unsupported hardware type ({}) received on {}"), 
		mess.htype, ifr.ifr_name);
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
  todo!();
  // ip.ip_len = htons(sizeof(struct ip) +  sizeof(struct udphdr) + len) ;
  todo!();
  //ip.ip_hl = sizeof(struct ip) / 4;
  ip.ip_v = IPVERSION;
  ip.ip_tos = 0;
  ip.ip_id = htons(0);
  ip.ip_off = htons(0x4000); /* don't fragment */
  ip.ip_ttl = IPDEFTTL;
  ip.ip_sum = 0;
  let mut sum = 0;
  // for (sum = 0, i = 0; i < mem::sizeof<ip>() / 2; i++) 
  for i in 0..((mem::size_of::<Ip4Header4>())/2)
  {
    sum += (&ip)[i];
  }
  while (sum>>16) {
    sum = (sum & 0xffff) + (sum >> 16);  
  }

  if sum == 0xffff {
    ip.ip_sum = sum;
  } else {
    ip.ip_sum = !sum;
  }

  udp.uh_sport = daemon.dhcp_server_port;
  udp.uh_dport = daemon.dhcp_client_port;
  if (len & 1) {
    (mess)[len] = 0; /* for checksum, in case length is odd. */
  }
  udp.uh_sum = 0;
  udp.uh_ulen = sum = mem::sizeof<udphdr>() + len;
  sum += htons(IPPROTO_UDP);
  sum += ip.ip_src.s_addr & 0xffff;
  sum += (ip.ip_src.s_addr >> 16) & 0xffff;
  sum += ip.ip_dst.s_addr & 0xffff;
  sum += (ip.ip_dst.s_addr >> 16) & 0xffff;
  for (i = 0; i < sizeof(struct udphdr)/2; i++)
    sum += (&udp)[i];
  for (i = 0; i < (len + 1) / 2; i++)
    sum += (mess)[i];
  while (sum>>16)
    sum = (sum & 0xffff) + (sum >> 16);
  udp.uh_sum = (sum == 0xffff) ? sum : ~sum;
  
  ioctl(daemon.dhcp_raw_fd, BIOCSETIF, ifr);
  
  iov[0].iov_base = &ether;
  iov[0].iov_len = sizeof(ether);
  iov[1].iov_base = &ip;
  iov[1].iov_len = sizeof(ip);
  iov[2].iov_base = &udp;
  iov[2].iov_len = sizeof(udp);
  iov[3].iov_base = mess;
  iov[3].iov_len = len;

  while (retry_send(writev(daemon.dhcp_raw_fd, iov, 4)));
}

 /* defined() && defined() */
 

 

pub fn route_init()
{
  /* AF_UNSPEC: all addr families */
  daemon.routefd = socket(PF_ROUTE, SOCK_RAW, AF_UNSPEC);
  
  if (daemon.routefd == -1 || !fix_fd(daemon.routefd))
    die(format!("cannot create PF_ROUTE socket: {}"), NULL, EC_BADNET);
}

pub fn route_sock()
{
  let mut msg: if_msghdr;
  int rc = recv(daemon.routefd, daemon.packet, daemon.packet_buff_sz, 0);

  if (rc < 4)
    {return;}

  msg = (struct if_msghdr *)daemon.packet;
  
  if (rc < msg.ifm_msglen)
    {return;}

   if (msg.ifm_version != RTM_VERSION)
     {
       let mut warned: i32 = 0;
       if (!warned)
	 {
	   my_syslog(LOG_WARNING, format!("Unknown protocol version from route socket"));
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
	  in a  variable, and omit it from the set returned by iface_enumerate() */
       int mask = ((struct ifa_msghdr *)msg).ifam_addrs;
       int maskvec[] = { RTA_DST, RTA_GATEWAY, RTA_NETMASK, RTA_GENMASK,
			 RTA_IFP, RTA_IFA, RTA_AUTHOR, RTA_BRD };
       let mut of: i32;
       let mut i: u32;
       
       for (i = 0,  of = sizeof(struct ifa_msghdr); of < rc && i < sizeof(maskvec)/sizeof(maskvec[0]); i++) 
	 if (mask & maskvec[i]) 
	   {
	     struct sockaddr *sa = (msg + of);
	     diff: usize = (sa.sa_len != 0) ? sa.sa_len : sizeof(long);
	     
	     if (maskvec[i] == RTA_IFA)
	       {
		 del_family = sa.sa_family;
		 if (del_family == AF_INET)
		   del_addr.addr4 = (sa).sin_addr;
		 else if (del_family == AF_INET6)
		   del_addr.addr6 = (sa).sin6_addr;
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

 /*  */


