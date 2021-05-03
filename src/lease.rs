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



 

 struct dhcp_lease *leases = NULL, *old_leases = NULL;
 dns_dirty: i32, file_dirty, leases_left;

 int read_leases(now: time::Instant, FILE *leasestream)
{
  unsigned let ei: i32;
  union all_addr addr;
  let mut lease: dhcp_lease;
  clid_len: i32, hw_len, hw_type;
  let mut items: i32;
  char *domain = NULL;

  *daemon.dhcp_buff3 = *daemon.dhcp_buff2 = '\0';

  /* client-id max length is 255 which is 255*2 digits + 254 colons
     borrow DNS packet buffer which is always larger than 1000 bytes

     Check various buffers are big enough for the code below */

#if (DHCP_BUFF_SZ < 255) || (MAXDNAME < 64) || (PACKETSZ+MAXDNAME+RRFIXEDSZ  < 764)
# error Buffer size breakage in leasefile parsing.


    while ((items=fscanf(leasestream, "%255s %255s", daemon.dhcp_buff3, daemon.dhcp_buff2)) == 2)
      {
	*daemon.namebuff = *daemon.dhcp_buff = *daemon.packet = '\0';
	hw_len = hw_type = clid_len = 0;
	
 
	if (strcmp(daemon.dhcp_buff3, "duid") == 0)
	  {
	    daemon.duid_len = parse_hex(daemon.dhcp_buff2, daemon.dhcp_buff2, 130, NULL, NULL);
	    if (daemon.duid_len < 0)
	      return 0;
	    daemon.duid = safe_malloc(daemon.duid_len);
	    memcpy(daemon.duid, daemon.dhcp_buff2, daemon.duid_len);
	    continue;
	  }

	
	if (fscanf(leasestream, " %64s %255s %764s",
		   daemon.namebuff, daemon.dhcp_buff, daemon.packet) != 3)
	  {
	    my_syslog(MS_DHCP | LOG_WARNING, format!("ignoring invalid line in lease database: {} {} {} {} ..."),
		      daemon.dhcp_buff3, daemon.dhcp_buff2,
		      daemon.namebuff, daemon.dhcp_buff);
	    continue;
	  }
		
	if (inet_pton(AF_INET, daemon.namebuff, &addr.addr4))
	  {
	    if ((lease = lease4_allocate(addr.addr4)))
	      domain = get_domain(lease.addr);
	    
	    hw_len = parse_hex(daemon.dhcp_buff2, daemon.dhcp_buff2, DHCP_CHADDR_MAX, NULL, &hw_type);
	    /* For backwards compatibility, no explicit MAC address type means ether. */
	    if (hw_type == 0 && hw_len != 0)
	      hw_type = ARPHRD_ETHER; 
	  }
 
	else if (inet_pton(AF_INET6, daemon.namebuff, &addr.addr6))
	  {
	    char *s = daemon.dhcp_buff2;
	    int lease_type = LEASE_NA;

	    if (s[0] == 'T')
	      {
		lease_type = LEASE_TA;
		s +=1;
	      }
	    
	    if ((lease = lease6_allocate(&addr.addr6, lease_type)))
	      {
		lease_set_iaid(lease, strtoul(s, NULL, 10));
		domain = get_domain6(&lease.addr6);
	      }
	  }

	else
	  {
	    my_syslog(MS_DHCP | LOG_WARNING, format!("ignoring invalid line in lease database, bad address: {}"),
		      daemon.namebuff);
	    continue;
	  }
	

	if (!lease)
	  die (format!("too many stored leases"), NULL, EC_MISC);

	if (strcmp(daemon.packet, "*") != 0)
	  clid_len = parse_hex(daemon.packet, daemon.packet, 255, NULL, NULL);
	
	lease_set_hwaddr(lease, daemon.dhcp_buff2, daemon.packet, 
			 hw_len, hw_type, clid_len, now, 0);
	
	if (strcmp(daemon.dhcp_buff, "*") !=  0)
	  lease_set_hostname(lease, daemon.dhcp_buff, 0, domain, NULL);

	ei = atol(daemon.dhcp_buff3);


	if (ei != 0)
	  lease.expires = (time_t)ei + now;
	else
	  lease.expires = (time_t)0;
	lease.length = ei;

	/* strictly is: time::Instant opaque, but this hack should work on all sane systems,
	   even when sizeof(time_t) == 8 */
	lease.expires = (time_t)ei;

	
	/* set these correctly: the "old" events are generated later from
	   the startup synthesised SIGHUP. */
	lease.flags &= ~(LEASE_NEW | LEASE_CHANGED);
	
	*daemon.dhcp_buff3 = *daemon.dhcp_buff2 = '\0';
      }
    
    return (items == 0 || items == EOF);
}

