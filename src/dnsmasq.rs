mod arp;
mod auth;
mod blockdata;
mod bpf;
mod cache;
mod conntrack;
mod crypto;
mod dbus;
mod defines;
mod dhcp;
mod dhcp6;
mod dhcp_common;
mod dnsmasq_log;
mod dnsmasq_loop;
mod domain;
mod dump;
mod edns0;
mod forward;
mod hash_questions;
mod helper;
mod in6_addr;
mod in_addr;
mod inotify;
mod ipset;
mod lease;
mod metrics;
mod netlink;
mod network;
mod option;
mod outpacket;
mod poll;
mod radv;
mod rfc1035;
mod rfc2131;
mod rfc3315;
mod rrfilter;
mod slaac;
mod tables;
mod tftp;
mod ubus;
mod util;

use defines::{
    C2rustUnnamed12, __sighandler_t, __sigset_t, CapUserHeader, gid_t, uid_t, DhcpContext,
    DhcpRelay, DnsmasqDaemon, Group, Iname, Passwd, Server, _SC_OPEN_MAX,
};

use crate::arp::{do_arp_script_run, find_mac};
use crate::blockdata::blockdata_init;
use crate::cache::{cache_init, cache_recv_insert, cache_reload, dump_cache};
use crate::defines::{
    NetAddress, C2rustUnnamed14, C2rustUnnamed16, C2rustUnnamed17, C2rustUnnamed26,
    C2rustUnnamed27, IcmpHdr, NetAddress, InAddrT, IpHdr, Irec, Listener, SaFamily, NetAddress,
    TftpTransfer, UserCapData, UserCapHeader, EventDesc, Sigaction,
    DhcpLease, DhcpPacket, NetAddress, Resolvc, ServerFd, TftpPrefix, DIR, IPPROTO_ICMP, SHUT_RDWR,
    SIGALRM, SIGCHLD, SIGHUP, SIGINT, SIGPIPE, SIGTERM, SIGUSR1, SIGUSR2, SOCK_RAW,
    ConstNetAddressArg, NetAddressArg,
};
use crate::dhcp::{dhcp_init, dhcp_read_ethers};
use crate::dhcp6::{dhcp6_init, dhcp6_packet, dhcp_construct_contexts};
use crate::dhcp_common::{
    bind_to_device, dhcp_common_init, dhcp_update_configs, dhcp_context_to_string, log_relay, whichdevice,
};
use crate::dnsmasq_log::{
    check_log_writer, die, flush_log, log_reopen, log_start, my_syslog, set_log_writer,
};
use crate::dump::dump_init;
use crate::forward::{get_new_frec, receive_query, reply_query, resend_query, tcp_request};
use crate::hash_questions::hash_questions_init;
use crate::helper::{create_helper, helper_buf_empty, helper_write};
use crate::inotify::{inotify_check, inotify_dnsmasq_init};
use crate::ipset::ipset_init;
use crate::lease::{
    do_script_run, lease_find_interfaces, lease_init, lease_make_duid, lease_prune,
    lease_update_dns, lease_update_file, lease_update_from_configs, rerun_scripts,
};
use crate::netlink::{netlink_init, netlink_multicast};
use crate::network::{
    check_servers, create_bound_listeners, create_wildcard_listeners, enumerate_interfaces, fix_fd,
    indextoname, is_dad_listeners, join_multicast, loopback_exception, newaddress,
    pre_allocate_sfds, reload_servers, tcp_interface, warn_bound_listeners, warn_int_names,
    warn_wild_labels,
};
use crate::poll::{do_poll, poll_check, poll_listen, poll_reset};
use crate::radv::{icmp6_packet, periodic_ra, ra_init};
use crate::tftp::{check_tftp_listeners, do_tftp_script_run, tftp_request};
use crate::util::{dnsmasq_time, rand16, retry_send, NetAddress_isequal};
use libc;
use log;
use std::process::exit;

// dnsmasq is Copyright (c) 2000-2021 Simon Kelley
//
//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; version 2 dated June, 1991, or
//    (at your option) version 3 dated 29 June, 2007.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <http://www.gnu.org/licenses/>.
/* Declare static char *compiler_opts  in config.h */
// 
// pub static mut daemon: *mut dnsmasq_daemon = 0 as *const daemon as *mut dnsmasq_daemon;

