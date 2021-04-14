/* defaults in case we die() before we log_start() */
use crate::defines::{
    time::Instant, ConstNetAddressArg, DnsmasqDaemon, GidT, NetAddress, NetAddressUn, Passwd,
    SaFamily, SyscallSlongT, TimeT, SOCK_DGRAM, SOCK_STREAM,
};
use crate::poll::{poll_check, poll_listen};
use crate::send_event;
use std::env::args;
use std::io::stderr;

// static mut log_fac: i32 = (3) << 3;
// static mut log_stderr: i32 = 0;
// static mut echo_stderr: i32 = 0;
// static mut log_fd: i32 = -(1);
// static mut log_to_file: i32 = 0;
// static mut entries_alloced: i32 = 0;
// static mut entries_lost: i32 = 0;
// static mut connection_good: i32 = 1;
// static mut max_logs: i32 = 0;
// static mut connection_type: i32 = SOCK_DGRAM;
// static mut entries: log_entry = 0;
// static mut free_entries: log_entry =
//     0 p;

// pub fn log_start(mut ent_pw: &mut Passwd,
//                                    mut errfd: i32) -> i32 {
//     let mut ret: i32 = 0;
//     echo_stderr =
//         (dnsmasq_daemon.options[(6 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
//                                                                                        ).wrapping_mul(8))
//                                        ] &
//              (1) <<
//                  (6 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
//                                                    ).wrapping_mul(8)))
//            ;
//     if dnsmasq_daemon.log_fac != -(1) {
//         log_fac = dnsmasq_daemon.log_fac
//     } else if dnsmasq_daemon.options[(6 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
//                                                                                                  ).wrapping_mul(8))
//                                             ] &
//                   (1) <<
//                       (6).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
//                                                              ).wrapping_mul(8))
//                   != 0 {
//         log_fac = (16) << 3
//     }
//     if !dnsmasq_daemon.log_file.is_null() {
//         log_to_file = 1;
//         dnsmasq_daemon.max_logs = 0;
//         if strcmp(dnsmasq_daemon.log_file,
//                   "-" ) ==
//                0 {
//             log_stderr = 1;
//             echo_stderr = 0;
//             log_fd = dup(2)
//         }
//     }
//     max_logs = dnsmasq_daemon.max_logs;
//     if log_reopen(dnsmasq_daemon.log_file) == 0 {
//         send_event(errfd, 17, *__errno_location(),
//                    if !dnsmasq_daemon.log_file.is_null() {
//                        dnsmasq_daemon.log_file
//                    } else { ""  });
//         _exit(0);
//     }
//     /* if queuing is inhibited, make sure we allocate
//      the one required buffer now. */
//     if max_logs == 0 {
//         free_entries =
//             safe_malloc(::std::mem::size_of::<log_entry>())
//                 ;
//         free_entries.next = 0 ;
//         entries_alloced = 1
//     }
//     /* If we're running as root and going to change uid later,
//      change the ownership here so that the file is always owned by
//      the dnsmasq user. Then logrotate can just copy the owner.
//      Failure of the chown call is OK, (for instance when started as non-root) */
//     if log_to_file != 0 && log_stderr == 0 && !ent_pw.is_null() &&
//            ent_pw.pw_uid != 0 &&
//            fchown(log_fd, ent_pw.pw_uid, -(1) ) !=
//                0 {
//         ret = *__errno_location()
//     }
//     return ret;
// }

