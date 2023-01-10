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

/* Declare static char *compiler_opts  in config.h */
// #define DNSMASQ_COMPILE_OPTS

/* dnsmasq.h has to be included first as it sources config.h */
// #include "dnsmasq.h"

// #if defined(HAVE_IDN) || defined(HAVE_LIBIDN2) || defined(LOCALEDIR)
// #include <locale.h>
// #endif







int main (int argc, char **argv)

}

static void sig_handler(int sig)
{
  if (pid == 0)
    {
      /* ignore anything other than TERM during startup
	 and in helper proc. (helper ignore TERM too) */
      if (sig == SIGTERM || sig == SIGINT)
	exit(EC_MISC);
    }
  else if (pid != getpid())
    {
      /* alarm is used to kill TCP children after a fixed time. */
      if (sig == SIGALRM)
	_exit(0);
    }
  else
    {
      /* master process */
      int event, errsave = errno;
      
      if (sig == SIGHUP)
	event = EVENT_RELOAD;
      else if (sig == SIGCHLD)
	event = EVENT_CHILD;
      else if (sig == SIGALRM)
	event = EVENT_ALARM;
      else if (sig == SIGTERM)
	event = EVENT_TERM;
      else if (sig == SIGUSR1)
	event = EVENT_DUMP;
      else if (sig == SIGUSR2)
	event = EVENT_REOPEN;
      else if (sig == SIGINT)
	{
	  /* Handle SIGINT normally in debug mode, so
	     ctrl-c continues to operate. */
	  if (option_bool(OPT_DEBUG))
	    exit(EC_MISC);
	  else
	    event = EVENT_TIME;
	}
      else
	return;

      send_event(pipewrite, event, 0, NULL); 
      errno = errsave;
    }
}

/* now == 0 -> queue immediate callback */
void send_alarm(time_t event, time_t now)
{
  if (now == 0 || event != 0)
    {
      /* alarm(0) or alarm(-ve) doesn't do what we want.... */
      if ((now == 0 || difftime(event, now) <= 0.0))
	send_event(pipewrite, EVENT_ALARM, 0, NULL);
      else 
	alarm((unsigned)difftime(event, now)); 
    }
}

void queue_event(int event)
{
  send_event(pipewrite, event, 0, NULL);
}

void send_event(int fd, int event, int data, char *msg)
{
  struct event_desc ev;
  struct iovec iov[2];

  ev.event = event;
  ev.data = data;
  ev.msg_sz = msg ? strlen(msg) : 0;
  
  iov[0].iov_base = &ev;
  iov[0].iov_len = sizeof(ev);
  iov[1].iov_base = msg;
  iov[1].iov_len = ev.msg_sz;
  
  /* error pipe, debug mode. */
  if (fd == -1)
    fatal_event(&ev, msg);
  else
    /* pipe is non-blocking and struct event_desc is smaller than
       PIPE_BUF, so this either fails or writes everything */
    while (writev(fd, iov, msg ? 2 : 1) == -1 && errno == EINTR);
}

/* NOTE: the memory used to return msg is leaked: use msgs in events only
   to describe fatal errors. */
static int read_event(int fd, struct event_desc *evp, char **msg)
{
  char *buf;

  if (!read_write(fd, (unsigned char *)evp, sizeof(struct event_desc), 1))
    return 0;
  
  *msg = NULL;
  
  if (evp->msg_sz != 0 && 
      (buf = malloc(evp->msg_sz + 1)) &&
      read_write(fd, (unsigned char *)buf, evp->msg_sz, 1))
    {
      buf[evp->msg_sz] = 0;
      *msg = buf;
    }

  return 1;
}
    
