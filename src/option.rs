use crate::defines::{Code, CODE, FACILITYNAMES, __sigset_t, OptionUsage, TxtRecord, DnsmasqDaemon, C2rustUnnamed10, NetAddress, NetAddress, SaFamily, In6Addr, __bswap_16, InAddrT, Server, DhcpNetId, DhcpNetIdList, DhcpConfig, HwaddrConfig, AddressListEntry, DhcpContext, DhcpOpt, __bswap_32, Mysubnet, Resolvc, time::Instant, MxSrvRecord, Iname, NameListEntry, AuthZone, AuthNameList, NetAddress, CondDomain, in_port_t, C2RustUnnamed, IpSets, TftpPrefix, DhcpBridge, SharedNetwork, _ISDIGIT, TagIf, DhcpMatchName, DhcpBoot, DelayConfig, PxeService, DhcpMac, AddrList2, DhcpPxeVendor, DhcpRelay, RaInterface, Doctor, InterfaceName, Cname, _ISSPACE, PtrRecord, NaPtr, HostRecord, BogusAddr, DhcpVendor, _ISXDIGIT, HostsFile, FILE, stat, timespec, DIR, Crec, OptionValue, TabEntryA};
use crate::util::{canonicalise, hostname_isequal, addr6part, setaddr6part, is_same_net4, is_same_net6, parse_hex, legal_hostname, string_from_offset};
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
pub static META: String = String::from("\x00123456 \x08\t\n78\r90abcdefABCDE\x1bF:,.");

 fn hide_meta(mut c: libc::c_char) -> libc::c_char {
    let mut i: u32 = 0;
    i = 0;
    while (i) <
              (::std::mem::size_of::<[libc::c_char; 33]>()).wrapping_sub(1libc::c_ulong) {
        if c == META[i ] {
            return i
        }
        i = i.wrapping_add(1)
    }
    return c;
}
 fn unhide_meta(mut cr: libc::c_char) -> libc::c_char {
    let mut c: u32 = cr;
    if (c) <
           (::std::mem::size_of::<[libc::c_char; 33]>()   ).wrapping_sub(1)
       {
        cr = META[c ]
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
     r.len = len.wrapping_add(1);
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
//     printf("Usage: dnsmasq [options]\n\n" as *const u8 as
//                *const libc::c_char);
//     printf("Valid options are:\n" as *const u8 as *const libc::c_char);
//     i = 0 as libc::c_int;
//     while usage[i as usize].opt != 0 as libc::c_int {
//         let mut desc: *mut libc::c_char = usage[i as usize].flagdesc;
//         let mut eq: *mut libc::c_char =
//             "=" as *const u8 as *const libc::c_char as *mut libc::c_char;
//         if desc.is_null() || *desc as libc::c_int == '[' as i32 {
//             eq =
//                 "" as *const u8 as *const libc::c_char as
//                     *mut libc::c_char
//         }
//         if desc.is_null() {
//             desc =
//                 "" as *const u8 as *const libc::c_char as
//                     *mut libc::c_char
//         }
//         j = 0 as libc::c_int;
//         while !opts[j as usize].name.is_null() {
//             if opts[j as usize].val == usage[i as usize].opt { break ; }
//             j += 1
//         }
//         if usage[i as usize].opt < 256 as libc::c_int {
//             sprintf(buff.as_mut_ptr(),
//                     "-%c, " as *const u8 as *const libc::c_char,
//                     usage[i as usize].opt);
//         } else {
//             sprintf(buff.as_mut_ptr(),
//                     "    " as *const u8 as *const libc::c_char);
//         }
//         sprintf(buff.as_mut_ptr().offset(4 as libc::c_int as isize),
//                 "--%s%s%s" as *const u8 as *const libc::c_char,
//                 opts[j as usize].name, eq, desc);
//         printf("%-55.55s" as *const u8 as *const libc::c_char,
//                buff.as_mut_ptr());
//         if !usage[i as usize].arg.is_null() {
//             strcpy(buff.as_mut_ptr(), usage[i as usize].arg);
//             j = 0 as libc::c_int;
//             while tab[j as usize].handle != 0 {
//                 if tab[j as usize].handle as libc::c_int ==
//                        *usage[i as usize].arg as libc::c_int {
//                     sprintf(buff.as_mut_ptr(),
//                             "%d" as *const u8 as *const libc::c_char,
//                             tab[j as usize].val);
//                 }
//                 j += 1
//             }
//         }
//         printf(usage[i as usize].desc, buff.as_mut_ptr());
//         printf("\n" as *const u8 as *const libc::c_char);
//         i += 1
//     };
// }

pub fn parse_NetAddress(mut arg: &String, mut addr: &mut NetAddress)
 -> Option<String> {



    if inet_pton(2, arg,
                 &mut addr.in_0.sin_addr              Vec<u8>) > 0 {
        addr.sa.sa_family = 2
    } else if inet_pton(10, arg,
                        &mut addr.in6.sin6_addr                     Vec<u8>) > 0 {
        addr.sa.sa_family = 10
    } else {
        return Some(String::from("bad address"));
    }
    return None;
}

pub  fn parse_server(arg: &String,
                     addr: &mut NetAddress,
                     source_addr: &mut NetAddress,
                     mut interface: &mut String,
                     mut flags: &mut i32) -> Option<String> {
    let mut source_port: i32 = 0;
    let mut serv_port: i32 = 53;
    let mut portno: String = String::new();
    let mut source: String = String::new();
    let mut interface_opt: String = String::new();
    let mut scope_index: i32 = 0;
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
        addr.in_0.sin_port = __bswap_16(serv_port);
        source_addr.sa.sa_family = 2;
        addr.sa.sa_family = source_addr.sa.sa_family;
        source_addr.in_0.sin_addr.s_addr = 0;
        source_addr.in_0.sin_port =
            __bswap_16(daemon.query_port);
        if !source.is_empty() {
            if !flags.is_empty() { *flags |= 16 }
            source_addr.in_0.sin_port =
                __bswap_16(source_port);
            if !(inet_pton(2, source,
                           &mut source_addr.in_0.sin_addr
                              ) > 0) {
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
        addr.in6.sin6_port = __bswap_16(serv_port);
        addr.in6.sin6_scope_id = scope_index;
        source_addr.in6.sin6_addr = in6addr_any;
        source_addr.in6.sin6_port =
            __bswap_16(daemon.query_port);
        source_addr.in6.sin6_scope_id = 0;
        source_addr.sa.sa_family = 10;
        addr.sa.sa_family = source_addr.sa.sa_family;
        source_addr.in6.sin6_flowinfo = 0;
        addr.in6.sin6_flowinfo = source_addr.in6.sin6_flowinfo;
        if !source.is_null() {
            if !flags.is_null() { *flags |= 16 }
            source_addr.in6.sin6_port =
                __bswap_16(source_port);
            if inet_pton(10, source,
                         &mut source_addr.in6.sin6_addr                      Vec<u8>) == 0 {
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
                mut addr: NetAddress,
                mut msize: i32) -> Option<Server> {
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
                                 "%u."                                *const libc::c_char,
                                 a & 0xff)                       isize);
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
                                 "%d."                                *const libc::c_char,
                                 a >> 8 &
                                     0xff)                       isize);
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
                                 "%d."                                *const libc::c_char,
                                 a >> 16 &
                                     0xff)                       isize)
        }
        _ => { }
    }
    /* fall through */
    p =
        p.offset(sprintf(p, "%d." ,
                         a >> 24 &
                             0xff)               isize); /* strlen("32*<n.>ip6.arpa")+1 */
    p =
        p.offset(sprintf(p,
                         "in-addr.arpa"                        *const libc::c_char));
    serv.flags = 8;
    serv.next = daemon.servers;
    daemon.servers = serv.clone();
    return Some(serv);
}

 pub fn add_rev6(mut addr: In6Addr, mut msize: i32) -> Option<Server> {
    let mut serv: Server = Default::default();
    let mut p: String = String::new();
    let mut i: i32 = 0;
    serv.domain = String::new();
    p = serv.domain.clone();
    i = msize - 1;
    while i >= 0 {
        let mut dig: i32 = addr.__in6_u.__u6_addr16[i >> 3];
        p =
            p.offset(sprintf(p,
                             "%.1x." ,
                             if i >> 2 & 1 != 0
                                {
                                 (dig) & 15
                             } else { (dig) >> 4 }));
        i -= 4
    }
    p =
        p.offset(sprintf(p,
                         "ip6.arpa" )
                    );
    serv.flags = 8;
    // serv.next = daemon.servers;
    daemon.servers = serv.clone();
    return Some(serv);
}
 fn is_tag_prefix(mut arg: &String)
 -> bool {
    if !arg.is_empty() &&
           (strstr(arg, "net:" ) ==
                arg ||
                strstr(arg, "tag:" )
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
//         let mut tmp: DhcpNetId = nid;
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
//         let mut hwaddr: HwaddrConfig = (*config).hwaddr;
//         while !hwaddr.is_null() {
//             let mut tmp: HwaddrConfig = hwaddr;
//             hwaddr = (*hwaddr).next;
//             free(tmp as *mut libc::c_void);
//         }
//         dhcp_netid_list_free((*config).netid);
//         dhcp_netid_free((*config).filter);
//         if (*config).flags & 2 as libc::c_int as libc::c_uint != 0 {
//             free((*config).clid as *mut libc::c_void);
//         }
//         if (*config).flags & 4096 as libc::c_int as libc::c_uint != 0 {
//             let mut addr: AddrList = 0 as *mut AddrList;
//             let mut tmp_0: AddrList = 0 as *mut AddrList;
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

 fn dhcp_context_free(mut ctx: DhcpContext) {
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
                opt_len = 0;
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
            if strstr(arg, "vendor:" )
                   == arg {
                new.u.vendor_class =
                    opt_string_alloc(arg.offset(7))                  mut Vec<u8>;
                new.flags |= 256
            } else if strstr(arg,
                             "encap:"                            *const libc::c_char) == arg {
                new.u.encap = atoi(arg.offset(6));
                new.flags |= 4
            } else if strstr(arg,
                             "vi-encap:"                            *const libc::c_char) == arg {
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
                   "unsupported encapsulation for IPv6 option"                 *const u8);
            current_block = 14151404249770905673;
        } else {
            if opt_len == 0 &&
                   new.flags & 2048 == 0 {
                opt_len =
                    lookup_dhcp_len(10, new.opt)
            }
            current_block = 317151059986244064;
        }
    } else {
        if opt_len == 0 &&
               new.flags &
                   (256 | 4 |
                        2048) == 0 {
            opt_len = lookup_dhcp_len(2, new.opt)
        }
        current_block = 317151059986244064;
    }
    match current_block {
        317151059986244064 =>
        /* option may be missing with rfc3925 match */
        {
            if option_ok == 0 {
                strcpy(errstr,
                       "bad dhcp-option" );
            } else {
                if !comma.is_null() {
                    /* characterise the value */
                    let mut c: libc::c_char = 0;
                    let mut found_dig: i32 = 0;
                    let mut found_colon: i32 = 0;
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
                            if cp.offset(1)                             libc::c_int == 0 &&
                                   is_dec != 0 &&
                                   (c == 'b' as i32 ||
                                        c == 's' as i32 ||
                                        c == 'i' as i32) {
                                lenchar = c;
                                cp = 0
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
                                 } else { 0x4000 })
                    }
                    /* We know that some options take addresses */
                    if opt_len & 0x8000 != 0 {
                        is_hex = 0;
                        is_dec = is_hex;
                        is_string = is_dec;
                        if is6 == 0 &&
                               (is_addr == 0 || dots == 0) {
                            strcpy(errstr,
                                   "bad IP address"                                  *const libc::c_char);
                            current_block = 14151404249770905673;
                        } else if is6 != 0 && is_addr6 == 0 {
                            strcpy(errstr,
                                   "bad IPv6 address"                                  *const libc::c_char);
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
                                       0 {
                                let mut val: i32 = 0;
                                let mut fac: i32 = 1;
                                let mut current_block_105: u64;
                                match *comma.offset(strlen(comma).wrapping_sub(1
                                                                                                                  libc::c_int
                                                                                                           )
                                                       )                                    libc::c_int {
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
                                                                                                                      libc::c_int
                                                                                                               )
                                                         ) =
                                            0
                                    }
                                    _ => { }
                                }
                                new.len = 4;
                                new.val =
                                    opt_malloc(4 )                                  mut Vec<u8>;
                                val = atoi(comma);
                                *(new.val) =
                                    __bswap_32((val * fac)) ;
                                current_block = 15439134456549723682;
                            } else if is_hex != 0 && digs > 1 {
                                new.len = digs;
                                new.val =
                                    opt_malloc(new.len )                                  mut Vec<u8>;
                                parse_hex(comma, new.val, digs,
                                          if flags & 128 != 0 {
                                              &mut new.u.wildcard_mask
                                          } else { 0 },
                                          0);
                                new.flags |= 512;
                                current_block = 15439134456549723682;
                            } else if is_dec != 0 {
                                let mut i: i32 = 0;
                                let mut val_0: i32 = atoi(comma);
                                /* assume numeric arg is 1 byte except for
	     options where it is known otherwise.
	     For vendor class option, we have to hack. */
                                if opt_len != 0
                                   {
                                    new.len = opt_len
                                } else if val_0 &
                                              0xffff0000 != 0
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
                                    opt_malloc(new.len )                                  mut Vec<u8>;
                                i = 0;
                                while i < new.len {
                                    *new.val.offset(i) =
                                        (val_0 >>
                                             (new.len - i -
                                                  1) *
                                                 8)                                      libc::c_uchar;
                                    i += 1
                                }
                                current_block = 15439134456549723682;
                            } else if is_addr != 0 && is6 == 0 {
                                let mut in_0: NetAddress = NetAddress {s_addr: 0,};
                                let mut op: mut Vec<u8> =
                                    0;
                                let mut slash: &mut String =
                                    0 ;
                                /* max length of address/subnet descriptor is five bytes,
	     add one for the option 120 enc byte too */
                                op =
                                    opt_malloc((5 * addrs +
                                                    1)usize)                                  mut Vec<u8>; /* RFC 3361 "enc byte" */
                                new.val = op;
                                new.flags |= 1;
                                if new.flags &
                                       (4 | 256
                                            | 2048) == 0 &&
                                       new.opt == 120 {
                                    let fresh6 = op;
                                    op = op.offset(1);
                                    *fresh6 =
                                        1;
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
                                                  '/' );
                                    if inet_pton(2, cp,
                                                 &mut in_0 Vec<u8>) == 0 {
                                        strcpy(errstr,
                                               "bad IPv4 address"*const u8*const libc::c_char);
                                        current_block = 14151404249770905673;
                                        break ;
                                    } else if slash.is_null() {
                                        memcpy(op,
                                               &mut in_0*const libc::c_void,
                                               4libc::c_ulong);
                                        op =
                                            op.offset(4       isize)
                                    } else {
                                        let mut p: mut Vec<u8> =
                                            &mut in_0                                          mut Vec<u8>;
                                        let mut netsize: i32 =
                                            atoi(slash);
                                        let fresh8 = op;
                                        op = op.offset(1);
                                        *fresh8 = netsize;
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
                                                                                         libc::c_int;
                                        current_block = 15439134456549723682;
                                    }
                                }
                            } else if is_addr6 != 0 && is6 != 0 {
                                let mut op_0: mut Vec<u8> =
                                    0;
                                op_0 =
                                    opt_malloc((16 * addrs)usize)                                  mut Vec<u8>;
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
                                           1
                                           &&
                                           cp.offset(strlen(cp).wrapping_sub(1
                                                                                                                libc::c_int
                                                                                                         )
                                                         )                                         libc::c_int == ']' as i32 {
                                        cp.offset(strlen(cp).wrapping_sub(1
                                                                                                          libc::c_int
                                                                                                   )
                                                      ) =
                                            0
                                    }
                                    if inet_pton(10, cp,
                                                 op_0) !=
                                           0 {
                                        op_0 =
                                            op_0.offset(16         isize)
                                    } else {
                                        strcpy(errstr,
                                               "bad IPv6 address"*const u8*const libc::c_char);
                                        current_block = 14151404249770905673;
                                        break ;
                                    }
                                }
                                match current_block {
                                    14151404249770905673 => { }
                                    _ => {
                                        new.len =
                                            op_0.wrapping_offset_from(new.val)
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
                                    let mut q: mut Vec<u8> =
                                        0;
                                    let mut r: mut Vec<u8> =
                                        0;
                                    let mut tail: mut Vec<u8> =
                                        0;
                                    let mut p_0: mut Vec<u8> =
                                        0;
                                    let mut m: mut Vec<u8> =
                                        0;
                                    let mut newp: mut Vec<u8> =
                                        0;
                                    let mut newlen: usize = 0;
                                    let mut len: usize =
                                        0 ;
                                    let mut header_size: i32 =
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
                                        let mut in_1: &mut String =
                                            0 ;
                                        let mut dom: &mut String =
                                            0 ;
                                        let mut domlen: usize =
                                            1 ;
                                        /* Allow "." as an empty domain */
                                        if strcmp(arg,
                                                  "."    *const libc::c_char) !=
                                               0 {
                                            dom = canonicalise_opt(arg);
                                            if dom.is_null() {
                                                strcpy(errstr,
                                                       "bad domain in dhcp-option"
                                                                   *const libc::c_char);
                                                current_block =
                                                    14151404249770905673;
                                                break ;
                                            } else {
                                                domlen =
                                                    strlen(dom).wrapping_add(2
                                                                                                              libc::c_int
                                                                                                       )
                                            }
                                        }
                                        newp =
                                            opt_malloc(len.wrapping_add(domlen).wrapping_add(header_size          ))
                                               ;
                                        if !m.is_null() {
                                            memcpy(newp,
                                                   m,
                                                   (header_size     libc::c_ulong).wrapping_add(len));
                                            free(m);
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
                                            let mut cp_0: mut Vec<u8> =
                                                fresh18;
                                            let mut j: i32 = 0;
                                            j = 0;
                                            while *in_1 != 0 &&
                                                      *in_1 !=
                                                          '.' as i32 {
                                                let fresh19 = q;
                                                q = q.offset(1);
                                                *fresh19 =
                                                    *in_1;
                                                in_1 = in_1.offset(1);
                                                j += 1
                                            }
                                            cp_0 = j;
                                            if *in_1 != 0 {
                                                in_1 = in_1.offset(1)
                                            }
                                        }
                                        let fresh20 = q;
                                        q = q.offset(1);
                                        *fresh20 =
                                            0;
                                        free(dom);
                                        /* Now tail-compress using earlier names. */
                                        newlen =
                                            q.wrapping_offset_from(p_0)                                          i32 ;
                                        tail = p_0.offset(len);
                                        's_1219:
                                            while *tail != 0 {
                                                r = p_0;
                                                while (r.wrapping_offset_from(p_0)
                                                          ) <
                                                          len

                                                      {
                                                    if strcmp(r               &mut String,
                                                              tail               &mut String)
                                                           == 0
                                                       {
                                                        let mut t_s: u16 =
                                                            (r.wrapping_offset_from(p_0)
                                                                              i32
                                                                 |
                                                                 0xc000                  libc::c_int
                                                                                      i32)
                                                               ;
                                                        let mut t_cp:
                                                                mut Vec<u8> =
                                                            tail;
                                                        let fresh21 = t_cp;
                                                        t_cp = t_cp.offset(1);
                                                        *fresh21 =
                                                            (t_s              libc::c_int
                                                                 >>
                                                                 8                  libc::c_int)
                                                                            libc::c_uchar;
                                                        *t_cp =
                                                            t_s             libc::c_uchar;
                                                        tail =
                                                            tail.offset(2                         libc::c_int
                                                              );
                                                        newlen =
                                                            tail.wrapping_offset_from(p_0)
                                                                            i32;
                                                        break 's_1219 ;
                                                    } else {
                                                        r =
                                                            r.offset((*r                       libc::c_int
                                                                          +
                                                                          1                           libc::c_int)
                                                        )
                                                    }
                                                }
                                                tail =
                                                    tail.offset((*tail                  libc::c_int
                                                                     +
                                                                     1                      libc::c_int)
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
                                                *m.offset(0           isize) =
                                                    0     libc::c_uchar
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
                                    let mut i_0: i32 = 0;
                                    let mut commas: i32 =
                                        1;
                                    let mut p_1: mut Vec<u8> =
                                        0;
                                    let mut newp_0: mut Vec<u8> =
                                        0;
                                    i_0 = 0;
                                    while *comma.offset(i_0) != 0 {
                                        if *comma.offset(i_0)                                         libc::c_int == ',' as i32 {
                                            commas += 1
                                        }
                                        i_0 += 1
                                    }
                                    newp_0 =
                                        opt_malloc(strlen(comma).wrapping_add((2
                                                                                                                  libc::c_int
                                                                                   *
                                                                                   commas)
                                                                                                         ))
                                           ;
                                    p_1 = newp_0;
                                    arg = comma;
                                    comma = split(arg);
                                    while !arg.is_null() &&
                                              *arg != 0 {
                                        let mut len_0: u16 =
                                            strlen(arg);
                                        unhide_metas(arg);
                                        let mut t_s_0: u16 = len_0;
                                        let mut t_cp_0: mut Vec<u8> =
                                            p_1;
                                        let fresh22 = t_cp_0;
                                        t_cp_0 = t_cp_0.offset(1);
                                        *fresh22 =
                                            (t_s_0 >>
                                                 8)                                          libc::c_uchar;
                                        *t_cp_0 = t_s_0;
                                        p_1 =
                                            p_1.offset(2        isize);
                                        memcpy(p_1,
                                               arg,
                                               len_0);
                                        p_1 =
                                            p_1.offset(len_0        isize);
                                        arg = comma;
                                        comma = split(arg)
                                    }
                                    new.val = newp_0;
                                    new.len =
                                        p_1.wrapping_offset_from(newp_0)                                      i32;
                                    current_block = 15439134456549723682;
                                } else if !comma.is_null() &&
                                              opt_len &
                                                  0x4000 != 0 {
                                    let mut p_2: mut Vec<u8> =
                                        0;
                                    let mut q_0: mut Vec<u8> =
                                        0;
                                    let mut newp_1: mut Vec<u8> =
                                        0;
                                    let mut end: mut Vec<u8> =
                                        0;
                                    let mut len_1: i32 =
                                        0;
                                    let mut header_size_0: i32 =
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
                                        let mut dom_0: &mut String =
                                            canonicalise_opt(arg);
                                        if dom_0.is_null() {
                                            strcpy(errstr,
                                                   "bad domain in dhcp-option"
                                                           *const libc::c_char);
                                            current_block =
                                                14151404249770905673;
                                            break ;
                                        } else {
                                            newp_1 =
                                                opt_malloc(((len_1 +
                                                                 header_size_0)
                                                                     ).wrapping_add(strlen(dom_0)).wrapping_add(2                                                                       libc::c_int                                                                ))
                                                   ;
                                            if !p_2.is_null() {
                                                memcpy(newp_1       Vec<u8>,
                                                       p_2        *const libc::c_void,
                                                       len_1 );
                                                free(p_2     Vec<u8>);
                                            }
                                            p_2 = newp_1;
                                            q_0 = p_2.offset(len_1);
                                            end =
                                                do_rfc1035_name(q_0.offset(header_size_0
                                                                    ),
                                                                dom_0,
                                                                0                 &mut String);
                                            let fresh23 = end;
                                            end = end.offset(1);
                                            *fresh23 =
                                                0 libc::c_uchar;
                                            if is6 != 0 &&
                                                   new.opt ==
                                                       56 {
                                                let mut t_s_1: u16 =
                                                    3;
                                                let mut t_cp_1:
                                                        mut Vec<u8> =
                                                    q_0;
                                                let fresh24 = t_cp_1;
                                                t_cp_1 = t_cp_1.offset(1);
                                                *fresh24 =
                                                    (t_s_1 >>
                                                         8)     libc::c_uchar;
                                                *t_cp_1 =
                                                    t_s_1;
                                                q_0 =
                                                    q_0.offset(2                libc::c_int
                                                                  );
                                                let mut t_s_2: u16 =
                                                    (end.wrapping_offset_from(q_0)
                                                         -
                                                         2          i32)     u16;
                                                let mut t_cp_2:
                                                        mut Vec<u8> =
                                                    q_0;
                                                let fresh25 = t_cp_2;
                                                t_cp_2 = t_cp_2.offset(1);
                                                *fresh25 =
                                                    (t_s_2 >>
                                                         8)     libc::c_uchar;
                                                *t_cp_2 =
                                                    t_s_2;
                                                q_0 =
                                                    q_0.offset(2                libc::c_int
                                                                  )
                                            }
                                            len_1 =
                                                end.wrapping_offset_from(p_2)
                                                    libc::c_int;
                                            free(dom_0);
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
                                        opt_string_alloc(comma)                                      mut Vec<u8>;
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
                                   "dhcp-option too long"                                  *const libc::c_char);
                        } else {
                            if flags == 128 {
                                if new.flags &
                                       (4 | 256)
                                       != 0 || new.netid.is_null() ||
                                       !(*new.netid).next.is_null() {
                                    strcpy(errstr,
                                           "illegal dhcp-match"                                         *const u8                                         *const libc::c_char);
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
pub  fn set_option_bool(mut opt: u32) {
    daemon.options[(opt).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                             ).wrapping_mul(8                       libc::c_int                ))
                                  ] |=
        (1) <<
            (opt    ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                  ).wrapping_mul(8
                                                                                                                libc::c_int
                                                                                                         ));
}
#[no_mangle]
pub  fn reset_option_bool(mut opt: u32) {
    daemon.options[(opt).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                             ).wrapping_mul(8                       libc::c_int                ))
                                  ] &=
        !((1) <<
              (opt).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                             ).wrapping_mul(8
                                                                                                                    libc::c_int
                                                                                                             )));
}
 fn server_list_free(mut list: Server) {
    while !list.is_null() {
        let mut tmp: Server = list;
        list = list.next;
        free(tmp);
    };
}
 fn one_opt(mut option: i32,
                             mut arg: &mut String,
                             mut errstr: &mut String,
                             mut gen_err: &mut String,
                             mut command_line: i32,
                             mut servers_only: i32) -> i32 {
    let mut current_block: u64;
    let mut i: i32 = 0;
    let mut comma: &mut String = 0 ;
    if option == '?' as i32 {
        strcpy(errstr, gen_err);
        return 0
    }
    i = 0;
    while usage[i ].opt != 0 {
        if usage[i ].opt == option {
            let mut rept: i32 = usage[i ].rept;
            if command_line != 0 {
                /* command line */
                if rept == 62 + 2 {
                    strcpy(errstr,
                           "illegal repeated flag"                          *const libc::c_char);
                    return 0
                }
                if rept == 62 + 1 {
                    usage[i ].rept =
                        (62 + 2)
                }
            } else {
                /* allow file to override command line */
                if rept == 62 + 3 {
                    strcpy(errstr,
                           "illegal repeated keyword"                          *const libc::c_char);
                    return 0
                }
                if rept == 62 + 2 ||
                       rept == 62 + 1 {
                    usage[i ].rept =
                        (62 + 3)
                }
            }
            if rept != 62 &&
                   rept != 62 + 1 &&
                   rept != 62 + 2 {
                set_option_bool(rept);
                return 1
            }
            break ;
        } else { i += 1 }
    }
    match option {
        67 => {
            /* --conf-file */
            let mut file: &mut String = opt_string_alloc(arg);
            if !file.is_null() {
                one_file(file, 0);
                free(file);
            }
            current_block = 7879481898411272068;
        }
        55 => {
            /* --conf-dir */
            let mut dir_stream: DIR = 0 ;
            let mut ent: dirent = 0 ent;
            let mut directory: &mut String = 0 ;
            let mut path: &mut String = 0 ;
            let mut ignore_suffix: list = 0 ;
            let mut match_suffix: list = 0 ;
            let mut files: list = 0 ;
            let mut li: list = 0 ;
            comma = split(arg);
            directory = opt_string_alloc(arg);
            if directory.is_null() {
                current_block = 7879481898411272068;
            } else {
                arg = comma;
                while !arg.is_null() {
                    comma = split(arg);
                    if strlen(arg) != 0 {
                        li =
                            opt_malloc(::std::mem::size_of::<list>()                              ) ;
                        if *arg == '*' as i32 {
                            /* "*" with no suffix is a no-op */
                            if *arg.offset(1)                             libc::c_int == 0 {
                                free(li);
                            } else {
                                li.next = match_suffix;
                                match_suffix = li;
                                /* Have to copy: buffer is overwritten */
                                li.name =
                                    opt_string_alloc(arg.offset(1                 libc::c_int
                                                                   ))
                            }
                        } else {
                            li.next = ignore_suffix;
                            ignore_suffix = li;
                            /* Have to copy: buffer is overwritten */
                            li.name = opt_string_alloc(arg)
                        }
                    }
                    arg = comma
                }
                dir_stream = opendir(directory);
                if dir_stream.is_null() {
                    die("cannot access directory %s: %s"  ,
                        directory, 3);
                }
                loop  {
                    ent = readdir(dir_stream);
                    if ent.is_null() { break ; }
                    let mut len: usize = strlen(ent.d_name.as_mut_ptr());
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
                    if len == 0 ||
                           ent.d_name[len.wrapping_sub(1    )
                                             ] ==
                               '~' as i32 ||
                           ent.d_name[0 ] == '#' as i32 &&
                               ent.d_name[len.wrapping_sub(1
                                                                         )
                                                 ] ==
                                   '#' as i32 ||
                           ent.d_name[0 ] == '.' as i32 {
                        continue ;
                    }
                    if !match_suffix.is_null() {
                        li = match_suffix;
                        while !li.is_null() {
                            /* check for required suffices */
                            let mut ls: usize = strlen(li.name);
                            if len > ls &&
                                   strcmp(li.name,
                                          &mut *ent.d_name.as_mut_ptr().offset(len.wrapping_sub(ls)
                                                                                  ))
                                       == 0 {
                                break ;
                            }
                            li = li.next
                        }
                        if li.is_null() { continue ; }
                    }
                    li = ignore_suffix;
                    while !li.is_null() {
                        /* check for proscribed suffices */
                        let mut ls_0: usize = strlen(li.name);
                        if len > ls_0 &&
                               strcmp(li.name,
                                      &mut *ent.d_name.as_mut_ptr().offset(len.wrapping_sub(ls_0)
                                                                          ))
                                   == 0 {
                            break ;
                        }
                        li = li.next
                    }
                    if !li.is_null() { continue ; }
                    path =
                        opt_malloc(strlen(directory).wrapping_add(len).wrapping_add(2
                                                                                                                            libc::c_int
                                                                                                                     ))
                            ;
                    strcpy(path, directory);
                    strcat(path,
                           "/" );
                    strcat(path, ent.d_name.as_mut_ptr());
                    /* files must be readable */
                    if stat(path, &mut buf) == -(1) {
                        die("cannot access %s: %s"                           *const libc::c_char ,
                            path, 3);
                    }
                    /* only reg files allowed. */
                    if buf.st_mode & 0o170000
                           == 0o100000 {
                        /* sort files into order. */
                        let mut up: list;
                        let mut new: list =
                            opt_malloc(::std::mem::size_of::<list>()                              ) ;
                        new.name = path;
                        up = &mut files;
                        li = files;
                        while !li.is_null() {
                            if strcmp(li.name, path) >= 0 {
                                break ;
                            }
                            up = &mut li.next;
                            li = li.next
                        }
                        new.next = li;
                        *up = new
                    }
                }
                li = files;
                while !li.is_null() {
                    one_file(li.name, 0);
                    li = li.next
                }
                closedir(dir_stream);
                free(directory);
                while !ignore_suffix.is_null() {
                    li = ignore_suffix.next;
                    free(ignore_suffix.name);
                    free(ignore_suffix);
                    ignore_suffix = li
                }
                while !match_suffix.is_null() {
                    li = match_suffix.next;
                    free(match_suffix.name);
                    free(match_suffix);
                    match_suffix = li
                }
                while !files.is_null() {
                    li = files.next;
                    free(files.name);
                    free(files);
                    files = li
                }
                current_block = 7879481898411272068;
            }
        }
        325 => {
            /* --add-subnet */
            set_option_bool(41);
            if !arg.is_null() {
                let mut err: &mut String = 0 ;
                let mut end: &mut String = 0 ;
                comma = split(arg);
                let mut new_0: Mysubnet =
                    opt_malloc(::std::mem::size_of::<Mysubnet>()) ;
                end = split_chr(arg, '/' );
                if !end.is_null() {
                    /* has subnet+len */
                    err = parse_NetAddress(arg, &mut new_0.addr);
                    if !err.is_null() {
                        strcpy(errstr, err);
                        free(new_0);
                        return 0
                    }
                    if atoi_check(end, &mut new_0.mask) == 0 {
                        strcpy(errstr, gen_err);
                        free(new_0);
                        return 0
                    }
                    new_0.addr_used = 1
                } else if atoi_check(arg, &mut new_0.mask) == 0 {
                    strcpy(errstr, gen_err);
                    free(new_0);
                    return 0
                }
                daemon.add_subnet4 = new_0;
                if !comma.is_null() {
                    new_0 =
                        opt_malloc(::std::mem::size_of::<Mysubnet>()                          ) ;
                    end = split_chr(comma, '/' );
                    if !end.is_null() {
                        /* has subnet+len */
                        err = parse_NetAddress(comma, &mut new_0.addr);
                        if !err.is_null() {
                            strcpy(errstr, err);
                            free(new_0);
                            return 0
                        }
                        if atoi_check(end, &mut new_0.mask) == 0 {
                            strcpy(errstr, gen_err);
                            free(new_0);
                            return 0
                        }
                        new_0.addr_used = 1
                    } else if atoi_check(comma, &mut new_0.mask) == 0 {
                        strcpy(errstr, gen_err);
                        free(new_0);
                        return 0
                    }
                    daemon.add_subnet6 = new_0
                }
            }
            current_block = 7879481898411272068;
        }
        49 => {
            /* --enable-dbus */
            set_option_bool(19);
            if !arg.is_null() {
                daemon.dbus_name = opt_string_alloc(arg)
            } else {
                daemon.dbus_name =
                    "uk.org.thekelleys.dnsmasq"                   *const libc::c_char
            }
            current_block = 7879481898411272068;
        }
        354 => {
            /* --enable-ubus */
            set_option_bool(58);
            if !arg.is_null() {
                daemon.ubus_name = opt_string_alloc(arg)
            } else {
                daemon.ubus_name =
                    "dnsmasq"                   &mut String
            }
            current_block = 7879481898411272068;
        }
        56 => {
            /* --log-facility */
            /* may be a filename */
            if !strchr(arg, '/' as i32).is_null() ||
                   strcmp(arg, "-" )
                       == 0 {
                daemon.log_file = opt_string_alloc(arg)
            } else {
                i = 0;
                while !FACILITYNAMES[i ].c_name.is_null() {
                    if hostname_isequal(FACILITYNAMES[i ].c_name, arg)
                           != 0 {
                        break ;
                    }
                    i += 1
                }
                if !FACILITYNAMES[i ].c_name.is_null() {
                    daemon.log_fac =
                        FACILITYNAMES[i ].c_val
                } else {
                    strcpy(errstr,
                           "bad log facility"                          *const libc::c_char);
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
            let mut name: &mut String = opt_string_alloc(arg);
            let mut new_1: Resolvc = 0 ;
            let mut list_0: Resolvc = daemon.resolv_files;
            if !list_0.is_null() && list_0.is_default != 0 {
                /* replace default resolv file - possibly with nothing */
                if !name.is_null() {
                    list_0.is_default = 0;
                    list_0.name = name
                } else { list_0 = 0  }
            } else if !name.is_null() {
                new_1 =
                    opt_malloc(::std::mem::size_of::<Resolvc>()) ;
                new_1.next = list_0;
                new_1.name = name;
                new_1.is_default = 0;
                new_1.mtime = 0;
                new_1.logged = 0;
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
            let mut pref: i32 = 1; /* may be NULL */
            let mut new_2: MxSrvRecord = 0 ;
            let mut name_0: &mut String = 0 ;
            let mut target: &mut String = 0 ;
            comma = split(arg);
            if !comma.is_null() {
                let mut prefstr: &mut String = 0 ;
                prefstr = split(comma);
                if !prefstr.is_null() && atoi_check16(prefstr, &mut pref) == 0
                   {
                    strcpy(errstr,
                           "bad MX preference"                          *const libc::c_char);
                    return 0
                }
            }
            name_0 = canonicalise_opt(arg);
            if name_0.is_null() ||
                   !comma.is_null() &&
                       { target = canonicalise_opt(comma); target.is_null() }
               {
                strcpy(errstr,
                       "bad MX name" );
                return 0
            }
            new_2 =
                opt_malloc(::std::mem::size_of::<MxSrvRecord>()) ;
            new_2.next = daemon.mxnames;
            daemon.mxnames = new_2;
            new_2.issrv = 0;
            new_2.name = name_0;
            new_2.target = target;
            new_2.weight = pref;
            current_block = 7879481898411272068;
        }
        116 => {
            /*  --mx-target */
            daemon.mxtarget = canonicalise_opt(arg);
            if daemon.mxtarget.is_null() {
                strcpy(errstr,
                       "bad MX target" );
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
                strtol(arg, 0 , 0) ;
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
                       "recompile with HAVE_LUASCRIPT defined to enable Lua scripts"
                           );
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
                let mut new_4: Iname =
                    opt_malloc(::std::mem::size_of::<Iname>());
                comma = split(arg);
                new_4.name = 0 ;
                unhide_metas(arg);
                if inet_pton(2, arg,
                             &mut new_4.addr.in_0.sin_addr
                                ) > 0 {
                    new_4.addr.sa.sa_family =
                        2
                } else if inet_pton(10, arg,
                                    &mut new_4.addr.in6.sin6_addr                                   &mut In6Addr)  >
                              0 {
                    new_4.addr.sa.sa_family =
                        10
                } else {
                    let mut fam: &mut String =
                        split_chr(arg, '/' );
                    new_4.name = opt_string_alloc(arg);
                    new_4.addr.sa.sa_family =
                        0;
                    if !fam.is_null() {
                        if strcmp(fam,
                                  "4") == 0
                           {
                            new_4.addr.sa.sa_family =
                                2
                        } else if strcmp(fam,
                                         "6") ==
                                      0 {
                            new_4.addr.sa.sa_family =
                                10
                        } else {
                            free(new_4.name);
                            strcpy(errstr, gen_err);
                            free(new_4);
                            return 0
                        }
                    }
                }
                new_4.next = daemon.authinterface;
                daemon.authinterface = new_4
            }
            current_block = 7879481898411272068;
        }
        317 => {
            /* --auth-sec-servers */
            let mut new_5: NameListEntry = 0 ;
            loop  {
                comma = split(arg);
                new_5 =
                    opt_malloc(::std::mem::size_of::<NameListEntry>()) ;
                new_5.name = opt_string_alloc(arg);
                new_5.next = daemon.secondary_forward_server;
                daemon.secondary_forward_server = new_5;
                arg = comma;
                if arg.is_null() { break ; }
            }
            current_block = 7879481898411272068;
        }
        313 => {
            /* --auth-zone */
            let mut new_6: AuthZone = 0 ;
            comma = split(arg);
            new_6 =
                opt_malloc(::std::mem::size_of::<AuthZone>()) ;
            new_6.domain = opt_string_alloc(arg);
            new_6.subnet = 0 ;
            new_6.exclude = 0 ;
            new_6.interface_names = 0 ;
            new_6.next = daemon.auth_zones;
            daemon.auth_zones = new_6;
            loop  {
                arg = comma;
                if arg.is_null() { break ; }
                let mut prefixlen: i32 = 0;
                let mut is_exclude: i32 = 0;
                let mut prefix: &mut String = 0 ;
                let mut subnet: AddressListEntry = 0 ;
                let mut addr: NetAddress =
                    NetAddress {addr4: NetAddress {s_addr: 0,},};
                comma = split(arg);
                prefix = split_chr(arg, '/' );
                if !prefix.is_null() &&
                       atoi_check(prefix, &mut prefixlen) == 0 {
                    strcpy(errstr, gen_err);
                    return 0
                }
                if strstr(arg,
                          "exclude:" )
                       == arg {
                    is_exclude = 1;
                    arg = arg.offset(8)
                }
                if inet_pton(2, arg,
                             &mut addr.addr4                          Vec<u8>) != 0 {
                    subnet =
                        opt_malloc(::std::mem::size_of::<AddressListEntry>()                          ) ;
                    subnet.prefixlen =
                        if prefixlen == 0 {
                            24
                        } else { prefixlen };
                    subnet.flags = 1
                } else if inet_pton(10, arg,
                                    &mut addr.addr6                                 Vec<u8>) != 0 {
                    subnet =
                        opt_malloc(::std::mem::size_of::<AddressListEntry>()                          ) ;
                    subnet.prefixlen =
                        if prefixlen == 0 {
                            64
                        } else { prefixlen };
                    subnet.flags = 1 | 2
                } else {
                    let mut name_1: AuthNameList =
                        opt_malloc(::std::mem::size_of::<AuthNameList>()                          ) ;
                    name_1.name = opt_string_alloc(arg);
                    name_1.flags = 2 | 1;
                    name_1.next = new_6.interface_names;
                    new_6.interface_names = name_1;
                    if !prefix.is_null() {
                        if prefixlen == 4 {
                            name_1.flags &= !(1)
                        } else if prefixlen == 6 {
                            name_1.flags &= !(2)
                        } else {
                            strcpy(errstr, gen_err);
                            return 0
                        }
                    }
                }
                if !subnet.is_null() {
                    subnet.addr = addr;
                    if is_exclude != 0 {
                        subnet.next = new_6.exclude;
                        new_6.exclude = subnet
                    } else {
                        subnet.next = new_6.subnet;
                        new_6.subnet = subnet
                    }
                }
            }
            current_block = 7879481898411272068;
        }
        316 => {
            /* --auth-soa */
            comma = split(arg);
            daemon.soa_sn = atoi(arg);
            if !comma.is_null() {
                let mut cp: &mut String = 0 ;
                arg = comma;
                comma = split(arg);
                daemon.hostmaster = opt_string_alloc(arg);
                cp = daemon.hostmaster;
                while cp != 0 {
                    if cp == '@' as i32 {
                        cp = '.'
                    }
                    cp = cp.offset(1)
                }
                if !comma.is_null() {
                    arg = comma;
                    comma = split(arg);
                    daemon.soa_refresh =
                        atoi(arg);
                    if !comma.is_null() {
                        arg = comma;
                        comma = split(arg);
                        daemon.soa_retry =
                            atoi(arg);
                        if !comma.is_null() {
                            daemon.soa_expiry =
                                atoi(comma)
                        }
                    }
                }
            }
            current_block = 7879481898411272068;
        }
        115 | 320 => {
            /* --domain */
            /* --synth-domain */
            if strcmp(arg, "#" ) ==
                   0 {
                set_option_bool(15);
            } else {
                let mut d: &mut String = 0 ;
                comma = split(arg);
                d = canonicalise_opt(arg);
                if d.is_null() {
                    strcpy(errstr, gen_err);
                    return 0
                } else {
                    if !comma.is_null() {
                        let mut new_7: CondDomain =
                            opt_malloc(::std::mem::size_of::<CondDomain>()                              ) ;
                        let mut netpart: &mut String =
                            0 ;
                        new_7.prefix = 0 ;
                        new_7.indexed = 0;
                        unhide_metas(comma);
                        netpart =
                            split_chr(comma, '/' );
                        if !netpart.is_null() {
                            let mut msize: i32 = 0;
                            arg = split(netpart);
                            if atoi_check(netpart, &mut msize) == 0 {
                                strcpy(errstr, gen_err);
                                free(new_7);
                                return 0
                            } else {
                                if inet_pton(2, comma,
                                             &mut new_7.start                                           NetAddress                                          Vec<u8>) != 0 {
                                    let mut mask: i32 =
                                        ((1) <<
                                             32 - msize) -
                                            1;
                                    new_7.is6 = 0;
                                    new_7.start.s_addr =
                                        __bswap_32(__bswap_32(new_7.start.s_addr)
                                                       &
                                                       !mask);
                                    new_7.end.s_addr =
                                        new_7.start.s_addr |
                                            __bswap_32(mask);
                                    if !arg.is_null() {
                                        if option != 's' as i32 {
                                            new_7.prefix =
                                                canonicalise_opt(arg);
                                            if new_7.prefix.is_null() ||
                                                   strlen(new_7.prefix) >
                                                       (63 -
                                                            16)
                                                           {
                                                strcpy(errstr,
                                                       "bad prefix"        *const u8        *const libc::c_char);
                                                free(new_7     Vec<u8>);
                                                return 0
                                            }
                                        } else if strcmp(arg,
                                                         "local"          *const u8          *const libc::c_char)
                                                      != 0 ||
                                                      msize !=
                                                          8 &&
                                                          msize !=
                                                              16               libc::c_int
                                                          &&
                                                          msize !=
                                                              24               libc::c_int
                                         {
                                            strcpy(errstr, gen_err);
                                            free(new_7);
                                            return 0
                                        } else {
                                            /* generate the equivalent of
				      local=/xxx.yyy.zzz.in-addr.arpa/ */
                                            let mut serv: Server =
                                                add_rev4(new_7.start,
                                                         msize);
                                            if serv.is_null() {
                                                strcpy(errstr, "bad prefix");
                                                return 0
                                            }
                                            serv.flags |= 2;
                                            /* local=/<domain>/ */
                                            serv = Default::default();
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
                                                    &mut new_7.start6     &mut In6Addr    Vec<u8>) !=
                                              0 {
                                    let mut mask_0: u64 =
                                        ((1long) <<
                                             128 -
                                                 msize).wrapping_sub(1                      libc::c_ulonglong);
                                    let mut addrpart: u64 =
                                        addr6part(&mut new_7.start6);
                                    new_7.is6 = 1;
                                    /* prefix==64 overflows the mask calculation above */
                                    if msize == 64 {
                                        mask_0 =
                                            -(1long) as u64
                                    }
                                    new_7.end6 = new_7.start6;
                                    setaddr6part(&mut new_7.start6,
                                                 addrpart & !mask_0);
                                    setaddr6part(&mut new_7.end6,
                                                 addrpart | mask_0);
                                    if msize < 64 {
                                        strcpy(errstr, gen_err);
                                        free(new_7);
                                        return 0
                                    } else {
                                        if !arg.is_null() {
                                            if option != 's' as i32 {
                                                new_7.prefix =
                                                    canonicalise_opt(arg);
                                                if new_7.prefix.is_null()
                                                       ||
                                                       strlen(new_7.prefix)
                                                           >
                                                           (63
                                                                -
                                                                46                 libc::c_int)
                                                                          libc::c_ulong {
                                                    strcpy(errstr,
                                                           "bad prefix"
                                                                           *const libc::c_char);
                                                    free(new_7         Vec<u8>);
                                                    return 0
                                                }
                                            } else if strcmp(arg,
                                                             "local"              *const u8              *const libc::c_char)
                                                          != 0
                                                          ||
                                                          msize &
                                                              4
                                                              !=
                                                              0
                                             {
                                                strcpy(errstr, gen_err);
                                                free(new_7     Vec<u8>);
                                                return 0
                                            } else {
                                                /* generate the equivalent of
				     local=/xxx.yyy.zzz.ip6.arpa/ */
                                                let mut serv_0: Server =
                                                    add_rev6(&mut new_7.start6,
                                                             msize);
                                                serv_0.flags |=
                                                    2;
                                                /* local=/<domain>/ */
                                                serv_0 =
                                                    opt_malloc(::std::mem::size_of::<Server>()
                                                                           )
                                                       ;
                                                serv_0.domain = d;
                                                serv_0.flags =
                                                    8 |
                                                        2;
                                                serv_0.next =
                                                    daemon.servers;
                                                daemon.servers =
                                                    serv_0
                                            }
                                        }
                                    }
                                } else {
                                    strcpy(errstr, gen_err);
                                    free(new_7);
                                    return 0
                                }
                            }
                        } else {
                            let mut prefstr_0: &mut String =
                                0 ;
                            arg = split(comma);
                            prefstr_0 = split(arg);
                            if inet_pton(2, comma,
                                         &mut new_7.start
                                            ) != 0 {
                                new_7.is6 = 0;
                                if arg.is_null() {
                                    new_7.end.s_addr =
                                        new_7.start.s_addr
                                } else if inet_pton(2, arg,
                                                    &mut new_7.end     NetAddress    Vec<u8>) ==
                                              0 {
                                    strcpy(errstr, gen_err);
                                    free(new_7);
                                    return 0
                                }
                            } else if inet_pton(10, comma,
                                                &mut new_7.start6 Vec<u8>) != 0 {
                                new_7.is6 = 1;
                                if arg.is_null() {
                                    memcpy(&mut new_7.end6
                                              ,
                                           &mut new_7.start6                                         &mut In6Addr
                                           16);
                                } else if inet_pton(10, arg,
                                                    &mut new_7.end6     &mut In6Addr    Vec<u8>) ==
                                              0 {
                                    strcpy(errstr, gen_err);
                                    free(new_7);
                                    return 0
                                }
                            } else {
                                strcpy(errstr, gen_err);
                                free(new_7);
                                return 0
                            }
                            if option != 's' as i32 && !prefstr_0.is_null() {
                                new_7.prefix = canonicalise_opt(prefstr_0);
                                if new_7.prefix.is_null() ||
                                       strlen(new_7.prefix) >
                                           (63 -
                                                16) {
                                    strcpy(errstr,
                                           "bad prefix" );
                                    free(new_7);
                                    return 0
                                }
                            }
                        }
                        new_7.domain = d;
                        if option == 's' as i32 {
                            new_7.next = daemon.cond_domain;
                            daemon.cond_domain = new_7
                        } else {
                            let mut star: &mut String =
                                0 ;
                            new_7.next = daemon.synth_domains;
                            daemon.synth_domains = new_7;
                            if !new_7.prefix.is_null() &&
                                   {
                                       star =
                                           strrchr(new_7.prefix,
                                                   '*' as i32);
                                       !star.is_null()
                                   } &&
                                   *star.offset(1)                                 libc::c_int == 0 {
                                *star = 0;
                                new_7.indexed = 1
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
                set_option_bool(32);
            } else {
                unhide_metas(arg);
                if strcmp(arg,
                          "base64" )
                       == 0 {
                    set_option_bool(54);
                } else if strcmp(arg,
                                 "text"                                *const libc::c_char) == 0
                 {
                    set_option_bool(55);
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
                let mut new_8: Iname =
                    opt_malloc(::std::mem::size_of::<Iname>());
                comma = split(arg);
                new_8.next = daemon.if_names;
                daemon.if_names = new_8;
                /* new->name may be NULL if someone does
	   "interface=" to disable all interfaces except loop. */
                new_8.name = opt_string_alloc(arg);
                new_8.used = 0;
                arg = comma;
                if arg.is_null() { break ; }
            }
            current_block = 7879481898411272068;
        }
        258 => {
            /* --enable-tftp */
            set_option_bool(40);
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
                let mut new_10: Iname =
                    opt_malloc(::std::mem::size_of::<Iname>());
                comma = split(arg);
                unhide_metas(arg);
                if !arg.is_null() &&
                       inet_pton(2, arg,
                                 &mut new_10.addr.in_0.sin_addr                               NetAddress) >
                           0 {
                    new_10.addr.sa.sa_family =
                        2;
                    new_10.addr.in_0.sin_port =
                        0 as in_port_t
                } else if !arg.is_null() &&
                              inet_pton(10, arg,
                                        &mut new_10.addr.in6.sin6_addr                                      &mut In6Addr                                     Vec<u8>) >
                                  0 {
                    new_10.addr.sa.sa_family =
                        10;
                    new_10.addr.in6.sin6_flowinfo =
                        0;
                    new_10.addr.in6.sin6_scope_id =
                        0;
                    new_10.addr.in6.sin6_port =
                        0 as in_port_t
                } else {
                    strcpy(errstr, gen_err);
                    free(new_10);
                    return 0
                }
                new_10.used = 0;
                if option == 'a' as i32 {
                    new_10.next = daemon.if_addrs;
                    daemon.if_addrs = new_10
                } else {
                    new_10.next = daemon.auth_peers;
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
            let mut string: &mut String = 0 ;
            let mut size: i32 = 0;
            let mut serv_2: Server = 0;
            let mut addr4: NetAddress = NetAddress {s_addr: 0,};
            let mut addr6: In6Addr =
                In6Addr {__in6_u: C2RustUnnamed{__u6_addr8: [0; 16],},};
            unhide_metas(arg);
            if arg.is_null() {
                strcpy(errstr, gen_err);
                return 0
            }
            comma = split(arg);
            string = split_chr(arg, '/' );
            if string.is_null() || atoi_check(string, &mut size) == 0 {
                strcpy(errstr, gen_err);
                return 0
            }
            if inet_pton(2, arg,
                         &mut addr4) != 0
               {
                serv_2 = add_rev4(addr4, size);
                if serv_2.is_null() {
                    strcpy(errstr,
                           "bad prefix"                          *const libc::c_char);
                    return 0
                }
            } else if inet_pton(10, arg,
                                &mut addr6                             Vec<u8>) != 0 {
                serv_2 = add_rev6(&mut addr6, size)
            } else { strcpy(errstr, gen_err); return 0 }
            string =
                parse_server(comma, &mut serv_2.addr,
                             &mut serv_2.source_addr,
                             serv_2.interface.as_mut_ptr(),
                             &mut serv_2.flags);
            if !string.is_null() {
                strcpy(errstr, string);
                return 0
            }
            if servers_only != 0 { serv_2.flags |= 4096 }
            current_block = 7879481898411272068;
        }
        319 => {
            /* --ipset */
            let mut ipsets_head: IpSets = Default::default();
            let mut ipsets: IpSets = Default::default();
            let mut size_0: i32 = 0;
            let mut end_1: String;
            let mut sets: String;
            let mut sets_pos: String;
            unhide_metas(arg);
            if !arg.is_null() && *arg == '/' as i32 {
                arg = arg.offset(1);
                loop  {
                    end_1 = split_chr(arg, '/' );
                    if end_1.is_null() { break ; }
                    let mut domain_0: String;
                    /* elide leading dots - they are implied in the search algorithm */
                    while *arg == '.' as i32 {
                        arg = arg.offset(1)
                    }
                    /* # matches everything and becomes a zero length domain string */
                    if strcmp(arg,
                              "#" ) ==
                           0 || *arg == 0 {
                        domain_0 = ""
                    } else if strlen(arg) != 0
                                  &&
                                  {
                                      domain_0 = canonicalise_opt(arg);
                                      domain_0.is_null()
                                  } {
                        strcpy(errstr, gen_err);
                        return 0
                    }
                    ipsets.next =
                        opt_malloc(::std::mem::size_of::<IpSets>()                          ) ;
                    ipsets = ipsets.next;
                    memset(ipsets, 0,
                           ::std::mem::size_of::<IpSets>());
                    ipsets.domain = domain_0;
                    arg = end_1
                }
            } else {
                // ipsets.next =
                //     opt_malloc(::std::mem::size_of::<IpSets>()) ;
                // ipsets = ipsets.next;
                // memset(ipsets, 0,
                //        ::std::mem::size_of::<IpSets>());
                ipsets = Default::default();
                ipsets.domain = ""
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
                opt_malloc((::std::mem::size_of::<&mut String>()                   ).wrapping_mul(size_0      ))
                    ;
            sets = sets_pos;
            loop  {
                end_1 = split(arg);
                let fresh27 = sets_pos;
                sets_pos = sets_pos.offset(1);
                *fresh27 = opt_string_alloc(arg);
                arg = end_1;
                if end_1.is_null() { break ; }
            }
            *sets_pos = 0 ;
            ipsets = &mut ipsets_head;
            while !ipsets.next.is_null() {
                (*ipsets.next).sets = sets;
                ipsets = ipsets.next
            }
            ipsets.next = daemon.ipsets;
            daemon.ipsets = ipsets_head.next;
            current_block = 7879481898411272068;
        }
        99 => {
            /* --cache-size */
            let mut size_1: i32 = 0;
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
            set_option_bool(2); /* default */
            if !arg.is_null() &&
                   strcmp(arg,
                          "extra" ) ==
                       0 {
                set_option_bool(51);
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
            let mut i_0: i32 = 0;
            if atoi_check(arg, &mut i_0) == 0 {
                strcpy(errstr, gen_err);
                return 0
            }
            daemon.edns_pktsz = i_0 ;
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
                let mut new_11: TftpPrefix =
                    opt_malloc(::std::mem::size_of::<TftpPrefix>()) ;
                new_11.interface = opt_string_alloc(comma);
                new_11.prefix = opt_string_alloc(arg);
                new_11.next = daemon.if_prefix;
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
                       "bad port range" );
                return 0
            }
            if daemon.start_tftp_port >
                   daemon.end_tftp_port {
                let mut tmp: i32 = daemon.start_tftp_port;
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
                              "ip" )
                       == 0 {
                set_option_bool(29);
            } else if strcasecmp(arg,
                                 "mac"                                *const libc::c_char) == 0
             {
                set_option_bool(56);
            } else { strcpy(errstr, gen_err); return 0 }
            current_block = 7879481898411272068;
        }
        262 => {
            /* --bridge-interface */
            let mut new_12: DhcpBridge = 0;
            comma = split(arg);
            if comma.is_null() ||
                   strlen(arg) >
                       (16 - 1)
               {
                strcpy(errstr,
                       "bad bridge-interface" );
                return 0
            }
            new_12 = daemon.bridges;
            while !new_12.is_null() {
                if strcmp(new_12.iface.as_mut_ptr(), arg) ==
                       0 {
                    break ;
                }
                new_12 = new_12.next
            }
            if new_12.is_null() {
                new_12 =
                    opt_malloc(::std::mem::size_of::<DhcpBridge>());
                strcpy(new_12.iface.as_mut_ptr(), arg);
                new_12.alias = 0;
                new_12.next = daemon.bridges;
                daemon.bridges = new_12
            }
            loop  {
                arg = comma;
                comma = split(arg);
                if strlen(arg) != 0 &&
                       strlen(arg) <=
                           (16 - 1) {
                    let mut b: DhcpBridge =
                        opt_malloc(::std::mem::size_of::<DhcpBridge>()                          );
                    b.next = new_12.alias;
                    new_12.alias = b;
                    strcpy(b.iface.as_mut_ptr(), arg);
                }
                if comma.is_null() { break ; }
            }
            current_block = 7879481898411272068;
        }
        357 => {
            /* --shared-network */
            let mut new_13: SharedNetwork =
                opt_malloc(::std::mem::size_of::<SharedNetwork>()) ;
            new_13.shared_addr.s_addr = 0;
            new_13.if_index = 0;
            comma = split(arg);
            if comma.is_null() {
                current_block = 3177757304694473968;
            } else {
                if inet_pton(2, comma,
                             &mut new_13.shared_addr                          Vec<u8>) != 0 {
                    if inet_pton(2, arg,
                                 &mut new_13.match_addr                              Vec<u8>) == 0 &&
                           {
                               new_13.if_index =
                                   if_nametoindex(arg);
                               (new_13.if_index) == 0
                           } {
                        current_block = 3177757304694473968;
                    } else { current_block = 12609744167958600007; }
                } else if inet_pton(10, comma,
                                    &mut new_13.shared_addr6                                   &mut In6Addr)  !=
                              0 {
                    if inet_pton(10, arg,
                                 &mut new_13.match_addr6
                                    ) == 0 &&
                           {
                               new_13.if_index =
                                   if_nametoindex(arg);
                               (new_13.if_index) == 0
                           } {
                        current_block = 3177757304694473968;
                    } else { current_block = 12609744167958600007; }
                } else { current_block = 3177757304694473968; }
                match current_block {
                    3177757304694473968 => { }
                    _ => {
                        new_13.next = daemon.shared_networks;
                        daemon.shared_networks = new_13;
                        current_block = 7879481898411272068;
                    }
                }
            }
            match current_block {
                7879481898411272068 => { }
                _ => {
                    free(new_13);
                    strcpy(errstr,
                           "bad shared-network"                          *const libc::c_char);
                    return 0
                }
            }
        }
        70 => {
            /* --dhcp-range */
            let mut k: i32 = 0;
            let mut leasepos: i32 = 2;
            let mut cp_0: &mut String = 0 ;
            let mut a: [&mut String; 8] =
                [0 , 0 ,
                 0 , 0 ,
                 0 , 0 ,
                 0 , 0 ];
            let mut new_14: DhcpContext;
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
                            new_14.filter =
                                dhcp_netid_create(arg.offset(4
                                                                ),
                                                  new_14.filter)
                        }
                    } else if !new_14.netid.net.is_null() {
                        dhcp_context_free(new_14); /* default */
                        strcpy(errstr,
                               "only one tag allowed"                              *const libc::c_char);
                        return 0
                    } else {
                        new_14.netid.net =
                            opt_string_alloc(set_prefix(arg))
                    }
                    arg = comma
                } else { a[0 ] = arg; break ; }
            }
            k = 1;
            while k < 8 {
                a[k ] = split(a[(k - 1) ]);
                if a[k ].is_null() { break ; }
                k += 1
            }
            if k < 2 {
                dhcp_context_free(new_14);
                strcpy(errstr,
                       "bad dhcp-range" );
                return 0
            }
            if inet_pton(2, a[0 ],
                         &mut new_14.start                      Vec<u8>) != 0 {
                new_14.next = daemon.dhcp;
                new_14.lease_time = 3600;
                daemon.dhcp = new_14;
                new_14.end = new_14.start;
                if strcmp(a[1 ],
                          "static" )
                       == 0 {
                    new_14.flags =
                        (new_14.flags |
                             (1) << 0)                      libc::c_int
                } else if strcmp(a[1 ],
                                 "proxy"                                *const libc::c_char) == 0
                 {
                    new_14.flags =
                        (new_14.flags |
                             (1) << 3)                      libc::c_int
                } else if inet_pton(2,
                                    a[1 ],
                                    &mut new_14.end                                 Vec<u8>) == 0 {
                    dhcp_context_free(new_14);
                    strcpy(errstr,
                           "bad dhcp-range"                          *const libc::c_char);
                    return 0
                }
                if __bswap_32(new_14.start.s_addr) >
                       __bswap_32(new_14.end.s_addr) {
                    let mut tmp_0: NetAddress = new_14.start;
                    new_14.start = new_14.end;
                    new_14.end = tmp_0
                }
                if k >= 3 &&
                       !strchr(a[2 ],
                               '.' as i32).is_null() &&
                       inet_pton(2,
                                 a[2 ],
                                 &mut new_14.netmask                              Vec<u8>) > 0 {
                    new_14.flags =
                        (new_14.flags |
                             (1) << 1)                      libc::c_int;
                    leasepos = 3;
                    if is_same_net4(new_14.start, new_14.end,
                                    new_14.netmask) == 0 {
                        dhcp_context_free(new_14);
                        strcpy(errstr,
                               "inconsistent DHCP range"                              *const libc::c_char);
                        return 0
                    }
                    if k >= 4 &&
                           !strchr(a[3 ],
                                   '.' as i32).is_null() &&
                           inet_pton(2,
                                     a[3 ],
                                     &mut new_14.broadcast
                                        ) >
                               0 {
                        new_14.flags =
                            (new_14.flags |
                                 (1) << 2)                          libc::c_int;
                        leasepos = 4
                    }
                }
            } else if inet_pton(10,
                                a[0 ],
                                &mut new_14.start6                             Vec<u8>) != 0 {
                let mut err_1: *const libc::c_char = 0;
                new_14.flags =
                    (new_14.flags |
                         (1) << 17) ;
                new_14.prefix = 64;
                new_14.end6 = new_14.start6;
                new_14.lease_time =
                    (3600 * 24);
                new_14.next = daemon.dhcp6;
                daemon.dhcp6 = new_14;
                leasepos = 1;
                while leasepos < k {
                    if strcmp(a[leasepos ],
                              "static") == 0 {
                        new_14.flags =
                            (new_14.flags |
                                 ((1) << 0 |
                                      (1) <<
                                          8))
                    } else if strcmp(a[leasepos ],
                                     "ra-only") ==
                                  0 ||
                                  strcmp(a[leasepos ],
                                         "slaac") ==
                                      0 {
                        new_14.flags =
                            (new_14.flags |
                                 (1) << 13)                          libc::c_int
                    } else if strcmp(a[leasepos ],
                                     "ra-names") ==
                                  0 {
                        new_14.flags =
                            (new_14.flags |
                                 ((1) << 6 |
                                      (1) <<
                                          13))
                    } else if strcmp(a[leasepos ],
                                     "ra-advrouter") ==
                                  0 {
                        new_14.flags =
                            (new_14.flags |
                                 ((1) << 4 |
                                      (1) <<
                                          13))
                    } else if strcmp(a[leasepos ],
                                     "ra-stateless") ==
                                  0 {
                        new_14.flags =
                            (new_14.flags |
                                 ((1) << 7 |
                                      (1) << 8
                                      |
                                      (1) <<
                                          13))
                    } else if strcmp(a[leasepos ],
                                     "off-link") ==
                                  0 {
                        new_14.flags =
                            (new_14.flags |
                                 (1) << 18)                          libc::c_int
                    } else if leasepos == 1 &&
                                  inet_pton(10,
                                            a[leasepos ],
                                            &mut new_14.end6                                          &mut In6Addr                                         Vec<u8>) != 0 {
                        new_14.flags =
                            (new_14.flags |
                                 (1) << 8)                          libc::c_int
                    } else {
                        if !(strstr(a[leasepos ],
                                    "constructor:"                                   *const libc::c_char) ==
                                 a[leasepos ]) {
                            break ;
                        }
                        new_14.template_interface =
                            opt_string_alloc(a[leaseposusize].offset(12                  libc::c_int
                                                ));
                        new_14.flags =
                            (new_14.flags |
                                 (1) << 10)                          libc::c_int
                    }
                    leasepos += 1
                }
                /* bare integer < 128 is prefix value */
                if leasepos < k {
                    let mut pref_0: i32 = 0;
                    cp_0 = a[leasepos ];
                    while cp_0 != 0 {
                        if !(cp_0 >= '0' as i32 &&
                                 cp_0 <= '9' as i32) {
                            break ;
                        }
                        cp_0 = cp_0.offset(1)
                    }
                    if cp_0 == 0 &&
                           {
                               pref_0 = atoi(a[leasepos ]);
                               (pref_0) <= 128
                           } {
                        new_14.prefix = pref_0;
                        leasepos += 1
                    }
                }
                if new_14.prefix > 64 {
                    if new_14.flags &
                           (1) << 13 != 0 {
                        err_1 =
                            "prefix length must be exactly 64 for RA subnets"

                    } else if new_14.flags &
                                  (1) << 10 !=
                                  0 {
                        err_1 =
                            "prefix length must be exactly 64 for subnet constructors"

                    }
                } else if new_14.prefix < 64 {
                    err_1 =
                        "prefix length must be at least 64"

                }
                if err_1.is_null() &&
                       is_same_net6(&mut new_14.start6,
                                    &mut new_14.end6, new_14.prefix) ==
                           0 {
                    err_1 =
                        "inconsistent DHCPv6 range"
                }
                if !err_1.is_null() {
                    dhcp_context_free(new_14);
                    strcpy(errstr, err_1);
                    return 0
                }
                /* dhcp-range=:: enables DHCP stateless on any interface */
                if ({
                        let mut __a: *const In6Addr =
                            &mut new_14.start6                          *const In6Addr;
                        (__a.__in6_u.__u6_addr32[0 ]
                             == 0 &&
                             __a.__in6_u.__u6_addr32[1         usize] ==
                                 0 &&
                             __a.__in6_u.__u6_addr32[2         usize] ==
                                 0 &&
                             __a.__in6_u.__u6_addr32[3         usize] ==
                                 0)                      libc::c_int
                    }) != 0 &&
                       new_14.flags &
                           (1) << 10 == 0 {
                    new_14.prefix = 0
                }
                if new_14.flags &
                       (1) << 10 != 0 {
                    let mut zero: In6Addr =
                        In6Addr {__in6_u:
                                     C2RustUnnamed{__u6_addr8: [0; 16],},};
                    if is_same_net6(&mut zero, &mut new_14.start6,
                                    new_14.prefix) == 0 {
                        dhcp_context_free(new_14);
                        strcpy(errstr,
                               "prefix must be zero with \"constructor:\" argument"
                                   );
                        return 0
                    }
                }
                if addr6part(&mut new_14.start6) >
                       addr6part(&mut new_14.end6) {
                    let mut tmp_1: In6Addr = new_14.start6;
                    new_14.start6 = new_14.end6;
                    new_14.end6 = tmp_1
                }
            } else {
                dhcp_context_free(new_14);
                strcpy(errstr,
                       "bad dhcp-range" );
                return 0
            }
            if leasepos < k {
                if leasepos != k - 1 {
                    dhcp_context_free(new_14);
                    strcpy(errstr,
                           "bad dhcp-range"                          *const libc::c_char);
                    return 0
                }
                if strcmp(a[leasepos ],
                          "infinite" )
                       == 0 {
                    new_14.lease_time = 0xffffffff;
                    new_14.flags =
                        (new_14.flags |
                             (1) << 19)                      libc::c_int
                } else if strcmp(a[leasepos ],
                                 "deprecated"                                *const libc::c_char) == 0
                 {
                    new_14.flags =
                        (new_14.flags |
                             (1) << 9)                      libc::c_int
                } else {
                    let mut fac: i32 = 1;
                    if strlen(a[leasepos ]) >
                           0 {
                        let mut current_block_1049: u64;
                        match *a[leasepos                               usize].offset(strlen(a[leasepos             usize]).wrapping_sub(1 libc::c_int
                                                                                                                       )
                                                      )                            libc::c_int {
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
                                *a[leasepos                                 usize].offset(strlen(a[leasepos               usize]).wrapping_sub(1     libc::c_int
                                                                                                                           )
                                                        ) =
                                    0
                            }
                            _ => { }
                        }
                        cp_0 = a[leasepos ];
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
                                   "bad dhcp-range"                                  *const libc::c_char);
                            free(new_14);
                            return 0
                        }
                        new_14.lease_time =
                            (atoi(a[leasepos ]) * fac)                          libc::c_uint;
                        new_14.flags =
                            (new_14.flags |
                                 (1) << 19)                          libc::c_int;
                        /* Leases of a minute or less confuse
		       some clients, notably Apple's */
                        if new_14.lease_time <
                               120 {
                            new_14.lease_time =
                                120
                        }
                    }
                }
            }
            current_block = 7879481898411272068;
        }
        272 | 71 => {
            /* --dhcp-host */
            let mut new_15: DhcpConfig = 0;
            let mut in_0: NetAddress = NetAddress {s_addr: 0,};
            new_15 =
                opt_malloc(::std::mem::size_of::<DhcpConfig>());
            new_15.next = daemon.dhcp_conf;
            new_15.flags =
                if option == 272 {
                    2048
                } else { 0 };
            new_15.hwaddr = 0;
            new_15.netid = 0;
            new_15.filter = 0 ;
            new_15.clid = 0;
            new_15.addr6 = 0 ;
            while !arg.is_null() {
                comma = split(arg);
                if !strchr(arg, ':' as i32).is_null() {
                    /* ethernet address, netid or binary CLID */
                    if (*arg.offset(0)
                            == 'i' as i32 ||
                            *arg.offset(0)                          libc::c_int == 'I' as i32) &&
                           (*arg.offset(1)                          libc::c_int == 'd' as i32 ||
                                *arg.offset(1)                              libc::c_int == 'D' as i32) &&
                           *arg.offset(2) == ':' as i32 {
                        if *arg.offset(3) == '*' as i32 {
                            new_15.flags |=
                                128
                        } else {
                            let mut len_0: i32 = 0; /* dump id: */
                            arg = arg.offset(3);
                            if !strchr(arg, ':' as i32).is_null() {
                                len_0 =
                                    parse_hex(arg, arg,
                                              -(1),
                                              0,
                                              0)
                            } else {
                                unhide_metas(arg);
                                len_0 = strlen(arg)
                            }
                            if len_0 == -(1) {
                                dhcp_config_free(new_15);
                                strcpy(errstr,
                                       "bad hex constant"                                      *const libc::c_char);
                                return 0
                            } else {
                                new_15.clid =
                                    opt_malloc(len_0 )                                  mut Vec<u8>;
                                if !new_15.clid.is_null() {
                                    new_15.flags |=
                                        2;
                                    new_15.clid_len = len_0;
                                    memcpy(new_15.clid                                        Vec<u8>,
                                           arg,
                                           len_0);
                                }
                            }
                        }
                    } else if strstr(arg,
                                     "net:") == arg ||
                                  strstr(arg,
                                         "set:") == arg {
                        let mut newlist_0: DhcpNetIdList =
                            opt_malloc(::std::mem::size_of::<DhcpNetIdList>()
                                          )                          DhcpNetIdList;
                        newlist_0.next = new_15.netid;
                        new_15.netid = newlist_0;
                        newlist_0.list =
                            dhcp_netid_create(arg.offset(4          isize),
                                              0 )
                    } else if strstr(arg,
                                     "tag:") == arg {
                        new_15.filter =
                            dhcp_netid_create(arg.offset(4          isize),
                                              new_15.filter)
                    } else if *arg.offset(0)                            libc::c_int == '[' as i32 &&
                                  *arg.offset(strlen(arg).wrapping_sub(1                        libc::c_int
                                                                                           )
                                                 ) ==
                                      ']' as i32 {
                        let mut pref_1: &mut String =
                            0 ;
                        let mut in6: In6Addr =
                            In6Addr {__in6_u:
                                         C2RustUnnamed{__u6_addr8:
                                                           [0; 16],},};
                        let mut new_addr: AddressListEntry = 0 ;
                        *arg.offset(strlen(arg).wrapping_sub(1
                                                                       )
                                       ) =
                            0;
                        arg = arg.offset(1);
                        pref_1 = split_chr(arg, '/' );
                        if inet_pton(10, arg,
                                     &mut in6                                  Vec<u8>) == 0 {
                            dhcp_config_free(new_15);
                            strcpy(errstr,
                                   "bad IPv6 address"                                  *const libc::c_char);
                            return 0
                        }
                        new_addr =
                            opt_malloc(::std::mem::size_of::<AddressListEntry>()                              ) ;
                        new_addr.next = new_15.addr6;
                        new_addr.flags = 0;
                        new_addr.addr.addr6 = in6;
                        new_15.addr6 = new_addr;
                        if !pref_1.is_null() {
                            let mut addrpart_0: u64 = addr6part(&mut in6);
                            if atoi_check(pref_1, &mut new_addr.prefixlen)
                                   == 0 ||
                                   new_addr.prefixlen > 128
                                   ||
                                   ((1 as u64) <<
                                        128 -
                                            new_addr.prefixlen).wrapping_sub(1
                                                                                                                    libc::c_int
                                                                                                                    libc::c_ulonglong)
                                       & addrpart_0 !=
                                       0long {
                                dhcp_config_free(new_15);
                                strcpy(errstr,
                                       "bad IPv6 prefix"                                      *const libc::c_char);
                                return 0
                            }
                            new_addr.flags |= 8
                        }
                        i = 0;
                        while i < 8 {
                            if in6.__in6_u.__u6_addr8[i ]                             libc::c_int != 0 {
                                break ;
                            }
                            i += 1
                        }
                        /* dhcp-host has strange backwards-compat needs. */
                        /* set WILDCARD if network part all zeros */
                        if i == 8 {
                            new_addr.flags |= 16
                        }
                        new_15.flags |= 4096
                    } else {
                        let mut newhw: HwaddrConfig =
                            opt_malloc(::std::mem::size_of::<HwaddrConfig>()
                                          )                          HwaddrConfig;
                        newhw.hwaddr_len =
                            parse_hex(arg, newhw.hwaddr.as_mut_ptr(),
                                      16,
                                      &mut newhw.wildcard_mask,
                                      &mut newhw.hwaddr_type);
                        if newhw.hwaddr_len == -(1) {
                            free(newhw);
                            dhcp_config_free(new_15);
                            strcpy(errstr,
                                   "bad hex constant"                                  *const libc::c_char);
                            return 0
                        } else {
                            newhw.next = new_15.hwaddr;
                            new_15.hwaddr = newhw
                        }
                    }
                } else if !strchr(arg, '.' as i32).is_null() &&
                              inet_pton(2, arg,
                                        &mut in_0                                     Vec<u8>) >
                                  0 {
                    let mut configs: DhcpConfig = 0;
                    new_15.addr = in_0;
                    new_15.flags |= 32;
                    /* If the same IP appears in more than one host config, then DISCOVER
		   for one of the hosts will get the address, but REQUEST will be NAKed,
		   since the address is reserved by the other one -> protocol loop. */
                    configs = daemon.dhcp_conf;
                    while !configs.is_null() {
                        if configs.flags &
                               32 != 0 &&
                               configs.addr.s_addr == in_0.s_addr {
                            sprintf(errstr,
                                    "duplicate dhcp-host IP address %s"
                                        ,
                                    inet_ntoa(in_0));
                            return 0
                        }
                        configs = configs.next
                    }
                } else {
                    let mut cp_1: &mut String = 0 ;
                    let mut lastp: &mut String = 0 ;
                    let mut last: libc::c_char =
                        0;
                    let mut fac_0: i32 = 1;
                    let mut isdig: i32 = 0;
                    if strlen(arg) > 1 {
                        lastp =
                            arg.offset(strlen(arg)                                     isize).offset(-(1            isize));
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
                                *lastp = 0
                            }
                            _ => { }
                        }
                    }
                    cp_1 = arg;
                    while cp_1 != 0 {
                        if *(*__ctype_b_loc()).offset(cp_1
                          ) &
                               _ISDIGIT                              libc::c_int != 0 {
                            isdig = 1
                        } else if cp_1 != ' ' as i32 {
                            break ;
                        }
                        cp_1 = cp_1.offset(1)
                    }
                    if cp_1 != 0 {
                        if !lastp.is_null() { *lastp = last }
                        if strcmp(arg,
                                  "infinite") == 0
                           {
                            new_15.lease_time = 0xffffffff;
                            new_15.flags |=
                                8
                        } else if strcmp(arg,
                                         "ignore") ==
                                      0 {
                            new_15.flags |=
                                1
                        } else {
                            new_15.hostname = canonicalise_opt(arg);
                            if new_15.hostname.is_null() ||
                                   legal_hostname(new_15.hostname) == 0 {
                                dhcp_config_free(new_15);
                                strcpy(errstr,
                                       "bad DHCP host name"             );
                                return 0
                            }
                            new_15.flags |=
                                16;
                            new_15.domain =
                                strip_hostname(new_15.hostname)
                        }
                    } else if isdig != 0 {
                        new_15.lease_time =
                            (atoi(arg) * fac_0);
                        /* Leases of a minute or less confuse
		       some clients, notably Apple's */
                        if new_15.lease_time <
                               120 {
                            new_15.lease_time =
                                120
                        }
                        new_15.flags |= 8
                    }
                }
                arg = comma
            }
            daemon.dhcp_conf = new_15;
            current_block = 7879481898411272068;
        }
        294 => {
            /* --tag-if */
            let mut new_16: TagIf =
                opt_malloc(::std::mem::size_of::<TagIf>())
                    ;
            new_16.tag = 0 ;
            new_16.set = 0;
            new_16.next = 0 ;
            /* preserve order */
            if daemon.tag_if.is_null() {
                daemon.tag_if = new_16
            } else {
                let mut tmp_2: TagIf = 0 ;
                tmp_2 = daemon.tag_if;
                while !tmp_2.next.is_null() { tmp_2 = tmp_2.next }
                tmp_2.next = new_16
            }
            while !arg.is_null() {
                let mut len_1: usize = 0;
                comma = split(arg);
                len_1 = strlen(arg);
                if len_1 < 5 {
                    new_16.set = 0;
                    break ;
                } else {
                    let mut newtag: DhcpNetId =
                        dhcp_netid_create(arg.offset(4      isize),
                                          0 );
                    if strstr(arg,
                              "set:" )
                           == arg {
                        let mut newlist_1: DhcpNetIdList =
                            opt_malloc(::std::mem::size_of::<DhcpNetIdList>()
                                          )                          DhcpNetIdList;
                        newlist_1.next = new_16.set;
                        new_16.set = newlist_1;
                        newlist_1.list = newtag
                    } else if strstr(arg,
                                     "tag:") == arg {
                        newtag.next = new_16.tag;
                        new_16.tag = newtag
                    } else {
                        new_16.set = 0;
                        dhcp_netid_free(newtag);
                        break ;
                    }
                    arg = comma
                }
            }
            if new_16.set.is_null() {
                dhcp_netid_free(new_16.tag);
                dhcp_netid_list_free(new_16.set);
                strcpy(errstr,
                       "bad tag-if" );
                free(new_16);
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
            let mut new_17: DhcpMatchName =
                opt_malloc(::std::mem::size_of::<DhcpMatchName>()) ;
            let mut id: DhcpNetId =
                opt_malloc(::std::mem::size_of::<DhcpNetId>()) ;
            let mut len_2: susize = 0;
            comma = split(arg);
            if comma.is_null() ||
                   {
                       len_2 = strlen(comma) as susize;
                       (len_2) == 0
                   } {
                strcpy(errstr, gen_err);
                return 0
            }
            new_17.wildcard = 0;
            new_17.netid = id;
            id.net = opt_string_alloc(set_prefix(arg));
            if *comma.offset((len_2 - 1)                           isize) == '*' as i32 {
                *comma.offset((len_2 - 1)                            isize) = 0;
                new_17.wildcard = 1
            }
            new_17.name = opt_string_alloc(comma);
            new_17.next = daemon.dhcp_name_match;
            daemon.dhcp_name_match = new_17;
            current_block = 7879481898411272068;
        }
        77 => {
            /* --dhcp-boot */
            let mut id_0: DhcpNetId = dhcp_tags(&mut arg);
            if arg.is_null() {
                strcpy(errstr, gen_err);
                return 0
            } else {
                let mut dhcp_file: &mut String = 0 ;
                let mut dhcp_sname: &mut String =
                    0 ;
                let mut tftp_sname: &mut String =
                    0 ;
                let mut dhcp_next_server: NetAddress = NetAddress {s_addr: 0,};
                let mut new_18: DhcpBoot = 0 ;
                comma = split(arg);
                dhcp_file = opt_string_alloc(arg);
                dhcp_next_server.s_addr = 0;
                if !comma.is_null() {
                    arg = comma;
                    comma = split(arg);
                    dhcp_sname = opt_string_alloc(arg);
                    if !comma.is_null() {
                        unhide_metas(comma);
                        if !(inet_pton(2, comma,
                                       &mut dhcp_next_server
                                          ) >
                                 0) {
                            /*
			 * The user may have specified the tftp hostname here.
			 * save it so that it can be resolved/looked up during
			 * actual dhcp_reply().
			 */
                            tftp_sname = opt_string_alloc(comma);
                            dhcp_next_server.s_addr =
                                0
                        }
                    }
                }
                new_18 =
                    opt_malloc(::std::mem::size_of::<DhcpBoot>()) ;
                new_18.file = dhcp_file;
                new_18.sname = dhcp_sname;
                new_18.tftp_sname = tftp_sname;
                new_18.next_server = dhcp_next_server;
                new_18.netid = id_0;
                new_18.next = daemon.boot_config;
                daemon.boot_config = new_18
            }
            current_block = 7879481898411272068;
        }
        350 => {
            /* --dhcp-reply-delay */
            let mut id_1: DhcpNetId = dhcp_tags(&mut arg);
            if arg.is_null() {
                strcpy(errstr, gen_err);
                return 0
            } else {
                let mut new_19: DelayConfig = 0 ;
                let mut delay: i32 = 0;
                if atoi_check(arg, &mut delay) == 0 {
                    strcpy(errstr, gen_err);
                    return 0
                }
                new_19 =
                    opt_malloc(::std::mem::size_of::<DelayConfig>()) ;
                new_19.delay = delay;
                new_19.netid = id_1;
                new_19.next = daemon.delay_conf;
                daemon.delay_conf = new_19
            }
            current_block = 7879481898411272068;
        }
        291 => {
            /* --pxe-prompt */
            let mut new_20: DhcpOpt =
                opt_malloc(::std::mem::size_of::<DhcpOpt>())
                    ; /* PXE_MENU_PROMPT */
            let mut timeout: i32 = 0;
            new_20.netid = 0 ;
            new_20.opt = 10;
            new_20.netid = dhcp_tags(&mut arg);
            if arg.is_null() {
                dhcp_opt_free(new_20);
                strcpy(errstr, gen_err);
                return 0
            } else {
                comma = split(arg);
                unhide_metas(arg);
                new_20.len =
                    strlen(arg).wrapping_add(1                                    ) ;
                new_20.val =
                    opt_malloc(new_20.len );
                memcpy(new_20.val.offset(1)                    Vec<u8>, arg,
                       (new_20.len - 1));
                new_20.u.vendor_class = 0;
                new_20.flags = 256 | 16384;
                if !comma.is_null() && atoi_check(comma, &mut timeout) != 0 {
                    *new_20.val = timeout
                } else {
                    *new_20.val = 255
                }
                new_20.next = daemon.dhcp_opts;
                daemon.dhcp_opts = new_20;
                daemon.enable_pxe = 1
            }
            current_block = 7879481898411272068;
        }
        292 => {
            /* --pxe-service */
            let mut new_21: PxeService =
                opt_malloc(::std::mem::size_of::<PxeService>())              PxeService; /* local boot */
            let mut CSA: [&mut String; 13] =
                ["x86PC"                &mut String,
                 "PC98"                &mut String,
                 "IA64_EFI"                &mut String,
                 "Alpha"                &mut String,
                 "Arc_x86"                &mut String,
                 "Intel_Lean_Client"
                     ,
                 "IA32_EFI"                &mut String,
                 "x86-64_EFI"                &mut String,
                 "Xscale_EFI"                &mut String,
                 "BC_EFI"                &mut String,
                 "ARM32_EFI"                &mut String,
                 "ARM64_EFI"                &mut String, 0 ];
            static mut boottype: i32 = 32768;
            new_21.netid = 0 ;
            new_21.sname = 0 ;
            new_21.server.s_addr = 0;
            new_21.netid = dhcp_tags(&mut arg);
            if !arg.is_null() && { comma = split(arg); !comma.is_null() } {
                i = 0;
                while !CSA[i ].is_null() {
                    if strcasecmp(CSA[i ], arg) == 0 {
                        break ;
                    }
                    i += 1
                }
                if !CSA[i ].is_null() || atoi_check(arg, &mut i) != 0
                   {
                    arg = comma;
                    comma = split(arg);
                    new_21.csa = i ;
                    new_21.menu = opt_string_alloc(arg);
                    if comma.is_null() {
                        new_21.type_0 = 0 ;
                        new_21.basename = 0
                    } else {
                        arg = comma;
                        comma = split(arg);
                        if atoi_check(arg, &mut i) != 0 {
                            new_21.type_0 = i ;
                            new_21.basename = 0
                        } else {
                            let fresh28 = boottype;
                            boottype = boottype + 1;
                            new_21.type_0 = fresh28 ;
                            new_21.basename = opt_string_alloc(arg)
                        }
                        if !comma.is_null() {
                            if inet_pton(2, comma,
                                         &mut new_21.server
                                            ) == 0 {
                                new_21.server.s_addr =
                                    0;
                                new_21.sname = opt_string_alloc(comma)
                            }
                        }
                    }
                    /* Order matters */
                    new_21.next = 0 ;
                    if daemon.pxe_services.is_null() {
                        daemon.pxe_services = new_21
                    } else {
                        let mut s: PxeService = 0 ;
                        s = daemon.pxe_services;
                        while !s.next.is_null() { s = s.next }
                        s.next = new_21
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
                let mut new_22: DhcpMac =
                    opt_malloc(::std::mem::size_of::<DhcpMac>()) ;
                new_22.netid.net = opt_string_alloc(set_prefix(arg));
                unhide_metas(comma);
                new_22.hwaddr_len =
                    parse_hex(comma, new_22.hwaddr.as_mut_ptr(),
                              16, &mut new_22.mask,
                              &mut new_22.hwaddr_type);
                if new_22.hwaddr_len == -(1) {
                    free(new_22.netid.net);
                    strcpy(errstr, gen_err);
                    free(new_22);
                    return 0
                } else {
                    new_22.next = daemon.dhcp_macs;
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
                           "invalid port number"                          *const libc::c_char);
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
                let mut new_25: AddrList2 =
                    opt_malloc(::std::mem::size_of::<AddrList2>()) 2;
                comma = split(arg);
                if !(inet_pton(2, arg,
                               &mut new_25.addr                            Vec<u8>) > 0) {
                    strcpy(errstr,
                           "bad dhcp-proxy address"                          *const libc::c_char);
                    free(new_25);
                    return 0
                }
                new_25.next = daemon.override_relays;
                daemon.override_relays = new_25;
                arg = comma
            }
            current_block = 7879481898411272068;
        }
        361 => {
            /* --dhcp-pxe-vendor */
            while !arg.is_null() {
                let mut new_26: DhcpPxeVendor =
                    opt_malloc(::std::mem::size_of::<DhcpPxeVendor>()) ;
                comma = split(arg);
                new_26.data = opt_string_alloc(arg);
                new_26.next = daemon.dhcp_pxe_vendors;
                daemon.dhcp_pxe_vendors = new_26;
                arg = comma
            }
            current_block = 7879481898411272068;
        }
        323 => {
            /* --dhcp-relay */
            let mut new_27: DhcpRelay =
                opt_malloc(::std::mem::size_of::<DhcpRelay>());
            comma = split(arg);
            new_27.interface = opt_string_alloc(split(comma));
            new_27.iface_index = 0;
            if inet_pton(2, arg,
                         &mut new_27.local                       Vec<u8>) != 0 &&
                   inet_pton(2, comma,
                             &mut new_27.server                           Vec<u8>) != 0 {
                new_27.next = daemon.relay4;
                daemon.relay4 = new_27
            } else if inet_pton(10, arg,
                                &mut new_27.local                              Vec<u8>) != 0 &&
                          inet_pton(10, comma,
                                    &mut new_27.server                                  Vec<u8>) != 0 {
                new_27.next = daemon.relay6;
                daemon.relay6 = new_27
            } else {
                free(new_27.interface);
                strcpy(errstr,
                       "Bad dhcp-relay" );
                free(new_27);
                return 0
            }
            current_block = 7879481898411272068;
        }
        324 => {
            /* --ra-param */
            comma = split(arg);
            if !comma.is_null() {
                let mut new_28: RaInterface =
                    opt_malloc(::std::mem::size_of::<RaInterface>()) ;
                new_28.lifetime = -(1);
                new_28.prio = 0;
                new_28.mtu = 0;
                new_28.mtu_name = 0 ;
                new_28.name = opt_string_alloc(arg);
                if strcasestr(comma,
                              "mtu:" )
                       == comma {
                    arg = comma.offset(4);
                    comma = split(comma);
                    if comma.is_null() {
                        current_block = 14730872864422895907;
                    } else if strcasecmp(arg,
                                         "off") == 0 {
                        new_28.mtu = -(1);
                        current_block = 1840194652026069277;
                    } else if atoi_check(arg, &mut new_28.mtu) == 0 {
                        new_28.mtu_name = opt_string_alloc(arg);
                        current_block = 1840194652026069277;
                    } else if new_28.mtu < 1280 {
                        current_block = 14730872864422895907;
                    } else { current_block = 1840194652026069277; }
                } else { current_block = 1840194652026069277; }
                match current_block {
                    1840194652026069277 => {
                        if strcasestr(comma,
                                      "high"                                     *const libc::c_char) == comma ||
                               strcasestr(comma,
                                          "low"                                         *const libc::c_char) == comma {
                            if *comma == 'l' as i32 ||
                                   *comma == 'L' as i32 {
                                new_28.prio = 0x18
                            } else { new_28.prio = 0x8 }
                            comma = split(comma)
                        }
                        arg = split(comma);
                        if atoi_check(comma, &mut new_28.interval) == 0 ||
                               !arg.is_null() &&
                                   atoi_check(arg, &mut new_28.lifetime) ==
                                       0 {
                            current_block = 14730872864422895907;
                        } else {
                            new_28.next = daemon.ra_interfaces;
                            daemon.ra_interfaces = new_28;
                            current_block = 7879481898411272068;
                        }
                    }
                    _ => { }
                }
                match current_block {
                    7879481898411272068 => { }
                    _ => {
                        free(new_28.name);
                        strcpy(errstr,
                               "bad RA-params"                              *const libc::c_char);
                        free(new_28);
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
                              &mut daemon.duid_enterprise                            &mut libc::c_uint) == 0
               {
                strcpy(errstr,
                       "bad DUID" );
                return 0
            } else {
                daemon.duid_config_len =
                    parse_hex(comma, comma,
                              strlen(comma),
                              0, 0)
                       ;
                daemon.duid_config =
                    opt_malloc(daemon.duid_config_len )                  mut Vec<u8>;
                memcpy(daemon.duid_config,
                       comma,
                       daemon.duid_config_len);
            }
            current_block = 7879481898411272068;
        }
        86 => {
            /* --alias */
            let mut dash: &mut String = 0 ;
            let mut a_0: [&mut String; 3] =
                [0 , 0 ,
                 0 ];
            let mut k_0: i32 = 0;
            let mut new_29: Doctor =
                opt_malloc(::std::mem::size_of::<Doctor>())
                    ;
            new_29.next = daemon.doctors;
            daemon.doctors = new_29;
            new_29.mask.s_addr = 0xffffffff;
            new_29.end.s_addr = 0;
            a_0[0 ] = arg;
            if !a_0[0 ].is_null() {
                k_0 = 1;
                while k_0 < 3 {
                    a_0[k_0 ] =
                        split(a_0[(k_0 - 1) ]);
                    if a_0[k_0 ].is_null() { break ; }
                    unhide_metas(a_0[k_0 ]);
                    k_0 += 1
                }
            }
            dash =
                split_chr(a_0[0 ],
                          '-' );
            if k_0 < 2 ||
                   !(inet_pton(2,
                               a_0[0 ],
                               &mut new_29.in_0                            Vec<u8>) > 0) ||
                   !(inet_pton(2,
                               a_0[1 ],
                               &mut new_29.out                            Vec<u8>) > 0) ||
                   k_0 == 3 &&
                       inet_pton(2,
                                 a_0[2 ],
                                 &mut new_29.mask                              Vec<u8>) == 0 {
                strcpy(errstr,
                       "missing address in alias" );
                return 0
            }
            if !dash.is_null() &&
                   (!(inet_pton(2, dash,
                                &mut new_29.end                             Vec<u8>) > 0) ||
                        is_same_net(new_29.in_0, new_29.end,
                                    new_29.mask) == 0 ||
                        __bswap_32(new_29.in_0.s_addr) >
                            __bswap_32(new_29.end.s_addr)) {
                strcpy(errstr,
                       "invalid alias range" );
                free(new_29);
                return 0
            }
            current_block = 7879481898411272068;
        }
        271 => {
            /* --interface-name */
            let mut new_30: InterfaceName = ;
            let mut up_0: &mut InterfaceName =
                0 ;
            let mut domain_1: &mut String = 0 ;
            comma = split(arg);
            if comma.is_null() ||
                   { domain_1 = canonicalise_opt(arg); domain_1.is_null() } {
                strcpy(errstr,
                       "bad interface name" );
                return 0
            }
            new_30 =
                opt_malloc(::std::mem::size_of::<InterfaceName>()) ;
            new_30.next = ;
            new_30.addresses = 0 ;
            /* Add to the end of the list, so that first name
	   of an interface is used for PTR lookups. */
            up_0 = &mut daemon.int_names;
            while !up_0.is_null() { up_0 = &mut (**up_0).next }
            *up_0 = new_30;
            new_30.name = domain_1;
            new_30.family = 0;
            arg = split_chr(comma, '/' );
            if !arg.is_null() {
                if strcmp(arg, "4" )
                       == 0 {
                    new_30.family = 2
                } else if strcmp(arg,
                                 "6" )
                              == 0 {
                    new_30.family = 10
                } else {
                    strcpy(errstr, gen_err);
                    free(new_30);
                    return 0
                }
            }
            new_30.intr = opt_string_alloc(comma);
            current_block = 7879481898411272068;
        }
        290 => {
            /* --cname */
            let mut new_31: Cname = 0 ;
            let mut alias: &mut String = 0 ;
            let mut target_0: &mut String = 0 ;
            let mut last_0: &mut String = 0 ;
            let mut pen: &mut String = 0 ;
            let mut ttl_0: i32 = -(1);
            pen = 0 ;
            last_0 = pen;
            comma = arg;
            while !comma.is_null() {
                pen = last_0;
                last_0 = comma;
                comma = split(comma)
            }
            if pen.is_null() {
                strcpy(errstr,
                       "bad CNAME" );
                return 0
            }
            if pen != arg && atoi_check(last_0, &mut ttl_0) != 0 {
                last_0 = pen
            }
            target_0 = canonicalise_opt(last_0);
            while arg != last_0 {
                let mut arglen: i32 = strlen(arg);
                alias = canonicalise_opt(arg);
                if alias.is_null() || target_0.is_null() {
                    free(target_0);
                    free(alias);
                    strcpy(errstr,
                           "bad CNAME"                          *const libc::c_char);
                    return 0
                }
                new_31 = daemon.cnames;
                while !new_31.is_null() {
                    if hostname_isequal(new_31.alias, alias) != 0 {
                        free(target_0);
                        free(alias);
                        strcpy(errstr,
                               "duplicate CNAME"                              *const libc::c_char);
                        return 0
                    }
                    new_31 = new_31.next
                }
                new_31 =
                    opt_malloc(::std::mem::size_of::<Cname>()) ;
                new_31.next = daemon.cnames;
                daemon.cnames = new_31;
                new_31.alias = alias;
                new_31.target = target_0;
                new_31.ttl = ttl_0;
                arg = arg.offset((arglen + 1));
                while *arg != 0 &&
                          *(*__ctype_b_loc()).offset(*arg      isize)
                              &
                              _ISSPACE                             libc::c_int != 0 {
                    arg = arg.offset(1)
                }
            }
            current_block = 7879481898411272068;
        }
        261 => {
            /* --ptr-record */
            let mut new_32: PtrRecord = 0 ;
            let mut dom: &mut String = 0 ;
            let mut target_1: &mut String = 0 ;
            comma = split(arg);
            dom = canonicalise_opt(arg);
            if dom.is_null() ||
                   !comma.is_null() &&
                       {
                           target_1 = canonicalise_opt(comma);
                           target_1.is_null()
                       } {
                free(dom);
                free(target_1);
                strcpy(errstr,
                       "bad PTR record" );
                return 0
            } else {
                new_32 =
                    opt_malloc(::std::mem::size_of::<PtrRecord>()) ;
                new_32.next = daemon.ptr;
                daemon.ptr = new_32;
                new_32.name = dom;
                new_32.ptr = target_1
            }
            current_block = 7879481898411272068;
        }
        287 => {
            /* --naptr-record */
            let mut a_1: [&mut String; 7] =
                [0 , 0 ,
                 0 , 0 ,
                 0 , 0 ,
                 0 ];
            let mut k_1: i32 = 0;
            let mut new_33: NaPtr = 0 ;
            let mut order: i32 = 0;
            let mut pref_2: i32 = 0;
            let mut name_2: &mut String = 0 ;
            let mut replace: &mut String = 0 ;
            a_1[0 ] = arg;
            if !a_1[0 ].is_null() {
                k_1 = 1;
                while k_1 < 7 {
                    a_1[k_1 ] =
                        split(a_1[(k_1 - 1) ]);
                    if a_1[k_1 ].is_null() { break ; }
                    k_1 += 1
                }
            }
            if k_1 < 6 ||
                   {
                       name_2 =
                           canonicalise_opt(a_1[0 ]);
                       name_2.is_null()
                   } ||
                   atoi_check16(a_1[1 ], &mut order) ==
                       0 ||
                   atoi_check16(a_1[2 ], &mut pref_2)
                       == 0 ||
                   k_1 == 7 &&
                       {
                           replace =
                               canonicalise_opt(a_1[6     usize]);
                           replace.is_null()
                       } {
                free(name_2);
                free(replace);
                strcpy(errstr,
                       "bad NAPTR record" );
                return 0
            } else {
                new_33 =
                    opt_malloc(::std::mem::size_of::<NaPtr>()) ;
                new_33.next = daemon.naptr;
                daemon.naptr = new_33;
                new_33.name = name_2;
                new_33.flags =
                    opt_string_alloc(a_1[3 ]);
                new_33.services =
                    opt_string_alloc(a_1[4 ]);
                new_33.regexp =
                    opt_string_alloc(a_1[5 ]);
                new_33.replace = replace;
                new_33.order = order;
                new_33.pref = pref_2
            }
            current_block = 7879481898411272068;
        }
        310 => {
            /* dns-rr */
            let mut new_34: TxtRecord = 0 ;
            let mut len_3: usize = 0 ;
            let mut data: &mut String = 0 ;
            let mut class: i32 = 0;
            comma = split(arg);
            data = split(comma);
            new_34 =
                opt_malloc(::std::mem::size_of::<TxtRecord>()) ;
            new_34.name = 0 ;
            if atoi_check(comma, &mut class) == 0 ||
                   {
                       new_34.name = canonicalise_opt(arg);
                       new_34.name.is_null()
                   } ||
                   !data.is_null() &&
                       {
                           len_3 =
                               parse_hex(data, data,
                                         -(1),
                                         0,
                                         0) ;
                           (len_3) ==
                               (1).wrapping_neg()
                       } {
                free(new_34.name);
                strcpy(errstr,
                       "bad RR record" );
                free(new_34);
                return 0
            }
            new_34.len = 0 ;
            new_34.class = class ;
            new_34.next = daemon.rr;
            daemon.rr = new_34;
            if !data.is_null() {
                new_34.txt = opt_malloc(len_3);
                new_34.len = len_3 ;
                memcpy(new_34.txt,
                       data, len_3);
            }
            current_block = 7879481898411272068;
        }
        356 => {
            /* --caa-record */
            let mut new_35: TxtRecord = 0 ;
            let mut tag: &mut String = 0 ;
            let mut value: &mut String = 0 ;
            let mut flags: i32 = 0;
            comma = split(arg);
            tag = split(comma);
            value = split(tag);
            new_35 =
                opt_malloc(::std::mem::size_of::<TxtRecord>()) ;
            new_35.next = daemon.rr;
            daemon.rr = new_35;
            if atoi_check(comma, &mut flags) == 0 || tag.is_null() ||
                   value.is_null() ||
                   {
                       new_35.name = canonicalise_opt(arg);
                       new_35.name.is_null()
                   } {
                strcpy(errstr,
                       "bad CAA record" );
                return 0
            }
            unhide_metas(tag);
            unhide_metas(value);
            new_35.len =
                strlen(tag).wrapping_add(strlen(value)).wrapping_add(2                      libc::c_int
                                                                                       )
                    ;
            new_35.txt =
                opt_malloc(new_35.len );
            *new_35.txt.offset(0) =
                flags;
            *new_35.txt.offset(1) =
                strlen(tag);
            memcpy(&mut *new_35.txt.offset(2)                 mut Vec<u8>,
                   tag, strlen(tag));
            memcpy(&mut *new_35.txt.offset((2libc::c_ulong).wrapping_add((strlen
                                                                                                                     fn(_:
                                                                                                             *const libc::c_char)
                                                                                        ->
                                                                                     )(tag))
                                                 )                 mut Vec<u8>,
                   value, strlen(value));
            new_35.class = 257 ;
            current_block = 7879481898411272068;
        }
        89 => {
            /* --txt-record */
            let mut new_36: TxtRecord =
                0 ; /* room for extra counts */
            let mut p_0: mut Vec<u8> = 0;
            let mut cnt: mut Vec<u8> = 0;
            let mut len_4: usize = 0;
            comma = split(arg);
            new_36 =
                opt_malloc(::std::mem::size_of::<TxtRecord>()) ;
            new_36.class = 1 ;
            new_36.stat = 0;
            new_36.name = canonicalise_opt(arg);
            if new_36.name.is_null() {
                strcpy(errstr,
                       "bad TXT record" );
                free(new_36);
                return 0
            }
            new_36.next = daemon.txt;
            daemon.txt = new_36;
            len_4 =
                if !comma.is_null() {
                    strlen(comma)
                } else { 0 };
            len_4 =
                (len_4        ).wrapping_add(len_4.wrapping_div(255                     libc::c_int
                                                                                     ).wrapping_add(1                               libc::c_int                        ))
                     ;
            p_0 = opt_malloc(len_4);
            new_36.txt = p_0;
            let fresh29 = p_0;
            p_0 = p_0.offset(1);
            cnt = fresh29;
            *cnt = 0;
            while !comma.is_null() && *comma != 0 {
                let fresh30 = comma;
                comma = comma.offset(1);
                let mut c: libc::c_uchar = *fresh30;
                if c == ',' as i32 ||
                       *cnt == 255 {
                    if c != ',' as i32 {
                        comma = comma.offset(-1)
                    }
                    let fresh31 = p_0;
                    p_0 = p_0.offset(1);
                    cnt = fresh31;
                    *cnt = 0
                } else {
                    let fresh32 = p_0;
                    p_0 = p_0.offset(1);
                    *fresh32 =
                        unhide_meta(c);
                    *cnt = cnt.wrapping_add(1)
                }
            }
            new_36.len =
                p_0.wrapping_offset_from(new_36.txt);
            current_block = 7879481898411272068;
        }
        87 => {
            /* --srv-host */
            let mut port: i32 = 1;
            let mut priority: i32 = 0;
            let mut weight: i32 = 0;
            let mut name_3: &mut String = 0 ;
            let mut target_2: &mut String = 0 ;
            let mut new_37: MxSrvRecord = 0 ;
            comma = split(arg);
            name_3 = canonicalise_opt(arg);
            if name_3.is_null() {
                strcpy(errstr,
                       "bad SRV record" );
                return 0
            }
            if !comma.is_null() {
                arg = comma;
                comma = split(arg);
                target_2 = canonicalise_opt(arg);
                if target_2.is_null() {
                    strcpy(errstr,
                           "bad SRV target"                          *const libc::c_char);
                    free(name_3);
                    return 0
                }
                if !comma.is_null() {
                    arg = comma;
                    comma = split(arg);
                    if atoi_check16(arg, &mut port) == 0 {
                        free(name_3);
                        strcpy(errstr,
                               "invalid port number"                              *const libc::c_char);
                        free(target_2);
                        return 0
                    }
                    if !comma.is_null() {
                        arg = comma;
                        comma = split(arg);
                        if atoi_check16(arg, &mut priority) == 0 {
                            free(name_3);
                            strcpy(errstr,
                                   "invalid priority"                                  *const libc::c_char);
                            free(target_2);
                            return 0
                        }
                        if !comma.is_null() &&
                               atoi_check16(comma, &mut weight) == 0 {
                            free(name_3);
                            strcpy(errstr,
                                   "invalid weight"                                  *const libc::c_char);
                            free(target_2);
                            return 0
                        }
                    }
                }
            }
            new_37 =
                opt_malloc(::std::mem::size_of::<MxSrvRecord>()) ;
            new_37.next = daemon.mxnames;
            daemon.mxnames = new_37;
            new_37.issrv = 1;
            new_37.name = name_3;
            new_37.target = target_2;
            new_37.srvport = port;
            new_37.priority = priority;
            new_37.weight = weight;
            current_block = 7879481898411272068;
        }
        308 => {
            /* --host-record */
            let mut new_38: HostRecord = Default::default();
            if arg.is_null() || { comma = split(arg); comma.is_null() } {
                strcpy(errstr, "Bad host-record" );
                return 0
            }
            new_38.ttl = -(1);
            new_38.flags = 0;
            while !arg.is_null() {
                let mut addr_1: NetAddress = NetAddress {addr4: NetAddress {s_addr: 0,},};
                let mut dig_0: &mut String = 0 ;
                dig_0 = arg;
                while *dig_0 != 0 {
                    if dig_0 < '0' as i32 ||
                           *dig_0 > '9' as i32 {
                        break ;
                    }
                    dig_0 = dig_0.offset(1)
                }
                if *dig_0 == 0 {
                    new_38.ttl = atoi(arg)
                } else if inet_pton(2, arg, &mut addr_1.addr4) != 0 {
                    new_38.addr = addr_1.addr4;
                    new_38.flags |= 2
                } else if inet_pton(10, arg, &mut addr_1.addr6) != 0 {
                    new_38.addr6 = addr_1.addr6;
                    new_38.flags |= 1
                } else {
                    let mut nomem: i32 = 0;
                    let mut canon: &mut String = canonicalise(arg, &mut nomem);
                    let mut nl: NameList;
                    if canon.is_null() {
                        let mut tmp_3: NameList = new_38.names;
                        let mut next: NameList = 0 ;
                        new_38.names.clear();
                        strcpy(errstr, "Bad name in host-record");
                        return 0
                    }
                    nl = Default::default();
                    nl.name = canon;
                    if new_38.names.is_null() {
                        new_38.names = nl
                    } else {
                        let mut tmp_4: NameList = 0 ;
                        // tmp_4 = new_38.names;
                        // while !(*tmp_4).next.is_null() {
                        //     tmp_4 = (*tmp_4).next
                        // }
                        // (*tmp_4).next = nl
                    }
                }
                arg = comma;
                comma = split(arg)
            }
            /* Keep list order */
            if daemon.host_records_tail.is_null() {
                daemon.host_records = new_38.clone()
            } else { daemon.host_records_tail.next = new_38.clone() }
            new_38.next = 0 ;
            daemon.host_records_tail = new_38.clone();
            current_block = 7879481898411272068;
        }
        _ => {
            strcpy(errstr, "unsupported option (check that dnsmasq was compiled with DHCP/TFTP/DNSSEC/DBus support)");
            return 0
        }
    }
    match current_block {
        2926860427235594157 =>
        /* --ignore-address */
        {
            let mut addr_0: NetAddress = Default::default(); /* error */
            unhide_metas(arg);
            if !arg.is_null() &&
                   inet_pton(2, arg, &mut addr_0)
                       > 0 {
                let mut baddr: BogusAddr = Default::default();
                if option == 'B' as i32 {
                    // baddr.next = daemon.bogus_addr;
                    // daemon.bogus_addr = baddr
                } else {
                    // baddr.next = daemon.ignore_addr;
                    // daemon.ignore_addr = baddr
                }
                baddr.addr = addr_0
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
                let mut new_9: Iname =
                    opt_malloc(::std::mem::size_of::<Iname>());
                comma = split(arg);
                new_9.name = opt_string_alloc(arg);
                if option == 'I' as i32 {
                    new_9.next = daemon.if_except;
                    daemon.if_except = new_9
                } else if option == 258 {
                    new_9.next = daemon.tftp_interfaces;
                    daemon.tftp_interfaces = new_9
                } else {
                    new_9.next = daemon.dhcp_except;
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
            let mut serv_1: Server = 0;
            let mut newlist: Server = 0;
            unhide_metas(arg);
            if !arg.is_null() &&
                   (*arg == '/' as i32 ||
                        option == 298) {
                let mut rebind: i32 =
                    !(*arg == '/' as i32);
                let mut end_0: &mut String = 0 ;
                if rebind == 0 { arg = arg.offset(1) }
                while rebind != 0 ||
                          {
                              end_0 =
                                  split_chr(arg, '/' );
                              !end_0.is_null()
                          } {
                    let mut domain: &mut String =
                        0 ;
                    /* elide leading dots - they are implied in the search algorithm */
                    while *arg == '.' as i32 {
                        arg = arg.offset(1)
                    }
                    /* # matches everything and becomes a zero length domain string */
                    if strcmp(arg,
                              "#" ) ==
                           0 {
                        domain =
                            ""                           &mut String
                    } else if strlen(arg) != 0
                                  &&
                                  {
                                      domain = canonicalise_opt(arg);
                                      domain.is_null()
                                  } {
                        strcpy(errstr, gen_err);
                        return 0
                    }
                    serv_1 = Default::default();
                    serv1.next = newlist;
                    newlist = serv_1;
                    serv1.domain = domain;
                    serv1.flags =
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
                newlist = Default::default();
                newlist.uid = rand32()
            }
            if servers_only != 0 && option == 'S' as i32 {
                newlist.flags |= 4096
            }
            if option == 'A' as i32 {
                newlist.flags |= 4;
                if newlist.flags & (8 | 32)
                       == 0 {
                    server_list_free(newlist);
                    strcpy(errstr, gen_err);
                    return 0
                }
            } else if option == 298 {
                newlist.flags |= 2048
            }
            if arg.is_null() || *arg == 0 {
                if newlist.flags & 2048 == 0 {
                    newlist.flags |= 2
                }
                /* no server */
            } else if strcmp(arg,
                             "#" ) ==
                          0 {
                newlist.flags |= 1024
            } else {
                let mut err_0: &mut String =
                    parse_server(arg, &mut newlist.addr,
                                 &mut newlist.source_addr,
                                 newlist.interface.as_mut_ptr(),
                                 &mut newlist.flags); /* treat in ordinary way */
                if !err_0.is_null() {
                    server_list_free(newlist);
                    strcpy(errstr, err_0);
                    return 0
                }
            }
            serv_1 = newlist;
            while !serv1.next.is_null() {
                (*serv1.next).flags |=
                    serv1.flags & !(8 | 32);
                (*serv1.next).addr = serv1.addr;
                (*serv1.next).source_addr = serv1.source_addr;
                strcpy((*serv1.next).interface.as_mut_ptr(),
                       serv1.interface.as_mut_ptr());
                serv_1 = serv1.next
            }
            serv1.next = daemon.servers;
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
            let mut new_24: DhcpNetIdList =
                opt_malloc(::std::mem::size_of::<DhcpNetIdList>());
            let mut list_1: DhcpNetId = 0 ;
            if option == 'J' as i32 {
                new_24.next = daemon.dhcp_ignore;
                daemon.dhcp_ignore = new_24
            } else if option == 282 {
                new_24.next = daemon.force_broadcast;
                daemon.force_broadcast = new_24
            } else if option == '3' as i32 {
                new_24.next = daemon.bootp_dynamic;
                daemon.bootp_dynamic = new_24
            } else if option == 296 {
                new_24.next = daemon.dhcp_gen_names;
                daemon.dhcp_gen_names = new_24
            } else {
                new_24.next = daemon.dhcp_ignore_names;
                daemon.dhcp_ignore_names = new_24
            }
            while !arg.is_null() {
                comma = split(arg);
                list_1 =
                    dhcp_netid_create(if is_tag_prefix(arg) != 0 {
                                          arg.offset(4      isize)
                                      } else { arg }, list_1);
                arg = comma
            }
            new_24.list = list_1;
            current_block = 7879481898411272068;
        }
        9763990383449182594 =>
        /* --dhcp-subscrid */
        {
            let mut p: mut Vec<u8> = 0;
            let mut dig: i32 = 0;
            let mut new_23: DhcpVendor =
                opt_malloc(::std::mem::size_of::<DhcpVendor>()) ;
            comma = split(arg);
            if comma.is_null() {
                strcpy(errstr, gen_err);
                free(new_23);
                return 0
            }
            new_23.netid.net = opt_string_alloc(set_prefix(arg));
            /* check for hex string - must digits may include : must not have nothing else,
	    only allowed for agent-options. */
            arg = comma;
            comma = split(arg);
            if !comma.is_null() {
                if option != 'U' as i32 ||
                       strstr(arg,
                              "enterprise:") != arg {
                    free(new_23.netid.net);
                    strcpy(errstr, gen_err);
                    free(new_23);
                    return 0
                } else {
                    new_23.enterprise =
                        atoi(arg.offset(11))                      libc::c_uint
                }
            } else { comma = arg }
            p = comma;
            while *p != 0 {
                if *(*__ctype_b_loc()).offsetp &
                       _ISXDIGIT                      libc::c_int != 0 {
                    dig = 1
                } else if *p != ':' as i32 { break ; }
                p = p.offset(1)
            }
            unhide_metas(comma);
            if option == 'U' as i32 || option == 'j' as i32 ||
                   *p != 0 || dig == 0 {
                new_23.len = strlen(comma);
                new_23.data =
                    opt_malloc(new_23.len ) ;
                memcpy(new_23.data,
                       comma,
                       new_23.len);
            } else {
                new_23.len =
                    parse_hex(comma, comma,
                              strlen(comma),
                              0, 0);
                new_23.data =
                    opt_malloc(new_23.len ) ;
                memcpy(new_23.data,
                       comma,
                       new_23.len);
            }
            match option {
                106 => { new_23.match_type = 2 }
                85 => { new_23.match_type = 1 }
                268 => { new_23.match_type = 3 }
                269 => { new_23.match_type = 4 }
                270 => { new_23.match_type = 5 }
                _ => { }
            }
            new_23.next = daemon.dhcp_vendors;
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
            let mut new_3: HostsFile =
                opt_malloc(::std::mem::size_of::<HostsFile>());
            static mut hosts_index: u32 =
                3;
            new_3.fname = opt_string_alloc(arg);
            let fresh26 = hosts_index;
            hosts_index = hosts_index.wrapping_add(1);
            new_3.index = fresh26;
            new_3.flags = 0;
            if option == 'H' as i32 {
                new_3.next = daemon.addn_hosts;
                daemon.addn_hosts = new_3
            } else if option == 273 {
                new_3.next = daemon.dhcp_hosts_file;
                daemon.dhcp_hosts_file = new_3
            } else if option == 280 {
                new_3.next = daemon.dhcp_opts_file;
                daemon.dhcp_opts_file = new_3
            } else {
                new_3.next = daemon.dynamic_dirs;
                daemon.dynamic_dirs = new_3;
                if option == 340 {
                    new_3.flags |= 16
                } else if option == 341 {
                    new_3.flags |= 32
                } else if option == 342 {
                    new_3.flags |= 8
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
            let mut ttl: i32 = 0;
            if atoi_check(arg, &mut ttl) == 0 {
                strcpy(errstr, gen_err);
                return 0
            } else {
                if option == 283 {
                    daemon.neg_ttl = ttl
                } else if option == 297 {
                    daemon.max_ttl = ttl
                } else if option == 339 {
                    if ttl > 3600 { ttl = 3600 }
                    daemon.min_cache_ttl = ttl
                } else if option == 312 {
                    daemon.max_cache_ttl = ttl
                } else if option == 315 {
                    daemon.auth_ttl = ttl
                } else if option == 348 {
                    daemon.dhcp_ttl = ttl;
                    daemon.use_dhcp_ttl =
                        1
                } else { daemon.local_ttl = ttl }
            }
        }
        _ => { }
    }
    return 1;
}
//  fn read_file(mut file: &mut String, mut f: *mut FILE,
//                                mut hard_opt: i32) {
//     let mut lineno: i32 = 0;
//     let mut buff: &mut String = daemon.namebuff;
//     let mut current_block_66: u64;
//     while !fgets(buff, 1025, f).is_null() {
//         let mut white: i32 = 0;
//         let mut i: i32 = 0;
//         let mut option: i32 =
//             if hard_opt == 332 {
//                 0
//             } else { hard_opt };
//         let mut errmess: &mut String = 0 ;
//         let mut p: &mut String = 0 ;
//         let mut arg: &mut String = 0 ;
//         let mut start: &mut String = 0 ;
//         let mut len: usize = 0;
//         /* Memory allocation failure longjmps here if mem_recover == 1 */
//         if option != 0 || hard_opt == 332 {
//             if _setjmp(mem_jmp.as_mut_ptr()) != 0 { continue ; }
//             ::std::ptr::write_volatile(&mut mem_recover,
//                                        1)
//         }
//         arg = 0 ;
//         ::std::ptr::write_volatile(&mut lineno,
//                                    ::std::ptr::read_volatile::<libc::c_int>(&lineno
//                                                                                                             *const libc::c_int)
//                                        + 1);
//         errmess = 0 ;
//         /* Implement quotes, inside quotes we allow \\ \" \n and \t
// 	 metacharacters get hidden also strip comments */
//         white = 1;
//         p = buff;
//         loop  {
//             if !(*p != 0) { current_block_66 = 12199444798915819164; break ; }
//             if *p == '\"' as i32 {
//                 memmove(p,
//                         p.offset(1)
//                         strlen(p.offset(1)).wrapping_add(1                  libc::c_int
//                                                                                ));
//                 while *p != 0 &&
//                           *p != '\"' as i32 {
//                     if *p == '\\' as i32 &&
//                            !strchr("\"tnebr\\"                                  *const libc::c_char,
//                                    *p.offset(1)                                 libc::c_int).is_null() {
//                         if *p.offset(1)
//                                == 't' as i32 {
//                             *p.offset(1) =
//                                 '\t'
//                         } else if *p.offset(1)                                libc::c_int == 'n' as i32 {
//                             *p.offset(1) =
//                                 '\n'
//                         } else if *p.offset(1)                                libc::c_int == 'b' as i32 {
//                             *p.offset(1) =
//                                 '\u{8}'
//                         } else if *p.offset(1)                                libc::c_int == 'r' as i32 {
//                             *p.offset(1) =
//                                 '\r'
//                         } else if *p.offset(1)                                libc::c_int == 'e' as i32 {
//                             /* escape */
//                             *p.offset(1) =
//                                 '\u{1b}'
//                         }
//                         memmove(p,
//                                 p.offset(1)
//                                 strlen(p.offset(1 isize)).wrapping_add(1                          libc::c_int
//                                                                                                ));
//                     }
//                     *p = hide_meta(*p);
//                     p = p.offset(1)
//                 }
//                 if *p == 0 {
//                     errmess =
//                         "missing \""
//                             ;
//                     current_block_66 = 15635431839692940240;
//                     break ;
//                 } else {
//                     memmove(p,
//                             p.offset(1)
//                             strlen(p.offset(1    )).wrapping_add(1                      libc::c_int
//                                                                                        ));
//                 }
//             }
//             if *(*__ctype_b_loc()).offset(*p)             libc::c_int &
//                    _ISSPACE  !=
//                    0 {
//                 *p = ' ' ;
//                 white = 1
//             } else if white != 0 && *p == '#' as i32 {
//                 *p = 0;
//                 current_block_66 = 12199444798915819164;
//                 break ;
//             } else { white = 0 }
//             p = p.offset(1)
//         }
//         match current_block_66 {
//             12199444798915819164 => {
//                 /* strip leading spaces */
//                 start = buff;
//                 while *start != 0 &&
//                           *start == ' ' as i32 {
//                     start = start.offset(1)
//                 }
//                 /* strip trailing spaces */
//                 len = strlen(start);
//                 while len != 0 &&
//                           *start.offset(len.wrapping_sub(1   )) ==
//                               ' ' as i32 {
//                     len = len.wrapping_sub(1)
//                 }
//                 if len == 0 { continue ; }
//                 *start.offset(len) =
//                     0;
//                 if option != 0 {
//                     arg = start
//                 } else {
//                     p = strchr(start, '=' as i32);
//                     if !p.is_null() {
//                         /* allow spaces around "=" */
//                         arg = p.offset(1);
//                         while *arg == ' ' as i32 {
//                             arg = arg.offset(1)
//                         }
//                         while p >= start &&
//                                   (*p == ' ' as i32 ||
//                                        *p == '=' as i32) {
//                             *p = 0;
//                             p = p.offset(-1)
//                         }
//                     } else { arg = 0  }
//                 }
//                 if option == 0 {
//                     ::std::ptr::write_volatile(&mut option,
//                                                0);
//                     i = 0;
//                     while !opts[i ].name.is_null() {
//                         if strcmp(opts[i ].name, start) ==
//                                0 {
//                             ::std::ptr::write_volatile(&mut option        ,
//                                                        opts[i ].val);
//                             break ;
//                         } else { i += 1 }
//                     }
//                     if option == 0 {
//                         errmess =
//                             "bad option"                           *const libc::c_char
//                     } else if opts[i ].has_arg == 0 &&
//                                   !arg.is_null() {
//                         errmess =
//                             "extraneous parameter"                           *const libc::c_char
//                     } else if opts[i ].has_arg == 1 &&
//                                   arg.is_null() {
//                         errmess =
//                             "missing parameter"                           *const libc::c_char
//                     } else if hard_opt == 332 &&
//                                   option != 'S' as i32 &&
//                                   option != 332 {
//                         errmess =
//                             "illegal option"                           *const libc::c_char
//                     }
//                 }
//             }
//             _ => { }
//         }
//         if !errmess.is_null() { strcpy(daemon.namebuff, errmess); }
//         if !errmess.is_null() ||
//                one_opt(option, arg, daemon.namebuff,
//                        "error"                      &mut String, 0,
//                        (hard_opt == 332)) == 0 {
//             sprintf(daemon.namebuff.offset(strlen(daemon.namebuff)
//                                                          ),
//                     " at line %d of %s"                   *const libc::c_char, lineno, file);
//             if hard_opt != 0 {
//                 my_syslog(3,
//                           "%s" ,
//                           daemon.namebuff);
//             } else {
//                 die("%s"                   &mut String, daemon.namebuff,
//                     1);
//             }
//         }
//     }
//     ::std::ptr::write_volatile(&mut mem_recover,
//                                0);
//     fclose(f);
// }

#[no_mangle]
pub  fn option_read_dynfile(mut file: &mut String,
                                             mut flags: i32)
 -> i32 {
    my_syslog((3) << 3 | 6,
              "read %s" , file);
    if flags & 16 != 0 {
        return one_file(file, 272)
    } else {
        if flags & 32 != 0 {
            return one_file(file, 279)
        }
    }
    return 0;
}
 fn one_file(mut file: &mut String,
                              mut hard_opt: i32) -> i32 {
    let mut f: FILE = 0 ;
    let mut nofile_ok: i32 = 0;
    static mut read_stdin: i32 = 0;
    static mut filesread: fileread =
        0 ;
    if hard_opt == '7' as i32 {
        /* default conf-file reading */
        hard_opt = 0;
        nofile_ok = 1
    }
    if hard_opt == 0 &&
           strcmp(file, "-" ) ==
               0 {
        if read_stdin == 1 { return 1 }
        read_stdin = 1;
        file =
            "stdin"           &mut String;
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
            let mut r: fileread = 0 read;
            r = filesread;
            while !r.is_null() {
                if r.dev == statbuf.st_dev && r.ino == statbuf.st_ino {
                    return 1
                }
                r = r.next
            }
            r =
                safe_malloc(::std::mem::size_of::<fileread>()                   ) read;
            r.next = filesread;
            filesread = r;
            r.dev = statbuf.st_dev;
            r.ino = statbuf.st_ino
        }
        f = fopen(file, "r" );
        if f.is_null() {
            if *__errno_location() == 2 && nofile_ok != 0 {
                return 1
            } else {
                let mut str: &mut String =
                    "cannot read %s: %s"                   *const libc::c_char ;
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
pub  fn expand_filelist(mut list: &mut HostsFile)
 -> HostsFile {
    let mut i: u32 = 0;
    let mut ah: HostsFile = 0;
    /* find largest used index */
    i = 3;
    ah = list;
    while !ah.is_null() {
        if i <= ah.index {
            i = ah.index.wrapping_add(1)
        }
        if ah.flags & 1 != 0 {
            ah.flags |= 2
        } else { ah.flags &= !(2) }
        ah = ah.next
    }
    ah = list;
    while !ah.is_null() {
        if ah.flags & 2 == 0 {
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
            if stat(ah.fname, &mut buf) != -(1) &&
                   buf.st_mode & 0o170000 ==
                       0o40000 {
                let mut dir_stream: DIR = 0 ;
                let mut ent: dirent = 0 ent;
                /* don't read this as a file */
                ah.flags |= 2;
                dir_stream = opendir(ah.fname);
                if dir_stream.is_null() {
                    my_syslog(3,
                              "cannot access directory %s: %s"                            *const u8,
                              ah.fname, strerror(*__errno_location()));
                } else {
                    loop  {
                        ent = readdir(dir_stream);
                        if ent.is_null() { break ; }
                        let mut lendir: usize = strlen(ah.fname);
                        let mut lenfile: usize =
                            strlen(ent.d_name.as_mut_ptr());
                        let mut ah1: HostsFile = 0;
                        let mut path: &mut String =
                            0 ;
                        /* ignore emacs backups and dotfiles */
                        if lenfile == 0 ||
                               ent.d_name[lenfile.wrapping_sub(1                   libc::c_int
                                                                                 )
                                                 ] ==
                                   '~' as i32 ||
                               ent.d_name[0 ]                             libc::c_int == '#' as i32 &&
                                   ent.d_name[lenfile.wrapping_sub(1                       libc::c_int
                                                                                         )
                                                     ]
                                       == '#' as i32 ||
                               ent.d_name[0 ]                             libc::c_int == '.' as i32 {
                            continue ;
                        }
                        /* see if we have an existing record.
		       dir is ah->fname
		       file is ent->d_name
		       path to match is ah1->fname */
                        ah1 = list;
                        while !ah1.is_null() {
                            if lendir < strlen(ah1.fname) &&
                                   strstr(ah1.fname, ah.fname) ==
                                       ah1.fname &&
                                   *ah1.fname.offset(lendir)                                 libc::c_int == '/' as i32 &&
                                   strcmp(ah1.fname.offset(lendir               isize).offset(1
                                                                                                                    libc::c_int
                                                                              ),
                                          ent.d_name.as_mut_ptr()) ==
                                       0 {
                                ah1.flags &= !(2);
                                break ;
                            } else { ah1 = ah1.next }
                        }
                        /* make new record */
                        if ah1.is_null() {
                            ah1 =
                                whine_malloc(::std::mem::size_of::<HostsFile>()
                                                )                              HostsFile;
                            if ah1.is_null() { continue ; }
                            path =
                                whine_malloc(lendir.wrapping_add(lenfile).wrapping_add(2     libc::c_int
                                                                                                                           ))
                                    ;
                            if path.is_null() {
                                free(ah1);
                                continue ;
                            } else {
                                strcpy(path, ah.fname);
                                strcat(path,
                                       "/"                                      *const libc::c_char);
                                strcat(path, ent.d_name.as_mut_ptr());
                                ah1.fname = path;
                                let fresh33 = i;
                                i = i.wrapping_add(1);
                                ah1.index = fresh33;
                                ah1.flags = 1;
                                ah1.next = list;
                                list = ah1
                            }
                        }
                        /* inactivate record if not regular file */
                        if ah1.flags & 1 != 0 &&
                               stat(ah1.fname, &mut buf) !=
                                   -(1) &&
                               !(buf.st_mode &
                                     0o170000
                                     ==
                                     0o100000)
                           {
                            ah1.flags |= 2
                        }
                    }
                    closedir(dir_stream);
                }
            }
        }
        ah = ah.next
    }
    return list;
}
#[no_mangle]
pub  fn read_servers_file() {
    let mut f: FILE = 0 ;
    f =
        fopen(daemon.servers_file,
              "r" );
    if f.is_null() {
        my_syslog(3,
                  "cannot read %s: %s", daemon.servers_file,
                  strerror(*__errno_location()));
        return
    }
    mark_servers(4096);
    cleanup_servers();
    read_file(daemon.servers_file, f, 332);
}
 fn clear_dynamic_conf() {
    let mut configs: DhcpConfig = 0;
    let mut cp: DhcpConfig = 0;
    let mut up: DhcpConfig;
    /* remove existing... */
    up = &mut daemon.dhcp_conf;
    configs = daemon.dhcp_conf;
    while !configs.is_null() {
        cp = configs.next;
        if configs.flags & 2048 != 0 {
            let mut mac: HwaddrConfig = 0;
            let mut tmp: HwaddrConfig = 0;
            let mut list: DhcpNetIdList = 0;
            let mut tmplist: DhcpNetIdList = 0;
            mac = configs.hwaddr;
            while !mac.is_null() {
                tmp = mac.next;
                free(mac);
                mac = tmp
            }
            if configs.flags & 2 != 0 {
                free(configs.clid);
            }
            list = configs.netid;
            while !list.is_null() {
                free(list.list);
                tmplist = list.next;
                free(list);
                list = tmplist
            }
            if configs.flags & 16 != 0 {
                free(configs.hostname);
            }
            *up = configs.next;
            free(configs);
        } else { up = &mut configs.next }
        configs = cp
    };
}
 fn clear_dynamic_opt() {
    let mut opts_0: DhcpOpt = 0 ;
    let mut cp: DhcpOpt = 0 ;
    let mut up: DhcpOpt;
    let mut id: DhcpNetId = 0 ;
    let mut next: DhcpNetId = 0 ;
    up = &mut daemon.dhcp_opts;
    opts_0 = daemon.dhcp_opts;
    while !opts_0.is_null() {
        cp = opts_0.next;
        if opts_0.flags & 32 != 0 {
            if opts_0.flags & 256 != 0 {
                free(opts_0.u.vendor_class);
            }
            free(opts_0.val);
            id = opts_0.netid;
            while !id.is_null() {
                next = id.next;
                free(id.net);
                free(id);
                id = next
            }
            *up = opts_0.next;
            free(opts_0);
        } else { up = &mut opts_0.next }
        opts_0 = cp
    };
}
#[no_mangle]
pub  fn reread_dhcp() {
    let mut hf: HostsFile = 0;
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
            if hf.flags & 2 == 0 {
                if one_file(hf.fname, 272) != 0 {
                    my_syslog((3) << 3 |
                                  6,
                              "read %s", hf.fname);
                }
            }
            hf = hf.next
        }
    }
    if !daemon.dhcp_opts_file.is_null() {
        daemon.dhcp_opts_file =
            expand_filelist(daemon.dhcp_opts_file);
        hf = daemon.dhcp_opts_file;
        while !hf.is_null() {
            if hf.flags & 2 == 0 {
                if one_file(hf.fname, 279) != 0 {
                    my_syslog((3) << 3 |
                                  6,
                              "read %s", hf.fname);
                }
            }
            hf = hf.next
        }
    }
    /* Setup notify and read pre-existing files. */
    set_dynamic_inotify(16 | 32,
                        0, 0,
                        0);
}
#[no_mangle]
pub  fn read_opts(mut argc: i32,
                                   mut argv: String,
                                   mut compile_opts: &mut String) {
    let mut argbuf_size: usize = 1025 ;
    let mut argbuf: &mut String =
        opt_malloc(argbuf_size) ;
    let mut buff: &mut String =
        opt_malloc(1025 ) ;
    let mut option: i32 = 0;
    let mut testmode: i32 = 0;
    let mut arg: &mut String = 0 ;
    let mut conffile: &mut String = 0 ;
    opterr = 0;
    dnsmasq_daemon =
        opt_malloc(::std::mem::size_of::<DnsmasqDaemon>())
            ;
    memset(dnsmasq_daemon, 0,
           ::std::mem::size_of::<DnsmasqDaemon>());
    daemon.namebuff = buff;
    /* Set defaults - everything else is zero or NULL */
    daemon.cachesize = 150;
    daemon.ftabsize = 150;
    daemon.port = 53;
    daemon.dhcp_client_port = 68;
    daemon.dhcp_server_port = 67;
    daemon.default_resolv.is_default = 1;
    daemon.default_resolv.name =
        "/etc/resolv.conf"       &mut String;
    daemon.resolv_files = &mut daemon.default_resolv;
    daemon.username =
        "nobody"       &mut String;
    daemon.runfile =
        "/var/run/dnsmasq.pid"       &mut String;
    daemon.dhcp_max = 1000;
    daemon.tftp_max = 50;
    daemon.edns_pktsz = 4096 ;
    daemon.log_fac = -(1);
    daemon.auth_ttl = 600;
    daemon.soa_refresh = 1200;
    daemon.soa_retry = 180;
    daemon.soa_expiry = 1209600;
    daemon.max_port = 65535;
    daemon.min_port = 1024;
    add_txt("version.bind"           &mut String,
            "dnsmasq-2.84rc2"           &mut String, 0);
    add_txt("authors.bind"           &mut String,
            "Simon Kelley"           &mut String, 0);
    add_txt("copyright.bind"           &mut String,
            "Copyright (c) 2000-2021 Simon Kelley", 0);
    add_txt("cachesize.bind"           &mut String, 0 , 1);
    add_txt("insertions.bind"           &mut String, 0 , 2);
    add_txt("evictions.bind"           &mut String, 0 , 3);
    add_txt("misses.bind"           &mut String, 0 , 4);
    add_txt("hits.bind"           &mut String, 0 , 5);
    add_txt("auth.bind"           &mut String, 0 , 6);
    add_txt("servers.bind"           &mut String, 0 , 7);
    loop  {
        option =
            getopt_long(argc, argv,
                        "951yZDNLERKzowefnbvhdkqr:m:p:c:l:s:i:t:u:g:a:x:S:C:A:T:H:Q:I:B:F:G:O:M:X:V:U:j:P:J:W:Y:2:4:6:7:8:0:3:"
                            ,
                        opts.as_ptr(), 0);
        if option == -(1) {
            while optind < argc {
                let mut c: mut Vec<u8> =
                    *argv.offset(optind);
                while *c != 0 {
                    if *(*__ctype_b_loc()).offsetc
                           &
                           _ISSPACE  == 0 {
                        die("junk found in command line"                           *const libc::c_char ,
                            0 , 1);
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
                    free(argbuf);
                    argbuf_size =
                        strlen(optarg).wrapping_add(1     libc::c_ulong);
                    argbuf = opt_malloc(argbuf_size)
                }
                safe_strncpy(argbuf, optarg, argbuf_size);
                arg = argbuf
            } else { arg = 0  }
            /* command-line only stuff */
            if option == 293 {
                testmode = 1
            } else if option == 'w' as i32 {
                if argc == 3 &&
                       strcmp(*argv.offset(2),
                              "dhcp" )
                           == 0 {
                    display_opts();
                } else if argc == 3 &&
                              strcmp(*argv.offset(2),
                                     "dhcp6") ==
                                  0 {
                    display_opts6();
                } else { do_usage(); }
                exit(0);
            } else {
                if option == 'v' as i32 {
                    printf("Dnsmasq version %s  %s\n"                          *const libc::c_char,
                           "2.84rc2" ,
                           "Copyright (c) 2000-2021 Simon Kelley"                         *const u8);
                    printf("Compile time options: %s\n\n"                          *const libc::c_char, compile_opts);
                    printf("This software comes with ABSOLUTELY NO WARRANTY.\n"
                               );
                    printf("Dnsmasq is free software, and you are welcome to redistribute it\n"
                               );
                    printf("under the terms of the GNU General Public License, version 2 or 3.\n"
                               );
                    exit(0);
                } else {
                    if option == 'C' as i32 {
                        if conffile.is_null() {
                            conffile = opt_string_alloc(arg)
                        } else {
                            let mut extra: &mut String =
                                opt_string_alloc(arg);
                            one_file(extra, 0);
                            free(extra);
                        }
                    } else if one_opt(option, arg, daemon.namebuff,
                                      "try --help"                                     *const libc::c_char                                    &mut String, 1,
                                      0) == 0 {
                        die("bad command line options: %s"   ,
                            daemon.namebuff, 1);
                    }
                }
            }
        }
    }
    free(argbuf);
    if !conffile.is_null() {
        one_file(conffile, 0);
        free(conffile);
    } else {
        one_file("/etc/dnsmasq.conf"
                     , '7' as i32);
    }
    /* port might not be known when the address is parsed - fill in here */
    if !daemon.servers.is_null() {
        let mut tmp: Server = 0;
        tmp = daemon.servers;
        while !tmp.is_null() {
            if tmp.flags & 16 == 0 {
                if tmp.source_addr.sa.sa_family ==
                       2 {
                    tmp.source_addr.in_0.sin_port =
                        __bswap_16(daemon.query_port)
                } else if tmp.source_addr.sa.sa_family ==
                              10 {
                    tmp.source_addr.in6.sin6_port =
                        __bswap_16(daemon.query_port)
                }
            }
            tmp = tmp.next
        }
    }
    if !daemon.host_records.is_null() {
        let mut hr: HostRecord = 0 ;
        hr = daemon.host_records;
        while !hr.is_null() {
            if hr.ttl == -(1) {
                hr.ttl = daemon.local_ttl
            }
            hr = hr.next
        }
    }
    if !daemon.cnames.is_null() {
        let mut cn: Cname = 0 ;
        let mut cn2: Cname = 0 ;
        let mut cn3: Cname = 0 ;
        /* Fill in TTL for CNAMES now we have local_ttl.
	 Also prepare to do loop detection. */
        cn = daemon.cnames;
        while !cn.is_null() {
            if cn.ttl == -(1) {
                cn.ttl = daemon.local_ttl
            }
            cn.flag = 0;
            cn.targetp = 0 ;
            cn2 = daemon.cnames;
            while !cn2.is_null() {
                if hostname_isequal(cn.target, cn2.alias) != 0 {
                    cn.targetp = cn2;
                    break ;
                } else { cn2 = cn2.next }
            }
            cn = cn.next
        }
        /* Find any CNAME loops.*/
        cn = daemon.cnames;
        while !cn.is_null() {
            cn2 = cn.targetp;
            while !cn2.is_null() {
                if cn2.flag == 1 { break ; }
                if cn2.flag == 2 {
                    die("CNAME loop involving %s"  ,
                        cn.alias, 1);
                }
                cn2.flag = 2;
                cn2 = cn2.targetp
            }
            cn3 = cn.targetp;
            while cn3 != cn2 {
                cn3.flag = 1;
                cn3 = cn3.targetp
            }
            cn = cn.next
        }
    }
    if !daemon.if_addrs.is_null() {
        let mut tmp_0: Iname = 0;
        tmp_0 = daemon.if_addrs;
        while !tmp_0.is_null() {
            if tmp_0.addr.sa.sa_family == 2 {
                tmp_0.addr.in_0.sin_port =
                    __bswap_16(daemon.port)
            } else if tmp_0.addr.sa.sa_family ==
                          10 {
                tmp_0.addr.in6.sin6_port =
                    __bswap_16(daemon.port)
            }
            tmp_0 = tmp_0.next
        }
    }
    /* create default, if not specified */
    if !daemon.authserver.is_null() &&
           daemon.hostmaster.is_null() {
        strcpy(buff, "hostmaster." );
        strcat(buff, daemon.authserver);
        daemon.hostmaster = opt_string_alloc(buff)
    }
    if daemon.dhcp_pxe_vendors.is_null() {
        daemon.dhcp_pxe_vendors =
            opt_malloc(::std::mem::size_of::<DhcpPxeVendor>()) ;
        (*daemon.dhcp_pxe_vendors).data =
            opt_string_alloc("PXEClient"                            *const libc::c_char);
        (*daemon.dhcp_pxe_vendors).next = 0
    }
    /* only one of these need be specified: the other defaults to the host-name */
    if daemon.options[(10).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (10).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 || !daemon.mxnames.is_null() ||
           !daemon.mxtarget.is_null() {
        let mut mx: MxSrvRecord = 0 ;
        if gethostname(buff, 1025 ) ==
               -(1) {
            die("cannot get host-name: %s"               *const libc::c_char ,
                0 , 5);
        }
        mx = daemon.mxnames;
        while !mx.is_null() {
            if mx.issrv == 0 && hostname_isequal(mx.name, buff) != 0 {
                break ;
            }
            mx = mx.next
        }
        if (!daemon.mxtarget.is_null() ||
                daemon.options[(10).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                     ).wrapping_mul(8                                               libc::c_int                                        ))
                                              ] &
                    (1) <<
                        (10).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 ).wrapping_mul(8           libc::c_int    ))
                    != 0) && mx.is_null() {
            mx =
                opt_malloc(::std::mem::size_of::<MxSrvRecord>()) ;
            mx.next = daemon.mxnames;
            mx.issrv = 0;
            mx.target = 0 ;
            mx.name = opt_string_alloc(buff);
            daemon.mxnames = mx
        }
        if daemon.mxtarget.is_null() {
            daemon.mxtarget = opt_string_alloc(buff)
        }
        mx = daemon.mxnames;
        while !mx.is_null() {
            if mx.issrv == 0 && mx.target.is_null() {
                mx.target = daemon.mxtarget
            }
            mx = mx.next
        }
    }
    if daemon.options[(8).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (8).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           == 0 && !daemon.resolv_files.is_null() &&
           !(*daemon.resolv_files).next.is_null() &&
           daemon.options[(5 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (5 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               != 0 {
        die("only one resolv.conf file allowed in no-poll mode."          *const u8 ,
            0 , 1);
    }
    if daemon.options[(15).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   ).wrapping_mul(8                             libc::c_int                      ))
                                     ] &
           (1) <<
               (15).wrapping_rem((::std::mem::size_of::<libc::c_uint>()).wrapping_mul(8
                                                                                                                      libc::c_int
                                                                                                               ))
           != 0 {
        let mut line: &mut String = 0 ;
        let mut f: FILE = 0 ;
        if daemon.options[(8 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                         ] &
               (1) <<
                   (8 )).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                       ).wrapping_mul(8 libc::c_int
                                                                                                                       ))
               != 0 || daemon.resolv_files.is_null() ||
               !(*daemon.resolv_files).next.is_null() {
            die("must have exactly one resolv.conf to read domain from."
                     ,
                0 , 1);
        }
        f =
            fopen((*daemon.resolv_files).name,
                  "r" );
        if f.is_null() {
            die("failed to read %s: %s"               *const libc::c_char ,
                (*daemon.resolv_files).name, 3);
        }
        loop  {
            line = fgets(buff, 1025, f);
            if line.is_null() { break ; }
            let mut token: &mut String =
                strtok(line,
                       " \t\n\r" );
            if token.is_null() ||
                   strcmp(token,
                          "search" )
                       != 0 {
                continue ;
            }
            token =
                strtok(0 ,
                       " \t\n\r" );
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
            die("no search directive found in %s"               *const libc::c_char ,
                (*daemon.resolv_files).name, 5);
        }
    }
    if !daemon.domain_suffix.is_null() {
        /* add domain for any srv record without one. */
        let mut srv: MxSrvRecord = 0 ;
        srv = daemon.mxnames;
        while !srv.is_null() {
            if srv.issrv != 0 && !strchr(srv.name, '.' as i32).is_null()
                   &&
                   strchr(srv.name, '.' as i32) ==
                       strrchr(srv.name, '.' as i32) {
                strcpy(buff, srv.name);
                strcat(buff, "." );
                strcat(buff, daemon.domain_suffix);
                free(srv.name);
                srv.name = opt_string_alloc(buff)
            }
            srv = srv.next
        }
    } else if daemon.options[(20 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                                 ).wrapping_mul(8))
                                            ] &
                  (1) <<
                      (20).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             ).wrapping_mul(8))
                  != 0 {
        die("there must be a default domain when --dhcp-fqdn is set"          *const u8 ,
            0 , 1);
    }
    /* If there's access-control config, then ignore --local-service, it's intended
     as a system default to keep otherwise unconfigured installations safe. */
    if !daemon.if_names.is_null() ||
           !daemon.if_except.is_null() ||
           !daemon.if_addrs.is_null() ||
           !daemon.authserver.is_null() {
        reset_option_bool(49);
    }
    if testmode != 0 {
        fprintf(stderr,
                "dnsmasq: %s.\n" ,
                "syntax check OK" );
        exit(0);
    };
}