// pub fn log_reopen(mut log_file: &mut String)
//  -> i32 {
//     if log_stderr == 0 {
//         if log_fd != -(1) { close(log_fd); }
//         /* NOTE: umask is set to 022 by the time this gets called */
//         if !log_file.is_null() {
//             log_fd =
//                 open(log_file,
//                      0o1 | 0o100 |
//                          0o2000,
//                      0o400 | 0o200 |
//                          0o400 >> 3)
//         } else {
//             let mut flags: i32 = 0;
//             log_fd =
//                 socket(1, connection_type, 0);
//             /* if max_logs is zero, leave the socket blocking */
//             if log_fd != -(1) && max_logs != 0
//                    &&
//                    {
//                        flags = fcntl(log_fd, 3);
//                        (flags) != -(1)
//                    } {
//                 fcntl(log_fd, 4,
//                       flags | 0o4000);
//             }
//         }
//     }
//     return (log_fd != -(1));
// }
// fn free_entry() {
//     let mut tmp: log_entry = entries;
//     entries = tmp.next;
//     tmp.next = free_entries;
//     free_entries = tmp;
// }
// fn log_write() {
//     let mut rc: isize = 0;
//     while !entries.is_null() {
//         /* The data in the payload is written with a terminating zero character
// 	 and the length reflects this. For a stream connection we need to
// 	 send the zero as a record terminator, but this isn't done for a
// 	 datagram connection, so treat the length as one less than reality
// 	 to elide the zero. If we're logging to a file, turn the zero into
// 	 a newline, and leave the length alone. */
//         let mut len_adjust: i32 = 0;
//         if log_to_file != 0 {
//             entries.payload[(entries.offset + entries.length -
//                                     1) ] =
//                 '\n'
//         } else if connection_type == SOCK_DGRAM {
//             len_adjust = 1
//         }
//         /* Avoid duplicates over a fork() */
//         if entries.pid != getpid() {
//             free_entry(); /* avoid wild recursion */
//         } else {
//             connection_good =
//                 1 ; /* syslogd busy, go again when select() or poll() says so */
//             rc =
//                 write(log_fd,
//                       entries.payload.as_mut_ptr().offset(entries.offset
//                                                                 ) ,
//                       (entries.length - len_adjust) );
//             if rc != -(1) {
//                 entries.length =
//                     (entries.length - rc);
//                 entries.offset =
//                     (entries.offset + rc);
//                 if entries.length == len_adjust {
//                     free_entry();
//                     if entries_lost != 0 {
//                         let mut e: i32 = entries_lost;
//                         entries_lost = 0;
//                         my_syslog(4,
//                                   "overflow: %d log entries lost"                                *const u8, e);
//                     }
//                 }
//             } else {
//                 if *__errno_location() == 4 { continue ; }
//                 if *__errno_location() == 11 ||
//                        *__errno_location() == 11 {
//                     return
//                 }
//                 if *__errno_location() == 105 {
//                     connection_good = 0;
//                     return
//                 }
//                 /* errors handling after this assumes sockets */
//                 if log_to_file == 0 {
//                     /* Once a stream socket hits EPIPE, we have to close and re-open
// 	     (we ignore SIGPIPE) */
//                     if *__errno_location() == 32 {
//                         if log_reopen(0 ) != 0 {
//                             continue ;
//                         }
//                     } else if *__errno_location() == 111 ||
//                                   *__errno_location() == 107 ||
//                                   *__errno_location() == 89 ||
//                                   *__errno_location() == 104 {
//                         /* socket went (syslogd down?), try and reconnect. If we fail,
// 		 stop trying until the next call to my_syslog()
// 		 ECONNREFUSED -> connection went down
// 		 ENOTCONN -> nobody listening
// 		 (ECONNRESET, EDESTADDRREQ are *BSD equivalents) */
//                         let mut logaddr: NetAddressUn =
//                             NetAddressUn {sun_family: 0, sun_path: [0; 108],};
//                         logaddr.sun_family = 1;
//                         safe_strncpy(logaddr.sun_path.as_mut_ptr(),
//                                      "/dev/log",
//                                      ::std::mem::size_of::<[libc::c_char; 108]>()
//                                         );
//                         /* Got connection back? try again. */
//                         if connect(log_fd,
//                                    ConstNetAddressArg {__NetAddress__:
//                                                             &mut logaddr             NetAddressUn
//                                                                             NetAddress,},
//                                    ::std::mem::size_of::<NetAddressUn>()                          ) !=
//                                -(1) {
//                             continue ;
//                         }
//                         /* errors from connect which mean we should keep trying */
//                         if *__errno_location() == 2 ||
//                                *__errno_location() == 114 ||
//                                *__errno_location() == 111 ||
//                                *__errno_location() == 106 ||
//                                *__errno_location() == 4 ||
//                                *__errno_location() == 11 ||
//                                *__errno_location() == 11 {
//                             /* try again on next syslog() call */
//                             connection_good = 0;
//                             return
//                         }
//                         /* try the other sort of socket... */
//                         if *__errno_location() == 91 {
//                             connection_type =
//                                 if connection_type ==
//                                        SOCK_DGRAM {
//                                     SOCK_STREAM
//                                 } else { SOCK_DGRAM };
//                             if log_reopen(0 ) != 0 {
//                                 continue ;
//                             }
//                         }
//                     }
//                 }
//                 /* give up - fall back to syslog() - this handles out-of-space
// 	 when logging to a file, for instance. */
//                 log_fd = -(1);
//                 my_syslog(2,
//                           "log failed: %s" ,
//                           strerror(*__errno_location()));
//                 return
//             }
//         }
//     };
// }
// /* priority is one of LOG_DEBUG, LOG_INFO, LOG_NOTICE, etc. See sys/syslog.h.
//    OR'd to priority can be MS_TFTP, MS_DHCP, ... to be able to do log separation between
//    DNS, DHCP and TFTP services.
// */
// pub fn my_syslog(mut priority: i32,
//                                    mut format: *const libc::c_char,
//                                    mut args: ...) {
//     let mut ap: ::std::ffi::VaListImpl;
//     let mut entry: log_entry = 0 ;
//     let mut time_now: time::Instant = 0;
//     let mut p: &mut String = 0 ;
//     let mut len: usize = 0;
//     let mut pid: pid_t = getpid();
//     let mut func: &mut String =
//         ""  ;
//     if 0x3f8 & priority ==
//            (1) << 3 {
//         func =
//             "-tftp"           &mut String
//     } else if 0x3f8 & priority ==
//                   (3) << 3 {
//         func =
//             "-dhcp"           &mut String
//     } else if 0x3f8 & priority ==
//                   (2) << 3 {
//         func =
//             "-script"           &mut String
//     }
//     priority = priority & 0x7;
//     if echo_stderr != 0 {
//         fprintf(stderr,
//                 "dnsmasq%s: " , func);
//         ap = args.clone();
//         vfprintf(stderr, format, ap.as_va_list());
//         fputc('\n' as i32, stderr);
//     }
//     if log_fd == -(1) {
//         /* fall-back to syslog if we die during startup or
// 	 fail during running (always on Solaris). */
//         static mut isopen: i32 = 0;
//         if isopen == 0 {
//             openlog("dnsmasq" ,
//                     0x1, log_fac);
//             isopen = 1
//         }
//         ap = args.clone();
//         vsyslog(priority, format, ap.as_va_list());
//         return
//     }
//     entry = free_entries;
//     if !entry.is_null() {
//         free_entries = entry.next
//     } else if entries_alloced < max_logs &&
//                   {
//                       // entry =
//                       //     malloc(::std::mem::size_of::<log_entry>()                        ) ;
//                       // !entry.is_null()
//                       true
//                   } {
//         entries_alloced += 1
//     }
//     if entry.is_null() {
//         entries_lost += 1
//     } else {
//         /* add to end of list, consumed from the start */
//         entry.next = 0 ;
//         if entries.is_null() {
//             entries = entry
//         } else {
//             let mut tmp: log_entry = 0 ;
//             tmp = entries;
//             while !tmp.next.is_null() { tmp = tmp.next }
//             tmp.next = entry
//         }
//         time(&mut time_now);
//         p = entry.payload.as_mut_ptr();
//         if log_to_file == 0 {
//             p =
//                 p.offset(sprintf(p,
//                                  "<%d>"                                *const libc::c_char, priority | log_fac)
//                             )
//         }
//         /* Omit timestamp for default daemontools situation */
//         if log_stderr == 0 ||
//                dnsmasq_daemon.options[(16                                 ).wrapping_div((::std::mem::size_of::<libc::c_uint>()
//                                                                                                    ).wrapping_mul(8                                                                                   ))
//                                              ] &
//                    (1) <<
//                        (16 ).wrapping_rem((::std::mem::size_of::<libc::c_uint>()
//                                                                ).wrapping_mul(8           ))
//                    == 0 {
//             p =
//                 p.offset(sprintf(p,
//                                  "%.15s "                                *const libc::c_char,
//                                  ctime(&mut time_now).offset(4
//                                                                 ))                       )
//         } /* include zero-terminator */
//         p =
//             p.offset(sprintf(p,
//                              "dnsmasq%s[%d]: "                            *const libc::c_char, func, pid));
//         len =
//             p.wrapping_offset_from(entry.payload.as_mut_ptr())          i32 ;
//         ap = args.clone();
//         len =
//             (len    ).wrapping_add((vsnprintf(p,
//                                                         (1024
//                                                                ).wrapping_sub(len),
//                                                         format,
//                                                         ap.as_va_list()) +
//                                                   1)                                    )           usize;
//         entry.length =
//             if len > 1024 {
//                 1024
//             } else { len };
//         entry.offset = 0;
//         entry.pid = pid
//     }
//     /* almost always, logging won't block, so try and write this now,
//      to save collecting too many log messages during a select loop. */
//     log_write();
//     /* Since we're doing things asynchronously, a cache-dump, for instance,
//      can now generate log lines very fast. With a small buffer (desirable),
//      that means it can overflow the log-buffer very quickly,
//      so that the cache dump becomes mainly a count of how many lines
//      overflowed. To avoid this, we delay here, the delay is controlled
//      by queue-occupancy, and grows exponentially. The delay is limited to (2^8)ms.
//      The scaling stuff ensures that when the queue is bigger than 8, the delay
//      only occurs for the last 8 entries. Once the queue is full, we stop delaying
//      to preserve performance.
//   */
//     if !entries.is_null() && max_logs != 0 {
//         let mut d: i32 = 0; /* 1 ms */
//         d = 0;
//         entry = entries;
//         while !entry.is_null() { entry = entry.next; d += 1 }
//         if d == max_logs {
//             d = 0
//         } else if max_logs > 8 {
//             d -= max_logs - 8
//         }
//         if d > 0 {
//             let mut waiter: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
//             waiter.tv_sec = 0 ;
//             waiter.tv_nsec =
//                 ((1000000) << d - 1)              SyscallSlongT;
//             nanosleep(&mut waiter, 0);
//             /* Have another go now */
//             log_write();
//         }
//     };
// }