static void fatal_event(struct event_desc *ev, char *msg)
{
  errno = ev->data;
  
  switch (ev->event)
    {
    case EVENT_DIE:
      exit(0);

    case EVENT_FORK_ERR:
      die(_("cannot fork into background: %s"), NULL, EC_MISC);

      /* fall through */
    case EVENT_PIPE_ERR:
      die(_("failed to create helper: %s"), NULL, EC_MISC);

      /* fall through */
    case EVENT_CAP_ERR:
      die(_("setting capabilities failed: %s"), NULL, EC_MISC);

      /* fall through */
    case EVENT_USER_ERR:
      die(_("failed to change user-id to %s: %s"), msg, EC_MISC);

      /* fall through */
    case EVENT_GROUP_ERR:
      die(_("failed to change group-id to %s: %s"), msg, EC_MISC);

      /* fall through */
    case EVENT_PIDFILE:
      die(_("failed to open pidfile %s: %s"), msg, EC_FILE);

      /* fall through */
    case EVENT_LOG_ERR:
      die(_("cannot open log %s: %s"), msg, EC_FILE);

      /* fall through */
    case EVENT_LUA_ERR:
      die(_("failed to load Lua script: %s"), msg, EC_MISC);

      /* fall through */
    case EVENT_TFTP_ERR:
      die(_("TFTP directory %s inaccessible: %s"), msg, EC_FILE);

      /* fall through */
    case EVENT_TIME_ERR:
      die(_("cannot create timestamp file %s: %s" ), msg, EC_BADCONF);
    }
}	
      
static void async_event(int pipe, time_t now)
{
  pid_t p;
  struct event_desc ev;
  int i, check = 0;
  char *msg;
  
  /* NOTE: the memory used to return msg is leaked: use msgs in events only
     to describe fatal errors. */
  
  if (read_event(pipe, &ev, &msg))
    switch (ev.event)
      {
      case EVENT_RELOAD:
	daemon->soa_sn++; /* Bump zone serial, as it may have changed. */
	
	/* fall through */
	
      case EVENT_INIT:
	clear_cache_and_reload(now);
	
	if (daemon->port != 0)
	  {
	    if (daemon->resolv_files && option_bool(OPT_NO_POLL))
	      {
		reload_servers(daemon->resolv_files->name);
		check = 1;
	      }

	    if (daemon->servers_file)
	      {
		read_servers_file();
		check = 1;
	      }

	    if (check)
	      check_servers(0);
	  }

// #ifdef HAVE_DHCP
	rerun_scripts();
// #endif
	break;
	
      case EVENT_DUMP:
	if (daemon->port != 0)
	  dump_cache(now);
	break;
	
      case EVENT_ALARM:
// #ifdef HAVE_DHCP
	if (daemon->dhcp || daemon->doing_dhcp6)
	  {
	    lease_prune(NULL, now);
	    lease_update_file(now);
	  }
// #ifdef HAVE_DHCP6
	else if (daemon->doing_ra)
	  /* Not doing DHCP, so no lease system, manage alarms for ra only */
	    send_alarm(periodic_ra(now), now);
// #endif
// #endif
	break;
		
      case EVENT_CHILD:
	/* See Stevens 5.10 */
	while ((p = waitpid(-1, NULL, WNOHANG)) != 0)
	  if (p == -1)
	    {
	      if (errno != EINTR)
		break;
	    }      
	  else 
	    for (i = 0 ; i < MAX_PROCS; i++)
	      if (daemon->tcp_pids[i] == p)
		daemon->tcp_pids[i] = 0;
	break;
	
#if defined(HAVE_SCRIPT)	
      case EVENT_KILLED:
	my_syslog(LOG_WARNING, _("script process killed by signal %d"), ev.data);
	break;

      case EVENT_EXITED:
	my_syslog(LOG_WARNING, _("script process exited with status %d"), ev.data);
	break;

      case EVENT_EXEC_ERR:
	my_syslog(LOG_ERR, _("failed to execute %s: %s"), 
		  daemon->lease_change_command, strerror(ev.data));
	break;

      case EVENT_SCRIPT_LOG:
	my_syslog(MS_SCRIPT | LOG_DEBUG, "%s", msg ? msg : "");
        free(msg);
	msg = NULL;
	break;

	/* necessary for fatal errors in helper */
      case EVENT_USER_ERR:
      case EVENT_DIE:
      case EVENT_LUA_ERR:
	fatal_event(&ev, msg);
	break;
// #endif

      case EVENT_REOPEN:
	/* Note: this may leave TCP-handling processes with the old file still open.
	   Since any such process will die in CHILD_LIFETIME or probably much sooner,
	   we leave them logging to the old file. */
	if (daemon->log_file != NULL)
	  log_reopen(daemon->log_file);
	break;

      case EVENT_NEWADDR:
	newaddress(now);
	break;

      case EVENT_NEWROUTE:
	resend_query();
	/* Force re-reading resolv file right now, for luck. */
	poll_resolv(0, 1, now);
	break;

      case EVENT_TIME:
// #ifdef HAVE_DNSSEC
	if (daemon->dnssec_no_time_check && option_bool(OPT_DNSSEC_VALID) && option_bool(OPT_DNSSEC_TIME))
	  {
	    my_syslog(LOG_INFO, _("now checking DNSSEC signature timestamps"));
	    daemon->dnssec_no_time_check = 0;
	    clear_cache_and_reload(now);
	  }
// #endif
	break;
	
      case EVENT_TERM:
	/* Knock all our children on the head. */
	for (i = 0; i < MAX_PROCS; i++)
	  if (daemon->tcp_pids[i] != 0)
	    kill(daemon->tcp_pids[i], SIGALRM);
	
#if defined(HAVE_SCRIPT) && defined(HAVE_DHCP)
	/* handle pending lease transitions */
	if (daemon->helperfd != -1)
	  {
	    /* block in writes until all done */
	    if ((i = fcntl(daemon->helperfd, F_GETFL)) != -1)
	      while(retry_send(fcntl(daemon->helperfd, F_SETFL, i & ~O_NONBLOCK)));
	    do {
	      helper_write();
	    } while (!helper_buf_empty() || do_script_run(now));
	    close(daemon->helperfd);
	  }
// #endif
	
	if (daemon->lease_stream)
	  fclose(daemon->lease_stream);

// #ifdef HAVE_DNSSEC
	/* update timestamp file on TERM if time is considered valid */
	if (daemon->back_to_the_future)
	  {
	     if (utimes(daemon->timestamp_file, NULL) == -1)
		my_syslog(LOG_ERR, _("failed to update mtime on %s: %s"), daemon->timestamp_file, strerror(errno));
	  }
// #endif

	if (daemon->runfile)
	  unlink(daemon->runfile);

// #ifdef HAVE_DUMPFILE
	if (daemon->dumpfd != -1)
	  close(daemon->dumpfd);
// #endif
	
	my_syslog(LOG_INFO, _("exiting on receipt of SIGTERM"));
	flush_log();
	exit(EC_GOOD);
      }
}

