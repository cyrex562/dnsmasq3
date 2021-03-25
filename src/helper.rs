use crate::defines::{dnsmasq_daemon, uid_t, gid_t, pid_t, sigaction, C2RustUnnamed_10, __sigset_t, C2RustUnnamed_12};
use crate::network::fix_fd;
use crate::send_event;
use crate::util::{close_fds, read_write};

// static mut buf: script_data;
// static mut bytes_in_buf: size_t = 0;
// static mut buf_size: size_t = 0;

pub fn create_helper(mut event_fd: libc::c_int,
                                       mut err_fd: i32,
                                       mut uid: uid_t,
                     mut gid: gid_t,
                                       mut max_fd: i32)
 -> i32 {
    let mut pid: pid_t = 0;
    let mut i: libc::c_int = 0;
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    let mut sigact: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_12{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    let mut alloc_buff: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* create the pipe through which the main program sends us commands,
     then fork our process. */
    if pipe(pipefd.as_mut_ptr()) == -(1 as libc::c_int) ||
           fix_fd(pipefd[1 as libc::c_int as usize]) == 0 ||
           {
               pid = fork(); /* close reader side */
               (pid) == -(1 as libc::c_int)
           } {
        send_event(err_fd, 10 as libc::c_int, *__errno_location(),
                   0 as *mut libc::c_char);
        _exit(0 as libc::c_int);
    }
    if pid != 0 as libc::c_int {
        close(pipefd[0 as libc::c_int as usize]);
        return pipefd[1 as libc::c_int as usize]
    }
    /* ignore SIGTERM and SIGINT, so that we can clean up when the main process gets hit
     and SIGALRM so that we can use sleep() */
    sigact.__sigaction_handler.sa_handler =
        ::std::mem::transmute::<libc::intptr_t,
                                __sighandler_t>(1 as libc::c_int as
                                                    libc::intptr_t);
    sigact.sa_flags = 0 as libc::c_int;
    sigemptyset(&mut sigact.sa_mask);
    sigaction(15 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    sigaction(14 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    sigaction(2 as libc::c_int, &mut sigact, 0 as *mut sigaction);
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
           == 0 && uid != 0 as libc::c_int as libc::c_uint {
        let mut dummy: gid_t = 0;
        if setgroups(0 as libc::c_int as size_t, &mut dummy) ==
               -(1 as libc::c_int) || setgid(gid) == -(1 as libc::c_int) ||
               setuid(uid) == -(1 as libc::c_int) {
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
                   != 0 {
                /* send error to daemon process if no-fork */
                send_event(event_fd, 11 as libc::c_int, *__errno_location(),
                           daemon.scriptuser);
            } else {
                /* kill daemon */
                send_event(event_fd, 16 as libc::c_int, 0 as libc::c_int,
                           0 as *mut libc::c_char);
                /* return error */
                send_event(err_fd, 11 as libc::c_int, *__errno_location(),
                           daemon.scriptuser);
            }
            _exit(0 as libc::c_int);
        }
    }
    /* close all the sockets etc, we don't need them here. 
     Don't close err_fd, in case the lua-init fails.
     Note that we have to do this before lua init
     so we don't close any lua fds. */
    close_fds(max_fd, pipefd[0 as libc::c_int as usize], event_fd, err_fd);
    /* All init done, close our copy of the error pipe, so that main process can return */
    if err_fd != -(1 as libc::c_int) { close(err_fd); }
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
                        addr: in_addr{s_addr: 0,},
                        giaddr: in_addr{s_addr: 0,},
                        remaining_time: 0,
                        expires: 0,
                        file_len: 0,
                        addr6:
                            in6_addr{__in6_u:
                                         C2RustUnnamed{__u6_addr8:
                                                           [0; 16],},},
                        vendorclass_count: 0,
                        iaid: 0,
                        hwaddr: [0; 16],
                        interface: [0; 16],};
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut action_str: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut buf_0: *mut libc::c_uchar =
            daemon.namebuff as *mut libc::c_uchar;
        let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut extradata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut is6: libc::c_int = 0;
        let mut err: libc::c_int = 0 as libc::c_int;
        let mut pipeout: [libc::c_int; 2] = [0; 2];
        /* Free rarely-allocated memory from previous iteration. */
        if !alloc_buff.is_null() {
            free(alloc_buff as *mut libc::c_void);
            alloc_buff = 0 as *mut libc::c_uchar
        }
        /* we read zero bytes when pipe closed: this is our signal to exit */
        if read_write(pipefd[0 as libc::c_int as usize],
                      &mut data as *mut script_data as *mut libc::c_uchar,
                      ::std::mem::size_of::<script_data>() as libc::c_ulong as
                          libc::c_int, 1 as libc::c_int) == 0 {
            _exit(0 as libc::c_int);
        }
        is6 =
            (data.flags & (64 as libc::c_int | 32 as libc::c_int) != 0) as
                libc::c_int;
        if data.action == 1 as libc::c_int {
            action_str =
                b"del\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else if data.action == 4 as libc::c_int {
            action_str =
                b"add\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else if data.action == 3 as libc::c_int ||
                      data.action == 2 as libc::c_int {
            action_str =
                b"old\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        } else if data.action == 5 as libc::c_int {
            action_str =
                b"tftp\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            is6 = (data.flags != 2 as libc::c_int) as libc::c_int
        } else if data.action == 6 as libc::c_int {
            action_str =
                b"arp-add\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            is6 = (data.flags != 2 as libc::c_int) as libc::c_int
        } else {
            if !(data.action == 7 as libc::c_int) { continue ; }
            action_str =
                b"arp-del\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            is6 = (data.flags != 2 as libc::c_int) as libc::c_int;
            data.action = 6 as libc::c_int
        }
        /* stringify MAC into dhcp_buff */
        p = daemon.dhcp_buff;
        if data.hwaddr_type != 1 as libc::c_int ||
               data.hwaddr_len == 0 as libc::c_int {
            p =
                p.offset(sprintf(p,
                                 b"%.2x-\x00" as *const u8 as
                                     *const libc::c_char, data.hwaddr_type) as
                             isize)
        }
        i = 0 as libc::c_int;
        while i < data.hwaddr_len && i < 16 as libc::c_int {
            p =
                p.offset(sprintf(p,
                                 b"%.2x\x00" as *const u8 as
                                     *const libc::c_char,
                                 data.hwaddr[i as usize] as libc::c_int) as
                             isize);
            if i != data.hwaddr_len - 1 as libc::c_int {
                p =
                    p.offset(sprintf(p,
                                     b":\x00" as *const u8 as
                                         *const libc::c_char) as isize)
            }
            i += 1
        }
        /* supplied data may just exceed normal buffer (unlikely) */
        if data.hostname_len + data.ed_len + data.clid_len >
               1025 as libc::c_int &&
               {
                   buf_0 =
                       malloc((data.hostname_len + data.ed_len +
                                   data.clid_len) as libc::c_ulong) as
                           *mut libc::c_uchar;
                   alloc_buff = buf_0;
                   alloc_buff.is_null()
               } {
            continue ;
        }
        if read_write(pipefd[0 as libc::c_int as usize], buf_0,
                      data.hostname_len + data.ed_len + data.clid_len,
                      1 as libc::c_int) == 0 {
            continue ;
        }
        /* CLID into packet */
        p = daemon.packet;
        i = 0 as libc::c_int;
        while i < data.clid_len {
            p =
                p.offset(sprintf(p,
                                 b"%.2x\x00" as *const u8 as
                                     *const libc::c_char,
                                 *buf_0.offset(i as isize) as libc::c_int) as
                             isize);
            if i != data.clid_len - 1 as libc::c_int {
                p =
                    p.offset(sprintf(p,
                                     b":\x00" as *const u8 as
                                         *const libc::c_char) as isize)
            }
            i += 1
        }
        if is6 != 0 {
            /* or IAID and server DUID for IPv6 */
            sprintf(daemon.dhcp_buff3,
                    b"%s%u\x00" as *const u8 as *const libc::c_char,
                    if data.flags & 64 as libc::c_int != 0 {
                        b"T\x00" as *const u8 as *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char },
                    data.iaid);
            p = daemon.dhcp_packet.iov_base as *mut libc::c_char;
            i = 0 as libc::c_int;
            while i < daemon.duid_len {
                p =
                    p.offset(sprintf(p,
                                     b"%.2x\x00" as *const u8 as
                                         *const libc::c_char,
                                     *daemon.duid.offset(i as
                                                                        isize)
                                         as libc::c_int) as isize);
                if i != daemon.duid_len - 1 as libc::c_int {
                    p =
                        p.offset(sprintf(p,
                                         b":\x00" as *const u8 as
                                             *const libc::c_char) as isize)
                }
                i += 1
            }
        }
        buf_0 = buf_0.offset(data.clid_len as isize);
        if data.hostname_len != 0 as libc::c_int {
            let mut dot: *mut libc::c_char = 0 as *mut libc::c_char;
            hostname = buf_0 as *mut libc::c_char;
            *hostname.offset((data.hostname_len - 1 as libc::c_int) as isize)
                = 0 as libc::c_int as libc::c_char;
            if data.action != 5 as libc::c_int {
                if legal_hostname(hostname) == 0 {
                    hostname = 0 as *mut libc::c_char
                } else {
                    dot = strchr(hostname, '.' as i32);
                    if !dot.is_null() {
                        domain = dot.offset(1 as libc::c_int as isize);
                        *dot = 0 as libc::c_int as libc::c_char
                    }
                }
            }
        }
        extradata = buf_0.offset(data.hostname_len as isize);
        if is6 == 0 {
            inet_ntop(2 as libc::c_int,
                      &mut data.addr as *mut in_addr as *const libc::c_void,
                      daemon.addrbuff,
                      46 as libc::c_int as socklen_t);
        } else {
            inet_ntop(10 as libc::c_int,
                      &mut data.addr6 as *mut in6_addr as *const libc::c_void,
                      daemon.addrbuff,
                      46 as libc::c_int as socklen_t);
        }
        /* file length */
        if data.action == 5 as libc::c_int {
            sprintf(if is6 != 0 {
                        daemon.packet
                    } else { daemon.dhcp_buff },
                    b"%lu\x00" as *const u8 as *const libc::c_char,
                    data.file_len as libc::c_ulong);
        }
        /* no script, just lua */
        if daemon.lease_change_command.is_null() { continue ; }
        /* Pipe to capture stdout and stderr from script */
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
               == 0 && pipe(pipeout.as_mut_ptr()) == -(1 as libc::c_int) {
            continue ;
        }
        loop 
             /* possible fork errors are all temporary resource problems */
             {
            pid = fork();
            if !(pid == -(1 as libc::c_int) &&
                     (*__errno_location() == 11 as libc::c_int ||
                          *__errno_location() == 12 as libc::c_int)) {
                break ;
            }
            sleep(2 as libc::c_int as libc::c_uint);
        }
        if pid == -(1 as libc::c_int) {
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
                close(pipeout[0 as libc::c_int as usize]);
                close(pipeout[1 as libc::c_int as usize]);
            }
        } else if pid != 0 as libc::c_int {
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
                let mut fp: *mut FILE = 0 as *mut FILE;
                close(pipeout[1 as libc::c_int as usize]);
                /* wait for child to complete */
                /* Read lines sent to stdout/err by the script and pass them back to be logged */
                fp =
                    fdopen(pipeout[0 as libc::c_int as usize],
                           b"r\x00" as *const u8 as *const libc::c_char);
                if fp.is_null() {
                    close(pipeout[0 as libc::c_int as usize]);
                } else {
                    while !fgets(daemon.packet,
                                 daemon.packet_buff_sz,
                                 fp).is_null() {
                        /* do not include new lines, log will append them */
                        let mut len: size_t =
                            strlen(daemon.packet);
                        if len > 0 as libc::c_int as libc::c_ulong {
                            len = len.wrapping_sub(1);
                            if *daemon.packet.offset(len as isize)
                                   as libc::c_int == '\n' as i32 {
                                *daemon.packet.offset(len as isize)
                                    = 0 as libc::c_int as libc::c_char
                            }
                        }
                        send_event(event_fd, 25 as libc::c_int,
                                   0 as libc::c_int,
                                   daemon.packet);
                    }
                    fclose(fp);
                }
            }
            loop 
                 /* reap our children's children, if necessary */
                 {
                let mut status: libc::c_int = 0;
                let mut rc: pid_t = wait(&mut status);
                if rc == pid {
                    /* On error send event back to main process for logging */
                    if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as
                           libc::c_schar as libc::c_int >> 1 as libc::c_int >
                           0 as libc::c_int {
                        send_event(event_fd, 8 as libc::c_int,
                                   status & 0x7f as libc::c_int,
                                   0 as *mut libc::c_char);
                    } else if status & 0x7f as libc::c_int == 0 as libc::c_int
                                  &&
                                  (status & 0xff00 as libc::c_int) >>
                                      8 as libc::c_int != 0 as libc::c_int {
                        send_event(event_fd, 7 as libc::c_int,
                                   (status & 0xff00 as libc::c_int) >>
                                       8 as libc::c_int,
                                   0 as *mut libc::c_char);
                    }
                    break ;
                } else if rc == -(1 as libc::c_int) &&
                              *__errno_location() != 4 as libc::c_int {
                    break ;
                }
            }
        } else {
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
                /* map stdout/stderr of script to pipeout */
                close(pipeout[0 as libc::c_int as usize]);
                dup2(pipeout[1 as libc::c_int as usize], 1 as libc::c_int);
                dup2(pipeout[1 as libc::c_int as usize], 2 as libc::c_int);
                close(pipeout[1 as libc::c_int as usize]);
            }
            if data.action != 5 as libc::c_int &&
                   data.action != 6 as libc::c_int {
                my_setenv(b"DNSMASQ_IAID\x00" as *const u8 as
                              *const libc::c_char,
                          if is6 != 0 {
                              daemon.dhcp_buff3
                          } else { 0 as *mut libc::c_char }, &mut err);
                my_setenv(b"DNSMASQ_SERVER_DUID\x00" as *const u8 as
                              *const libc::c_char,
                          if is6 != 0 {
                              daemon.dhcp_packet.iov_base
                          } else { 0 as *mut libc::c_void } as
                              *const libc::c_char, &mut err);
                my_setenv(b"DNSMASQ_MAC\x00" as *const u8 as
                              *const libc::c_char,
                          if is6 != 0 && data.hwaddr_len != 0 as libc::c_int {
                              daemon.dhcp_buff
                          } else { 0 as *mut libc::c_char }, &mut err);
                my_setenv(b"DNSMASQ_CLIENT_ID\x00" as *const u8 as
                              *const libc::c_char,
                          if is6 == 0 && data.clid_len != 0 as libc::c_int {
                              daemon.packet
                          } else { 0 as *mut libc::c_char }, &mut err);
                my_setenv(b"DNSMASQ_INTERFACE\x00" as *const u8 as
                              *const libc::c_char,
                          if strlen(data.interface.as_mut_ptr()) !=
                                 0 as libc::c_int as libc::c_ulong {
                              data.interface.as_mut_ptr()
                          } else { 0 as *mut libc::c_char }, &mut err);
                sprintf(daemon.dhcp_buff2,
                        b"%lu\x00" as *const u8 as *const libc::c_char,
                        data.expires as libc::c_ulong);
                my_setenv(b"DNSMASQ_LEASE_EXPIRES\x00" as *const u8 as
                              *const libc::c_char,
                          daemon.dhcp_buff2, &mut err);
                my_setenv(b"DNSMASQ_DOMAIN\x00" as *const u8 as
                              *const libc::c_char, domain, &mut err);
                end = extradata.offset(data.ed_len as isize);
                buf_0 = extradata;
                if is6 == 0 {
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_VENDOR_CLASS\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err)
                } else if data.vendorclass_count != 0 as libc::c_int {
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_VENDOR_CLASS_ID\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    i = 0 as libc::c_int;
                    while i < data.vendorclass_count - 1 as libc::c_int {
                        sprintf(daemon.dhcp_buff2,
                                b"DNSMASQ_VENDOR_CLASS%i\x00" as *const u8 as
                                    *const libc::c_char, i);
                        buf_0 =
                            grab_extradata(buf_0, end,
                                           daemon.dhcp_buff2,
                                           &mut err);
                        i += 1
                    }
                }
                buf_0 =
                    grab_extradata(buf_0, end,
                                   b"DNSMASQ_SUPPLIED_HOSTNAME\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char, &mut err);
                if is6 == 0 {
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_CPEWAN_OUI\x00" as *const u8
                                           as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_CPEWAN_SERIAL\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_CPEWAN_CLASS\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_CIRCUIT_ID\x00" as *const u8
                                           as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_SUBSCRIBER_ID\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_REMOTE_ID\x00" as *const u8
                                           as *const libc::c_char as
                                           *mut libc::c_char, &mut err);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_REQUESTED_OPTIONS\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err)
                }
                buf_0 =
                    grab_extradata(buf_0, end,
                                   b"DNSMASQ_TAGS\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, &mut err);
                if is6 != 0 {
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       b"DNSMASQ_RELAY_ADDRESS\x00" as
                                           *const u8 as *const libc::c_char as
                                           *mut libc::c_char, &mut err)
                } else {
                    my_setenv(b"DNSMASQ_RELAY_ADDRESS\x00" as *const u8 as
                                  *const libc::c_char,
                              if data.giaddr.s_addr !=
                                     0 as libc::c_int as libc::c_uint {
                                  inet_ntoa(data.giaddr)
                              } else { 0 as *mut libc::c_char }, &mut err);
                }
                i = 0 as libc::c_int;
                while !buf_0.is_null() {
                    sprintf(daemon.dhcp_buff2,
                            b"DNSMASQ_USER_CLASS%i\x00" as *const u8 as
                                *const libc::c_char, i);
                    buf_0 =
                        grab_extradata(buf_0, end,
                                       daemon.dhcp_buff2,
                                       &mut err);
                    i += 1
                }
                sprintf(daemon.dhcp_buff2,
                        b"%u\x00" as *const u8 as *const libc::c_char,
                        data.remaining_time);
                my_setenv(b"DNSMASQ_TIME_REMAINING\x00" as *const u8 as
                              *const libc::c_char,
                          if data.action != 1 as libc::c_int &&
                                 data.remaining_time !=
                                     0 as libc::c_int as libc::c_uint {
                              daemon.dhcp_buff2
                          } else { 0 as *mut libc::c_char }, &mut err);
                my_setenv(b"DNSMASQ_OLD_HOSTNAME\x00" as *const u8 as
                              *const libc::c_char,
                          if data.action == 2 as libc::c_int {
                              hostname
                          } else { 0 as *mut libc::c_char }, &mut err);
                if data.action == 2 as libc::c_int {
                    hostname = 0 as *mut libc::c_char
                }
                my_setenv(b"DNSMASQ_LOG_DHCP\x00" as *const u8 as
                              *const libc::c_char,
                          if daemon.options[(28 as libc::c_int as
                                                            libc::c_ulong).wrapping_div((::std::mem::size_of::<libc::c_uint>()
                                                                                             as
                                                                                             libc::c_ulong).wrapping_mul(8
                                                                                                                             as
                                                                                                                             libc::c_int
                                                                                                                             as
                                                                                                                             libc::c_ulong))
                                                           as usize] &
                                 (1 as libc::c_uint) <<
                                     (28 as libc::c_int as
                                          libc::c_ulong).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
                                                                           as
                                                                           libc::c_ulong).wrapping_mul(8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_ulong))
                                 != 0 {
                              b"1\x00" as *const u8 as *const libc::c_char
                          } else { 0 as *const libc::c_char }, &mut err);
            }
            /* we need to have the event_fd around if exec fails */
            i = fcntl(event_fd, 1 as libc::c_int);
            if i != -(1 as libc::c_int) {
                fcntl(event_fd, 2 as libc::c_int, i | 1 as libc::c_int);
            }
            close(pipefd[0 as libc::c_int as usize]);
            p = strrchr(daemon.lease_change_command, '/' as i32);
            if err == 0 as libc::c_int {
                execl(daemon.lease_change_command,
                      if !p.is_null() {
                          p.offset(1 as libc::c_int as isize)
                      } else { daemon.lease_change_command },
                      action_str,
                      if is6 != 0 && data.action != 6 as libc::c_int {
                          daemon.packet
                      } else { daemon.dhcp_buff },
                      daemon.addrbuff, hostname,
                      0 as *mut libc::c_void as *mut libc::c_char);
                err = *__errno_location()
            }
            /* failed, send event so the main process logs the problem */
            send_event(event_fd, 9 as libc::c_int, err,
                       0 as *mut libc::c_char);
            _exit(0 as libc::c_int);
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
unsafe extern "C" fn my_setenv(mut name: *const libc::c_char,
                               mut value: *const libc::c_char,
                               mut error: *mut libc::c_int) {
    if *error == 0 as libc::c_int {
        if value.is_null() {
            unsetenv(name);
        } else if setenv(name, value, 1 as libc::c_int) != 0 as libc::c_int {
            *error = *__errno_location()
        }
    };
}
unsafe extern "C" fn grab_extradata(mut buf_0: *mut libc::c_uchar,
                                    mut end: *mut libc::c_uchar,
                                    mut env: *mut libc::c_char,
                                    mut err: *mut libc::c_int)
 -> *mut libc::c_uchar {
    let mut next: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    if !buf_0.is_null() && buf_0 != end {
        next = buf_0;
        loop  {
            if next == end {
                next = 0 as *mut libc::c_uchar;
                break ;
            } else {
                if *next as libc::c_int == 0 as libc::c_int { break ; }
                next = next.offset(1)
            }
        }
        if !next.is_null() && next != buf_0 {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            /* No "=" in value */
            p = strchr(buf_0 as *mut libc::c_char, '=' as i32);
            if !p.is_null() { *p = 0 as libc::c_int as libc::c_char }
            val = buf_0 as *mut libc::c_char
        }
    }
    my_setenv(env, val, err);
    return if !next.is_null() {
               next.offset(1 as libc::c_int as isize)
           } else { 0 as *mut libc::c_uchar };
}
unsafe extern "C" fn buff_alloc(mut size: size_t) {
    if size > buf_size {
        let mut new: *mut script_data = 0 as *mut script_data;
        /* start with reasonable size, will almost never need extending. */
        if size <
               (::std::mem::size_of::<script_data>() as
                    libc::c_ulong).wrapping_add(200 as libc::c_int as
                                                    libc::c_ulong) {
            size =
                (::std::mem::size_of::<script_data>() as
                     libc::c_ulong).wrapping_add(200 as libc::c_int as
                                                     libc::c_ulong)
        }
        new = whine_malloc(size) as *mut script_data;
        if new.is_null() { return }
        if !buf.is_null() { free(buf as *mut libc::c_void); }
        buf = new;
        buf_size = size
    };
}
/* pack up lease data into a buffer */
#[no_mangle]
pub unsafe extern "C" fn queue_script(mut action: libc::c_int,
                                      mut lease: *mut dhcp_lease,
                                      mut hostname: *mut libc::c_char,
                                      mut now: time_t) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut hostname_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut clid_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ed_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut fd: libc::c_int = daemon.dhcpfd;
    if daemon.dhcp.is_null() { fd = daemon.dhcp6fd }
    /* no script */
    if daemon.helperfd == -(1 as libc::c_int) { return }
    if !(*lease).extradata.is_null() { ed_len = (*lease).extradata_len }
    if !(*lease).clid.is_null() {
        clid_len = (*lease).clid_len as libc::c_uint
    }
    if !hostname.is_null() {
        hostname_len =
            strlen(hostname).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_uint
    }
    buff_alloc((::std::mem::size_of::<script_data>() as
                    libc::c_ulong).wrapping_add(clid_len as
                                                    libc::c_ulong).wrapping_add(ed_len
                                                                                    as
                                                                                    libc::c_ulong).wrapping_add(hostname_len
                                                                                                                    as
                                                                                                                    libc::c_ulong));
    (*buf).action = action;
    (*buf).flags = (*lease).flags;
    (*buf).vendorclass_count = (*lease).vendorclass_count;
    (*buf).addr6 = (*lease).addr6;
    (*buf).iaid = (*lease).iaid;
    (*buf).hwaddr_len = (*lease).hwaddr_len;
    (*buf).hwaddr_type = (*lease).hwaddr_type;
    (*buf).clid_len = clid_len as libc::c_int;
    (*buf).ed_len = ed_len as libc::c_int;
    (*buf).hostname_len = hostname_len as libc::c_int;
    (*buf).addr = (*lease).addr;
    (*buf).giaddr = (*lease).giaddr;
    memcpy((*buf).hwaddr.as_mut_ptr() as *mut libc::c_void,
           (*lease).hwaddr.as_mut_ptr() as *const libc::c_void,
           16 as libc::c_int as libc::c_ulong);
    if indextoname(fd, (*lease).last_interface, (*buf).interface.as_mut_ptr())
           == 0 {
        (*buf).interface[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char
    }
    (*buf).expires = (*lease).expires;
    if (*lease).expires != 0 as libc::c_int as libc::c_long {
        (*buf).remaining_time =
            difftime((*lease).expires, now) as libc::c_uint
    } else { (*buf).remaining_time = 0 as libc::c_int as libc::c_uint }
    p = buf.offset(1 as libc::c_int as isize) as *mut libc::c_uchar;
    if clid_len != 0 as libc::c_int as libc::c_uint {
        memcpy(p as *mut libc::c_void, (*lease).clid as *const libc::c_void,
               clid_len as libc::c_ulong);
        p = p.offset(clid_len as isize)
    }
    if hostname_len != 0 as libc::c_int as libc::c_uint {
        memcpy(p as *mut libc::c_void, hostname as *const libc::c_void,
               hostname_len as libc::c_ulong);
        p = p.offset(hostname_len as isize)
    }
    if ed_len != 0 as libc::c_int as libc::c_uint {
        memcpy(p as *mut libc::c_void,
               (*lease).extradata as *const libc::c_void,
               ed_len as libc::c_ulong);
        p = p.offset(ed_len as isize)
    }
    bytes_in_buf =
        p.wrapping_offset_from(buf as *mut libc::c_uchar) as libc::c_long as
            size_t;
}
/* This nastily re-uses DHCP-fields for TFTP stuff */
#[no_mangle]
pub unsafe extern "C" fn queue_tftp(mut file_len: off_t,
                                    mut filename: *mut libc::c_char,
                                    mut peer: *mut mysockaddr) {
    let mut filename_len: libc::c_uint = 0;
    /* no script */
    if daemon.helperfd == -(1 as libc::c_int) { return }
    filename_len =
        strlen(filename).wrapping_add(1 as libc::c_int as libc::c_ulong) as
            libc::c_uint;
    buff_alloc((::std::mem::size_of::<script_data>() as
                    libc::c_ulong).wrapping_add(filename_len as
                                                    libc::c_ulong));
    memset(buf as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<script_data>() as libc::c_ulong);
    (*buf).action = 5 as libc::c_int;
    (*buf).hostname_len = filename_len as libc::c_int;
    (*buf).file_len = file_len;
    (*buf).flags = (*peer).sa.sa_family as libc::c_int;
    if (*buf).flags == 2 as libc::c_int {
        (*buf).addr = (*peer).in_0.sin_addr
    } else { (*buf).addr6 = (*peer).in6.sin6_addr }
    memcpy(buf.offset(1 as libc::c_int as isize) as *mut libc::c_uchar as
               *mut libc::c_void, filename as *const libc::c_void,
           filename_len as libc::c_ulong);
    bytes_in_buf =
        (::std::mem::size_of::<script_data>() as
             libc::c_ulong).wrapping_add(filename_len as libc::c_ulong);
}