void lease_init(now: time::Instant)
{
  FILE *leasestream;

  leases_left = daemon.dhcp_max;

  if (option_bool(OPT_LEASE_RO))
    {
      /* run "<lease_change_script> init" once to get the
	 initial state of the database. If leasefile-ro is
	 set without a script, we just do without any
	 lease database. */
 
      if (daemon.lease_change_command)
	{
	  strcpy(daemon.dhcp_buff, daemon.lease_change_command);
	  strcat(daemon.dhcp_buff, " init");
	  leasestream = popen(daemon.dhcp_buff, "r");
	}
      else

	{
          file_dirty = dns_dirty = 0;
          return;
        }

    }
  else
    {
      /* NOTE: need a+ mode to create file if it doesn't exist */
      leasestream = daemon.lease_stream = fopen(daemon.lease_file, "a+");

      if (!leasestream)
	die(format!("cannot open or create lease file {}: {}"), daemon.lease_file, EC_FILE);

      /* a+ mode leaves pointer at end. */
      rewind(leasestream);
    }

  if (leasestream)
    {
      if (!read_leases(now, leasestream))
	my_syslog(MS_DHCP | LOG_ERR, format!("failed to parse lease database cleanly"));
      
      if (ferror(leasestream))
	die(format!("failed to read lease file {}: {}"), daemon.lease_file, EC_FILE);
    }
  
 
  if (!daemon.lease_stream)
    {
      let mut rc: i32 = 0;

      /* shell returns 127 for "command not found", 126 for bad permissions. */
      if (!leasestream || (rc = pclose(leasestream)) == -1 || WEXITSTATUS(rc) == 127 || WEXITSTATUS(rc) == 126)
	{
	  if (WEXITSTATUS(rc) == 127)
	    errno = ENOENT;
	  else if (WEXITSTATUS(rc) == 126)
	    errno = EACCES;

	  die(format!("cannot run lease-init script {}: {}"), daemon.lease_change_command, EC_FILE);
	}
      
      if (WEXITSTATUS(rc) != 0)
	{
	  sprintf(daemon.dhcp_buff, "{}", WEXITSTATUS(rc));
	  die(format!("lease-init script returned exit code {}"), daemon.dhcp_buff, WEXITSTATUS(rc) + EC_INIT_OFFSET);
	}
    }


  /* Some leases may have expired */
  file_dirty = 0;
  lease_prune(NULL, now);
  dns_dirty = 1;
}

void lease_update_from_configs()
{
  /* changes to the config may change current leases. */
  
  let mut lease: dhcp_lease;
  let mut config: dhcp_config;
  char *name;
  
  for (lease = leases; lease; lease = lease.next)
    if (lease.flags & (LEASE_TA | LEASE_NA))
      continue;
    else if ((config = find_config(daemon.dhcp_conf, NULL, lease.clid, lease.clid_len, 
				   lease.hwaddr, lease.hwaddr_len, lease.hwaddr_type, NULL, NULL)) && 
	     (config.flags & CONFIG_NAME) &&
	     (!(config.flags & CONFIG_ADDR) || config.addr.s_addr == lease.addr.s_addr))
      lease_set_hostname(lease, config.hostname, 1, get_domain(lease.addr), NULL);
    else if ((name = host_from_dns(lease.addr)))
      lease_set_hostname(lease, name, 1, get_domain(lease.addr), NULL); /* updates auth flag only */
}

pub fn ourprintf(int *errp, format: &mut String, ...)
{
  va_list ap;
  
  va_start(ap, format);
  if (!(*errp) && vfprintf(daemon.lease_stream, format, ap) < 0)
    *errp = errno;
  va_end(ap);
}