static void poll_resolv(int force, int do_reload, time_t now)
{
  struct resolvc *res, *latest;
  struct stat statbuf;
  time_t last_change = 0;
  /* There may be more than one possible file. 
     Go through and find the one which changed _last_.
     Warn of any which can't be read. */

  if (daemon->port == 0 || option_bool(OPT_NO_POLL))
    return;
  
  for (latest = NULL, res = daemon->resolv_files; res; res = res->next)
    if (stat(res->name, &statbuf) == -1)
      {
	if (force)
	  {
	    res->mtime = 0; 
	    continue;
	  }

	if (!res->logged)
	  my_syslog(LOG_WARNING, _("failed to access %s: %s"), res->name, strerror(errno));
	res->logged = 1;
	
	if (res->mtime != 0)
	  { 
	    /* existing file evaporated, force selection of the latest
	       file even if its mtime hasn't changed since we last looked */
	    poll_resolv(1, do_reload, now);
	    return;
	  }
      }
    else
      {
	res->logged = 0;
	if (force || (statbuf.st_mtime != res->mtime || statbuf.st_ino != res->ino))
          {
            res->mtime = statbuf.st_mtime;
	    res->ino = statbuf.st_ino;
	    if (difftime(statbuf.st_mtime, last_change) > 0.0)
	      {
		last_change = statbuf.st_mtime;
		latest = res;
	      }
	  }
      }
  
  if (latest)
    {
      static int warned = 0;
      if (reload_servers(latest->name))
	{
	  my_syslog(LOG_INFO, _("reading %s"), latest->name);
	  warned = 0;
	  check_servers(0);
	  if (option_bool(OPT_RELOAD) && do_reload)
	    clear_cache_and_reload(now);
	}
      else 
	{
	  /* If we're delaying things, we don't call check_servers(), but 
	     reload_servers() may have deleted some servers, rendering the server_array
	     invalid, so just rebuild that here. Once reload_servers() succeeds,
	     we call check_servers() above, which calls build_server_array itself. */
	  build_server_array();
	  latest->mtime = 0;
	  if (!warned)
	    {
	      my_syslog(LOG_WARNING, _("no servers found in %s, will retry"), latest->name);
	      warned = 1;
	    }
	}
    }
}       

