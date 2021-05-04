use std::{mem, net, time};

use winapi::shared::ws2def::AF_INET;

use crate::{
    dhcp_protocol::DHCP_CHADDR_MAX,
    dnsmasq_h::{dhcp_lease, DnsmasqDaemon, ACTION_TFTP, ARPHRD_ETHER},
    util::close_fds,
};

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

/* This file has code to fork a helper process which receives data via a pipe
   shared with the main process and which is responsible for calling a script when
   DHCP leases change.

   The helper process is forked before the main process drops root, so it retains root
   privs to pass on to the script. For this reason it tries to be paranoid about
   data received from the main process, in case that has been compromised. We don't
   want the helper to give an attacker root. In particular, the script to be run is
   not settable via the pipe, once the fork has taken place it is not alterable by the
   main process.
*/

// lua_State *lua;

pub struct script_data {
    pub flags: i32,
    pub action: i32,
    pub hwaddr_len: i32,
    pub hwaddr_type: i32,
    pub clid_len: i32,
    pub hostname_len: i32,
    pub ed_len: i32,
    pub addr: net::IpAddr,
    pub giaddr: net::IpAddr,
    pub remaining_time: u32,
    pub length: u32,
    pub expires: time::Instant,
    pub file_len: usize,
    pub addr6: net::IpAddr,
    pub vendorclass_count: i32,
    pub iaid: u32,
    pub hwaddr: [u8; DHCP_CHADDR_MAX],
    pub interface: String,
}

//  struct script_data *buf = NULL;
//  bytes_in_buf: usize = 0, buf_size = 0;

