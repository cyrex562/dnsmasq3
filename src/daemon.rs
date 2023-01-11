use std::ffi::c_void;
use std::net::IpAddr;
use libc::{c_char, FILE, time_t};
use crate::addrlist::AddrList;
use crate::allow_list::AllowList;
use crate::auth_zone::AuthZone;
use crate::bogus_addr::BogusAddr;
use crate::cname::Cname;
use crate::cond_domain::CondDomain;
use crate::config::MAX_PROCS;
use crate::delay_config::DelayConfig;
use crate::dhcp_boot::DhcpBoot;
use crate::dhcp_bridge::DhcpBridge;
use crate::dhcp_config::DhcpConfig;
use crate::dhcp_context::DhcpContext;
use crate::dhcp_mac::DhcpMac;
use crate::dhcp_match_name::DhcpMatchName;
use crate::dhcp_netid_list::DhcpNetidList;
use crate::dhcp_opt::DhcpOpt;
use crate::dhcp_pxe_vendor::DhcpPxeVendor;
use crate::dhcp_relay::DhcpRelay;
use crate::doctor::Doctor;
use crate::ds_config::DsConfig;
use crate::dyndir::DynDir;
use crate::frec::Frec;
use crate::frec_src::FrecSrc;
use crate::host_record::HostRecord;
use crate::hosts_file::HostsFile;
use crate::iname::Iname;
use crate::interface_name::InterfaceName;
use crate::ipsets::IpSets;
use crate::irec::Irec;
use crate::listener::Listener;
use crate::mx_srv_record::MxSrvRecord;
use crate::mysockaddr::MySockAddr;
use crate::mysubnet::MySubnet;
use crate::name_list::NameList;
use crate::naptr::NaPtr;
use crate::ping_result::PingResult;
use crate::ptr_record::PtrRecord;
use crate::pxe_service::PxeService;
use crate::ra_interface::RaInterface;
use crate::rand_fd::RandFd;
use crate::rand_fd_list::RandFdList;
use crate::rebind_domain::RebindDomain;
use crate::resolvc::Resolvc;
use crate::server::Server;
use crate::server_fd::ServerFd;
use crate::shared_network::SharedNetwork;
use crate::snoop_record::SnoopRecord;
use crate::tag_if::TagIf;
use crate::tftp_prefix::TftpPrefix;
use crate::tftp_transfer::TftpTransfer;
use crate::txt_record::TxtRecord;
use crate::watch::Watch;