// static mut pid: pid_t = 0;
// static mut pipewrite: libc::c_int = 0;
unsafe fn main_0(mut argc: i32, mut argv: String) -> i32 {
    let mut compiler_opts: String = String::from("");
    let mut bind_fallback: i32 = 0;
    let mut now: time::Instant = 0;
    let mut sigact: libc::sigaction = libc::sigaction {
        sa_sigaction: 0,
        sa_mask: libc::sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut if_tmp: Iname = Default::default();
    let mut piperead: i32 = 0;
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    let mut err_pipe: [libc::c_int; 2] = [0; 2];
    let mut ent_pw: *mut Passwd = 0 ;
    let mut script_uid: uid_t = 0 ;
    let mut script_gid: gid_t = 0 ;
    let mut gp: *mut Group = 0 ;
    let mut i: i32 = 0;
    // if cfg!(target_os = "linux") {
    //     let mut max_fd: libc::c_long = libc::sysconf(libc::_SC_OPEN_MAX);
    // }
    let mut baduser: String;
    let mut log_err: i32 = 0;
    let mut chown_warn: i32 = 0;
    // let mut hdr: cap_user_header_t = 0 as cap_user_header_t;
    let mut hdr: UserCapHeader = Default::default();
    let mut data: UserCapData = Default::default();
    let mut need_cap_net_admin: i32 = 0;
    let mut need_cap_net_raw: i32 = 0;
    let mut need_cap_net_bind_service: i32 = 0;
    let mut bound_device: String = String::new();
    let mut did_bind: bool = false;
    let mut serv: Server = Default::default();
    let mut netlink_warn: String = String::new();
    let mut context: DhcpContext = Default::default();
    let mut relay: DhcpRelay = Default::default();
    let mut tftp_prefix_missing: i32 = 0;
    sigact.__sigaction_handler.sa_handler =
        Some(sig_handler as fn(_: i32) -> ());
    sigact.sa_flags = 0;
    if cfg!(target = "linux") {
        libc::sigemptyset(&mut sigact.sa_mask );
        libc::sigaction(SIGUSR1, &mut sigact, 0 );
        libc::sigaction(SIGUSR2, &mut sigact, 0 );
        libc::sigaction(SIGHUP, &mut sigact, 0 );
        libc::sigaction(SIGTERM, &mut sigact, 0 );
        libc::sigaction(SIGALRM, &mut sigact, 0 );
        libc::sigaction(SIGCHLD, &mut sigact, 0 );
        libc::sigaction(SIGINT, &mut sigact, 0 );
        sigact.__sigaction_handler.sa_handler = ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 ); /* known umask, create leases and pid files as 0644 */
        libc::sigaction(
            SIGPIPE,
            &mut sigact,
            0 ,
        ); /* Must precede read_opts() */
        libc::umask(0o22 );
    }

    /* ignore SIGPIPE */

    util::rand_init();
    option::read_opts(argc, argv, compile_opts);

    let mut daemon: DnsmasqDaemon = Default::default();

    if cfg!(target_os = "linux") {
        daemon.kernel_version = util::get_linux_kernel_version();
    }

    if (daemon.edns_pktsz) < 512 {
        daemon.edns_pktsz = 512
    }
    /* Min buffer size: we check after adding each record, so there must be
    memory for the largest packet, and the largest record so the
    min for DNS is PACKETSZ+MAXDNAME+RRFIXEDSZ which is < 1000.
    This might be increased is EDNS packet size if greater than the minimum. */
    daemon.packet_buff_sz =
        daemon.edns_pktsz + 1025 + 10;
    // daemon.packet =
    //     safe_malloc(daemon.packet_buff_sz as libc::ABDAY_3size_t) as
    //         *mut libc::c_char;
    daemon.packet = Vec::new();
    // daemon.addrbuff =
    //     safe_malloc(46 as libc::size_t) as *mut libc::c_char;
    daemon.addrbuff = Vec::new();
    if daemon.options[(51).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (51).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        != 0
    {
        // daemon.addrbuff2 =
        //     safe_malloc(46 as libc::size_t) as *mut libc::c_char
        daemon.addrbuff2 = Vec::new();
    }
    if daemon.lease_file.is_null() {
        if !daemon.dhcp.is_null() || !daemon.dhcp6.is_null() {
            daemon.lease_file = String::from("/var/lib/misc/dnsmasq.leases\x00");
        }
    }
    /* Ensure that at least stdin, stdout and stderr (fd 0, 1, 2) exist,
    otherwise file descriptors we create can end up being 0, 1, or 2
    and then get accidentally closed later when we make 0, 1, and 2
    open to /dev/null. Normally we'll be started with 0, 1 and 2 open,
    but it's not guaranteed. By opening /dev/null three times, we
    ensure that we're not using those fds for real stuff. */
    i = 0;
    while i < 3 {
        open(
            b"/dev/null\x00" ,
            0o2,
        );
        i += 1
    }
    /* Close any file descriptors we inherited apart from std{in|out|err} */
    // TODO:
    // close_fds(max_fd, -(1), -(1),
    //           -(1));
    if daemon.options[(45).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (45).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        != 0
    {
        die(
            b"DNSSEC not available: set HAVE_DNSSEC in src/config.h\x00"
                ,
            0 ,
            1,
        );
    }
    if daemon.options[(35).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (35).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        != 0
    {
        die(
            b"conntrack support not available: set HAVE_CONNTRACK in src/config.h\x00"
                ,
            0 ,
            1,
        );
    }
    if daemon.options[(58).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (58).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        != 0
    {
        die(
            b"Ubus not available: set HAVE_UBUS in src/config.h\x00"
                ,
            0 ,
            1,
        );
    }
    if daemon.max_port < daemon.min_port {
        die(
            b"max_port cannot be smaller than min_port\x00"
                ,
            0 ,
            1,
        );
    }
    now = dnsmasq_time();
    if !daemon.auth_zones.is_null() {
        if daemon.authserver.is_null() {
            die(
                b"--auth-server required when an auth zone is defined.\x00"
                    ,
                0 ,
                1,
            );
        }
        /* Create a serial at startup if not configured. */
        if daemon.soa_sn == 0 {
            daemon.soa_sn = now
        }
    }

    /* Note that order matters here, we must call lease_init before
    creating any file descriptors which shouldn't be leaked
    to the lease-script init process. We need to call common_init
    before lease_init to allocate buffers it uses.
    The script subsystem relies on DHCP buffers, hence the last two
    conditions below. */
    if daemon.dhcp_enabled
        || daemon.dhcp6_enabled
        || daemon.relay4_enabled
        || daemon.relay6_enabled
        || daemon.options[40]
        || daemon.options[53]
    {
        dhcp_common_init(&mut daemon);
        if daemon.dhcp_enabled || daemon.dhcp6_enabled {
            lease_init(now);
        }
    }
    if daemon.dhcp_enabled || daemon.relay4_enabled {
        dhcp_init();
        if !daemon.options[21] {
            need_cap_net_raw = 1
        }
        need_cap_net_admin = 1
    }
    if daemon.ra_enabled || daemon.dhcp6_enabled || daemon.relay6_enabled {
        ra_init(now);
        need_cap_net_raw = 1;
        need_cap_net_admin = 1
    }
    if daemon.dhcp6_enabled || daemon.relay6_enabled {
        dhcp6_init();
    }
    if !daemon.ipsets_enabled {
        ipset_init();
        need_cap_net_admin = 1
    }
    netlink_warn = netlink_init(&mut daemon);
    if daemon.options[(13).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (13).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        != 0
        && daemon.options[(39).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>())
                .wrapping_mul(8),
        ) ]
            & (1)
                << (39).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>())
                        .wrapping_mul(8),
                )
            != 0
    {
        die(
            b"cannot set --bind-interfaces and --bind-dynamic\x00"
                ,
            0 ,
            1,
        );
    }
    if enumerate_interfaces(1) == 0 || enumerate_interfaces(0) == 0 {
        die(
            b"failed to find list of interfaces: %s\x00"
                ,
            0 ,
            5,
        );
    }
    if daemon.options[(13).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (13).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        != 0
        || daemon.options[(39).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>())
                .wrapping_mul(8),
        ) ]
            & (1)
                << (39).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>())
                        .wrapping_mul(8),
                )
            != 0
    {
        create_bound_listeners(1);
        if daemon.options[(39).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>())
                .wrapping_mul(8),
        ) ]
            & (1)
                << (39).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>())
                        .wrapping_mul(8),
                )
            == 0
        {
            if_tmp = daemon.if_names;
            while !if_tmp.is_null() {
                if !(*if_tmp).name.is_null() && (*if_tmp).used == 0 {
                    die(
                        b"unknown interface %s\x00"
                            ,
                        (*if_tmp).name,
                        2,
                    );
                }
                if_tmp = (*if_tmp).next
            }
        }
        /* after enumerate_interfaces()  */
        match whichdevice(&mut daemon) {
            Some(x) => bound_device = x,
            None => log::info!("bound_device not found"),
        };

        if daemon.doing_dhcp {
            if !daemon.relay4_enabled && !bound_device.is_empty() {
                bind_to_device(&bound_device, &mut daemon.dhcpfd.unwrap());
                did_bind = true
            }
            if daemon.enable_pxe && !bound_device.is_empty() {
                bind_to_device(&bound_device, &mut daemon.pxefd.unwrap());
                did_bind = true
            }
        }
        if daemon.dhcp6_enabled && !daemon.relay6_enabled && !bound_device.is_empty() {
            bind_to_device(&bound_device, &mut daemon.dhcp6fd);
            did_bind = true
        }
    } else {
        create_wildcard_listeners();
    }
    /* after enumerate_interfaces() */
    if daemon.dhcp6_enabled == true || daemon.doing_relay_6 == true || daemon.ra_enabled == true {
        join_multicast(1);
    }
    /* After netlink_init() and before create_helper() */
    lease_make_duid(now);
    if daemon.port != 0 {
        cache_init();
        blockdata_init(&mut daemon);
        hash_questions_init();
    }
    if (daemon.port != 0 || daemon.dhcp_enabled || daemon.dhcp6_enabled)
        && (daemon.options[(8).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>())
                .wrapping_mul(8),
        ) ]
            & (1)
                << (8).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>())
                        .wrapping_mul(8),
                )
            == 0
            || daemon.use_dynamic_dirs)
    {
        inotify_dnsmasq_init();
    } else {
        daemon.inotifyfd = -(1)
    }
    if !daemon.dump_file.is_empty() {
        dump_init(&mut daemon);
    } else {
        // daemon.dumpfd = -1;
    }
    if daemon.options[(19).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (19).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        != 0
    {
        die(
            b"DBus not available: set HAVE_DBUS in src/config.h\x00"
                ,
            0 ,
            1,
        );
    }
    if daemon.options[(58).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (58).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        != 0
    {
        die(
            b"UBus not available: set HAVE_UBUS in src/config.h\x00"
                ,
            0 ,
            1,
        );
    }
    if daemon.port != 0 {
        pre_allocate_sfds();
    }
    /* Note getpwnam returns static storage */
    if (!daemon.dhcp.is_null() || !daemon.dhcp6.is_null())
        && !daemon.scriptuser.is_null()
        && (!daemon.lease_change_command.is_null() || !daemon.luascript.is_null())
    {
        let mut scr_pw: *mut Passwd = 0 ;
        scr_pw = getpwnam(daemon.scriptuser);
        if !scr_pw.is_null() {
            script_uid = (*scr_pw).pw_uid;
            script_gid = (*scr_pw).pw_gid
        } else {
            baduser = daemon.scriptuser.clone()
        }
    }
    if !daemon.username.is_null() && {
        ent_pw = getpwnam(daemon.username);
        ent_pw.is_null()
    } {
        baduser = daemon.username.clone()
    } else if !daemon.groupname.is_null() && {
        gp = getgrnam(daemon.groupname);
        gp.is_null()
    } {
        baduser = daemon.groupname.clone()
    }
    if !baduser.is_null() {
        // die(
        //     b"unknown user or group: %s\x00" as *const u8 as *const libc::c_char
        //         as *mut libc::c_char,
        //     baduser,
        //     1,
        // );
    }
    /* implement group defaults, "dip" if available, or group associated with uid */
    if daemon.group_set == 0 && gp.is_null() {
        gp = getgrnam(b"dip\x00" );
        if gp.is_null() && !ent_pw.is_null() {
            gp = getgrgid(ent_pw.pw_gid)
        }
        /* for error message */
        if !gp.is_null() {
            daemon.groupname = gp.gr_name.clone()
        }
    }
    /* We keep CAP_NETADMIN (for ARP-injection) and
    CAP_NET_RAW (for icmp) if we're doing dhcp,
    if we have yet to bind ports because of DAD,
    or we're doing it dynamically, we need CAP_NET_BIND_SERVICE. */
    if (is_dad_listeners() != 0
        || daemon.options[(39).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>())
                .wrapping_mul(8),
        ) ]
            & (1)
                << (39).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>())
                        .wrapping_mul(8),
                )
            != 0)
        && (daemon.options[(40).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>())
                .wrapping_mul(8),
        ) ]
            & (1)
                << (40).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>())
                        .wrapping_mul(8),
                )
            != 0
            || daemon.port != 0 && daemon.port <= 1024)
    {
        need_cap_net_bind_service = 1
    }
    /* usptream servers which bind to an interface call SO_BINDTODEVICE
    for each TCP connection, so need CAP_NET_RAW */
    serv = daemon.servers;
    while !serv.is_null() {
        if (*serv).interface[0 ] != 0 {
            need_cap_net_raw = 1
        }
        serv = (*serv).next
    }
    /* If we're doing Dbus or UBus, the above can be set dynamically,
    (as can ports) so always (potentially) needed. */
    /* determine capability API version here, while we can still
    call safe_malloc */
    let mut capsize: i32 = 1; /* for header version 1 */
    let mut fail: &mut String = 0 ;
    // hdr =
    //     safe_malloc(::std::mem::size_of::<__user_cap_header_struct>() as
    //                     libc::c_ulong) as cap_user_header_t;
    // /* find version supported by kernel */
    // memset(hdr as *mut libc::c_void, 0,
    //        ::std::mem::size_of::<__user_cap_header_struct>() as
    //            libc::c_ulong);
    capget(hdr, 0 as cap_user_data_t);
    if hdr.version != 0x19980330 {
        /* if unknown version, use largest supported version (3) */
        if hdr.version != 0x20071026 {
            hdr.version = 0x20071026;
        } /* Get current values, for verification */
        capsize = 2
    }
    data = safe_malloc(
        (::std::mem::size_of::<UserCapData>())
            .wrapping_mul(capsize),
    ) as cap_user_data_t;
    capget(hdr, data);
    if need_cap_net_admin != 0
        && data.permitted & ((1) << 12) == 0
    {
        fail = b"NET_ADMIN\x00"
    } else if need_cap_net_raw != 0
        && data.permitted & ((1) << 13) == 0
    {
        fail = b"NET_RAW\x00"
    } else if need_cap_net_bind_service != 0
        && data.permitted & ((1) << 10) == 0
    {
        fail = b"NET_BIND_SERVICE\x00"
    }
    if !fail.is_null() {
        die(
            b"process is missing required capability %s\x00"
                ,
            fail,
            5,
        );
    }
    /* Now set bitmaps to set caps after daemonising */
    data = Default::default();

    );
    if need_cap_net_admin != 0 {
        data.effective |= ((1) << 12)
    }
    if need_cap_net_raw != 0 {
        data.effective |= ((1) << 13)
    }
    if need_cap_net_bind_service != 0 {
        data.effective |= ((1) << 10)
    }
    data.permitted = data.effective;
    /* Use a pipe to carry signals and other events back to the event loop
    in a race-free manner and another to carry errors to daemon-invoking process */
    safe_pipe(pipefd.as_mut_ptr(), 1);
    piperead = pipefd[0 ];
    ::std::ptr::write_volatile(
        &mut pipewrite,
        pipefd[1 ],
    );
    /* prime the pipe to load stuff first time. */
    send_event(
        pipewrite,
        21,
        0,
        None,
    );
    err_pipe[1 ] = -(1);
    if daemon.options[(6).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (6).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        == 0
    {
        /* The following code "daemonizes" the process.
        See Stevens section 12.4 */
        if chdir(b"/\x00" ) != 0 {
            die(
                b"cannot chdir to filesystem root: %s\x00"
                    ,
                0 ,
                5,
            );
        }
        if daemon.options[(16).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>())
                .wrapping_mul(8),
        ) ]
            & (1)
                << (16).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>())
                        .wrapping_mul(8),
                )
            == 0
        {
            let mut pid_0: pid_t = 0;
            /* pipe to carry errors back to original process.
            When startup is complete we close this and the process terminates. */
            safe_pipe(err_pipe.as_mut_ptr(), 0);
            pid_0 = fork();
            if pid_0 == -(1) {
                /* fd == -1 since we've not forked, never returns. */
                send_event(
                    -(1),
                    18,
                    *__errno_location(),
                    None,
                );
            }
            if pid_0 != 0 {
                let mut ev: EventDesc = EventDesc {
                    event: 0,
                    data: 0,
                    msg_sz: 0,
                };
                let mut msg: &mut String = 0 ;
                /* close our copy of write-end */
                close(err_pipe[1 ]);
                /* check for errors after the fork */
                if read_event(err_pipe[0 ], &mut ev, &mut msg) != 0 {
                    fatal_event(&mut ev, msg);
                }
                _exit(0);
            }
            close(err_pipe[0 ]);
            /* NO calls to die() from here on. */
            setsid();
            pid_0 = fork();
            if pid_0 == -(1) {
                send_event(
                    err_pipe[1 ],
                    18,
                    *__errno_location(),
                    None,
                );
            }
            if pid_0 != 0 {
                _exit(0);
            }
        }
        /* write pidfile _after_ forking ! */
        if !daemon.runfile.is_null() {
            let mut fd: i32 = 0;
            let mut err: i32 = 0;
            sprintf(
                daemon.namebuff,
                b"%d\n\x00" ,
                getpid(),
            );
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
            unlink(daemon.runfile);
            fd = open(
                daemon.runfile.clone(),
                0o1
                    | 0o100
                    | 0o1000
                    | 0o200,
                0o200
                    | 0o400
                    | 0o400 >> 3
                    | 0o400 >> 3 >> 3,
            );
            if fd == -(1) {
                /* only complain if started as root */
                if getuid() == 0 {
                    err = 1
                }
            } else {
                /* We're still running as root here. Change the ownership of the PID file
                to the user we will be running as. Note that this is not to allow
                us to delete the file, since that depends on the permissions
                of the directory containing the file. That directory will
                need to by owned by the dnsmasq user, and the ownership of the
                file has to match, to keep systemd >273 happy. */
                if getuid() == 0
                    && !ent_pw.is_null()
                    && ent_pw.pw_uid != 0
                    && fchown(fd, ent_pw.pw_uid, ent_pw.pw_gid) == -(1)
                {
                    chown_warn = *__errno_location()
                }
                if read_write(
                    fd,
                    daemon.namebuff.clone(),
                    strlen(daemon.namebuff.clone()),
                    0,
                ) == 0
                {
                    err = 1
                } else if close(fd) == -(1) {
                    err = 1
                }
            }
            if err != 0 {
                send_event(
                    err_pipe[1 ],
                    13,
                    *__errno_location(),
                    Some(&mut daemon.runfile),
                );
                _exit(0);
            }
        }
    }
    log_err = log_start(ent_pw, err_pipe[1 ]);
    if daemon.options[(6).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (6).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        == 0
    {
        /* open  stdout etc to /dev/null */
        let mut nullfd: i32 = open(
            b"/dev/null\x00" ,
            0o2,
        );
        if nullfd != -(1) {
            dup2(nullfd, 1);
            dup2(nullfd, 2);
            dup2(nullfd, 0);
            close(nullfd);
        }
    }
    /* if we are to run scripts, we need to fork a helper before dropping root. */
    daemon.helperfd;
    if (!daemon.dhcp.is_null()
        || !daemon.dhcp6.is_null()
        || daemon.options[40]
            != 0 || daemon.options[53] != 0)
        && (!daemon.lease_change_command.is_null() || !daemon.luascript.is_null())
    {
        daemon.helperfd = create_helper( pipewrite, err_pipe[1], script_uid, script_gid, max_fd)
    }
    if daemon.options[6] == 0 && getuid() == 0
    {
        let mut bad_capabilities: i32 = 0;
        let mut dummy: gid_t = 0;
        /* remove all supplementary groups */
        if !gp.is_null() && (setgroups(0 , &mut dummy) == -(1) || setgid((*gp).gr_gid) == -(1))
        {
            send_event( err_pipe[1 ], 15, *__errno_location(), Some(&mut daemon.groupname));
            _exit(0);
        }
        if !ent_pw.is_null() && ent_pw.pw_uid != 0 {
            /* Need to be able to drop root. */
            data.effective |= ((1) << 7);
            data.permitted |= ((1) << 7);
            /* Tell kernel to not clear capabilities when dropping root */
            if capset(hdr, data) == -(1) || prctl(8, 1, 0, 0, 0) == -1
            {
                bad_capabilities = *__errno_location()
            }
            if bad_capabilities != 0 {
                send_event( err_pipe[1 ], 12, bad_capabilities, None);
                _exit(0);
            }
            /* finally drop root */
            if setuid(ent_pw.pw_uid) == -(1) {
                send_event(err_pipe[1 ], 11, *__errno_location(), Some(&mut daemon.username));
                _exit(0);
            }
            data.effective &= !((1) << 7);
            data.permitted &= !((1) << 7);
            /* lose the setuid capability */
            if capset(hdr, data) == -(1) {
                send_event(err_pipe[1 ], 12, *__errno_location(), None);
                _exit(0);
            }
        }
    }
    free(hdr);
    free(data);
    if daemon.options[6]
    {
        prctl(
            4,
            1,
            0,
            0,
            0,
        );
    }
    if daemon.options[40] != 0
    {
        let mut dir: *mut DIR = 0 ;
        let mut p: *mut TftpPrefix = 0 ;
        if !daemon.tftp_prefix.is_null() {
            dir = opendir(daemon.tftp_prefix);
            if dir.is_null() {
                tftp_prefix_missing = 1;
                if daemon.options[52] == false
                {
                    send_event( err_pipe[1 ], 20, *__errno_location(), Some(&mut daemon.tftp_prefix));
                    _exit(0);
                }
            } else {
                closedir(dir);
            }
        }
        p = daemon.if_prefix;
        while !p.is_null() {
            p.missing = 0;
            dir = opendir(p.prefix);
            if dir.is_null() {
                p.missing = 1;
                if daemon.options[52] == false
                {
                    send_event(
                        err_pipe[1 ],
                        20,
                        *__errno_location(),
                        p.prefix,
                    );
                    _exit(0);
                }
            } else {
                closedir(dir);
            }
            p = p.next
        }
    }
    if daemon.port == 0 {
        log::info!("started, version {} DNS disabled", "2.84rc2");
    } else {
        if daemon.cachesize != 0 {
            my_syslog(
                6,
                b"started, version %s cachesize %d\x00" ,
                b"2.84rc2\x00" ,
                daemon.cachesize,
            );
            if daemon.cachesize > 10000 {
                my_syslog(4,
                          b"cache size greater than 10000 may cause performance issues, and is unlikely to be useful.\x00"
                              );
            }
        } else {
            my_syslog(
                6,
                b"started, version %s cache disabled\x00" ,
                b"2.84rc2\x00" ,
            );
        }
        if daemon.options[(49).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>())
                .wrapping_mul(8),
        ) ]
            & (1)
                << (49).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>())
                        .wrapping_mul(8),
                )
            != 0
        {
            my_syslog(
                6,
                b"DNS service limited to local subnets\x00" ,
            );
        }
    }
    my_syslog(
        6,
        b"compile time options: %s\x00" ,
        compile_opts,
    );
    if chown_warn != 0 {
        my_syslog(
            4,
            b"chown of PID file %s failed: %s\x00" ,
            daemon.runfile,
            strerror(chown_warn),
        );
    }
    if log_err != 0 {
        my_syslog(
            4,
            b"warning: failed to change owner of %s: %s\x00" ,
            daemon.log_file,
            strerror(log_err),
        );
    }
    if bind_fallback != 0 {
        my_syslog(
            4,
            b"setting --bind-interfaces option because of OS limitations\x00"
               ,
        );
    }
    if daemon.options[(13).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (13).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        != 0
    {
        warn_bound_listeners();
    } else if daemon.options[(39).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (39).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        == 0
    {
        warn_wild_labels();
    }
    warn_int_names();
    if daemon.options[(13).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (13).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        == 0
    {
        if_tmp = daemon.if_names;
        while !if_tmp.is_null() {
            if !(*if_tmp).name.is_null() && (*if_tmp).used == 0 {
                my_syslog(
                    4,
                    b"warning: interface %s does not currently exist\x00"
                       ,
                    (*if_tmp).name,
                );
            }
            if_tmp = (*if_tmp).next
        }
    }
    if daemon.port != 0
        && daemon.options[(8).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>())
                .wrapping_mul(8),
        ) ]
            & (1)
                << (8).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>())
                        .wrapping_mul(8),
                )
            != 0
    {
        if !daemon.resolv_files.is_null() && (*daemon.resolv_files).is_default == 0 {
            my_syslog(
                4,
                b"warning: ignoring resolv-file flag because no-resolv is set\x00"
                   ,
            );
        }
        daemon.resolv_files = 0 ;
        if daemon.servers.is_null() {
            my_syslog(
                4,
                b"warning: no upstream servers configured\x00" ,
            );
        }
    }
    if daemon.max_logs != 0 {
        my_syslog(
            6,
            b"asynchronous logging enabled, queue limit is %d messages\x00"
               ,
            daemon.max_logs,
        );
    }
    context = daemon.dhcp;
    while !context.is_null() {
        dhcp_context_to_string(2, context);
        context = (*context).next
    }
    relay = daemon.relay4;
    while !relay.is_null() {
        log_relay(2, relay);
        relay = (*relay).next
    }
    context = daemon.dhcp6;
    while !context.is_null() {
        dhcp_context_to_string(10, context);
        context = (*context).next
    }
    relay = daemon.relay6;
    while !relay.is_null() {
        log_relay(10, relay);
        relay = (*relay).next
    }
    if daemon.dhcp6_enabled != 0 || daemon.ra_enabled != 0 {
        dhcp_construct_contexts(now);
    }
    if daemon.options[(37).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (37).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        != 0
    {
        my_syslog(
            (3) << 3 | 6,
            b"IPv6 router advertisement enabled\x00" ,
        );
    }
    if did_bind != 0 {
        my_syslog(
            (3) << 3 | 6,
            b"DHCP, sockets bound exclusively to interface %s\x00"
               ,
            bound_device,
        );
    }
    if !netlink_warn.is_null() {
        my_syslog(4, netlink_warn);
    }
    /* after dhcp_construct_contexts */
    if !daemon.dhcp.is_null() || daemon.dhcp6_enabled != 0 {
        lease_find_interfaces(now);
    }
    if daemon.options[(40).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (40).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        != 0
    {
        let mut p_0: *mut TftpPrefix = 0 ;
        my_syslog(
            (1) << 3 | 6,
            b"TFTP %s%s %s %s\x00" ,
            if !daemon.tftp_prefix.is_null() {
                b"root is \x00"
            } else {
                b"enabled\x00"
            },
            if !daemon.tftp_prefix.is_null() {
                daemon.tftp_prefix
            } else {
                b"\x00"
            },
            if daemon.options[(26).wrapping_div(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            ) ]
                & (1)
                    << (26).wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>())
                            .wrapping_mul(8),
                    )
                != 0
            {
                b"secure mode\x00"
            } else {
                b"\x00"
            },
            if daemon.options[(60).wrapping_div(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            ) ]
                & (1)
                    << (60).wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>())
                            .wrapping_mul(8),
                    )
                != 0
            {
                b"single port mode\x00"
            } else {
                b"\x00"
            },
        );
        if tftp_prefix_missing != 0 {
            my_syslog(
                (1) << 3 | 4,
                b"warning: %s inaccessible\x00" ,
                daemon.tftp_prefix,
            );
        }
        p_0 = daemon.if_prefix;
        while !p_0.is_null() {
            if (*p_0).missing != 0 {
                my_syslog(
                    (1) << 3 | 4,
                    b"warning: TFTP directory %s inaccessible\x00"
                       ,
                    (*p_0).prefix,
                );
            }
            p_0 = (*p_0).next
        }
        /* This is a guess, it assumes that for small limits,
        disjoint files might be served, but for large limits,
        a single file will be sent to may clients (the file only needs
        one fd). */
        max_fd -= 30; /* use other than TFTP */
        if max_fd < 0 {
            max_fd = 5
        } else if max_fd < 100
            && daemon.options[(60).wrapping_div(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            ) ]
                & (1)
                    << (60).wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>())
                            .wrapping_mul(8),
                    )
                == 0
        {
            max_fd = max_fd / 2
        } else {
            max_fd = max_fd - 20
        }
        /* if we have to use a limited range of ports,
        that will limit the number of transfers */
        if daemon.start_tftp_port != 0
            && ((daemon.end_tftp_port - daemon.start_tftp_port + 1))
                < max_fd
        {
            max_fd =
                (daemon.end_tftp_port - daemon.start_tftp_port + 1)
        }
        if daemon.tftp_max > max_fd {
            daemon.tftp_max = max_fd;
            my_syslog(
                (1) << 3 | 4,
                b"restricting maximum simultaneous TFTP transfers to %d\x00"
                   ,
                daemon.tftp_max,
            );
        }
    }
    /* finished start-up - release original process */
    if err_pipe[1 ] != -(1) {
        close(err_pipe[1 ]);
    }
    if daemon.port != 0 {
        check_servers();
    }
    ::std::ptr::write_volatile(&mut pid , getpid());
    daemon.pipe_to_parent = -(1);
    i = 0;
    while i < 20 {
        daemon.tcp_pipes[i ] = -(1);
        i += 1
    }
    /* Using inotify, have to select a resolv file at startup */
    poll_resolv(1, 0, now);
    loop {
        let mut t: i32 = 0;
        let mut timeout: i32 = -(1);
        poll_reset();
        /* if we are out of resources, find how long we have to wait
        for some to come free, we'll loop around then and restart
        listening for queries */
        t = set_dns_listeners(now);
        if t != 0 {
            timeout = t * 1000
        }
        /* Whilst polling for the dbus, or doing a tftp transfer, wake every quarter second */
        if !daemon.tftp_trans.is_null()
            || daemon.options[(19).wrapping_div(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            ) ]
                & (1)
                    << (19).wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>())
                            .wrapping_mul(8),
                    )
                != 0
                && daemon.dbus.is_null()
        {
            timeout = 250
        } else if is_dad_listeners() != 0 {
            timeout = 1000
        }
        if !daemon.dhcp.is_null() || !daemon.relay4.is_null() {
            poll_listen(daemon.dhcpfd, 0x1 );
            if daemon.pxefd != -(1) {
                poll_listen(daemon.pxefd, 0x1 );
            }
        }
        if daemon.dhcp6_enabled != 0 || !daemon.relay6.is_null() {
            poll_listen(daemon.dhcp6fd, 0x1 );
        }
        if daemon.ra_enabled != 0 {
            poll_listen(daemon.icmp6fd, 0x1 );
        }
        if daemon.inotifyfd != -(1) {
            poll_listen(daemon.inotifyfd, 0x1 );
        }
        poll_listen(daemon.netlinkfd, 0x1 );
        poll_listen(piperead, 0x1 );
        while helper_buf_empty() != 0 && do_script_run(now) != 0 {}
        /* Wake every second whilst waiting for DAD to complete */
        /* Refresh cache */
        if daemon.options[(53).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>())
                .wrapping_mul(8),
        ) ]
            & (1)
                << (53).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>())
                        .wrapping_mul(8),
                )
            != 0
        {
            find_mac(
                0 ,
                0,
                0,
                now,
            );
        }
        while helper_buf_empty() != 0 && do_arp_script_run() != 0 {}
        while helper_buf_empty() != 0 && do_tftp_script_run() != 0 {}
        if helper_buf_empty() == 0 {
            poll_listen(daemon.helperfd, 0x4 );
        }
        /* must do this just before do_poll(), when we know no
        more calls to my_syslog() can occur */
        set_log_writer();
        if do_poll(timeout) < 0 {
            continue;
        }
        now = dnsmasq_time();
        check_log_writer(0);
        /* prime. */
        enumerate_interfaces(1);
        /* Check the interfaces to see if any have exited DAD state
        and if so, bind the address. */
        if is_dad_listeners() != 0 {
            enumerate_interfaces(0);
            /* NB, is_dad_listeners() == 1 --> we're binding interfaces */
            create_bound_listeners(0);
            warn_bound_listeners();
        }
        if poll_check(daemon.netlinkfd, 0x1 ) != 0 {
            netlink_multicast();
        }
        if daemon.inotifyfd != -(1)
            && poll_check(daemon.inotifyfd, 0x1 ) != 0
            && inotify_check(now) != 0
        {
            if daemon.port != 0
                && daemon.options[(5).wrapping_div(
                    (::std::mem::size_of::<libc::c_uint>())
                        .wrapping_mul(8),
                ) ]
                    & (1)
                        << (5).wrapping_rem(
                            (::std::mem::size_of::<libc::c_uint>())
                                .wrapping_mul(8),
                        )
                    == 0
            {
                poll_resolv(1, 1, now);
            }
        }
        if poll_check(piperead, 0x1 ) != 0 {
            async_event(piperead, now);
        }
        check_dns_listeners(now);
        check_tftp_listeners(now);
        if !daemon.dhcp.is_null() || !daemon.relay4.is_null() {
            if poll_check(daemon.dhcpfd, 0x1 ) != 0 {
                dhcp_packet(now, 0);
            }
            if daemon.pxefd != -(1)
                && poll_check(daemon.pxefd, 0x1 ) != 0
            {
                dhcp_packet(now, 1);
            }
        }
        if (daemon.dhcp6_enabled != 0 || !daemon.relay6.is_null())
            && poll_check(daemon.dhcp6fd, 0x1 ) != 0
        {
            dhcp6_packet(now);
        }
        if daemon.ra_enabled != 0
            && poll_check(daemon.icmp6fd, 0x1 ) != 0
        {
            icmp6_packet(now);
        }
        if daemon.helperfd != -(1)
            && poll_check(daemon.helperfd, 0x4 ) != 0
        {
            helper_write();
        }
    }
}
fn sig_handler(mut sig: i32) {
    if pid == 0 {
        /* ignore anything other than TERM during startup
        and in helper proc. (helper ignore TERM too) */
        if sig == 15 || sig == 2 {
            exit(5);
        }
    } else if pid != getpid() {
        /* alarm is used to kill TCP children after a fixed time. */
        if sig == 14 {
            _exit(0);
        }
    } else {
        /* master process */
        let mut event: i32 = 0;
        let mut errsave: i32 = *__errno_location();
        if sig == 1 {
            event = 1
        } else if sig == 17 {
            event = 5
        } else if sig == 14 {
            event = 3
        } else if sig == 15 {
            event = 4
        } else if sig == 10 {
            event = 2
        } else if sig == 12 {
            event = 6
        } else if sig == 2 {
            /* Handle SIGINT normally in debug mode, so
            ctrl-c continues to operate. */
            if daemon.options[(6).wrapping_div(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            ) ]
                & (1)
                    << (6).wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>())
                            .wrapping_mul(8),
                    )
                != 0
            {
                exit(5);
            } else {
                event = 26
            }
        } else {
            return;
        }
        send_event(pipewrite, event, 0, 0 );
        *__errno_location() = errsave
    };
}
/* now == 0 -> queue immediate callback */

