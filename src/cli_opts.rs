use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct CmdLineArgs {
    // name,has_arg,flag,val,opt,rept,flagdesc,desc,arg
    // "version# ",0,0,'v','v',0,NULL,gettext_noop(# "Display dnsmasq version and copyright information.# "),NULL,
    #[arg(long,help="Display version")]
    version: bool,

    // "no-hosts# ",0,0,'h','h',OPT_NO_HOSTS,NULL,gettext_noop(# "Do NOT load %s file.# "),HOSTSFILE,
    #[arg(long)]
    no_hosts: bool,

    // "no-poll# ",0,0,'n','n',OPT_NO_POLL,NULL,gettext_noop(# "Do NOT poll %s file,reload only on SIGHUP.# "),RESOLVFILE,
    #[arg(long)]
    no_poll: bool,

    // "help# ",0,0,'w','w',0,NULL,gettext_noop(# "Display this message. Use --help dhcp or --help dhcp6 for known DHCP options.# "),NULL,
    #[arg(long)]
    help: bool,

    // "no-daemon# ",0,0,'d','d',OPT_DEBUG,NULL,gettext_noop(# "Do NOT fork into the background: run in debug mode.# "),NULL,
    #[arg(long)]
    no_daemon: bool,

    // "log-queries# ",2,0,'q','q',ARG_DUP,NULL,gettext_noop(# "Log DNS queries.# "),NULL,
    #[arg(long)]
    log_queries: bool,

    // "user# ",2,0,'u','u',ARG_ONE,# "<username># ",gettext_noop(# "Change to this user after startup. (defaults to %s).# "),CHUSER,
    #[arg(long)]
    user: Option<String>,

    // "group# ",2,0,'g','g',ARG_ONE,# "<groupname># ",gettext_noop(# "Change to this group after startup (defaults to %s).# "),CHGRP,
    #[arg(long)]
    group: Option<String>,

    // "resolv-file# ",2,0,'r','r',ARG_DUP,# "<path># ",gettext_noop(# "Specify path to resolv.conf (defaults to %s).# "),RESOLVFILE,
    #[arg(long)]
    resolv_file: Vec<String>,

    // "servers-file# ",1,0,LOPT_SERVERS_FILE,LOPT_SERVERS_FILE,ARG_ONE,# "<path># ",gettext_noop(# "Specify path to file with server= options# "),NULL,
    #[arg(long)]
    servers_file: Option<String>,

    // "mx-host# ",1,0,'m','m',ARG_DUP,# "<host_name>,<target>,<pref># ",gettext_noop(# "Specify an MX record.# "),NULL,
    #[arg(long)]
    mx_host: Vec<String>,

    // "mx-target# ",1,0,'t','t',ARG_ONE,# "<host_name># ",gettext_noop(# "Specify default target in an MX record.# "),NULL,
    #[arg(long)]
    mx_target: Option<String>,

    // "cache-size# ",2,0,'c','c',ARG_ONE,# "<integer># ",gettext_noop(# "Specify the size of the cache in entries (defaults to %s).# "),# "$# ",
    #[arg(long)]
    cache_size: Option<usize>,

    // "port# ",1,0,'p','p',ARG_ONE,# "<integer># ",gettext_noop(# "Specify port to listen for DNS requests on (defaults to 53).# "),NULL,
    #[arg(long)]
    port: Option<u16>,

    // "dhcp-leasefile# ",2,0,'l','l',ARG_ONE,# "<path># ",gettext_noop(# "Specify where to store DHCP leases (defaults to %s).# "),LEASEFILE,
    #[arg(long)]
    dhcp_leasefile: Option<String>,

    // "dhcp-lease# ",1,0,'l',
    // "dhcp-host# ",1,0,'G','G',ARG_DUP,# "<hostspec># ",gettext_noop(# "Set address or hostname for a specified machine.# "),NULL,
    #[arg(long)]
    dhcp_host: Vec<String>,

    // "dhcp-range# ",1,0,'F','F',ARG_DUP,# "<ipaddr>,...# ",gettext_noop(# "Enable DHCP in the range given with lease duration.# "),NULL,
    #[arg(long)]
    dhcp_range: Vec<String>,

    // "dhcp-option# ",1,0,'O','O',ARG_DUP,# "<optspec># ",gettext_noop(# "Specify options to be sent to DHCP clients.# "),NULL,
    #[arg(long)]
    dhcp_option: Vec<String>,

    // "dhcp-boot# ",1,0,'M','M',ARG_DUP,# "<bootp opts># ",gettext_noop(# "Specify BOOTP options to DHCP server.# "),NULL,
    #[arg(long)]
    dhcp_boot: Vec<String>,

    // "domain# ",1,0,'s','s',ARG_DUP,# "<domain>[,<range>]# ",gettext_noop(# "Specify the domain to be assigned in DHCP leases.# "),NULL,
    #[arg(long)]
    domain: Vec<String>,

    // "domain-suffix# ",1,0,'s',
    // #[arg(long)]
    // domain_suffix: Option<String>,

    // "interface# ",1,0,'i','i',ARG_DUP,# "<interface># ",gettext_noop(# "Specify interface(s) to listen on.# "),NULL,
    #[arg(long)]
    interface: Vec<String>,

    // "listen-address# ",1,0,'a','a',ARG_DUP,# "<ipaddr># ",gettext_noop(# "Specify local address(es) to listen on.# "),NULL,
    #[arg(long)]
    listen_address: Vec<String>,

    // "local-service# ",0,0,LOPT_LOCAL_SERVICE,LOPT_LOCAL_SERVICE,OPT_LOCAL_SERVICE,NULL,gettext_noop(# "Accept queries only from directly-connected networks.# "),NULL,
    #[arg(long)]
    local_service: bool,

    // "bogus-priv# ",0,0,'b','b',OPT_BOGUSPRIV,NULL,gettext_noop(# "Fake reverse lookups for RFC1918 private address ranges.# "),NULL,
    #[arg(long)]
    bogus_priv: bool,

    // "bogus-nxdomain# ",1,0,'B','B',ARG_DUP,# "<ipaddr># ",gettext_noop(# "Treat ipaddr as NXDOMAIN (defeats Verisign wildcard).# "),NULL,
    #[arg(long)]
    bogus_nxdomain: bool,

    // "ignore-address# ",1,0,LOPT_IGNORE_ADDR,LOPT_IGNORE_ADDR,ARG_DUP,# "<ipaddr># ",gettext_noop(# "Ignore DNS responses containing ipaddr.# "),NULL,
    #[arg(long)]
    ignore_address: bool,

    // "selfmx# ",0,0,'e','e',OPT_SELFMX,NULL,gettext_noop(# "Return self-pointing MX records for local hosts.# "),NULL,
    #[arg(long)]
    selfmx: bool,

    // "filterwin2k# ",0,0,'f','f',OPT_FILTER,NULL,gettext_noop(# "Don't forward spurious DNS requests from Windows hosts.# "),NULL,
    #[arg(long)]
    filterwin2k: bool,

    // "filter-A# ",0,0,LOPT_FILTER_A,LOPT_FILTER_A,OPT_FILTER_A,NULL,gettext_noop(# "Don't include IPv4 addresses in DNS answers.# "),NULL,
    #[arg(long)]
    filter_a: bool,

    // "filter-AAAA# ",0,0,LOPT_FILTER_AAAA,LOPT_FILTER_AAAA,OPT_FILTER_AAAA,NULL,gettext_noop(# "Don't include IPv6 addresses in DNS answers.# "),NULL,
    #[arg(long)]
    filter_aaaa: bool,

    // "pid-file# ",2,0,'x','x',ARG_ONE,# "<path># ",gettext_noop(# "Specify path of PID file (defaults to %s).# "),RUNFILE,
    #[arg(long)]
    pid_file: Option<String>,

    // "strict-order# ",0,0,'o','o',OPT_ORDER,NULL,gettext_noop(# "Use nameservers strictly in the order given in %s.# "),RESOLVFILE,
    #[arg(long)]
    strict_order: bool,

    // "server# ",1,0,'S','S',ARG_DUP,# "/<domain>/<ipaddr># ",gettext_noop(# "Specify address(es) of upstream servers with optional domains.# "),NULL,
    #[arg(long)]
    server: Vec<String>,

    // "rev-server# ",1,0,LOPT_REV_SERV,LOPT_REV_SERV,ARG_DUP,# "<addr>/<prefix>,<ipaddr># ",gettext_noop(# "Specify address of upstream servers for reverse address queries# "),NULL,
    #[arg(long)]
    rev_server: Vec<String>,

    // "local# ",1,0,LOPT_LOCAL,LOPT_LOCAL,ARG_DUP,# "/<domain>/# ",gettext_noop(# "Never forward queries to specified domains.# "),NULL,
    #[arg(long)]
    local: Vec<String>,

    // "address# ",1,0,'A','A',ARG_DUP,# "/<domain>/<ipaddr># ",gettext_noop(# "Return ipaddr for all hosts in specified domains.# "),NULL,
    #[arg(long)]
    address: Vec<String>,

    // "conf-file# ",2,0,'C','C',ARG_DUP,# "<path># ",gettext_noop(# "Specify configuration file (defaults to %s).# "),CONFFILE,
    #[arg(long)]
    conf_file: Vec<String>,

    // "conf-script# ",1,0,LOPT_CONF_SCRIPT,LOPT_CONF_SCRIPT,ARG_DUP,# "<path># ",gettext_noop(# "Execute file and read configuration from stdin.# "),NULL,
    #[arg(long)]
    conf_script: Vec<String>,

    // "no-resolv# ",0,0,'R','R',OPT_NO_RESOLV,NULL,gettext_noop(# "Do NOT read resolv.conf.# "),NULL,
    #[arg(long)]
    no_resolv: bool,

    // "expand-hosts# ",0,0,'E','E',OPT_EXPAND,NULL,gettext_noop(# "Expand simple names in /etc/hosts with domain-suffix.# "),NULL,
    #[arg(long)]
    expand_hosts: bool,

    // "localmx# ",0,0,'L','L',OPT_LOCALMX,NULL,gettext_noop(# "Return MX records for local hosts.# "),NULL,
    #[arg(long)]
    localmx: bool,

    // "local-ttl# ",1,0,'T','T',ARG_ONE,# "<integer># ",gettext_noop(# "Specify time-to-live in seconds for replies from /etc/hosts.# "),NULL,
    #[arg(long)]
    local_ttl: Option<u32>,

    // "no-negcache# ",0,0,'N','N',OPT_NO_NEG,NULL,gettext_noop(# "Do NOT cache failed search results.# "),NULL,
    #[arg(long)]
    no_negcache: bool,

    // "no-round-robin# ",0,0,LOPT_NORR,LOPT_NORR,OPT_NORR,NULL,gettext_noop(# "Suppress round-robin ordering of DNS records.# "),NULL,
    #[arg(long)]
    no_round_robin: bool,

    // "addn-hosts# ",1,0,'H','H',ARG_DUP,# "<path># ",gettext_noop(# "Specify a hosts file to be read in addition to %s.# "),HOSTSFILE,
    #[arg(long)]
    addn_hosts: Vec<String>,

    // "hostsdir# ",1,0,LOPT_HOST_INOTIFY,LOPT_HOST_INOTIFY,ARG_DUP,# "<path># ",gettext_noop(# "Read hosts files from a directory.# "),NULL,
    #[arg(long)]
    hostsdir: Vec<String>,

    // "query-port# ",1,0,'Q','Q',ARG_ONE,# "<integer># ",gettext_noop(# "Force the originating port for upstream DNS queries.# "),NULL,
    #[arg(long)]
    query_port: Option<u16>,

    // "except-interface# ",1,0,'I','I',ARG_DUP,# "<interface># ",gettext_noop(# "Specify interface(s) NOT to listen on.# "),NULL,
    #[arg(long)]
    except_interface: Vec<String>,

    // "no-dhcp-interface# ",1,0,'2','2',ARG_DUP,# "<interface># ",gettext_noop(# "Do not provide DHCP on this interface,only provide DNS.# "),NULL,
    #[arg(long)]
    no_dhcp_interface: Vec<String>,

    // "domain-needed# ",0,0,'D','D',OPT_NODOTS_LOCAL,NULL,gettext_noop(# "Do NOT forward queries with no domain part.# "),NULL,
    #[arg(long)]
    domain_needed: bool,

    // "dhcp-lease-max# ",1,0,'X','X',ARG_ONE,# "<integer># ",gettext_noop(# "Specify maximum number of DHCP leases (defaults to %s).# "),# "&# ",
    #[arg(long)]
    dhcp_lease_max: Option<u32>,

    // "bind-interfaces# ",0,0,'z','z',OPT_NOWILD,NULL,gettext_noop(# "Bind only to interfaces in use.# "),NULL,
    #[arg(long)]
    bind_interfaces: bool,

    // "read-ethers# ",0,0,'Z','Z',OPT_ETHERS,NULL,gettext_noop(# "Read DHCP static host information from %s.# "),ETHERSFILE,
    #[arg(long)]
    read_ethers: Option<String>,

    // "alias# ",1,0,'V','V',ARG_DUP,# "<ipaddr>,<ipaddr>,<netmask># ",gettext_noop(# "Translate IPv4 addresses from upstream servers.# "),NULL,
    #[arg(long)]
    alias: Vec<String>,

    // "dhcp-vendorclass# ",1,0,'U','U',ARG_DUP,# "set:<tag>,<class># ",gettext_noop(# "Map DHCP vendor class to tag.# "),NULL,
    #[arg(long)]
    dhcp_vendorclass: Vec<String>,

    // "dhcp-userclass# ",1,0,'j','j',ARG_DUP,# "set:<tag>,<class># ",gettext_noop(# "Map DHCP user class to tag.# "),NULL,
    #[arg(long)]
    dhcp_userclass: Vec<String>,

    // "dhcp-ignore# ",1,0,'J','J',ARG_DUP,# "tag:<tag>...# ",gettext_noop(# "Don't do DHCP for hosts with tag set.# "),NULL,
    #[arg(long)]
    dhcp_ignore: Vec<String>,

    // "edns-packet-max# ",1,0,'P','P',ARG_ONE,# "<integer># ",gettext_noop(# "Maximum supported UDP packet size for EDNS.0 (defaults to %s).# "),# "*# ",
    #[arg(long)]
    edns_packet_max: Option<u32>,

    // "keep-in-foreground# ",0,0,'k','k',OPT_NO_FORK,NULL,gettext_noop(# "Do NOT fork into the background,do NOT run in debug mode.# "),NULL,
    #[arg(long)]
    keep_in_foreground: bool,

    // "dhcp-authoritative# ",0,0,'K','K',OPT_AUTHORITATIVE,NULL,gettext_noop(# "Assume we are the only DHCP server on the local network.# "),NULL,
    #[arg(long)]
    dhcp_authoritative: bool,

    // "srv-host# ",1,0,'W','W',ARG_DUP,# "<name>,<target>,...# ",gettext_noop(# "Specify a SRV record.# "),NULL,
    #[arg(long)]
    srv_host: Vec<String>,

    // "localise-queries# ",0,0,'y','y',OPT_LOCALISE,NULL,gettext_noop(# "Answer DNS queries based on the interface a query was sent to.# "),NULL,
    #[arg(long)]
    localise_queries: bool,

    // "txt-record# ",1,0,'Y','Y',ARG_DUP,# "<name>,<txt>[,<txt]# ",gettext_noop(# "Specify TXT DNS record.# "),NULL,
    #[arg(long)]
    txt_record: Vec<String>,

    // "caa-record# ",1,0,LOPT_CAA,LOPT_CAA,ARG_DUP,# "<name>,<flags>,<tag>,<value># ",gettext_noop(# "Specify certification authority authorization record# "),NULL,
    #[arg(long)]
    caa_record: Vec<String>,

    // "dns-rr# ",1,0,LOPT_RR,LOPT_RR,ARG_DUP,# "<name>,<RR-number>,[<data>]# ",gettext_noop(# "Specify arbitrary DNS resource record# "),NULL,
    #[arg(long)]
    dns_rr: Vec<String>,

    // "enable-dbus# ",2,0,'1','1',ARG_ONE,# "[=<busname>]# ",gettext_noop(# "Enable the DBus interface for setting upstream servers,etc.# "),NULL,
    #[arg(long)]
    enable_dbus: bool,

    // "enable-ubus# ",2,0,LOPT_UBUS,LOPT_UBUS,ARG_ONE,# "[=<busname>]# ",gettext_noop(# "Enable the UBus interface.# "),NULL,
    #[arg(long)]
    enable_ubus: bool,

    // "bootp-dynamic# ",2,0,'3','3',ARG_DUP,# "[=tag:<tag>]...# ",gettext_noop(# "Enable dynamic address allocation for bootp.# "),NULL,
    #[arg(long)]
    bootp_dynamic: Vec<String>,

    // "dhcp-mac# ",1,0,'4','4',ARG_DUP,# "set:<tag>,<mac address># ",gettext_noop(# "Map MAC address (with wildcards) to option set.# "),NULL,
    #[arg(long)]
    dhcp_mac: Vec<String>,

    // "no-ping# ",0,0,'5','5',OPT_NO_PING,NULL,gettext_noop(# "Disable ICMP echo address checking in the DHCP server.# "),NULL,
    #[arg(long)]
    no_ping: bool,

    // "dhcp-script# ",1,0,'6','6',ARG_ONE,# "<path># ",gettext_noop(# "Shell script to run on DHCP lease creation and destruction.# "),NULL,
    #[arg(long)]
    dhcp_script:  Option<String>,

    // "conf-dir# ",1,0,'7','7',ARG_DUP,# "<path># ",gettext_noop(# "Read configuration from all the files in this directory.# "),NULL,
    #[arg(long)]
    conf_dir: Vec<String>,

    // "log-facility# ",1,0,'8','8',ARG_ONE,# "<facility>|<file># ",gettext_noop(# "Log to this syslog facility or file. (defaults to DAEMON)# "),NULL,
    #[arg(long)]
    log_facility: Option<String>,

    // "leasefile-ro# ",0,0,'9','9',OPT_LEASE_RO,NULL,gettext_noop(# "Do not use leasefile.# "),NULL,
    #[arg(long)]
    leasefile_ro: bool,

    // "script-on-renewal# ",0,0,LOPT_SCRIPT_TIME,LOPT_SCRIPT_TIME,OPT_LEASE_RENEW,NULL,gettext_noop(# "Call dhcp-script when lease expiry changes.# "),NULL,
    #[arg(long)]
    script_on_renewal: Option<String>,

    // "dns-forward-max# ",1,0,'0','0',ARG_ONE,# "<integer># ",gettext_noop(# "Maximum number of concurrent DNS queries. (defaults to %s)# "),# "!# ",
    #[arg(long)]
    dns_forward_max: Option<u32>,

    // "clear-on-reload# ",0,0,LOPT_RELOAD,LOPT_RELOAD,OPT_RELOAD,NULL,gettext_noop(# "Clear DNS cache when reloading %s.# "),RESOLVFILE,
    #[arg(long)]
    clear_on_reload: bool,

    // "dhcp-ignore-names# ",2,0,LOPT_NO_NAMES,LOPT_NO_NAMES,ARG_DUP,# "[=tag:<tag>]...# ",gettext_noop(# "Ignore hostnames provided by DHCP clients.# "),NULL,
    #[arg(long)]
    dhcp_ignore_names: Vec<String>,

    // "enable-tftp# ",2,0,LOPT_TFTP,LOPT_TFTP,ARG_DUP,# "[=<intr>[,<intr>]]# ",gettext_noop(# "Enable integrated read-only TFTP server.# "),NULL,
    #[arg(long)]
    enable_tftp: Vec<String>,

    // "tftp-secure# ",0,0,LOPT_SECURE,LOPT_SECURE,OPT_TFTP_SECURE,NULL,gettext_noop(# "Allow access only to files owned by the user running dnsmasq.# "),NULL,
    #[arg(long)]
    tftp_secure: bool,

    // "tftp-no-fail# ",0,0,LOPT_TFTP_NO_FAIL,LOPT_TFTP_NO_FAIL,OPT_TFTP_NO_FAIL,NULL,gettext_noop(# "Do not terminate the service if TFTP directories are inaccessible.# "),NULL,
    #[arg(long)]
    tftp_no_fail: bool,

    // "tftp-unique-root# ",2,0,LOPT_APREF,LOPT_APREF,ARG_DUP,# "[=ip|mac]# ",gettext_noop(# "Add client IP or hardware address to tftp-root.# "),NULL,
    #[arg(long)]
    tftp_unique_root: Option<String>,

    // "tftp-root# ",1,0,LOPT_PREFIX,LOPT_PREFIX,ARG_DUP,# "<dir>[,<iface>]# ",gettext_noop(# "Export files by TFTP only from the specified subtree.# "),NULL,
    #[arg(long)]
    tftp_root: Option<String>,

    // "tftp-max# ",1,0,LOPT_TFTP_MAX,LOPT_TFTP_MAX,ARG_ONE,# "<integer># ",gettext_noop(# "Maximum number of concurrent TFTP transfers (defaults to %s).# "),# "## ",
    #[arg(long)]
    tftp_max: Option<u32>,

    // "tftp-mtu# ",1,0,LOPT_TFTP_MTU,LOPT_TFTP_MTU,ARG_ONE,# "<integer># ",gettext_noop(# "Maximum MTU to use for TFTP transfers.# "),NULL,
    #[arg(long)]
    tftp_mut: Option<u16>,

    // "tftp-lowercase# ",0,0,LOPT_TFTP_LC,LOPT_TFTP_LC,OPT_TFTP_LC,NULL,gettext_noop(# "Convert TFTP filenames to lowercase# "),NULL,
    #[arg(long)]
    tftp_lowercase: bool,

    // "tftp-single-port# ",0,0,LOPT_SINGLE_PORT,LOPT_SINGLE_PORT,OPT_SINGLE_PORT,NULL,gettext_noop(# "Use only one port for TFTP server.# "),NULL,
    #[arg(long)]
    tftp_single_port: Option<u16>,

    // "ptr-record# ",1,0,LOPT_PTR,LOPT_PTR,ARG_DUP,# "<name>,<target># ",gettext_noop(# "Specify PTR DNS record.# "),NULL,
    #[arg(long)]
    ptr_record: Vec<String>,

    // "naptr-record# ",1,0,LOPT_NAPTR,LOPT_NAPTR,ARG_DUP,# "<name>,<naptr># ",gettext_noop(# "Specify NAPTR DNS record.# "),NULL,
    #[arg(long)]
    naptr_record: Vec<String>,

    // "bridge-interface# ",1,0,LOPT_BRIDGE,LOPT_BRIDGE,ARG_DUP,# "<iface>,<alias>..# ",gettext_noop(# "Treat DHCP requests on aliases as arriving from interface.# "),NULL,
    #[arg(long)]
    bridge_interface: Vec<String>,

    // "shared-network# ",1,0,LOPT_SHARED_NET,LOPT_SHARED_NET,ARG_DUP,# "<iface>|<addr>,<addr># ",gettext_noop(# "Specify extra networks sharing a broadcast domain for DHCP# "),NULL,
    #[arg(long)]
    shared_network: Vec<String>,

    // "dhcp-option-force# ",1,0,LOPT_FORCE,LOPT_FORCE,ARG_DUP,# "<optspec># ",gettext_noop(# "DHCP option sent even if the client does not request it.# "),NULL,
    #[arg(long)]
    dhcp_option_force: Vec<String>,

    // "tftp-no-blocksize# ",0,0,LOPT_NOBLOCK,LOPT_NOBLOCK,OPT_TFTP_NOBLOCK,NULL,gettext_noop(# "Disable the TFTP blocksize extension.# "),NULL,
    #[arg(long)]
    tftp_no_blocksize: bool,

    // "log-dhcp# ",0,0,LOPT_LOG_OPTS,LOPT_LOG_OPTS,OPT_LOG_OPTS,NULL,gettext_noop(# "Extra logging for DHCP.# "),NULL,
    #[arg(long)]
    log_dhcp: bool,

    // "log-async# ",2,0,LOPT_MAX_LOGS,LOPT_MAX_LOGS,ARG_ONE,# "[=<integer>]# ",gettext_noop(# "Enable async. logging; optionally set queue length.# "),NULL,
    #[arg(long)]
    log_async: Option<u32>,

    // "dhcp-circuitid# ",1,0,LOPT_CIRCUIT,LOPT_CIRCUIT,ARG_DUP,# "set:<tag>,<circuit># ",gettext_noop(# "Map RFC3046 circuit-id to tag.# "),NULL,
    #[arg(long)]
    dhcp_circuitid: Vec<String>,

    // "dhcp-remoteid# ",1,0,LOPT_REMOTE,LOPT_REMOTE,ARG_DUP,# "set:<tag>,<remote># ",gettext_noop(# "Map RFC3046 remote-id to tag.# "),NULL,
    #[arg(long)]
    dhcp_remoteid: Vec<String>,

    // "dhcp-subscrid# ",1,0,LOPT_SUBSCR,LOPT_SUBSCR,ARG_DUP,# "set:<tag>,<remote># ",gettext_noop(# "Map RFC3993 subscriber-id to tag.# "),NULL,
    #[arg(long)]
    dhcp_subscrid: Vec<String>,

    // "dhcp-pxe-vendor# ",1,0,LOPT_PXE_VENDOR,LOPT_PXE_VENDOR,ARG_DUP,# "<vendor>[,...]# ",gettext_noop(# "Specify vendor class to match for PXE requests.# "),NULL,
    #[arg(long)]
    dhcp_pxe_vendor: Vec<String>,

    // "interface-name# ",1,0,LOPT_INTNAME,LOPT_INTNAME,ARG_DUP,# "<name>,<interface># ",gettext_noop(# "Give DNS name to IPv4 address of interface.# "),NULL,
    #[arg(long)]
    interface_name: Vec<String>,

    // "dhcp-hostsfile# ",1,0,LOPT_DHCP_HOST,LOPT_DHCP_HOST,ARG_DUP,# "<path># ",gettext_noop(# "Read DHCP host specs from file.# "),NULL,
    #[arg(long)]
    dhcp_hostsfile: Option<String>,

    // "dhcp-optsfile# ",1,0,LOPT_DHCP_OPTS,LOPT_DHCP_OPTS,ARG_DUP,# "<path># ",gettext_noop(# "Read DHCP option specs from file.# "),NULL,
    #[arg(long)]
    dhcp_optsfile: Option<String>,

    // "dhcp-hostsdir# ",1,0,LOPT_DHCP_INOTIFY,LOPT_DHCP_INOTIFY,ARG_DUP,# "<path># ",gettext_noop(# "Read DHCP host specs from a directory.# "),NULL,
    #[arg(long)]
    dhcp_hostsdir: Option<String>,

    // "dhcp-optsdir# ",1,0,LOPT_DHOPT_INOTIFY,LOPT_DHOPT_INOTIFY,ARG_DUP,# "<path># ",gettext_noop(# "Read DHCP options from a directory.# "),NULL,
    #[arg(long)]
    dhcp_optsdir: Option<String>,

    // "dhcp-no-override# ",0,0,LOPT_OVERRIDE,LOPT_OVERRIDE,OPT_NO_OVERRIDE,NULL,gettext_noop(# "Do NOT reuse filename and server fields for extra DHCP options.# "),NULL,
    #[arg(long)]
    dhcp_no_override: bool,

    // "tftp-port-range# ",1,0,LOPT_TFTPPORTS,LOPT_TFTPPORTS,ARG_ONE,# "<start>,<end># ",gettext_noop(# "Ephemeral port range for use by TFTP transfers.# "),NULL,
    #[arg(long)]
    tftp_port_range: Option<String>,

    // "stop-dns-rebind# ",0,0,LOPT_REBIND,LOPT_REBIND,OPT_NO_REBIND,NULL,gettext_noop(# "Stop DNS rebinding. Filter private IP ranges when resolving.# "),NULL,
    #[arg(long)]
    stop_dns_rebind: bool,

    // "rebind-domain-ok# ",1,0,LOPT_NO_REBIND,LOPT_NO_REBIND,ARG_DUP,# "/<domain>/# ",gettext_noop(# "Inhibit DNS-rebind protection on this domain.# "),NULL,
    #[arg(long)]
    rebind_domain_ok: bool,

    // "all-servers# ",0,0,LOPT_NOLAST,LOPT_NOLAST,OPT_ALL_SERVERS,NULL,gettext_noop(# "Always perform DNS queries to all servers.# "),NULL,
    #[arg(long)]
    all_servers: bool,

    // "dhcp-match# ",1,0,LOPT_MATCH,LOPT_MATCH,ARG_DUP,# "set:<tag>,<optspec># ",gettext_noop(# "Set tag if client includes matching option in request.# "),NULL,
    #[arg(long)]
    dhcp_match: Vec<String>,

    // "dhcp-name-match# ",1,0,LOPT_NAME_MATCH,LOPT_NAME_MATCH,ARG_DUP,# "set:<tag>,<string>[*]# ",gettext_noop(# "Set tag if client provides given name.# "),NULL,
    #[arg(long)]
    dhcp_name_match: Vec<String>,

    // "dhcp-broadcast# ",2,0,LOPT_BROADCAST,LOPT_BROADCAST,ARG_DUP,# "[=tag:<tag>...]# ",gettext_noop(# "Force broadcast replies for hosts with tag set.# "),NULL,
    #[arg(long)]
    dhcp_broadcast: Vec<String>,

    // "neg-ttl# ",1,0,LOPT_NEGTTL,LOPT_NEGTTL,ARG_ONE,# "<integer># ",gettext_noop(# "Specify time-to-live in seconds for negative caching.# "),NULL,
    #[arg(long)]
    neg_ttl: Option<u32>,

    // "max-ttl# ",1,0,LOPT_MAXTTL,LOPT_MAXTTL,ARG_ONE,# "<integer># ",gettext_noop(# "Specify time-to-live in seconds for maximum TTL to send to clients.# "),NULL,
    #[arg(long)]
    max_ttl: Option<u32>,

    // "min-cache-ttl# ",1,0,LOPT_MINCTTL,LOPT_MINCTTL,ARG_ONE,# "<integer># ",gettext_noop(# "Specify time-to-live floor for cache.# "),NULL,
    #[arg(long)]
    min_cache_ttl: Option<u32>,

    // "max-cache-ttl# ",1,0,LOPT_MAXCTTL,LOPT_MAXCTTL,ARG_ONE,# "<integer># ",gettext_noop(# "Specify time-to-live ceiling for cache.# "),NULL,
    #[arg(long)]
    max_cache_ttl: Option<u32>,

    // "dhcp-alternate-port# ",2,0,LOPT_ALTPORT,LOPT_ALTPORT,ARG_ONE,# "[=<ports>]# ",gettext_noop(# "Use alternative ports for DHCP.# "),NULL,
    #[arg(long)]
    dhcp_alternate_port: Option<u16>,

    // "dhcp-scriptuser# ",1,0,LOPT_SCRIPTUSR,LOPT_SCRIPTUSR,ARG_ONE,# "<username># ",gettext_noop(# "Run lease-change scripts as this user.# "),NULL,
    #[arg(long)]
    dhcp_scriptuser: Option<String>,

    // "min-port# ",1,0,LOPT_MINPORT,LOPT_MINPORT,ARG_ONE,# "<port># ",gettext_noop(# "Specify lowest port available for DNS query transmission.# "),NULL,
    #[arg(long)]
    min_port: Option<u16>,

    // "max-port# ",1,0,LOPT_MAXPORT,LOPT_MAXPORT,ARG_ONE,# "<port># ",gettext_noop(# "Specify highest port available for DNS query transmission.# "),NULL,
    #[arg(long)]
    max_port: Option<u16>,

    // "dhcp-fqdn# ",0,0,LOPT_DHCP_FQDN,LOPT_DHCP_FQDN,OPT_DHCP_FQDN,NULL,gettext_noop(# "Use only fully qualified domain names for DHCP clients.# "),NULL,
    #[arg(long)]
    dhcp_fqdn: bool,

    // "cname# ",1,0,LOPT_CNAME,LOPT_CNAME,ARG_DUP,# "<alias>,<target>[,<ttl>]# ",gettext_noop(# "Specify alias name for LOCAL DNS name.# "),NULL,
    #[arg(long)]
    cname: Vec<String>,

    // "pxe-prompt# ",1,0,LOPT_PXE_PROMT,LOPT_PXE_PROMT,ARG_DUP,# "<prompt>,[<timeout>]# ",gettext_noop(# "Prompt to send to PXE clients.# "),NULL,
    #[arg(long)]
    pxe_prompt: Option<String>,

    // "pxe-service# ",1,0,LOPT_PXE_SERV,LOPT_PXE_SERV,ARG_DUP,# "<service># ",gettext_noop(# "Boot service for PXE menu.# "),NULL,
    #[arg(long)]
    pxe_service: Option<String>,

    // "test# ",0,0,LOPT_TEST,LOPT_TEST,0,NULL,gettext_noop(# "Check configuration syntax.# "),NULL,
    #[arg(long)]
    test: bool,

    // "tag-if# ",1,0,LOPT_TAG_IF,LOPT_TAG_IF,ARG_DUP,# "tag-expression# ",gettext_noop(# "Evaluate conditional tag expression.# "),NULL,
    #[arg(long)]
    tag_if: Vec<String>,

    // "dhcp-proxy# ",2,0,LOPT_PROXY,LOPT_PROXY,ARG_DUP,# "[=<ipaddr>]...# ",gettext_noop(# "Use these DHCP relays as full proxies.# "),NULL,
    #[arg(long)]
    dhcp_proxy: Vec<String>,

    // "dhcp-generate-names# ",2,0,LOPT_GEN_NAMES,LOPT_GEN_NAMES,ARG_DUP,# "[=tag:<tag>]# ",gettext_noop(# "Generate hostnames based on MAC address for nameless clients.# "),NULL,
    #[arg(long)]
    dhcp_generate_name: Vec<String>,

    // "rebind-localhost-ok# ",0,0,LOPT_LOC_REBND,LOPT_LOC_REBND,OPT_LOCAL_REBIND,NULL,gettext_noop(# "Allow rebinding of 127.0.0.0/8,for RBL servers.# "),NULL,
    #[arg(long)]
    rebind_localhost_ok: bool,

    // "add-mac# ",2,0,LOPT_ADD_MAC,LOPT_ADD_MAC,ARG_DUP,# "[=base64|text]# ",gettext_noop(# "Add requestor's MAC address to forwarded DNS queries.# "),NULL,
    #[arg(long)]
    add_mac: Vec<String>,

    // "strip-mac# ",0,0,LOPT_STRIP_MAC,LOPT_STRIP_MAC,OPT_STRIP_MAC,NULL,gettext_noop(# "Strip MAC information from queries.# "),NULL,
    #[arg(long)]
    strip_mac: bool,

    // "add-subnet# ",2,0,LOPT_ADD_SBNET,LOPT_ADD_SBNET,ARG_ONE,# "<v4 pref>[,<v6 pref>]# ",gettext_noop(# "Add specified IP subnet to forwarded DNS queries.# "),NULL,
    #[arg(long)]
    add_subnet: Vec<String>,

    // "strip-subnet# ",0,0,LOPT_STRIP_SBNET,LOPT_STRIP_SBNET,OPT_STRIP_ECS,NULL,gettext_noop(# "Strip ECS information from queries.# "),NULL,
    #[arg(long)]
    strip_subnet: bool,

    // "add-cpe-id# ",1,0,LOPT_CPE_ID,LOPT_CPE_ID,ARG_ONE,# "<text># ",gettext_noop(# "Add client identification to forwarded DNS queries.# "),NULL,
    #[arg(long)]
    add_cpe_id: bool,

    // "proxy-dnssec# ",0,0,LOPT_DNSSEC,LOPT_DNSSEC,OPT_DNSSEC_PROXY,NULL,gettext_noop(# "Proxy DNSSEC validation results from upstream nameservers.# "),NULL,
    #[arg(long)]
    proxy_dnssec: bool,

    // "dhcp-sequential-ip# ",0,0,LOPT_INCR_ADDR,LOPT_INCR_ADDR,OPT_CONSEC_ADDR,NULL,gettext_noop(# "Attempt to allocate sequential IP addresses to DHCP clients.# "),NULL,
    #[arg(long)]
    dhcp_sequential_ip: bool,

    // "conntrack# ",0,0,LOPT_CONNTRACK,LOPT_CONNTRACK,OPT_CONNTRACK,NULL,gettext_noop(# "Copy connection-track mark from queries to upstream connections.# "),NULL,
    #[arg(long)]
    conntrack: bool,

    // "dhcp-client-update# ",0,0,LOPT_FQDN,LOPT_FQDN,OPT_FQDN_UPDATE,NULL,gettext_noop(# "Allow DHCP clients to do their own DDNS updates.# "),NULL,
    #[arg(long)]
    dhcp_client_update: bool,

    // "dhcp-luascript# ",1,0,LOPT_LUASCRIPT,LOPT_LUASCRIPT,ARG_DUP,# "path# ",gettext_noop(# "Lua script to run on DHCP lease creation and destruction.# "),NULL,
    #[arg(long)]
    dhcp_luascript: Option<String>,

    // "enable-ra# ",0,0,LOPT_RA,LOPT_RA,OPT_RA,NULL,gettext_noop(# "Send router-advertisements for interfaces doing DHCPv6# "),NULL,
    #[arg(long)]
    enable_ra: bool,

    // "dhcp-duid# ",1,0,LOPT_DUID,LOPT_DUID,ARG_ONE,# "<enterprise>,<duid># ",gettext_noop(# "Specify DUID_EN-type DHCPv6 server DUID# "),NULL,
    #[arg(long)]
    dhcp_duid: Vec<String>,

    // "host-record# ",1,0,LOPT_HOST_REC,LOPT_HOST_REC,ARG_DUP,# "<name>,<address>[,<ttl>]# ",gettext_noop(# "Specify host (A/AAAA and PTR) records# "),NULL,
    #[arg(long)]
    host_record: Vec<String>,

    // "bind-dynamic# ",0,0,LOPT_CLVERBIND,LOPT_CLVERBIND,OPT_CLEVERBIND,NULL,gettext_noop(# "Bind to interfaces in use - check for new interfaces# "),NULL,
    #[arg(long)]
    bind_dynamics: bool,

    // "auth-zone# ",1,0,LOPT_AUTHZONE,LOPT_AUTHZONE,ARG_DUP,# "<domain>,[<subnet>...]# ",gettext_noop(# "Domain to export to global DNS# "),NULL,
    #[arg(long)]
    auth_zone: Vec<String>,

    // "auth-server# ",1,0,LOPT_AUTHSERV,LOPT_AUTHSERV,ARG_ONE,# "<NS>,<interface># ",gettext_noop(# "Export local names to global DNS# "),NULL,
    #[arg(long)]
    auth_server: Vec<String>,

    // "auth-ttl# ",1,0,LOPT_AUTHTTL,LOPT_AUTHTTL,ARG_ONE,# "<integer># ",gettext_noop(# "Set TTL for authoritative replies# "),NULL,
    #[arg(long)]
    auth_ttl: Option<u8>,

    // "auth-soa# ",1,0,LOPT_AUTHSOA,LOPT_AUTHSOA,ARG_ONE,# "<serial>[,...]# ",gettext_noop(# "Set authoritative zone information# "),NULL,
    #[arg(long)]
    auth_soa: Vec<String>,

    // "auth-sec-servers# ",1,0,LOPT_AUTHSFS,LOPT_AUTHSFS,ARG_DUP,# "<NS>[,<NS>...]# ",gettext_noop(# "Secondary authoritative nameservers for forward domains# "),NULL,
    #[arg(long)]
    auth_sec_servers: Vec<String>,

    // "auth-peer# ",1,0,LOPT_AUTHPEER,LOPT_AUTHPEER,ARG_DUP,# "<ipaddr>[,<ipaddr>...]# ",gettext_noop(# "Peers which are allowed to do zone transfer# "),NULL,
    #[arg(long)]
    auth_peer: Vec<String>,

    // "ipset# ",1,0,LOPT_IPSET,LOPT_IPSET,ARG_DUP,# "/<domain>[/<domain>...]/<ipset>...# ",gettext_noop(# "Specify ipsets to which matching domains should be added# "),NULL,
    #[arg(long)]
    ipset: Vec<String>,

    // "nftset# ",1,0,LOPT_NFTSET,LOPT_NFTSET,ARG_DUP,# "/<domain>[/<domain>...]/<nftset>...# ",gettext_noop(# "Specify nftables sets to which matching domains should be added# "),NULL,
    #[arg(long)]
    nftset: Vec<String>,

    // "connmark-allowlist-enable# ",2,0,LOPT_CMARK_ALST_EN,LOPT_CMARK_ALST_EN,ARG_ONE,# "[=<mask>]# ",gettext_noop(# "Enable filtering of DNS queries with connection-track marks.# "),NULL,
    #[arg(long)]
    connmark_allowlist_enable: Option<String>,

    // "connmark-allowlist# ",1,0,LOPT_CMARK_ALST,LOPT_CMARK_ALST,ARG_DUP,# "<connmark>[/<mask>][,<pattern>[/<pattern>...]]# ",gettext_noop(# "Set allowed DNS patterns for a connection-track mark.# "),NULL,
    #[arg(long)]
    connmark_allowlist: Vec<String>,

    // "synth-domain# ",1,0,LOPT_SYNTH,LOPT_SYNTH,ARG_DUP,# "<domain>,<range>,[<prefix>]# ",gettext_noop(# "Specify a domain and address range for synthesised names# "),NULL,
    #[arg(long)]
    synth_domaion: Vec<String>,

    // "dnssec# ",0,0,LOPT_SEC_VALID,LOPT_SEC_VALID,OPT_DNSSEC_VALID,NULL,gettext_noop(# "Activate DNSSEC validation# "),NULL,
    #[arg(long)]
    dnssec: bool,

    // "trust-anchor# ",1,0,LOPT_TRUST_ANCHOR,LOPT_TRUST_ANCHOR,ARG_DUP,# "<domain>,[<class>],...# ",gettext_noop(# "Specify trust anchor key digest.# "),NULL,
    #[arg(long)]
    trust_anchor: Vec<String>,

    // "dnssec-debug# ",0,0,LOPT_DNSSEC_DEBUG,LOPT_DNSSEC_DEBUG,OPT_DNSSEC_DEBUG,NULL,gettext_noop(# "Disable upstream checking for DNSSEC debugging.# "),NULL,
    #[arg(long)]
    dnssec_debug: bool,

    // "dnssec-check-unsigned# ",2,0,LOPT_DNSSEC_CHECK,LOPT_DNSSEC_CHECK,ARG_DUP,NULL,gettext_noop(# "Ensure answers without DNSSEC are in unsigned zones.# "),NULL,
    #[arg(long)]
    dnssec_check_unsigned: bool,

    // "dnssec-no-timecheck# ",0,0,LOPT_DNSSEC_TIME,LOPT_DNSSEC_TIME,OPT_DNSSEC_TIME,NULL,gettext_noop(# "Don't check DNSSEC signature timestamps until first cache-reload# "),NULL,
    #[arg(long)]
    dnssec_no_timecheck: bool,

    // "dnssec-timestamp# ",1,0,LOPT_DNSSEC_STAMP,LOPT_DNSSEC_STAMP,ARG_ONE,# "<path># ",gettext_noop(# "Timestamp file to verify system clock for DNSSEC# "),NULL,
    #[arg(long)]
    dnssec_timestamp: Option<String>,

    // "dhcp-relay# ",1,0,LOPT_RELAY,LOPT_RELAY,ARG_DUP,# "<local-addr>,<server>[,<iface>]# ",gettext_noop(# "Relay DHCP requests to a remote server# "),NULL,
    #[arg(long)]
    dhcp_relay: Vec<String>,

    // "ra-param# ",1,0,LOPT_RA_PARAM,LOPT_RA_PARAM,ARG_DUP,# "<iface>,[mtu:<value>|<interface>|off,][<prio>,]<intval>[,<lifetime>]# ",gettext_noop(# "Set MTU,priority,resend-interval and router-lifetime# "),NULL,
    #[arg(long)]
    ra_parama: Vec<String>,

    // "quiet-dhcp# ",0,0,LOPT_QUIET_DHCP,LOPT_QUIET_DHCP,OPT_QUIET_DHCP,NULL,gettext_noop(# "Do not log routine DHCP.# "),NULL,
    #[arg(long)]
    quiet_dhcp: bool,

    // "quiet-dhcp6# ",0,0,LOPT_QUIET_DHCP6,LOPT_QUIET_DHCP6,OPT_QUIET_DHCP6,NULL,gettext_noop(# "Do not log routine DHCPv6.# "),NULL,
    #[arg(long)]
    quiet_dhcp6: bool,

    // "quiet-ra# ",0,0,LOPT_QUIET_RA,LOPT_QUIET_RA,OPT_QUIET_RA,NULL,gettext_noop(# "Do not log RA.# "),NULL,
    #[arg(long)]
    quiet_ra: bool,

    // "dns-loop-detect# ",0,0,LOPT_LOOP_DETECT,LOPT_LOOP_DETECT,OPT_LOOP_DETECT,NULL,gettext_noop(# "Detect and remove DNS forwarding loops.# "),NULL,
    #[arg(long)]
    dns_loop_detect: bool,

    // "script-arp# ",0,0,LOPT_SCRIPT_ARP,LOPT_SCRIPT_ARP,OPT_SCRIPT_ARP,NULL,gettext_noop(# "Call dhcp-script with changes to local ARP table.# "),NULL,
    #[arg(long)]
    script_arp: bool,

    // "dhcp-ttl# ",1,0,LOPT_DHCPTTL,LOPT_DHCPTTL,ARG_ONE,# "<ttl># ",gettext_noop(# "Set TTL in DNS responses with DHCP-derived addresses.# "),NULL,
    #[arg(long)]
    dhcp_ttl: Option<u16>,

    // "dhcp-reply-delay# ",1,0,LOPT_REPLY_DELAY,LOPT_REPLY_DELAY,ARG_ONE,# "<integer># ",gettext_noop(# "Delay DHCP replies for at least number of seconds.# "),NULL,
    #[arg(long)]
    dhcp_relay_delay: Option<u32>,

    // "dhcp-rapid-commit# ",0,0,LOPT_RAPID_COMMIT,LOPT_RAPID_COMMIT,OPT_RAPID_COMMIT,NULL,gettext_noop(# "Enables DHCPv4 Rapid Commit option.# "),NULL,
    #[arg(long)]
    dhcp_rapid_commit: bool,

    // "dumpfile# ",1,0,LOPT_DUMPFILE,LOPT_DUMPFILE,ARG_ONE,# "<path># ",gettext_noop(# "Path to debug packet dump file# "),NULL,
    #[arg(long)]
    dumpfile: Option<String>,

    // "dumpmask# ",1,0,LOPT_DUMPMASK,LOPT_DUMPMASK,ARG_ONE,# "<hex># ",gettext_noop(# "Mask which packets to dump# "),NULL,
    #[arg(long)]
    dumpmask: Option<String>,

    // "dhcp-ignore-clid# ",0,0,LOPT_IGNORE_CLID,LOPT_IGNORE_CLID,OPT_IGNORE_CLID,NULL,gettext_noop(# "Ignore client identifier option sent by DHCP clients.# "),NULL,
    #[arg(long)]
    dhcp_ignore_clid: bool,

    // "dynamic-host# ",1,0,LOPT_DYNHOST,LOPT_DYNHOST,ARG_DUP,# "<name>,[<IPv4>][,<IPv6>],<interface-name># ",gettext_noop(# "Specify host record in interface subnet# "),NULL,
    #[arg(long)]
    dynamic_host: Vec<String>,

    // "log-debug# ",0,0,LOPT_LOG_DEBUG,LOPT_LOG_DEBUG,OPT_LOG_DEBUG,NULL,gettext_noop(# "Log debugging information.# "),NULL,
    #[arg(long)]
    log_debug: bool,

    // "umbrella# ",2,0,LOPT_UMBRELLA,LOPT_UMBRELLA,ARG_ONE,# "[=<optspec>]# ",gettext_noop(# "Send Cisco Umbrella identifiers including remote IP.# "),NULL,
    #[arg(long)]
    umbrella: Vec<String>,

    // "quiet-tftp# ",0,0,LOPT_QUIET_TFTP,LOPT_QUIET_TFTP,OPT_QUIET_TFTP,NULL,gettext_noop(# "Do not log routine TFTP.# "),NULL,
    #[arg(long)]
    quiet_tftp: bool,

    // "port-limit# ",1,0,LOPT_RANDPORT_LIM,LOPT_RANDPORT_LIM,ARG_ONE,# "#ports# ",gettext_noop(# "Set maximum number of random originating ports for a query.# "),NULL,
    #[arg(long)]
    port_limit: Option<u32>,

    // "fast-dns-retry# ",2,0,LOPT_FAST_RETRY,LOPT_FAST_RETRY,ARG_ONE,# "<milliseconds># ",gettext_noop(# "Retry DNS queries after this many milliseconds.# "),NULL,
    #[arg(long)]
    fast_dns_retry: Option<u64>,

    // "use-stale-cache# ",2,0,LOPT_STALE_CACHE,LOPT_STALE_CACHE,ARG_ONE,# "[=<max_expired>]# ",gettext_noop(# "Use expired cache data for faster reply.# "),NULL,
    #[arg(long)]
    use_stale_cache: Option<String>,
}

pub fn parse_cmd_line() {
    let mut parsed_cmd_line = CmdLineArgs::parse();
}

// TODO: parse command line elements
