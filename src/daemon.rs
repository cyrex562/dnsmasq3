
#[derive(Clone)]
pub struct daemon {
  /* datastuctures representing the command-line and 
     config file arguments. All set (including defaults)
     in option.c */

  unsigned int options[OPTION_SIZE];
  struct resolvc default_resolv, *resolv_files;
  time_t last_resolv;
  char *servers_file;
  struct mx_srv_record *mxnames;
  struct naptr *naptr;
  struct txt_record *txt, *rr;
  struct ptr_record *ptr;
  struct host_record *host_records, *host_records_tail;
  struct cname *cnames;
  struct auth_zone *auth_zones;
  struct interface_name *int_names;
  char *mxtarget;
  struct mysubnet *add_subnet4;
  struct mysubnet *add_subnet6;
  char *lease_file;
  char *username, *groupname, *scriptuser;
  char *luascript;
  char *authserver, *hostmaster;
  struct iname *authinterface;
  struct name_list *secondary_forward_server;
  int group_set, osport;
  char *domain_suffix;
  struct cond_domain *cond_domain, *synth_domains;
  char *runfile; 
  char *lease_change_command;
  struct iname *if_names, *if_addrs, *if_except, *dhcp_except, *auth_peers, *tftp_interfaces;
  struct bogus_addr *bogus_addr, *ignore_addr;
  struct server *servers, *servers_tail, *local_domains, **serverarray;
  struct rebind_domain *no_rebind;
  server_has_wildcard: i32;
  int serverarraysz, serverarrayhwm;
  struct ipsets *ipsets, *nftsets;
  u32 allowlist_mask;
  struct allowlist *allowlists;
  log_fac: i32; /* log facility */
  char *log_file; /* optional log file */
  max_logs: i32;  /* queue limit */
  randport_limit: i32; /* Maximum number of source ports for query. */
  int cachesize, ftabsize;
  int port, query_port, min_port, max_port;
  unsigned long local_ttl, neg_ttl, max_ttl, min_cache_ttl, max_cache_ttl, auth_ttl, dhcp_ttl, use_dhcp_ttl;
  char *dns_client_id;
  u32 umbrella_org;
  u32 umbrella_asset;
  u8 umbrella_device[8];
  host_index: i32;
  struct hostsfile *addn_hosts;
  struct dhcp_context *dhcp, *dhcp6;
  struct ra_interface *ra_interfaces;
  struct dhcp_config *dhcp_conf;
  struct dhcp_opt *dhcp_opts, *dhcp_match, *dhcp_opts6, *dhcp_match6;
  struct dhcp_match_name *dhcp_name_match;
  struct dhcp_pxe_vendor *dhcp_pxe_vendors;
  struct dhcp_vendor *dhcp_vendors;
  struct dhcp_mac *dhcp_macs;
  struct dhcp_boot *boot_config;
  struct pxe_service *pxe_services;
  struct tag_if *tag_if; 
  struct addr_list *override_relays;
  struct dhcp_relay *relay4, *relay6;
  struct delay_config *delay_conf;
  override: i32;
  enable_pxe: i32;
  int doing_ra, doing_dhcp6;
  struct dhcp_netid_list *dhcp_ignore, *dhcp_ignore_names, *dhcp_gen_names; 
  struct dhcp_netid_list *force_broadcast, *bootp_dynamic;
  struct hostsfile *dhcp_hosts_file, *dhcp_opts_file;
  struct dyndir *dynamic_dirs;
  int dhcp_max, tftp_max, tftp_mtu;
  int dhcp_server_port, dhcp_client_port;
  int start_tftp_port, end_tftp_port; 
  unsigned min_leasetime: i32;
  struct doctor *doctors;
  unsigned short edns_pktsz;
  char *tftp_prefix; 
  struct tftp_prefix *if_prefix; /* per-interface TFTP prefixes */
  unsigned int duid_enterprise, duid_config_len;
  unsigned char *duid_config;
  char *dbus_name;
  char *ubus_name;
  char *dump_file;
  dump_mask: i32;
  unsigned long soa_sn, soa_refresh, soa_retry, soa_expiry;
  u32 metrics[__METRIC_MAX];
  int fast_retry_time, fast_retry_timeout;
  cache_max_expiry: i32;
// #ifdef HAVE_DNSSEC
  struct ds_config *ds;
  char *timestamp_file;
// #endif

  /* globally used stuff for DNS */
  char *packet; /* packet buffer */
  packet_buff_sz: i32; /* size of above */
  char *namebuff; /* MAXDNAME size buffer */
// #if (defined(HAVE_CONNTRACK) && defined(HAVE_UBUS)) || defined(HAVE_DNSSEC)
  /* CONNTRACK UBUS code uses this buffer, as well as DNSSEC code. */
  char *workspacename;
// #endif
// #ifdef HAVE_DNSSEC
  char *keyname; /* MAXDNAME size buffer */
  unsigned long *rr_status; /* ceiling in TTL from DNSSEC or zero for insecure */
  rr_status_sz: i32;
  dnssec_no_time_check: i32;
  back_to_the_future: i32;
// #endif
  struct frec *frec_list;
  struct frec_src *free_frec_src;
  frec_src_count: i32;
  struct serverfd *sfds;
  struct irec *interfaces;
  struct listener *listeners;
  struct server *srv_save; /* Used for resend on DoD */
  packet_len: usize;       /*      "        "        */
  int    fd_save;          /*      "        "        */
  pid_t tcp_pids[MAX_PROCS];
  int tcp_pipes[MAX_PROCS];
  pipe_to_parent: i32;
  numrrand: i32;
  struct randfd *randomsocks;
  struct randfd_list *rfl_spare, *rfl_poll;
  v6pktinfo: i32;
  interface_addrs: *mut addrlist; /* list of all addresses/prefix lengths associated with all local interfaces */
  int log_id, log_display_id; /* ids of transactions for logging */
  union mysockaddr *log_source_addr;

  /* DHCP state */
  int dhcpfd, helperfd, pxefd; 
// #ifdef HAVE_INOTIFY
  inotifyfd: i32;
// #endif
// #if defined(HAVE_LINUX_NETWORK)
  int netlinkfd, kernel_version;
// #elif defined(HAVE_BSD_NETWORK)
  int dhcp_raw_fd, dhcp_icmp_fd, routefd;
// #endif
  struct iovec dhcp_packet;
  char *dhcp_buff, *dhcp_buff2, *dhcp_buff3;
  struct ping_result *ping_results;
  FILE *lease_stream;
  struct dhcp_bridge *bridges;
  struct shared_network *shared_networks;
// #ifdef HAVE_DHCP6
  duid_len: i32;
  unsigned char *duid;
  struct iovec outpacket;
  int dhcp6fd, icmp6fd;
// #  ifdef HAVE_SCRIPT
  struct snoop_record *free_snoops;
// #  endif
// #endif
  
  /* DBus stuff */
  /* void * here to avoid depending on dbus headers outside dbus.c */
  void *dbus;
// #ifdef HAVE_DBUS
  struct watch *watches;
// #endif

  /* UBus stuff */
// #ifdef HAVE_UBUS
  /* void * here to avoid depending on ubus headers outside ubus.c */
  void *ubus;
// #endif

  /* TFTP stuff */
  struct tftp_transfer *tftp_trans, *tftp_done_trans;

  /* utility string buffer, hold max sized IP address as string */
  char *addrbuff;
  char *addrbuff2; /* only allocated when OPT_EXTRALOG */

// #ifdef HAVE_DUMPFILE
  /* file for packet dumps. */
  dumpfd: i32;
// #endif
}