void lease_update_file(now: time::Instant)
{
  let mut lease: dhcp_lease;
  next_event: time::Instant;
  i: i32, err = 0;

  if (file_dirty != 0 && daemon.lease_stream)
    {
      errno = 0;
      rewind(daemon.lease_stream);
      if (errno != 0 || ftruncate(fileno(daemon.lease_stream), 0) != 0)
	err = errno;
      
      for (lease = leases; lease; lease = lease.next)
	{

 
	  if (lease.flags & (LEASE_TA | LEASE_NA))
	    continue;



	  ourprintf(&err, "{} ", lease.length);

	  ourprintf(&err, "{} ", (unsigned long)lease.expires);


	  if (lease.hwaddr_type != ARPHRD_ETHER || lease.hwaddr_len == 0) 
	    ourprintf(&err, "%.2x-", lease.hwaddr_type);
	  for (i = 0; i < lease.hwaddr_len; i++)
	    {
	      ourprintf(&err, "%.2x", lease.hwaddr[i]);
	      if (i != lease.hwaddr_len - 1)
		ourprintf(&err, ":");
	    }
	  
	  inet_ntop(AF_INET, &lease.addr, daemon.addrbuff, ADDRSTRLEN); 

	  ourprintf(&err, " {} ", daemon.addrbuff);
	  ourprintf(&err, "{} ", lease.hostname ? lease.hostname : "*");
	  	  
	  if (lease.clid && lease.clid_len != 0)
	    {
	      for (i = 0; i < lease.clid_len - 1; i++)
		ourprintf(&err, "%.2x:", lease.clid[i]);
	      ourprintf(&err, "%.2x\n", lease.clid[i]);
	    }
	  else
	    ourprintf(&err, "*\n");	  
	}
      
   
      if (daemon.duid)
	{
	  ourprintf(&err, "duid ");
	  for (i = 0; i < daemon.duid_len - 1; i++)
	    ourprintf(&err, "%.2x:", daemon.duid[i]);
	  ourprintf(&err, "%.2x\n", daemon.duid[i]);
	  
	  for (lease = leases; lease; lease = lease.next)
	    {
	      
	      if (!(lease.flags & (LEASE_TA | LEASE_NA)))
		continue;


	      ourprintf(&err, "{} ", lease.length);

	      ourprintf(&err, "{} ", (unsigned long)lease.expires);

    
	      inet_ntop(AF_INET6, &lease.addr6, daemon.addrbuff, ADDRSTRLEN);
	 
	      ourprintf(&err, "{}{} {} ", (lease.flags & LEASE_TA) ? "T" : "",
			lease.iaid, daemon.addrbuff);
	      ourprintf(&err, "{} ", lease.hostname ? lease.hostname : "*");
	      
	      if (lease.clid && lease.clid_len != 0)
		{
		  for (i = 0; i < lease.clid_len - 1; i++)
		    ourprintf(&err, "%.2x:", lease.clid[i]);
		  ourprintf(&err, "%.2x\n", lease.clid[i]);
		}
	      else
		ourprintf(&err, "*\n");	  
	    }
	}
      
	  
      if (fflush(daemon.lease_stream) != 0 ||
	  fsync(fileno(daemon.lease_stream)) < 0)
	err = errno;
      
      if (!err)
	file_dirty = 0;
    }
  
  /* Set alarm for when the first lease expires. */
  next_event = 0;

 
  /* do timed RAs and determine when the next is, also pings to potential SLAAC addresses */
  if (daemon.doing_ra)
    {
      event: time::Instant;
      
      if ((event = periodic_slaac(now, leases)) != 0)
	{
	  if (next_event == 0 || difftime(next_event, event) > 0.0)
	    next_event = event;
	}
      
      if ((event = periodic_ra(now)) != 0)
	{
	  if (next_event == 0 || difftime(next_event, event) > 0.0)
	    next_event = event;
	}
    }


  for (lease = leases; lease; lease = lease.next)
    if (lease.expires != 0 &&
	(next_event == 0 || difftime(next_event, lease.expires) > 0.0))
      next_event = lease.expires;
   
  if (err)
    {
      if (next_event == 0 || difftime(next_event, LEASE_RETRY + now) > 0.0)
	next_event = LEASE_RETRY + now;
      
      my_syslog(MS_DHCP | LOG_ERR, format!("failed to write {}: {} (retry in {})"), 
		daemon.lease_file, strerror(err),
		(unsigned int)difftime(next_event, now));
    }

  send_alarm(next_event, now);
}


 int find_interface_v4(local: net::IpAddr, if_index: i32, label: &mut String,
			     netmask: net::IpAddr, broadcast: net::IpAddr, vparam: Vec<u8>)
{
  let mut lease: dhcp_lease;
  int prefix = netmask_length(netmask);

  () label;
  () broadcast;
  () vparam;

  for (lease = leases; lease; lease = lease.next)
    if (!(lease.flags & (LEASE_TA | LEASE_NA)) &&
	is_same_net(local, lease.addr, netmask) && 
	prefix > lease.new_prefixlen) 
      {
	lease.new_interface = if_index;
        lease.new_prefixlen = prefix;
      }

  return 1;
}

 
 int find_interface_v6(local: &mut net::IpAddr,  prefix: i32,
			     scope: i32, if_index: i32, flags: i32, 
			     preferred: i32, valid: i32, vparam: Vec<u8>)
{
  let mut lease: dhcp_lease;

  ()scope;
  ()flags;
  ()preferred;
  ()valid;
  ()vparam;

  for (lease = leases; lease; lease = lease.next)
    if ((lease.flags & (LEASE_TA | LEASE_NA)))
      if (is_same_net6(local, &lease.addr6, prefix) && prefix > lease.new_prefixlen) {
        /* save prefix length for comparison, as we might get shorter matching
         * prefix in upcoming netlink GETADDR responses
         * */
        lease.new_interface = if_index;
        lease.new_prefixlen = prefix;
      }

  return 1;
}

