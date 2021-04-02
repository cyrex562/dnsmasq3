use crate::defines::{DnsmasqDaemon, uid_t, gid_t, pid_t, Sigaction, C2rustUnnamed10, __sigset_t, C2rustUnnamed12, __sighandler_t, size_t, NetAddress, In6Addr, C2RustUnnamed, socklen_t, FILE, DhcpLease, time::Instant, off_t, NetAddress, ssize_t};
use crate::network::{fix_fd, indextoname};
use crate::send_event;
use crate::util::{close_fds, read_write, legal_hostname, whine_malloc};
use crate::slack::script_data;
use std::fmt::write;
use socket2::Socket;

// static mut buf: script_data;
// static mut bytes_in_buf: size_t = 0;
// static mut buf_size: size_t = 0;

pub fn create_helper(mut event_fd: i32,
                     mut err_fd: i32,
                     mut uid: uid_t,
                     mut gid: gid_t,
                     mut max_fd: i32) -> Socket
{
    let mut pid: pid_t = 0;
    let mut i: i32 = 0;
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    // let mut sigact: Sigaction = Sigaction {__sigaction_handler: C2rustUnnamed12 {sa_handler: None,},
    //               sa_mask: __sigset_t{__val: [0; 16],},
    //               sa_flags: 0,
    //               sa_restorer: None,};
    let mut alloc_buff: String;
    /* create the pipe through which the main program sends us commands,
     then fork our process. */
    if pipe(pipefd.as_mut_ptr()) == -(1 ) ||
           fix_fd(pipefd[1  ]) == 0 ||
           {
               pid = fork(); /* close reader side */
               (pid) == -(1 )
           } {
        send_event(err_fd, 10 , *__errno_location(), None);
        _exit(0 );
    }
    if pid != 0  {
        close(pipefd[0  ]);
        return pipefd[1  ]
    }
    /* ignore SIGTERM and SIGINT, so that we can clean up when the main process gets hit
     and SIGALRM so that we can use sleep() */
    // sigact.__sigaction_handler.sa_handler = ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1  as libc::intptr_t);
    // sigact.sa_flags = 0 ;
    // sigemptyset(&mut sigact.sa_mask);
    // sigaction(15 , &mut sigact, 0 as *mut Sigaction);
    // sigaction(14 , &mut sigact, 0 as *mut Sigaction);
    // sigaction(2 , &mut sigact, 0 as *mut Sigaction);
    if daemon.options[6] == false && uid != 0 {
        let mut dummy: gid_t = 0;
        if setgroups(0  , &mut dummy) == -1 || setgid(gid) == -1 || setuid(uid) == -1 {
            if daemon.options[16] != 0 {
                /* send error to daemon process if no-fork */
                send_event(event_fd, 11 , *__errno_location(), daemon.scriptuser);
            } else {
                /* kill daemon */
                send_event(event_fd, 16 , 0 , None);
                /* return error */
                send_event(err_fd, 11 , *__errno_location(), daemon.scriptuser);
            }
            _exit(0 );
        }
    }
    /* close all the sockets etc, we don't need them here. 
     Don't close err_fd, in case the lua-init fails.
     Note that we have to do this before lua init
     so we don't close any lua fds. */
    close_fds(max_fd, pipefd[0  ], event_fd, err_fd);
    /* All init done, close our copy of the error pipe, so that main process can return */
    if err_fd != -(1 ) { close(err_fd); }
    loop 
         /* loop here */
         {
        let mut data: script_data =
            script_data{flags: 0,
                        action: 0,
                        hwaddr_len: 0,
                        hwaddr_type: 0,
                        clid_len: 0,
                        hostname_len: 0,
                        ed_len: 0,
                        addr: NetAddress {s_addr: 0,},
                        giaddr: NetAddress {s_addr: 0,},
                        remaining_time: 0,
                        expires: 0,
                        file_len: 0,
                        addr6:
                            In6Addr {__in6_u:
                                         C2RustUnnamed{__u6_addr8:
                                                           [0; 16],},},
                        vendorclass_count: 0,
                        iaid: 0,
                        hwaddr: [0; 16],
                        interface: [0; 16],};
        let mut p: &mut String = 0 ;
        let mut action_str: &mut String = 0 ;
        let mut hostname: &mut String = 0 ;
        let mut domain: &mut String = 0 ;
        let mut buf_0: mut Vec<u8> =
            daemon.namebuff;
        let mut end: mut Vec<u8> = 0;
        let mut extradata: mut Vec<u8> = 0;
        let mut is6: i32 = 0;
        let mut err: i32 = 0 ;
        let mut pipeout: [libc::c_int; 2] = [0; 2];
        /* Free rarely-allocated memory from previous iteration. */
        // if !alloc_buff.is_null() {
        //     free(alloc_buff as *mut libc::c_void);
        //     alloc_buff = 0 as *mut libc::c_uchar
        // }
        /* we read zero bytes when pipe closed: this is our signal to exit */
        if read_write(pipefd[0  ],
                      &mut data ),
                      ::std::mem::size_of::<script_data>(), 1 ) == 0 {
            _exit(0 );
        }
        is6 =
            (data.flags & (64  | 32 ) != 0) ;
        if data.action == 1  {
            action_str =
                b"del\x00"  )
        } else if data.action == 4  {
            action_str =
                b"add\x00"  )
        } else if data.action == 3  ||
                      data.action == 2  {
            action_str =
                b"old\x00"  )
        } else if data.action == 5  {
            action_str =
                b"tftp\x00"  );
            is6 = (data.flags != 2 ) 
        } else if data.action == 6  {
            action_str =
                b"arp-add\x00"  );
            is6 = (data.flags != 2 ) 
        } else {
            if !(data.action == 7 ) { continue ; }
            action_str =
                b"arp-del\x00"  );
            is6 = (data.flags != 2 ) ;
            data.action = 6 
        }
        /* stringify MAC into dhcp_buff */
        p = daemon.dhcp_buff;
        if data.hwaddr_type != 1  || data.hwaddr_len == 0  {
            p = p.offset(sprintf(p, b"%.2x-\x00" , data.hwaddr_type) )
        }
        i = 0 ;
        while i < data.hwaddr_len && i < 16  {
            p = p.offset(sprintf(p, b"%.2x\x00" , data.hwaddr[i ] ) );
            if i != data.hwaddr_len - 1  {
                p =
                    p.offset(sprintf(p, b":\x00" ) )
            }
            i += 1
        }
        /* supplied data may just exceed normal buffer (unlikely) */
        if data.hostname_len + data.ed_len + data.clid_len > 1025  &&
               {
                   buf_0 =
                       malloc((data.hostname_len + data.ed_len +
                                   data.clid_len))                     mut Vec<u8>;
                   alloc_buff = buf_0;
                   alloc_buff.is_null()
               } {
            continue ;
        }
        if read_write(pipefd[0  ], buf_0,
                      data.hostname_len + data.ed_len + data.clid_len,
                      1 ) == 0 {
            continue ;
        }
        /* CLID into packet */
        p = daemon.packet;
        i = 0 ;
        while i < data.clid_len {
            p = p.offset(sprintf(p, b"%.2x\x00" , *buf_0.offset(i ) ) );
            if i != data.clid_len - 1  {
                p = p.offset(sprintf(p, b":\x00" ) )
            }
            i += 1
        }
        if is6 != 0 {
            /* or IAID and server DUID for IPv6 */
            sprintf(daemon.dhcp_buff3, b"%s%u\x00" , if data.flags & 64  != 0 { b"T\x00"  } else { b"\x00"  }, data.iaid);
            p = daemon.dhcp_packet.iov_base ;
            i = 0 ;
            while i < daemon.duid_len {
                p = p.offset(sprintf(p, b"%.2x\x00" , *daemon.duid.offset(i as  isize) ) );
                if i != daemon.duid_len - 1  {
                    p = p.offset(sprintf(p, b":\x00" ) )
                }
                i += 1
            }
        }
        buf_0 = buf_0.offset(data.clid_len );
        if data.hostname_len != 0  {
            let mut dot: &mut String = 0 ;
            hostname = buf_0 ;
            *hostname.offset((data.hostname_len - 1 ) )
                = 0 ;
            if data.action != 5  {
                if legal_hostname(hostname) == 0 {
                    hostname = 0
                } else {
                    dot = strchr(hostname, '.' as i32);
                    if !dot.is_null() {
                        domain = dot.offset(1  );
                        *dot = 0
                    }
                }
            }
        }
        extradata = buf_0.offset(data.hostname_len );
        if is6 == 0 {
            inet_ntop(2 ,
                      &mut data.addr,
                      daemon.addrbuff,
                      46 );
        } else {
            inet_ntop(10 ,
                      &mut data.addr6,
                      daemon.addrbuff,
                      46 );
        }
        /* file length */
        if data.action == 5  {
            sprintf(if is6 != 0 {
                        daemon.packet
                    } else { daemon.dhcp_buff },
                    b"%lu\x00" ,
                    data.file_len);
        }
        /* no script, just lua */
        if daemon.lease_change_command.is_null() { continue ; }
        /* Pipe to capture stdout and stderr from script */
        if daemon.options[6] == 0 && pipe(pipeout.as_mut_ptr()) == -1 {
            continue;
        }
        loop 
             /* possible fork errors are all temporary resource problems */
             {
            pid = fork();
            if !(pid == -(1 ) &&
                     (*__errno_location() == 11  ||
                          *__errno_location() == 12 )) {
                break ;
            }
            sleep(2 );
        }
        if pid == -(1 ) {
            if daemon.options[6] == 0 {
                close(pipeout[0  ]);
                close(pipeout[1  ]);
            }
        } else if pid != 0  {
            if daemon.options[6] == 0 {
                let mut fp: *mut FILE = 0 ;
                close(pipeout[1  ]);
                /* wait for child to complete */
                /* Read lines sent to stdout/err by the script and pass them back to be logged */
                fp = fdopen(pipeout[0  ],
                           b"r\x00" );
                if fp.is_null() {
                    close(pipeout[0  ]);
                } else {
                    while !fgets(daemon.packet,
                                 daemon.packet_buff_sz,
                                 fp).is_null() {
                        /* do not include new lines, log will append them */
                        let mut len: usize =
                            strlen(daemon.packet);
                        if len > 0  {
                            len = len.wrapping_sub(1);
                            if *daemon.packet.offset(len )
                                    == '\n' as i32 {
                                *daemon.packet.offset(len )
                                    = 0
                            }
                        }
                        send_event(event_fd, 25 ,
                                   0 ,
                                   daemon.packet);
                    }
                    fclose(fp);
                }
            }
            loop 
                 /* reap our children's children, if necessary */
                 {
                let mut status: i32 = 0;
                let mut rc: pid_t = wait(&mut status);
                if rc == pid {
                    /* On error send event back to main process for logging */
                    if ((status & 0x7f ) + 1 )                     libc::c_schar  >> 1  >
                           0  {
                        send_event(event_fd, 8, status & 0x7f, None);
                    } else if status & 0x7f == 0 && (status & 0xff00 ) >> 8 != 0  {
                        send_event(event_fd, 7 , (status & 0xff00 ) >> 8 , None);
                    }
                    break;
                } else if rc == -(1 ) && *__errno_location() != 4  {
                    break ;
                }
            }
        } else {
            if daemon.options[6] == 0 {
                /* map stdout/stderr of script to pipeout */
                close(pipeout[0]);
                dup2(pipeout[1], 1 );
                dup2(pipeout[1], 2 );
                close(pipeout[1]);
            }
            if data.action != 5  && data.action != 6  {
                my_setenv(b"DNSMASQ_IAID\x00" , if is6 != 0 {
                              daemon.dhcp_buff3
                          } else { 0  }, &mut err);
                my_setenv(b"DNSMASQ_SERVER_DUID\x00" ,
                          if is6 != 0 {
                              daemon.dhcp_packet.iov_base
                          } else { 0 }                        *const libc::c_char, &mut err);
                my_setenv(b"DNSMASQ_MAC\x00" ,
                          if is6 != 0 && data.hwaddr_len != 0  {
                              daemon.dhcp_buff
                          } else { 0  }, &mut err);
                my_setenv(b"DNSMASQ_CLIENT_ID\x00" ,
                          if is6 == 0 && data.clid_len != 0  {
                              daemon.packet
                          } else { 0  }, &mut err);
                my_setenv(b"DNSMASQ_INTERFACE\x00" , if strlen(data.interface.as_mut_ptr()) != 0  { data.interface.as_mut_ptr() } else { 0  }, &mut err);
                sprintf(daemon.dhcp_buff2, b"%lu\x00" , data.expires);
                my_setenv(b"DNSMASQ_LEASE_EXPIRES\x00" , daemon.dhcp_buff2, &mut err);
                my_setenv(b"DNSMASQ_DOMAIN\x00" , domain, &mut err);
                end = extradata.offset(data.ed_len );
                buf_0 = extradata;
                if is6 == 0 {
                    buf_0 = grab_extradata(buf_0, end, b"DNSMASQ_VENDOR_CLASS\x00"  , &mut err)
                } else if data.vendorclass_count != 0  {
                    buf_0 = grab_extradata(buf_0, end, b"DNSMASQ_VENDOR_CLASS_ID\x00"  , &mut err);
                    i = 0 ;
                    while i < data.vendorclass_count - 1  {
                        sprintf(daemon.dhcp_buff2, b"DNSMASQ_VENDOR_CLASS%i\x00" , i);
                        buf_0 = grab_extradata(buf_0, end, daemon.dhcp_buff2, &mut err);
                        i += 1
                    }
                }
                buf_0 =
                    grab_extradata(buf_0, end,
                                   b"DNSMASQ_SUPPLIED_HOSTNAME\x00"                                 *const u8                                 &mut String, &mut err);
                if is6 == 0 {
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_CPEWAN_OUI\x00"                                                  &mut String, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_CPEWAN_SERIAL\x00"                                     *const u8                                     &mut String, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_CPEWAN_CLASS\x00"                                     *const u8                                     &mut String, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_CIRCUIT_ID\x00"                                                  &mut String, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_SUBSCRIBER_ID\x00"                                     *const u8                                     &mut String, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_REMOTE_ID\x00"                                                  &mut String, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_REQUESTED_OPTIONS\x00"                                     *const u8                                     &mut String, &mut err)
                }
                buf_0 =
                    grab_extradata(buf_0, end,
                                   b"DNSMASQ_TAGS\x00", &mut err);
                if is6 != 0 {
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_RELAY_ADDRESS\x00"                                     *const u8                                     &mut String, &mut err)
                } else {
                    my_setenv(b"DNSMASQ_RELAY_ADDRESS\x00",
                              if data.giaddr.s_addr !=
                                     0  {
                                  inet_ntoa(data.giaddr)
                              } else { 0  }, &mut err);
                }
                i = 0 ;
                while !buf_0.is_null() {
                    sprintf(daemon.dhcp_buff2,
                            b"DNSMASQ_USER_CLASS%i\x00"                           *const libc::c_char, i);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       daemon.dhcp_buff2,
                                       &mut err);
                    i += 1
                }
                sprintf(daemon.dhcp_buff2,
                        b"%u\x00" ,
                        data.remaining_time);
                my_setenv(b"DNSMASQ_TIME_REMAINING\x00" ,
                          if data.action != 1  &&
                                 data.remaining_time !=
                                     0  {
                              daemon.dhcp_buff2
                          } else { 0  }, &mut err);
                my_setenv(b"DNSMASQ_OLD_HOSTNAME\x00" ,
                          if data.action == 2  {
                              hostname
                          } else { 0  }, &mut err);
                if data.action == 2  {
                    hostname = 0
                }
                my_setenv(b"DNSMASQ_LOG_DHCP\x00" ,
                          if daemon.options[(28   ).wrapping_div((::std::mem::size_of::<libc::c_uint>()  ).wrapping_mul(8                                                                         libc::c_int                                                                  ))
                                                           ] &
                                 (1) <<
                                     (28  ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                                           ).wrapping_mul(8                                     libc::c_int                              ))
                                 != 0 {
                              b"1\x00"
                          } else { 0 }, &mut err);
            }
            /* we need to have the event_fd around if exec fails */
            i = fcntl(event_fd, 1 );
            if i != -(1 ) {
                fcntl(event_fd, 2 , i | 1 );
            }
            close(pipefd[0  ]);
            p = strrchr(daemon.lease_change_command, '/' as i32);
            if err == 0  {
                execl(daemon.lease_change_command,
                      if !p.is_null() {
                          p.offset(1  )
                      } else { daemon.lease_change_command },
                      action_str,
                      if is6 != 0 && data.action != 6  {
                          daemon.packet
                      } else { daemon.dhcp_buff },
                      daemon.addrbuff, hostname,
                      0 );
                err = *__errno_location()
            }
            /* failed, send event so the main process logs the problem */
            send_event(event_fd, 9 , err,
                       0 );
            _exit(0 );
        }
    };
}
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
fn my_setenv(mut name: *const libc::c_char,
                               mut value: *const libc::c_char,
                               mut error: ) {
    if *error == 0  {
        if value.is_null() {
            unsetenv(name);
        } else if setenv(name, value, 1 ) != 0  {
            *error = *__errno_location()
        }
    };
}
fn grab_extradata(mut buf_0: mut Vec<u8>,
                                    mut end: mut Vec<u8>,
                                    mut env: &mut String,
                                    mut err: )
 -> mut Vec<u8> {
    let mut next: mut Vec<u8> = 0;
    let mut val: &mut String = 0 ;
    if !buf_0.is_null() && buf_0 != end {
        next = buf_0;
        loop  {
            if next == end {
                next = 0;
                break ;
            } else {
                if *next  == 0  { break ; }
                next = next.offset(1)
            }
        }
        if !next.is_null() && next != buf_0 {
            let mut p: &mut String = 0 ;
            /* No "=" in value */
            p = strchr(buf_0 , '=' as i32);
            if !p.is_null() { *p = 0  }
            val = buf_0
        }
    }
    my_setenv(env, val, err);
    return if !next.is_null() {
               next.offset(1  )
           } else { 0 };
}
fn buff_alloc(mut size: usize) {
    if size > buf_size {
        let mut new: *mut script_data = 0 );
        /* start with reasonable size, will almost never need extending. */
        if size <
               (::std::mem::size_of::<script_data>()).wrapping_add(200  libc::c_ulong) {
            size =
                (::std::mem::size_of::<script_data>()        ).wrapping_add(200   libc::c_ulong)
        }
        new = whine_malloc(size) );
        if new.is_null() { return }
        if !buf.is_null() { free(buf); }
        buf = new;
        buf_size = size
    };
}
/* pack up lease data into a buffer */