pub fn create_helper(
    daemon: &mut DnsmasqDaemon,
    event_fd: i32,
    err_fd: i32,
    uid: u32,
    gid: u32,
    max_fd: libc::c_long,
) -> i32 {
    let mut pid: u32;
    //i: i32, pipefd[2];
    let mut i: i32;
    let mut pipefd: [i32; 2];
    //struct sigaction sigact;
    // let mut sigact: sigaction;
    //let mut alloc_buff: *mut u8 = NULL;
    let mut alloc_buf: Vec<u8>;
    /* create the pipe through which the main program sends us commands,
    then fork our process. */
    // TODO: re-implement
    // if (pipe(pipefd) == -1 || !fix_fd(pipefd[1]) || (pid = fork()) == -1) {
    //     send_event(err_fd, EVENT_PIPE_ERR, errno, NULL);
    //     _exit(0);
    // }

    // TODO: re-implement
    // if (pid != 0) {
    //     close(pipefd[0]); /* close reader side */
    //     return pipefd[1];
    // }

    /* ignore SIGTERM and SIGINT, so that we can clean up when the main process gets hit
    and SIGALRM so that we can use sleep() */
    // TODO: re-implement
    // sigact.sa_handler = SIG_IGN;
    // sigact.sa_flags = 0;
    // sigemptyset(&sigact.sa_mask);
    // sigaction(SIGTERM, &sigact, NULL);
    // sigaction(SIGALRM, &sigact, NULL);
    // sigaction(SIGINT, &sigact, NULL);

    // if (!daemon.opt_debug && uid != 0) {
    //     //   gid_t dummy;
    //     let mut dummy: gid_t;
    //     if (setgroups(0, &dummy) == -1 || setgid(gid) == -1 || setuid(uid) == -1) {
    //         if (daemon.opt_no_fork) {
    //             /* send error to daemon process if no-fork */
    //             send_event(event_fd, EVENT_USER_ERR, errno, daemon.scriptuser);
    //         } else {
    //             /* kill daemon */
    //             send_event(event_fd, EVENT_DIE, 0, NULL);
    //             /* return error */
    //             send_event(err_fd, EVENT_USER_ERR, errno, daemon.scriptuser);
    //         }
    //         _exit(0);
    //     }
    // }

    /* close all the sockets etc, we don't need them here.
    Don't close err_fd, in case the lua-init fails.
    Note that we have to do this before lua init
    so we don't close any lua fds. */
    close_fds(max_fd, pipefd[0], event_fd, err_fd);

    // if (daemon.luascript) {
    //   const char *lua_err = NULL;
    // let mut lua_err: String;
    // let lua = lua_open();
    // luaL_openlibs(lua);

    /* get Lua to load our script file */
    // if (luaL_dofile(lua, daemon.luascript) != 0) {
    //     lua_err = lua_tostring(lua, -1);
    // } else {
    //     lua_getglobal(lua, "lease");
    //     if (lua_type(lua, -1) != LUA_TFUNCTION) {
    //         lua_err = String::from("lease() function missing in Lua script");
    //     }
    // }

    // if (lua_err) {
    //     if (daemon.opt_no_fork || daemon.opt_debug) {
    //         /* send error to daemon process if no-fork */
    //         send_event(event_fd, EVENT_LUA_ERR, 0, lua_err);
    //     } else {
    //         /* kill daemon */
    //         send_event(event_fd, EVENT_DIE, 0, NULL);
    //         /* return error */
    //         send_event(err_fd, EVENT_LUA_ERR, 0, lua_err);
    //     }
    //     _exit(0);
    // }

    // lua_pop(lua, 1); /* remove nil from stack */
    // lua_getglobal(lua, "init");
    // if (lua_type(lua, -1) == LUA_TFUNCTION) {
    //     lua_call(lua, 0, 0);
    // } else {
    //     lua_pop(lua, 1); /* remove nil from stack */
    // }
    // }

    /* All init done, close our copy of the error pipe, so that main process can return */
    // if (err_fd != -1) {
    //     close(err_fd);
    // }

    /* loop here */
    // loop {
    //     let mut data: script_data;
    //     //   p: &mut String, *action_str, *hostname = NULL, *domain = NULL;
    //     let p: String;
    //     let action_str: String;
    //     let hostname: String;
    //     let domain: String;
    //     // unsigned char *buf = daemon.namebuff;
    //     let buf = daemon.namebuff;
    //     let mut end: Vec<u8>;
    //     let mut extradata: Vec<u8>;
    //     //   is6: i32, err = 0;
    //     let mut is6: i32;
    //     let mut err: i32;
    //     // int pipeout[2];
    //     let mut pipeout: [i32; 2];

    /* Free rarely-allocated memory from previous iteration. */
    // if (alloc_buff) {
    //     free(alloc_buff);
    //     alloc_buff = NULL;
    // }

    /* we read zero bytes when pipe closed: this is our signal to exit */
    // if (!read_write(pipefd[0], &data, sizeof(data), 1)) {
    //     if (daemon.luascript) {
    //         lua_getglobal(lua, "shutdown");
    //         if (lua_type(lua, -1) == LUA_TFUNCTION) {
    //             lua_call(lua, 0, 0);
    //         }
    //     }

    //     _exit(0);
    // }

    //     is6 = !!(data.flags & (LEASE_TA | LEASE_NA));

    //     if (data.action == ACTION_DEL) {
    //         action_str = "del";
    //     } else if (data.action == ACTION_ADD) {
    //         action_str = "add";
    //     } else if (data.action == ACTION_OLD || data.action == ACTION_OLD_HOSTNAME) {
    //         action_str = "old";
    //     } else if (data.action == ACTION_TFTP) {
    //         action_str = "tftp";
    //         is6 = (data.flags != AF_INET);
    //     } else if (data.action == ACTION_ARP) {
    //         action_str = "arp-add";
    //         is6 = (data.flags != AF_INET);
    //     } else if (data.action == ACTION_ARP_DEL) {
    //         action_str = "arp-del";
    //         is6 = (data.flags != AF_INET);
    //         data.action = ACTION_ARP;
    //     } else {
    //         continue;
    //     }

    //     /* stringify MAC into dhcp_buff */
    //     p = daemon.dhcp_buff;
    //     if (data.hwaddr_type != ARPHRD_ETHER || data.hwaddr_len == 0) {
    //         p += sprintf(p, "%.2x-", data.hwaddr_type);
    //     }
    //     let mut i = 0;
    //     //   for (i = 0; (i < data.hwaddr_len) && (i < DHCP_CHADDR_MAX); i++)
    //     while (i < data.hwaddr_len) && (i < DHCP_CHADDR_MAX) {
    //         p += sprintf(p, "%.2x", data.hwaddr[i]);
    //         if (i != data.hwaddr_len - 1) {
    //             p += sprintf(p, ":");
    //         }
    //         i += 1;
    //     }

    //     /* supplied data may just exceed normal buffer (unlikely) */
    //     if ((data.hostname_len + data.ed_len + data.clid_len) > MAXDNAME
    //         && !(alloc_buff = buf = malloc(data.hostname_len + data.ed_len + data.clid_len)))
    //     {
    //         continue;
    //     }

    //     if (!read_write(
    //         pipefd[0],
    //         buf,
    //         data.hostname_len + data.ed_len + data.clid_len,
    //         1,
    //     )) {
    //         continue;
    //     }

    //     /* CLID into packet */
    //     //   for (p = daemon.packet, i = 0; i < data.clid_len; i++)

    //     p = daemon.packet;
    //     for i in 0..data.clid_len {
    //         p += sprintf(p, "%.2x", buf[i]);
    //         if (i != data.clid_len - 1) {
    //             p += sprintf(p, ":");
    //         }
    //     }

    //     if (is6) {
    //         /* or IAID and server DUID for IPv6 */
    //         //   sprintf(daemon.dhcp_buff3, "{}{}", data.flags & LEASE_TA ? "T" : "", data.iaid);
    //         daemon.dhcp_buff3 = format!(
    //             "{}{}",
    //             if data.flags & LEASE_TA { "T" } else { "" },
    //             data.iaid
    //         );

    //         p = daemon.dhcp_packet.iov_base;
    //         //   for (p = daemon.dhcp_packet.iov_base, i = 0; i < daemon.duid_len; i++)
    //         for i in 0..daemon.duid_len {
    //             p += sprintf(p, "%.2x", daemon.duid[i]);
    //             if (i != daemon.duid_len - 1) {
    //                 p += sprintf(p, ":");
    //             }
    //         }
    //     }

    //     buf += data.clid_len;

    //     if (data.hostname_len != 0) {
    //         char * dot;
    //         hostname = buf;
    //         hostname[data.hostname_len - 1] = 0;
    //         if (data.action != ACTION_TFTP) {
    //             if (!legal_hostname(hostname)) {
    //                 hostname = NULL;
    //             } else if (dot = strchr(hostname, '.')) {
    //                 domain = dot + 1;
    //                 *dot = 0;
    //             }
    //         }
    //     }

    //     extradata = buf + data.hostname_len;

    //     if (!is6) {
    //         inet_ntop(AF_INET, &data.addr, daemon.addrbuff, ADDRSTRLEN);
    //     } else {
    //         inet_ntop(AF_INET6, &data.addr6, daemon.addrbuff, ADDRSTRLEN);
    //     }

    //     /* file length */
    //     if (data.action == ACTION_TFTP) {
    //         // sprintf(is6 ? daemon.packet : daemon.dhcp_buff, "{}", data.file_len);
    //         if (is6) {
    //             daemon.packet = format!("{}", data.file_len);
    //         } else {
    //             daemon.dhcp_buff = format!("{}", data.file_len);
    //         }
    //     }

    //     if (daemon.luascript) {
    //         if (data.action == ACTION_TFTP) {
    //             lua_getglobal(lua, "tftp");
    //             if (lua_type(lua, -1) != LUA_TFUNCTION) {
    //                 lua_pop(lua, 1); /* tftp function optional */
    //             } else {
    //                 lua_pushstring(lua, action_str); /* arg1 - action */
    //                 lua_newtable(lua); /* arg2 - data table */
    //                 lua_pushstring(lua, daemon.addrbuff);
    //                 lua_setfield(lua, -2, "destination_address");
    //                 lua_pushstring(lua, hostname);
    //                 lua_setfield(lua, -2, "file_name");
    //                 lua_pushstring(lua, if is6 { daemon.packet } else { daemon.dhcp_buff });
    //                 lua_setfield(lua, -2, "file_size");
    //                 lua_call(lua, 2, 0); /* pass 2 values, expect 0 */
    //             }
    //         } else if (data.action == ACTION_ARP) {
    //             lua_getglobal(lua, "arp");
    //             if (lua_type(lua, -1) != LUA_TFUNCTION) {
    //                 lua_pop(lua, 1); /* arp function optional */
    //             } else {
    //                 lua_pushstring(lua, action_str); /* arg1 - action */
    //                 lua_newtable(lua); /* arg2 - data table */
    //                 lua_pushstring(lua, daemon.addrbuff);
    //                 lua_setfield(lua, -2, "client_address");
    //                 lua_pushstring(lua, daemon.dhcp_buff);
    //                 lua_setfield(lua, -2, "mac_address");
    //                 lua_call(lua, 2, 0); /* pass 2 values, expect 0 */
    //             }
    //         } else {
    //             lua_getglobal(lua, "lease"); /* function to call */
    //             lua_pushstring(lua, action_str); /* arg1 - action */
    //             lua_newtable(lua); /* arg2 - data table */

    //             if (is6) {
    //                 lua_pushstring(lua, daemon.packet);
    //                 lua_setfield(lua, -2, "client_duid");
    //                 lua_pushstring(lua, daemon.dhcp_packet.iov_base);
    //                 lua_setfield(lua, -2, "server_duid");
    //                 lua_pushstring(lua, daemon.dhcp_buff3);
    //                 lua_setfield(lua, -2, "iaid");
    //             }

    //             if (!is6 && data.clid_len != 0) {
    //                 lua_pushstring(lua, daemon.packet);
    //                 lua_setfield(lua, -2, "client_id");
    //             }

    //             if (strlen(data.interface) != 0) {
    //                 lua_pushstring(lua, data.interface);
    //                 lua_setfield(lua, -2, "interface");
    //             }

    //             lua_pushnumber(lua, data.length);
    //             lua_setfield(lua, -2, "lease_length");

    //             lua_pushnumber(lua, data.expires);
    //             lua_setfield(lua, -2, "lease_expires");

    //             if (hostname) {
    //                 lua_pushstring(lua, hostname);
    //                 lua_setfield(lua, -2, "hostname");
    //             }

    //             if (domain) {
    //                 lua_pushstring(lua, domain);
    //                 lua_setfield(lua, -2, "domain");
    //             }

    //             end = extradata + data.ed_len;
    //             buf = extradata;

    //             if (!is6) {
    //                 buf = grab_extradata_lua(buf, end, "vendor_class");
    //             } else if (data.vendorclass_count != 0) {
    //                 sprintf(daemon.dhcp_buff2, "vendor_class_id");
    //                 buf = grab_extradata_lua(buf, end, daemon.dhcp_buff2);
    //                 //   for (i = 0; i < data.vendorclass_count - 1; i++)
    //                 for i in 0..(data.vendorclass_count - 1) {
    //                     sprintf(daemon.dhcp_buff2, "vendor_class%i", i);
    //                     buf = grab_extradata_lua(buf, end, daemon.dhcp_buff2);
    //                 }
    //             }

    //             buf = grab_extradata_lua(buf, end, "supplied_hostname");

    //             if (!is6) {
    //                 buf = grab_extradata_lua(buf, end, "cpewan_oui");
    //                 buf = grab_extradata_lua(buf, end, "cpewan_serial");
    //                 buf = grab_extradata_lua(buf, end, "cpewan_class");
    //                 buf = grab_extradata_lua(buf, end, "circuit_id");
    //                 buf = grab_extradata_lua(buf, end, "subscriber_id");
    //                 buf = grab_extradata_lua(buf, end, "remote_id");
    //             }

    //             buf = grab_extradata_lua(buf, end, "tags");

    //             if (is6) {
    //                 buf = grab_extradata_lua(buf, end, "relay_address");
    //             } else if (data.giaddr.s_addr != 0) {
    //                 lua_pushstring(lua, inet_ntoa(data.giaddr));
    //                 lua_setfield(lua, -2, "relay_address");
    //             }

    //             //   for (i = 0; buf; i++)
    //             i = 0;
    //             while (buf) {
    //                 sprintf(daemon.dhcp_buff2, "user_class%i", i);
    //                 buf = grab_extradata_lua(buf, end, daemon.dhcp_buff2);
    //                 i += 1;
    //             }

    //             if (data.action != ACTION_DEL && data.remaining_time != 0) {
    //                 lua_pushnumber(lua, data.remaining_time);
    //                 lua_setfield(lua, -2, "time_remaining");
    //             }

    //             if (data.action == ACTION_OLD_HOSTNAME && hostname) {
    //                 lua_pushstring(lua, hostname);
    //                 lua_setfield(lua, -2, "old_hostname");
    //             }

    //             if (!is6 || data.hwaddr_len != 0) {
    //                 lua_pushstring(lua, daemon.dhcp_buff);
    //                 lua_setfield(lua, -2, "mac_address");
    //             }

    //             lua_pushstring(lua, daemon.addrbuff);
    //             lua_setfield(lua, -2, "ip_address");

    //             lua_call(lua, 2, 0); /* pass 2 values, expect 0 */
    //         }
    //     }

    //     /* no script, just lua */
    //     if (!daemon.lease_change_command) {
    //         continue;
    //     }

    //     /* Pipe to capture stdout and stderr from script */
    //     if (!daemon.opt_debug && pipe(pipeout) == -1) {
    //         continue;
    //     }

    //     /* possible fork errors are all temporary resource problems */
    //     while ((pid = fork()) == -1 && (errno == EAGAIN || errno == ENOMEM)) {
    //         sleep(2);
    //     }

    //     if (pid == -1) {
    //         if (!daemon.opt_debug) {
    //             close(pipeout[0]);
    //             close(pipeout[1]);
    //         }
    //         continue;
    //     }

    //     /* wait for child to complete */
    //     if (pid != 0) {
    //         if (!daemon.opt_debug) {
    //             FILE * fp;

    //             close(pipeout[1]);

    //             /* Read lines sent to stdout/err by the script and pass them back to be logged */
    //             if (!(fp = fdopen(pipeout[0], "r"))) {
    //                 close(pipeout[0]);
    //             } else {
    //                 while (fgets(daemon.packet, daemon.packet_buff_sz, fp)) {
    //                     /* do not include new lines, log will append them */
    //                     let len: usize = strlen(daemon.packet);
    //                     if (len > 0) {
    //                         --len;
    //                         if (daemon.packet[len] == '\n') {
    //                             daemon.packet[len] = 0;
    //                         }
    //                     }
    //                     send_event(event_fd, EVENT_SCRIPT_LOG, 0, daemon.packet);
    //                 }
    //                 fclose(fp);
    //             }
    //         }

    //         /* reap our children's children, if necessary */
    //         while (1) {
    //             let mut status: i32;
    //             let rc = wait(&status);

    //             if (rc == pid) {
    //                 /* On error send event back to main process for logging */
    //                 if (WIFSIGNALED(status)) {
    //                     send_event(event_fd, EVENT_KILLED, WTERMSIG(status), NULL);
    //                 } else if (WIFEXITED(status) && WEXITSTATUS(status) != 0) {
    //                     send_event(event_fd, EVENT_EXITED, WEXITSTATUS(status), NULL);
    //                 }
    //                 break;
    //             }

    //             if (rc == -1 && errno != EINTR) {
    //                 break;
    //             }
    //         }

    //         continue;
    //     }

    //     if (!daemon.opt_debug) {
    //         /* map stdout/stderr of script to pipeout */
    //         close(pipeout[0]);
    //         dup2(pipeout[1], STDOUT_FILENO);
    //         dup2(pipeout[1], STDERR_FILENO);
    //         close(pipeout[1]);
    //     }

    //     if (data.action != ACTION_TFTP && data.action != ACTION_ARP) {
    //         my_setenv(
    //             "DNSMASQ_IAID",
    //             if is6 { daemon.dhcp_buff3 } else { NULL },
    //             &err,
    //         );
    //         my_setenv(
    //             "DNSMASQ_SERVER_DUID",
    //             if is6 {
    //                 daemon.dhcp_packet.iov_base
    //             } else {
    //                 NULL
    //             },
    //             &err,
    //         );
    //         my_setenv(
    //             "DNSMASQ_MAC",
    //             if is6 && data.hwaddr_len != 0 {
    //                 daemon.dhcp_buff
    //             } else {
    //                 NULL
    //             },
    //             &err,
    //         );

    //         my_setenv(
    //             "DNSMASQ_CLIENT_ID",
    //             if !is6 && data.clid_len != 0 {
    //                 daemon.packet
    //             } else {
    //                 NULL
    //             },
    //             &err,
    //         );
    //         my_setenv(
    //             "DNSMASQ_INTERFACE",
    //             if strlen(data.interface) != 0 {
    //                 data.interface
    //             } else {
    //                 NULL
    //             },
    //             &err,
    //         );

    //         sprintf(daemon.dhcp_buff2, "{}", data.length);
    //         my_setenv("DNSMASQ_LEASE_LENGTH", daemon.dhcp_buff2, &err);

    //         sprintf(daemon.dhcp_buff2, "{}", data.expires);
    //         my_setenv("DNSMASQ_LEASE_EXPIRES", daemon.dhcp_buff2, &err);

    //         my_setenv("DNSMASQ_DOMAIN", domain, &err);

    //         end = extradata + data.ed_len;
    //         buf = extradata;

    //         if (!is6) {
    //             buf = grab_extradata(buf, end, "DNSMASQ_VENDOR_CLASS", &err);
    //         } else {
    //             if (data.vendorclass_count != 0) {
    //                 buf = grab_extradata(buf, end, "DNSMASQ_VENDOR_CLASS_ID", &err);
    //                 //   for (i = 0; i < data.vendorclass_count - 1; i++)
    //                 for i in 0..data.vendorclass_count {
    //                     sprintf(daemon.dhcp_buff2, "DNSMASQ_VENDOR_CLASS%i", i);
    //                     buf = grab_extradata(buf, end, daemon.dhcp_buff2, &err);
    //                 }
    //             }
    //         }

    //         buf = grab_extradata(buf, end, "DNSMASQ_SUPPLIED_HOSTNAME", &err);

    //         if (!is6) {
    //             buf = grab_extradata(buf, end, "DNSMASQ_CPEWAN_OUI", &err);
    //             buf = grab_extradata(buf, end, "DNSMASQ_CPEWAN_SERIAL", &err);
    //             buf = grab_extradata(buf, end, "DNSMASQ_CPEWAN_CLASS", &err);
    //             buf = grab_extradata(buf, end, "DNSMASQ_CIRCUIT_ID", &err);
    //             buf = grab_extradata(buf, end, "DNSMASQ_SUBSCRIBER_ID", &err);
    //             buf = grab_extradata(buf, end, "DNSMASQ_REMOTE_ID", &err);
    //             buf = grab_extradata(buf, end, "DNSMASQ_REQUESTED_OPTIONS", &err);
    //         }

    //         buf = grab_extradata(buf, end, "DNSMASQ_TAGS", &err);

    //         if (is6) {
    //             buf = grab_extradata(buf, end, "DNSMASQ_RELAY_ADDRESS", &err);
    //         } else {
    //             my_setenv(
    //                 "DNSMASQ_RELAY_ADDRESS",
    //                 if data.giaddr.s_addr != 0 {
    //                     inet_ntoa(data.giaddr)
    //                 } else {
    //                     NULL
    //                 },
    //                 &err,
    //             );
    //         }

    //         i = 0;
    //         //   for (i = 0; buf; i++)
    //         while (buff) {
    //             sprintf(daemon.dhcp_buff2, "DNSMASQ_USER_CLASS%i", i);
    //             buf = grab_extradata(buf, end, daemon.dhcp_buff2, &err);
    //         }

    //         sprintf(daemon.dhcp_buff2, "{}", data.remaining_time);
    //         my_setenv(
    //             "DNSMASQ_TIME_REMAINING",
    //             if data.action != ACTION_DEL && data.remaining_time != 0 {
    //                 daemon.dhcp_buff2
    //             } else {
    //                 NULL
    //             },
    //             &err,
    //         );

    //         my_setenv(
    //             "DNSMASQ_OLD_HOSTNAME",
    //             if data.action == ACTION_OLD_HOSTNAME {
    //                 hostname
    //             } else {
    //                 NULL
    //             },
    //             &err,
    //         );
    //         if (data.action == ACTION_OLD_HOSTNAME) {
    //             hostname = NULL;
    //         }

    //         my_setenv(
    //             "DNSMASQ_LOG_DHCP",
    //             if daemon.opt_log_opts { "1" } else { NULL },
    //             &err,
    //         );
    //     }

    //     /* we need to have the event_fd around if exec fails */
    //     if ((i = fcntl(event_fd, F_GETFD)) != -1) {
    //         fcntl(event_fd, F_SETFD, i | FD_CLOEXEC);
    //     }
    //     close(pipefd[0]);

    //     p = strrchr(daemon.lease_change_command, '/');
    //     if (err == 0) {
    //         execl(
    //             daemon.lease_change_command,
    //             if p {
    //                 p + 1
    //             } else {
    //                 daemon.lease_change_command
    //             },
    //             action_str,
    //             if is6 && (data.action != ACTION_ARP) {
    //                 daemon.packet
    //             } else {
    //                 daemon.dhcp_buff
    //             },
    //             daemon.addrbuff,
    //             hostname,
    //             None,
    //         );
    //         err = errno;
    //     }
    //     /* failed, send event so the main process logs the problem */
    //     send_event(event_fd, EVENT_EXEC_ERR, err, NULL);
    //     _exit(0);
    // }
}