pub fn queue_arp(daemon: &mut dnsmasq_daemon,
                 mut action: u32,
                 mut mac: &[u8;6],
                 mut family: u32,
                 mut addr: all_addr) {
    /* no script */
    if daemon.helperfd == -(1 as libc::c_int) { return }
    buff_alloc(::std::mem::size_of::<script_data>() as libc::c_ulong);
    memset(buf as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<script_data>() as libc::c_ulong);
    (*buf).action = action;
    (*buf).hwaddr_len = maclen;
    (*buf).hwaddr_type = 1 as libc::c_int;
    (*buf).flags = family;
    if (*buf).flags == 2 as libc::c_int {
        (*buf).addr = (*addr).addr4
    } else { (*buf).addr6 = (*addr).addr6 }
    memcpy((*buf).hwaddr.as_mut_ptr() as *mut libc::c_void,
           mac as *const libc::c_void, maclen as libc::c_ulong);
    bytes_in_buf = ::std::mem::size_of::<script_data>() as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn helper_buf_empty() -> libc::c_int {
    return (bytes_in_buf == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn helper_write() {
    let mut rc: ssize_t = 0;
    if bytes_in_buf == 0 as libc::c_int as libc::c_ulong { return }
    rc =
        write(daemon.helperfd, buf as *const libc::c_void,
              bytes_in_buf);
    if rc != -(1 as libc::c_int) as libc::c_long {
        if bytes_in_buf != rc as size_t {
            memmove(buf as *mut libc::c_void,
                    buf.offset(rc as isize) as *const libc::c_void,
                    bytes_in_buf.wrapping_sub(rc as libc::c_ulong));
        }
        bytes_in_buf =
            (bytes_in_buf as libc::c_ulong).wrapping_sub(rc as libc::c_ulong)
                as size_t as size_t
    } else {
        if *__errno_location() == 11 as libc::c_int ||
               *__errno_location() == 4 as libc::c_int {
            return
        }
        bytes_in_buf = 0 as libc::c_int as size_t
    };
}
