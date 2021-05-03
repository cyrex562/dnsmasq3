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






/* Implement logging to /dev/log asynchronously. If syslogd is 
   making DNS lookups through dnsmasq, and dnsmasq blocks awaiting
   syslogd, then the two daemons can deadlock. We get around this
   by not blocking when talking to syslog, instead we queue up to 
   MAX_LOGS messages. If more are queued, they will be dropped,
   and the drop event itself logged. */

/* The "wire" protocol for logging is defined in RFC 3164 */

/* From RFC 3164 */
pub const MAX_MESSAGE: u32 = 1024;

/* defaults in case we die() before we log_start() */
//  int log_fac = LOG_DAEMON;
// let mut log_stderr: i32 = 0;
// let mut echo_stderr: i32 = 0;
//  int log_fd = -1;
// let mut log_to_file: i32 = 0;
// let mut entries_alloced: i32 = 0;
// let mut entries_lost: i32 = 0;
// let mut connection_good: i32 = 1;
// let mut max_logs: i32 = 0;
//  int connection_type = SOCK_DGRAM;

pub struct log_entry {
  // offset: i32, length;
  pub offset: i32,
  pub length: i32,
  // pid_t pid; /* to avoid duplicates over a fork */
  pub pid: pid_t,
  // let mut next: log_entry;
  // char payload[MAX_MESSAGE];
  pub payload: String,
}

//  struct log_entry *entries = NULL;
//  struct log_entry *free_entries = NULL;


pub fn log_start(ent_pw: passwd, errfd: i32)
{
  let mut ret: i32 = 0;

  echo_stderr = daemon.opt_debug;

  if (daemon.log_fac != -1) {
    log_fac = daemon.log_fac;
  }
  else if (daemon.opt_debug) {
    log_fac = LOG_LOCAL0;
  }


  if (daemon.log_file)
    { 
      log_to_file = 1;
      daemon.max_logs = 0;
      if (strcmp(daemon.log_file, "-") == 0)
	{
	  log_stderr = 1;
	  echo_stderr = 0;
	  log_fd = dup(STDERR_FILENO);
	}
    }
  
  max_logs = daemon.max_logs;

  if (!log_reopen(daemon.log_file))
    {
      send_event(errfd, EVENT_LOG_ERR, errno, if daemon.log_file {daemon.log_file} else { ""});
      _exit(0);
    }

  /* if queuing is inhibited, make sure we allocate
     the one required buffer now. */
  if (max_logs == 0)
    {  
      // free_entries = safe_malloc(mem::sizeof::<log_entry>());
      // free_entries.next = NULL;
      entries_alloced = 1;
    }

  /* If we're running as root and going to change uid later,
     change the ownership here so that the file is always owned by
     the dnsmasq user. Then logrotate can just copy the owner.
     Failure of the chown call is OK, (for instance when started as non-root) */
  if (log_to_file && !log_stderr && ent_pw && ent_pw.pw_uid != 0 && 
      fchown(log_fd, ent_pw.pw_uid, -1) != 0)
    ret = errno;

  return ret;
}

pub fn log_reopen(log_file: &mut String)
{
  if (!log_stderr)
    {      
      if (log_fd != -1)
	close(log_fd);
      
      /* NOTE: umask is set to 022 by the time this gets called */
      
      if (log_file)
	log_fd = open(log_file, O_WRONLY|O_CREAT|O_APPEND, S_IRUSR|S_IWUSR|S_IRGRP);      
      else
	{
	  /* Solaris logging is "different", /dev/log is not unix-domain socket.
	     Just leave log_fd == -1 and use the vsyslog call for everything.... */
	  return 1;

	  let mut flags: i32;
	  log_fd = socket(AF_UNIX, connection_type, 0);
	  
	  /* if max_logs is zero, leave the socket blocking */
	  if (log_fd != -1 && max_logs != 0 && (flags = fcntl(log_fd, F_GETFL)) != -1)
	    fcntl(log_fd, F_SETFL, flags | O_NONBLOCK);

	}
    }
  
  return log_fd != -1;
}

pub fn free_entry()
{
  struct log_entry *tmp = entries;
  entries = tmp.next;
  tmp.next = free_entries;
  free_entries = tmp;
}      