pub fn queue_script(mut action: i32,
                                      mut lease: DhcpLease,
                                      mut hostname: &mut String,
                                      mut now: time::Instant) {
    let mut p: mut Vec<u8> = 0;
    let mut hostname_len: u32 = 0 ;
    let mut clid_len: u32 = 0 ;
    let mut ed_len: u32 = 0 ;
    let mut fd: i32 = daemon.dhcpfd;
    if daemon.dhcp.is_null() { fd = daemon.dhcp6fd }
    /* no script */
    if daemon.helperfd == -(1 ) { return }
    if !(*lease).extradata.is_null() { ed_len = (*lease).extradata_len }
    if !(*lease).clid.is_null() {
        clid_len = (*lease).clid_len
    }
    if !hostname.is_null() {
        hostname_len =
            strlen(hostname).wrapping_add(1 )

    }
    buff_alloc((::std::mem::size_of::<script_data>()).wrapping_add(clid_len libc::c_ulong).wrapping_add(ed_len
                                                                                                             ).wrapping_add(hostname_len                                                ));
    (*buf).action = action;
    (*buf).flags = (*lease).flags;
    (*buf).vendorclass_count = (*lease).vendorclass_count;
    (*buf).addr6 = (*lease).addr6;
    (*buf).iaid = (*lease).iaid;
    (*buf).hwaddr_len = (*lease).hwaddr_len;
    (*buf).hwaddr_type = (*lease).hwaddr_type;
    (*buf).clid_len = clid_len ;
    (*buf).ed_len = ed_len ;
    (*buf).hostname_len = hostname_len ;
    (*buf).addr = (*lease).addr;
    (*buf).giaddr = (*lease).giaddr;
    memcpy((*buf).hwaddr.as_mut_ptr(),
           (*lease).hwaddr.as_mut_ptr(),
           16 );
    if indextoname(fd, (*lease).last_interface, (*buf).interface.as_mut_ptr())
           == 0 {
        (*buf).interface[0  ] =
            0
    }
    (*buf).expires = (*lease).expires;
    if (*lease).expires != 0  {
        (*buf).remaining_time =
            difftime((*lease).expires, now)
    } else { (*buf).remaining_time = 0  }
    p = buf.offset(1  );
    if clid_len != 0  {
        memcpy(p, (*lease).clid,
               clid_len);
        p = p.offset(clid_len )
    }
    if hostname_len != 0  {
        memcpy(p, hostname,
               hostname_len);
        p = p.offset(hostname_len )
    }
    if ed_len != 0  {
        memcpy(p,
               (*lease).extradata,
               ed_len);
        p = p.offset(ed_len )
    }
    bytes_in_buf =
        p.wrapping_offset_from(buf)      size_t;
}
/* This nastily re-uses DHCP-fields for TFTP stuff */