// pub fn set_log_writer() {
//     if !entries.is_null() && log_fd != -(1) &&
//            connection_good != 0 {
//         poll_listen(log_fd, 0x4 );
//     };
// }

// pub fn check_log_writer(mut force: i32) {
//     if log_fd != -(1) &&
//            (force != 0 ||
//                 poll_check(log_fd, 0x4 ) != 0)
//        {
//         log_write();
//     };
// }

// pub fn flush_log() {
//     /* write until queue empty, but don't loop forever if there's
//    no connection to the syslog in existence */
//     while log_fd != -(1) {
//         let mut waiter: timespec =
//             timespec{tv_sec: 0, tv_nsec: 0,}; /* 1 ms */
//         log_write(); /* print as well as log when we die.... */
//         if entries.is_null() || connection_good == 0 {
//             close(log_fd);
//             break ;
//         } else {
//             waiter.tv_sec = 0 ;
//             waiter.tv_nsec = 1000000;
//             nanosleep(&mut waiter, 0);
//         }
//     };
// }

// pub fn die(mut message: &mut String,
//                              mut arg1: &mut String,
//                              mut exit_code: i32) -> ! {
//     let mut errmess: &mut String = strerror(*__errno_location());
//     if arg1.is_null() { arg1 = errmess }
//     if log_stderr == 0 {
//         echo_stderr = 1;
//         fputc('\n' as i32, stderr);
//         /* prettyfy  startup-script message */
//     }
//     my_syslog(2, message, arg1, errmess);
//     echo_stderr = 0;
//     my_syslog(2,
//               "FAILED to start up" );
//     flush_log();
//     exit(exit_code);
// }