pub fn my_setenv(name: &mut String, value: Option<&mut String>, error: &i32) {
    // if (*error == 0) {
    //     if value.is_none() {
    //         unsetenv(name);
    //     } else if (setenv(name, value, 1) != 0) {
    //     }
    // *error = errno;
    // }
    todo!()
}

pub fn grab_extradata(
    buf: &mut Vec<u8>,
    end: &mut Vec<u8>,
    env: &mut String,
    err: &i32,
) -> Vec<u8> {
    //   let mut next: *mut u8 = NULL;
    //   char *val = NULL;

    //   if (buf && (buf != end))
    //     {
    //       for (next = buf; ; next++)
    // 	if (next == end)
    // 	  {
    // 	    next = NULL;
    // 	    break;
    // 	  }
    // 	else if (*next == 0)
    // 	  break;

    //       if (next && (next != buf))
    // 	{
    // 	  char *p;
    // 	  /* No "=" in value */
    // 	  if ((p = strchr(buf, '=')))
    // 	    *p = 0;
    // 	  val = buf;
    // 	}
    //     }

    //   my_setenv(env, val, err);

    //   return next ? next + 1 : NULL;
    todo!()
}

pub fn grab_extradata_lua(buf: &mut Vec<u8>, end: &mut Vec<u8>, field: &mut String) -> Vec<u8> {
    //   let mut next: *mut u8;

    //   if (!buf || (buf == end))
    //     return NULL;

    //   for (next = buf; *next != 0; next++)
    //     if (next == end)
    //       return NULL;

    //   if (next != buf)
    //     {
    //       lua_pushstring(lua,  buf);
    //       lua_setfield(lua, -2, field);
    //     }

    //   return next + 1;
    todo!()
}