void clear_cache_and_reload(time_t now)
{
  (void)now;

  if (daemon->port != 0)
    cache_reload();
  
// #ifdef HAVE_DHCP
  if (daemon->dhcp || daemon->doing_dhcp6)
    {
      if (option_bool(OPT_ETHERS))
	dhcp_read_ethers();
      reread_dhcp();
      dhcp_update_configs(daemon->dhcp_conf);
      lease_update_from_configs(); 
      lease_update_file(now); 
      lease_update_dns(1);
    }
// #ifdef HAVE_DHCP6
  else if (daemon->doing_ra)
    /* Not doing DHCP, so no lease system, manage 
       alarms for ra only */
    send_alarm(periodic_ra(now), now);
// #endif
// #endif
}

static void set_dns_listeners(void)
{
  struct serverfd *serverfdp;
  struct listener *listener;
  struct randfd_list *rfl;
  i: i32;
  
// #ifdef HAVE_TFTP
  int  tftp = 0;
  struct tftp_transfer *transfer;
  if (!option_bool(OPT_SINGLE_PORT))
    for (transfer = daemon->tftp_trans; transfer; transfer = transfer->next)
      {
	tftp++;
	poll_listen(transfer->sockfd, POLLIN);
      }
// #endif
  
  for (serverfdp = daemon->sfds; serverfdp; serverfdp = serverfdp->next)
    poll_listen(serverfdp->fd, POLLIN);
    
  for (i = 0; i < daemon->numrrand; i++)
    if (daemon->randomsocks[i].refcount != 0)
      poll_listen(daemon->randomsocks[i].fd, POLLIN);

  /* Check overflow random sockets too. */
  for (rfl = daemon->rfl_poll; rfl; rfl = rfl->next)
    poll_listen(rfl->rfd->fd, POLLIN);
  
  /* check to see if we have free tcp process slots. */
  for (i = MAX_PROCS - 1; i >= 0; i--)
    if (daemon->tcp_pids[i] == 0 && daemon->tcp_pipes[i] == -1)
      break;

  for (listener = daemon->listeners; listener; listener = listener->next)
    {
      if (listener->fd != -1)
	poll_listen(listener->fd, POLLIN);
      
      /* Only listen for TCP connections when a process slot
	 is available. Death of a child goes through the select loop, so
	 we don't need to explicitly arrange to wake up here,
	 we'll be called again when a slot becomes available. */
      if  (listener->tcpfd != -1 && i >= 0)
	poll_listen(listener->tcpfd, POLLIN);
      
// #ifdef HAVE_TFTP
      /* tftp == 0 in single-port mode. */
      if (tftp <= daemon->tftp_max && listener->tftpfd != -1)
	poll_listen(listener->tftpfd, POLLIN);
// #endif
    }
  
  if (!option_bool(OPT_DEBUG))
    for (i = 0; i < MAX_PROCS; i++)
      if (daemon->tcp_pipes[i] != -1)
	poll_listen(daemon->tcp_pipes[i], POLLIN);
}