void lease_ping_reply(sender: &mut net::IpAddr, packet: &mut Vec<u8>, interface: &mut String)
{
  /* We may be doing RA but not DHCPv4, in which case the lease
     database may not exist and we have nothing to do anyway */
  if (daemon.dhcp)
    slaac_ping_reply(sender, packet, interface, leases);
}

void lease_update_slaac(now: time::Instant)
{
  /* Called when we construct a new RA-names context, to add putative
     new SLAAC addresses to existing leases. */

  let mut lease: dhcp_lease;
  
  if (daemon.dhcp)
    for (lease = leases; lease; lease = lease.next)
      slaac_add_addrs(lease, now, 0);
}




/* Find interfaces associated with leases at start-up. This gets updated as
   we do DHCP transactions, but information about directly-connected subnets
   is useful from scrips and necessary for determining SLAAC addresses from
   start-time. */
void lease_find_interfaces(now: time::Instant)
{
  let mut lease: dhcp_lease;
  
  for (lease = leases; lease; lease = lease.next)
    lease.new_prefixlen = lease.new_interface = 0;

  iface_enumerate(AF_INET, &now, find_interface_v4);
 
  iface_enumerate(AF_INET6, &now, find_interface_v6);


  for (lease = leases; lease; lease = lease.next)
    if (lease.new_interface != 0) 
      lease_set_interface(lease, lease.new_interface, now);
}

 
void lease_make_duid(now: time::Instant)
{
  /* If we're not doing DHCPv6, and there are not v6 leases, don't add the DUID to the database */
  if (!daemon.duid && daemon.doing_dhcp6)
    {
      file_dirty = 1;
      make_duid(now);
    }
}





void lease_update_dns(int force)
{
  let mut lease: dhcp_lease;

  if (daemon.port != 0 && (dns_dirty || force))
    {
#ifndef
      /* force transfer to authoritative secondaries */
      daemon.soa_sn +=1;

      
      cache_unhash_dhcp();

      for (lease = leases; lease; lease = lease.next)
	{
	  int prot = AF_INET;
	  
 
	  if (lease.flags & (LEASE_TA | LEASE_NA))
	    prot = AF_INET6;
	  else if (lease.hostname || lease.fqdn)
	    {
	      let mut slaac: slaac_address;

	      for (slaac = lease.slaac_address; slaac; slaac = slaac.next)
		if (slaac.backoff == 0)
		  {
		    if (lease.fqdn)
		      cache_add_dhcp_entry(lease.fqdn, AF_INET6, (union all_addr *)&slaac.addr, lease.expires);
		    if (!option_bool(OPT_DHCP_FQDN) && lease.hostname)
		      cache_add_dhcp_entry(lease.hostname, AF_INET6, (union all_addr *)&slaac.addr, lease.expires);
		  }
	    }
	  
	  if (lease.fqdn)
	    cache_add_dhcp_entry(lease.fqdn, prot, 
				 prot == AF_INET ? (union all_addr *)&lease.addr : (union all_addr *)&lease.addr6,
				 lease.expires);
	     
	  if (!option_bool(OPT_DHCP_FQDN) && lease.hostname)
	    cache_add_dhcp_entry(lease.hostname, prot, 
				 prot == AF_INET ? (union all_addr *)&lease.addr : (union all_addr *)&lease.addr6, 
				 lease.expires);
       

	  if (lease.fqdn)
	    cache_add_dhcp_entry(lease.fqdn, prot, (union all_addr *)&lease.addr, lease.expires);
	  
	  if (!option_bool(OPT_DHCP_FQDN) && lease.hostname)
	    cache_add_dhcp_entry(lease.hostname, prot, (union all_addr *)&lease.addr, lease.expires);

	}
      
      dns_dirty = 0;
    }
}