// pub fn buff_alloc(size: usize)
// {
//   if (size > buf_size)
//     {
//       let mut new: script_data;

//       /* start with reasonable size, will almost never need extending. */
//       if (size < sizeof(struct script_data) + 200)
// 	size = sizeof(struct script_data) + 200;

//       if (!(new = whine_malloc(size)))
// 	return;
//       if (buf)
// 	free(buf);
//       buf = new;
//       buf_size = size;
//     }
// }

/* pack up lease data into a buffer */
pub fn queue_script(
    daemon: &mut DnsmasqDaemon,
    buf: &mut script_data,
    bytes_in_buf: &mut usize,
    action: i32,
    lease: &mut dhcp_lease,
    hostname: &mut String,
    now: &time::Instant,
) {
    let mut p: *mut u8;
    //   unsigned int hostname_len = 0, clid_len = 0, ed_len = 0;
    let mut hostname_len: u32 = 0;
    let mut clid_len: u32 = 0;
    let mut ed_len: u32 = 0;
    // int fd = daemon.dhcpfd;
    let mut fd = daemon.dhcpfd;

    if !daemon.dhcp {
        fd = daemon.dhcp6fd;
    }

    /* no script */
    if daemon.helperfd == -1 {
        return;
    }

    if lease.extradata {
        ed_len = lease.extradata_len;
    }
    if lease.clid {
        clid_len = lease.clid_len;
    }
    if hostname {
        hostname_len = hostname.len() + 1;
    }

    //   buff_alloc(sizeof(struct script_data) +  clid_len + ed_len + hostname_len);

    buf.action = action;
    buf.flags = lease.flags;

    buf.vendorclass_count = lease.vendorclass_count;
    buf.addr6 = lease.addr6;
    buf.iaid = lease.iaid;

    buf.hwaddr_len = lease.hwaddr_len;
    buf.hwaddr_type = lease.hwaddr_type;
    buf.clid_len = clid_len;
    buf.ed_len = ed_len;
    buf.hostname_len = hostname_len;
    buf.addr = lease.addr;
    buf.giaddr = lease.giaddr;
    // TODO: copyt hwaddr with rust code
    // memcpy(buf.hwaddr, lease.hwaddr, DHCP_CHADDR_MAX);
    // if (!indextoname(fd, lease.last_interface, buf.interface)) {
    //     buf.interface[0] = 0;
    // }

    buf.length = lease.length;

    buf.expires = lease.expires;

    if lease.expires != 0 {
        // todo: replace difftime c func
        // buf.remaining_time = difftime(lease.expires, now);
    } else {
        buf.remaining_time = 0;
    }

    p = buf + 1;
    if clid_len != 0 {
        // memcpy(p, lease.clid, clid_len);
        p += clid_len;
    }
    if hostname_len != 0 {
        // memcpy(p, hostname, hostname_len);
        p += hostname_len;
    }
    if ed_len != 0 {
        // memcpy(p, lease.extradata, ed_len);
        p += ed_len;
    }
    bytes_in_buf = p - buf;
}