static void check_dns_listeners(time_t now)
{
  struct serverfd *serverfdp;
  struct listener *listener;
  struct randfd_list *rfl;
  i: i32;
  int pipefd[2];
  
  for (serverfdp = daemon->sfds; serverfdp; serverfdp = serverfdp->next)
    if (poll_check(serverfdp->fd, POLLIN))
      reply_query(serverfdp->fd, now);
  
  for (i = 0; i < daemon->numrrand; i++)
    if (daemon->randomsocks[i].refcount != 0 && 
	poll_check(daemon->randomsocks[i].fd, POLLIN))
      reply_query(daemon->randomsocks[i].fd, now);

  /* Check overflow random sockets too. */
  for (rfl = daemon->rfl_poll; rfl; rfl = rfl->next)
    if (poll_check(rfl->rfd->fd, POLLIN))
      reply_query(rfl->rfd->fd, now);

  /* Races. The child process can die before we read all of the data from the
     pipe, or vice versa. Therefore send tcp_pids to zero when we wait() the 
     process, and tcp_pipes to -1 and close the FD when we read the last
     of the data - indicated by cache_recv_insert returning zero.
     The order of these events is indeterminate, and both are needed
     to free the process slot. Once the child process has gone, poll()
     returns POLLHUP, not POLLIN, so have to check for both here. */
  if (!option_bool(OPT_DEBUG))
    for (i = 0; i < MAX_PROCS; i++)
      if (daemon->tcp_pipes[i] != -1 &&
	  poll_check(daemon->tcp_pipes[i], POLLIN | POLLHUP) &&
	  !cache_recv_insert(now, daemon->tcp_pipes[i]))
	{
	  close(daemon->tcp_pipes[i]);
	  daemon->tcp_pipes[i] = -1;	
	}
	
  for (listener = daemon->listeners; listener; listener = listener->next)
    {
      if (listener->fd != -1 && poll_check(listener->fd, POLLIN))
	receive_query(listener, now); 
      
// #ifdef HAVE_TFTP
      if (listener->tftpfd != -1 && poll_check(listener->tftpfd, POLLIN))
	tftp_request(listener, now);
// #endif

      /* check to see if we have a free tcp process slot.
	 Note that we can't assume that because we had
	 at least one a poll() time, that we still do.
	 There may be more waiting connections after
	 poll() returns then free process slots. */
      for (i = MAX_PROCS - 1; i >= 0; i--)
	if (daemon->tcp_pids[i] == 0 && daemon->tcp_pipes[i] == -1)
	  break;

      if (listener->tcpfd != -1 && i >= 0 && poll_check(listener->tcpfd, POLLIN))
	{
	  int confd, client_ok = 1;
	  struct irec *iface = NULL;
	  pid_t p;
	  union mysockaddr tcp_addr;
	  socklen_t tcp_len = sizeof(union mysockaddr);

	  while ((confd = accept(listener->tcpfd, NULL, NULL)) == -1 && errno == EINTR);
	  
	  if (confd == -1)
	    continue;
	  
	  if (getsockname(confd, (struct sockaddr *)&tcp_addr, &tcp_len) == -1)
	    {
	      close(confd);
	      continue;
	    }
	  
	  /* Make sure that the interface list is up-to-date.
	     
	     We do this here as we may need the results below, and
	     the DNS code needs them for --interface-name stuff.

	     Multiple calls to enumerate_interfaces() per select loop are
	     inhibited, so calls to it in the child process (which doesn't select())
	     have no effect. This avoids two processes reading from the same
	     netlink fd and screwing the pooch entirely.
	  */
 
	  enumerate_interfaces(0);
	  
	  if (option_bool(OPT_NOWILD))
	    iface = listener->iface; /* May be NULL */
	  else 
	    {
	      if_index: i32;
	      char intr_name[IF_NAMESIZE];
	      
	      /* if we can find the arrival interface, check it's one that's allowed */
	      if ((if_index = tcp_interface(confd, tcp_addr.sa.sa_family)) != 0 &&
		  indextoname(listener->tcpfd, if_index, intr_name))
		{
		  union all_addr addr;
		  
		  if (tcp_addr.sa.sa_family == AF_INET6)
		    addr.addr6 = tcp_addr.in6.sin6_addr;
		  else
		    addr.addr4 = tcp_addr.in.sin_addr;
		  
		  for (iface = daemon->interfaces; iface; iface = iface->next)
		    if (iface->index == if_index &&
		        iface->addr.sa.sa_family == tcp_addr.sa.sa_family)
		      break;
		  
		  if (!iface && !loopback_exception(listener->tcpfd, tcp_addr.sa.sa_family, &addr, intr_name))
		    client_ok = 0;
		}
	      
	      if (option_bool(OPT_CLEVERBIND))
		iface = listener->iface; /* May be NULL */
	      else
		{
		  /* Check for allowed interfaces when binding the wildcard address:
		     we do this by looking for an interface with the same address as 
		     the local address of the TCP connection, then looking to see if that's
		     an allowed interface. As a side effect, we get the netmask of the
		     interface too, for localisation. */
		  
		  for (iface = daemon->interfaces; iface; iface = iface->next)
		    if (sockaddr_isequal(&iface->addr, &tcp_addr))
		      break;
		  
		  if (!iface)
		    client_ok = 0;
		}
	    }
	  
	  if (!client_ok)
	    {
	      shutdown(confd, SHUT_RDWR);
	      close(confd);
	    }
	  else if (!option_bool(OPT_DEBUG) && pipe(pipefd) == 0 && (p = fork()) != 0)
	    {
	      close(pipefd[1]); /* parent needs read pipe end. */
	      if (p == -1)
		close(pipefd[0]);
	      else
		{
// #ifdef HAVE_LINUX_NETWORK
		  /* The child process inherits the netlink socket, 
		     which it never uses, but when the parent (us) 
		     uses it in the future, the answer may go to the 
		     child, resulting in the parent blocking
		     forever awaiting the result. To avoid this
		     the child closes the netlink socket, but there's
		     a nasty race, since the parent may use netlink
		     before the child has done the close.
		     
		     To avoid this, the parent blocks here until a 
		     single byte comes back up the pipe, which
		     is sent by the child after it has closed the
		     netlink socket. */
		  
		  unsigned char a;
		  read_write(pipefd[0], &a, 1, 1);
// #endif

		  /* i holds index of free slot */
		  daemon->tcp_pids[i] = p;
		  daemon->tcp_pipes[i] = pipefd[0];
		}
	      close(confd);

	      /* The child can use up to TCP_MAX_QUERIES ids, so skip that many. */
	      daemon->log_id += TCP_MAX_QUERIES;
	    }
	  else
	    {
	      unsigned char *buff;
	      struct server *s; 
	      flags: i32;
	      netmask: in_addr;
	      auth_dns: i32;
	   
	      if (iface)
		{
		  netmask = iface->netmask;
		  auth_dns = iface->dns_auth;
		}
	      else
		{
		  netmask.s_addr = 0;
		  auth_dns = 0;
		}

	      /* Arrange for SIGALRM after CHILD_LIFETIME seconds to
		 terminate the process. */
	      if (!option_bool(OPT_DEBUG))
		{
// #ifdef HAVE_LINUX_NETWORK
		  /* See comment above re: netlink socket. */
		  unsigned char a = 0;

		  close(daemon->netlinkfd);
		  read_write(pipefd[1], &a, 1, 0);
// #endif
		  alarm(CHILD_LIFETIME);
		  close(pipefd[0]); /* close read end in child. */
		  daemon->pipe_to_parent = pipefd[1];
		}

	      /* start with no upstream connections. */
	      for (s = daemon->servers; s; s = s->next)
		 s->tcpfd = -1; 
	      
	      /* The connected socket inherits non-blocking
		 attribute from the listening socket. 
		 Reset that here. */
	      if ((flags = fcntl(confd, F_GETFL, 0)) != -1)
		while(retry_send(fcntl(confd, F_SETFL, flags & ~O_NONBLOCK)));
	      
	      buff = tcp_request(confd, now, &tcp_addr, netmask, auth_dns);
	       
	      if (buff)
		free(buff);
	      
	      for (s = daemon->servers; s; s = s->next)
		if (s->tcpfd != -1)
		  {
		    shutdown(s->tcpfd, SHUT_RDWR);
		    close(s->tcpfd);
		  }
	      
	      if (!option_bool(OPT_DEBUG))
		{
		  close(daemon->pipe_to_parent);
		  flush_log();
		  _exit(0);
		}
	    }
	}
    }
}