void lease_prune(struct dhcp_lease *target, now: &time::Instant)
{
  struct dhcp_lease *lease, *tmp, **up;

  for (lease = leases, up = &leases; lease; lease = tmp)
    {
      tmp = lease.next;
      if ((lease.expires != 0 && difftime(now, lease.expires) >= 0) || lease == target)
	{
	  file_dirty = 1;
	  if (lease.hostname)
	    dns_dirty = 1;

	  daemon.metrics[lease.addr.s_addr ? METRIC_LEASES_PRUNED_4 : METRIC_LEASES_PRUNED_6]++;

 	  *up = lease.next; /* unlink */
	  
	  /* Put on old_leases list 'till we
	     can run the script */
	  lease.next = old_leases;
	  old_leases = lease;
	  
	  leases_left +=1;
	}
      else
	up = &lease.next;
    }
} 
	
  
struct dhcp_lease *lease_find_by_client(hwaddr: &mut Vec<u8>, hw_len: i32, hw_type: i32,
					clid: &mut Vec<u8>, clid_len: i32)
{
  let mut lease: dhcp_lease;

  if (clid)
    for (lease = leases; lease; lease = lease.next)
      {
 
	if (lease.flags & (LEASE_TA | LEASE_NA))
	  continue;

	if (lease.clid && clid_len == lease.clid_len &&
	    memcmp(clid, lease.clid, clid_len) == 0)
	  return lease;
      }
  
  for (lease = leases; lease; lease = lease.next)	
    {
 
      if (lease.flags & (LEASE_TA | LEASE_NA))
	continue;
   
      if ((!lease.clid || !clid) && 
	  hw_len != 0 && 
	  lease.hwaddr_len == hw_len &&
	  lease.hwaddr_type == hw_type &&
	  memcmp(hwaddr, lease.hwaddr, hw_len) == 0)
	return lease;
    }

  return NULL;
}

struct dhcp_lease *lease_find_by_addr(addr: net::IpAddr)
{
  let mut lease: dhcp_lease;

  for (lease = leases; lease; lease = lease.next)
    {
 
      if (lease.flags & (LEASE_TA | LEASE_NA))
	continue;
  
      if (lease.addr.s_addr == addr.s_addr)
	return lease;
    }

  return NULL;
}

 
/* find address for {CLID, IAID, address} */
struct dhcp_lease *lease6_find(clid: &mut Vec<u8>, clid_len: i32, 
			       lease_type: i32, unsigned iaid: i32,
			       addr: &mut net::IpAddr)
{
  let mut lease: dhcp_lease;
  
  for (lease = leases; lease; lease = lease.next)
    {
      if (!(lease.flags & lease_type) || lease.iaid != iaid)
	continue;

      if (!IN6_ARE_ADDR_EQUAL(&lease.addr6, addr))
	continue;
      
      if ((clid_len != lease.clid_len ||
	   memcmp(clid, lease.clid, clid_len) != 0))
	continue;
      
      return lease;
    }
  
  return NULL;
}

/* reset "USED flags */
void lease6_reset()
{
  let mut lease: dhcp_lease;
  
  for (lease = leases; lease; lease = lease.next)
    lease.flags &= ~LEASE_USED;
}

/* enumerate all leases belonging to {CLID, IAID} */
struct dhcp_lease *lease6_find_by_client(struct dhcp_lease *first, lease_type: i32,
					 clid: &mut Vec<u8>, clid_len: i32,
					 unsigned int iaid)
{
  let mut lease: dhcp_lease;

  if (!first)
    first = leases;
  else
    first = first.next;

  for (lease = first; lease; lease = lease.next)
    {
      if (lease.flags & LEASE_USED)
	continue;

      if (!(lease.flags & lease_type) || lease.iaid != iaid)
	continue;
 
      if ((clid_len != lease.clid_len ||
	   memcmp(clid, lease.clid, clid_len) != 0))
	continue;

      return lease;
    }
  
  return NULL;
}

struct dhcp_lease *lease6_find_by_addr(net: &mut net::IpAddr, prefix: i32, u64 addr)
{
  let mut lease: dhcp_lease;
    
  for (lease = leases; lease; lease = lease.next)
    {
      if (!(lease.flags & (LEASE_TA | LEASE_NA)))
	continue;
      
      if (is_same_net6(&lease.addr6, net, prefix) &&
	  (prefix == 128 || addr6part(&lease.addr6) == addr))
	return lease;
    }
  
  return NULL;
} 

/* Find largest assigned address in context */
pub fn lease_find_max_addr6(struct dhcp_context *context) -> u64
{
  let mut lease: dhcp_lease;
  u64 addr = addr6part(&context.start6);
  
  if (!(context.flags & (CONTEXT_ | CONTEXT_PROXY)))
    for (lease = leases; lease; lease = lease.next)
      {
	if (!(lease.flags & (LEASE_TA | LEASE_NA)))
	  continue;

	if (is_same_net6(&lease.addr6, &context.start6, 64) &&
	    addr6part(&lease.addr6) > addr6part(&context.start6) &&
	    addr6part(&lease.addr6) <= addr6part(&context.end6) &&
	    addr6part(&lease.addr6) > addr)
	  addr = addr6part(&lease.addr6);
      }
  
  return addr;
}