pub fn send_alarm(mut event: time::Instant, mut now: time::Instant) {
    if now == 0 || event != 0 {
        /* alarm(0) or alarm(-ve) doesn't do what we want.... */
        if now == 0 || difftime(event, now) <= 0.0f64 {
            send_event(
                pipewrite,
                3,
                0,
                None,
            );
        } else {
            alarm(difftime(event, now));
        }
    };
}

pub fn queue_event(mut event: i32) {
    send_event(pipewrite, event, 0, None);
}


pub fn send_event(
    mut fd: i32,
    mut event: i32,
    mut data: i32,
    mut msg: Option<&mut String>)
{
    let mut ev: EventDesc = EventDesc {
        event: 0,
        data: 0,
        msg_sz: 0,
    };
    let mut iov: [iovec; 2] = [iovec {
        iov_base: 0,
        iov_len: 0,
    }; 2];
    ev.event = event;
    ev.data = data;
    ev.msg_sz = if !msg.is_null() {
        strlen(msg)
    } else {
        0
    };
    iov[0 ].iov_base = &mut ev ;
    iov[0 ].iov_len = ::std::mem::size_of::<EventDesc>();
    iov[1 ].iov_base = msg;
    iov[1 ].iov_len = ev.msg_sz ;
    /* error pipe, debug mode. */
    if fd == -(1) {
        fatal_event(&mut ev, msg);
    } else {
        /* pipe is non-blocking and struct event_desc is smaller than
        PIPE_BUF, so this either fails or writes everything */
        while writev(
            fd,
            iov.as_mut_ptr(),
            (if !msg.is_null() {
                2
            } else {
                1
            }),
        ) == -(1)
            && *__errno_location() == 4
        {}
    };
}
/* NOTE: the memory used to return msg is leaked: use msgs in events only
to describe fatal errors. */
fn read_event(
    mut fd: i32,
    mut evp: *mut EventDesc,
    mut msg: String,
) -> i32 {
    let mut buf: &mut String = 0 ;
    if read_write(
        fd,
        evp,
        ::std::mem::size_of::<EventDesc>(),
        1,
    ) == 0
    {
        return 0;
    }
    *msg = 0 ;
    if (*evp).msg_sz != 0
        && {
            buf = malloc(((*evp).msg_sz + 1)) ;
            !buf.is_null()
        }
        && read_write(
            fd,
            buf,
            (*evp).msg_sz,
            1,
        ) != 0
    {
        *buf.offset((*evp).msg_sz) = 0;
        *msg = buf
    }
    return 1;
}
fn fatal_event(mut ev: *mut EventDesc, mut msg: &mut String) {
    *__errno_location() = (*ev).data;
    match (*ev).event {
        16 => {
            exit(0);
        }
        18 => {
            die(
                b"cannot fork into background: %s\x00"
                    ,
                0 ,
                5,
            );
        }
        10 => {
            /* fall through */
            die(
                b"failed to create helper: %s\x00"
                    ,
                0 ,
                5,
            );
        }
        12 => {
            /* fall through */
            die(
                b"setting capabilities failed: %s\x00"
                    ,
                0 ,
                5,
            );
        }
        11 => {
            /* fall through */
            die(
                b"failed to change user-id to %s: %s\x00"
                    ,
                msg,
                5,
            );
        }
        15 => {
            /* fall through */
            die(
                b"failed to change group-id to %s: %s\x00"
                    ,
                msg,
                5,
            );
        }
        13 => {
            /* fall through */
            die(
                b"failed to open pidfile %s: %s\x00"
                    ,
                msg,
                3,
            );
        }
        17 => {
            /* fall through */
            die(
                b"cannot open log %s: %s\x00"
                    ,
                msg,
                3,
            );
        }
        19 => {
            /* fall through */
            die(
                b"failed to load Lua script: %s\x00"
                    ,
                msg,
                5,
            );
        }
        20 => {
            /* fall through */
            die(
                b"TFTP directory %s inaccessible: %s\x00"
                    ,
                msg,
                3,
            );
        }
        24 => {
            /* fall through */
            die(
                b"cannot create timestamp file %s: %s\x00"
                    ,
                msg,
                1,
            );
        }
        _ => {}
    };
}
fn async_event(mut pipe_0: i32, mut now: time::Instant) {
    let mut p: pid_t = 0;
    let mut ev: EventDesc = EventDesc {
        event: 0,
        data: 0,
        msg_sz: 0,
    };
    let mut i: i32 = 0;
    let mut check: i32 = 0;
    let mut msg: &mut String = 0 ;
    /* NOTE: the memory used to return msg is leaked: use msgs in events only
    to describe fatal errors. */
    if read_event(pipe_0, &mut ev, &mut msg) != 0 {
        let mut current_block_60: u64; /* Bump zone serial, as it may have changed. */
        match ev.event {
            1 => {
                daemon.soa_sn = daemon.soa_sn.wrapping_add(1);
                current_block_60 = 11195962526119371365;
            }
            21 => {
                current_block_60 = 11195962526119371365;
            }
            2 => {
                if daemon.port != 0 {
                    dump_cache(now);
                }
                current_block_60 = 6367734732029634840;
            }
            3 => {
                if !daemon.dhcp.is_null() || daemon.doing_dhcp6 != 0 {
                    lease_prune(0, now);
                    lease_update_file(now);
                } else if daemon.doing_ra != 0 {
                    /* Not doing DHCP, so no lease system, manage alarms for ra only */
                    send_alarm(periodic_ra(now), now);
                }
                current_block_60 = 6367734732029634840;
            }
            5 => {
                loop
                /* See Stevens 5.10 */
                {
                    p = waitpid(-(1), 0, 1);
                    if !(p != 0) {
                        break;
                    }
                    if p == -(1) {
                        if *__errno_location() != 4 {
                            break;
                        }
                    } else {
                        i = 0;
                        while i < 20 {
                            if daemon.tcp_pids[i ] == p {
                                daemon.tcp_pids[i ] = 0
                            }
                            i += 1
                        }
                    }
                }
                current_block_60 = 6367734732029634840;
            }
            8 => {
                my_syslog(
                    4,
                    b"script process killed by signal %d\x00" ,
                    ev.data,
                );
                current_block_60 = 6367734732029634840;
            }
            7 => {
                my_syslog(
                    4,
                    b"script process exited with status %d\x00" ,
                    ev.data,
                );
                current_block_60 = 6367734732029634840;
            }
            9 => {
                my_syslog(
                    3,
                    b"failed to execute %s: %s\x00" ,
                    daemon.lease_change_command,
                    strerror(ev.data),
                );
                current_block_60 = 6367734732029634840;
            }
            25 => {
                my_syslog(
                    (2) << 3 | 7,
                    b"%s\x00" ,
                    if !msg.is_null() {
                        msg
                    } else {
                        b"\x00"
                    },
                );
                free(msg);
                msg = 0 ;
                current_block_60 = 6367734732029634840;
            }
            11 | 16 | 19 => {
                /* necessary for fatal errors in helper */
                fatal_event(&mut ev, msg);
                current_block_60 = 6367734732029634840;
            }
            6 => {
                /* Note: this may leave TCP-handling processes with the old file still open.
                Since any such process will die in CHILD_LIFETIME or probably much sooner,
                we leave them logging to the old file. */
                if !daemon.log_file.is_null() {
                    log_reopen(daemon.log_file);
                }
                current_block_60 = 6367734732029634840;
            }
            22 => {
                newaddress(now);
                current_block_60 = 6367734732029634840;
            }
            23 => {
                resend_query();
                /* Force re-reading resolv file right now, for luck. */
                poll_resolv(0, 1, now);
                current_block_60 = 6367734732029634840;
            }
            4 => {
                /* Knock all our children on the head. */
                i = 0;
                while i < 20 {
                    if daemon.tcp_pids[i ] != 0 {
                        kill(daemon.tcp_pids[i ], 14);
                    }
                    i += 1
                }
                /* handle pending lease transitions */
                if daemon.helperfd != -(1) {
                    /* block in writes until all done */
                    i = fcntl(daemon.helperfd, 3);
                    if i != -(1) {
                        fcntl(
                            daemon.helperfd,
                            4,
                            i & !(0o4000),
                        );
                    }
                    loop {
                        helper_write();
                        if !(helper_buf_empty() == 0 || do_script_run(now) != 0) {
                            break;
                        }
                    }
                    close(daemon.helperfd);
                }
                if !daemon.lease_stream.is_null() {
                    fclose(daemon.lease_stream);
                }
                if !daemon.runfile.is_null() {
                    unlink(daemon.runfile);
                }
                if daemon.dumpfd != -(1) {
                    close(daemon.dumpfd);
                }
                my_syslog(
                    6,
                    b"exiting on receipt of SIGTERM\x00" ,
                );
                flush_log();
                exit(0);
            }
            26 | _ => {
                current_block_60 = 6367734732029634840;
            }
        }
        match current_block_60 {
            11195962526119371365 =>
            /* fall through */
            {
                clear_cache_and_reload(now);
                if daemon.port != 0 {
                    if !daemon.resolv_files.is_null()
                        && daemon.options[(5).wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>())
                                .wrapping_mul(8),
                        ) ]
                            & (1)
                                << (5).wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>())
                                        .wrapping_mul(8),
                                )
                            != 0
                    {
                        reload_servers((*daemon.resolv_files).name);
                        check = 1
                    }
                    if !daemon.servers_file.is_null() {
                        read_servers_file();
                        check = 1
                    }
                    if check != 0 {
                        check_servers();
                    }
                }
                rerun_scripts();
            }
            _ => {}
        }
    };
}
fn poll_resolv(
    mut force: i32,
    mut do_reload: i32,
    mut now: time::Instant,
) {
    let mut res: *mut Resolvc = 0 ;
    let mut latest: *mut Resolvc = 0 ;
    let mut statbuf: stat = stat {
        st_dev: 0,
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
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut last_change: time::Instant = 0;
    /* There may be more than one possible file.
    Go through and find the one which changed _last_.
    Warn of any which can't be read. */
    if daemon.port == 0
        || daemon.options[(5).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>())
                .wrapping_mul(8),
        ) ]
            & (1)
                << (5).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>())
                        .wrapping_mul(8),
                )
            != 0
    {
        return;
    }
    latest = 0 ;
    res = daemon.resolv_files;
    while !res.is_null() {
        if stat((*res).name, &mut statbuf) == -(1) {
            if force != 0 {
                (*res).mtime = 0
            } else {
                if (*res).logged == 0 {
                    my_syslog(
                        4,
                        b"failed to access %s: %s\x00" ,
                        (*res).name,
                        strerror(*__errno_location()),
                    );
                }
                (*res).logged = 1;
                if (*res).mtime != 0 {
                    /* existing file evaporated, force selection of the latest
                    file even if its mtime hasn't changed since we last looked */
                    poll_resolv(1, do_reload, now);
                    return;
                }
            }
        } else {
            (*res).logged = 0;
            if force != 0 || statbuf.st_mtim.tv_sec != (*res).mtime {
                (*res).mtime = statbuf.st_mtim.tv_sec;
                if difftime(statbuf.st_mtim.tv_sec, last_change) > 0.0f64 {
                    last_change = statbuf.st_mtim.tv_sec;
                    latest = res
                }
            }
        }
        res = (*res).next
    }
    if !latest.is_null() {
        static mut warned: i32 = 0;
        if reload_servers((*latest).name) != 0 {
            my_syslog(
                6,
                b"reading %s\x00" ,
                (*latest).name,
            );
            warned = 0;
            check_servers();
            if daemon.options[(24).wrapping_div(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            ) ]
                & (1)
                    << (24).wrapping_rem(
                        (::std::mem::size_of::<libc::c_uint>())
                            .wrapping_mul(8),
                    )
                != 0
                && do_reload != 0
            {
                clear_cache_and_reload(now);
            }
        } else {
            (*latest).mtime = 0;
            if warned == 0 {
                my_syslog(
                    4,
                    b"no servers found in %s, will retry\x00" ,
                    (*latest).name,
                );
                warned = 1
            }
        }
    };
}