pub fn queue_tftp(mut file_len: off_t, mut filename: &mut String, mut peer: &mut NetAddress) {
    let mut filename_len: u32 = 0;
    /* no script */
    if daemon.helperfd == -(1 ) { return }
    (*buf).action = 5 ;
    (*buf).hostname_len = filename_len ;
    (*buf).file_len = file_len;
    (*buf).flags = (*peer).sa.sa_family ;
    if (*buf).flags == 2  {
        (*buf).addr = (*peer).in_0.sin_addr
    } else { (*buf).addr6 = (*peer).in6.sin6_addr }
    memcpy(buf.offset(1  )        Vec<u8>, filename,
           filename_len);
    bytes_in_buf =
        (::std::mem::size_of::<script_data>() as
      ).wrapping_add(filename_len);
}

pub fn queue_arp(daemon: &mut DnsmasqDaemon,
                 mut action: u32,
                 mut mac: &[u8;16],
                 mut family: u32,
                 mut addr: all_addr) {
    /* no script */
    if daemon.helperfd == -(1 ) { return }
    buff_alloc(::std::mem::size_of::<script_data>());
    (*buf).action = action;
    (*buf).hwaddr_len = maclen;
    (*buf).hwaddr_type = 1 ;
    (*buf).flags = family;
    if (*buf).flags == 2  {
        (*buf).addr = addr.addr4
    } else { (*buf).addr6 = addr.addr6 }
    memcpy((*buf).hwaddr.as_mut_ptr(),
           mac, maclen);
    bytes_in_buf = ::std::mem::size_of::<script_data>();
}

pub fn helper_buf_empty() -> i32 {
    return (bytes_in_buf == 0 ) ;
}

pub fn helper_write() {
    let mut rc: ssize_t = 0;
    if bytes_in_buf == 0  { return }
    rc =
        write(daemon.helperfd, buf,
              bytes_in_buf);
    if rc != -(1 ) {
        if bytes_in_buf != rc  {
            memmove(buf,
                    buf.offset(rc ),
                    bytes_in_buf.wrapping_sub(rc));
        }
        bytes_in_buf =
            (bytes_in_buf).wrapping_sub(rc)

    } else {
        if *__errno_location() == 11  ||
               *__errno_location() == 4  {
            return
        }
        bytes_in_buf = 0
    };
}
