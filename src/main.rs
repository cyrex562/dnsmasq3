use std::ptr::null_mut;
use libc::{_SC_OPEN_MAX, c_char, c_long, gid_t, group, passwd, SIG_IGN, sigaction, SIGALRM, SIGCHLD, sigemptyset, SIGHUP, SIGINT, SIGPIPE, sigset_t, SIGTERM, SIGUSR1, SIGUSR2, sysconf, time_t, uid_t, umask};
use crate::app_context::AppContext;
use crate::dns_protocol::PACKETSZ;
use crate::surf_rand::rand_init;

mod arp;
mod auth;
mod blockdata_ops;
mod bpf;
mod cache;
mod config;
mod conntrack;
mod crypto;
mod dbus;
mod dhcp;
mod dhcp6;
mod dhcp6_protocol;
mod dhcp_common;
mod dhcp_lease_time;
mod dhcp_protocol;
mod dhcp_release;
mod dhcp_release6;
mod dnsmasq;
mod dnsmasq_defines;
mod dnssec;
mod domain;
mod domain_match;
mod dump;
mod edns0;
mod forward;
mod hash_questions;
mod helper;
mod inotify;
mod ip6addr;
mod ipset;
mod lease;
mod log;
mod dnsmasq_loop;
mod metrics_h;
mod metrics;
mod netlink;
mod network;
mod nftset;
mod option;
mod outpacket;
mod pattern;
mod poll;
mod radv;
mod radv_protocol;
mod rfc1035;
mod rfc2131;
mod rrfilter;
mod slaac;
mod tables;
mod tftp;
mod ubus;
mod util;
mod arp_record;
mod app_context;
mod dhcp_constants;
mod dhcp_packet;
mod udp_hdr;
mod watch;
mod iface_param;
mod match_param;
mod dhcp6_option;
mod dhcp6_iaaddr_option;
mod dhcp6_iana_option;
mod dhcp6_packet;
mod event_desc;
mod all_addr;
mod bogus_addr;
mod doctor;
mod mx_srv_record;
mod naptr;
mod txt_record;
mod ptr_record;
mod cname;
mod ds_config;
mod addrlist;
mod auth_zone;
mod host_record;
mod interface_name;
mod bigname;
mod blockdata;
mod crec;
mod mysockaddr;
mod server_fd;
mod rand_fd;
mod rand_fd_list;
mod server;
mod serv_addr4;
mod serv_addr6;
mod serv_local;
mod rebind_domain;
mod ipsets;
mod allow_list;
mod irec;
mod listener;
mod iname;
mod mysubnet;
mod resolvc;
mod hosts_file;
mod dyndir;
mod frec;
mod cname_struct;
mod target_union;
mod dhcp_lease;
mod dhcp_netid;
mod dhcp_netid_list;
mod tag_if;
mod delay_config;
mod hwaddr_config;
mod dhcp_config;
mod dhcp_opt;
mod dhcp_boot;
mod dhcp_match_name;
mod pxe_service;
mod surf_rand;
mod dhcp_vendor;
mod dhcp_pxe_vendor;
mod daemon;
mod dhcp_mac;
mod dhcp_bridge;
mod cond_domain;
mod ra_interface;
mod dhcp_context;
mod shared_network;
mod ping_result;
mod tftp_file;
mod tftp_transfer;
mod addr_list;
mod tftp_prefix;
mod dhcp_relay;
mod server_details;
mod pcap_hdr_s;
mod pcap_rec_hdr_s;
mod subnet_opt;
mod dns_protocol;
mod dns_header;
mod script_data;
mod my_nlattr;
mod my_nfgenmsg;
mod ra_param;
mod search_param;
mod alias_param;
mod ping_packet;
mod ra_packet;
mod neigh_packet;
mod prefix_opt;
mod rfc3315;
mod state;

