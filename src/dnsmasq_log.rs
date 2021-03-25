
/* defaults in case we die() before we log_start() */
static mut log_fac: libc::c_int = (3 as libc::c_int) << 3 as libc::c_int;
static mut log_stderr: libc::c_int = 0 as libc::c_int;
static mut echo_stderr: libc::c_int = 0 as libc::c_int;
static mut log_fd: libc::c_int = -(1 as libc::c_int);
static mut log_to_file: libc::c_int = 0 as libc::c_int;
static mut entries_alloced: libc::c_int = 0 as libc::c_int;
static mut entries_lost: libc::c_int = 0 as libc::c_int;
static mut connection_good: libc::c_int = 1 as libc::c_int;
static mut max_logs: libc::c_int = 0 as libc::c_int;
static mut connection_type: libc::c_int = SOCK_DGRAM as libc::c_int;
static mut entries: *mut log_entry = 0 as *const log_entry as *mut log_entry;
static mut free_entries: *mut log_entry =
    0 as *const log_entry as *mut log_entry;
#[no_mangle]
pub unsafe extern "C" fn log_start(mut ent_pw: *mut passwd,
                                   mut errfd: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    echo_stderr =
        ((*dnsmasq_daemon).options[(6 as libc::c_int as
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
                                                                                       libc::c_ulong)))
            as libc::c_int;
    if (*dnsmasq_daemon).log_fac != -(1 as libc::c_int) {
        log_fac = (*dnsmasq_daemon).log_fac
    } else if (*dnsmasq_daemon).options[(6 as libc::c_int as
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
        log_fac = (16 as libc::c_int) << 3 as libc::c_int
    }
    if !(*dnsmasq_daemon).log_file.is_null() {
        log_to_file = 1 as libc::c_int;
        (*dnsmasq_daemon).max_logs = 0 as libc::c_int;
        if strcmp((*dnsmasq_daemon).log_file,
                  b"-\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            log_stderr = 1 as libc::c_int;
            echo_stderr = 0 as libc::c_int;
            log_fd = dup(2 as libc::c_int)
        }
    }
    max_logs = (*dnsmasq_daemon).max_logs;
    if log_reopen((*dnsmasq_daemon).log_file) == 0 {
        send_event(errfd, 17 as libc::c_int, *__errno_location(),
                   if !(*dnsmasq_daemon).log_file.is_null() {
                       (*dnsmasq_daemon).log_file as *const libc::c_char
                   } else { b"\x00" as *const u8 as *const libc::c_char } as
                       *mut libc::c_char);
        _exit(0 as libc::c_int);
    }
    /* if queuing is inhibited, make sure we allocate
     the one required buffer now. */
    if max_logs == 0 as libc::c_int {
        free_entries =
            safe_malloc(::std::mem::size_of::<log_entry>() as libc::c_ulong)
                as *mut log_entry;
        (*free_entries).next = 0 as *mut log_entry;
        entries_alloced = 1 as libc::c_int
    }
    /* If we're running as root and going to change uid later,
     change the ownership here so that the file is always owned by
     the dnsmasq user. Then logrotate can just copy the owner.
     Failure of the chown call is OK, (for instance when started as non-root) */
    if log_to_file != 0 && log_stderr == 0 && !ent_pw.is_null() &&
           (*ent_pw).pw_uid != 0 as libc::c_int as libc::c_uint &&
           fchown(log_fd, (*ent_pw).pw_uid, -(1 as libc::c_int) as __gid_t) !=
               0 as libc::c_int {
        ret = *__errno_location()
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn log_reopen(mut log_file: *mut libc::c_char)
 -> libc::c_int {
    if log_stderr == 0 {
        if log_fd != -(1 as libc::c_int) { close(log_fd); }
        /* NOTE: umask is set to 022 by the time this gets called */
        if !log_file.is_null() {
            log_fd =
                open(log_file,
                     0o1 as libc::c_int | 0o100 as libc::c_int |
                         0o2000 as libc::c_int,
                     0o400 as libc::c_int | 0o200 as libc::c_int |
                         0o400 as libc::c_int >> 3 as libc::c_int)
        } else {
            let mut flags: libc::c_int = 0;
            log_fd =
                socket(1 as libc::c_int, connection_type, 0 as libc::c_int);
            /* if max_logs is zero, leave the socket blocking */
            if log_fd != -(1 as libc::c_int) && max_logs != 0 as libc::c_int
                   &&
                   {
                       flags = fcntl(log_fd, 3 as libc::c_int);
                       (flags) != -(1 as libc::c_int)
                   } {
                fcntl(log_fd, 4 as libc::c_int,
                      flags | 0o4000 as libc::c_int);
            }
        }
    }
    return (log_fd != -(1 as libc::c_int)) as libc::c_int;
}
unsafe extern "C" fn free_entry() {
    let mut tmp: *mut log_entry = entries;
    entries = (*tmp).next;
    (*tmp).next = free_entries;
    free_entries = tmp;
}
unsafe extern "C" fn log_write() {
    let mut rc: ssize_t = 0;
    while !entries.is_null() {
        /* The data in the payload is written with a terminating zero character 
	 and the length reflects this. For a stream connection we need to 
	 send the zero as a record terminator, but this isn't done for a 
	 datagram connection, so treat the length as one less than reality 
	 to elide the zero. If we're logging to a file, turn the zero into 
	 a newline, and leave the length alone. */
        let mut len_adjust: libc::c_int = 0 as libc::c_int;
        if log_to_file != 0 {
            (*entries).payload[((*entries).offset + (*entries).length -
                                    1 as libc::c_int) as usize] =
                '\n' as i32 as libc::c_char
        } else if connection_type == SOCK_DGRAM as libc::c_int {
            len_adjust = 1 as libc::c_int
        }
        /* Avoid duplicates over a fork() */
        if (*entries).pid != getpid() {
            free_entry(); /* avoid wild recursion */
        } else {
            connection_good =
                1 as
                    libc::c_int; /* syslogd busy, go again when select() or poll() says so */
            rc =
                write(log_fd,
                      (*entries).payload.as_mut_ptr().offset((*entries).offset
                                                                 as isize) as
                          *const libc::c_void,
                      ((*entries).length - len_adjust) as size_t);
            if rc != -(1 as libc::c_int) as libc::c_long {
                (*entries).length =
                    ((*entries).length as libc::c_long - rc) as libc::c_int;
                (*entries).offset =
                    ((*entries).offset as libc::c_long + rc) as libc::c_int;
                if (*entries).length == len_adjust {
                    free_entry();
                    if entries_lost != 0 as libc::c_int {
                        let mut e: libc::c_int = entries_lost;
                        entries_lost = 0 as libc::c_int;
                        my_syslog(4 as libc::c_int,
                                  b"overflow: %d log entries lost\x00" as
                                      *const u8 as *const libc::c_char, e);
                    }
                }
            } else {
                if *__errno_location() == 4 as libc::c_int { continue ; }
                if *__errno_location() == 11 as libc::c_int ||
                       *__errno_location() == 11 as libc::c_int {
                    return
                }
                if *__errno_location() == 105 as libc::c_int {
                    connection_good = 0 as libc::c_int;
                    return
                }
                /* errors handling after this assumes sockets */
                if log_to_file == 0 {
                    /* Once a stream socket hits EPIPE, we have to close and re-open
	     (we ignore SIGPIPE) */
                    if *__errno_location() == 32 as libc::c_int {
                        if log_reopen(0 as *mut libc::c_char) != 0 {
                            continue ;
                        }
                    } else if *__errno_location() == 111 as libc::c_int ||
                                  *__errno_location() == 107 as libc::c_int ||
                                  *__errno_location() == 89 as libc::c_int ||
                                  *__errno_location() == 104 as libc::c_int {
                        /* socket went (syslogd down?), try and reconnect. If we fail,
		 stop trying until the next call to my_syslog() 
		 ECONNREFUSED -> connection went down
		 ENOTCONN -> nobody listening
		 (ECONNRESET, EDESTADDRREQ are *BSD equivalents) */
                        let mut logaddr: sockaddr_un =
                            sockaddr_un{sun_family: 0, sun_path: [0; 108],};
                        logaddr.sun_family = 1 as libc::c_int as sa_family_t;
                        safe_strncpy(logaddr.sun_path.as_mut_ptr(),
                                     b"/dev/log\x00" as *const u8 as
                                         *const libc::c_char,
                                     ::std::mem::size_of::<[libc::c_char; 108]>()
                                         as libc::c_ulong);
                        /* Got connection back? try again. */
                        if connect(log_fd,
                                   __CONST_SOCKADDR_ARG{__sockaddr__:
                                                            &mut logaddr as
                                                                *mut sockaddr_un
                                                                as
                                                                *mut sockaddr,},
                                   ::std::mem::size_of::<sockaddr_un>() as
                                       libc::c_ulong as socklen_t) !=
                               -(1 as libc::c_int) {
                            continue ;
                        }
                        /* errors from connect which mean we should keep trying */
                        if *__errno_location() == 2 as libc::c_int ||
                               *__errno_location() == 114 as libc::c_int ||
                               *__errno_location() == 111 as libc::c_int ||
                               *__errno_location() == 106 as libc::c_int ||
                               *__errno_location() == 4 as libc::c_int ||
                               *__errno_location() == 11 as libc::c_int ||
                               *__errno_location() == 11 as libc::c_int {
                            /* try again on next syslog() call */
                            connection_good = 0 as libc::c_int;
                            return
                        }
                        /* try the other sort of socket... */
                        if *__errno_location() == 91 as libc::c_int {
                            connection_type =
                                if connection_type ==
                                       SOCK_DGRAM as libc::c_int {
                                    SOCK_STREAM as libc::c_int
                                } else { SOCK_DGRAM as libc::c_int };
                            if log_reopen(0 as *mut libc::c_char) != 0 {
                                continue ;
                            }
                        }
                    }
                }
                /* give up - fall back to syslog() - this handles out-of-space
	 when logging to a file, for instance. */
                log_fd = -(1 as libc::c_int);
                my_syslog(2 as libc::c_int,
                          b"log failed: %s\x00" as *const u8 as
                              *const libc::c_char,
                          strerror(*__errno_location()));
                return
            }
        }
    };
}
/* priority is one of LOG_DEBUG, LOG_INFO, LOG_NOTICE, etc. See sys/syslog.h.
   OR'd to priority can be MS_TFTP, MS_DHCP, ... to be able to do log separation between
   DNS, DHCP and TFTP services.
*/
#[no_mangle]
pub unsafe extern "C" fn my_syslog(mut priority: libc::c_int,
                                   mut format: *const libc::c_char,
                                   mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut entry: *mut log_entry = 0 as *mut log_entry;
    let mut time_now: time_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut pid: pid_t = getpid();
    let mut func: *mut libc::c_char =
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if 0x3f8 as libc::c_int & priority ==
           (1 as libc::c_int) << 3 as libc::c_int {
        func =
            b"-tftp\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if 0x3f8 as libc::c_int & priority ==
                  (3 as libc::c_int) << 3 as libc::c_int {
        func =
            b"-dhcp\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if 0x3f8 as libc::c_int & priority ==
                  (2 as libc::c_int) << 3 as libc::c_int {
        func =
            b"-script\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    }
    priority = priority & 0x7 as libc::c_int;
    if echo_stderr != 0 {
        fprintf(stderr,
                b"dnsmasq%s: \x00" as *const u8 as *const libc::c_char, func);
        ap = args.clone();
        vfprintf(stderr, format, ap.as_va_list());
        fputc('\n' as i32, stderr);
    }
    if log_fd == -(1 as libc::c_int) {
        /* fall-back to syslog if we die during startup or 
	 fail during running (always on Solaris). */
        static mut isopen: libc::c_int = 0 as libc::c_int;
        if isopen == 0 {
            openlog(b"dnsmasq\x00" as *const u8 as *const libc::c_char,
                    0x1 as libc::c_int, log_fac);
            isopen = 1 as libc::c_int
        }
        ap = args.clone();
        vsyslog(priority, format, ap.as_va_list());
        return
    }
    entry = free_entries;
    if !entry.is_null() {
        free_entries = (*entry).next
    } else if entries_alloced < max_logs &&
                  {
                      entry =
                          malloc(::std::mem::size_of::<log_entry>() as
                                     libc::c_ulong) as *mut log_entry;
                      !entry.is_null()
                  } {
        entries_alloced += 1
    }
    if entry.is_null() {
        entries_lost += 1
    } else {
        /* add to end of list, consumed from the start */
        (*entry).next = 0 as *mut log_entry;
        if entries.is_null() {
            entries = entry
        } else {
            let mut tmp: *mut log_entry = 0 as *mut log_entry;
            tmp = entries;
            while !(*tmp).next.is_null() { tmp = (*tmp).next }
            (*tmp).next = entry
        }
        time(&mut time_now);
        p = (*entry).payload.as_mut_ptr();
        if log_to_file == 0 {
            p =
                p.offset(sprintf(p,
                                 b"<%d>\x00" as *const u8 as
                                     *const libc::c_char, priority | log_fac)
                             as isize)
        }
        /* Omit timestamp for default daemontools situation */
        if log_stderr == 0 ||
               (*dnsmasq_daemon).options[(16 as libc::c_int as
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
            p =
                p.offset(sprintf(p,
                                 b"%.15s \x00" as *const u8 as
                                     *const libc::c_char,
                                 ctime(&mut time_now).offset(4 as libc::c_int
                                                                 as isize)) as
                             isize)
        } /* include zero-terminator */
        p =
            p.offset(sprintf(p,
                             b"dnsmasq%s[%d]: \x00" as *const u8 as
                                 *const libc::c_char, func, pid) as isize);
        len =
            p.wrapping_offset_from((*entry).payload.as_mut_ptr()) as
                libc::c_long as size_t;
        ap = args.clone();
        len =
            (len as
                 libc::c_ulong).wrapping_add((vsnprintf(p,
                                                        (1024 as libc::c_int
                                                             as
                                                             libc::c_ulong).wrapping_sub(len),
                                                        format,
                                                        ap.as_va_list()) +
                                                  1 as libc::c_int) as
                                                 libc::c_ulong) as size_t as
                size_t;
        (*entry).length =
            if len > 1024 as libc::c_int as libc::c_ulong {
                1024 as libc::c_int as libc::c_ulong
            } else { len } as libc::c_int;
        (*entry).offset = 0 as libc::c_int;
        (*entry).pid = pid
    }
    /* almost always, logging won't block, so try and write this now,
     to save collecting too many log messages during a select loop. */
    log_write();
    /* Since we're doing things asynchronously, a cache-dump, for instance,
     can now generate log lines very fast. With a small buffer (desirable),
     that means it can overflow the log-buffer very quickly,
     so that the cache dump becomes mainly a count of how many lines 
     overflowed. To avoid this, we delay here, the delay is controlled 
     by queue-occupancy, and grows exponentially. The delay is limited to (2^8)ms.
     The scaling stuff ensures that when the queue is bigger than 8, the delay
     only occurs for the last 8 entries. Once the queue is full, we stop delaying
     to preserve performance.
  */
    if !entries.is_null() && max_logs != 0 as libc::c_int {
        let mut d: libc::c_int = 0; /* 1 ms */
        d = 0 as libc::c_int;
        entry = entries;
        while !entry.is_null() { entry = (*entry).next; d += 1 }
        if d == max_logs {
            d = 0 as libc::c_int
        } else if max_logs > 8 as libc::c_int {
            d -= max_logs - 8 as libc::c_int
        }
        if d > 0 as libc::c_int {
            let mut waiter: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
            waiter.tv_sec = 0 as libc::c_int as __time_t;
            waiter.tv_nsec =
                ((1000000 as libc::c_int) << d - 1 as libc::c_int) as
                    __syscall_slong_t;
            nanosleep(&mut waiter, 0 as *mut timespec);
            /* Have another go now */
            log_write();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_log_writer() {
    if !entries.is_null() && log_fd != -(1 as libc::c_int) &&
           connection_good != 0 {
        poll_listen(log_fd, 0x4 as libc::c_int as libc::c_short);
    };
}
#[no_mangle]
pub unsafe extern "C" fn check_log_writer(mut force: libc::c_int) {
    if log_fd != -(1 as libc::c_int) &&
           (force != 0 ||
                poll_check(log_fd, 0x4 as libc::c_int as libc::c_short) != 0)
       {
        log_write();
    };
}
#[no_mangle]
pub unsafe extern "C" fn flush_log() {
    /* write until queue empty, but don't loop forever if there's
   no connection to the syslog in existence */
    while log_fd != -(1 as libc::c_int) {
        let mut waiter: timespec =
            timespec{tv_sec: 0, tv_nsec: 0,}; /* 1 ms */
        log_write(); /* print as well as log when we die.... */
        if entries.is_null() || connection_good == 0 {
            close(log_fd);
            break ;
        } else {
            waiter.tv_sec = 0 as libc::c_int as __time_t;
            waiter.tv_nsec = 1000000 as libc::c_int as __syscall_slong_t;
            nanosleep(&mut waiter, 0 as *mut timespec);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn die(mut message: *mut libc::c_char,
                             mut arg1: *mut libc::c_char,
                             mut exit_code: libc::c_int) -> ! {
    let mut errmess: *mut libc::c_char = strerror(*__errno_location());
    if arg1.is_null() { arg1 = errmess }
    if log_stderr == 0 {
        echo_stderr = 1 as libc::c_int;
        fputc('\n' as i32, stderr);
        /* prettyfy  startup-script message */
    }
    my_syslog(2 as libc::c_int, message, arg1, errmess);
    echo_stderr = 0 as libc::c_int;
    my_syslog(2 as libc::c_int,
              b"FAILED to start up\x00" as *const u8 as *const libc::c_char);
    flush_log();
    exit(exit_code);
}