pub fn log_write()
{
  src: usize;
   
  while (entries)
    {
      /* The data in the payload is written with a terminating zero character 
	 and the length reflects this. For a stream connection we need to 
	 send the zero as a record terminator, but this isn't done for a 
	 datagram connection, so treat the length as one less than reality 
	 to elide the zero. If we're logging to a file, turn the zero into 
	 a newline, and leave the length alone. */
      let mut len_adjust: i32 = 0;

      if (log_to_file) {
	entries.payload[entries.offset + entries.length - 1] = '\n';
      }
      else if (connection_type == SOCK_DGRAM) {
	len_adjust = 1;
      }

      /* Avoid duplicates over a fork() */
      if (entries.pid != getpid())
	{
	  free_entry();
	  continue;
	}

      connection_good = 1;

      if ((rc = write(log_fd, entries.payload + entries.offset, entries.length - len_adjust)) != -1)
	{
	  entries.length -= rc;
	  entries.offset += rc;
	  if (entries.length == len_adjust)
	    {
	      free_entry();
	      if (entries_lost != 0)
		{
		  int e = entries_lost;
		  entries_lost = 0; /* avoid wild recursion */
		  my_syslog(LOG_WARNING, format!("overflow: {} log entries lost"), e);
		}	  
	    }
	  continue;
	}
      
      if (errno == EINTR)
	continue;

      if (errno == EAGAIN || errno == EWOULDBLOCK)
	return; /* syslogd busy, go again when select() or poll() says so */
      
      if (errno == ENOBUFS)
	{
	  connection_good = 0;
	  return;
	}

      /* errors handling after this assumes sockets */ 
      if (!log_to_file)
	{
	  /* Once a stream socket hits EPIPE, we have to close and re-open
	     (we ignore SIGPIPE) */
	  if (errno == EPIPE)
	    {
	      if (log_reopen(NULL))
		continue;
	    }
	  else if (errno == ECONNREFUSED || 
		   errno == ENOTCONN || 
		   errno == EDESTADDRREQ || 
		   errno == ECONNRESET)
	    {
	      /* socket went (syslogd down?), try and reconnect. If we fail,
		 stop trying until the next call to my_syslog() 
		 ECONNREFUSED . connection went down
		 ENOTCONN . nobody listening
		 (ECONNRESET, EDESTADDRREQ are *BSD equivalents) */
	      
	      struct sockaddr_un logaddr;
	      
 HAVE_SOCKADDR_SA_LEN
	      logaddr.sun_len = sizeof(logaddr) - sizeof(logaddr.sun_path) + strlen(_PATH_LOG) + 1; 

	      logaddr.sun_family = AF_UNIX;
	      safe_strncpy(logaddr.sun_path, _PATH_LOG, sizeof(logaddr.sun_path));
	      
	      /* Got connection back? try again. */
	      if (connect(log_fd, &logaddr, sizeof(logaddr)) != -1)
		continue;
	      
	      /* errors from connect which mean we should keep trying */
	      if (errno == ENOENT || 
		  errno == EALREADY || 
		  errno == ECONNREFUSED ||
		  errno == EISCONN || 
		  errno == EINTR ||
		  errno == EAGAIN || 
		  errno == EWOULDBLOCK)
		{
		  /* try again on next syslog() call */
		  connection_good = 0;
		  return;
		}
	      
	      /* try the other sort of socket... */
	      if (errno == EPROTOTYPE)
		{
		  connection_type = connection_type == SOCK_DGRAM ? SOCK_STREAM : SOCK_DGRAM;
		  if (log_reopen(NULL))
		    continue;
		}
	    }
	}

      /* give up - fall back to syslog() - this handles out-of-space
	 when logging to a file, for instance. */
      log_fd = -1;
      my_syslog(LOG_CRIT, format!("log failed: {}"), strerror(errno));
      return;
    }
}