pub fn clear_cache_and_reload(mut now: time::Instant) {
    if daemon.port != 0 {
        cache_reload();
    }
    if !daemon.dhcp.is_null() || daemon.doing_dhcp6 != 0 {
        if daemon.options[(14).wrapping_div(
            (::std::mem::size_of::<libc::c_uint>())
                .wrapping_mul(8),
        ) ]
            & (1)
                << (14).wrapping_rem(
                    (::std::mem::size_of::<libc::c_uint>())
                        .wrapping_mul(8),
                )
            != 0
        {
            dhcp_read_ethers();
        }
        reread_dhcp();
        dhcp_update_configs(daemon.dhcp_conf);
        lease_update_from_configs();
        lease_update_file(now);
        lease_update_dns(1);
    } else if daemon.doing_ra != 0 {
        /* Not doing DHCP, so no lease system, manage
        alarms for ra only */
        send_alarm(periodic_ra(now), now);
    };
}
fn set_dns_listeners(mut now: time::Instant) -> i32 {
    let mut serverfdp:ServerFd = 0Fd;
    let mut listener: Listener = 0 ;
    let mut wait: i32 = 0;
    let mut i: i32 = 0;
    let mut tftp: i32 = 0;
    let mut transfer: *mut TftpTransfer = 0 ;
    if daemon.options[(60).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (60).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        == 0
    {
        transfer = daemon.tftp_trans;
        while !transfer.is_null() {
            tftp += 1;
            poll_listen((*transfer).sockfd, 0x1 );
            transfer = (*transfer).next
        }
    }
    /* will we be able to get memory? */
    if daemon.port != 0 {
        get_new_frec(now, &mut wait, 0 );
    }
    serverfdp = daemon.sfds;
    while !serverfdp.is_null() {
        poll_listen((*serverfdp).fd, 0x1 );
        serverfdp = (*serverfdp).next
    }
    if daemon.port != 0 && daemon.osport == 0 {
        i = 0;
        while i < 64 {
            if daemon.randomsocks[i ].refcount != 0 {
                poll_listen(
                    daemon.randomsocks[i ].fd,
                    0x1 ,
                );
            }
            i += 1
        }
    }
    listener = daemon.listeners;
    while !listener.is_null() {
        /* only listen for queries if we have resources */
        if (*listener).fd != -(1) && wait == 0 {
            poll_listen((*listener).fd, 0x1 );
        }
        /* death of a child goes through the select loop, so
        we don't need to explicitly arrange to wake up here */
        if (*listener).tcpfd != -(1) {
            i = 0;
            while i < 20 {
                if daemon.tcp_pids[i ] == 0
                    && daemon.tcp_pipes[i ] == -(1)
                {
                    poll_listen((*listener).tcpfd, 0x1 );
                    break;
                } else {
                    i += 1
                }
            }
        }
        /* tftp == 0 in single-port mode. */
        if tftp <= daemon.tftp_max && (*listener).tftpfd != -(1) {
            poll_listen((*listener).tftpfd, 0x1 );
        }
        listener = (*listener).next
    }
    if daemon.options[(6).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (6).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        == 0
    {
        i = 0;
        while i < 20 {
            if daemon.tcp_pipes[i ] != -(1) {
                poll_listen(
                    daemon.tcp_pipes[i ],
                    0x1 ,
                );
            }
            i += 1
        }
    }
    return wait;
}
fn check_dns_listeners(mut now: time::Instant) {
    let mut serverfdp:ServerFd = 0Fd;
    let mut listener: Listener = 0 ;
    let mut i: i32 = 0;
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    serverfdp = daemon.sfds;
    while !serverfdp.is_null() {
        if poll_check((*serverfdp).fd, 0x1 ) != 0 {
            reply_query(
                (*serverfdp).fd,
                (*serverfdp).source_addr.sa.sa_family,
                now,
            );
        }
        serverfdp = (*serverfdp).next
    }
    if daemon.port != 0 && daemon.osport == 0 {
        i = 0;
        while i < 64 {
            if daemon.randomsocks[i ].refcount != 0
                && poll_check(
                    daemon.randomsocks[i ].fd,
                    0x1 ,
                ) != 0
            {
                reply_query(
                    daemon.randomsocks[i ].fd,
                    daemon.randomsocks[i ].family,
                    now,
                );
            }
            i += 1
        }
    }
    /* Races. The child process can die before we read all of the data from the
    pipe, or vice versa. Therefore send tcp_pids to zero when we wait() the
    process, and tcp_pipes to -1 and close the FD when we read the last
    of the data - indicated by cache_recv_insert returning zero.
    The order of these events is indeterminate, and both are needed
    to free the process slot. Once the child process has gone, poll()
    returns POLLHUP, not POLLIN, so have to check for both here. */
    if daemon.options[(6).wrapping_div(
        (::std::mem::size_of::<libc::c_uint>())
            .wrapping_mul(8),
    ) ]
        & (1)
            << (6).wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>())
                    .wrapping_mul(8),
            )
        == 0
    {
        i = 0;
        while i < 20 {
            if daemon.tcp_pipes[i ] != -(1)
                && poll_check(
                    daemon.tcp_pipes[i ],
                    (0x1 | 0x10) ,
                ) != 0
                && cache_recv_insert(now, daemon.tcp_pipes[i ]) == 0
            {
                close(daemon.tcp_pipes[i ]);
                daemon.tcp_pipes[i ] = -(1)
            }
            i += 1
        }
    }
    listener = daemon.listeners;
    while !listener.is_null() {
        if (*listener).fd != -(1)
            && poll_check((*listener).fd, 0x1 ) != 0
        {
            receive_query(listener, now);
        }
        if (*listener).tftpfd != -(1)
            && poll_check((*listener).tftpfd, 0x1 ) != 0
        {
            tftp_request(listener, now);
        }
        if (*listener).tcpfd != -(1)
            && poll_check((*listener).tcpfd, 0x1 ) != 0
        {
            let mut confd: i32 = 0;
            let mut client_ok: i32 = 1;
            let mut iface: *mut Irec = 0 ;
            let mut p: pid_t = 0;
            let mut tcp_addr: NetAddress = NetAddress {
                sa: NetAddress {
                    sa_family: 0,
                    sa_data: [0; 14],
                },
            };
            let mut tcp_len: socklen_t =
                ::std::mem::size_of::<NetAddress>();
            loop {
                confd = accept(
                    (*listener).tcpfd,
                    NetAddressArg {
                        __NetAddress__: 0,
                    },
                    0 ,
                );
                if !(confd == -(1) && *__errno_location() == 4) {
                    break;
                }
            }
            if !(confd == -(1)) {
                if getsockname(
                    confd,
                    NetAddressArg {
                        __NetAddress__: &mut tcp_addr ,
                    },
                    &mut tcp_len,
                ) == -(1)
                {
                    close(confd);
                } else {
                    /* Make sure that the interface list is up-to-date.

                       We do this here as we may need the results below, and
                       the DNS code needs them for --interface-name stuff.

                       Multiple calls to enumerate_interfaces() per select loop are
                       inhibited, so calls to it in the child process (which doesn't select())
                       have no effect. This avoids two processes reading from the same
                       netlink fd and screwing the pooch entirely.
                    */
                    enumerate_interfaces(0); /* May be NULL */
                    if daemon.options[(13).wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>())
                            .wrapping_mul(8),
                    ) ]
                        & (1)
                            << (13).wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>())
                                    .wrapping_mul(8),
                            )
                        != 0
                    {
                        iface = (*listener).iface
                    } else {
                        let mut if_index: i32 = 0;
                        let mut intr_name: [libc::c_char; 16] = [0; 16];
                        /* if we can find the arrival interface, check it's one that's allowed */
                        if_index = tcp_interface(confd, tcp_addr.sa.sa_family); /* May be NULL */
                        if if_index != 0
                            && indextoname((*listener).tcpfd, if_index, intr_name.as_mut_ptr()) != 0
                        {
                            let mut addr: NetAddress = NetAddress {
                                addr4: NetAddress { s_addr: 0 },
                            };
                            if tcp_addr.sa.sa_family == 10 {
                                addr.addr6 = tcp_addr.in6.sin6_addr
                            } else {
                                addr.addr4 = tcp_addr.in_0.sin_addr
                            }
                            iface = daemon.interfaces;
                            while !iface.is_null() {
                                if (*iface).index == if_index
                                    && (*iface).addr.sa.sa_family
                                        == tcp_addr.sa.sa_family
                                {
                                    break;
                                }
                                iface = (*iface).next
                            }
                            if iface.is_null()
                                && loopback_exception(
                                    (*listener).tcpfd,
                                    tcp_addr.sa.sa_family,
                                    &mut addr,
                                    intr_name.as_mut_ptr(),
                                ) == 0
                            {
                                client_ok = 0
                            }
                        }
                        if daemon.options[(39).wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>())
                                .wrapping_mul(8),
                        ) ]
                            & (1)
                                << (39).wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>())
                                        .wrapping_mul(8),
                                )
                            != 0
                        {
                            iface = (*listener).iface
                        } else {
                            /* Check for allowed interfaces when binding the wildcard address:
                            we do this by looking for an interface with the same address as
                            the local address of the TCP connection, then looking to see if that's
                            an allowed interface. As a side effect, we get the netmask of the
                            interface too, for localisation. */
                            iface = daemon.interfaces; /* parent needs read pipe end. */
                            while !iface.is_null() {
                                if NetAddress_isequal(&mut (*iface).addr, &mut tcp_addr) != 0 {
                                    break;
                                }
                                iface = (*iface).next
                            }
                            if iface.is_null() {
                                client_ok = 0
                            }
                        }
                    }
                    if client_ok == 0 {
                        shutdown(confd, SHUT_RDWR);
                        close(confd);
                    } else if daemon.options[(6).wrapping_div(
                        (::std::mem::size_of::<libc::c_uint>())
                            .wrapping_mul(8),
                    ) ]
                        & (1)
                            << (6).wrapping_rem(
                                (::std::mem::size_of::<libc::c_uint>())
                                    .wrapping_mul(8),
                            )
                        == 0
                        && pipe(pipefd.as_mut_ptr()) == 0
                        && {
                            p = fork();
                            (p) != 0
                        }
                    {
                        close(pipefd[1 ]);
                        if p == -(1) {
                            close(pipefd[0 ]);
                        } else {
                            let mut i_0: i32 = 0;
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
                            let mut a: libc::c_uchar = 0;
                            read_write(
                                pipefd[0 ],
                                &mut a,
                                1,
                                1,
                            );
                            i_0 = 0;
                            while i_0 < 20 {
                                if daemon.tcp_pids[i_0 ] == 0
                                    && daemon.tcp_pipes[i_0 ] == -(1)
                                {
                                    daemon.tcp_pids[i_0 ] = p;
                                    daemon.tcp_pipes[i_0 ] =
                                        pipefd[0 ];
                                    break;
                                } else {
                                    i_0 += 1
                                }
                            }
                        }
                        close(confd);
                        /* The child can use up to TCP_MAX_QUERIES ids, so skip that many. */
                        daemon.log_id += 100
                    } else {
                        let mut buff: mut Vec<u8> = 0;
                        let mut s: Server = 0;
                        let mut flags: i32 = 0;
                        let mut netmask: NetAddress = NetAddress { s_addr: 0 };
                        let mut auth_dns: i32 = 0;
                        if !iface.is_null() {
                            netmask = (*iface).netmask;
                            auth_dns = (*iface).dns_auth
                        } else {
                            netmask.s_addr = 0;
                            auth_dns = 0
                        }
                        /* Arrange for SIGALRM after CHILD_LIFETIME seconds to
                        terminate the process. */
                        if daemon.options[(6).wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>())
                                .wrapping_mul(8),
                        ) ]
                            & (1)
                                << (6).wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>())
                                        .wrapping_mul(8),
                                )
                            == 0
                        {
                            /* See comment above re: netlink socket. */
                            let mut a_0: libc::c_uchar = 0; /* close read end in child. */
                            close(daemon.netlinkfd);
                            read_write(
                                pipefd[1 ],
                                &mut a_0,
                                1,
                                0,
                            );
                            alarm(150);
                            close(pipefd[0 ]);
                            daemon.pipe_to_parent = pipefd[1 ]
                        }
                        /* start with no upstream connections. */
                        s = daemon.servers;
                        while !s.is_null() {
                            (*s).tcpfd = -(1);
                            s = (*s).next
                        }
                        /* The connected socket inherits non-blocking
                        attribute from the listening socket.
                        Reset that here. */
                        flags = fcntl(confd, 3, 0);
                        if flags != -(1) {
                            fcntl(confd, 4, flags & !(0o4000));
                        }
                        buff = tcp_request(confd, now, &mut tcp_addr, netmask, auth_dns);
                        shutdown(confd, SHUT_RDWR);
                        close(confd);
                        if !buff.is_null() {
                            free(buff);
                        }
                        s = daemon.servers;
                        while !s.is_null() {
                            if (*s).tcpfd != -(1) {
                                shutdown((*s).tcpfd, SHUT_RDWR);
                                close((*s).tcpfd);
                            }
                            s = (*s).next
                        }
                        if daemon.options[(6).wrapping_div(
                            (::std::mem::size_of::<libc::c_uint>())
                                .wrapping_mul(8),
                        ) ]
                            & (1)
                                << (6).wrapping_rem(
                                    (::std::mem::size_of::<libc::c_uint>())
                                        .wrapping_mul(8),
                                )
                            == 0
                        {
                            close(daemon.pipe_to_parent);
                            flush_log();
                            _exit(0);
                        }
                    }
                }
            }
        }
        listener = (*listener).next
    }
}