fn main() {

    let mut app_ctx: AppContext = Default::default();

        let mut now: time_t = 0;
    let mut sigact: sigaction = sigaction{
        sa_sigaction: 0,
        sa_mask: sigset_t{ __val: [0;16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut if_tmp: *mut iname = null_mut();

    let mut piperead: i32 = 0;
    let mut pipefd: [i32;2] = [0;2];
    let mut err_pipe: [i32;2] = [0;2];

    let mut ent_pw: *mut passwd = null_mut();

    let mut script_uid: uid_t = 0;
    let mut script_gid: gid_t = 0;

    let mut gp: *mut group = null_mut();
    let mut i: c_long = 0;
    let mut max_fd: c_long = unsafe { sysconf(_SC_OPEN_MAX) };

    let mut baduser: *mut c_char = null_mut();

    let mut log_err: i32 = 0;

    let mut chown_warn: i32 = 0;

    // linux networking cap net raw only
    if cfg!(linux) {
        let mut hdr: cap_user_header_t: null_mut();
        let mut data: cap_user_data_t: null_mut();
    }
    let mut need_cap_net_admin: i32 = 0;
    let mut need_cap_net_raw: i32 = 0;
    let mut need_cap_net_bind_service: i32 = 0;
    let mut bound_device: *mut c_char = null_mut();
    let mut did_bind: i32 = 0;
    let mut serv: *mut server = null_mut();
    let mut netlink_warn: *mut c_char = null_mut();
    // end cap net raw only
    let mut bind_fallback: i32 = 0;
    let mut context: *mut dhcp_context = null_mut();
    let mut relay: *mut dhcp_relay = null_mut();
    let mut tftp_prefix_missing: i32 = 0;




// #if defined(HAVE_IDN) || defined(HAVE_LIBIDN2) || defined(LOCALEDIR)
//   setlocale(LC_ALL, "");
// // #endif
// // #ifdef LOCALEDIR
//   bindtextdomain("dnsmasq", LOCALEDIR);
//   textdomain("dnsmasq");
// // #endif

  sigact.sa_handler = sig_handler;
  sigact.sa_flags = 0;
    unsafe { sigemptyset(&mut sigact.sa_mask); }
    unsafe { sigaction(SIGUSR1, &sigact, null_mut()); }
    unsafe { sigaction(SIGUSR2, &sigact, null_mut()); }
    unsafe { sigaction(SIGHUP, &sigact, null_mut()); }
    unsafe { sigaction(SIGTERM, &sigact, null_mut()); }
    unsafe { sigaction(SIGALRM, &sigact, null_mut()); }
    unsafe { sigaction(SIGCHLD, &sigact, null_mut()); }
    unsafe { sigaction(SIGINT, &sigact, null_mut()); }

  /* ignore SIGPIPE */
  sigact.sa_handler = SIG_IGN;
    unsafe { sigaction(SIGPIPE, &sigact, null_mut()); }

    unsafe { umask(022); } /* known umask, create leases and pid files as 0644 */

  rand_init(); /* Must precede read_opts() */



  read_opts(argc, argv, compile_opts);

// #ifdef HAVE_LINUX_NETWORK
  app_ctx.daemon.kernel_version = kernel_version();
// #endif

  if (app_ctx.daemon.edns_pktsz < PACKETSZ) {
      app_ctx.daemon.edns_pktsz = PACKETSZ;
  }

  /* Min buffer size: we check after adding each record, so there must be
     memory for the largest packet, and the largest record so the
     min for DNS is PACKETSZ+MAXDNAME+RRFIXEDSZ which is < 1000.
     This might be increased is EDNS packet size if greater than the minimum. */
  app_ctx.daemon.packet_buff_sz = app_ctx.daemon.edns_pktsz + MAXDNAME + RRFIXEDSZ;
  app_ctx.daemon.packet = safe_malloc(app_ctx.daemon.packet_buff_sz);

  if (option_bool(OPT_EXTRALOG)) {
      app_ctx.daemon.addrbuff2 = safe_malloc(ADDRSTRLEN);
  }

// #ifdef HAVE_DNSSEC
  if (option_bool(OPT_DNSSEC_VALID))
    {
      /* Note that both /000 and '.' are allowed within labels. These get
	 represented in presentation format using NAME_ESCAPE as an escape
	 character when in DNSSEC mode.
	 In theory, if all the characters in a name were /000 or
	 '.' or NAME_ESCAPE then all would have to be escaped, so the
	 presentation format would be twice as long as the spec.

	 daemon->namebuff was previously allocated by the option-reading
	 code before we knew if we're in DNSSEC mode, so reallocate here. */
      free(app_ctx.daemon.namebuff);
      daemon->namebuff = safe_malloc(MAXDNAME * 2);
      daemon->keyname = safe_malloc(MAXDNAME * 2);
      daemon->workspacename = safe_malloc(MAXDNAME * 2);
      /* one char flag per possible RR in answer section (may get extended). */
      daemon->rr_status_sz = 64;
      daemon->rr_status = safe_malloc(sizeof(*daemon->rr_status) * daemon->rr_status_sz);
    }
// #endif

#if defined(HAVE_CONNTRACK) && defined(HAVE_UBUS)
  /* CONNTRACK UBUS code uses this buffer, so if not allocated above,
     we need to allocate it here. */
  if (option_bool(OPT_CMARK_ALST_EN) && !daemon->workspacename)
    daemon->workspacename = safe_malloc(MAXDNAME);
// #endif

// #ifdef HAVE_DHCP
  if (!daemon->lease_file)
    {
      if (daemon->dhcp || daemon->dhcp6)
	daemon->lease_file = LEASEFILE;
    }
// #endif

  /* Ensure that at least stdin, stdout and stderr (fd 0, 1, 2) exist,
     otherwise file descriptors we create can end up being 0, 1, or 2
     and then get accidentally closed later when we make 0, 1, and 2
     open to /dev/null. Normally we'll be started with 0, 1 and 2 open,
     but it's not guaranteed. By opening /dev/null three times, we
     ensure that we're not using those fds for real stuff. */
  for (i = 0; i < 3; i++)
    open("/dev/null", O_RDWR);

  /* Close any file descriptors we inherited apart from std{in|out|err} */
  close_fds(max_fd, -1, -1, -1);

// #endif HAVE_LINUX_NETWORK
#  if !(defined(IP_RECVDSTADDR) && defined(IP_RECVIF) && defined(IP_SENDSRCADDR))
  if (!option_bool(OPT_NOWILD))
    {
      bind_fallback = 1;
      set_option_bool(OPT_NOWILD);
    }
#  endif

  /* -- bind-dynamic not supported on !Linux, fall back to --bind-interfaces */
  if (option_bool(OPT_CLEVERBIND))
    {
      bind_fallback = 1;
      set_option_bool(OPT_NOWILD);
      reset_option_bool(OPT_CLEVERBIND);
    }
// #endif

// #endif HAVE_INOTIFY
  if (daemon->dynamic_dirs)
    die(_("dhcp-hostsdir, dhcp-optsdir and hostsdir are not supported on this platform"), NULL, EC_BADCONF);
// #endif

  if (option_bool(OPT_DNSSEC_VALID))
    {
// #ifdef HAVE_DNSSEC
      struct ds_config *ds;

      /* Must have at least a root trust anchor, or the DNSSEC code
	 can loop forever. */
      for (ds = daemon->ds; ds; ds = ds->next)
	if (ds->name[0] == 0)
	  break;

      if (!ds)
	die(_("no root trust anchor provided for DNSSEC"), NULL, EC_BADCONF);

      if (daemon->cachesize < CACHESIZ)
	die(_("cannot reduce cache size from default when DNSSEC enabled"), NULL, EC_BADCONF);
#else
      die(_("DNSSEC not available: set HAVE_DNSSEC in src/config.h"), NULL, EC_BADCONF);
// #endif
    }

// #endif HAVE_TFTP
  if (option_bool(OPT_TFTP))
    die(_("TFTP server not available: set HAVE_TFTP in src/config.h"), NULL, EC_BADCONF);
// #endif

// #ifdef HAVE_CONNTRACK
  if (option_bool(OPT_CONNTRACK))
    {
      if (daemon->query_port != 0 || daemon->osport)
	die (_("cannot use --conntrack AND --query-port"), NULL, EC_BADCONF);

      need_cap_net_admin = 1;
    }
#else
  if (option_bool(OPT_CONNTRACK))
    die(_("conntrack support not available: set HAVE_CONNTRACK in src/config.h"), NULL, EC_BADCONF);
// #endif

// #ifdef HAVE_SOLARIS_NETWORK
  if (daemon->max_logs != 0)
    die(_("asynchronous logging is not available under Solaris"), NULL, EC_BADCONF);
// #endif

// #ifdef __ANDROID__
  if (daemon->max_logs != 0)
    die(_("asynchronous logging is not available under Android"), NULL, EC_BADCONF);
// #endif

// #endif HAVE_AUTH
  if (daemon->auth_zones)
    die(_("authoritative DNS not available: set HAVE_AUTH in src/config.h"), NULL, EC_BADCONF);
// #endif

// #endif HAVE_LOOP
  if (option_bool(OPT_LOOP_DETECT))
    die(_("loop detection not available: set HAVE_LOOP in src/config.h"), NULL, EC_BADCONF);
// #endif

// #endif HAVE_UBUS
  if (option_bool(OPT_UBUS))
    die(_("Ubus not available: set HAVE_UBUS in src/config.h"), NULL, EC_BADCONF);
// #endif

  /* Handle only one of min_port/max_port being set. */
  if (daemon->min_port != 0 && daemon->max_port == 0)
    daemon->max_port = MAX_PORT;

  if (daemon->max_port != 0 && daemon->min_port == 0)
    daemon->min_port = MIN_PORT;

  if (daemon->max_port < daemon->min_port)
    die(_("max_port cannot be smaller than min_port"), NULL, EC_BADCONF);

  if (daemon->max_port != 0 &&
      daemon->max_port - daemon->min_port + 1 < daemon->randport_limit)
    die(_("port_limit must not be larger than available port range"), NULL, EC_BADCONF);

  now = dnsmasq_time();

  if (daemon->auth_zones)
    {
      if (!daemon->authserver)
	die(_("--auth-server required when an auth zone is defined."), NULL, EC_BADCONF);

      /* Create a serial at startup if not configured. */
// #ifdef HAVE_BROKEN_RTC
      if (daemon->soa_sn == 0)
	die(_("zone serial must be configured in --auth-soa"), NULL, EC_BADCONF);
#else
      if (daemon->soa_sn == 0)
	daemon->soa_sn = now;
// #endif
    }

// #ifdef HAVE_DHCP6
  if (daemon->dhcp6)
    {
      daemon->doing_ra = option_bool(OPT_RA);

      for (context = daemon->dhcp6; context; context = context->next)
	{
	  if (context->flags & CONTEXT_DHCP)
	    daemon->doing_dhcp6 = 1;
	  if (context->flags & CONTEXT_RA)
	    daemon->doing_ra = 1;
#if !defined(HAVE_LINUX_NETWORK) && !defined(HAVE_BSD_NETWORK)
	  if (context->flags & CONTEXT_TEMPLATE)
	    die (_("dhcp-range constructor not available on this platform"), NULL, EC_BADCONF);
// #endif
	}
    }
// #endif

// #ifdef HAVE_DHCP
  /* Note that order matters here, we must call lease_init before
     creating any file descriptors which shouldn't be leaked
     to the lease-script init process. We need to call common_init
     before lease_init to allocate buffers it uses.
     The script subsystem relies on DHCP buffers, hence the last two
     conditions below. */
  if (daemon->dhcp || daemon->doing_dhcp6 || daemon->relay4 ||
      daemon->relay6 || option_bool(OPT_TFTP) || option_bool(OPT_SCRIPT_ARP))
    {
      dhcp_common_init();
      if (daemon->dhcp || daemon->doing_dhcp6)
	lease_init(now);
    }

  if (daemon->dhcp || daemon->relay4)
    {
      dhcp_init();
#   ifdef HAVE_LINUX_NETWORK
      if (!option_bool(OPT_NO_PING))
	need_cap_net_raw = 1;
      need_cap_net_admin = 1;
#   endif
    }

#  ifdef HAVE_DHCP6
  if (daemon->doing_ra || daemon->doing_dhcp6 || daemon->relay6)
    {
      ra_init(now);
#   ifdef HAVE_LINUX_NETWORK
      need_cap_net_raw = 1;
      need_cap_net_admin = 1;
#   endif
    }

  if (daemon->doing_dhcp6 || daemon->relay6)
    dhcp6_init();
#  endif

// #endif

// #ifdef HAVE_IPSET
  if (daemon->ipsets)
    {
      ipset_init();
#  ifdef HAVE_LINUX_NETWORK
      need_cap_net_admin = 1;
#  endif
    }
// #endif

// #ifdef HAVE_NFTSET
  if (daemon->nftsets)
    {
      nftset_init();
#  ifdef HAVE_LINUX_NETWORK
      need_cap_net_admin = 1;
#  endif
    }
// #endif

#if  defined(HAVE_LINUX_NETWORK)
  netlink_warn = netlink_init();
#elif defined(HAVE_BSD_NETWORK)
  route_init();
// #endif

  if (option_bool(OPT_NOWILD) && option_bool(OPT_CLEVERBIND))
    die(_("cannot set --bind-interfaces and --bind-dynamic"), NULL, EC_BADCONF);

  if (!enumerate_interfaces(1) || !enumerate_interfaces(0))
    die(_("failed to find list of interfaces: %s"), NULL, EC_MISC);

  if (option_bool(OPT_NOWILD) || option_bool(OPT_CLEVERBIND))
    {
      create_bound_listeners(1);

      if (!option_bool(OPT_CLEVERBIND))
	for (if_tmp = daemon->if_names; if_tmp; if_tmp = if_tmp->next)
	  if (if_tmp->name && !if_tmp->used)
	    die(_("unknown interface %s"), if_tmp->name, EC_BADNET);

#if defined(HAVE_LINUX_NETWORK) && defined(HAVE_DHCP)
      /* after enumerate_interfaces()  */
      bound_device = whichdevice();

      if ((did_bind = bind_dhcp_devices(bound_device)) & 2)
	die(_("failed to set SO_BINDTODEVICE on DHCP socket: %s"), NULL, EC_BADNET);
// #endif
    }
  else
    create_wildcard_listeners();

// #ifdef HAVE_DHCP6
  /* after enumerate_interfaces() */
  if (daemon->doing_dhcp6 || daemon->relay6 || daemon->doing_ra)
    join_multicast(1);

  /* After netlink_init() and before create_helper() */
  lease_make_duid(now);
// #endif

  if (daemon->port != 0)
    {
      cache_init();
      blockdata_init();
      hash_questions_init();

      /* Scale random socket pool by ftabsize, but
	 limit it based on available fds. */
      daemon->numrrand = daemon->ftabsize/2;
      if (daemon->numrrand > max_fd/3)
	daemon->numrrand = max_fd/3;
      /* safe_malloc returns zero'd memory */
      daemon->randomsocks = safe_malloc(daemon->numrrand * sizeof(struct randfd));
    }

// #ifdef HAVE_INOTIFY
  if ((daemon->port != 0 || daemon->dhcp || daemon->doing_dhcp6)
      && (!option_bool(OPT_NO_RESOLV) || daemon->dynamic_dirs))
    inotify_dnsmasq_init();
  else
    daemon->inotifyfd = -1;
// #endif

  if (daemon->dump_file)
// #ifdef HAVE_DUMPFILE
    dump_init();
  else
    daemon->dumpfd = -1;
#else
  die(_("Packet dumps not available: set HAVE_DUMP in src/config.h"), NULL, EC_BADCONF);
// #endif

  if (option_bool(OPT_DBUS))
// #ifdef HAVE_DBUS
    {
      char *err;
      if ((err = dbus_init()))
	die(_("DBus error: %s"), err, EC_MISC);
    }
#else
  die(_("DBus not available: set HAVE_DBUS in src/config.h"), NULL, EC_BADCONF);
// #endif

  if (option_bool(OPT_UBUS))
// #ifdef HAVE_UBUS
    {
      char *err;
      if ((err = ubus_init()))
	die(_("UBus error: %s"), err, EC_MISC);
    }
#else
  die(_("UBus not available: set HAVE_UBUS in src/config.h"), NULL, EC_BADCONF);
// #endif

  if (daemon->port != 0)
    pre_allocate_sfds();

#if defined(HAVE_SCRIPT)
  /* Note getpwnam returns static storage */
  if ((daemon->dhcp || daemon->dhcp6) &&
      daemon->scriptuser &&
      (daemon->lease_change_command || daemon->luascript))
    {
      struct passwd *scr_pw;

      if ((scr_pw = getpwnam(daemon->scriptuser)))
	{
	  script_uid = scr_pw->pw_uid;
	  script_gid = scr_pw->pw_gid;
	 }
      else
	baduser = daemon->scriptuser;
    }
// #endif

  if (daemon->username && !(ent_pw = getpwnam(daemon->username)))
    baduser = daemon->username;
  else if (daemon->groupname && !(gp = getgrnam(daemon->groupname)))
    baduser = daemon->groupname;

  if (baduser)
    die(_("unknown user or group: %s"), baduser, EC_BADCONF);

  /* implement group defaults, "dip" if available, or group associated with uid */
  if (!daemon->group_set && !gp)
    {
      if (!(gp = getgrnam(CHGRP)) && ent_pw)
	gp = getgrgid(ent_pw->pw_gid);

      /* for error message */
      if (gp)
	daemon->groupname = gp->gr_name;
    }

#if defined(HAVE_LINUX_NETWORK)
  /* We keep CAP_NETADMIN (for ARP-injection) and
     CAP_NET_RAW (for icmp) if we're doing dhcp,
     if we have yet to bind ports because of DAD,
     or we're doing it dynamically, we need CAP_NET_BIND_SERVICE. */
  if ((is_dad_listeners() || option_bool(OPT_CLEVERBIND)) &&
      (option_bool(OPT_TFTP) || (daemon->port != 0 && daemon->port <= 1024)))
    need_cap_net_bind_service = 1;

  /* usptream servers which bind to an interface call SO_BINDTODEVICE
     for each TCP connection, so need CAP_NET_RAW */
  for (serv = daemon->servers; serv; serv = serv->next)
    if (serv->interface[0] != 0)
      need_cap_net_raw = 1;

  /* If we're doing Dbus or UBus, the above can be set dynamically,
     (as can ports) so always (potentially) needed. */
// #ifdef HAVE_DBUS
  if (option_bool(OPT_DBUS))
    {
      need_cap_net_bind_service = 1;
      need_cap_net_raw = 1;
    }
// #endif

// #ifdef HAVE_UBUS
  if (option_bool(OPT_UBUS))
    {
      need_cap_net_bind_service = 1;
      need_cap_net_raw = 1;
    }
// #endif

  /* determine capability API version here, while we can still
     call safe_malloc */
  int capsize = 1; /* for header version 1 */
  char *fail = NULL;

  hdr = safe_malloc(sizeof(*hdr));

  /* find version supported by kernel */
  memset(hdr, 0, sizeof(*hdr));
  capget(hdr, NULL);

  if (hdr->version != LINUX_CAPABILITY_VERSION_1)
    {
      /* if unknown version, use largest supported version (3) */
      if (hdr->version != LINUX_CAPABILITY_VERSION_2)
	hdr->version = LINUX_CAPABILITY_VERSION_3;
      capsize = 2;
    }

  data = safe_malloc(sizeof(*data) * capsize);
  capget(hdr, data); /* Get current values, for verification */

  if (need_cap_net_admin && !(data->permitted & (1 << CAP_NET_ADMIN)))
    fail = "NET_ADMIN";
  else if (need_cap_net_raw && !(data->permitted & (1 << CAP_NET_RAW)))
    fail = "NET_RAW";
  else if (need_cap_net_bind_service && !(data->permitted & (1 << CAP_NET_BIND_SERVICE)))
    fail = "NET_BIND_SERVICE";

  if (fail)
    die(_("process is missing required capability %s"), fail, EC_MISC);

  /* Now set bitmaps to set caps after daemonising */
  memset(data, 0, sizeof(*data) * capsize);

  if (need_cap_net_admin)
    data->effective |= (1 << CAP_NET_ADMIN);
  if (need_cap_net_raw)
    data->effective |= (1 << CAP_NET_RAW);
  if (need_cap_net_bind_service)
    data->effective |= (1 << CAP_NET_BIND_SERVICE);

  data->permitted = data->effective;
// #endif

  /* Use a pipe to carry signals and other events back to the event loop
     in a race-free manner and another to carry errors to daemon-invoking process */
  safe_pipe(pipefd, 1);

  piperead = pipefd[0];
  pipewrite = pipefd[1];
  /* prime the pipe to load stuff first time. */
  send_event(pipewrite, EVENT_INIT, 0, NULL);

  err_pipe[1] = -1;

  if (!option_bool(OPT_DEBUG))
    {
      /* The following code "daemonizes" the process.
	 See Stevens section 12.4 */

      if (chdir("/") != 0)
	die(_("cannot chdir to filesystem root: %s"), NULL, EC_MISC);

      if (!option_bool(OPT_NO_FORK))
	{
	  pid_t pid;

	  /* pipe to carry errors back to original process.
	     When startup is complete we close this and the process terminates. */
	  safe_pipe(err_pipe, 0);

	  if ((pid = fork()) == -1)
	    /* fd == -1 since we've not forked, never returns. */
	    send_event(-1, EVENT_FORK_ERR, errno, NULL);

	  if (pid != 0)
	    {
	      struct event_desc ev;
	      char *msg;

	      /* close our copy of write-end */
	      close(err_pipe[1]);

	      /* check for errors after the fork */
	      if (read_event(err_pipe[0], &ev, &msg))
		fatal_event(&ev, msg);

	      _exit(EC_GOOD);
	    }

	  close(err_pipe[0]);

	  /* NO calls to die() from here on. */

	  setsid();

	  if ((pid = fork()) == -1)
	    send_event(err_pipe[1], EVENT_FORK_ERR, errno, NULL);

	  if (pid != 0)
	    _exit(0);
	}

      /* write pidfile _after_ forking ! */
      if (daemon->runfile)
	{
	  int fd, err = 0;

	  sprintf(daemon->namebuff, "%d\n", (int) getpid());

	  /* Explanation: Some installations of dnsmasq (eg Debian/Ubuntu) locate the pid-file
	     in a directory which is writable by the non-privileged user that dnsmasq runs as. This
	     allows the daemon to delete the file as part of its shutdown. This is a security hole to the
	     extent that an attacker running as the unprivileged  user could replace the pidfile with a
	     symlink, and have the target of that symlink overwritten as root next time dnsmasq starts.

	     The following code first deletes any existing file, and then opens it with the O_EXCL flag,
	     ensuring that the open() fails should there be any existing file (because the unlink() failed,
	     or an attacker exploited the race between unlink() and open()). This ensures that no symlink
	     attack can succeed.

	     Any compromise of the non-privileged user still theoretically allows the pid-file to be
	     replaced whilst dnsmasq is running. The worst that could allow is that the usual
	     "shutdown dnsmasq" shell command could be tricked into stopping any other process.

	     Note that if dnsmasq is started as non-root (eg for testing) it silently ignores
	     failure to write the pid-file.
	  */

	  unlink(daemon->runfile);

	  if ((fd = open(daemon->runfile, O_WRONLY|O_CREAT|O_TRUNC|O_EXCL, S_IWUSR|S_IRUSR|S_IRGRP|S_IROTH)) == -1)
	    {
	      /* only complain if started as root */
	      if (getuid() == 0)
		err = 1;
	    }
	  else
	    {
	      /* We're still running as root here. Change the ownership of the PID file
		 to the user we will be running as. Note that this is not to allow
		 us to delete the file, since that depends on the permissions
		 of the directory containing the file. That directory will
		 need to by owned by the dnsmasq user, and the ownership of the
		 file has to match, to keep systemd >273 happy. */
	      if (getuid() == 0 && ent_pw && ent_pw->pw_uid != 0 && fchown(fd, ent_pw->pw_uid, ent_pw->pw_gid) == -1)
		chown_warn = errno;

	      if (!read_write(fd, (unsigned char *)daemon->namebuff, strlen(daemon->namebuff), 0))
		err = 1;
	      else
		{
		  if (close(fd) == -1)
		    err = 1;
		}
	    }

	  if (err)
	    {
	      send_event(err_pipe[1], EVENT_PIDFILE, errno, daemon->runfile);
	      _exit(0);
	    }
	}
    }

   log_err = log_start(ent_pw, err_pipe[1]);

   if (!option_bool(OPT_DEBUG))
     {
       /* open  stdout etc to /dev/null */
       int nullfd = open("/dev/null", O_RDWR);
       if (nullfd != -1)
	 {
	   dup2(nullfd, STDOUT_FILENO);
	   dup2(nullfd, STDERR_FILENO);
	   dup2(nullfd, STDIN_FILENO);
	   close(nullfd);
	 }
     }

   /* if we are to run scripts, we need to fork a helper before dropping root. */
  daemon->helperfd = -1;
// #ifdef HAVE_SCRIPT
  if ((daemon->dhcp ||
       daemon->dhcp6 ||
       daemon->relay6 ||
       option_bool(OPT_TFTP) ||
       option_bool(OPT_SCRIPT_ARP)) &&
      (daemon->lease_change_command || daemon->luascript))
      daemon->helperfd = create_helper(pipewrite, err_pipe[1], script_uid, script_gid, max_fd);
// #endif

  if (!option_bool(OPT_DEBUG) && getuid() == 0)
    {
      int bad_capabilities = 0;
      gid_t dummy;

      /* remove all supplementary groups */
      if (gp &&
	  (setgroups(0, &dummy) == -1 ||
	   setgid(gp->gr_gid) == -1))
	{
	  send_event(err_pipe[1], EVENT_GROUP_ERR, errno, daemon->groupname);
	  _exit(0);
	}

      if (ent_pw && ent_pw->pw_uid != 0)
	{
#if defined(HAVE_LINUX_NETWORK)
	  /* Need to be able to drop root. */
	  data->effective |= (1 << CAP_SETUID);
	  data->permitted |= (1 << CAP_SETUID);
	  /* Tell kernel to not clear capabilities when dropping root */
	  if (capset(hdr, data) == -1 || prctl(PR_SET_KEEPCAPS, 1, 0, 0, 0) == -1)
	    bad_capabilities = errno;

#elif defined(HAVE_SOLARIS_NETWORK)
	  /* http://developers.sun.com/solaris/articles/program_privileges.html */
	  priv_set_t *priv_set;

	  if (!(priv_set = priv_str_to_set("basic", ",", NULL)) ||
	      priv_addset(priv_set, PRIV_NET_ICMPACCESS) == -1 ||
	      priv_addset(priv_set, PRIV_SYS_NET_CONFIG) == -1)
	    bad_capabilities = errno;

	  if (priv_set && bad_capabilities == 0)
	    {
	      priv_inverse(priv_set);

	      if (setppriv(PRIV_OFF, PRIV_LIMIT, priv_set) == -1)
		bad_capabilities = errno;
	    }

	  if (priv_set)
	    priv_freeset(priv_set);

// #endif

	  if (bad_capabilities != 0)
	    {
	      send_event(err_pipe[1], EVENT_CAP_ERR, bad_capabilities, NULL);
	      _exit(0);
	    }

	  /* finally drop root */
	  if (setuid(ent_pw->pw_uid) == -1)
	    {
	      send_event(err_pipe[1], EVENT_USER_ERR, errno, daemon->username);
	      _exit(0);
	    }

// #ifdef HAVE_LINUX_NETWORK
	  data->effective &= ~(1 << CAP_SETUID);
	  data->permitted &= ~(1 << CAP_SETUID);

	  /* lose the setuid capability */
	  if (capset(hdr, data) == -1)
	    {
	      send_event(err_pipe[1], EVENT_CAP_ERR, errno, NULL);
	      _exit(0);
	    }
// #endif

	}
    }

// #ifdef HAVE_LINUX_NETWORK
  free(hdr);
  free(data);
  if (option_bool(OPT_DEBUG))
    prctl(PR_SET_DUMPABLE, 1, 0, 0, 0);
// #endif

// #ifdef HAVE_TFTP
  if (option_bool(OPT_TFTP))
    {
      DIR *dir;
      struct tftp_prefix *p;

      if (daemon->tftp_prefix)
	{
	  if (!((dir = opendir(daemon->tftp_prefix))))
	    {
	      tftp_prefix_missing = 1;
	      if (!option_bool(OPT_TFTP_NO_FAIL))
	        {
	          send_event(err_pipe[1], EVENT_TFTP_ERR, errno, daemon->tftp_prefix);
	          _exit(0);
	        }
	    }
	  else
	    closedir(dir);
	}

      for (p = daemon->if_prefix; p; p = p->next)
	{
	  p->missing = 0;
	  if (!((dir = opendir(p->prefix))))
	    {
	      p->missing = 1;
	      if (!option_bool(OPT_TFTP_NO_FAIL))
		{
		  send_event(err_pipe[1], EVENT_TFTP_ERR, errno, p->prefix);
		  _exit(0);
		}
	    }
	  else
	    closedir(dir);
	}
    }
// #endif

  if (daemon->port == 0)
    my_syslog(LOG_INFO, _("started, version %s DNS disabled"), VERSION);
  else
    {
      if (daemon->cachesize != 0)
	{
	  my_syslog(LOG_INFO, _("started, version %s cachesize %d"), VERSION, daemon->cachesize);
	  if (daemon->cachesize > 10000)
	    my_syslog(LOG_WARNING, _("cache size greater than 10000 may cause performance issues, and is unlikely to be useful."));
	}
      else
	my_syslog(LOG_INFO, _("started, version %s cache disabled"), VERSION);

      if (option_bool(OPT_LOCAL_SERVICE))
	my_syslog(LOG_INFO, _("DNS service limited to local subnets"));
    }

  my_syslog(LOG_INFO, _("compile time options: %s"), compile_opts);

  if (chown_warn != 0)
    my_syslog(LOG_WARNING, "chown of PID file %s failed: %s", daemon->runfile, strerror(chown_warn));

// #ifdef HAVE_DBUS
  if (option_bool(OPT_DBUS))
    {
      if (daemon->dbus)
	my_syslog(LOG_INFO, _("DBus support enabled: connected to system bus"));
      else
	my_syslog(LOG_INFO, _("DBus support enabled: bus connection pending"));
    }
// #endif

// #ifdef HAVE_UBUS
  if (option_bool(OPT_UBUS))
    {
      if (daemon->ubus)
        my_syslog(LOG_INFO, _("UBus support enabled: connected to system bus"));
      else
        my_syslog(LOG_INFO, _("UBus support enabled: bus connection pending"));
    }
// #endif

// #ifdef HAVE_DNSSEC
  if (option_bool(OPT_DNSSEC_VALID))
    {
      rc: i32;
      struct ds_config *ds;

      /* Delay creating the timestamp file until here, after we've changed user, so that
	 it has the correct owner to allow updating the mtime later.
	 This means we have to report fatal errors via the pipe. */
      if ((rc = setup_timestamp()) == -1)
	{
	  send_event(err_pipe[1], EVENT_TIME_ERR, errno, daemon->timestamp_file);
	  _exit(0);
	}

      if (option_bool(OPT_DNSSEC_IGN_NS))
	my_syslog(LOG_INFO, _("DNSSEC validation enabled but all unsigned answers are trusted"));
      else
	my_syslog(LOG_INFO, _("DNSSEC validation enabled"));

      daemon->dnssec_no_time_check = option_bool(OPT_DNSSEC_TIME);
      if (option_bool(OPT_DNSSEC_TIME) && !daemon->back_to_the_future)
	my_syslog(LOG_INFO, _("DNSSEC signature timestamps not checked until receipt of SIGINT"));

      if (rc == 1)
	my_syslog(LOG_INFO, _("DNSSEC signature timestamps not checked until system time valid"));

      for (ds = daemon->ds; ds; ds = ds->next)
	my_syslog(LOG_INFO, _("configured with trust anchor for %s keytag %u"),
		  ds->name[0] == 0 ? "<root>" : ds->name, ds->keytag);
    }
// #endif

  if (log_err != 0)
    my_syslog(LOG_WARNING, _("warning: failed to change owner of %s: %s"),
	      daemon->log_file, strerror(log_err));

// #endif HAVE_LINUX_NETWORK
  if (bind_fallback)
    my_syslog(LOG_WARNING, _("setting --bind-interfaces option because of OS limitations"));
// #endif

  if (option_bool(OPT_NOWILD))
    warn_bound_listeners();
  else if (!option_bool(OPT_CLEVERBIND))
    warn_wild_labels();

  warn_int_names();

  if (!option_bool(OPT_NOWILD))
    for (if_tmp = daemon->if_names; if_tmp; if_tmp = if_tmp->next)
      if (if_tmp->name && !if_tmp->used)
	my_syslog(LOG_WARNING, _("warning: interface %s does not currently exist"), if_tmp->name);

  if (daemon->port != 0 && option_bool(OPT_NO_RESOLV))
    {
      if (daemon->resolv_files && !daemon->resolv_files->is_default)
	my_syslog(LOG_WARNING, _("warning: ignoring resolv-file flag because no-resolv is set"));
      daemon->resolv_files = NULL;
      if (!daemon->servers)
	my_syslog(LOG_WARNING, _("warning: no upstream servers configured"));
    }

  if (daemon->max_logs != 0)
    my_syslog(LOG_INFO, _("asynchronous logging enabled, queue limit is %d messages"), daemon->max_logs);


// #ifdef HAVE_DHCP
  for (context = daemon->dhcp; context; context = context->next)
    log_context(AF_INET, context);

  for (relay = daemon->relay4; relay; relay = relay->next)
    log_relay(AF_INET, relay);

#  ifdef HAVE_DHCP6
  for (context = daemon->dhcp6; context; context = context->next)
    log_context(AF_INET6, context);

  for (relay = daemon->relay6; relay; relay = relay->next)
    log_relay(AF_INET6, relay);

  if (daemon->doing_dhcp6 || daemon->doing_ra)
    dhcp_construct_contexts(now);

  if (option_bool(OPT_RA))
    my_syslog(MS_DHCP | LOG_INFO, _("IPv6 router advertisement enabled"));
#  endif

#  ifdef HAVE_LINUX_NETWORK
  if (did_bind)
    my_syslog(MS_DHCP | LOG_INFO, _("DHCP, sockets bound exclusively to interface %s"), bound_device);

  if (netlink_warn)
    my_syslog(LOG_WARNING, netlink_warn);
#  endif

  /* after dhcp_construct_contexts */
  if (daemon->dhcp || daemon->doing_dhcp6)
    lease_find_interfaces(now);
// #endif

// #ifdef HAVE_TFTP
  if (option_bool(OPT_TFTP))
    {
      struct tftp_prefix *p;

      my_syslog(MS_TFTP | LOG_INFO, "TFTP %s%s %s %s",
		daemon->tftp_prefix ? _("root is ") : _("enabled"),
		daemon->tftp_prefix ? daemon->tftp_prefix : "",
		option_bool(OPT_TFTP_SECURE) ? _("secure mode") : "",
		option_bool(OPT_SINGLE_PORT) ? _("single port mode") : "");

      if (tftp_prefix_missing)
	my_syslog(MS_TFTP | LOG_WARNING, _("warning: %s inaccessible"), daemon->tftp_prefix);

      for (p = daemon->if_prefix; p; p = p->next)
	if (p->missing)
	   my_syslog(MS_TFTP | LOG_WARNING, _("warning: TFTP directory %s inaccessible"), p->prefix);

      /* This is a guess, it assumes that for small limits,
	 disjoint files might be served, but for large limits,
	 a single file will be sent to may clients (the file only needs
	 one fd). */

      max_fd -= 30 + daemon->numrrand; /* use other than TFTP */

      if (max_fd < 0)
	max_fd = 5;
      else if (max_fd < 100 && !option_bool(OPT_SINGLE_PORT))
	max_fd = max_fd/2;
      else
	max_fd = max_fd - 20;

      /* if we have to use a limited range of ports,
	 that will limit the number of transfers */
      if (daemon->start_tftp_port != 0 &&
	  daemon->end_tftp_port - daemon->start_tftp_port + 1 < max_fd)
	max_fd = daemon->end_tftp_port - daemon->start_tftp_port + 1;

      if (daemon->tftp_max > max_fd)
	{
	  daemon->tftp_max = max_fd;
	  my_syslog(MS_TFTP | LOG_WARNING,
		    _("restricting maximum simultaneous TFTP transfers to %d"),
		    daemon->tftp_max);
	}
    }
// #endif

  /* finished start-up - release original process */
  if (err_pipe[1] != -1)
    close(err_pipe[1]);

  if (daemon->port != 0)
    check_servers(0);

  pid = getpid();

  daemon->pipe_to_parent = -1;
  for (i = 0; i < MAX_PROCS; i++)
    daemon->tcp_pipes[i] = -1;

// #ifdef HAVE_INOTIFY
  /* Using inotify, have to select a resolv file at startup */
  poll_resolv(1, 0, now);
// #endif

  while (1)
    {
      int timeout = fast_retry(now);

      poll_reset();

      /* Whilst polling for the dbus, or doing a tftp transfer, wake every quarter second */
      if ((daemon->tftp_trans || (option_bool(OPT_DBUS) && !daemon->dbus)) &&
	  (timeout == -1 || timeout > 250))
	timeout = 250;

      /* Wake every second whilst waiting for DAD to complete */
      else if (is_dad_listeners() &&
	       (timeout == -1 || timeout > 1000))
	timeout = 1000;

      set_dns_listeners();

// #ifdef HAVE_DBUS
      if (option_bool(OPT_DBUS))
	set_dbus_listeners();
// #endif

// #ifdef HAVE_UBUS
      if (option_bool(OPT_UBUS))
        set_ubus_listeners();
// #endif

// #ifdef HAVE_DHCP
#  if defined(HAVE_LINUX_NETWORK)
      if (bind_dhcp_devices(bound_device) & 2)
	{
	  static int warned = 0;
	  if (!warned)
	    {
	      my_syslog(LOG_ERR, _("error binding DHCP socket to device %s"), bound_device);
	      warned = 1;
	    }
	}
# endif
      if (daemon->dhcp || daemon->relay4)
	{
	  poll_listen(daemon->dhcpfd, POLLIN);
	  if (daemon->pxefd != -1)
	    poll_listen(daemon->pxefd, POLLIN);
	}
// #endif

// #ifdef HAVE_DHCP6
      if (daemon->doing_dhcp6 || daemon->relay6)
	poll_listen(daemon->dhcp6fd, POLLIN);

      if (daemon->doing_ra)
	poll_listen(daemon->icmp6fd, POLLIN);
// #endif

// #ifdef HAVE_INOTIFY
      if (daemon->inotifyfd != -1)
	poll_listen(daemon->inotifyfd, POLLIN);
// #endif

#if defined(HAVE_LINUX_NETWORK)
      poll_listen(daemon->netlinkfd, POLLIN);
#elif defined(HAVE_BSD_NETWORK)
      poll_listen(daemon->routefd, POLLIN);
// #endif

      poll_listen(piperead, POLLIN);

// #ifdef HAVE_SCRIPT
#    ifdef HAVE_DHCP
      while (helper_buf_empty() && do_script_run(now));
#    endif

      /* Refresh cache */
      if (option_bool(OPT_SCRIPT_ARP))
	find_mac(NULL, NULL, 0, now);
      while (helper_buf_empty() && do_arp_script_run());

#    ifdef HAVE_TFTP
      while (helper_buf_empty() && do_tftp_script_run());
#    endif

#    ifdef HAVE_DHCP6
      while (helper_buf_empty() && do_snoop_script_run());
#    endif

      if (!helper_buf_empty())
	poll_listen(daemon->helperfd, POLLOUT);
#else
      /* need this for other side-effects */
#    ifdef HAVE_DHCP
      while (do_script_run(now));
#    endif

      while (do_arp_script_run());

#    ifdef HAVE_TFTP
      while (do_tftp_script_run());
#    endif

// #endif


      /* must do this just before do_poll(), when we know no
	 more calls to my_syslog() can occur */
      set_log_writer();

      if (do_poll(timeout) < 0)
	continue;

      now = dnsmasq_time();

      check_log_writer(0);

      /* prime. */
      enumerate_interfaces(1);

      /* Check the interfaces to see if any have exited DAD state
	 and if so, bind the address. */
      if (is_dad_listeners())
	{
	  enumerate_interfaces(0);
	  /* NB, is_dad_listeners() == 1 --> we're binding interfaces */
	  create_bound_listeners(0);
	  warn_bound_listeners();
	}

#if defined(HAVE_LINUX_NETWORK)
      if (poll_check(daemon->netlinkfd, POLLIN))
	netlink_multicast();
#elif defined(HAVE_BSD_NETWORK)
      if (poll_check(daemon->routefd, POLLIN))
	route_sock();
// #endif

// #ifdef HAVE_INOTIFY
      if  (daemon->inotifyfd != -1 && poll_check(daemon->inotifyfd, POLLIN) && inotify_check(now))
	{
	  if (daemon->port != 0 && !option_bool(OPT_NO_POLL))
	    poll_resolv(1, 1, now);
	}
#else
      /* Check for changes to resolv files once per second max. */
      /* Don't go silent for long periods if the clock goes backwards. */
      if (daemon->last_resolv == 0 ||
	  difftime(now, daemon->last_resolv) > 1.0 ||
	  difftime(now, daemon->last_resolv) < -1.0)
	{
	  /* poll_resolv doesn't need to reload first time through, since
	     that's queued anyway. */

	  poll_resolv(0, daemon->last_resolv != 0, now);
	  daemon->last_resolv = now;
	}
// #endif

      if (poll_check(piperead, POLLIN))
	async_event(piperead, now);

// #ifdef HAVE_DBUS
      /* if we didn't create a DBus connection, retry now. */
      if (option_bool(OPT_DBUS))
	{
	  if (!daemon->dbus)
	    {
	      char *err  = dbus_init();

	      if (daemon->dbus)
		my_syslog(LOG_INFO, _("connected to system DBus"));
	      else if (err)
		{
		  my_syslog(LOG_ERR, _("DBus error: %s"), err);
		  reset_option_bool(OPT_DBUS); /* fatal error, stop trying. */
		}
	    }

	  check_dbus_listeners();
	}
// #endif

// #ifdef HAVE_UBUS
      /* if we didn't create a UBus connection, retry now. */
      if (option_bool(OPT_UBUS))
	{
	  if (!daemon->ubus)
	    {
	      char *err = ubus_init();

	      if (daemon->ubus)
		my_syslog(LOG_INFO, _("connected to system UBus"));
	      else if (err)
		{
		  my_syslog(LOG_ERR, _("UBus error: %s"), err);
		  reset_option_bool(OPT_UBUS); /* fatal error, stop trying. */
		}
	    }

	  check_ubus_listeners();
	}
// #endif

      check_dns_listeners(now);

// #ifdef HAVE_TFTP
      check_tftp_listeners(now);
// #endif

// #ifdef HAVE_DHCP
      if (daemon->dhcp || daemon->relay4)
	{
	  if (poll_check(daemon->dhcpfd, POLLIN))
	    dhcp_packet(now, 0);
	  if (daemon->pxefd != -1 && poll_check(daemon->pxefd, POLLIN))
	    dhcp_packet(now, 1);
	}

// #ifdef HAVE_DHCP6
      if ((daemon->doing_dhcp6 || daemon->relay6) && poll_check(daemon->dhcp6fd, POLLIN))
	dhcp6_packet(now);

      if (daemon->doing_ra && poll_check(daemon->icmp6fd, POLLIN))
	icmp6_packet(now);
// #endif

#  ifdef HAVE_SCRIPT
      if (daemon->helperfd != -1 && poll_check(daemon->helperfd, POLLOUT))
	helper_write();
#  endif
// #endif

    }
}