/* priority is one of LOG_DEBUG, LOG_INFO, LOG_NOTICE, etc. See sys/syslog.h.
   OR'd to priority can be MS_TFTP, MS_DHCP, ... to be able to do log separation between
   DNS, DHCP and TFTP services.
*/
void my_syslog(priority: i32, const format: &mut String, ...)
{
  va_list ap;
  let mut entry: log_entry;
  time_now: time::Instant;
  char *p;
  len: usize;
  pid_t pid = getpid();
  char *func = "";

  if ((LOG_FACMASK & priority) == MS_TFTP)
    func = "-tftp";
  else if ((LOG_FACMASK & priority) == MS_DHCP)
    func = "-dhcp";
  else if ((LOG_FACMASK & priority) == MS_SCRIPT)
    func = "-script";
	    
 LOG_PRI
  priority = LOG_PRI(priority);

  /* Solaris doesn't have LOG_PRI */
  priority &= LOG_PRIMASK;


  if (echo_stderr) 
    {
      fprintf(stderr, "dnsmasq{}: ", func);
      va_start(ap, format);
      vfprintf(stderr, format, ap);
      va_end(ap);
      fputc('\n', stderr);
    }

  if (log_fd == -1)
    {
 __ANDROID__
      /* do android-specific logging. 
	 log_fd is always -1 on Android except when logging to a file. */
      let mut alog_lvl: i32;
      
      if (priority <= LOG_ERR)
	alog_lvl = ANDROID_LOG_ERROR;
      else if (priority == LOG_WARNING)
	alog_lvl = ANDROID_LOG_WARN;
      else if (priority <= LOG_INFO)
	alog_lvl = ANDROID_LOG_INFO;
      else
	alog_lvl = ANDROID_LOG_DEBUG;

      va_start(ap, format);
      __android_log_vprint(alog_lvl, "dnsmasq", format, ap);
      va_end(ap);

      /* fall-back to syslog if we die during startup or 
	 fail during running (always on Solaris). */
      let mut isopen: i32 = 0;

      if (!isopen)
	{
	  openlog("dnsmasq", LOG_PID, log_fac);
	  isopen = 1;
	}
      va_start(ap, format);  
      vsyslog(priority, format, ap);
      va_end(ap);


      return;
    }
  
  if ((entry = free_entries))
    free_entries = entry.next;
  else if (entries_alloced < max_logs && (entry = malloc(sizeof(struct log_entry))))
    entries_alloced +=1;
  
  if (!entry)
    entries_lost +=1;
  else
    {
      /* add to end of list, consumed from the start */
      entry.next = NULL;
      if (!entries)
	entries = entry;
      else
	{
	  let mut tmp: log_entry;
	  for (tmp = entries; tmp.next; tmp = tmp.next);
	  tmp.next = entry;
	}
      
      time(&time_now);
      p = entry.payload;
      if (!log_to_file)
	p += sprintf(p, "<{}>", priority | log_fac);

      /* Omit timestamp for default daemontools situation */
      if (!log_stderr || !daemon.opt_no_fork) 
	p += sprintf(p, "%.15s ", ctime(&time_now) + 4);
      
      p += sprintf(p, "dnsmasq{}[{}]: ", func, pid);
        
      len = p - entry.payload;
      va_start(ap, format);  
      len += vsnprintf(p, MAX_MESSAGE - len, format, ap) + 1; /* include zero-terminator */
      va_end(ap);
      entry.length = len > MAX_MESSAGE ? MAX_MESSAGE : len;
      entry.offset = 0;
      entry.pid = pid;
    }
  
  /* almost always, logging won't block, so try and write this now,
     to save collecting too many log messages during a select loop. */
  log_write();
  
  /* Since we're doing things asynchronously, a cache-dump, for instance,
     can now generate log lines very fast. With a small buffer (desirable),
     that means it can overflow the log-buffer very quickly,
     so that the cache dump becomes mainly a count of how many lines 
     overflowed. To avoid this, we delay here, the delay is controlled 
     by queue-occupancy, and grows exponentially. The delay is limited to (2^8)ms.
     The scaling stuff ensures that when the queue is bigger than 8, the delay
     only occurs for the last 8 entries. Once the queue is full, we stop delaying
     to preserve performance.
  */

  if (entries && max_logs != 0)
    {
      let mut d: i32;
      
      for (d = 0,entry = entries; entry; entry = entry.next, d++);
      
      if (d == max_logs)
	d = 0;
      else if (max_logs > 8)
	d -= max_logs - 8;

      if (d > 0)
	{
	  struct timespec waiter;
	  waiter.tv_sec = 0;
	  waiter.tv_nsec = 1000000 << (d - 1); /* 1 ms */
	  nanosleep(&waiter, NULL);
      
	  /* Have another go now */
	  log_write();
	}
    } 
}

void set_log_writer()
{
  if (entries && log_fd != -1 && connection_good)
    poll_listen(log_fd, POLLOUT);
}

void check_log_writer(int force)
{
  if (log_fd != -1 && (force || poll_check(log_fd, POLLOUT)))
    log_write();
}

void flush_log()
{
  /* write until queue empty, but don't loop forever if there's
   no connection to the syslog in existence */
  while (log_fd != -1)
    {
      struct timespec waiter;
      log_write();
      if (!entries || !connection_good)
	{
	  close(log_fd);	
	  break;
	}
      waiter.tv_sec = 0;
      waiter.tv_nsec = 1000000; /* 1 ms */
      nanosleep(&waiter, NULL);
    }
}

void die(message: &mut String, arg1: &mut String, exit_code: i32)
{
  char *errmess = strerror(errno);
  
  if (!arg1)
    arg1 = errmess;

  if (!log_stderr)
    {
      echo_stderr = 1; /* print as well as log when we die.... */
      fputc('\n', stderr); /* prettyfy  startup-script message */
    }
  my_syslog(LOG_CRIT, message, arg1, errmess);
  echo_stderr = 0;
  my_syslog(LOG_CRIT, format!("FAILED to start up"));
  flush_log();
  
  exit(exit_code);
}