#[derive(Clone)]
pub struct Daemon {
    /* datastuctures representing the command-line and
       config file arguments. All set (including defaults)
       in option.c */
    pub options: [u32; OPTION_SIZE],
    pub default_resolv: Resolvc,
    pub resolv_files: Vec<Resolvc>,
    pub last_resolv: time_t,
    pub servers_file: String,
    pub mxnames: Vec<MxSrvRecord>,
    pub naptr: Vec<NaPtr>,
    pub txt: Vec<TxtRecord>,
    pub rr: Vec<TxtRecord>,
    pub ptr: Vec<PtrRecord>,
    pub host_records: Vec<HostRecord>,
    pub host_records_tail: usize,
    pub cnames: Vec<Cname>,
    pub auth_zones: Vec<AuthZone>,
    pub int_names: Vec<InterfaceName>,
    pub mxtarget: String,
    pub add_subnet4: Vec<MySubnet>,
    pub add_subnet6: Vec<MySubnet>,
    pub lease_file: String,
    pub username: String,
    pub groupname: String,
    pub scriptuser: String,
    pub luascript: String,
    pub authserver: String,
    pub hostmaster: String,
    pub authinterface: Vec<Iname>,
    pub secondary_forward_server: Vec<NameList>,
    pub osport: i32,
    pub group_set: i32,
    pub domain_suffix: String,
    pub cond_domain: Vec<CondDomain>,
    pub synth_domains: Vec<CondDomain>,
    pub runfile: String,
    pub lease_change_command: String,
    pub if_names: Vec<Iname>,
    pub if_addrs: Vec<Iname>,
    pub if_except: Vec<Iname>,
    pub dhcp_except: Vec<Iname>,
    pub auth_peers: Vec<Iname>,
    pub tftp_interfaces: Vec<Iname>,
    pub bogus_addr: Vec<BogusAddr>,
    pub ignore_addr: Vec<BogusAddr>,
    pub servers: Vec<Server>,
    pub servers_tail: usize,
    pub local_domains: Vec<Server>,
    pub server_array: Vec<Server>,
    pub no_rebind: Vec<RebindDomain>,
    pub server_has_wildcard: i32,
    pub serverarraysz: usize,
    pub serverarrayhwm: i32,
    pub ipsets: Vec<IpSets>,
    pub nftsets: Vec<IpSets>,
    pub allowlist_mask: u32,
    pub allowlists: Vec<AllowList>,
    pub log_fac: i32,
    pub log_file: String,
    pub max_logs: usize,
    pub randport_limit: i32,
    pub cachesize: usize,
    pub ftabsize: usize,
    pub port: u16,
    pub query_port: u16,
    pub min_port: u16,
    pub max_port: u16,
    pub local_ttl: u8,
    pub neg_ttl: u8,
    pub max_ttl: u8,
    pub min_cache_ttl: u8,
    pub max_cache_ttl: u8,
    pub auth_ttl: u8,
    pub dhcp_ttl: u8,
    pub use_dhcp_ttl: u32,
    pub dns_client_id: String,
    pub umbrella_org: u32,
    pub umbrella_asset: u32,
    pub umbrella_device: [u8; 8],
    pub host_index: u32,
    pub addn_hosts: Vec<HostsFile>,
    pub dhcp: Vec<DhcpContext>,
    pub dhcp6: Vec<DhcpContext>,
    pub ra_interfaces: Vec<RaInterface>,
    pub dhcp_confg: Vec<DhcpConfig>,
    pub dhcp_opts: Vec<DhcpOpt>,
    pub dhcp_match: Vec<DhcpOpt>,
    pub dhcp_opts6: Vec<DhcpOpt>,
    pub dhcp_match6: Vec<DhcpOpt>,
    pub dhcp_name_match: Vec<DhcpMatchName>,
    pub dhcp_pxe_vendors: Vec<DhcpPxeVendor>,
    pub dhcp_macs: Vec<DhcpMac>,
    pub boot_config: Vec<DhcpBoot>,
    pub pxe_services: Vec<PxeService>,
    pub tag_if: Vec<TagIf>,
    pub override_relays: Vec<AddrList>,
    pub relay4: Vec<DhcpRelay>,
    pub relay6: Vec<DhcpRelay>,
    pub delay_conf: Vec<DelayConfig>,
    pub xoverride: i32,
    pub enable_pxe: i32,
    pub doing_ra: i32,
    pub doing_dhcp6: i32,
    pub dhcp_ignore: Vec<DhcpNetidList>,
    pub dhcp_ignore_names: Vec<DhcpNetidList>,
    pub dhcp_gen_names: Vec<DhcpNetidList>,
    pub force_broadcast: Vec<DhcpNetidList>,
    pub bootp_dynamic: Vec<DhcpNetidList>,
    pub dhcp_hosts_file: Vec<HostsFile>,
    pub dhcp_opts_file: Vec<HostsFile>,
    pub dynamic_dirs: Vec<DynDir>,
    pub dhcp_max: i32,
    pub tftp_max: i32,
    pub fttp_mtu: i32,
    pub dhcp_server_port: i32,
    pub dhcp_client_port: i32,
    pub start_tftp_port: i32,
    pub end_tftp_port: i32,
    pub min_leasetime: u32,
    pub doctors: Vec<Doctor>,
    pub edns_pktsz: u32,
    pub tftp_prefix: String,
    pub if_prefix: Vec<TftpPrefix>,
    pub duid_enterprise: u32,
    pub duid_config_len: u32,
    pub duid_config: *mut u8,
    pub dbus_name: String,
    pub ubus_name: String,
    pub dump_file: String,
    pub dump_mask: i32,
    pub soa_sn: u32,
    pub soa_refresh: u32,
    pub soa_retry: u32,
    pub soa_expiry: u32,
    pub metrics: [u32; __METRIC_MAX],
    pub fast_retry_time: i32,
    pub fast_retry_timeout: i32,
    pub cache_max_expiry: i32,
    pub ds: Vec<DsConfig>,
    pub timestamp_file: String,
    pub packet: String,
    pub namebuff: String,
    pub workspacename: String,
    pub keyname: String,
    pub rr_status: *mut u32,
    pub rr_status_sz: i32,
    pub dnssec_no_time_check: i32,
    pub back_to_the_future: i32,
    pub frec_list: Vec<Frec>,
    pub free_frec_src: Vec<FrecSrc>,
    pub frec_src_count: i32,
    pub sfds: Vec<ServerFd>,
    pub interfaces: Vec<Irec>,
    pub listeners: Vec<Listener>,
    pub srv_save: Vec<Server>,
    pub packet_len: usize,
    pub fd_save: i32,
    pub tcp_pids: [pid_t; MAX_PROCS as usize],
    pub tcp_pipes: [i32; MAX_PROCS as usize],
    pub pipe_to_parent: i32,
    pub numrrand: i32,
    pub randomsocks: Vec<RandFd>,
    pub rfl_spare: Vec<RandFdList>,
    pub rfl_poll: Vec<RandFdList>,
    pub v6pktinfo: i32,
    pub interface_addrs: Vec<AddrList>,
    pub log_id: i32,
    pub log_display_id: i32,
    pub log_source_addr: Vec<IpAddr>,
    pub dhcpfd: i32,
    pub helperfd: i32,
    pub pxefd: i32,
    pub inotifyfd: i32,
    pub netlinkfd: i32,
    pub kernel_version: i32,
    pub dhcp_raw_fd: i32,
    pub dhcp_icmp_fd: i32,
    pub routefd: i32,
    pub dhcp_packet: iovec,
    pub dhcp_buff: String,
    pub dhcp_buff2: String,
    pub dhcp_buff3: String,
    pub ping_results: Vec<PingResult>,
    pub lease_stream: Vec<FILE>,
    pub bridges: Vec<DhcpBridge>,
    pub shared_networks: Vec<SharedNetwork>,
    pub duid_len: usize,
    pub duid: Vec<u8>,
    pub outpacket: Vec<u8>,
    pub dhcp6fd: i32,
    pub icmp6fd: i32,
    pub free_snoops: Vec<SnoopRecord>,
    #[cfg(target_os = "linux")]
    pub dbus: *mut c_void,
    #[cfg(target_os = "linux")]
    pub watches: Vec<Watch>,
    #[cfg(target_os = "linux")]
    pub ubus: *mut c_void,
    pub tftp_trans: Vec<TftpTransfer>,
    pub tftp_done_trans: Vec<TftpTransfer>,
    pub addrbuff: String,
    pub addrbuff2: String,
    pub dumpfd: i32,
}