// #ifdef HAVE_DHCP
int make_icmp_sock(void)
{
  fd: i32;
  int zeroopt = 0;

  if ((fd = socket (AF_INET, SOCK_RAW, IPPROTO_ICMP)) != -1)
    {
      if (!fix_fd(fd) ||
	  setsockopt(fd, SOL_SOCKET, SO_DONTROUTE, &zeroopt, sizeof(zeroopt)) == -1)
	{
	  close(fd);
	  fd = -1;
	}
    }

  return fd;
}

int icmp_ping(struct in_addr addr)
{
  /* Try and get an ICMP echo from a machine. */

  fd: i32;
  struct sockaddr_in saddr;
  struct { 
    struct ip ip;
    struct icmp icmp;
  } packet;
  unsigned short id = rand16();
  unsigned int i, j;
  int gotreply = 0;

#if defined(HAVE_LINUX_NETWORK) || defined (HAVE_SOLARIS_NETWORK)
  if ((fd = make_icmp_sock()) == -1)
    return 0;
#else
  int opt = 2000;
  fd = daemon->dhcp_icmp_fd;
  setsockopt(fd, SOL_SOCKET, SO_RCVBUF, &opt, sizeof(opt));
// #endif

  saddr.sin_family = AF_INET;
  saddr.sin_port = 0;
  saddr.sin_addr = addr;
// #ifdef HAVE_SOCKADDR_SA_LEN
  saddr.sin_len = sizeof(struct sockaddr_in);
// #endif
  
  memset(&packet.icmp, 0, sizeof(packet.icmp));
  packet.icmp.icmp_type = ICMP_ECHO;
  packet.icmp.icmp_id = id;
  for (j = 0, i = 0; i < sizeof(struct icmp) / 2; i++)
    j += ((u16 *)&packet.icmp)[i];
  while (j>>16)
    j = (j & 0xffff) + (j >> 16);  
  packet.icmp.icmp_cksum = (j == 0xffff) ? j : ~j;
  
  while (retry_send(sendto(fd, (char *)&packet.icmp, sizeof(struct icmp), 0, 
			   (struct sockaddr *)&saddr, sizeof(saddr))));
  
  gotreply = delay_dhcp(dnsmasq_time(), PING_WAIT, fd, addr.s_addr, id);

#if defined(HAVE_LINUX_NETWORK) || defined(HAVE_SOLARIS_NETWORK)
  close(fd);
#else
  opt = 1;
  setsockopt(fd, SOL_SOCKET, SO_RCVBUF, &opt, sizeof(opt));
// #endif

  return gotreply;
}

