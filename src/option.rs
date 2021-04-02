use crate::defines::{Code, CODE, FACILITYNAMES, __sigset_t, OptionUsage, TxtRecord, DnsmasqDaemon, C2rustUnnamed10, MySockAddr, InAddr, SaFamily, In6Addr, __bswap_16, InAddrT, Server, DhcpNetId, DhcpNetIdList, DhcpConfig, HwaddrConfig, AddrList, DhcpContext, DhcpOpt, __bswap_32, Mysubnet, Resolvc, time_t, MxSrvRecord, Iname, NameList, AuthZone, AuthNameList, AllAddr, CondDomain, in_port_t, C2RustUnnamed, IpSets, TftpPrefix, DhcpBridge, SharedNetwork, _ISDIGIT, TagIf, DhcpMatchName, DhcpBoot, DelayConfig, PxeService, DhcpMac, AddrList2, DhcpPxeVendor, DhcpRelay, RaInterface, Doctor, InterfaceName, Cname, _ISSPACE, PtrRecord, NaPtr, HostRecord, BogusAddr, DhcpVendor, _ISXDIGIT, HostsFile, FILE, stat, timespec, DIR, Crec, OptionValue, TabEntryA};
use crate::util::{whine_malloc, safe_malloc, canonicalise, safe_strncpy, hostname_isequal, addr6part, setaddr6part, is_same_net, is_same_net6, parse_hex, legal_hostname, rand32, string_from_offset};
use crate::dnsmasq_log::{die, my_syslog};
use crate::dhcp_common::{lookup_dhcp_opt, lookup_dhcp_len, strip_hostname, display_opts, display_opts6};
use std::io::{stdin, stderr};
use crate::slack::dirent;
use crate::network::{mark_servers, cleanup_servers};
use crate::inotify::set_dynamic_inotify;
use std::process::exit;
use clap::{Arg, App, ArgMatches};
use winapi::um::ws2tcpip::inet_pton;

// todo: get value of doing dhcp6
// todo: get value of doing ra (implies dhcp6)

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
/* define this to get FACILITYNAMES */
// static mut mem_recover: libc::c_int = 0 as libc::c_int;
// static mut mem_jmp: jmp_buf =
//     [__jmp_buf_tag{__jmpbuf: [0; 8],
//                    __mask_was_saved: 0,
//                    __saved_mask: __sigset_t{__val: [0; 16],},}; 1];



pub fn prepare_matches() -> ArgMatches {
    App::new("dnsmasq-rs")
        .version("0.1")
        .author("Josh M.")
        .about("Rust port of dnsmasq with enhanced functionality")
        .arg(Arg::with_name("version")
            .short("v")
            .long("config")
            .takes_value(false)
            .help("show the version and exit"))
        .arg(Arg::with_name("no-hosts")
            .short("h")
            .long("no-hosts")
            .takes_value(false)
            .help("do not load the hosts file"))
        .arg(Arg::with_name("no-poll")
            .short("n")
            .long("no-poll")
            .takes_value(false)
            .help("do not poll the conf file, reload only on SIGHUP"))
        .arg(Arg::with_name("no-daemon").short("d").long("no-daemon").takes_value(false).help("do not run in the background"))
        .arg(Arg::with_name("log-queries").short("q").long("log-queries").takes_value(false).help("log DNS queries"))
        .arg(Arg::with_name("user").short("u").long("user").value_name("USER").help("user to run as"))
        .arg(Arg::with_name("group").short("g").long("group").value_name("GROUP").help("group to run as"))
        .arg(Arg::with_name("resolv-file").short("r").long("resolv-file").value_name("RESOLV_FILE").help("path to resolv.conf file"))
        .arg(Arg::with_name("servers-file").long("servers-files").value_name("SERVERS_FILE").hel("path to server options file"))
        .arg(Arg::with_name("mx-host").long("mx-host").short("m").value_name("MX_HOST"))
        .arg(Arg::with_name("mx-target").long("mx-target").short("t").value_name("MX_TARGET"))
        .arg(Arg::with_name("cache-size").long("cache-size").short("c").value_name("CACHE_SIZE"))
        .arg(Arg::with_name("port").long("port").short("p").value_name("PORT"))
                 .arg(Arg::with_name("dhcp-leasefile").long("dhcp-leasefile").short("l").value_name("DHCP_LEASE_FILE"))
        .arg(Arg::with_name("dhcp-lease").long("dhcp-lease").short("l").value_name("DHCP_LEASE"))
        .arg(Arg::with_name("dhcp-host").long("dhcp-host").short("G").value_name("DHCP_HOST"))
        .arg(Arg::with_name("dhcp-range").long("dhcp-range").short("F").value_name("DHCP_RANGE"))
        .arg(Arg::with_name("dhcp-option").long("dhcp-option").short("O").value_name("DHCP_OPTION"))
        .arg(Arg::with_Name("dhcp-boot").long("dhcp-boot").short("M").value_name("DHCP_BOOT"))

        .arg(Arg::with_name("domain").long("domain").short("s").value_name("DOMAIN"))
        .arg(Arg::with_name("domain-suffix").long("domain-suffix").short("s").value_name("DOMAIN_SUFFIX"))
        .arg(Arg::with_name("interface").long("interface").short("i").value_name("INTERFACE"))
        .arg(Arg::with_name("listen-address").long("listen-address").short(a).value_name("LISTEN_ADDRESS"))
        .arg(Arg::with_name("local-service").long("local-service").takes_value(false))
        .arg(Arg::with_name("bogus-priv").long("bogus-priv").short("b").takes_value(false))
        .arg(Arg::with_name("bogus-nxdomain").long("bogus-nxdomain").short("B").value_name("BOGUS_NXDOMAIN"))
        .arg(Arg::with_name("ignore-address").long("ignore-address").value_name("IGNORE_ADDRESS"))
        .arg(Arg::with_name("selfmx").long("selfmx").short("e").takes_value(false))
        .arg(Arg::with_name("filterwin2k").long("filterwin2k").short("f").takes_value(false))
        .arg(Arg::with_name("pid-file").long("pid-file").short("x").value_name("PID_FILE"))
        .arg(Arg::with_name("strict-order").long("strict-order").short("o").takes_value(false))
        .arg(Arg::with_name("server").long("server").short("S").value_name("SERVER"))
        .arg(Arg::with_name("rev-server").long("rev-server").value_name("REV_SERVER"))
        .arg(Arg::with_name("local").long("local").value_name("LOCAL"))
        .arg(Arg::with_name("address").long("address").short("A").value_name("ADDRESS"))
        .arg(Arg::with_name("conf-file").long("conf-file").short("C").value_name("CONF_FILE"))
        .arg(Arg::with_name("no-resolv").long("no-resolv").short("R").takes_value(false))
        .arg(Arg::with_name("expand-hosts").long("expand-hosts").short("E").takes_value(false))
        .arg(Arg::with_name("localmx").long("localmx").short("L").takes_value(false))
        .arg(Arg::with_name("local-ttl").long("local-ttl").short("T").value_name("LOCAL_TTL"))
        .arg(Arg::with_name("no-negcache").long("no-negcache").short("N").takes_value(false))
        .arg(Arg::with_name("addn-hosts").long("addn-hosts").short("H").takes_value(false))
        .arg(Arg::with_name("hostsdir").long("hostsdir").value_name("HOSTS_DIR"))
        .arg(Arg::with_name("query-port").long("query-port").value_name("QUERY_PORT"))
        .arg(Arg::with_name("except-interface").long("except-interface").value_name("EXCEPT_INTERFACE"))
        .arg(Arg::with_name("no-dhcp-interface").long("no-dhcp-interface").value_name("NO_DHCP_INTERFACE"))
        .arg(Arg::with_name("domain-needed").long("domain-needed").short("D").takes_value(false))
        .arg(Arg::with_name("dhcp-lease-max").long("dhcp-lease-max").short("X").value_name("DHCP_LEASE_MAX"))
        .arg(Arg::with_name("bind-interfaces").long("bind-interfaces").short("z").takes_value(false))
        .arg(Arg::with_name("read-ethers").long("read-ethers").short("Z").takes_value(false))
        .arg(Arg::with_name("alias").long("alias").short("V").value_name("ALIAS"))
        .arg(Arg::with_name("dhcp-vendorclass").long("dhcp-vendorclass").short("U").value_name("DHCP_VENDOR_CLASS"))
        .arg(Arg::with_name("dhcp-userclass").long("dhcp-userclass").short("j").value_name("DHCP_USER_CLASS"))
        .arg(Arg::with_name("dhcp-ignore").long("dhcp-ignore").short("J").value_name("DHCP_IGNORE"))
        .arg(Arg::with_name("edns-packet-max").long("edns-packet-max").short("P").value_name("EDNS_PACKET_MAX"))
        .arg(Arg::with_name("keep-in-foreground").long("keep-in-foreground").short("k").takes_value(false))
        .arg(Arg::with_name("dhcp-authoritative").long("dhcp-authoritative").short("K").takes_value(false))
        .arg(Arg::with_name("srv-host").long("srv-host").short("W").value_name("SRV_HOST"))
        .arg(Arg::with_name("localise-queries").long("localise-queries").short("y").takes_value(false))
        .arg(Arg::with_name("txt-record").long("txt-record").short("Y").value_name("TXT_RECORD"))
        .arg(Arg::with_name("caa-record").long("caa-record").value_name("CAA_RECORD"))
        .arg(Arg::with_name("dns-rr").long("dns-rr").value_name("DNS_RR"))
        .arg(Arg::with_name("enable-dbus").long("enable-dbus").short("1").takes_value(false))
        .arg(Arg::with_name("enable-ubus").long("enable-ubus").takes_value(false))
        .arg(Arg::with_name("bootp-dynamic").long("bootp-dynamic").short("3").value_name("BOOTP_DYNAMIC"))
        .arg(Arg::with_name("dhcp-mac").long("dhcp-mac").short("4").value_name("DHCP_MAC"))
        .arg(Arg::with_name("no-ping").long("no-ping").short("5").takes_value(false))
        .arg(Arg::with_name("dhcp-script").long("dhcp-script").short("6").value_name("DHCP_SCRIPT"))
        .arg(Arg::with_name("conf-dir").long("conf-dir").short("7").value_name("CONF_DIR"))
        .arg(Arg::with_name("log-facility").long("log-facility").short("8").value_name("LOG_FACILITY"))
        .arg(Arg::with_name("leasefile-ro").long("leasefile-ro").short("9").takes_value(false))
        .arg(Arg::with_name("script-on-renewal").long("script-on-renewal").takes_value(false))
        .arg(Arg::with_name("dns-forward-max").long("dns-forward-max").short("0").value_name("DNS_FORWARD_MAX"))
        .arg(Arg::with_name("clear-on-reload").long("clear-on-reload").takes_value(false))
        .arg(Arg::with_name("dhcp-ignore-names").long("dhcp-ignore-names").value_name("DHCP_IGNORE_NAMES"))
        .arg(Arg::with_name("enable-tftp").long("enable-tftp").takes_value(false))
        .arg(Arg::with_name("tftp-secure").long("tftp-secure").takes_value(false))
        .arg(Arg::with_name("tftp-no-fail").long("tftp-no-fail").takes_value(false))
        .arg(Arg::with_name("tftp-unique-root").long("tftp-unique-root").value_name("TFTP_UNIQUE_ROOT"))
        .arg(Arg::with_name("tftp-root").long("tftp-root").value_name("TFTP_ROOT"))
        .arg(Arg::with_name("tftp-max").long("tftp-max").value_name("TFTP_MAX"))
        .arg(Arg::with_name("tftp-mtu").long("tftp-mtu").value_name("TFTP_MTU"))
        .arg(Arg::with_name("tftp-lowercase").long("tftp-lowercase").takes_value(false))
        .arg(Arg::with_name("tftp-single-port").long("tftp-single-port").takes_value(false))
        .arg(Arg::with_name("ptr-record").long("ptr-record").value_name("PTR_RECORD"))
        .arg(Arg::with_name("naptr-record").long("naptr-record").value_name("NAPTR_RECORD"))
        .arg(Arg::with_name("bridge-interface").long("bridge-interface").value_name("BRIDGE_INTERFACE"))
        .arg(Arg::with_name("shared-network").long("shared-network").value_name("SHARED_NETWORK"))
        .arg(Arg::with_name("dhcp-option-force").long("dhcp-option-force").value_name("DHCP_OPTION_FORCE"))
        .arg(Arg::with_name("tftp-no-blocksize").long("tftp-no-blocksize").takes_value(false))
        .arg(Arg::with_name("log-dhcp").long("log-dhcp").takes_value(false))
        .arg(Arg::with_name("log-async").long("log-async").takes_value(false))
        .arg(Arg::with_name("dhcp-circuitid").long("dhcp-circuitid").value_name("DHCP_CIRCUIT_ID"))
        .arg(Arg::with_name("dhcp-remoteid").long("dhcp-remoteid").value_name("DHCP_REMOTE_ID"))
        .arg(Arg::with_name("dhcp-subscrid").long("dhcp-subscrid").value_name("DHCP_SUBSCR_ID"))
        .arg(Arg::with_name("dhcp-pxe-vendor").long("dhcp-pxe-vendor").value_name("DHCP_PXE_VENDOR"))
        .arg(Arg::with_name("interface-name").long("interface-name").value_name("INTERFACE_NAME"))
        .arg(Arg::with_name("dhcp-hostsfile").long("dhcp-hostsfiles").value_name("DHCP_HOSTS_FILE"))
        .arg(Arg::with_name("dhcp-optsfile").long("dhcp-optsfile").value_name("DHCP_OPTS_FILE"))
        .arg(Arg::with_name("dhcp-hostsdir").long("dhcp-hostsdir").value_name("DHCP_HOSTS_DIR"))
        .arg(Arg::with_name("dhcp-optsdir").long("dhcp-optsdir").value_name("DHCP_OPTS_DIR"))
        .arg(Arg::with_name("dhcp-no-override").long("dhcp-no-override").takes_value(false))
        .arg(Arg::with_name("tftp-port-range").long("tftp-port-range").value_name("TFTP_PORT_RANGE"))
        .arg(Arg::with_name("stop-dns-rebind").long("stop-dns-rebind").takes_value(false))
        .arg(Arg::with_name("rebind-domain-ok").long("rebind-domain-ok").takes_value(false))
        .arg(Arg::with_name("all-servers").long("all-servers").takes_value(false))
        .arg(Arg::with_name("dhcp-match").long("dhcp-match").value_name("DHCP_MATCH"))
        .arg(Arg::with_name("dhcp-name-match").long("dhcp-name-match").value_name("DHCP_NAME_MATCH"))
        .arg(Arg::with_name("dhcp-broadcast").long("dhcp-broadcast").value_name("DHCP_BROADCAST"))
        .arg(Arg::with_name("neg-ttl").long("neg-ttl").value_name("NEG_TTL"))
        .arg(Arg::with_name("max-ttl").long("max-ttl").value_name("MAX_TTL"))
        .arg(Arg::with_name("min-cache-ttl").long("min-cache-ttl").value_name("MIN_CACHE_TTL"))
        .arg(Arg::with_name("max-cache-ttl").long("max-cache-ttl").value_name("MAX_CACHE_TTL"))
        .arg(Arg::with_name("dhcp-alternate-port").long("dhcp-alternate-port").value_name("DHCP_ALTERNATE_PORT"))
        .arg(Arg::with_name("dhcp-scriptuser").long("dhcp-scriptuser").value_name("DHCP_SCRIPT_USER"))
        .arg(Arg::with_name("min-port").long("min-port").value_name("MIN_PORT"))
        .arg(Arg::with_name("max-port").long("max-port").value_name("MAX_PORT"))
        .arg(Arg::with_name("dhcp-fqdn").long("dhcp-fqdn").value_name("DHCP_FQDN"))
        .arg(Arg::with_name("cname").long("cname").value_name("CNAME"))
        .arg(Arg::with_name("pxe-prompt").long("pxe-prompt").value_name("PXE_PROMPT"))
        .arg(Arg::with_name("pxe-service").long("pxe-service").value_name("PXE_SERVICE"))
        .arg(Arg::with_name("test").long("test").value_name("TEST"))
        .arg(Arg::with_name("tag-if").long("tag-if").value_name("TAG_IF"))
        .arg(Arg::with_name("dhcp-proxy").long("dhcp-proxy").value_name("DHCP_PROXY"))
        .arg(Arg::with_name("dhcp-generate-names").long("dhcp-generate-names").takes_value(false))
        .arg(Arg::with_name("rebind-localhost-ok").long("rebind-localhost-ok").takes_value(false))
        .arg(Arg::with_name("add-mac").long("add-mac").value_name("MAC"))
        .arg(Arg::with_name("add-sbunet").long("add-subnet").value_name("SUBNET"))
        .arg(Arg::with_name("add-cpe-id").long("add-cpe-id").value_name("CPE_ID"))
        .arg(Arg::with_name("proxy-dnssec").long("proxy-dnssec").takes_value(false))
        .arg(Arg::with_name("dhcp-sequential-ip").long("dhcp-sequential-ip").takes_value(false))
        .arg(Arg::with_name("conntrack").long("conntrack").takes_value(false))
        .arg(Arg::with_name("dhcp-client-update").long("dhcp-client-update").takes_value(false))
        .arg(Arg::with_name("dhcp-luascript").long("dhcp-luascript").value_name("SCRIPT_NAME"))
        .arg(Arg::with_name("enable-ra").long("enable-ra").takes_value(false))
        .arg(Arg::with_name("dhcp-duid").long("dhcp-duid").value_name("DUID"))
        .arg(Arg::with_name("host-record").long("host-record").value("HOST_RECORD"))
        .arg(Arg::with_name("bind-dynamic").long("bind-dynamic").takes_value(false))
        .arg(Arg::with_name("auth-zone").long("auth-zone").value_name("AUTH_ZONE"))
        .arg(Arg::with_name("auth-server").long("auth-server").value_name("AUTH_SERVER"))
        .arg(Arg::with_name("auth-ttl").long("auth-ttl").value_name("TTL"))
        .arg(Arg::with_name("auth-soa").long("auth-soa").value_name("SOA"))
        .arg(Arg::with_name("auth-sec-servers").long("auth-sec-servers").value_name("SERVER"))
        .arg(Arg::with_name("auth-peer").long("auth-peer").value_name("PEER"))
        .arg(Arg::with_name("ipset").long("ipset").value_name("IPSET"))
        .arg(Arg::with_name("synth-domain").long("synth-domain").value_name("SYNTH_DOMAIN"))
        .arg(Arg::with_name("dnssec").long("dnssec").takes_value(false))
        .arg(Arg::with_name("trust-anchor").long("trust-anchor").value_name("TRUST_ANCHOR"))
        .arg(Arg::with_name("dnssec-debug").long("dnssec-debug").takes_value(false))
        .arg(Arg::with_name("dnssec-check-unsigned").long("dnssec-check-unsigned").takes_value(false))
        .arg(Arg::with_name("dnssec-no-timecheck").long("dnssec-no-timecheck").takes_value(false))
        .arg(Arg::with_name("dnssec-timestamp").long("dnssec-timestamp").value_name("TIMESTAMP"))
        .arg(Arg::with_name("dhcp-relay").long("dhcp-relay").value_name("DHCP_RELAY"))
        .arg(Arg::with_name("ra-param").long("ra-param").value_name("RA_PARAM"))
        .arg(Arg::with_name("quiet-dhcp").long("quiet-dhcp").takes_value(false))
        .arg(Arg::with_name("quiet-dhcp6").long("quiet-dhcp6").takes_value(false))
        .arg(Arg::with_name("quiet-ra").long("quiet-ra").takes_value(false))
        .arg(Arg::with_name("dns-loop-detect").long("dns-loop-detect").takes_value(false))
        .arg(Arg::with_name("script-arp").long("script-arp").takes_value(false))
        .arg(Arg::with_name("dhcp-ttl").long("dhcp-ttl").value_name("TTL"))
        .arg(Arg::with_name("dhcp-reply-delay").long("dhcp-relply-delay").value_name("DELAY"))
        .arg(Arg::with_name("dhcp-rapid-commit").long("dhcp-rapid-commit").takes_value(false))
        .arg(Arg::with_name("dumpfile").long("dumpfile").value_name("FILE"))
        .arg(Arg::with_name("dumpmask").long("dumpmask").value_name("MASK"))
        .arg(Arg::with_name("dhcp-ignore-clid").long("dhcp-ignore-clid").takes_value(false))
        .get_matches()
}

// TODO: move usage data to match arg
// TODO: correct for N, flag type, etc.

pub static option_values: [OptionValue; 166] =
    [OptionValue{ name: String::from("version"), val: 'v' as i32,},
        OptionValue{ name: String::from("no-hosts"), val: 'h' as i32,},
        OptionValue{ name: String::from("no-poll"), val: 'n' as i32,},
        OptionValue{ name: String::from("help"), val: 'w' as i32,},
        OptionValue{ name: String::from("no-daemon"), val: 'd' as i32,},
        OptionValue{ name: String::from("log-queries"), val: 'q' as i32,},
        OptionValue{ name: String::from("user"), val: 'u' as i32,},
        OptionValue{ name: String::from("group"), val: 'g' as i32,},
        OptionValue{ name: String::from("resolv-file"), val: 'r' as i32,},
        OptionValue{ name: String::from("servers-file"), val: 333,},
        OptionValue{ name: String::from("mx-host"), val: 'm' as i32,},
        OptionValue{ name: String::from("mx-target"), val: 't' as i32,},
        OptionValue{ name: String::from("cache-size"), val: 'c' as i32,},
        OptionValue{ name: String::from("port"), val: 'p' as i32,},
        OptionValue{ name: String::from("dhcp-leasefile"), val: 'l' as i32,},
        OptionValue{ name: String::from("dhcp-lease"), val: 'l' as i32,},
        OptionValue{ name: String::from("dhcp-host"), val: 'G' as i32,},
        OptionValue{ name: String::from("dhcp-range"), val: 'F' as i32,},
        OptionValue{ name: String::from("dhcp-option"), val: 'O' as i32,},
        OptionValue{ name: String::from("dhcp-boot"), val: 'M' as i32,},
        OptionValue{ name: String::from("domain"), val: 's' as i32,},
        OptionValue{ name: String::from("domain-suffix"), val: 's' as i32,},
        OptionValue{ name: String::from("interface"), val: 'i' as i32,},
        OptionValue{ name: String::from("listen-address"), val: 'a' as i32,},
        OptionValue{ name: String::from("local-service"), val: 335,},
        OptionValue{ name: String::from("bogus-priv"), val: 'b' as i32,},
        OptionValue{ name: String::from("bogus-nxdomain"), val: 'B' as i32,},
        OptionValue{ name: String::from("ignore-address"), val: 338,},
        OptionValue{ name: String::from("selfmx"), val: 'e' as i32,},
        OptionValue{ name: String::from("filterwin2k"), val: 'f' as i32,},
        OptionValue{ name: String::from("pid-file"), val: 'x' as i32,},
        OptionValue{ name: String::from("strict-order"), val: 'o' as i32,},
        OptionValue{ name: String::from("server"), val: 'S' as i32,},
        OptionValue{ name: String::from("rev-server"), val: 332,},
        OptionValue{ name: String::from("local"), val: 286,},
        OptionValue{ name: String::from("address"), val: 'A' as i32,},
        OptionValue{ name: String::from("conf-file"), val: 'C' as i32,},
        OptionValue{ name: String::from("no-resolv"), val: 'R' as i32,},
        OptionValue{ name: String::from("expand-hosts"), val: 'E' as i32,},
        OptionValue{ name: String::from("localmx"), val: 'L' as i32,},
        OptionValue{ name: String::from("local-ttl"), val: 'T' as i32,},
        OptionValue{ name: String::from("no-negcache"), val: 'N' as i32,},
        OptionValue{ name: String::from("addn-hosts"), val: 'H' as i32,},
        OptionValue{ name: String::from("hostsdir"), val: 342,},
        OptionValue{ name: String::from("query-port"), val: 'Q' as i32,},
        OptionValue{ name: String::from("except-interface"), val: 'I' as i32,},
        OptionValue{ name: String::from("no-dhcp-interface"), val: '2' as i32,},
        OptionValue{ name: String::from("domain-needed"), val: 'D' as i32,},
        OptionValue{ name: String::from("dhcp-lease-max"), val: 'X' as i32,},
        OptionValue{ name: String::from("bind-interfaces"), val: 'z' as i32,},
        OptionValue{ name: String::from("read-ethers"), val: 'Z' as i32,},
        OptionValue{ name: String::from("alias"), val: 'V' as i32,},
        OptionValue{ name: String::from("dhcp-vendorclass"), val: 'U' as i32,},
        OptionValue{ name: String::from("dhcp-userclass"), val: 'j' as i32,},
        OptionValue{ name: String::from("dhcp-ignore"), val: 'J' as i32,},
        OptionValue{ name: String::from("edns-packet-max"), val: 'P' as i32,},
        OptionValue{ name: String::from("keep-in-foreground"), val: 'k' as i32,},
        OptionValue{ name: String::from("dhcp-authoritative"), val: 'K' as i32,},
        OptionValue{ name: String::from("srv-host"), val: 'W' as i32,},
        OptionValue{ name: String::from("localise-queries"), val: 'y' as i32,},
        OptionValue{ name: String::from("txt-record"), val: 'Y' as i32,},
        OptionValue{ name: String::from("caa-record"), val: 356,},
        OptionValue{ name: String::from("dns-rr"), val: 310,},
        OptionValue{ name: String::from("enable-dbus"), val: '1' as i32,},
        OptionValue{ name: String::from("enable-ubus"), val: 354,},
        OptionValue{ name: String::from("bootp-dynamic"), val: '3' as i32,},
        OptionValue{ name: String::from("dhcp-mac"), val: '4' as i32,},
        OptionValue{ name: String::from("no-ping"), val: '5' as i32,},
        OptionValue{ name: String::from("dhcp-script"), val: '6' as i32,},
        OptionValue{ name: String::from("conf-dir"), val: '7' as i32,},
        OptionValue{ name: String::from("log-facility"), val: '8' as i32,},
        OptionValue{ name: String::from("leasefile-ro"), val: '9' as i32,},
        OptionValue{ name: String::from("script-on-renewal"), val: 360,},
        OptionValue{ name: String::from("dns-forward-max"), val: '0' as i32,},
        OptionValue{ name: String::from("clear-on-reload"), val: 256,},
        OptionValue{ name: String::from("dhcp-ignore-names"), val: 257,},
        OptionValue{ name: String::from("enable-tftp"), val: 258,},
        OptionValue{ name: String::from("tftp-secure"), val: 259,},
        OptionValue{ name: String::from("tftp-no-fail"), val: 344,},
        OptionValue{ name: String::from("tftp-unique-root"), val: 274,},
        OptionValue{ name: String::from("tftp-root"), val: 260,},
        OptionValue{ name: String::from("tftp-max"), val: 263,},
        OptionValue{ name: String::from("tftp-mtu"), val: 349,},
        OptionValue{ name: String::from("tftp-lowercase"), val: 309,},
        OptionValue{ name: String::from("tftp-single-port"), val: 359,},
        OptionValue{ name: String::from("ptr-record"), val: 261,},
        OptionValue{ name: String::from("naptr-record"), val: 287,},
        OptionValue{ name: String::from("bridge-interface"), val: 262,},
        OptionValue{ name: String::from("shared-network"), val: 357,},
        OptionValue{ name: String::from("dhcp-option-force"), val: 264,},
        OptionValue{ name: String::from("tftp-no-blocksize"), val: 265,},
        OptionValue{ name: String::from("log-dhcp"), val: 266,},
        OptionValue{ name: String::from("log-async"), val: 267,},
        OptionValue{ name: String::from("dhcp-circuitid"), val: 268,},
        OptionValue{ name: String::from("dhcp-remoteid"), val: 269,},
        OptionValue{ name: String::from("dhcp-subscrid"), val: 270,},
        OptionValue{ name: String::from("dhcp-pxe-vendor"), val: 361,},
        OptionValue{ name: String::from("interface-name"), val: 271,},
        OptionValue{ name: String::from("dhcp-hostsfile"), val: 273,},
        OptionValue{ name: String::from("dhcp-optsfile"), val: 280,},
        OptionValue{ name: String::from("dhcp-hostsdir"), val: 340,},
        OptionValue{ name: String::from("dhcp-optsdir"), val: 341,},
        OptionValue{ name: String::from("dhcp-no-override"), val: 275,},
        OptionValue{ name: String::from("tftp-port-range"), val: 276,},
        OptionValue{ name: String::from("stop-dns-rebind"), val: 277,},
        OptionValue{ name: String::from("rebind-domain-ok"), val: 298,},
        OptionValue{ name: String::from("all-servers"), val: 278,},
        OptionValue{ name: String::from("dhcp-match"), val: 281,},
        OptionValue{ name: String::from("dhcp-name-match"), val: 355,},
        OptionValue{ name: String::from("dhcp-broadcast"), val: 282,},
        OptionValue{ name: String::from("neg-ttl"), val: 283,},
        OptionValue{ name: String::from("max-ttl"), val: 297,},
        OptionValue{ name: String::from("min-cache-ttl"), val: 339,},
        OptionValue{ name: String::from("max-cache-ttl"), val: 312,},
        OptionValue{ name: String::from("dhcp-alternate-port"), val: 284,},
        OptionValue{ name: String::from("dhcp-scriptuser"), val: 285,},
        OptionValue{ name: String::from("min-port"), val: 288,},
        OptionValue{ name: String::from("max-port"), val: 345,},
        OptionValue{ name: String::from("dhcp-fqdn"), val: 289,},
        OptionValue{ name: String::from("cname"), val: 290,},
        OptionValue{ name: String::from("pxe-prompt"), val: 291,},
        OptionValue{ name: String::from("pxe-service"), val: 292,},
        OptionValue{ name: String::from("test"), val: 293,},
        OptionValue{ name: String::from("tag-if"), val: 294,},
        OptionValue{ name: String::from("dhcp-proxy"), val: 295,},
        OptionValue{ name: String::from("dhcp-generate-names"), val: 296,},
        OptionValue{ name: String::from("rebind-localhost-ok"), val: 299,},
        OptionValue{ name: String::from("add-mac"), val: 300,},
        OptionValue{ name: String::from("add-subnet"), val: 325,},
        OptionValue{ name: String::from("add-cpe-id"), val: 346,},
        OptionValue{ name: String::from("proxy-dnssec"), val: 301,},
        OptionValue{ name: String::from("dhcp-sequential-ip"), val: 302,},
        OptionValue{ name: String::from("conntrack"), val: 303,},
        OptionValue{ name: String::from("dhcp-client-update"), val: 304,},
        OptionValue{ name: String::from("dhcp-luascript"), val: 305,},
        OptionValue{ name: String::from("enable-ra"), val: 306,},
        OptionValue{ name: String::from("dhcp-duid"), val: 307,},
        OptionValue{ name: String::from("host-record"), val: 308,},
        OptionValue{ name: String::from("bind-dynamic"), val: 311,},
        OptionValue{ name: String::from("auth-zone"), val: 313,},
        OptionValue{ name: String::from("auth-server"), val: 314,},
        OptionValue{ name: String::from("auth-ttl"), val: 315,},
        OptionValue{ name: String::from("auth-soa"), val: 316,},
        OptionValue{ name: String::from("auth-sec-servers"), val: 317,},
        OptionValue{ name: String::from("auth-peer"), val: 318,},
        OptionValue{ name: String::from("ipset"), val: 319,},
        OptionValue{ name: String::from("synth-domain"), val: 320,},
        OptionValue{ name: String::from("dnssec"), val: 329,},
        OptionValue{ name: String::from("trust-anchor"), val: 330,},
        OptionValue{ name: String::from("dnssec-debug"), val: 331,},
        OptionValue{ name: String::from("dnssec-check-unsigned"), val: 334,},
        OptionValue{ name: String::from("dnssec-no-timecheck"), val: 336,},
        OptionValue{ name: String::from("dnssec-timestamp"), val: 343,},
        OptionValue{ name: String::from("dhcp-relay"), val: 323,},
        OptionValue{ name: String::from("ra-param"), val: 324,},
        OptionValue{ name: String::from("quiet-dhcp"), val: 326,},
        OptionValue{ name: String::from("quiet-dhcp6"), val: 327,},
        OptionValue{ name: String::from("quiet-ra"), val: 328,},
        OptionValue{ name: String::from("dns-loop-detect"), val: 337,},
        OptionValue{ name: String::from("script-arp"), val: 347,},
        OptionValue{ name: String::from("dhcp-ttl"), val: 348,},
        OptionValue{ name: String::from("dhcp-reply-delay"), val: 350,},
        OptionValue{ name: String::from("dhcp-rapid-commit"), val: 351,},
        OptionValue{ name: String::from("dumpfile"), val: 352,},
        OptionValue{ name: String::from("dumpmask"), val: 353,},
        OptionValue{ name: String::from("dhcp-ignore-clid"), val: 358,}];