pub fn make_icmp_sock() -> i32 {
    let mut fd: i32 = 0;
    let mut zeroopt: i32 = 0;
    fd = socket(
        2,
        SOCK_RAW,
        IPPROTO_ICMP,
    );
    if fd != -(1) {
        if fix_fd(fd) == 0
            || setsockopt(
                fd,
                1,
                5,
                &mut zeroopt,
                ::std::mem::size_of::<libc::c_int>(),
            ) == -(1)
        {
            close(fd);
            fd = -(1)
        }
    }
    return fd;
}

pub fn icmp_ping(mut addr: NetAddress) -> i32 {
    /* Try and get an ICMP echo from a machine. */
    let mut fd: i32 = 0;
    let mut saddr: NetAddress = NetAddress {
        sin_family: 0,
        sin_port: 0,
        sin_addr: NetAddress { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut packet: C2rustUnnamed27 = C2rustUnnamed27 {
        ip: IpHdr {
            ip_hl_ip_v: [0; 1],
            ip_tos: 0,
            ip_len: 0,
            ip_id: 0,
            ip_off: 0,
            ip_ttl: 0,
            ip_p: 0,
            ip_sum: 0,
            ip_src: NetAddress { s_addr: 0 },
            ip_dst: NetAddress { s_addr: 0 },
        },
        icmp: IcmpHdr {
            icmp_type: 0,
            icmp_code: 0,
            icmp_cksum: 0,
            icmp_hun: C2rustUnnamed17 { ih_pptr: 0 },
            icmp_dun: C2rustUnnamed14 {
                id_ts: C2rustUnnamed16 {
                    its_otime: 0,
                    its_rtime: 0,
                    its_ttime: 0,
                },
            },
        },
    };
    let mut id: u16 = rand16();
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut gotreply: i32 = 0;
    fd = make_icmp_sock();
    if fd == -(1) {
        return 0;
    }
    saddr.sin_family = 2;
    saddr.sin_port = 0 as in_port_t;
    saddr.sin_addr = addr;
    packet.icmp.icmp_type = 8 ;
    packet.icmp.icmp_hun.ih_idseq.icd_id = id;
    j = 0;
    i = 0;
    while (i)
        < (::std::mem::size_of::<IcmpHdr>())
            .wrapping_div(2)
    {
        j = j.wrapping_add(
            *(&mut packet.icmp ).offset(i),
        );
        i = i.wrapping_add(1)
    }
    while j >> 16 != 0 {
        j = (j & 0xffff).wrapping_add(j >> 16)
    }
    packet.icmp.icmp_cksum = if j == 0xffff {
        j
    } else {
        !j
    };
    while retry_send(sendto(
        fd,
        &mut packet.icmp  ,
        ::std::mem::size_of::<IcmpHdr>(),
        0,
        ConstNetAddressArg {
            __NetAddress__: &mut saddr,
        },
        ::std::mem::size_of::<NetAddress>(),
    )) != 0
    {}
    gotreply = delay_dhcp(dnsmasq_time(), 3, fd, addr.s_addr, id);
    close(fd);
    return gotreply;
}

pub fn delay_dhcp(
    mut start: time::Instant,
    mut sec: i32,
    mut fd: i32,
    mut addr: u32,
    mut id: u16,
) -> i32 {
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
    let mut rc: i32 = 0;
    let mut timeout_count: i32 = 0;
    let mut now: time::Instant = 0;
    now = dnsmasq_time();
    timeout_count = 0;
    while difftime(now, start) <= sec
        && timeout_count < sec * 4
    {
        poll_reset();
        if fd != -(1) {
            poll_listen(fd, 0x1 );
        }
        set_dns_listeners(now);
        set_log_writer();
        if daemon.doing_ra != 0 {
            poll_listen(daemon.icmp6fd, 0x1 );
        }
        rc = do_poll(250);
        if rc < 0 {
            continue;
        }
        if rc == 0 {
            timeout_count += 1
        }
        now = dnsmasq_time();
        check_log_writer(0);
        check_dns_listeners(now);
        if daemon.doing_ra != 0
            && poll_check(daemon.icmp6fd, 0x1 ) != 0
        {
            icmp6_packet(now);
        }
        check_tftp_listeners(now);
        if fd != -(1) {
            let mut packet: C2rustUnnamed26 = C2rustUnnamed26 {
                ip: IpHdr {
                    ip_hl_ip_v: [0; 1],
                    ip_tos: 0,
                    ip_len: 0,
                    ip_id: 0,
                    ip_off: 0,
                    ip_ttl: 0,
                    ip_p: 0,
                    ip_sum: 0,
                    ip_src: NetAddress { s_addr: 0 },
                    ip_dst: NetAddress { s_addr: 0 },
                },
                icmp: IcmpHdr {
                    icmp_type: 0,
                    icmp_code: 0,
                    icmp_cksum: 0,
                    icmp_hun: C2rustUnnamed17 { ih_pptr: 0 },
                    icmp_dun: C2rustUnnamed14 {
                        id_ts: C2rustUnnamed16 {
                            its_otime: 0,
                            its_rtime: 0,
                            its_ttime: 0,
                        },
                    },
                },
            };
            let mut faddr: NetAddress = NetAddress {
                sin_family: 0,
                sin_port: 0,
                sin_addr: NetAddress { s_addr: 0 },
                sin_zero: [0; 8],
            };
            let mut len: socklen_t =
                ::std::mem::size_of::<NetAddress>();
            if poll_check(fd, 0x1 ) != 0
                && recvfrom(
                fd,
                &mut packet ,
                ::std::mem::size_of::<C2rustUnnamed26>(),
                0,
                NetAddressArg {
                        __NetAddress__: &mut faddr,
                    },
                &mut len,
                )
                    == ::std::mem::size_of::<C2rustUnnamed26>()
                && addr == faddr.sin_addr.s_addr
                && packet.icmp.icmp_type == 0
                && packet.icmp.icmp_hun.ih_idseq.icd_seq == 0
                && packet.icmp.icmp_hun.ih_idseq.icd_id == id
            {
                return 1;
            }
        }
    }
    return 0;
}
#[main]
pub fn main() {
    let mut args: Vec<&mut String> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1),
            args.as_mut_ptr() ,
        ) as i32)
    }
}
/* HAVE_DHCP */