int delay_dhcp(time_t start, int sec, int fd, uint32_t addr, unsigned short id)
{
  /* Delay processing DHCP packets for "sec" seconds counting from "start".
     If "fd" is not -1 it will stop waiting if an ICMP echo reply is received
     from "addr" with ICMP ID "id" and return 1 */

  /* Note that whilst waiting, we check for
     (and service) events on the DNS and TFTP  sockets, (so doing that
     better not use any resources our caller has in use...)
     but we remain deaf to signals or further DHCP packets. */

  /* There can be a problem using dnsmasq_time() to end the loop, since
     it's not monotonic, and can go backwards if the system clock is
     tweaked, leading to the code getting stuck in this loop and
     ignoring DHCP requests. To fix this, we check to see if select returned
     as a result of a timeout rather than a socket becoming available. We
     only allow this to happen as many times as it takes to get to the wait time
     in quarter-second chunks. This provides a fallback way to end loop. */

  int rc, timeout_count;
  time_t now;

  for (now = dnsmasq_time(), timeout_count = 0;
       (difftime(now, start) <= (float)sec) && (timeout_count < sec * 4);)
    {
      poll_reset();
      if (fd != -1)
        poll_listen(fd, POLLIN);
      set_dns_listeners();
      set_log_writer();
      
// #ifdef HAVE_DHCP6
      if (daemon->doing_ra)
	poll_listen(daemon->icmp6fd, POLLIN); 
// #endif
      
      rc = do_poll(250);
      
      if (rc < 0)
	continue;
      else if (rc == 0)
	timeout_count++;

      now = dnsmasq_time();
      
      check_log_writer(0);
      check_dns_listeners(now);
      
// #ifdef HAVE_DHCP6
      if (daemon->doing_ra && poll_check(daemon->icmp6fd, POLLIN))
	icmp6_packet(now);
// #endif
      
// #ifdef HAVE_TFTP
      check_tftp_listeners(now);
// #endif

      if (fd != -1)
        {
          struct {
            struct ip ip;
            struct icmp icmp;
          } packet;
          struct sockaddr_in faddr;
          socklen_t len = sizeof(faddr);
	  
          if (poll_check(fd, POLLIN) &&
	      recvfrom(fd, &packet, sizeof(packet), 0, (struct sockaddr *)&faddr, &len) == sizeof(packet) &&
	      addr == faddr.sin_addr.s_addr &&
	      packet.icmp.icmp_type == ICMP_ECHOREPLY &&
	      packet.icmp.icmp_seq == 0 &&
	      packet.icmp.icmp_id == id)
	    return 1;
	}
    }

  return 0;
}
// #endif /* HAVE_DHCP */
