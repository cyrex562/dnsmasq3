

#![allow(dead_code, mutable_transmutes, unused_assignments, unused_mut)]
mod defines;
mod util;
mod option;
mod dhcp_common;
mod arp;
mod auth;

use defines::{C2RustUnnamed_12, _SC_OPEN_MAX, __sighandler_t, __sigset_t, cap_user_data_t, cap_user_header_t, dhcp_context, dhcp_relay, dnsmasq_daemon, gid_t, group, iname, passwd, pid_t, server, sigaction, time_t, uid_t};

use libc;
use crate::util::dnsmasq_time;
use crate::defines::{__user_cap_header_struct, __user_cap_data_struct};
use crate::dhcp_common::{dhcp_common_init, whichdevice, bind_to_device};

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
/* Declare static char *compiler_opts  in config.h */
// #[no_mangle]
// pub static mut daemon: *mut dnsmasq_daemon = 0 as *const daemon as *mut dnsmasq_daemon;

// static mut pid: pid_t = 0 as libc::c_int;
// static mut pipewrite: libc::c_int = 0;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
     let mut compiler_opts: String = String::from("");
    let mut bind_fallback: libc::c_int = 0 as libc::c_int;
    let mut now: time_t = 0;
    let mut sigact: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_12{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    let mut if_tmp: iname = Default::default();
    let mut piperead: libc::c_int = 0;
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    let mut err_pipe: [libc::c_int; 2] = [0; 2];
    let mut ent_pw: *mut passwd = 0 as *mut passwd;
    let mut script_uid: uid_t = 0 as libc::c_int as uid_t;
    let mut script_gid: gid_t = 0 as libc::c_int as gid_t;
    let mut gp: *mut group = 0 as *mut group;
    let mut i: libc::c_long = 0;
    let mut max_fd: libc::c_long = libc::sysconf(libc::_SC_OPEN_MAX as libc::c_int);
    let mut baduser: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut log_err: libc::c_int = 0;
    let mut chown_warn: libc::c_int = 0 as libc::c_int;
    // let mut hdr: cap_user_header_t = 0 as cap_user_header_t;
    let mut hdr: __user_cap_header_struct = Default::default();
    let mut data: __user_cap_data_struct = Default::default();
    let mut need_cap_net_admin: libc::c_int = 0;
    let mut need_cap_net_raw: libc::c_int = 0;
    let mut need_cap_net_bind_service: libc::c_int = 0;
    let mut bound_device: String = String::new();
    let mut did_bind: bool = false;
    let mut serv: server = Default::default();
    let mut netlink_warn: String = String::new();
    let mut context: dhcp_context = Default::default();
    let mut relay: dhcp_relay = Default::default();
    let mut tftp_prefix_missing: libc::c_int = 0;
    sigact.__sigaction_handler.sa_handler =
        Some(sig_handler as unsafe extern "C" fn(_: libc::c_int) -> ());
    sigact.sa_flags = 0 as libc::c_int;
    libc::sigemptyset(&mut sigact.sa_mask as *mut __sigset_t);
    libc::sigaction(10 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    libc::sigaction(12 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    libc::sigaction(1 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    libc::sigaction(15 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    libc::sigaction(14 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    libc::sigaction(17 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    libc::sigaction(2 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    /* ignore SIGPIPE */
    sigact.__sigaction_handler.sa_handler =
        ::std::mem::transmute::<libc::intptr_t,
                                __sighandler_t>(1 as libc::c_int as
                                                    libc::intptr_t); /* known umask, create leases and pid files as 0644 */
    libc::sigaction(13 as libc::c_int, &mut sigact,
              0 as *mut sigaction); /* Must precede read_opts() */
    libc::umask(0o22 as libc::c_int as defines::__mode_t);
    util::rand_init();
    option::read_opts(argc, argv, compile_opts);

    let mut daemon: dnsmasq_daemon = Default::default();

    if cfg!(target_os = "linux") {
        daemon.kernel_version = util::get_linux_kernel_version();
    }

    if (daemon.edns_pktsz as libc::c_int) < 512 as libc::c_int {
        daemon.edns_pktsz = 512 as libc::c_int as libc::c_ushort
    }
    /* Min buffer size: we check after adding each record, so there must be 
     memory for the largest packet, and the largest record so the
     min for DNS is PACKETSZ+MAXDNAME+RRFIXEDSZ which is < 1000.
     This might be increased is EDNS packet size if greater than the minimum. */
    daemon.packet_buff_sz =
        daemon.edns_pktsz as libc::c_int + 1025 as libc::c_int +
            10 as libc::c_int;
    // daemon.packet =
    //     safe_malloc(daemon.packet_buff_sz as libc::ABDAY_3size_t) as
    //         *mut libc::c_char;
    daemon.packet = Vec::new();
    // daemon.addrbuff =
    //     safe_malloc(46 as libc::c_int as libc::size_t) as *mut libc::c_char;
    daemon.addrbuff = Vec::new();
    if daemon.options[(51 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (51 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        // daemon.addrbuff2 =
        //     safe_malloc(46 as libc::c_int as libc::size_t) as *mut libc::c_char
        daemon.addrbuff2 = Vec::new();
    }
    if daemon.lease_file.is_null() {
        if !daemon.dhcp.is_null() ||
               !daemon.dhcp6.is_null() {
            daemon.lease_file = String::from("/var/lib/misc/dnsmasq.leases\x00");
        }
    }
    /* Ensure that at least stdin, stdout and stderr (fd 0, 1, 2) exist,
     otherwise file descriptors we create can end up being 0, 1, or 2 
     and then get accidentally closed later when we make 0, 1, and 2 
     open to /dev/null. Normally we'll be started with 0, 1 and 2 open, 
     but it's not guaranteed. By opening /dev/null three times, we 
     ensure that we're not using those fds for real stuff. */
    i = 0 as libc::c_int as libc::c_long;
    while i < 3 as libc::c_int as libc::c_long {
        open(b"/dev/null\x00" as *const u8 as *const libc::c_char,
             0o2 as libc::c_int);
        i += 1
    }
    /* Close any file descriptors we inherited apart from std{in|out|err} */
    // TODO:
    // close_fds(max_fd, -(1 as libc::c_int), -(1 as libc::c_int),
    //           -(1 as libc::c_int));
    if daemon.options[(45 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (45 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        die(b"DNSSEC not available: set HAVE_DNSSEC in src/config.h\x00" as
                *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 1 as libc::c_int);
    }
    if daemon.options[(35 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (35 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        die(b"conntrack support not available: set HAVE_CONNTRACK in src/config.h\x00"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 1 as libc::c_int);
    }
    if daemon.options[(58 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (58 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        die(b"Ubus not available: set HAVE_UBUS in src/config.h\x00" as
                *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 1 as libc::c_int);
    }
    if daemon.max_port < daemon.min_port {
        die(b"max_port cannot be smaller than min_port\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 1 as libc::c_int);
    }
    now = dnsmasq_time();
    if !daemon.auth_zones.is_null() {
        if daemon.authserver.is_null() {
            die(b"--auth-server required when an auth zone is defined.\x00" as
                    *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char, 1 as libc::c_int);
        }
        /* Create a serial at startup if not configured. */
        if daemon.soa_sn == 0 as libc::c_int as libc::c_ulong {
            daemon.soa_sn = now as libc::c_ulong
        }
    }
    if !daemon.dhcp6.is_null() {
        daemon.doing_ra =
            (daemon.options[(37 as libc::c_int as
                                            libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                             as
                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong))
                                           as usize] &
                 (1 as libc::c_uint) <<
                     (37 as libc::c_int as
                          libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                           as
                                                           libc::c_ulong).wrapping_mul(8
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong)))
                as libc::c_int;
        context = daemon.dhcp6;
        while !context.is_null() {
            if (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 8 as libc::c_int != 0 {
                daemon.doing_dhcp6 = true
            }
            if (*context).flags as libc::c_uint &
                   (1 as libc::c_uint) << 13 as libc::c_int != 0 {
                daemon.doing_ra = true
            }
            context = (*context).next
        }
    }
    /* Note that order matters here, we must call lease_init before
     creating any file descriptors which shouldn't be leaked
     to the lease-script init process. We need to call common_init
     before lease_init to allocate buffers it uses.
     The script subsystem relies on DHCP buffers, hence the last two
     conditions below. */
    if !daemon.dhcp.is_null() || daemon.doing_dhcp6 != false
           || !daemon.relay4.is_null() ||
           !daemon.relay6.is_null() ||
           daemon.options[(40 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (40 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 ||
           daemon.options[(53 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (53 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        dhcp_common_init(&mut daemon);
        if !daemon.dhcp.is_null() ||
               daemon.doing_dhcp6 != false {
            lease_init(now);
        }
    }
    if !daemon.dhcp.is_null() ||
           !daemon.relay4.is_null() {
        dhcp_init();
        if daemon.options[(21 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (21 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
            need_cap_net_raw = 1 as libc::c_int
        }
        need_cap_net_admin = 1 as libc::c_int
    }
    if daemon.doing_ra != false || daemon.doing_dhcp6 != false
           || !daemon.relay6.is_null() {
        ra_init(now);
        need_cap_net_raw = 1 as libc::c_int;
        need_cap_net_admin = 1 as libc::c_int
    }
    if daemon.doing_dhcp6 != false ||
           !daemon.relay6.is_null() {
        dhcp6_init();
    }
    if !daemon.ipsets.is_null() {
        ipset_init();
        need_cap_net_admin = 1 as libc::c_int
    }
    netlink_warn = netlink_init();
    if daemon.options[(13 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (13 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 &&
           daemon.options[(39 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (39 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        die(b"cannot set --bind-interfaces and --bind-dynamic\x00" as
                *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 1 as libc::c_int);
    }
    if enumerate_interfaces(1 as libc::c_int) == 0 ||
           enumerate_interfaces(0 as libc::c_int) == 0 {
        die(b"failed to find list of interfaces: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 5 as libc::c_int);
    }
    if daemon.options[(13 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (13 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 ||
           daemon.options[(39 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (39 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        create_bound_listeners(1 as libc::c_int);
        if daemon.options[(39 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (39 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
            if_tmp = daemon.if_names;
            while !if_tmp.is_null() {
                if !(*if_tmp).name.is_null() && (*if_tmp).used == 0 {
                    die(b"unknown interface %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        (*if_tmp).name, 2 as libc::c_int);
                }
                if_tmp = (*if_tmp).next
            }
        }
        /* after enumerate_interfaces()  */
        match whichdevice(&mut daemon) {
            Some(x) => bound_device = x,
            None => info!("bound_device not found")
        };

        if daemon.doing_dhcp {
            if daemon.doing_relay4 == false && !bound_device.is_empty() {
                bind_to_device(&bound_device, &mut daemon.dhcpfd);
                did_bind = true
            }
            if daemon.enable_pxe != 0 && !bound_device.is_empty() {
                bind_to_device(&bound_device, &mut daemon.pxefd);
                did_bind = true
            }
        }
        if daemon.doing_dhcp6 && daemon.doing_relay6 == false && !bound_device.is_empty() {
            bind_to_device(&bound_device, &mut daemon.dhcp6fd);
            did_bind = true
        }
    } else { create_wildcard_listeners(); }
    /* after enumerate_interfaces() */
    if daemon.doing_dhcp6 == true ||
           daemon.doing_relay_6 == true ||
           daemon.doing_ra == true {
        join_multicast(1 as libc::c_int);
    }
    /* After netlink_init() and before create_helper() */
    lease_make_duid(now);
    if daemon.port != 0 as libc::c_int {
        cache_init();
        blockdata_init();
        hash_questions_init();
    }
    if (daemon.port != 0 as libc::c_int ||
            !daemon.dhcp.is_null() ||
            daemon.doing_dhcp6 != 0) &&
           (daemon.options[(8 as libc::c_int as
                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                            as
                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong))
                                          as usize] &
                (1 as libc::c_uint) <<
                    (8 as libc::c_int as
                         libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                          as
                                                          libc::c_ulong).wrapping_mul(8
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong))
                == 0 || !daemon.dynamic_dirs.is_null()) {
        inotify_dnsmasq_init();
    } else { daemon.inotifyfd = -(1 as libc::c_int) }
    if !daemon.dump_file.is_null() {
        dump_init();
    } else { daemon.dumpfd = -(1 as libc::c_int) }
    if daemon.options[(19 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (19 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        die(b"DBus not available: set HAVE_DBUS in src/config.h\x00" as
                *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 1 as libc::c_int);
    }
    if daemon.options[(58 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (58 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        die(b"UBus not available: set HAVE_UBUS in src/config.h\x00" as
                *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char, 1 as libc::c_int);
    }
    if daemon.port != 0 as libc::c_int { pre_allocate_sfds(); }
    /* Note getpwnam returns static storage */
    if (!daemon.dhcp.is_null() ||
            !daemon.dhcp6.is_null()) &&
           !daemon.scriptuser.is_null() &&
           (!daemon.lease_change_command.is_null() ||
                !daemon.luascript.is_null()) {
        let mut scr_pw: *mut passwd = 0 as *mut passwd;
        scr_pw = getpwnam(daemon.scriptuser);
        if !scr_pw.is_null() {
            script_uid = (*scr_pw).pw_uid;
            script_gid = (*scr_pw).pw_gid
        } else { baduser = daemon.scriptuser }
    }
    if !daemon.username.is_null() &&
           { ent_pw = getpwnam(daemon.username); ent_pw.is_null() }
       {
        baduser = daemon.username
    } else if !daemon.groupname.is_null() &&
                  { gp = getgrnam(daemon.groupname); gp.is_null() }
     {
        baduser = daemon.groupname
    }
    if !baduser.is_null() {
        die(b"unknown user or group: %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char, baduser,
            1 as libc::c_int);
    }
    /* implement group defaults, "dip" if available, or group associated with uid */
    if daemon.group_set == 0 && gp.is_null() {
        gp = getgrnam(b"dip\x00" as *const u8 as *const libc::c_char);
        if gp.is_null() && !ent_pw.is_null() {
            gp = getgrgid((*ent_pw).pw_gid)
        }
        /* for error message */
        if !gp.is_null() { daemon.groupname = (*gp).gr_name }
    }
    /* We keep CAP_NETADMIN (for ARP-injection) and
     CAP_NET_RAW (for icmp) if we're doing dhcp,
     if we have yet to bind ports because of DAD, 
     or we're doing it dynamically, we need CAP_NET_BIND_SERVICE. */
    if (is_dad_listeners() != 0 ||
            daemon.options[(39 as libc::c_int as
                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                            as
                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong))
                                          as usize] &
                (1 as libc::c_uint) <<
                    (39 as libc::c_int as
                         libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                          as
                                                          libc::c_ulong).wrapping_mul(8
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong))
                != 0) &&
           (daemon.options[(40 as libc::c_int as
                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                            as
                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong))
                                          as usize] &
                (1 as libc::c_uint) <<
                    (40 as libc::c_int as
                         libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                          as
                                                          libc::c_ulong).wrapping_mul(8
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong))
                != 0 ||
                daemon.port != 0 as libc::c_int &&
                    daemon.port <= 1024 as libc::c_int) {
        need_cap_net_bind_service = 1 as libc::c_int
    }
    /* usptream servers which bind to an interface call SO_BINDTODEVICE
     for each TCP connection, so need CAP_NET_RAW */
    serv = daemon.servers;
    while !serv.is_null() {
        if (*serv).interface[0 as libc::c_int as usize] as libc::c_int !=
               0 as libc::c_int {
            need_cap_net_raw = 1 as libc::c_int
        }
        serv = (*serv).next
    }
    /* If we're doing Dbus or UBus, the above can be set dynamically,
     (as can ports) so always (potentially) needed. */
    /* determine capability API version here, while we can still
     call safe_malloc */
    let mut capsize: libc::c_int =
        1 as libc::c_int; /* for header version 1 */
    let mut fail: *mut libc::c_char = 0 as *mut libc::c_char;
    // hdr =
    //     safe_malloc(::std::mem::size_of::<__user_cap_header_struct>() as
    //                     libc::c_ulong) as cap_user_header_t;
    // /* find version supported by kernel */
    // memset(hdr as *mut libc::c_void, 0 as libc::c_int,
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
    data =
        safe_malloc((::std::mem::size_of::<__user_cap_data_struct>() as
                         libc::c_ulong).wrapping_mul(capsize as
                                                         libc::c_ulong)) as
            cap_user_data_t;
    capget(hdr, data);
    if need_cap_net_admin != 0 &&
           (*data).permitted &
               ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint == 0
       {
        fail =
            b"NET_ADMIN\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if need_cap_net_raw != 0 &&
                  (*data).permitted &
                      ((1 as libc::c_int) << 13 as libc::c_int) as
                          libc::c_uint == 0 {
        fail =
            b"NET_RAW\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if need_cap_net_bind_service != 0 &&
                  (*data).permitted &
                      ((1 as libc::c_int) << 10 as libc::c_int) as
                          libc::c_uint == 0 {
        fail =
            b"NET_BIND_SERVICE\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    }
    if !fail.is_null() {
        die(b"process is missing required capability %s\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char, fail,
            5 as libc::c_int);
    }
    /* Now set bitmaps to set caps after daemonising */
    memset(data as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<__user_cap_data_struct>() as
                libc::c_ulong).wrapping_mul(capsize as libc::c_ulong));
    if need_cap_net_admin != 0 {
        (*data).effective |=
            ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint
    }
    if need_cap_net_raw != 0 {
        (*data).effective |=
            ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint
    }
    if need_cap_net_bind_service != 0 {
        (*data).effective |=
            ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_uint
    }
    (*data).permitted = (*data).effective;
    /* Use a pipe to carry signals and other events back to the event loop 
     in a race-free manner and another to carry errors to daemon-invoking process */
    safe_pipe(pipefd.as_mut_ptr(), 1 as libc::c_int);
    piperead = pipefd[0 as libc::c_int as usize];
    ::std::ptr::write_volatile(&mut pipewrite as *mut libc::c_int,
                               pipefd[1 as libc::c_int as usize]);
    /* prime the pipe to load stuff first time. */
    send_event(pipewrite, 21 as libc::c_int, 0 as libc::c_int,
               0 as *mut libc::c_char);
    err_pipe[1 as libc::c_int as usize] = -(1 as libc::c_int);
    if daemon.options[(6 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (6 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        /* The following code "daemonizes" the process. 
	 See Stevens section 12.4 */
        if chdir(b"/\x00" as *const u8 as *const libc::c_char) !=
               0 as libc::c_int {
            die(b"cannot chdir to filesystem root: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char, 5 as libc::c_int);
        }
        if daemon.options[(16 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (16 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               == 0 {
            let mut pid_0: pid_t = 0;
            /* pipe to carry errors back to original process.
	     When startup is complete we close this and the process terminates. */
            safe_pipe(err_pipe.as_mut_ptr(), 0 as libc::c_int);
            pid_0 = fork();
            if pid_0 == -(1 as libc::c_int) {
                /* fd == -1 since we've not forked, never returns. */
                send_event(-(1 as libc::c_int), 18 as libc::c_int,
                           *__errno_location(), 0 as *mut libc::c_char);
            }
            if pid_0 != 0 as libc::c_int {
                let mut ev: event_desc =
                    event_desc{event: 0, data: 0, msg_sz: 0,};
                let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
                /* close our copy of write-end */
                close(err_pipe[1 as libc::c_int as usize]);
                /* check for errors after the fork */
                if read_event(err_pipe[0 as libc::c_int as usize], &mut ev,
                              &mut msg) != 0 {
                    fatal_event(&mut ev, msg);
                }
                _exit(0 as libc::c_int);
            }
            close(err_pipe[0 as libc::c_int as usize]);
            /* NO calls to die() from here on. */
            setsid();
            pid_0 = fork();
            if pid_0 == -(1 as libc::c_int) {
                send_event(err_pipe[1 as libc::c_int as usize],
                           18 as libc::c_int, *__errno_location(),
                           0 as *mut libc::c_char);
            }
            if pid_0 != 0 as libc::c_int { _exit(0 as libc::c_int); }
        }
        /* write pidfile _after_ forking ! */
        if !daemon.runfile.is_null() {
            let mut fd: libc::c_int = 0;
            let mut err: libc::c_int = 0 as libc::c_int;
            sprintf(daemon.namebuff,
                    b"%d\n\x00" as *const u8 as *const libc::c_char,
                    getpid());
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
            fd =
                open(daemon.runfile,
                     0o1 as libc::c_int | 0o100 as libc::c_int |
                         0o1000 as libc::c_int | 0o200 as libc::c_int,
                     0o200 as libc::c_int | 0o400 as libc::c_int |
                         0o400 as libc::c_int >> 3 as libc::c_int |
                         0o400 as libc::c_int >> 3 as libc::c_int >>
                             3 as libc::c_int);
            if fd == -(1 as libc::c_int) {
                /* only complain if started as root */
                if getuid() == 0 as libc::c_int as libc::c_uint {
                    err = 1 as libc::c_int
                }
            } else {
                /* We're still running as root here. Change the ownership of the PID file
		 to the user we will be running as. Note that this is not to allow
		 us to delete the file, since that depends on the permissions 
		 of the directory containing the file. That directory will
		 need to by owned by the dnsmasq user, and the ownership of the
		 file has to match, to keep systemd >273 happy. */
                if getuid() == 0 as libc::c_int as libc::c_uint &&
                       !ent_pw.is_null() &&
                       (*ent_pw).pw_uid != 0 as libc::c_int as libc::c_uint &&
                       fchown(fd, (*ent_pw).pw_uid, (*ent_pw).pw_gid) ==
                           -(1 as libc::c_int) {
                    chown_warn = *__errno_location()
                }
                if read_write(fd,
                              daemon.namebuff as
                                  *mut libc::c_uchar,
                              strlen(daemon.namebuff) as
                                  libc::c_int, 0 as libc::c_int) == 0 {
                    err = 1 as libc::c_int
                } else if close(fd) == -(1 as libc::c_int) {
                    err = 1 as libc::c_int
                }
            }
            if err != 0 {
                send_event(err_pipe[1 as libc::c_int as usize],
                           13 as libc::c_int, *__errno_location(),
                           daemon.runfile);
                _exit(0 as libc::c_int);
            }
        }
    }
    log_err = log_start(ent_pw, err_pipe[1 as libc::c_int as usize]);
    if daemon.options[(6 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (6 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        /* open  stdout etc to /dev/null */
        let mut nullfd: libc::c_int =
            open(b"/dev/null\x00" as *const u8 as *const libc::c_char,
                 0o2 as libc::c_int);
        if nullfd != -(1 as libc::c_int) {
            dup2(nullfd, 1 as libc::c_int);
            dup2(nullfd, 2 as libc::c_int);
            dup2(nullfd, 0 as libc::c_int);
            close(nullfd);
        }
    }
    /* if we are to run scripts, we need to fork a helper before dropping root. */
    daemon.helperfd = -(1 as libc::c_int);
    if (!daemon.dhcp.is_null() ||
            !daemon.dhcp6.is_null() ||
            daemon.options[(40 as libc::c_int as
                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                            as
                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong))
                                          as usize] &
                (1 as libc::c_uint) <<
                    (40 as libc::c_int as
                         libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                          as
                                                          libc::c_ulong).wrapping_mul(8
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong))
                != 0 ||
            daemon.options[(53 as libc::c_int as
                                           libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                            as
                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong))
                                          as usize] &
                (1 as libc::c_uint) <<
                    (53 as libc::c_int as
                         libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                          as
                                                          libc::c_ulong).wrapping_mul(8
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong))
                != 0) &&
           (!daemon.lease_change_command.is_null() ||
                !daemon.luascript.is_null()) {
        daemon.helperfd =
            create_helper(pipewrite, err_pipe[1 as libc::c_int as usize],
                          script_uid, script_gid, max_fd)
    }
    if daemon.options[(6 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (6 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 && getuid() == 0 as libc::c_int as libc::c_uint {
        let mut bad_capabilities: libc::c_int = 0 as libc::c_int;
        let mut dummy: gid_t = 0;
        /* remove all supplementary groups */
        if !gp.is_null() &&
               (setgroups(0 as libc::c_int as size_t, &mut dummy) ==
                    -(1 as libc::c_int) ||
                    setgid((*gp).gr_gid) == -(1 as libc::c_int)) {
            send_event(err_pipe[1 as libc::c_int as usize], 15 as libc::c_int,
                       *__errno_location(), daemon.groupname);
            _exit(0 as libc::c_int);
        }
        if !ent_pw.is_null() &&
               (*ent_pw).pw_uid != 0 as libc::c_int as libc::c_uint {
            /* Need to be able to drop root. */
            (*data).effective |=
                ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
            (*data).permitted |=
                ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
            /* Tell kernel to not clear capabilities when dropping root */
            if capset(hdr, data) == -(1 as libc::c_int) ||
                   prctl(8 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int,
                         0 as libc::c_int, 0 as libc::c_int) ==
                       -(1 as libc::c_int) {
                bad_capabilities = *__errno_location()
            }
            if bad_capabilities != 0 as libc::c_int {
                send_event(err_pipe[1 as libc::c_int as usize],
                           12 as libc::c_int, bad_capabilities,
                           0 as *mut libc::c_char);
                _exit(0 as libc::c_int);
            }
            /* finally drop root */
            if setuid((*ent_pw).pw_uid) == -(1 as libc::c_int) {
                send_event(err_pipe[1 as libc::c_int as usize],
                           11 as libc::c_int, *__errno_location(),
                           daemon.username);
                _exit(0 as libc::c_int);
            }
            (*data).effective &=
                !((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
            (*data).permitted &=
                !((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
            /* lose the setuid capability */
            if capset(hdr, data) == -(1 as libc::c_int) {
                send_event(err_pipe[1 as libc::c_int as usize],
                           12 as libc::c_int, *__errno_location(),
                           0 as *mut libc::c_char);
                _exit(0 as libc::c_int);
            }
        }
    }
    free(hdr as *mut libc::c_void);
    free(data as *mut libc::c_void);
    if daemon.options[(6 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (6 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        prctl(4 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int,
              0 as libc::c_int, 0 as libc::c_int);
    }
    if daemon.options[(40 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (40 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        let mut dir: *mut DIR = 0 as *mut DIR;
        let mut p: *mut tftp_prefix = 0 as *mut tftp_prefix;
        if !daemon.tftp_prefix.is_null() {
            dir = opendir(daemon.tftp_prefix);
            if dir.is_null() {
                tftp_prefix_missing = 1 as libc::c_int;
                if daemon.options[(52 as libc::c_int as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &
                       (1 as libc::c_uint) <<
                           (52 as libc::c_int as
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                       == 0 {
                    send_event(err_pipe[1 as libc::c_int as usize],
                               20 as libc::c_int, *__errno_location(),
                               daemon.tftp_prefix);
                    _exit(0 as libc::c_int);
                }
            } else { closedir(dir); }
        }
        p = daemon.if_prefix;
        while !p.is_null() {
            (*p).missing = 0 as libc::c_int;
            dir = opendir((*p).prefix);
            if dir.is_null() {
                (*p).missing = 1 as libc::c_int;
                if daemon.options[(52 as libc::c_int as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &
                       (1 as libc::c_uint) <<
                           (52 as libc::c_int as
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                       == 0 {
                    send_event(err_pipe[1 as libc::c_int as usize],
                               20 as libc::c_int, *__errno_location(),
                               (*p).prefix);
                    _exit(0 as libc::c_int);
                }
            } else { closedir(dir); }
            p = (*p).next
        }
    }
    if daemon.port == 0 as libc::c_int {
        my_syslog(6 as libc::c_int,
                  b"started, version %s DNS disabled\x00" as *const u8 as
                      *const libc::c_char,
                  b"2.84rc2\x00" as *const u8 as *const libc::c_char);
    } else {
        if daemon.cachesize != 0 as libc::c_int {
            my_syslog(6 as libc::c_int,
                      b"started, version %s cachesize %d\x00" as *const u8 as
                          *const libc::c_char,
                      b"2.84rc2\x00" as *const u8 as *const libc::c_char,
                      daemon.cachesize);
            if daemon.cachesize > 10000 as libc::c_int {
                my_syslog(4 as libc::c_int,
                          b"cache size greater than 10000 may cause performance issues, and is unlikely to be useful.\x00"
                              as *const u8 as *const libc::c_char);
            }
        } else {
            my_syslog(6 as libc::c_int,
                      b"started, version %s cache disabled\x00" as *const u8
                          as *const libc::c_char,
                      b"2.84rc2\x00" as *const u8 as *const libc::c_char);
        }
        if daemon.options[(49 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (49 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
            my_syslog(6 as libc::c_int,
                      b"DNS service limited to local subnets\x00" as *const u8
                          as *const libc::c_char);
        }
    }
    my_syslog(6 as libc::c_int,
              b"compile time options: %s\x00" as *const u8 as
                  *const libc::c_char, compile_opts);
    if chown_warn != 0 as libc::c_int {
        my_syslog(4 as libc::c_int,
                  b"chown of PID file %s failed: %s\x00" as *const u8 as
                      *const libc::c_char, daemon.runfile,
                  strerror(chown_warn));
    }
    if log_err != 0 as libc::c_int {
        my_syslog(4 as libc::c_int,
                  b"warning: failed to change owner of %s: %s\x00" as
                      *const u8 as *const libc::c_char,
                  daemon.log_file, strerror(log_err));
    }
    if bind_fallback != 0 {
        my_syslog(4 as libc::c_int,
                  b"setting --bind-interfaces option because of OS limitations\x00"
                      as *const u8 as *const libc::c_char);
    }
    if daemon.options[(13 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (13 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        warn_bound_listeners();
    } else if daemon.options[(39 as libc::c_int as
                                             libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                              as
                                                                              libc::c_ulong).wrapping_mul(8
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_ulong))
                                            as usize] &
                  (1 as libc::c_uint) <<
                      (39 as libc::c_int as
                           libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                            as
                                                            libc::c_ulong).wrapping_mul(8
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_ulong))
                  == 0 {
        warn_wild_labels();
    }
    warn_int_names();
    if daemon.options[(13 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (13 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        if_tmp = daemon.if_names;
        while !if_tmp.is_null() {
            if !(*if_tmp).name.is_null() && (*if_tmp).used == 0 {
                my_syslog(4 as libc::c_int,
                          b"warning: interface %s does not currently exist\x00"
                              as *const u8 as *const libc::c_char,
                          (*if_tmp).name);
            }
            if_tmp = (*if_tmp).next
        }
    }
    if daemon.port != 0 as libc::c_int &&
           daemon.options[(8 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (8 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        if !daemon.resolv_files.is_null() &&
               (*daemon.resolv_files).is_default == 0 {
            my_syslog(4 as libc::c_int,
                      b"warning: ignoring resolv-file flag because no-resolv is set\x00"
                          as *const u8 as *const libc::c_char);
        }
        daemon.resolv_files = 0 as *mut resolvc;
        if daemon.servers.is_null() {
            my_syslog(4 as libc::c_int,
                      b"warning: no upstream servers configured\x00" as
                          *const u8 as *const libc::c_char);
        }
    }
    if daemon.max_logs != 0 as libc::c_int {
        my_syslog(6 as libc::c_int,
                  b"asynchronous logging enabled, queue limit is %d messages\x00"
                      as *const u8 as *const libc::c_char,
                  daemon.max_logs);
    }
    context = daemon.dhcp;
    while !context.is_null() {
        log_context(2 as libc::c_int, context);
        context = (*context).next
    }
    relay = daemon.relay4;
    while !relay.is_null() {
        log_relay(2 as libc::c_int, relay);
        relay = (*relay).next
    }
    context = daemon.dhcp6;
    while !context.is_null() {
        log_context(10 as libc::c_int, context);
        context = (*context).next
    }
    relay = daemon.relay6;
    while !relay.is_null() {
        log_relay(10 as libc::c_int, relay);
        relay = (*relay).next
    }
    if daemon.doing_dhcp6 != 0 || daemon.doing_ra != 0 {
        dhcp_construct_contexts(now);
    }
    if daemon.options[(37 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (37 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"IPv6 router advertisement enabled\x00" as *const u8 as
                      *const libc::c_char);
    }
    if did_bind != 0 {
        my_syslog((3 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"DHCP, sockets bound exclusively to interface %s\x00" as
                      *const u8 as *const libc::c_char, bound_device);
    }
    if !netlink_warn.is_null() { my_syslog(4 as libc::c_int, netlink_warn); }
    /* after dhcp_construct_contexts */
    if !daemon.dhcp.is_null() || daemon.doing_dhcp6 != 0
       {
        lease_find_interfaces(now);
    }
    if daemon.options[(40 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (40 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           != 0 {
        let mut p_0: *mut tftp_prefix = 0 as *mut tftp_prefix;
        my_syslog((1 as libc::c_int) << 3 as libc::c_int | 6 as libc::c_int,
                  b"TFTP %s%s %s %s\x00" as *const u8 as *const libc::c_char,
                  if !daemon.tftp_prefix.is_null() {
                      b"root is \x00" as *const u8 as *const libc::c_char
                  } else {
                      b"enabled\x00" as *const u8 as *const libc::c_char
                  },
                  if !daemon.tftp_prefix.is_null() {
                      daemon.tftp_prefix as *const libc::c_char
                  } else { b"\x00" as *const u8 as *const libc::c_char },
                  if daemon.options[(26 as libc::c_int as
                                                    libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong))
                                                   as usize] &
                         (1 as libc::c_uint) <<
                             (26 as libc::c_int as
                                  libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong))
                         != 0 {
                      b"secure mode\x00" as *const u8 as *const libc::c_char
                  } else { b"\x00" as *const u8 as *const libc::c_char },
                  if daemon.options[(60 as libc::c_int as
                                                    libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong))
                                                   as usize] &
                         (1 as libc::c_uint) <<
                             (60 as libc::c_int as
                                  libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong))
                         != 0 {
                      b"single port mode\x00" as *const u8 as
                          *const libc::c_char
                  } else { b"\x00" as *const u8 as *const libc::c_char });
        if tftp_prefix_missing != 0 {
            my_syslog((1 as libc::c_int) << 3 as libc::c_int |
                          4 as libc::c_int,
                      b"warning: %s inaccessible\x00" as *const u8 as
                          *const libc::c_char, daemon.tftp_prefix);
        }
        p_0 = daemon.if_prefix;
        while !p_0.is_null() {
            if (*p_0).missing != 0 {
                my_syslog((1 as libc::c_int) << 3 as libc::c_int |
                              4 as libc::c_int,
                          b"warning: TFTP directory %s inaccessible\x00" as
                              *const u8 as *const libc::c_char,
                          (*p_0).prefix);
            }
            p_0 = (*p_0).next
        }
        /* This is a guess, it assumes that for small limits, 
	 disjoint files might be served, but for large limits, 
	 a single file will be sent to may clients (the file only needs
	 one fd). */
        max_fd -= 30 as libc::c_int as libc::c_long; /* use other than TFTP */
        if max_fd < 0 as libc::c_int as libc::c_long {
            max_fd = 5 as libc::c_int as libc::c_long
        } else if max_fd < 100 as libc::c_int as libc::c_long &&
                      daemon.options[(60 as libc::c_int as
                                                     libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                      as
                                                                                      libc::c_ulong).wrapping_mul(8
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_ulong))
                                                    as usize] &
                          (1 as libc::c_uint) <<
                              (60 as libc::c_int as
                                   libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(8
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong))
                          == 0 {
            max_fd = max_fd / 2 as libc::c_int as libc::c_long
        } else { max_fd = max_fd - 20 as libc::c_int as libc::c_long }
        /* if we have to use a limited range of ports, 
	 that will limit the number of transfers */
        if daemon.start_tftp_port != 0 as libc::c_int &&
               ((daemon.end_tftp_port -
                     daemon.start_tftp_port + 1 as libc::c_int) as
                    libc::c_long) < max_fd {
            max_fd =
                (daemon.end_tftp_port -
                     daemon.start_tftp_port + 1 as libc::c_int) as
                    libc::c_long
        }
        if daemon.tftp_max as libc::c_long > max_fd {
            daemon.tftp_max = max_fd as libc::c_int;
            my_syslog((1 as libc::c_int) << 3 as libc::c_int |
                          4 as libc::c_int,
                      b"restricting maximum simultaneous TFTP transfers to %d\x00"
                          as *const u8 as *const libc::c_char,
                      daemon.tftp_max);
        }
    }
    /* finished start-up - release original process */
    if err_pipe[1 as libc::c_int as usize] != -(1 as libc::c_int) {
        close(err_pipe[1 as libc::c_int as usize]);
    }
    if daemon.port != 0 as libc::c_int { check_servers(); }
    ::std::ptr::write_volatile(&mut pid as *mut pid_t, getpid());
    daemon.pipe_to_parent = -(1 as libc::c_int);
    i = 0 as libc::c_int as libc::c_long;
    while i < 20 as libc::c_int as libc::c_long {
        daemon.tcp_pipes[i as usize] = -(1 as libc::c_int);
        i += 1
    }
    /* Using inotify, have to select a resolv file at startup */
    poll_resolv(1 as libc::c_int, 0 as libc::c_int, now);
    loop  {
        let mut t: libc::c_int = 0;
        let mut timeout: libc::c_int = -(1 as libc::c_int);
        poll_reset();
        /* if we are out of resources, find how long we have to wait
	 for some to come free, we'll loop around then and restart
	 listening for queries */
        t = set_dns_listeners(now);
        if t != 0 as libc::c_int { timeout = t * 1000 as libc::c_int }
        /* Whilst polling for the dbus, or doing a tftp transfer, wake every quarter second */
        if !daemon.tftp_trans.is_null() ||
               daemon.options[(19 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (19 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 && daemon.dbus.is_null() {
            timeout = 250 as libc::c_int
        } else if is_dad_listeners() != 0 { timeout = 1000 as libc::c_int }
        if !daemon.dhcp.is_null() ||
               !daemon.relay4.is_null() {
            poll_listen(daemon.dhcpfd,
                        0x1 as libc::c_int as libc::c_short);
            if daemon.pxefd != -(1 as libc::c_int) {
                poll_listen(daemon.pxefd,
                            0x1 as libc::c_int as libc::c_short);
            }
        }
        if daemon.doing_dhcp6 != 0 ||
               !daemon.relay6.is_null() {
            poll_listen(daemon.dhcp6fd,
                        0x1 as libc::c_int as libc::c_short);
        }
        if daemon.doing_ra != 0 {
            poll_listen(daemon.icmp6fd,
                        0x1 as libc::c_int as libc::c_short);
        }
        if daemon.inotifyfd != -(1 as libc::c_int) {
            poll_listen(daemon.inotifyfd,
                        0x1 as libc::c_int as libc::c_short);
        }
        poll_listen(daemon.netlinkfd,
                    0x1 as libc::c_int as libc::c_short);
        poll_listen(piperead, 0x1 as libc::c_int as libc::c_short);
        while helper_buf_empty() != 0 && do_script_run(now) != 0 { }
        /* Wake every second whilst waiting for DAD to complete */
        /* Refresh cache */
        if daemon.options[(53 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (53 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
            find_mac(0 as *mut mysockaddr, 0 as *mut libc::c_uchar,
                     0 as libc::c_int, now);
        }
        while helper_buf_empty() != 0 && do_arp_script_run() != 0 { }
        while helper_buf_empty() != 0 && do_tftp_script_run() != 0 { }
        if helper_buf_empty() == 0 {
            poll_listen(daemon.helperfd,
                        0x4 as libc::c_int as libc::c_short);
        }
        /* must do this just before do_poll(), when we know no
	 more calls to my_syslog() can occur */
        set_log_writer();
        if do_poll(timeout) < 0 as libc::c_int { continue ; }
        now = dnsmasq_time();
        check_log_writer(0 as libc::c_int);
        /* prime. */
        enumerate_interfaces(1 as libc::c_int);
        /* Check the interfaces to see if any have exited DAD state
	 and if so, bind the address. */
        if is_dad_listeners() != 0 {
            enumerate_interfaces(0 as libc::c_int);
            /* NB, is_dad_listeners() == 1 --> we're binding interfaces */
            create_bound_listeners(0 as libc::c_int);
            warn_bound_listeners();
        }
        if poll_check(daemon.netlinkfd,
                      0x1 as libc::c_int as libc::c_short) != 0 {
            netlink_multicast();
        }
        if daemon.inotifyfd != -(1 as libc::c_int) &&
               poll_check(daemon.inotifyfd,
                          0x1 as libc::c_int as libc::c_short) != 0 &&
               inotify_check(now) != 0 {
            if daemon.port != 0 as libc::c_int &&
                   daemon.options[(5 as libc::c_int as
                                                  libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                   as
                                                                                   libc::c_ulong).wrapping_mul(8
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong))
                                                 as usize] &
                       (1 as libc::c_uint) <<
                           (5 as libc::c_int as
                                libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                 as
                                                                 libc::c_ulong).wrapping_mul(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong))
                       == 0 {
                poll_resolv(1 as libc::c_int, 1 as libc::c_int, now);
            }
        }
        if poll_check(piperead, 0x1 as libc::c_int as libc::c_short) != 0 {
            async_event(piperead, now);
        }
        check_dns_listeners(now);
        check_tftp_listeners(now);
        if !daemon.dhcp.is_null() ||
               !daemon.relay4.is_null() {
            if poll_check(daemon.dhcpfd,
                          0x1 as libc::c_int as libc::c_short) != 0 {
                dhcp_packet(now, 0 as libc::c_int);
            }
            if daemon.pxefd != -(1 as libc::c_int) &&
                   poll_check(daemon.pxefd,
                              0x1 as libc::c_int as libc::c_short) != 0 {
                dhcp_packet(now, 1 as libc::c_int);
            }
        }
        if (daemon.doing_dhcp6 != 0 ||
                !daemon.relay6.is_null()) &&
               poll_check(daemon.dhcp6fd,
                          0x1 as libc::c_int as libc::c_short) != 0 {
            dhcp6_packet(now);
        }
        if daemon.doing_ra != 0 &&
               poll_check(daemon.icmp6fd,
                          0x1 as libc::c_int as libc::c_short) != 0 {
            icmp6_packet(now);
        }
        if daemon.helperfd != -(1 as libc::c_int) &&
               poll_check(daemon.helperfd,
                          0x4 as libc::c_int as libc::c_short) != 0 {
            helper_write();
        }
    };
}
unsafe extern "C" fn sig_handler(mut sig: libc::c_int) {
    if pid == 0 as libc::c_int {
        /* ignore anything other than TERM during startup
	 and in helper proc. (helper ignore TERM too) */
        if sig == 15 as libc::c_int || sig == 2 as libc::c_int {
            exit(5 as libc::c_int);
        }
    } else if pid != getpid() {
        /* alarm is used to kill TCP children after a fixed time. */
        if sig == 14 as libc::c_int { _exit(0 as libc::c_int); }
    } else {
        /* master process */
        let mut event: libc::c_int = 0;
        let mut errsave: libc::c_int = *__errno_location();
        if sig == 1 as libc::c_int {
            event = 1 as libc::c_int
        } else if sig == 17 as libc::c_int {
            event = 5 as libc::c_int
        } else if sig == 14 as libc::c_int {
            event = 3 as libc::c_int
        } else if sig == 15 as libc::c_int {
            event = 4 as libc::c_int
        } else if sig == 10 as libc::c_int {
            event = 2 as libc::c_int
        } else if sig == 12 as libc::c_int {
            event = 6 as libc::c_int
        } else if sig == 2 as libc::c_int {
            /* Handle SIGINT normally in debug mode, so
	     ctrl-c continues to operate. */
            if daemon.options[(6 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (6 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 {
                exit(5 as libc::c_int);
            } else { event = 26 as libc::c_int }
        } else { return }
        send_event(pipewrite, event, 0 as libc::c_int,
                   0 as *mut libc::c_char);
        *__errno_location() = errsave
    };
}
/* now == 0 -> queue immediate callback */
#[no_mangle]
pub unsafe extern "C" fn send_alarm(mut event: time_t, mut now: time_t) {
    if now == 0 as libc::c_int as libc::c_long ||
           event != 0 as libc::c_int as libc::c_long {
        /* alarm(0) or alarm(-ve) doesn't do what we want.... */
        if now == 0 as libc::c_int as libc::c_long ||
               difftime(event, now) <= 0.0f64 {
            send_event(pipewrite, 3 as libc::c_int, 0 as libc::c_int,
                       0 as *mut libc::c_char);
        } else { alarm(difftime(event, now) as libc::c_uint); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn queue_event(mut event: libc::c_int) {
    send_event(pipewrite, event, 0 as libc::c_int, 0 as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn send_event(mut fd: libc::c_int,
                                    mut event: libc::c_int,
                                    mut data: libc::c_int,
                                    mut msg: *mut libc::c_char) {
    let mut ev: event_desc = event_desc{event: 0, data: 0, msg_sz: 0,};
    let mut iov: [iovec; 2] =
        [iovec{iov_base: 0 as *mut libc::c_void, iov_len: 0,}; 2];
    ev.event = event;
    ev.data = data;
    ev.msg_sz =
        if !msg.is_null() {
            strlen(msg)
        } else { 0 as libc::c_int as libc::c_ulong } as libc::c_int;
    iov[0 as libc::c_int as usize].iov_base =
        &mut ev as *mut event_desc as *mut libc::c_void;
    iov[0 as libc::c_int as usize].iov_len =
        ::std::mem::size_of::<event_desc>() as libc::c_ulong;
    iov[1 as libc::c_int as usize].iov_base = msg as *mut libc::c_void;
    iov[1 as libc::c_int as usize].iov_len = ev.msg_sz as size_t;
    /* error pipe, debug mode. */
    if fd == -(1 as libc::c_int) {
        fatal_event(&mut ev, msg);
    } else {
        /* pipe is non-blocking and struct event_desc is smaller than
       PIPE_BUF, so this either fails or writes everything */
        while writev(fd, iov.as_mut_ptr(),
                     (if !msg.is_null() {
                          2 as libc::c_int
                      } else { 1 as libc::c_int })) ==
                  -(1 as libc::c_int) as libc::c_long &&
                  *__errno_location() == 4 as libc::c_int {
        }
    };
}
/* NOTE: the memory used to return msg is leaked: use msgs in events only
   to describe fatal errors. */
unsafe extern "C" fn read_event(mut fd: libc::c_int, mut evp: *mut event_desc,
                                mut msg: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    if read_write(fd, evp as *mut libc::c_uchar,
                  ::std::mem::size_of::<event_desc>() as libc::c_ulong as
                      libc::c_int, 1 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    *msg = 0 as *mut libc::c_char;
    if (*evp).msg_sz != 0 as libc::c_int &&
           {
               buf =
                   malloc(((*evp).msg_sz + 1 as libc::c_int) as libc::c_ulong)
                       as *mut libc::c_char;
               !buf.is_null()
           } &&
           read_write(fd, buf as *mut libc::c_uchar, (*evp).msg_sz,
                      1 as libc::c_int) != 0 {
        *buf.offset((*evp).msg_sz as isize) =
            0 as libc::c_int as libc::c_char;
        *msg = buf
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fatal_event(mut ev: *mut event_desc,
                                 mut msg: *mut libc::c_char) {
    *__errno_location() = (*ev).data;
    match (*ev).event {
        16 => { exit(0 as libc::c_int); }
        18 => {
            die(b"cannot fork into background: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char, 5 as libc::c_int);
        }
        10 => {
            /* fall through */
            die(b"failed to create helper: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char, 5 as libc::c_int);
        }
        12 => {
            /* fall through */
            die(b"setting capabilities failed: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_char, 5 as libc::c_int);
        }
        11 => {
            /* fall through */
            die(b"failed to change user-id to %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char, msg,
                5 as libc::c_int);
        }
        15 => {
            /* fall through */
            die(b"failed to change group-id to %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char, msg,
                5 as libc::c_int);
        }
        13 => {
            /* fall through */
            die(b"failed to open pidfile %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char, msg,
                3 as libc::c_int);
        }
        17 => {
            /* fall through */
            die(b"cannot open log %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char, msg,
                3 as libc::c_int);
        }
        19 => {
            /* fall through */
            die(b"failed to load Lua script: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char, msg,
                5 as libc::c_int);
        }
        20 => {
            /* fall through */
            die(b"TFTP directory %s inaccessible: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char, msg,
                3 as libc::c_int);
        }
        24 => {
            /* fall through */
            die(b"cannot create timestamp file %s: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char, msg,
                1 as libc::c_int);
        }
        _ => { }
    };
}
unsafe extern "C" fn async_event(mut pipe_0: libc::c_int, mut now: time_t) {
    let mut p: pid_t = 0;
    let mut ev: event_desc = event_desc{event: 0, data: 0, msg_sz: 0,};
    let mut i: libc::c_int = 0;
    let mut check: libc::c_int = 0 as libc::c_int;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    /* NOTE: the memory used to return msg is leaked: use msgs in events only
     to describe fatal errors. */
    if read_event(pipe_0, &mut ev, &mut msg) != 0 {
        let mut current_block_60:
                u64; /* Bump zone serial, as it may have changed. */
        match ev.event {
            1 => {
                daemon.soa_sn =
                    daemon.soa_sn.wrapping_add(1);
                current_block_60 = 11195962526119371365;
            }
            21 => { current_block_60 = 11195962526119371365; }
            2 => {
                if daemon.port != 0 as libc::c_int {
                    dump_cache(now);
                }
                current_block_60 = 6367734732029634840;
            }
            3 => {
                if !daemon.dhcp.is_null() ||
                       daemon.doing_dhcp6 != 0 {
                    lease_prune(0 as *mut dhcp_lease, now);
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
                    p =
                        waitpid(-(1 as libc::c_int), 0 as *mut libc::c_int,
                                1 as libc::c_int);
                    if !(p != 0 as libc::c_int) { break ; }
                    if p == -(1 as libc::c_int) {
                        if *__errno_location() != 4 as libc::c_int { break ; }
                    } else {
                        i = 0 as libc::c_int;
                        while i < 20 as libc::c_int {
                            if daemon.tcp_pids[i as usize] == p {
                                daemon.tcp_pids[i as usize] =
                                    0 as libc::c_int
                            }
                            i += 1
                        }
                    }
                }
                current_block_60 = 6367734732029634840;
            }
            8 => {
                my_syslog(4 as libc::c_int,
                          b"script process killed by signal %d\x00" as
                              *const u8 as *const libc::c_char, ev.data);
                current_block_60 = 6367734732029634840;
            }
            7 => {
                my_syslog(4 as libc::c_int,
                          b"script process exited with status %d\x00" as
                              *const u8 as *const libc::c_char, ev.data);
                current_block_60 = 6367734732029634840;
            }
            9 => {
                my_syslog(3 as libc::c_int,
                          b"failed to execute %s: %s\x00" as *const u8 as
                              *const libc::c_char,
                          daemon.lease_change_command,
                          strerror(ev.data));
                current_block_60 = 6367734732029634840;
            }
            25 => {
                my_syslog((2 as libc::c_int) << 3 as libc::c_int |
                              7 as libc::c_int,
                          b"%s\x00" as *const u8 as *const libc::c_char,
                          if !msg.is_null() {
                              msg as *const libc::c_char
                          } else {
                              b"\x00" as *const u8 as *const libc::c_char
                          });
                free(msg as *mut libc::c_void);
                msg = 0 as *mut libc::c_char;
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
            22 => { newaddress(now); current_block_60 = 6367734732029634840; }
            23 => {
                resend_query();
                /* Force re-reading resolv file right now, for luck. */
                poll_resolv(0 as libc::c_int, 1 as libc::c_int, now);
                current_block_60 = 6367734732029634840;
            }
            4 => {
                /* Knock all our children on the head. */
                i = 0 as libc::c_int;
                while i < 20 as libc::c_int {
                    if daemon.tcp_pids[i as usize] !=
                           0 as libc::c_int {
                        kill(daemon.tcp_pids[i as usize],
                             14 as libc::c_int);
                    }
                    i += 1
                }
                /* handle pending lease transitions */
                if daemon.helperfd != -(1 as libc::c_int) {
                    /* block in writes until all done */
                    i = fcntl(daemon.helperfd, 3 as libc::c_int);
                    if i != -(1 as libc::c_int) {
                        fcntl(daemon.helperfd, 4 as libc::c_int,
                              i & !(0o4000 as libc::c_int));
                    }
                    loop  {
                        helper_write();
                        if !(helper_buf_empty() == 0 ||
                                 do_script_run(now) != 0) {
                            break ;
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
                if daemon.dumpfd != -(1 as libc::c_int) {
                    close(daemon.dumpfd);
                }
                my_syslog(6 as libc::c_int,
                          b"exiting on receipt of SIGTERM\x00" as *const u8 as
                              *const libc::c_char);
                flush_log();
                exit(0 as libc::c_int);
            }
            26 | _ => { current_block_60 = 6367734732029634840; }
        }
        match current_block_60 {
            11195962526119371365 =>
            /* fall through */
            {
                clear_cache_and_reload(now);
                if daemon.port != 0 as libc::c_int {
                    if !daemon.resolv_files.is_null() &&
                           daemon.options[(5 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (5 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               != 0 {
                        reload_servers((*daemon.resolv_files).name);
                        check = 1 as libc::c_int
                    }
                    if !daemon.servers_file.is_null() {
                        read_servers_file();
                        check = 1 as libc::c_int
                    }
                    if check != 0 { check_servers(); }
                }
                rerun_scripts();
            }
            _ => { }
        }
    };
}
unsafe extern "C" fn poll_resolv(mut force: libc::c_int,
                                 mut do_reload: libc::c_int,
                                 mut now: time_t) {
    let mut res: *mut resolvc = 0 as *mut resolvc;
    let mut latest: *mut resolvc = 0 as *mut resolvc;
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
             __glibc_reserved: [0; 3],};
    let mut last_change: time_t = 0 as libc::c_int as time_t;
    /* There may be more than one possible file. 
     Go through and find the one which changed _last_.
     Warn of any which can't be read. */
    if daemon.port == 0 as libc::c_int ||
           daemon.options[(5 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (5 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
        return
    }
    latest = 0 as *mut resolvc;
    res = daemon.resolv_files;
    while !res.is_null() {
        if stat((*res).name, &mut statbuf) == -(1 as libc::c_int) {
            if force != 0 {
                (*res).mtime = 0 as libc::c_int as time_t
            } else {
                if (*res).logged == 0 {
                    my_syslog(4 as libc::c_int,
                              b"failed to access %s: %s\x00" as *const u8 as
                                  *const libc::c_char, (*res).name,
                              strerror(*__errno_location()));
                }
                (*res).logged = 1 as libc::c_int;
                if (*res).mtime != 0 as libc::c_int as libc::c_long {
                    /* existing file evaporated, force selection of the latest
	       file even if its mtime hasn't changed since we last looked */
                    poll_resolv(1 as libc::c_int, do_reload, now);
                    return
                }
            }
        } else {
            (*res).logged = 0 as libc::c_int;
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
        static mut warned: libc::c_int = 0 as libc::c_int;
        if reload_servers((*latest).name) != 0 {
            my_syslog(6 as libc::c_int,
                      b"reading %s\x00" as *const u8 as *const libc::c_char,
                      (*latest).name);
            warned = 0 as libc::c_int;
            check_servers();
            if daemon.options[(24 as libc::c_int as
                                              libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                               as
                                                                               libc::c_ulong).wrapping_mul(8
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_ulong))
                                             as usize] &
                   (1 as libc::c_uint) <<
                       (24 as libc::c_int as
                            libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                             as
                                                             libc::c_ulong).wrapping_mul(8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong))
                   != 0 && do_reload != 0 {
                clear_cache_and_reload(now);
            }
        } else {
            (*latest).mtime = 0 as libc::c_int as time_t;
            if warned == 0 {
                my_syslog(4 as libc::c_int,
                          b"no servers found in %s, will retry\x00" as
                              *const u8 as *const libc::c_char,
                          (*latest).name);
                warned = 1 as libc::c_int
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn clear_cache_and_reload(mut now: time_t) {
    if daemon.port != 0 as libc::c_int { cache_reload(); }
    if !daemon.dhcp.is_null() || daemon.doing_dhcp6 != 0
       {
        if daemon.options[(14 as libc::c_int as
                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                         as usize] &
               (1 as libc::c_uint) <<
                   (14 as libc::c_int as
                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                         as
                                                         libc::c_ulong).wrapping_mul(8
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong))
               != 0 {
            dhcp_read_ethers();
        }
        reread_dhcp();
        dhcp_update_configs(daemon.dhcp_conf);
        lease_update_from_configs();
        lease_update_file(now);
        lease_update_dns(1 as libc::c_int);
    } else if daemon.doing_ra != 0 {
        /* Not doing DHCP, so no lease system, manage 
       alarms for ra only */
        send_alarm(periodic_ra(now), now);
    };
}
unsafe extern "C" fn set_dns_listeners(mut now: time_t) -> libc::c_int {
    let mut serverfdp: *mut serverfd = 0 as *mut serverfd;
    let mut listener: *mut listener = 0 as *mut listener;
    let mut wait: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut tftp: libc::c_int = 0 as libc::c_int;
    let mut transfer: *mut tftp_transfer = 0 as *mut tftp_transfer;
    if daemon.options[(60 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (60 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        transfer = daemon.tftp_trans;
        while !transfer.is_null() {
            tftp += 1;
            poll_listen((*transfer).sockfd,
                        0x1 as libc::c_int as libc::c_short);
            transfer = (*transfer).next
        }
    }
    /* will we be able to get memory? */
    if daemon.port != 0 as libc::c_int {
        get_new_frec(now, &mut wait, 0 as *mut frec);
    }
    serverfdp = daemon.sfds;
    while !serverfdp.is_null() {
        poll_listen((*serverfdp).fd, 0x1 as libc::c_int as libc::c_short);
        serverfdp = (*serverfdp).next
    }
    if daemon.port != 0 as libc::c_int &&
           daemon.osport == 0 {
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            if daemon.randomsocks[i as usize].refcount as
                   libc::c_int != 0 as libc::c_int {
                poll_listen(daemon.randomsocks[i as usize].fd,
                            0x1 as libc::c_int as libc::c_short);
            }
            i += 1
        }
    }
    listener = daemon.listeners;
    while !listener.is_null() {
        /* only listen for queries if we have resources */
        if (*listener).fd != -(1 as libc::c_int) && wait == 0 as libc::c_int {
            poll_listen((*listener).fd, 0x1 as libc::c_int as libc::c_short);
        }
        /* death of a child goes through the select loop, so
	 we don't need to explicitly arrange to wake up here */
        if (*listener).tcpfd != -(1 as libc::c_int) {
            i = 0 as libc::c_int;
            while i < 20 as libc::c_int {
                if daemon.tcp_pids[i as usize] == 0 as libc::c_int
                       &&
                       daemon.tcp_pipes[i as usize] ==
                           -(1 as libc::c_int) {
                    poll_listen((*listener).tcpfd,
                                0x1 as libc::c_int as libc::c_short);
                    break ;
                } else { i += 1 }
            }
        }
        /* tftp == 0 in single-port mode. */
        if tftp <= daemon.tftp_max &&
               (*listener).tftpfd != -(1 as libc::c_int) {
            poll_listen((*listener).tftpfd,
                        0x1 as libc::c_int as libc::c_short);
        }
        listener = (*listener).next
    }
    if daemon.options[(6 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (6 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        i = 0 as libc::c_int;
        while i < 20 as libc::c_int {
            if daemon.tcp_pipes[i as usize] != -(1 as libc::c_int)
               {
                poll_listen(daemon.tcp_pipes[i as usize],
                            0x1 as libc::c_int as libc::c_short);
            }
            i += 1
        }
    }
    return wait;
}
unsafe extern "C" fn check_dns_listeners(mut now: time_t) {
    let mut serverfdp: *mut serverfd = 0 as *mut serverfd;
    let mut listener: *mut listener = 0 as *mut listener;
    let mut i: libc::c_int = 0;
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    serverfdp = daemon.sfds;
    while !serverfdp.is_null() {
        if poll_check((*serverfdp).fd, 0x1 as libc::c_int as libc::c_short) !=
               0 {
            reply_query((*serverfdp).fd,
                        (*serverfdp).source_addr.sa.sa_family as libc::c_int,
                        now);
        }
        serverfdp = (*serverfdp).next
    }
    if daemon.port != 0 as libc::c_int &&
           daemon.osport == 0 {
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            if daemon.randomsocks[i as usize].refcount as
                   libc::c_int != 0 as libc::c_int &&
                   poll_check(daemon.randomsocks[i as usize].fd,
                              0x1 as libc::c_int as libc::c_short) != 0 {
                reply_query(daemon.randomsocks[i as usize].fd,
                            daemon.randomsocks[i as usize].family
                                as libc::c_int, now);
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
    if daemon.options[(6 as libc::c_int as
                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                       as
                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_ulong))
                                     as usize] &
           (1 as libc::c_uint) <<
               (6 as libc::c_int as
                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           == 0 {
        i = 0 as libc::c_int;
        while i < 20 as libc::c_int {
            if daemon.tcp_pipes[i as usize] != -(1 as libc::c_int)
                   &&
                   poll_check(daemon.tcp_pipes[i as usize],
                              (0x1 as libc::c_int | 0x10 as libc::c_int) as
                                  libc::c_short) != 0 &&
                   cache_recv_insert(now,
                                     daemon.tcp_pipes[i as usize])
                       == 0 {
                close(daemon.tcp_pipes[i as usize]);
                daemon.tcp_pipes[i as usize] = -(1 as libc::c_int)
            }
            i += 1
        }
    }
    listener = daemon.listeners;
    while !listener.is_null() {
        if (*listener).fd != -(1 as libc::c_int) &&
               poll_check((*listener).fd, 0x1 as libc::c_int as libc::c_short)
                   != 0 {
            receive_query(listener, now);
        }
        if (*listener).tftpfd != -(1 as libc::c_int) &&
               poll_check((*listener).tftpfd,
                          0x1 as libc::c_int as libc::c_short) != 0 {
            tftp_request(listener, now);
        }
        if (*listener).tcpfd != -(1 as libc::c_int) &&
               poll_check((*listener).tcpfd,
                          0x1 as libc::c_int as libc::c_short) != 0 {
            let mut confd: libc::c_int = 0;
            let mut client_ok: libc::c_int = 1 as libc::c_int;
            let mut iface: *mut irec = 0 as *mut irec;
            let mut p: pid_t = 0;
            let mut tcp_addr: mysockaddr =
                mysockaddr{sa: sockaddr{sa_family: 0, sa_data: [0; 14],},};
            let mut tcp_len: socklen_t =
                ::std::mem::size_of::<mysockaddr>() as libc::c_ulong as
                    socklen_t;
            loop  {
                confd =
                    accept((*listener).tcpfd,
                           __SOCKADDR_ARG{__sockaddr__:
                                              0 as *mut libc::c_void as
                                                  *mut sockaddr,},
                           0 as *mut socklen_t);
                if !(confd == -(1 as libc::c_int) &&
                         *__errno_location() == 4 as libc::c_int) {
                    break ;
                }
            }
            if !(confd == -(1 as libc::c_int)) {
                if getsockname(confd,
                               __SOCKADDR_ARG{__sockaddr__:
                                                  &mut tcp_addr as
                                                      *mut mysockaddr as
                                                      *mut sockaddr,},
                               &mut tcp_len) == -(1 as libc::c_int) {
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
                    enumerate_interfaces(0 as libc::c_int); /* May be NULL */
                    if daemon.options[(13 as libc::c_int as
                                                      libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                       as
                                                                                       libc::c_ulong).wrapping_mul(8
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_ulong))
                                                     as usize] &
                           (1 as libc::c_uint) <<
                               (13 as libc::c_int as
                                    libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                     as
                                                                     libc::c_ulong).wrapping_mul(8
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong))
                           != 0 {
                        iface = (*listener).iface
                    } else {
                        let mut if_index: libc::c_int = 0;
                        let mut intr_name: [libc::c_char; 16] = [0; 16];
                        /* if we can find the arrival interface, check it's one that's allowed */
                        if_index =
                            tcp_interface(confd,
                                          tcp_addr.sa.sa_family as
                                              libc::c_int); /* May be NULL */
                        if if_index != 0 as libc::c_int &&
                               indextoname((*listener).tcpfd, if_index,
                                           intr_name.as_mut_ptr()) != 0 {
                            let mut addr: all_addr =
                                all_addr{addr4: in_addr{s_addr: 0,},};
                            if tcp_addr.sa.sa_family as libc::c_int ==
                                   10 as libc::c_int {
                                addr.addr6 = tcp_addr.in6.sin6_addr
                            } else { addr.addr4 = tcp_addr.in_0.sin_addr }
                            iface = daemon.interfaces;
                            while !iface.is_null() {
                                if (*iface).index == if_index &&
                                       (*iface).addr.sa.sa_family as
                                           libc::c_int ==
                                           tcp_addr.sa.sa_family as
                                               libc::c_int {
                                    break ;
                                }
                                iface = (*iface).next
                            }
                            if iface.is_null() &&
                                   loopback_exception((*listener).tcpfd,
                                                      tcp_addr.sa.sa_family as
                                                          libc::c_int,
                                                      &mut addr,
                                                      intr_name.as_mut_ptr())
                                       == 0 {
                                client_ok = 0 as libc::c_int
                            }
                        }
                        if daemon.options[(39 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (39 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               != 0 {
                            iface = (*listener).iface
                        } else {
                            /* Check for allowed interfaces when binding the wildcard address:
		     we do this by looking for an interface with the same address as 
		     the local address of the TCP connection, then looking to see if that's
		     an allowed interface. As a side effect, we get the netmask of the
		     interface too, for localisation. */
                            iface =
                                daemon.interfaces; /* parent needs read pipe end. */
                            while !iface.is_null() {
                                if sockaddr_isequal(&mut (*iface).addr,
                                                    &mut tcp_addr) != 0 {
                                    break ;
                                }
                                iface = (*iface).next
                            }
                            if iface.is_null() {
                                client_ok = 0 as libc::c_int
                            }
                        }
                    }
                    if client_ok == 0 {
                        shutdown(confd, SHUT_RDWR as libc::c_int);
                        close(confd);
                    } else if daemon.options[(6 as libc::c_int as
                                                             libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                              as
                                                                                              libc::c_ulong).wrapping_mul(8
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_ulong))
                                                            as usize] &
                                  (1 as libc::c_uint) <<
                                      (6 as libc::c_int as
                                           libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                            as
                                                                            libc::c_ulong).wrapping_mul(8
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_ulong))
                                  == 0 &&
                                  pipe(pipefd.as_mut_ptr()) ==
                                      0 as libc::c_int &&
                                  { p = fork(); (p) != 0 as libc::c_int } {
                        close(pipefd[1 as libc::c_int as usize]);
                        if p == -(1 as libc::c_int) {
                            close(pipefd[0 as libc::c_int as usize]);
                        } else {
                            let mut i_0: libc::c_int = 0;
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
                            read_write(pipefd[0 as libc::c_int as usize],
                                       &mut a, 1 as libc::c_int,
                                       1 as libc::c_int);
                            i_0 = 0 as libc::c_int;
                            while i_0 < 20 as libc::c_int {
                                if daemon.tcp_pids[i_0 as usize] ==
                                       0 as libc::c_int &&
                                       daemon.tcp_pipes[i_0 as
                                                                       usize]
                                           == -(1 as libc::c_int) {
                                    daemon.tcp_pids[i_0 as usize] =
                                        p;
                                    daemon.tcp_pipes[i_0 as usize]
                                        = pipefd[0 as libc::c_int as usize];
                                    break ;
                                } else { i_0 += 1 }
                            }
                        }
                        close(confd);
                        /* The child can use up to TCP_MAX_QUERIES ids, so skip that many. */
                        daemon.log_id += 100 as libc::c_int
                    } else {
                        let mut buff: *mut libc::c_uchar =
                            0 as *mut libc::c_uchar;
                        let mut s: *mut server = 0 as *mut server;
                        let mut flags: libc::c_int = 0;
                        let mut netmask: in_addr = in_addr{s_addr: 0,};
                        let mut auth_dns: libc::c_int = 0;
                        if !iface.is_null() {
                            netmask = (*iface).netmask;
                            auth_dns = (*iface).dns_auth
                        } else {
                            netmask.s_addr = 0 as libc::c_int as in_addr_t;
                            auth_dns = 0 as libc::c_int
                        }
                        /* Arrange for SIGALRM after CHILD_LIFETIME seconds to
		 terminate the process. */
                        if daemon.options[(6 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (6 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               == 0 {
                            /* See comment above re: netlink socket. */
                            let mut a_0: libc::c_uchar =
                                0 as libc::c_int as
                                    libc::c_uchar; /* close read end in child. */
                            close(daemon.netlinkfd);
                            read_write(pipefd[1 as libc::c_int as usize],
                                       &mut a_0, 1 as libc::c_int,
                                       0 as libc::c_int);
                            alarm(150 as libc::c_int as libc::c_uint);
                            close(pipefd[0 as libc::c_int as usize]);
                            daemon.pipe_to_parent =
                                pipefd[1 as libc::c_int as usize]
                        }
                        /* start with no upstream connections. */
                        s = daemon.servers;
                        while !s.is_null() {
                            (*s).tcpfd = -(1 as libc::c_int);
                            s = (*s).next
                        }
                        /* The connected socket inherits non-blocking
		 attribute from the listening socket. 
		 Reset that here. */
                        flags =
                            fcntl(confd, 3 as libc::c_int, 0 as libc::c_int);
                        if flags != -(1 as libc::c_int) {
                            fcntl(confd, 4 as libc::c_int,
                                  flags & !(0o4000 as libc::c_int));
                        }
                        buff =
                            tcp_request(confd, now, &mut tcp_addr, netmask,
                                        auth_dns);
                        shutdown(confd, SHUT_RDWR as libc::c_int);
                        close(confd);
                        if !buff.is_null() {
                            free(buff as *mut libc::c_void);
                        }
                        s = daemon.servers;
                        while !s.is_null() {
                            if (*s).tcpfd != -(1 as libc::c_int) {
                                shutdown((*s).tcpfd,
                                         SHUT_RDWR as libc::c_int);
                                close((*s).tcpfd);
                            }
                            s = (*s).next
                        }
                        if daemon.options[(6 as libc::c_int as
                                                          libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                           as
                                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_ulong))
                                                         as usize] &
                               (1 as libc::c_uint) <<
                                   (6 as libc::c_int as
                                        libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                         as
                                                                         libc::c_ulong).wrapping_mul(8
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                               == 0 {
                            close(daemon.pipe_to_parent);
                            flush_log();
                            _exit(0 as libc::c_int);
                        }
                    }
                }
            }
        }
        listener = (*listener).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn make_icmp_sock() -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut zeroopt: libc::c_int = 0 as libc::c_int;
    fd =
        socket(2 as libc::c_int, SOCK_RAW as libc::c_int,
               IPPROTO_ICMP as libc::c_int);
    if fd != -(1 as libc::c_int) {
        if fix_fd(fd) == 0 ||
               setsockopt(fd, 1 as libc::c_int, 5 as libc::c_int,
                          &mut zeroopt as *mut libc::c_int as
                              *const libc::c_void,
                          ::std::mem::size_of::<libc::c_int>() as
                              libc::c_ulong as socklen_t) ==
                   -(1 as libc::c_int) {
            close(fd);
            fd = -(1 as libc::c_int)
        }
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn icmp_ping(mut addr: in_addr) -> libc::c_int {
    /* Try and get an ICMP echo from a machine. */
    let mut fd: libc::c_int = 0;
    let mut saddr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut packet: C2RustUnnamed_27 =
        C2RustUnnamed_27{ip:
                             ip{ip_hl_ip_v: [0; 1],
                                ip_tos: 0,
                                ip_len: 0,
                                ip_id: 0,
                                ip_off: 0,
                                ip_ttl: 0,
                                ip_p: 0,
                                ip_sum: 0,
                                ip_src: in_addr{s_addr: 0,},
                                ip_dst: in_addr{s_addr: 0,},},
                         icmp:
                             icmp{icmp_type: 0,
                                  icmp_code: 0,
                                  icmp_cksum: 0,
                                  icmp_hun: C2RustUnnamed_17{ih_pptr: 0,},
                                  icmp_dun:
                                      C2RustUnnamed_14{id_ts:
                                                           C2RustUnnamed_16{its_otime:
                                                                                0,
                                                                            its_rtime:
                                                                                0,
                                                                            its_ttime:
                                                                                0,},},},};
    let mut id: libc::c_ushort = rand16();
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut gotreply: libc::c_int = 0 as libc::c_int;
    fd = make_icmp_sock();
    if fd == -(1 as libc::c_int) { return 0 as libc::c_int }
    saddr.sin_family = 2 as libc::c_int as sa_family_t;
    saddr.sin_port = 0 as libc::c_int as in_port_t;
    saddr.sin_addr = addr;
    memset(&mut packet.icmp as *mut icmp as *mut libc::c_void,
           0 as libc::c_int, ::std::mem::size_of::<icmp>() as libc::c_ulong);
    packet.icmp.icmp_type = 8 as libc::c_int as uint8_t;
    packet.icmp.icmp_hun.ih_idseq.icd_id = id;
    j = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<icmp>() as
                   libc::c_ulong).wrapping_div(2 as libc::c_int as
                                                   libc::c_ulong) {
        j =
            j.wrapping_add(*(&mut packet.icmp as *mut icmp as
                                 *mut u16_0).offset(i as isize) as
                               libc::c_uint);
        i = i.wrapping_add(1)
    }
    while j >> 16 as libc::c_int != 0 {
        j =
            (j &
                 0xffff as libc::c_int as
                     libc::c_uint).wrapping_add(j >> 16 as libc::c_int)
    }
    packet.icmp.icmp_cksum =
        if j == 0xffff as libc::c_int as libc::c_uint { j } else { !j } as
            uint16_t;
    while retry_send(sendto(fd,
                            &mut packet.icmp as *mut icmp as *mut libc::c_char
                                as *const libc::c_void,
                            ::std::mem::size_of::<icmp>() as libc::c_ulong,
                            0 as libc::c_int,
                            __CONST_SOCKADDR_ARG{__sockaddr__:
                                                     &mut saddr as
                                                         *mut sockaddr_in as
                                                         *mut sockaddr,},
                            ::std::mem::size_of::<sockaddr_in>() as
                                libc::c_ulong as socklen_t)) != 0 {
    }
    gotreply =
        delay_dhcp(dnsmasq_time(), 3 as libc::c_int, fd, addr.s_addr, id);
    close(fd);
    return gotreply;
}
#[no_mangle]
pub unsafe extern "C" fn delay_dhcp(mut start: time_t, mut sec: libc::c_int,
                                    mut fd: libc::c_int, mut addr: uint32_t,
                                    mut id: libc::c_ushort) -> libc::c_int {
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
    let mut rc: libc::c_int = 0;
    let mut timeout_count: libc::c_int = 0;
    let mut now: time_t = 0;
    now = dnsmasq_time();
    timeout_count = 0 as libc::c_int;
    while difftime(now, start) <= sec as libc::c_float as libc::c_double &&
              timeout_count < sec * 4 as libc::c_int {
        poll_reset();
        if fd != -(1 as libc::c_int) {
            poll_listen(fd, 0x1 as libc::c_int as libc::c_short);
        }
        set_dns_listeners(now);
        set_log_writer();
        if daemon.doing_ra != 0 {
            poll_listen(daemon.icmp6fd,
                        0x1 as libc::c_int as libc::c_short);
        }
        rc = do_poll(250 as libc::c_int);
        if rc < 0 as libc::c_int { continue ; }
        if rc == 0 as libc::c_int { timeout_count += 1 }
        now = dnsmasq_time();
        check_log_writer(0 as libc::c_int);
        check_dns_listeners(now);
        if daemon.doing_ra != 0 &&
               poll_check(daemon.icmp6fd,
                          0x1 as libc::c_int as libc::c_short) != 0 {
            icmp6_packet(now);
        }
        check_tftp_listeners(now);
        if fd != -(1 as libc::c_int) {
            let mut packet: C2RustUnnamed_26 =
                C2RustUnnamed_26{ip:
                                     ip{ip_hl_ip_v: [0; 1],
                                        ip_tos: 0,
                                        ip_len: 0,
                                        ip_id: 0,
                                        ip_off: 0,
                                        ip_ttl: 0,
                                        ip_p: 0,
                                        ip_sum: 0,
                                        ip_src: in_addr{s_addr: 0,},
                                        ip_dst: in_addr{s_addr: 0,},},
                                 icmp:
                                     icmp{icmp_type: 0,
                                          icmp_code: 0,
                                          icmp_cksum: 0,
                                          icmp_hun:
                                              C2RustUnnamed_17{ih_pptr: 0,},
                                          icmp_dun:
                                              C2RustUnnamed_14{id_ts:
                                                                   C2RustUnnamed_16{its_otime:
                                                                                        0,
                                                                                    its_rtime:
                                                                                        0,
                                                                                    its_ttime:
                                                                                        0,},},},};
            let mut faddr: sockaddr_in =
                sockaddr_in{sin_family: 0,
                            sin_port: 0,
                            sin_addr: in_addr{s_addr: 0,},
                            sin_zero: [0; 8],};
            let mut len: socklen_t =
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                    socklen_t;
            if poll_check(fd, 0x1 as libc::c_int as libc::c_short) != 0 &&
                   recvfrom(fd,
                            &mut packet as *mut C2RustUnnamed_26 as
                                *mut libc::c_void,
                            ::std::mem::size_of::<C2RustUnnamed_26>() as
                                libc::c_ulong, 0 as libc::c_int,
                            __SOCKADDR_ARG{__sockaddr__:
                                               &mut faddr as *mut sockaddr_in
                                                   as *mut sockaddr,},
                            &mut len) as libc::c_ulong ==
                       ::std::mem::size_of::<C2RustUnnamed_26>() as
                           libc::c_ulong && addr == faddr.sin_addr.s_addr &&
                   packet.icmp.icmp_type as libc::c_int == 0 as libc::c_int &&
                   packet.icmp.icmp_hun.ih_idseq.icd_seq as libc::c_int ==
                       0 as libc::c_int &&
                   packet.icmp.icmp_hun.ih_idseq.icd_id as libc::c_int ==
                       id as libc::c_int {
                return 1 as libc::c_int
            }
        }
    }
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
/* HAVE_DHCP */