static mut usage: [OptionUsage; 164] =
    [
        OptionUsage{opt: 97, rept: 62 , flagdesc: String::from("<ipaddr>"), desc:  String::from("Specify local address(es) to listen on."), arg: String::from("") },
        OptionUsage{ opt: 65, rept: 62 , flagdesc: String::from("/<domain>/<ipaddr>"), desc:  String::from("Return ipaddr for all hosts in specified domains."), arg: String::from("") },
        OptionUsage{ opt: 98, rept: 0 ,flagdesc: String::from(""), desc:  String::from("Fake reverse lookups for RFC1918 private address ranges."), arg: String::from("") },
        OptionUsage{ opt: 66, rept: 62 , flagdesc: String::from("<ipaddr>"), desc:  String::from("Treat ipaddr NXDOMAIN (defeats Verisign wildcard)."), arg: String::from("") },
        OptionUsage{ opt: 99, rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Specify the size of the cache in entries (defaults to %s)."), arg: String::from("")},
OptionUsage{ opt: 67, rept: 62 , flagdesc: String::from("<path>"), desc:  String::from("Specify configuration file (defaults to %s)."), arg: String::from("/etc/dnsmasq.conf")},
OptionUsage{ opt: 100, rept: 6 ,flagdesc: String::from(""), desc:  String::from("Do NOT fork into the background: run in debug mode."), arg: String::from("") },
OptionUsage{ opt: 68, rept: 12 ,flagdesc: String::from(""), desc:  String::from("Do NOT forward queries with no domain part."), arg: String::from("") },
OptionUsage{ opt: 101, rept: 3 ,flagdesc: String::from(""), desc:  String::from("Return self-pointing MX records for local hosts."), arg: String::from("") },
OptionUsage{ opt: 69, rept: 9 ,flagdesc: String::from(""), desc:  String::from("Expand simple names in /etc/hosts with domain-suffix."), arg: String::from("") },
OptionUsage{ opt: 102, rept: 1 ,flagdesc: String::from(""), desc:  String::from("Don\'t forward spurious DNS requests from Windows hosts."), arg: String::from("") },
OptionUsage{ opt: 70, rept: 62 , flagdesc:  String::from("<ipaddr>"), desc:  String::from("Enable DHCP in the range given with lease duration."), arg: String::from("") },
OptionUsage{ opt: 103, rept: (62  + 1 ), flagdesc: String::from("<groupname>"), desc:  String::from("Change to this group after startup (defaults to %s)."), arg: String::from("dip")},
OptionUsage{ opt: 71, rept: 62 , flagdesc: String::from("<hostspec>"), desc:  String::from("Set address or hostname for a specified machine."), arg: String::from("") },
OptionUsage{ opt: 273 , rept: 62 , flagdesc: String::from("<path>"), desc:  String::from("Read DHCP host specs from file."), arg: String::from("") },
OptionUsage{ opt: 280 , rept: 62 , flagdesc: String::from("<path>"), desc:  String::from("Read DHCP option specs from file."), arg: String::from("") },
OptionUsage{ opt: 340 , rept: 62 , flagdesc: String::from("<path>"), desc:  String::from("Read DHCP host specs from a directory."), arg: String::from("") },
OptionUsage{ opt: 341 , rept: 62 , flagdesc: String::from("<path>"), desc:  String::from("Read DHCP options from a directory."), arg: String::from("") },
OptionUsage{ opt: 294 , rept: 62 , flagdesc: String::from("tag-expression"), desc:  String::from("Evaluate conditional tag expression."), arg: String::from("") },
OptionUsage{ opt: 104, rept: 4 ,flagdesc: String::from(""), desc:  String::from("Do NOT load %s file."), arg: String::from("/etc/hosts")},
OptionUsage{ opt: 72, rept: 62 , flagdesc: String::from("<path>"), desc:  String::from("Specify a hosts file to be read in addition to %s."), arg: String::from("/etc/hosts")},
OptionUsage{ opt: 342 , rept: 62 , flagdesc: String::from("<path>"), desc:  String::from("Read hosts files from a directory."), arg: String::from("") },
OptionUsage{ opt: 105, rept: 62 , flagdesc: String::from("<interface>"), desc:  String::from("Specify interface(s) to listen on."), arg: String::from("") },
OptionUsage{ opt: 73, rept: 62 , flagdesc: String::from("<interface>"), desc:  String::from("Specify interface(s) NOT to listen on."), arg: String::from("") },
OptionUsage{ opt: 106, rept: 62 , flagdesc:  String::from("set:<tag>, class>"), desc:  String::from("Map DHCP user class to tag."), arg: String::from("") },
OptionUsage{ opt: 268 , rept: 62 , flagdesc:  String::from("set:<tag>, circuit>"), desc:  String::from("Map RFC3046 circuit-id to tag."), arg: String::from("") },
OptionUsage{ opt: 269 , rept: 62 , flagdesc:  String::from("set:<tag>, remote>"), desc:  String::from("Map RFC3046 remote-id to tag."), arg: String::from("") },
OptionUsage{ opt: 270 , rept: 62 , flagdesc:  String::from("set:<tag>, remote>"), desc:  String::from("Map RFC3993 subscriber-id to tag."), arg: String::from("") },
OptionUsage{ opt: 361 , rept: 62 , flagdesc:  String::from("<vendor>["), desc:  String::from("Specify vendor class to match for PXE requests."), arg: String::from("") },
        OptionUsage{ opt: 74, rept: 62 , flagdesc: String::from("tag:<tag>..."), desc:  String::from("Don\'t do DHCP for hosts with tag set."), arg: String::from("") },
        OptionUsage{ opt: 282 , rept: 62 , flagdesc: String::from("[=tag:<tag>...]"), desc:  String::from("Force broadcast replies for hosts with tag set."), arg: String::from("") },
        OptionUsage{ opt: 107, rept: 16 , flagdesc: String::from(""), desc:  String::from("Do NOT fork into the background, do NOT run in debug mode."), arg: String::from("") },
        OptionUsage{ opt: 75, rept: 17 , flagdesc: String::from(""), desc:  String::from("Assume we are the only DHCP server on the local network."), arg: String::from("") },
        OptionUsage{ opt: 108, rept: (62  + 1 ), flagdesc: String::from("<path>"), desc:  String::from("Specify where to store DHCP leases (defaults to %s)."), arg: String::from("/var/lib/misc/dnsmasq.leases")},
        OptionUsage{ opt: 76, rept: 10 , flagdesc: String::from(""), desc:  String::from("Return MX records for local hosts."), arg: String::from("") },
        OptionUsage{ opt: 109, rept: 62 , flagdesc:  String::from("<host_name>, target>, pref>"), desc:  String::from("Specify an MX record."), arg: String::from("") },
        OptionUsage{ opt: 77, rept: 62 , flagdesc:  String::from("<bootp opts>"), desc:  String::from("Specify BOOTP options to DHCP server."), arg: String::from("") },
        OptionUsage{ opt: 110, rept: 5 , flagdesc: String::from(""), desc:  String::from("Do NOT poll %s file, reload only on SIGHUP."), arg: String::from("/etc/resolv.conf")},
        OptionUsage{ opt: 78, rept: 11 , flagdesc: String::from(""), desc:  String::from("Do NOT cache failed search results."), arg: String::from("") },
        OptionUsage{ opt: 111, rept: 7 , flagdesc: String::from(""), desc:  String::from("Use nameservers strictly in the order given in %s."), arg: String::from("/etc/resolv.conf")},
        OptionUsage{ opt: 79, rept: 62 , flagdesc: String::from("<optspec>"), desc:  String::from("Specify options to be sent to DHCP clients."), arg: String::from("") },
        OptionUsage{ opt: 264 , rept: 62 , flagdesc: String::from("<optspec>"), desc:  String::from("DHCP option sent even if the client does not request it."), arg: String::from("") },
        OptionUsage{ opt: 112, rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Specify port to listen for DNS requests on (defaults to 53)."), arg: String::from("") },
        OptionUsage{ opt: 80, rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Maximum supported UDP packet size for EDNS.0 (defaults to %s)."), arg: String::from("")},
OptionUsage{ opt: 113, rept: 62 ,flagdesc: String::from(""), desc:  String::from("Log DNS queries."), arg: String::from("") },
OptionUsage{ opt: 81, rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Force the originating port for upstream DNS queries."), arg: String::from("") },
OptionUsage{ opt: 82, rept: 8 ,flagdesc: String::from(""), desc:  String::from("Do NOT read resolv.conf."), arg: String::from("") },
OptionUsage{ opt: 114, rept: 62 , flagdesc: String::from("<path>"), desc:  String::from("Specify path to resolv.conf (defaults to %s)."), arg: String::from("/etc/resolv.conf")},
OptionUsage{ opt: 333 , rept: (62  + 1 ), flagdesc: String::from("<path>"), desc:  String::from("Specify path to file with server= options"), arg: String::from("") },
OptionUsage{ opt: 83, rept: 62 , flagdesc: String::from("/<domain>/<ipaddr>"), desc:  String::from("Specify address(es) of upstream servers with optional domains."), arg: String::from("") },
OptionUsage{ opt: 332 , rept: 62 , flagdesc:  String::from("<addr>/<prefix>, ipaddr>"), desc:  String::from("Specify address of upstream servers for reverse address queries"), arg: String::from("") },
OptionUsage{ opt: 286 , rept: 62 , flagdesc: String::from("/<domain>/"), desc:  String::from("Never forward queries to specified domains."), arg: String::from("") },
OptionUsage{ opt: 115, rept: 62 , flagdesc:  String::from("<domain>[, range>]"), desc:  String::from("Specify the domain to besigned in DHCP leases."), arg: String::from("") },
OptionUsage{ opt: 116, rept: (62  + 1 ), flagdesc: String::from("<host_name>"), desc:  String::from("Specify default target in an MX record."), arg: String::from("") },
OptionUsage{ opt: 84, rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Specify time-to-live in seconds for replies from /etc/hosts."), arg: String::from("") },
OptionUsage{ opt: 283 , rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Specify time-to-live in seconds for negative caching."), arg: String::from("") },
OptionUsage{ opt: 297 , rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Specify time-to-live in seconds for maximum TTL to send to clients."), arg: String::from("") },
OptionUsage{ opt: 312 , rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Specify time-to-live ceiling for cache."), arg: String::from("") },
OptionUsage{ opt: 339 , rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Specify time-to-live floor for cache."), arg: String::from("") },
OptionUsage{ opt: 117, rept: (62  + 1 ), flagdesc: String::from("<username>"), desc:  String::from("Change to this user after startup. (defaults to %s)."), arg: String::from("nobody")},
OptionUsage{ opt: 85, rept: 62 , flagdesc:  String::from("set:<tag>, class>"), desc:  String::from("Map DHCP vendor class to tag."), arg: String::from("") },
OptionUsage{ opt: 118, rept: 0 ,flagdesc: String::from(""), desc:  String::from("Display dnsmasq version and copyright information."), arg: String::from("") },
OptionUsage{ opt: 86, rept: 62 , flagdesc:  String::from("<ipaddr>, ipaddr>, netmask>"), desc:  String::from("Translate IPv4 addresses from upstream servers."), arg: String::from("") },
OptionUsage{ opt: 87, rept: 62 , flagdesc:  String::from("<name>, target>"), desc:  String::from("Specify a SRV record."), arg: String::from("") },
        OptionUsage{ opt: 119, rept: 0 , flagdesc: String::from(""), desc:  String::from("Display this message. Use --help dhcp or --help dhcp6 for known DHCP options."), arg: String::from("") },
        OptionUsage{ opt: 120, rept: (62  + 1 ), flagdesc: String::from("<path>"), desc:  String::from("Specify path of PID file (defaults to %s)."), arg: String::from("/var/run/dnsmasq.pid")},
        OptionUsage{ opt: 88, rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Specify maximum number of DHCP leases (defaults to %s)."), arg: String::from("")},
OptionUsage{ opt: 121, rept: 18 ,flagdesc: String::from(""), desc:  String::from("Answer DNS queries based on the interface a query was sent to."), arg: String::from("") },
OptionUsage{ opt: 89, rept: 62 , flagdesc:  String::from("<name>, txt>[, txt]"), desc:  String::from("Specify TXT DNS record."), arg: String::from("") },
OptionUsage{ opt: 261 , rept: 62 , flagdesc:  String::from("<name>, target>"), desc:  String::from("Specify PTR DNS record."), arg: String::from("") },
OptionUsage{ opt: 271 , rept: 62 , flagdesc:  String::from("<name>, interface>"), desc:  String::from("Give DNS name to IPv4 address of interface."), arg: String::from("") },
OptionUsage{ opt: 122, rept: 13 ,flagdesc: String::from(""), desc:  String::from("Bind only to interfaces in use."), arg: String::from("") },
OptionUsage{ opt: 90, rept: 14 ,flagdesc: String::from(""), desc:  String::from("Read DHCP static host information from %s."), arg: String::from("/etc/ethers")},
OptionUsage{ opt: 49, rept: (62  + 1 ), flagdesc: String::from("[=<busname>]"), desc:  String::from("Enable the DBus interface for setting upstream servers, etc."), arg: String::from("") },
OptionUsage{ opt: 354 , rept: (62  + 1 ), flagdesc: String::from("[=<busname>]"), desc:  String::from("Enable the UBus interface."), arg: String::from("") },
OptionUsage{ opt: 50, rept: 62 , flagdesc: String::from("<interface>"), desc:  String::from("Do not provide DHCP on this interface, only provide DNS."), arg: String::from("") },
OptionUsage{ opt: 51, rept: 62 , flagdesc: String::from("[=tag:<tag>]..."), desc:  String::from("Enable dynamic address allocation for bootp."), arg: String::from("") },
OptionUsage{ opt: 52, rept: 62 , flagdesc:  String::from("set:<tag>, mac address>"), desc:  String::from("Map MAC address (with wildcards) to option set."), arg: String::from("") },
OptionUsage{ opt: 262 , rept: 62 , flagdesc:  String::from("<iface>, alias>.."), desc:  String::from("Treat DHCP requests on aliases arriving from interface."), arg: String::from("") },
OptionUsage{ opt: 357 , rept: 62 , flagdesc:  String::from("<iface>|<addr>, addr>"), desc:  String::from("Specify extra networks sharing a broadcast domain for DHCP"), arg: String::from("") },
OptionUsage{ opt: 53, rept: 21 ,flagdesc: String::from(""), desc:  String::from("Disable ICMP echo address checking in the DHCP server."), arg: String::from("") },
OptionUsage{ opt: 54, rept: (62  + 1 ), flagdesc: String::from("<path>"), desc:  String::from("Shell script to run on DHCP lease creation and destruction."), arg: String::from("") },
OptionUsage{ opt: 305 , rept: 62 , flagdesc: String::from("path"), desc:  String::from("Lua script to run on DHCP lease creation and destruction."), arg: String::from("") },
OptionUsage{ opt: 285 , rept: (62  + 1 ), flagdesc: String::from("<username>"), desc:  String::from("Run lease-change scripts this user."), arg: String::from("") },
OptionUsage{ opt: 347 , rept: 53 ,flagdesc: String::from(""), desc:  String::from("Call dhcp-script with changes to local ARP table."), arg: String::from("") },
OptionUsage{ opt: 55, rept: 62 , flagdesc: String::from("<path>"), desc:  String::from("Read configuration from all the files in this directory."), arg: String::from("") },
OptionUsage{ opt: 56, rept: (62  + 1 ), flagdesc: String::from("<facility>|<file>"), desc:  String::from("Log to this syslog facility or file. (defaults to DAEMON)"), arg: String::from("") },
OptionUsage{ opt: 57, rept: 22 ,flagdesc: String::from(""), desc:  String::from("Do not use leasefile."), arg: String::from("") },
OptionUsage{ opt: 48, rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Maximum number of concurrent DNS queries. (defaults to %s)"), arg: String::from("")},
OptionUsage{ opt: 256 , rept: 24 , flagdesc: String::from(""), desc:  String::from("Clear DNS cache when reloading %s."), arg: String::from("/etc/resolv.conf")},
OptionUsage{ opt: 257 , rept: 62 , flagdesc: String::from("[=tag:<tag>]..."), desc:  String::from("Ignore hostnames provided by DHCP clients."), arg: String::from("") },
OptionUsage{ opt: 275 , rept: 30 , flagdesc: String::from(""), desc:  String::from("Do NOT reuse filename and server fields for extra DHCP options."), arg: String::from("") },
OptionUsage{ opt: 258 , rept: 62 , flagdesc:  String::from("[=<intr>[, intr>]]"), desc:  String::from("Enable integrated read-only TFTP server."), arg: String::from("") },
OptionUsage{ opt: 260 , rept: 62 , flagdesc:  String::from("<dir>[, iface>]"), desc:  String::from("Export files by TFTP only from the specified subtree."), arg: String::from("") },
OptionUsage{ opt: 274 , rept: 62 , flagdesc: String::from("[=ip|mac]"), desc:  String::from("Add client IP or hardware address to tftp-root."), arg: String::from("") },
OptionUsage{ opt: 259 , rept: 26 , flagdesc: String::from(""), desc:  String::from("Allow access only to files owned by the user running dnsmasq."), arg: String::from("") },
OptionUsage{ opt: 344 , rept: 52 , flagdesc: String::from(""), desc:  String::from("Do not terminate the service if TFTP directories are inaccessible."), arg: String::from("") },
OptionUsage{ opt: 263 , rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Maximum number of concurrent TFTP transfers (defaults to %s)."), arg: String::from("")},
OptionUsage{ opt: 349 , rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Maximum MTU to use for TFTP transfers."), arg: String::from("") },
OptionUsage{ opt: 265 , rept: 27 ,flagdesc: String::from(""), desc:  String::from("Disable the TFTP blocksize extension."), arg: String::from("") },
OptionUsage{ opt: 309 , rept: 38 ,flagdesc: String::from(""), desc:  String::from("Convert TFTP filenames to lowercase"), arg: String::from("") },
OptionUsage{ opt: 276 , rept: (62  + 1 ), flagdesc:  String::from("<start>, end>"), desc:  String::from("Ephemeral port range for use by TFTP transfers."), arg: String::from("") },
OptionUsage{ opt: 359 , rept: 60 ,flagdesc: String::from(""), desc:  String::from("Use only one port for TFTP server."), arg: String::from("") },
OptionUsage{ opt: 266 , rept: 28 ,flagdesc: String::from(""), desc:  String::from("Extra logging for DHCP."), arg: String::from("") },
OptionUsage{ opt: 267 , rept: (62  + 1 ), flagdesc: String::from("[=<integer>]"), desc:  String::from("Enableync. logging; optionally set queue length."), arg: String::from("") },
OptionUsage{ opt: 277 , rept: 31 ,flagdesc: String::from(""), desc:  String::from("Stop DNS rebinding. Filter private IP ranges when resolving."), arg: String::from("") },
OptionUsage{ opt: 299 , rept: 25 ,flagdesc: String::from(""), desc:  String::from("Allow rebinding of 127.0.0.0/8, for RBL servers."), arg: String::from("") },
OptionUsage{ opt: 298 , rept: 62 , flagdesc: String::from("/<domain>/"), desc:  String::from("Inhibit DNS-rebind protection on this domain."), arg: String::from("") },
OptionUsage{ opt: 278 , rept: 23 ,flagdesc: String::from(""), desc:  String::from("Always perform DNS queries to all servers."), arg: String::from("") },
OptionUsage{ opt: 281 , rept: 62 , flagdesc:  String::from("set:<tag>, OptionUsage{ optspec>"), desc:  String::from("Set tag if client includes matching option in request."), arg: String::from("") },
OptionUsage{ opt: 355 , rept: 62 , flagdesc:  String::from("set:<tag>, string>[*]"), desc:  String::from("Set tag if client provides given name."), arg: String::from("") },
OptionUsage{ opt: 284 , rept: (62  + 1 ), flagdesc: String::from("[=<ports>]"), desc:  String::from("Use alternative ports for DHCP."), arg: String::from("") },
OptionUsage{ opt: 287 , rept: 62 , flagdesc:  String::from("<name>, naptr>"), desc:  String::from("Specify NAPTR DNS record."), arg: String::from("") },
OptionUsage{ opt: 288 , rept: (62  + 1 ), flagdesc: String::from("<port>"), desc:  String::from("Specify lowest port available for DNS query transmission."), arg: String::from("") },
OptionUsage{ opt: 345 , rept: (62  + 1 ), flagdesc: String::from("<port>"), desc:  String::from("Specify highest port available for DNS query transmission."), arg: String::from("") },
OptionUsage{ opt: 289 , rept: 20 ,flagdesc: String::from(""), desc:  String::from("Use only fully qualified domain names for DHCP clients."), arg: String::from("") },
OptionUsage{ opt: 296 , rept: 62 , flagdesc: String::from("[=tag:<tag>]"), desc:  String::from("Generate hostnames based on MAC address for nameless clients."), arg: String::from("") },
OptionUsage{ opt: 295 , rept: 62 , flagdesc: String::from("[=<ipaddr>]..."), desc:  String::from("Use these DHCP relays full proxies."), arg: String::from("") },
OptionUsage{ opt: 323 , rept: 62 , flagdesc:  String::from("<local-addr>, server>[, iface>]"), desc:  String::from("Relay DHCP requests to a remote server"), arg: String::from("") },
OptionUsage{ opt: 290 , rept: 62 , flagdesc:  String::from("<alias>, target>[, ttl>]"), desc:  String::from("Specify alias name for LOCAL DNS name."), arg: String::from("") },
OptionUsage{ opt: 291 , rept: 62 , flagdesc:  String::from("<prompt>, timeout>]"), desc:  String::from("Prompt to send to PXE clients."), arg: String::from("") },
OptionUsage{ opt: 292 , rept: 62 , flagdesc: String::from("<service>"), desc:  String::from("Boot service for PXE menu."), arg: String::from("") },
OptionUsage{ opt: 293 , rept: 0 ,flagdesc: String::from(""), desc:  String::from("Check configuration syntax."), arg: String::from("") },
OptionUsage{ opt: 300 , rept: 62 , flagdesc: String::from("[=base64|text]"), desc:  String::from("Add requestor\'s MAC address to forwarded DNS queries."), arg: String::from("") },
OptionUsage{ opt: 325 , rept: (62  + 1 ), flagdesc:  String::from("<v4 pref>[, v6 pref>]"), desc:  String::from("Add specified IP subnet to forwarded DNS queries."), arg: String::from("") },
OptionUsage{ opt: 346 , rept: (62  + 1 ), flagdesc: String::from("<text>"), desc:  String::from("Add client identification to forwarded DNS queries."), arg: String::from("") },
OptionUsage{ opt: 301 , rept: 33 ,flagdesc: String::from(""), desc:  String::from("Proxy DNSSEC validation results from upstream nameservers."), arg: String::from("") },
OptionUsage{ opt: 302 , rept: 34 ,flagdesc: String::from(""), desc:  String::from("Attempt to allocate sequential IP addresses to DHCP clients."), arg: String::from("") },
OptionUsage{ opt: 358 , rept: 59 ,flagdesc: String::from(""), desc:  String::from("Ignore client identifier option sent by DHCP clients."), arg: String::from("") },
OptionUsage{ opt: 303 , rept: 35 ,flagdesc: String::from(""), desc:  String::from("Copy connection-track mark from queries to upstream connections."), arg: String::from("") },
OptionUsage{ opt: 304 , rept: 36 ,flagdesc: String::from(""), desc:  String::from("Allow DHCP clients to do their own DDNS updates."), arg: String::from("") },
OptionUsage{ opt: 306 , rept: 37 ,flagdesc: String::from(""), desc:  String::from("Send router-advertisements for interfaces doing DHCPv6"), arg: String::from("") },
OptionUsage{ opt: 307 , rept: (62  + 1 ), flagdesc:  String::from("<enterprise>, duid>"), desc:  String::from("Specify DUID_EN-type DHCPv6 server DUID"), arg: String::from("") },
OptionUsage{ opt: 308 , rept: 62 , flagdesc:  String::from("<name>, address>[, ttl>]"), desc:  String::from("Specify host (A/AAAA and PTR) records"), arg: String::from("") },
OptionUsage{ opt: 356 , rept: 62 , flagdesc:  String::from("<name>, flags>, tag>, value>"), desc:  String::from("Specify certification authority authorization record"), arg: String::from("") },
OptionUsage{ opt: 310 , rept: 62 , flagdesc:  String::from("<name>, RR-number>, data>]"), desc:  String::from("Specify arbitrary DNS resource record"), arg: String::from("") },
OptionUsage{ opt: 311 , rept: 39 ,flagdesc: String::from(""), desc:  String::from("Bind to interfaces in use - check for new interfaces"), arg: String::from("") },
OptionUsage{ opt: 314 , rept: (62  + 1 ), flagdesc:  String::from("<NS>, interface>"), desc:  String::from("Export local names to global DNS"), arg: String::from("") },
OptionUsage{ opt: 313 , rept: 62 , flagdesc:  String::from("<domain>, subnet>...]"), desc:  String::from("Domain to export to global DNS"), arg: String::from("") },
OptionUsage{ opt: 315 , rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Set TTL for authoritative replies"), arg: String::from("") },
OptionUsage{ opt: 316 , rept: (62  + 1 ), flagdesc:  String::from("<serial>["), desc:  String::from("Set authoritative zone information"), arg: String::from("") },
OptionUsage{ opt: 317 , rept: 62 , flagdesc:  String::from("<NS>[, NS>...]"), desc:  String::from("Secondary authoritative nameservers for forward domains"), arg: String::from("") },
OptionUsage{ opt: 318 , rept: 62 , flagdesc:  String::from("<ipaddr>[, ipaddr>...]"), desc:  String::from("Peers which are allowed to do zone transfer"), arg: String::from("") },
OptionUsage{ opt: 319 , rept: 62 , flagdesc: String::from("/<domain>[/<domain>...]/<ipset>..."), desc:  String::from("Specify ipsets to which matching domains should be added"), arg: String::from("") },
OptionUsage{ opt: 320 , rept: 62 , flagdesc:  String::from("<domain>, range>, prefix>]"), desc:  String::from("Specify a domain and address range for synthesised names"), arg: String::from("") },
OptionUsage{ opt: 329 , rept: 45 , flagdesc: String::from(""), desc:  String::from("Activate DNSSEC validation"), arg: String::from("") },
OptionUsage{ opt: 330 , rept: 62 , flagdesc:  String::from("<domain>, class>]"), desc:  String::from("Specify trust anchor key digest."), arg: String::from("") },
OptionUsage{ opt: 331 , rept: 47 ,flagdesc: String::from(""), desc:  String::from("Disable upstream checking for DNSSEC debugging."), arg: String::from("") },
OptionUsage{ opt: 334 , rept: 62 ,flagdesc: String::from(""), desc:  String::from("Ensure answers without DNSSEC are in unsigned zones."), arg: String::from("") },
OptionUsage{ opt: 336 , rept: 46 ,flagdesc: String::from(""), desc:  String::from("Don\'t check DNSSEC signature timestamps until first cache-reload"), arg: String::from("") },
OptionUsage{ opt: 343 , rept: (62  + 1 ), flagdesc: String::from("<path>"), desc:  String::from("Timestamp file to verify system clock for DNSSEC"), arg: String::from("") },
OptionUsage{ opt: 324 , rept: 62 , flagdesc:  String::from("<iface>, mtu:<value>|<interface>|off, prio>, intval>[, lifetime>]"), desc:  String::from("Set MTU, priority, resend-interval and router-lifetime"), arg: String::from("") },
OptionUsage{ opt: 326 , rept: 42 ,flagdesc: String::from(""), desc:  String::from("Do not log routine DHCP."), arg: String::from("") },
OptionUsage{ opt: 327 , rept: 43 ,flagdesc: String::from(""), desc:  String::from("Do not log routine DHCPv6."), arg: String::from("") },
OptionUsage{ opt: 328 , rept: 44 ,flagdesc: String::from(""), desc:  String::from("Do not log RA."), arg: String::from("") },
OptionUsage{ opt: 335 , rept: 49 ,flagdesc: String::from(""), desc:  String::from("Accept queries only from directly-connected networks."), arg: String::from("") },
OptionUsage{ opt: 337 , rept: 50 ,flagdesc: String::from(""), desc:  String::from("Detect and remove DNS forwarding loops."), arg: String::from("") },
OptionUsage{ opt: 338 , rept: 62 , flagdesc: String::from("<ipaddr>"), desc:  String::from("Ignore DNS responses containing ipaddr."), arg: String::from("") },
OptionUsage{ opt: 348 , rept: (62  + 1 ), flagdesc: String::from("<ttl>"), desc:  String::from("Set TTL in DNS responses with DHCP-derived addresses."), arg: String::from("") },
OptionUsage{ opt: 350 , rept: (62  + 1 ), flagdesc: String::from("<integer>"), desc:  String::from("Delay DHCP replies for at least number of seconds."), arg: String::from("") },
OptionUsage{ opt: 351 , rept: 57 ,flagdesc: String::from(""), desc:  String::from("Enables DHCPv4 Rapid Commit option."), arg: String::from("") },
OptionUsage{ opt: 352 , rept: (62  + 1 ), flagdesc: String::from("<path>"), desc:  String::from("Path to debug packet dump file"), arg: String::from("") },
OptionUsage{ opt: 353 , rept: (62  + 1 ), flagdesc: String::from("<hex>"), desc:  String::from("Mask which packets to dump"), arg: String::from("") },
OptionUsage{ opt: 360 , rept: 61 ,flagdesc: String::from(""), desc:  String::from("Call dhcp-script when lease expiry changes."), arg: String::from("") }];
/* We hide metacharacters in quoted strings by mapping them into the ASCII control
   character space. Note that the \0, \t \b \r \033 and \n characters are carefully placed in the
   following sequence so that they map to themselves: it is therefore possible to call
   unhide_metas repeatedly on string without breaking things.
   The transformation gets undone by opt_canonicalise, atoi_check and opt_string_alloc, and a
   couple of other places.
   Note that space is included here so that
   --dhcp-option=3, string
   has five characters, whilst
   --dhcp-option=3," string"
   has six.
*/
pub static META: String = String::from(b"\x00123456 \x08\t\n78\r90abcdefABCDE\x1bF:,.\x00");

 fn hide_meta(mut c: libc::c_char) -> libc::c_char {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[libc::c_char; 33]>() as
                   libc::c_ulong).wrapping_sub(1 as
                                                   libc::c_ulong) {
        if c == META[i as usize] {
            return i as libc::c_char
        }
        i = i.wrapping_add(1)
    }
    return c;
}
 fn unhide_meta(mut cr: libc::c_char) -> libc::c_char {
    let mut c: libc::c_uint = cr as libc::c_uint;
    if (c as libc::c_ulong) <
           (::std::mem::size_of::<[libc::c_char; 33]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_ulong)
       {
        cr = META[c as usize]
    }
    return cr;
}
 fn unhide_metas(cp: &mut String) {
    if !cp.is_empty() {
        let mut i = 0;
        for c in cp {
            cp[i] = unhide_meta(c);
            i += 1;
        }
    }
}

//  fn opt_malloc(mut size: usize) -> *mut libc::c_void {
//     let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
//     if mem_recover != 0 {
//         ret = whine_malloc(size);
//         if ret.is_null() { longjmp(mem_jmp.as_mut_ptr(), 1 as libc::c_int); }
//     } else { ret = safe_malloc(size) }
//     return ret;
// }
//  fn opt_string_alloc(mut cp: *const libc::c_char)
//  -> *mut libc::c_char {
//     let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
//     let mut len: usize = 0;
//     if !cp.is_null() &&
//            { len = strlen(cp); (len) != 0 } {
//         ret =
//             opt_malloc(len.wrapping_add(1)) as
//                 *mut libc::c_char;
//         memcpy(ret as *mut libc::c_void, cp as *const libc::c_void,
//                len.wrapping_add(1));
//         /* restore hidden metachars */
//         unhide_metas(ret);
//     }
//     return ret;
// }
/* find next comma, split string with zero and eliminate spaces.
   return start of string following comma */
 fn split_chr(mut s: &String, mut c: char)
 -> String {
    let mut res = s.split(",");
    let res2 = res.next().unwrap();
    String::from(res2)
}

//  fn split(mut s: *mut libc::c_char) -> *mut libc::c_char {
//     return split_chr(s, ',' as i32 as libc::c_char);
// }

 fn canonicalise_opt(s: &mut String)
 -> Option<String> {
    let mut ret: String::new();
    let mut nomem = 0;
    unhide_metas(s);
    canonicalise(s, nomem)
}

//  fn atoi_check(mut a: *mut libc::c_char,
//                                 mut res: *mut libc::c_int) -> libc::c_int {
//     let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
//     if a.is_null() { return 0 as libc::c_int }
//     unhide_metas(a);
//     p = a;
//     while *p != 0 {
//         if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '9' as i32
//            {
//             return 0 as libc::c_int
//         }
//         p = p.offset(1)
//     }
//     *res = atoi(a);
//     return 1 as libc::c_int;
// }

//

 pub fn add_txt(name: &String, txt: &String, stat_0: i32) {
     let mut r: TxtRecord = Default::default();
     let mut len: usize = strlen(txt);
     r.txt = String::from("");
     r.len = len.wrapping_add(1) as u16;
     // r.txt = len;
     // TODO: copy value
     //memcpy(r.txt.offset(1 as libc::c_int as isize), txt, len);
     r.stat = stat_0;
     r.name = name.clone();
     // r.next = (*dnsmasq_daemon).txt;
     daemon.txt = r;
     r.class = 3;
}

// pub fn do_usage() {
//     let mut buff: [libc::c_char; 100] = [0; 100];
//     let mut i: libc::c_int = 0;
//     let mut j: libc::c_int = 0;
//     let mut tab: [TabEntryA; 6] =
//         [{
//              let mut init =
//                  TabEntryA{handle: '$', val: 150 as libc::c_int,};
//              init
//          },
//          {
//              let mut init =
//                  TabEntryA{handle: '*', val: 4096 as libc::c_int,};
//              init
//          },
//          {
//              let mut init =
//                  TabEntryA{handle: '&', val: 1000 as libc::c_int,};
//              init
//          },
//          {
//              let mut init =
//                  TabEntryA{handle: '!', val: 150 as libc::c_int,};
//              init
//          },
//          {
//              let mut init =
//                  TabEntryA{handle: '#', val: 50 as libc::c_int,};
//              init
//          },
//          {
//              let mut init =
//                  TabEntryA{handle: '\u{0}', val: 0 as libc::c_int,};
//              init
//          }];
//     printf(b"Usage: dnsmasq [options]\n\n\x00" as *const u8 as
//                *const libc::c_char);
//     printf(b"Valid options are:\n\x00" as *const u8 as *const libc::c_char);
//     i = 0 as libc::c_int;
//     while usage[i as usize].opt != 0 as libc::c_int {
//         let mut desc: *mut libc::c_char = usage[i as usize].flagdesc;
//         let mut eq: *mut libc::c_char =
//             b"=\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
//         if desc.is_null() || *desc as libc::c_int == '[' as i32 {
//             eq =
//                 b"\x00" as *const u8 as *const libc::c_char as
//                     *mut libc::c_char
//         }
//         if desc.is_null() {
//             desc =
//                 b"\x00" as *const u8 as *const libc::c_char as
//                     *mut libc::c_char
//         }
//         j = 0 as libc::c_int;
//         while !opts[j as usize].name.is_null() {
//             if opts[j as usize].val == usage[i as usize].opt { break ; }
//             j += 1
//         }
//         if usage[i as usize].opt < 256 as libc::c_int {
//             sprintf(buff.as_mut_ptr(),
//                     b"-%c, \x00" as *const u8 as *const libc::c_char,
//                     usage[i as usize].opt);
//         } else {
//             sprintf(buff.as_mut_ptr(),
//                     b"    \x00" as *const u8 as *const libc::c_char);
//         }
//         sprintf(buff.as_mut_ptr().offset(4 as libc::c_int as isize),
//                 b"--%s%s%s\x00" as *const u8 as *const libc::c_char,
//                 opts[j as usize].name, eq, desc);
//         printf(b"%-55.55s\x00" as *const u8 as *const libc::c_char,
//                buff.as_mut_ptr());
//         if !usage[i as usize].arg.is_null() {
//             strcpy(buff.as_mut_ptr(), usage[i as usize].arg);
//             j = 0 as libc::c_int;
//             while tab[j as usize].handle != 0 {
//                 if tab[j as usize].handle as libc::c_int ==
//                        *usage[i as usize].arg as libc::c_int {
//                     sprintf(buff.as_mut_ptr(),
//                             b"%d\x00" as *const u8 as *const libc::c_char,
//                             tab[j as usize].val);
//                 }
//                 j += 1
//             }
//         }
//         printf(usage[i as usize].desc, buff.as_mut_ptr());
//         printf(b"\n\x00" as *const u8 as *const libc::c_char);
//         i += 1
//     };
// }

pub fn parse_mysockaddr(mut arg: &String, mut addr: &mut MySockAddr)
 -> Option<String> {



    if inet_pton(2, arg,
                 &mut addr.in_0.sin_addr as *mut InAddr as
                     *mut libc::c_void) > 0 {
        addr.sa.sa_family = 2 as SaFamily
    } else if inet_pton(10, arg,
                        &mut addr.in6.sin6_addr as *mut In6Addr as
                            *mut libc::c_void) > 0 {
        addr.sa.sa_family = 10 as SaFamily
    } else {
        return Some(String::from("bad address"));
    }
    return None;
}

pub  fn parse_server(arg: &String,
                     addr: &mut MySockAddr,
                     source_addr: &mut MySockAddr,
                     mut interface: &mut String,
                     mut flags: &mut i32) -> Option<String> {
    let mut source_port: libc::c_int = 0;
    let mut serv_port: libc::c_int = 53;
    let mut portno: String = String::new();
    let mut source: String = String::new();
    let mut interface_opt: String = String::new();
    let mut scope_index: libc::c_int = 0;
    let mut scope_id: String = String::new();
    if arg.is_empty() {
        flags = flags | 2;
        interface.clear();
        return None
    }
    source = split_chr(arg, '@');
    if !source.is_empty() &&
           {
               portno = split_chr(&source, '#');
               !portno.is_empty()
           } && atoi_check16(portno, &mut source_port) == 0 {
        return Some(String::from("bad port"));
    }
    portno = split_chr(arg, '#');
    if !portno.is_empty() && atoi_check16(portno, &mut serv_port) == 0 {
        return Some(String::from("bad port"));
    }
    scope_id = split_chr(arg, '%');
    if !source.is_empty() {
        interface_opt = split_chr(&source, '@');
        if !interface_opt.is_empty() {
            *interface = interface_opt.clone();
        }
    }
    if inet_pton(2, arg, &mut addr.in_0.sin_addr) > 0 {
        addr.in_0.sin_port = __bswap_16(serv_port as u16);
        source_addr.sa.sa_family = 2 as SaFamily;
        addr.sa.sa_family = source_addr.sa.sa_family;
        source_addr.in_0.sin_addr.s_addr = 0 as InAddrT;
        source_addr.in_0.sin_port =
            __bswap_16(daemon.query_port as u16);
        if !source.is_empty() {
            if !flags.is_empty() { *flags |= 16 }
            source_addr.in_0.sin_port =
                __bswap_16(source_port as u16);
            if !(inet_pton(2, source,
                           &mut source_addr.in_0.sin_addr as *mut InAddr
                               as *mut libc::c_void) > 0) {
                if !interface_opt.is_null() {
                    return Some(String::from("interface can only be specified once"))
                }
                source_addr.in_0.sin_addr.s_addr = 0;
                *interface = source.clone();
            }
        }
    } else if inet_pton(10, arg, &mut addr.in6.sin6_addr) > 0 {
        if !scope_id.is_null() &&
               {
                   scope_index = if_nametoindex(scope_id);
                   (scope_index) == 0
               } {
            return Some(string::from("bad interface name"))
        }
        addr.in6.sin6_port = __bswap_16(serv_port as u16);
        addr.in6.sin6_scope_id = scope_index as u32;
        source_addr.in6.sin6_addr = in6addr_any;
        source_addr.in6.sin6_port =
            __bswap_16(daemon.query_port as u16);
        source_addr.in6.sin6_scope_id = 0 as u32;
        source_addr.sa.sa_family = 10 as SaFamily;
        addr.sa.sa_family = source_addr.sa.sa_family;
        source_addr.in6.sin6_flowinfo = 0 as u32;
        addr.in6.sin6_flowinfo = source_addr.in6.sin6_flowinfo;
        if !source.is_null() {
            if !flags.is_null() { *flags |= 16 }
            source_addr.in6.sin6_port =
                __bswap_16(source_port as u16);
            if inet_pton(10, source,
                         &mut source_addr.in6.sin6_addr as *mut In6Addr as
                             *mut libc::c_void) == 0 {
                if !interface_opt.is_null() {
                    return Some(String::from("interface can only be specified once"))
                }
                source_addr.in6.sin6_addr = in6addr_any;
                *interface = source.clone();
            }
        }
    } else {
        return Some(String::from("bad address"))
    }
    return None
}

pub fn add_rev4(daemon: &mut DnsmasqDaemon,
                mut addr: InAddr,
                mut msize: libc::c_int) -> Option<Server> {
    let mut serv: Server = Default::default();
    let mut a: InAddrT = __bswap_32(addr.s_addr);
    let mut p: String = String::new();
    serv.domain = String::new();
    p = serv.domain.clone();
    let mut current_block_8: u64;
    match msize {
        32 => {
            p =
                p.offset(sprintf(p,
                                 b"%u.\x00" as *const u8 as
                                     *const libc::c_char,
                                 a & 0xff as libc::c_uint) as
                             isize);
            current_block_8 = 643824972125085941;
        }
        24 => { current_block_8 = 643824972125085941; }
        16 => { current_block_8 = 17981750616475279043; }
        8 => { current_block_8 = 12821047218658804999; }
        _ => {
            // free(serv.domain as *mut libc::c_void);
            // free(serv as *mut libc::c_void);
            return None
        }
    }
    match current_block_8 {
        643824972125085941 =>
        /* fall through */
        {
            p =
                p.offset(sprintf(p,
                                 b"%d.\x00" as *const u8 as
                                     *const libc::c_char,
                                 a >> 8 &
                                     0xff as libc::c_uint) as
                             isize);
            current_block_8 = 17981750616475279043;
        }
        _ => { }
    }
    match current_block_8 {
        17981750616475279043 =>
        /* fall through */
        {
            p =
                p.offset(sprintf(p,
                                 b"%d.\x00" as *const u8 as
                                     *const libc::c_char,
                                 a >> 16 &
                                     0xff as libc::c_uint) as
                             isize)
        }
        _ => { }
    }
    /* fall through */
    p =
        p.offset(sprintf(p, b"%d.\x00" as *const u8 as *const libc::c_char,
                         a >> 24 &
                             0xff as libc::c_uint) as
                     isize); /* strlen("32*<n.>ip6.arpa")+1 */
    p =
        p.offset(sprintf(p,
                         b"in-addr.arpa\x00" as *const u8 as
                             *const libc::c_char));
    serv.flags = 8;
    serv.next = daemon.servers;
    daemon.servers = serv.clone();
    return Some(serv);
}

 pub fn add_rev6(mut addr: In6Addr, mut msize: libc::c_int) -> Option<Server> {
    let mut serv: Server = Default::default();
    let mut p: String = String::new();
    let mut i: libc::c_int = 0;
    serv.domain = String::new();
    p = serv.domain.clone();
    i = msize - 1;
    while i >= 0 {
        let mut dig: libc::c_int = addr.__in6_u.__u6_addr16[i >> 3];
        p =
            p.offset(sprintf(p,
                             b"%.1x.\x00" as *const u8 as *const libc::c_char,
                             if i >> 2 & 1 != 0
                                {
                                 (dig) & 15
                             } else { (dig) >> 4 }));
        i -= 4
    }
    p =
        p.offset(sprintf(p,
                         b"ip6.arpa\x00" as *const u8 as *const libc::c_char)
                    );
    serv.flags = 8;
    // serv.next = daemon.servers;
    daemon.servers = serv.clone();
    return Some(serv);
}
 fn is_tag_prefix(mut arg: &String)
 -> bool {
    if !arg.is_empty() &&
           (strstr(arg, b"net:\x00" as *const u8 as *const libc::c_char) ==
                arg ||
                strstr(arg, b"tag:\x00" as *const u8 as *const libc::c_char)
                    == arg) {
        return true
    }
    return false
}


 fn set_prefix(mut arg: &mut String)
 -> Option<String> {
     if arg.find("set:") == 0 {
         let sl = arg.as_bytes()[4..];
         Some(String::from(sl))
     }
     None
}


 fn dhcp_netid_create(mut net: &String, mut next: &DhcpNetId)
 -> DhcpNetId {
    let mut tt: DhcpNetId = Default::default();
    tt.net = net.clone();
    tt.next = next;
    return tt;
}


//  fn dhcp_netid_free(mut nid: *mut DhcpNetId) {
//     while !nid.is_null() {
//         let mut tmp: *mut DhcpNetId = nid;
//         nid = nid.next;
//         free((*tmp).net as *mut libc::c_void);
//         free(tmp as *mut libc::c_void);
//     };
// }


/* Parse one or more tag:s before parameters.
 * Moves arg to the end of tags. */
fn dhcp_tags(mut arg: &mut String)
 -> DhcpNetId {
    let mut id: DhcpNetId = Default::default();
    while is_tag_prefix(arg) != false {
        let mut comma = arg.split(",");
        id = dhcp_netid_create(&String::from(arg.as_bytes()[4..]), &id);
        *arg = comma.next().unwrap().to_string();
    }
    return id;
}


// fn dhcp_netid_list_free(mut netid: &mut DhcpNetIdList) {
//     while !netid.is_null() {
//         let mut tmplist: mut DhcpNetIdList = netid;
//         netid = (*netid).next;
//     };
// }

//  fn dhcp_config_free(mut config: *mut DhcpConfig) {
//     if !config.is_null() {
//         let mut hwaddr: *mut HwaddrConfig = (*config).hwaddr;
//         while !hwaddr.is_null() {
//             let mut tmp: *mut HwaddrConfig = hwaddr;
//             hwaddr = (*hwaddr).next;
//             free(tmp as *mut libc::c_void);
//         }
//         dhcp_netid_list_free((*config).netid);
//         dhcp_netid_free((*config).filter);
//         if (*config).flags & 2 as libc::c_int as libc::c_uint != 0 {
//             free((*config).clid as *mut libc::c_void);
//         }
//         if (*config).flags & 4096 as libc::c_int as libc::c_uint != 0 {
//             let mut addr: *mut AddrList = 0 as *mut AddrList;
//             let mut tmp_0: *mut AddrList = 0 as *mut AddrList;
//             addr = (*config).addr6;
//             while !addr.is_null() {
//                 tmp_0 = addr.next;
//                 free(addr as *mut libc::c_void);
//                 addr = tmp_0
//             }
//         }
//         free(config as *mut libc::c_void);
//     };
// }

 fn dhcp_context_free(mut ctx: *mut DhcpContext) {
//     if !ctx.is_null() {
//         dhcp_netid_free((*ctx).filter);
//         free((*ctx).netid.net as *mut libc::c_void);
//         free((*ctx).template_interface as *mut libc::c_void);
//         free(ctx as *mut libc::c_void);
//     };
// }

//  fn dhcp_opt_free(mut opt: *mut DhcpOpt) {
//     if (*opt).flags & 256 as libc::c_int != 0 {
//         free((*opt).u.vendor_class as *mut libc::c_void);
//     }
//     dhcp_netid_free((*opt).netid);
//     free((*opt).val as *mut libc::c_void);
//     free(opt as *mut libc::c_void);
// }

/* This is too insanely large to keep in-line in the switch */
 fn parse_dhcp_opt(mut errstr: &mut String, mut arg: &mut String, mut flags: u32) -> u32 {
    let mut current_block: u64;
    let mut new: DhcpOpt = Default:: default();
    let mut lenchar: u8 = 0;
    let mut cp: String = String::new();
    let mut addrs: u32 = 0;
    let mut digs: u32 = 0;
    let mut is_addr: bool = false;
    let mut is_addr6: bool = false;
    let mut is_hex: bool = false;
    let mut is_dec: bool = false;
    let mut is_string: bool = false;
    let mut dots: u32 = 0;
    let mut comma: String = String::new();
    let mut opt_len: usize = 0;
    let mut is6: bool = false;
    let mut option_ok: bool = false;
    new.len = 0;
    new.flags = flags;
    new.netid = Default::default();
    new.val = Vec::new();
    new.opt = 0;

     for chunk in arg.split(",") {
        if chunk.find("option:") == 0 {
            // arg.offset(7)
            new.opt = lookup_dhcp_opt(2, &string_from_offset(&String::from(chunk), 7));
            if new.opt != -(1) {
                opt_len = lookup_dhcp_len(2, new.opt);
                /* option:<optname> must follow tag and vendor string. */
                if opt_len & 0x2000 == 0 || flags == 128 {
                    option_ok = 1
                }
            }
            break ;
        } else if chunk.find("option6") == 0 {
            cp = arg.offset(8);
            while cp != 0 {
                if (cp) < '0' as i32 ||
                       cp > '9' as i32 {
                    break ;
                }
                cp = cp.offset(1)
            }
            if cp == 0 {
                new.opt = atoi(arg.offset(8));
                opt_len = 0 as u16;
                option_ok = 1
            } else {
                new.opt = lookup_dhcp_opt(10, arg.offset(8));
                if new.opt != -(1) {
                    opt_len =
                        lookup_dhcp_len(10, new.opt);
                    if opt_len & 0x2000 == 0 ||
                           flags == 128 {
                        option_ok = 1
                    }
                }
            }
            /* option6:<opt>|<optname> must follow tag and vendor string. */
            is6 = 1;
            break ;
        } else {
            if strstr(arg, b"vendor:\x00" as *const u8 as *const libc::c_char)
                   == arg {
                new.u.vendor_class =
                    opt_string_alloc(arg.offset(7)) as
                        *mut libc::c_uchar;
                new.flags |= 256
            } else if strstr(arg,
                             b"encap:\x00" as *const u8 as
                                 *const libc::c_char) == arg {
                new.u.encap = atoi(arg.offset(6));
                new.flags |= 4
            } else if strstr(arg,
                             b"vi-encap:\x00" as *const u8 as
                                 *const libc::c_char) == arg {
                new.u.encap = atoi(arg.offset(9));
                new.flags |= 2048;
                if flags == 128 {
                    option_ok = 1;
                    break ;
                }
            } else {
                /* allow optional "net:" or "tag:" for consistency */
                let mut name: *const libc::c_char =
                    if is_tag_prefix(arg) != 0 {
                        arg.offset(4)
                    } else { set_prefix(arg) };
                new.netid = dhcp_netid_create(name, new.netid)
            }
            arg = comma
        }
    }
    if is6 != 0 {
        if new.flags & (256 | 4) != 0 {
            strcpy(errstr,
                   b"unsupported encapsulation for IPv6 option\x00" as
                       *const u8 as *const libc::c_char);
            current_block = 14151404249770905673;
        } else {
            if opt_len == 0 &&
                   new.flags & 2048 == 0 {
                opt_len =
                    lookup_dhcp_len(10, new.opt) as u16
            }
            current_block = 317151059986244064;
        }
    } else {
        if opt_len == 0 &&
               new.flags &
                   (256 | 4 |
                        2048) == 0 {
            opt_len = lookup_dhcp_len(2, new.opt) as u16
        }
        current_block = 317151059986244064;
    }
    match current_block {
        317151059986244064 =>
        /* option may be missing with rfc3925 match */
        {
            if option_ok == 0 {
                strcpy(errstr,
                       b"bad dhcp-option\x00" as *const u8 as
                           *const libc::c_char);
            } else {
                if !comma.is_null() {
                    /* characterise the value */
                    let mut c: libc::c_char = 0;
                    let mut found_dig: libc::c_int = 0;
                    let mut found_colon: libc::c_int = 0;
                    is_string = 1;
                    is_dec = is_string;
                    is_hex = is_dec;
                    is_addr6 = is_hex;
                    is_addr = is_addr6;
                    digs = 1;
                    addrs = digs;
                    dots = 0;
                    cp = comma;
                    loop  {
                        c = cp;
                        if !(c != 0) { break ; }
                        if c == ',' as i32 {
                            addrs += 1;
                            is_hex = 0;
                            is_dec = is_hex
                        } else if c == ':' as i32 {
                            digs += 1;
                            is_addr = 0;
                            is_dec = is_addr;
                            found_colon = 1
                        } else if c == '/' as i32 {
                            is_hex = 0;
                            is_dec = is_hex;
                            is_addr6 = is_dec;
                            if cp == comma {
                                /* leading / means a pathname */
                                is_addr = 0
                            }
                        } else if c == '.' as i32 {
                            is_hex = 0;
                            is_dec = is_hex;
                            dots += 1
                        } else if c == '-' as i32 {
                            is_addr6 = 0;
                            is_addr = is_addr6;
                            is_hex = is_addr
                        } else if c == ' ' as i32 {
                            is_hex = 0;
                            is_dec = is_hex
                        } else if !(c >= '0' as i32 &&
                                        c <= '9' as i32) {
                            is_addr = 0;
                            if cp.offset(1) as
                                   libc::c_int == 0 &&
                                   is_dec != 0 &&
                                   (c == 'b' as i32 ||
                                        c == 's' as i32 ||
                                        c == 'i' as i32) {
                                lenchar = c;
                                cp = 0 as libc::c_char
                            } else { is_dec = 0 }
                            if !(c >= 'A' as i32 &&
                                     c <= 'F' as i32 ||
                                     c >= 'a' as i32 &&
                                         c <= 'f' as i32 ||
                                     c == '*' as i32 &&
                                         flags & 128 != 0) {
                                is_hex = 0;
                                if c != '[' as i32 &&
                                       c != ']' as i32 {
                                    is_addr6 = 0
                                }
                            }
                        } else { found_dig = 1 }
                        cp = cp.offset(1)
                    }
                    if found_dig == 0 {
                        is_addr = 0;
                        is_dec = is_addr
                    }
                    if found_colon == 0 { is_addr6 = 0 }
                    /* NTP server option takes hex, addresses or FQDN */
                    if is6 != 0 && new.opt == 56 &&
                           is_hex == 0 {
                        opt_len =
                            (opt_len |
                                 if is_addr6 != 0 {
                                     0x8000
                                 } else { 0x4000 }) as u16
                    }
                    /* We know that some options take addresses */
                    if opt_len & 0x8000 != 0 {
                        is_hex = 0;
                        is_dec = is_hex;
                        is_string = is_dec;
                        if is6 == 0 &&
                               (is_addr == 0 || dots == 0) {
                            strcpy(errstr,
                                   b"bad IP address\x00" as *const u8 as
                                       *const libc::c_char);
                            current_block = 14151404249770905673;
                        } else if is6 != 0 && is_addr6 == 0 {
                            strcpy(errstr,
                                   b"bad IPv6 address\x00" as *const u8 as
                                       *const libc::c_char);
                            current_block = 14151404249770905673;
                        } else { current_block = 6407515180622463684; }
                    } else {
                        /* or names */
                        if opt_len &
                               (0x1000 | 0x4000
                                    | 0x800) != 0 {
                            is_hex = 0;
                            is_dec = is_hex;
                            is_addr = is_dec;
                            is_addr6 = is_addr
                        }
                        current_block = 6407515180622463684;
                    }
                    match current_block {
                        14151404249770905673 => { }
                        _ => {
                            if found_dig != 0 &&
                                   opt_len &
                                       0x200 != 0 &&
                                   strlen(comma) >
                                       0 as libc::c_ulong {
                                let mut val: libc::c_int = 0;
                                let mut fac: libc::c_int = 1;
                                let mut current_block_105: u64;
                                match *comma.offset(strlen(comma).wrapping_sub(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong)
                                                       ) as
                                          libc::c_int {
                                    119 | 87 => {
                                        fac *= 7;
                                        current_block_105 =
                                            13975322803090976125;
                                    }
                                    100 | 68 => {
                                        current_block_105 =
                                            13975322803090976125;
                                    }
                                    104 | 72 => {
                                        current_block_105 =
                                            16222678236269241473;
                                    }
                                    109 | 77 => {
                                        current_block_105 =
                                            15298318582559719273;
                                    }
                                    115 | 83 => {
                                        current_block_105 =
                                            10680264320049698114;
                                    }
                                    _ => {
                                        current_block_105 =
                                            7761909515536616543;
                                    }
                                }
                                match current_block_105 {
                                    13975322803090976125 =>
                                    /* fall through */
                                    {
                                        fac *= 24;
                                        current_block_105 =
                                            16222678236269241473;
                                    }
                                    _ => { }
                                }
                                match current_block_105 {
                                    16222678236269241473 =>
                                    /* fall through */
                                    {
                                        fac *= 60;
                                        current_block_105 =
                                            15298318582559719273;
                                    }
                                    _ => { }
                                }
                                match current_block_105 {
                                    15298318582559719273 =>
                                    /* fall through */
                                    {
                                        fac *= 60;
                                        current_block_105 =
                                            10680264320049698114;
                                    }
                                    _ => { }
                                }
                                match current_block_105 {
                                    10680264320049698114 =>
                                    /* fall through */
                                    {
                                        *comma.offset(strlen(comma).wrapping_sub(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong)
                                                         ) =
                                            0 as libc::c_char
                                    }
                                    _ => { }
                                }
                                new.len = 4;
                                new.val =
                                    opt_malloc(4 as usize) as
                                        *mut libc::c_uchar;
                                val = atoi(comma);
                                *(new.val as *mut libc::c_int) =
                                    __bswap_32((val * fac) as u32) as
                                        libc::c_int;
                                current_block = 15439134456549723682;
                            } else if is_hex != 0 && digs > 1 {
                                new.len = digs;
                                new.val =
                                    opt_malloc(new.len as usize) as
                                        *mut libc::c_uchar;
                                parse_hex(comma, new.val, digs,
                                          if flags & 128 != 0 {
                                              &mut new.u.wildcard_mask
                                          } else { 0 as *mut libc::c_uint },
                                          0 as *mut libc::c_int);
                                new.flags |= 512;
                                current_block = 15439134456549723682;
                            } else if is_dec != 0 {
                                let mut i: libc::c_int = 0;
                                let mut val_0: libc::c_int = atoi(comma);
                                /* assume numeric arg is 1 byte except for
	     options where it is known otherwise.
	     For vendor class option, we have to hack. */
                                if opt_len != 0
                                   {
                                    new.len = opt_len
                                } else if val_0 as libc::c_uint &
                                              0xffff0000 as libc::c_uint != 0
                                 {
                                    new.len = 4
                                } else if val_0 & 0xff00 != 0 {
                                    new.len = 2
                                } else { new.len = 1 }
                                if lenchar == 'b' as i32 {
                                    new.len = 1
                                } else if lenchar == 's' as i32
                                 {
                                    new.len = 2
                                } else if lenchar == 'i' as i32
                                 {
                                    new.len = 4
                                }
                                new.val =
                                    opt_malloc(new.len as usize) as
                                        *mut libc::c_uchar;
                                i = 0;
                                while i < new.len {
                                    *new.val.offset(i) =
                                        (val_0 >>
                                             (new.len - i -
                                                  1) *
                                                 8) as
                                            libc::c_uchar;
                                    i += 1
                                }
                                current_block = 15439134456549723682;
                            } else if is_addr != 0 && is6 == 0 {
                                let mut in_0: InAddr = InAddr {s_addr: 0,};
                                let mut op: *mut libc::c_uchar =
                                    0 as *mut libc::c_uchar;
                                let mut slash: *mut libc::c_char =
                                    0 as *mut libc::c_char;
                                /* max length of address/subnet descriptor is five bytes,
	     add one for the option 120 enc byte too */
                                op =
                                    opt_malloc((5 * addrs +
                                                    1) as
                                                   usize) as
                                        *mut libc::c_uchar; /* RFC 3361 "enc byte" */
                                new.val = op;
                                new.flags |= 1;
                                if new.flags &
                                       (4 | 256
                                            | 2048) == 0 &&
                                       new.opt == 120 {
                                    let fresh6 = op;
                                    op = op.offset(1);
                                    *fresh6 =
                                        1 as libc::c_uchar;
                                    new.flags &= !(1)
                                }
                                loop  {
                                    let fresh7 = addrs;
                                    addrs = addrs - 1;
                                    if !(fresh7 != 0) {
                                        current_block = 1131197912709891142;
                                        break ;
                                    }
                                    cp = comma;
                                    comma = split(cp);
                                    slash =
                                        split_chr(cp,
                                                  '/' as i32 as libc::c_char);
                                    if inet_pton(2, cp,
                                                 &mut in_0 as *mut InAddr as
                                                     *mut libc::c_void) == 0 {
                                        strcpy(errstr,
                                               b"bad IPv4 address\x00" as
                                                   *const u8 as
                                                   *const libc::c_char);
                                        current_block = 14151404249770905673;
                                        break ;
                                    } else if slash.is_null() {
                                        memcpy(op as *mut libc::c_void,
                                               &mut in_0 as *mut InAddr as
                                                   *const libc::c_void,
                                               4 as
                                                   libc::c_ulong);
                                        op =
                                            op.offset(4 as
                                                          isize)
                                    } else {
                                        let mut p: *mut libc::c_uchar =
                                            &mut in_0 as *mut InAddr as
                                                *mut libc::c_uchar;
                                        let mut netsize: libc::c_int =
                                            atoi(slash);
                                        let fresh8 = op;
                                        op = op.offset(1);
                                        *fresh8 = netsize as libc::c_uchar;
                                        if netsize > 0 {
                                            let fresh9 = p;
                                            p = p.offset(1);
                                            let fresh10 = op;
                                            op = op.offset(1);
                                            *fresh10 = *fresh9
                                        }
                                        if netsize > 8 {
                                            let fresh11 = p;
                                            p = p.offset(1);
                                            let fresh12 = op;
                                            op = op.offset(1);
                                            *fresh12 = *fresh11
                                        }
                                        if netsize > 16 {
                                            let fresh13 = p;
                                            p = p.offset(1);
                                            let fresh14 = op;
                                            op = op.offset(1);
                                            *fresh14 = *fresh13
                                        }
                                        if netsize > 24 {
                                            let fresh15 = p;
                                            p = p.offset(1);
                                            let fresh16 = op;
                                            op = op.offset(1);
                                            *fresh16 = *fresh15
                                        }
                                        new.flags &= !(1)
                                        /* cannot re-write descriptor format */
                                    }
                                }
                                match current_block {
                                    14151404249770905673 => { }
                                    _ => {
                                        new.len =
                                            op.wrapping_offset_from(new.val)
                                                as libc::c_long as
                                                libc::c_int;
                                        current_block = 15439134456549723682;
                                    }
                                }
                            } else if is_addr6 != 0 && is6 != 0 {
                                let mut op_0: *mut libc::c_uchar =
                                    0 as *mut libc::c_uchar;
                                op_0 =
                                    opt_malloc((16 * addrs) as
                                                   usize) as
                                        *mut libc::c_uchar;
                                new.val = op_0;
                                new.flags |= 8192;
                                loop  {
                                    let fresh17 = addrs;
                                    addrs = addrs - 1;
                                    if !(fresh17 != 0) {
                                        current_block = 6930811285952240378;
                                        break ;
                                    }
                                    cp = comma;
                                    comma = split(cp);
                                    /* check for [1234::7] */
                                    if cp == '[' as i32 {
                                        cp = cp.offset(1)
                                    }
                                    if strlen(cp) >
                                           1 as libc::c_ulong
                                           &&
                                           cp.offset(strlen(cp).wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong)
                                                         ) as
                                               libc::c_int == ']' as i32 {
                                        cp.offset(strlen(cp).wrapping_sub(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong)
                                                      ) =
                                            0 as libc::c_char
                                    }
                                    if inet_pton(10, cp,
                                                 op_0 as *mut libc::c_void) !=
                                           0 {
                                        op_0 =
                                            op_0.offset(16 as
                                                            isize)
                                    } else {
                                        strcpy(errstr,
                                               b"bad IPv6 address\x00" as
                                                   *const u8 as
                                                   *const libc::c_char);
                                        current_block = 14151404249770905673;
                                        break ;
                                    }
                                }
                                match current_block {
                                    14151404249770905673 => { }
                                    _ => {
                                        new.len =
                                            op_0.wrapping_offset_from(new.val)
                                                as libc::c_long as
                                                libc::c_int;
                                        current_block = 15439134456549723682;
                                    }
                                }
                            } else if is_string != 0 {
                                /* text arg */
                                if (new.opt == 119 ||
                                        new.opt == 120) &&
                                       is6 == 0 &&
                                       new.flags &
                                           (4 |
                                                256 |
                                                2048) == 0 {
                                    /* dns search, RFC 3397, or SIP, RFC 3361 */
                                    let mut q: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut r: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut tail: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut p_0: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut m: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut newp: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut newlen: usize = 0;
                                    let mut len: usize =
                                        0 as usize;
                                    let mut header_size: libc::c_int =
                                        if new.opt == 119 {
                                            0
                                        } else { 1 };
                                    arg = comma;
                                    comma = split(arg);
                                    loop  {
                                        if !(!arg.is_null() &&
                                                 *arg != 0) {
                                            current_block =
                                                5913497314667414582;
                                            break ;
                                        }
                                        let mut in_1: *mut libc::c_char =
                                            0 as *mut libc::c_char;
                                        let mut dom: *mut libc::c_char =
                                            0 as *mut libc::c_char;
                                        let mut domlen: usize =
                                            1 as usize;
                                        /* Allow "." as an empty domain */
                                        if strcmp(arg,
                                                  b".\x00" as *const u8 as
                                                      *const libc::c_char) !=
                                               0 {
                                            dom = canonicalise_opt(arg);
                                            if dom.is_null() {
                                                strcpy(errstr,
                                                       b"bad domain in dhcp-option\x00"
                                                           as *const u8 as
                                                           *const libc::c_char);
                                                current_block =
                                                    14151404249770905673;
                                                break ;
                                            } else {
                                                domlen =
                                                    strlen(dom).wrapping_add(2
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                                            }
                                        }
                                        newp =
                                            opt_malloc(len.wrapping_add(domlen).wrapping_add(header_size
                                                                                                 as
                                                                                                 libc::c_ulong))
                                                as *mut libc::c_uchar;
                                        if !m.is_null() {
                                            memcpy(newp as *mut libc::c_void,
                                                   m as *const libc::c_void,
                                                   (header_size as
                                                        libc::c_ulong).wrapping_add(len));
                                            free(m as *mut libc::c_void);
                                        }
                                        m = newp;
                                        p_0 = m.offset(header_size);
                                        q = p_0.offset(len);
                                        /* add string on the end in RFC1035 format */
                                        in_1 = dom;
                                        while !in_1.is_null() &&
                                                  *in_1 != 0 {
                                            let fresh18 = q;
                                            q = q.offset(1);
                                            let mut cp_0: *mut libc::c_uchar =
                                                fresh18;
                                            let mut j: libc::c_int = 0;
                                            j = 0;
                                            while *in_1 != 0 &&
                                                      *in_1 !=
                                                          '.' as i32 {
                                                let fresh19 = q;
                                                q = q.offset(1);
                                                *fresh19 =
                                                    *in_1 as libc::c_uchar;
                                                in_1 = in_1.offset(1);
                                                j += 1
                                            }
                                            cp_0 = j as libc::c_uchar;
                                            if *in_1 != 0 {
                                                in_1 = in_1.offset(1)
                                            }
                                        }
                                        let fresh20 = q;
                                        q = q.offset(1);
                                        *fresh20 =
                                            0 as libc::c_uchar;
                                        free(dom as *mut libc::c_void);
                                        /* Now tail-compress using earlier names. */
                                        newlen =
                                            q.wrapping_offset_from(p_0) as
                                                libc::c_long as usize;
                                        tail = p_0.offset(len);
                                        's_1219:
                                            while *tail != 0 {
                                                r = p_0;
                                                while (r.wrapping_offset_from(p_0)
                                                           as libc::c_long) <
                                                          len
                                                              as libc::c_long
                                                      {
                                                    if strcmp(r as
                                                                  *mut libc::c_char,
                                                              tail as
                                                                  *mut libc::c_char)
                                                           == 0
                                                       {
                                                        let mut t_s: u16 =
                                                            (r.wrapping_offset_from(p_0)
                                                                 as
                                                                 libc::c_long
                                                                 |
                                                                 0xc000 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_long)
                                                                as u16;
                                                        let mut t_cp:
                                                                *mut libc::c_uchar =
                                                            tail;
                                                        let fresh21 = t_cp;
                                                        t_cp = t_cp.offset(1);
                                                        *fresh21 =
                                                            (t_s as
                                                                 libc::c_int
                                                                 >>
                                                                 8 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uchar;
                                                        *t_cp =
                                                            t_s as
                                                                libc::c_uchar;
                                                        tail =
                                                            tail.offset(2 as
                                                                            libc::c_int
                                                                            as
                                                                            isize);
                                                        newlen =
                                                            tail.wrapping_offset_from(p_0)
                                                                as
                                                                libc::c_long
                                                                as usize;
                                                        break 's_1219 ;
                                                    } else {
                                                        r =
                                                            r.offset((*r as
                                                                          libc::c_int
                                                                          +
                                                                          1 as
                                                                              libc::c_int)
                                                                         as
                                                                         isize)
                                                    }
                                                }
                                                tail =
                                                    tail.offset((*tail as
                                                                     libc::c_int
                                                                     +
                                                                     1 as
                                                                         libc::c_int)
                                                                   )
                                            }
                                        len = newlen;
                                        arg = comma;
                                        comma = split(arg)
                                    }
                                    match current_block {
                                        14151404249770905673 => { }
                                        _ => {
                                            /* RFC 3361, enc byte is zero for names */
                                            if new.opt ==
                                                   120 &&
                                                   !m.is_null() {
                                                *m.offset(0 as
                                                              isize) =
                                                    0 as
                                                        libc::c_uchar
                                            }
                                            new.len =
                                                len +
                                                    header_size;
                                            new.val = m;
                                            current_block =
                                                15439134456549723682;
                                        }
                                    }
                                } else if !comma.is_null() &&
                                              opt_len &
                                                  0x800 != 0 {
                                    /* length fields are two bytes so need 16 bits for each string */
                                    let mut i_0: libc::c_int = 0;
                                    let mut commas: libc::c_int =
                                        1;
                                    let mut p_1: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut newp_0: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    i_0 = 0;
                                    while *comma.offset(i_0) != 0 {
                                        if *comma.offset(i_0) as
                                               libc::c_int == ',' as i32 {
                                            commas += 1
                                        }
                                        i_0 += 1
                                    }
                                    newp_0 =
                                        opt_malloc(strlen(comma).wrapping_add((2
                                                                                   as
                                                                                   libc::c_int
                                                                                   *
                                                                                   commas)
                                                                                  as
                                                                                  libc::c_ulong))
                                            as *mut libc::c_uchar;
                                    p_1 = newp_0;
                                    arg = comma;
                                    comma = split(arg);
                                    while !arg.is_null() &&
                                              *arg != 0 {
                                        let mut len_0: u16 =
                                            strlen(arg) as u16;
                                        unhide_metas(arg);
                                        let mut t_s_0: u16 = len_0;
                                        let mut t_cp_0: *mut libc::c_uchar =
                                            p_1;
                                        let fresh22 = t_cp_0;
                                        t_cp_0 = t_cp_0.offset(1);
                                        *fresh22 =
                                            (t_s_0 >>
                                                 8) as
                                                libc::c_uchar;
                                        *t_cp_0 = t_s_0 as libc::c_uchar;
                                        p_1 =
                                            p_1.offset(2 as
                                                           isize);
                                        memcpy(p_1 as *mut libc::c_void,
                                               arg as *const libc::c_void,
                                               len_0 as libc::c_ulong);
                                        p_1 =
                                            p_1.offset(len_0 as
                                                           isize);
                                        arg = comma;
                                        comma = split(arg)
                                    }
                                    new.val = newp_0;
                                    new.len =
                                        p_1.wrapping_offset_from(newp_0) as
                                            libc::c_long;
                                    current_block = 15439134456549723682;
                                } else if !comma.is_null() &&
                                              opt_len &
                                                  0x4000 != 0 {
                                    let mut p_2: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut q_0: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut newp_1: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut end: *mut libc::c_uchar =
                                        0 as *mut libc::c_uchar;
                                    let mut len_1: libc::c_int =
                                        0;
                                    let mut header_size_0: libc::c_int =
                                        if is6 != 0 &&
                                               new.opt == 56
                                           {
                                            4
                                        } else { 0 };
                                    arg = comma;
                                    comma = split(arg);
                                    loop  {
                                        if !(!arg.is_null() &&
                                                 *arg != 0) {
                                            current_block =
                                                7499465236084769340;
                                            break ;
                                        }
                                        let mut dom_0: *mut libc::c_char =
                                            canonicalise_opt(arg);
                                        if dom_0.is_null() {
                                            strcpy(errstr,
                                                   b"bad domain in dhcp-option\x00"
                                                       as *const u8 as
                                                       *const libc::c_char);
                                            current_block =
                                                14151404249770905673;
                                            break ;
                                        } else {
                                            newp_1 =
                                                opt_malloc(((len_1 +
                                                                 header_size_0)
                                                                as
                                                                libc::c_ulong).wrapping_add(strlen(dom_0)).wrapping_add(2
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_ulong))
                                                    as *mut libc::c_uchar;
                                            if !p_2.is_null() {
                                                memcpy(newp_1 as
                                                           *mut libc::c_void,
                                                       p_2 as
                                                           *const libc::c_void,
                                                       len_1 as
                                                           libc::c_ulong);
                                                free(p_2 as
                                                         *mut libc::c_void);
                                            }
                                            p_2 = newp_1;
                                            q_0 = p_2.offset(len_1);
                                            end =
                                                do_rfc1035_name(q_0.offset(header_size_0
                                                                               as
                                                                               isize),
                                                                dom_0,
                                                                0 as
                                                                    *mut libc::c_char);
                                            let fresh23 = end;
                                            end = end.offset(1);
                                            *fresh23 =
                                                0 as
                                                    libc::c_uchar;
                                            if is6 != 0 &&
                                                   new.opt ==
                                                       56 {
                                                let mut t_s_1: u16 =
                                                    3 as u16;
                                                let mut t_cp_1:
                                                        *mut libc::c_uchar =
                                                    q_0;
                                                let fresh24 = t_cp_1;
                                                t_cp_1 = t_cp_1.offset(1);
                                                *fresh24 =
                                                    (t_s_1 >>
                                                         8) as
                                                        libc::c_uchar;
                                                *t_cp_1 =
                                                    t_s_1 as libc::c_uchar;
                                                q_0 =
                                                    q_0.offset(2 as
                                                                   libc::c_int
                                                                  );
                                                let mut t_s_2: u16 =
                                                    (end.wrapping_offset_from(q_0)
                                                         as libc::c_long -
                                                         2 as
                                                             libc::c_long) as
                                                        u16;
                                                let mut t_cp_2:
                                                        *mut libc::c_uchar =
                                                    q_0;
                                                let fresh25 = t_cp_2;
                                                t_cp_2 = t_cp_2.offset(1);
                                                *fresh25 =
                                                    (t_s_2 >>
                                                         8) as
                                                        libc::c_uchar;
                                                *t_cp_2 =
                                                    t_s_2 as libc::c_uchar;
                                                q_0 =
                                                    q_0.offset(2 as
                                                                   libc::c_int
                                                                  )
                                            }
                                            len_1 =
                                                end.wrapping_offset_from(p_2)
                                                    as libc::c_long as
                                                    libc::c_int;
                                            free(dom_0 as *mut libc::c_void);
                                            arg = comma;
                                            comma = split(arg)
                                        }
                                    }
                                    match current_block {
                                        14151404249770905673 => { }
                                        _ => {
                                            new.val = p_2;
                                            new.len = len_1;
                                            current_block =
                                                15439134456549723682;
                                        }
                                    }
                                } else {
                                    new.len = strlen(comma);
                                    /* keep terminating zero on string */
                                    new.val =
                                        opt_string_alloc(comma) as
                                            *mut libc::c_uchar;
                                    new.flags |= 2;
                                    current_block = 15439134456549723682;
                                }
                            } else { current_block = 15439134456549723682; }
                        }
                    }
                } else { current_block = 15439134456549723682; }
                match current_block {
                    14151404249770905673 => { }
                    _ => {
                        if is6 == 0 &&
                               (new.len > 255 ||
                                    new.len > 253 &&
                                        new.flags &
                                            (256 |
                                                 4) != 0 ||
                                    new.len > 250 &&
                                        new.flags & 2048 !=
                                            0) {
                            strcpy(errstr,
                                   b"dhcp-option too long\x00" as *const u8 as
                                       *const libc::c_char);
                        } else {
                            if flags == 128 {
                                if new.flags &
                                       (4 | 256)
                                       != 0 || new.netid.is_null() ||
                                       !(*new.netid).next.is_null() {
                                    strcpy(errstr,
                                           b"illegal dhcp-match\x00" as
                                               *const u8 as
                                               *const libc::c_char);
                                    current_block = 14151404249770905673;
                                } else {
                                    if is6 != 0 {
                                        new.next =
                                            daemon.dhcp_match6;
                                        daemon.dhcp_match6 = new
                                    } else {
                                        new.next =
                                            daemon.dhcp_match;
                                        daemon.dhcp_match = new
                                    }
                                    current_block = 7621687701095090519;
                                }
                            } else {
                                if is6 != 0 {
                                    new.next =
                                        daemon.dhcp_opts6;
                                    daemon.dhcp_opts6 = new
                                } else {
                                    new.next = daemon.dhcp_opts;
                                    daemon.dhcp_opts = new
                                }
                                current_block = 7621687701095090519;
                            }
                            match current_block {
                                14151404249770905673 => { }
                                _ => { return 1 }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    dhcp_opt_free(new);
    return 0;
}
#[no_mangle]
pub  fn set_option_bool(mut opt: libc::c_uint) {
    daemon.options[(opt as
                                   libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong))
                                  as usize] |=
        (1 as libc::c_uint) <<
            (opt as
                 libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                  as
                                                  libc::c_ulong).wrapping_mul(8
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_ulong));
}
#[no_mangle]
pub  fn reset_option_bool(mut opt: libc::c_uint) {
    daemon.options[(opt as
                                   libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong))
                                  as usize] &=
        !((1 as libc::c_uint) <<
              (opt as
                   libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                    as
                                                    libc::c_ulong).wrapping_mul(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)));
}
 fn server_list_free(mut list: *mut Server) {
    while !list.is_null() {
        let mut tmp: *mut Server = list;
        list = (*list).next;
        free(tmp as *mut libc::c_void);
    };
}
 fn one_opt(mut option: libc::c_int,
                             mut arg: *mut libc::c_char,
                             mut errstr: *mut libc::c_char,
                             mut gen_err: *mut libc::c_char,
                             mut command_line: libc::c_int,
                             mut servers_only: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut comma: *mut libc::c_char = 0 as *mut libc::c_char;
    if option == '?' as i32 {
        strcpy(errstr, gen_err);
        return 0
    }
    i = 0;
    while usage[i as usize].opt != 0 {
        if usage[i as usize].opt == option {
            let mut rept: libc::c_int = usage[i as usize].rept;
            if command_line != 0 {
                /* command line */
                if rept == 62 + 2 {
                    strcpy(errstr,
                           b"illegal repeated flag\x00" as *const u8 as
                               *const libc::c_char);
                    return 0
                }
                if rept == 62 + 1 {
                    usage[i as usize].rept =
                        (62 + 2) as libc::c_uint
                }
            } else {
                /* allow file to override command line */
                if rept == 62 + 3 {
                    strcpy(errstr,
                           b"illegal repeated keyword\x00" as *const u8 as
                               *const libc::c_char);
                    return 0
                }
                if rept == 62 + 2 ||
                       rept == 62 + 1 {
                    usage[i as usize].rept =
                        (62 + 3) as libc::c_uint
                }
            }
            if rept != 62 &&
                   rept != 62 + 1 &&
                   rept != 62 + 2 {
                set_option_bool(rept as libc::c_uint);
                return 1
            }
            break ;
        } else { i += 1 }
    }
    match option {
        67 => {
            /* --conf-file */
            let mut file: *mut libc::c_char = opt_string_alloc(arg);
            if !file.is_null() {
                one_file(file, 0);
                free(file as *mut libc::c_void);
            }
            current_block = 7879481898411272068;
        }
        55 => {
            /* --conf-dir */
            let mut dir_stream: *mut DIR = 0 as *mut DIR;
            let mut ent: *mut dirent = 0 as *mut dirent;
            let mut directory: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut ignore_suffix: *mut list = 0 as *mut list;
            let mut match_suffix: *mut list = 0 as *mut list;
            let mut files: *mut list = 0 as *mut list;
            let mut li: *mut list = 0 as *mut list;
            comma = split(arg);
            directory = opt_string_alloc(arg);
            if directory.is_null() {
                current_block = 7879481898411272068;
            } else {
                arg = comma;
                while !arg.is_null() {
                    comma = split(arg);
                    if strlen(arg) != 0 as libc::c_ulong {
                        li =
                            opt_malloc(::std::mem::size_of::<list>() as
                                           libc::c_ulong) as *mut list;
                        if *arg == '*' as i32 {
                            /* "*" with no suffix is a no-op */
                            if *arg.offset(1) as
                                   libc::c_int == 0 {
                                free(li as *mut libc::c_void);
                            } else {
                                (*li).next = match_suffix;
                                match_suffix = li;
                                /* Have to copy: buffer is overwritten */
                                (*li).name =
                                    opt_string_alloc(arg.offset(1 as
                                                                    libc::c_int
                                                                   ))
                            }
                        } else {
                            (*li).next = ignore_suffix;
                            ignore_suffix = li;
                            /* Have to copy: buffer is overwritten */
                            (*li).name = opt_string_alloc(arg)
                        }
                    }
                    arg = comma
                }
                dir_stream = opendir(directory);
                if dir_stream.is_null() {
                    die(b"cannot access directory %s: %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        directory, 3);
                }
                loop  {
                    ent = readdir(dir_stream);
                    if ent.is_null() { break ; }
                    let mut len: usize = strlen((*ent).d_name.as_mut_ptr());
                    let mut buf: stat =
                        stat{st_dev: 0,
                             st_ino: 0,
                             st_nlink: 0,
                             st_mode: 0,
                             st_uid: 0,
                             st_gid: 0,
                             __pad0: 0,
                             st_rdev: 0,
                             st_size: 0,
                             st_blksize: 0,
                             st_blocks: 0,
                             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
                             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
                             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
                             __glibc_reserved: [0; 3],};
                    /* ignore emacs backups and dotfiles */
                    if len == 0 as libc::c_ulong ||
                           (*ent).d_name[len.wrapping_sub(1 as
                                                              libc::c_ulong)
                                             as usize] ==
                               '~' as i32 ||
                           (*ent).d_name[0 as usize] as
                               libc::c_int == '#' as i32 &&
                               (*ent).d_name[len.wrapping_sub(1
                                                                  as
                                                                  libc::c_ulong)
                                                 as usize] ==
                                   '#' as i32 ||
                           (*ent).d_name[0 as usize] as
                               libc::c_int == '.' as i32 {
                        continue ;
                    }
                    if !match_suffix.is_null() {
                        li = match_suffix;
                        while !li.is_null() {
                            /* check for required suffices */
                            let mut ls: usize = strlen((*li).name);
                            if len > ls &&
                                   strcmp((*li).name,
                                          &mut *(*ent).d_name.as_mut_ptr().offset(len.wrapping_sub(ls)
                                                                                      as
                                                                                      isize))
                                       == 0 {
                                break ;
                            }
                            li = (*li).next
                        }
                        if li.is_null() { continue ; }
                    }
                    li = ignore_suffix;
                    while !li.is_null() {
                        /* check for proscribed suffices */
                        let mut ls_0: usize = strlen((*li).name);
                        if len > ls_0 &&
                               strcmp((*li).name,
                                      &mut *(*ent).d_name.as_mut_ptr().offset(len.wrapping_sub(ls_0)
                                                                                  as
                                                                                  isize))
                                   == 0 {
                            break ;
                        }
                        li = (*li).next
                    }
                    if !li.is_null() { continue ; }
                    path =
                        opt_malloc(strlen(directory).wrapping_add(len).wrapping_add(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong))
                            as *mut libc::c_char;
                    strcpy(path, directory);
                    strcat(path,
                           b"/\x00" as *const u8 as *const libc::c_char);
                    strcat(path, (*ent).d_name.as_mut_ptr());
                    /* files must be readable */
                    if stat(path, &mut buf) == -(1) {
                        die(b"cannot access %s: %s\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            path, 3);
                    }
                    /* only reg files allowed. */
                    if buf.st_mode & 0o170000 as libc::c_uint
                           == 0o100000 as libc::c_uint {
                        /* sort files into order. */
                        let mut up: *mut *mut list = 0 as *mut *mut list;
                        let mut new: *mut list =
                            opt_malloc(::std::mem::size_of::<list>() as
                                           libc::c_ulong) as *mut list;
                        new.name = path;
                        up = &mut files;
                        li = files;
                        while !li.is_null() {
                            if strcmp((*li).name, path) >= 0 {
                                break ;
                            }
                            up = &mut (*li).next;
                            li = (*li).next
                        }
                        new.next = li;
                        *up = new
                    }
                }
                li = files;
                while !li.is_null() {
                    one_file((*li).name, 0);
                    li = (*li).next
                }
                closedir(dir_stream);
                free(directory as *mut libc::c_void);
                while !ignore_suffix.is_null() {
                    li = (*ignore_suffix).next;
                    free((*ignore_suffix).name as *mut libc::c_void);
                    free(ignore_suffix as *mut libc::c_void);
                    ignore_suffix = li
                }
                while !match_suffix.is_null() {
                    li = (*match_suffix).next;
                    free((*match_suffix).name as *mut libc::c_void);
                    free(match_suffix as *mut libc::c_void);
                    match_suffix = li
                }
                while !files.is_null() {
                    li = (*files).next;
                    free((*files).name as *mut libc::c_void);
                    free(files as *mut libc::c_void);
                    files = li
                }
                current_block = 7879481898411272068;
            }
        }
        325 => {
            /* --add-subnet */
            set_option_bool(41 as libc::c_uint);
            if !arg.is_null() {
                let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
                comma = split(arg);
                let mut new_0: *mut Mysubnet =
                    opt_malloc(::std::mem::size_of::<Mysubnet>() as
                                   libc::c_ulong) as *mut Mysubnet;
                end = split_chr(arg, '/' as i32 as libc::c_char);
                if !end.is_null() {
                    /* has subnet+len */
                    err = parse_mysockaddr(arg, &mut (*new_0).addr);
                    if !err.is_null() {
                        strcpy(errstr, err);
                        free(new_0 as *mut libc::c_void);
                        return 0
                    }
                    if atoi_check(end, &mut (*new_0).mask) == 0 {
                        strcpy(errstr, gen_err);
                        free(new_0 as *mut libc::c_void);
                        return 0
                    }
                    (*new_0).addr_used = 1
                } else if atoi_check(arg, &mut (*new_0).mask) == 0 {
                    strcpy(errstr, gen_err);
                    free(new_0 as *mut libc::c_void);
                    return 0
                }
                daemon.add_subnet4 = new_0;
                if !comma.is_null() {
                    new_0 =
                        opt_malloc(::std::mem::size_of::<Mysubnet>() as
                                       libc::c_ulong) as *mut Mysubnet;
                    end = split_chr(comma, '/' as i32 as libc::c_char);
                    if !end.is_null() {
                        /* has subnet+len */
                        err = parse_mysockaddr(comma, &mut (*new_0).addr);
                        if !err.is_null() {
                            strcpy(errstr, err);
                            free(new_0 as *mut libc::c_void);
                            return 0
                        }
                        if atoi_check(end, &mut (*new_0).mask) == 0 {
                            strcpy(errstr, gen_err);
                            free(new_0 as *mut libc::c_void);
                            return 0
                        }
                        (*new_0).addr_used = 1
                    } else if atoi_check(comma, &mut (*new_0).mask) == 0 {
                        strcpy(errstr, gen_err);
                        free(new_0 as *mut libc::c_void);
                        return 0
                    }
                    daemon.add_subnet6 = new_0
                }
            }
            current_block = 7879481898411272068;
        }
        49 => {
            /* --enable-dbus */
            set_option_bool(19 as libc::c_uint);
            if !arg.is_null() {
                daemon.dbus_name = opt_string_alloc(arg)
            } else {
                daemon.dbus_name =
                    b"uk.org.thekelleys.dnsmasq\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char
            }
            current_block = 7879481898411272068;
        }
        354 => {
            /* --enable-ubus */
            set_option_bool(58 as libc::c_uint);
            if !arg.is_null() {
                daemon.ubus_name = opt_string_alloc(arg)
            } else {
                daemon.ubus_name =
                    b"dnsmasq\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            }
            current_block = 7879481898411272068;
        }
        56 => {
            /* --log-facility */
            /* may be a filename */
            if !strchr(arg, '/' as i32).is_null() ||
                   strcmp(arg, b"-\x00" as *const u8 as *const libc::c_char)
                       == 0 {
                daemon.log_file = opt_string_alloc(arg)
            } else {
                i = 0;
                while !FACILITYNAMES[i as usize].c_name.is_null() {
                    if hostname_isequal(FACILITYNAMES[i as usize].c_name, arg)
                           != 0 {
                        break ;
                    }
                    i += 1
                }
                if !FACILITYNAMES[i as usize].c_name.is_null() {
                    daemon.log_fac =
                        FACILITYNAMES[i as usize].c_val
                } else {
                    strcpy(errstr,
                           b"bad log facility\x00" as *const u8 as
                               *const libc::c_char);
                    return 0
                }
            }
            current_block = 7879481898411272068;
        }
        120 => {
            /* --pid-file */
            daemon.runfile = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        114 => {
            /* --resolv-file */
            let mut name: *mut libc::c_char = opt_string_alloc(arg);
            let mut new_1: *mut Resolvc = 0 as *mut Resolvc;
            let mut list_0: *mut Resolvc = daemon.resolv_files;
            if !list_0.is_null() && (*list_0).is_default != 0 {
                /* replace default resolv file - possibly with nothing */
                if !name.is_null() {
                    (*list_0).is_default = 0;
                    (*list_0).name = name
                } else { list_0 = 0 as *mut Resolvc }
            } else if !name.is_null() {
                new_1 =
                    opt_malloc(::std::mem::size_of::<Resolvc>() as
                                   libc::c_ulong) as *mut Resolvc;
                (*new_1).next = list_0;
                (*new_1).name = name;
                (*new_1).is_default = 0;
                (*new_1).mtime = 0 as time_t;
                (*new_1).logged = 0;
                list_0 = new_1
            }
            daemon.resolv_files = list_0;
            current_block = 7879481898411272068;
        }
        333 => {
            daemon.servers_file = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        109 => {
            /* --mx-host */
            let mut pref: libc::c_int = 1; /* may be NULL */
            let mut new_2: *mut MxSrvRecord = 0 as *mut MxSrvRecord;
            let mut name_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
            comma = split(arg);
            if !comma.is_null() {
                let mut prefstr: *mut libc::c_char = 0 as *mut libc::c_char;
                prefstr = split(comma);
                if !prefstr.is_null() && atoi_check16(prefstr, &mut pref) == 0
                   {
                    strcpy(errstr,
                           b"bad MX preference\x00" as *const u8 as
                               *const libc::c_char);
                    return 0
                }
            }
            name_0 = canonicalise_opt(arg);
            if name_0.is_null() ||
                   !comma.is_null() &&
                       { target = canonicalise_opt(comma); target.is_null() }
               {
                strcpy(errstr,
                       b"bad MX name\x00" as *const u8 as
                           *const libc::c_char);
                return 0
            }
            new_2 =
                opt_malloc(::std::mem::size_of::<MxSrvRecord>() as
                               libc::c_ulong) as *mut MxSrvRecord;
            (*new_2).next = daemon.mxnames;
            daemon.mxnames = new_2;
            (*new_2).issrv = 0;
            (*new_2).name = name_0;
            (*new_2).target = target;
            (*new_2).weight = pref;
            current_block = 7879481898411272068;
        }
        116 => {
            /*  --mx-target */
            daemon.mxtarget = canonicalise_opt(arg);
            if daemon.mxtarget.is_null() {
                strcpy(errstr,
                       b"bad MX target\x00" as *const u8 as
                           *const libc::c_char);
                return 0
            }
            current_block = 7879481898411272068;
        }
        352 => {
            /* --dumpfile */
            daemon.dump_file = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        353 => {
            /* --dumpmask */
            daemon.dump_mask =
                strtol(arg, 0 as *mut *mut libc::c_char, 0) as
                    libc::c_int;
            current_block = 7879481898411272068;
        }
        108 => {
            /* --dhcp-leasefile */
            daemon.lease_file = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        54 | 305 => {
            /* --dhcp-script */
            /* --dhcp-luascript */
            if option == 305 {
                strcpy(errstr,
                       b"recompile with HAVE_LUASCRIPT defined to enable Lua scripts\x00"
                           as *const u8 as *const libc::c_char);
                return 0
            } else {
                daemon.lease_change_command = opt_string_alloc(arg)
            }
            current_block = 7879481898411272068;
        }
        273 => {
            /* --dhcp-hostsfile */
            current_block = 12010070245366740438;
        }
        280 => { current_block = 12010070245366740438; }
        340 => { current_block = 2812646229686797995; }
        341 => { current_block = 10566976656908717602; }
        342 => { current_block = 2602045500541335152; }
        72 => { current_block = 4533671380017093834; }
        314 => {
            /* --auth-server */
            comma = split(arg);
            daemon.authserver = opt_string_alloc(arg);
            loop  {
                arg = comma;
                if arg.is_null() { break ; }
                let mut new_4: *mut Iname =
                    opt_malloc(::std::mem::size_of::<Iname>() as
                                   libc::c_ulong) as *mut Iname;
                comma = split(arg);
                (*new_4).name = 0 as *mut libc::c_char;
                unhide_metas(arg);
                if inet_pton(2, arg,
                             &mut (*new_4).addr.in_0.sin_addr as *mut InAddr
                                 as *mut libc::c_void) > 0 {
                    (*new_4).addr.sa.sa_family =
                        2 as SaFamily
                } else if inet_pton(10, arg,
                                    &mut (*new_4).addr.in6.sin6_addr as
                                        *mut In6Addr as *mut libc::c_void) >
                              0 {
                    (*new_4).addr.sa.sa_family =
                        10 as SaFamily
                } else {
                    let mut fam: *mut libc::c_char =
                        split_chr(arg, '/' as i32 as libc::c_char);
                    (*new_4).name = opt_string_alloc(arg);
                    (*new_4).addr.sa.sa_family =
                        0 as SaFamily;
                    if !fam.is_null() {
                        if strcmp(fam,
                                  b"4\x00" as *const u8 as
                                      *const libc::c_char) == 0
                           {
                            (*new_4).addr.sa.sa_family =
                                2 as SaFamily
                        } else if strcmp(fam,
                                         b"6\x00" as *const u8 as
                                             *const libc::c_char) ==
                                      0 {
                            (*new_4).addr.sa.sa_family =
                                10 as SaFamily
                        } else {
                            free((*new_4).name as *mut libc::c_void);
                            strcpy(errstr, gen_err);
                            free(new_4 as *mut libc::c_void);
                            return 0
                        }
                    }
                }
                (*new_4).next = daemon.authinterface;
                daemon.authinterface = new_4
            }
            current_block = 7879481898411272068;
        }
        317 => {
            /* --auth-sec-servers */
            let mut new_5: *mut NameList = 0 as *mut NameList;
            loop  {
                comma = split(arg);
                new_5 =
                    opt_malloc(::std::mem::size_of::<NameList>() as
                                   libc::c_ulong) as *mut NameList;
                (*new_5).name = opt_string_alloc(arg);
                (*new_5).next = daemon.secondary_forward_server;
                daemon.secondary_forward_server = new_5;
                arg = comma;
                if arg.is_null() { break ; }
            }
            current_block = 7879481898411272068;
        }
        313 => {
            /* --auth-zone */
            let mut new_6: *mut AuthZone = 0 as *mut AuthZone;
            comma = split(arg);
            new_6 =
                opt_malloc(::std::mem::size_of::<AuthZone>() as
                               libc::c_ulong) as *mut AuthZone;
            (*new_6).domain = opt_string_alloc(arg);
            (*new_6).subnet = 0 as *mut AddrList;
            (*new_6).exclude = 0 as *mut AddrList;
            (*new_6).interface_names = 0 as *mut AuthNameList;
            (*new_6).next = daemon.auth_zones;
            daemon.auth_zones = new_6;
            loop  {
                arg = comma;
                if arg.is_null() { break ; }
                let mut prefixlen: libc::c_int = 0;
                let mut is_exclude: libc::c_int = 0;
                let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut subnet: *mut AddrList = 0 as *mut AddrList;
                let mut addr: AllAddr =
                    AllAddr {addr4: InAddr {s_addr: 0,},};
                comma = split(arg);
                prefix = split_chr(arg, '/' as i32 as libc::c_char);
                if !prefix.is_null() &&
                       atoi_check(prefix, &mut prefixlen) == 0 {
                    strcpy(errstr, gen_err);
                    return 0
                }
                if strstr(arg,
                          b"exclude:\x00" as *const u8 as *const libc::c_char)
                       == arg {
                    is_exclude = 1;
                    arg = arg.offset(8)
                }
                if inet_pton(2, arg,
                             &mut addr.addr4 as *mut InAddr as
                                 *mut libc::c_void) != 0 {
                    subnet =
                        opt_malloc(::std::mem::size_of::<AddrList>() as
                                       libc::c_ulong) as *mut AddrList;
                    (*subnet).prefixlen =
                        if prefixlen == 0 {
                            24
                        } else { prefixlen };
                    (*subnet).flags = 1
                } else if inet_pton(10, arg,
                                    &mut addr.addr6 as *mut In6Addr as
                                        *mut libc::c_void) != 0 {
                    subnet =
                        opt_malloc(::std::mem::size_of::<AddrList>() as
                                       libc::c_ulong) as *mut AddrList;
                    (*subnet).prefixlen =
                        if prefixlen == 0 {
                            64
                        } else { prefixlen };
                    (*subnet).flags = 1 | 2
                } else {
                    let mut name_1: *mut AuthNameList =
                        opt_malloc(::std::mem::size_of::<AuthNameList>() as
                                       libc::c_ulong) as *mut AuthNameList;
                    (*name_1).name = opt_string_alloc(arg);
                    (*name_1).flags = 2 | 1;
                    (*name_1).next = (*new_6).interface_names;
                    (*new_6).interface_names = name_1;
                    if !prefix.is_null() {
                        if prefixlen == 4 {
                            (*name_1).flags &= !(1)
                        } else if prefixlen == 6 {
                            (*name_1).flags &= !(2)
                        } else {
                            strcpy(errstr, gen_err);
                            return 0
                        }
                    }
                }
                if !subnet.is_null() {
                    (*subnet).addr = addr;
                    if is_exclude != 0 {
                        (*subnet).next = (*new_6).exclude;
                        (*new_6).exclude = subnet
                    } else {
                        (*subnet).next = (*new_6).subnet;
                        (*new_6).subnet = subnet
                    }
                }
            }
            current_block = 7879481898411272068;
        }
        316 => {
            /* --auth-soa */
            comma = split(arg);
            daemon.soa_sn = atoi(arg) as u32 as libc::c_ulong;
            if !comma.is_null() {
                let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
                arg = comma;
                comma = split(arg);
                daemon.hostmaster = opt_string_alloc(arg);
                cp = daemon.hostmaster;
                while cp != 0 {
                    if cp == '@' as i32 {
                        cp = '.' as i32 as libc::c_char
                    }
                    cp = cp.offset(1)
                }
                if !comma.is_null() {
                    arg = comma;
                    comma = split(arg);
                    daemon.soa_refresh =
                        atoi(arg) as u32 as libc::c_ulong;
                    if !comma.is_null() {
                        arg = comma;
                        comma = split(arg);
                        daemon.soa_retry =
                            atoi(arg) as u32 as libc::c_ulong;
                        if !comma.is_null() {
                            daemon.soa_expiry =
                                atoi(comma) as u32 as libc::c_ulong
                        }
                    }
                }
            }
            current_block = 7879481898411272068;
        }
        115 | 320 => {
            /* --domain */
            /* --synth-domain */
            if strcmp(arg, b"#\x00" as *const u8 as *const libc::c_char) ==
                   0 {
                set_option_bool(15 as libc::c_uint);
            } else {
                let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
                comma = split(arg);
                d = canonicalise_opt(arg);
                if d.is_null() {
                    strcpy(errstr, gen_err);
                    return 0
                } else {
                    if !comma.is_null() {
                        let mut new_7: *mut CondDomain =
                            opt_malloc(::std::mem::size_of::<CondDomain>() as
                                           libc::c_ulong) as *mut CondDomain;
                        let mut netpart: *mut libc::c_char =
                            0 as *mut libc::c_char;
                        (*new_7).prefix = 0 as *mut libc::c_char;
                        (*new_7).indexed = 0;
                        unhide_metas(comma);
                        netpart =
                            split_chr(comma, '/' as i32 as libc::c_char);
                        if !netpart.is_null() {
                            let mut msize: libc::c_int = 0;
                            arg = split(netpart);
                            if atoi_check(netpart, &mut msize) == 0 {
                                strcpy(errstr, gen_err);
                                free(new_7 as *mut libc::c_void);
                                return 0
                            } else {
                                if inet_pton(2, comma,
                                             &mut (*new_7).start as
                                                 *mut InAddr as
                                                 *mut libc::c_void) != 0 {
                                    let mut mask: libc::c_int =
                                        ((1) <<
                                             32 - msize) -
                                            1;
                                    (*new_7).is6 = 0;
                                    (*new_7).start.s_addr =
                                        __bswap_32(__bswap_32((*new_7).start.s_addr)
                                                       &
                                                       !mask as libc::c_uint);
                                    (*new_7).end.s_addr =
                                        (*new_7).start.s_addr |
                                            __bswap_32(mask as u32);
                                    if !arg.is_null() {
                                        if option != 's' as i32 {
                                            (*new_7).prefix =
                                                canonicalise_opt(arg);
                                            if (*new_7).prefix.is_null() ||
                                                   strlen((*new_7).prefix) >
                                                       (63 -
                                                            16)
                                                           as libc::c_ulong {
                                                strcpy(errstr,
                                                       b"bad prefix\x00" as
                                                           *const u8 as
                                                           *const libc::c_char);
                                                free(new_7 as
                                                         *mut libc::c_void);
                                                return 0
                                            }
                                        } else if strcmp(arg,
                                                         b"local\x00" as
                                                             *const u8 as
                                                             *const libc::c_char)
                                                      != 0 ||
                                                      msize !=
                                                          8 &&
                                                          msize !=
                                                              16 as
                                                                  libc::c_int
                                                          &&
                                                          msize !=
                                                              24 as
                                                                  libc::c_int
                                         {
                                            strcpy(errstr, gen_err);
                                            free(new_7 as *mut libc::c_void);
                                            return 0
                                        } else {
                                            /* generate the equivalent of
				      local=/xxx.yyy.zzz.in-addr.arpa/ */
                                            let mut serv: *mut Server =
                                                add_rev4((*new_7).start,
                                                         msize);
                                            if serv.is_null() {
                                                strcpy(errstr,
                                                       b"bad prefix\x00" as
                                                           *const u8 as
                                                           *const libc::c_char);
                                                free(new_7 as
                                                         *mut libc::c_void);
                                                return 0
                                            }
                                            serv.flags |= 2;
                                            /* local=/<domain>/ */
                                            serv =
                                                opt_malloc(::std::mem::size_of::<Server>()
                                                               as
                                                               libc::c_ulong)
                                                    as *mut Server;
                                            memset(serv as *mut libc::c_void,
                                                   0,
                                                   ::std::mem::size_of::<Server>()
                                                       as libc::c_ulong);
                                            serv.domain = d;
                                            serv.flags =
                                                8 |
                                                    2;
                                            serv.next =
                                                daemon.servers;
                                            daemon.servers = serv
                                        }
                                    }
                                } else if inet_pton(10, comma,
                                                    &mut (*new_7).start6 as
                                                        *mut In6Addr as
                                                        *mut libc::c_void) !=
                                              0 {
                                    let mut mask_0: u64 =
                                        ((1 as libc::c_ulonglong) <<
                                             128 -
                                                 msize).wrapping_sub(1 as
                                                                         libc::c_ulonglong);
                                    let mut addrpart: u64 =
                                        addr6part(&mut (*new_7).start6);
                                    (*new_7).is6 = 1;
                                    /* prefix==64 overflows the mask calculation above */
                                    if msize == 64 {
                                        mask_0 =
                                            -(1 as libc::c_longlong) as u64
                                    }
                                    (*new_7).end6 = (*new_7).start6;
                                    setaddr6part(&mut (*new_7).start6,
                                                 addrpart & !mask_0);
                                    setaddr6part(&mut (*new_7).end6,
                                                 addrpart | mask_0);
                                    if msize < 64 {
                                        strcpy(errstr, gen_err);
                                        free(new_7 as *mut libc::c_void);
                                        return 0
                                    } else {
                                        if !arg.is_null() {
                                            if option != 's' as i32 {
                                                (*new_7).prefix =
                                                    canonicalise_opt(arg);
                                                if (*new_7).prefix.is_null()
                                                       ||
                                                       strlen((*new_7).prefix)
                                                           >
                                                           (63
                                                                -
                                                                46 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_ulong {
                                                    strcpy(errstr,
                                                           b"bad prefix\x00"
                                                               as *const u8 as
                                                               *const libc::c_char);
                                                    free(new_7 as
                                                             *mut libc::c_void);
                                                    return 0
                                                }
                                            } else if strcmp(arg,
                                                             b"local\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char)
                                                          != 0
                                                          ||
                                                          msize &
                                                              4
                                                              !=
                                                              0
                                             {
                                                strcpy(errstr, gen_err);
                                                free(new_7 as
                                                         *mut libc::c_void);
                                                return 0
                                            } else {
                                                /* generate the equivalent of
				     local=/xxx.yyy.zzz.ip6.arpa/ */
                                                let mut serv_0: *mut Server =
                                                    add_rev6(&mut (*new_7).start6,
                                                             msize);
                                                (*serv_0).flags |=
                                                    2;
                                                /* local=/<domain>/ */
                                                serv_0 =
                                                    opt_malloc(::std::mem::size_of::<Server>()
                                                                   as
                                                                   libc::c_ulong)
                                                        as *mut Server;
                                                memset(serv_0 as
                                                           *mut libc::c_void,
                                                       0,
                                                       ::std::mem::size_of::<Server>()
                                                           as libc::c_ulong);
                                                (*serv_0).domain = d;
                                                (*serv_0).flags =
                                                    8 |
                                                        2;
                                                (*serv_0).next =
                                                    daemon.servers;
                                                daemon.servers =
                                                    serv_0
                                            }
                                        }
                                    }
                                } else {
                                    strcpy(errstr, gen_err);
                                    free(new_7 as *mut libc::c_void);
                                    return 0
                                }
                            }
                        } else {
                            let mut prefstr_0: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            arg = split(comma);
                            prefstr_0 = split(arg);
                            if inet_pton(2, comma,
                                         &mut (*new_7).start as *mut InAddr
                                             as *mut libc::c_void) != 0 {
                                (*new_7).is6 = 0;
                                if arg.is_null() {
                                    (*new_7).end.s_addr =
                                        (*new_7).start.s_addr
                                } else if inet_pton(2, arg,
                                                    &mut (*new_7).end as
                                                        *mut InAddr as
                                                        *mut libc::c_void) ==
                                              0 {
                                    strcpy(errstr, gen_err);
                                    free(new_7 as *mut libc::c_void);
                                    return 0
                                }
                            } else if inet_pton(10, comma,
                                                &mut (*new_7).start6 as
                                                    *mut In6Addr as
                                                    *mut libc::c_void) != 0 {
                                (*new_7).is6 = 1;
                                if arg.is_null() {
                                    memcpy(&mut (*new_7).end6 as *mut In6Addr
                                               as *mut libc::c_void,
                                           &mut (*new_7).start6 as
                                               *mut In6Addr as
                                               *const libc::c_void,
                                           16 as
                                               libc::c_ulong);
                                } else if inet_pton(10, arg,
                                                    &mut (*new_7).end6 as
                                                        *mut In6Addr as
                                                        *mut libc::c_void) ==
                                              0 {
                                    strcpy(errstr, gen_err);
                                    free(new_7 as *mut libc::c_void);
                                    return 0
                                }
                            } else {
                                strcpy(errstr, gen_err);
                                free(new_7 as *mut libc::c_void);
                                return 0
                            }
                            if option != 's' as i32 && !prefstr_0.is_null() {
                                (*new_7).prefix = canonicalise_opt(prefstr_0);
                                if (*new_7).prefix.is_null() ||
                                       strlen((*new_7).prefix) >
                                           (63 -
                                                16) as
                                               libc::c_ulong {
                                    strcpy(errstr,
                                           b"bad prefix\x00" as *const u8 as
                                               *const libc::c_char);
                                    free(new_7 as *mut libc::c_void);
                                    return 0
                                }
                            }
                        }
                        (*new_7).domain = d;
                        if option == 's' as i32 {
                            (*new_7).next = daemon.cond_domain;
                            daemon.cond_domain = new_7
                        } else {
                            let mut star: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            (*new_7).next = daemon.synth_domains;
                            daemon.synth_domains = new_7;
                            if !(*new_7).prefix.is_null() &&
                                   {
                                       star =
                                           strrchr((*new_7).prefix,
                                                   '*' as i32);
                                       !star.is_null()
                                   } &&
                                   *star.offset(1) as
                                       libc::c_int == 0 {
                                *star = 0 as libc::c_char;
                                (*new_7).indexed = 1
                            }
                        }
                    } else if option == 's' as i32 {
                        daemon.domain_suffix = d
                    } else {
                        strcpy(errstr, gen_err);
                        return 0
                    }
                }
            }
            current_block = 7879481898411272068;
        }
        346 => {
            /* --add-dns-client */
            if !arg.is_null() {
                daemon.dns_client_id = opt_string_alloc(arg)
            }
            current_block = 7879481898411272068;
        }
        300 => {
            /* --add-mac */
            if arg.is_null() {
                set_option_bool(32 as libc::c_uint);
            } else {
                unhide_metas(arg);
                if strcmp(arg,
                          b"base64\x00" as *const u8 as *const libc::c_char)
                       == 0 {
                    set_option_bool(54 as libc::c_uint);
                } else if strcmp(arg,
                                 b"text\x00" as *const u8 as
                                     *const libc::c_char) == 0
                 {
                    set_option_bool(55 as libc::c_uint);
                } else { strcpy(errstr, gen_err); return 0 }
            }
            current_block = 7879481898411272068;
        }
        117 => {
            /* --user */
            daemon.username = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        103 => {
            /* --group */
            daemon.groupname = opt_string_alloc(arg);
            daemon.group_set = 1;
            current_block = 7879481898411272068;
        }
        285 => {
            /* --scriptuser */
            daemon.scriptuser = opt_string_alloc(arg);
            current_block = 7879481898411272068;
        }
        105 => {
            loop
                 /* --interface */
                 {
                let mut new_8: *mut Iname =
                    opt_malloc(::std::mem::size_of::<Iname>() as
                                   libc::c_ulong) as *mut Iname;
                comma = split(arg);
                (*new_8).next = daemon.if_names;
                daemon.if_names = new_8;
                /* new->name may be NULL if someone does
	   "interface=" to disable all interfaces except loop. */
                (*new_8).name = opt_string_alloc(arg);
                (*new_8).used = 0;
                arg = comma;
                if arg.is_null() { break ; }
            }
            current_block = 7879481898411272068;
        }
        258 => {
            /* --enable-tftp */
            set_option_bool(40 as libc::c_uint);
            if arg.is_null() {
                current_block = 7879481898411272068;
            } else {
                /* fall through */
                current_block = 887445304002143054;
            }
        }
        73 | 50 => { current_block = 887445304002143054; }
        66 => {
            /* --bogus-nxdomain */
            current_block = 2926860427235594157;
        }
        338 => { current_block = 2926860427235594157; }
        97 | 318 => {
            /* --listen-address */
            loop
                 /* --auth-peer */
                 {
                let mut new_10: *mut Iname =
                    opt_malloc(::std::mem::size_of::<Iname>() as
                                   libc::c_ulong) as *mut Iname;
                comma = split(arg);
                unhide_metas(arg);
                if !arg.is_null() &&
                       inet_pton(2, arg,
                                 &mut (*new_10).addr.in_0.sin_addr as
                                     *mut InAddr as *mut libc::c_void) >
                           0 {
                    (*new_10).addr.sa.sa_family =
                        2 as SaFamily;
                    (*new_10).addr.in_0.sin_port =
                        0 as in_port_t
                } else if !arg.is_null() &&
                              inet_pton(10, arg,
                                        &mut (*new_10).addr.in6.sin6_addr as
                                            *mut In6Addr as
                                            *mut libc::c_void) >
                                  0 {
                    (*new_10).addr.sa.sa_family =
                        10 as SaFamily;
                    (*new_10).addr.in6.sin6_flowinfo =
                        0 as u32;
                    (*new_10).addr.in6.sin6_scope_id =
                        0 as u32;
                    (*new_10).addr.in6.sin6_port =
                        0 as in_port_t
                } else {
                    strcpy(errstr, gen_err);
                    free(new_10 as *mut libc::c_void);
                    return 0
                }
                (*new_10).used = 0;
                if option == 'a' as i32 {
                    (*new_10).next = daemon.if_addrs;
                    daemon.if_addrs = new_10
                } else {
                    (*new_10).next = daemon.auth_peers;
                    daemon.auth_peers = new_10
                }
                arg = comma;
                if arg.is_null() { break ; }
            }
            current_block = 7879481898411272068;
        }
        83 => {
            /*  --server */
            current_block = 9676380469790025234;
        }
        286 => { current_block = 9676380469790025234; }
        65 => { current_block = 6480954168551069607; }
        298 => { current_block = 14399141444697241811; }
        332 => {
            /* --rev-server */
            let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut size: libc::c_int = 0;
            let mut serv_2: *mut Server = 0 as *mut Server;
            let mut addr4: InAddr = InAddr {s_addr: 0,};
            let mut addr6: In6Addr =
                In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
            unhide_metas(arg);
            if arg.is_null() {
                strcpy(errstr, gen_err);
                return 0
            }
            comma = split(arg);
            string = split_chr(arg, '/' as i32 as libc::c_char);
            if string.is_null() || atoi_check(string, &mut size) == 0 {
                strcpy(errstr, gen_err);
                return 0
            }
            if inet_pton(2, arg,
                         &mut addr4 as *mut InAddr as *mut libc::c_void) != 0
               {
                serv_2 = add_rev4(addr4, size);
                if serv_2.is_null() {
                    strcpy(errstr,
                           b"bad prefix\x00" as *const u8 as
                               *const libc::c_char);
                    return 0
                }
            } else if inet_pton(10, arg,
                                &mut addr6 as *mut In6Addr as
                                    *mut libc::c_void) != 0 {
                serv_2 = add_rev6(&mut addr6, size)
            } else { strcpy(errstr, gen_err); return 0 }
            string =
                parse_server(comma, &mut (*serv_2).addr,
                             &mut (*serv_2).source_addr,
                             (*serv_2).interface.as_mut_ptr(),
                             &mut (*serv_2).flags);
            if !string.is_null() {
                strcpy(errstr, string);
                return 0
            }
            if servers_only != 0 { (*serv_2).flags |= 4096 }
            current_block = 7879481898411272068;
        }
        319 => {
            /* --ipset */
            let mut ipsets_head: IpSets =
                IpSets {sets: 0 as *mut *mut libc::c_char,
                       domain: 0 as *mut libc::c_char,
                       next: 0 as *mut IpSets,};
            let mut ipsets: *mut IpSets = &mut ipsets_head;
            let mut size_0: libc::c_int = 0;
            let mut end_1: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut sets: *mut *mut libc::c_char =
                0 as *mut *mut libc::c_char;
            let mut sets_pos: *mut *mut libc::c_char =
                0 as *mut *mut libc::c_char;
            memset(ipsets as *mut libc::c_void, 0,
                   ::std::mem::size_of::<IpSets>() as libc::c_ulong);
            unhide_metas(arg);
            if !arg.is_null() && *arg == '/' as i32 {
                arg = arg.offset(1);
                loop  {
                    end_1 = split_chr(arg, '/' as i32 as libc::c_char);
                    if end_1.is_null() { break ; }
                    let mut domain_0: *mut libc::c_char =
                        0 as *mut libc::c_char;
                    /* elide leading dots - they are implied in the search algorithm */
                    while *arg == '.' as i32 {
                        arg = arg.offset(1)
                    }
                    /* # matches everything and becomes a zero length domain string */
                    if strcmp(arg,
                              b"#\x00" as *const u8 as *const libc::c_char) ==
                           0 || *arg == 0 {
                        domain_0 =
                            b"\x00" as *const u8 as *const libc::c_char as
                                *mut libc::c_char
                    } else if strlen(arg) != 0 as libc::c_ulong
                                  &&
                                  {
                                      domain_0 = canonicalise_opt(arg);
                                      domain_0.is_null()
                                  } {
                        strcpy(errstr, gen_err);
                        return 0
                    }
                    (*ipsets).next =
                        opt_malloc(::std::mem::size_of::<IpSets>() as
                                       libc::c_ulong) as *mut IpSets;
                    ipsets = (*ipsets).next;
                    memset(ipsets as *mut libc::c_void, 0,
                           ::std::mem::size_of::<IpSets>() as libc::c_ulong);
                    (*ipsets).domain = domain_0;
                    arg = end_1
                }
            } else {
                (*ipsets).next =
                    opt_malloc(::std::mem::size_of::<IpSets>() as
                                   libc::c_ulong) as *mut IpSets;
                ipsets = (*ipsets).next;
                memset(ipsets as *mut libc::c_void, 0,
                       ::std::mem::size_of::<IpSets>() as libc::c_ulong);
                (*ipsets).domain =
                    b"\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            }
            if arg.is_null() || *arg == 0 {
                strcpy(errstr, gen_err);
                return 0
            }
            size_0 = 2;
            end_1 = arg;
            while *end_1 != 0 {
                if *end_1 == ',' as i32 { size_0 += 1 }
                end_1 = end_1.offset(1)
            }
            sets_pos =
                opt_malloc((::std::mem::size_of::<*mut libc::c_char>() as
                                libc::c_ulong).wrapping_mul(size_0 as
                                                                libc::c_ulong))
                    as *mut *mut libc::c_char;
            sets = sets_pos;
            loop  {
                end_1 = split(arg);
                let fresh27 = sets_pos;
                sets_pos = sets_pos.offset(1);
                *fresh27 = opt_string_alloc(arg);
                arg = end_1;
                if end_1.is_null() { break ; }
            }
            *sets_pos = 0 as *mut libc::c_char;
            ipsets = &mut ipsets_head;
            while !(*ipsets).next.is_null() {
                (*(*ipsets).next).sets = sets;
                ipsets = (*ipsets).next
            }
            (*ipsets).next = daemon.ipsets;
            daemon.ipsets = ipsets_head.next;
            current_block = 7879481898411272068;
        }
        99 => {
            /* --cache-size */
            let mut size_1: libc::c_int = 0;
            if atoi_check(arg, &mut size_1) == 0 {
                strcpy(errstr, gen_err);
                return 0
            } else {
                /* zero is OK, and means no caching. */
                if size_1 < 0 { size_1 = 0 }
                /* Note that for very large cache sizes, the malloc()
	       will overflow. For the size of the cache record
	       at the time this was noted, the value of "very large"
               was 46684428. Limit to an order of magnitude less than
	       that to be safe from changes to the cache record. */
                if size_1 > 5000000 {
                    size_1 = 5000000
                }
                daemon.cachesize = size_1
            }
            current_block = 7879481898411272068;
        }
        112 => {
            /* --port */
            if atoi_check16(arg, &mut daemon.port) == 0 {
                strcpy(errstr, gen_err);
                return 0
            }
            current_block = 7879481898411272068;
        }
        288 => {
            /* --min-port */
            if atoi_check16(arg, &mut daemon.min_port) == 0 {
                strcpy(errstr, gen_err);
                return 0
            }
            current_block = 7879481898411272068;
        }
        345 => {
            /* --max-port */
            if atoi_check16(arg, &mut daemon.max_port) == 0 {
                strcpy(errstr, gen_err);
                return 0
            }
            current_block = 7879481898411272068;
        }
        48 => {
            /* --dns-forward-max */
            if atoi_check(arg, &mut daemon.ftabsize) == 0 {
                strcpy(errstr, gen_err);
                return 0
            }
            current_block = 7879481898411272068;
        }
        113 => {
            /* --log-queries */
            set_option_bool(2 as libc::c_uint); /* default */
            if !arg.is_null() &&
                   strcmp(arg,
                          b"extra\x00" as *const u8 as *const libc::c_char) ==
                       0 {
                set_option_bool(51 as libc::c_uint);
            }
            current_block = 7879481898411272068;
        }
        267 => {
            /* --log-async */
            daemon.max_logs = 5;
            if !arg.is_null() &&
                   atoi_check(arg, &mut daemon.max_logs) == 0 {
                strcpy(errstr, gen_err);
                return 0
            } else {
                if daemon.max_logs > 100 {
                    daemon.max_logs = 100
                }
            }
            current_block = 7879481898411272068;
        }
        80 => {
            /* --edns-packet-max */
            let mut i_0: libc::c_int = 0;
            if atoi_check(arg, &mut i_0) == 0 {
                strcpy(errstr, gen_err);
                return 0
            }
            daemon.edns_pktsz = i_0 as libc::c_ushort;
            current_block = 7879481898411272068;
        }
        81 => {
            /* --query-port */
            if atoi_check16(arg, &mut daemon.query_port) == 0 {
                strcpy(errstr, gen_err);
                return 0
            }
            /* if explicitly set to zero, use single OS ephemeral port
	 and disable random ports */
            if daemon.query_port == 0 {
                daemon.osport = 1
            }
            current_block = 7879481898411272068;
        }
        84 => {
            /* --local-ttl */
            current_block = 15489771604880449635;
        }
        283 => { current_block = 15489771604880449635; }
        297 => { current_block = 6082976577402880686; }
        339 => { current_block = 16916584745428150692; }
        312 => { current_block = 13094692781038244044; }
        315 => { current_block = 13035992208579083528; }
        348 => { current_block = 5893551302610724882; }
        88 => {
            /* --dhcp-lease-max */
            if atoi_check(arg, &mut daemon.dhcp_max) == 0 {
                strcpy(errstr, gen_err);
                return 0
            }
            current_block = 7879481898411272068;
        }
        263 => {
            /*  --tftp-max */
            if atoi_check(arg, &mut daemon.tftp_max) == 0 {
                strcpy(errstr, gen_err);
                return 0
            }
            current_block = 7879481898411272068;
        }
        349 => {
            /*  --tftp-mtu */
            if atoi_check(arg, &mut daemon.tftp_mtu) == 0 {
                strcpy(errstr, gen_err);
                return 0
            }
            current_block = 7879481898411272068;
        }
        260 => {
            /* --tftp-prefix */
            comma = split(arg);
            if !comma.is_null() {
                let mut new_11: *mut TftpPrefix =
                    opt_malloc(::std::mem::size_of::<TftpPrefix>() as
                                   libc::c_ulong) as *mut TftpPrefix;
                (*new_11).interface = opt_string_alloc(comma);
                (*new_11).prefix = opt_string_alloc(arg);
                (*new_11).next = daemon.if_prefix;
                daemon.if_prefix = new_11
            } else { daemon.tftp_prefix = opt_string_alloc(arg) }
            current_block = 7879481898411272068;
        }
        276 => {
            /* --tftp-port-range */
            comma = split(arg);
            if comma.is_null() ||
                   atoi_check16(arg, &mut daemon.start_tftp_port)
                       == 0 ||
                   atoi_check16(comma, &mut daemon.end_tftp_port)
                       == 0 {
                strcpy(errstr,
                       b"bad port range\x00" as *const u8 as
                           *const libc::c_char);
                return 0
            }
            if daemon.start_tftp_port >
                   daemon.end_tftp_port {
                let mut tmp: libc::c_int = daemon.start_tftp_port;
                daemon.start_tftp_port =
                    daemon.end_tftp_port;
                daemon.end_tftp_port = tmp
            }
            current_block = 7879481898411272068;
        }
        274 => {
            /* --tftp-unique-root */
            if arg.is_null() ||
                   strcasecmp(arg,
                              b"ip\x00" as *const u8 as *const libc::c_char)
                       == 0 {
                set_option_bool(29 as libc::c_uint);
            } else if strcasecmp(arg,
                                 b"mac\x00" as *const u8 as
                                     *const libc::c_char) == 0
             {
                set_option_bool(56 as libc::c_uint);
            } else { strcpy(errstr, gen_err); return 0 }
            current_block = 7879481898411272068;
        }
        262 => {
            /* --bridge-interface */
            let mut new_12: *mut DhcpBridge = 0 as *mut DhcpBridge;
            comma = split(arg);
            if comma.is_null() ||
                   strlen(arg) >
                       (16 - 1) as libc::c_ulong
               {
                strcpy(errstr,
                       b"bad bridge-interface\x00" as *const u8 as
                           *const libc::c_char);
                return 0
            }
            new_12 = daemon.bridges;
            while !new_12.is_null() {
                if strcmp((*new_12).iface.as_mut_ptr(), arg) ==
                       0 {
                    break ;
                }
                new_12 = (*new_12).next
            }
            if new_12.is_null() {
                new_12 =
                    opt_malloc(::std::mem::size_of::<DhcpBridge>() as
                                   libc::c_ulong) as *mut DhcpBridge;
                strcpy((*new_12).iface.as_mut_ptr(), arg);
                (*new_12).alias = 0 as *mut DhcpBridge;
                (*new_12).next = daemon.bridges;
                daemon.bridges = new_12
            }
            loop  {
                arg = comma;
                comma = split(arg);
                if strlen(arg) != 0 as libc::c_ulong &&
                       strlen(arg) <=
                           (16 - 1) as
                               libc::c_ulong {
                    let mut b: *mut DhcpBridge =
                        opt_malloc(::std::mem::size_of::<DhcpBridge>() as
                                       libc::c_ulong) as *mut DhcpBridge;
                    (*b).next = (*new_12).alias;
                    (*new_12).alias = b;
                    strcpy((*b).iface.as_mut_ptr(), arg);
                }
                if comma.is_null() { break ; }
            }
            current_block = 7879481898411272068;
        }
        357 => {
            /* --shared-network */
            let mut new_13: *mut SharedNetwork =
                opt_malloc(::std::mem::size_of::<SharedNetwork>() as
                               libc::c_ulong) as *mut SharedNetwork;
            (*new_13).shared_addr.s_addr = 0 as InAddrT;
            (*new_13).if_index = 0;
            comma = split(arg);
            if comma.is_null() {
                current_block = 3177757304694473968;
            } else {
                if inet_pton(2, comma,
                             &mut (*new_13).shared_addr as *mut InAddr as
                                 *mut libc::c_void) != 0 {
                    if inet_pton(2, arg,
                                 &mut (*new_13).match_addr as *mut InAddr as
                                     *mut libc::c_void) == 0 &&
                           {
                               (*new_13).if_index =
                                   if_nametoindex(arg);
                               ((*new_13).if_index) == 0
                           } {
                        current_block = 3177757304694473968;
                    } else { current_block = 12609744167958600007; }
                } else if inet_pton(10, comma,
                                    &mut (*new_13).shared_addr6 as
                                        *mut In6Addr as *mut libc::c_void) !=
                              0 {
                    if inet_pton(10, arg,
                                 &mut (*new_13).match_addr6 as *mut In6Addr
                                     as *mut libc::c_void) == 0 &&
                           {
                               (*new_13).if_index =
                                   if_nametoindex(arg);
                               ((*new_13).if_index) == 0
                           } {
                        current_block = 3177757304694473968;
                    } else { current_block = 12609744167958600007; }
                } else { current_block = 3177757304694473968; }
                match current_block {
                    3177757304694473968 => { }
                    _ => {
                        (*new_13).next = daemon.shared_networks;
                        daemon.shared_networks = new_13;
                        current_block = 7879481898411272068;
                    }
                }
            }
            match current_block {
                7879481898411272068 => { }
                _ => {
                    free(new_13 as *mut libc::c_void);
                    strcpy(errstr,
                           b"bad shared-network\x00" as *const u8 as
                               *const libc::c_char);
                    return 0
                }
            }
        }
        70 => {
            /* --dhcp-range */
            let mut k: libc::c_int = 0;
            let mut leasepos: libc::c_int = 2;
            let mut cp_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut a: [*mut libc::c_char; 8] =
                [0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char, 0 as *mut libc::c_char];
            let mut new_14: *mut DhcpContext =
                opt_malloc(::std::mem::size_of::<DhcpContext>() as
                               libc::c_ulong) as *mut DhcpContext;
            memset(new_14 as *mut libc::c_void, 0,
                   ::std::mem::size_of::<DhcpContext>() as libc::c_ulong);
            loop  {
                cp_0 = arg;
                while cp_0 != 0 {
                    if !(cp_0 == ' ' as i32 ||
                             cp_0 == '.' as i32 ||
                             cp_0 == ':' as i32 ||
                             cp_0 >= 'a' as i32 &&
                                 cp_0 <= 'f' as i32 ||
                             cp_0 >= 'A' as i32 &&
                                 cp_0 <= 'F' as i32 ||
                             cp_0 >= '0' as i32 &&
                                 cp_0 <= '9' as i32) {
                        break ;
                    }
                    cp_0 = cp_0.offset(1)
                }
                if cp_0 != ',' as i32 &&
                       { comma = split(arg); !comma.is_null() } {
                    if is_tag_prefix(arg) != 0 {
                        /* ignore empty tag */
                        if *arg.offset(4) != 0 {
                            (*new_14).filter =
                                dhcp_netid_create(arg.offset(4
                                                                ),
                                                  (*new_14).filter)
                        }
                    } else if !(*new_14).netid.net.is_null() {
                        dhcp_context_free(new_14); /* default */
                        strcpy(errstr,
                               b"only one tag allowed\x00" as *const u8 as
                                   *const libc::c_char);
                        return 0
                    } else {
                        (*new_14).netid.net =
                            opt_string_alloc(set_prefix(arg))
                    }
                    arg = comma
                } else { a[0 as usize] = arg; break ; }
            }
            k = 1;
            while k < 8 {
                a[k as usize] = split(a[(k - 1) as usize]);
                if a[k as usize].is_null() { break ; }
                k += 1
            }
            if k < 2 {
                dhcp_context_free(new_14);
                strcpy(errstr,
                       b"bad dhcp-range\x00" as *const u8 as
                           *const libc::c_char);
                return 0
            }
            if inet_pton(2, a[0 as usize],
                         &mut (*new_14).start as *mut InAddr as
                             *mut libc::c_void) != 0 {
                (*new_14).next = daemon.dhcp;
                (*new_14).lease_time = 3600 as libc::c_uint;
                daemon.dhcp = new_14;
                (*new_14).end = (*new_14).start;
                if strcmp(a[1 as usize],
                          b"static\x00" as *const u8 as *const libc::c_char)
                       == 0 {
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint |
                             (1 as libc::c_uint) << 0) as
                            libc::c_int
                } else if strcmp(a[1 as usize],
                                 b"proxy\x00" as *const u8 as
                                     *const libc::c_char) == 0
                 {
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint |
                             (1 as libc::c_uint) << 3) as
                            libc::c_int
                } else if inet_pton(2,
                                    a[1 as usize],
                                    &mut (*new_14).end as *mut InAddr as
                                        *mut libc::c_void) == 0 {
                    dhcp_context_free(new_14);
                    strcpy(errstr,
                           b"bad dhcp-range\x00" as *const u8 as
                               *const libc::c_char);
                    return 0
                }
                if __bswap_32((*new_14).start.s_addr) >
                       __bswap_32((*new_14).end.s_addr) {
                    let mut tmp_0: InAddr = (*new_14).start;
                    (*new_14).start = (*new_14).end;
                    (*new_14).end = tmp_0
                }
                if k >= 3 &&
                       !strchr(a[2 as usize],
                               '.' as i32).is_null() &&
                       inet_pton(2,
                                 a[2 as usize],
                                 &mut (*new_14).netmask as *mut InAddr as
                                     *mut libc::c_void) > 0 {
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint |
                             (1 as libc::c_uint) << 1) as
                            libc::c_int;
                    leasepos = 3;
                    if is_same_net((*new_14).start, (*new_14).end,
                                   (*new_14).netmask) == 0 {
                        dhcp_context_free(new_14);
                        strcpy(errstr,
                               b"inconsistent DHCP range\x00" as *const u8 as
                                   *const libc::c_char);
                        return 0
                    }
                    if k >= 4 &&
                           !strchr(a[3 as usize],
                                   '.' as i32).is_null() &&
                           inet_pton(2,
                                     a[3 as usize],
                                     &mut (*new_14).broadcast as *mut InAddr
                                         as *mut libc::c_void) >
                               0 {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (1 as libc::c_uint) << 2) as
                                libc::c_int;
                        leasepos = 4
                    }
                }
            } else if inet_pton(10,
                                a[0 as usize],
                                &mut (*new_14).start6 as *mut In6Addr as
                                    *mut libc::c_void) != 0 {
                let mut err_1: *const libc::c_char = 0 as *const libc::c_char;
                (*new_14).flags =
                    ((*new_14).flags as libc::c_uint |
                         (1 as libc::c_uint) << 17) as
                        libc::c_int;
                (*new_14).prefix = 64;
                (*new_14).end6 = (*new_14).start6;
                (*new_14).lease_time =
                    (3600 * 24) as libc::c_uint;
                (*new_14).next = daemon.dhcp6;
                daemon.dhcp6 = new_14;
                leasepos = 1;
                while leasepos < k {
                    if strcmp(a[leasepos as usize],
                              b"static\x00" as *const u8 as
                                  *const libc::c_char) == 0 {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 ((1 as libc::c_uint) << 0 |
                                      (1 as libc::c_uint) <<
                                          8))
                    } else if strcmp(a[leasepos as usize],
                                     b"ra-only\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 ||
                                  strcmp(a[leasepos as usize],
                                         b"slaac\x00" as *const u8 as
                                             *const libc::c_char) ==
                                      0 {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (1 as libc::c_uint) << 13) as
                                libc::c_int
                    } else if strcmp(a[leasepos as usize],
                                     b"ra-names\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 ((1 as libc::c_uint) << 6 |
                                      (1 as libc::c_uint) <<
                                          13))
                    } else if strcmp(a[leasepos as usize],
                                     b"ra-advrouter\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 ((1 as libc::c_uint) << 4 |
                                      (1 as libc::c_uint) <<
                                          13))
                    } else if strcmp(a[leasepos as usize],
                                     b"ra-stateless\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 ((1 as libc::c_uint) << 7 |
                                      (1 as libc::c_uint) << 8
                                      |
                                      (1 as libc::c_uint) <<
                                          13))
                    } else if strcmp(a[leasepos as usize],
                                     b"off-link\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (1 as libc::c_uint) << 18) as
                                libc::c_int
                    } else if leasepos == 1 &&
                                  inet_pton(10,
                                            a[leasepos as usize],
                                            &mut (*new_14).end6 as
                                                *mut In6Addr as
                                                *mut libc::c_void) != 0 {
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (1 as libc::c_uint) << 8) as
                                libc::c_int
                    } else {
                        if !(strstr(a[leasepos as usize],
                                    b"constructor:\x00" as *const u8 as
                                        *const libc::c_char) ==
                                 a[leasepos as usize]) {
                            break ;
                        }
                        (*new_14).template_interface =
                            opt_string_alloc(a[leasepos as
                                                   usize].offset(12 as
                                                                     libc::c_int
                                                                     as
                                                                     isize));
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (1 as libc::c_uint) << 10) as
                                libc::c_int
                    }
                    leasepos += 1
                }
                /* bare integer < 128 is prefix value */
                if leasepos < k {
                    let mut pref_0: libc::c_int = 0;
                    cp_0 = a[leasepos as usize];
                    while cp_0 != 0 {
                        if !(cp_0 >= '0' as i32 &&
                                 cp_0 <= '9' as i32) {
                            break ;
                        }
                        cp_0 = cp_0.offset(1)
                    }
                    if cp_0 == 0 &&
                           {
                               pref_0 = atoi(a[leasepos as usize]);
                               (pref_0) <= 128
                           } {
                        (*new_14).prefix = pref_0;
                        leasepos += 1
                    }
                }
                if (*new_14).prefix > 64 {
                    if (*new_14).flags as libc::c_uint &
                           (1 as libc::c_uint) << 13 != 0 {
                        err_1 =
                            b"prefix length must be exactly 64 for RA subnets\x00"
                                as *const u8 as *const libc::c_char
                    } else if (*new_14).flags as libc::c_uint &
                                  (1 as libc::c_uint) << 10 !=
                                  0 {
                        err_1 =
                            b"prefix length must be exactly 64 for subnet constructors\x00"
                                as *const u8 as *const libc::c_char
                    }
                } else if (*new_14).prefix < 64 {
                    err_1 =
                        b"prefix length must be at least 64\x00" as *const u8
                            as *const libc::c_char
                }
                if err_1.is_null() &&
                       is_same_net6(&mut (*new_14).start6,
                                    &mut (*new_14).end6, (*new_14).prefix) ==
                           0 {
                    err_1 =
                        b"inconsistent DHCPv6 range\x00" as *const u8 as
                            *const libc::c_char
                }
                if !err_1.is_null() {
                    dhcp_context_free(new_14);
                    strcpy(errstr, err_1);
                    return 0
                }
                /* dhcp-range=:: enables DHCP stateless on any interface */
                if ({
                        let mut __a: *const In6Addr =
                            &mut (*new_14).start6 as *mut In6Addr as
                                *const In6Addr;
                        ((*__a).__in6_u.__u6_addr32[0 as usize]
                             == 0 as libc::c_uint &&
                             (*__a).__in6_u.__u6_addr32[1 as
                                                            usize] ==
                                 0 as libc::c_uint &&
                             (*__a).__in6_u.__u6_addr32[2 as
                                                            usize] ==
                                 0 as libc::c_uint &&
                             (*__a).__in6_u.__u6_addr32[3 as
                                                            usize] ==
                                 0 as libc::c_uint) as
                            libc::c_int
                    }) != 0 &&
                       (*new_14).flags as libc::c_uint &
                           (1 as libc::c_uint) << 10 == 0 {
                    (*new_14).prefix = 0
                }
                if (*new_14).flags as libc::c_uint &
                       (1 as libc::c_uint) << 10 != 0 {
                    let mut zero: In6Addr =
                        In6Addr {__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},};
                    memset(&mut zero as *mut In6Addr as *mut libc::c_void,
                           0,
                           ::std::mem::size_of::<In6Addr>() as
                               libc::c_ulong);
                    if is_same_net6(&mut zero, &mut (*new_14).start6,
                                    (*new_14).prefix) == 0 {
                        dhcp_context_free(new_14);
                        strcpy(errstr,
                               b"prefix must be zero with \"constructor:\" argument\x00"
                                   as *const u8 as *const libc::c_char);
                        return 0
                    }
                }
                if addr6part(&mut (*new_14).start6) >
                       addr6part(&mut (*new_14).end6) {
                    let mut tmp_1: In6Addr = (*new_14).start6;
                    (*new_14).start6 = (*new_14).end6;
                    (*new_14).end6 = tmp_1
                }
            } else {
                dhcp_context_free(new_14);
                strcpy(errstr,
                       b"bad dhcp-range\x00" as *const u8 as
                           *const libc::c_char);
                return 0
            }
            if leasepos < k {
                if leasepos != k - 1 {
                    dhcp_context_free(new_14);
                    strcpy(errstr,
                           b"bad dhcp-range\x00" as *const u8 as
                               *const libc::c_char);
                    return 0
                }
                if strcmp(a[leasepos as usize],
                          b"infinite\x00" as *const u8 as *const libc::c_char)
                       == 0 {
                    (*new_14).lease_time = 0xffffffff as libc::c_uint;
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint |
                             (1 as libc::c_uint) << 19) as
                            libc::c_int
                } else if strcmp(a[leasepos as usize],
                                 b"deprecated\x00" as *const u8 as
                                     *const libc::c_char) == 0
                 {
                    (*new_14).flags =
                        ((*new_14).flags as libc::c_uint |
                             (1 as libc::c_uint) << 9) as
                            libc::c_int
                } else {
                    let mut fac: libc::c_int = 1;
                    if strlen(a[leasepos as usize]) >
                           0 as libc::c_ulong {
                        let mut current_block_1049: u64;
                        match *a[leasepos as
                                     usize].offset(strlen(a[leasepos as
                                                                usize]).wrapping_sub(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                      ) as
                                  libc::c_int {
                            119 | 87 => {
                                fac *= 7;
                                current_block_1049 = 9610247714461258384;
                            }
                            100 | 68 => {
                                current_block_1049 = 9610247714461258384;
                            }
                            104 | 72 => {
                                current_block_1049 = 9280197982685904555;
                            }
                            109 | 77 => {
                                current_block_1049 = 17378754114849407475;
                            }
                            115 | 83 => {
                                current_block_1049 = 8582955123963743225;
                            }
                            _ => { current_block_1049 = 8758648760486203175; }
                        }
                        match current_block_1049 {
                            9610247714461258384 =>
                            /* fall through */
                            {
                                fac *= 24;
                                current_block_1049 = 9280197982685904555;
                            }
                            _ => { }
                        }
                        match current_block_1049 {
                            9280197982685904555 =>
                            /* fall through */
                            {
                                fac *= 60;
                                current_block_1049 = 17378754114849407475;
                            }
                            _ => { }
                        }
                        match current_block_1049 {
                            17378754114849407475 =>
                            /* fall through */
                            {
                                fac *= 60;
                                current_block_1049 = 8582955123963743225;
                            }
                            _ => { }
                        }
                        match current_block_1049 {
                            8582955123963743225 =>
                            /* fall through */
                            {
                                *a[leasepos as
                                       usize].offset(strlen(a[leasepos as
                                                                  usize]).wrapping_sub(1
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong)
                                                        ) =
                                    0 as libc::c_char
                            }
                            _ => { }
                        }
                        cp_0 = a[leasepos as usize];
                        while cp_0 != 0 {
                            if !(cp_0 >= '0' as i32 &&
                                     cp_0 <= '9' as i32) {
                                break ;
                            }
                            cp_0 = cp_0.offset(1)
                        }
                        if cp_0 != 0 ||
                               (leasepos + 1) < k {
                            strcpy(errstr,
                                   b"bad dhcp-range\x00" as *const u8 as
                                       *const libc::c_char);
                            free(new_14 as *mut libc::c_void);
                            return 0
                        }
                        (*new_14).lease_time =
                            (atoi(a[leasepos as usize]) * fac) as
                                libc::c_uint;
                        (*new_14).flags =
                            ((*new_14).flags as libc::c_uint |
                                 (1 as libc::c_uint) << 19) as
                                libc::c_int;
                        /* Leases of a minute or less confuse
		       some clients, notably Apple's */
                        if (*new_14).lease_time <
                               120 as libc::c_uint {
                            (*new_14).lease_time =
                                120 as libc::c_uint
                        }
                    }
                }
            }
            current_block = 7879481898411272068;
        }
        272 | 71 => {
            /* --dhcp-host */
            let mut new_15: *mut DhcpConfig = 0 as *mut DhcpConfig;
            let mut in_0: InAddr = InAddr {s_addr: 0,};
            new_15 =
                opt_malloc(::std::mem::size_of::<DhcpConfig>() as
                               libc::c_ulong) as *mut DhcpConfig;
            (*new_15).next = daemon.dhcp_conf;
            (*new_15).flags =
                if option == 272 {
                    2048
                } else { 0 } as libc::c_uint;
            (*new_15).hwaddr = 0 as *mut HwaddrConfig;
            (*new_15).netid = 0 as *mut DhcpNetIdList;
            (*new_15).filter = 0 as *mut DhcpNetId;
            (*new_15).clid = 0 as *mut libc::c_uchar;
            (*new_15).addr6 = 0 as *mut AddrList;
            while !arg.is_null() {
                comma = split(arg);
                if !strchr(arg, ':' as i32).is_null() {
                    /* ethernet address, netid or binary CLID */
                    if (*arg.offset(0)
                            == 'i' as i32 ||
                            *arg.offset(0) as
                                libc::c_int == 'I' as i32) &&
                           (*arg.offset(1) as
                                libc::c_int == 'd' as i32 ||
                                *arg.offset(1) as
                                    libc::c_int == 'D' as i32) &&
                           *arg.offset(2) as
                               libc::c_int == ':' as i32 {
                        if *arg.offset(3) as
                               libc::c_int == '*' as i32 {
                            (*new_15).flags |=
                                128 as libc::c_uint
                        } else {
                            let mut len_0: libc::c_int = 0; /* dump id: */
                            arg = arg.offset(3);
                            if !strchr(arg, ':' as i32).is_null() {
                                len_0 =
                                    parse_hex(arg, arg as *mut libc::c_uchar,
                                              -(1),
                                              0 as *mut libc::c_uint,
                                              0 as *mut libc::c_int)
                            } else {
                                unhide_metas(arg);
                                len_0 = strlen(arg)
                            }
                            if len_0 == -(1) {
                                dhcp_config_free(new_15);
                                strcpy(errstr,
                                       b"bad hex constant\x00" as *const u8 as
                                           *const libc::c_char);
                                return 0
                            } else {
                                (*new_15).clid =
                                    opt_malloc(len_0 as usize) as
                                        *mut libc::c_uchar;
                                if !(*new_15).clid.is_null() {
                                    (*new_15).flags |=
                                        2 as libc::c_uint;
                                    (*new_15).clid_len = len_0;
                                    memcpy((*new_15).clid as
                                               *mut libc::c_void,
                                           arg as *const libc::c_void,
                                           len_0 as libc::c_ulong);
                                }
                            }
                        }
                    } else if strstr(arg,
                                     b"net:\x00" as *const u8 as
                                         *const libc::c_char) == arg ||
                                  strstr(arg,
                                         b"set:\x00" as *const u8 as
                                             *const libc::c_char) == arg {
                        let mut newlist_0: *mut DhcpNetIdList =
                            opt_malloc(::std::mem::size_of::<DhcpNetIdList>()
                                           as libc::c_ulong) as
                                *mut DhcpNetIdList;
                        (*newlist_0).next = (*new_15).netid;
                        (*new_15).netid = newlist_0;
                        (*newlist_0).list =
                            dhcp_netid_create(arg.offset(4 as
                                                             isize),
                                              0 as *mut DhcpNetId)
                    } else if strstr(arg,
                                     b"tag:\x00" as *const u8 as
                                         *const libc::c_char) == arg {
                        (*new_15).filter =
                            dhcp_netid_create(arg.offset(4 as
                                                             isize),
                                              (*new_15).filter)
                    } else if *arg.offset(0) as
                                  libc::c_int == '[' as i32 &&
                                  *arg.offset(strlen(arg).wrapping_sub(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
                                                 ) ==
                                      ']' as i32 {
                        let mut pref_1: *mut libc::c_char =
                            0 as *mut libc::c_char;
                        let mut in6: In6Addr =
                            In6Addr {__in6_u:
                                         C2RustUnnamed{__u6_addr8:
                                                           [0; 16],},};
                        let mut new_addr: *mut AddrList = 0 as *mut AddrList;
                        *arg.offset(strlen(arg).wrapping_sub(1
                                                                 as
                                                                 libc::c_ulong)
                                       ) =
                            0 as libc::c_char;
                        arg = arg.offset(1);
                        pref_1 = split_chr(arg, '/' as i32 as libc::c_char);
                        if inet_pton(10, arg,
                                     &mut in6 as *mut In6Addr as
                                         *mut libc::c_void) == 0 {
                            dhcp_config_free(new_15);
                            strcpy(errstr,
                                   b"bad IPv6 address\x00" as *const u8 as
                                       *const libc::c_char);
                            return 0
                        }
                        new_addr =
                            opt_malloc(::std::mem::size_of::<AddrList>() as
                                           libc::c_ulong) as *mut AddrList;
                        (*new_addr).next = (*new_15).addr6;
                        (*new_addr).flags = 0;
                        (*new_addr).addr.addr6 = in6;
                        (*new_15).addr6 = new_addr;
                        if !pref_1.is_null() {
                            let mut addrpart_0: u64 = addr6part(&mut in6);
                            if atoi_check(pref_1, &mut (*new_addr).prefixlen)
                                   == 0 ||
                                   (*new_addr).prefixlen > 128
                                   ||
                                   ((1 as u64) <<
                                        128 -
                                            (*new_addr).prefixlen).wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulonglong)
                                       & addrpart_0 !=
                                       0 as libc::c_ulonglong {
                                dhcp_config_free(new_15);
                                strcpy(errstr,
                                       b"bad IPv6 prefix\x00" as *const u8 as
                                           *const libc::c_char);
                                return 0
                            }
                            (*new_addr).flags |= 8
                        }
                        i = 0;
                        while i < 8 {
                            if in6.__in6_u.__u6_addr8[i as usize] as
                                   libc::c_int != 0 {
                                break ;
                            }
                            i += 1
                        }
                        /* dhcp-host has strange backwards-compat needs. */
                        /* set WILDCARD if network part all zeros */
                        if i == 8 {
                            (*new_addr).flags |= 16
                        }
                        (*new_15).flags |= 4096 as libc::c_uint
                    } else {
                        let mut newhw: *mut HwaddrConfig =
                            opt_malloc(::std::mem::size_of::<HwaddrConfig>()
                                           as libc::c_ulong) as
                                *mut HwaddrConfig;
                        (*newhw).hwaddr_len =
                            parse_hex(arg, (*newhw).hwaddr.as_mut_ptr(),
                                      16,
                                      &mut (*newhw).wildcard_mask,
                                      &mut (*newhw).hwaddr_type);
                        if (*newhw).hwaddr_len == -(1) {
                            free(newhw as *mut libc::c_void);
                            dhcp_config_free(new_15);
                            strcpy(errstr,
                                   b"bad hex constant\x00" as *const u8 as
                                       *const libc::c_char);
                            return 0
                        } else {
                            (*newhw).next = (*new_15).hwaddr;
                            (*new_15).hwaddr = newhw
                        }
                    }
                } else if !strchr(arg, '.' as i32).is_null() &&
                              inet_pton(2, arg,
                                        &mut in_0 as *mut InAddr as
                                            *mut libc::c_void) >
                                  0 {
                    let mut configs: *mut DhcpConfig = 0 as *mut DhcpConfig;
                    (*new_15).addr = in_0;
                    (*new_15).flags |= 32 as libc::c_uint;
                    /* If the same IP appears in more than one host config, then DISCOVER
		   for one of the hosts will get the address, but REQUEST will be NAKed,
		   since the address is reserved by the other one -> protocol loop. */
                    configs = daemon.dhcp_conf;
                    while !configs.is_null() {
                        if (*configs).flags &
                               32 as libc::c_uint != 0 &&
                               (*configs).addr.s_addr == in_0.s_addr {
                            sprintf(errstr,
                                    b"duplicate dhcp-host IP address %s\x00"
                                        as *const u8 as *const libc::c_char,
                                    inet_ntoa(in_0));
                            return 0
                        }
                        configs = (*configs).next
                    }
                } else {
                    let mut cp_1: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut lastp: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut last: libc::c_char =
                        0 as libc::c_char;
                    let mut fac_0: libc::c_int = 1;
                    let mut isdig: libc::c_int = 0;
                    if strlen(arg) > 1 as libc::c_ulong {
                        lastp =
                            arg.offset(strlen(arg) as
                                           isize).offset(-(1 as
                                                               isize));
                        last = *lastp;
                        let mut current_block_1169: u64;
                        match last {
                            119 | 87 => {
                                fac_0 *= 7;
                                current_block_1169 = 16827258629745096341;
                            }
                            100 | 68 => {
                                current_block_1169 = 16827258629745096341;
                            }
                            104 | 72 => {
                                current_block_1169 = 1699689399587118340;
                            }
                            109 | 77 => {
                                current_block_1169 = 9134426092733397760;
                            }
                            115 | 83 => {
                                current_block_1169 = 13003683363839989667;
                            }
                            _ => {
                                current_block_1169 = 14492088476923213239;
                            }
                        }
                        match current_block_1169 {
                            16827258629745096341 =>
                            /* fall through */
                            {
                                fac_0 *= 24;
                                current_block_1169 = 1699689399587118340;
                            }
                            _ => { }
                        }
                        match current_block_1169 {
                            1699689399587118340 =>
                            /* fall through */
                            {
                                fac_0 *= 60;
                                current_block_1169 = 9134426092733397760;
                            }
                            _ => { }
                        }
                        match current_block_1169 {
                            9134426092733397760 =>
                            /* fall through */
                            {
                                fac_0 *= 60;
                                current_block_1169 = 13003683363839989667;
                            }
                            _ => { }
                        }
                        match current_block_1169 {
                            13003683363839989667 =>
                            /* fall through */
                            {
                                *lastp = 0 as libc::c_char
                            }
                            _ => { }
                        }
                    }
                    cp_1 = arg;
                    while cp_1 != 0 {
                        if *(*__ctype_b_loc()).offset(cp_1 as libc::c_uchar
                                                          as
                                                          isize) as
                               libc::c_int &
                               _ISDIGIT as libc::c_ushort as
                                   libc::c_int != 0 {
                            isdig = 1
                        } else if cp_1 != ' ' as i32 {
                            break ;
                        }
                        cp_1 = cp_1.offset(1)
                    }
                    if cp_1 != 0 {
                        if !lastp.is_null() { *lastp = last }
                        if strcmp(arg,
                                  b"infinite\x00" as *const u8 as
                                      *const libc::c_char) == 0
                           {
                            (*new_15).lease_time = 0xffffffff as libc::c_uint;
                            (*new_15).flags |=
                                8 as libc::c_uint
                        } else if strcmp(arg,
                                         b"ignore\x00" as *const u8 as
                                             *const libc::c_char) ==
                                      0 {
                            (*new_15).flags |=
                                1 as libc::c_uint
                        } else {
                            (*new_15).hostname = canonicalise_opt(arg);
                            if (*new_15).hostname.is_null() ||
                                   legal_hostname((*new_15).hostname) == 0 {
                                dhcp_config_free(new_15);
                                strcpy(errstr,
                                       b"bad DHCP host name\x00" as *const u8
                                           as *const libc::c_char);
                                return 0
                            }
                            (*new_15).flags |=
                                16 as libc::c_uint;
                            (*new_15).domain =
                                strip_hostname((*new_15).hostname)
                        }
                    } else if isdig != 0 {
                        (*new_15).lease_time =
                            (atoi(arg) * fac_0) as libc::c_uint;
                        /* Leases of a minute or less confuse
		       some clients, notably Apple's */
                        if (*new_15).lease_time <
                               120 as libc::c_uint {
                            (*new_15).lease_time =
                                120 as libc::c_uint
                        }
                        (*new_15).flags |= 8 as libc::c_uint
                    }
                }
                arg = comma
            }
            daemon.dhcp_conf = new_15;
            current_block = 7879481898411272068;
        }
        294 => {
            /* --tag-if */
            let mut new_16: *mut TagIf =
                opt_malloc(::std::mem::size_of::<TagIf>() as libc::c_ulong)
                    as *mut TagIf;
            (*new_16).tag = 0 as *mut DhcpNetId;
            (*new_16).set = 0 as *mut DhcpNetIdList;
            (*new_16).next = 0 as *mut TagIf;
            /* preserve order */
            if daemon.tag_if.is_null() {
                daemon.tag_if = new_16
            } else {
                let mut tmp_2: *mut TagIf = 0 as *mut TagIf;
                tmp_2 = daemon.tag_if;
                while !(*tmp_2).next.is_null() { tmp_2 = (*tmp_2).next }
                (*tmp_2).next = new_16
            }
            while !arg.is_null() {
                let mut len_1: usize = 0;
                comma = split(arg);
                len_1 = strlen(arg);
                if len_1 < 5 as libc::c_ulong {
                    (*new_16).set = 0 as *mut DhcpNetIdList;
                    break ;
                } else {
                    let mut newtag: *mut DhcpNetId =
                        dhcp_netid_create(arg.offset(4 as
                                                         isize),
                                          0 as *mut DhcpNetId);
                    if strstr(arg,
                              b"set:\x00" as *const u8 as *const libc::c_char)
                           == arg {
                        let mut newlist_1: *mut DhcpNetIdList =
                            opt_malloc(::std::mem::size_of::<DhcpNetIdList>()
                                           as libc::c_ulong) as
                                *mut DhcpNetIdList;
                        (*newlist_1).next = (*new_16).set;
                        (*new_16).set = newlist_1;
                        (*newlist_1).list = newtag
                    } else if strstr(arg,
                                     b"tag:\x00" as *const u8 as
                                         *const libc::c_char) == arg {
                        (*newtag).next = (*new_16).tag;
                        (*new_16).tag = newtag
                    } else {
                        (*new_16).set = 0 as *mut DhcpNetIdList;
                        dhcp_netid_free(newtag);
                        break ;
                    }
                    arg = comma
                }
            }
            if (*new_16).set.is_null() {
                dhcp_netid_free((*new_16).tag);
                dhcp_netid_list_free((*new_16).set);
                strcpy(errstr,
                       b"bad tag-if\x00" as *const u8 as *const libc::c_char);
                free(new_16 as *mut libc::c_void);
                return 0
            }
            current_block = 7879481898411272068;
        }
        79 => {
            /* --dhcp-option */
            current_block = 18295461473828413614;
        }
        264 | 279 | 281 => { current_block = 18295461473828413614; }
        355 => {
            /* --dhcp-name-match */
            let mut new_17: *mut DhcpMatchName =
                opt_malloc(::std::mem::size_of::<DhcpMatchName>() as
                               libc::c_ulong) as *mut DhcpMatchName;
            let mut id: *mut DhcpNetId =
                opt_malloc(::std::mem::size_of::<DhcpNetId>() as
                               libc::c_ulong) as *mut DhcpNetId;
            let mut len_2: susize = 0;
            comma = split(arg);
            if comma.is_null() ||
                   {
                       len_2 = strlen(comma) as susize;
                       (len_2) == 0 as libc::c_long
                   } {
                strcpy(errstr, gen_err);
                return 0
            }
            (*new_17).wildcard = 0;
            (*new_17).netid = id;
            (*id).net = opt_string_alloc(set_prefix(arg));
            if *comma.offset((len_2 - 1 as libc::c_long) as
                                 isize) == '*' as i32 {
                *comma.offset((len_2 - 1 as libc::c_long) as
                                  isize) = 0 as libc::c_char;
                (*new_17).wildcard = 1
            }
            (*new_17).name = opt_string_alloc(comma);
            (*new_17).next = daemon.dhcp_name_match;
            daemon.dhcp_name_match = new_17;
            current_block = 7879481898411272068;
        }
        77 => {
            /* --dhcp-boot */
            let mut id_0: *mut DhcpNetId = dhcp_tags(&mut arg);
            if arg.is_null() {
                strcpy(errstr, gen_err);
                return 0
            } else {
                let mut dhcp_file: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut dhcp_sname: *mut libc::c_char =
                    0 as *mut libc::c_char;
                let mut tftp_sname: *mut libc::c_char =
                    0 as *mut libc::c_char;
                let mut dhcp_next_server: InAddr = InAddr {s_addr: 0,};
                let mut new_18: *mut DhcpBoot = 0 as *mut DhcpBoot;
                comma = split(arg);
                dhcp_file = opt_string_alloc(arg);
                dhcp_next_server.s_addr = 0 as InAddrT;
                if !comma.is_null() {
                    arg = comma;
                    comma = split(arg);
                    dhcp_sname = opt_string_alloc(arg);
                    if !comma.is_null() {
                        unhide_metas(comma);
                        if !(inet_pton(2, comma,
                                       &mut dhcp_next_server as *mut InAddr
                                           as *mut libc::c_void) >
                                 0) {
                            /*
			 * The user may have specified the tftp hostname here.
			 * save it so that it can be resolved/looked up during
			 * actual dhcp_reply().
			 */
                            tftp_sname = opt_string_alloc(comma);
                            dhcp_next_server.s_addr =
                                0 as InAddrT
                        }
                    }
                }
                new_18 =
                    opt_malloc(::std::mem::size_of::<DhcpBoot>() as
                                   libc::c_ulong) as *mut DhcpBoot;
                (*new_18).file = dhcp_file;
                (*new_18).sname = dhcp_sname;
                (*new_18).tftp_sname = tftp_sname;
                (*new_18).next_server = dhcp_next_server;
                (*new_18).netid = id_0;
                (*new_18).next = daemon.boot_config;
                daemon.boot_config = new_18
            }
            current_block = 7879481898411272068;
        }
        350 => {
            /* --dhcp-reply-delay */
            let mut id_1: *mut DhcpNetId = dhcp_tags(&mut arg);
            if arg.is_null() {
                strcpy(errstr, gen_err);
                return 0
            } else {
                let mut new_19: *mut DelayConfig = 0 as *mut DelayConfig;
                let mut delay: libc::c_int = 0;
                if atoi_check(arg, &mut delay) == 0 {
                    strcpy(errstr, gen_err);
                    return 0
                }
                new_19 =
                    opt_malloc(::std::mem::size_of::<DelayConfig>() as
                                   libc::c_ulong) as *mut DelayConfig;
                (*new_19).delay = delay;
                (*new_19).netid = id_1;
                (*new_19).next = daemon.delay_conf;
                daemon.delay_conf = new_19
            }
            current_block = 7879481898411272068;
        }
        291 => {
            /* --pxe-prompt */
            let mut new_20: *mut DhcpOpt =
                opt_malloc(::std::mem::size_of::<DhcpOpt>() as libc::c_ulong)
                    as *mut DhcpOpt; /* PXE_MENU_PROMPT */
            let mut timeout: libc::c_int = 0;
            (*new_20).netid = 0 as *mut DhcpNetId;
            (*new_20).opt = 10;
            (*new_20).netid = dhcp_tags(&mut arg);
            if arg.is_null() {
                dhcp_opt_free(new_20);
                strcpy(errstr, gen_err);
                return 0
            } else {
                comma = split(arg);
                unhide_metas(arg);
                (*new_20).len =
                    strlen(arg).wrapping_add(1 as
                                                 libc::c_ulong) as
                        libc::c_int;
                (*new_20).val =
                    opt_malloc((*new_20).len as usize) as *mut libc::c_uchar;
                memcpy((*new_20).val.offset(1) as
                           *mut libc::c_void, arg as *const libc::c_void,
                       ((*new_20).len - 1) as libc::c_ulong);
                (*new_20).u.vendor_class = 0 as *mut libc::c_uchar;
                (*new_20).flags = 256 | 16384;
                if !comma.is_null() && atoi_check(comma, &mut timeout) != 0 {
                    *(*new_20).val = timeout as libc::c_uchar
                } else {
                    *(*new_20).val = 255 as libc::c_uchar
                }
                (*new_20).next = daemon.dhcp_opts;
                daemon.dhcp_opts = new_20;
                daemon.enable_pxe = 1
            }
            current_block = 7879481898411272068;
        }
        292 => {
            /* --pxe-service */
            let mut new_21: *mut PxeService =
                opt_malloc(::std::mem::size_of::<PxeService>() as
                               libc::c_ulong) as
                    *mut PxeService; /* local boot */
            let mut CSA: [*mut libc::c_char; 13] =
                [b"x86PC\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"PC98\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"IA64_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"Alpha\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"Arc_x86\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"Intel_Lean_Client\x00" as *const u8 as *const libc::c_char
                     as *mut libc::c_char,
                 b"IA32_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"x86-64_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"Xscale_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"BC_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"ARM32_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 b"ARM64_EFI\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char, 0 as *mut libc::c_char];
            static mut boottype: libc::c_int = 32768;
            (*new_21).netid = 0 as *mut DhcpNetId;
            (*new_21).sname = 0 as *mut libc::c_char;
            (*new_21).server.s_addr = 0 as InAddrT;
            (*new_21).netid = dhcp_tags(&mut arg);
            if !arg.is_null() && { comma = split(arg); !comma.is_null() } {
                i = 0;
                while !CSA[i as usize].is_null() {
                    if strcasecmp(CSA[i as usize], arg) == 0 {
                        break ;
                    }
                    i += 1
                }
                if !CSA[i as usize].is_null() || atoi_check(arg, &mut i) != 0
                   {
                    arg = comma;
                    comma = split(arg);
                    (*new_21).csa = i as libc::c_ushort;
                    (*new_21).menu = opt_string_alloc(arg);
                    if comma.is_null() {
                        (*new_21).type_0 = 0 as libc::c_ushort;
                        (*new_21).basename = 0 as *mut libc::c_char
                    } else {
                        arg = comma;
                        comma = split(arg);
                        if atoi_check(arg, &mut i) != 0 {
                            (*new_21).type_0 = i as libc::c_ushort;
                            (*new_21).basename = 0 as *mut libc::c_char
                        } else {
                            let fresh28 = boottype;
                            boottype = boottype + 1;
                            (*new_21).type_0 = fresh28 as libc::c_ushort;
                            (*new_21).basename = opt_string_alloc(arg)
                        }
                        if !comma.is_null() {
                            if inet_pton(2, comma,
                                         &mut (*new_21).server as *mut InAddr
                                             as *mut libc::c_void) == 0 {
                                (*new_21).server.s_addr =
                                    0 as InAddrT;
                                (*new_21).sname = opt_string_alloc(comma)
                            }
                        }
                    }
                    /* Order matters */
                    (*new_21).next = 0 as *mut PxeService;
                    if daemon.pxe_services.is_null() {
                        daemon.pxe_services = new_21
                    } else {
                        let mut s: *mut PxeService = 0 as *mut PxeService;
                        s = daemon.pxe_services;
                        while !(*s).next.is_null() { s = (*s).next }
                        (*s).next = new_21
                    }
                    daemon.enable_pxe = 1;
                    current_block = 7879481898411272068;
                } else { current_block = 6421703339113101262; }
            } else { current_block = 6421703339113101262; }
            match current_block {
                7879481898411272068 => { }
                _ => { strcpy(errstr, gen_err); return 0 }
            }
        }
        52 => {
            /* --dhcp-mac */
            comma = split(arg);
            if comma.is_null() {
                strcpy(errstr, gen_err);
                return 0
            } else {
                let mut new_22: *mut DhcpMac =
                    opt_malloc(::std::mem::size_of::<DhcpMac>() as
                                   libc::c_ulong) as *mut DhcpMac;
                (*new_22).netid.net = opt_string_alloc(set_prefix(arg));
                unhide_metas(comma);
                (*new_22).hwaddr_len =
                    parse_hex(comma, (*new_22).hwaddr.as_mut_ptr(),
                              16, &mut (*new_22).mask,
                              &mut (*new_22).hwaddr_type);
                if (*new_22).hwaddr_len == -(1) {
                    free((*new_22).netid.net as *mut libc::c_void);
                    strcpy(errstr, gen_err);
                    free(new_22 as *mut libc::c_void);
                    return 0
                } else {
                    (*new_22).next = daemon.dhcp_macs;
                    daemon.dhcp_macs = new_22
                }
            }
            current_block = 7879481898411272068;
        }
        85 => {
            /* --dhcp-vendorclass */
            current_block = 10375845272849059847;
        }
        106 => { current_block = 10375845272849059847; }
        268 => { current_block = 17332795835978703980; }
        269 => { current_block = 15503158355981179141; }
        270 => { current_block = 9763990383449182594; }
        284 => {
            /* --dhcp-alternate-port */
            if arg.is_null() {
                daemon.dhcp_server_port = 1067;
                daemon.dhcp_client_port = 1068
            } else {
                comma = split(arg);
                if atoi_check16(arg, &mut daemon.dhcp_server_port)
                       == 0 ||
                       !comma.is_null() &&
                           atoi_check16(comma,
                                        &mut daemon.dhcp_client_port)
                               == 0 {
                    strcpy(errstr,
                           b"invalid port number\x00" as *const u8 as
                               *const libc::c_char);
                    return 0
                }
                if comma.is_null() {
                    daemon.dhcp_client_port =
                        daemon.dhcp_server_port + 1
                }
            }
            current_block = 7879481898411272068;
        }
        74 => {
            /* --dhcp-ignore */
            current_block = 8728755645494476224;
        }
        257 => { current_block = 8728755645494476224; }
        282 => { current_block = 9783966086509161201; }
        51 => { current_block = 9535337827963792624; }
        296 => { current_block = 8762260891357387630; }
        295 => {
            /* --dhcp-proxy */
            daemon.override_0 = 1;
            while !arg.is_null() {
                let mut new_25: *mut AddrList2 =
                    opt_malloc(::std::mem::size_of::<AddrList2>() as
                                   libc::c_ulong) as *mut AddrList2;
                comma = split(arg);
                if !(inet_pton(2, arg,
                               &mut (*new_25).addr as *mut InAddr as
                                   *mut libc::c_void) > 0) {
                    strcpy(errstr,
                           b"bad dhcp-proxy address\x00" as *const u8 as
                               *const libc::c_char);
                    free(new_25 as *mut libc::c_void);
                    return 0
                }
                (*new_25).next = daemon.override_relays;
                daemon.override_relays = new_25;
                arg = comma
            }
            current_block = 7879481898411272068;
        }
        361 => {
            /* --dhcp-pxe-vendor */
            while !arg.is_null() {
                let mut new_26: *mut DhcpPxeVendor =
                    opt_malloc(::std::mem::size_of::<DhcpPxeVendor>() as
                                   libc::c_ulong) as *mut DhcpPxeVendor;
                comma = split(arg);
                (*new_26).data = opt_string_alloc(arg);
                (*new_26).next = daemon.dhcp_pxe_vendors;
                daemon.dhcp_pxe_vendors = new_26;
                arg = comma
            }
            current_block = 7879481898411272068;
        }
        323 => {
            /* --dhcp-relay */
            let mut new_27: *mut DhcpRelay =
                opt_malloc(::std::mem::size_of::<DhcpRelay>() as
                               libc::c_ulong) as *mut DhcpRelay;
            comma = split(arg);
            (*new_27).interface = opt_string_alloc(split(comma));
            (*new_27).iface_index = 0;
            if inet_pton(2, arg,
                         &mut (*new_27).local as *mut AllAddr as
                             *mut libc::c_void) != 0 &&
                   inet_pton(2, comma,
                             &mut (*new_27).server as *mut AllAddr as
                                 *mut libc::c_void) != 0 {
                (*new_27).next = daemon.relay4;
                daemon.relay4 = new_27
            } else if inet_pton(10, arg,
                                &mut (*new_27).local as *mut AllAddr as
                                    *mut libc::c_void) != 0 &&
                          inet_pton(10, comma,
                                    &mut (*new_27).server as *mut AllAddr as
                                        *mut libc::c_void) != 0 {
                (*new_27).next = daemon.relay6;
                daemon.relay6 = new_27
            } else {
                free((*new_27).interface as *mut libc::c_void);
                strcpy(errstr,
                       b"Bad dhcp-relay\x00" as *const u8 as
                           *const libc::c_char);
                free(new_27 as *mut libc::c_void);
                return 0
            }
            current_block = 7879481898411272068;
        }
        324 => {
            /* --ra-param */
            comma = split(arg);
            if !comma.is_null() {
                let mut new_28: *mut RaInterface =
                    opt_malloc(::std::mem::size_of::<RaInterface>() as
                                   libc::c_ulong) as *mut RaInterface;
                (*new_28).lifetime = -(1);
                (*new_28).prio = 0;
                (*new_28).mtu = 0;
                (*new_28).mtu_name = 0 as *mut libc::c_char;
                (*new_28).name = opt_string_alloc(arg);
                if strcasestr(comma,
                              b"mtu:\x00" as *const u8 as *const libc::c_char)
                       == comma {
                    arg = comma.offset(4);
                    comma = split(comma);
                    if comma.is_null() {
                        current_block = 14730872864422895907;
                    } else if strcasecmp(arg,
                                         b"off\x00" as *const u8 as
                                             *const libc::c_char) == 0 {
                        (*new_28).mtu = -(1);
                        current_block = 1840194652026069277;
                    } else if atoi_check(arg, &mut (*new_28).mtu) == 0 {
                        (*new_28).mtu_name = opt_string_alloc(arg);
                        current_block = 1840194652026069277;
                    } else if (*new_28).mtu < 1280 {
                        current_block = 14730872864422895907;
                    } else { current_block = 1840194652026069277; }
                } else { current_block = 1840194652026069277; }
                match current_block {
                    1840194652026069277 => {
                        if strcasestr(comma,
                                      b"high\x00" as *const u8 as
                                          *const libc::c_char) == comma ||
                               strcasestr(comma,
                                          b"low\x00" as *const u8 as
                                              *const libc::c_char) == comma {
                            if *comma == 'l' as i32 ||
                                   *comma == 'L' as i32 {
                                (*new_28).prio = 0x18
                            } else { (*new_28).prio = 0x8 }
                            comma = split(comma)
                        }
                        arg = split(comma);
                        if atoi_check(comma, &mut (*new_28).interval) == 0 ||
                               !arg.is_null() &&
                                   atoi_check(arg, &mut (*new_28).lifetime) ==
                                       0 {
                            current_block = 14730872864422895907;
                        } else {
                            (*new_28).next = daemon.ra_interfaces;
                            daemon.ra_interfaces = new_28;
                            current_block = 7879481898411272068;
                        }
                    }
                    _ => { }
                }
                match current_block {
                    7879481898411272068 => { }
                    _ => {
                        free((*new_28).name as *mut libc::c_void);
                        strcpy(errstr,
                               b"bad RA-params\x00" as *const u8 as
                                   *const libc::c_char);
                        free(new_28 as *mut libc::c_void);
                        return 0
                    }
                }
            } else { current_block = 7879481898411272068; }
        }
        307 => {
            /* --dhcp-duid */
            comma = split(arg);
            if comma.is_null() ||
                   atoi_check(arg,
                              &mut daemon.duid_enterprise as
                                  *mut libc::c_uint as *mut libc::c_int) == 0
               {
                strcpy(errstr,
                       b"bad DUID\x00" as *const u8 as *const libc::c_char);
                return 0
            } else {
                daemon.duid_config_len =
                    parse_hex(comma, comma as *mut libc::c_uchar,
                              strlen(comma),
                              0 as *mut libc::c_uint, 0 as *mut libc::c_int)
                        as libc::c_uint;
                daemon.duid_config =
                    opt_malloc(daemon.duid_config_len as usize) as
                        *mut libc::c_uchar;
                memcpy(daemon.duid_config as *mut libc::c_void,
                       comma as *const libc::c_void,
                       daemon.duid_config_len as libc::c_ulong);
            }
            current_block = 7879481898411272068;
        }
        86 => {
            /* --alias */
            let mut dash: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut a_0: [*mut libc::c_char; 3] =
                [0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char];
            let mut k_0: libc::c_int = 0;
            let mut new_29: *mut Doctor =
                opt_malloc(::std::mem::size_of::<Doctor>() as libc::c_ulong)
                    as *mut Doctor;
            (*new_29).next = daemon.doctors;
            daemon.doctors = new_29;
            (*new_29).mask.s_addr = 0xffffffff as libc::c_uint;
            (*new_29).end.s_addr = 0 as InAddrT;
            a_0[0 as usize] = arg;
            if !a_0[0 as usize].is_null() {
                k_0 = 1;
                while k_0 < 3 {
                    a_0[k_0 as usize] =
                        split(a_0[(k_0 - 1) as usize]);
                    if a_0[k_0 as usize].is_null() { break ; }
                    unhide_metas(a_0[k_0 as usize]);
                    k_0 += 1
                }
            }
            dash =
                split_chr(a_0[0 as usize],
                          '-' as i32 as libc::c_char);
            if k_0 < 2 ||
                   !(inet_pton(2,
                               a_0[0 as usize],
                               &mut (*new_29).in_0 as *mut InAddr as
                                   *mut libc::c_void) > 0) ||
                   !(inet_pton(2,
                               a_0[1 as usize],
                               &mut (*new_29).out as *mut InAddr as
                                   *mut libc::c_void) > 0) ||
                   k_0 == 3 &&
                       inet_pton(2,
                                 a_0[2 as usize],
                                 &mut (*new_29).mask as *mut InAddr as
                                     *mut libc::c_void) == 0 {
                strcpy(errstr,
                       b"missing address in alias\x00" as *const u8 as
                           *const libc::c_char);
                return 0
            }
            if !dash.is_null() &&
                   (!(inet_pton(2, dash,
                                &mut (*new_29).end as *mut InAddr as
                                    *mut libc::c_void) > 0) ||
                        is_same_net((*new_29).in_0, (*new_29).end,
                                    (*new_29).mask) == 0 ||
                        __bswap_32((*new_29).in_0.s_addr) >
                            __bswap_32((*new_29).end.s_addr)) {
                strcpy(errstr,
                       b"invalid alias range\x00" as *const u8 as
                           *const libc::c_char);
                free(new_29 as *mut libc::c_void);
                return 0
            }
            current_block = 7879481898411272068;
        }
        271 => {
            /* --interface-name */
            let mut new_30: *mut InterfaceName = 0 as *mut InterfaceName;
            let mut up_0: *mut *mut InterfaceName =
                0 as *mut *mut InterfaceName;
            let mut domain_1: *mut libc::c_char = 0 as *mut libc::c_char;
            comma = split(arg);
            if comma.is_null() ||
                   { domain_1 = canonicalise_opt(arg); domain_1.is_null() } {
                strcpy(errstr,
                       b"bad interface name\x00" as *const u8 as
                           *const libc::c_char);
                return 0
            }
            new_30 =
                opt_malloc(::std::mem::size_of::<InterfaceName>() as
                               libc::c_ulong) as *mut InterfaceName;
            (*new_30).next = 0 as *mut InterfaceName;
            (*new_30).addr = 0 as *mut AddrList;
            /* Add to the end of the list, so that first name
	   of an interface is used for PTR lookups. */
            up_0 = &mut daemon.int_names;
            while !(*up_0).is_null() { up_0 = &mut (**up_0).next }
            *up_0 = new_30;
            (*new_30).name = domain_1;
            (*new_30).family = 0;
            arg = split_chr(comma, '/' as i32 as libc::c_char);
            if !arg.is_null() {
                if strcmp(arg, b"4\x00" as *const u8 as *const libc::c_char)
                       == 0 {
                    (*new_30).family = 2
                } else if strcmp(arg,
                                 b"6\x00" as *const u8 as *const libc::c_char)
                              == 0 {
                    (*new_30).family = 10
                } else {
                    strcpy(errstr, gen_err);
                    free(new_30 as *mut libc::c_void);
                    return 0
                }
            }
            (*new_30).intr = opt_string_alloc(comma);
            current_block = 7879481898411272068;
        }
        290 => {
            /* --cname */
            let mut new_31: *mut Cname = 0 as *mut Cname;
            let mut alias: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut target_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut last_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut pen: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut ttl_0: libc::c_int = -(1);
            pen = 0 as *mut libc::c_char;
            last_0 = pen;
            comma = arg;
            while !comma.is_null() {
                pen = last_0;
                last_0 = comma;
                comma = split(comma)
            }
            if pen.is_null() {
                strcpy(errstr,
                       b"bad CNAME\x00" as *const u8 as *const libc::c_char);
                return 0
            }
            if pen != arg && atoi_check(last_0, &mut ttl_0) != 0 {
                last_0 = pen
            }
            target_0 = canonicalise_opt(last_0);
            while arg != last_0 {
                let mut arglen: libc::c_int = strlen(arg);
                alias = canonicalise_opt(arg);
                if alias.is_null() || target_0.is_null() {
                    free(target_0 as *mut libc::c_void);
                    free(alias as *mut libc::c_void);
                    strcpy(errstr,
                           b"bad CNAME\x00" as *const u8 as
                               *const libc::c_char);
                    return 0
                }
                new_31 = daemon.cnames;
                while !new_31.is_null() {
                    if hostname_isequal((*new_31).alias, alias) != 0 {
                        free(target_0 as *mut libc::c_void);
                        free(alias as *mut libc::c_void);
                        strcpy(errstr,
                               b"duplicate CNAME\x00" as *const u8 as
                                   *const libc::c_char);
                        return 0
                    }
                    new_31 = (*new_31).next
                }
                new_31 =
                    opt_malloc(::std::mem::size_of::<Cname>() as
                                   libc::c_ulong) as *mut Cname;
                (*new_31).next = daemon.cnames;
                daemon.cnames = new_31;
                (*new_31).alias = alias;
                (*new_31).target = target_0;
                (*new_31).ttl = ttl_0;
                arg = arg.offset((arglen + 1));
                while *arg != 0 &&
                          *(*__ctype_b_loc()).offset(*arg as
                                                         isize)
                              &
                              _ISSPACE as libc::c_ushort as
                                  libc::c_int != 0 {
                    arg = arg.offset(1)
                }
            }
            current_block = 7879481898411272068;
        }
        261 => {
            /* --ptr-record */
            let mut new_32: *mut PtrRecord = 0 as *mut PtrRecord;
            let mut dom: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut target_1: *mut libc::c_char = 0 as *mut libc::c_char;
            comma = split(arg);
            dom = canonicalise_opt(arg);
            if dom.is_null() ||
                   !comma.is_null() &&
                       {
                           target_1 = canonicalise_opt(comma);
                           target_1.is_null()
                       } {
                free(dom as *mut libc::c_void);
                free(target_1 as *mut libc::c_void);
                strcpy(errstr,
                       b"bad PTR record\x00" as *const u8 as
                           *const libc::c_char);
                return 0
            } else {
                new_32 =
                    opt_malloc(::std::mem::size_of::<PtrRecord>() as
                                   libc::c_ulong) as *mut PtrRecord;
                (*new_32).next = daemon.ptr;
                daemon.ptr = new_32;
                (*new_32).name = dom;
                (*new_32).ptr = target_1
            }
            current_block = 7879481898411272068;
        }
        287 => {
            /* --naptr-record */
            let mut a_1: [*mut libc::c_char; 7] =
                [0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char, 0 as *mut libc::c_char,
                 0 as *mut libc::c_char];
            let mut k_1: libc::c_int = 0;
            let mut new_33: *mut NaPtr = 0 as *mut NaPtr;
            let mut order: libc::c_int = 0;
            let mut pref_2: libc::c_int = 0;
            let mut name_2: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut replace: *mut libc::c_char = 0 as *mut libc::c_char;
            a_1[0 as usize] = arg;
            if !a_1[0 as usize].is_null() {
                k_1 = 1;
                while k_1 < 7 {
                    a_1[k_1 as usize] =
                        split(a_1[(k_1 - 1) as usize]);
                    if a_1[k_1 as usize].is_null() { break ; }
                    k_1 += 1
                }
            }
            if k_1 < 6 ||
                   {
                       name_2 =
                           canonicalise_opt(a_1[0 as usize]);
                       name_2.is_null()
                   } ||
                   atoi_check16(a_1[1 as usize], &mut order) ==
                       0 ||
                   atoi_check16(a_1[2 as usize], &mut pref_2)
                       == 0 ||
                   k_1 == 7 &&
                       {
                           replace =
                               canonicalise_opt(a_1[6 as
                                                        usize]);
                           replace.is_null()
                       } {
                free(name_2 as *mut libc::c_void);
                free(replace as *mut libc::c_void);
                strcpy(errstr,
                       b"bad NAPTR record\x00" as *const u8 as
                           *const libc::c_char);
                return 0
            } else {
                new_33 =
                    opt_malloc(::std::mem::size_of::<NaPtr>() as
                                   libc::c_ulong) as *mut NaPtr;
                (*new_33).next = daemon.naptr;
                daemon.naptr = new_33;
                (*new_33).name = name_2;
                (*new_33).flags =
                    opt_string_alloc(a_1[3 as usize]);
                (*new_33).services =
                    opt_string_alloc(a_1[4 as usize]);
                (*new_33).regexp =
                    opt_string_alloc(a_1[5 as usize]);
                (*new_33).replace = replace;
                (*new_33).order = order as libc::c_uint;
                (*new_33).pref = pref_2 as libc::c_uint
            }
            current_block = 7879481898411272068;
        }
        310 => {
            /* dns-rr */
            let mut new_34: *mut TxtRecord = 0 as *mut TxtRecord;
            let mut len_3: usize = 0 as usize;
            let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut class: libc::c_int = 0;
            comma = split(arg);
            data = split(comma);
            new_34 =
                opt_malloc(::std::mem::size_of::<TxtRecord>() as
                               libc::c_ulong) as *mut TxtRecord;
            (*new_34).name = 0 as *mut libc::c_char;
            if atoi_check(comma, &mut class) == 0 ||
                   {
                       (*new_34).name = canonicalise_opt(arg);
                       (*new_34).name.is_null()
                   } ||
                   !data.is_null() &&
                       {
                           len_3 =
                               parse_hex(data, data as *mut libc::c_uchar,
                                         -(1),
                                         0 as *mut libc::c_uint,
                                         0 as *mut libc::c_int) as usize;
                           (len_3) ==
                               (1 as libc::c_uint).wrapping_neg() as
                                   libc::c_ulong
                       } {
                free((*new_34).name as *mut libc::c_void);
                strcpy(errstr,
                       b"bad RR record\x00" as *const u8 as
                           *const libc::c_char);
                free(new_34 as *mut libc::c_void);
                return 0
            }
            (*new_34).len = 0 as libc::c_ushort;
            (*new_34).class = class as libc::c_ushort;
            (*new_34).next = daemon.rr;
            daemon.rr = new_34;
            if !data.is_null() {
                (*new_34).txt = opt_malloc(len_3) as *mut libc::c_uchar;
                (*new_34).len = len_3 as libc::c_ushort;
                memcpy((*new_34).txt as *mut libc::c_void,
                       data as *const libc::c_void, len_3);
            }
            current_block = 7879481898411272068;
        }
        356 => {
            /* --caa-record */
            let mut new_35: *mut TxtRecord = 0 as *mut TxtRecord;
            let mut tag: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut flags: libc::c_int = 0;
            comma = split(arg);
            tag = split(comma);
            value = split(tag);
            new_35 =
                opt_malloc(::std::mem::size_of::<TxtRecord>() as
                               libc::c_ulong) as *mut TxtRecord;
            (*new_35).next = daemon.rr;
            daemon.rr = new_35;
            if atoi_check(comma, &mut flags) == 0 || tag.is_null() ||
                   value.is_null() ||
                   {
                       (*new_35).name = canonicalise_opt(arg);
                       (*new_35).name.is_null()
                   } {
                strcpy(errstr,
                       b"bad CAA record\x00" as *const u8 as
                           *const libc::c_char);
                return 0
            }
            unhide_metas(tag);
            unhide_metas(value);
            (*new_35).len =
                strlen(tag).wrapping_add(strlen(value)).wrapping_add(2 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                    as libc::c_ushort;
            (*new_35).txt =
                opt_malloc((*new_35).len as usize) as *mut libc::c_uchar;
            *(*new_35).txt.offset(0) =
                flags as libc::c_uchar;
            *(*new_35).txt.offset(1) =
                strlen(tag) as libc::c_uchar;
            memcpy(&mut *(*new_35).txt.offset(2) as
                       *mut libc::c_uchar as *mut libc::c_void,
                   tag as *const libc::c_void, strlen(tag));
            memcpy(&mut *(*new_35).txt.offset((2 as
                                                   libc::c_ulong).wrapping_add((strlen
                                                                                    as
                                                                                     fn(_:
                                                                                                             *const libc::c_char)
                                                                                        ->
                                                                                            libc::c_ulong)(tag))
                                                 ) as
                       *mut libc::c_uchar as *mut libc::c_void,
                   value as *const libc::c_void, strlen(value));
            (*new_35).class = 257 as libc::c_ushort;
            current_block = 7879481898411272068;
        }
        89 => {
            /* --txt-record */
            let mut new_36: *mut TxtRecord =
                0 as *mut TxtRecord; /* room for extra counts */
            let mut p_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut cnt: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut len_4: usize = 0;
            comma = split(arg);
            new_36 =
                opt_malloc(::std::mem::size_of::<TxtRecord>() as
                               libc::c_ulong) as *mut TxtRecord;
            (*new_36).class = 1 as libc::c_ushort;
            (*new_36).stat = 0;
            (*new_36).name = canonicalise_opt(arg);
            if (*new_36).name.is_null() {
                strcpy(errstr,
                       b"bad TXT record\x00" as *const u8 as
                           *const libc::c_char);
                free(new_36 as *mut libc::c_void);
                return 0
            }
            (*new_36).next = daemon.txt;
            daemon.txt = new_36;
            len_4 =
                if !comma.is_null() {
                    strlen(comma)
                } else { 0 as libc::c_ulong };
            len_4 =
                (len_4 as
                     libc::c_ulong).wrapping_add(len_4.wrapping_div(255 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong).wrapping_add(1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong))
                    as usize as usize;
            p_0 = opt_malloc(len_4) as *mut libc::c_uchar;
            (*new_36).txt = p_0;
            let fresh29 = p_0;
            p_0 = p_0.offset(1);
            cnt = fresh29;
            *cnt = 0 as libc::c_uchar;
            while !comma.is_null() && *comma != 0 {
                let fresh30 = comma;
                comma = comma.offset(1);
                let mut c: libc::c_uchar = *fresh30 as libc::c_uchar;
                if c == ',' as i32 ||
                       *cnt == 255 {
                    if c != ',' as i32 {
                        comma = comma.offset(-1)
                    }
                    let fresh31 = p_0;
                    p_0 = p_0.offset(1);
                    cnt = fresh31;
                    *cnt = 0 as libc::c_uchar
                } else {
                    let fresh32 = p_0;
                    p_0 = p_0.offset(1);
                    *fresh32 =
                        unhide_meta(c as libc::c_char) as libc::c_uchar;
                    *cnt = (*cnt).wrapping_add(1)
                }
            }
            (*new_36).len =
                p_0.wrapping_offset_from((*new_36).txt) as libc::c_long as
                    libc::c_ushort;
            current_block = 7879481898411272068;
        }
        87 => {
            /* --srv-host */
            let mut port: libc::c_int = 1;
            let mut priority: libc::c_int = 0;
            let mut weight: libc::c_int = 0;
            let mut name_3: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut target_2: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut new_37: *mut MxSrvRecord = 0 as *mut MxSrvRecord;
            comma = split(arg);
            name_3 = canonicalise_opt(arg);
            if name_3.is_null() {
                strcpy(errstr,
                       b"bad SRV record\x00" as *const u8 as
                           *const libc::c_char);
                return 0
            }
            if !comma.is_null() {
                arg = comma;
                comma = split(arg);
                target_2 = canonicalise_opt(arg);
                if target_2.is_null() {
                    strcpy(errstr,
                           b"bad SRV target\x00" as *const u8 as
                               *const libc::c_char);
                    free(name_3 as *mut libc::c_void);
                    return 0
                }
                if !comma.is_null() {
                    arg = comma;
                    comma = split(arg);
                    if atoi_check16(arg, &mut port) == 0 {
                        free(name_3 as *mut libc::c_void);
                        strcpy(errstr,
                               b"invalid port number\x00" as *const u8 as
                                   *const libc::c_char);
                        free(target_2 as *mut libc::c_void);
                        return 0
                    }
                    if !comma.is_null() {
                        arg = comma;
                        comma = split(arg);
                        if atoi_check16(arg, &mut priority) == 0 {
                            free(name_3 as *mut libc::c_void);
                            strcpy(errstr,
                                   b"invalid priority\x00" as *const u8 as
                                       *const libc::c_char);
                            free(target_2 as *mut libc::c_void);
                            return 0
                        }
                        if !comma.is_null() &&
                               atoi_check16(comma, &mut weight) == 0 {
                            free(name_3 as *mut libc::c_void);
                            strcpy(errstr,
                                   b"invalid weight\x00" as *const u8 as
                                       *const libc::c_char);
                            free(target_2 as *mut libc::c_void);
                            return 0
                        }
                    }
                }
            }
            new_37 =
                opt_malloc(::std::mem::size_of::<MxSrvRecord>() as
                               libc::c_ulong) as *mut MxSrvRecord;
            (*new_37).next = daemon.mxnames;
            daemon.mxnames = new_37;
            (*new_37).issrv = 1;
            (*new_37).name = name_3;
            (*new_37).target = target_2;
            (*new_37).srvport = port;
            (*new_37).priority = priority;
            (*new_37).weight = weight;
            current_block = 7879481898411272068;
        }
        308 => {
            /* --host-record */
            let mut new_38: *mut HostRecord = 0 as *mut HostRecord;
            if arg.is_null() || { comma = split(arg); comma.is_null() } {
                strcpy(errstr,
                       b"Bad host-record\x00" as *const u8 as
                           *const libc::c_char);
                return 0
            }
            new_38 =
                opt_malloc(::std::mem::size_of::<HostRecord>() as
                               libc::c_ulong) as *mut HostRecord;
            memset(new_38 as *mut libc::c_void, 0,
                   ::std::mem::size_of::<HostRecord>() as libc::c_ulong);
            (*new_38).ttl = -(1);
            (*new_38).flags = 0;
            while !arg.is_null() {
                let mut addr_1: AllAddr =
                    AllAddr {addr4: InAddr {s_addr: 0,},};
                let mut dig_0: *mut libc::c_char = 0 as *mut libc::c_char;
                dig_0 = arg;
                while *dig_0 != 0 {
                    if (*dig_0) < '0' as i32 ||
                           *dig_0 > '9' as i32 {
                        break ;
                    }
                    dig_0 = dig_0.offset(1)
                }
                if *dig_0 == 0 {
                    (*new_38).ttl = atoi(arg)
                } else if inet_pton(2, arg,
                                    &mut addr_1.addr4 as *mut InAddr as
                                        *mut libc::c_void) != 0 {
                    (*new_38).addr = addr_1.addr4;
                    (*new_38).flags |= 2
                } else if inet_pton(10, arg,
                                    &mut addr_1.addr6 as *mut In6Addr as
                                        *mut libc::c_void) != 0 {
                    (*new_38).addr6 = addr_1.addr6;
                    (*new_38).flags |= 1
                } else {
                    let mut nomem: libc::c_int = 0;
                    let mut canon: *mut libc::c_char =
                        canonicalise(arg, &mut nomem);
                    let mut nl: *mut NameList = 0 as *mut NameList;
                    if canon.is_null() {
                        let mut tmp_3: *mut NameList = (*new_38).names;
                        let mut next: *mut NameList = 0 as *mut NameList;
                        tmp_3 = (*new_38).names;
                        while !tmp_3.is_null() {
                            next = (*tmp_3).next;
                            free(tmp_3 as *mut libc::c_void);
                            tmp_3 = next
                        }
                        strcpy(errstr,
                               b"Bad name in host-record\x00" as *const u8 as
                                   *const libc::c_char);
                        free(new_38 as *mut libc::c_void);
                        return 0
                    }
                    nl =
                        opt_malloc(::std::mem::size_of::<NameList>() as
                                       libc::c_ulong) as *mut NameList;
                    (*nl).name = canon;
                    /* keep order, so that PTR record goes to first name */
                    (*nl).next = 0 as *mut NameList;
                    if (*new_38).names.is_null() {
                        (*new_38).names = nl
                    } else {
                        let mut tmp_4: *mut NameList = 0 as *mut NameList;
                        tmp_4 = (*new_38).names;
                        while !(*tmp_4).next.is_null() {
                            tmp_4 = (*tmp_4).next
                        }
                        (*tmp_4).next = nl
                    }
                }
                arg = comma;
                comma = split(arg)
            }
            /* Keep list order */
            if daemon.host_records_tail.is_null() {
                daemon.host_records = new_38
            } else { (*daemon.host_records_tail).next = new_38 }
            (*new_38).next = 0 as *mut HostRecord;
            daemon.host_records_tail = new_38;
            current_block = 7879481898411272068;
        }
        _ => {
            strcpy(errstr,
                   b"unsupported option (check that dnsmasq was compiled with DHCP/TFTP/DNSSEC/DBus support)\x00"
                       as *const u8 as *const libc::c_char);
            return 0
        }
    }
    match current_block {
        2926860427235594157 =>
        /* --ignore-address */
        {
            let mut addr_0: InAddr = InAddr {s_addr: 0,}; /* error */
            unhide_metas(arg);
            if !arg.is_null() &&
                   inet_pton(2, arg,
                             &mut addr_0 as *mut InAddr as *mut libc::c_void)
                       > 0 {
                let mut baddr: *mut BogusAddr =
                    opt_malloc(::std::mem::size_of::<BogusAddr>() as
                                   libc::c_ulong) as *mut BogusAddr;
                if option == 'B' as i32 {
                    (*baddr).next = daemon.bogus_addr;
                    daemon.bogus_addr = baddr
                } else {
                    (*baddr).next = daemon.ignore_addr;
                    daemon.ignore_addr = baddr
                }
                (*baddr).addr = addr_0
            } else { strcpy(errstr, gen_err); return 0 }
            current_block = 7879481898411272068;
        }
        12010070245366740438 =>
        /* --dhcp-optsfile */
        {
            current_block = 2812646229686797995;
        }
        887445304002143054 =>
        /* --except-interface */
        {
            loop
                 /* --no-dhcp-interface */
                 {
                let mut new_9: *mut Iname =
                    opt_malloc(::std::mem::size_of::<Iname>() as
                                   libc::c_ulong) as *mut Iname;
                comma = split(arg);
                (*new_9).name = opt_string_alloc(arg);
                if option == 'I' as i32 {
                    (*new_9).next = daemon.if_except;
                    daemon.if_except = new_9
                } else if option == 258 {
                    (*new_9).next = daemon.tftp_interfaces;
                    daemon.tftp_interfaces = new_9
                } else {
                    (*new_9).next = daemon.dhcp_except;
                    daemon.dhcp_except = new_9
                }
                arg = comma;
                if arg.is_null() { break ; }
            }
            current_block = 7879481898411272068;
        }
        9676380469790025234 =>
        /*  --local */
        {
            current_block = 6480954168551069607;
        }
        15489771604880449635 =>
        /* --neg-ttl */
        {
            current_block = 6082976577402880686;
        }
        18295461473828413614 =>
        /* --dhcp-option-force */
        /* --dhcp-match */
        {
            return parse_dhcp_opt(errstr, arg,
                                  if option == 264 {
                                      16
                                  } else if option == 281 {
                                      128
                                  } else if option == 279 {
                                      32
                                  } else { 0 })
        }
        10375845272849059847 =>
        /* --dhcp-userclass */
        {
            current_block = 17332795835978703980;
        }
        8728755645494476224 =>
        /* --dhcp-ignore-names */
        {
            current_block = 9783966086509161201;
        }
        _ => { }
    }
    match current_block {
        2812646229686797995 =>
        /* --dhcp-hostsdir */
        {
            current_block = 10566976656908717602;
        }
        6480954168551069607 =>
        /*  --address */
        {
            current_block = 14399141444697241811;
        }
        6082976577402880686 =>
        /* --max-ttl */
        {
            current_block = 16916584745428150692;
        }
        17332795835978703980 =>
        /* --dhcp-circuitid */
        {
            current_block = 15503158355981179141;
        }
        9783966086509161201 =>
        /* --dhcp-broadcast */
        {
            current_block = 9535337827963792624;
        }
        _ => { }
    }
    match current_block {
        14399141444697241811 =>
        /*  --rebind-domain-ok */
        {
            let mut serv_1: *mut Server = 0 as *mut Server;
            let mut newlist: *mut Server = 0 as *mut Server;
            unhide_metas(arg);
            if !arg.is_null() &&
                   (*arg == '/' as i32 ||
                        option == 298) {
                let mut rebind: libc::c_int =
                    !(*arg == '/' as i32);
                let mut end_0: *mut libc::c_char = 0 as *mut libc::c_char;
                if rebind == 0 { arg = arg.offset(1) }
                while rebind != 0 ||
                          {
                              end_0 =
                                  split_chr(arg, '/' as i32 as libc::c_char);
                              !end_0.is_null()
                          } {
                    let mut domain: *mut libc::c_char =
                        0 as *mut libc::c_char;
                    /* elide leading dots - they are implied in the search algorithm */
                    while *arg == '.' as i32 {
                        arg = arg.offset(1)
                    }
                    /* # matches everything and becomes a zero length domain string */
                    if strcmp(arg,
                              b"#\x00" as *const u8 as *const libc::c_char) ==
                           0 {
                        domain =
                            b"\x00" as *const u8 as *const libc::c_char as
                                *mut libc::c_char
                    } else if strlen(arg) != 0 as libc::c_ulong
                                  &&
                                  {
                                      domain = canonicalise_opt(arg);
                                      domain.is_null()
                                  } {
                        strcpy(errstr, gen_err);
                        return 0
                    }
                    serv_1 =
                        opt_malloc(::std::mem::size_of::<Server>() as
                                       libc::c_ulong) as *mut Server;
                    memset(serv_1 as *mut libc::c_void, 0,
                           ::std::mem::size_of::<Server>() as libc::c_ulong);
                    (*serv_1).next = newlist;
                    newlist = serv_1;
                    (*serv_1).domain = domain;
                    (*serv_1).flags =
                        if !domain.is_null() {
                            8
                        } else { 32 };
                    arg = end_0;
                    if rebind != 0 { break ; }
                }
                if newlist.is_null() {
                    strcpy(errstr, gen_err);
                    return 0
                }
            } else {
                newlist =
                    opt_malloc(::std::mem::size_of::<Server>() as
                                   libc::c_ulong) as *mut Server;
                memset(newlist as *mut libc::c_void, 0,
                       ::std::mem::size_of::<Server>() as libc::c_ulong);
                (*newlist).uid = rand32()
            }
            if servers_only != 0 && option == 'S' as i32 {
                (*newlist).flags |= 4096
            }
            if option == 'A' as i32 {
                (*newlist).flags |= 4;
                if (*newlist).flags & (8 | 32)
                       == 0 {
                    server_list_free(newlist);
                    strcpy(errstr, gen_err);
                    return 0
                }
            } else if option == 298 {
                (*newlist).flags |= 2048
            }
            if arg.is_null() || *arg == 0 {
                if (*newlist).flags & 2048 == 0 {
                    (*newlist).flags |= 2
                }
                /* no server */
            } else if strcmp(arg,
                             b"#\x00" as *const u8 as *const libc::c_char) ==
                          0 {
                (*newlist).flags |= 1024
            } else {
                let mut err_0: *mut libc::c_char =
                    parse_server(arg, &mut (*newlist).addr,
                                 &mut (*newlist).source_addr,
                                 (*newlist).interface.as_mut_ptr(),
                                 &mut (*newlist).flags); /* treat in ordinary way */
                if !err_0.is_null() {
                    server_list_free(newlist);
                    strcpy(errstr, err_0);
                    return 0
                }
            }
            serv_1 = newlist;
            while !(*serv_1).next.is_null() {
                (*(*serv_1).next).flags |=
                    (*serv_1).flags & !(8 | 32);
                (*(*serv_1).next).addr = (*serv_1).addr;
                (*(*serv_1).next).source_addr = (*serv_1).source_addr;
                strcpy((*(*serv_1).next).interface.as_mut_ptr(),
                       (*serv_1).interface.as_mut_ptr());
                serv_1 = (*serv_1).next
            }
            (*serv_1).next = daemon.servers;
            daemon.servers = newlist;
            current_block = 7879481898411272068;
        }
        10566976656908717602 =>
        /* --dhcp-optsdir */
        {
            current_block = 2602045500541335152;
        }
        16916584745428150692 =>
        /* --min-cache-ttl */
        {
            current_block = 13094692781038244044;
        }
        15503158355981179141 =>
        /* --dhcp-remoteid */
        {
            current_block = 9763990383449182594;
        }
        9535337827963792624 =>
        /* --bootp-dynamic */
        {
            current_block = 8762260891357387630;
        }
        _ => { }
    }
    match current_block {
        8762260891357387630 =>
        /* --dhcp-generate-names */
        {
            let mut new_24: *mut DhcpNetIdList =
                opt_malloc(::std::mem::size_of::<DhcpNetIdList>() as
                               libc::c_ulong) as *mut DhcpNetIdList;
            let mut list_1: *mut DhcpNetId = 0 as *mut DhcpNetId;
            if option == 'J' as i32 {
                (*new_24).next = daemon.dhcp_ignore;
                daemon.dhcp_ignore = new_24
            } else if option == 282 {
                (*new_24).next = daemon.force_broadcast;
                daemon.force_broadcast = new_24
            } else if option == '3' as i32 {
                (*new_24).next = daemon.bootp_dynamic;
                daemon.bootp_dynamic = new_24
            } else if option == 296 {
                (*new_24).next = daemon.dhcp_gen_names;
                daemon.dhcp_gen_names = new_24
            } else {
                (*new_24).next = daemon.dhcp_ignore_names;
                daemon.dhcp_ignore_names = new_24
            }
            while !arg.is_null() {
                comma = split(arg);
                list_1 =
                    dhcp_netid_create(if is_tag_prefix(arg) != 0 {
                                          arg.offset(4 as
                                                         isize)
                                      } else { arg }, list_1);
                arg = comma
            }
            (*new_24).list = list_1;
            current_block = 7879481898411272068;
        }
        9763990383449182594 =>
        /* --dhcp-subscrid */
        {
            let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut dig: libc::c_int = 0;
            let mut new_23: *mut DhcpVendor =
                opt_malloc(::std::mem::size_of::<DhcpVendor>() as
                               libc::c_ulong) as *mut DhcpVendor;
            comma = split(arg);
            if comma.is_null() {
                strcpy(errstr, gen_err);
                free(new_23 as *mut libc::c_void);
                return 0
            }
            (*new_23).netid.net = opt_string_alloc(set_prefix(arg));
            /* check for hex string - must digits may include : must not have nothing else,
	    only allowed for agent-options. */
            arg = comma;
            comma = split(arg);
            if !comma.is_null() {
                if option != 'U' as i32 ||
                       strstr(arg,
                              b"enterprise:\x00" as *const u8 as
                                  *const libc::c_char) != arg {
                    free((*new_23).netid.net as *mut libc::c_void);
                    strcpy(errstr, gen_err);
                    free(new_23 as *mut libc::c_void);
                    return 0
                } else {
                    (*new_23).enterprise =
                        atoi(arg.offset(11)) as
                            libc::c_uint
                }
            } else { comma = arg }
            p = comma as *mut libc::c_uchar;
            while *p != 0 {
                if *(*__ctype_b_loc()).offset(*p) as
                       libc::c_int &
                       _ISXDIGIT as libc::c_ushort as
                           libc::c_int != 0 {
                    dig = 1
                } else if *p != ':' as i32 { break ; }
                p = p.offset(1)
            }
            unhide_metas(comma);
            if option == 'U' as i32 || option == 'j' as i32 ||
                   *p != 0 || dig == 0 {
                (*new_23).len = strlen(comma);
                (*new_23).data =
                    opt_malloc((*new_23).len as usize) as *mut libc::c_char;
                memcpy((*new_23).data as *mut libc::c_void,
                       comma as *const libc::c_void,
                       (*new_23).len as libc::c_ulong);
            } else {
                (*new_23).len =
                    parse_hex(comma, comma as *mut libc::c_uchar,
                              strlen(comma),
                              0 as *mut libc::c_uint, 0 as *mut libc::c_int);
                (*new_23).data =
                    opt_malloc((*new_23).len as usize) as *mut libc::c_char;
                memcpy((*new_23).data as *mut libc::c_void,
                       comma as *const libc::c_void,
                       (*new_23).len as libc::c_ulong);
            }
            match option {
                106 => { (*new_23).match_type = 2 }
                85 => { (*new_23).match_type = 1 }
                268 => { (*new_23).match_type = 3 }
                269 => { (*new_23).match_type = 4 }
                270 => { (*new_23).match_type = 5 }
                _ => { }
            }
            (*new_23).next = daemon.dhcp_vendors;
            daemon.dhcp_vendors = new_23;
            current_block = 7879481898411272068;
        }
        2602045500541335152 =>
        /* --hostsdir */
        {
            current_block = 4533671380017093834;
        }
        13094692781038244044 =>
        /* --max-cache-ttl */
        {
            current_block = 13035992208579083528;
        }
        _ => { }
    }
    match current_block {
        4533671380017093834 =>
        /* --addn-hosts */
        {
            let mut new_3: *mut HostsFile =
                opt_malloc(::std::mem::size_of::<HostsFile>() as
                               libc::c_ulong) as *mut HostsFile;
            static mut hosts_index: libc::c_uint =
                3 as libc::c_uint;
            (*new_3).fname = opt_string_alloc(arg);
            let fresh26 = hosts_index;
            hosts_index = hosts_index.wrapping_add(1);
            (*new_3).index = fresh26;
            (*new_3).flags = 0;
            if option == 'H' as i32 {
                (*new_3).next = daemon.addn_hosts;
                daemon.addn_hosts = new_3
            } else if option == 273 {
                (*new_3).next = daemon.dhcp_hosts_file;
                daemon.dhcp_hosts_file = new_3
            } else if option == 280 {
                (*new_3).next = daemon.dhcp_opts_file;
                daemon.dhcp_opts_file = new_3
            } else {
                (*new_3).next = daemon.dynamic_dirs;
                daemon.dynamic_dirs = new_3;
                if option == 340 {
                    (*new_3).flags |= 16
                } else if option == 341 {
                    (*new_3).flags |= 32
                } else if option == 342 {
                    (*new_3).flags |= 8
                }
            }
            current_block = 7879481898411272068;
        }
        13035992208579083528 =>
        /* --auth-ttl */
        {
            current_block = 5893551302610724882;
        }
        _ => { }
    }
    match current_block {
        5893551302610724882 =>
        /* --dhcp-ttl */
        {
            let mut ttl: libc::c_int = 0;
            if atoi_check(arg, &mut ttl) == 0 {
                strcpy(errstr, gen_err);
                return 0
            } else {
                if option == 283 {
                    daemon.neg_ttl = ttl as libc::c_ulong
                } else if option == 297 {
                    daemon.max_ttl = ttl as libc::c_ulong
                } else if option == 339 {
                    if ttl > 3600 { ttl = 3600 }
                    daemon.min_cache_ttl = ttl as libc::c_ulong
                } else if option == 312 {
                    daemon.max_cache_ttl = ttl as libc::c_ulong
                } else if option == 315 {
                    daemon.auth_ttl = ttl as libc::c_ulong
                } else if option == 348 {
                    daemon.dhcp_ttl = ttl as libc::c_ulong;
                    daemon.use_dhcp_ttl =
                        1 as libc::c_ulong
                } else { daemon.local_ttl = ttl as libc::c_ulong }
            }
        }
        _ => { }
    }
    return 1;
}
 fn read_file(mut file: *mut libc::c_char, mut f: *mut FILE,
                               mut hard_opt: libc::c_int) {
    let mut lineno: libc::c_int = 0;
    let mut buff: *mut libc::c_char = daemon.namebuff;
    let mut current_block_66: u64;
    while !fgets(buff, 1025, f).is_null() {
        let mut white: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut option: libc::c_int =
            if hard_opt == 332 {
                0
            } else { hard_opt };
        let mut errmess: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: usize = 0;
        /* Memory allocation failure longjmps here if mem_recover == 1 */
        if option != 0 || hard_opt == 332 {
            if _setjmp(mem_jmp.as_mut_ptr()) != 0 { continue ; }
            ::std::ptr::write_volatile(&mut mem_recover as *mut libc::c_int,
                                       1)
        }
        arg = 0 as *mut libc::c_char;
        ::std::ptr::write_volatile(&mut lineno as *mut libc::c_int,
                                   ::std::ptr::read_volatile::<libc::c_int>(&lineno
                                                                                as
                                                                                *const libc::c_int)
                                       + 1);
        errmess = 0 as *mut libc::c_char;
        /* Implement quotes, inside quotes we allow \\ \" \n and \t
	 metacharacters get hidden also strip comments */
        white = 1;
        p = buff;
        loop  {
            if !(*p != 0) { current_block_66 = 12199444798915819164; break ; }
            if *p == '\"' as i32 {
                memmove(p as *mut libc::c_void,
                        p.offset(1) as
                            *const libc::c_void,
                        strlen(p.offset(1 as
                                            isize)).wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong));
                while *p != 0 &&
                          *p != '\"' as i32 {
                    if *p == '\\' as i32 &&
                           !strchr(b"\"tnebr\\\x00" as *const u8 as
                                       *const libc::c_char,
                                   *p.offset(1) as
                                       libc::c_int).is_null() {
                        if *p.offset(1)
                               == 't' as i32 {
                            *p.offset(1) =
                                '\t' as i32 as libc::c_char
                        } else if *p.offset(1) as
                                      libc::c_int == 'n' as i32 {
                            *p.offset(1) =
                                '\n' as i32 as libc::c_char
                        } else if *p.offset(1) as
                                      libc::c_int == 'b' as i32 {
                            *p.offset(1) =
                                '\u{8}' as i32 as libc::c_char
                        } else if *p.offset(1) as
                                      libc::c_int == 'r' as i32 {
                            *p.offset(1) =
                                '\r' as i32 as libc::c_char
                        } else if *p.offset(1) as
                                      libc::c_int == 'e' as i32 {
                            /* escape */
                            *p.offset(1) =
                                '\u{1b}' as i32 as libc::c_char
                        }
                        memmove(p as *mut libc::c_void,
                                p.offset(1) as
                                    *const libc::c_void,
                                strlen(p.offset(1 as
                                                    isize)).wrapping_add(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong));
                    }
                    *p = hide_meta(*p);
                    p = p.offset(1)
                }
                if *p == 0 {
                    errmess =
                        b"missing \"\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    current_block_66 = 15635431839692940240;
                    break ;
                } else {
                    memmove(p as *mut libc::c_void,
                            p.offset(1) as
                                *const libc::c_void,
                            strlen(p.offset(1 as
                                                isize)).wrapping_add(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong));
                }
            }
            if *(*__ctype_b_loc()).offset(*p) as
                   libc::c_int &
                   _ISSPACE as libc::c_ushort !=
                   0 {
                *p = ' ' as i32 as libc::c_char;
                white = 1
            } else if white != 0 && *p == '#' as i32 {
                *p = 0 as libc::c_char;
                current_block_66 = 12199444798915819164;
                break ;
            } else { white = 0 }
            p = p.offset(1)
        }
        match current_block_66 {
            12199444798915819164 => {
                /* strip leading spaces */
                start = buff;
                while *start != 0 &&
                          *start == ' ' as i32 {
                    start = start.offset(1)
                }
                /* strip trailing spaces */
                len = strlen(start);
                while len != 0 as libc::c_ulong &&
                          *start.offset(len.wrapping_sub(1 as
                                                             libc::c_ulong) as
                                            isize) ==
                              ' ' as i32 {
                    len = len.wrapping_sub(1)
                }
                if len == 0 as libc::c_ulong { continue ; }
                *start.offset(len) =
                    0 as libc::c_char;
                if option != 0 {
                    arg = start
                } else {
                    p = strchr(start, '=' as i32);
                    if !p.is_null() {
                        /* allow spaces around "=" */
                        arg = p.offset(1);
                        while *arg == ' ' as i32 {
                            arg = arg.offset(1)
                        }
                        while p >= start &&
                                  (*p == ' ' as i32 ||
                                       *p == '=' as i32) {
                            *p = 0 as libc::c_char;
                            p = p.offset(-1)
                        }
                    } else { arg = 0 as *mut libc::c_char }
                }
                if option == 0 {
                    ::std::ptr::write_volatile(&mut option as
                                                   *mut libc::c_int,
                                               0);
                    i = 0;
                    while !opts[i as usize].name.is_null() {
                        if strcmp(opts[i as usize].name, start) ==
                               0 {
                            ::std::ptr::write_volatile(&mut option as
                                                           *mut libc::c_int,
                                                       opts[i as usize].val);
                            break ;
                        } else { i += 1 }
                    }
                    if option == 0 {
                        errmess =
                            b"bad option\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char
                    } else if opts[i as usize].has_arg == 0 &&
                                  !arg.is_null() {
                        errmess =
                            b"extraneous parameter\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char
                    } else if opts[i as usize].has_arg == 1 &&
                                  arg.is_null() {
                        errmess =
                            b"missing parameter\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char
                    } else if hard_opt == 332 &&
                                  option != 'S' as i32 &&
                                  option != 332 {
                        errmess =
                            b"illegal option\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char
                    }
                }
            }
            _ => { }
        }
        if !errmess.is_null() { strcpy(daemon.namebuff, errmess); }
        if !errmess.is_null() ||
               one_opt(option, arg, daemon.namebuff,
                       b"error\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char, 0,
                       (hard_opt == 332)) == 0 {
            sprintf(daemon.namebuff.offset(strlen(daemon.namebuff)
                                                         ),
                    b" at line %d of %s\x00" as *const u8 as
                        *const libc::c_char, lineno, file);
            if hard_opt != 0 {
                my_syslog(3,
                          b"%s\x00" as *const u8 as *const libc::c_char,
                          daemon.namebuff);
            } else {
                die(b"%s\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char, daemon.namebuff,
                    1);
            }
        }
    }
    ::std::ptr::write_volatile(&mut mem_recover as *mut libc::c_int,
                               0);
    fclose(f);
}
#[no_mangle]
pub  fn option_read_dynfile(mut file: *mut libc::c_char,
                                             mut flags: libc::c_int)
 -> libc::c_int {
    my_syslog((3) << 3 | 6,
              b"read %s\x00" as *const u8 as *const libc::c_char, file);
    if flags & 16 != 0 {
        return one_file(file, 272)
    } else {
        if flags & 32 != 0 {
            return one_file(file, 279)
        }
    }
    return 0;
}
 fn one_file(mut file: *mut libc::c_char,
                              mut hard_opt: libc::c_int) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut nofile_ok: libc::c_int = 0;
    static mut read_stdin: libc::c_int = 0;
    static mut filesread: *mut fileread =
        0 as *const fileread as *mut fileread;
    if hard_opt == '7' as i32 {
        /* default conf-file reading */
        hard_opt = 0;
        nofile_ok = 1
    }
    if hard_opt == 0 &&
           strcmp(file, b"-\x00" as *const u8 as *const libc::c_char) ==
               0 {
        if read_stdin == 1 { return 1 }
        read_stdin = 1;
        file =
            b"stdin\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        f = stdin
    } else {
        /* ignore repeated files. */
        let mut statbuf: stat =
            stat{st_dev: 0,
                 st_ino: 0,
                 st_nlink: 0,
                 st_mode: 0,
                 st_uid: 0,
                 st_gid: 0,
                 __pad0: 0,
                 st_rdev: 0,
                 st_size: 0,
                 st_blksize: 0,
                 st_blocks: 0,
                 st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
                 st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
                 st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
                 __glibc_reserved: [0; 3],}; /* No conffile, all done. */
        if hard_opt == 0 &&
               stat(file, &mut statbuf) == 0 {
            let mut r: *mut fileread = 0 as *mut fileread;
            r = filesread;
            while !r.is_null() {
                if r.dev == statbuf.st_dev && r.ino == statbuf.st_ino {
                    return 1
                }
                r = r.next
            }
            r =
                safe_malloc(::std::mem::size_of::<fileread>() as
                                libc::c_ulong) as *mut fileread;
            r.next = filesread;
            filesread = r;
            r.dev = statbuf.st_dev;
            r.ino = statbuf.st_ino
        }
        f = fopen(file, b"r\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            if *__errno_location() == 2 && nofile_ok != 0 {
                return 1
            } else {
                let mut str: *mut libc::c_char =
                    b"cannot read %s: %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char;
                if hard_opt != 0 {
                    my_syslog(3, str, file,
                              strerror(*__errno_location()));
                    return 0
                } else { die(str, file, 3); }
            }
        }
    }
    read_file(file, f, hard_opt);
    return 1;
}
/* expand any name which is a directory */
#[no_mangle]
pub  fn expand_filelist(mut list: *mut HostsFile)
 -> *mut HostsFile {
    let mut i: libc::c_uint = 0;
    let mut ah: *mut HostsFile = 0 as *mut HostsFile;
    /* find largest used index */
    i = 3 as libc::c_uint;
    ah = list;
    while !ah.is_null() {
        if i <= (*ah).index {
            i = (*ah).index.wrapping_add(1 as libc::c_uint)
        }
        if (*ah).flags & 1 != 0 {
            (*ah).flags |= 2
        } else { (*ah).flags &= !(2) }
        ah = (*ah).next
    }
    ah = list;
    while !ah.is_null() {
        if (*ah).flags & 2 == 0 {
            let mut buf: stat =
                stat{st_dev: 0,
                     st_ino: 0,
                     st_nlink: 0,
                     st_mode: 0,
                     st_uid: 0,
                     st_gid: 0,
                     __pad0: 0,
                     st_rdev: 0,
                     st_size: 0,
                     st_blksize: 0,
                     st_blocks: 0,
                     st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
                     st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
                     st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
                     __glibc_reserved: [0; 3],};
            if stat((*ah).fname, &mut buf) != -(1) &&
                   buf.st_mode & 0o170000 as libc::c_uint ==
                       0o40000 as libc::c_uint {
                let mut dir_stream: *mut DIR = 0 as *mut DIR;
                let mut ent: *mut dirent = 0 as *mut dirent;
                /* don't read this as a file */
                (*ah).flags |= 2;
                dir_stream = opendir((*ah).fname);
                if dir_stream.is_null() {
                    my_syslog(3,
                              b"cannot access directory %s: %s\x00" as
                                  *const u8 as *const libc::c_char,
                              (*ah).fname, strerror(*__errno_location()));
                } else {
                    loop  {
                        ent = readdir(dir_stream);
                        if ent.is_null() { break ; }
                        let mut lendir: usize = strlen((*ah).fname);
                        let mut lenfile: usize =
                            strlen((*ent).d_name.as_mut_ptr());
                        let mut ah1: *mut HostsFile = 0 as *mut HostsFile;
                        let mut path: *mut libc::c_char =
                            0 as *mut libc::c_char;
                        /* ignore emacs backups and dotfiles */
                        if lenfile == 0 as libc::c_ulong ||
                               (*ent).d_name[lenfile.wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong)
                                                 as usize] ==
                                   '~' as i32 ||
                               (*ent).d_name[0 as usize] as
                                   libc::c_int == '#' as i32 &&
                                   (*ent).d_name[lenfile.wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong)
                                                     as usize]
                                       == '#' as i32 ||
                               (*ent).d_name[0 as usize] as
                                   libc::c_int == '.' as i32 {
                            continue ;
                        }
                        /* see if we have an existing record.
		       dir is ah->fname
		       file is ent->d_name
		       path to match is ah1->fname */
                        ah1 = list;
                        while !ah1.is_null() {
                            if lendir < strlen((*ah1).fname) &&
                                   strstr((*ah1).fname, (*ah).fname) ==
                                       (*ah1).fname &&
                                   *(*ah1).fname.offset(lendir) as
                                       libc::c_int == '/' as i32 &&
                                   strcmp((*ah1).fname.offset(lendir as
                                                                  isize).offset(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize),
                                          (*ent).d_name.as_mut_ptr()) ==
                                       0 {
                                (*ah1).flags &= !(2);
                                break ;
                            } else { ah1 = (*ah1).next }
                        }
                        /* make new record */
                        if ah1.is_null() {
                            ah1 =
                                whine_malloc(::std::mem::size_of::<HostsFile>()
                                                 as libc::c_ulong) as
                                    *mut HostsFile;
                            if ah1.is_null() { continue ; }
                            path =
                                whine_malloc(lendir.wrapping_add(lenfile).wrapping_add(2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong))
                                    as *mut libc::c_char;
                            if path.is_null() {
                                free(ah1 as *mut libc::c_void);
                                continue ;
                            } else {
                                strcpy(path, (*ah).fname);
                                strcat(path,
                                       b"/\x00" as *const u8 as
                                           *const libc::c_char);
                                strcat(path, (*ent).d_name.as_mut_ptr());
                                (*ah1).fname = path;
                                let fresh33 = i;
                                i = i.wrapping_add(1);
                                (*ah1).index = fresh33;
                                (*ah1).flags = 1;
                                (*ah1).next = list;
                                list = ah1
                            }
                        }
                        /* inactivate record if not regular file */
                        if (*ah1).flags & 1 != 0 &&
                               stat((*ah1).fname, &mut buf) !=
                                   -(1) &&
                               !(buf.st_mode &
                                     0o170000 as libc::c_uint
                                     ==
                                     0o100000 as libc::c_uint)
                           {
                            (*ah1).flags |= 2
                        }
                    }
                    closedir(dir_stream);
                }
            }
        }
        ah = (*ah).next
    }
    return list;
}
#[no_mangle]
pub  fn read_servers_file() {
    let mut f: *mut FILE = 0 as *mut FILE;
    f =
        fopen(daemon.servers_file,
              b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        my_syslog(3,
                  b"cannot read %s: %s\x00" as *const u8 as
                      *const libc::c_char, daemon.servers_file,
                  strerror(*__errno_location()));
        return
    }
    mark_servers(4096);
    cleanup_servers();
    read_file(daemon.servers_file, f, 332);
}
 fn clear_dynamic_conf() {
    let mut configs: *mut DhcpConfig = 0 as *mut DhcpConfig;
    let mut cp: *mut DhcpConfig = 0 as *mut DhcpConfig;
    let mut up: *mut *mut DhcpConfig = 0 as *mut *mut DhcpConfig;
    /* remove existing... */
    up = &mut daemon.dhcp_conf;
    configs = daemon.dhcp_conf;
    while !configs.is_null() {
        cp = (*configs).next;
        if (*configs).flags & 2048 as libc::c_uint != 0 {
            let mut mac: *mut HwaddrConfig = 0 as *mut HwaddrConfig;
            let mut tmp: *mut HwaddrConfig = 0 as *mut HwaddrConfig;
            let mut list: *mut DhcpNetIdList = 0 as *mut DhcpNetIdList;
            let mut tmplist: *mut DhcpNetIdList = 0 as *mut DhcpNetIdList;
            mac = (*configs).hwaddr;
            while !mac.is_null() {
                tmp = (*mac).next;
                free(mac as *mut libc::c_void);
                mac = tmp
            }
            if (*configs).flags & 2 as libc::c_uint != 0 {
                free((*configs).clid as *mut libc::c_void);
            }
            list = (*configs).netid;
            while !list.is_null() {
                free((*list).list as *mut libc::c_void);
                tmplist = (*list).next;
                free(list as *mut libc::c_void);
                list = tmplist
            }
            if (*configs).flags & 16 as libc::c_uint != 0 {
                free((*configs).hostname as *mut libc::c_void);
            }
            *up = (*configs).next;
            free(configs as *mut libc::c_void);
        } else { up = &mut (*configs).next }
        configs = cp
    };
}
 fn clear_dynamic_opt() {
    let mut opts_0: *mut DhcpOpt = 0 as *mut DhcpOpt;
    let mut cp: *mut DhcpOpt = 0 as *mut DhcpOpt;
    let mut up: *mut *mut DhcpOpt = 0 as *mut *mut DhcpOpt;
    let mut id: *mut DhcpNetId = 0 as *mut DhcpNetId;
    let mut next: *mut DhcpNetId = 0 as *mut DhcpNetId;
    up = &mut daemon.dhcp_opts;
    opts_0 = daemon.dhcp_opts;
    while !opts_0.is_null() {
        cp = (*opts_0).next;
        if (*opts_0).flags & 32 != 0 {
            if (*opts_0).flags & 256 != 0 {
                free((*opts_0).u.vendor_class as *mut libc::c_void);
            }
            free((*opts_0).val as *mut libc::c_void);
            id = (*opts_0).netid;
            while !id.is_null() {
                next = (*id).next;
                free((*id).net as *mut libc::c_void);
                free(id as *mut libc::c_void);
                id = next
            }
            *up = (*opts_0).next;
            free(opts_0 as *mut libc::c_void);
        } else { up = &mut (*opts_0).next }
        opts_0 = cp
    };
}
#[no_mangle]
pub  fn reread_dhcp() {
    let mut hf: *mut HostsFile = 0 as *mut HostsFile;
    /* Do these even if there is no daemon->dhcp_hosts_file or
      daemon->dhcp_opts_file since entries may have been created by the
      inotify dynamic file reading system. */
    clear_dynamic_conf();
    clear_dynamic_opt();
    if !daemon.dhcp_hosts_file.is_null() {
        daemon.dhcp_hosts_file =
            expand_filelist(daemon.dhcp_hosts_file);
        hf = daemon.dhcp_hosts_file;
        while !hf.is_null() {
            if (*hf).flags & 2 == 0 {
                if one_file((*hf).fname, 272) != 0 {
                    my_syslog((3) << 3 |
                                  6,
                              b"read %s\x00" as *const u8 as
                                  *const libc::c_char, (*hf).fname);
                }
            }
            hf = (*hf).next
        }
    }
    if !daemon.dhcp_opts_file.is_null() {
        daemon.dhcp_opts_file =
            expand_filelist(daemon.dhcp_opts_file);
        hf = daemon.dhcp_opts_file;
        while !hf.is_null() {
            if (*hf).flags & 2 == 0 {
                if one_file((*hf).fname, 279) != 0 {
                    my_syslog((3) << 3 |
                                  6,
                              b"read %s\x00" as *const u8 as
                                  *const libc::c_char, (*hf).fname);
                }
            }
            hf = (*hf).next
        }
    }
    /* Setup notify and read pre-existing files. */
    set_dynamic_inotify(16 | 32,
                        0, 0 as *mut *mut Crec,
                        0);
}
#[no_mangle]
pub  fn read_opts(mut argc: libc::c_int,
                                   mut argv: *mut *mut libc::c_char,
                                   mut compile_opts: *mut libc::c_char) {
    let mut argbuf_size: usize = 1025 as usize;
    let mut argbuf: *mut libc::c_char =
        opt_malloc(argbuf_size) as *mut libc::c_char;
    let mut buff: *mut libc::c_char =
        opt_malloc(1025 as usize) as *mut libc::c_char;
    let mut option: libc::c_int = 0;
    let mut testmode: libc::c_int = 0;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut conffile: *mut libc::c_char = 0 as *mut libc::c_char;
    opterr = 0;
    dnsmasq_daemon =
        opt_malloc(::std::mem::size_of::<DnsmasqDaemon>() as libc::c_ulong)
            as *mut DnsmasqDaemon;
    memset(dnsmasq_daemon as *mut libc::c_void, 0,
           ::std::mem::size_of::<DnsmasqDaemon>() as libc::c_ulong);
    daemon.namebuff = buff;
    /* Set defaults - everything else is zero or NULL */
    daemon.cachesize = 150;
    daemon.ftabsize = 150;
    daemon.port = 53;
    daemon.dhcp_client_port = 68;
    daemon.dhcp_server_port = 67;
    daemon.default_resolv.is_default = 1;
    daemon.default_resolv.name =
        b"/etc/resolv.conf\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    daemon.resolv_files = &mut daemon.default_resolv;
    daemon.username =
        b"nobody\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    daemon.runfile =
        b"/var/run/dnsmasq.pid\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    daemon.dhcp_max = 1000;
    daemon.tftp_max = 50;
    daemon.edns_pktsz = 4096 as libc::c_ushort;
    daemon.log_fac = -(1);
    daemon.auth_ttl = 600 as libc::c_ulong;
    daemon.soa_refresh = 1200 as libc::c_ulong;
    daemon.soa_retry = 180 as libc::c_ulong;
    daemon.soa_expiry = 1209600 as libc::c_ulong;
    daemon.max_port = 65535 as libc::c_uint;
    daemon.min_port = 1024;
    add_txt(b"version.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"dnsmasq-2.84rc2\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0);
    add_txt(b"authors.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"Simon Kelley\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0);
    add_txt(b"copyright.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char,
            b"Copyright (c) 2000-2021 Simon Kelley\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char, 0);
    add_txt(b"cachesize.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 1);
    add_txt(b"insertions.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 2);
    add_txt(b"evictions.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 3);
    add_txt(b"misses.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 4);
    add_txt(b"hits.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 5);
    add_txt(b"auth.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 6);
    add_txt(b"servers.bind\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char, 0 as *mut libc::c_char, 7);
    loop  {
        option =
            getopt_long(argc, argv,
                        b"951yZDNLERKzowefnbvhdkqr:m:p:c:l:s:i:t:u:g:a:x:S:C:A:T:H:Q:I:B:F:G:O:M:X:V:U:j:P:J:W:Y:2:4:6:7:8:0:3:\x00"
                            as *const u8 as *const libc::c_char,
                        opts.as_ptr(), 0 as *mut libc::c_int);
        if option == -(1) {
            while optind < argc {
                let mut c: *mut libc::c_uchar =
                    *argv.offset(optind) as *mut libc::c_uchar;
                while *c != 0 {
                    if *(*__ctype_b_loc()).offset(*c)
                           &
                           _ISSPACE as libc::c_ushort as
                               libc::c_int == 0 {
                        die(b"junk found in command line\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            0 as *mut libc::c_char, 1);
                    }
                    c = c.offset(1)
                }
                optind += 1
            }
            break ;
        } else {
            /* Copy optarg so that argv doesn't get changed */
            if !optarg.is_null() {
                if strlen(optarg) >= argbuf_size {
                    free(argbuf as *mut libc::c_void);
                    argbuf_size =
                        strlen(optarg).wrapping_add(1 as
                                                        libc::c_ulong);
                    argbuf = opt_malloc(argbuf_size) as *mut libc::c_char
                }
                safe_strncpy(argbuf, optarg, argbuf_size);
                arg = argbuf
            } else { arg = 0 as *mut libc::c_char }
            /* command-line only stuff */
            if option == 293 {
                testmode = 1
            } else if option == 'w' as i32 {
                if argc == 3 &&
                       strcmp(*argv.offset(2),
                              b"dhcp\x00" as *const u8 as *const libc::c_char)
                           == 0 {
                    display_opts();
                } else if argc == 3 &&
                              strcmp(*argv.offset(2),
                                     b"dhcp6\x00" as *const u8 as
                                         *const libc::c_char) ==
                                  0 {
                    display_opts6();
                } else { do_usage(); }
                exit(0);
            } else {
                if option == 'v' as i32 {
                    printf(b"Dnsmasq version %s  %s\n\x00" as *const u8 as
                               *const libc::c_char,
                           b"2.84rc2\x00" as *const u8 as *const libc::c_char,
                           b"Copyright (c) 2000-2021 Simon Kelley\x00" as
                               *const u8 as *const libc::c_char);
                    printf(b"Compile time options: %s\n\n\x00" as *const u8 as
                               *const libc::c_char, compile_opts);
                    printf(b"This software comes with ABSOLUTELY NO WARRANTY.\n\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"Dnsmasq is free software, and you are welcome to redistribute it\n\x00"
                               as *const u8 as *const libc::c_char);
                    printf(b"under the terms of the GNU General Public License, version 2 or 3.\n\x00"
                               as *const u8 as *const libc::c_char);
                    exit(0);
                } else {
                    if option == 'C' as i32 {
                        if conffile.is_null() {
                            conffile = opt_string_alloc(arg)
                        } else {
                            let mut extra: *mut libc::c_char =
                                opt_string_alloc(arg);
                            one_file(extra, 0);
                            free(extra as *mut libc::c_void);
                        }
                    } else if one_opt(option, arg, daemon.namebuff,
                                      b"try --help\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char, 1,
                                      0) == 0 {
                        die(b"bad command line options: %s\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            daemon.namebuff, 1);
                    }
                }
            }
        }
    }
    free(argbuf as *mut libc::c_void);
    if !conffile.is_null() {
        one_file(conffile, 0);
        free(conffile as *mut libc::c_void);
    } else {
        one_file(b"/etc/dnsmasq.conf\x00" as *const u8 as *const libc::c_char
                     as *mut libc::c_char, '7' as i32);
    }
    /* port might not be known when the address is parsed - fill in here */
    if !daemon.servers.is_null() {
        let mut tmp: *mut Server = 0 as *mut Server;
        tmp = daemon.servers;
        while !tmp.is_null() {
            if (*tmp).flags & 16 == 0 {
                if (*tmp).source_addr.sa.sa_family ==
                       2 {
                    (*tmp).source_addr.in_0.sin_port =
                        __bswap_16(daemon.query_port as u16)
                } else if (*tmp).source_addr.sa.sa_family ==
                              10 {
                    (*tmp).source_addr.in6.sin6_port =
                        __bswap_16(daemon.query_port as u16)
                }
            }
            tmp = (*tmp).next
        }
    }
    if !daemon.host_records.is_null() {
        let mut hr: *mut HostRecord = 0 as *mut HostRecord;
        hr = daemon.host_records;
        while !hr.is_null() {
            if (*hr).ttl == -(1) {
                (*hr).ttl = daemon.local_ttl
            }
            hr = (*hr).next
        }
    }
    if !daemon.cnames.is_null() {
        let mut cn: *mut Cname = 0 as *mut Cname;
        let mut cn2: *mut Cname = 0 as *mut Cname;
        let mut cn3: *mut Cname = 0 as *mut Cname;
        /* Fill in TTL for CNAMES now we have local_ttl.
	 Also prepare to do loop detection. */
        cn = daemon.cnames;
        while !cn.is_null() {
            if (*cn).ttl == -(1) {
                (*cn).ttl = daemon.local_ttl
            }
            (*cn).flag = 0;
            (*cn).targetp = 0 as *mut Cname;
            cn2 = daemon.cnames;
            while !cn2.is_null() {
                if hostname_isequal((*cn).target, (*cn2).alias) != 0 {
                    (*cn).targetp = cn2;
                    break ;
                } else { cn2 = (*cn2).next }
            }
            cn = (*cn).next
        }
        /* Find any CNAME loops.*/
        cn = daemon.cnames;
        while !cn.is_null() {
            cn2 = (*cn).targetp;
            while !cn2.is_null() {
                if (*cn2).flag == 1 { break ; }
                if (*cn2).flag == 2 {
                    die(b"CNAME loop involving %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        (*cn).alias, 1);
                }
                (*cn2).flag = 2;
                cn2 = (*cn2).targetp
            }
            cn3 = (*cn).targetp;
            while cn3 != cn2 {
                (*cn3).flag = 1;
                cn3 = (*cn3).targetp
            }
            cn = (*cn).next
        }
    }
    if !daemon.if_addrs.is_null() {
        let mut tmp_0: *mut Iname = 0 as *mut Iname;
        tmp_0 = daemon.if_addrs;
        while !tmp_0.is_null() {
            if (*tmp_0).addr.sa.sa_family == 2 {
                (*tmp_0).addr.in_0.sin_port =
                    __bswap_16(daemon.port as u16)
            } else if (*tmp_0).addr.sa.sa_family ==
                          10 {
                (*tmp_0).addr.in6.sin6_port =
                    __bswap_16(daemon.port as u16)
            }
            tmp_0 = (*tmp_0).next
        }
    }
    /* create default, if not specified */
    if !daemon.authserver.is_null() &&
           daemon.hostmaster.is_null() {
        strcpy(buff, b"hostmaster.\x00" as *const u8 as *const libc::c_char);
        strcat(buff, daemon.authserver);
        daemon.hostmaster = opt_string_alloc(buff)
    }
    if daemon.dhcp_pxe_vendors.is_null() {
        daemon.dhcp_pxe_vendors =
            opt_malloc(::std::mem::size_of::<DhcpPxeVendor>() as
                           libc::c_ulong) as *mut DhcpPxeVendor;
        (*daemon.dhcp_pxe_vendors).data =
            opt_string_alloc(b"PXEClient\x00" as *const u8 as
                                 *const libc::c_char);
        (*daemon.dhcp_pxe_vendors).next = 0 as *mut DhcpPxeVendor
    }
    /* only one of these need be specified: the other defaults to the host-name */
    if daemon.options[(10 as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (10 as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 || !daemon.mxnames.is_null() ||
           !daemon.mxtarget.is_null() {
        let mut mx: *mut MxSrvRecord = 0 as *mut MxSrvRecord;
        if gethostname(buff, 1025 as usize) ==
               -(1) {
            die(b"cannot get host-name: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char, 5);
        }
        mx = daemon.mxnames;
        while !mx.is_null() {
            if (*mx).issrv == 0 && hostname_isequal((*mx).name, buff) != 0 {
                break ;
            }
            mx = (*mx).next
        }
        if (!daemon.mxtarget.is_null() ||
                daemon.options[(10 as
                                               libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                as
                                                                                libc::c_ulong).wrapping_mul(8
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_ulong))
                                              as usize] &
                    (1 as libc::c_uint) <<
                        (10 as
                             libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                              as
                                                              libc::c_ulong).wrapping_mul(8
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong))
                    != 0) && mx.is_null() {
            mx =
                opt_malloc(::std::mem::size_of::<MxSrvRecord>() as
                               libc::c_ulong) as *mut MxSrvRecord;
            (*mx).next = daemon.mxnames;
            (*mx).issrv = 0;
            (*mx).target = 0 as *mut libc::c_char;
            (*mx).name = opt_string_alloc(buff);
            daemon.mxnames = mx
        }
        if daemon.mxtarget.is_null() {
            daemon.mxtarget = opt_string_alloc(buff)
        }
        mx = daemon.mxnames;
        while !mx.is_null() {
            if (*mx).issrv == 0 && (*mx).target.is_null() {
                (*mx).target = daemon.mxtarget
            }
            mx = (*mx).next
        }
    }
    if daemon.options[(8 as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (8 as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 && !daemon.resolv_files.is_null() &&
           !(*daemon.resolv_files).next.is_null() &&
           daemon.options[(5 as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (5 as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        die(b"only one resolv.conf file allowed in no-poll mode.\x00" as
                *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 1);
    }
    if daemon.options[(15 as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (15 as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut f: *mut FILE = 0 as *mut FILE;
        if daemon.options[(8 as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (8 as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 || daemon.resolv_files.is_null() ||
               !(*daemon.resolv_files).next.is_null() {
            die(b"must have exactly one resolv.conf to read domain from.\x00"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char, 1);
        }
        f =
            fopen((*daemon.resolv_files).name,
                  b"r\x00" as *const u8 as *const libc::c_char);
        if f.is_null() {
            die(b"failed to read %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*daemon.resolv_files).name, 3);
        }
        loop  {
            line = fgets(buff, 1025, f);
            if line.is_null() { break ; }
            let mut token: *mut libc::c_char =
                strtok(line,
                       b" \t\n\r\x00" as *const u8 as *const libc::c_char);
            if token.is_null() ||
                   strcmp(token,
                          b"search\x00" as *const u8 as *const libc::c_char)
                       != 0 {
                continue ;
            }
            token =
                strtok(0 as *mut libc::c_char,
                       b" \t\n\r\x00" as *const u8 as *const libc::c_char);
            if !token.is_null() &&
                   {
                       daemon.domain_suffix =
                           canonicalise_opt(token);
                       !daemon.domain_suffix.is_null()
                   } {
                break ;
            }
        }
        fclose(f);
        if daemon.domain_suffix.is_null() {
            die(b"no search directive found in %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                (*daemon.resolv_files).name, 5);
        }
    }
    if !daemon.domain_suffix.is_null() {
        /* add domain for any srv record without one. */
        let mut srv: *mut MxSrvRecord = 0 as *mut MxSrvRecord;
        srv = daemon.mxnames;
        while !srv.is_null() {
            if (*srv).issrv != 0 && !strchr((*srv).name, '.' as i32).is_null()
                   &&
                   strchr((*srv).name, '.' as i32) ==
                       strrchr((*srv).name, '.' as i32) {
                strcpy(buff, (*srv).name);
                strcat(buff, b".\x00" as *const u8 as *const libc::c_char);
                strcat(buff, daemon.domain_suffix);
                free((*srv).name as *mut libc::c_void);
                (*srv).name = opt_string_alloc(buff)
            }
            srv = (*srv).next
        }
    } else if daemon.options[(20 as
                                             libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                              as
                                                                              libc::c_ulong).wrapping_mul(8
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_ulong))
                                            as usize] &
                  (1 as libc::c_uint) <<
                      (20 as
                           libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                            as
                                                            libc::c_ulong).wrapping_mul(8
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_ulong))
                  != 0 {
        die(b"there must be a default domain when --dhcp-fqdn is set\x00" as
                *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 1);
    }
    /* If there's access-control config, then ignore --local-service, it's intended
     as a system default to keep otherwise unconfigured installations safe. */
    if !daemon.if_names.is_null() ||
           !daemon.if_except.is_null() ||
           !daemon.if_addrs.is_null() ||
           !daemon.authserver.is_null() {
        reset_option_bool(49 as libc::c_uint);
    }
    if testmode != 0 {
        fprintf(stderr,
                b"dnsmasq: %s.\n\x00" as *const u8 as *const libc::c_char,
                b"syntax check OK\x00" as *const u8 as *const libc::c_char);
        exit(0);
    };
}