/* This nastily re-uses DHCP-fields for TFTP stuff */
pub fn queue_tftp(
    daemon: &mut DnsmasqDaemon,
    buf: &mut script_data,
    bytes_in_buf: &mut usize,
    file_len: usize,
    filename: &mut String,
    peer: &mut net::IpAddr,
) {
    let mut filename_len: u32;

    /* no script */
    if daemon.helperfd == -1 {
        return;
    }

    filename_len = filename.len() + 1;
    //   buff_alloc(sizeof(struct script_data) +  filename_len);
    //   memset(buf, 0, sizeof(struct script_data));

    buf.action = ACTION_TFTP;
    buf.hostname_len = filename_len;
    buf.file_len = file_len;

    if (buf.flags = peer.sa.sa_family) == AF_INET {
        buf.addr = peer.addr;
    } else {
        buf.addr6 = peer.in6.sin6_addr;
    }

    // memcpy((buf + 1), filename, filename_len);

    bytes_in_buf = mem::size_of::<script_data>() + filename_len;
}

pub fn queue_arp(
    daemon: &mut DnsmasqDaemon,
    action: i32,
    buf: &mut script_data,
    bytes_in_buf: &mut usize,
    mac: &mut Vec<u8>,
    maclen: i32,
    family: i32,
    addr: net::IpAddr,
) {
    /* no script */
    if daemon.helperfd == -1 {
        return;
    }

    //   buff_alloc(sizeof(struct script_data));
    //   memset(buf, 0, sizeof(struct script_data));

    buf.action = action;
    buf.hwaddr_len = maclen;
    buf.hwaddr_type = ARPHRD_ETHER;
    if (buf.flags = family) == AF_INET {
        buf.addr = addr.addr4;
    } else {
        buf.addr6 = addr.addr6;
    }

    // memcpy(buf.hwaddr, mac, maclen);

    bytes_in_buf = mem::size_of::<script_data>();
}

pub fn helper_buf_empty(bytes_in_buf: &usize) -> i32 {
    return bytes_in_buf == 0;
}

pub fn helper_write(daemon: &mut DnsmasqDaemon, buf: &mut script_data, bytes_in_buf: &mut usize) {
    let mut src: usize;
    todo!();

    // if bytes_in_buf == 0 {
    //     return;
    // }

    // if (rc = write(daemon.helperfd, buf, bytes_in_buf)) != -1 {
    //     if bytes_in_buf != rc {
    //         memmove(buf, buf + rc, bytes_in_buf - rc);
    //     }
    //     bytes_in_buf -= rc;
    // } else {
    //     if errno == EAGAIN || errno == EINTR {
    //         return;
    //     }
    //     bytes_in_buf = 0;
    // }
}