/* Find largest assigned address in context */
lease_find_max_addr: net::IpAddr(struct dhcp_context *context)
{
  let mut lease: dhcp_lease;
  addr: net::IpAddr = context.start;
  
  if (!(context.flags & (CONTEXT_ | CONTEXT_PROXY)))
    for (lease = leases; lease; lease = lease.next)
      {
 
	if (lease.flags & (LEASE_TA | LEASE_NA))
	  continue;

	if (((unsigned)ntohl(lease.addr.s_addr)) > ((unsigned)ntohl(context.start.s_addr)) &&
	    ((unsigned)ntohl(lease.addr.s_addr)) <= ((unsigned)ntohl(context.end.s_addr)) &&
	    ((unsigned)ntohl(lease.addr.s_addr)) > ((unsigned)ntohl(addr.s_addr)))
	  addr = lease.addr;
      }
  
  return addr;
}

 struct dhcp_lease *lease_allocate()
{
  let mut lease: dhcp_lease;
  if (!leases_left || !(lease = whine_malloc(sizeof(struct dhcp_lease))))
    return NULL;

  memset(lease, 0, sizeof(struct dhcp_lease));
  lease.flags = LEASE_NEW;
  lease.expires = 1;

  lease.length = 0xffffffff; /* illegal value */

  lease.hwaddr_len = 256; /* illegal value */
  lease.next = leases;
  leases = lease;
  
  file_dirty = 1;
  leases_left--;

  return lease;
}

struct dhcp_lease *lease4_allocate(addr: net::IpAddr)
{
  struct dhcp_lease *lease = lease_allocate();
  if (lease)
    {
      lease.addr = addr;
      daemon.metrics[METRIC_LEASES_ALLOCATED_4]++;
    }
  
  return lease;
}

 
struct dhcp_lease *lease6_allocate(addrp: &mut net::IpAddr, lease_type: i32)
{
  struct dhcp_lease *lease = lease_allocate();

  if (lease)
    {
      lease.addr6 = *addrp;
      lease.flags |= lease_type;
      lease.iaid = 0;

      daemon.metrics[METRIC_LEASES_ALLOCATED_6]++;
    }

  return lease;
}


void lease_set_expires(struct dhcp_lease *lease, unsigned len: i32, now: &time::Instant)
{
  exp: time::Instant;

  if (len == 0xffffffff)
    {
      exp = 0;
      len = 0;
    }
  else
    {
      exp = now + (time_t)len;
      /* Check for 2038 overflow. Make the lease
	 infinite in that case, as the least disruptive
	 thing we can do. */
      if (difftime(exp, now) <= 0.0)
	exp = 0;
    }

  if (exp != lease.expires)
    {
      dns_dirty = 1;
      lease.expires = exp;
#ifndef
      lease.flags |= LEASE_AUX_CHANGED | LEASE_EXP_CHANGED;
      file_dirty = 1;

    }
  

  if (len != lease.length)
    {
      lease.length = len;
      lease.flags |= LEASE_AUX_CHANGED;
      file_dirty = 1; 
    }

} 

 
void lease_set_iaid(struct dhcp_lease *lease, unsigned int iaid)
{
  if (lease.iaid != iaid)
    {
      lease.iaid = iaid;
      lease.flags |= LEASE_CHANGED;
    }
}


void lease_set_hwaddr(struct dhcp_lease *lease, const hwaddr: &mut Vec<u8>,
		      const clid: &mut Vec<u8>, hw_len: i32, hw_type: i32,
		      clid_len: i32, now: &time::Instant, force: i32)
{
 
  int change = force;
  lease.flags |= LEASE_HAVE_HWADDR;


  ()force;
  ()now;

  if (hw_len != lease.hwaddr_len ||
      hw_type != lease.hwaddr_type || 
      (hw_len != 0 && memcmp(lease.hwaddr, hwaddr, hw_len) != 0))
    {
      if (hw_len != 0)
	memcpy(lease.hwaddr, hwaddr, hw_len);
      lease.hwaddr_len = hw_len;
      lease.hwaddr_type = hw_type;
      lease.flags |= LEASE_CHANGED;
      file_dirty = 1; /* run script on change */
    }

  /* only update clid when one is available, stops packets
     without a clid removing the record. Lease init uses
     clid_len == 0 for no clid. */
  if (clid_len != 0 && clid)
    {
      if (!lease.clid)
	lease.clid_len = 0;

      if (lease.clid_len != clid_len)
	{
	  lease.flags |= LEASE_AUX_CHANGED;
	  file_dirty = 1;
	  free(lease.clid);
	  if (!(lease.clid = whine_malloc(clid_len)))
	    return;
 
	  change = 1;
	   
	}
      else if (memcmp(lease.clid, clid, clid_len) != 0)
	{
	  lease.flags |= LEASE_AUX_CHANGED;
	  file_dirty = 1;
 
	  change = 1;
	
	}
      
      lease.clid_len = clid_len;
      memcpy(lease.clid, clid, clid_len);
    }
  
 
  if (change)
    slaac_add_addrs(lease, now, force);

}

pub fn kill_name(struct dhcp_lease *lease)
{
  /* run script to say we lost our old name */
  
  /* this shouldn't happen unless updates are very quick and the
     script very slow, we just avoid a memory leak if it does. */
  free(lease.old_hostname);
  
  /* If we know the fqdn, pass that. The helper will derive the
     unqualified name from it, free the unqualified name here. */

  if (lease.fqdn)
    {
      lease.old_hostname = lease.fqdn;
      free(lease.hostname);
    }
  else
    lease.old_hostname = lease.hostname;

  lease.hostname = lease.fqdn = NULL;
}

void lease_set_hostname(struct dhcp_lease *lease, const name: &mut String, auth: i32, domain: &mut String, config_domain: &mut String)
{
  let mut lease_tmp: dhcp_lease;
  char *new_name = NULL, *new_fqdn = NULL;

  if (config_domain && (!domain || !hostname_isequal(domain, config_domain)))
    my_syslog(MS_DHCP | LOG_WARNING, format!("Ignoring domain {} for DHCP host name {}"), config_domain, name);
  
  if (lease.hostname && name && hostname_isequal(lease.hostname, name))
    {
      if (auth)
	lease.flags |= LEASE_AUTH_NAME;
      return;
    }
  
  if (!name && !lease.hostname)
    return;

  /* If a machine turns up on a new net without dropping the old lease,
     or two machines claim the same name, then we end up with two interfaces with
     the same name. Check for that here and remove the name from the old lease.
     Note that IPv6 leases are different. All the leases to the same DUID are 
     allowed the same name.

     Don't allow a name from the client to override a name from dnsmasq config. */
  
  if (name)
    {
      if ((new_name = whine_malloc(strlen(name) + 1)))
	{
	  strcpy(new_name, name);
	  if (domain && (new_fqdn = whine_malloc(strlen(new_name) + strlen(domain) + 2)))
	    {
	      strcpy(new_fqdn, name);
	      strcat(new_fqdn, ".");
	      strcat(new_fqdn, domain);
	    }
	}
	  
      /* Depending on mode, we check either unqualified name or FQDN. */
      for (lease_tmp = leases; lease_tmp; lease_tmp = lease_tmp.next)
	{
	  if (option_bool(OPT_DHCP_FQDN))
	    {
	      if (!new_fqdn || !lease_tmp.fqdn || !hostname_isequal(lease_tmp.fqdn, new_fqdn))
		continue;
	    }
	  else
	    {
	      if (!new_name || !lease_tmp.hostname || !hostname_isequal(lease_tmp.hostname, new_name) )
		continue; 
	    }

	  if (lease.flags & (LEASE_TA | LEASE_NA))
	    {
	      if (!(lease_tmp.flags & (LEASE_TA | LEASE_NA)))
		continue;

	      /* another lease for the same DUID is OK for IPv6 */
	      if (lease.clid_len == lease_tmp.clid_len &&
		  lease.clid && lease_tmp.clid &&
		  memcmp(lease.clid, lease_tmp.clid, lease.clid_len) == 0)
		continue;	      
	    }
	  else if (lease_tmp.flags & (LEASE_TA | LEASE_NA))
	    continue;
		   
	  if ((lease_tmp.flags & LEASE_AUTH_NAME) && !auth)
	    {
	      free(new_name);
	      free(new_fqdn);
	      return;
	    }
	
	  kill_name(lease_tmp);
	  break;
	}
    }

  if (lease.hostname)
    kill_name(lease);

  lease.hostname = new_name;
  lease.fqdn = new_fqdn;
  
  if (auth)
    lease.flags |= LEASE_AUTH_NAME;
  
  file_dirty = 1;
  dns_dirty = 1; 
  lease.flags |= LEASE_CHANGED; /* run script on change */
}

void lease_set_interface(struct dhcp_lease *lease, interface: i32, now: &time::Instant)
{
  ()now;

  if (lease.last_interface == interface)
    return;

  lease.last_interface = interface;
  lease.flags |= LEASE_CHANGED; 

 
  slaac_add_addrs(lease, now, 0);

}

void rerun_scripts()
{
  let mut lease: dhcp_lease;
  
  for (lease = leases; lease; lease = lease.next)
    lease.flags |= LEASE_CHANGED; 
}

/* deleted leases get transferred to the old_leases list.
   remove them here, after calling the lease change
   script. Also run the lease change script on new/modified leases.

   Return zero if nothing to do. */
int do_script_run(now: time::Instant)
{
  let mut lease: dhcp_lease;

  ()now;

 HAVE_DBUS
  /* If we're going to be sending DBus signals, but the connection is not yet up,
     delay everything until it is. */
  if (daemon.opt_dbus && !daemon.dbus)
    return 0;


  if (old_leases)
    {
      lease = old_leases;
                  
      /* If the lease still has an old_hostname, do the "old" action on that first */
      if (lease.old_hostname)
	{
 
	  queue_script(ACTION_OLD_HOSTNAME, lease, lease.old_hostname, now);

	  free(lease.old_hostname);
	  lease.old_hostname = NULL;
	  return 1;
	}
      else 
	{
 
	  struct slaac_address *slaac, *tmp;
	  for (slaac = lease.slaac_address; slaac; slaac = tmp)
	    {
	      tmp = slaac.next;
	      free(slaac);
	    }

	  kill_name(lease);
 
	  queue_script(ACTION_DEL, lease, lease.old_hostname, now);

 HAVE_DBUS
	  emit_dbus_signal(ACTION_DEL, lease, lease.old_hostname);

	  old_leases = lease.next;
	  
	  free(lease.old_hostname); 
	  free(lease.clid);
	  free(lease.extradata);
	  free(lease);
	    
	  return 1; 
	}
    }
  
  /* make sure we announce the loss of a hostname before its new location. */
  for (lease = leases; lease; lease = lease.next)
    if (lease.old_hostname)
      {	
 
	queue_script(ACTION_OLD_HOSTNAME, lease, lease.old_hostname, now);

	free(lease.old_hostname);
	lease.old_hostname = NULL;
	return 1;
      }
  
  for (lease = leases; lease; lease = lease.next)
    if ((lease.flags & (LEASE_NEW | LEASE_CHANGED)) || 
	((lease.flags & LEASE_AUX_CHANGED) && option_bool(OPT_LEASE_RO)) ||
	((lease.flags & LEASE_EXP_CHANGED) && option_bool(OPT_LEASE_RENEW)))
      {
 
	queue_script((lease.flags & LEASE_NEW) ? ACTION_ADD : ACTION_OLD, lease, 
		     lease.fqdn ? lease.fqdn : lease.hostname, now);

 HAVE_DBUS
	emit_dbus_signal((lease.flags & LEASE_NEW) ? ACTION_ADD : ACTION_OLD, lease,
			 lease.fqdn ? lease.fqdn : lease.hostname);

	lease.flags &= ~(LEASE_NEW | LEASE_CHANGED | LEASE_AUX_CHANGED | LEASE_EXP_CHANGED);
	
	/* this is used for the "add" call, then junked, since they're not in the database */
	free(lease.extradata);
	lease.extradata = NULL;
	
	return 1;
      }

  return 0; /* nothing to do */
}

 
/* delim == -1 . delim = 0, but embedded 0s, creating extra records, are OK. */
void lease_add_extradata(struct dhcp_lease *lease, data: &mut Vec<u8>, unsigned len: i32, delim: i32)
{
  let mut i: u32;
  
  if (delim == -1)
    delim = 0;
  else
    /* check for embedded NULLs */
    for i in 0..len
      if (data[i] == 0)
	{
	  len = i;
	  break;
	}
  
  if ((lease.extradata_size - lease.extradata_len) < (len + 1))
    {
      newsz: usize = lease.extradata_len + len + 100;
      unsigned char *new = whine_malloc(newsz);
  
      if (!new)
	return;
      
      if (lease.extradata)
	{
	  memcpy(new, lease.extradata, lease.extradata_len);
	  free(lease.extradata);
	}

      lease.extradata = new;
      lease.extradata_size = newsz;
    }

  if (len != 0)
    memcpy(lease.extradata + lease.extradata_len, data, len);
  lease.extradata[lease.extradata_len + len] = delim;
  lease.extradata_len += len + 1; 
}



	  

      

